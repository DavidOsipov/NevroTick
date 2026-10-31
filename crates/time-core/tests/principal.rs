// SPDX-License-Identifier: GPL-2.0-only
//! Independent CHECK-7/8/14/25 principal hysteresis through genuine producers.
//!
//! Flat zero-phase histories and literal dyadic distances own the oracle. A
//! principal of distance one compares against a challenger of distance one
//! quarter with reselection one quarter: its pair multiplier is exactly two.
//! Three genuine updates yield scores two, four and eight; the fourth replaces
//! the principal and resets scores to one. Reprojection creates no new event.
//! Counters precharge actual external calls and emit their complete attempted
//! prefix on unwind. Internal history/selector work needs separate reservations.
//! This inert fixture establishes no production freshness or clock authority.
#![forbid(unsafe_code)]

use std::{
    cell::Cell,
    error::Error,
    io::{self, Write},
    num::NonZeroU64,
};

use time_core::{
    FixtureContext, FixtureHistory, FixtureMinimum, FixtureObservation, FixtureOutcomeKind,
    FixturePrincipalDecision, FixturePrincipalOutcomeKind, FixturePrincipalParameters,
    FixturePrincipalState, FixturePrincipalStratum, FixtureProjectedBatch,
    FixtureProjectedInterval, FixtureProjectionTerms, enclose_fixture_projection,
    project_fixture_anchor,
};
use time_domain::{MonotonicNs, PhaseOffsetQ33, UtcCoordinateQ32, exchange_arithmetic};

/// Harness failures remain distinct from actual product rejections.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Closed field order printed by the source-owned profile below.
type Recipe = [u64; 17];

/// One bounded, local actual-attempt profile without shared counters.
struct Profile {
    /// Fixed maximum external attempts in the documented field order.
    ceiling: Recipe,
    /// Attempts charged before the product call, including calls returning error.
    counts: [Cell<u64>; 17],
}

impl Profile {
    /// Start an all-zero scope with explicit finite external-call ceilings.
    const fn new() -> Self {
        Self {
            ceiling: [2, 10, 10, 2, 64, 1, 1, 320, 18, 18, 18, 9, 1, 1, 2, 1, 9],
            counts: [const { Cell::new(0) }; 17],
        }
    }

    /// Precharge one call and preserve its actual result without reinterpretation.
    /// # Errors
    /// Rejects invalid accounting slots, overflow or exhaustion before the call.
    fn call<T>(&self, slot: usize, call: impl FnOnce() -> T) -> TestResult<T> {
        let counter = self
            .counts
            .get(slot)
            .ok_or_else(|| io::Error::other("principal counter slot"))?;
        let ceiling = self
            .ceiling
            .get(slot)
            .ok_or_else(|| io::Error::other("principal ceiling slot"))?;
        let next = counter
            .get()
            .checked_add(1)
            .ok_or_else(|| io::Error::other("principal counter overflow"))?;
        if next > *ceiling {
            return Err(io::Error::other("principal counter ceiling").into());
        }
        counter.set(next);
        Ok(call())
    }

    /// Charge the sole public-getter slot before the immutable observation.
    /// # Errors
    /// Propagates accounting failure without calling the getter.
    fn getter<T>(&self, getter: impl FnOnce() -> T) -> TestResult<T> {
        self.call(7, getter)
    }

    /// Check the complete healthy recipe independently of partial RED profiles.
    /// # Panics
    /// The completed test must perform exactly its declared external calls.
    fn assert_complete(&self) {
        assert_eq!(
            self.counts.each_ref().map(Cell::get),
            [2, 10, 10, 2, 33, 1, 1, 259, 18, 18, 18, 9, 1, 1, 2, 1, 9]
        );
    }
}

impl Drop for Profile {
    /// Emit actual attempted counts even during assertion unwind.
    /// Missing or failed emission remains incomplete supervisor evidence.
    fn drop(&mut self) {
        let c = &self.counts;
        let row = format!(
            "CORE_PRINCIPAL_PROFILE case=principal_seed_and_genuine_update_hysteresis histories={} appends={} observations={} contexts={} monotonic={} coordinates={} exchanges={} getters={} terms={} projections={} enclosures={} batches={} minima={} parameters={} strata={} seeds={} advances={}\n",
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
            c[13].get(),
            c[14].get(),
            c[15].get(),
            c[16].get()
        );
        let _emission_result = io::stderr().lock().write_all(row.as_bytes());
    }
}

/// Admit an explicit nonzero fixture tag without fallback or product authority.
/// # Errors
/// Rejects zero before a context or metadata constructor.
fn tag(raw: u64) -> TestResult<NonZeroU64> {
    NonZeroU64::new(raw).ok_or_else(|| io::Error::other("zero principal fixture tag").into())
}

/// Produce exact zero phase through real checked UTC and exchange arithmetic.
/// # Errors
/// Preserves actual constructor/arithmetic rejection or accounting failure.
fn zero_phase(profile: &Profile) -> TestResult<PhaseOffsetQ33> {
    let coordinate = profile.call(5, || UtcCoordinateQ32::new(0))??;
    let exchange = profile.call(6, || {
        exchange_arithmetic(coordinate, coordinate, coordinate, coordinate)
    })??;
    profile.getter(|| exchange.offset())
}

/// Commit three genuine constant rows, with occupied append lengths one to three.
/// # Errors
/// Preserves actual history/observation admission and bounded accounting errors.
fn flat_history(
    profile: &Profile,
    context: FixtureContext,
    phase: PhaseOffsetQ33,
) -> TestResult<FixtureHistory> {
    let mut history = profile.call(0, || FixtureHistory::new(context))?;
    for raw_at in [0, 1_000_000_000, 2_000_000_000] {
        let at = profile.call(4, || MonotonicNs::new(raw_at))?;
        let row = profile.call(2, || FixtureObservation::new(context, at, phase, 1.0))??;
        profile.call(1, || history.try_append(row))??;
    }
    Ok(history)
}

/// Project and enclose one real retained zero-phase anchor without a raw factory.
/// # Errors
/// Preserves actual terms/projection/enclosure and accounting errors.
fn projected(
    profile: &Profile,
    history: &FixtureHistory,
    context: FixtureContext,
    at: MonotonicNs,
    distance: f64,
) -> TestResult<FixtureProjectedInterval> {
    let anchor_at = profile.call(4, || MonotonicNs::new(0))?;
    let terms = profile.call(8, || FixtureProjectionTerms::new(distance, 0.0, 0.0))??;
    let projection = profile.call(9, || {
        project_fixture_anchor(history, context, 0, anchor_at, at, terms)
    })??;
    Ok(profile.call(10, || enclose_fixture_projection(&projection))??)
}

/// Immutable tags and metadata shared by this single two-source scenario.
struct Inputs {
    /// Fixed ordered asserted labels; input wrapper order is the same here.
    labels: [NonZeroU64; 2],
    /// Exact frame assertion shared by genuine producer histories.
    frame: NonZeroU64,
    /// Exact mapping-generation assertion, not a peer generation.
    generation: NonZeroU64,
    /// Actual same-context history labels.
    contexts: [FixtureContext; 2],
    /// Checked per-source stratum assertions.
    strata: [FixturePrincipalStratum; 2],
    /// Principal A's unchanged actual latest-observation time.
    initial_at: MonotonicNs,
}

/// Independent literal expected view of one completed state.
#[derive(Clone, Copy)]
struct StateFact {
    /// Optional principal, with no actual product-derived expected value.
    principal: Option<NonZeroU64>,
    /// Supplied exact evaluation time.
    at: MonotonicNs,
    /// Latest genuine accepted observation per source.
    latest: [MonotonicNs; 2],
    /// Original supplied projected distances.
    distances: [f64; 2],
    /// Exact dyadic scores after the declared transition/reset.
    scores: [f64; 2],
    /// Unconsumed fixture credit, distinct from the pair-scoring event.
    pending: [bool; 2],
}

/// Inspect every relevant source field through actual public state accessors.
/// # Errors
/// Rejects an absent expected source or accounting exhaustion.
/// # Panics
/// The actual state must match the independent exact observation and score facts.
fn assert_state(
    profile: &Profile,
    state: &FixturePrincipalState,
    labels: &[NonZeroU64; 2],
    fact: StateFact,
) -> TestResult {
    assert_eq!(profile.getter(|| state.principal())?, fact.principal);
    assert_eq!(profile.getter(|| state.evaluated_at())?, fact.at);
    for (index, &label) in labels.iter().enumerate() {
        let source = profile
            .getter(|| state.source_state(label))?
            .ok_or_else(|| io::Error::other("missing principal source state"))?;
        assert_eq!(profile.getter(|| source.source_id())?, label);
        assert_eq!(
            profile.getter(|| source.score())?.to_bits(),
            fact.scores[index].to_bits()
        );
        assert_eq!(
            profile.getter(|| source.latest_observed_at())?,
            Some(fact.latest[index])
        );
        assert_eq!(profile.getter(|| source.pending())?, fact.pending[index]);
        assert_eq!(
            profile
                .getter(|| source.distance_seconds())?
                .map(f64::to_bits),
            Some(fact.distances[index].to_bits())
        );
        assert_eq!(profile.getter(|| source.stratum())?, Some(1));
    }
    Ok(())
}

/// Independent expected decision, without an algorithm-computed oracle.
#[derive(Clone, Copy)]
struct DecisionFact {
    /// Expected current principal label.
    principal: NonZeroU64,
    /// Closed fixture emission/retention outcome.
    kind: FixturePrincipalOutcomeKind,
    /// Exactly one genuine new source, or none for reevaluation.
    updated: Option<NonZeroU64>,
    /// Whether this transition consumes a principal's pending fixture credit.
    ready: bool,
}

/// Check the real selector reason and exact original-entry principal association.
/// # Errors
/// Rejects missing principal data or exhausted accounting.
/// # Panics
/// Decision fields and borrowed wrapper association must match literal facts.
fn assert_decision(
    profile: &Profile,
    value: &FixturePrincipalDecision<'_>,
    labels: &[NonZeroU64; 2],
    fact: DecisionFact,
) -> TestResult {
    assert_eq!(profile.getter(|| value.kind())?, fact.kind);
    assert_eq!(
        profile.getter(|| value.principal_label())?,
        Some(fact.principal)
    );
    assert_eq!(profile.getter(|| value.updated_source())?, fact.updated);
    assert_eq!(profile.getter(|| value.reference_ready())?, fact.ready);
    let selection = profile.getter(|| value.selection())?;
    assert_eq!(
        profile.getter(|| selection.outcome())?,
        FixtureOutcomeKind::WitnessedFixture
    );
    let entries = profile.getter(|| value.entries())?;
    assert_eq!(entries.len(), 2);
    let principal = profile
        .getter(|| value.principal_entry())?
        .ok_or_else(|| io::Error::other("missing original principal entry"))?;
    let expected_index = usize::from(fact.principal != labels[0]);
    assert!(std::ptr::eq(principal, &raw const entries[expected_index]));
    let interval = profile.getter(|| principal.interval())?;
    assert_eq!(profile.getter(|| interval.label())?, fact.principal);
    Ok(())
}

/// Independent expected facts for a single genuine-event or reevaluation call.
#[derive(Clone, Copy)]
struct EvaluationFact {
    /// Supplied common evaluation instant.
    at: MonotonicNs,
    /// Challenger's latest actual observation time, unchanged by reprojection.
    latest_b: MonotonicNs,
    /// Challenger score after the transition, including score reset on selection.
    score_b: f64,
    /// Declared current principal.
    principal: NonZeroU64,
    /// Declared outcome of the inert transition.
    kind: FixturePrincipalOutcomeKind,
    /// One genuine changed observation, or none.
    updated: Option<NonZeroU64>,
    /// Principal credit emission and challenger credit retention facts.
    credits: [bool; 2],
}

/// Reach genuine producers, checked batch admission and the actual state receiver.
/// # Errors
/// Preserves all actual product/observer failures without retries or substitution.
fn advance_and_assert(
    profile: &Profile,
    state: &mut FixturePrincipalState,
    inputs: &Inputs,
    histories: &[FixtureHistory; 2],
    fact: EvaluationFact,
) -> TestResult {
    let entries = [
        projected(profile, &histories[0], inputs.contexts[0], fact.at, 1.0)?,
        projected(profile, &histories[1], inputs.contexts[1], fact.at, 0.25)?,
    ];
    let batch = profile.call(11, || {
        FixtureProjectedBatch::new(inputs.frame, inputs.generation, fact.at, &entries)
    })??;
    let decision = profile.call(16, || state.advance(&batch, &inputs.strata))??;
    assert_decision(
        profile,
        &decision,
        &inputs.labels,
        DecisionFact {
            principal: fact.principal,
            kind: fact.kind,
            updated: fact.updated,
            ready: fact.credits[0],
        },
    )?;
    assert_state(
        profile,
        state,
        &inputs.labels,
        StateFact {
            principal: Some(fact.principal),
            at: fact.at,
            latest: [inputs.initial_at, fact.latest_b],
            distances: [1.0, 0.25],
            scores: [1.0, fact.score_b],
            pending: [false, fact.credits[1]],
        },
    )
}

/// Immutable owners and mutable histories/state of the one accepted scenario.
struct Scenario {
    /// Checked fixed fixture labels and metadata.
    inputs: Inputs,
    /// Real history owners, with no forged reports or projections.
    histories: [FixtureHistory; 2],
    /// Actual consumer state returned by its sole seed path.
    state: FixturePrincipalState,
    /// Genuine checked zero phase reused without reconstructing raw values.
    phase: PhaseOffsetQ33,
}

/// Check seed metadata and pending credit before the first selection call.
/// # Errors
/// Propagates observer accounting or missing-source failure.
/// # Panics
/// The seeded state must preserve exact metadata and leave selection unperformed.
fn assert_seed(profile: &Profile, state: &FixturePrincipalState, inputs: &Inputs) -> TestResult {
    assert_eq!(profile.getter(|| state.frame())?, inputs.frame);
    assert_eq!(
        profile.getter(|| state.mapping_generation())?,
        inputs.generation
    );
    assert_eq!(profile.getter(|| state.roster_len())?, 2);
    let stored_minimum = profile.getter(|| state.minimum())?;
    assert_eq!(profile.getter(|| stored_minimum.get())?, 2);
    let stored_parameters = profile.getter(|| state.parameters())?;
    assert_eq!(
        profile
            .getter(|| stored_parameters.reselect_distance_seconds())?
            .to_bits(),
        0.25_f64.to_bits()
    );
    assert_eq!(
        profile
            .getter(|| stored_parameters.stratum_weight_seconds())?
            .to_bits(),
        0.0_f64.to_bits()
    );
    assert_state(
        profile,
        state,
        &inputs.labels,
        StateFact {
            principal: None,
            at: inputs.initial_at,
            latest: [inputs.initial_at; 2],
            distances: [1.0, 2.0],
            scores: [1.0; 2],
            pending: [true; 2],
        },
    )
}

/// Admit genuine seed data and select once using its unconsumed fixture credit.
/// # Errors
/// Preserves actual product admission and observer failures.
/// # Panics
/// Healthy seed admission and the first principal decision must match exact facts.
fn seed_and_select(profile: &Profile) -> TestResult<Scenario> {
    let labels = [tag(101)?, tag(202)?];
    let frame = tag(7)?;
    let generation = tag(11)?;
    let contexts = [
        profile.call(3, || FixtureContext::new(labels[0], frame, generation))?,
        profile.call(3, || FixtureContext::new(labels[1], frame, generation))?,
    ];
    let phase = zero_phase(profile)?;
    let histories = [
        flat_history(profile, contexts[0], phase)?,
        flat_history(profile, contexts[1], phase)?,
    ];
    let initial_at = profile.call(4, || MonotonicNs::new(2_000_000_000))?;
    let entries = [
        projected(profile, &histories[0], contexts[0], initial_at, 1.0)?,
        projected(profile, &histories[1], contexts[1], initial_at, 2.0)?,
    ];
    let batch = profile.call(11, || {
        FixtureProjectedBatch::new(frame, generation, initial_at, &entries)
    })??;
    let minimum = profile.call(12, || FixtureMinimum::new(2))??;
    let parameters = profile.call(13, || FixturePrincipalParameters::new(0.25, 0.0))??;
    let strata = [
        profile.call(14, || FixturePrincipalStratum::new(labels[0], 1))??,
        profile.call(14, || FixturePrincipalStratum::new(labels[1], 1))??,
    ];
    let seeded = profile.call(15, || {
        FixturePrincipalState::seed(&labels, parameters, minimum, &batch, &strata)
    })?;
    assert!(
        seeded.is_ok(),
        "genuine same-context principal seed must be admitted"
    );
    let mut state = seeded?;
    let inputs = Inputs {
        labels,
        frame,
        generation,
        contexts,
        strata,
        initial_at,
    };
    assert_seed(profile, &state, &inputs)?;
    let decision = profile.call(16, || state.advance(&batch, &inputs.strata))??;
    assert_decision(
        profile,
        &decision,
        &labels,
        DecisionFact {
            principal: labels[0],
            kind: FixturePrincipalOutcomeKind::ReferenceReady,
            updated: None,
            ready: true,
        },
    )?;
    assert_state(
        profile,
        &state,
        &labels,
        StateFact {
            principal: Some(labels[0]),
            at: initial_at,
            latest: [initial_at; 2],
            distances: [1.0, 2.0],
            scores: [1.0; 2],
            pending: [false; 2],
        },
    )?;
    Ok(Scenario {
        inputs,
        histories,
        state,
        phase,
    })
}

/// Distinguish four genuine updates from repeated projection of old observations.
///
/// Against the original seed scaffold, the healthy-seed assertion was expected
/// to fail; subsequent assertions were initially unreached until it existed.
/// Final switch observes score reset1; pre-switch16 is independently derived,
/// not exposed by this API and not claimed as an observed score.
/// # Errors
/// Preserve actual producer, principal-receiver and observer-budget failures.
/// # Panics
/// Seed admission, exact state/decision facts and the completed recipe must match.
#[test]
fn principal_seed_and_genuine_update_hysteresis() -> TestResult {
    let profile = Profile::new();
    let Scenario {
        inputs,
        mut histories,
        mut state,
        phase,
    } = seed_and_select(&profile)?;
    let labels = inputs.labels;
    let contexts = inputs.contexts;
    for (raw_event, raw_reevaluation, expected_score, switched) in [
        (3_000_000_000, 3_500_000_000, 2.0, false),
        (4_000_000_000, 4_500_000_000, 4.0, false),
        (5_000_000_000, 5_500_000_000, 8.0, false),
        (6_000_000_000, 6_500_000_000, 1.0, true),
    ] {
        let event_at = profile.call(4, || MonotonicNs::new(raw_event))?;
        let row = profile.call(2, || {
            FixtureObservation::new(contexts[1], event_at, phase, 1.0)
        })??;
        profile.call(1, || histories[1].try_append(row))??;
        let principal = if switched { labels[1] } else { labels[0] };
        let kind = if switched {
            FixturePrincipalOutcomeKind::ReferenceReady
        } else {
            FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
        };
        advance_and_assert(
            &profile,
            &mut state,
            &inputs,
            &histories,
            EvaluationFact {
                at: event_at,
                latest_b: event_at,
                score_b: expected_score,
                principal,
                kind,
                updated: Some(labels[1]),
                credits: [switched, !switched],
            },
        )?;
        let reevaluation_at = profile.call(4, || MonotonicNs::new(raw_reevaluation))?;
        advance_and_assert(
            &profile,
            &mut state,
            &inputs,
            &histories,
            EvaluationFact {
                at: reevaluation_at,
                latest_b: event_at,
                score_b: expected_score,
                principal,
                kind: FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate,
                updated: None,
                credits: [false, !switched],
            },
        )?;
    }
    profile.assert_complete();
    Ok(())
}

/// Additional receiver cases with separate, bounded actual-attempt profiles.
#[path = "support/principal_cases.rs"]
mod principal_cases;

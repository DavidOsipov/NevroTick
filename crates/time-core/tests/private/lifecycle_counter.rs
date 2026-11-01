// SPDX-License-Identifier: GPL-2.0-only

//! Direct evidence for increment, source admission and raw output postconditions.
//! Checked source, decision, witness and estimate values are never forged.
//! Uncommitted raw output candidates intentionally violate relationships before
//! publication. Four increment calls claim no practical public overflow journey.
//! Twenty-four literal raw source tuples independently challenge each receiving
//! owner across u8/u8/u64 domains, including MAX ordinals and valid neighbors.

use std::{
    error::Error,
    io::{self, Write},
    num::NonZeroU64,
};

use garde::Validate;

use super::{
    FixtureCombinationLifecycleError, FixtureCombinationParameters, FixtureCombinationSourceState,
    FixtureCombinationSourceStatus, FixtureCombinationState, Inner, OutcomeInput, RawOutcome,
    RawSourceFields, SourceOutcome, checked_next, estimate_relation_valid, outcome_valid,
    source_fields_valid,
};

/// Retain distinct precharged attempts when an independent assertion unwinds.
/// Constructor-internal validation is source work owned by `source_constructions`,
/// not another direct predicate or direct Garde observation.
struct AttemptedPrefix {
    /// Calls to the actual raw checked increment helper only.
    attempts: u64,
    /// Calls to the fallible private source-state constructor.
    source_constructions: u64,
    /// Direct calls to the raw tuple predicate, outside constructor/callback work.
    source_predicates: u64,
    /// Direct raw-payload Garde validations, outside private construction.
    garde: u64,
    /// One entered fixed genuine-producer setup; its source upper is separate.
    fixture_setups: u64,
    /// Direct raw output relationship predicate calls.
    outcome_predicates: u64,
    /// Direct estimate relationship calls, including arbitrary raw counts.
    estimate_predicates: u64,
    /// Direct raw output Garde validations, distinct from predicate observations.
    outcome_garde: u64,
}

impl Drop for AttemptedPrefix {
    /// Report the precharged prefix without introducing a teardown panic.
    fn drop(&mut self) {
        let _emission = writeln!(
            io::stderr().lock(),
            "CORE_LIFECYCLE_COUNTER_PROFILE case=lifecycle_counter_success_and_exhaustion attempts={} source_constructions={} source_predicates={} garde={} fixture_setups={} outcome_predicates={} estimate_predicates={} outcome_garde={}",
            self.attempts,
            self.source_constructions,
            self.source_predicates,
            self.garde,
            self.fixture_setups,
            self.outcome_predicates,
            self.estimate_predicates,
            self.outcome_garde
        );
    }
}

/// Independently specified raw-domain cases, not values forged into checked state.
///
/// The complete 24-case scope performs 24 direct predicates, 24 direct Garde
/// validations and 24 constructor calls. Including constructor internals yields
/// at most 48 Garde validations/callbacks and 96 source predicates. Ten accepted
/// constructions additionally check their four privately retained fields.
/// # Errors
/// Propagate an unexpected rejection of a valid tuple or fixture-label failure.
/// # Panics
/// Range, saturation and ordinal relationship facts agree at all three owners.
fn source_counter_neighbors(
    attempted: &mut AttemptedPrefix,
) -> Result<(), FixtureCombinationLifecycleError> {
    let label = NonZeroU64::new(202).ok_or(FixtureCombinationLifecycleError::Invariant)?;
    for (raw, expected) in [
        ((0, 0, 0), true),
        ((0, 32, 0), true),
        ((1, 0, 1), true),
        ((1, 32, 1), true),
        ((7, 31, 7), true),
        ((8, 31, 8), true),
        ((8, 32, 8), true),
        ((8, 32, 9), true),
        ((8, 0, u64::MAX - 1), true),
        ((8, 32, u64::MAX), true),
        ((0, 33, 0), false),
        ((8, 33, 8), false),
        ((8, u8::MAX, u64::MAX), false),
        ((9, 0, 9), false),
        ((u8::MAX, 32, u64::MAX), false),
        ((0, 0, 1), false),
        ((1, 0, 0), false),
        ((1, 32, 2), false),
        ((7, 32, 6), false),
        ((7, 32, 8), false),
        ((8, 0, 7), false),
        ((0, 32, u64::MAX), false),
        ((7, 0, u64::MAX), false),
        ((9, 33, 9), false),
    ] {
        attempted.source_predicates += 1;
        assert_eq!(source_fields_valid(raw), expected);
        attempted.garde += 1;
        assert_eq!(
            RawSourceFields { counters: raw }.validate().is_ok(),
            expected
        );
        attempted.source_constructions += 1;
        let constructed = FixtureCombinationSourceState::new(label, raw.0, raw.1, raw.2);
        if expected {
            let admitted = constructed?;
            assert_eq!(
                (
                    admitted.source_id,
                    admitted.reachability_updates,
                    admitted.penalty,
                    admitted.last_reachability_ordinal
                ),
                (label, raw.0, raw.1, raw.2)
            );
        } else {
            assert!(matches!(
                constructed,
                Err(FixtureCombinationLifecycleError::Invariant)
            ));
        }
    }
    Ok(())
}

/// Check exact increment neighbors and actual raw source-field admission.
/// # Errors
/// Propagate an unexpected rejection of an admitted neighboring primitive/tuple.
/// # Panics
/// Four increment boundaries, range and relationship controls must match.
#[test]
fn lifecycle_counter_success_and_exhaustion() -> TestResult {
    let mut attempted = AttemptedPrefix {
        attempts: 0,
        source_constructions: 0,
        source_predicates: 0,
        garde: 0,
        fixture_setups: 0,
        outcome_predicates: 0,
        estimate_predicates: 0,
        outcome_garde: 0,
    };
    (|| {
        attempted.attempts += 1;
        assert_eq!(checked_next(0)?.get(), 1);
        attempted.attempts += 1;
        assert_eq!(checked_next(7)?.get(), 8);
        attempted.attempts += 1;
        assert_eq!(checked_next(u64::MAX - 1)?.get(), u64::MAX);
        attempted.attempts += 1;
        assert_eq!(
            checked_next(u64::MAX),
            Err(FixtureCombinationLifecycleError::CounterExhausted)
        );
        source_counter_neighbors(&mut attempted)?;
        outcome_guard_matrix(&mut attempted)?;
        assert_eq!(
            (
                attempted.attempts,
                attempted.source_constructions,
                attempted.source_predicates,
                attempted.garde,
                attempted.fixture_setups,
                attempted.outcome_predicates,
                attempted.estimate_predicates,
                attempted.outcome_garde
            ),
            (4, 24, 24, 24, 1, 34, 18, 34)
        );
        Ok(())
    })()
}

// Raw postconditions are tested before any published outcome/state construction.
// Every Decision, Witness, projection, estimate and per-source value comes from
// its real checked owner. Three deliberately inconsistent uncommitted Inner
// candidates assemble those genuine components; none is a published state.
//
// Fixed setup source reservation (not observed fit counters): four three-row
// histories, twelve appends, six projections/enclosures/batches, eleven actual
// lifecycle seeds and twelve advances. It reserves twelve fits/768 points.
// Three explicit raw Inner clones add three principal clones/source-array copies.
// The direct raw probes add 34 outcome predicates, 18 estimate predicates and
// 34 raw-output Garde validations. Each complete predicate scans at most sixteen
// slots; at most 86 estimate scans and 1,088 source-slot visits are reserved for
// the raw probes (68 outcome scans plus eighteen direct estimate scans). Their product
// accessor work has a separate 8,192-per-predicate upper, not fake observations.
// The fixture's direct producer/accessor envelope is supplied to the coordinator
// as a closed twenty-field producer vector and a closed twenty-two-field lifecycle vector.

use crate::{
    FixtureContext, FixtureHistory, FixtureMinimum, FixtureObservation, FixturePrincipalDecision,
    FixturePrincipalOutcomeKind, FixturePrincipalParameters, FixturePrincipalStratum,
    FixtureProjectedBatch, FixtureProjectedInterval, FixtureProjectionTerms,
    FixtureSupporterUncertainty, enclose_fixture_projection, project_fixture_anchor,
};
use time_domain::{MonotonicNs, UtcCoordinateQ32, exchange_arithmetic};

/// Harness errors remain distinguishable from actual domain rejections.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;

/// Precharge a separately bounded direct probe before calling its real receiver.
/// # Errors
/// Reject counter overflow or exhaustion before any product entry.
fn guard_charge(value: &mut u64, maximum: u64) -> TestResult {
    let next = value
        .checked_add(1)
        .ok_or_else(|| io::Error::other("raw guard counter overflow"))?;
    if next > maximum {
        return Err(io::Error::other("raw guard counter exhausted").into());
    }
    *value = next;
    Ok(())
}

/// Admit a literal fixture tag; a checked product identity is never manufactured.
/// # Errors
/// Reject an accidental zero test tag.
fn guard_tag(raw: u64) -> TestResult<NonZeroU64> {
    Ok(NonZeroU64::new(raw).ok_or_else(|| io::Error::other("zero raw guard tag"))?)
}

/// Build one real flat three-row history in its independently asserted context.
/// # Errors
/// Preserve actual observation, append and numerical admission failures.
fn guard_history(source: u64, frame: u64, generation: u64) -> TestResult<FixtureHistory> {
    let context = FixtureContext::new(
        guard_tag(source)?,
        guard_tag(frame)?,
        guard_tag(generation)?,
    );
    let mut history = FixtureHistory::new(context);
    let zero = UtcCoordinateQ32::new(0)?;
    let phase = exchange_arithmetic(zero, zero, zero, zero)?.offset();
    for raw in [0, 1_000_000_000, 2_000_000_000] {
        history.try_append(FixtureObservation::new(
            context,
            MonotonicNs::new(raw),
            phase,
            1.0,
        )?)?;
    }
    Ok(history)
}

/// Obtain a genuine alternate anchor/context/evaluation projection.
/// # Errors
/// Preserve the actual same-history and enclosure owners' rejection causes.
fn guard_projection(
    history: &FixtureHistory,
    anchor: usize,
    expected_at: u64,
    evaluated_at: u64,
) -> TestResult<FixtureProjectedInterval> {
    let projection = project_fixture_anchor(
        history,
        history.context(),
        anchor,
        MonotonicNs::new(expected_at),
        MonotonicNs::new(evaluated_at),
        FixtureProjectionTerms::new(0.125, 0.0, 0.0)?,
    )?;
    Ok(enclose_fixture_projection(&projection)?)
}

/// Borrow one actual entry slice through the real contextual batch constructor.
/// # Errors
/// Reject an empty test slice or any actual batch admission error.
fn guard_batch(entries: &[FixtureProjectedInterval]) -> TestResult<FixtureProjectedBatch<'_>> {
    let first = entries
        .first()
        .ok_or_else(|| io::Error::other("missing raw guard entry"))?;
    let projection = first.projection();
    Ok(FixtureProjectedBatch::new(
        projection.context().frame(),
        projection.context().generation(),
        projection.projected_at(),
        entries,
    )?)
}

/// Genuine output components retained separately for raw receiving probes.
struct GuardCase<'batch> {
    /// Real same-batch checked decision; never edited by the tests.
    principal: FixturePrincipalDecision<'batch>,
    /// Real checked estimate, or its actual non-ready absence.
    combined: Option<super::FixtureCombinedEstimate>,
    /// Actual public emission identity.
    round: Option<NonZeroU64>,
    /// Output metadata, copied only into explicitly unvalidated candidates.
    statuses: [Option<SourceOutcome>; super::MAX_SUPPORTERS],
    /// Real post-transition transaction components, separate from published state.
    inner: Inner,
    /// Actual entry-state emission watermark.
    previous_rounds: u64,
}

impl<'batch> GuardCase<'batch> {
    /// Borrow genuine components as an unvalidated relationship candidate.
    fn input(&self) -> OutcomeInput<'_, 'batch> {
        OutcomeInput {
            principal: &self.principal,
            combined: self.combined.as_ref(),
            round: self.round,
            statuses: &self.statuses,
            inner: &self.inner,
            previous_rounds: self.previous_rounds,
        }
    }
}

/// Preserve a discriminating positive-fixture failure before raw probes begin.
/// # Errors
/// Preserve conversion of the typed operation result after its success check.
/// # Panics
/// A healthy fixture must succeed; retain its bounded code and delegated cause.
fn require_guard_success<T>(result: Result<T, FixtureCombinationLifecycleError>) -> TestResult<T> {
    assert!(
        result.is_ok(),
        "healthy raw guard fixture rejected: {}",
        result
            .as_ref()
            .err()
            .map_or_else(String::new, |error| format!(
                "{error}; cause={}",
                error.source().map_or_else(String::new, ToString::to_string)
            ))
    );
    Ok(result?)
}

/// Build a genuine owner/output with independently specified ties and limits.
/// # Errors
/// Preserve real parameter, metadata, seed and transition failures.
/// # Panics
/// Flat equal-source ties select the first roster label; replay creates no round.
fn guard_case<'batch>(
    batch: &'batch FixtureProjectedBatch<'_>,
    roster: &[NonZeroU64],
    minimum: usize,
    combine_limit: f64,
    retained: bool,
) -> TestResult<GuardCase<'batch>> {
    let strata = roster
        .iter()
        .map(|label| FixturePrincipalStratum::new(*label, 1))
        .collect::<Result<Vec<_>, _>>()?;
    let mut state = FixtureCombinationState::seed(
        roster,
        FixturePrincipalParameters::new(0.25, 0.0)?,
        FixtureCombinationParameters::new(combine_limit, 0.031_25)?,
        FixtureMinimum::new(minimum)?,
        batch,
        &strata,
    )?;
    let records = batch
        .entries()
        .iter()
        .map(|entry| FixtureSupporterUncertainty::new(entry, 1.0, 1.0, 0.015_625))
        .collect::<Result<Vec<_>, _>>()?;
    if retained {
        let _first = require_guard_success(state.advance(batch, &strata, &records))?;
    }
    let previous_rounds = state.emitted_rounds();
    let outcome = require_guard_success(state.advance(batch, &strata, &records))?;
    let expected = if retained {
        FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    } else {
        FixturePrincipalOutcomeKind::ReferenceReady
    };
    assert_eq!(outcome.principal.kind(), expected);
    assert_eq!(outcome.principal.principal_label(), roster.first().copied());
    Ok(GuardCase {
        principal: outcome.principal,
        combined: outcome.combined,
        round: outcome.emitted_round,
        statuses: outcome.sources,
        inner: state.inner,
        previous_rounds,
    })
}

/// Challenge one independently expected raw relationship at both actual owners.
/// # Errors
/// Preserve independent probe precharge failures.
/// # Panics
/// Predicate and mandatory raw Garde must match the literal expected result.
fn guard_outcome(
    attempted: &mut AttemptedPrefix,
    input: OutcomeInput<'_, '_>,
    expected: bool,
) -> TestResult {
    guard_charge(&mut attempted.outcome_predicates, 34)?;
    assert_eq!(outcome_valid(&input), expected);
    guard_charge(&mut attempted.outcome_garde, 34)?;
    assert_eq!(RawOutcome { value: input }.validate().is_ok(), expected);
    Ok(())
}

/// Keep a healthy actual receiver neighbor beside each malformed relationship.
/// # Errors
/// Preserve product-probe accounting errors.
/// # Panics
/// The healthy candidate is admitted and its independently perturbed neighbor rejects.
fn guard_outcome_pair(
    attempted: &mut AttemptedPrefix,
    good: OutcomeInput<'_, '_>,
    bad: OutcomeInput<'_, '_>,
) -> TestResult {
    guard_outcome(attempted, good, true)?;
    guard_outcome(attempted, bad, false)
}

/// Check raw estimate counts/provenance without deriving truth from output accessors.
/// # Errors
/// Preserve the separate direct-estimate precharge bound.
/// # Panics
/// The actual helper must match each literal cardinality/provenance fact.
fn guard_estimate(
    attempted: &mut AttemptedPrefix,
    input: &OutcomeInput<'_, '_>,
    contributors: usize,
    principals: usize,
    expected: bool,
) -> TestResult {
    guard_charge(&mut attempted.estimate_predicates, 18)?;
    assert_eq!(
        estimate_relation_valid(input, contributors, principals),
        expected
    );
    Ok(())
}

/// Related genuine owners used only to assemble unvalidated output relationships.
struct GuardCases<'case, 'batch> {
    /// Healthy two-source owner selecting101.
    a: &'case GuardCase<'batch>,
    /// Healthy two-source owner selecting202 by the opposite stable roster tie.
    b: &'case GuardCase<'batch>,
    /// Healthy same-principal owner with independently configured minimum two.
    minimum_two: &'case GuardCase<'batch>,
    /// Healthy original singleton with a genuine one-source witness.
    singleton: &'case GuardCase<'batch>,
    /// Actual replay result with no estimate/round.
    retained: &'case GuardCase<'batch>,
    /// Real principal101 transition that distance-excludes202 with penalty one.
    penalty_a: &'case GuardCase<'batch>,
    /// Real principal202 transition that distance-excludes101 with penalty one.
    penalty_b: &'case GuardCase<'batch>,
}

/// Vary raw roster association, minimum and round metadata independently.
///
/// The raw count case intentionally assembles an uncommitted `Inner` candidate with a
/// genuine two-source principal owner but a one-source raw output prefix. It is
/// rejected before constructing any published `FixtureCombinationState`/outcome.
/// No `Decision`, `Witness` or checked source/estimate is fabricated or edited.
/// # Errors
/// Preserve raw-probe precharges.
/// # Panics
/// Each independent malformed relationship rejects beside an admitted neighbor.
fn guard_identity_neighbors(
    attempted: &mut AttemptedPrefix,
    cases: &GuardCases<'_, '_>,
) -> TestResult {
    let mut short = cases.a.inner.clone();
    short.len = 1;
    short.sources[1] = None;
    guard_outcome_pair(
        attempted,
        cases.singleton.input(),
        OutcomeInput {
            inner: &short,
            ..cases.singleton.input()
        },
    )?;
    let mut labels = cases.b.statuses;
    labels[0]
        .as_mut()
        .ok_or_else(|| io::Error::other("missing B status"))?
        .status = FixtureCombinationSourceStatus::Contributor;
    labels[1]
        .as_mut()
        .ok_or_else(|| io::Error::other("missing A status"))?
        .status = FixtureCombinationSourceStatus::Principal;
    guard_outcome_pair(
        attempted,
        cases.a.input(),
        OutcomeInput {
            inner: &cases.b.inner,
            statuses: &labels,
            ..cases.a.input()
        },
    )?;
    guard_outcome_pair(
        attempted,
        cases.a.input(),
        OutcomeInput {
            inner: &cases.minimum_two.inner,
            ..cases.a.input()
        },
    )?;
    guard_outcome_pair(
        attempted,
        cases.a.input(),
        OutcomeInput {
            round: None,
            previous_rounds: 1,
            ..cases.a.input()
        },
    )?;
    guard_outcome_pair(
        attempted,
        cases.a.input(),
        OutcomeInput {
            previous_rounds: 1,
            ..cases.a.input()
        },
    )
}

/// Reject padding and label changes without touching genuine source values.
/// # Errors
/// Preserve literal-tag and raw-probe failures.
/// # Panics
/// A phantom padded status and a mismatched occupied label reject independently.
fn guard_storage_neighbors(attempted: &mut AttemptedPrefix, case: &GuardCase<'_>) -> TestResult {
    let mut padded = case.statuses;
    padded[2] = Some(SourceOutcome {
        source_id: guard_tag(303)?,
        status: FixtureCombinationSourceStatus::NotEvaluated,
    });
    guard_outcome_pair(
        attempted,
        case.input(),
        OutcomeInput {
            statuses: &padded,
            ..case.input()
        },
    )?;
    let mut mislabeled = case.statuses;
    mislabeled[1]
        .as_mut()
        .ok_or_else(|| io::Error::other("missing contributor status"))?
        .source_id = guard_tag(303)?;
    guard_outcome_pair(
        attempted,
        case.input(),
        OutcomeInput {
            statuses: &mislabeled,
            ..case.input()
        },
    )
}

/// Reject incorrect roles and penalties using only genuine copied source values.
/// # Errors
/// Preserve missing fixture components and bounded probe accounting failures.
/// # Panics
/// Raw roles, contributor penalties and absent ready estimates must reject.
fn guard_status_neighbors(
    attempted: &mut AttemptedPrefix,
    cases: &GuardCases<'_, '_>,
) -> TestResult {
    for status in [
        FixtureCombinationSourceStatus::OutsideWitness,
        FixtureCombinationSourceStatus::NotEvaluated,
    ] {
        let mut statuses = cases.penalty_a.statuses;
        statuses[1]
            .as_mut()
            .ok_or_else(|| io::Error::other("missing penalized status"))?
            .status = status;
        guard_outcome_pair(
            attempted,
            cases.penalty_a.input(),
            OutcomeInput {
                statuses: &statuses,
                ..cases.penalty_a.input()
            },
        )?;
    }
    let mut swapped = cases.a.statuses;
    swapped[0]
        .as_mut()
        .ok_or_else(|| io::Error::other("missing A role"))?
        .status = FixtureCombinationSourceStatus::Contributor;
    swapped[1]
        .as_mut()
        .ok_or_else(|| io::Error::other("missing B role"))?
        .status = FixtureCombinationSourceStatus::Principal;
    guard_outcome_pair(
        attempted,
        cases.a.input(),
        OutcomeInput {
            statuses: &swapped,
            ..cases.a.input()
        },
    )?;
    let mut penalty = cases.a.inner.clone();
    penalty.sources[1] = cases.penalty_a.inner.sources[1];
    guard_outcome_pair(
        attempted,
        cases.a.input(),
        OutcomeInput {
            inner: &penalty,
            ..cases.a.input()
        },
    )?;
    let mut staged = cases.a.inner.clone();
    staged.sources[0] = cases.penalty_b.inner.sources[1];
    staged.sources[1] = cases.penalty_a.inner.sources[1];
    let mut unevaluated = cases.a.statuses;
    for status in unevaluated.iter_mut().take(2).flatten() {
        status.status = FixtureCombinationSourceStatus::PenaltyCountdown;
    }
    guard_outcome_pair(
        attempted,
        cases.a.input(),
        OutcomeInput {
            combined: None,
            inner: &staged,
            statuses: &unevaluated,
            ..cases.a.input()
        },
    )
}

/// Challenge the helper's full raw usize cardinality domain beside healthy counts.
/// # Errors
/// Preserve the separate estimate probe precharge bound.
/// # Panics
/// A positive wrong count, absent principal or nonzero absent-estimate count rejects.
fn guard_count_neighbors(
    attempted: &mut AttemptedPrefix,
    cases: &GuardCases<'_, '_>,
) -> TestResult {
    let ready = cases.a.input();
    for (contributors, principals) in [(2, 0), (3, 1)] {
        guard_estimate(attempted, &ready, 2, 1, true)?;
        guard_estimate(attempted, &ready, contributors, principals, false)?;
    }
    let retained = cases.retained.input();
    for (contributors, principals) in [(1, 0), (0, 1)] {
        guard_estimate(attempted, &retained, 0, 0, true)?;
        guard_estimate(attempted, &retained, contributors, principals, false)?;
    }
    Ok(())
}

/// Pair independently genuine estimates/decisions across one provenance change.
///
/// Count/identity facts are authored from fixture inputs. No checked estimate is
/// edited to separate its coupled latest-observation and fit-reference fields;
/// the line 1031 equivalence question remains separate, with no claimed mutant kill.
/// # Errors
/// Preserve finite raw-receiver and estimate precharges.
/// # Panics
/// Correct provenance admits; source/frame/generation/anchor/evaluation swaps reject.
fn guard_provenance_neighbors(
    attempted: &mut AttemptedPrefix,
    pairs: &[(&GuardCase<'_>, &GuardCase<'_>, usize); 5],
) -> TestResult {
    for &(target, source, contributors) in pairs {
        let good = target.input();
        let bad = OutcomeInput {
            combined: source.combined.as_ref(),
            ..target.input()
        };
        guard_estimate(attempted, &good, contributors, 1, true)?;
        guard_estimate(attempted, &bad, contributors, 1, false)?;
        guard_outcome_pair(attempted, good, bad)?;
    }
    Ok(())
}

/// Enter one finite genuine setup and every independent raw postcondition probe.
///
/// Public sixteen/seventeen capacity trajectories live in the integration suite;
/// this setup never duplicates their sixteen-history numerical work. Three raw
/// `Inner` clones are intentionally unpublished relationship candidates. Genuine
/// published states, decisions, witnesses and estimates are never manufactured.
/// # Errors
/// Preserve all real producer and raw-probe admission failures.
/// # Panics
/// Independently specified principal, penalty and raw postcondition facts hold.
fn outcome_guard_matrix(attempted: &mut AttemptedPrefix) -> TestResult {
    guard_charge(&mut attempted.fixture_setups, 1)?;
    let history_a = guard_history(101, 7, 11)?;
    let history_b = guard_history(202, 7, 11)?;
    let history_frame = guard_history(101, 17, 11)?;
    let history_generation = guard_history(101, 7, 19)?;
    let a = guard_projection(&history_a, 0, 0, 2_000_000_000)?;
    let b = guard_projection(&history_b, 0, 0, 2_000_000_000)?;
    let frame = guard_projection(&history_frame, 0, 0, 2_000_000_000)?;
    let generation = guard_projection(&history_generation, 0, 0, 2_000_000_000)?;
    let anchor = guard_projection(&history_a, 1, 1_000_000_000, 2_000_000_000)?;
    let at = guard_projection(&history_a, 0, 0, 3_000_000_000)?;
    let pair_entries = [a, b];
    let singleton_entries = [a];
    let frame_entries = [frame];
    let generation_entries = [generation];
    let anchor_entries = [anchor];
    let later_entries = [at];
    let pair_batch = guard_batch(&pair_entries)?;
    let single_batch = guard_batch(&singleton_entries)?;
    let frame_batch = guard_batch(&frame_entries)?;
    let generation_batch = guard_batch(&generation_entries)?;
    let anchor_batch = guard_batch(&anchor_entries)?;
    let later_batch = guard_batch(&later_entries)?;
    let a_label = guard_tag(101)?;
    let b_label = guard_tag(202)?;
    let ab = [a_label, b_label];
    let ba = [b_label, a_label];
    let a_roster = [a_label];
    let ready_a = guard_case(&pair_batch, &ab, 1, 2.0, false)?;
    let ready_b = guard_case(&pair_batch, &ba, 1, 2.0, false)?;
    let minimum_two = guard_case(&pair_batch, &ab, 2, 2.0, false)?;
    let singleton = guard_case(&single_batch, &a_roster, 1, 2.0, false)?;
    let frame_case = guard_case(&frame_batch, &a_roster, 1, 2.0, false)?;
    let generation_case = guard_case(&generation_batch, &a_roster, 1, 2.0, false)?;
    let anchor_case = guard_case(&anchor_batch, &a_roster, 1, 2.0, false)?;
    let at_case = guard_case(&later_batch, &a_roster, 1, 2.0, false)?;
    let retained = guard_case(&single_batch, &a_roster, 1, 2.0, true)?;
    let penalty_a = guard_case(&pair_batch, &ab, 1, 0.25, false)?;
    let penalty_b = guard_case(&pair_batch, &ba, 1, 0.25, false)?;
    assert_eq!(
        penalty_a.inner.sources[1]
            .ok_or_else(|| io::Error::other("missing penalized B"))?
            .penalty,
        1
    );
    assert_eq!(
        penalty_b.inner.sources[1]
            .ok_or_else(|| io::Error::other("missing penalized A"))?
            .penalty,
        1
    );
    let cases = GuardCases {
        a: &ready_a,
        b: &ready_b,
        minimum_two: &minimum_two,
        singleton: &singleton,
        retained: &retained,
        penalty_a: &penalty_a,
        penalty_b: &penalty_b,
    };
    guard_identity_neighbors(attempted, &cases)?;
    guard_storage_neighbors(attempted, &ready_a)?;
    guard_status_neighbors(attempted, &cases)?;
    guard_count_neighbors(attempted, &cases)?;
    guard_provenance_neighbors(
        attempted,
        &[
            (&ready_b, &ready_a, 2),
            (&frame_case, &singleton, 1),
            (&generation_case, &singleton, 1),
            (&anchor_case, &singleton, 1),
            (&at_case, &singleton, 1),
        ],
    )
}

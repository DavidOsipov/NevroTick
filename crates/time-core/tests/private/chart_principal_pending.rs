// SPDX-License-Identifier: GPL-2.0-only
//! S157 chart receiving must consume existing pending credit without a new event.
//!
//! One test has three genuine histories, ten total observations/appends, one
//! chart, six prepare/tracking projections, six chart and six raw enclosures,
//! two legacy batches, one chart batch, one seed, two legacy advances and two
//! chart advances. At most eighteen source-state snapshots inspect only three
//! fixed roster members. These are source-only bounds, not observed execution.
//! No checked product state, detached fit or transformed legacy wrapper is forged.

use super::{SECOND, Source, TestResult, chart, label, terms};
use std::{io, num::NonZeroU64, ptr};
use time_core::{
    FixtureAffineChart, FixtureChartBatch, FixtureChartInterval, FixtureChartOutcome,
    FixtureContext, FixtureMinimum, FixturePrincipalOutcomeKind, FixturePrincipalParameters,
    FixturePrincipalState, FixturePrincipalStratum, FixtureProjectedBatch,
    FixtureProjectedInterval, enclose_fixture_chart_projection, enclose_fixture_projection,
};
use time_domain::MonotonicNs;

/// Independent bit-exact observation, quality and credit facts for one source.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SourceFacts {
    /// Original observation watermark, never replaced by evaluation time.
    latest: Option<MonotonicNs>,
    /// Exact committed score bits, with no approximate comparison.
    score_bits: u64,
    /// Original root-distance bits, not expanded chart geometry.
    distance_bits: Option<u64>,
    /// Original checked stratum retained through absence.
    stratum: Option<u8>,
    /// Inert unconsumed observation credit.
    pending: bool,
}

/// Observe the complete public per-source state of exactly three roster members.
/// # Errors
/// Reject an absent registered member instead of supplying substitute metadata.
fn source_facts(
    state: &FixturePrincipalState,
    ids: [NonZeroU64; 3],
) -> TestResult<[SourceFacts; 3]> {
    let inspect = |id| {
        let source = state
            .source_state(id)
            .ok_or_else(|| io::Error::other("missing pending-credit source"))?;
        Ok::<_, Box<dyn std::error::Error>>(SourceFacts {
            latest: source.latest_observed_at(),
            score_bits: source.score().to_bits(),
            distance_bits: source.distance_seconds().map(f64::to_bits),
            stratum: source.stratum(),
            pending: source.pending(),
        })
    };
    Ok([inspect(ids[0])?, inspect(ids[1])?, inspect(ids[2])?])
}

/// Produce real same-chart intervals from each owner's actual latest fitted row.
/// # Errors
/// Preserve actual chart preparation, tracking, terms and enclosure failures.
fn chart_entries<'chart>(
    owners: &[Source; 3],
    map: &'chart FixtureAffineChart,
    query: u64,
) -> TestResult<[FixtureChartInterval<'chart>; 3]> {
    let project = |owner: &Source| {
        let view = owner
            .history
            .prepare_chart(owner.context, FixtureChartOutcome::Known(map))?
            .ok_or_else(|| io::Error::other("missing pending-credit chart view"))?;
        let anchor = owner
            .history
            .reference()
            .ok_or_else(|| io::Error::other("missing pending-credit fitted anchor"))?;
        let index = owner
            .history
            .sample_count()
            .checked_sub(1)
            .ok_or_else(|| io::Error::other("missing pending-credit fitted row"))?;
        let projection = view.project_tracking(
            view.context(),
            index,
            anchor,
            MonotonicNs::new(query * SECOND),
            terms()?,
            0.125,
        )?;
        Ok::<_, Box<dyn std::error::Error>>(enclose_fixture_chart_projection(&projection)?)
    };
    Ok([
        project(&owners[0])?,
        project(&owners[1])?,
        project(&owners[2])?,
    ])
}

/// Enclose only the genuine unchanged raw projections retained by chart entries.
/// # Errors
/// Preserve the original enclosure failures; never relabel transformed values.
fn raw_entries(
    entries: &[FixtureChartInterval<'_>; 3],
) -> TestResult<[FixtureProjectedInterval; 3]> {
    Ok([
        enclose_fixture_projection(entries[0].projection().source_projection())?,
        enclose_fixture_projection(entries[1].projection().source_projection())?,
        enclose_fixture_projection(entries[2].projection().source_projection())?,
    ])
}

/// Existing pending B credit causes one replacement estimate without a new event.
///
/// At t5, original distances are A=C=5/8 and B=1/2. With reselection distance
/// 1/4, B's one update gives ratio (5/8)/(1/2+1/4)=5/6, floored to score one.
/// A therefore remains the roster-first principal while B's credit stays pending.
/// Removing A leaves the witnessed B/C pair at minimum two; B is roster-first
/// among survivors and already has the credit required for replacement.
/// # Errors
/// Preserve genuine producer, batch, metadata and principal admission failures.
/// # Panics
/// Reject credit loss/minting, changed raw provenance, detached batch or replay.
#[test]
fn chart_replacement_consumes_preexisting_pending_credit_once() -> TestResult {
    let ids = [label(11)?, label(22)?, label(33)?];
    let mut owners = [
        Source::with_context(
            FixtureContext::new(ids[0], label(17)?, label(23)?),
            0,
            &[8, 10, 12],
            22,
            &[1.0; 3],
            3,
        )?,
        Source::with_context(
            FixtureContext::new(ids[1], label(17)?, label(23)?),
            0,
            &[9, 11, 13],
            22,
            &[1.0; 3],
            3,
        )?,
        Source::with_context(
            FixtureContext::new(ids[2], label(17)?, label(23)?),
            0,
            &[10, 12, 14],
            22,
            &[1.0; 3],
            3,
        )?,
    ];
    let strata = [
        FixturePrincipalStratum::new(ids[0], 2)?,
        FixturePrincipalStratum::new(ids[1], 2)?,
        FixturePrincipalStratum::new(ids[2], 2)?,
    ];
    let map = chart(0, 2, 6, 4.0 / 1024.0, 1.0 / 1024.0)?;
    let initial = chart_entries(&owners, &map, 4)?;
    let initial_raw = raw_entries(&initial)?;
    let initial_batch = FixtureProjectedBatch::new(
        label(17)?,
        label(23)?,
        MonotonicNs::new(4 * SECOND),
        &initial_raw,
    )?;
    let parameters = FixturePrincipalParameters::new(0.25, 0.0)?;
    let minimum = FixtureMinimum::new(2)?;
    let mut state =
        FixturePrincipalState::seed(&ids, parameters, minimum, &initial_batch, &strata)?;
    let ready = state.advance(&initial_batch, &strata)?;
    assert_eq!(ready.kind(), FixturePrincipalOutcomeKind::ReferenceReady);
    assert_eq!(ready.principal_label(), Some(ids[0]));
    assert_eq!(ready.updated_source(), None);
    let initial_facts = source_facts(&state, ids)?;
    for source in initial_facts {
        assert_eq!(source.latest, Some(MonotonicNs::new(2 * SECOND)));
        assert_eq!(source.score_bits, 1.0_f64.to_bits());
        assert_eq!(source.distance_bits, Some(0.5_f64.to_bits()));
        assert_eq!(source.stratum, Some(2));
        assert!(!source.pending);
    }

    owners[1].append(3, 15, 22, 1.0)?;
    let current = chart_entries(&owners, &map, 5)?;
    let current_raw = raw_entries(&current)?;
    let raw_batch = FixtureProjectedBatch::new(
        label(17)?,
        label(23)?,
        MonotonicNs::new(5 * SECOND),
        &current_raw,
    )?;
    let retained = state.advance(&raw_batch, &strata)?;
    assert_eq!(
        retained.kind(),
        FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    );
    assert_eq!(retained.principal_label(), Some(ids[0]));
    assert_eq!(retained.updated_source(), Some(ids[1]));
    assert!(!retained.reference_ready());
    let pending_facts = source_facts(&state, ids)?;
    for (index, source) in pending_facts.iter().enumerate() {
        let latest = if index == 1 { 3 } else { 2 };
        let distance = if index == 1 { 0.5_f64 } else { 0.625_f64 };
        assert_eq!(source.latest, Some(MonotonicNs::new(latest * SECOND)));
        assert_eq!(source.score_bits, 1.0_f64.to_bits());
        assert_eq!(source.distance_bits, Some(distance.to_bits()));
        assert_eq!(source.stratum, Some(2));
        assert_eq!(source.pending, index == 1);
    }

    let survivors = current
        .get(1..)
        .ok_or_else(|| io::Error::other("missing pending-credit survivors"))?;
    let survivor_strata = strata
        .get(1..)
        .ok_or_else(|| io::Error::other("missing pending-credit strata"))?;
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(5 * SECOND), survivors)?;
    let replacement = state.advance_chart(&map, &batch, survivor_strata)?;
    assert_eq!(replacement.kind(), FixturePrincipalOutcomeKind::ReferenceReady);
    assert_eq!(replacement.principal_label(), Some(ids[1]));
    assert_eq!(replacement.updated_source(), None);
    assert!(replacement.reference_ready());
    assert_eq!(replacement.selection().minimum(), 2);
    assert!(ptr::eq(replacement.batch(), &batch));
    assert!(ptr::eq(replacement.chart_selection().batch(), &batch));
    assert!(ptr::eq(replacement.entries().as_ptr(), survivors.as_ptr()));
    assert!(ptr::eq(
        replacement
            .principal_entry()
            .ok_or_else(|| io::Error::other("missing pending-credit principal"))?,
        &survivors[0]
    ));
    for entry in replacement.entries() {
        let projection = entry.projection();
        let original = projection.source_projection();
        assert!(ptr::eq(projection.chart(), &map));
        assert_eq!(original.context().frame(), label(17)?);
        assert_eq!(original.context().generation(), label(23)?);
    }
    assert_eq!(state.frame(), label(17)?);
    assert_eq!(state.mapping_generation(), label(23)?);
    assert_eq!(state.minimum().get(), 2);
    assert_eq!(state.parameters(), parameters);
    assert_eq!(state.roster_len(), 3);
    assert_eq!(state.evaluated_at(), MonotonicNs::new(5 * SECOND));
    let mut consumed_facts = pending_facts;
    consumed_facts[1].pending = false;
    assert_eq!(source_facts(&state, ids)?, consumed_facts);

    let committed = state.clone();
    let replay = state.advance_chart(&map, &batch, survivor_strata)?;
    assert_eq!(
        replay.kind(),
        FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    );
    assert_eq!(replay.principal_label(), Some(ids[1]));
    assert_eq!(replay.updated_source(), None);
    assert!(!replay.reference_ready());
    assert!(ptr::eq(replay.batch(), &batch));
    assert_eq!(state, committed);
    assert_eq!(source_facts(&state, ids)?, consumed_facts);
    Ok(())
}

// SPDX-License-Identifier: GPL-2.0-only
//! S157 retained raw-source principal receiving through genuine chart batches.
//!
//! These finite fixtures use two histories of at most four rows per setup.
//! Original root distances are equal and ample: roster-first source 11 is the
//! independent principal. A common affine view changes neither observation
//! identity nor credit. All descriptors, raw phases and transitions are literal;
//! no transformed legacy wrapper or detached selection is fabricated.

use super::{SECOND, Source, TestResult, chart, label, snapshot, terms};
use std::{io, ptr};
use time_core::{
    FixtureAffineChart, FixtureChartBatch, FixtureChartDomain, FixtureChartFrames,
    FixtureChartInterval, FixtureChartMismatchKind, FixtureChartOutcome,
    FixtureChartPrincipalError, FixtureContext, FixtureMinimum, FixturePrincipalContextField,
    FixturePrincipalError, FixturePrincipalOutcomeKind, FixturePrincipalParameters,
    FixturePrincipalState, FixturePrincipalStratum, FixtureProjectedBatch,
    enclose_fixture_chart_projection, enclose_fixture_projection,
};
use time_domain::MonotonicNs;

/// Two genuine retained owners with distinct labels and overlapping geometry.
/// # Errors
/// Preserve actual source, observation and adaptive-fit admission failures.
fn sources(frame: u64, generation: u64) -> TestResult<[Source; 2]> {
    let make = |id, phases| {
        Source::with_context(
            FixtureContext::new(label(id)?, label(frame)?, label(generation)?),
            0,
            phases,
            22,
            &[1.0; 3],
            3,
        )
    };
    Ok([make(11, &[8, 10, 12])?, make(22, &[9, 11, 13])?])
}

/// Derive each actual chart interval from its owner's latest fitted anchor.
/// # Errors
/// Preserve the real history/view, tracking and enclosure failures.
fn entries<'chart>(
    owners: &[Source; 2],
    maps: [&'chart FixtureAffineChart; 2],
    query: u64,
) -> TestResult<[FixtureChartInterval<'chart>; 2]> {
    let project = |owner: &Source, map: &'chart FixtureAffineChart| {
        let view = owner
            .history
            .prepare_chart(owner.context, FixtureChartOutcome::Known(map))?
            .ok_or_else(|| io::Error::other("missing actual chart view"))?;
        let anchor = owner
            .history
            .reference()
            .ok_or_else(|| io::Error::other("missing actual anchor"))?;
        let index = owner
            .history
            .sample_count()
            .checked_sub(1)
            .ok_or_else(|| io::Error::other("missing actual sample"))?;
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
    Ok([project(&owners[0], maps[0])?, project(&owners[1], maps[1])?])
}

/// Complete strata keyed to the original roster, without caller update flags.
/// # Errors
/// Preserve actual checked label/stratum admission failures.
fn strata() -> TestResult<[FixturePrincipalStratum; 2]> {
    Ok([
        FixturePrincipalStratum::new(label(11)?, 2)?,
        FixturePrincipalStratum::new(label(22)?, 2)?,
    ])
}

/// Seed and consume original pending credit through the existing legacy route.
///
/// Raw intervals are newly enclosed from genuine unchanged source projections;
/// no chart center/geometry is relabelled as a legacy projection.
/// # Errors
/// Preserve real raw enclosure, batch, metadata and principal failures.
/// # Panics
/// Reject a different initial principal, raw frame/generation or credit outcome.
fn legacy_ready(
    current: &[FixtureChartInterval<'_>; 2],
    query: u64,
) -> TestResult<FixturePrincipalState> {
    let raw = [
        enclose_fixture_projection(current[0].projection().source_projection())?,
        enclose_fixture_projection(current[1].projection().source_projection())?,
    ];
    let batch = FixtureProjectedBatch::new(
        label(17)?,
        label(23)?,
        MonotonicNs::new(query * SECOND),
        &raw,
    )?;
    let mut state = FixturePrincipalState::seed(
        &[label(11)?, label(22)?],
        FixturePrincipalParameters::new(0.25, 0.0)?,
        FixtureMinimum::new(2)?,
        &batch,
        &strata()?,
    )?;
    let ready = state.advance(&batch, &strata()?)?;
    assert_eq!(ready.kind(), FixturePrincipalOutcomeKind::ReferenceReady);
    assert_eq!(ready.principal_label(), Some(label(11)?));
    assert_eq!(state.frame(), label(17)?);
    assert_eq!(state.mapping_generation(), label(23)?);
    Ok(state)
}

/// Legacy-ready state crosses into two target mathematical views without credit.
/// # Errors
/// Preserve actual producers, descriptor, batch and principal failures.
/// # Panics
/// Reject detached provenance, changed raw state or manufactured update credit.
#[test]
fn retained_owner_preserves_credit_and_actual_chart_provenance() -> TestResult {
    let owners = sources(17, 23)?;
    let before = [snapshot(&owners[0])?, snapshot(&owners[1])?];
    let expected = chart(0, 2, 6, 4.0 / 1024.0, 1.0 / 1024.0)?;
    let equal = chart(0, 2, 6, 4.0 / 1024.0, 1.0 / 1024.0)?;
    let current = entries(&owners, [&expected, &equal], 4)?;
    let mut state = legacy_ready(&current, 4)?;
    let prior = state.clone();
    let batch = FixtureChartBatch::new(&expected, MonotonicNs::new(4 * SECOND), &current)?;
    let retained = state.advance_chart(&equal, &batch, &strata()?)?;
    assert_eq!(
        retained.kind(),
        FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    );
    assert_eq!(retained.updated_source(), None);
    assert!(!retained.reference_ready());
    assert_eq!(retained.selection().minimum(), 2);
    assert_eq!(retained.principal_label(), Some(label(11)?));
    assert!(ptr::eq(retained.batch(), &batch));
    assert!(ptr::eq(retained.chart_selection().batch(), &batch));
    assert!(ptr::eq(retained.entries().as_ptr(), current.as_ptr()));
    assert!(ptr::eq(
        retained
            .principal_entry()
            .ok_or_else(|| io::Error::other("missing principal"))?,
        &current[0]
    ));
    assert!(ptr::eq(
        retained.entries()[0].projection().chart(),
        &expected
    ));
    assert!(ptr::eq(retained.entries()[1].projection().chart(), &equal));
    assert_eq!(
        retained.entries()[0]
            .projection()
            .source_projection()
            .latest_observed_at(),
        MonotonicNs::new(2 * SECOND)
    );
    assert_eq!(
        retained.entries()[0].projection().center_seconds(),
        10.0 / 1024.0
    );
    assert_eq!(
        retained.entries()[0]
            .projection()
            .phase_rate_seconds_per_second(),
        1.0 / 1024.0
    );
    assert!(retained.entries()[0].projection().center_roundoff_seconds() > 0.0);
    assert_eq!(state, prior);
    let other = FixtureAffineChart::new(
        FixtureChartFrames::new(label(17)?,label(23)?,label(31)?,label(37)?)?,
        expected.domain(),
        5.0 / 1024.0,
        2.0 / 1024.0,
    )?;
    let changed = entries(&owners, [&other, &other], 4)?;
    let changed_batch = FixtureChartBatch::new(&other, MonotonicNs::new(4 * SECOND), &changed)?;
    assert_eq!(
        state
            .advance_chart(&other, &changed_batch, &strata()?)?
            .updated_source(),
        None
    );
    assert_eq!(state, prior);
    assert_eq!([snapshot(&owners[0])?, snapshot(&owners[1])?], before);
    Ok(())
}

/// Only a genuine principal append produces another ready result; replay does not.
/// # Errors
/// Preserve real append, report, chart, batch and principal failures.
/// # Panics
/// Reject a fabricated event, duplicate estimate or changed raw origin.
#[test]
fn genuine_new_observation_survives_chart_receiving_once() -> TestResult {
    let mut owners = sources(17, 23)?;
    let map = chart(0, 2, 6, 4.0 / 1024.0, 1.0 / 1024.0)?;
    let original = entries(&owners, [&map, &map], 4)?;
    let mut state = legacy_ready(&original, 4)?;
    owners[0].append(3, 14, 22, 1.0)?;
    let fresh = entries(&owners, [&map, &map], 5)?;
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(5 * SECOND), &fresh)?;
    let ready = state.advance_chart(&map, &batch, &strata()?)?;
    assert_eq!(ready.kind(), FixturePrincipalOutcomeKind::ReferenceReady);
    assert_eq!(ready.updated_source(), Some(label(11)?));
    assert_eq!(ready.principal_label(), Some(label(11)?));
    assert_eq!(
        state
            .source_state(label(11)?)
            .ok_or_else(|| io::Error::other("missing source"))?
            .latest_observed_at(),
        Some(MonotonicNs::new(3 * SECOND))
    );
    let prior = state.clone();
    let replay = state.advance_chart(&map, &batch, &strata()?)?;
    assert_eq!(
        replay.kind(),
        FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    );
    assert_eq!(replay.updated_source(), None);
    assert_eq!(state, prior);
    Ok(())
}

/// Unexpected descriptor roles and signed-zero bits reject without state changes.
/// # Errors
/// Preserve actual descriptor, batch, metadata and principal failures.
/// # Panics
/// Reject wrong causes, partial state changes or failed valid-neighbor recovery.
#[test]
fn unexpected_chart_rejects_atomically_and_equal_neighbor_recovers() -> TestResult {
    let owners = sources(17, 23)?;
    let map = chart(0, 2, 6, 4.0 / 1024.0, 1.0 / 1024.0)?;
    let current = entries(&owners, [&map, &map], 4)?;
    let mut state = legacy_ready(&current, 4)?;
    let prior = state.clone();
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &current)?;
    let cases = [
        (
            chart(1, 2, 6, 4.0 / 1024.0, 1.0 / 1024.0)?,
            FixtureChartMismatchKind::Domain,
        ),
        (
            chart(0, 3, 6, 4.0 / 1024.0, 1.0 / 1024.0)?,
            FixtureChartMismatchKind::Domain,
        ),
        (
            chart(0, 2, 7, 4.0 / 1024.0, 1.0 / 1024.0)?,
            FixtureChartMismatchKind::Domain,
        ),
        (
            chart(0, 2, 6, 5.0 / 1024.0, 1.0 / 1024.0)?,
            FixtureChartMismatchKind::Displacement,
        ),
        (
            chart(0, 2, 6, 4.0 / 1024.0, 2.0 / 1024.0)?,
            FixtureChartMismatchKind::Rate,
        ),
    ];
    for (expected, kind) in &cases {
        assert!(
            matches!(state.advance_chart(expected, &batch, &strata()?), Err(FixtureChartPrincipalError::ChartMismatch {kind: actual}) if actual==*kind)
        );
        assert_eq!(state, prior);
    }
    let mut control = prior.clone();
    let actual = state.advance_chart(&map, &batch, &strata()?)?;
    let healthy = control.advance_chart(&map, &batch, &strata()?)?;
    assert_eq!(actual.kind(), healthy.kind());
    assert_eq!(state, control);
    let zero = chart(0, 2, 6, 0.0, 0.0)?;
    let zeros = entries(&owners, [&zero, &zero], 4)?;
    let zero_batch = FixtureChartBatch::new(&zero, MonotonicNs::new(4 * SECOND), &zeros)?;
    for (expected, kind) in [
        (
            chart(0, 2, 6, -0.0, 0.0)?,
            FixtureChartMismatchKind::Displacement,
        ),
        (chart(0, 2, 6, 0.0, -0.0)?, FixtureChartMismatchKind::Rate),
    ] {
        assert!(
            matches!(state.advance_chart(&expected, &zero_batch, &strata()?), Err(FixtureChartPrincipalError::ChartMismatch{kind: actual}) if actual==kind)
        );
        assert_eq!(state, prior);
    }
    Ok(())
}

/// Wrong raw source origins and backward queries cannot be laundered by a chart.
/// # Errors
/// Preserve actual history, frame/domain, batch and principal failures.
/// # Panics
/// Reject a wrong contextual cause or any changed retained owner.
#[test]
fn source_origin_and_evaluation_guards_remain_unavoidable() -> TestResult {
    let owners = sources(17, 23)?;
    let map = chart(0, 2, 6, 4.0 / 1024.0, 1.0 / 1024.0)?;
    let current = entries(&owners, [&map, &map], 4)?;
    let mut state = legacy_ready(&current, 4)?;
    let prior = state.clone();
    for (frame, generation, field) in [
        (31, 23, FixturePrincipalContextField::Frame),
        (17, 37, FixturePrincipalContextField::MappingGeneration),
    ] {
        let wrong_owners = sources(frame, generation)?;
        let wrong = FixtureAffineChart::new(
            FixtureChartFrames::new(label(frame)?, label(generation)?, label(19)?, label(29)?)?,
            FixtureChartDomain::new(
                MonotonicNs::new(0),
                MonotonicNs::new(2 * SECOND),
                MonotonicNs::new(6 * SECOND),
            )?,
            4.0 / 1024.0,
            1.0 / 1024.0,
        )?;
        let wrong_entries = entries(&wrong_owners, [&wrong, &wrong], 4)?;
        let batch = FixtureChartBatch::new(&wrong, MonotonicNs::new(4 * SECOND), &wrong_entries)?;
        assert!(
            matches!(state.advance_chart(&wrong,&batch,&strata()?),Err(FixtureChartPrincipalError::Principal{source:FixturePrincipalError::ContextMismatch{field:actual}}) if actual==field)
        );
        assert_eq!(state, prior);
    }
    let earlier = entries(&owners, [&map, &map], 3)?;
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(3 * SECOND), &earlier)?;
    assert!(matches!(
        state.advance_chart(&map, &batch, &strata()?),
        Err(FixtureChartPrincipalError::Principal {
            source: FixturePrincipalError::BackwardEvaluation
        })
    ));
    assert_eq!(state, prior);
    Ok(())
}

/// Every expected frame tag is checked and capacity retains first-error priority.
/// # Errors
/// Preserve actual chart, batch, checked metadata and principal failures.
/// # Panics
/// Reject a missing descriptor check, changed error ordering or changed state.
#[test]
fn expected_frame_bits_and_metadata_capacity_reject_before_commit() -> TestResult {
    let owners = sources(17, 23)?;
    let map = chart(0, 2, 6, 4.0 / 1024.0, 1.0 / 1024.0)?;
    let current = entries(&owners, [&map, &map], 4)?;
    let mut state = legacy_ready(&current, 4)?;
    let prior = state.clone();
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &current)?;
    for ids in [[31, 23, 19, 29], [17, 37, 19, 29], [17, 23, 41, 29], [17, 23, 19, 43]] {
        let expected = FixtureAffineChart::new(
            FixtureChartFrames::new(label(ids[0])?, label(ids[1])?, label(ids[2])?, label(ids[3])?)?,
            map.domain(), 4.0/1024.0, 1.0/1024.0,
        )?;
        assert!(matches!(state.advance_chart(&expected, &batch, &strata()?), Err(FixtureChartPrincipalError::ChartMismatch{kind:FixtureChartMismatchKind::Frames})));
        assert_eq!(state,prior);
        let many=[FixturePrincipalStratum::new(label(11)?,2)?;17];
        assert!(matches!(state.advance_chart(&expected, &batch, &many), Err(FixtureChartPrincipalError::Principal{source:FixturePrincipalError::MetadataCapacityExceeded})));
        assert_eq!(state,prior);
    }
    Ok(())
}

/// Multiple updates and stratum edits cannot bypass the shared event receiver.
/// # Errors
/// Preserve actual histories, chart, batch, stratum and principal failures.
/// # Panics
/// Reject a wrong cause, partial update or failed genuine-neighbor retry.
#[test]
fn chart_event_rejections_preserve_credit_for_valid_retry() -> TestResult {
    let mut owners = sources(17,23)?;
    let map = chart(0,2,6,4.0/1024.0,1.0/1024.0)?;
    let original=entries(&owners,[&map,&map],4)?;
    let mut state=legacy_ready(&original,4)?;
    let prior=state.clone();
    owners[0].append(3,14,22,1.0)?;
    owners[1].append(3,15,22,1.0)?;
    let both=entries(&owners,[&map,&map],5)?;
    let both_batch=FixtureChartBatch::new(&map,MonotonicNs::new(5*SECOND),&both)?;
    assert!(matches!(state.advance_chart(&map,&both_batch,&strata()?),Err(FixtureChartPrincipalError::Principal{source:FixturePrincipalError::MultipleObservations})));
    assert_eq!(state,prior);
    let mut neighbor=sources(17,23)?;
    neighbor[0].append(3,14,22,1.0)?;
    let current=entries(&neighbor,[&map,&map],5)?;
    let batch=FixtureChartBatch::new(&map,MonotonicNs::new(5*SECOND),&current)?;
    let mut control=prior.clone();
    assert_eq!(state.advance_chart(&map,&batch,&strata()?)?.kind(),FixturePrincipalOutcomeKind::ReferenceReady);
    assert_eq!(control.advance_chart(&map,&batch,&strata()?)?.kind(),FixturePrincipalOutcomeKind::ReferenceReady);
    assert_eq!(state,control);
    let changed=[FixturePrincipalStratum::new(label(11)?,3)?,FixturePrincipalStratum::new(label(22)?,2)?];
    assert!(matches!(state.advance_chart(&map,&batch,&changed),Err(FixtureChartPrincipalError::Principal{source:FixturePrincipalError::StratumWithoutObservation{index:0}})));
    assert_eq!(state,control);
    let old_owners=sources(17,23)?;
    let older=entries(&old_owners,[&map,&map],5)?;
    let old_batch=FixtureChartBatch::new(&map,MonotonicNs::new(5*SECOND),&older)?;
    assert!(matches!(state.advance_chart(&map,&old_batch,&strata()?),Err(FixtureChartPrincipalError::Principal{source:FixturePrincipalError::BackwardObservation{index:0}})));
    assert_eq!(state,control);
    Ok(())
}

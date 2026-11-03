// SPDX-License-Identifier: GPL-2.0-only
//! S155 genuine history -> chart interval -> associated majority consumer.
//!
//! Literal Q33 endpoints and majority intersections are independent fixtures.
//! No checked projection/interval is relabelled or fabricated. This source-only
//! recipe does not establish physical confidence, freshness or clock authority.
//! Fixed added recipe:32 histories/96 appends,22 maps,46 prepares/projections/
//! enclosures/terms,44 batches,8 minima/selectors,64 three-row snapshots.
//! Direct observers use fewer than4096 getters and64 formatting/error/iterator
//! requests. Source-owned guards/selection and delegated producers are additional
//! prepaid upper work, never observed counters from this module.

use super::{SECOND, Source, SourceSnapshot, TestResult, chart, label, snapshot};
use std::{error::Error, io, ptr};
use time_core::{
    BatchError, FixtureAffineChart, FixtureChartBatch, FixtureChartBatchError, FixtureChartDomain,
    FixtureChartFrames, FixtureChartInterval, FixtureChartMismatchKind, FixtureChartOutcome,
    FixtureContext, FixtureEndpointRounding, FixtureMinimum, FixtureOutcomeKind,
    FixtureProjectionTerms, FixtureSelection, enclose_fixture_chart_projection,
};
use time_domain::MonotonicNs;

/// Build a genuine three-row owner, with its identity admitted before each row.
/// # Errors
/// Preserve checked labels and actual history/observation admission failures.
fn source(
    id: u64,
    start: u64,
    values: &[i32; 3],
    frame: u64,
    generation: u64,
) -> TestResult<Source> {
    Source::with_context(
        FixtureContext::new(label(id)?, label(frame)?, label(generation)?),
        start,
        values,
        22,
        &[1.0; 3],
        3,
    )
}

/// Derive the real interval without modifying the source owner or chart.
/// # Errors
/// Preserve preparation, projection and exact enclosure failures.
/// # Panics
/// Reject a missing/nontrivial arithmetic bound or an excessive fixture bound.
fn interval<'chart>(
    owner: &Source,
    map: &'chart FixtureAffineChart,
    query: u64,
) -> TestResult<FixtureChartInterval<'chart>> {
    let view = owner
        .history
        .prepare_chart(owner.context, FixtureChartOutcome::Known(map))?
        .ok_or_else(|| io::Error::other("missing genuine chart view"))?;
    let target = FixtureContext::new(
        owner.context.source_id(),
        map.frames().target_frame(),
        map.frames().target_generation(),
    );
    let anchor = owner
        .history
        .reference()
        .ok_or_else(|| io::Error::other("missing genuine reference"))?;
    let projection = view.project_tracking(
        target,
        2,
        anchor,
        MonotonicNs::new(query * SECOND),
        FixtureProjectionTerms::new(4.0 / 1024.0, 0.0, 0.0)?,
        0.125,
    )?;
    assert!(
        projection.center_roundoff_seconds() > 0.0
            && projection.center_roundoff_seconds() < 1.0 / 72_057_594_037_927_936.0
    );
    Ok(enclose_fixture_chart_projection(&projection)?)
}

/// Make all four independent raw/target chart labels explicit.
/// # Errors
/// Preserve frame, inclusive-domain and scalar admission failures.
fn map_with_frames(frames: [u64; 4]) -> TestResult<FixtureAffineChart> {
    Ok(FixtureAffineChart::new(
        FixtureChartFrames::new(
            label(frames[0])?,
            label(frames[1])?,
            label(frames[2])?,
            label(frames[3])?,
        )?,
        FixtureChartDomain::new(
            MonotonicNs::new(0),
            MonotonicNs::new(2 * SECOND),
            MonotonicNs::new(4 * SECOND),
        )?,
        4.0 / 1024.0,
        1.0 / 1024.0,
    )?)
}

/// Capture each genuine public history independently of batch construction.
/// # Errors
/// Preserve bounded row/metadata observation failures.
fn snapshots(owners: &[Source]) -> TestResult<Vec<SourceSnapshot>> {
    owners.iter().map(snapshot).collect()
}

/// Extract only an actual rejected public constructor, without a panic fallback.
/// # Errors
/// Reject unexpected acceptance as a fixture failure.
fn rejected<T>(result: Result<T, FixtureChartBatchError>) -> TestResult<FixtureChartBatchError> {
    result
        .err()
        .ok_or_else(|| io::Error::other("chart batch unexpectedly accepted").into())
}

/// Check the typed first discrepancy independently of payload formatting.
/// # Panics
/// Reject a wrong index, mismatch role or unrelated error.
fn mismatch(error: FixtureChartBatchError, index: usize, kind: FixtureChartMismatchKind) {
    assert!(
        matches!(error, FixtureChartBatchError::Mismatch { index: actual, kind: role }
        if actual == index && role == kind)
    );
}

/// Assert the independently known complete three-source maximum component.
/// # Errors
/// Reject a missing genuine common witness.
/// # Panics
/// Reject changed topology, depth, minimum or actual supporter identities.
fn majority(result: &FixtureSelection, lower: i64, upper: i64) -> TestResult {
    assert_eq!(result.population(), 4);
    assert_eq!(result.minimum(), 3);
    assert_eq!(result.maximum_depth(), 3);
    assert!(result.strict_majority() && result.minimum_met());
    assert_eq!(result.outcome(), FixtureOutcomeKind::WitnessedFixture);
    // The numeric owner exposes this optional count only for ambiguous outcomes;
    // the actual witnessed component and intersection are checked below.
    assert_eq!(result.component_count(), None);
    let witness = result
        .witnessed()
        .ok_or_else(|| io::Error::other("missing three-source witness"))?;
    assert_eq!(
        (witness.component_lower(), witness.component_upper()),
        (lower, upper)
    );
    assert_eq!(
        (witness.intersection_lower(), witness.intersection_upper()),
        (lower, upper)
    );
    assert_eq!(witness.witness(), lower);
    assert_eq!(
        witness
            .supporters()
            .map(std::num::NonZeroU64::get)
            .collect::<Vec<_>>(),
        vec![1, 2, 3]
    );
    Ok(())
}

/// Compare complete genuine intervals with the fixed outward Q33 oracle.
/// # Panics
/// Reject any changed endpoint or rounding direction.
fn exact_intervals(entries: &[FixtureChartInterval<'_>]) {
    let expected = [
        (50_331_647, 117_440_513),
        (67_108_863, 134_217_729),
        (83_886_079, 150_994_945),
        (318_767_103, 385_875_969),
    ];
    assert_eq!(entries.len(), expected.len());
    for (entry, bounds) in entries.iter().zip(expected) {
        assert_eq!((entry.interval().lower(), entry.interval().upper()), bounds);
        assert_eq!(
            entry.lower_rounding(),
            FixtureEndpointRounding::OutwardFraction
        );
        assert_eq!(
            entry.upper_rounding(),
            FixtureEndpointRounding::OutwardFraction
        );
    }
}

/// Check retained source time separately from the supplied evaluation instant.
///
/// The primary fixture's actual anchor/latest/reference are all at2 seconds;
/// query2/4 therefore has individually exact anchor/latest ages0/2 seconds.
/// # Panics
/// Reject manufactured observation credit or truncated/refreshed time metadata.
fn projection_times(entries: &[FixtureChartInterval<'_>], query: u64, elapsed: u64) {
    for entry in entries {
        let original = entry.projection().source_projection();
        assert_eq!(original.anchor_observed_at().nanoseconds(), 2 * SECOND);
        assert_eq!(original.latest_observed_at().nanoseconds(), 2 * SECOND);
        assert_eq!(original.fit_reference().nanoseconds(), 2 * SECOND);
        assert_eq!(original.projected_at().nanoseconds(), query * SECOND);
        assert_eq!(original.anchor_elapsed().nanoseconds(), elapsed * SECOND);
        assert_eq!(original.latest_elapsed().nanoseconds(), elapsed * SECOND);
    }
}

/// Exact dyadic intervals keep source identity and borrowed batch association.
/// # Errors
/// Preserve genuine producer, interval and selector failures.
/// # Panics
/// Reject wrong endpoints, detached provenance, old-time selection or new credit.
#[test]
fn chart_batch_selects_genuine_majority_at_common_time() -> TestResult {
    let owners = vec![
        source(1, 0, &[8, 10, 12], 17, 23)?,
        source(2, 0, &[10, 12, 14], 17, 23)?,
        source(3, 0, &[12, 14, 16], 17, 23)?,
        source(4, 0, &[40, 42, 44], 17, 23)?,
    ];
    let before = snapshots(&owners)?;
    let map = chart(0, 2, 4, 4.0 / 1024.0, 1.0 / 1024.0)?;
    let equal = chart(0, 2, 4, 4.0 / 1024.0, 1.0 / 1024.0)?;
    let entries = [
        interval(&owners[0], &map, 4)?,
        interval(&owners[1], &equal, 4)?,
        interval(&owners[2], &map, 4)?,
        interval(&owners[3], &map, 4)?,
    ];
    exact_intervals(&entries);
    projection_times(&entries, 4, 2);
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &entries)?;
    assert!(
        ptr::eq(batch.chart(), ptr::from_ref(&map)) && ptr::eq(batch.entries(), entries.as_slice())
    );
    assert!(ptr::eq(
        batch.entries()[1].projection().chart(),
        ptr::from_ref(&equal)
    ));
    assert_eq!(batch.projected_at(), MonotonicNs::new(4 * SECOND));
    assert_eq!(batch.len(), 4);
    assert!(!batch.is_empty());
    let selected = batch.select(FixtureMinimum::new(3)?)?;
    assert!(ptr::eq(selected.batch(), ptr::from_ref(&batch)));
    majority(selected.selection(), 83_886_079, 117_440_513)?;
    // Both headers are in-domain, but one nanosecond is still a distinct instant.
    mismatch(
        rejected(FixtureChartBatch::new(
            &map,
            MonotonicNs::new(4 * SECOND - 1),
            &entries,
        ))?,
        0,
        FixtureChartMismatchKind::EvaluationTime,
    );
    let at_two = owners
        .iter()
        .map(|owner| interval(owner, &map, 2))
        .collect::<TestResult<Vec<_>>>()?;
    projection_times(&at_two, 2, 0);
    let earlier = FixtureChartBatch::new(&map, MonotonicNs::new(2 * SECOND), &at_two)?;
    majority(
        earlier.select(FixtureMinimum::new(3)?)?.selection(),
        67_108_863,
        100_663_297,
    )?;
    majority(
        batch.select(FixtureMinimum::new(3)?)?.selection(),
        83_886_079,
        117_440_513,
    )?;
    projection_times(batch.entries(), 4, 2);
    assert_eq!(snapshots(&owners)?, before);
    for owner in &owners {
        assert_eq!(
            owner.history.reference(),
            Some(MonotonicNs::new(2 * SECOND))
        );
    }
    assert_eq!(format!("{batch:?}"), "FixtureChartBatch { .. }");
    assert_eq!(format!("{selected:?}"), "FixtureChartSelection { .. }");
    Ok(())
}

/// Loss/disagreement never lowers the independently configured minimum.
/// # Errors
/// Preserve actual producer and selection failures.
/// # Panics
/// Reject conflated population/agreement/majority outcomes or changed histories.
#[test]
fn chart_batch_source_loss_does_not_lower_minimum() -> TestResult {
    let owners = vec![
        source(1, 0, &[8, 10, 12], 17, 23)?,
        source(2, 0, &[10, 12, 14], 17, 23)?,
        source(3, 0, &[40, 42, 44], 17, 23)?,
        source(4, 0, &[42, 44, 46], 17, 23)?,
    ];
    let before = snapshots(&owners)?;
    let map = chart(0, 2, 4, 4.0 / 1024.0, 1.0 / 1024.0)?;
    let entries = owners
        .iter()
        .map(|owner| interval(owner, &map, 4))
        .collect::<TestResult<Vec<_>>>()?;
    for (slice, expected, population, strict) in [
        (
            &entries[..3],
            FixtureOutcomeKind::InsufficientAgreement,
            3,
            true,
        ),
        (
            &entries[..2],
            FixtureOutcomeKind::InsufficientPopulation,
            2,
            true,
        ),
        (&entries[..], FixtureOutcomeKind::NoMajority, 4, false),
    ] {
        let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), slice)?;
        let selected = batch.select(FixtureMinimum::new(3)?)?;
        let facts = selected.selection();
        assert_eq!(facts.outcome(), expected);
        assert_eq!(facts.population(), population);
        assert_eq!(facts.maximum_depth(), 2);
        assert_eq!(facts.strict_majority(), strict);
        assert!(!facts.minimum_met());
        assert_eq!(facts.minimum(), 3);
        assert!(facts.witnessed().is_none());
    }
    assert_eq!(snapshots(&owners)?, before);
    Ok(())
}

/// Empty queries still admit their header; sixteen genuine identities are legal.
/// # Errors
/// Preserve true producers and public receiving failures.
/// # Panics
/// Reject a capacity boundary/priority error, lost source, or changed owner.
#[test]
fn chart_batch_empty_and_capacity_boundaries_use_real_owners() -> TestResult {
    let owners = (1..=16)
        .map(|id| source(id, 0, &[8, 10, 12], 17, 23))
        .collect::<TestResult<Vec<_>>>()?;
    let before = snapshots(&owners)?;
    let map = chart(0, 2, 4, 4.0 / 1024.0, 1.0 / 1024.0)?;
    let entries = owners
        .iter()
        .map(|owner| interval(owner, &map, 4))
        .collect::<TestResult<Vec<_>>>()?;
    let empty = FixtureChartBatch::new(&map, MonotonicNs::new(0), &[])?;
    assert!(empty.is_empty() && empty.entries().is_empty());
    assert_eq!(empty.len(), 0);
    assert_eq!(
        empty.select(FixtureMinimum::new(1)?)?.selection().outcome(),
        FixtureOutcomeKind::Empty
    );
    assert!(matches!(
        rejected(FixtureChartBatch::new(
            &map,
            MonotonicNs::new(5 * SECOND),
            &[]
        ))?,
        FixtureChartBatchError::EvaluationOutsideDomain
    ));
    let full = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &entries)?;
    let selected = full.select(FixtureMinimum::new(16)?)?;
    assert_eq!(selected.selection().population(), 16);
    assert_eq!(selected.selection().maximum_depth(), 16);
    assert_eq!(
        selected.selection().outcome(),
        FixtureOutcomeKind::WitnessedFixture
    );
    let witness = selected
        .selection()
        .witnessed()
        .ok_or_else(|| io::Error::other("missing full witness"))?;
    assert_eq!(
        witness
            .supporters()
            .map(std::num::NonZeroU64::get)
            .collect::<Vec<_>>(),
        (1..=16).collect::<Vec<_>>()
    );
    let mut excessive = entries.clone();
    excessive.push(entries[0]);
    assert!(matches!(
        rejected(FixtureChartBatch::new(
            &map,
            MonotonicNs::new(5 * SECOND),
            &excessive
        ))?,
        FixtureChartBatchError::CapacityExceeded
    ));
    assert!(matches!(
        rejected(FixtureChartBatch::new(
            &map,
            MonotonicNs::new(5 * SECOND),
            &entries
        ))?,
        FixtureChartBatchError::EvaluationOutsideDomain
    ));
    assert_eq!(snapshots(&owners)?, before);
    Ok(())
}

/// Every source/target frame/generation field precedes duplicate-ID admission.
/// # Errors
/// Preserve each distinct genuine owner and descriptor's admission failures.
/// # Panics
/// Reject a forgotten chart label, wrong priority/index or mutated source.
#[test]
fn chart_batch_compares_every_chart_frame_field() -> TestResult {
    let owner = source(1, 0, &[8, 10, 12], 17, 23)?;
    let before = snapshot(&owner)?;
    let map = map_with_frames([17, 23, 19, 29])?;
    let good = interval(&owner, &map, 4)?;
    let healthy = [good];
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &healthy)?;
    assert_eq!(batch.len(), 1);
    for labels in [
        [18, 23, 19, 29],
        [17, 24, 19, 29],
        [17, 23, 20, 29],
        [17, 23, 19, 30],
    ] {
        let other = source(1, 0, &[8, 10, 12], labels[0], labels[1])?;
        let other_before = snapshot(&other)?;
        let altered = map_with_frames(labels)?;
        let entries = [good, interval(&other, &altered, 4)?];
        mismatch(
            rejected(FixtureChartBatch::new(
                &map,
                MonotonicNs::new(4 * SECOND),
                &entries,
            ))?,
            1,
            FixtureChartMismatchKind::Frames,
        );
        // First input's time mismatch precedes a later input's frame mismatch.
        mismatch(
            rejected(FixtureChartBatch::new(
                &map,
                MonotonicNs::new(3 * SECOND),
                &entries,
            ))?,
            0,
            FixtureChartMismatchKind::EvaluationTime,
        );
        // Inside one entry, frames precede its simultaneously wrong query time.
        let single = [entries[1]];
        mismatch(
            rejected(FixtureChartBatch::new(
                &map,
                MonotonicNs::new(3 * SECOND),
                &single,
            ))?,
            0,
            FixtureChartMismatchKind::Frames,
        );
        assert_eq!(snapshot(&other)?, other_before);
    }
    assert_eq!(snapshot(&owner)?, before);
    Ok(())
}

/// Complete domains and raw scalar bits are associated before entry time/ID.
/// # Errors
/// Preserve genuine chart/projection admission and actual batch failures.
/// # Panics
/// Reject skipped metadata or a later-entry error masking the first mismatch.
#[test]
fn chart_batch_orders_domain_scalar_time_and_entry_mismatches() -> TestResult {
    let owner = source(1, 1, &[8, 10, 12], 17, 23)?;
    let before = snapshot(&owner)?;
    let map = chart(0, 2, 4, 4.0 / 1024.0, 1.0 / 1024.0)?;
    let good = interval(&owner, &map, 4)?;
    let healthy = [good];
    assert_eq!(
        FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &healthy)?.len(),
        1
    );
    for (start, reference, end, displacement, rate, query, kind) in [
        (1, 2, 4, 4.0, 1.0, 4, FixtureChartMismatchKind::Domain),
        (0, 3, 4, 4.0, 1.0, 4, FixtureChartMismatchKind::Domain),
        (0, 2, 5, 4.0, 1.0, 4, FixtureChartMismatchKind::Domain),
        (0, 2, 4, 8.0, 1.0, 4, FixtureChartMismatchKind::Displacement),
        (0, 2, 4, 4.0, 2.0, 4, FixtureChartMismatchKind::Rate),
    ] {
        let altered = chart(start, reference, end, displacement / 1024.0, rate / 1024.0)?;
        let entries = [good, interval(&owner, &altered, query)?];
        mismatch(
            rejected(FixtureChartBatch::new(
                &map,
                MonotonicNs::new(4 * SECOND),
                &entries,
            ))?,
            1,
            kind,
        );
        let single = [entries[1]];
        mismatch(
            rejected(FixtureChartBatch::new(
                &map,
                MonotonicNs::new(3 * SECOND),
                &single,
            ))?,
            0,
            kind,
        );
    }
    for (reference, kind) in [
        (3, FixtureChartMismatchKind::Domain),
        (2, FixtureChartMismatchKind::Displacement),
    ] {
        let altered = chart(0, reference, 4, 8.0 / 1024.0, 2.0 / 1024.0)?;
        let single = [interval(&owner, &altered, 4)?];
        mismatch(
            rejected(FixtureChartBatch::new(
                &map,
                MonotonicNs::new(4 * SECOND),
                &single,
            ))?,
            0,
            kind,
        );
    }
    let old = interval(&owner, &map, 3)?;
    let entries = [good, old];
    mismatch(
        rejected(FixtureChartBatch::new(
            &map,
            MonotonicNs::new(4 * SECOND),
            &entries,
        ))?,
        1,
        FixtureChartMismatchKind::EvaluationTime,
    );
    // Earlier input time failure wins over the later duplicate identity.
    let reversed = [old, good];
    mismatch(
        rejected(FixtureChartBatch::new(
            &map,
            MonotonicNs::new(4 * SECOND),
            &reversed,
        ))?,
        0,
        FixtureChartMismatchKind::EvaluationTime,
    );
    assert_eq!(snapshot(&owner)?, before);
    Ok(())
}

/// A real duplicate preserves the original batch error and redacted boundary.
/// # Errors
/// Preserve genuine producer/constructor failures.
/// # Panics
/// Reject wrong error association, lost original cause, raw Debug or mutation.
#[test]
fn chart_batch_preserves_real_duplicate_cause_and_redaction() -> TestResult {
    let owner = source(1, 0, &[8, 10, 12], 17, 23)?;
    let before = snapshot(&owner)?;
    let map = chart(0, 2, 4, 4.0 / 1024.0, 1.0 / 1024.0)?;
    let one = interval(&owner, &map, 4)?;
    let healthy = [one];
    assert_eq!(
        FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &healthy)?.len(),
        1
    );
    let duplicate = [one, one];
    let error = rejected(FixtureChartBatch::new(
        &map,
        MonotonicNs::new(4 * SECOND),
        &duplicate,
    ))?;
    assert!(matches!(
        error,
        FixtureChartBatchError::Batch {
            source: BatchError::DuplicateLabel { later_index: 1 }
        }
    ));
    assert_eq!(
        error
            .source()
            .and_then(|cause| cause.downcast_ref::<BatchError>()),
        Some(&BatchError::DuplicateLabel { later_index: 1 })
    );
    assert_eq!(format!("{error:?}"), "FixtureChartBatchError { .. }");
    assert_eq!(snapshot(&owner)?, before);
    Ok(())
}

/// Equal values accept separate descriptors, while legal signed-zero bits differ.
/// # Errors
/// Preserve genuine descriptor/interval and batch admission failures.
/// # Panics
/// Reject pointer-only equality, lost provenance, scalar-bit conflation or mutation.
#[test]
fn chart_batch_equal_descriptor_values_preserve_pointers_and_zero_bits() -> TestResult {
    let owner = source(1, 0, &[8, 10, 12], 17, 23)?;
    let before = snapshot(&owner)?;
    let map = chart(0, 2, 4, 0.0, 0.0)?;
    let equal = chart(0, 2, 4, 0.0, 0.0)?;
    let negative_d = chart(0, 2, 4, -0.0, 0.0)?;
    let negative_f = chart(0, 2, 4, 0.0, -0.0)?;
    let equivalent = [interval(&owner, &equal, 4)?];
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &equivalent)?;
    assert!(ptr::eq(batch.chart(), ptr::from_ref(&map)));
    assert!(ptr::eq(
        batch.entries()[0].projection().chart(),
        ptr::from_ref(&equal)
    ));
    for (other, kind) in [
        (&negative_d, FixtureChartMismatchKind::Displacement),
        (&negative_f, FixtureChartMismatchKind::Rate),
    ] {
        let entries = [interval(&owner, other, 4)?];
        mismatch(
            rejected(FixtureChartBatch::new(
                &map,
                MonotonicNs::new(4 * SECOND),
                &entries,
            ))?,
            0,
            kind,
        );
    }
    assert_eq!(snapshot(&owner)?, before);
    Ok(())
}

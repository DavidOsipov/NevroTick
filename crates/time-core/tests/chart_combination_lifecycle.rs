// SPDX-License-Identifier: GPL-2.0-only
//! Public genuine-entry chart combination journeys and complete-owner rollback.
//!
//! Each setup owns two actual three-row adaptive histories; a single later A
//! observation exercises real credit. All chart/raw entries, seeds, decisions and
//! outputs come through public checked APIs. No fitted payload, selection, owner
//! state or uncertainty is fabricated. These are fixture contracts, not production
//! acquisition, installed source migration or applied clock qualification.

use std::{error::Error, io, num::NonZeroU64, ptr};

use time_core::{
    FixtureAdaptiveHistory, FixtureAffineChart, FixtureChartBatch, FixtureChartCombinationError,
    FixtureChartCombinationOutcome, FixtureChartDomain, FixtureChartFrames, FixtureChartInterval,
    FixtureChartOutcome, FixtureCombinationParameters, FixtureCombinationSourceStatus,
    FixtureCombinationState, FixtureContext, FixtureMinimum, FixtureObservation,
    FixturePrincipalOutcomeKind, FixturePrincipalParameters, FixturePrincipalStratum,
    FixtureProjectedBatch, FixtureProjectionTerms, enclose_fixture_chart_projection,
    enclose_fixture_projection,
};
use time_domain::{MonotonicNs, UtcCoordinateQ32, exchange_arithmetic};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;
const SECOND: u64 = 1_000_000_000;

/// Admit a fixed fixture label without substituting another identity.
/// # Errors
/// Reject an accidental zero before any fixture is constructed.
fn label(value: u64) -> TestResult<NonZeroU64> {
    NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero fixture label").into())
}

/// One unchanged actual raw context and its solely owned adaptive history.
struct Source {
    context: FixtureContext,
    history: FixtureAdaptiveHistory,
}

impl Source {
    /// Build three actual chronological exchange-derived dyadic observations.
    /// # Errors
    /// Preserve context, observation, append and fitted-history failures.
    fn new(id: u64, values: [i32; 3]) -> TestResult<Self> {
        let context = FixtureContext::new(label(id)?, label(17)?, label(23)?);
        let mut result = Self {
            context,
            history: FixtureAdaptiveHistory::new(context, 3)?,
        };
        for ((second, value), divisor) in [0, 1, 2].into_iter().zip(values).zip([1.0, 1.0, 4.0]) {
            result.append(second, value, divisor)?;
        }
        Ok(result)
    }

    /// Append one genuine bounded fixture observation, never a caller update flag.
    /// # Errors
    /// Reject fixture time overflow or preserve actual domain and owner failures.
    fn append(&mut self, second: u64, value: i32, divisor: f64) -> TestResult {
        if second > 3 {
            return Err(io::Error::other("fixture time bound").into());
        }
        let zero = UtcCoordinateQ32::new(0)?;
        let shifted = UtcCoordinateQ32::new(i128::from(value) << 28)?;
        let phase = exchange_arithmetic(zero, shifted, shifted, zero)?.offset();
        self.history.try_append(FixtureObservation::new(
            self.context,
            MonotonicNs::new(second * SECOND),
            phase,
            divisor,
        )?)?;
        Ok(())
    }

    /// Project the actual latest fitted row while retaining source and map identity.
    /// # Errors
    /// Reject fixture query bounds or preserve genuine view/projection/enclosure failures.
    fn entry<'chart>(
        &self,
        map: &'chart FixtureAffineChart,
        query: u64,
        base_distance: f64,
        tracking: bool,
        skew: f64,
    ) -> TestResult<FixtureChartInterval<'chart>> {
        if !(4..=5).contains(&query) {
            return Err(io::Error::other("fixture query bound").into());
        }
        let view = self
            .history
            .prepare_chart(self.context, FixtureChartOutcome::Known(map))?
            .ok_or_else(|| io::Error::other("missing actual chart view"))?;
        let index = self
            .history
            .sample_count()
            .checked_sub(1)
            .ok_or_else(|| io::Error::other("missing actual fitted row"))?;
        let anchor = self
            .history
            .reference()
            .ok_or_else(|| io::Error::other("missing actual anchor"))?;
        let at = MonotonicNs::new(query * SECOND);
        let terms = FixtureProjectionTerms::new(base_distance, 0.0, 0.125)?;
        let projection = if tracking {
            view.project_tracking(view.context(), index, anchor, at, terms, skew)?
        } else {
            view.project_anchor(view.context(), index, anchor, at, terms)?
        };
        Ok(enclose_fixture_chart_projection(&projection)?)
    }
}

/// Admit one nonzero mathematical view over a fixed raw origin and target.
/// # Errors
/// Preserve checked frame, inclusive-domain and descriptor scalar failures.
fn chart() -> TestResult<FixtureAffineChart> {
    Ok(FixtureAffineChart::new(
        FixtureChartFrames::new(label(17)?, label(23)?, label(19)?, label(29)?)?,
        FixtureChartDomain::new(
            MonotonicNs::new(0),
            MonotonicNs::new(2 * SECOND),
            MonotonicNs::new(8 * SECOND),
        )?,
        4.0 / 1024.0,
        1.0 / 1024.0,
    )?)
}

/// Build final contiguous actual chart slots, with explicit B metadata availability.
/// # Errors
/// Preserve the genuine source projection and enclosure failures.
fn entries<'chart>(
    a: &Source,
    b: &Source,
    map: &'chart FixtureAffineChart,
    query: u64,
    b_distance: f64,
    b_tracking: bool,
) -> TestResult<[FixtureChartInterval<'chart>; 2]> {
    Ok([
        a.entry(map, query, 0.25, true, 0.125)?,
        b.entry(map, query, b_distance, b_tracking, 0.25)?,
    ])
}

/// Admit the complete original roster's fixed stratum assertions.
/// # Errors
/// Preserve actual nonzero-label or stratum admission failures.
fn strata() -> TestResult<[FixturePrincipalStratum; 2]> {
    Ok([
        FixturePrincipalStratum::new(label(11)?, 2)?,
        FixturePrincipalStratum::new(label(22)?, 2)?,
    ])
}

/// Seed the existing raw-source lifecycle without relabelling transformed values.
/// # Errors
/// Preserve actual raw enclosures, batch, parameters and retained-owner admission.
fn seed(
    entries: &[FixtureChartInterval<'_>; 2],
    strata: &[FixturePrincipalStratum; 2],
) -> TestResult<FixtureCombinationState> {
    let raw = [
        enclose_fixture_projection(entries[0].projection().source_projection())?,
        enclose_fixture_projection(entries[1].projection().source_projection())?,
    ];
    let batch = FixtureProjectedBatch::new(
        label(17)?,
        label(23)?,
        raw[0].projection().projected_at(),
        &raw,
    )?;
    Ok(FixtureCombinationState::seed(
        &[label(11)?, label(22)?],
        FixturePrincipalParameters::new(1.0 / 1024.0, 0.0)?,
        FixtureCombinationParameters::new(2.0, 0.0)?,
        FixtureMinimum::new(2)?,
        &batch,
        strata,
    )?)
}

/// Compare independent nonzero ideal-fit facts within the preexisting 2^-38 bound.
/// # Panics
/// Reject nonfinite values or an output inconsistent with the independent reference.
fn near(actual: f64, expected: f64) {
    assert!(actual.is_finite() && expected != 0.0);
    assert!((actual / expected - 1.0).abs() <= 1.0 / 274_877_906_944.0);
}

/// Inspect an actual first ready result without deriving expected means from it.
/// # Errors
/// Preserve missing actual estimate failures rather than supplying a fallback.
/// # Panics
/// Reject changed provenance, raw/target context, witness, source ages or fitted means.
fn check_first_ready(
    outcome: &FixtureChartCombinationOutcome<'_, '_, '_>,
    batch: &FixtureChartBatch<'_, '_>,
    original: &[FixtureChartInterval<'_>; 2],
) -> TestResult {
    assert_eq!(
        outcome.principal().kind(),
        FixturePrincipalOutcomeKind::ReferenceReady
    );
    assert_eq!(outcome.witness_count(), Some(2));
    assert_eq!(outcome.contributor_count(), Some(2));
    assert_eq!(outcome.emitted_round(), Some(label(1)?));
    assert_eq!(
        outcome.source_status(label(11)?),
        Some(FixtureCombinationSourceStatus::Principal)
    );
    assert_eq!(
        outcome.source_status(label(22)?),
        Some(FixtureCombinationSourceStatus::Contributor)
    );
    let estimate = outcome
        .combined()
        .ok_or_else(|| io::Error::other("missing actual chart estimate"))?;
    assert!(ptr::eq(outcome.principal().batch(), ptr::from_ref(batch)));
    assert!(ptr::eq(estimate.batch(), ptr::from_ref(batch)));
    assert!(ptr::eq(estimate.chart(), batch.chart()));
    assert!(ptr::eq(
        estimate.principal_entry(),
        ptr::from_ref(&original[0])
    ));
    assert_eq!(outcome.principal().entries().as_ptr(), original.as_ptr());
    assert_eq!(
        estimate.source_context(),
        FixtureContext::new(label(11)?, label(17)?, label(23)?)
    );
    assert_eq!(
        estimate.context(),
        FixtureContext::new(label(11)?, label(19)?, label(29)?)
    );
    assert_eq!(estimate.frame(), label(19)?);
    assert_eq!(estimate.mapping_generation(), label(29)?);
    assert_eq!(estimate.principal_label(), label(11)?);
    assert_eq!(estimate.supporter_count(), 2);
    assert_eq!(
        estimate.supporters().collect::<Vec<_>>(),
        [label(11)?, label(22)?]
    );
    assert_eq!(estimate.anchor_observed_at(), MonotonicNs::new(2 * SECOND));
    assert_eq!(estimate.latest_observed_at(), MonotonicNs::new(2 * SECOND));
    assert_eq!(estimate.fit_reference(), MonotonicNs::new(2 * SECOND));
    assert_eq!(estimate.projected_at(), MonotonicNs::new(4 * SECOND));
    near(estimate.phase_seconds(), 1013.0 / 4608.0);
    near(estimate.phase_rate_seconds_per_second(), 125.0 / 3072.0);
    assert!((estimate.phase_seconds() - 167.0 / 1536.0).abs() > 0.1);
    // At the fit reference, the independent variances are 5/324 and 1/144.
    // The tracking contract grows phase SD by rate SD times the two-second
    // fit-reference age. Equal distances add half the squared 1/16 phase gap;
    // rate and literal-skew precisions add to 288 and 80, respectively.
    let projected_phase_sd = (5.0_f64.sqrt() + 3.0) / 18.0;
    near(
        estimate.phase_standard_deviation_seconds(),
        (projected_phase_sd * projected_phase_sd + 1.0 / 512.0).sqrt(),
    );
    near(
        estimate.rate_standard_deviation_seconds_per_second(),
        (1.0_f64 / 288.0).sqrt(),
    );
    near(
        estimate.tracking_skew_seconds_per_second(),
        (1.0_f64 / 80.0).sqrt(),
    );
    for bound in [
        estimate.phase_mean_roundoff_seconds(),
        estimate.rate_mean_roundoff_seconds_per_second(),
    ] {
        assert!(bound.is_finite() && bound >= 0.0);
    }
    Ok(())
}

/// Genuine chart receiving preserves observation credit through reevaluation and replay.
/// # Errors
/// Preserve actual source, owner, view and full-lifecycle failures.
/// # Panics
/// Require genuine new observations, original state tags and exactly two emissions.
#[test]
fn chart_combination_retains_real_provenance_and_observation_credit() -> TestResult {
    let mut a = Source::new(11, [0, 2, 0])?;
    let b = Source::new(22, [1, 3, 1])?;
    let map = chart()?;
    let strata = strata()?;
    let initial = entries(&a, &b, &map, 4, 0.25, true)?;
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &initial)?;
    let mut owner = seed(&initial, &strata)?;
    let first = owner.advance_chart(&map, &batch, &strata)?;
    check_first_ready(&first, &batch, &initial)?;
    assert_eq!(owner.principal_state().frame(), label(17)?);
    assert_eq!(owner.principal_state().mapping_generation(), label(23)?);
    let later = entries(&a, &b, &map, 5, 0.25, true)?;
    let later_batch = FixtureChartBatch::new(&map, MonotonicNs::new(5 * SECOND), &later)?;
    let retained = owner.advance_chart(&map, &later_batch, &strata)?;
    assert_eq!(
        retained.principal().kind(),
        FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    );
    assert_eq!(retained.principal().updated_source(), None);
    assert_eq!(retained.emitted_round(), None);
    assert!(retained.combined().is_none());
    assert_eq!(owner.emitted_rounds(), 1);
    a.append(3, 0, 1.0)?;
    let fresh = entries(&a, &b, &map, 5, 0.25, true)?;
    let fresh_batch = FixtureChartBatch::new(&map, MonotonicNs::new(5 * SECOND), &fresh)?;
    let ready = owner.advance_chart(&map, &fresh_batch, &strata)?;
    assert_eq!(
        ready.principal().kind(),
        FixturePrincipalOutcomeKind::ReferenceReady
    );
    assert_eq!(ready.principal().updated_source(), Some(label(11)?));
    assert_eq!(ready.emitted_round(), Some(label(2)?));
    let replay = owner.advance_chart(&map, &fresh_batch, &strata)?;
    assert_eq!(
        replay.principal().kind(),
        FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    );
    assert_eq!(replay.emitted_round(), None);
    assert!(replay.combined().is_none());
    assert_eq!(owner.emitted_rounds(), 2);
    Ok(())
}

/// A missing later-excluded attachment rolls back the complete owner before retry.
/// # Errors
/// Preserve genuine producer, state and corrected-event failures.
/// # Panics
/// Reject partial observation/penalty/emission commits or duplicate replay credit.
#[test]
fn chart_combination_failure_rolls_back_before_same_event_retry() -> TestResult {
    let mut a = Source::new(11, [0, 2, 0])?;
    let b = Source::new(22, [1, 3, 1])?;
    let map = chart()?;
    let strata = strata()?;
    let initial = entries(&a, &b, &map, 4, 0.25, true)?;
    let initial_batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &initial)?;
    let mut owner = seed(&initial, &strata)?;
    let mut control = seed(&initial, &strata)?;
    owner.advance_chart(&map, &initial_batch, &strata)?;
    control.advance_chart(&map, &initial_batch, &strata)?;
    assert_eq!(owner, control);
    a.append(3, 0, 1.0)?;
    let missing = entries(&a, &b, &map, 5, 1.5, false)?;
    let missing_batch = FixtureChartBatch::new(&map, MonotonicNs::new(5 * SECOND), &missing)?;
    let error = owner
        .advance_chart(&map, &missing_batch, &strata)
        .err()
        .ok_or_else(|| io::Error::other("missing expected tracking rejection"))?;
    assert!(error.source().is_none());
    assert_eq!(format!("{error:?}"), "FixtureChartCombinationError { .. }");
    let FixtureChartCombinationError::MissingTracking { index } = &error else {
        return Err(Box::new(error));
    };
    assert_eq!(*index, 1);
    assert_eq!(owner, control);
    assert_eq!(owner.emitted_rounds(), 1);
    let corrected = entries(&a, &b, &map, 5, 1.5, true)?;
    let corrected_batch = FixtureChartBatch::new(&map, MonotonicNs::new(5 * SECOND), &corrected)?;
    let ready = owner.advance_chart(&map, &corrected_batch, &strata)?;
    assert_eq!(
        ready.principal().kind(),
        FixturePrincipalOutcomeKind::ReferenceReady
    );
    assert_eq!(ready.principal().updated_source(), Some(label(11)?));
    assert_eq!(ready.witness_count(), Some(2));
    assert_eq!(ready.contributor_count(), Some(1));
    assert_eq!(ready.emitted_round(), Some(label(2)?));
    assert_eq!(
        ready.source_status(label(22)?),
        Some(FixtureCombinationSourceStatus::DistanceExcluded)
    );
    control.advance_chart(&map, &corrected_batch, &strata)?;
    assert_eq!(owner, control);
    let penalty = owner
        .source_state(label(22)?)
        .ok_or_else(|| io::Error::other("missing B status"))?
        .penalty();
    let replay = owner.advance_chart(&map, &corrected_batch, &strata)?;
    assert_eq!(
        replay.principal().kind(),
        FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    );
    assert!(replay.combined().is_none());
    assert_eq!(replay.emitted_round(), None);
    assert_eq!(owner.emitted_rounds(), 2);
    assert_eq!(
        owner
            .source_state(label(22)?)
            .ok_or_else(|| io::Error::other("missing replay B status"))?
            .penalty(),
        penalty
    );
    assert_eq!(owner, control);
    Ok(())
}

/// Public readiness admission reuses genuine source and lifecycle producers.
#[path = "private/reference_input_readiness.rs"]
mod reference_input_readiness;

/// Complete descriptor-bound identity is enforced before actual lifecycle changes.
#[path = "private/chart_composition_lifecycle.rs"]
mod chart_composition_lifecycle;

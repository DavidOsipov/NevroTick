// SPDX-License-Identifier: GPL-2.0-only
//! Direct raw postcondition controls, as a private child of `lifecycle::chart`.
//!
//! Two genuine fixture populations own eighteen three-row histories and54
//! appends, twenty prepares, eighteen tracking projections/chart/raw enclosures,
//! two anchor projections, two combination seeds, three chart batches, two
//! combination advances and one direct genuine principal advance. The extra
//! anchor pair reuses a retained source; it creates no history, append or fit.
//! Raw estimates below deliberately use literal supported quantities: they are
//! unpublished postcondition candidates, never claims of validated fit math.
//! Checked histories, projections, batches, selections and decisions are genuine.

use std::{error::Error, io, num::NonZeroU64, ptr};

use garde::Validate;
use time_domain::{ActiveAwakeNs, MonotonicNs, UtcCoordinateQ32, exchange_arithmetic};

use crate::combination::admission::chart_receiving::{ChartTrackingSupporters, admit_chart_source};
use crate::combination::chart::ChartQuantities;
use crate::combination::{MAX_SUPPORTERS, Quantities};
use crate::{
    FixtureAdaptiveHistory, FixtureAffineChart, FixtureChartBatch, FixtureChartDomain,
    FixtureChartFrames, FixtureChartInterval, FixtureChartOutcome, FixtureCombinationError,
    FixtureCombinationParameters, FixtureCombinationState, FixtureContext, FixtureMinimum,
    FixtureObservation, FixturePrincipalOutcomeKind, FixturePrincipalParameters,
    FixturePrincipalStratum, FixtureProjectedBatch, FixtureProjectionTerms,
    enclose_fixture_chart_projection, enclose_fixture_projection,
};

use super::{
    ChartOutcomeInput, CheckedEstimateView, FixtureChartCombinationError,
    FixtureChartCombinationOutcome, FixtureChartCombinedEstimate, Inner, RawChartOutcome,
    RawContributors, chart_outcome_valid, combine_chart_supporters, contributor_count,
    estimate_valid, quantities_valid,
};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;
const SECOND: u64 = 1_000_000_000;

/// Admit only a supplied nonzero fixture identity, with no fallback or effects.
/// # Errors
/// Reject zero before constructing a source, roster or descriptor.
fn label(value: u64) -> TestResult<NonZeroU64> {
    NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero fixture label").into())
}

/// A genuine source owns exactly three actual exchange-derived observations.
struct Source {
    context: FixtureContext,
    history: FixtureAdaptiveHistory,
}

impl Source {
    /// Append the fixed nonzero-residual dyadic family in one raw context.
    /// # Errors
    /// Preserve the actual domain, exchange, observation, append and fit cause.
    fn new(id: u64) -> TestResult<Self> {
        let context = FixtureContext::new(label(id)?, label(17)?, label(23)?);
        let mut history = FixtureAdaptiveHistory::new(context, 3)?;
        for (second, value) in [0_u64, 1, 2].into_iter().zip([0_i128, 2, 0]) {
            let zero = UtcCoordinateQ32::new(0)?;
            let shifted = UtcCoordinateQ32::new(value << 28)?;
            let phase = exchange_arithmetic(zero, shifted, shifted, zero)?.offset();
            history.try_append(FixtureObservation::new(
                context,
                MonotonicNs::new(second * SECOND),
                phase,
                1.0,
            )?)?;
        }
        Ok(Self { context, history })
    }

    /// Project the actual fitted final anchor at4s through the borrowed chart.
    /// # Errors
    /// Preserve genuine preparation, tracking and enclosure failures.
    fn entry<'chart>(
        &self,
        chart: &'chart FixtureAffineChart,
    ) -> TestResult<FixtureChartInterval<'chart>> {
        let view = self
            .history
            .prepare_chart(self.context, FixtureChartOutcome::Known(chart))?
            .ok_or_else(|| io::Error::other("missing genuine chart view"))?;
        let projection = view.project_tracking(
            view.context(),
            2,
            MonotonicNs::new(2 * SECOND),
            MonotonicNs::new(4 * SECOND),
            FixtureProjectionTerms::new(1.0, 0.0, 0.125)?,
            0.125,
        )?;
        Ok(enclose_fixture_chart_projection(&projection)?)
    }
}

/// Propagate an asymmetric settled bound through one real retained zero anchor.
///
/// The existing three-row source has phases0,1/8,0 at0,1,2s and zero slope.
/// Both checked descriptors reuse its complete0..4s domain/reference and tags.
/// # Errors
/// Preserve typed awake-duration, descriptor, whole-history and producer failures.
/// # Panics
/// Reject lost coefficient error, changed original geometry or wrong output bits.
fn check_asymmetric_settled_transform(source: &Source, map: &FixtureAffineChart) -> TestResult {
    let duration = ActiveAwakeNs::new(0).duration_since(ActiveAwakeNs::new(0))?;
    let effect =
        FixtureAffineChart::settled_rate_effect(map.frames(), map.domain(), 0.0, duration)?;
    assert_eq!(effect.phase_at_reference_seconds().to_bits(), 0);
    assert_eq!(
        effect.rate_seconds_per_second().to_bits(),
        (-0.0_f64).to_bits()
    );
    assert_eq!(
        effect.phase_coefficient_roundoff_seconds().to_bits(),
        0x0028_0000_0000_0004
    );
    assert_eq!(
        effect
            .rate_coefficient_roundoff_seconds_per_second()
            .to_bits(),
        0
    );
    let exact = FixtureAffineChart::new(map.frames(), map.domain(), 0.0, -0.0)?;
    // Independent integer IEEE oracle, eta=2^-1022 and delta=2^-1074:
    // exact input:5*eta+20*delta; retained effect:8*eta+48*delta.
    // A changed OR-to-AND guard wrongly returns the first bound for both.
    for (chart, expected_bound) in [
        (&exact, 0x0034_0000_0000_0005),
        (&effect, 0x0040_0000_0000_0006),
    ] {
        let view = source
            .history
            .prepare_chart(source.context, FixtureChartOutcome::Known(chart))?
            .ok_or_else(|| io::Error::other("missing asymmetric settled view"))?;
        let value = view.project_anchor(
            view.context(),
            2,
            MonotonicNs::new(2 * SECOND),
            MonotonicNs::new(4 * SECOND),
            FixtureProjectionTerms::new(1.0, 0.0, 0.125)?,
        )?;
        let original = value.source_projection();
        assert_eq!(original.context(), source.context);
        assert_eq!(original.center_seconds().to_bits(), 0);
        assert_eq!(original.phase_rate_seconds_per_second().to_bits(), 0);
        assert_eq!(value.center_seconds().to_bits(), 0);
        assert_eq!(value.phase_rate_seconds_per_second().to_bits(), 0);
        assert_eq!(value.center_roundoff_seconds().to_bits(), expected_bound);
        assert!(ptr::eq(value.chart(), ptr::from_ref(chart)));
    }
    Ok(())
}

/// Admit one fixed map from the genuine source frame into a distinct target.
/// # Errors
/// Preserve checked frames, inclusive time domain or scalar failures.
fn chart() -> TestResult<FixtureAffineChart> {
    Ok(FixtureAffineChart::new(
        FixtureChartFrames::new(label(17)?, label(23)?, label(19)?, label(29)?)?,
        FixtureChartDomain::new(
            MonotonicNs::new(0),
            MonotonicNs::new(2 * SECOND),
            MonotonicNs::new(4 * SECOND),
        )?,
        4.0 / 1024.0,
        1.0 / 1024.0,
    )?)
}

/// Seed exactly the supplied genuine raw population and its actual pending credit.
/// # Errors
/// Reject any extent other than the two declared fixtures; preserve public causes.
fn seed(
    entries: &[FixtureChartInterval<'_>],
) -> TestResult<(FixtureCombinationState, Vec<FixturePrincipalStratum>)> {
    if !matches!(entries.len(), 2 | 16) {
        return Err(io::Error::other("unexpected fixture extent").into());
    }
    let raw = entries
        .iter()
        .map(|entry| enclose_fixture_projection(entry.projection().source_projection()))
        .collect::<Result<Vec<_>, _>>()?;
    let roster = raw
        .iter()
        .map(|entry| entry.projection().context().source_id())
        .collect::<Vec<_>>();
    let strata = roster
        .iter()
        .map(|id| FixturePrincipalStratum::new(*id, 2))
        .collect::<Result<Vec<_>, _>>()?;
    let batch =
        FixtureProjectedBatch::new(label(17)?, label(23)?, MonotonicNs::new(4 * SECOND), &raw)?;
    Ok((
        FixtureCombinationState::seed(
            &roster,
            FixturePrincipalParameters::new(1.0 / 1024.0, 0.0)?,
            FixtureCombinationParameters::new(2.0, 0.0)?,
            FixtureMinimum::new(entries.len())?,
            &batch,
            &strata,
        )?,
        strata,
    ))
}

/// Copy genuine provenance/prefix only; install explicit RAW representation facts.
///
/// These literals are supported scalar test inputs, not recomputed fitted results.
/// No checked estimate is published, no source is refitted, and nothing mutates.
fn raw_candidate<'batch, 'entries, 'chart>(
    genuine: &FixtureChartCombinedEstimate<'batch, 'entries, 'chart>,
) -> FixtureChartCombinedEstimate<'batch, 'entries, 'chart> {
    FixtureChartCombinedEstimate {
        batch: genuine.batch,
        principal: genuine.principal,
        labels: genuine.labels,
        count: genuine.count,
        quantities: raw_quantities(),
    }
}

/// Supply only explicit RAW scalar representations, with no fit-math assertion.
fn raw_quantities() -> ChartQuantities {
    ChartQuantities {
        points: Quantities {
            phase_seconds: 0.25,
            phase_rate_seconds_per_second: -0.125,
            phase_standard_deviation_seconds: 0.5,
            rate_standard_deviation_seconds_per_second: 0.25,
            tracking_skew_seconds_per_second: 0.125,
        },
        phase_error: 0.0,
        rate_error: 0.0,
    }
}

/// Check both direct helper contracts and the complete unpublished Garde carrier.
///
/// The actual ready outcome supplies unchanged decision/status/round/state facts;
/// only the supplied RAW estimate varies. Every call is read-only and bounded.
/// # Panics
/// Reject disagreement with the independent scalar/provenance expectations.
fn check_candidate(
    candidate: &FixtureChartCombinedEstimate<'_, '_, '_>,
    outcome: &FixtureChartCombinationOutcome<'_, '_, '_>,
    inner: &Inner,
    expected_quantities: bool,
    expected_estimate: bool,
) {
    assert_eq!(quantities_valid(candidate), expected_quantities);
    assert_eq!(
        estimate_valid(candidate, &outcome.principal),
        expected_estimate
    );
    let raw = RawChartOutcome {
        value: ChartOutcomeInput {
            principal: &outcome.principal,
            combined: Some(candidate),
            round: outcome.emitted_round,
            statuses: &outcome.sources,
            inner,
            previous_rounds: 0,
        },
    };
    assert_eq!(raw.validate().is_ok(), expected_estimate);
    assert_eq!(chart_outcome_valid(&raw.value), expected_estimate);
}

/// Challenge every scalar axis, keeping the other fields at legal RAW neighbors.
///
/// There are38 negative candidates, two signed-zero candidates and two positive
/// subnormal-error candidates. Each delegates exactly once to `check_candidate`.
/// # Panics
/// Require supported normal-or-zero points/SDs and finite nonnegative error bounds.
fn check_quantity_axes(
    genuine: &FixtureChartCombinedEstimate<'_, '_, '_>,
    outcome: &FixtureChartCombinationOutcome<'_, '_, '_>,
    inner: &Inner,
) {
    let tiny = f64::from_bits(1);
    for axis in 0..2 {
        for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, tiny, -tiny] {
            let mut raw = raw_candidate(genuine);
            if axis == 0 {
                raw.quantities.points.phase_seconds = invalid;
            } else {
                raw.quantities.points.phase_rate_seconds_per_second = invalid;
            }
            check_candidate(&raw, outcome, inner, false, false);
        }
    }
    for axis in 0..3 {
        for invalid in [
            -1.0,
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            tiny,
            -tiny,
        ] {
            let mut raw = raw_candidate(genuine);
            match axis {
                0 => raw.quantities.points.phase_standard_deviation_seconds = invalid,
                1 => {
                    raw.quantities
                        .points
                        .rate_standard_deviation_seconds_per_second = invalid;
                }
                _ => raw.quantities.points.tracking_skew_seconds_per_second = invalid,
            }
            check_candidate(&raw, outcome, inner, false, false);
        }
    }
    for axis in 0..2 {
        for invalid in [-1.0, -tiny, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut raw = raw_candidate(genuine);
            if axis == 0 {
                raw.quantities.phase_error = invalid;
            } else {
                raw.quantities.rate_error = invalid;
            }
            check_candidate(&raw, outcome, inner, false, false);
        }
    }
    for zero in [0.0, -0.0] {
        let mut raw = raw_candidate(genuine);
        raw.quantities.points = Quantities {
            phase_seconds: zero,
            phase_rate_seconds_per_second: zero,
            phase_standard_deviation_seconds: zero,
            rate_standard_deviation_seconds_per_second: zero,
            tracking_skew_seconds_per_second: zero,
        };
        raw.quantities.phase_error = zero;
        raw.quantities.rate_error = zero;
        check_candidate(&raw, outcome, inner, true, true);
    }
    for axis in 0..2 {
        let mut raw = raw_candidate(genuine);
        if axis == 0 {
            raw.quantities.phase_error = tiny;
        } else {
            raw.quantities.rate_error = tiny;
        }
        check_candidate(&raw, outcome, inner, true, true);
    }
}

/// Check actual principal provenance against a value-identical copied projection.
///
/// The RAW candidate retains the genuine principal reference. All five copied
/// metadata facts remain equal, while the distinct address must reject the copy.
/// No input is changed and no numerical producer or Garde validator runs.
/// # Panics
/// Require actual projection identity, equal copied metadata and copy rejection.
fn check_principal_projection_identity(candidate: &FixtureChartCombinedEstimate<'_, '_, '_>) {
    let original = candidate.principal.projection().source_projection();
    let copied = *original;
    assert!(!ptr::eq(original, ptr::from_ref(&copied)));
    assert_eq!(original.context(), copied.context());
    assert_eq!(original.anchor_observed_at(), copied.anchor_observed_at());
    assert_eq!(original.latest_observed_at(), copied.latest_observed_at());
    assert_eq!(original.fit_reference(), copied.fit_reference());
    assert_eq!(original.projected_at(), copied.projected_at());
    assert!(CheckedEstimateView::matches_principal(candidate, original));
    assert!(!CheckedEstimateView::matches_principal(candidate, &copied));
}

/// Genuine pair provenance supports raw scalar, identity, mask and selection faults.
///
/// Source census:2 histories/6 appends,2 prepares/tracking/chart/raw enclosures,
/// one seed,2 chart batches,2 combination advances,1 holder/admission,1 direct
/// rejected combine attempt,3 direct contributor Garde/typed checks,49 `check_candidate`
/// calls and2 direct `matches_principal` calls. No state clone or extra fit occurs.
/// # Errors
/// Preserve all genuine producer failures and missing actual checked outputs.
/// # Panics
/// Reject weakened pointer/scalar/prefix/mask predicates or reused selections.
#[test]
fn chart_raw_pair_guards_preserve_complete_relationships() -> TestResult {
    let sources = (1..=2).map(Source::new).collect::<TestResult<Vec<_>>>()?;
    let map = chart()?;
    check_asymmetric_settled_transform(
        sources
            .first()
            .ok_or_else(|| io::Error::other("missing asymmetric settled source"))?,
        &map,
    )?;
    let entries = sources
        .iter()
        .map(|source| source.entry(&map))
        .collect::<TestResult<Vec<_>>>()?;
    let (mut owner, strata) = seed(&entries)?;
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &entries)?;
    let ready = owner.advance_chart(&map, &batch, &strata)?;
    assert_eq!(
        ready.principal.kind(),
        FixturePrincipalOutcomeKind::ReferenceReady
    );
    let genuine = ready
        .combined
        .as_ref()
        .ok_or_else(|| io::Error::other("missing genuine pair estimate"))?;
    assert_eq!(genuine.count, 2);
    assert_eq!(genuine.principal_label(), label(1)?);
    let healthy = raw_candidate(genuine);
    check_candidate(&healthy, &ready, &owner.inner, true, true);

    check_principal_projection_identity(&healthy);

    check_quantity_axes(genuine, &ready, &owner.inner);
    let mut raw = raw_candidate(genuine);
    raw.count = 0;
    check_candidate(&raw, &ready, &owner.inner, true, false);
    let alias_batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &entries)?;
    assert!(!ptr::eq(ptr::from_ref(&alias_batch), ptr::from_ref(&batch)));
    let mut raw = raw_candidate(genuine);
    raw.batch = &alias_batch;
    check_candidate(&raw, &ready, &owner.inner, true, false);
    let copied_principal = *genuine.principal;
    assert!(!ptr::eq(
        genuine.principal,
        ptr::from_ref(&copied_principal)
    ));
    let mut raw = raw_candidate(genuine);
    raw.principal = &copied_principal;
    check_candidate(&raw, &ready, &owner.inner, true, false);
    let mut raw = raw_candidate(genuine);
    raw.labels[1] = raw.labels[0];
    check_candidate(&raw, &ready, &owner.inner, true, false);
    let mut raw = raw_candidate(genuine);
    raw.labels.swap(0, 1);
    check_candidate(&raw, &ready, &owner.inner, true, false);
    let mut raw = raw_candidate(genuine);
    raw.labels[1] = label(99)?;
    check_candidate(&raw, &ready, &owner.inner, true, false);

    let holder = ChartTrackingSupporters::from_decision(&ready.principal)?;
    let admitted = admit_chart_source(&ready.principal, &holder)?
        .ok_or_else(|| io::Error::other("missing genuine pair admission"))?;
    let mut mask = [false; MAX_SUPPORTERS];
    mask[0] = true;
    mask[1] = true;
    let healthy_mask = RawContributors {
        value: (&admitted, mask),
    };
    assert!(healthy_mask.validate().is_ok());
    assert_eq!(contributor_count(&healthy_mask.value), Ok(2));
    let mut padded = mask;
    padded[MAX_SUPPORTERS - 1] = true;
    let raw = RawContributors {
        value: (&admitted, padded),
    };
    assert!(raw.validate().is_err());
    assert_eq!(
        contributor_count(&raw.value),
        Err(FixtureCombinationError::Invariant)
    );
    let mut omitted = mask;
    omitted[0] = false;
    let raw = RawContributors {
        value: (&admitted, omitted),
    };
    assert!(raw.validate().is_err());
    assert_eq!(
        contributor_count(&raw.value),
        Err(FixtureCombinationError::Invariant)
    );
    let retained = owner.advance_chart(&map, &batch, &strata)?;
    assert_eq!(
        retained.principal.kind(),
        FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    );
    assert!(ptr::eq(ready.principal.batch(), retained.principal.batch()));
    assert!(!ptr::eq(
        ready.principal.selection(),
        retained.principal.selection()
    ));
    assert_eq!(
        combine_chart_supporters(&retained.principal, &admitted, mask).err(),
        Some(FixtureChartCombinationError::Invariant)
    );
    Ok(())
}

/// A raw count17 cannot acquire a seventeenth slot from a genuine16-source witness.
///
/// Source census:16 histories/48 appends,16 prepares/tracking/chart/raw enclosures,
/// one seed,one chart batch/direct principal advance,2 `quantities_valid` and2
/// `estimate_valid` calls. No combination reduction, round emission or completed
/// chart outcome is manufactured. The actual16-prefix is complete, sorted and
/// witnessed; only the raw count changes.
/// # Errors
/// Preserve real producer, seed, batch and completed-estimate failures.
/// # Panics
/// Reject iterator clipping as a substitute for the explicit raw count ceiling.
#[test]
fn chart_raw_count_seventeen_cannot_reuse_sixteen_slots() -> TestResult {
    let sources = (1..=16).map(Source::new).collect::<TestResult<Vec<_>>>()?;
    let map = chart()?;
    let entries = sources
        .iter()
        .map(|source| source.entry(&map))
        .collect::<TestResult<Vec<_>>>()?;
    let (mut owner, strata) = seed(&entries)?;
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &entries)?;
    let decision = owner.inner.principal.advance_chart(&map, &batch, &strata)?;
    assert_eq!(decision.kind(), FixturePrincipalOutcomeKind::ReferenceReady);
    let witness = decision
        .selection()
        .witnessed()
        .ok_or_else(|| io::Error::other("missing genuine16 witness"))?;
    assert_eq!(witness.supporters().len(), 16);
    let mut labels = [NonZeroU64::MIN; MAX_SUPPORTERS];
    for (index, label) in witness.supporters().enumerate() {
        labels[index] = label;
    }
    let mut raw = FixtureChartCombinedEstimate {
        batch: &batch,
        principal: decision
            .principal_entry()
            .ok_or_else(|| io::Error::other("missing genuine16 principal"))?,
        labels,
        count: 16,
        quantities: raw_quantities(),
    };
    assert!(raw.supporters().eq((1..=16).filter_map(NonZeroU64::new)));
    assert!(quantities_valid(&raw));
    assert!(estimate_valid(&raw, &decision));
    raw.count = MAX_SUPPORTERS + 1;
    assert_eq!(raw.supporters().len(), 16);
    assert!(quantities_valid(&raw));
    assert!(!estimate_valid(&raw, &decision));
    Ok(())
}

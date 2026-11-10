// SPDX-License-Identifier: GPL-2.0-only
//! Private geometry and retained-state contracts over checked chart descriptors.
//!
//! Model cases use real histories, producer advancement and reference admission.
//! Direct rebase calls intentionally challenge the component's descriptor guard;
//! they do not represent physical delivery or manufacture a public completion.

use std::{error::Error, fmt, io, num::NonZeroU64};

use time_domain::{MonotonicNs, UtcCoordinateQ32, exchange_arithmetic};

use super::{
    Basis, FixtureFrequencyAdmission, FixtureFrequencyClockAssociation, FixtureFrequencyError,
    FixtureFrequencyHistory, FixtureFrequencyProfile, FrequencyState, Relationship,
    checked_relationship, contains, query_domain, same_domain,
};
use crate::{
    FixtureAdaptiveHistory, FixtureAffineChart, FixtureChartBatch, FixtureChartDomain,
    FixtureChartError, FixtureChartFrames, FixtureChartInterval, FixtureChartOutcome,
    FixtureCombinationParameters, FixtureCombinationState, FixtureContext, FixtureMinimum,
    FixtureObservation, FixturePhaseRetirementReason, FixturePrincipalParameters,
    FixturePrincipalStratum, FixtureProjectedBatch, FixtureProjectionTerms,
    FixtureRateClockContext, FixtureRateSettlementCursor, FixtureReferenceInputState,
    enclose_fixture_chart_projection, enclose_fixture_projection,
};

/// Preserve actual typed setup failures without fallback product values.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Explicit fixture nanoseconds per second, unrelated to host elapsed time.
const SECOND: u64 = 1_000_000_000;

/// Require success of independently supported pure product inputs.
/// # Errors
/// Preserve the original bounded refusal after the semantic assertion.
/// # Panics
/// Reject a product refusal of an independently valid fixture.
#[track_caller]
fn product_ok<T, E: fmt::Debug + fmt::Display>(value: Result<T, E>) -> TestResult<T> {
    assert!(
        value.is_ok(),
        "checked frequency-state fixture must succeed: {:?}",
        value.as_ref().err().map(ToString::to_string)
    );
    value.map_err(|error| io::Error::other(format!("frequency-state fixture: {error:?}")).into())
}

/// Admit a literal identity instead of replacing an invalid zero label.
/// # Errors
/// Reject a zero fixture label.
fn label(value: u64) -> TestResult<NonZeroU64> {
    NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero state fixture label").into())
}

/// Keep active and monotonic origins distinct in the declared fixture.
/// # Errors
/// Preserve literal nonzero-label admission.
fn clocks() -> TestResult<FixtureRateClockContext> {
    Ok(FixtureRateClockContext::new(
        label(101)?,
        label(103)?,
        label(107)?,
    ))
}

/// Associate the original source with the independently supplied monotonic axis.
/// # Errors
/// Preserve literal label admission; no physical relationship is inferred.
fn association() -> TestResult<FixtureFrequencyClockAssociation> {
    Ok(FixtureFrequencyClockAssociation::new(
        label(17)?,
        label(23)?,
        label(101)?,
        label(107)?,
    ))
}

/// Build a domain through the actual inclusive-order and exact-width owner.
/// # Errors
/// Preserve checked domain rejection.
/// # Panics
/// Reject refusal of the independently supported domain literals.
fn domain(times: [u64; 3]) -> TestResult<FixtureChartDomain> {
    product_ok(FixtureChartDomain::new(
        MonotonicNs::new(times[0]),
        MonotonicNs::new(times[1]),
        MonotonicNs::new(times[2]),
    ))
}

/// Build checked frame identities and immutable scalar geometry together.
/// # Errors
/// Preserve checked frame and chart construction errors.
/// # Panics
/// Reject refusal of the independently supported chart inputs.
fn chart(
    tags: [u64; 4],
    support: FixtureChartDomain,
    phase: f64,
    rate: f64,
) -> TestResult<FixtureAffineChart> {
    product_ok(FixtureAffineChart::new(
        product_ok(FixtureChartFrames::new(
            label(tags[0])?,
            label(tags[1])?,
            label(tags[2])?,
            label(tags[3])?,
        ))?,
        support,
        phase,
        rate,
    ))
}

/// Make the exact predecessor used for genuine source intake.
/// # Errors
/// Preserve checked domain and chart admission.
/// # Panics
/// Reject refusal of the zero-valued checked map.
fn predecessor(reference: u64) -> TestResult<FixtureAffineChart> {
    chart(
        [17, 23, 19, 29],
        domain([0, reference, 64 * SECOND])?,
        0.0,
        0.0,
    )
}

/// One independently chosen input to the real coefficient-composition path.
#[derive(Clone, Copy)]
struct Geometry {
    /// Source and final target frame/generation tags; the intermediate is fixed.
    tags: [u64; 4],
    /// Actual inclusive support and independent common reference.
    times: [u64; 3],
    /// Separate left/right phase operands in seconds.
    phases: [f64; 2],
    /// Separate left/right rate operands in seconds per second.
    rates: [f64; 2],
}

/// Exact cancellations with genuine nonzero composition error bounds.
const GEOMETRY: Geometry = Geometry {
    tags: [17, 23, 31, 7],
    times: [0, 2 * SECOND, 64 * SECOND],
    phases: [1.0 / 64.0, -1.0 / 64.0],
    rates: [1.0 / 2048.0, -1.0 / 2048.0],
};

/// Compose two checked maps instead of forging nonzero coefficient errors.
/// # Errors
/// Preserve all real chart/domain and composition failures.
/// # Panics
/// Reject refusal of a valid chain with identical domains.
fn composed(value: Geometry) -> TestResult<FixtureAffineChart> {
    let support = domain(value.times)?;
    let left = chart(
        [value.tags[0], value.tags[1], 19, 29],
        support,
        value.phases[0],
        value.rates[0],
    )?;
    let right = chart(
        [19, 29, value.tags[2], value.tags[3]],
        support,
        value.phases[1],
        value.rates[1],
    )?;
    product_ok(left.compose(&right))
}

/// Read six independent geometry components without using the matching helper.
fn differences(left: &FixtureAffineChart, right: &FixtureAffineChart) -> [bool; 6] {
    [
        left.frames() != right.frames(),
        left.domain() != right.domain(),
        left.phase_at_reference_seconds().to_bits() != right.phase_at_reference_seconds().to_bits(),
        left.rate_seconds_per_second().to_bits() != right.rate_seconds_per_second().to_bits(),
        left.phase_coefficient_roundoff_seconds().to_bits()
            != right.phase_coefficient_roundoff_seconds().to_bits(),
        left.rate_coefficient_roundoff_seconds_per_second()
            .to_bits()
            != right
                .rate_coefficient_roundoff_seconds_per_second()
                .to_bits(),
    ]
}

/// Compare the whole basis while independently changing each checked component.
/// # Errors
/// Preserve checked chart construction and composition failures.
/// # Panics
/// Reject wrong single-component geometry, failed cancellation or loose matching.
#[test]
fn basis_matches_requires_each_checked_geometry_component() -> TestResult {
    let base = composed(GEOMETRY)?;
    let equal = composed(GEOMETRY)?;
    let basis = Basis::from_chart(&base);
    assert!(basis.matches(&base));
    assert!(basis.matches(&equal));
    assert!(same_domain(base.domain(), equal.domain()));
    assert_eq!(
        base.phase_at_reference_seconds().to_bits(),
        0.0_f64.to_bits()
    );
    assert_eq!(base.rate_seconds_per_second().to_bits(), 0.0_f64.to_bits());
    assert!(base.phase_coefficient_roundoff_seconds() > 0.0);
    assert!(base.rate_coefficient_roundoff_seconds_per_second() > 0.0);
    for (index, tag) in [(0, 41), (1, 47), (2, 37), (3, 53)] {
        let mut inputs = GEOMETRY;
        inputs.tags[index] = tag;
        check_geometry_change(&base, inputs, 0, [0.0, 0.0])?;
    }
    for times in [
        [SECOND, 2 * SECOND, 64 * SECOND],
        [0, 3 * SECOND, 64 * SECOND],
        [0, 2 * SECOND, 63 * SECOND],
    ] {
        check_geometry_change(&base, Geometry { times, ..GEOMETRY }, 1, [0.0, 0.0])?;
    }
    for (inputs, component, phase, rate) in [
        (
            Geometry {
                phases: [1.0 / 64.0, 1.0 / 64.0],
                ..GEOMETRY
            },
            2,
            1.0 / 32.0,
            0.0,
        ),
        (
            Geometry {
                rates: [1.0 / 2048.0, 1.0 / 2048.0],
                ..GEOMETRY
            },
            3,
            0.0,
            1.0 / 1024.0,
        ),
        (
            Geometry {
                phases: [1.0 / 32.0, -1.0 / 32.0],
                ..GEOMETRY
            },
            4,
            0.0,
            0.0,
        ),
        (
            Geometry {
                rates: [1.0 / 1024.0, -1.0 / 1024.0],
                ..GEOMETRY
            },
            5,
            0.0,
            0.0,
        ),
    ] {
        check_geometry_change(&base, inputs, component, [phase, rate])?;
    }
    check_signed_zero_basis()?;
    Ok(())
}

/// Verify the independent single-component premise before challenging identity.
/// # Errors
/// Preserve actual checked composition failures.
/// # Panics
/// Reject a coupled change, wrong exact scalar or accepted foreign geometry.
fn check_geometry_change(
    base: &FixtureAffineChart,
    inputs: Geometry,
    component: usize,
    points: [f64; 2],
) -> TestResult {
    let candidate = composed(inputs)?;
    let mut expected = [false; 6];
    expected[component] = true;
    assert_eq!(differences(base, &candidate), expected);
    assert_eq!(candidate.phase_at_reference_seconds(), points[0]);
    assert_eq!(candidate.rate_seconds_per_second(), points[1]);
    assert_eq!(
        same_domain(base.domain(), candidate.domain()),
        component != 1
    );
    if component == 4 {
        assert!(
            candidate.phase_coefficient_roundoff_seconds()
                > base.phase_coefficient_roundoff_seconds()
        );
    }
    if component == 5 {
        assert!(
            candidate.rate_coefficient_roundoff_seconds_per_second()
                > base.rate_coefficient_roundoff_seconds_per_second()
        );
    }
    assert!(!Basis::from_chart(base).matches(&candidate));
    Ok(())
}

/// Signed zero is supported geometry whose stored coefficient bits still matter.
/// # Errors
/// Preserve checked chart/domain construction failures.
/// # Panics
/// Reject normalization of an independently signed-zero coefficient or loose identity.
fn check_signed_zero_basis() -> TestResult {
    let support = domain(GEOMETRY.times)?;
    let zero = chart(GEOMETRY.tags, support, 0.0, 0.0)?;
    for (phase, rate, component) in [(-0.0_f64, 0.0_f64, 2), (0.0, -0.0, 3)] {
        let candidate = chart(GEOMETRY.tags, support, phase, rate)?;
        let mut expected = [false; 6];
        expected[component] = true;
        assert_eq!(differences(&zero, &candidate), expected);
        assert!(!Basis::from_chart(&zero).matches(&candidate));
    }
    Ok(())
}

/// Inclusive domain support and query order are independent component contracts.
/// # Errors
/// Preserve checked domain admission.
/// # Panics
/// Reject accepted exterior coordinates, excluded endpoints or wrong refusal order.
#[test]
fn checked_domain_boundaries_and_query_refusal_order_are_exact() -> TestResult {
    let start = SECOND;
    let reference = 2 * SECOND;
    let end = 64 * SECOND;
    let support = domain([start, reference, end])?;
    for (at, expected) in [
        (start - 1, false),
        (start, true),
        (reference, true),
        (end, true),
        (end + 1, false),
    ] {
        assert_eq!(contains(support, MonotonicNs::new(at)), expected);
    }
    for (reference, now) in [(start, start), (2 * SECOND, 2 * SECOND), (start, end)] {
        assert_eq!(
            query_domain(support, MonotonicNs::new(reference), MonotonicNs::new(now)),
            Ok(())
        );
    }
    for (reference, now) in [
        (reference, end + 1),
        (start - 1, start),
        (start - 1, end + 1),
    ] {
        assert_eq!(
            query_domain(support, MonotonicNs::new(reference), MonotonicNs::new(now)),
            Err(FixtureFrequencyError::QueryOutsideDomain)
        );
    }
    for now in [start, start - 1] {
        assert_eq!(
            query_domain(support, MonotonicNs::new(reference), MonotonicNs::new(now)),
            Err(FixtureFrequencyError::QueryBeforeReference)
        );
    }
    let singleton = domain([reference; 3])?;
    assert!(contains(singleton, MonotonicNs::new(reference)));
    for at in [reference - 1, reference + 1] {
        assert!(!contains(singleton, MonotonicNs::new(at)));
    }
    assert_eq!(
        query_domain(
            singleton,
            MonotonicNs::new(reference),
            MonotonicNs::new(reference)
        ),
        Ok(())
    );
    assert_eq!(
        query_domain(
            singleton,
            MonotonicNs::new(reference),
            MonotonicNs::new(reference + 1)
        ),
        Err(FixtureFrequencyError::QueryOutsideDomain)
    );
    Ok(())
}

/// Both independently supplied source coordinates must match the real chart.
/// # Errors
/// Preserve checked chart/context construction failures.
/// # Panics
/// Reject a missing source-coordinate check or a refused matching neighbor.
#[test]
fn checked_relationship_rejects_each_supplied_source_coordinate() -> TestResult {
    let chart = predecessor(2 * SECOND)?;
    let association = association()?;
    let context = clocks()?;
    for source in [
        FixtureContext::new(label(11)?, label(999)?, label(23)?),
        FixtureContext::new(label(11)?, label(17)?, label(999)?),
    ] {
        assert_eq!(
            checked_relationship(Relationship {
                association,
                chart: &chart,
                context,
                source: Some(source)
            }),
            Err(FixtureFrequencyError::ClockAssociationMismatch)
        );
    }
    for source in [
        None,
        Some(FixtureContext::new(label(11)?, label(17)?, label(23)?)),
    ] {
        assert_eq!(
            checked_relationship(Relationship {
                association,
                chart: &chart,
                context,
                source
            }),
            Ok(())
        );
    }
    Ok(())
}

/// Project actual three-point evidence with literal mean 1/32 and zero slope.
/// # Errors
/// Preserve real exchange, fit, mapping and projection failures.
/// # Panics
/// Reject absent fitted evidence or refusal of the independently supported inputs.
fn entry(chart: &FixtureAffineChart) -> TestResult<FixtureChartInterval<'_>> {
    let context = FixtureContext::new(label(11)?, label(17)?, label(23)?);
    let mut history = product_ok(FixtureAdaptiveHistory::new(context, 3))?;
    for (second, offset) in [(0, 3_i128 << 26), (1, 0), (2, 3_i128 << 26)] {
        let zero = product_ok(UtcCoordinateQ32::new(0))?;
        let shifted = product_ok(UtcCoordinateQ32::new(offset))?;
        let phase = product_ok(exchange_arithmetic(zero, shifted, shifted, zero))?.offset();
        product_ok(history.try_append(product_ok(FixtureObservation::new(
            context,
            MonotonicNs::new(second * SECOND),
            phase,
            1.0,
        ))?))?;
    }
    let prepared = product_ok(history.prepare_chart(context, FixtureChartOutcome::Known(chart)))?;
    assert!(
        prepared.is_some(),
        "genuine fitted history must produce a chart view"
    );
    let Some(view) = prepared else {
        return Err(io::Error::other("missing fitted chart view").into());
    };
    assert_eq!(history.reference(), Some(MonotonicNs::new(2 * SECOND)));
    let projection = product_ok(view.project_tracking(
        view.context(),
        2,
        MonotonicNs::new(2 * SECOND),
        MonotonicNs::new(4 * SECOND),
        product_ok(FixtureProjectionTerms::new(0.25, 0.0, 0.125))?,
        0.125,
    ))?;
    product_ok(enclose_fixture_chart_projection(&projection))
}

/// Initialize frequency state only from an actual producer emission and token.
/// # Errors
/// Preserve checked producer, reference, configuration and cursor admission.
/// # Panics
/// Reject missing genuine round/model or wrong literal anchor/provenance.
fn admitted(chart: &FixtureAffineChart) -> TestResult<FrequencyState> {
    let entries = [entry(chart)?];
    let original = [product_ok(enclose_fixture_projection(
        entries[0].projection().source_projection(),
    ))?];
    let strata = [product_ok(FixturePrincipalStratum::new(label(11)?, 2))?];
    let batch = product_ok(FixtureProjectedBatch::new(
        label(17)?,
        label(23)?,
        MonotonicNs::new(4 * SECOND),
        &original,
    ))?;
    let mut producer = product_ok(FixtureCombinationState::seed(
        &[label(11)?],
        product_ok(FixturePrincipalParameters::new(1.0 / 1024.0, 0.0))?,
        product_ok(FixtureCombinationParameters::new(2.0, 0.0))?,
        product_ok(FixtureMinimum::new(1))?,
        &batch,
        &strata,
    ))?;
    let batch = product_ok(FixtureChartBatch::new(
        chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let outcome = product_ok(producer.advance_chart(chart, &batch, &strata))?;
    let readiness = FixtureReferenceInputState::new(chart);
    let token = product_ok(readiness.admit_reference_input(&outcome))?;
    assert_eq!(token.emitted_round(), label(1)?);
    let cursor = FixtureRateSettlementCursor::new(chart, clocks()?, label(1)?);
    let profile = product_ok(FixtureFrequencyProfile::new(
        1.0,
        0.0,
        1.0,
        product_ok(MonotonicNs::new(32 * SECOND).duration_since(MonotonicNs::new(0)))?,
    ))?;
    let mut state = product_ok(FrequencyState::enabled(
        profile,
        association()?,
        chart,
        &cursor,
    ))?;
    state.assimilate(
        token.combined(),
        token.emitted_round(),
        clocks()?,
        MonotonicNs::new(4 * SECOND),
    );
    assert_eq!(
        state.admission(),
        FixtureFrequencyAdmission::Accepted {
            emitted_round: label(1)?
        }
    );
    let saved = history(&state)?;
    assert_eq!(saved.anchor_phase_seconds(), 1.0 / 32.0);
    assert_eq!(saved.projected_at(), MonotonicNs::new(4 * SECOND));
    assert_eq!(saved.latest_observed_at(), MonotonicNs::new(2 * SECOND));
    Ok(state)
}

/// Extract retained history only after asserting the genuine model exists.
/// # Errors
/// Preserve the missing-history context after the primary semantic assertion.
/// # Panics
/// Reject loss of a genuinely admitted complete model.
fn history(state: &FrequencyState) -> TestResult<FixtureFrequencyHistory> {
    let saved = state.history();
    assert!(
        saved.is_some(),
        "genuine frequency model history must be retained"
    );
    saved.ok_or_else(|| io::Error::other("missing genuine model history").into())
}

/// Complete immutable record observation, with bit-level numerical provenance.
#[derive(Debug, Eq, PartialEq)]
struct SavedRecord {
    /// Prior point/error pairs, anchor point/error and original statistical SDs.
    numbers: [u64; 10],
    /// Complete current mathematical coefficient bits.
    coefficients: [u64; 4],
    /// Original genuine source identity.
    source: FixtureContext,
    /// Current and original reference mathematical frame tags.
    frames: [FixtureChartFrames; 2],
    /// Original round, observed anchor/latest/fit/projected and current domain.
    epochs: [u64; 8],
}

/// Copy a complete record without modifying any model or authority state.
fn saved_record(value: &FixtureFrequencyHistory) -> SavedRecord {
    let model = value.model;
    SavedRecord {
        numbers: [
            model.prior.mean().to_bits(),
            model.prior.mean_error().to_bits(),
            model.prior.skew().to_bits(),
            model.prior.skew_error().to_bits(),
            model.prior.residual().to_bits(),
            model.prior.residual_error().to_bits(),
            model.anchor.phase().to_bits(),
            model.anchor.error().to_bits(),
            model.phase_sd.to_bits(),
            model.rate_sd.to_bits(),
        ],
        coefficients: [
            model.basis.phase,
            model.basis.rate,
            model.basis.phase_error,
            model.basis.rate_error,
        ],
        source: model.source,
        frames: [model.basis.frames, model.reference_frames],
        epochs: [
            model.round.get(),
            model.observed_anchor.nanoseconds(),
            model.latest.nanoseconds(),
            model.fit.nanoseconds(),
            model.projected.nanoseconds(),
            model.basis.domain.start().nanoseconds(),
            model.basis.domain.common_reference().nanoseconds(),
            model.basis.domain.end().nanoseconds(),
        ],
    }
}

/// Ordinary checked descriptors for independently challenging one rebase guard.
struct RebaseCharts {
    /// Actual component predecessor; the genuine model is separately admitted.
    predecessor: FixtureAffineChart,
    /// Checked actual-effect descriptor, never a physical acknowledgment.
    effect: FixtureAffineChart,
    /// Checked replacement descriptor for the component's relationship check.
    replacement: FixtureAffineChart,
}

/// Make the healthy dyadic triple, retaining the selected common reference.
/// # Errors
/// Preserve checked chart construction failures.
/// # Panics
/// Reject refusal of valid descriptor geometry.
fn rebase_charts(reference: u64) -> TestResult<RebaseCharts> {
    let support = domain([0, reference, 64 * SECOND])?;
    let predecessor = predecessor(reference)?;
    let effect = chart([19, 29, 31, 7], support, 5.0 / 512.0, 0.0)?;
    let replacement = product_ok(predecessor.compose(&effect))?;
    Ok(RebaseCharts {
        predecessor,
        effect,
        replacement,
    })
}

/// Construct a zero point-rate effect with genuine positive rate-coefficient error.
/// # Errors
/// Preserve real chart composition and numerical admission failures.
/// # Panics
/// Reject failed cancellation or absent constructor-owned rate uncertainty.
fn rate_error_effect(support: FixtureChartDomain) -> TestResult<FixtureAffineChart> {
    let left = chart([19, 29, 37, 41], support, 0.0, 1.0 / 1024.0)?;
    let right = chart([37, 41, 31, 7], support, 5.0 / 512.0, -1.0 / 1024.0)?;
    let effect = product_ok(left.compose(&right))?;
    assert_eq!(effect.rate_seconds_per_second(), 0.0);
    assert!(effect.rate_coefficient_roundoff_seconds_per_second() > 0.0);
    assert_eq!(effect.phase_at_reference_seconds(), 5.0 / 512.0);
    Ok(effect)
}

/// Derive one isolated A–P relationship failure from checked constructors.
/// # Errors
/// Preserve descriptor/domain admission failures or refuse an unknown row.
/// # Panics
/// Reject refusal of valid individual descriptors.
fn invalid_rebase(row: char) -> TestResult<RebaseCharts> {
    let mut values = rebase_charts(2 * SECOND)?;
    let support = values.predecessor.domain();
    match row {
        'A' => {
            values.predecessor = chart([17, 23, 19, 29], support, 1.0 / 1024.0, 0.0)?;
            // Keep the foreign predecessor's actual composed coefficient bounds;
            // a separately reconstructed exact chart would mask the identity guard.
            values.replacement =
                product_ok(values.predecessor.compose_intersection(&values.effect))?;
        }
        'B'..='G' => replace_rebase_tags(row, &mut values)?,
        'H' => values.effect = chart([19, 29, 31, 7], support, 5.0 / 512.0, 1.0 / 1024.0)?,
        'I' => values.effect = rate_error_effect(support)?,
        'J' => values.replacement = chart([17, 23, 31, 7], support, 5.0 / 512.0, 1.0 / 1024.0)?,
        'K'..='N' => replace_rebase_domain(row, &mut values)?,
        'O' => {
            let short = domain([0, 2 * SECOND, 3 * SECOND])?;
            values.effect = chart([19, 29, 31, 7], short, 5.0 / 512.0, 0.0)?;
            values.replacement = chart([17, 23, 31, 7], short, 5.0 / 512.0, 0.0)?;
        }
        'P' => {
            assert!(values.replacement.phase_coefficient_roundoff_seconds() > 0.0);
            values.replacement = chart([17, 23, 31, 7], support, 5.0 / 512.0, 0.0)?;
        }
        _ => return Err(io::Error::other("unknown reviewed rebase row").into()),
    }
    Ok(values)
}

/// Vary one frame/generation relationship through the actual checked constructor.
/// # Errors
/// Preserve descriptor admission failures or refuse an unknown row.
/// # Panics
/// Reject refusal of a valid individual descriptor.
fn replace_rebase_tags(row: char, values: &mut RebaseCharts) -> TestResult {
    let (effect, tags) = match row {
        'B' => (true, [37, 29, 31, 7]),
        'C' => (true, [19, 43, 31, 7]),
        'D' => (false, [37, 23, 31, 7]),
        'E' => (false, [17, 43, 31, 7]),
        'F' => (false, [17, 23, 37, 7]),
        'G' => (false, [17, 23, 31, 43]),
        _ => return Err(io::Error::other("unknown reviewed rebase tag row").into()),
    };
    let changed = chart(tags, values.predecessor.domain(), 5.0 / 512.0, 0.0)?;
    if effect {
        values.effect = changed;
    } else {
        values.replacement = changed;
    }
    Ok(())
}

/// Vary one reference/intersection relationship with a checked inclusive domain.
/// # Errors
/// Preserve domain/chart admission failures or refuse an unknown row.
/// # Panics
/// Reject refusal of a valid individual descriptor.
fn replace_rebase_domain(row: char, values: &mut RebaseCharts) -> TestResult {
    let times = match row {
        'K' | 'L' => [0, 3 * SECOND, 64 * SECOND],
        'M' => [SECOND, 2 * SECOND, 64 * SECOND],
        'N' => [0, 2 * SECOND, 63 * SECOND],
        _ => return Err(io::Error::other("unknown reviewed rebase domain row").into()),
    };
    if row == 'K' {
        values.effect = chart([19, 29, 31, 7], domain(times)?, 5.0 / 512.0, 0.0)?;
    } else {
        values.replacement = chart([17, 23, 31, 7], domain(times)?, 5.0 / 512.0, 0.0)?;
    }
    Ok(())
}

/// Failure must preserve the complete old record before any query can mask it.
/// # Errors
/// Preserve genuine intake, saved-history and clock fixture construction failures.
/// # Panics
/// Reject partial model publication, wrong anchor or a missing sticky refusal.
fn check_rebase_refusal(original: &FixtureAffineChart, values: &RebaseCharts) -> TestResult {
    let mut state = admitted(original)?;
    let before = saved_record(&history(&state)?);
    state.rebase(
        &values.predecessor,
        Ok(Some(&values.effect)),
        Ok(Some(&values.replacement)),
    );
    let after = history(&state)?;
    assert_eq!(after.anchor_phase_seconds(), 1.0 / 32.0);
    assert_eq!(saved_record(&after), before);
    assert!(matches!(
        state.project(original, clocks()?, MonotonicNs::new(4 * SECOND), false),
        Err(FixtureFrequencyError::AnchorRebaseUnavailable)
    ));
    assert_eq!(saved_record(&history(&state)?), before);
    Ok(())
}

/// Successful re-expression changes the anchor/basis and retains original evidence.
/// # Errors
/// Preserve genuine intake, checked projection and saved-history failures.
/// # Panics
/// Reject wrong dyadic anchor, lost bounds or altered original provenance.
fn check_rebase_success(original: &FixtureAffineChart, values: &RebaseCharts) -> TestResult {
    let mut state = admitted(original)?;
    let before = history(&state)?;
    state.rebase(
        &values.predecessor,
        Ok(Some(&values.effect)),
        Ok(Some(&values.replacement)),
    );
    let after = history(&state)?;
    assert_eq!(after.anchor_phase_seconds(), 11.0 / 512.0);
    assert!(after.anchor_roundoff_seconds() >= before.anchor_roundoff_seconds());
    assert_eq!(after.frames(), values.replacement.frames());
    assert_eq!(after.domain(), values.replacement.domain());
    let mut expected = saved_record(&before);
    let actual = saved_record(&after);
    expected.numbers[6] = (11.0_f64 / 512.0).to_bits();
    // The numerical kernel owns the new conservative bound, checked above.
    expected.numbers[7] = actual.numbers[7];
    expected.coefficients = [
        values.replacement.phase_at_reference_seconds().to_bits(),
        values.replacement.rate_seconds_per_second().to_bits(),
        values
            .replacement
            .phase_coefficient_roundoff_seconds()
            .to_bits(),
        values
            .replacement
            .rate_coefficient_roundoff_seconds_per_second()
            .to_bits(),
    ];
    assert_eq!(actual.coefficients, expected.coefficients);
    expected.frames[0] = values.replacement.frames();
    expected.epochs[5..].copy_from_slice(&[
        values.replacement.domain().start().nanoseconds(),
        values.replacement.domain().common_reference().nanoseconds(),
        values.replacement.domain().end().nanoseconds(),
    ]);
    assert_eq!(actual, expected);
    let projection = product_ok(state.project(
        &values.replacement,
        clocks()?,
        MonotonicNs::new(4 * SECOND),
        false,
    ))?;
    assert_eq!(projection.nominal_phase_seconds(), 11.0 / 512.0);
    assert_eq!(saved_record(projection.history()), saved_record(&after));
    Ok(())
}

/// Each guarded descriptor relationship is necessary independently of the others.
/// # Errors
/// Preserve all actual chart and genuine model construction failures.
/// # Panics
/// Reject partial publication or an admitted single-relationship violation.
#[test]
fn rebase_checks_each_descriptor_relationship_and_preserves_whole_history() -> TestResult {
    let original = predecessor(2 * SECOND)?;
    for row in 'A'..='P' {
        check_rebase_refusal(&original, &invalid_rebase(row)?)?;
        check_rebase_success(&original, &rebase_charts(2 * SECOND)?)?;
    }
    Ok(())
}

/// Support of the genuine projected epoch is inclusive on both sides.
/// Private component descriptors may restrict the original mathematical domain;
/// these checked operands do not assert a public or physical effect receipt.
/// # Errors
/// Preserve genuine histories and checked descriptor construction failures.
/// # Panics
/// Reject an excluded model epoch or a refused exact supported endpoint.
#[test]
fn rebase_projected_epoch_support_has_both_inclusive_neighbors() -> TestResult {
    for (reference, invalid, valid) in [
        (
            2 * SECOND,
            [0, 2 * SECOND, 3 * SECOND],
            [0, 2 * SECOND, 4 * SECOND],
        ),
        (
            8 * SECOND,
            [5 * SECOND, 8 * SECOND, 64 * SECOND],
            [4 * SECOND, 8 * SECOND, 64 * SECOND],
        ),
    ] {
        let original = predecessor(reference)?;
        for (times, accepted) in [(invalid, false), (valid, true)] {
            let mut values = rebase_charts(reference)?;
            let support = domain(times)?;
            values.effect = chart([19, 29, 31, 7], support, 5.0 / 512.0, 0.0)?;
            values.replacement =
                product_ok(values.predecessor.compose_intersection(&values.effect))?;
            check_typed_rebase_support(&original, &values, accepted)?;
            if accepted {
                check_rebase_success(&original, &values)?;
            } else {
                check_rebase_refusal(&original, &values)?;
            }
        }
    }
    Ok(())
}

/// Preserve the private receiver's exact support cause before legacy cause collapse.
/// All descriptors and the saved model come from their actual checked producers;
/// no public baseline feedback or physical completion is asserted by this call.
/// # Errors
/// Preserve genuine admission, history and independently valid descriptor failures.
/// # Panics
/// Reject a changed typed refusal, partial history, or refused inclusive neighbor.
fn check_typed_rebase_support(
    original: &FixtureAffineChart,
    values: &RebaseCharts,
    accepted: bool,
) -> TestResult {
    let mut state = admitted(original)?;
    let before = history(&state)?;
    let actual = state.rebase_baseline(&values.predecessor, &values.effect, &values.replacement);
    if accepted {
        product_ok(actual)?;
        let after = history(&state)?;
        assert_eq!(after.anchor_phase_seconds(), 11.0 / 512.0);
        assert!(after.anchor_roundoff_seconds() >= before.anchor_roundoff_seconds());
        assert_eq!(after.frames(), values.replacement.frames());
        assert_eq!(after.domain(), values.replacement.domain());
        assert_eq!(state.unavailable, None);
        let projection = product_ok(state.project(
            &values.replacement,
            clocks()?,
            MonotonicNs::new(4 * SECOND),
            false,
        ))?;
        assert_eq!(projection.nominal_phase_seconds(), 11.0 / 512.0);
        assert_eq!(saved_record(projection.history()), saved_record(&after));
    } else {
        assert_eq!(actual, Err(FixtureFrequencyError::AnchorRebaseUnavailable));
        assert_eq!(saved_record(&history(&state)?), saved_record(&before));
        assert_eq!(
            state.unavailable,
            Some(FixtureFrequencyError::AnchorRebaseUnavailable)
        );
        assert!(matches!(
            state.project(original, clocks()?, MonotonicNs::new(4 * SECOND), false),
            Err(FixtureFrequencyError::AnchorRebaseUnavailable)
        ));
        assert_eq!(saved_record(&history(&state)?), saved_record(&before));
    }
    Ok(())
}

/// Restriction retains inherited errors instead of reconstructing exact inputs.
/// # Errors
/// Preserve checked source, domain and ordinary-composition construction failures.
/// # Panics
/// Reject changed operands, expanded support, dropped bounds or wrong refusal order.
#[test]
fn intersection_composition_retains_both_operand_bounds_and_input_identity() -> TestResult {
    let first = composed(GEOMETRY)?;
    let next_geometry = Geometry {
        tags: [31, 7, 37, 41],
        times: [0, 2 * SECOND, 4 * SECOND],
        ..GEOMETRY
    };
    let next = composed(next_geometry)?;
    let first_basis = Basis::from_chart(&first);
    let next_basis = Basis::from_chart(&next);
    for operand in [&first, &next] {
        assert!(operand.phase_coefficient_roundoff_seconds() > 0.0);
        assert!(operand.rate_coefficient_roundoff_seconds_per_second() > 0.0);
    }
    let restricted_first = composed(Geometry {
        times: next_geometry.times,
        ..GEOMETRY
    })?;
    let expected = product_ok(restricted_first.compose(&next))?;
    let actual = product_ok(first.compose_intersection(&next))?;
    assert_eq!(differences(&actual, &expected), [false; 6]);
    assert_eq!(actual.phase_at_reference_seconds(), 0.0);
    assert_eq!(actual.rate_seconds_per_second(), 0.0);
    assert!(
        actual.phase_coefficient_roundoff_seconds() >= first.phase_coefficient_roundoff_seconds()
    );
    assert!(
        actual.rate_coefficient_roundoff_seconds_per_second()
            >= next.rate_coefficient_roundoff_seconds_per_second()
    );
    assert!(first_basis.matches(&first));
    assert!(next_basis.matches(&next));
    let wrong_reference = composed(Geometry {
        times: [0, 3 * SECOND, 4 * SECOND],
        ..next_geometry
    })?;
    assert!(matches!(
        first.compose_intersection(&wrong_reference),
        Err(FixtureChartError::CompositionDomainMismatch)
    ));
    let wrong_mapping = composed(Geometry {
        tags: [43, 7, 37, 41],
        times: [0, 3 * SECOND, 4 * SECOND],
        ..next_geometry
    })?;
    assert!(matches!(
        first.compose_intersection(&wrong_mapping),
        Err(FixtureChartError::CompositionMappingMismatch)
    ));
    Ok(())
}

/// Missing or failed actual descriptor projections cannot change saved evidence.
/// # Errors
/// Preserve genuine model and checked descriptor construction failures.
/// # Panics
/// Reject treating a descriptor refusal as a successful re-expression.
#[test]
fn rebase_missing_or_failed_descriptors_preserve_the_complete_record() -> TestResult {
    let original = predecessor(2 * SECOND)?;
    let values = rebase_charts(2 * SECOND)?;
    for (effect, replacement) in [
        (Ok(None), Ok(Some(&values.replacement))),
        (
            Err(FixtureChartError::InvalidDomain),
            Ok(Some(&values.replacement)),
        ),
        (Ok(Some(&values.effect)), Ok(None)),
        (
            Ok(Some(&values.effect)),
            Err(FixtureChartError::InvalidFrames),
        ),
    ] {
        let mut state = admitted(&original)?;
        let before = saved_record(&history(&state)?);
        state.rebase(&original, effect, replacement);
        assert_eq!(saved_record(&history(&state)?), before);
        assert!(matches!(
            state.project(&original, clocks()?, MonotonicNs::new(4 * SECOND), false),
            Err(FixtureFrequencyError::AnchorRebaseUnavailable)
        ));
    }
    check_rebase_success(&original, &values)
}

/// The first hard retirement remains authoritative even after later reasons.
/// # Errors
/// Preserve genuine model and literal clock/descriptor construction failures.
/// # Panics
/// Reject a cleared/replaced retirement or altered historical record.
#[test]
fn first_hard_retirement_survives_later_reasons_and_pending_queries() -> TestResult {
    use FixturePhaseRetirementReason::{Conflict, Generation, Stop};
    let original = predecessor(2 * SECOND)?;
    for (first, second, expected) in [
        (Stop, Generation, FixtureFrequencyError::Stopped),
        (Generation, Stop, FixtureFrequencyError::Generation),
        (Stop, Conflict, FixtureFrequencyError::Stopped),
        (Generation, Conflict, FixtureFrequencyError::Generation),
        (Conflict, Stop, FixtureFrequencyError::Stopped),
        (Conflict, Generation, FixtureFrequencyError::Generation),
    ] {
        let mut state = admitted(&original)?;
        let before = saved_record(&history(&state)?);
        state.retire(first);
        if first == Conflict {
            product_ok(state.project(&original, clocks()?, MonotonicNs::new(4 * SECOND), false))?;
        }
        state.retire(second);
        assert_eq!(saved_record(&history(&state)?), before);
        // A pending operation must not hide an already irreversible block.
        let result = state.project(&original, clocks()?, MonotonicNs::new(4 * SECOND), true);
        assert!(matches!(result, Err(error) if error == expected));
        assert_eq!(saved_record(&history(&state)?), before);
    }
    Ok(())
}

/// Component Unknown is sticky and distinct from actual parent completion tests.
/// # Errors
/// Preserve genuine intake, saved-history and literal clock admission failures.
/// # Panics
/// Reject forgotten Unknown, changed history or incorrect hard-block precedence.
#[test]
fn unknown_component_latch_preserves_genuine_history_and_block_precedence() -> TestResult {
    let original = predecessor(2 * SECOND)?;
    let mut state = admitted(&original)?;
    let before = saved_record(&history(&state)?);
    state.unknown();
    state.unknown();
    assert!(state.unknown);
    assert_eq!(saved_record(&history(&state)?), before);
    assert!(matches!(
        state.project(&original, clocks()?, MonotonicNs::new(4 * SECOND), true),
        Err(FixtureFrequencyError::Unknown)
    ));
    state.retire(FixturePhaseRetirementReason::Stop);
    assert!(
        state.unknown,
        "hard retirement must retain the separate Unknown fact"
    );
    assert!(matches!(
        state.project(&original, clocks()?, MonotonicNs::new(4 * SECOND), false),
        Err(FixtureFrequencyError::Stopped)
    ));
    assert_eq!(saved_record(&history(&state)?), before);
    Ok(())
}

/// Factory-created Disabled/empty states cannot acquire history through latches.
/// # Errors
/// Preserve checked configuration, chart, cursor and clock admission failures.
/// # Panics
/// Reject enabled opt-out, invented model history or wrong empty-state behavior.
#[test]
fn disabled_latches_are_noops_and_enabled_empty_state_retains_no_model() -> TestResult {
    let original = predecessor(2 * SECOND)?;
    let mut disabled = FrequencyState::disabled();
    for reason in [
        FixturePhaseRetirementReason::Conflict,
        FixturePhaseRetirementReason::Stop,
        FixturePhaseRetirementReason::Generation,
    ] {
        disabled.retire(reason);
        disabled.unknown();
    }
    disabled.rebase(&original, Ok(None), Err(FixtureChartError::InvalidFrames));
    assert!(!disabled.is_enabled());
    assert!(!disabled.unknown);
    assert!(disabled.retired.is_none());
    assert!(disabled.history().is_none());
    assert_eq!(disabled.admission(), FixtureFrequencyAdmission::Disabled);
    assert!(matches!(
        disabled.project(&original, clocks()?, MonotonicNs::new(4 * SECOND), true),
        Err(FixtureFrequencyError::Disabled)
    ));

    let cursor = FixtureRateSettlementCursor::new(&original, clocks()?, label(1)?);
    let age = product_ok(MonotonicNs::new(32 * SECOND).duration_since(MonotonicNs::new(0)))?;
    let profile = product_ok(FixtureFrequencyProfile::new(1.0, 0.0, 1.0, age))?;
    let mut empty = product_ok(FrequencyState::enabled(
        profile,
        association()?,
        &original,
        &cursor,
    ))?;
    empty.retire(FixturePhaseRetirementReason::Conflict);
    empty.rebase(&original, Ok(None), Err(FixtureChartError::InvalidFrames));
    assert!(empty.is_enabled());
    assert!(empty.history().is_none());
    assert_eq!(
        empty.admission(),
        FixtureFrequencyAdmission::NoReady {
            emitted_round: None
        }
    );
    assert!(matches!(
        empty.project(&original, clocks()?, MonotonicNs::new(4 * SECOND), false),
        Err(FixtureFrequencyError::NoModel)
    ));
    Ok(())
}

/// Private checked-model operand control, not a public physical source trace.
/// Genuine admission supplies provenance; only the component's numerical anchor
/// operand is replaced with an independently checked finite extreme. No emission
/// or completion is created by this helper.
/// # Errors
/// Preserve genuine intake, checked anchor/chart and original history admission.
/// # Panics
/// Reject swallowed exact numeric cause, partial model publication or missing healthy neighbor.
#[test]
fn private_checked_anchor_rebase_failure_retains_exact_cause_and_whole_model() -> TestResult {
    let original = predecessor(2 * SECOND)?;
    let effect = chart([19, 29, 31, 7], original.domain(), -(f64::MAX / 2.0), 0.0)?;
    let replacement = product_ok(original.compose(&effect))?;
    for (phase, fails) in [(1.0 / 32.0, false), (f64::MAX, true)] {
        let mut state = admitted(&original)?;
        let checked = product_ok(crate::regression::FrequencyAnchor::new(phase, 0.0))?;
        let Some(model) = state.model.as_mut() else {
            return Err(io::Error::other("missing genuinely admitted model").into());
        };
        model.anchor = checked;
        let before = saved_record(&history(&state)?);
        let actual = state.rebase_baseline(&original, &effect, &replacement);
        if fails {
            assert!(matches!(actual, Err(FixtureFrequencyError::Numeric { .. })));
            assert_eq!(saved_record(&history(&state)?), before);
            assert!(matches!(
                state.project(&original, clocks()?, MonotonicNs::new(4 * SECOND), false),
                Err(FixtureFrequencyError::Numeric { .. })
            ));
        } else {
            product_ok(actual)?;
            assert_eq!(history(&state)?.anchor_phase_seconds(), f64::MAX / 2.0);
            assert_eq!(history(&state)?.frames(), replacement.frames());
        }
    }
    Ok(())
}

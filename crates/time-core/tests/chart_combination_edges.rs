// SPDX-License-Identifier: GPL-2.0-only
//! Independent public chart-combination arithmetic and population boundaries.
//!
//! CHECK-7/8/14/25: four fixed cases own22 three-row histories/66 appends,
//! 25 chart prepares/projections/enclosures,27 raw enclosures, seven raw seeds,
//! seven chart batches and eleven chart advances, plus one direct batch selection.
//! Independently seeded controls receive the same successful transitions;
//! eight complete-state equalities check
//! setup, failure, retry and replay. No state clone is needed. All observations,
//! entries and outcomes use public owners.
//! The integer oracle decodes stored bits and divides rationals independently;
//! it does not repeat the production error-propagation equations. These finite
//! source bounds require the coordinator's normal admission, not post-run credit.
#![forbid(unsafe_code)]

use std::{error::Error, io, num::NonZeroU64, ptr};

use time_core::{
    FixtureAdaptiveHistory, FixtureAffineChart, FixtureChartBatch, FixtureChartBoundIssue,
    FixtureChartBoundOperation, FixtureChartCombinationError, FixtureChartDomain,
    FixtureChartError, FixtureChartFrames, FixtureChartInterval, FixtureChartOutcome,
    FixtureCombinationDenominatorField, FixtureCombinationError, FixtureCombinationLifecycleError,
    FixtureCombinationParameters, FixtureCombinationSourceStatus, FixtureCombinationState,
    FixtureContext, FixtureMinimum, FixtureObservation, FixturePrincipalOutcomeKind,
    FixturePrincipalParameters, FixturePrincipalStratum, FixtureProjectedBatch,
    FixtureProjectionTerms, enclose_fixture_chart_projection, enclose_fixture_projection,
};
use time_domain::{MonotonicNs, UtcCoordinateQ32, exchange_arithmetic};

/// A genuine public-owner or bounded independent-oracle failure.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Exact nanoseconds in one fixture second.
const SECOND: u64 = 1_000_000_000;
/// Distinct stored SD numerators, each with a mathematically inexact square.
const SD_A: u32 = (1 << 28) + 1;
/// The second square rounds in the opposite direction to the first.
const SD_B: u32 = (1 << 28) + 3;

/// Observe the exact typed cause already stored in one actual failed operation.
/// # Errors
/// Reject a missing or differently typed cause without fabricating a replacement.
/// # Panics
/// The borrowed cause must point to the original stored payload.
fn check_stored_cause<C: Error + 'static>(error: &(dyn Error + 'static), stored: &C) -> TestResult {
    let actual = error
        .source()
        .and_then(|cause| cause.downcast_ref::<C>())
        .ok_or_else(|| io::Error::other("missing original typed failure cause"))?;
    assert!(ptr::eq(actual, stored));
    Ok(())
}

/// Admit a literal nonzero source/frame/generation label.
/// # Errors
/// Reject zero without replacing the requested identity.
fn label(value: u64) -> TestResult<NonZeroU64> {
    NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero fixture label").into())
}

/// One genuine explicitly contextualized three-row adaptive source owner.
struct Source {
    /// Original raw assertions shared by the actual observations.
    context: FixtureContext,
    /// Sole owner of the history; no fitted attachment is fabricated.
    history: FixtureAdaptiveHistory,
}

impl Source {
    /// Append exactly three chronological exchange-derived Q32 inputs.
    /// # Errors
    /// Preserve actual coordinate, observation, append and adaptive-fit failures.
    fn with_context(
        context: FixtureContext,
        times: [u64; 3],
        ticks: [i64; 3],
        divisors: [f64; 3],
    ) -> TestResult<Self> {
        let mut history = FixtureAdaptiveHistory::new(context, 3)?;
        for ((at, tick), divisor) in times.into_iter().zip(ticks).zip(divisors) {
            let zero = UtcCoordinateQ32::new(0)?;
            let shifted = UtcCoordinateQ32::new(i128::from(tick))?;
            let phase = exchange_arithmetic(zero, shifted, shifted, zero)?.offset();
            history.try_append(FixtureObservation::new(
                context,
                MonotonicNs::new(at),
                phase,
                divisor,
            )?)?;
        }
        Ok(Self { context, history })
    }

    /// Project the real latest fitted row, preserving original SD and chart identity.
    /// # Errors
    /// Reject unexpected fixture occupancy or preserve public producer failures.
    fn entry<'chart>(
        &self,
        map: &'chart FixtureAffineChart,
        at: u64,
        distance: f64,
        skew: f64,
    ) -> TestResult<FixtureChartInterval<'chart>> {
        if self.history.sample_count() != 3 {
            return Err(io::Error::other("fixture occupancy").into());
        }
        let view = self
            .history
            .prepare_chart(self.context, FixtureChartOutcome::Known(map))?
            .ok_or_else(|| io::Error::other("missing actual chart view"))?;
        let anchor = self
            .history
            .reference()
            .ok_or_else(|| io::Error::other("missing actual reference"))?;
        let projection = view.project_tracking(
            view.context(),
            2,
            anchor,
            MonotonicNs::new(at),
            FixtureProjectionTerms::new(distance, 0.0, 0.0)?,
            skew,
        )?;
        Ok(enclose_fixture_chart_projection(&projection)?)
    }
}

/// Return a fixed raw context for one genuinely distinct source.
/// # Errors
/// Preserve literal label admission failures.
fn context(id: u64) -> TestResult<FixtureContext> {
    Ok(FixtureContext::new(label(id)?, label(17)?, label(23)?))
}

/// Admit an identity mathematical map into a distinct target context.
/// # Errors
/// Preserve exact-domain, frame and scalar admission failures.
fn identity(at: u64) -> TestResult<FixtureAffineChart> {
    chart_with_rate(at, 0.0)
}

/// Admit a literal rate adjustment at the exact evaluation/reference instant.
/// # Errors
/// Preserve actual descriptor, domain and scalar admission failures.
fn chart_with_rate(at: u64, rate: f64) -> TestResult<FixtureAffineChart> {
    Ok(FixtureAffineChart::new(
        FixtureChartFrames::new(label(17)?, label(23)?, label(19)?, label(29)?)?,
        FixtureChartDomain::new(
            MonotonicNs::new(0),
            MonotonicNs::new(at),
            MonotonicNs::new(at),
        )?,
        0.0,
        rate,
    )?)
}

/// Seed a complete real raw roster from original projections, retaining pending credit.
/// # Errors
/// Reject empty/oversized fixtures or preserve genuine enclosure and seed failures.
fn seed(
    entries: &[FixtureChartInterval<'_>],
) -> TestResult<(FixtureCombinationState, Vec<FixturePrincipalStratum>)> {
    if !(1..=16).contains(&entries.len()) {
        return Err(io::Error::other("fixture roster bound").into());
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
    let at = raw
        .first()
        .ok_or_else(|| io::Error::other("missing original entry"))?
        .projection()
        .projected_at();
    let batch = FixtureProjectedBatch::new(label(17)?, label(23)?, at, &raw)?;
    let owner = FixtureCombinationState::seed(
        &roster,
        FixturePrincipalParameters::new(1.0 / 1024.0, 0.0)?,
        FixtureCombinationParameters::new(2.0, 1.0 / 32.0)?,
        FixtureMinimum::new(entries.len())?,
        &batch,
        &strata,
    )?;
    Ok((owner, strata))
}

/// Preserve exact integer arithmetic or fail outside this finite oracle envelope.
/// # Errors
/// Reject overflow before it can influence an expected result.
fn checked<T>(value: Option<T>) -> TestResult<T> {
    value.ok_or_else(|| io::Error::other("exact oracle integer envelope").into())
}

/// Decode a signed normal-or-zero binary64 into an integer times an exact power.
/// # Errors
/// Reject nonfinite/subnormal fixture points; no floating arithmetic is used.
fn parts(value: f64) -> TestResult<(i128, i32)> {
    if value == 0.0 {
        return Ok((0, 0));
    }
    if !value.is_normal() {
        return Err(io::Error::other("exact oracle point envelope").into());
    }
    let bits = value.to_bits();
    let magnitude = i128::from((bits & ((1_u64 << 52) - 1)) | (1_u64 << 52));
    let signed = if bits >> 63 == 0 {
        magnitude
    } else {
        -magnitude
    };
    Ok((signed, i32::try_from((bits >> 52) & 0x7ff)? - 1023 - 52))
}

/// Multiply a signed integer by a finite exact nonnegative power of two.
/// # Errors
/// Reject unsupported shifts or actual multiplication overflow.
fn shift(value: i128, exponent: i32) -> TestResult<i128> {
    if !(0..=126).contains(&exponent) {
        return Err(io::Error::other("exact oracle shift envelope").into());
    }
    checked(value.checked_mul(checked(1_i128.checked_shl(u32::try_from(exponent)?))?))
}

/// Decode actual fitted phases into integral 2^-60 units without rounding.
/// # Errors
/// Reject a point outside the fixture's exact dyadic integer envelope.
fn phase_units(value: f64) -> TestResult<i128> {
    let (mantissa, exponent) = parts(value)?;
    let exponent = exponent + 60;
    if exponent >= 0 {
        shift(mantissa, exponent)
    } else {
        let divisor = shift(1, -exponent)?;
        if mantissa % divisor != 0 {
            return Err(io::Error::other("nonintegral fixture phase units").into());
        }
        Ok(mantissa / divisor)
    }
}

/// Normalize an exact positive rational without exceeding120-bit operands.
/// # Errors
/// Reject an unsupported envelope, exhausted fixed loop or integer overflow.
fn normalize(
    mut numerator: u128,
    mut denominator: u128,
    mut exponent: i32,
) -> TestResult<(u128, u128, i32)> {
    if numerator == 0 || denominator == 0 || numerator.max(denominator) >= 1_u128 << 120 {
        return Err(io::Error::other("exact rational envelope").into());
    }
    for _ in 0..128 {
        if numerator >= denominator {
            break;
        }
        numerator = checked(numerator.checked_mul(2))?;
        exponent = checked(exponent.checked_sub(1))?;
    }
    for _ in 0..128 {
        if numerator < checked(denominator.checked_mul(2))? {
            break;
        }
        denominator = checked(denominator.checked_mul(2))?;
        exponent = checked(exponent.checked_add(1))?;
    }
    if numerator < denominator || numerator >= checked(denominator.checked_mul(2))? {
        return Err(io::Error::other("exact rational normalization bound").into());
    }
    Ok((numerator, denominator, exponent))
}

/// Obtain the least binary64 upper for an exact positive rational times2^exponent.
///
/// Independent binary long division emits52 fraction bits, then rounds upward
/// exactly when a nonzero remainder remains. No production bound equation is used.
/// # Errors
/// Reject an unsupported normal-result envelope or integer overflow.
fn rational_upper(numerator: u128, denominator: u128, exponent: i32) -> TestResult<f64> {
    if numerator == 0 {
        return Ok(0.0);
    }
    let (numerator, denominator, mut exponent) = normalize(numerator, denominator, exponent)?;
    let mut remainder = numerator - denominator;
    let mut mantissa = 1_u64 << 52;
    for bit in (0..52).rev() {
        remainder = checked(remainder.checked_mul(2))?;
        if remainder >= denominator {
            remainder -= denominator;
            mantissa |= 1_u64 << bit;
        }
    }
    if remainder != 0 {
        mantissa += 1;
    }
    if mantissa == 1_u64 << 53 {
        mantissa >>= 1;
        exponent = checked(exponent.checked_add(1))?;
    }
    if !(-1022..=1023).contains(&exponent) {
        return Err(io::Error::other("exact rational result envelope").into());
    }
    let encoded_exponent = u64::try_from(exponent + 1023)? << 52;
    Ok(f64::from_bits(
        encoded_exponent | (mantissa - (1_u64 << 52)),
    ))
}

/// Require a reported bound to contain the exact stored-input rational reference.
/// # Errors
/// Reject an unsupported exact-integer envelope or nonpositive denominator.
/// # Panics
/// Reject negative/nonfinite/trivial bounds or failure of exact containment.
fn covers(point: f64, bound: f64, numerator: i128, denominator: i128, scale: i32) -> TestResult {
    if denominator <= 0 {
        return Err(io::Error::other("exact oracle denominator").into());
    }
    let (mantissa, exponent) = parts(point)?;
    let common = exponent.min(scale);
    let represented = shift(
        checked(mantissa.checked_mul(denominator))?,
        exponent - common,
    )?;
    let reference = shift(numerator, scale - common)?;
    let error = checked(represented.checked_sub(reference))?.unsigned_abs();
    let required = rational_upper(error, u128::try_from(denominator)?, common)?;
    assert!(bound.is_finite() && (0.0..1.0 / 1_000_000_000.0).contains(&bound));
    assert!(bound >= required);
    Ok(())
}

/// Anchor the independent divider with exact, non-dyadic and halfway references.
/// # Errors
/// Preserve an unexpected oracle-envelope failure.
/// # Panics
/// Reject incorrect upward rounding, a lost carry or a nonzero exact-zero bound.
fn check_oracle_anchors() -> TestResult {
    assert_eq!(rational_upper(0, 3, -60)?.to_bits(), 0);
    assert_eq!(rational_upper(1, 2, 0)?.to_bits(), 0x3fe0_0000_0000_0000);
    assert_eq!(rational_upper(1, 3, 0)?.to_bits(), 0x3fd5_5555_5555_5556);
    assert_eq!(
        rational_upper((1_u128 << 53) + 1, 1, -53)?.to_bits(),
        0x3ff0_0000_0000_0001
    );
    Ok(())
}

/// Unequal original distances and inexact SD squares retain exact stored-input means.
/// # Errors
/// Preserve genuine producer, state, output and independent exact-oracle failures.
/// # Panics
/// Reject wrong weight/phase inputs, equal-weight replacement or uncovered error.
#[test]
fn unequal_original_weights_contain_exact_stored_input_means() -> TestResult {
    check_oracle_anchors()?;
    let ma = i64::from(SD_A) * 8;
    let mb = i64::from(SD_B) * 8;
    let eighth = 1_i64 << 29;
    let times = [0, SECOND, 2 * SECOND];
    let a = Source::with_context(
        context(11)?,
        times,
        [ma - eighth, 0, ma + eighth],
        [1.0, 1.5, 1.0],
    )?;
    let b = Source::with_context(
        context(22)?,
        times,
        [mb + eighth, 0, mb - eighth],
        [1.0, 1.5, 1.0],
    )?;
    let map = identity(2 * SECOND)?;
    let entries = [
        a.entry(&map, 2 * SECOND, 1.0, 0.5)?,
        b.entry(&map, 2 * SECOND, 2.0, 0.5)?,
    ];
    let (mut owner, strata) = seed(&entries)?;
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(2 * SECOND), &entries)?;
    let outcome = owner.advance_chart(&map, &batch, &strata)?;
    assert_eq!(
        outcome.principal().kind(),
        FixturePrincipalOutcomeKind::ReferenceReady
    );
    assert_eq!(outcome.witness_count(), Some(2));
    assert_eq!(outcome.contributor_count(), Some(2));
    let result = outcome
        .combined()
        .ok_or_else(|| io::Error::other("missing unequal mean"))?;
    assert!(ptr::eq(
        ptr::from_ref(result.batch()),
        ptr::from_ref(&batch)
    ));
    let original_a = entries[0].projection().source_projection();
    let original_b = entries[1].projection().source_projection();
    let fitted_a = original_a
        .tracking()
        .ok_or_else(|| io::Error::other("missing A fit"))?;
    let fitted_b = original_b
        .tracking()
        .ok_or_else(|| io::Error::other("missing B fit"))?;
    assert_eq!(original_a.distance_seconds(), 1.0);
    assert_eq!(original_b.distance_seconds(), 2.0);
    assert_eq!(original_a.phase_rate_seconds_per_second(), 1.0 / 8.0);
    assert_eq!(original_b.phase_rate_seconds_per_second(), -1.0 / 8.0);
    assert_eq!(
        fitted_a.rate_standard_deviation_seconds_per_second(),
        f64::from(SD_A) / 1_073_741_824.0
    );
    assert_eq!(
        fitted_b.rate_standard_deviation_seconds_per_second(),
        f64::from(SD_B) / 1_073_741_824.0
    );
    let phase = checked(
        checked(phase_units(fitted_a.phase_seconds())?.checked_mul(2))?
            .checked_add(phase_units(fitted_b.phase_seconds())?),
    )?;
    covers(
        result.phase_seconds(),
        result.phase_mean_roundoff_seconds(),
        phase,
        3,
        -60,
    )?;
    let aa = i128::from(SD_A) * i128::from(SD_A);
    let bb = i128::from(SD_B) * i128::from(SD_B);
    assert_eq!(aa % 16, 1);
    assert_eq!(bb % 16, 9);
    covers(
        result.phase_rate_seconds_per_second(),
        result.rate_mean_roundoff_seconds_per_second(),
        bb - aa,
        8 * (aa + bb),
        0,
    )?;
    // Equal rate weights would give exactly zero, far beyond the admitted bound.
    assert!(result.phase_rate_seconds_per_second() > 1.0 / 10_000_000_000.0);
    Ok(())
}

/// Sixteen real stored-tenth rates need a reduction bound larger than each input bound.
/// # Errors
/// Preserve real three-row fit, full-capacity public admission and arithmetic failures.
/// # Panics
/// Reject an unrealized rate/weight fixture, dropped reduction error or changed mean bits.
#[test]
fn sixteen_contributors_bound_new_reduction_roundoff() -> TestResult {
    let at = 625_000_000;
    let times = [0, 312_500_000, at];
    let ticks = [20_i64 << 27, 1_i64 << 27, 22_i64 << 27];
    let sources = (1..=16)
        .map(|id| Source::with_context(context(id)?, times, ticks, [1.0, 1.5, 1.0]))
        .collect::<TestResult<Vec<_>>>()?;
    let map = identity(at)?;
    let entries = sources
        .iter()
        .map(|source| source.entry(&map, at, 1.0, 1.0))
        .collect::<TestResult<Vec<_>>>()?;
    let stored_tenth = f64::from_bits(0x3fb9_9999_9999_999a);
    let exact_error = 1.0 / 72_057_594_037_927_936.0;
    for entry in &entries {
        let projection = entry.projection();
        let original = projection.source_projection();
        let tracking = original
            .tracking()
            .ok_or_else(|| io::Error::other("missing actual sixteen fit"))?;
        assert_eq!(
            original.phase_rate_seconds_per_second().to_bits(),
            stored_tenth.to_bits()
        );
        assert_eq!(tracking.rate_standard_deviation_seconds_per_second(), 1.0);
        assert_eq!(original.distance_seconds(), 1.0);
        assert_eq!(
            projection.phase_rate_seconds_per_second().to_bits(),
            stored_tenth.to_bits()
        );
        assert!(projection.rate_roundoff_seconds_per_second() < exact_error);
    }
    let (mut owner, strata) = seed(&entries)?;
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(at), &entries)?;
    let outcome = owner.advance_chart(&map, &batch, &strata)?;
    assert_eq!(
        outcome.principal().kind(),
        FixturePrincipalOutcomeKind::ReferenceReady
    );
    assert_eq!(outcome.witness_count(), Some(16));
    assert_eq!(outcome.contributor_count(), Some(16));
    let result = outcome
        .combined()
        .ok_or_else(|| io::Error::other("missing sixteen mean"))?;
    assert_eq!(result.supporter_count(), 16);
    assert!(result.supporters().eq((1..=16).filter_map(NonZeroU64::new)));
    assert_eq!(
        result.phase_rate_seconds_per_second().to_bits(),
        0x3fb9_9999_9999_999b
    );
    assert!(result.rate_mean_roundoff_seconds_per_second() >= exact_error);
    assert!(result.rate_mean_roundoff_seconds_per_second().is_finite());
    assert!(ptr::eq(
        ptr::from_ref(result.batch()),
        ptr::from_ref(&batch)
    ));
    Ok(())
}

/// A genuine singleton bypasses denominators; an original pair filtered to one does not.
/// # Errors
/// Preserve public source, receiving, healthy retry and retained-replay failures.
/// # Panics
/// Reject singleton rounding, pair-to-singleton shortcuts, partial commit or minted credit.
#[test]
fn original_population_controls_singleton_and_filtered_one_retry() -> TestResult {
    let times = [0, SECOND, 2 * SECOND];
    let a = Source::with_context(context(11)?, times, [0, 2_i64 << 28, 0], [1.0, 1.0, 4.0])?;
    let b = Source::with_context(
        context(22)?,
        times,
        [1_i64 << 28, 3_i64 << 28, 1_i64 << 28],
        [1.0, 1.0, 4.0],
    )?;
    let map = identity(2 * SECOND)?;
    let singleton = [a.entry(&map, 2 * SECOND, 0.0, 0.0)?];
    let (mut one, one_strata) = seed(&singleton)?;
    let one_batch = FixtureChartBatch::new(&map, MonotonicNs::new(2 * SECOND), &singleton)?;
    let one_outcome = one.advance_chart(&map, &one_batch, &one_strata)?;
    assert_eq!(one_outcome.witness_count(), Some(1));
    assert_eq!(one_outcome.contributor_count(), Some(1));
    let copied = one_outcome
        .combined()
        .ok_or_else(|| io::Error::other("missing singleton"))?;
    let projection = singleton[0].projection();
    let tracking = projection
        .tracking()
        .ok_or_else(|| io::Error::other("missing singleton fit"))?;
    assert_eq!(
        copied.phase_seconds().to_bits(),
        tracking.phase_seconds().to_bits()
    );
    assert_eq!(
        copied.phase_rate_seconds_per_second().to_bits(),
        projection.phase_rate_seconds_per_second().to_bits()
    );
    assert_eq!(
        copied.phase_mean_roundoff_seconds().to_bits(),
        tracking.phase_roundoff_seconds().to_bits()
    );
    assert_eq!(
        copied.rate_mean_roundoff_seconds_per_second().to_bits(),
        projection.rate_roundoff_seconds_per_second().to_bits()
    );
    assert_eq!(copied.tracking_skew_seconds_per_second(), 0.0);
    let initial = [singleton[0], b.entry(&map, 2 * SECOND, 2.0, 0.5)?];
    let (mut pair, strata) = seed(&initial)?;
    let (mut control, control_strata) = seed(&initial)?;
    assert_eq!(pair, control);
    let bad_batch = FixtureChartBatch::new(&map, MonotonicNs::new(2 * SECOND), &initial)?;
    let actual = pair.advance_chart(&map, &bad_batch, &strata).err();
    let expected = FixtureChartCombinationError::Lifecycle {
        source: FixtureCombinationLifecycleError::Combination {
            source: FixtureCombinationError::ZeroDenominator {
                index: 0,
                field: FixtureCombinationDenominatorField::RootDistance,
            },
        },
    };
    assert_eq!(actual, Some(expected));
    assert_eq!(pair, control);
    let error = actual
        .as_ref()
        .ok_or_else(|| io::Error::other("missing actual lifecycle rejection"))?;
    let FixtureChartCombinationError::Lifecycle { source } = error else {
        return Err(io::Error::other("unexpected actual lifecycle rejection").into());
    };
    check_stored_cause(error, source)?;
    let FixtureCombinationLifecycleError::Combination { source: point } = source else {
        return Err(io::Error::other("missing actual denominator rejection").into());
    };
    check_stored_cause(source, point)?;
    let corrected = [a.entry(&map, 2 * SECOND, 0.25, 0.5)?, initial[1]];
    let good_batch = FixtureChartBatch::new(&map, MonotonicNs::new(2 * SECOND), &corrected)?;
    let ready = pair.advance_chart(&map, &good_batch, &strata)?;
    assert_eq!(ready.witness_count(), Some(2));
    assert_eq!(ready.contributor_count(), Some(1));
    assert_eq!(
        ready.source_status(label(22)?),
        Some(FixtureCombinationSourceStatus::DistanceExcluded)
    );
    assert_eq!(ready.emitted_round(), Some(label(1)?));
    let principal_label = label(11)?;
    assert!(
        ready
            .combined()
            .is_some_and(|estimate| estimate.supporters().eq([principal_label]))
    );
    control.advance_chart(&map, &good_batch, &control_strata)?;
    assert_eq!(pair, control);
    let replay = pair.advance_chart(&map, &good_batch, &strata)?;
    assert_eq!(
        replay.principal().kind(),
        FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    );
    assert!(replay.combined().is_none());
    assert_eq!(replay.emitted_round(), None);
    assert_eq!(pair, control);
    Ok(())
}

/// Verify realized public inputs reach the intended finite point/bound separation.
/// # Errors
/// Preserve a missing genuine fitted attachment without fabricating a replacement.
/// # Panics
/// Reject a different original fit, upstream nonfinite bound or wrong chart rate.
fn check_bound_only_inputs(entries: &[FixtureChartInterval<'_>; 2]) -> TestResult {
    for entry in entries {
        let projection = entry.projection();
        let original = projection.source_projection();
        let tracking = original
            .tracking()
            .ok_or_else(|| io::Error::other("missing bound-only fit"))?;
        assert_eq!(original.phase_rate_seconds_per_second(), 0.0);
        assert_eq!(tracking.rate_standard_deviation_seconds_per_second(), 0.5);
        assert_eq!(projection.phase_rate_seconds_per_second(), -f64::MAX / 4.0);
        assert!(projection.center_roundoff_seconds().is_finite());
        assert!(projection.rate_roundoff_seconds_per_second().is_finite());
        let fitted = projection
            .tracking()
            .ok_or_else(|| io::Error::other("missing bound-only chart fit"))?;
        assert!(fitted.phase_roundoff_seconds().is_finite());
    }
    Ok(())
}

/// A finite weighted point can exhaust only the companion's outward successor.
///
/// With query equal to chart reference, displacement remains zero. The original
/// rate SD is1/2, so its rate weight is4. F=MAX/4 gives finite weighted rate-MAX,
/// but the outward magnitude upper for4*F requires the unavailable MAX successor.
/// # Errors
/// Preserve genuine producer, admission, corrected-event and replay failures.
/// # Panics
/// Reject an unrealized fixture, earlier point failure, partial commit or lost retry.
#[test]
fn bound_only_failure_rolls_back_and_allows_same_observation_retry() -> TestResult {
    let times = [0, SECOND, 2 * SECOND];
    let ticks = [1_i64 << 32, 0, 1_i64 << 32];
    let a = Source::with_context(context(11)?, times, ticks, [1.0, 1.5, 1.0])?;
    let b = Source::with_context(context(22)?, times, ticks, [1.0, 1.5, 1.0])?;
    let bad_chart = chart_with_rate(2 * SECOND, f64::MAX / 4.0)?;
    let initial = [
        a.entry(&bad_chart, 2 * SECOND, 0.25, 0.5)?,
        b.entry(&bad_chart, 2 * SECOND, 2.0, 0.5)?,
    ];
    check_bound_only_inputs(&initial)?;
    let bad_batch = FixtureChartBatch::new(&bad_chart, MonotonicNs::new(2 * SECOND), &initial)?;
    let selected = bad_batch.select(FixtureMinimum::new(2)?)?;
    assert_eq!(
        selected
            .selection()
            .witnessed()
            .map(|value| value.supporters().len()),
        Some(2)
    );
    let (mut owner, strata) = seed(&initial)?;
    let (mut control, control_strata) = seed(&initial)?;
    assert_eq!(owner, control);
    let actual = owner.advance_chart(&bad_chart, &bad_batch, &strata).err();
    let expected = FixtureChartCombinationError::Bound {
        source: FixtureChartError::Bound {
            operation: FixtureChartBoundOperation::Multiply,
            issue: FixtureChartBoundIssue::NonFiniteResult,
        },
    };
    assert_eq!(actual, Some(expected));
    assert_eq!(owner, control);
    let error = actual
        .as_ref()
        .ok_or_else(|| io::Error::other("missing actual bound rejection"))?;
    let FixtureChartCombinationError::Bound { source } = error else {
        return Err(io::Error::other("unexpected actual bound rejection").into());
    };
    check_stored_cause(error, source)?;
    let good_chart = identity(2 * SECOND)?;
    let corrected = [
        a.entry(&good_chart, 2 * SECOND, 0.25, 0.5)?,
        b.entry(&good_chart, 2 * SECOND, 2.0, 0.5)?,
    ];
    let good_batch = FixtureChartBatch::new(&good_chart, MonotonicNs::new(2 * SECOND), &corrected)?;
    let ready = owner.advance_chart(&good_chart, &good_batch, &strata)?;
    assert_eq!(
        ready.principal().kind(),
        FixturePrincipalOutcomeKind::ReferenceReady
    );
    assert_eq!(ready.witness_count(), Some(2));
    assert_eq!(ready.contributor_count(), Some(1));
    assert_eq!(ready.emitted_round(), Some(label(1)?));
    assert_eq!(
        ready.source_status(label(22)?),
        Some(FixtureCombinationSourceStatus::DistanceExcluded)
    );
    let result = ready
        .combined()
        .ok_or_else(|| io::Error::other("missing bound retry result"))?;
    assert_eq!(result.phase_rate_seconds_per_second(), 0.0);
    assert!(result.rate_mean_roundoff_seconds_per_second().is_finite());
    control.advance_chart(&good_chart, &good_batch, &control_strata)?;
    assert_eq!(owner, control);
    let replay = owner.advance_chart(&good_chart, &good_batch, &strata)?;
    assert_eq!(
        replay.principal().kind(),
        FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    );
    assert!(replay.combined().is_none());
    assert_eq!(replay.emitted_round(), None);
    assert_eq!(owner, control);
    Ok(())
}

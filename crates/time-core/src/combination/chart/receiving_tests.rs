// SPDX-License-Identifier: GPL-2.0-only
//! Independent actual-input controls for the private chart tracking receiver.
//!
//! Each source owns exactly three real exchange-derived observations. Setup
//! admits at most three sources to one real principal owner; the hostile-context
//! loop additionally creates three independent one-source projections. All loops
//! have literal bounds, and no checked decision, selection or attachment is forged.
//! Only raw private relationship candidates are rearranged for negative controls.

use std::{error::Error, io, num::NonZeroU64, ptr};

use garde::Validate;
use time_domain::{MonotonicNs, UtcCoordinateQ32, exchange_arithmetic};

use crate::{
    FixtureAdaptiveHistory, FixtureAffineChart, FixtureChartBatch, FixtureChartDomain,
    FixtureChartFrames, FixtureChartInterval, FixtureChartOutcome, FixtureChartPrincipalDecision,
    FixtureCombinationDenominatorField, FixtureContext, FixtureMinimum, FixtureObservation,
    FixturePrincipalOutcomeKind, FixturePrincipalParameters, FixturePrincipalState,
    FixturePrincipalStratum, FixtureProjectedBatch, FixtureProjectionTerms,
    enclose_fixture_chart_projection, enclose_fixture_projection,
};

use crate::combination::chart::{ChartQuantities, ChartReductionError, ChartSums, CompanionError};

use super::{
    AdmittedChartSupporters, ChartTrackingRecord, ChartTrackingSupporters,
    FixtureCombinationContextField, FixtureCombinationError, FixtureTrackingCombinationError,
    MAX_SUPPORTERS, RawChartSourceAdmission, admit_chart_records, admit_chart_source,
};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;
const SECOND: u64 = 1_000_000_000;

/// Admit one fixed nonzero fixture label, with no state or external effects.
/// # Errors
/// Reject zero instead of substituting another source identity.
fn label(value: u64) -> TestResult<NonZeroU64> {
    NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero fixture label").into())
}

/// One genuine adaptive owner in its unchanged explicitly supplied raw context.
struct Source {
    context: FixtureContext,
    history: FixtureAdaptiveHistory,
}

impl Source {
    /// Append exactly three chronological dyadic phases through public owners.
    /// # Errors
    /// Preserve actual coordinate, observation, append and adaptive-fit failures.
    fn new(context: FixtureContext, values: [i32; 3]) -> TestResult<Self> {
        let mut history = FixtureAdaptiveHistory::new(context, 3)?;
        for ((second, value), divisor) in [0_u64, 1, 2].into_iter().zip(values).zip([1.0, 1.0, 4.0])
        {
            let zero = UtcCoordinateQ32::new(0)?;
            let shifted = UtcCoordinateQ32::new(i128::from(value) << 28)?;
            let phase = exchange_arithmetic(zero, shifted, shifted, zero)?.offset();
            history.try_append(FixtureObservation::new(
                context,
                MonotonicNs::new(second * SECOND),
                phase,
                divisor,
            )?)?;
        }
        Ok(Self { context, history })
    }
}

/// Produce one three-row source in the fixed healthy raw frame and generation.
/// # Errors
/// Preserve genuine label, history and fit admission failures.
fn source(id: u64, values: [i32; 3]) -> TestResult<Source> {
    Source::new(
        FixtureContext::new(label(id)?, label(17)?, label(23)?),
        values,
    )
}

/// Admit a nonzero affine map with explicit raw origin and a fixed target.
/// # Errors
/// Preserve actual frame, inclusive-domain and chart scalar admission failures.
fn chart(frame: u64, generation: u64) -> TestResult<FixtureAffineChart> {
    Ok(FixtureAffineChart::new(
        FixtureChartFrames::new(label(frame)?, label(generation)?, label(19)?, label(29)?)?,
        FixtureChartDomain::new(
            MonotonicNs::new(0),
            MonotonicNs::new(2 * SECOND),
            MonotonicNs::new(5 * SECOND),
        )?,
        4.0 / 1024.0,
        1.0 / 1024.0,
    )?)
}

/// Project the real final fitted row into its own target-source context.
/// # Errors
/// Preserve genuine view, projection, tracking and enclosure failures.
fn entry<'chart>(
    source: &Source,
    map: &'chart FixtureAffineChart,
    query: u64,
    distance: f64,
    tracked: bool,
) -> TestResult<FixtureChartInterval<'chart>> {
    entry_with_tracking_skew(source, map, query, distance, tracked, 0.125)
}

/// Supply an explicit source-owned tracking skew independently of geometry growth.
/// # Errors
/// Preserve fixture query bounds and all real projection/enclosure failures.
fn entry_with_tracking_skew<'chart>(
    source: &Source,
    map: &'chart FixtureAffineChart,
    query: u64,
    distance: f64,
    tracked: bool,
    tracking_skew: f64,
) -> TestResult<FixtureChartInterval<'chart>> {
    if !(4..=5).contains(&query) {
        return Err(io::Error::other("fixture query bound").into());
    }
    entry_at_with_tracking_skew(
        source,
        map,
        MonotonicNs::new(query * SECOND),
        distance,
        tracked,
        tracking_skew,
    )
}

/// Project one genuine source at a bounded exact query, including fractional seconds.
/// # Errors
/// Preserve exact query bounds, same-owner tracking and enclosure failures.
fn entry_at_with_tracking_skew<'chart>(
    source: &Source,
    map: &'chart FixtureAffineChart,
    at: MonotonicNs,
    distance: f64,
    tracked: bool,
    tracking_skew: f64,
) -> TestResult<FixtureChartInterval<'chart>> {
    if !(4 * SECOND..=5 * SECOND).contains(&at.nanoseconds()) {
        return Err(io::Error::other("fixture nanosecond query bound").into());
    }
    let view = source
        .history
        .prepare_chart(source.context, FixtureChartOutcome::Known(map))?
        .ok_or_else(|| io::Error::other("missing genuine chart view"))?;
    let terms = FixtureProjectionTerms::new(distance, 0.0, 0.125)?;
    let projection = if tracked {
        view.project_tracking(
            view.context(),
            2,
            MonotonicNs::new(2 * SECOND),
            at,
            terms,
            tracking_skew,
        )?
    } else {
        view.project_anchor(view.context(), 2, MonotonicNs::new(2 * SECOND), at, terms)?
    };
    Ok(enclose_fixture_chart_projection(&projection)?)
}

/// Seed the genuine retained raw owner without transforming or relabelling data.
/// # Errors
/// Reject a fixture population outside two/three or preserve actual admission failures.
fn seed(entries: &[FixtureChartInterval<'_>]) -> TestResult<FixturePrincipalState> {
    seed_with_minimum(entries, 2)
}

/// Seed one bounded genuine raw population at its actual supplied query.
/// # Errors
/// Reject fixture population/minimum errors or preserve actual receiving failures.
fn seed_with_minimum(
    entries: &[FixtureChartInterval<'_>],
    minimum: usize,
) -> TestResult<FixturePrincipalState> {
    if !(1..=3).contains(&entries.len()) {
        return Err(io::Error::other("fixture seed population").into());
    }
    let raw = entries
        .iter()
        .map(|entry| enclose_fixture_projection(entry.projection().source_projection()))
        .collect::<Result<Vec<_>, _>>()?;
    let roster = raw
        .iter()
        .map(|entry| entry.projection().context().source_id())
        .collect::<Vec<_>>();
    let strata = strata(&roster)?;
    let at = raw
        .first()
        .ok_or_else(|| io::Error::other("missing genuine raw entry"))?
        .projection()
        .projected_at();
    let batch = FixtureProjectedBatch::new(label(17)?, label(23)?, at, &raw)?;
    Ok(FixturePrincipalState::seed(
        &roster,
        FixturePrincipalParameters::new(1.0 / 1024.0, 0.0)?,
        FixtureMinimum::new(minimum)?,
        &batch,
        &strata,
    )?)
}

/// Admit complete checked stratum metadata for a bounded literal fixture roster.
/// # Errors
/// Reject fixture overcapacity or preserve the checked stratum failure.
fn strata(roster: &[NonZeroU64]) -> TestResult<Vec<FixturePrincipalStratum>> {
    if roster.len() > 3 {
        return Err(io::Error::other("fixture stratum bound").into());
    }
    Ok(roster
        .iter()
        .map(|id| FixturePrincipalStratum::new(*id, 2))
        .collect::<Result<Vec<_>, _>>()?)
}

/// Preserve exact oracle arithmetic or reject its finite integer envelope.
/// # Errors
/// Reject an overflowing checked operation before it can affect a comparison.
fn checked_integer<T>(value: Option<T>) -> TestResult<T> {
    value.ok_or_else(|| io::Error::other("fixture rational integer bound").into())
}

/// Decode bounded actual stored binary64 points into exact integral 2^-60 units.
/// # Errors
/// Reject nonnormal, nonintegral or magnitude-above-two fixture values.
fn exact_units(value: f64) -> TestResult<i128> {
    if !value.is_finite() || value.abs() > 2.0 {
        return Err(io::Error::other("fixture exact point envelope").into());
    }
    if value == 0.0 {
        return Ok(0);
    }
    let bits = value.to_bits();
    let exponent = i32::try_from((bits >> 52) & 0x7ff)?;
    if exponent == 0 {
        return Err(io::Error::other("fixture exact point subnormal").into());
    }
    let mantissa = i128::from((bits & ((1_u64 << 52) - 1)) | (1_u64 << 52));
    let shift = exponent - 1023 - 52 + 60;
    let magnitude = if (0..=60).contains(&shift) {
        checked_integer(
            mantissa.checked_mul(checked_integer(1_i128.checked_shl(u32::try_from(shift)?))?),
        )?
    } else if (-52..0).contains(&shift) {
        let right = u32::try_from(-shift)?;
        if mantissa & ((1_i128 << right) - 1) != 0 {
            return Err(io::Error::other("fixture nonintegral exact point").into());
        }
        mantissa >> right
    } else {
        return Err(io::Error::other("fixture exact point precision").into());
    };
    Ok(if bits >> 63 == 0 {
        magnitude
    } else {
        -magnitude
    })
}

/// Compare exact stored-input error with the exact binary64 reported bound.
///
/// The reference denominator is `2 * scale * 2^60`, where scale is one or 10^9.
/// Factor scale into its power of two and odd part, then cross-multiply integers.
/// No rounded floating error estimate or production propagation equation is used.
/// # Errors
/// Reject unsupported fixture points/scales or any exact-integer overflow.
/// # Panics
/// Reject negative/nonfinite bounds or failure to contain the exact rational error.
fn covers_scaled_mean(
    point: f64,
    bound: f64,
    reference_numerator: i128,
    scale: u64,
) -> TestResult<u128> {
    if ![1, SECOND].contains(&scale) {
        return Err(io::Error::other("fixture rational scale").into());
    }
    let twice_point = checked_integer(exact_units(point)?.checked_mul(2))?;
    let represented = checked_integer(twice_point.checked_mul(i128::from(scale)))?;
    let error = checked_integer(represented.checked_sub(reference_numerator))?.unsigned_abs();
    assert!(bound.is_finite() && bound >= 0.0);
    if bound == 0.0 {
        assert_eq!(error, 0);
        return Ok(error);
    }
    let bits = bound.to_bits();
    let exponent_bits = i32::try_from((bits >> 52) & 0x7ff)?;
    let fraction = bits & ((1_u64 << 52) - 1);
    let (mantissa, exponent) = if exponent_bits == 0 {
        (fraction, -1074)
    } else {
        (fraction | (1_u64 << 52), exponent_bits - 1023 - 52)
    };
    let twos = scale.trailing_zeros();
    let odd = scale >> twos;
    let power = exponent + 61 + i32::try_from(twos)?;
    let factor = checked_integer(1_u128.checked_shl(power.unsigned_abs()))?;
    let weighted_bound = checked_integer(u128::from(mantissa).checked_mul(u128::from(odd)))?;
    let (left, right) = if power < 0 {
        (checked_integer(error.checked_mul(factor))?, weighted_bound)
    } else {
        (error, checked_integer(weighted_bound.checked_mul(factor))?)
    };
    assert!(left <= right);
    Ok(error)
}

/// Compare ideal-fit facts separately from new-arithmetic containment.
/// # Panics
/// Reject nonfinite/zero references or relative disagreement above 2^-38.
fn near_reference(actual: f64, expected: f64) {
    assert!(actual.is_finite() && expected.is_finite() && expected != 0.0);
    assert!((actual / expected - 1.0).abs() <= 1.0 / 274_877_906_944.0);
}

/// Derive exact equal-weight mean references from the actual original stored bits.
///
/// Equal positive distances and equal positive original rate-SD bits are checked
/// before canceling weights. Phase uses the exact one-nanosecond chart displacement;
/// rate uses the exact stored rates and chart F. Ideal fit values are not substituted.
/// # Errors
/// Preserve missing actual attachments or unsupported exact-integer operations.
/// # Panics
/// Reject unequal weight inputs, an uncovered actual mean error or a dyadic phase oracle.
fn check_pair_mean_bounds(
    admitted: &AdmittedChartSupporters<'_, '_, '_>,
    result: &ChartQuantities,
) -> TestResult {
    let a = admitted.record(label(11)?)?;
    let b = admitted.record(label(22)?)?;
    assert_eq!(
        a.original().distance_seconds().to_bits(),
        b.original().distance_seconds().to_bits()
    );
    assert!(a.original().distance_seconds() > 0.0);
    let at = a
        .original()
        .tracking()
        .ok_or_else(|| io::Error::other("missing original A fit"))?;
    let bt = b
        .original()
        .tracking()
        .ok_or_else(|| io::Error::other("missing original B fit"))?;
    assert_eq!(
        at.rate_standard_deviation_seconds_per_second().to_bits(),
        bt.rate_standard_deviation_seconds_per_second().to_bits()
    );
    assert!(at.rate_standard_deviation_seconds_per_second() > 0.0);
    let phases = checked_integer(
        exact_units(at.phase_seconds())?.checked_add(exact_units(bt.phase_seconds())?),
    )?;
    let twice_d = checked_integer(exact_units(4.0 / 1024.0)?.checked_mul(2))?;
    let shifted = checked_integer(phases.checked_sub(twice_d))?;
    let scaled = checked_integer(shifted.checked_mul(i128::from(SECOND)))?;
    let twice_elapsed = checked_integer(i128::from(2 * SECOND + 1).checked_mul(2))?;
    let drift = checked_integer(exact_units(1.0 / 1024.0)?.checked_mul(twice_elapsed))?;
    let phase_reference = checked_integer(scaled.checked_sub(drift))?;
    let rates = checked_integer(
        exact_units(a.original().phase_rate_seconds_per_second())?
            .checked_add(exact_units(b.original().phase_rate_seconds_per_second())?),
    )?;
    let twice_f = checked_integer(exact_units(1.0 / 1024.0)?.checked_mul(2))?;
    let rate_reference = checked_integer(rates.checked_sub(twice_f))?;
    let phase_error = covers_scaled_mean(
        result.points.phase_seconds,
        result.phase_error,
        phase_reference,
        SECOND,
    )?;
    covers_scaled_mean(
        result.points.phase_rate_seconds_per_second,
        result.rate_error,
        rate_reference,
        1,
    )?;
    // The odd 5^9 denominator cannot cancel the one-nanosecond nonzero F term.
    assert!(phase_error > 0);
    Ok(())
}

/// Copy every observable completed point and separate bound as exact bits.
fn quantity_bits(value: &ChartQuantities) -> [u64; 7] {
    [
        value.points.phase_seconds,
        value.points.phase_rate_seconds_per_second,
        value.points.phase_standard_deviation_seconds,
        value.points.rate_standard_deviation_seconds_per_second,
        value.points.tracking_skew_seconds_per_second,
        value.phase_error,
        value.rate_error,
    ]
    .map(f64::to_bits)
}

/// Check immutable actual slots and independently expected original metadata.
/// # Errors
/// Preserve missing actual record/attachment failures without substituting values.
/// # Panics
/// Reject altered wrapper/attachment identity, raw provenance, SD/skew or phase mode.
fn check_original_records(
    admitted: &AdmittedChartSupporters<'_, '_, '_>,
    entries: &[FixtureChartInterval<'_>; 2],
) -> TestResult {
    for (index, (id, expected_skew)) in [(label(11)?, 0.125_f64), (label(22)?, 0.25_f64)]
        .into_iter()
        .enumerate()
    {
        let record = admitted.record(id)?;
        let original = admitted.originals().record(id)?;
        assert!(ptr::eq(original.original(), record.original()));
        assert_eq!(
            original.tracking_skew_seconds_per_second().to_bits(),
            expected_skew.to_bits()
        );
        assert!(ptr::eq(record.entry(), ptr::from_ref(&entries[index])));
        assert!(ptr::eq(
            record.tracking(),
            entries[index]
                .projection()
                .tracking()
                .ok_or_else(|| io::Error::other("missing source attachment"))?
        ));
        assert_eq!(record.original().context().source_id(), id);
        assert_eq!(record.original().context().frame(), label(17)?);
        assert_eq!(record.original().context().generation(), label(23)?);
        assert_eq!(
            record.original().projected_at(),
            MonotonicNs::new(4 * SECOND)
        );
        assert_eq!(
            record.original().distance_seconds().to_bits(),
            0.5_f64.to_bits()
        );
        assert_eq!(
            record
                .tracking()
                .tracking_skew_seconds_per_second()
                .to_bits(),
            expected_skew.to_bits()
        );
        let original_tracking = record
            .original()
            .tracking()
            .ok_or_else(|| io::Error::other("missing original tracking"))?;
        assert_eq!(
            record
                .tracking()
                .phase_standard_deviation_seconds()
                .to_bits(),
            original_tracking
                .phase_standard_deviation_seconds()
                .to_bits()
        );
        assert_eq!(
            record
                .tracking()
                .rate_standard_deviation_seconds_per_second()
                .to_bits(),
            original_tracking
                .rate_standard_deviation_seconds_per_second()
                .to_bits()
        );
        assert_ne!(
            record.original().phase_rate_seconds_per_second().to_bits(),
            record
                .entry()
                .projection()
                .phase_rate_seconds_per_second()
                .to_bits()
        );
        assert_ne!(
            record.tracking().phase_seconds().to_bits(),
            record.entry().projection().center_seconds().to_bits()
        );
    }
    Ok(())
}

/// Actual immutable chart slots, distinct equal descriptors and retained reception.
/// # Errors
/// Preserve real producer and receiver failures.
/// # Panics
/// Reject altered source identity, witness population, attachment or descriptor borrow.
#[test]
fn chart_tracking_receiver_keeps_actual_slots_and_complete_witness() -> TestResult {
    let a = source(11, [0, 2, 0])?;
    let b = source(22, [1, 3, 1])?;
    let map = chart(17, 23)?;
    let equal_map = chart(17, 23)?;
    let entries = [
        entry(&a, &map, 4, 0.25, true)?,
        entry_with_tracking_skew(&b, &equal_map, 4, 0.25, true, 0.25)?,
    ];
    let mut owner = seed(&entries)?;
    let strata = strata(&[label(11)?, label(22)?])?;
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &entries)?;
    for expected_kind in [
        FixturePrincipalOutcomeKind::ReferenceReady,
        FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate,
    ] {
        let decision = owner.advance_chart(&map, &batch, &strata)?;
        assert_eq!(decision.kind(), expected_kind);
        let holder = ChartTrackingSupporters::from_decision(&decision)?;
        let admitted = admit_chart_source(&decision, &holder)?
            .ok_or_else(|| io::Error::other("missing admitted chart witness"))?;
        assert_eq!(admitted.count(), 2);
        assert_eq!(admitted.originals().count(), 2);
        assert_eq!(
            admitted.originals().labels().collect::<Vec<_>>(),
            [label(11)?, label(22)?]
        );
        assert_eq!(
            admitted.labels().collect::<Vec<_>>(),
            [label(11)?, label(22)?]
        );
        assert!(ptr::eq(admitted.selection(), decision.selection()));
        assert!(ptr::eq(admitted.principal(), ptr::from_ref(&entries[0])));
        assert!(ptr::eq(
            admitted.principal_original(),
            entries[0].projection().source_projection()
        ));
        assert!(ptr::eq(
            admitted.originals().principal_original(),
            admitted.principal_original()
        ));
        check_original_records(&admitted, &entries)?;
        assert!(ptr::eq(
            admitted.record(label(11)?)?.entry().projection().chart(),
            ptr::from_ref(&map)
        ));
        assert!(ptr::eq(
            admitted.record(label(22)?)?.entry().projection().chart(),
            ptr::from_ref(&equal_map)
        ));
        assert!(admitted.record(label(999)?).is_err());
    }
    Ok(())
}

/// Challenge actual wrapper, attachment, duplicate and witness relationships.
/// # Errors
/// Preserve genuine missing-attachment failures in the negative fixture setup.
/// # Panics
/// Mandatory Garde and typed receiving must reject the specified first cause.
fn check_entry_associations(
    decision: &FixtureChartPrincipalDecision<'_, '_, '_>,
    holder: &ChartTrackingSupporters<'_, '_>,
    entries: &[FixtureChartInterval<'_>; 3],
) -> TestResult {
    let copied = entries[0];
    let actual = decision
        .principal_entry()
        .ok_or_else(|| io::Error::other("missing genuine principal"))?;
    assert!(ptr::eq(actual, ptr::from_ref(&entries[0])));
    assert!(!ptr::eq(actual, ptr::from_ref(&copied)));
    let copied_context = copied.projection().source_projection().context();
    let actual_context = actual.projection().source_projection().context();
    assert_eq!(copied_context, actual_context);
    let copied_label = copied_context.source_id();
    assert_eq!(decision.principal_label(), Some(copied_label));
    let witnessed = decision
        .selection()
        .witnessed()
        .ok_or_else(|| io::Error::other("missing genuine witness"))?;
    assert!(witnessed.supporters().any(|label| label == copied_label));
    let raw = RawChartSourceAdmission {
        inputs: (decision, holder, &copied),
    };
    assert!(raw.validate().is_err());
    assert_eq!(
        admit_chart_records(&raw.inputs),
        Err(FixtureCombinationError::Invariant)
    );

    let mut bad = ChartTrackingSupporters {
        records: holder.records,
        count: holder.count,
    };
    bad.records[0] = Some(ChartTrackingRecord {
        entry: &copied,
        tracking: copied
            .projection()
            .tracking()
            .ok_or_else(|| io::Error::other("copied tracking"))?,
    });
    let raw = RawChartSourceAdmission {
        inputs: (decision, &bad, &entries[0]),
    };
    assert!(raw.validate().is_err());
    assert_eq!(
        admit_chart_records(&raw.inputs),
        Err(FixtureCombinationError::EntryMismatch { index: 0 })
    );
    assert_eq!(
        admit_chart_source(decision, &bad).err(),
        Some(FixtureTrackingCombinationError::Combination {
            source: FixtureCombinationError::EntryMismatch { index: 0 }
        })
    );

    let mut bad = ChartTrackingSupporters {
        records: holder.records,
        count: holder.count,
    };
    bad.records[1] = bad.records[0];
    assert_eq!(
        admit_chart_records(&(decision, &bad, &entries[0])),
        Err(FixtureCombinationError::DuplicateUncertainty { index: 1 })
    );

    let mut bad = ChartTrackingSupporters {
        records: holder.records,
        count: holder.count,
    };
    bad.records[0] = Some(ChartTrackingRecord {
        entry: &entries[0],
        tracking: entries[1]
            .projection()
            .tracking()
            .ok_or_else(|| io::Error::other("other tracking"))?,
    });
    let raw = RawChartSourceAdmission {
        inputs: (decision, &bad, &entries[0]),
    };
    assert!(raw.validate().is_err());
    assert_eq!(
        admit_chart_records(&raw.inputs),
        Err(FixtureCombinationError::Invariant)
    );
    assert_eq!(
        admit_chart_source(decision, &bad).err(),
        Some(FixtureTrackingCombinationError::Combination {
            source: FixtureCombinationError::Invariant
        })
    );

    let mut bad = ChartTrackingSupporters {
        records: holder.records,
        count: holder.count,
    };
    bad.records[0] = Some(ChartTrackingRecord {
        entry: &entries[2],
        tracking: entries[2]
            .projection()
            .tracking()
            .ok_or_else(|| io::Error::other("outside tracking"))?,
    });
    assert_eq!(
        admit_chart_records(&(decision, &bad, &entries[0])),
        Err(FixtureCombinationError::UnexpectedUncertainty { index: 0 })
    );

    Ok(())
}

/// Challenge complete metadata occupancy, capacity, padding and principal identity.
/// # Panics
/// Reject incomplete/over-capacity holders, hidden padding and a wrong actual principal.
fn check_receiver_counts(
    decision: &FixtureChartPrincipalDecision<'_, '_, '_>,
    holder: &ChartTrackingSupporters<'_, '_>,
    entries: &[FixtureChartInterval<'_>; 3],
) {
    for count in [0, 1, MAX_SUPPORTERS + 1] {
        let bad = ChartTrackingSupporters {
            records: holder.records,
            count,
        };
        let expected = if count > MAX_SUPPORTERS {
            FixtureCombinationError::CapacityExceeded
        } else {
            FixtureCombinationError::MetadataCountMismatch
        };
        assert_eq!(
            admit_chart_source(decision, &bad).err(),
            Some(FixtureTrackingCombinationError::Combination { source: expected })
        );
    }
    let mut bad = ChartTrackingSupporters {
        records: holder.records,
        count: holder.count,
    };
    bad.records[2] = bad.records[0];
    assert_eq!(
        admit_chart_records(&(decision, &bad, &entries[0])),
        Err(FixtureCombinationError::Invariant)
    );
    assert_eq!(
        admit_chart_records(&(decision, holder, &entries[1])),
        Err(FixtureCombinationError::Invariant)
    );
}

/// Challenge each raw context field before a wrong-address failure can mask it.
/// # Errors
/// Preserve the three genuine foreign-source construction and projection failures.
/// # Panics
/// Preserve frame, generation and query mismatch identities at canonical index zero.
fn check_raw_context(
    decision: &FixtureChartPrincipalDecision<'_, '_, '_>,
    holder: &ChartTrackingSupporters<'_, '_>,
    entries: &[FixtureChartInterval<'_>; 3],
) -> TestResult {
    for (frame, generation, query, field) in [
        (99, 23, 4, FixtureCombinationContextField::Frame),
        (17, 99, 4, FixtureCombinationContextField::MappingGeneration),
        (17, 23, 5, FixtureCombinationContextField::ProjectedAt),
    ] {
        let foreign = Source::new(
            FixtureContext::new(label(44)?, label(frame)?, label(generation)?),
            [0, 2, 0],
        )?;
        let foreign_map = chart(frame, generation)?;
        let foreign_entry = entry(&foreign, &foreign_map, query, 0.25, true)?;
        let mut bad = ChartTrackingSupporters {
            records: holder.records,
            count: holder.count,
        };
        bad.records[0] = Some(ChartTrackingRecord {
            entry: &foreign_entry,
            tracking: foreign_entry
                .projection()
                .tracking()
                .ok_or_else(|| io::Error::other("foreign tracking"))?,
        });
        assert_eq!(
            admit_chart_records(&(decision, &bad, &entries[0])),
            Err(FixtureCombinationError::ContextMismatch { index: 0, field })
        );
    }
    Ok(())
}

/// Genuine values at wrong addresses and attachments must fail receiving.
/// # Errors
/// Preserve real setup failures without forging product decisions or selections.
/// # Panics
/// The mandatory rule and typed projection retain first-error order and exact membership.
#[test]
fn chart_tracking_receiver_rejects_wrong_relationships() -> TestResult {
    let a = source(11, [0, 2, 0])?;
    let b = source(22, [1, 3, 1])?;
    let outside = source(33, [15, 15, 15])?;
    let map = chart(17, 23)?;
    let entries = [
        entry(&a, &map, 4, 0.25, true)?,
        entry(&b, &map, 4, 0.25, true)?,
        entry(&outside, &map, 4, 0.0, true)?,
    ];
    // C's raw lower endpoint is11/16; B's upper is31/48. Their exact
    // 1/24-second gap survives the common chart shift and outward enclosure.
    assert!(entries[2].interval().lower() > entries[0].interval().upper());
    assert!(entries[2].interval().lower() > entries[1].interval().upper());
    let mut owner = seed(&entries)?;
    let strata = strata(&[label(11)?, label(22)?, label(33)?])?;
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &entries)?;
    let decision = owner.advance_chart(&map, &batch, &strata)?;
    assert_eq!(
        decision
            .selection()
            .witnessed()
            .ok_or_else(|| io::Error::other("missing actual pair witness"))?
            .supporters()
            .collect::<Vec<_>>(),
        [label(11)?, label(22)?]
    );
    let holder = ChartTrackingSupporters::from_decision(&decision)?;
    assert_eq!(holder.count, 2);
    let healthy = RawChartSourceAdmission {
        inputs: (&decision, &holder, &entries[0]),
    };
    assert!(healthy.validate().is_ok());
    let admitted = admit_chart_source(&decision, &holder)?
        .ok_or_else(|| io::Error::other("missing healthy original metadata"))?;
    assert_eq!(admitted.originals().count(), 2);
    assert!(admitted.originals().record(label(33)?).is_err());

    check_entry_associations(&decision, &holder, &entries)?;
    check_receiver_counts(&decision, &holder, &entries);
    check_raw_context(&decision, &holder, &entries)?;
    Ok(())
}

/// Complete admission must not drop tracking from a later distance exclusion.
/// # Errors
/// Preserve genuine histories, partial projections and actual chart decision failures.
/// # Panics
/// Ready and retained reception both require the same later-excluded attachment.
#[test]
fn chart_tracking_receiver_requires_excluded_and_retained_tracking() -> TestResult {
    let a = source(11, [0, 2, 0])?;
    let b = source(22, [1, 3, 1])?;
    let map = chart(17, 23)?;
    let entries = [
        entry(&a, &map, 4, 0.25, true)?,
        entry(&b, &map, 4, 1.5, false)?,
    ];
    let mut owner = seed(&entries)?;
    let strata = strata(&[label(11)?, label(22)?])?;
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &entries)?;
    assert!(
        entries[1]
            .projection()
            .source_projection()
            .distance_seconds()
            > 2.0
                * (entries[0]
                    .projection()
                    .source_projection()
                    .distance_seconds()
                    + 1.0 / 1024.0)
    );
    for expected_kind in [
        FixturePrincipalOutcomeKind::ReferenceReady,
        FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate,
    ] {
        let decision = owner.advance_chart(&map, &batch, &strata)?;
        assert_eq!(decision.kind(), expected_kind);
        assert_eq!(
            decision
                .selection()
                .witnessed()
                .ok_or_else(|| io::Error::other("missing genuine witness"))?
                .supporters()
                .len(),
            2
        );
        assert_eq!(
            ChartTrackingSupporters::from_decision(&decision).err(),
            Some(FixtureTrackingCombinationError::MissingTracking { index: 1 })
        );
    }
    Ok(())
}

/// Genuine absence has no invented tracking requirement or numerical fallback.
/// # Errors
/// Preserve actual source, raw seed, empty chart batch and selection failures.
/// # Panics
/// The honest absent decision admits exactly an empty metadata holder.
#[test]
fn chart_tracking_receiver_absence_requires_empty_metadata() -> TestResult {
    let a = source(11, [0, 2, 0])?;
    let b = source(22, [1, 3, 1])?;
    let map = chart(17, 23)?;
    let entries = [
        entry(&a, &map, 4, 0.25, false)?,
        entry(&b, &map, 4, 0.25, false)?,
    ];
    let mut owner = seed(&entries)?;
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &[])?;
    let decision = owner.advance_chart(&map, &batch, &[])?;
    assert!(decision.principal_entry().is_none());
    let holder = ChartTrackingSupporters::from_decision(&decision)?;
    assert_eq!(holder.count, 0);
    assert!(admit_chart_source(&decision, &holder)?.is_none());
    let bad = ChartTrackingSupporters {
        records: holder.records,
        count: 1,
    };
    assert_eq!(
        admit_chart_source(&decision, &bad).err(),
        Some(FixtureTrackingCombinationError::Combination {
            source: FixtureCombinationError::MetadataCountMismatch
        })
    );
    let tracked = entry(&a, &map, 4, 0.25, true)?;
    let mut bad = ChartTrackingSupporters {
        records: holder.records,
        count: 0,
    };
    bad.records[0] = Some(ChartTrackingRecord {
        entry: &tracked,
        tracking: tracked
            .projection()
            .tracking()
            .ok_or_else(|| io::Error::other("missing forbidden absent metadata"))?,
    });
    assert_eq!(
        admit_chart_source(&decision, &bad).err(),
        Some(FixtureTrackingCombinationError::Combination {
            source: FixtureCombinationError::Invariant
        })
    );
    Ok(())
}

/// Reduce two genuine actual entries in canonical order with an exact non-dyadic oracle.
/// # Errors
/// Preserve actual producer, receiving, reduction and rational-oracle failures.
/// # Panics
/// Reject raw-anchor substitution, changed weight inputs or uncovered arithmetic error.
#[test]
fn chart_tracking_reduction_contains_exact_stored_input_means() -> TestResult {
    let a = source(11, [0, 2, 0])?;
    let b = source(22, [1, 3, 1])?;
    let map = chart(17, 23)?;
    let at = MonotonicNs::new(4 * SECOND + 1);
    let entries = [
        entry_at_with_tracking_skew(&a, &map, at, 0.25, true, 0.125)?,
        entry_at_with_tracking_skew(&b, &map, at, 0.25, true, 0.25)?,
    ];
    let mut owner = seed(&entries)?;
    let strata = strata(&[label(11)?, label(22)?])?;
    let batch = FixtureChartBatch::new(&map, at, &entries)?;
    let decision = owner.advance_chart(&map, &batch, &strata)?;
    assert_eq!(decision.kind(), FixturePrincipalOutcomeKind::ReferenceReady);
    let holder = ChartTrackingSupporters::from_decision(&decision)?;
    let admitted = admit_chart_source(&decision, &holder)?
        .ok_or_else(|| io::Error::other("missing genuine reduction witness"))?;
    assert_eq!(
        admitted.labels().collect::<Vec<_>>(),
        [label(11)?, label(22)?]
    );
    assert!(matches!(
        ChartQuantities::singleton(&admitted),
        Err(ChartReductionError::Invariant)
    ));
    let principal = admitted.record(label(11)?)?.tracking().phase_seconds();
    let mut sums = ChartSums::empty()?;
    for (index, id) in admitted.labels().enumerate() {
        sums = sums.with_record(admitted.record(id)?, principal, index)?;
    }
    let result = sums.finish()?;
    check_pair_mean_bounds(&admitted, &result)?;
    let ideal_rate = 125.0 / 3072.0;
    let nanosecond_increment = ideal_rate / 1_000_000_000.0;
    near_reference(
        result.points.phase_seconds,
        1013.0 / 4608.0 + nanosecond_increment,
    );
    near_reference(result.points.phase_rate_seconds_per_second, ideal_rate);
    let raw_mean = 167.0 / 1536.0 + nanosecond_increment;
    assert!((result.points.phase_seconds - raw_mean).abs() > 0.1);
    Ok(())
}

/// A genuine second record with zero rate SD cannot change the existing valid sums.
/// # Errors
/// Preserve actual producer, receiving and healthy reduction failures.
/// # Panics
/// Require the original-witness denominator rejection and bit-identical prior results.
#[test]
fn chart_tracking_reduction_failure_preserves_prior_sums() -> TestResult {
    let a = source(11, [0, 2, 0])?;
    let b = source(22, [1, 1, 1])?;
    let map = chart(17, 23)?;
    let entries = [
        entry(&a, &map, 4, 0.25, true)?,
        entry(&b, &map, 4, 0.25, true)?,
    ];
    let mut owner = seed(&entries)?;
    let strata = strata(&[label(11)?, label(22)?])?;
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &entries)?;
    let decision = owner.advance_chart(&map, &batch, &strata)?;
    let holder = ChartTrackingSupporters::from_decision(&decision)?;
    let admitted = admit_chart_source(&decision, &holder)?
        .ok_or_else(|| io::Error::other("missing genuine pair for failure"))?;
    assert_eq!(admitted.count(), 2);
    let first = admitted.record(label(11)?)?;
    let second = admitted.record(label(22)?)?;
    assert_eq!(
        second
            .tracking()
            .rate_standard_deviation_seconds_per_second(),
        0.0
    );
    let principal_phase = first.tracking().phase_seconds();
    let partial = ChartSums::empty()?.with_record(first, principal_phase, 0)?;
    let before = quantity_bits(&partial.finish()?);
    assert!(matches!(
        partial.with_record(second, principal_phase, 1),
        Err(ChartReductionError::Combination(
            FixtureCombinationError::ZeroDenominator {
                index: 1,
                field: FixtureCombinationDenominatorField::RateStandardDeviation,
            }
        ))
    ));
    assert_eq!(quantity_bits(&partial.finish()?), before);
    // These are raw legal error values, not additional numerical reductions.
    // Every delegated cause must be the payload actually stored in this error.
    let combination = ChartReductionError::Combination(FixtureCombinationError::ZeroDenominator {
        index: 1,
        field: FixtureCombinationDenominatorField::RateStandardDeviation,
    });
    let ChartReductionError::Combination(stored) = &combination else {
        return Err(io::Error::other("unexpected raw combination error").into());
    };
    let cause = combination
        .source()
        .and_then(|error| error.downcast_ref::<FixtureCombinationError>())
        .ok_or_else(|| io::Error::other("missing stored combination cause"))?;
    assert!(ptr::eq(cause, stored));
    let companion = ChartReductionError::Companion(CompanionError::InvalidError);
    let ChartReductionError::Companion(stored) = &companion else {
        return Err(io::Error::other("unexpected raw companion error").into());
    };
    let cause = companion
        .source()
        .and_then(|error| error.downcast_ref::<CompanionError>())
        .ok_or_else(|| io::Error::other("missing stored companion cause"))?;
    assert!(ptr::eq(cause, stored));
    assert!(ChartReductionError::Invariant.source().is_none());
    Ok(())
}

/// A genuine original singleton copies transformed attachments without denominator work.
/// # Errors
/// Preserve actual zero-history, raw seed, chart projection and receiving failures.
/// # Panics
/// Reject changed scalar/error bits or accidental denominator requirements.
#[test]
fn chart_tracking_singleton_copies_zero_denominator_attachment_bits() -> TestResult {
    let zero = source(11, [0, 0, 0])?;
    let map = chart(17, 23)?;
    let at = MonotonicNs::new(4 * SECOND);
    let view = zero
        .history
        .prepare_chart(zero.context, FixtureChartOutcome::Known(&map))?
        .ok_or_else(|| io::Error::other("missing genuine zero view"))?;
    let projection = view.project_tracking(
        view.context(),
        2,
        MonotonicNs::new(2 * SECOND),
        at,
        FixtureProjectionTerms::new(0.0, 0.0, 0.0)?,
        0.0,
    )?;
    let entries = [enclose_fixture_chart_projection(&projection)?];
    let mut owner = seed_with_minimum(&entries, 1)?;
    let strata = strata(&[label(11)?])?;
    let batch = FixtureChartBatch::new(&map, at, &entries)?;
    let decision = owner.advance_chart(&map, &batch, &strata)?;
    assert_eq!(decision.kind(), FixturePrincipalOutcomeKind::ReferenceReady);
    let holder = ChartTrackingSupporters::from_decision(&decision)?;
    let admitted = admit_chart_source(&decision, &holder)?
        .ok_or_else(|| io::Error::other("missing genuine singleton witness"))?;
    assert_eq!(admitted.originals().count(), 1);
    let record = admitted.record(label(11)?)?;
    let tracking = record.tracking();
    assert_eq!(record.original().distance_seconds(), 0.0);
    assert_eq!(tracking.rate_standard_deviation_seconds_per_second(), 0.0);
    assert_eq!(tracking.tracking_skew_seconds_per_second(), 0.0);
    let result = ChartQuantities::singleton(&admitted)?;
    assert_eq!(
        quantity_bits(&result),
        [
            tracking.phase_seconds(),
            record.entry().projection().phase_rate_seconds_per_second(),
            tracking.phase_standard_deviation_seconds(),
            tracking.rate_standard_deviation_seconds_per_second(),
            tracking.tracking_skew_seconds_per_second(),
            tracking.phase_roundoff_seconds(),
            record
                .entry()
                .projection()
                .rate_roundoff_seconds_per_second(),
        ]
        .map(f64::to_bits)
    );
    assert_eq!(
        result.points.phase_seconds.to_bits(),
        (-3.0_f64 / 512.0).to_bits()
    );
    assert_eq!(
        result.points.phase_rate_seconds_per_second.to_bits(),
        (-1.0_f64 / 1024.0).to_bits()
    );
    Ok(())
}

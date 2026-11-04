// SPDX-License-Identifier: GPL-2.0-only
//! Independent S154 mathematical-view fixture checks.
//!
//! The fixed source-only upper is ten histories, 96 observations/appends,
//! 24 chart constructors, 19 prepare calls, 17 projections, seven enclosures,
//! and at most 20 snapshots of at most 64 rows. These are unobserved source
//! bounds; setup/indirect selector work must be prepaid by the existing owner.
//! Actual operation counts and results require a configured Rust test run.
#![forbid(unsafe_code)]

use std::{error::Error, io, num::NonZeroU64};

use time_core::{
    FixtureAdaptiveChartView, FixtureAdaptiveHistory, FixtureAffineChart, FixtureChartDomain,
    FixtureChartError, FixtureChartField, FixtureChartFrames, FixtureChartInterval,
    FixtureChartIssue, FixtureChartOutcome, FixtureChartProjection, FixtureChartTracking,
    FixtureContext, FixtureEnclosureError, FixtureEndpointRounding, FixtureObservation,
    FixtureProjectionError, FixtureProjectionTerms, enclose_fixture_chart_projection,
};
use time_domain::{MonotonicNs, UtcCoordinateQ32, exchange_arithmetic};

/// Redacted bounded fixture or genuine checked-owner error.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Exact held instant, Q33 numerator and variance-divisor bits for one row.
type RowSnapshot = (u64, i128, u64);
/// Retained rows, fitted/auxiliary counts and unchanged fitted coefficient bits.
type SourceSnapshot = (Vec<RowSnapshot>, usize, usize, Option<(u64, u64)>);
/// Exact monotonic nanoseconds per fixture second.
const SECOND: u64 = 1_000_000_000;
/// Independent fitted-fixture relative comparison, frozen before product output.
const RELATIVE_TOLERANCE: f64 = 1.0 / 274_877_906_944.0;

/// Admit a nonzero fixture label without effects.
/// # Errors
/// Reject zero.
fn label(value: u64) -> TestResult<NonZeroU64> {
    NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero fixture label").into())
}

/// Compare a finite nonzero fitted result with the independent rational oracle.
/// # Panics
/// Reject a zero, nonfinite, or relatively inaccurate fitted value.
fn near(actual: f64, expected: f64) {
    assert!(actual.is_finite() && expected.is_finite() && expected != 0.0);
    assert!((actual / expected - 1.0).abs() <= RELATIVE_TOLERANCE);
}

/// One genuine adaptive owner whose exact input rows remain accessible.
struct Source {
    /// Exact caller-asserted labels shared by its genuine observations.
    context: FixtureContext,
    /// Sole owner of the actual staged and committed fixture history.
    history: FixtureAdaptiveHistory,
}

impl Source {
    /// Construct a bounded owner from exact dyadic input phases.
    /// # Errors
    /// Reject inconsistent counts or preserve constructor and append failures.
    fn new(
        start: u64,
        values: &[i32],
        shift: u32,
        divisors: &[f64],
        minimum: usize,
    ) -> TestResult<Self> {
        Self::with_context(
            FixtureContext::new(label(11)?, label(17)?, label(23)?),
            start,
            values,
            shift,
            divisors,
            minimum,
        )
    }

    /// Admit actual rows in an explicit source/frame/generation context.
    /// # Errors
    /// Preserve row bounds and genuine history/observation admission failures.
    fn with_context(
        context: FixtureContext,
        start: u64,
        values: &[i32],
        shift: u32,
        divisors: &[f64],
        minimum: usize,
    ) -> TestResult<Self> {
        if values.len() != divisors.len() || values.len() > 64 {
            return Err(io::Error::other("fixture row bound").into());
        }
        let mut source = Self {
            context,
            history: FixtureAdaptiveHistory::new(context, minimum)?,
        };
        for (index, (&value, &divisor)) in values.iter().zip(divisors).enumerate() {
            let second = start
                .checked_add(u64::try_from(index)?)
                .ok_or_else(|| io::Error::other("fixture time"))?;
            source.append(second, value, shift, divisor)?;
        }
        Ok(source)
    }

    /// Append one real exchange-derived Q33 phase at an exact whole second.
    /// # Errors
    /// Preserve domain, observation and full adaptive-history admission failures.
    fn append(&mut self, second: u64, value: i32, shift: u32, divisor: f64) -> TestResult {
        let ticks = i128::from(value)
            .checked_shl(shift)
            .ok_or_else(|| io::Error::other("fixture phase"))?;
        let time = second
            .checked_mul(SECOND)
            .ok_or_else(|| io::Error::other("fixture time"))?;
        let zero = UtcCoordinateQ32::new(0)?;
        let shifted = UtcCoordinateQ32::new(ticks)?;
        let phase = exchange_arithmetic(zero, shifted, shifted, zero)?.offset();
        let row = FixtureObservation::new(self.context, MonotonicNs::new(time), phase, divisor)?;
        self.history.try_append(row)?;
        Ok(())
    }
}

#[path = "private/chart_batch_cases.rs"]
mod chart_batch_cases;

/// Existing raw-source principal continuity through original chart batches.
#[path = "private/chart_principal_cases.rs"]
mod chart_principal_cases;

/// Already pending supporter credit remains usable through the chart receiver.
#[path = "private/chart_principal_pending.rs"]
mod chart_principal_pending;

/// Same-reference composition retains numeric error through real chart consumers.
#[path = "private/chart_composition.rs"]
mod chart_composition;

/// Capture every real retained row and the owner report, without creating evidence.
/// # Errors
/// Reject impossible public counts or a missing required row.
fn snapshot(source: &Source) -> TestResult<SourceSnapshot> {
    let sample = source.history.sample_count();
    let auxiliary = source.history.auxiliary_count();
    let total = sample
        .checked_add(auxiliary)
        .ok_or_else(|| io::Error::other("count overflow"))?;
    if total > 64 || auxiliary > sample {
        return Err(io::Error::other("invalid owner count").into());
    }
    let mut rows = Vec::with_capacity(total);
    for index in 0..total {
        let row = source
            .history
            .decision_observation(index)
            .ok_or_else(|| io::Error::other("missing row"))?;
        rows.push((
            row.observed_at().nanoseconds(),
            row.phase().numerator(),
            row.variance_divisor().to_bits(),
        ));
    }
    let report = source.history.report().map(|r| {
        let line = r.selection().statistics().line();
        (
            line.phase_seconds().to_bits(),
            line.phase_rate_seconds_per_second().to_bits(),
        )
    });
    Ok((rows, sample, auxiliary, report))
}

/// Fixed explicit ample geometry; it is a fixture term, not a product default.
/// # Errors
/// Preserve checked projection-term admission.
fn terms() -> TestResult<FixtureProjectionTerms> {
    Ok(FixtureProjectionTerms::new(0.25, 0.0, 0.125)?)
}

/// Convert a nonzero bounded normal binary64 to exact integral 2^-60 units.
/// # Errors
/// Reject nonfinite, out-of-range, or nonintegral fixture values.
fn exact_units(value: f64) -> TestResult<i128> {
    if value == 0.0 {
        return Ok(0);
    }
    let bits = value.to_bits();
    let exponent = i32::try_from((bits >> 52) & 0x7ff)?;
    if exponent == 0 || exponent == 0x7ff {
        return Err(io::Error::other("fixture nonnormal").into());
    }
    let mantissa = i128::from((bits & ((1_u64 << 52) - 1)) | (1_u64 << 52));
    let shift = exponent - 1023 - 52 + 60;
    let magnitude = if (0..=60).contains(&shift) {
        mantissa
            .checked_shl(u32::try_from(shift)?)
            .ok_or_else(|| io::Error::other("fixture integer bound"))?
    } else if (-52..0).contains(&shift) {
        let right = u32::try_from(-shift)?;
        let mask = (1_i128 << right) - 1;
        if mantissa & mask != 0 {
            return Err(io::Error::other("nonintegral fixture units").into());
        }
        mantissa >> right
    } else {
        return Err(io::Error::other("fixture precision bound").into());
    };
    Ok(if bits >> 63 == 0 {
        magnitude
    } else {
        -magnitude
    })
}

/// Compare the actual rounded result with the exact stored-bit dyadic reference.
/// # Errors
/// Reject a fixture value outside the small exact-integer oracle envelope.
/// # Panics
/// Reject a negative/nonfinite bound or an error larger than its public bound.
fn covers_subtraction(actual: f64, old: f64, exact_delta: f64, bound: f64) -> TestResult {
    let error = exact_units(actual)? - (exact_units(old)? - exact_units(exact_delta)?);
    let magnitude = u32::try_from(error.unsigned_abs())?;
    let error_seconds = f64::from(magnitude) / 1_152_921_504_606_846_976.0;
    assert!(bound.is_finite() && bound >= 0.0 && error_seconds <= bound);
    Ok(())
}

/// Check the frozen one-nanosecond affine displacement with an integer oracle.
///
/// The exact error numerator has units `1 / (10^9 * 2^60)` seconds. Outward
/// conversion bounds that error independently of the production transform.
/// # Errors
/// Reject a value outside the declared exact fixture-integer envelope.
/// # Panics
/// Reject zero observed error or a reported bound below the exact error's upper.
fn covers_nanosecond_drift(actual: f64, old: f64, bound: f64) -> TestResult {
    let old = exact_units(old)?;
    let actual = exact_units(actual)?;
    let base = exact_units(6.0 / 1024.0)?;
    let drift = exact_units(1.0 / 1024.0)?;
    let error = ((actual - old + base) * 1_000_000_000 + drift).unsigned_abs();
    assert!(error > 0);
    let high = u32::try_from(error >> 32)?;
    let low = u32::try_from(error & u128::from(u32::MAX))?;
    // The high-word multiplication is exact; each inexact step rounds outward.
    let numerator_upper = (f64::from(high) * 4_294_967_296.0 + f64::from(low)).next_up();
    let scaled_upper = (numerator_upper / 1_000_000_000.0).next_up();
    let error_seconds = (scaled_upper / 1_152_921_504_606_846_976.0).next_up();
    assert!(bound.is_finite() && error_seconds <= bound);
    Ok(())
}

/// Create one explicit source-to-target affine chart with inclusive whole-second domain.
/// # Errors
/// Preserve frame, domain, and scalar admission failures.
fn chart(start: u64, reference: u64, end: u64, d: f64, f: f64) -> TestResult<FixtureAffineChart> {
    let at = |value: u64| -> TestResult<MonotonicNs> {
        Ok(MonotonicNs::new(
            value
                .checked_mul(SECOND)
                .ok_or_else(|| io::Error::other("time bound"))?,
        ))
    };
    let frames = FixtureChartFrames::new(label(17)?, label(23)?, label(19)?, label(29)?)?;
    let domain = FixtureChartDomain::new(at(start)?, at(reference)?, at(end)?)?;
    Ok(FixtureAffineChart::new(frames, domain, d, f)?)
}

/// Return a matching target frame while preserving the actual source identity.
/// # Errors
/// Reject a zero fixed fixture label.
fn target() -> TestResult<FixtureContext> {
    Ok(FixtureContext::new(label(11)?, label(19)?, label(29)?))
}

/// Check S144's literal mapping metadata and the actual borrowed descriptor.
/// # Panics
/// Reject changed source/target labels, common reference, scalars or descriptor identity.
fn assert_s144_descriptor(map: &FixtureAffineChart, view: &FixtureAdaptiveChartView<'_, '_>) {
    let frames = map.frames();
    assert_eq!(frames.source_frame().get(), 17);
    assert_eq!(frames.source_generation().get(), 23);
    assert_eq!(frames.target_frame().get(), 19);
    assert_eq!(frames.target_generation().get(), 29);
    let domain = map.domain();
    assert_eq!(domain.start().nanoseconds(), 0);
    assert_eq!(domain.common_reference().nanoseconds(), 2 * SECOND);
    assert_eq!(domain.end().nanoseconds(), 4 * SECOND);
    assert_eq!(
        map.phase_at_reference_seconds().to_bits(),
        (4.0_f64 / 1024.0).to_bits()
    );
    assert_eq!(
        map.rate_seconds_per_second().to_bits(),
        (1.0_f64 / 1024.0).to_bits()
    );
    assert!(std::ptr::eq(view.chart(), map));
}

/// Check S144's independently fixed represented geometry at the existing consumer.
/// # Panics
/// Reject detached provenance, changed skew, loose bounds or wrong Q33 endpoints.
fn assert_s144_tracking_output(
    map: &FixtureAffineChart,
    projection: &FixtureChartProjection<'_>,
    tracking: &FixtureChartTracking,
    enclosed: &FixtureChartInterval<'_>,
) {
    assert!(std::ptr::eq(projection.chart(), map));
    let original = projection.source_projection();
    assert_eq!(
        original.center_seconds().to_bits(),
        (16.0_f64 / 1024.0).to_bits()
    );
    assert_eq!(original.distance_seconds().to_bits(), 0.5_f64.to_bits());
    assert_eq!(
        projection.center_seconds().to_bits(),
        (10.0_f64 / 1024.0).to_bits()
    );
    let error = projection.center_roundoff_seconds();
    assert!(error.is_finite() && error > 0.0 && error < 1.0 / 18_014_398_509_481_984.0);
    assert_eq!(
        tracking.tracking_skew_seconds_per_second().to_bits(),
        0.125_f64.to_bits()
    );
    // Exact dyadic inputs give center 5/512 and original radius 1/2.
    // For 0 < error < 2^-54, the specified widened radius is 1/2 + 2^-53.
    // Thus the exact endpoints are -251/512 - 2^-53 and 261/512 + 2^-53.
    // Multiplying by 2^33 leaves fractions -2^-20/+2^-20 before floor/ceil.
    let interval = enclosed.interval();
    assert_eq!(interval.lower(), -4_211_081_217);
    assert_eq!(interval.upper(), 4_378_853_377);
    assert_eq!(
        enclosed.lower_rounding(),
        FixtureEndpointRounding::OutwardFraction
    );
    assert_eq!(
        enclosed.upper_rounding(),
        FixtureEndpointRounding::OutwardFraction
    );
}

/// S144's affine chart preserves the raw owner and supplies both projection paths.
/// # Errors
/// Preserve real fixture, chart, projection, and enclosure failures.
/// # Panics
/// Reject changed source provenance, fitted values, or numerical bounds.
#[test]
fn s144_chart_reexpresses_without_mutating_genuine_history() -> TestResult {
    let source = Source::new(0, &[8, 10, 12], 22, &[1.0; 3], 3)?;
    let before = snapshot(&source)?;
    let map = chart(0, 2, 4, 4.0 / 1024.0, 1.0 / 1024.0)?;
    let view = source
        .history
        .prepare_chart(source.context, FixtureChartOutcome::Known(&map))?
        .ok_or_else(|| io::Error::other("missing chart view"))?;
    assert_s144_descriptor(&map, &view);
    assert_eq!(view.source_context(), source.context);
    assert_eq!(view.context(), target()?);
    for (index, anchor) in [(0, 0), (1, 1), (2, 2)] {
        let p = view.project_anchor(
            target()?,
            index,
            MonotonicNs::new(anchor * SECOND),
            MonotonicNs::new(2 * SECOND),
            terms()?,
        )?;
        near(p.center_seconds(), 8.0 / 1024.0);
        near(p.phase_rate_seconds_per_second(), 1.0 / 1024.0);
        covers_subtraction(
            p.center_seconds(),
            p.source_projection().center_seconds(),
            4.0 / 1024.0,
            p.center_roundoff_seconds(),
        )?;
        covers_subtraction(
            p.phase_rate_seconds_per_second(),
            p.source_projection().phase_rate_seconds_per_second(),
            1.0 / 1024.0,
            p.rate_roundoff_seconds_per_second(),
        )?;
    }
    let p = view.project_tracking(
        target()?,
        2,
        MonotonicNs::new(2 * SECOND),
        MonotonicNs::new(4 * SECOND),
        terms()?,
        0.125,
    )?;
    near(p.center_seconds(), 10.0 / 1024.0);
    let tracked = p
        .tracking()
        .ok_or_else(|| io::Error::other("missing tracking"))?;
    near(tracked.phase_seconds(), 10.0 / 1024.0);
    let interval = enclose_fixture_chart_projection(&p)?;
    assert_s144_tracking_output(&map, &p, tracked, &interval);
    assert_eq!(interval.projection().context(), target()?);
    assert!(interval.interval().lower() <= interval.interval().upper());
    // These are real checked objects. Debug is a bounded type description and
    // cannot disclose map labels, parameters, observation times or estimates.
    for (actual, expected) in [
        (
            format!("{:?}", map.frames()),
            "FixtureChartFrames { checked: true, .. }",
        ),
        (
            format!("{:?}", map.domain()),
            "FixtureChartDomain { checked: true, .. }",
        ),
        (
            format!("{map:?}"),
            "FixtureAffineChart { checked: true, .. }",
        ),
        (
            format!("{view:?}"),
            "FixtureAdaptiveChartView { checked: true, .. }",
        ),
        (
            format!("{p:?}"),
            "FixtureChartProjection { checked: true, .. }",
        ),
        (
            format!("{tracked:?}"),
            "FixtureChartTracking { checked: true, .. }",
        ),
        (format!("{interval:?}"), "FixtureChartInterval { .. }"),
    ] {
        assert_eq!(actual, expected);
    }
    assert_eq!(snapshot(&source)?, before);
    Ok(())
}

/// Independent weighted-fit facts, shared map reference, and same-context twins.
/// # Errors
/// Preserve actual producer and chart failures.
/// # Panics
/// Reject incorrect reference mapping, bounds, or history/descriptor association.
#[test]
fn weighted_tracking_uses_one_common_chart_reference_and_genuine_owner() -> TestResult {
    let a = Source::new(0, &[0, 2, 0], 28, &[1.0, 1.0, 4.0], 3)?;
    let twin = Source::new(1, &[1, 3, 1], 28, &[1.0, 1.0, 4.0], 3)?;
    let before_a = snapshot(&a)?;
    let before_twin = snapshot(&twin)?;
    let map = chart(0, 2, 5, 4.0 / 1024.0, 1.0 / 1024.0)?;
    let av = a
        .history
        .prepare_chart(a.context, FixtureChartOutcome::Known(&map))?
        .ok_or_else(|| io::Error::other("missing A view"))?;
    let bv = twin
        .history
        .prepare_chart(twin.context, FixtureChartOutcome::Known(&map))?
        .ok_or_else(|| io::Error::other("missing twin view"))?;
    let ap = av.project_tracking(
        target()?,
        2,
        MonotonicNs::new(2 * SECOND),
        MonotonicNs::new(4 * SECOND),
        terms()?,
        0.125,
    )?;
    let bp = bv.project_tracking(
        target()?,
        2,
        MonotonicNs::new(3 * SECOND),
        MonotonicNs::new(4 * SECOND),
        terms()?,
        0.125,
    )?;
    near(ap.center_seconds(), 119.0 / 1536.0);
    near(ap.phase_rate_seconds_per_second(), 125.0 / 3072.0);
    let at = ap
        .tracking()
        .ok_or_else(|| io::Error::other("missing A tracking"))?;
    near(at.phase_seconds(), 869.0 / 4608.0);
    let old_at = ap
        .source_projection()
        .tracking()
        .ok_or_else(|| io::Error::other("missing raw A tracking"))?;
    assert_eq!(
        at.phase_standard_deviation_seconds().to_bits(),
        old_at.phase_standard_deviation_seconds().to_bits()
    );
    assert_eq!(
        at.rate_standard_deviation_seconds_per_second().to_bits(),
        old_at
            .rate_standard_deviation_seconds_per_second()
            .to_bits()
    );
    covers_subtraction(
        ap.center_seconds(),
        ap.source_projection().center_seconds(),
        6.0 / 1024.0,
        ap.center_roundoff_seconds(),
    )?;
    covers_subtraction(
        at.phase_seconds(),
        old_at.phase_seconds(),
        6.0 / 1024.0,
        at.phase_roundoff_seconds(),
    )?;
    covers_subtraction(
        bp.center_seconds(),
        bp.source_projection().center_seconds(),
        6.0 / 1024.0,
        bp.center_roundoff_seconds(),
    )?;
    assert_ne!(ap.center_seconds().to_bits(), bp.center_seconds().to_bits());
    let interval = enclose_fixture_chart_projection(&ap)?;
    let exact_center =
        exact_units(ap.source_projection().center_seconds())? - exact_units(6.0 / 1024.0)?;
    let exact_radius = exact_units(ap.source_projection().distance_seconds())?;
    let scale = 1_i128 << 27; // 2^-60 seconds to 2^-33 interval ticks.
    let lower = (exact_center - exact_radius).div_euclid(scale);
    let upper = -(-(exact_center + exact_radius)).div_euclid(scale);
    assert!(i128::from(interval.interval().lower()) <= lower);
    assert!(i128::from(interval.interval().upper()) >= upper);

    // The one-nanosecond term is not an integral 2^-60 unit. Compare using
    // the exact integer numerator over (10^9 * 2^60), not a second float model.
    let fractional = av.project_tracking(
        target()?,
        2,
        MonotonicNs::new(2 * SECOND),
        MonotonicNs::new(4 * SECOND + 1),
        terms()?,
        0.125,
    )?;
    covers_nanosecond_drift(
        fractional.center_seconds(),
        fractional.source_projection().center_seconds(),
        fractional.center_roundoff_seconds(),
    )?;
    assert_eq!(snapshot(&a)?, before_a);
    assert_eq!(snapshot(&twin)?, before_twin);
    Ok(())
}

/// A retained auxiliary row participates in domain admission, not anchor indexing.
/// # Errors
/// Preserve actual source, map and projection failures.
/// # Panics
/// Reject an omitted auxiliary row or changed negative tracking facts.
#[test]
fn post_prune_auxiliary_row_is_required_and_selected_fit_is_preserved() -> TestResult {
    let source = Source::new(
        0,
        &[512, -21, 27, 27, 27, -27, -27, -27, -27],
        23,
        &[1.0; 9],
        3,
    )?;
    let before = snapshot(&source)?;
    assert_eq!(
        (
            source.history.auxiliary_count(),
            source.history.sample_count()
        ),
        (1, 8)
    );
    let narrow = chart(1, 2, 9, 4.0 / 1024.0, 1.0 / 1024.0)?;
    let rejected = matches!(
        source
            .history
            .prepare_chart(source.context, FixtureChartOutcome::Known(&narrow)),
        Err(FixtureChartError::HistoryOutsideDomain { index: 0 })
    );
    assert!(rejected);
    let map = chart(0, 2, 9, 4.0 / 1024.0, 1.0 / 1024.0)?;
    let view = source
        .history
        .prepare_chart(source.context, FixtureChartOutcome::Known(&map))?
        .ok_or_else(|| io::Error::other("missing post-prune view"))?;
    let p = view.project_tracking(
        target()?,
        0,
        MonotonicNs::new(SECOND),
        MonotonicNs::new(9 * SECOND),
        terms()?,
        0.125,
    )?;
    near(p.center_seconds(), -1075.0 / 7168.0);
    near(p.phase_rate_seconds_per_second(), -95.0 / 7168.0);
    let t = p
        .tracking()
        .ok_or_else(|| io::Error::other("missing tracking"))?;
    near(t.phase_seconds(), -557.0 / 7168.0);
    let raw_t = p
        .source_projection()
        .tracking()
        .ok_or_else(|| io::Error::other("missing raw tracking"))?;
    covers_subtraction(
        p.center_seconds(),
        p.source_projection().center_seconds(),
        11.0 / 1024.0,
        p.center_roundoff_seconds(),
    )?;
    covers_subtraction(
        t.phase_seconds(),
        raw_t.phase_seconds(),
        11.0 / 1024.0,
        t.phase_roundoff_seconds(),
    )?;
    assert!(enclose_fixture_chart_projection(&p)?.interval().lower() < 0);
    assert_eq!(snapshot(&source)?, before);
    Ok(())
}

/// Refusal, unknown outcome and raw-context ordering do not alter the history.
/// # Errors
/// Preserve genuine source and map failures.
/// # Panics
/// Reject a published refused/unknown view or an altered source snapshot.
#[test]
fn refusal_unknown_and_context_mismatch_leave_history_unchanged() -> TestResult {
    let source = Source::new(0, &[0, 2, 0], 28, &[1.0, 1.0, 4.0], 3)?;
    let before = snapshot(&source)?;
    assert!(
        source
            .history
            .prepare_chart(source.context, FixtureChartOutcome::Refused)?
            .is_none()
    );
    let unknown = matches!(
        source
            .history
            .prepare_chart(source.context, FixtureChartOutcome::Unknown),
        Err(FixtureChartError::UnknownOutcome)
    );
    assert!(unknown);
    let wrong = FixtureContext::new(label(12)?, label(17)?, label(23)?);
    let mismatch = matches!(
        source
            .history
            .prepare_chart(wrong, FixtureChartOutcome::Unknown),
        Err(FixtureChartError::ContextMismatch)
    );
    assert!(mismatch);
    let warm = Source::new(0, &[0, 2], 28, &[1.0; 2], 3)?;
    let warm_before = snapshot(&warm)?;
    let map = chart(0, 2, 4, 0.0, 0.0)?;
    let missing = matches!(
        warm.history
            .prepare_chart(warm.context, FixtureChartOutcome::Known(&map)),
        Err(FixtureChartError::MissingReport)
    );
    assert!(missing);
    assert_eq!(snapshot(&source)?, before);
    assert_eq!(snapshot(&warm)?, warm_before);
    Ok(())
}

/// Check producer-error precedence before query-domain rejection.
/// # Errors
/// Preserve actual source and descriptor failures.
/// # Panics
/// Reject reordered target, missing-anchor, wrong-anchor, or query-domain errors.
#[test]
fn producer_errors_precede_query_domain_and_preserve_source() -> TestResult {
    let source = Source::new(0, &[0, 2, 0], 28, &[1.0, 1.0, 4.0], 3)?;
    let before = snapshot(&source)?;
    let other_frames = FixtureChartFrames::new(label(18)?, label(23)?, label(19)?, label(29)?)?;
    let domain = FixtureChartDomain::new(
        MonotonicNs::new(0),
        MonotonicNs::new(2 * SECOND),
        MonotonicNs::new(4 * SECOND),
    )?;
    let other = FixtureAffineChart::new(other_frames, domain, 0.0, 0.0)?;
    let mapping = matches!(
        source
            .history
            .prepare_chart(source.context, FixtureChartOutcome::Known(&other)),
        Err(FixtureChartError::SourceMappingMismatch)
    );
    assert!(mapping);
    let generation_frames =
        FixtureChartFrames::new(label(17)?, label(24)?, label(19)?, label(29)?)?;
    let generation_map = FixtureAffineChart::new(generation_frames, domain, 0.0, 0.0)?;
    assert!(matches!(
        source
            .history
            .prepare_chart(source.context, FixtureChartOutcome::Known(&generation_map)),
        Err(FixtureChartError::SourceMappingMismatch)
    ));
    let map = chart(0, 2, 4, 0.0, 0.0)?;
    let view = source
        .history
        .prepare_chart(source.context, FixtureChartOutcome::Known(&map))?
        .ok_or_else(|| io::Error::other("missing view"))?;
    let wrong = FixtureContext::new(label(12)?, label(19)?, label(29)?);
    let mismatch = matches!(
        view.project_anchor(
            wrong,
            usize::MAX,
            MonotonicNs::new(0),
            MonotonicNs::new(5 * SECOND),
            terms()?
        ),
        Err(FixtureChartError::ContextMismatch)
    );
    assert!(mismatch);
    let missing = matches!(
        view.project_anchor(
            target()?,
            usize::MAX,
            MonotonicNs::new(0),
            MonotonicNs::new(5 * SECOND),
            terms()?
        ),
        Err(FixtureChartError::Projection {
            source: FixtureProjectionError::MissingAnchor
        })
    );
    assert!(missing);
    let wrong_anchor = matches!(
        view.project_anchor(
            target()?,
            0,
            MonotonicNs::new(SECOND),
            MonotonicNs::new(5 * SECOND),
            terms()?
        ),
        Err(FixtureChartError::Projection {
            source: FixtureProjectionError::AnchorMismatch
        })
    );
    assert!(wrong_anchor);
    let query = matches!(
        view.project_anchor(
            target()?,
            0,
            MonotonicNs::new(0),
            MonotonicNs::new(5 * SECOND),
            terms()?
        ),
        Err(FixtureChartError::QueryOutsideDomain)
    );
    assert!(query);
    assert_eq!(snapshot(&source)?, before);
    Ok(())
}

/// Exercise the full fixed-capacity owner and inclusive domain endpoint.
/// # Errors
/// Preserve all 64 real observation, normalization and projection failures.
/// # Panics
/// Reject dropped rows, an omitted terminal row, or altered source values.
#[test]
fn full_capacity_and_inclusive_domain_preserve_all_actual_rows() -> TestResult {
    let source = Source::new(0, &[0; 64], 23, &[1.0; 64], 64)?;
    let before = snapshot(&source)?;
    assert_eq!(
        (
            source.history.sample_count(),
            source.history.auxiliary_count()
        ),
        (64, 0)
    );
    let short = chart(0, 32, 62, 0.0, 0.0)?;
    let outside = matches!(
        source
            .history
            .prepare_chart(source.context, FixtureChartOutcome::Known(&short)),
        Err(FixtureChartError::HistoryOutsideDomain { index: 63 })
    );
    assert!(outside);
    let map = chart(0, 32, 63, 0.0, 0.0)?;
    let view = source
        .history
        .prepare_chart(source.context, FixtureChartOutcome::Known(&map))?
        .ok_or_else(|| io::Error::other("missing full view"))?;
    let zero_terms = FixtureProjectionTerms::new(0.0, 0.0, 0.0)?;
    let p = view.project_anchor(
        target()?,
        63,
        MonotonicNs::new(63 * SECOND),
        MonotonicNs::new(63 * SECOND),
        zero_terms,
    )?;
    assert_eq!(p.center_seconds(), 0.0);
    assert!(p.center_roundoff_seconds().is_finite() && p.center_roundoff_seconds() > 0.0);
    let interval = enclose_fixture_chart_projection(&p)?;
    assert!(interval.interval().lower() < 0 && interval.interval().upper() > 0);
    assert_eq!(snapshot(&source)?, before);
    Ok(())
}

/// Invalid map scalar and arithmetic failures remain isolated from the raw owner.
/// # Errors
/// Preserve real fixture and accepted-map admission failures.
/// # Panics
/// Reject an invalid scalar, accept a valid neighbor, and preserve source state.
#[test]
fn invalid_scalar_and_overflow_leave_real_source_unchanged() -> TestResult {
    let source = Source::new(0, &[0, 2, 0], 28, &[1.0, 1.0, 4.0], 3)?;
    let before = snapshot(&source)?;
    let frames = FixtureChartFrames::new(label(17)?, label(23)?, label(19)?, label(29)?)?;
    let domain = FixtureChartDomain::new(
        MonotonicNs::new(0),
        MonotonicNs::new(2 * SECOND),
        MonotonicNs::new(4 * SECOND),
    )?;
    for (bad, issue) in [
        (f64::NAN, FixtureChartIssue::NonFinite),
        (f64::INFINITY, FixtureChartIssue::NonFinite),
        (f64::NEG_INFINITY, FixtureChartIssue::NonFinite),
        (f64::from_bits(1), FixtureChartIssue::Subnormal),
    ] {
        let displacement = matches!(FixtureAffineChart::new(frames, domain, bad, 0.0),
                                   Err(FixtureChartError::InvalidScalar { field: FixtureChartField::Displacement, issue: actual }) if actual == issue);
        assert!(displacement);
        let rate = matches!(FixtureAffineChart::new(frames, domain, 0.0, bad),
                            Err(FixtureChartError::InvalidScalar { field: FixtureChartField::Rate, issue: actual }) if actual == issue);
        assert!(rate);
    }
    let healthy = chart(0, 2, 4, -0.0, 0.0)?;
    assert!(
        source
            .history
            .prepare_chart(source.context, FixtureChartOutcome::Known(&healthy))?
            .is_some()
    );
    let large = chart(0, 2, 4, f64::MAX, f64::MAX)?;
    let view = source
        .history
        .prepare_chart(source.context, FixtureChartOutcome::Known(&large))?
        .ok_or_else(|| io::Error::other("missing large view"))?;
    let failed = matches!(
        view.project_anchor(
            target()?,
            2,
            MonotonicNs::new(2 * SECOND),
            MonotonicNs::new(4 * SECOND),
            terms()?
        ),
        Err(FixtureChartError::Arithmetic { .. })
    );
    assert!(failed);
    let edge = chart(0, 2, 4, -1_073_741_824.0, 0.0)?;
    let edge_view = source
        .history
        .prepare_chart(source.context, FixtureChartOutcome::Known(&edge))?
        .ok_or_else(|| io::Error::other("missing edge view"))?;
    let edge_projection = edge_view.project_anchor(
        target()?,
        2,
        MonotonicNs::new(2 * SECOND),
        MonotonicNs::new(4 * SECOND),
        terms()?,
    )?;
    let unrepresentable = matches!(
        enclose_fixture_chart_projection(&edge_projection),
        Err(FixtureChartError::Enclosure {
            source: FixtureEnclosureError::Unrepresentable
        })
    );
    assert!(unrepresentable);
    assert_eq!(snapshot(&source)?, before);
    Ok(())
}

/// Descriptor admission includes exact domain limits and distinct frame identities.
/// # Errors
/// Preserve genuine label and healthy descriptor construction failures.
/// # Panics
/// Reject invalid frame/domain neighbors or an unsupported domain span.
#[test]
fn chart_descriptor_identity_and_integer_domain_boundaries() -> TestResult {
    let duplicate = matches!(
        FixtureChartFrames::new(label(17)?, label(23)?, label(17)?, label(29)?),
        Err(FixtureChartError::InvalidFrames)
    );
    assert!(duplicate);
    let duplicate_generation = matches!(
        FixtureChartFrames::new(label(17)?, label(23)?, label(19)?, label(23)?),
        Err(FixtureChartError::InvalidFrames)
    );
    assert!(duplicate_generation);
    let frames = FixtureChartFrames::new(label(17)?, label(23)?, label(19)?, label(29)?)?;
    let reversed = matches!(
        FixtureChartDomain::new(
            MonotonicNs::new(2),
            MonotonicNs::new(1),
            MonotonicNs::new(3)
        ),
        Err(FixtureChartError::InvalidDomain)
    );
    assert!(reversed);
    let too_long = matches!(
        FixtureChartDomain::new(
            MonotonicNs::new(0),
            MonotonicNs::new(0),
            MonotonicNs::new((1_u64 << 53) + 1)
        ),
        Err(FixtureChartError::InvalidDomain)
    );
    assert!(too_long);
    let edge = FixtureChartDomain::new(
        MonotonicNs::new(0),
        MonotonicNs::new(0),
        MonotonicNs::new(1_u64 << 53),
    )?;
    let _healthy = FixtureAffineChart::new(frames, edge, -0.0, 0.0)?;
    Ok(())
}

/// Signed chart time and rate reach both the real producer and enclosure.
/// # Errors
/// Preserve genuine source, descriptor, projection and exact-endpoint failures.
/// # Panics
/// Reject either independently fixed S144 phase/rate or changed source state.
#[test]
fn negative_elapsed_and_rate_reach_actual_chart_enclosure() -> TestResult {
    let source = Source::new(0, &[8, 10, 12], 22, &[1.0; 3], 3)?;
    let before = snapshot(&source)?;
    for (reference, rate, query, expected_center, expected_rate) in [
        (4, 1.0 / 1024.0, 2, 10.0 / 1024.0, 1.0 / 1024.0),
        (2, -1.0 / 1024.0, 4, 14.0 / 1024.0, 3.0 / 1024.0),
    ] {
        let map = chart(0, reference, 4, 4.0 / 1024.0, rate)?;
        let view = source
            .history
            .prepare_chart(source.context, FixtureChartOutcome::Known(&map))?
            .ok_or_else(|| io::Error::other("missing signed chart view"))?;
        let p = view.project_anchor(
            target()?,
            2,
            MonotonicNs::new(2 * SECOND),
            MonotonicNs::new(query * SECOND),
            terms()?,
        )?;
        near(p.center_seconds(), expected_center);
        near(p.phase_rate_seconds_per_second(), expected_rate);
        covers_subtraction(
            p.center_seconds(),
            p.source_projection().center_seconds(),
            2.0 / 1024.0,
            p.center_roundoff_seconds(),
        )?;
        let enclosed = enclose_fixture_chart_projection(&p)?;
        assert_eq!(enclosed.projection().context(), target()?);
        assert!(enclosed.interval().lower() <= enclosed.interval().upper());
    }
    assert_eq!(snapshot(&source)?, before);
    Ok(())
}

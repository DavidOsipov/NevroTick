// SPDX-License-Identifier: GPL-2.0-only
//! Independent S1.4b mathematical composition and genuine-projection checks.
//!
//! Literal dyadics and a separately rounded cancellation chain provide the
//! expected facts. The original adaptive histories and stored observations are
//! never rewritten. Source reservations are reported to the coordinator before
//! any Rust execution; this module grants no achieved-correction authority.

use std::{io, ptr};

use super::{SECOND, Source, TestResult, label, near, snapshot, terms};
use time_core::{
    FixtureAffineChart, FixtureChartBoundIssue, FixtureChartBoundOperation, FixtureChartDomain,
    FixtureChartError, FixtureChartFrames, FixtureChartOutcome, FixtureChartProjection,
    FixtureChartStep, enclose_fixture_chart_projection,
};
use time_domain::MonotonicNs;

/// Build one immutable literal descriptor without substituting mapping labels.
/// # Errors
/// Preserve frame, complete-domain and supported-scalar admission failures.
fn mapping(
    source: [u64; 2],
    target: [u64; 2],
    domain: FixtureChartDomain,
    phase: f64,
    rate: f64,
) -> TestResult<FixtureAffineChart> {
    Ok(FixtureAffineChart::new(
        FixtureChartFrames::new(
            label(source[0])?,
            label(source[1])?,
            label(target[0])?,
            label(target[1])?,
        )?,
        domain,
        phase,
        rate,
    )?)
}

/// Keep all genuine source rows and both frozen query instants in one domain.
/// # Errors
/// Preserve exact monotonic-domain admission failures.
fn domain(reference: u64) -> TestResult<FixtureChartDomain> {
    Ok(FixtureChartDomain::new(
        MonotonicNs::new(0),
        MonotonicNs::new(reference),
        MonotonicNs::new(8 * SECOND),
    )?)
}

/// Obtain both anchor and fitted phase from the same genuine held report.
/// # Errors
/// Preserve actual history/view, tracking and checked-term failures.
fn projection<'chart>(
    source: &Source,
    map: &'chart FixtureAffineChart,
    at: u64,
) -> TestResult<FixtureChartProjection<'chart>> {
    let view = source
        .history
        .prepare_chart(source.context, FixtureChartOutcome::Known(map))?
        .ok_or_else(|| io::Error::other("missing composition chart view"))?;
    let anchor = source
        .history
        .reference()
        .ok_or_else(|| io::Error::other("missing composition anchor"))?;
    Ok(view.project_tracking(
        view.context(),
        2,
        anchor,
        MonotonicNs::new(at),
        terms()?,
        0.125,
    )?)
}

/// Inspect unchanged source/time and separate same-fit uncertainty provenance.
/// # Errors
/// Reject absent genuine fitted attachments instead of supplying an oracle.
/// # Panics
/// Reject relabelled observations, refreshed ages, detached charts or changed SD.
fn provenance(value: &FixtureChartProjection<'_>, source: &Source) -> TestResult {
    let original = value.source_projection();
    assert_eq!(original.context(), source.context);
    assert_eq!(original.anchor_observed_at(), MonotonicNs::new(2 * SECOND));
    assert_eq!(original.latest_observed_at(), MonotonicNs::new(2 * SECOND));
    assert_eq!(original.fit_reference(), MonotonicNs::new(2 * SECOND));
    assert_eq!(value.context().source_id(), source.context.source_id());
    assert_eq!(
        value.context().frame(),
        value.chart().frames().target_frame()
    );
    assert_eq!(
        value.context().generation(),
        value.chart().frames().target_generation()
    );
    let before = original
        .tracking()
        .ok_or_else(|| io::Error::other("missing original fitted attachment"))?;
    let after = value
        .tracking()
        .ok_or_else(|| io::Error::other("missing mapped fitted attachment"))?;
    assert_eq!(
        after.phase_standard_deviation_seconds().to_bits(),
        before.phase_standard_deviation_seconds().to_bits()
    );
    assert_eq!(
        after.rate_standard_deviation_seconds_per_second().to_bits(),
        before
            .rate_standard_deviation_seconds_per_second()
            .to_bits()
    );
    assert_eq!(
        after.tracking_skew_seconds_per_second().to_bits(),
        before.tracking_skew_seconds_per_second().to_bits()
    );
    Ok(())
}

/// Exact dyadic composition and a stored-input decimal rounding fact stay distinct.
/// # Errors
/// Preserve literal descriptor, genuine history and actual projection failures.
/// # Panics
/// Reject wrong signs/units, lost coefficient error or changed source provenance.
#[test]
fn composition_preserves_dyadic_values_and_rounding_provenance() -> TestResult {
    // Three exact phases are 3/8, 1/2, 5/8: rate=1/8 and phase at t=4 is 7/8.
    let source = Source::new(0, &[6, 8, 10], 28, &[1.0; 3], 3)?;
    let before = snapshot(&source)?;
    let common = domain(0)?;
    for (left_phase, right_phase, left_rate, right_rate, expected_phase, expected_rate) in [
        (
            1.0 / 8.0,
            3.0 / 16.0,
            1.0 / 16.0,
            1.0 / 32.0,
            3.0 / 16.0,
            1.0 / 32.0,
        ),
        (
            -1.0 / 8.0,
            -3.0 / 16.0,
            -1.0 / 16.0,
            -1.0 / 32.0,
            25.0 / 16.0,
            7.0 / 32.0,
        ),
        (
            1.0 / 8.0,
            -1.0 / 8.0,
            1.0 / 16.0,
            -1.0 / 16.0,
            7.0 / 8.0,
            1.0 / 8.0,
        ),
    ] {
        let left = mapping([17, 23], [19, 29], common, left_phase, left_rate)?;
        let right = mapping([19, 29], [31, 37], common, right_phase, right_rate)?;
        assert_eq!(left.phase_coefficient_roundoff_seconds().to_bits(), 0);
        assert_eq!(
            left.rate_coefficient_roundoff_seconds_per_second()
                .to_bits(),
            0
        );
        let composed = left.compose(&right)?;
        let value = projection(&source, &composed, 4 * SECOND)?;
        assert!(ptr::eq(value.chart(), ptr::from_ref(&composed)));
        near(value.source_projection().center_seconds(), 7.0 / 8.0);
        near(value.center_seconds(), expected_phase);
        near(value.phase_rate_seconds_per_second(), expected_rate);
        near(
            value
                .tracking()
                .ok_or_else(|| io::Error::other("missing composed fitted phase"))?
                .phase_seconds(),
            expected_phase,
        );
        provenance(&value, &source)?;
        let interval = enclose_fixture_chart_projection(&value)?;
        assert!(ptr::eq(
            interval.projection().chart(),
            ptr::from_ref(&composed)
        ));
    }
    // Exact stored binary64 inputs 0.1 and 0.2 sum between adjacent results.
    // RN is 0x3fd3333333333334, differing from that exact sum by precisely 2^-55.
    let decimal_left = mapping([17, 23], [19, 29], common, 0.1, 0.0)?;
    let decimal_right = mapping([19, 29], [31, 37], common, 0.2, 0.0)?;
    let decimal = decimal_left.compose(&decimal_right)?;
    assert_eq!(
        decimal.phase_at_reference_seconds().to_bits(),
        0x3fd3_3333_3333_3334
    );
    let error = decimal.phase_coefficient_roundoff_seconds();
    assert!(error >= 1.0 / 36_028_797_018_963_968.0);
    assert!(error < 1.0 / 9_007_199_254_740_992.0);
    assert_eq!(snapshot(&source)?, before);
    Ok(())
}

/// Compose six literal edges so three lost units require inherited error.
/// # Errors
/// Preserve real checked constructors and each ordered composition failure.
fn cancellation(
    common: FixtureChartDomain,
    coefficients: [(f64, f64); 6],
    inherited_right: bool,
) -> TestResult<FixtureAffineChart> {
    let [first, second, third, fourth, fifth, sixth] = coefficients;
    let first = mapping([17, 23], [19, 29], common, first.0, first.1)?;
    let second = mapping([19, 29], [31, 37], common, second.0, second.1)?;
    let third = mapping([31, 37], [41, 43], common, third.0, third.1)?;
    let fourth = mapping([41, 43], [43, 47], common, fourth.0, fourth.1)?;
    let fifth = mapping([43, 47], [47, 49], common, fifth.0, fifth.1)?;
    let sixth = mapping([47, 49], [51, 53], common, sixth.0, sixth.1)?;
    if inherited_right {
        Ok(first.compose(
            &second
                .compose(&third)?
                .compose(&fourth)?
                .compose(&fifth)?
                .compose(&sixth)?,
        )?)
    } else {
        Ok(first
            .compose(&second)?
            .compose(&third)?
            .compose(&fourth)?
            .compose(&fifth)?
            .compose(&sixth)?)
    }
}

/// Lost units survive coefficient cancellation on either operand and signed time.
/// # Errors
/// Preserve real source, descriptor, projection and enclosure failures.
/// # Panics
/// Reject zeroed inherited error, signed elapsed bounds or an anchor/fitted mismatch.
#[test]
fn composition_retains_both_operand_errors_through_signed_elapsed() -> TestResult {
    let source = Source::new(0, &[0, 0, 0], 28, &[1.0; 3], 3)?;
    let before = snapshot(&source)?;
    let common = domain(4 * SECOND)?;
    let exact_zero = mapping([17, 23], [51, 53], common, 0.0, 0.0)?;
    let queries = [(2_500_000_000, 1.5), (6_500_000_000, 2.5)];
    let controls = [
        projection(&source, &exact_zero, queries[0].0)?,
        projection(&source, &exact_zero, queries[1].0)?,
    ];
    // Independent outward-operation oracle: 5*2^-1022 + 20*2^-1074.
    // A singleton interval checks the exact positive bound without a new query.
    let direct_bound = f64::from_bits(0x0034_0000_0000_0005);
    let big = 9_007_199_254_740_992.0;
    for (coefficients, inherited_right, rate_case) in [
        (
            [
                (big, 0.0),
                (1.0, 0.0),
                (1.0, 0.0),
                (1.0, 0.0),
                (-big, 0.0),
                (0.0, 0.0),
            ],
            false,
            false,
        ),
        (
            [
                (0.0, 0.0),
                (big, 0.0),
                (1.0, 0.0),
                (1.0, 0.0),
                (1.0, 0.0),
                (-big, 0.0),
            ],
            true,
            false,
        ),
        (
            [
                (0.0, big),
                (0.0, 1.0),
                (0.0, 1.0),
                (0.0, 1.0),
                (0.0, -big),
                (0.0, 0.0),
            ],
            false,
            true,
        ),
        (
            [
                (0.0, 0.0),
                (0.0, big),
                (0.0, 1.0),
                (0.0, 1.0),
                (0.0, 1.0),
                (0.0, -big),
            ],
            true,
            true,
        ),
    ] {
        // Each RN(2^53+1)=2^53 loses one unit; cancellation leaves0, exact sum3.
        // The final zero edge makes inherited error indispensable on that side.
        let composed = cancellation(common, coefficients, inherited_right)?;
        assert_eq!(composed.phase_at_reference_seconds().to_bits(), 0);
        assert_eq!(composed.rate_seconds_per_second().to_bits(), 0);
        let coefficient_error = if rate_case {
            composed.rate_coefficient_roundoff_seconds_per_second()
        } else {
            composed.phase_coefficient_roundoff_seconds()
        };
        assert!((3.0..8.0).contains(&coefficient_error));
        for ((at, magnitude), control) in queries.into_iter().zip(&controls) {
            let value = projection(&source, &composed, at)?;
            let required = if rate_case { 3.0 * magnitude } else { 3.0 };
            assert_eq!(value.center_seconds().to_bits(), 0);
            assert_eq!(value.phase_rate_seconds_per_second().to_bits(), 0);
            assert!(value.center_roundoff_seconds() >= required);
            assert!(value.center_roundoff_seconds() < 32.0);
            assert!((direct_bound..=direct_bound).contains(&control.center_roundoff_seconds()));
            let tracking = value
                .tracking()
                .ok_or_else(|| io::Error::other("missing cancellation fitted phase"))?;
            assert_eq!(tracking.phase_seconds().to_bits(), 0);
            assert!(tracking.phase_roundoff_seconds() >= required);
            assert!(tracking.phase_roundoff_seconds() < 32.0);
            if rate_case {
                assert!(value.rate_roundoff_seconds_per_second() >= 3.0);
                assert!(value.rate_roundoff_seconds_per_second() < 8.0);
                assert!(control.rate_roundoff_seconds_per_second() < 0.25);
            }
            provenance(&value, &source)?;
            let interval = enclose_fixture_chart_projection(&value)?;
            assert!(ptr::eq(
                interval.projection().chart(),
                ptr::from_ref(&composed)
            ));
        }
    }
    assert_eq!(snapshot(&source)?, before);
    Ok(())
}

/// Reject actual chain/domain neighbors without changing the left descriptor.
/// # Errors
/// Preserve unexpected literal descriptor-construction failures.
/// # Panics
/// Reject changed relationship causes or silent endpoint relabelling.
fn relationship_rejections(left: &FixtureAffineChart, common: FixtureChartDomain) -> TestResult {
    for (source, target, expected) in [
        (
            [31, 29],
            [41, 43],
            FixtureChartError::CompositionMappingMismatch,
        ),
        (
            [19, 37],
            [41, 43],
            FixtureChartError::CompositionMappingMismatch,
        ),
        ([19, 29], [17, 43], FixtureChartError::InvalidFrames),
        ([19, 29], [41, 23], FixtureChartError::InvalidFrames),
    ] {
        let right = mapping(source, target, common, 0.0, 0.0)?;
        assert!(matches!(left.compose(&right), Err(error) if error == expected));
    }
    for different in [
        FixtureChartDomain::new(
            MonotonicNs::new(1),
            MonotonicNs::new(4 * SECOND),
            MonotonicNs::new(8 * SECOND),
        )?,
        domain(3 * SECOND)?,
        FixtureChartDomain::new(
            MonotonicNs::new(0),
            MonotonicNs::new(4 * SECOND),
            MonotonicNs::new(7 * SECOND),
        )?,
    ] {
        let right = mapping([19, 29], [31, 37], different, 0.0, 0.0)?;
        assert!(matches!(
            left.compose(&right),
            Err(FixtureChartError::CompositionDomainMismatch)
        ));
    }
    Ok(())
}

/// Reject actual point and outward-bound failures beside a legal descriptor.
/// # Errors
/// Preserve unexpected literal descriptor-construction failures.
/// # Panics
/// Reject changed precedence, unsupported arithmetic acceptance or mutated operands.
#[test]
fn composition_rejects_relationships_and_ordered_arithmetic_failures() -> TestResult {
    let common = domain(4 * SECOND)?;
    let left = mapping([17, 23], [19, 29], common, 1.0 / 8.0, 1.0 / 16.0)?;
    let scalar_bits = (
        left.phase_at_reference_seconds().to_bits(),
        left.rate_seconds_per_second().to_bits(),
    );
    relationship_rejections(&left, common)?;
    let phase_large = mapping([17, 23], [19, 29], common, f64::MAX, 0.0)?;
    let phase_same = mapping([19, 29], [31, 37], common, f64::MAX, 0.0)?;
    assert!(matches!(
        phase_large.compose(&phase_same),
        Err(FixtureChartError::Arithmetic {
            step: FixtureChartStep::ComposeDisplacement,
            ..
        })
    ));
    let rate_large = mapping([17, 23], [19, 29], common, f64::MAX, f64::MAX)?;
    let rate_same = mapping([19, 29], [31, 37], common, -f64::MAX, f64::MAX)?;
    assert!(matches!(
        rate_large.compose(&rate_same),
        Err(FixtureChartError::Arithmetic {
            step: FixtureChartStep::ComposeRate,
            ..
        })
    ));
    let phase_cancel = mapping([19, 29], [31, 37], common, -f64::MAX, 0.0)?;
    assert!(matches!(
        phase_large.compose(&phase_cancel),
        Err(FixtureChartError::Bound {
            operation: FixtureChartBoundOperation::Add,
            issue: FixtureChartBoundIssue::NonFiniteResult
        })
    ));
    let healthy_large = mapping([17, 23], [19, 29], common, f64::MAX / 4.0, 0.0)?;
    let healthy_cancel = mapping([19, 29], [31, 37], common, -f64::MAX / 4.0, 0.0)?;
    assert_eq!(
        healthy_large
            .compose(&healthy_cancel)?
            .phase_at_reference_seconds()
            .to_bits(),
        0
    );
    let right = mapping([19, 29], [31, 37], common, 0.0, 0.0)?;
    let healthy = left.compose(&right)?;
    assert_eq!(
        healthy.phase_at_reference_seconds().to_bits(),
        scalar_bits.0
    );
    assert_eq!(healthy.rate_seconds_per_second().to_bits(), scalar_bits.1);
    assert_eq!(left.phase_at_reference_seconds().to_bits(), scalar_bits.0);
    assert_eq!(left.rate_seconds_per_second().to_bits(), scalar_bits.1);
    Ok(())
}

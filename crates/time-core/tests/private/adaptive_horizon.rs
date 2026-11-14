// SPDX-License-Identifier: GPL-2.0-only
//! Literal adaptive-area and horizon arithmetic controls, without source authority.
//!
//! Six original numerical controls moved unchanged with their invariant owner.
//! Additional diagnostic and refusal controls exercise its actual checked helpers.
//! Their independent dyadic and exact binary64/integer expectations grant no
//! observation credit, physical uncertainty, native correction or clock mapping.

use std::{error::Error, fmt, io};

use time_domain::ActiveAwakeNs;

/// Preserve the original typed pure-fixture result contract.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Exact nanoseconds per declared second, never an elapsed-clock conversion.
const SECOND: u64 = 1_000_000_000;

/// Exact binary64-to-integer witnesses, separate from genuine reference evidence.
/// # Errors
/// Preserve a supported exact upward conversion failure.
/// # Panics
/// Detect rounding the nanosecond product before taking its integer ceiling.
#[test]
fn adaptive_duration_ceiling_preserves_fractional_nanoseconds_at_large_seconds() -> TestResult {
    for (fraction_bits, expected) in [
        (10_u64, 4_194_304_000_000_010_u64),
        (11, 4_194_304_000_000_011),
        (12, 4_194_304_000_000_012),
    ] {
        // At exponent 22, each mantissa bit is exactly 2^-30 seconds.
        // The middle exact product has fractional ns 262581760/1073741824.
        let seconds = f64::from_bits((1045_u64 << 52) | fraction_bits);
        assert_eq!(
            product_ok(super::adaptive_ceil_nanoseconds(seconds))?,
            expected
        );
    }
    assert_eq!(product_ok(super::adaptive_ceil_nanoseconds(1.0))?, SECOND);
    assert_eq!(
        product_ok(super::adaptive_ceil_nanoseconds(f64::MIN_POSITIVE))?,
        1
    );
    for seconds in [
        0.0,
        -0.0,
        -1.0,
        f64::INFINITY,
        f64::NAN,
        f64::MAX,
        f64::from_bits(1),
    ] {
        assert!(super::adaptive_ceil_nanoseconds(seconds).is_err());
    }
    Ok(())
}

/// Literal policy math checks no source eligibility, physical SD or nonlinear bound.
/// # Errors
/// Preserve checked profile, duration, area, debt and proposal construction.
/// # Panics
/// Detect missing half/ratio/time factors or claiming a fixed-horizon bound for D²/A.
#[test]
fn adaptive_policy_uses_exact_selected_horizon_for_its_existing_rate_bound() -> TestResult {
    let duration = |n| product_ok(super::adaptive_duration(n));
    let policy = product_ok(super::FixtureAdaptiveHorizonProfile::new(
        4.0,
        duration(SECOND)?,
        duration(16 * SECOND)?,
        0.0,
    ))?;
    let area = product_ok(super::AdaptiveArea::from_reference(
        policy,
        1.0 / 64.0,
        duration(8 * SECOND)?,
    ))?;
    assert_eq!(area.point, 1.0 / 4.0);
    for point in [-1.0 / 8.0, 1.0 / 8.0] {
        let debt = product_ok(super::PhaseDebt::new(point, 1.0 / 64.0))?;
        let horizon = product_ok(area.horizon(policy, debt))?;
        assert_eq!(horizon.nanoseconds(), 2 * SECOND);
        let rates = product_ok(debt.proposal(0.0, horizon, 1.0, 2.0))?;
        assert_eq!(rates.transient_rate(), point / 2.0);
        assert!(rates.transient_error() >= 1.0 / 128.0);
        // The nonlinear ideal over [7/64,9/64] deviates by 17/1024.
        // The published bound intentionally concerns D / exact_selected_T.
        assert!(rates.transient_error() < 17.0 / 1024.0);
    }
    Ok(())
}

/// A sign-neutral debt must not execute an irrelevant overflowing policy quotient.
/// # Errors
/// Preserve valid finite modeled inputs and the original proposal-bound construction.
/// # Panics
/// Detect division before neutral admission or loss of exact baseline signed-zero bits.
#[test]
fn adaptive_neutral_debt_preserves_baseline_without_overflowing_area_division() -> TestResult {
    let policy = product_ok(super::FixtureAdaptiveHorizonProfile::new(
        4.0,
        product_ok(super::adaptive_duration(SECOND))?,
        product_ok(super::adaptive_duration(16 * SECOND))?,
        0.0,
    ))?;
    let area = product_ok(super::AdaptiveArea::new(f64::from_bits(1623_u64 << 52)))?;
    let debt = product_ok(super::PhaseDebt::new(
        f64::from_bits(523_u64 << 52),
        f64::from_bits(524_u64 << 52),
    ))?;
    let horizon = product_ok(area.horizon(policy, debt))?;
    assert_eq!(horizon.nanoseconds(), 16 * SECOND);
    let rates = product_ok(debt.proposal(-0.0, horizon, 1.0, 2.0))?;
    assert!(!rates.has_useful_transient());
    assert_eq!(rates.aggregate_rate().to_bits(), (-0.0_f64).to_bits());
    Ok(())
}

/// Clamp endpoints remain exact typed nanoseconds, independent of binary64 seconds.
/// # Errors
/// Preserve valid explicit profile, area and debt admission.
/// # Panics
/// Detect an integer endpoint changed by a seconds roundtrip or concealed overflow.
#[test]
fn adaptive_clamps_keep_original_integer_limits_and_refuse_nonzero_underflow() -> TestResult {
    let minimum = (1_u64 << 53) - 3;
    let maximum = (1_u64 << 53) - 1;
    let policy = product_ok(super::FixtureAdaptiveHorizonProfile::new(
        4.0,
        product_ok(super::adaptive_duration(minimum))?,
        product_ok(super::adaptive_duration(maximum))?,
        0.0,
    ))?;
    let debt = product_ok(super::PhaseDebt::new(1.0, 0.0))?;
    let short = product_ok(super::AdaptiveArea::new(1.0))?;
    let long = product_ok(super::AdaptiveArea::new(f64::from_bits(1123_u64 << 52)))?;
    assert_eq!(
        product_ok(short.horizon(policy, debt))?.nanoseconds(),
        minimum
    );
    assert_eq!(
        product_ok(long.horizon(policy, debt))?.nanoseconds(),
        maximum
    );
    let tiny = product_ok(super::FixtureAdaptiveHorizonProfile::new(
        f64::MIN_POSITIVE,
        product_ok(super::adaptive_duration(SECOND))?,
        product_ok(super::adaptive_duration(2 * SECOND))?,
        0.0,
    ))?;
    assert!(
        super::AdaptiveArea::from_reference(
            tiny,
            1.0,
            product_ok(super::adaptive_duration(SECOND))?
        )
        .is_err()
    );
    assert_eq!(
        product_ok(super::AdaptiveArea::from_reference(
            tiny,
            1.0,
            ActiveAwakeNs::new(0).duration_since(ActiveAwakeNs::new(0))?
        ))?
        .point,
        0.0
    );
    Ok(())
}

/// Exact ceiling precedes integer endpoint clamping at the supported high boundary.
/// These literal binary64 witnesses test policy arithmetic, not source eligibility.
/// # Errors
/// Preserve checked profile, debt, area and selected duration construction.
/// # Panics
/// Detect clamping against a rounded seconds representation of an integer minimum.
#[test]
fn adaptive_near_minimum_quantizes_before_integer_clamping() -> TestResult {
    let maximum = 1_u64 << 53;
    let debt = product_ok(super::PhaseDebt::new(1.0, 0.0))?;
    for (minimum, bits, expected, above) in [
        (
            9_007_199_254_740_985,
            0x4161_2e0b_e826_d691,
            9_007_199_254_740_986,
            9_007_199_254_740_987,
        ),
        (
            9_007_199_254_740_988,
            0x4161_2e0b_e826_d693,
            9_007_199_254_740_989,
            9_007_199_254_740_991,
        ),
        (
            9_007_199_254_740_990,
            0x4161_2e0b_e826_d694,
            9_007_199_254_740_991,
            9_007_199_254_740_992,
        ),
    ] {
        let policy = product_ok(super::FixtureAdaptiveHorizonProfile::new(
            4.0,
            product_ok(super::adaptive_duration(minimum))?,
            product_ok(super::adaptive_duration(maximum))?,
            0.0,
        ))?;
        for (input_bits, selected) in [(bits - 1, minimum), (bits, expected), (bits + 1, above)] {
            let area = product_ok(super::AdaptiveArea::new(f64::from_bits(input_bits)))?;
            assert_eq!(
                product_ok(area.horizon(policy, debt))?.nanoseconds(),
                selected
            );
        }
    }
    Ok(())
}

/// Exact threshold equality is not silently classified as below-threshold debt.
/// # Errors
/// Preserve the checked numerical profile, literal area and debt construction.
/// # Panics
/// Detect a non-strict threshold or sign-dependent time-constant decision.
#[test]
fn adaptive_small_debt_threshold_equality_and_adjacent_bits_are_distinct() -> TestResult {
    let policy = product_ok(super::FixtureAdaptiveHorizonProfile::new(
        4.0,
        product_ok(super::adaptive_duration(SECOND))?,
        product_ok(super::adaptive_duration(16 * SECOND))?,
        1.0 / 8.0,
    ))?;
    let area = product_ok(super::AdaptiveArea::new(1.0 / 4.0))?;
    for sign in [-1.0, 1.0] {
        for (bits, expected) in [
            (0x3fbf_ffff_ffff_ffff, 16 * SECOND),
            (0x3fc0_0000_0000_0000, 2 * SECOND),
            (0x3fc0_0000_0000_0001, 2 * SECOND),
        ] {
            let debt = product_ok(super::PhaseDebt::new(sign * f64::from_bits(bits), 0.0))?;
            assert_eq!(
                product_ok(area.horizon(policy, debt))?.nanoseconds(),
                expected
            );
        }
    }
    Ok(())
}

/// Distinct admitted/rejected scalars must not become diagnostic payloads.
/// # Errors
/// Preserve checked valid profile, duration and healthy area construction failures.
/// # Panics
/// Detect leaked profile values, lost leaf identity or an invented nested cause.
#[test]
fn adaptive_profile_and_leaf_failures_keep_bounded_redacted_diagnostics() -> TestResult {
    let minimum = product_ok(super::adaptive_duration(SECOND))?;
    let maximum = product_ok(super::adaptive_duration(16 * SECOND))?;
    let first = product_ok(super::FixtureAdaptiveHorizonProfile::new(
        4.0, minimum, maximum, 0.0,
    ))?;
    let private_value = 9_876_543.25;
    let second = product_ok(super::FixtureAdaptiveHorizonProfile::new(
        private_value,
        minimum,
        maximum,
        1.0 / 32.0,
    ))?;
    let first_debug = format!("{first:?}");
    assert_eq!(first_debug, format!("{second:?}"));
    assert!(first_debug.len() <= 64);
    assert!(!first_debug.contains(&private_value.to_string()));
    assert!(!first_debug.contains("correction_time_ratio"));
    assert!(!first_debug.contains("small_debt_threshold"));
    let unsupported =
        ActiveAwakeNs::new((1_u64 << 53) + 1).duration_since(ActiveAwakeNs::new(0))?;
    let cases = [
        (
            super::adaptive_duration_seconds(unsupported).map(|_| ()),
            super::FixtureAdaptiveHorizonError::UnsupportedInterval,
        ),
        (
            super::AdaptiveArea::from_reference(first, -private_value, minimum).map(|_| ()),
            super::FixtureAdaptiveHorizonError::UnsupportedStatistic,
        ),
        (
            super::adaptive_duration(0).map(|_| ()),
            super::FixtureAdaptiveHorizonError::DurationConversion,
        ),
        (
            super::AdaptiveArea::new(-private_value).map(|_| ()),
            super::FixtureAdaptiveHorizonError::ValidationInvariant,
        ),
    ];
    for (result, expected) in cases {
        assert!(matches!(&result,
            Err(super::FixturePhaseError::AdaptiveHorizon { source }) if *source == expected));
        if let Err(super::FixturePhaseError::AdaptiveHorizon { source }) = result {
            let display = source.to_string();
            let debug = format!("{source:?}");
            assert!(display.starts_with("adaptive horizon:"));
            assert!(display.len() <= 96 && debug.len() <= 96);
            assert!(!display.contains(&private_value.to_string()));
            assert!(!debug.contains(&private_value.to_string()));
            assert!(source.source().is_none());
        }
    }
    assert_eq!(
        product_ok(super::AdaptiveArea::new(1.0 / 4.0))?.point,
        1.0 / 4.0
    );
    Ok(())
}

/// Actual arithmetic failures retain bounded operation facts and their original cause.
/// # Errors
/// Preserve the independent dyadic healthy arithmetic construction failure.
/// # Panics
/// Detect flattened sources, incorrect operation classification or leaked operands.
#[test]
fn adaptive_arithmetic_failure_preserves_original_typed_source() -> TestResult {
    let expected = super::FitError::Arithmetic {
        stage: super::ArithmeticStage::Coefficients,
        operation: super::ArithmeticOperation::Multiply,
        issue: crate::ArithmeticIssue::NonFinite,
    };
    let mut diagnostic = None;
    for right in [2.0, 4.0] {
        let result =
            super::adaptive_arithmetic(f64::MAX, right, super::ArithmeticOperation::Multiply);
        assert!(matches!(&result,
            Err(super::FixturePhaseError::AdaptiveHorizon {
                source: super::FixtureAdaptiveHorizonError::Arithmetic { source },
            }) if *source == expected));
        if let Err(error) = result {
            let adaptive = error
                .source()
                .and_then(|source| source.downcast_ref::<super::FixtureAdaptiveHorizonError>());
            assert!(matches!(adaptive,
                Some(super::FixtureAdaptiveHorizonError::Arithmetic { source }) if *source == expected));
            let numeric = adaptive
                .and_then(|source| source.source())
                .and_then(|source| source.downcast_ref::<super::FitError>());
            assert_eq!(numeric, Some(&expected));
            if let Some(source) = adaptive {
                let display = source.to_string();
                let debug = format!("{source:?}");
                assert!(display.len() <= 96 && debug.len() <= 192);
                assert!(!display.contains(&f64::MAX.to_string()));
                assert!(!debug.contains(&f64::MAX.to_string()));
                if let Some(previous) = &diagnostic {
                    assert_eq!(previous, &display);
                }
                diagnostic = Some(display);
            }
        }
    }
    assert_eq!(
        product_ok(super::adaptive_arithmetic(
            1.0 / 8.0,
            1.0 / 4.0,
            super::ArithmeticOperation::Multiply,
        ))?,
        1.0 / 32.0
    );
    Ok(())
}

/// Exact integer and conversion boundaries keep supported neighboring durations.
/// # Errors
/// Preserve supported typed durations and exact seconds conversion failures.
/// # Panics
/// Detect broadened duration admission or unchecked integer conversion.
#[test]
fn adaptive_private_numeric_boundaries_keep_healthy_neighbors() -> TestResult {
    let maximum = 1_u64 << 53;
    for nanos in [1, SECOND, maximum] {
        assert_eq!(
            product_ok(super::adaptive_duration(nanos))?.nanoseconds(),
            nanos
        );
    }
    for nanos in [0, maximum + 1, u64::MAX] {
        assert!(matches!(
            super::adaptive_duration(nanos),
            Err(super::FixturePhaseError::AdaptiveHorizon {
                source: super::FixtureAdaptiveHorizonError::DurationConversion,
            })
        ));
    }
    // The finite 2^24-second case exceeds 2^53ns while still fitting u64;
    // the larger powers additionally challenge integer conversion/scratch.
    for seconds in [
        f64::from_bits(1047_u64 << 52),
        f64::from_bits(1075_u64 << 52),
        f64::from_bits(1175_u64 << 52),
    ] {
        assert!(matches!(
            super::adaptive_ceil_nanoseconds(seconds),
            Err(super::FixturePhaseError::AdaptiveHorizon {
                source: super::FixtureAdaptiveHorizonError::DurationConversion,
            })
        ));
    }
    assert_eq!(product_ok(super::adaptive_ceil_nanoseconds(1.0))?, SECOND);
    assert_eq!(
        product_ok(super::adaptive_duration_seconds(product_ok(
            super::adaptive_duration(SECOND)
        )?))?,
        1.0
    );
    Ok(())
}

/// Raw area/statistical support refusals retain exact zero and finite neighbors.
/// # Errors
/// Preserve checked profile, duration and valid scalar construction failures.
/// # Panics
/// Detect unsupported scalar admission, lost signed zero or changed area factors.
#[test]
fn adaptive_private_area_boundaries_keep_healthy_neighbors() -> TestResult {
    for raw in [f64::NAN, f64::INFINITY, -1.0, f64::from_bits(1)] {
        assert!(matches!(
            super::AdaptiveArea::new(raw),
            Err(super::FixturePhaseError::AdaptiveHorizon {
                source: super::FixtureAdaptiveHorizonError::ValidationInvariant,
            })
        ));
    }
    for raw in [0.0_f64, -0.0, 1.0 / 4.0] {
        assert_eq!(
            product_ok(super::AdaptiveArea::new(raw))?.point.to_bits(),
            raw.to_bits()
        );
    }
    let duration = product_ok(super::adaptive_duration(SECOND))?;
    let profile = product_ok(super::FixtureAdaptiveHorizonProfile::new(
        4.0,
        duration,
        product_ok(super::adaptive_duration(16 * SECOND))?,
        0.0,
    ))?;
    for raw in [f64::NAN, f64::INFINITY, -1.0, f64::from_bits(1)] {
        assert!(matches!(
            super::AdaptiveArea::from_reference(profile, raw, duration),
            Err(super::FixturePhaseError::AdaptiveHorizon {
                source: super::FixtureAdaptiveHorizonError::UnsupportedStatistic,
            })
        ));
    }
    assert_eq!(
        product_ok(super::AdaptiveArea::from_reference(
            profile,
            1.0 / 64.0,
            duration
        ))?
        .point,
        1.0 / 32.0
    );
    for deviation in [0.0_f64, -0.0] {
        assert_eq!(
            product_ok(super::AdaptiveArea::from_reference(
                profile, deviation, duration
            ))?
            .point
            .to_bits(),
            0.0_f64.to_bits()
        );
    }
    let maximum = product_ok(super::adaptive_duration(1_u64 << 53))?;
    assert_eq!(
        product_ok(super::AdaptiveArea::from_reference(profile, 0.0, maximum))?.point,
        0.0
    );
    // RN((2^53 / 10^9) / 32): the power-of-two scaling is exact.
    assert_eq!(
        product_ok(super::AdaptiveArea::from_reference(
            profile,
            1.0 / 64.0,
            maximum
        ))?
        .point
        .to_bits(),
        0x4111_2e0b_e826_d695
    );
    let unsupported =
        product_ok(ActiveAwakeNs::new((1_u64 << 53) + 1).duration_since(ActiveAwakeNs::new(0)))?;
    assert!(matches!(
        super::AdaptiveArea::from_reference(profile, 0.0, unsupported),
        Err(super::FixturePhaseError::AdaptiveHorizon {
            source: super::FixtureAdaptiveHorizonError::UnsupportedInterval,
        })
    ));
    Ok(())
}

/// Assert a known-valid pure product trajectory before preserving its refusal.
/// # Errors
/// Preserve the typed failure as fixture context after its semantic assertion.
/// # Panics
/// Reject refusal of the independently specified supported inputs.
#[track_caller]
fn product_ok<T, E: fmt::Debug + fmt::Display>(value: Result<T, E>) -> TestResult<T> {
    assert!(
        value.is_ok(),
        "valid NoEffect fixture was refused: {:?}",
        value.as_ref().err().map(ToString::to_string)
    );
    value.map_err(|error| io::Error::other(format!("NoEffect fixture: {error:?}")).into())
}

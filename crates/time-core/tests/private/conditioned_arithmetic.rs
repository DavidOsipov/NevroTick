// SPDX-License-Identifier: GPL-2.0-only
//! Private N2a raw-guard and directed-primitive controls.
//!
//! Whole-fit original-rational oracles remain independent external tests. Literal
//! endpoint facts below were frozen with Python Fraction/adjacent-float controls
//! before this owner was implemented. Fabricated candidates challenge the actual
//! mandatory publishers, never manufacture a checked interval for a public fit.

use garde::Validate;

use super::{
    ArithmeticError, Interval, IntervalCandidate, PointCandidate, ScalarCandidate, ScalarEnclosure,
    checked_point, point_add, point_div_positive, point_mul, point_neg, point_sub, publish_scalar,
};

/// Preserve unexpected typed refusals separately from discriminating assertions.
type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Smallest positive representable binary64 value; no normal-floor substitution.
const TINY: f64 = f64::from_bits(1);

/// Assert literal endpoint bits, including canonical numerical zero.
/// # Panics
/// Fail when either directed endpoint differs from its frozen binary64 fact.
fn endpoints(interval: Interval, lower: u64, upper: u64) {
    assert_eq!(interval.lower().to_bits(), lower);
    assert_eq!(interval.upper().to_bits(), upper);
}

/// Raw scalar rules and their actual producer reject each nonfinite input.
/// # Errors
/// Preserve unexpected refusal of valid signed-subnormal and zero neighbors.
/// # Panics
/// Fail if mandatory admission is omitted or a valid representation is changed.
#[test]
fn raw_point_admission_is_mandatory_with_subnormal_neighbors() -> TestResult {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(PointCandidate { value }.validate().is_err());
        assert_eq!(checked_point(value), Err(ArithmeticError::NonFinite));
        assert_eq!(point_add(value, 0.0), Err(ArithmeticError::NonFinite));
        assert_eq!(point_sub(0.0, value), Err(ArithmeticError::NonFinite));
        assert_eq!(point_mul(0.0, value), Err(ArithmeticError::NonFinite));
        assert_eq!(
            point_div_positive(0.0, value),
            Err(ArithmeticError::NonFinite)
        );
        assert_eq!(point_neg(value), Err(ArithmeticError::NonFinite));
    }
    assert!(PointCandidate { value: -0.0 }.validate().is_err());
    for value in [-f64::MAX, -TINY, 0.0, TINY, f64::MAX] {
        assert!(PointCandidate { value }.validate().is_ok());
        assert_eq!(checked_point(value)?.to_bits(), value.to_bits());
    }
    assert_eq!(checked_point(-0.0)?.to_bits(), 0);
    assert_eq!(point_neg(-0.0)?.to_bits(), 0);
    assert_eq!(point_neg(-TINY)?.to_bits(), 1);
    assert_eq!(point_neg(TINY)?.to_bits(), 0x8000_0000_0000_0001);
    Ok(())
}

/// Separately rounded point arithmetic retains the frozen primitive bits/order.
/// # Errors
/// Preserve unexpected refusal of valid finite arithmetic and exact cancellation.
/// # Panics
/// Fail on reassociation, point clamping, signed-zero leakage or lost subnormals.
#[test]
fn point_primitives_preserve_rounded_bits_and_legal_zero() -> TestResult {
    assert_eq!(point_add(0.1, 0.2)?.to_bits(), 0x3fd3_3333_3333_3334);
    assert_eq!(point_mul(0.1, 0.2)?.to_bits(), 0x3f94_7ae1_47ae_147c);
    assert_eq!(
        point_div_positive(1.0, 3.0)?.to_bits(),
        0x3fd5_5555_5555_5555
    );
    assert_eq!(point_add(1.0, -1.0)?.to_bits(), 0);
    assert_eq!(point_sub(-1.0, -1.0)?.to_bits(), 0);
    assert_eq!(point_mul(-0.0, -1.0)?.to_bits(), 0);
    assert_eq!(point_div_positive(-0.0, TINY)?.to_bits(), 0);
    assert_eq!(point_mul(TINY, 1.0)?.to_bits(), 1);
    assert_eq!(point_mul(-TINY, 1.0)?.to_bits(), 0x8000_0000_0000_0001);
    assert_eq!(point_div_positive(TINY, 1.0)?.to_bits(), 1);
    assert_eq!(
        point_div_positive(1.0, f64::MAX)?.to_bits(),
        0x0004_0000_0000_0000
    );
    let large = 9_007_199_254_740_992.0;
    assert_eq!(point_add(point_add(large, 1.0)?, -large)?.to_bits(), 0);
    assert_eq!(
        point_add(large, point_add(1.0, -large)?)?.to_bits(),
        1.0_f64.to_bits()
    );
    Ok(())
}

/// Real finite-input overflow, nonpositive division and nonzero zero loss refuse.
/// # Errors
/// Preserve unexpected refusal of the nearest supported subnormal neighbors.
/// # Panics
/// Fail if an invalid primitive returns a finite fallback or the wrong typed cause.
#[test]
fn point_failures_distinguish_overflow_denominator_and_underflow() -> TestResult {
    assert_eq!(
        point_add(f64::MAX, f64::MAX),
        Err(ArithmeticError::NonFinite)
    );
    assert_eq!(
        point_sub(-f64::MAX, f64::MAX),
        Err(ArithmeticError::NonFinite)
    );
    assert_eq!(point_mul(f64::MAX, 2.0), Err(ArithmeticError::NonFinite));
    assert_eq!(
        point_div_positive(f64::MAX, TINY),
        Err(ArithmeticError::NonFinite)
    );
    for denominator in [-1.0, -TINY, -0.0, 0.0] {
        assert_eq!(
            point_div_positive(0.0, denominator),
            Err(ArithmeticError::NonPositiveDenominator)
        );
        assert_eq!(
            point_div_positive(1.0, denominator),
            Err(ArithmeticError::NonPositiveDenominator)
        );
    }
    for numerator in [-TINY, TINY] {
        assert_eq!(
            point_mul(numerator, 0.5),
            Err(ArithmeticError::PointUnderflow)
        );
        assert_eq!(
            point_div_positive(numerator, 2.0),
            Err(ArithmeticError::PointUnderflow)
        );
    }
    assert_eq!(point_mul(TINY, 1.5)?.to_bits(), 2);
    assert_eq!(point_div_positive(f64::from_bits(2), 2.0)?.to_bits(), 1);
    Ok(())
}

/// Every endpoint field/order is checked by the actual private interval producer.
/// # Errors
/// Preserve unexpected refusal of finite extrema, subnormals and normalized zeros.
/// # Panics
/// Fail if raw Garde coverage or mandatory construction omits a malformed field.
#[test]
fn raw_interval_guard_covers_fields_order_and_canonical_zero() -> TestResult {
    for pair in [
        [f64::NAN, 1.0],
        [0.0, f64::NAN],
        [f64::NEG_INFINITY, 1.0],
        [0.0, f64::INFINITY],
        [1.0, 0.0],
        [-0.0, 1.0],
        [0.0, -0.0],
    ] {
        assert!(IntervalCandidate { endpoints: pair }.validate().is_err());
        if pair[0].to_bits() != (-0.0_f64).to_bits() && pair[1].to_bits() != (-0.0_f64).to_bits() {
            assert!(matches!(
                Interval::new(pair[0], pair[1]),
                Err(ArithmeticError::InvalidInterval)
            ));
        }
    }
    for pair in [[-f64::MAX, f64::MAX], [-TINY, TINY], [0.0, 0.0]] {
        assert!(IntervalCandidate { endpoints: pair }.validate().is_ok());
        endpoints(
            Interval::new(pair[0], pair[1])?,
            pair[0].to_bits(),
            pair[1].to_bits(),
        );
    }
    endpoints(Interval::new(-0.0, -0.0)?, 0, 0);
    assert!(matches!(
        Interval::exact(f64::NAN),
        Err(ArithmeticError::InvalidInterval)
    ));
    Ok(())
}

/// Explicit exact identities preserve finite endpoints without gratuitous widening.
/// # Errors
/// Preserve any unexpected refusal of exact identities or interval independence.
/// # Panics
/// Fail if zeros/ones/equal points widen or equal nonsingleton operands collapse.
#[test]
fn interval_exact_identities_preserve_zero_one_and_independence() -> TestResult {
    let zero = Interval::exact(-0.0)?;
    let one = Interval::exact(1.0)?;
    let range = Interval::new(-f64::MAX, f64::MAX)?;
    for result in [
        range.add(zero)?,
        zero.add(range)?,
        range.sub(zero)?,
        range.mul(one)?,
        one.mul(range)?,
        range.div_positive(one)?,
    ] {
        endpoints(result, (-f64::MAX).to_bits(), f64::MAX.to_bits());
    }
    endpoints(zero.mul(range)?, 0, 0);
    endpoints(range.mul(zero)?, 0, 0);
    endpoints(Interval::exact(-3.0)?.sub(Interval::exact(-3.0)?)?, 0, 0);
    endpoints(
        Interval::exact(3.0)?.div_positive(Interval::exact(3.0)?)?,
        1.0_f64.to_bits(),
        1.0_f64.to_bits(),
    );
    endpoints(zero.div_positive(one)?, 0, 0);
    let small = Interval::new(-2.0, 3.0)?;
    endpoints(zero.sub(small)?, (-3.0_f64).to_bits(), 2.0_f64.to_bits());
    endpoints(small.neg()?, (-3.0_f64).to_bits(), 2.0_f64.to_bits());
    let difference = small.sub(small)?;
    assert!(difference.lower() < -4.0 && difference.upper() > 4.0);
    let nonzero = Interval::new(2.0, 3.0)?;
    let quotient = nonzero.div_positive(nonzero)?;
    assert!(quotient.lower() < 1.0 && quotient.upper() > 1.0);
    endpoints(Interval::from_error(TINY, 0.0)?, 1, 1);
    Ok(())
}

/// Frozen exact-real endpoint facts discriminate directed binary64 rounding.
/// # Errors
/// Preserve unexpected refusal of the independent finite primitive facts.
/// # Panics
/// Fail if bounds use nearest-only arithmetic or the normal minimum for widening.
#[test]
fn interval_primitive_bits_match_frozen_outward_facts() -> TestResult {
    endpoints(
        Interval::exact(0.1)?.add(Interval::exact(0.2)?)?,
        0x3fd3_3333_3333_3333,
        0x3fd3_3333_3333_3335,
    );
    endpoints(
        Interval::exact(0.1)?.mul(Interval::exact(0.2)?)?,
        0x3f94_7ae1_47ae_147b,
        0x3f94_7ae1_47ae_147d,
    );
    endpoints(
        Interval::exact(1.0)?.div_positive(Interval::exact(3.0)?)?,
        0x3fd5_5555_5555_5554,
        0x3fd5_5555_5555_5556,
    );
    endpoints(
        Interval::exact(2.0)?.mul(Interval::exact(3.0)?)?,
        0x4017_ffff_ffff_ffff,
        0x4018_0000_0000_0001,
    );
    endpoints(
        Interval::exact(3.0)?.sub(Interval::exact(2.0)?)?,
        0x3fef_ffff_ffff_ffff,
        0x3ff0_0000_0000_0001,
    );
    endpoints(Interval::exact(TINY)?.mul(Interval::exact(0.5)?)?, 0, 1);
    endpoints(
        Interval::exact(-TINY)?.mul(Interval::exact(0.5)?)?,
        0x8000_0000_0000_0001,
        1,
    );
    endpoints(
        Interval::exact(TINY)?.div_positive(Interval::exact(2.0)?)?,
        0,
        1,
    );
    let reciprocal = Interval::exact(1.0)?.div_positive(Interval::exact(f64::MAX)?)?;
    endpoints(reciprocal, 0x0003_ffff_ffff_ffff, 0x0004_0000_0000_0001);
    Ok(())
}

/// Four-corner multiplication/division handles all signs and square's zero vertex.
/// # Errors
/// Preserve unexpected refusal of supported fixed sign/geometry cases.
/// # Panics
/// Fail on omitted endpoint combinations or a square lower bound above zero.
#[test]
fn interval_four_corners_and_square_use_exact_sign_geometry() -> TestResult {
    endpoints(
        Interval::new(-2.0, 3.0)?.mul(Interval::new(-4.0, 5.0)?)?,
        0xc028_0000_0000_0001,
        0x402e_0000_0000_0001,
    );
    for (left, right) in [([2.0, 3.0], [4.0, 5.0]), ([-3.0, -2.0], [-5.0, -4.0])] {
        endpoints(
            Interval::new(left[0], left[1])?.mul(Interval::new(right[0], right[1])?)?,
            0x401f_ffff_ffff_ffff,
            0x402e_0000_0000_0001,
        );
    }
    for (left, right) in [([2.0, 3.0], [-5.0, -4.0]), ([-3.0, -2.0], [4.0, 5.0])] {
        endpoints(
            Interval::new(left[0], left[1])?.mul(Interval::new(right[0], right[1])?)?,
            0xc02e_0000_0000_0001,
            0xc01f_ffff_ffff_ffff,
        );
    }
    endpoints(
        Interval::new(-6.0, 9.0)?.div_positive(Interval::new(2.0, 3.0)?)?,
        0xc008_0000_0000_0001,
        0x4012_0000_0000_0001,
    );
    endpoints(
        Interval::new(-2.0, 3.0)?.square()?,
        0,
        0x4022_0000_0000_0001,
    );
    for interval in [Interval::new(2.0, 3.0)?, Interval::new(-3.0, -2.0)?] {
        endpoints(
            interval.square()?,
            0x400f_ffff_ffff_ffff,
            0x4022_0000_0000_0001,
        );
    }
    endpoints(Interval::exact(TINY)?.square()?, 0, 1);
    endpoints(Interval::exact(0.0)?.square()?, 0, 0);
    endpoints(
        Interval::exact(1.0)?.square()?,
        1.0_f64.to_bits(),
        1.0_f64.to_bits(),
    );
    Ok(())
}

/// Invalid denominator margins and exhausted finite bounds refuse atomically.
/// # Errors
/// Preserve unexpected refusal of exact maximal endpoints with zero/one identities.
/// # Panics
/// Fail if infinity, fallback geometry or unsupported outward bounds escape.
#[test]
fn interval_failures_keep_positive_margin_and_finite_outward_bounds() -> TestResult {
    let one = Interval::exact(1.0)?;
    for denominator in [
        Interval::exact(0.0)?,
        Interval::new(0.0, 1.0)?,
        Interval::new(-1.0, 1.0)?,
        Interval::new(-2.0, -1.0)?,
    ] {
        assert!(matches!(
            one.div_positive(denominator),
            Err(ArithmeticError::NonPositiveDenominator)
        ));
        assert!(matches!(
            Interval::exact(0.0)?.div_positive(denominator),
            Err(ArithmeticError::NonPositiveDenominator)
        ));
    }
    let maximum = Interval::exact(f64::MAX)?;
    for result in [
        maximum.add(one),
        maximum.sub(Interval::exact(-f64::MAX)?),
        maximum.mul(Interval::exact(2.0)?),
        maximum.div_positive(Interval::exact(TINY)?),
        maximum.square(),
    ] {
        assert!(matches!(result, Err(ArithmeticError::ErrorRange)));
    }
    endpoints(maximum.mul(one)?, f64::MAX.to_bits(), f64::MAX.to_bits());
    assert!(matches!(
        Interval::from_error(0.0, -TINY),
        Err(ArithmeticError::InvalidInterval)
    ));
    assert!(matches!(
        Interval::from_error(f64::NAN, 0.0),
        Err(ArithmeticError::NonFinite)
    ));
    assert!(matches!(
        Interval::from_error(0.0, f64::INFINITY),
        Err(ArithmeticError::NonFinite)
    ));
    assert!(matches!(
        Interval::from_error(f64::MAX, f64::MAX),
        Err(ArithmeticError::ErrorRange)
    ));
    endpoints(Interval::from_error(TINY, -0.0)?, 1, 1);
    endpoints(Interval::from_error(0.0, TINY)?, 0x8000_0000_0000_0001, 1);
    Ok(())
}

/// Final raw output admission covers every field, point containment and error.
/// # Errors
/// Preserve unexpected refusal of valid bounded scalar representations.
/// # Panics
/// Fail if the actual publisher bypasses Garde or accepts a fabricated bad field.
#[test]
fn scalar_raw_faults_cannot_bypass_mandatory_output_admission() -> TestResult {
    for values in [
        [f64::NAN, -1.0, 1.0, 1.0],
        [0.0, f64::NEG_INFINITY, 1.0, 1.0],
        [0.0, -1.0, f64::INFINITY, 1.0],
        [0.0, -1.0, 1.0, f64::INFINITY],
        [0.0, 1.0, -1.0, 1.0],
        [2.0, -1.0, 1.0, 1.0],
        [-2.0, -1.0, 1.0, 1.0],
        [0.0, -1.0, 1.0, -1.0],
        [0.0, -1.0, 1.0, 0.0],
        [-0.0, -1.0, 1.0, 1.0],
        [0.0, -0.0, 1.0, 1.0],
        [0.0, -1.0, -0.0, 1.0],
        [0.0, 0.0, 0.0, -0.0],
    ] {
        assert!(ScalarCandidate { values }.validate().is_err());
        assert!(matches!(
            publish_scalar(ScalarCandidate { values }),
            Err(ArithmeticError::InvalidInterval)
        ));
    }
    for values in [[0.0, -1.0, 1.0, 1.0], [TINY, 0.0, TINY, TINY], [0.0; 4]] {
        assert!(ScalarCandidate { values }.validate().is_ok());
        let result = publish_scalar(ScalarCandidate { values })?;
        assert_eq!(result.value().to_bits(), values[0].to_bits());
        assert_eq!(result.lower().to_bits(), values[1].to_bits());
        assert_eq!(result.upper().to_bits(), values[2].to_bits());
        assert_eq!(result.absolute_error().to_bits(), values[3].to_bits());
    }
    Ok(())
}

/// Hull publication preserves the actual point and nonzero exact-target facts.
/// # Errors
/// Preserve unexpected refusal of supported singleton, small-gap and hull cases.
/// # Panics
/// Fail on dropped exact subnormal corrections, point replacement or nearest-only error.
#[test]
fn scalar_hull_and_outward_error_cover_zero_cancellation_and_outside_points() -> TestResult {
    let tiny = ScalarEnclosure::new(0.0, Interval::exact(TINY)?)?;
    assert_eq!(tiny.value().to_bits(), 0);
    assert_eq!(tiny.lower().to_bits(), 0);
    assert_eq!(tiny.upper().to_bits(), 1);
    assert_eq!(tiny.absolute_error().to_bits(), 1);
    let exact = ScalarEnclosure::new(-0.0, Interval::exact(-0.0)?)?;
    assert_eq!(exact.value().to_bits(), 0);
    assert_eq!(exact.lower().to_bits(), 0);
    assert_eq!(exact.upper().to_bits(), 0);
    assert_eq!(exact.absolute_error().to_bits(), 0);
    let outside = ScalarEnclosure::new(1.0, Interval::new(2.0, 3.0)?)?;
    assert_eq!(outside.value().to_bits(), 1.0_f64.to_bits());
    assert_eq!(outside.lower().to_bits(), 1.0_f64.to_bits());
    assert_eq!(outside.upper().to_bits(), 3.0_f64.to_bits());
    assert_eq!(outside.absolute_error().to_bits(), 0x4000_0000_0000_0001);
    let adjacent = ScalarEnclosure::new(
        1.0,
        Interval::new(
            f64::from_bits(0x3fef_ffff_ffff_ffff),
            f64::from_bits(0x3ff0_0000_0000_0001),
        )?,
    )?;
    assert_eq!(adjacent.absolute_error().to_bits(), 0x3cb0_0000_0000_0001);
    assert!(matches!(
        ScalarEnclosure::new(f64::NAN, Interval::exact(0.0)?),
        Err(ArithmeticError::NonFinite)
    ));
    assert!(matches!(
        ScalarEnclosure::new(-f64::MAX, Interval::exact(f64::MAX)?),
        Err(ArithmeticError::ErrorRange)
    ));
    Ok(())
}

/// Diagnostic formatting exposes fixed types/causes, without numeric payloads.
/// # Errors
/// Preserve unexpected construction failure of recognizable private scalar data.
/// # Panics
/// Fail when interval/scalar fields leak or the typed causes lose their fixed text.
#[test]
fn private_arithmetic_debug_and_errors_redact_numeric_payloads() -> TestResult {
    let interval = Interval::new(-123_456.75, 987_654.25)?;
    let scalar = ScalarEnclosure::new(777.125, interval)?;
    assert_eq!(format!("{interval:?}"), "Interval { .. }");
    assert_eq!(format!("{scalar:?}"), "ScalarEnclosure { .. }");
    for (cause, display, debug) in [
        (
            ArithmeticError::NonFinite,
            "nonfinite conditioned point arithmetic",
            "NonFinite",
        ),
        (
            ArithmeticError::NonPositiveDenominator,
            "conditioned denominator lacks positive margin",
            "NonPositiveDenominator",
        ),
        (
            ArithmeticError::PointUnderflow,
            "conditioned nonzero point underflow",
            "PointUnderflow",
        ),
        (
            ArithmeticError::InvalidInterval,
            "invalid conditioned enclosure representation",
            "InvalidInterval",
        ),
        (
            ArithmeticError::ErrorRange,
            "conditioned enclosure exceeds finite range",
            "ErrorRange",
        ),
    ] {
        assert_eq!(cause.to_string(), display);
        assert_eq!(format!("{cause:?}"), debug);
        assert!(std::error::Error::source(&cause).is_none());
    }
    Ok(())
}

/// A zero endpoint does not make a nonsingleton interval identically zero.
///
/// Independent exact ranges are [0,1]-[1,1]=[-1,0] and
/// [-1,0]-[1,1]=[-2,-1]; both endpoints must stay enclosed.
/// # Errors
/// Preserve unexpected refusal of these supported finite checked intervals.
/// # Panics
/// Fail if either zero endpoint activates the zero-singleton shortcut.
#[test]
fn interval_subtraction_preserves_nonsingleton_zero_endpoints() -> TestResult {
    let one = Interval::exact(1.0)?;
    let nonnegative_left = Interval::new(0.0, 1.0)?.sub(one)?;
    assert!(nonnegative_left.lower() <= -1.0);
    assert!(nonnegative_left.upper() >= 0.0);
    let nonpositive_left = Interval::new(-1.0, 0.0)?.sub(one)?;
    assert!(nonpositive_left.lower() <= -2.0);
    assert!(nonpositive_left.upper() >= -1.0);
    endpoints(
        Interval::exact(0.0)?.sub(one)?,
        0xbff0_0000_0000_0000,
        0xbff0_0000_0000_0000,
    );
    Ok(())
}

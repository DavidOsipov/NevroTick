// SPDX-License-Identifier: GPL-2.0-only
//! Private S154 arithmetic controls; no checked history or chart is fabricated.

use time_domain::MonotonicNs;

use super::super::super::{ArithmeticIssue, ArithmeticOperation, FitError, NormalizationError};
use super::{
    ArithmeticStage, FixtureChartBoundIssue, FixtureChartBoundOperation, FixtureChartError,
    FixtureChartStep, NormalizationComponent, UNIT_ROUNDOFF, bound_arithmetic, elapsed,
    value_arithmetic,
};

/// A private arithmetic check returns its existing typed failure directly.
type TestResult = Result<(), FixtureChartError>;

/// Exact zeros and positive underflow are separate, independently fixed cases.
/// # Errors
/// Preserve any unexpected upper-operation rejection beside legal neighbors.
/// # Panics
/// Reject incorrect zero signs, silent positive-zero loss or a missing next-up.
#[test]
fn upper_operators_preserve_zero_and_underflow_neighbors() -> TestResult {
    for (operation, left, right) in [
        (FixtureChartBoundOperation::Add, -0.0, 0.0),
        (FixtureChartBoundOperation::Multiply, 0.0, f64::MAX),
        (FixtureChartBoundOperation::Divide, -0.0, 1.0),
    ] {
        assert_eq!(bound_arithmetic(left, right, operation)?.to_bits(), 0);
    }
    let smallest = f64::from_bits(1);
    let largest_subnormal = f64::from_bits(0x000f_ffff_ffff_ffff);
    let next_minimum = f64::from_bits(0x0010_0000_0000_0001);
    let below_one = f64::from_bits(0x3fef_ffff_ffff_ffff);
    for (operation, left, right, expected) in [
        (
            FixtureChartBoundOperation::Multiply,
            smallest,
            0.5,
            f64::MIN_POSITIVE,
        ),
        (
            FixtureChartBoundOperation::Divide,
            smallest,
            2.0,
            f64::MIN_POSITIVE,
        ),
        (
            FixtureChartBoundOperation::Add,
            smallest,
            smallest,
            f64::MIN_POSITIVE,
        ),
        (
            FixtureChartBoundOperation::Add,
            largest_subnormal,
            0.0,
            f64::MIN_POSITIVE,
        ),
        (
            FixtureChartBoundOperation::Add,
            f64::MIN_POSITIVE,
            0.0,
            next_minimum,
        ),
        (
            FixtureChartBoundOperation::Divide,
            1.0,
            2.0,
            f64::from_bits(0x3fe0_0000_0000_0001),
        ),
        (
            FixtureChartBoundOperation::Multiply,
            f64::MIN_POSITIVE,
            below_one,
            next_minimum,
        ),
    ] {
        assert_eq!(
            bound_arithmetic(left, right, operation)?.to_bits(),
            expected.to_bits()
        );
    }
    Ok(())
}

/// Reject the complete raw operand domain before exact-zero shortcuts.
/// # Panics
/// Reject negative/nonfinite acceptance, wrong precedence or a finite MAX cap.
#[test]
fn upper_operators_reject_invalid_and_exhausted_bounds() {
    for operation in [
        FixtureChartBoundOperation::Add,
        FixtureChartBoundOperation::Multiply,
        FixtureChartBoundOperation::Divide,
    ] {
        for invalid in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let expected = Err(FixtureChartError::Bound {
                operation,
                issue: FixtureChartBoundIssue::InvalidOperand,
            });
            assert_eq!(bound_arithmetic(invalid, 0.0, operation), expected);
            assert_eq!(bound_arithmetic(0.0, invalid, operation), expected);
        }
    }
    for divisor in [0.0, -0.0] {
        assert_eq!(
            bound_arithmetic(0.0, divisor, FixtureChartBoundOperation::Divide),
            Err(FixtureChartError::Bound {
                operation: FixtureChartBoundOperation::Divide,
                issue: FixtureChartBoundIssue::NonPositiveDivisor,
            })
        );
    }
    for (operation, right) in [
        (FixtureChartBoundOperation::Add, 0.0),
        (FixtureChartBoundOperation::Multiply, 2.0),
        (FixtureChartBoundOperation::Divide, 0.5),
    ] {
        assert_eq!(
            bound_arithmetic(f64::MAX, right, operation),
            Err(FixtureChartError::Bound {
                operation,
                issue: FixtureChartBoundIssue::NonFiniteResult,
            })
        );
    }
}

/// Bound widening never changes the coordinate owner's supported domain.
/// # Errors
/// Preserve any unexpected rejection of the signed-zero neighboring value.
/// # Panics
/// Reject a wrong primitive/stage/cause or clamped coordinate underflow.
#[test]
fn value_primitives_preserve_steps_and_reject_underflow() -> TestResult {
    assert_eq!(
        value_arithmetic(-0.0, 0.0, FixtureChartStep::AnchorCenter)?.to_bits(),
        (-0.0_f64).to_bits()
    );
    for (right, issue) in [
        (0.5, ArithmeticIssue::Subnormal),
        (UNIT_ROUNDOFF, ArithmeticIssue::ZeroUnderflow),
    ] {
        assert_eq!(
            value_arithmetic(f64::MIN_POSITIVE, right, FixtureChartStep::RateTimesElapsed),
            Err(FixtureChartError::Arithmetic {
                step: FixtureChartStep::RateTimesElapsed,
                source: FitError::Arithmetic {
                    stage: ArithmeticStage::Response,
                    operation: ArithmeticOperation::Multiply,
                    issue,
                },
            })
        );
        assert_eq!(
            bound_arithmetic(
                f64::MIN_POSITIVE,
                right,
                FixtureChartBoundOperation::Multiply
            )?
            .to_bits(),
            f64::MIN_POSITIVE.to_bits()
        );
    }
    for (step, left, right, operation, issue) in [
        (
            FixtureChartStep::SecondsDivide,
            1.0,
            0.0,
            ArithmeticOperation::Divide,
            ArithmeticIssue::NonPositiveDenominator,
        ),
        (
            FixtureChartStep::RateTimesElapsed,
            f64::MAX,
            2.0,
            ArithmeticOperation::Multiply,
            ArithmeticIssue::NonFinite,
        ),
        (
            FixtureChartStep::Displacement,
            f64::MAX,
            f64::MAX,
            ArithmeticOperation::Add,
            ArithmeticIssue::NonFinite,
        ),
        (
            FixtureChartStep::AnchorCenter,
            f64::MAX,
            -f64::MAX,
            ArithmeticOperation::Subtract,
            ArithmeticIssue::NonFinite,
        ),
        (
            FixtureChartStep::TrackingPhase,
            f64::MAX,
            -f64::MAX,
            ArithmeticOperation::Subtract,
            ArithmeticIssue::NonFinite,
        ),
        (
            FixtureChartStep::Rate,
            f64::MAX,
            -f64::MAX,
            ArithmeticOperation::Subtract,
            ArithmeticIssue::NonFinite,
        ),
    ] {
        assert_eq!(
            value_arithmetic(left, right, step),
            Err(FixtureChartError::Arithmetic {
                step,
                source: FitError::Arithmetic {
                    stage: ArithmeticStage::Response,
                    operation,
                    issue,
                },
            })
        );
    }
    Ok(())
}

/// Signed differences reuse exact integer limits rather than absolute casts.
/// # Errors
/// Preserve unexpected conversion rejection beside exact limit neighbors.
/// # Panics
/// Reject a lost sign, changed integer magnitude, overflow or weakened guard.
#[test]
fn signed_elapsed_preserves_exact_integer_limits() -> TestResult {
    for (query, reference, expected_seconds) in [
        (0, 0, 0),
        (1, 0, 0x3e11_2e0b_e826_d695),
        (0, 1, 0xbe11_2e0b_e826_d695),
    ] {
        let result = elapsed(MonotonicNs::new(query), MonotonicNs::new(reference))?;
        assert_eq!(result.seconds.to_bits(), expected_seconds);
    }
    let limit = 1_u64 << 53;
    let result = elapsed(MonotonicNs::new(limit), MonotonicNs::new(0))?;
    assert_eq!(result.nanoseconds.to_bits(), 0x4340_0000_0000_0000);
    for query in [limit + 1, u64::MAX] {
        assert!(matches!(
            elapsed(MonotonicNs::new(query), MonotonicNs::new(0)),
            Err(FixtureChartError::Normalization {
                source: NormalizationError::IntegerRange {
                    component: NormalizationComponent::Time,
                },
            })
        ));
    }
    Ok(())
}

// SPDX-License-Identifier: GPL-2.0-only
//! Source-derived controls for raw private scalar guards, separate from the
//! independently authored public numeric fixtures. No invalid domain value is
//! constructed here. These tests make no claim that admitted geometry can overflow.

use std::io::{self, Write};

use super::{
    ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitError, PointField, PointIssue,
    arithmetic, point_issue,
};

/// Exact typed geometry cause expected from one raw private-helper failure.
fn error(operation: ArithmeticOperation, issue: ArithmeticIssue) -> FitError {
    FitError::Arithmetic {
        stage: ArithmeticStage::Geometry,
        operation,
        issue,
    }
}

/// Emit this test's exact zero public-point/fit count directly to stderr.
///
/// These raw guard controls call neither public constructor nor fit. The receipt
/// precedes assertions, so an assertion failure cannot conceal reached work.
/// # Errors
/// Propagates stderr failure rather than silently omitting required accounting.
fn profile_counts() -> io::Result<()> {
    io::stderr()
        .lock()
        .write_all(b"CORE_PROFILE_COUNTS fits=0 points=0\n")
}

/// Nonfinite operands/results are rejected without entering a domain constructor.
/// # Panics
/// Fails if raw nonfinite arithmetic escapes or loses its actual operation code.
/// # Errors
/// Propagates required direct-stderr accounting failure.
#[test]
fn nonfinite_operands_and_results_preserve_typed_cause() -> io::Result<()> {
    profile_counts()?;
    for (left, right, operation) in [
        (f64::NAN, 1.0, ArithmeticOperation::Add),
        (f64::INFINITY, 1.0, ArithmeticOperation::Subtract),
        // Operand rejection must hold even when the raw quotient would be zero.
        (0.0, f64::INFINITY, ArithmeticOperation::Divide),
        (f64::MAX, 2.0, ArithmeticOperation::Multiply),
        (f64::MAX, f64::MIN_POSITIVE, ArithmeticOperation::Divide),
    ] {
        assert_eq!(
            arithmetic(left, right, operation, ArithmeticStage::Geometry),
            Err(error(operation, ArithmeticIssue::NonFinite))
        );
    }
    assert_eq!(
        arithmetic(
            0.0,
            1.0,
            ArithmeticOperation::Divide,
            ArithmeticStage::Geometry
        ),
        Ok(0.0)
    );
    Ok(())
}

/// Nonzero subnormal raw operands/results are unsupported, with normal neighbors.
/// # Panics
/// Fails if a subnormal value escapes or a legal minimum-normal value is rejected.
/// # Errors
/// Propagates required direct-stderr accounting failure.
#[test]
fn subnormal_operands_and_results_have_a_normal_neighbor() -> io::Result<()> {
    profile_counts()?;
    for (left, right, operation) in [
        (f64::from_bits(1), 0.0, ArithmeticOperation::Add),
        // 2^-1023 is subnormal even though doubling it produces MIN_POSITIVE.
        (
            f64::from_bits(1_u64 << 51),
            2.0,
            ArithmeticOperation::Multiply,
        ),
        (f64::MIN_POSITIVE, 2.0, ArithmeticOperation::Divide),
        (f64::MIN_POSITIVE, 0.5, ArithmeticOperation::Multiply),
    ] {
        assert_eq!(
            arithmetic(left, right, operation, ArithmeticStage::Geometry),
            Err(error(operation, ArithmeticIssue::Subnormal))
        );
    }
    assert_eq!(
        arithmetic(
            f64::MIN_POSITIVE,
            1.0,
            ArithmeticOperation::Multiply,
            ArithmeticStage::Geometry
        ),
        Ok(f64::MIN_POSITIVE)
    );
    Ok(())
}

/// Loss of nonzero normal operands to signed zero is distinct from cancellation.
/// # Panics
/// Fails if true multiplication/division underflow is accepted or misclassified.
/// # Errors
/// Propagates required direct-stderr accounting failure.
#[test]
fn nonzero_to_zero_underflow_is_explicit() -> io::Result<()> {
    profile_counts()?;
    for (left, right, operation) in [
        (
            f64::MIN_POSITIVE,
            f64::MIN_POSITIVE,
            ArithmeticOperation::Multiply,
        ),
        (
            -f64::MIN_POSITIVE,
            f64::MIN_POSITIVE,
            ArithmeticOperation::Multiply,
        ),
        (f64::MIN_POSITIVE, f64::MAX, ArithmeticOperation::Divide),
    ] {
        assert_eq!(
            arithmetic(left, right, operation, ArithmeticStage::Geometry),
            Err(error(operation, ArithmeticIssue::ZeroUnderflow))
        );
    }
    Ok(())
}

/// A denominator must be positive even when division by negative would be finite.
/// # Panics
/// Fails if either signed zero or a negative denominator is accepted.
/// # Errors
/// Propagates required direct-stderr accounting failure.
#[test]
fn division_requires_a_positive_denominator() -> io::Result<()> {
    profile_counts()?;
    for denominator in [0.0, -0.0, -1.0] {
        assert_eq!(
            arithmetic(
                1.0,
                denominator,
                ArithmeticOperation::Divide,
                ArithmeticStage::Geometry
            ),
            Err(error(
                ArithmeticOperation::Divide,
                ArithmeticIssue::NonPositiveDenominator
            ))
        );
    }
    Ok(())
}

/// Ordinary cancellation and zero operands retain legal zero/normal controls.
/// # Panics
/// Fails if cancellation is called underflow or signed zero loses its raw bits.
/// # Errors
/// Propagates required direct-stderr accounting failure.
#[test]
fn cancellation_and_zero_operands_remain_legal() -> io::Result<()> {
    profile_counts()?;
    assert_eq!(
        arithmetic(
            1.0,
            1.0,
            ArithmeticOperation::Subtract,
            ArithmeticStage::Geometry
        ),
        Ok(0.0)
    );
    assert_eq!(
        arithmetic(
            0.0,
            f64::MIN_POSITIVE,
            ArithmeticOperation::Multiply,
            ArithmeticStage::Geometry
        ),
        Ok(0.0)
    );
    let zero = arithmetic(
        -0.0,
        1.0,
        ArithmeticOperation::Divide,
        ArithmeticStage::Geometry,
    );
    assert_eq!(zero.map(f64::to_bits), Ok((-0.0_f64).to_bits()));
    assert_eq!(
        arithmetic(
            0.0,
            1.0,
            ArithmeticOperation::Add,
            ArithmeticStage::Geometry
        ),
        Ok(1.0)
    );
    Ok(())
}

/// The shared scalar rule preserves nonfinite/subnormal/range precedence.
/// # Panics
/// Fails if the typed field predicate changes its fixed first-issue ordering.
/// # Errors
/// Propagates required direct-stderr accounting failure.
#[test]
fn scalar_rule_precedence_is_explicit() -> io::Result<()> {
    profile_counts()?;
    for field in [
        PointField::RelativeSeconds,
        PointField::PhaseSeconds,
        PointField::VarianceDivisor,
    ] {
        assert_eq!(point_issue(f64::NAN, field), Some(PointIssue::NonFinite));
        assert_eq!(
            point_issue(f64::from_bits(1), field),
            Some(PointIssue::Subnormal)
        );
    }
    assert_eq!(
        point_issue(1.0, PointField::RelativeSeconds),
        Some(PointIssue::OutOfRange)
    );
    assert_eq!(
        point_issue(0.0, PointField::VarianceDivisor),
        Some(PointIssue::OutOfRange)
    );
    assert_eq!(
        point_issue(f64::MIN_POSITIVE, PointField::PhaseSeconds),
        None
    );
    assert_eq!(point_issue(-0.0, PointField::RelativeSeconds), None);
    Ok(())
}

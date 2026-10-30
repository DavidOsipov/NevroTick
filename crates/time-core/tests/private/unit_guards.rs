// SPDX-License-Identifier: GPL-2.0-only
//! Source-derived controls for raw private scalar guards, separate from the
//! independently authored public numeric fixtures. No invalid domain value is
//! constructed here. These tests make no claim that admitted geometry can overflow.

use std::io::{self, Write};

use garde::Validate;

use super::{
    ArithmeticIssue, ArithmeticOperation, ArithmeticStage, Coefficient, FitError, PointCandidate,
    PointField, PointIssue, arithmetic, point_issue,
};

/// Exact typed geometry cause expected from one raw private-helper failure.
fn error(operation: ArithmeticOperation, issue: ArithmeticIssue) -> FitError {
    FitError::Arithmetic {
        stage: ArithmeticStage::Geometry,
        operation,
        issue,
    }
}

/// Exact output-admission cause, distinct from the raw geometry helper's stage.
fn result_error(operation: ArithmeticOperation, issue: ArithmeticIssue) -> FitError {
    FitError::Arithmetic {
        stage: ArithmeticStage::Result,
        operation,
        issue,
    }
}

/// Charge a finite local raw attempt before entering its actual private owner.
///
/// This is a test-local source recipe, not public point/fit work or a claim about
/// the number of internal operations performed by an arbitrary bad variant.
/// # Errors
/// Rejects counter overflow or an attempt beyond the test's admitted ceiling.
fn charge_raw(counter: &mut usize, ceiling: usize) -> io::Result<()> {
    let next = counter
        .checked_add(1)
        .ok_or_else(|| io::Error::other("raw guard attempt counter overflow"))?;
    if next > ceiling {
        return Err(io::Error::other("raw guard attempt ceiling"));
    }
    *counter = next;
    Ok(())
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

/// Nonfinite operands and raw coefficient outputs preserve their typed causes.
/// # Panics
/// Fails if raw nonfinite arithmetic escapes or loses its actual operation code.
/// # Errors
/// Propagates required stderr accounting or a raw-attempt charge failure.
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
    let mut coefficient_attempts = 0;
    for (value, operation) in [
        (f64::NAN, ArithmeticOperation::Add),
        (f64::INFINITY, ArithmeticOperation::Subtract),
        (f64::NEG_INFINITY, ArithmeticOperation::Multiply),
        (
            f64::from_bits(0xfff8_0000_0000_0001),
            ArithmeticOperation::Divide,
        ),
    ] {
        charge_raw(&mut coefficient_attempts, 4)?;
        assert_eq!(
            Coefficient::new(value, operation).map(|coefficient| coefficient.value.to_bits()),
            Err(result_error(operation, ArithmeticIssue::NonFinite))
        );
    }
    assert_eq!(coefficient_attempts, 4);
    Ok(())
}

/// Nonzero subnormal raw operands/results are unsupported, with normal neighbors.
/// # Panics
/// Fails if a subnormal value escapes or a legal minimum-normal value is rejected.
/// # Errors
/// Propagates required stderr accounting or a raw-attempt charge failure.
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
    raw_coefficient_boundary_controls()?;
    Ok(())
}

/// Raw coefficient admission rejects signed subnormals and preserves legal bits.
///
/// Unlike the point envelope, this output contract allows finite extrema. These
/// four rejected and eight accepted values perform no arithmetic or public fit.
/// # Panics
/// Fails if rejection loses Result-stage/operation identity or accepted bits change.
/// # Errors
/// Propagates a raw-attempt charge failure before the next constructor call.
fn raw_coefficient_boundary_controls() -> io::Result<()> {
    let mut coefficient_attempts = 0;
    for (bits, operation) in [
        (0x0000_0000_0000_0001, ArithmeticOperation::Add),
        (0x8000_0000_0000_0001, ArithmeticOperation::Subtract),
        (0x000f_ffff_ffff_ffff, ArithmeticOperation::Multiply),
        (0x800f_ffff_ffff_ffff, ArithmeticOperation::Divide),
    ] {
        charge_raw(&mut coefficient_attempts, 12)?;
        assert_eq!(
            Coefficient::new(f64::from_bits(bits), operation)
                .map(|coefficient| coefficient.value.to_bits()),
            Err(result_error(operation, ArithmeticIssue::Subnormal))
        );
    }
    for (bits, operation) in [
        (0x0000_0000_0000_0000, ArithmeticOperation::Add),
        (0x8000_0000_0000_0000, ArithmeticOperation::Subtract),
        (0x0010_0000_0000_0000, ArithmeticOperation::Multiply),
        (0x8010_0000_0000_0000, ArithmeticOperation::Divide),
        (0x7fef_ffff_ffff_ffff, ArithmeticOperation::Add),
        (0xffef_ffff_ffff_ffff, ArithmeticOperation::Subtract),
        (0x3ff0_0000_0000_0000, ArithmeticOperation::Multiply),
        (0xbff0_0000_0000_0000, ArithmeticOperation::Divide),
    ] {
        charge_raw(&mut coefficient_attempts, 12)?;
        assert_eq!(
            Coefficient::new(f64::from_bits(bits), operation)
                .map(|coefficient| coefficient.value.to_bits()),
            Ok(bits)
        );
    }
    assert_eq!(coefficient_attempts, 12);
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
/// Propagates required stderr accounting or a raw-attempt charge failure.
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
    raw_candidate_field_controls()?;
    Ok(())
}

/// Challenge each raw derived field while leaving the other two fields healthy.
///
/// Inclusive endpoints and exact representable neighbors are fixed independently
/// of the production predicate. No checked point is fabricated, and rejection is
/// required from the actual garde derive rather than the constructor's fallback.
/// # Panics
/// Fails if a single bad field passes or an admitted endpoint/neighbor is rejected.
/// # Errors
/// Propagates a raw-attempt charge failure before the next validation call.
fn raw_candidate_field_controls() -> io::Result<()> {
    let mut validation_attempts = 0;
    for field in [
        PointField::RelativeSeconds,
        PointField::PhaseSeconds,
        PointField::VarianceDivisor,
    ] {
        for value in [
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::from_bits(0x0000_0000_0000_0001),
            f64::from_bits(0x8000_0000_0000_0001),
        ] {
            raw_candidate_field_rejection(field, value, &mut validation_attempts)?;
        }
    }
    for (field, value) in [
        (
            PointField::RelativeSeconds,
            f64::from_bits(0xc0b0_0000_0000_0001),
        ),
        (PointField::RelativeSeconds, f64::MIN_POSITIVE),
        (
            PointField::PhaseSeconds,
            f64::from_bits(0xbff0_0000_0000_0001),
        ),
        (
            PointField::PhaseSeconds,
            f64::from_bits(0x3ff0_0000_0000_0001),
        ),
        (
            PointField::VarianceDivisor,
            f64::from_bits(0x3faf_ffff_ffff_ffff),
        ),
        (
            PointField::VarianceDivisor,
            f64::from_bits(0x4030_0000_0000_0001),
        ),
    ] {
        raw_candidate_field_rejection(field, value, &mut validation_attempts)?;
    }
    for (relative_seconds, phase_seconds, variance_divisor) in [
        (-1.0, 0.0, 1.0),
        (-4096.0, -1.0, 1.0 / 16.0),
        (0.0, 1.0, 16.0),
        (-0.0, -0.0, 1.0),
        (-f64::MIN_POSITIVE, f64::MIN_POSITIVE, 1.0),
        (
            f64::from_bits(0xc0af_ffff_ffff_ffff),
            f64::from_bits(0x3fef_ffff_ffff_ffff),
            f64::from_bits(0x3fb0_0000_0000_0001),
        ),
        (
            -1.0,
            f64::from_bits(0xbfef_ffff_ffff_ffff),
            f64::from_bits(0x402f_ffff_ffff_ffff),
        ),
    ] {
        charge_raw(&mut validation_attempts, 28)?;
        assert!(
            PointCandidate {
                relative_seconds,
                phase_seconds,
                variance_divisor
            }
            .validate()
            .is_ok()
        );
    }
    assert_eq!(validation_attempts, 28);
    Ok(())
}

/// Require one raw bad field to be rejected by its derived validation wiring.
///
/// The other two raw fields are fixed healthy neighbors. This creates only an
/// unvalidated candidate, never a checked point or coefficient.
/// # Panics
/// Fails if the field designated invalid by the caller's fixed case is accepted.
/// # Errors
/// Propagates an attempt charge failure before invoking the actual garde derive.
fn raw_candidate_field_rejection(
    field: PointField,
    value: f64,
    validation_attempts: &mut usize,
) -> io::Result<()> {
    charge_raw(validation_attempts, 28)?;
    let mut raw = PointCandidate {
        relative_seconds: -1.0,
        phase_seconds: 0.0,
        variance_divisor: 1.0,
    };
    match field {
        PointField::RelativeSeconds => raw.relative_seconds = value,
        PointField::PhaseSeconds => raw.phase_seconds = value,
        PointField::VarianceDivisor => raw.variance_divisor = value,
    }
    assert!(raw.validate().is_err());
    Ok(())
}

// SPDX-License-Identifier: GPL-2.0-only
//! Source-derived residual scalar/raw primitive controls, outside the critical
//! module and separate from independent public numerical expectations. No point,
//! input, line or statistics report is constructed and no fit/statistics API runs.

use std::{
    error::Error,
    io::{self, Write},
};

use garde::Validate;

use super::{
    ArithmeticOperation, ArithmeticStage, FitError, ResidualError, ResidualField, ResidualIssue,
    ResidualScalar, ResidualStep, StatisticCandidate, residual_arithmetic, residual_issue,
};
use crate::{ArithmeticIssue, PointField, PointIssue};

/// Write one complete actual zero-work record before any assertion can fail.
///
/// Raw private controls construct no public points and call neither fit nor the
/// statistics API. A single `write_all` holds this record together on direct stderr.
/// # Errors
/// Propagates accounting-sink failure instead of silently dropping the record.
fn profile_counts() -> io::Result<()> {
    io::stderr()
        .lock()
        .write_all(b"CORE_STATS_PROFILE_COUNTS stats=0 points=0\n")
}

/// Build the exact typed Response-stage arithmetic cause without raw operands.
fn arithmetic_error(
    step: ResidualStep,
    operation: ArithmeticOperation,
    issue: ArithmeticIssue,
) -> ResidualError {
    ResidualError::Arithmetic {
        step,
        source: FitError::Arithmetic {
            stage: ArithmeticStage::Response,
            operation,
            issue,
        },
    }
}

/// Every statistic uses the same mandatory scalar/garde admission and precedence.
/// # Panics
/// Fails if raw rejection, shared garde execution, positive output range or signed
/// zero preservation disagrees with the frozen private construction contract.
/// # Errors
/// Propagates the required direct-stderr accounting failure.
#[test]
fn scalar_admission_is_shared_and_preserves_supported_bits() -> io::Result<()> {
    profile_counts()?;
    for field in [
        ResidualField::WeightedSse,
        ResidualField::ResidualScale,
        ResidualField::NormalizedDataVariance,
    ] {
        for (value, issue) in [
            (f64::NAN, ResidualIssue::NonFinite),
            (f64::INFINITY, ResidualIssue::NonFinite),
            (f64::NEG_INFINITY, ResidualIssue::NonFinite),
            (f64::from_bits(1), ResidualIssue::Subnormal),
            (-f64::from_bits(1), ResidualIssue::Subnormal),
            (-f64::MIN_POSITIVE, ResidualIssue::Negative),
            (-1.0, ResidualIssue::Negative),
        ] {
            assert_eq!(residual_issue(value), Some(issue));
            assert!(StatisticCandidate { value }.validate().is_err());
            assert_eq!(
                ResidualScalar::new(value, field).map(|scalar| scalar.value.to_bits()),
                Err(ResidualError::InvalidStatistic { field, issue })
            );
        }
        for value in [0.0, -0.0, f64::MIN_POSITIVE, 1.0, 2.0, 64.0, f64::MAX] {
            assert_eq!(residual_issue(value), None);
            assert!(StatisticCandidate { value }.validate().is_ok());
            assert_eq!(
                ResidualScalar::new(value, field).map(|scalar| scalar.value.to_bits()),
                Ok(value.to_bits())
            );
        }
    }
    Ok(())
}

/// Each named step selects its actual primitive and preserves its exact context.
/// # Panics
/// Fails if repeated Divide steps become indistinguishable or a primitive/stage
/// is relabelled when raw nonfinite operands are rejected.
/// # Errors
/// Propagates the required direct-stderr accounting failure.
#[test]
fn every_step_preserves_its_primitive_and_typed_source() -> io::Result<()> {
    profile_counts()?;
    for (step, operation) in [
        (ResidualStep::PhaseDifference, ArithmeticOperation::Subtract),
        (ResidualStep::Trend, ArithmeticOperation::Multiply),
        (ResidualStep::Residual, ArithmeticOperation::Subtract),
        (ResidualStep::Square, ArithmeticOperation::Multiply),
        (ResidualStep::Weight, ArithmeticOperation::Divide),
        (ResidualStep::Accumulate, ArithmeticOperation::Add),
        (ResidualStep::Scale, ArithmeticOperation::Divide),
        (
            ResidualStep::NormalizeMultiply,
            ArithmeticOperation::Multiply,
        ),
        (ResidualStep::NormalizeDivide, ArithmeticOperation::Divide),
    ] {
        assert_eq!(step.operation(), operation);
        assert_eq!(
            residual_arithmetic(f64::NAN, 1.0, step),
            Err(arithmetic_error(
                step,
                operation,
                ArithmeticIssue::NonFinite
            ))
        );
    }
    Ok(())
}

/// Frozen rejected raw operand/step tuples; no public product operation occurs.
const REJECTED_ARITHMETIC_CASES: [(f64, f64, ResidualStep, ArithmeticOperation, ArithmeticIssue);
    15] = [
    // Raw 0/infinity would be zero; the nonfinite divisor still rejects.
    (
        0.0,
        f64::INFINITY,
        ResidualStep::Weight,
        ArithmeticOperation::Divide,
        ArithmeticIssue::NonFinite,
    ),
    // Raw subnormal addition/subtraction would round to an ordinary normal.
    (
        f64::from_bits(1),
        1.0,
        ResidualStep::Accumulate,
        ArithmeticOperation::Add,
        ArithmeticIssue::Subnormal,
    ),
    (
        1.0,
        f64::from_bits(1),
        ResidualStep::PhaseDifference,
        ArithmeticOperation::Subtract,
        ArithmeticIssue::Subnormal,
    ),
    // 2^-1023 * 2 is MIN_POSITIVE; a normalized result cannot admit the input.
    (
        f64::from_bits(1_u64 << 51),
        2.0,
        ResidualStep::Trend,
        ArithmeticOperation::Multiply,
        ArithmeticIssue::Subnormal,
    ),
    // Raw equal-subnormal division would yield normal one.
    (
        f64::from_bits(1),
        f64::from_bits(1),
        ResidualStep::Scale,
        ArithmeticOperation::Divide,
        ArithmeticIssue::Subnormal,
    ),
    (
        f64::MIN_POSITIVE,
        0.5,
        ResidualStep::Square,
        ArithmeticOperation::Multiply,
        ArithmeticIssue::Subnormal,
    ),
    (
        f64::MIN_POSITIVE,
        2.0,
        ResidualStep::NormalizeDivide,
        ArithmeticOperation::Divide,
        ArithmeticIssue::Subnormal,
    ),
    (
        f64::MIN_POSITIVE,
        f64::MIN_POSITIVE,
        ResidualStep::Square,
        ArithmeticOperation::Multiply,
        ArithmeticIssue::ZeroUnderflow,
    ),
    (
        -f64::MIN_POSITIVE,
        f64::MIN_POSITIVE,
        ResidualStep::Trend,
        ArithmeticOperation::Multiply,
        ArithmeticIssue::ZeroUnderflow,
    ),
    (
        f64::MIN_POSITIVE,
        f64::MAX,
        ResidualStep::NormalizeDivide,
        ArithmeticOperation::Divide,
        ArithmeticIssue::ZeroUnderflow,
    ),
    (
        f64::MAX,
        2.0,
        ResidualStep::NormalizeMultiply,
        ArithmeticOperation::Multiply,
        ArithmeticIssue::NonFinite,
    ),
    (
        f64::MAX,
        f64::MIN_POSITIVE,
        ResidualStep::Weight,
        ArithmeticOperation::Divide,
        ArithmeticIssue::NonFinite,
    ),
    (
        1.0,
        -1.0,
        ResidualStep::Scale,
        ArithmeticOperation::Divide,
        ArithmeticIssue::NonPositiveDenominator,
    ),
    (
        1.0,
        0.0,
        ResidualStep::Weight,
        ArithmeticOperation::Divide,
        ArithmeticIssue::NonPositiveDenominator,
    ),
    (
        1.0,
        -0.0,
        ResidualStep::NormalizeDivide,
        ArithmeticOperation::Divide,
        ArithmeticIssue::NonPositiveDenominator,
    ),
];

/// Full raw operand domains reject invalid inputs even when a result would mask it.
/// # Panics
/// Fails if unsupported operands/results, zero underflow or denominator failures
/// escape the real residual wrapper or lose the actual shared typed cause.
/// # Errors
/// Propagates the required direct-stderr accounting failure.
#[test]
fn raw_arithmetic_rejects_masked_operands_and_unsupported_results() -> io::Result<()> {
    profile_counts()?;
    for (left, right, step, operation, issue) in REJECTED_ARITHMETIC_CASES {
        assert_eq!(
            residual_arithmetic(left, right, step),
            Err(arithmetic_error(step, operation, issue))
        );
    }
    Ok(())
}

/// Neighboring supported raw arithmetic preserves zeros, cancellation and normals.
/// # Panics
/// Fails if legal operations are falsely rejected or signed-zero bits change.
/// # Errors
/// Propagates the required direct-stderr accounting failure.
#[test]
fn legal_raw_arithmetic_preserves_supported_neighbors() -> io::Result<()> {
    profile_counts()?;
    let controls: [(f64, f64, ResidualStep, f64); 9] = [
        (1.0, 1.0, ResidualStep::PhaseDifference, 0.0),
        (-0.0, 0.0, ResidualStep::Residual, -0.0),
        (0.0, f64::MIN_POSITIVE, ResidualStep::Square, 0.0),
        (
            f64::MIN_POSITIVE,
            1.0,
            ResidualStep::Trend,
            f64::MIN_POSITIVE,
        ),
        (-0.0, 1.0, ResidualStep::Weight, -0.0),
        (1.0, 1.0, ResidualStep::Accumulate, 2.0),
        (
            f64::MIN_POSITIVE,
            1.0,
            ResidualStep::Scale,
            f64::MIN_POSITIVE,
        ),
        (2.0, 64.0, ResidualStep::NormalizeMultiply, 128.0),
        (128.0, 64.0, ResidualStep::NormalizeDivide, 2.0),
    ];
    for (left, right, step, expected) in controls {
        assert_eq!(
            residual_arithmetic(left, right, step).map(f64::to_bits),
            Ok(expected.to_bits())
        );
    }
    Ok(())
}

/// Public error formatting/source remain total for manually constructed variants.
/// # Panics
/// Fails if a typed source is erased or a closed field/issue context is omitted.
/// # Errors
/// Propagates the required direct-stderr accounting failure.
#[test]
fn public_error_sources_and_formatting_are_total_and_bounded() -> io::Result<()> {
    profile_counts()?;
    let source = FitError::Point {
        field: PointField::PhaseSeconds,
        issue: PointIssue::NonFinite,
    };
    for error in [
        ResidualError::Fit(source),
        ResidualError::Arithmetic {
            step: ResidualStep::Weight,
            source,
        },
    ] {
        let retained = error
            .source()
            .and_then(|cause| cause.downcast_ref::<FitError>());
        assert_eq!(retained, Some(&source));
        assert!(format!("{error}").len() < 128);
        assert!(format!("{error:?}").len() < 256);
    }
    for error in [
        ResidualError::InvalidStatistic {
            field: ResidualField::WeightedSse,
            issue: ResidualIssue::Negative,
        },
        ResidualError::ValidationInvariant {
            field: ResidualField::ResidualScale,
        },
        ResidualError::CountInvariant,
    ] {
        assert!(error.source().is_none());
        assert!(format!("{error}").len() < 128);
        assert!(format!("{error:?}").len() < 256);
    }
    assert_eq!(
        ResidualError::InvalidStatistic {
            field: ResidualField::NormalizedDataVariance,
            issue: ResidualIssue::Subnormal,
        }
        .to_string(),
        "residual statistic NormalizedDataVariance: Subnormal"
    );
    Ok(())
}

// SPDX-License-Identifier: GPL-2.0-only
//! Six source-derived raw guard scopes, separate from independent public facts.
//!
//! These controls construct no context, observation, monotonic or UTC value,
//! public point/input/payload, coefficient or statistic. They invoke no public
//! normalization, getter, fit, statistics or domain-arithmetic entry point. Garde
//! candidates, closed errors and raw private primitives retain their own bounds;
//! none is a fabricated invalid checked domain value or numeric preparation.

use std::{
    error::Error,
    io::{self, Write},
};

use garde::Validate;
use time_domain::DomainError;

use super::{
    ArithmeticOperation, ArithmeticStage, FitError, MAX_EXACT_INTEGER, MAX_PHASE_NUMERATOR,
    NormalizationComponent, NormalizationError, NormalizationStep, ObservationCandidate,
    ObservationError, PointField, PointIssue, integer_to_f64, normalization_arithmetic,
    phase_in_range, phase_rule, point_issue,
};
use crate::ArithmeticIssue;

/// Emit one complete zero-public-work record before assertions on every path.
///
/// Six raw test bodies are the separately source-audited guard-scope bound. No
/// public or domain operation occurs before or after this single assembled
/// `write_all`; assertion unwinding therefore retains the same actual zeros.
/// # Errors
/// Propagates a failed accounting sink rather than manufacturing passing evidence.
fn profile_counts() -> io::Result<()> {
    io::stderr().lock().write_all(
        b"CORE_NORMALIZATION_PROFILE_COUNTS normalizations=0 observations=0 contexts=0 monotonic=0 coordinates=0 exchanges=0 getters=0 stats=0 points=0\n",
    )
}

/// Build an independently specified typed cause without invoking the producer map.
///
/// Closed explicit component/step/stage/operation/issue inputs are preserved;
/// this total test helper performs no arithmetic, publication or external effect.
fn arithmetic_error(
    component: NormalizationComponent,
    step: NormalizationStep,
    stage: ArithmeticStage,
    operation: ArithmeticOperation,
    issue: ArithmeticIssue,
) -> NormalizationError {
    NormalizationError::Arithmetic {
        component,
        step,
        source: FitError::Arithmetic {
            stage,
            operation,
            issue,
        },
    }
}

/// Supply every closed domain error code for formatting, without domain operations.
///
/// This total fixed-array helper constructs only error variants, never a checked
/// domain value, input payload, numerical result or external effect.
fn domain_error_sources() -> [DomainError; 8] {
    [
        DomainError::CoordinateOutOfRange,
        DomainError::InvalidEraWindow,
        DomainError::NoEraCandidate,
        DomainError::AmbiguousEra,
        DomainError::ArithmeticOverflow,
        DomainError::DurationOutOfOrder,
        DomainError::NanosecondsOutOfRange,
        DomainError::ResidualOutOfRange,
    ]
}

/// Supply bounded deliberately foreign fit causes without a point or fit call.
///
/// The fixed variants challenge total public error formatting; no checked value,
/// raw float, arithmetic, normalization result or external effect is produced.
fn fit_error_sources() -> [FitError; 4] {
    [
        FitError::CountOutOfRange,
        FitError::NonIncreasingTime { index: usize::MAX },
        FitError::Point {
            field: PointField::VarianceDivisor,
            issue: PointIssue::Subnormal,
        },
        FitError::Arithmetic {
            stage: ArithmeticStage::Geometry,
            operation: ArithmeticOperation::Multiply,
            issue: ArithmeticIssue::ZeroUnderflow,
        },
    ]
}

/// The exact integer phase predicate and garde adapter agree at both endpoints.
/// # Errors
/// Propagates the required zero-work record failure.
/// # Panics
/// Fails if an endpoint/neighbor or a full-width integer bypasses the shared rule.
#[test]
fn phase_predicate_and_garde_share_exact_integer_bounds() -> io::Result<()> {
    profile_counts()?;
    for (value, accepted) in [
        (i128::MIN, false),
        (-MAX_PHASE_NUMERATOR - 1, false),
        (-MAX_PHASE_NUMERATOR, true),
        (-1, true),
        (0, true),
        (1, true),
        (MAX_PHASE_NUMERATOR, true),
        (MAX_PHASE_NUMERATOR + 1, false),
        (i128::MAX, false),
    ] {
        assert_eq!(phase_in_range(value), accepted);
        assert_eq!(phase_rule(&value, &()).is_ok(), accepted);
        assert_eq!(
            ObservationCandidate {
                phase_numerator: value,
                variance_divisor: 1.0,
            }
            .validate()
            .is_ok(),
            accepted
        );
    }
    Ok(())
}

/// Both raw scalar fields have real mandatory garde coverage and legal neighbors.
/// # Errors
/// Propagates the required zero-work record failure.
/// # Panics
/// Fails if either raw field lacks its maintained validation or issue precedence.
#[test]
fn divisor_garde_reuses_parent_precedence_and_independent_field_coverage() -> io::Result<()> {
    profile_counts()?;
    for (value, issue) in [
        (f64::NAN, PointIssue::NonFinite),
        (f64::INFINITY, PointIssue::NonFinite),
        (f64::NEG_INFINITY, PointIssue::NonFinite),
        (f64::from_bits(1), PointIssue::Subnormal),
        (-f64::from_bits(1), PointIssue::Subnormal),
        (-0.0, PointIssue::OutOfRange),
        (0.0, PointIssue::OutOfRange),
        (-1.0, PointIssue::OutOfRange),
        (f64::MIN_POSITIVE, PointIssue::OutOfRange),
        (32.0, PointIssue::OutOfRange),
    ] {
        assert_eq!(point_issue(value, PointField::VarianceDivisor), Some(issue));
        assert!(
            ObservationCandidate {
                phase_numerator: 0,
                variance_divisor: value,
            }
            .validate()
            .is_err()
        );
    }
    for divisor in [1.0 / 16.0, 1.0, 16.0] {
        assert_eq!(point_issue(divisor, PointField::VarianceDivisor), None);
        assert!(
            ObservationCandidate {
                phase_numerator: MAX_PHASE_NUMERATOR,
                variance_divisor: divisor,
            }
            .validate()
            .is_ok()
        );
        assert!(
            ObservationCandidate {
                phase_numerator: MAX_PHASE_NUMERATOR + 1,
                variance_divisor: divisor,
            }
            .validate()
            .is_err()
        );
    }
    assert!(
        ObservationCandidate {
            phase_numerator: i128::MIN,
            variance_divisor: f64::NAN,
        }
        .validate()
        .is_err()
    );
    Ok(())
}

/// Each closed component/step selects its real stage/primitive and typed cause.
/// # Errors
/// Propagates the required zero-work record failure.
/// # Panics
/// Fails if a map changes, a supported primitive is rejected or context is lost.
#[test]
fn primitive_maps_preserve_distinct_component_and_operation_owners() -> io::Result<()> {
    profile_counts()?;
    for (component, stage) in [
        (NormalizationComponent::Time, ArithmeticStage::Geometry),
        (NormalizationComponent::Phase, ArithmeticStage::Response),
    ] {
        assert_eq!(component.stage(), stage);
        for (step, operation, expected) in [
            (
                NormalizationStep::IntegerHighProduct,
                ArithmeticOperation::Multiply,
                8.0,
            ),
            (
                NormalizationStep::IntegerLowSum,
                ArithmeticOperation::Add,
                6.0,
            ),
            (
                NormalizationStep::SecondsDivide,
                ArithmeticOperation::Divide,
                0.5,
            ),
        ] {
            assert_eq!(step.operation(), operation);
            assert_eq!(
                normalization_arithmetic(2.0, 4.0, component, step),
                Ok(expected)
            );
            assert_eq!(
                normalization_arithmetic(f64::NAN, 1.0, component, step),
                Err(arithmetic_error(
                    component,
                    step,
                    stage,
                    operation,
                    ArithmeticIssue::NonFinite
                ))
            );
        }
    }
    Ok(())
}

/// Raw arithmetic rejects unsupported intermediates beside legal signed-zero cases.
/// # Errors
/// Propagates the required zero-work record failure.
/// # Panics
/// Fails if unsupported values are masked or the actual guarded cause is relabelled.
#[test]
fn raw_arithmetic_rejections_and_signed_zero_neighbors_keep_actual_causes() -> io::Result<()> {
    profile_counts()?;
    for (component, stage) in [
        (NormalizationComponent::Time, ArithmeticStage::Geometry),
        (NormalizationComponent::Phase, ArithmeticStage::Response),
    ] {
        for (left, right, step, operation, issue) in [
            (
                1.0,
                0.0,
                NormalizationStep::SecondsDivide,
                ArithmeticOperation::Divide,
                ArithmeticIssue::NonPositiveDenominator,
            ),
            (
                1.0,
                -1.0,
                NormalizationStep::SecondsDivide,
                ArithmeticOperation::Divide,
                ArithmeticIssue::NonPositiveDenominator,
            ),
            (
                f64::MIN_POSITIVE,
                2.0,
                NormalizationStep::SecondsDivide,
                ArithmeticOperation::Divide,
                ArithmeticIssue::Subnormal,
            ),
            (
                f64::MIN_POSITIVE,
                f64::MAX,
                NormalizationStep::SecondsDivide,
                ArithmeticOperation::Divide,
                ArithmeticIssue::ZeroUnderflow,
            ),
            (
                f64::MIN_POSITIVE,
                f64::MIN_POSITIVE,
                NormalizationStep::IntegerHighProduct,
                ArithmeticOperation::Multiply,
                ArithmeticIssue::ZeroUnderflow,
            ),
            (
                f64::MAX,
                2.0,
                NormalizationStep::IntegerHighProduct,
                ArithmeticOperation::Multiply,
                ArithmeticIssue::NonFinite,
            ),
            (
                f64::from_bits(1),
                1.0,
                NormalizationStep::IntegerLowSum,
                ArithmeticOperation::Add,
                ArithmeticIssue::Subnormal,
            ),
        ] {
            assert_eq!(
                normalization_arithmetic(left, right, component, step),
                Err(arithmetic_error(component, step, stage, operation, issue))
            );
        }
        for (left, right, step) in [
            (-0.0, 1.0, NormalizationStep::IntegerHighProduct),
            (-0.0, -0.0, NormalizationStep::IntegerLowSum),
            (-0.0, 1.0, NormalizationStep::SecondsDivide),
        ] {
            assert_eq!(
                normalization_arithmetic(left, right, component, step).map(f64::to_bits),
                Ok((-0.0_f64).to_bits())
            );
        }
    }
    Ok(())
}

/// Independent successful integer bits and the raw envelope define conversion.
/// # Errors
/// Propagates the required zero-work record failure.
/// # Panics
/// Fails if a supported result differs or an unsupported integer escapes rejection.
#[test]
fn integer_conversion_matches_independent_supported_bits_and_rejects_outside_envelope()
-> io::Result<()> {
    profile_counts()?;
    // Frozen normalization-facts-v2 integer controls, never the stub output.
    let supported = [
        (0, 0x0000_0000_0000_0000_u64),
        (1, 0x3ff0_0000_0000_0000),
        ((1_u64 << 32) - 1, 0x41ef_ffff_ffe0_0000),
        (1_u64 << 32, 0x41f0_0000_0000_0000),
        ((1_u64 << 32) + 1, 0x41f0_0000_0010_0000),
        (MAX_EXACT_INTEGER - 1, 0x433f_ffff_ffff_ffff),
        (MAX_EXACT_INTEGER, 0x4340_0000_0000_0000),
    ];
    for component in [NormalizationComponent::Time, NormalizationComponent::Phase] {
        for (value, expected_bits) in supported {
            assert_eq!(
                integer_to_f64(value, component).map(f64::to_bits),
                Ok(expected_bits)
            );
        }
        for value in [MAX_EXACT_INTEGER + 1, u64::MAX] {
            assert_eq!(
                integer_to_f64(value, component),
                Err(NormalizationError::IntegerRange { component })
            );
        }
    }
    Ok(())
}

/// Every manually forged public error retains total bounded typed context/source.
/// # Errors
/// Propagates the required zero-work record failure.
/// # Panics
/// Fails if formatting leaks payloads, loses a retained cause or assumes a real producer.
#[test]
fn public_error_formatting_and_source_projection_are_total_and_bounded() -> io::Result<()> {
    profile_counts()?;
    for error in [
        ObservationError::PhaseOutOfRange,
        ObservationError::Divisor {
            issue: PointIssue::NonFinite,
        },
        ObservationError::Divisor {
            issue: PointIssue::Subnormal,
        },
        ObservationError::Divisor {
            issue: PointIssue::OutOfRange,
        },
        ObservationError::ValidationInvariant,
    ] {
        assert!(error.source().is_none());
        assert!(format!("{error}").len() < 128);
        assert!(format!("{error:?}").len() < 256);
    }
    for error in [
        NormalizationError::CountOutOfRange,
        NormalizationError::ContextMismatch { index: usize::MAX },
        NormalizationError::NonIncreasingTime { index: usize::MAX },
        NormalizationError::ElapsedRange { index: usize::MAX },
        NormalizationError::SpanTooShort,
        NormalizationError::IntegerRange {
            component: NormalizationComponent::Time,
        },
        NormalizationError::ConversionInvariant {
            component: NormalizationComponent::Phase,
        },
    ] {
        assert!(error.source().is_none());
        assert!(format!("{error}").len() < 128);
        assert!(format!("{error:?}").len() < 256);
    }
    for source in domain_error_sources() {
        let error = NormalizationError::Domain {
            index: usize::MAX,
            source,
        };
        assert_eq!(
            error
                .source()
                .and_then(|cause| cause.downcast_ref::<DomainError>()),
            Some(&source)
        );
        assert!(format!("{error}").len() < 128);
        assert!(format!("{error:?}").len() < 256);
    }
    for source in fit_error_sources() {
        for error in [
            NormalizationError::Point {
                index: usize::MAX,
                source,
            },
            NormalizationError::Input(source),
            NormalizationError::Arithmetic {
                component: NormalizationComponent::Time,
                step: NormalizationStep::IntegerHighProduct,
                source,
            },
        ] {
            assert_eq!(
                error
                    .source()
                    .and_then(|cause| cause.downcast_ref::<FitError>()),
                Some(&source)
            );
            assert!(format!("{error}").len() < 128);
            assert!(format!("{error:?}").len() < 256);
        }
    }
    Ok(())
}

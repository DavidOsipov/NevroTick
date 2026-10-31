// SPDX-License-Identifier: GPL-2.0-only
//! Four source-derived raw guard scopes, separate from public fixture facts.
//!
//! These tests invoke no public constructor, projection or domain operation and
//! forge no checked history, terms, output, observation or duration. Only raw
//! garde candidates, raw private arithmetic and closed error codes are used.

use std::{
    error::Error,
    io::{self, Write},
};

use garde::Validate;
use time_domain::DomainError;

use super::{
    ArithmeticOperation, ArithmeticStage, FitError, FixtureProjectionError, FixtureProjectionField,
    FixtureProjectionIssue, FixtureProjectionStep, NormalizationComponent, NormalizationError,
    ProjectionCandidate, projection_arithmetic, term_issue, term_rule,
};
use crate::{ArithmeticIssue, PointField, PointIssue};

/// Closed guard scope names keep every zero-public-work record bounded.
#[derive(Clone, Copy)]
enum GuardCase {
    /// Twelve independently specified raw scalar predicate controls.
    Predicate,
    /// Eighteen single-bad candidates, five healthy and one multiple-bad control.
    Candidate,
    /// Fifty common step controls and six multiplication-only controls.
    Arithmetic,
    /// Fixed closed errors and retained typed source controls.
    Errors,
}

/// Emit one complete fixed record before assertions on every test path.
///
/// Only the closed four-name scope is accepted; no public work occurs before or
/// after the record, so a later assertion failure leaves its actual zeros intact.
/// # Errors
/// Propagates sink failure rather than reporting a passing accounting record.
fn profile_counts(case: GuardCase) -> io::Result<()> {
    let record: &[u8] = match case {
        GuardCase::Predicate => b"CORE_PROJECTION_PROFILE case=projection_private_predicate histories=0 appends=0 observations=0 contexts=0 monotonic=0 coordinates=0 exchanges=0 getters=0 terms=0 projections=0\n",
        GuardCase::Candidate => b"CORE_PROJECTION_PROFILE case=projection_private_candidate histories=0 appends=0 observations=0 contexts=0 monotonic=0 coordinates=0 exchanges=0 getters=0 terms=0 projections=0\n",
        GuardCase::Arithmetic => b"CORE_PROJECTION_PROFILE case=projection_private_arithmetic histories=0 appends=0 observations=0 contexts=0 monotonic=0 coordinates=0 exchanges=0 getters=0 terms=0 projections=0\n",
        GuardCase::Errors => b"CORE_PROJECTION_PROFILE case=projection_private_errors histories=0 appends=0 observations=0 contexts=0 monotonic=0 coordinates=0 exchanges=0 getters=0 terms=0 projections=0\n",
    };
    io::stderr().lock().write_all(record)
}

/// Build only a raw candidate with one explicitly chosen potentially bad field.
///
/// The other two fields are healthy normal ones; this total fixed-work helper
/// performs no checked construction, validation, mutation or external effect.
fn single_bad_candidate(field: FixtureProjectionField, value: f64) -> ProjectionCandidate {
    ProjectionCandidate {
        half_delay_seconds: if field == FixtureProjectionField::HalfDelaySeconds {
            value
        } else {
            1.0
        },
        dispersion_seconds: if field == FixtureProjectionField::DispersionSeconds {
            value
        } else {
            1.0
        },
        skew_seconds_per_second: if field == FixtureProjectionField::SkewSecondsPerSecond {
            value
        } else {
            1.0
        },
    }
}

/// Independently supply the actual expected closed arithmetic cause structure.
///
/// This helper constructs only a public error value from fixed role/operation
/// codes; it invokes no primitive, producer map or checked domain construction.
fn expected_arithmetic_error(
    step: FixtureProjectionStep,
    operation: ArithmeticOperation,
    issue: ArithmeticIssue,
) -> FixtureProjectionError {
    FixtureProjectionError::Arithmetic {
        step,
        source: FitError::Arithmetic {
            stage: ArithmeticStage::Response,
            operation,
            issue,
        },
    }
}

/// Check fixed output bounds for a closed public error without public operations.
///
/// The error type retains no raw scalar or report. Formatting and assertions are
/// the only effects; failed bounds panic as a test observation, never recovery.
/// # Panics
/// Fails if Display or Debug exceeds the declared fixed character envelope.
fn assert_bounded(error: FixtureProjectionError) {
    assert!(format!("{error}").len() < 256);
    assert!(format!("{error:?}").len() < 512);
}

/// Challenge raw multiplication intermediates with fixed finite operands only.
///
/// Six calls construct no checked value; each must preserve its actual shared
/// multiply failure before any later operation could hide it.
/// # Panics
/// Fails if subnormal, nonzero-to-zero or overflow rejection disappears.
fn assert_multiply_failures() {
    for step in [
        FixtureProjectionStep::RateShift,
        FixtureProjectionStep::Growth,
    ] {
        for (left, right, issue) in [
            (f64::MIN_POSITIVE, 0.5, ArithmeticIssue::Subnormal),
            (
                f64::MIN_POSITIVE,
                f64::MIN_POSITIVE,
                ArithmeticIssue::ZeroUnderflow,
            ),
            (f64::MAX, 2.0, ArithmeticIssue::NonFinite),
        ] {
            assert_eq!(
                projection_arithmetic(left, right, step),
                Err(expected_arithmetic_error(
                    step,
                    ArithmeticOperation::Multiply,
                    issue,
                ))
            );
        }
    }
}

/// Raw predicates and their garde adapter accept both zero signs and legal neighbors.
/// # Errors
/// Propagates the required zero-public-work record failure.
/// # Panics
/// Fails if issue precedence, zero policy or a finite endpoint differs.
#[test]
fn raw_predicate_and_garde_share_finite_normal_nonnegative_or_signed_zero_domain() -> io::Result<()>
{
    profile_counts(GuardCase::Predicate)?;
    for (value, expected) in [
        (f64::NAN, Some(FixtureProjectionIssue::NonFinite)),
        (f64::INFINITY, Some(FixtureProjectionIssue::NonFinite)),
        (f64::NEG_INFINITY, Some(FixtureProjectionIssue::NonFinite)),
        (f64::from_bits(1), Some(FixtureProjectionIssue::Subnormal)),
        (-f64::from_bits(1), Some(FixtureProjectionIssue::Subnormal)),
        (-f64::MIN_POSITIVE, Some(FixtureProjectionIssue::Negative)),
        (-f64::MAX, Some(FixtureProjectionIssue::Negative)),
        (-0.0, None),
        (0.0, None),
        (f64::MIN_POSITIVE, None),
        (1.0, None),
        (f64::MAX, None),
    ] {
        assert_eq!(term_issue(value), expected);
        assert_eq!(term_rule(&value, &()).is_ok(), expected.is_none());
    }
    Ok(())
}

/// Each raw candidate field has real mandatory garde coverage beside healthy controls.
/// # Errors
/// Propagates the required zero-public-work record failure.
/// # Panics
/// Fails on an unvalidated field, wrong first-field order or rejected legal input.
#[test]
fn single_bad_raw_candidates_cover_all_three_rules_and_declared_first_error_order() -> io::Result<()>
{
    profile_counts(GuardCase::Candidate)?;
    for field in [
        FixtureProjectionField::HalfDelaySeconds,
        FixtureProjectionField::DispersionSeconds,
        FixtureProjectionField::SkewSecondsPerSecond,
    ] {
        for (value, issue) in [
            (f64::NAN, FixtureProjectionIssue::NonFinite),
            (f64::INFINITY, FixtureProjectionIssue::NonFinite),
            (f64::NEG_INFINITY, FixtureProjectionIssue::NonFinite),
            (f64::from_bits(1), FixtureProjectionIssue::Subnormal),
            (-f64::from_bits(1), FixtureProjectionIssue::Subnormal),
            (-1.0, FixtureProjectionIssue::Negative),
        ] {
            let candidate = single_bad_candidate(field, value);
            assert!(candidate.validate().is_err());
            assert_eq!(
                candidate.first_error(),
                Some(FixtureProjectionError::InvalidTerm { field, issue })
            );
        }
    }
    for value in [-0.0, 0.0, f64::MIN_POSITIVE, 1.0, f64::MAX] {
        let candidate = ProjectionCandidate {
            half_delay_seconds: value,
            dispersion_seconds: value,
            skew_seconds_per_second: value,
        };
        assert!(candidate.validate().is_ok());
        assert_eq!(candidate.first_error(), None);
    }
    let candidate = ProjectionCandidate {
        half_delay_seconds: -1.0,
        dispersion_seconds: f64::NAN,
        skew_seconds_per_second: f64::INFINITY,
    };
    assert!(candidate.validate().is_err());
    assert_eq!(
        candidate.first_error(),
        Some(FixtureProjectionError::InvalidTerm {
            field: FixtureProjectionField::HalfDelaySeconds,
            issue: FixtureProjectionIssue::Negative,
        })
    );
    Ok(())
}

/// Raw step guards preserve actual typed causes and separately rounded zero semantics.
/// # Errors
/// Propagates the required zero-public-work record failure.
/// # Panics
/// Fails if a primitive map, first unsupported intermediate or legal neighbor changes.
#[test]
fn raw_arithmetic_steps_reject_unsupported_values_before_hidden_intermediates() -> io::Result<()> {
    profile_counts(GuardCase::Arithmetic)?;
    for (step, operation, healthy, negative_zero_times_one, negative_zeros, opposites) in [
        (
            FixtureProjectionStep::RateShift,
            ArithmeticOperation::Multiply,
            8.0_f64,
            -0.0_f64,
            0.0_f64,
            -1.0_f64,
        ),
        (
            FixtureProjectionStep::Center,
            ArithmeticOperation::Add,
            6.0,
            1.0,
            -0.0,
            0.0,
        ),
        (
            FixtureProjectionStep::BaseDistance,
            ArithmeticOperation::Add,
            6.0,
            1.0,
            -0.0,
            0.0,
        ),
        (
            FixtureProjectionStep::Growth,
            ArithmeticOperation::Multiply,
            8.0,
            -0.0,
            0.0,
            -1.0,
        ),
        (
            FixtureProjectionStep::Distance,
            ArithmeticOperation::Add,
            6.0,
            1.0,
            -0.0,
            0.0,
        ),
    ] {
        assert_eq!(step.operation(), operation);
        assert_eq!(
            projection_arithmetic(2.0, 4.0, step).map(f64::to_bits),
            Ok(healthy.to_bits())
        );
        for (left, right, issue) in [
            (f64::NAN, 1.0, ArithmeticIssue::NonFinite),
            (1.0, f64::INFINITY, ArithmeticIssue::NonFinite),
            (f64::NEG_INFINITY, 1.0, ArithmeticIssue::NonFinite),
            (f64::from_bits(1), 1.0, ArithmeticIssue::Subnormal),
            (1.0, -f64::from_bits(1), ArithmeticIssue::Subnormal),
            (f64::MAX, f64::MAX, ArithmeticIssue::NonFinite),
        ] {
            assert_eq!(
                projection_arithmetic(left, right, step),
                Err(expected_arithmetic_error(step, operation, issue))
            );
        }
        for (left, right, expected) in [
            (-0.0, 1.0, negative_zero_times_one),
            (-0.0, -0.0, negative_zeros),
            (1.0, -1.0, opposites),
        ] {
            assert_eq!(
                projection_arithmetic(left, right, step).map(f64::to_bits),
                Ok(expected.to_bits())
            );
        }
    }
    assert_multiply_failures();
    Ok(())
}

/// Closed public error formatting and retained source chains are total and bounded.
/// # Errors
/// Propagates the required zero-public-work record failure.
/// # Panics
/// Fails if a fixed cause disappears, formatting grows unbounded or assumes a producer.
#[test]
fn closed_errors_preserve_bounded_formatting_and_actual_typed_sources() -> io::Result<()> {
    profile_counts(GuardCase::Errors)?;
    for error in [
        FixtureProjectionError::ValidationInvariant,
        FixtureProjectionError::ContextMismatch,
        FixtureProjectionError::MissingReport,
        FixtureProjectionError::MissingAnchor,
        FixtureProjectionError::AnchorMismatch,
        FixtureProjectionError::BackwardProjection,
    ] {
        assert!(error.source().is_none());
        assert_bounded(error);
    }
    for field in [
        FixtureProjectionField::HalfDelaySeconds,
        FixtureProjectionField::DispersionSeconds,
        FixtureProjectionField::SkewSecondsPerSecond,
    ] {
        for issue in [
            FixtureProjectionIssue::NonFinite,
            FixtureProjectionIssue::Subnormal,
            FixtureProjectionIssue::Negative,
        ] {
            let error = FixtureProjectionError::InvalidTerm { field, issue };
            assert!(error.source().is_none());
            assert_bounded(error);
        }
    }
    for source in [
        DomainError::CoordinateOutOfRange,
        DomainError::InvalidEraWindow,
        DomainError::NoEraCandidate,
        DomainError::AmbiguousEra,
        DomainError::ArithmeticOverflow,
        DomainError::DurationOutOfOrder,
        DomainError::NanosecondsOutOfRange,
        DomainError::ResidualOutOfRange,
    ] {
        let error = FixtureProjectionError::Domain { source };
        assert_eq!(
            error
                .source()
                .and_then(|cause| cause.downcast_ref::<DomainError>()),
            Some(&source)
        );
        assert_bounded(error);
    }
    for source in [
        NormalizationError::IntegerRange {
            component: NormalizationComponent::Time,
        },
        NormalizationError::ConversionInvariant {
            component: NormalizationComponent::Phase,
        },
        NormalizationError::ContextMismatch { index: usize::MAX },
        NormalizationError::Input(FitError::CountOutOfRange),
    ] {
        let error = FixtureProjectionError::Conversion { source };
        assert_eq!(
            error
                .source()
                .and_then(|cause| cause.downcast_ref::<NormalizationError>()),
            Some(&source)
        );
        assert_bounded(error);
    }
    for step in [
        FixtureProjectionStep::RateShift,
        FixtureProjectionStep::Center,
        FixtureProjectionStep::BaseDistance,
        FixtureProjectionStep::Growth,
        FixtureProjectionStep::Distance,
    ] {
        for source in [
            FitError::CountOutOfRange,
            FitError::NonIncreasingTime { index: usize::MAX },
            FitError::Point {
                field: PointField::PhaseSeconds,
                issue: PointIssue::NonFinite,
            },
            FitError::Arithmetic {
                stage: ArithmeticStage::Geometry,
                operation: ArithmeticOperation::Divide,
                issue: ArithmeticIssue::ZeroUnderflow,
            },
        ] {
            let error = FixtureProjectionError::Arithmetic { step, source };
            assert_eq!(
                error
                    .source()
                    .and_then(|cause| cause.downcast_ref::<FitError>()),
                Some(&source)
            );
            assert_bounded(error);
        }
    }
    Ok(())
}

//! Raw validator, actual suffix consumer, bounded diagnostic and late-lift controls.
//!
//! External tests own frozen exact-rational and rounded-recipe facts. These tests
//! challenge private admission/output helpers with independent faults and healthy
//! neighbors; injected values establish no public-population numerical claim.

use std::num::NonZeroU64;

use garde::Validate;

use super::{
    ArithmeticContext, ConditionedFitError, ConditionedFitIssue, ConditionedFitOperation,
    ConditionedFitStage, ConditionedNormalizedEstimate, ConditionedPhaseSeconds,
    ConditionedRateRatio, ConditionedSuffixOutcome, ConditionedWeightedSecondsSquared,
    LiftPowerCandidate, LiftRole, StagedEstimate, SuffixCandidate, fit_conditioned_suffix,
    lift_estimate, lifting_factor, prepare_conditioned_suffix, prepare_geometry,
    publish_normalized, retained_point,
};
use crate::conditioned_window::{
    ConditionedBatchOutcome, DeclaredObservationContext, ExactObservationRow,
    prepare_conditioned_batch,
};

use super::arithmetic::{ArithmeticError, Interval, ScalarEnclosure};

/// Keep unexpected typed construction failures distinct from test assertions.
type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Real checked declarations and distinct keys; no source authority is implied.
/// # Errors
/// Preserve unexpected checked-row rejection without fabricating domain values.
fn rows() -> Result<[ExactObservationRow; 3], Box<dyn std::error::Error>> {
    let context = DeclaredObservationContext::new(
        NonZeroU64::MIN,
        NonZeroU64::MIN,
        NonZeroU64::MIN,
        NonZeroU64::MIN,
        NonZeroU64::MIN,
    );
    let key_two = NonZeroU64::new(2).ok_or("nonzero key two")?;
    let key_three = NonZeroU64::new(3).ok_or("nonzero key three")?;
    Ok([
        ExactObservationRow::new(context, NonZeroU64::MIN, 0, false, 0, 1.0)?,
        ExactObservationRow::new(context, key_two, 1_000_000_000, false, 0, 1.0)?,
        ExactObservationRow::new(context, key_three, 2_000_000_000, false, 0, 1.0)?,
    ])
}

/// The mandatory range validator covers both bounds before subtraction/access.
/// # Errors
/// Propagate only unexpected legal control construction failures.
/// # Panics
/// Require raw-validator rejection and the matching actual consumer refusal.
#[test]
fn suffix_range_admission_is_mandatory_at_the_actual_consumer() -> TestResult {
    for count in [3, 4, 64] {
        for start in [0, 1, count - 1, count] {
            assert!(
                SuffixCandidate {
                    range: (start, count)
                }
                .validate()
                .is_ok()
            );
        }
        for start in [count + 1, usize::MAX] {
            assert!(
                SuffixCandidate {
                    range: (start, count)
                }
                .validate()
                .is_err()
            );
        }
    }
    for count in [0, 1, 2, 65, usize::MAX] {
        assert!(SuffixCandidate { range: (0, count) }.validate().is_err());
    }
    let rows = rows()?;
    let ConditionedBatchOutcome::Prepared(parent) =
        prepare_conditioned_batch(rows[0].context(), &rows, 1.0)?
    else {
        return Err("legal parent unexpectedly insufficient".into());
    };
    for start in [4, usize::MAX] {
        assert!(matches!(
            prepare_conditioned_suffix(&parent, start),
            Err(ConditionedFitError::SuffixRange { start: actual, count: 3 }) if actual == start
        ));
    }
    let ConditionedSuffixOutcome::Ready(ready) = prepare_conditioned_suffix(&parent, 0)? else {
        return Err("whole parent unexpectedly insufficient".into());
    };
    assert!(std::ptr::eq(ready.parent(), &raw const parent));
    assert_eq!(ready.original_start(), 0);
    assert_eq!(ready.sample_count(), 3);
    assert!(matches!(
        retained_point(&ready, usize::MAX),
        Err(ConditionedFitError::OutputInvariant)
    ));
    assert!(matches!(
        ArithmeticContext::row(ConditionedFitStage::Geometry, &ready, usize::MAX),
        Err(ConditionedFitError::OutputInvariant)
    ));
    for start in 1..=3 {
        let ConditionedSuffixOutcome::Insufficient(short) =
            prepare_conditioned_suffix(&parent, start)?
        else {
            return Err("short suffix unexpectedly ready".into());
        };
        assert!(std::ptr::eq(short.parent(), &raw const parent));
        assert_eq!(short.original_start(), start);
        assert_eq!(short.sample_count(), 3 - start);
        for local in 0..short.sample_count() {
            let actual = short.original_row(local).ok_or("missing short row")?;
            assert!(std::ptr::eq(actual, &raw const rows[start + local]));
            let point = short.point(local).ok_or("missing short point")?;
            assert!(std::ptr::eq(point.original_row(), actual));
        }
        assert!(short.point(short.sample_count()).is_none());
        assert!(short.original_row(short.sample_count()).is_none());
        assert!(short.point(usize::MAX).is_none());
        assert!(short.original_row(usize::MAX).is_none());
    }
    Ok(())
}

/// Each lift role admits both exact endpoints and refuses adjacent/raw extremes.
/// # Errors
/// Preserve unexpected legal exact-factor construction failures.
/// # Panics
/// Require mandatory validator and direct transformed-output consumer agreement.
#[test]
fn lift_power_validation_is_mandatory_with_exact_endpoint_controls() -> TestResult {
    for (role, minimum, maximum, minimum_bits, maximum_bits) in [
        (
            LiftRole::Phase,
            -62,
            66,
            0x3c10_0000_0000_0000,
            0x4410_0000_0000_0000,
        ),
        (
            LiftRole::Rate,
            -97,
            95,
            0x39e0_0000_0000_0000,
            0x45e0_0000_0000_0000,
        ),
        (
            LiftRole::Sse,
            -124,
            132,
            0x3830_0000_0000_0000,
            0x4830_0000_0000_0000,
        ),
    ] {
        for exponent in [minimum, 0, maximum] {
            assert!(
                LiftPowerCandidate {
                    power: (role, exponent)
                }
                .validate()
                .is_ok()
            );
        }
        for exponent in [minimum - 1, maximum + 1, i32::MIN, i32::MAX] {
            assert!(
                LiftPowerCandidate {
                    power: (role, exponent)
                }
                .validate()
                .is_err()
            );
            assert!(matches!(
                lifting_factor(role, exponent),
                Err(ConditionedFitError::OutputInvariant)
            ));
        }
        assert_eq!(lifting_factor(role, minimum)?.to_bits(), minimum_bits);
        assert_eq!(lifting_factor(role, maximum)?.to_bits(), maximum_bits);
        assert_eq!(lifting_factor(role, 0)?.to_bits(), 1.0_f64.to_bits());
    }
    Ok(())
}

/// Private arithmetic causes leave only permitted stage/index/operation fields.
/// # Errors
/// Propagate an unexpected failure of the legal result-mapping neighbor.
/// # Panics
/// Require every typed cause and diagnostic redaction to remain discriminating.
#[test]
fn diagnostic_projection_preserves_all_causes_without_numeric_payloads() -> TestResult {
    let context = ArithmeticContext {
        stage: ConditionedFitStage::ResidualSum,
        index: Some(63),
    };
    for (cause, issue) in [
        (ArithmeticError::NonFinite, ConditionedFitIssue::NonFinite),
        (
            ArithmeticError::NonPositiveDenominator,
            ConditionedFitIssue::NonPositiveDenominator,
        ),
        (
            ArithmeticError::PointUnderflow,
            ConditionedFitIssue::PointUnderflow,
        ),
        (
            ArithmeticError::InvalidInterval,
            ConditionedFitIssue::InvalidInterval,
        ),
        (ArithmeticError::ErrorRange, ConditionedFitIssue::ErrorRange),
    ] {
        let error = context.apply::<()>(ConditionedFitOperation::Multiply, Err(cause));
        assert_eq!(
            error,
            Err(ConditionedFitError::Arithmetic {
                stage: ConditionedFitStage::ResidualSum,
                index: Some(63),
                operation: ConditionedFitOperation::Multiply,
                issue,
            })
        );
    }
    assert_eq!(context.apply(ConditionedFitOperation::Add, Ok(7))?, 7);
    for error in [
        ConditionedFitError::SuffixRange {
            start: usize::MAX,
            count: 64,
        },
        ConditionedFitError::UnresolvedGeometry,
        ConditionedFitError::OutputInvariant,
        ConditionedFitError::ValidationInvariant,
    ] {
        assert_ne!(error.to_string(), "");
        assert_ne!(format!("{error:?}"), "");
    }
    let error = ConditionedFitError::Arithmetic {
        stage: ConditionedFitStage::Geometry,
        index: Some(4),
        operation: ConditionedFitOperation::Multiply,
        issue: ConditionedFitIssue::PointUnderflow,
    };
    assert_eq!(
        error.to_string(),
        "conditioned Geometry Multiply failed at Some(4): PointUnderflow"
    );
    Ok(())
}

/// Late point or enclosure failures occur at the actual lifting/output consumers.
/// # Errors
/// Propagate unexpected legal normalized publication and exact-factor failures.
/// # Panics
/// Require stage-specific rejection with no successful partial lifted result.
#[test]
fn late_lifting_refusals_follow_normalized_success_without_partial_publication() -> TestResult {
    let tiny = StagedEstimate {
        point: f64::from_bits(1),
        target: Interval::exact(f64::from_bits(1))?,
    };
    let normalized = publish_normalized(tiny)?;
    assert_eq!(normalized.value().to_bits(), 1);
    assert_eq!(normalized.absolute_error().to_bits(), 0);
    assert!(matches!(
        lift_estimate(tiny, LiftRole::Phase, -62),
        Err(ConditionedFitError::Arithmetic {
            stage: ConditionedFitStage::PhaseLifting,
            index: None,
            operation: ConditionedFitOperation::Multiply,
            issue: ConditionedFitIssue::PointUnderflow,
        })
    ));
    let broad = StagedEstimate {
        point: 0.0,
        target: Interval::exact(f64::from_bits(0x7fcf_ffff_ffff_ffff))?,
    };
    let normalized_broad = publish_normalized(broad)?;
    assert_eq!(normalized_broad.value().to_bits(), 0);
    assert!(normalized_broad.absolute_error().is_finite());
    assert!(matches!(
        lift_estimate(broad, LiftRole::Sse, 132),
        Err(ConditionedFitError::Arithmetic {
            stage: ConditionedFitStage::SseLifting,
            index: None,
            operation: ConditionedFitOperation::Multiply,
            issue: ConditionedFitIssue::ErrorRange,
        })
    ));
    let exact = StagedEstimate {
        point: 0.5,
        target: Interval::exact(0.5)?,
    };
    let lifted = lift_estimate(exact, LiftRole::Rate, -1)?;
    assert_eq!(lifted.value().to_bits(), 0.25_f64.to_bits());
    assert!(lifted.lower() <= 0.25 && lifted.upper() >= 0.25);
    Ok(())
}

/// All unit wrappers preserve the checked scalar's distinct fields and redaction.
/// # Errors
/// Preserve unexpected private scalar producer refusal of the legal neighbor.
/// # Panics
/// Require every getter's field identity and fixed Debug text.
#[test]
fn unit_wrappers_preserve_checked_fields_and_redact_debug() -> TestResult {
    let enclosed = ScalarEnclosure::new(1.5, Interval::exact(2.0)?)?;
    let normalized = ConditionedNormalizedEstimate { enclosed };
    let phase = ConditionedPhaseSeconds { enclosed };
    let rate = ConditionedRateRatio { enclosed };
    let sse = ConditionedWeightedSecondsSquared { enclosed };
    for (value, lower, upper, error) in [
        (
            normalized.value(),
            normalized.lower(),
            normalized.upper(),
            normalized.absolute_error(),
        ),
        (
            phase.value(),
            phase.lower(),
            phase.upper(),
            phase.absolute_error(),
        ),
        (
            rate.value(),
            rate.lower(),
            rate.upper(),
            rate.absolute_error(),
        ),
        (sse.value(), sse.lower(), sse.upper(), sse.absolute_error()),
    ] {
        assert_eq!(value.to_bits(), 1.5_f64.to_bits());
        assert_eq!(lower.to_bits(), 1.5_f64.to_bits());
        assert_eq!(upper.to_bits(), 2.0_f64.to_bits());
        assert_eq!(error.to_bits(), enclosed.absolute_error().to_bits());
        assert!(error >= 0.5);
    }
    assert_eq!(
        format!("{normalized:?}"),
        "ConditionedNormalizedEstimate { .. }"
    );
    assert_eq!(format!("{phase:?}"), "ConditionedPhaseSeconds { .. }");
    assert_eq!(format!("{rate:?}"), "ConditionedRateRatio { .. }");
    assert_eq!(
        format!("{sse:?}"),
        "ConditionedWeightedSecondsSquared { .. }"
    );
    Ok(())
}

/// Real constant rows reach all five passes, borrowed anchors and six outputs.
/// # Errors
/// Preserve unexpected checked-public-input or fit rejection.
/// # Panics
/// Require exact constant identities, actual object borrows and redacted displays.
#[test]
fn real_constant_fit_retains_actual_borrows_and_exact_zero_results() -> TestResult {
    let rows = rows()?;
    let ConditionedBatchOutcome::Prepared(parent) =
        prepare_conditioned_batch(rows[0].context(), &rows, 1.0)?
    else {
        return Err("legal parent unexpectedly insufficient".into());
    };
    let ConditionedSuffixOutcome::Ready(suffix) = prepare_conditioned_suffix(&parent, 0)? else {
        return Err("whole parent unexpectedly insufficient".into());
    };
    let prepared = prepare_geometry(&suffix)?;
    assert!(prepared.geometry.point > 0.0);
    assert!(prepared.geometry.target.lower() > 0.0);
    let fit = fit_conditioned_suffix(&suffix)?;
    assert!(std::ptr::eq(fit.suffix(), &raw const suffix));
    assert!(std::ptr::eq(fit.newest_row(), &raw const rows[2]));
    assert_eq!(fit.exact_phase_anchor_numerator(), 0);
    assert_eq!(
        fit.exact_phase_anchor_denominator(),
        8_589_934_592_000_000_000
    );
    for (point, lower, upper, error) in [
        (
            fit.normalized_intercept().value(),
            fit.normalized_intercept().lower(),
            fit.normalized_intercept().upper(),
            fit.normalized_intercept().absolute_error(),
        ),
        (
            fit.normalized_rate().value(),
            fit.normalized_rate().lower(),
            fit.normalized_rate().upper(),
            fit.normalized_rate().absolute_error(),
        ),
        (
            fit.normalized_weighted_sse().value(),
            fit.normalized_weighted_sse().lower(),
            fit.normalized_weighted_sse().upper(),
            fit.normalized_weighted_sse().absolute_error(),
        ),
        (
            fit.phase_correction_seconds().value(),
            fit.phase_correction_seconds().lower(),
            fit.phase_correction_seconds().upper(),
            fit.phase_correction_seconds().absolute_error(),
        ),
        (
            fit.frequency_ratio().value(),
            fit.frequency_ratio().lower(),
            fit.frequency_ratio().upper(),
            fit.frequency_ratio().absolute_error(),
        ),
        (
            fit.weighted_seconds_squared().value(),
            fit.weighted_seconds_squared().lower(),
            fit.weighted_seconds_squared().upper(),
            fit.weighted_seconds_squared().absolute_error(),
        ),
    ] {
        assert_eq!(point.to_bits(), 0);
        assert_eq!(lower.to_bits(), 0);
        assert_eq!(upper.to_bits(), 0);
        assert_eq!(error.to_bits(), 0);
    }
    assert_eq!(format!("{suffix:?}"), "ConditionedSuffix { .. }");
    assert_eq!(format!("{fit:?}"), "ConditionedFit { .. }");
    let short = prepare_conditioned_suffix(&parent, 3)?;
    assert_eq!(
        format!("{short:?}"),
        "Insufficient(InsufficientConditionedSuffix { .. })"
    );
    Ok(())
}

/// The diagnostic consumer rejects the first absent local suffix index.
/// # Errors
/// Preserve unexpected legal row, parent, suffix or last-row refusal.
/// # Panics
/// Require the last valid index and first invalid index to remain distinct.
#[test]
fn local_index_context_rejects_suffix_count_with_last_valid_neighbor() -> TestResult {
    let rows = rows()?;
    let ConditionedBatchOutcome::Prepared(parent) =
        prepare_conditioned_batch(rows[0].context(), &rows, 1.0)?
    else {
        return Err("legal parent unexpectedly insufficient".into());
    };
    let ConditionedSuffixOutcome::Ready(ready) = prepare_conditioned_suffix(&parent, 0)? else {
        return Err("whole parent unexpectedly insufficient".into());
    };
    assert_eq!(ready.sample_count(), 3);
    let last = ArithmeticContext::row(ConditionedFitStage::Geometry, &ready, 2)?;
    assert_eq!(last.stage, ConditionedFitStage::Geometry);
    assert_eq!(last.index, Some(2));
    for local in [ready.sample_count(), usize::MAX] {
        assert!(matches!(
            ArithmeticContext::row(ConditionedFitStage::Geometry, &ready, local),
            Err(ConditionedFitError::OutputInvariant)
        ));
    }
    Ok(())
}

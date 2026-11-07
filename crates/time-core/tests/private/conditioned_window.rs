//! Raw mandatory-output, integer-boundary and bit-exact scaling controls.
//!
//! External tests own independent whole-window numerical facts. These controls
//! challenge private output admission with fabricated faults and legal neighbors;
//! they do not substitute injected values for public prepared populations.

use std::num::NonZeroU64;

use garde::Validate;
use time_domain::{RationalSign, RoundedRationalSeconds};

use super::{
    ConditionedBatchOutcome, ConditionedWindowError, CountCandidate, DeclaredObservationContext,
    ExactObservationRow, MembershipCandidate, PHASE_DENOMINATOR, PointCandidate,
    PrecisionCandidate, REFERENCE_DENOMINATOR, ScaleCandidate, admit_scales, ceiling_exponent,
    denominator, exact_scale, membership_error, prepare_conditioned_batch, publish_point,
    reference_ticks,
};

/// Preserve unexpected typed failures separately from discriminating assertions.
type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Typed nonzero declarations deliberately establish no real source or authority.
fn context() -> DeclaredObservationContext {
    let tag = NonZeroU64::MIN;
    DeclaredObservationContext::new(tag, tag, tag, tag, tag)
}

/// Real checked unit-distance row for private rules, without raw domain fabrication.
/// # Errors
/// Preserve unexpected refusal of the legal exact row.
fn row() -> Result<ExactObservationRow, ConditionedWindowError> {
    ExactObservationRow::new(context(), NonZeroU64::MIN, 0, false, 0, 1.0)
}

/// Legal raw transformed scalars, independent of the algorithm selecting them.
fn healthy_candidate() -> PointCandidate {
    PointCandidate {
        reference: -1.0,
        phase: 1.0,
        reference_roundoff: 0.0,
        phase_roundoff: 0.0,
        divisor: 1.0,
    }
}

/// Public construction obligatorily admits combined reference and quality fields.
/// # Errors
/// Propagate unexpected failure of valid full-domain phase/reference neighbors.
/// # Panics
/// Require fixed ordered errors and bit-preserving positive-subnormal neighbors.
#[test]
fn raw_row_rules_reject_with_full_domain_legal_neighbors() -> TestResult {
    for distance in [
        0.0,
        -0.0,
        -1.0,
        f64::from_bits(0x8000_0000_0000_0001),
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        assert!(matches!(
            ExactObservationRow::new(
                context(),
                NonZeroU64::MIN,
                u64::MAX,
                true,
                i128::MAX,
                distance
            ),
            Err(ConditionedWindowError::InvalidReference)
        ));
        assert!(matches!(
            ExactObservationRow::new(
                context(),
                NonZeroU64::MIN,
                u64::MAX,
                false,
                i128::MIN,
                distance
            ),
            Err(ConditionedWindowError::InvalidPeerDistance)
        ));
    }
    for (whole, half) in [
        (0, false),
        (0, true),
        (u64::MAX, false),
        (u64::MAX - 1, true),
    ] {
        for phase in [i128::MIN, -1, 0, 1, i128::MAX] {
            let value = ExactObservationRow::new(
                context(),
                NonZeroU64::MIN,
                whole,
                half,
                phase,
                f64::from_bits(1),
            )?;
            assert_eq!(value.reference_whole_ns(), whole);
            assert_eq!(value.reference_half(), half);
            assert_eq!(value.corrected_phase_numerator(), phase);
            assert_eq!(value.peer_distance_seconds().to_bits(), 1);
            assert_eq!(
                reference_ticks(&value),
                u128::from(whole) * 2 + u128::from(half)
            );
        }
    }
    Ok(())
}

/// Direct raw validator faults exercise the actual rule callbacks and legal neighbors.
/// # Errors
/// Preserve unexpected checked-row construction failure.
/// # Panics
/// Require count-before-traversal, positive-subnormal precision and bounded membership.
#[test]
fn raw_count_precision_and_membership_rules_are_mandatory() -> TestResult {
    for count in [0, 1, 2, 3, 64] {
        assert!(CountCandidate { count }.validate().is_ok());
    }
    for count in [65, usize::MAX] {
        assert!(CountCandidate { count }.validate().is_err());
    }
    for seconds in [0.0, -0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(PrecisionCandidate { seconds }.validate().is_err());
    }
    for seconds in [f64::from_bits(1), f64::MIN_POSITIVE, 1.0, f64::MAX] {
        assert!(PrecisionCandidate { seconds }.validate().is_ok());
    }
    let value = row()?;
    let excessive = [value; 65];
    assert_eq!(
        membership_error(&excessive, context()),
        Some(ConditionedWindowError::CountOutOfRange { count: 65 })
    );
    assert!(
        MembershipCandidate { rows: &[] }
            .validate_with(&context())
            .is_ok()
    );
    assert!(
        MembershipCandidate { rows: &[value] }
            .validate_with(&context())
            .is_ok()
    );
    assert!(
        MembershipCandidate {
            rows: &[value, value]
        }
        .validate_with(&context())
        .is_err()
    );
    Ok(())
}

/// The exact quotient contract determines powers and adjacent numerator cutoffs.
/// # Errors
/// Propagate impossible denominator or checked integer failures.
/// # Panics
/// Require predeclared exact exponents, including the complete unsigned envelope.
#[test]
fn integer_ceiling_exponent_endpoints_and_exact_neighbors() -> TestResult {
    for (magnitude, denominator, expected) in [
        (0, 1, 0),
        (1, 1, 0),
        (2, 1, 1),
        (3, 1, 2),
        (4, 1, 2),
        (1, 2, -1),
        (1, 3, -1),
        (1, 4, -2),
        (3, 8, -1),
        (4, 8, -1),
        (5, 8, 0),
        (7, 8, 0),
        (8, 8, 0),
        (9, 8, 1),
        (1, u64::MAX, -63),
        (u128::MAX, 1, 128),
    ] {
        let denominator = NonZeroU64::new(denominator).ok_or("nonzero denominator")?;
        assert_eq!(ceiling_exponent(magnitude, denominator)?, expected);
    }
    assert_eq!(
        ceiling_exponent(2, denominator(REFERENCE_DENOMINATOR)?)?,
        -29
    );
    assert_eq!(
        ceiling_exponent(
            2 * u128::from(u64::MAX),
            denominator(REFERENCE_DENOMINATOR)?
        )?,
        35
    );
    assert_eq!(ceiling_exponent(1, denominator(PHASE_DENOMINATOR)?)?, -62);
    assert_eq!(
        ceiling_exponent(u128::MAX, denominator(PHASE_DENOMINATOR)?)?,
        66
    );
    let phase_denominator = u128::from(PHASE_DENOMINATOR);
    assert_eq!(
        ceiling_exponent(phase_denominator - 1, denominator(PHASE_DENOMINATOR)?)?,
        0
    );
    assert_eq!(
        ceiling_exponent(phase_denominator, denominator(PHASE_DENOMINATOR)?)?,
        0
    );
    assert_eq!(
        ceiling_exponent(phase_denominator + 1, denominator(PHASE_DENOMINATOR)?)?,
        1
    );
    assert_eq!(
        denominator(0),
        Err(ConditionedWindowError::ArithmeticInvariant)
    );
    Ok(())
}

/// The actual production scale guard rejects both one-beyond boundaries.
/// # Panics
/// Require the frozen exact-domain envelopes and all-zero phase's exponent zero.
#[test]
fn scale_output_rules_enforce_the_frozen_integer_envelope() {
    for time in [-29, 0, 35] {
        for phase in [-62, 0, 66] {
            assert!(admit_scales(ScaleCandidate { time, phase }).is_ok());
        }
    }
    for time in [i32::MIN, -30, 36, i32::MAX] {
        assert!(matches!(
            admit_scales(ScaleCandidate { time, phase: 0 }),
            Err(ConditionedWindowError::OutputInvariant)
        ));
    }
    for phase in [i32::MIN, -63, 67, i32::MAX] {
        assert!(matches!(
            admit_scales(ScaleCandidate { time: 0, phase }),
            Err(ConditionedWindowError::OutputInvariant)
        ));
    }
}

/// Binary exponent adjustment preserves every sign/significand bit and canonical zero.
/// # Errors
/// Propagate rejection of legal exactly representable normal products.
/// # Panics
/// Require literal binary64 outputs, including exponent-envelope endpoints.
#[test]
fn exact_binary_scaling_preserves_literal_bits() -> TestResult {
    for (value, exponent, expected) in [
        (0_u64, -66, 0_u64),
        (0, 62, 0),
        (0x3ff0_0000_0000_0000, -66, 0x3bd0_0000_0000_0000),
        (0xbff0_0000_0000_0000, 62, 0xc3d0_0000_0000_0000),
        (0x3ff0_0000_0000_0001, 1, 0x4000_0000_0000_0001),
        (0xbfef_ffff_ffff_ffff, -1, 0xbfdf_ffff_ffff_ffff),
        (0x0010_0000_0000_0000, 0, 0x0010_0000_0000_0000),
        (0x7fef_ffff_ffff_ffff, 0, 0x7fef_ffff_ffff_ffff),
    ] {
        assert_eq!(
            exact_scale(f64::from_bits(value), exponent)?.to_bits(),
            expected
        );
    }
    Ok(())
}

/// Raw unsupported inputs and normal-to-subnormal/overflow products cannot silently pass.
/// # Panics
/// Require fixed rejection beside the preceding legal normal controls.
#[test]
fn exact_binary_scaling_refuses_loss_and_invalid_inputs() {
    for (value, exponent) in [
        (-0.0, 0),
        (f64::from_bits(1), 0),
        (f64::NAN, 0),
        (f64::INFINITY, 0),
        (f64::NEG_INFINITY, 0),
        (f64::MIN_POSITIVE, -1),
        (f64::MAX, 1),
        (1.0, i32::MIN),
        (1.0, i32::MAX),
        (0.0, -1023),
        (0.0, 1024),
    ] {
        assert_eq!(
            exact_scale(value, exponent),
            Err(ConditionedWindowError::OutputInvariant)
        );
    }
}

/// The actual private publication boundary obligatorily validates both coordinates.
/// # Errors
/// Propagate genuine checked-row/delta construction and the healthy publication.
/// # Panics
/// Require malformed reference/phase rejection and unchanged coordinate neighbors.
#[test]
fn transformed_coordinate_faults_cannot_publish_without_validation() -> TestResult {
    let row = row()?;
    let reference = RoundedRationalSeconds::new(RationalSign::Negative, 1, NonZeroU64::MIN)?;
    let phase = RoundedRationalSeconds::new(RationalSign::Positive, 1, NonZeroU64::MIN)?;
    let healthy = publish_point(&row, reference, phase, healthy_candidate())?;
    assert!(std::ptr::eq(
        std::ptr::from_ref(healthy.original_row()),
        std::ptr::from_ref(&row)
    ));
    assert_eq!(
        healthy.normalized_reference().to_bits(),
        (-1.0_f64).to_bits()
    );
    assert_eq!(healthy.normalized_phase().to_bits(), 1.0_f64.to_bits());
    for value in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        -0.0,
        f64::from_bits(1),
        -1.000_000_000_000_000_2,
        0.5,
    ] {
        assert!(matches!(
            publish_point(
                &row,
                reference,
                phase,
                PointCandidate {
                    reference: value,
                    ..healthy_candidate()
                }
            ),
            Err(ConditionedWindowError::OutputInvariant)
        ));
    }
    for value in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        -0.0,
        f64::from_bits(1),
        -1.000_000_000_000_000_2,
        1.000_000_000_000_000_2,
    ] {
        assert!(matches!(
            publish_point(
                &row,
                reference,
                phase,
                PointCandidate {
                    phase: value,
                    ..healthy_candidate()
                }
            ),
            Err(ConditionedWindowError::OutputInvariant)
        ));
    }
    Ok(())
}

/// The actual private publication boundary obligatorily validates both bounds.
/// # Errors
/// Propagate genuine checked-row/delta construction and healthy bound publication.
/// # Panics
/// Require malformed bound rejection beside positive-normal and zero neighbors.
#[test]
fn transformed_bound_faults_cannot_publish_without_validation() -> TestResult {
    let row = row()?;
    let reference = RoundedRationalSeconds::new(RationalSign::Negative, 1, NonZeroU64::MIN)?;
    let phase = RoundedRationalSeconds::new(RationalSign::Positive, 1, NonZeroU64::MIN)?;
    for value in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        -0.0,
        f64::from_bits(1),
        -1.0,
        0.75,
    ] {
        assert!(matches!(
            publish_point(
                &row,
                reference,
                phase,
                PointCandidate {
                    reference_roundoff: value,
                    ..healthy_candidate()
                }
            ),
            Err(ConditionedWindowError::OutputInvariant)
        ));
        assert!(matches!(
            publish_point(
                &row,
                reference,
                phase,
                PointCandidate {
                    phase_roundoff: value,
                    ..healthy_candidate()
                }
            ),
            Err(ConditionedWindowError::OutputInvariant)
        ));
    }
    let healthy = publish_point(
        &row,
        reference,
        phase,
        PointCandidate {
            reference_roundoff: f64::MIN_POSITIVE,
            phase_roundoff: 0.5,
            ..healthy_candidate()
        },
    )?;
    assert_eq!(
        healthy.normalized_reference_roundoff().to_bits(),
        f64::MIN_POSITIVE.to_bits()
    );
    assert_eq!(
        healthy.normalized_phase_roundoff().to_bits(),
        0.5_f64.to_bits()
    );
    Ok(())
}

/// The actual private publication boundary obligatorily validates uncapped divisors.
/// # Errors
/// Propagate genuine checked-row/delta construction and healthy divisor publication.
/// # Panics
/// Require malformed divisor rejection beside unchanged one/large/MAX neighbors.
#[test]
fn transformed_divisor_faults_cannot_publish_without_validation() -> TestResult {
    let row = row()?;
    let reference = RoundedRationalSeconds::new(RationalSign::Negative, 1, NonZeroU64::MIN)?;
    let phase = RoundedRationalSeconds::new(RationalSign::Positive, 1, NonZeroU64::MIN)?;
    for value in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        -0.0,
        0.0,
        -1.0,
        f64::from_bits(0x3fef_ffff_ffff_ffff),
    ] {
        assert!(matches!(
            publish_point(
                &row,
                reference,
                phase,
                PointCandidate {
                    divisor: value,
                    ..healthy_candidate()
                }
            ),
            Err(ConditionedWindowError::OutputInvariant)
        ));
    }
    for divisor in [1.0, 1024.0, f64::MAX] {
        let healthy = publish_point(
            &row,
            reference,
            phase,
            PointCandidate {
                divisor,
                reference_roundoff: f64::MIN_POSITIVE,
                phase_roundoff: 0.5,
                ..healthy_candidate()
            },
        )?;
        assert_eq!(healthy.variance_divisor().to_bits(), divisor.to_bits());
    }
    Ok(())
}

/// Short populations must bypass conversion/conditioning even with extreme raw phases.
/// # Errors
/// Propagate unexpected row or admitted insufficient-result failures.
/// # Panics
/// Require unchanged extrema/quality and absent indexes at count and usize maximum.
#[test]
fn insufficient_extrema_retain_only_original_quality_metadata() -> TestResult {
    let two = NonZeroU64::new(2).ok_or("key")?;
    let rows = [
        ExactObservationRow::new(context(), NonZeroU64::MIN, 0, true, i128::MIN, 1.0)?,
        ExactObservationRow::new(context(), two, u64::MAX, false, i128::MAX, 1.0)?,
    ];
    for count in 0..=2 {
        let ConditionedBatchOutcome::Insufficient(result) =
            prepare_conditioned_batch(context(), &rows[..count], f64::from_bits(1))?
        else {
            return Err("unexpected prepared short population".into());
        };
        assert_eq!(result.sample_count(), count);
        assert!(std::ptr::eq(
            std::ptr::from_ref(result.original_rows()),
            std::ptr::from_ref(&rows[..count])
        ));
        assert_eq!(result.current_precision_seconds().to_bits(), 1);
        assert_eq!(result.divisors.is_some(), count != 0);
        assert_eq!(result.newest_row().is_some(), count != 0);
        assert!(result.variance_divisor(count).is_none());
        assert!(result.variance_divisor(usize::MAX).is_none());
        for index in 0..count {
            assert_eq!(
                result.variance_divisor(index).map(f64::to_bits),
                Some(1.0_f64.to_bits())
            );
        }
    }
    Ok(())
}

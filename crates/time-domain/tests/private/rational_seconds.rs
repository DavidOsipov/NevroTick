//! Raw internal guard controls, distinct from the external numerical oracle.

use std::num::NonZeroU64;

use super::{
    MagnitudeRounding, RationalSecondsError, RationalSign, RoundedRationalSeconds, binary_power,
    normalized_ratio, original_binade, publish, roundoff_rule, scale_integer, seconds_rule,
};

/// Healthy metadata for the exact positive unit; no arbitrary public value is made.
const EXACT_UNIT: MagnitudeRounding = MagnitudeRounding {
    seconds: 1.0,
    absolute_roundoff_seconds: 0.0,
    original_exponent: 0,
    exact: true,
};

/// Internal seconds faults reach mandatory output admission before publication.
/// # Errors
/// Propagate failure of the genuine legal unit control.
/// # Panics
/// Panic if a malformed transformed value publishes a result or a legal guard fails.
#[test]
fn transformed_seconds_faults_have_valid_scalar_neighbors() -> Result<(), RationalSecondsError> {
    let healthy = RoundedRationalSeconds::new(RationalSign::Positive, 1, NonZeroU64::MIN)?;
    assert_eq!(healthy.seconds().to_bits(), 0x3ff0_0000_0000_0000);
    for seconds in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::from_bits(1),
        f64::from_bits(0x8000_0000_0000_0001),
        -0.0,
        0.0,
        -1.0,
        f64::from_bits(0x3bef_ffff_ffff_ffff),
        f64::from_bits(0x47f0_0000_0000_0001),
    ] {
        let rounded = MagnitudeRounding {
            seconds,
            ..EXACT_UNIT
        };
        assert!(matches!(
            publish(RationalSign::Positive, 1, NonZeroU64::MIN, rounded),
            Err(RationalSecondsError::OutputInvariant)
        ));
    }
    // These calls test raw scalar acceptance, without publishing fabricated
    // exact tuples or substituting for external rational-result controls.
    for seconds in [
        0.0,
        1.0,
        -1.0,
        f64::from_bits(0x3bf0_0000_0000_0000),
        -f64::from_bits(0x3bf0_0000_0000_0000),
        f64::from_bits(0x47f0_0000_0000_0000),
        -f64::from_bits(0x47f0_0000_0000_0000),
    ] {
        assert!(seconds_rule(&seconds, &()).is_ok());
    }
    Ok(())
}

/// Scalar bound faults and wrong derivation associations cannot publish a result.
/// # Errors
/// Propagate failure of the genuine legal zero control.
/// # Panics
/// Panic if a malformed bound escapes mandatory admission or a legal guard fails.
#[test]
fn transformed_bounds_require_power_and_exactness_association() -> Result<(), RationalSecondsError>
{
    let zero = RoundedRationalSeconds::new(RationalSign::Negative, 0, NonZeroU64::MIN)?;
    assert_eq!(zero.absolute_roundoff_seconds().to_bits(), 0);
    for absolute_roundoff_seconds in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::from_bits(1),
        -0.0,
        -1.0,
        f64::from_bits(0x389f_ffff_ffff_ffff),
        f64::from_bits(0x4490_0000_0000_0001),
        0.1,
        f64::from_bits(0x3ca0_0000_0000_0000),
    ] {
        let rounded = MagnitudeRounding {
            absolute_roundoff_seconds,
            ..EXACT_UNIT
        };
        assert!(matches!(
            publish(RationalSign::Positive, 1, NonZeroU64::MIN, rounded),
            Err(RationalSecondsError::OutputInvariant)
        ));
    }
    for bound in [
        0.0,
        f64::from_bits(0x38a0_0000_0000_0000),
        f64::from_bits(0x3ca0_0000_0000_0000),
        f64::from_bits(0x4490_0000_0000_0000),
    ] {
        assert!(roundoff_rule(&bound, &()).is_ok());
    }
    for rounded in [
        MagnitudeRounding {
            exact: false,
            ..EXACT_UNIT
        },
        MagnitudeRounding {
            original_exponent: -65,
            ..EXACT_UNIT
        },
        MagnitudeRounding {
            original_exponent: 128,
            ..EXACT_UNIT
        },
        MagnitudeRounding {
            exact: false,
            absolute_roundoff_seconds: f64::from_bits(0x3c90_0000_0000_0000),
            ..EXACT_UNIT
        },
    ] {
        assert!(matches!(
            publish(RationalSign::Positive, 1, NonZeroU64::MIN, rounded),
            Err(RationalSecondsError::OutputInvariant)
        ));
    }
    Ok(())
}

/// Raw finite normals must satisfy both the binary-power shape and bound range.
/// These direct rule calls cannot be masked by later publication associations.
/// # Panics
/// Panic if a valid power is rejected or an invalid normal passes the raw rule.
#[test]
fn raw_roundoff_rule_rejects_invalid_normals_without_publication_guards() {
    for bits in [
        0_u64,
        0x38a0_0000_0000_0000,
        0x3ca0_0000_0000_0000,
        0x4490_0000_0000_0000,
    ] {
        assert!(
            roundoff_rule(&f64::from_bits(bits), &()).is_ok(),
            "raw roundoff healthy zero or bounded binary power must succeed"
        );
    }

    // The immediate predecessor of the valid minimum 2^-117 is normal,
    // fractional and below the range. Replacing the first && by || admits
    // every normal; replacing only the second && still rejects this input.
    let below_minimum_fraction = f64::from_bits(0x389f_ffff_ffff_ffff);
    assert!(
        roundoff_rule(&below_minimum_fraction, &()).is_err(),
        "raw roundoff first conjunction must reject the fractional minimum neighbor"
    );

    // 2^-118 is an exact positive normal binary power, but lies below 2^-117.
    // Replacing the range conjunction by || wrongly admits this input solely
    // because its normal and power predicates are both true.
    let below_minimum_power = f64::from_bits(0x3890_0000_0000_0000);
    assert!(
        roundoff_rule(&below_minimum_power, &()).is_err(),
        "raw roundoff range conjunction must reject the out-of-range power 2^-118"
    );

    // The immediate successor of 2^-53 remains within the range but is not a
    // binary power. The exact 2^-53 healthy neighbor was admitted above.
    let within_range_fraction = f64::from_bits(0x3ca0_0000_0000_0001);
    assert!(
        roundoff_rule(&within_range_fraction, &()).is_err(),
        "raw roundoff power shape must reject the fractional in-range neighbor"
    );

    // The same range contract excludes the next power above the valid 2^74
    // endpoint, independently of the lower-bound counterexamples.
    let above_maximum_power = f64::from_bits(0x44a0_0000_0000_0000);
    assert!(
        roundoff_rule(&above_maximum_power, &()).is_err(),
        "raw roundoff range must reject the out-of-range power 2^75"
    );
}

/// Zero associations are checked independently of scalar shape admission.
/// # Panics
/// Panic if a noncanonical zero or incorrect derivation metadata is published.
#[test]
fn zero_publication_rejects_noncanonical_associations() {
    let exact_zero = MagnitudeRounding {
        seconds: 0.0,
        ..EXACT_UNIT
    };
    assert!(matches!(
        publish(RationalSign::Negative, 0, NonZeroU64::MIN, exact_zero),
        Err(RationalSecondsError::OutputInvariant)
    ));
    for rounded in [
        EXACT_UNIT,
        MagnitudeRounding {
            exact: false,
            ..exact_zero
        },
        MagnitudeRounding {
            original_exponent: 1,
            ..exact_zero
        },
        MagnitudeRounding {
            absolute_roundoff_seconds: f64::from_bits(0x3ca0_0000_0000_0000),
            ..exact_zero
        },
    ] {
        assert!(matches!(
            publish(RationalSign::Positive, 0, NonZeroU64::MIN, rounded),
            Err(RationalSecondsError::OutputInvariant)
        ));
    }
}

/// Checked helper failures retain genuine legal neighboring operations.
/// # Errors
/// Propagate an unexpected failure in a supported integer or power neighbor.
/// # Panics
/// Panic if an unsupported raw helper input is admitted or a neighbor is changed.
#[test]
fn checked_helper_faults_do_not_use_truncated_shifts() -> Result<(), RationalSecondsError> {
    assert_eq!(scale_integer(1, 127)?, 1_u128 << 127);
    assert_eq!(scale_integer(u128::MAX, 0)?, u128::MAX);
    for (value, shift) in [(1, 128), (1, u32::MAX), (2, 127), (u128::MAX, 1)] {
        assert_eq!(
            scale_integer(value, shift),
            Err(RationalSecondsError::ArithmeticInvariant)
        );
    }
    assert!(matches!(
        original_binade(0, NonZeroU64::MIN),
        Err(RationalSecondsError::ArithmeticInvariant)
    ));
    assert!(matches!(
        normalized_ratio(0, NonZeroU64::MIN),
        Err(RationalSecondsError::ArithmeticInvariant)
    ));
    assert_eq!(binary_power(-1022)?.to_bits(), 0x0010_0000_0000_0000);
    assert_eq!(binary_power(1023)?.to_bits(), 0x7fe0_0000_0000_0000);
    for exponent in [i32::MIN, -1023, 1024, i32::MAX] {
        assert!(matches!(
            binary_power(exponent),
            Err(RationalSecondsError::ArithmeticInvariant)
        ));
    }
    Ok(())
}

/// Public diagnostics retain fixed bounded text without any exact numeric input.
/// # Errors
/// Propagates an unexpected refusal of the legitimate full-magnitude input.
/// # Panics
/// Fails if formatting exposes values or changes either typed failure's message.
#[test]
fn diagnostics_omit_numeric_payloads_and_use_fixed_errors() -> Result<(), RationalSecondsError> {
    let value = RoundedRationalSeconds::new(RationalSign::Negative, u128::MAX, NonZeroU64::MIN)?;
    assert_eq!(format!("{value:?}"), "RoundedRationalSeconds { .. }");
    for (failure, text) in [
        (
            RationalSecondsError::ArithmeticInvariant,
            "rational seconds arithmetic invariant",
        ),
        (
            RationalSecondsError::OutputInvariant,
            "rational seconds output invariant",
        ),
    ] {
        assert_eq!(failure.to_string(), text);
    }
    Ok(())
}

/// Unsigned magnitude admission precedes sign application at the raw owner.
/// # Errors
/// Propagates unexpected refusal of legitimate signed-unit and zero controls.
/// # Panics
/// Fails if requested Negative can hide a malformed negative raw magnitude.
#[test]
fn raw_unsigned_magnitude_is_admitted_before_sign_application() -> Result<(), RationalSecondsError>
{
    let unsigned_unit = MagnitudeRounding {
        seconds: 1.0,
        absolute_roundoff_seconds: 0.0,
        original_exponent: 0,
        exact: true,
    };
    let negative = publish(RationalSign::Negative, 1, NonZeroU64::MIN, unsigned_unit)?;
    assert_eq!(negative.seconds().to_bits(), 0xbff0_0000_0000_0000);
    let zero = MagnitudeRounding {
        seconds: 0.0,
        absolute_roundoff_seconds: 0.0,
        original_exponent: 0,
        exact: true,
    };
    assert_eq!(
        publish(RationalSign::Positive, 0, NonZeroU64::MIN, zero)?
            .seconds()
            .to_bits(),
        0,
    );
    for seconds in [-1.0, f64::from_bits(0x8000_0000_0000_0000)] {
        let raw = MagnitudeRounding {
            seconds,
            ..unsigned_unit
        };
        assert!(
            matches!(
                publish(RationalSign::Negative, 1, NonZeroU64::MIN, raw),
                Err(RationalSecondsError::OutputInvariant)
            ),
            "unsigned magnitude sign must be admitted before applying Negative",
        );
    }
    Ok(())
}

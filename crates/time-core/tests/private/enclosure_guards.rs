// SPDX-License-Identifier: GPL-2.0-only
//! Six raw enclosure guard scopes with zero public producer or enclosure calls.
//!
//! Inputs and exact expected endpoints come from the frozen analytic task facts.
//! No checked history, projection, interval, observation or duration is forged.
//! Only actual raw garde admission, checked private scalar transformations and
//! closed error values are exercised; public reachability belongs to integration.

use std::{
    cmp::Ordering,
    error::Error,
    io::{self, Write},
};

use garde::Validate;

use super::{
    EnclosureOperands, FixtureEnclosureError, FixtureEndpointRounding, Fraction, IntervalError,
    OPERAND_LIMIT, RawEnclosureOperands, ScaledValue, add_fractions, center_admissible,
    center_rule, distance_admissible, distance_rule, enclose_scalars,
};

/// Exact half of a Q33 tick, supplied as an independent binary64 bit pattern.
const HALF_TICK: f64 = f64::from_bits(989_u64 << 52);
/// Exact quarter of a Q33 tick, supplied as an independent binary64 bit pattern.
const QUARTER_TICK: f64 = f64::from_bits(988_u64 << 52);
/// Exact tiny normal distance `2^-80`, independently smaller than one Q33 tick.
const TINY_NORMAL: f64 = f64::from_bits(943_u64 << 52);
/// Largest binary64 value below `2^30`, exactly `2^30 - 2^-23`.
const LAST_CENTER: f64 = f64::from_bits(0x41cf_ffff_ffff_ffff);
/// Exact Q33 integer scale for the task's analytic expected endpoints.
const Q: i64 = 8_589_934_592;

/// Six closed accounting scopes; every scope performs zero public source work.
#[derive(Clone, Copy)]
enum GuardCase {
    /// Raw pure predicates and their mandatory garde callback adapters.
    Rules,
    /// Actual raw candidate validation and private checked operand construction.
    Admission,
    /// Independent exact raw-scalar enclosure facts, including subnormals.
    Facts,
    /// Raw numerical domain and final endpoint-range rejection facts.
    Ranges,
    /// Bounded private fraction constructor, comparison and carry guards.
    Fractions,
    /// Redacted closed error formatting and actual typed cause chaining.
    Errors,
}

/// Emit exactly one closed accounting row before any assertion in a test scope.
///
/// The fixed field order records all public attempt families as zero. The
/// selected private scope owns its separately reviewed finite helper budget.
/// # Errors
/// Propagates stderr failure instead of claiming a complete accounting row.
fn profile_counts(case: GuardCase) -> io::Result<()> {
    let name = match case {
        GuardCase::Rules => "enclosure_private_rules",
        GuardCase::Admission => "enclosure_private_admission",
        GuardCase::Facts => "enclosure_private_facts",
        GuardCase::Ranges => "enclosure_private_ranges",
        GuardCase::Fractions => "enclosure_private_fractions",
        GuardCase::Errors => "enclosure_private_errors",
    };
    writeln!(
        io::stderr().lock(),
        "CORE_ENCLOSURE_PROFILE case={name} histories=0 appends=0 observations=0 contexts=0 monotonic=0 coordinates=0 exchanges=0 getters=0 terms=0 projections=0 enclosures=0"
    )
}

/// Compare one raw helper result with independent exact integers and rounding facts.
///
/// The real private admission/decomposition/completion path runs exactly once;
/// this observer constructs no public checked carrier and changes no state.
/// # Errors
/// Preserves a helper failure instead of manufacturing expected output.
/// # Panics
/// Fails when a bound or exact residual classification differs from the task fact.
fn assert_bounds(
    center: f64,
    distance: f64,
    expected: (i64, i64, FixtureEndpointRounding, FixtureEndpointRounding),
) -> Result<(), FixtureEnclosureError> {
    let bounds = enclose_scalars(center, distance)?;
    assert_eq!(
        (
            bounds.lower,
            bounds.upper,
            bounds.lower_rounding,
            bounds.upper_rounding,
        ),
        expected
    );
    Ok(())
}

/// Every raw rule accepts signed zeros/subnormals and rejects its actual invalid domain.
/// # Errors
/// Propagates the mandatory zero-public-work row's sink failure.
/// # Panics
/// Fails if either garde adapter diverges from its shared raw predicate.
#[test]
fn enclosure_private_rules() -> io::Result<()> {
    profile_counts(GuardCase::Rules)?;
    for (value, center_ok, distance_ok) in [
        (f64::NAN, false, false),
        (f64::INFINITY, false, false),
        (f64::NEG_INFINITY, false, false),
        (-2.0 * OPERAND_LIMIT, false, false),
        (OPERAND_LIMIT, false, false),
        (-OPERAND_LIMIT, true, false),
        (-1.0, true, false),
        (-0.0, true, true),
        (0.0, true, true),
        (f64::from_bits(1), true, true),
        (-f64::from_bits(1), true, false),
        (f64::MIN_POSITIVE, true, true),
        (1.0, true, true),
        (LAST_CENTER, true, true),
    ] {
        assert_eq!(center_admissible(value), center_ok);
        assert_eq!(distance_admissible(value), distance_ok);
        assert_eq!(center_rule(&value, &()).is_ok(), center_ok);
        assert_eq!(distance_rule(&value, &()).is_ok(), distance_ok);
    }
    Ok(())
}

/// Actual mandatory validation rejects each bad field beside five supported neighbors.
/// # Errors
/// Propagates the zero-public-work row or an unexpected legal-neighbor failure.
/// # Panics
/// Fails if a raw field lacks garde coverage or a rejected constructor publishes operands.
#[test]
fn enclosure_private_admission() -> Result<(), Box<dyn Error>> {
    profile_counts(GuardCase::Admission)?;
    for (center, distance) in [
        (f64::NAN, 0.0),
        (f64::INFINITY, 0.0),
        (f64::NEG_INFINITY, 0.0),
        (-OPERAND_LIMIT - 1.0, 0.0),
        (OPERAND_LIMIT, 0.0),
        (f64::MAX, 0.0),
        (0.0, f64::NAN),
        (0.0, f64::INFINITY),
        (0.0, f64::NEG_INFINITY),
        (0.0, -f64::MIN_POSITIVE),
        (0.0, -f64::from_bits(1)),
        (0.0, -1.0),
        (0.0, OPERAND_LIMIT),
    ] {
        let raw = RawEnclosureOperands {
            center_seconds: center,
            distance_seconds: distance,
        };
        assert!(raw.validate().is_err());
        assert!(matches!(
            EnclosureOperands::new(center, distance),
            Err(FixtureEnclosureError::Unrepresentable)
        ));
    }
    for (center, distance) in [
        (-OPERAND_LIMIT, 0.0),
        (-0.0, -0.0),
        (0.0, f64::from_bits(1)),
        (-f64::from_bits(1), f64::MIN_POSITIVE),
        (LAST_CENTER, f64::MIN_POSITIVE),
    ] {
        let raw = RawEnclosureOperands {
            center_seconds: center,
            distance_seconds: distance,
        };
        assert!(raw.validate().is_ok());
        let operands = EnclosureOperands::new(center, distance)?;
        assert_eq!(operands.center.negative, center.is_sign_negative());
        assert_eq!(operands.distance.negative, distance.is_sign_negative());
    }
    Ok(())
}

/// Twenty-three frozen exact real facts discriminate tiny residuals and fractional carries.
/// # Errors
/// Propagates the zero-public-work row or actual raw-helper failure.
/// # Panics
/// Fails on rounded float cancellation, subnormal loss, wrong signs or incorrect exactness.
#[test]
fn enclosure_private_facts() -> Result<(), Box<dyn Error>> {
    profile_counts(GuardCase::Facts)?;
    let exact = FixtureEndpointRounding::Exact;
    let outward = FixtureEndpointRounding::OutwardFraction;
    for (center, distance, expected) in [
        (0.0, 0.0, (0, 0, exact, exact)),
        (-0.0, 0.0, (0, 0, exact, exact)),
        (0.0, -0.0, (0, 0, exact, exact)),
        (-0.0, -0.0, (0, 0, exact, exact)),
        (f64::MIN_POSITIVE, 0.0, (0, 1, outward, outward)),
        (f64::from_bits(1), 0.0, (0, 1, outward, outward)),
        (-f64::MIN_POSITIVE, 0.0, (-1, 0, outward, outward)),
        (-f64::from_bits(1), 0.0, (-1, 0, outward, outward)),
        (1.0, TINY_NORMAL, (Q - 1, Q + 1, outward, outward)),
        (1.0, f64::from_bits(1), (Q - 1, Q + 1, outward, outward)),
        (HALF_TICK, HALF_TICK, (0, 1, exact, exact)),
        (QUARTER_TICK, 3.0 * QUARTER_TICK, (-1, 1, outward, exact)),
        (-QUARTER_TICK, 3.0 * QUARTER_TICK, (-1, 1, exact, outward)),
        (
            3.0 * QUARTER_TICK,
            3.0 * QUARTER_TICK,
            (0, 2, exact, outward),
        ),
        (1.0 + HALF_TICK, HALF_TICK, (Q, Q + 1, exact, exact)),
        (-OPERAND_LIMIT, 0.0, (i64::MIN, i64::MIN, exact, exact)),
        (
            LAST_CENTER,
            1023.0 * f64::from_bits(990_u64 << 52),
            (i64::MAX - 2046, i64::MAX, exact, exact),
        ),
        (
            -OPERAND_LIMIT / 2.0,
            OPERAND_LIMIT / 2.0,
            (i64::MIN, 0, exact, exact),
        ),
        (1.0, 0.0, (Q, Q, exact, exact)),
        (-1.0, 0.0, (-Q, -Q, exact, exact)),
        (0.0, HALF_TICK, (-1, 1, outward, outward)),
        (0.0, f64::from_bits(1), (-1, 1, outward, outward)),
        (QUARTER_TICK, QUARTER_TICK, (0, 1, exact, outward)),
    ] {
        assert_bounds(center, distance, expected)?;
    }
    Ok(())
}

/// Seventeen frozen invalid-domain or final i64-range facts retain no partial result.
/// # Errors
/// Propagates the mandatory zero-public-work accounting row's sink failure.
/// # Panics
/// Fails if prebounds replace exact endpoints or a signed tiny distance is lost.
#[test]
fn enclosure_private_ranges() -> io::Result<()> {
    profile_counts(GuardCase::Ranges)?;
    for (center, distance) in [
        (f64::NAN, 0.0),
        (f64::INFINITY, 0.0),
        (f64::NEG_INFINITY, 0.0),
        (0.0, f64::NAN),
        (0.0, f64::INFINITY),
        (0.0, f64::NEG_INFINITY),
        (0.0, -1.0),
        (0.0, -f64::from_bits(1)),
        (OPERAND_LIMIT, 0.0),
        (-2.0 * OPERAND_LIMIT, 0.0),
        (0.0, OPERAND_LIMIT),
        (-OPERAND_LIMIT, f64::from_bits(1)),
        (-OPERAND_LIMIT, TINY_NORMAL),
        (LAST_CENTER, 2047.0 * HALF_TICK),
        (OPERAND_LIMIT / 2.0, OPERAND_LIMIT / 2.0),
        (LAST_CENTER, 1024.0 * f64::from_bits(990_u64 << 52)),
        (-OPERAND_LIMIT, 1.0),
    ] {
        assert!(matches!(
            enclose_scalars(center, distance),
            Err(FixtureEnclosureError::Unrepresentable)
        ));
    }
    Ok(())
}

/// Raw fraction constructors and exact comparison/carry helpers enforce their own bounds.
/// # Errors
/// Propagates the zero-public-work row or an unexpected legal private-transform failure.
/// # Panics
/// Fails if a private guard, denominator bound, zero case or carry classification changes.
#[test]
fn enclosure_private_fractions() -> Result<(), Box<dyn Error>> {
    profile_counts(GuardCase::Fractions)?;
    for (numerator, shift) in [
        (1, 0),
        (2, 1),
        (1_u64 << 53, 54),
        (u64::MAX, 1041),
        (1, 1042),
        (0, 1042),
    ] {
        assert!(matches!(
            Fraction::new(numerator, shift),
            Err(FixtureEnclosureError::Invariant)
        ));
    }
    for shift in [0, 1041] {
        let zero = Fraction::new(0, shift)?;
        assert_eq!((zero.numerator, zero.shift), (0, 0));
    }
    for (left, right, expected) in [
        ((0, 0), (0, 0), Ordering::Equal),
        ((0, 0), (1, 1041), Ordering::Less),
        ((1, 1041), (0, 0), Ordering::Greater),
        ((1, 1), (2, 2), Ordering::Equal),
        ((2, 2), (1, 1), Ordering::Equal),
        ((1, 1), (1, 2), Ordering::Greater),
        ((1, 1041), (1, 1040), Ordering::Less),
        ((1, 1040), (2, 1041), Ordering::Equal),
        ((3, 1041), (1, 1040), Ordering::Greater),
    ] {
        assert_eq!(
            Fraction::new(left.0, left.1)?.compare(Fraction::new(right.0, right.1)?)?,
            expected
        );
    }
    for (left, right, expected) in [
        ((0, 0), (0, 0), (false, false)),
        ((0, 0), (1, 2), (false, true)),
        ((1, 2), (1, 2), (false, true)),
        ((1, 1), (1, 2), (false, true)),
        ((1, 1), (1, 1), (true, false)),
        ((3, 2), (1, 1), (true, true)),
        ((3, 2), (1, 2), (true, false)),
    ] {
        let sum = add_fractions(
            Fraction::new(left.0, left.1)?,
            Fraction::new(right.0, right.1)?,
        )?;
        assert_eq!((sum.carry, sum.residual), expected);
    }
    let complement = Fraction::new((1_u64 << 53) - 1, 53)?.complement()?;
    assert_eq!((complement.numerator, complement.shift), (1, 53));
    for (numerator, shift) in [(0, 0), (1, 2), (1, 1041)] {
        assert!(matches!(
            Fraction::new(numerator, shift)?.complement(),
            Err(FixtureEnclosureError::Invariant)
        ));
    }
    for (significand, scale) in [(1_u64 << 53, 0), (1, 12), (1, -1042)] {
        assert!(matches!(
            ScaledValue::split(significand, scale),
            Err(FixtureEnclosureError::Invariant)
        ));
    }
    for bits in [f64::NAN.to_bits(), f64::MAX.to_bits(), u64::MAX] {
        assert!(matches!(
            ScaledValue::from_bits(bits),
            Err(FixtureEnclosureError::Invariant)
        ));
    }
    Ok(())
}

/// Closed errors format bounded redacted codes while retaining each actual interval cause.
/// # Errors
/// Propagates the mandatory zero-public-work accounting row's sink failure.
/// # Panics
/// Fails if formatting exposes a retained cause or a source chain loses its typed cause.
#[test]
fn enclosure_private_errors() -> io::Result<()> {
    profile_counts(GuardCase::Errors)?;
    for error in [
        FixtureEnclosureError::Unrepresentable,
        FixtureEnclosureError::Invariant,
    ] {
        assert!(error.source().is_none());
        assert!(format!("{error}").len() < 64);
        assert!(format!("{error:?}").len() < 64);
    }
    for source in [
        IntervalError::ZeroLabel,
        IntervalError::ReversedBounds,
        IntervalError::ValidationInvariant,
    ] {
        let error = FixtureEnclosureError::Interval { source };
        assert_eq!(
            error
                .source()
                .and_then(|cause| cause.downcast_ref::<IntervalError>()),
            Some(&source)
        );
        assert_eq!(
            format!("{error}"),
            "fixture enclosure: interval construction"
        );
        assert_eq!(
            format!("{error:?}"),
            "FixtureEnclosureError::Interval { .. }"
        );
    }
    Ok(())
}

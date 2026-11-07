// SPDX-License-Identifier: GPL-2.0-only
//! Independent exact-reference and raw numerical-boundary controls.

use super::*;
use time_domain::RationalSign::{Negative, Positive};

/// Raw lower/upper/whole/half tuples with independently declared admission facts.
/// Reversed bounds can have the correct doubled sum and still require rejection.
const MIDPOINT_PART_CASES: [([u64; 4], bool); 14] = [
    ([10, 11, 10, 1], true),
    ([u64::MAX, u64::MAX, u64::MAX, 0], true),
    ([0, 0, 0, 0], true),
    ([0, u64::MAX, 9_223_372_036_854_775_807, 1], true),
    ([10, 11, 10, 0], false),
    ([10, 11, 11, 0], false),
    ([11, 10, 10, 1], false),
    ([10, 11, 10, 2], false),
    ([10, 11, 10, u64::MAX], false),
    ([10, 12, 10, 2], false),
    ([0, u64::MAX, 0, u64::MAX], false),
    ([u64::MAX, u64::MAX, u64::MAX, 1], false),
    ([0, 0, 1, 0], false),
    ([0, 0, 0, 1], false),
];

/// Preserve raw scalar input for the actual publisher and validation boundary.
/// This creates no validated reference or transport/capture provenance.
fn raw_midpoint_parts(values: [u64; 4]) -> MidpointParts {
    let [lower, upper, whole, half] = values;
    MidpointParts {
        lower,
        upper,
        whole,
        half,
    }
}

/// Assert an independent exact tuple, sign, binary64 result and arithmetic bound.
/// # Panics
/// Panic if units, retained magnitude, rounding, canonical zero or bound changes.
fn assert_relative_seconds(
    result: &RoundedRationalSeconds,
    sign: RationalSign,
    magnitude: u128,
    denominator: u64,
    seconds_bits: u64,
    bound_bits: u64,
) {
    assert_eq!(result.sign(), sign);
    assert_eq!(result.numerator_magnitude(), magnitude);
    assert_eq!(result.denominator().get(), denominator);
    assert_eq!(result.seconds().to_bits(), seconds_bits);
    assert_eq!(result.absolute_roundoff_seconds().to_bits(), bound_bits);
}

/// Exact half-nanosecond labels remain representable even when a u64 sum would fail.
/// # Errors
/// Propagate an unexpected refusal of an independently valid endpoint pair.
/// # Panics
/// Panic on a changed midpoint, discarded half bit or accepted reversed pair.
#[test]
fn midpoint_bounds_preserve_half_nanoseconds_and_integer_endpoints()
-> Result<(), RelativeMeasurementRefusal> {
    for (lower, upper, whole, half) in [
        (10, 11, 10, true),
        (10, 10, 10, false),
        (0, 0, 0, false),
        (u64::MAX, u64::MAX, u64::MAX, false),
        (u64::MAX - 1, u64::MAX, u64::MAX - 1, true),
        (0, u64::MAX, 9_223_372_036_854_775_807, true),
    ] {
        let reference = reference_from_bounds(lower, upper)?;
        assert_eq!(reference.whole_nanoseconds(), whole);
        assert_eq!(reference.has_half_nanosecond(), half);
        assert_eq!(
            u128::from(reference.whole_nanoseconds()) * 2
                + u128::from(reference.has_half_nanosecond()),
            u128::from(lower) + u128::from(upper)
        );
    }
    assert!(matches!(
        reference_from_bounds(11, 10),
        Err(RelativeMeasurementRefusal::ReferenceOrder)
    ));
    Ok(())
}

/// Challenge the raw rule, Garde field dispatch and actual mandatory publisher.
/// # Errors
/// Propagate rejection of an independently valid transformed midpoint.
/// # Panics
/// Panic if malformed associations publish, or rule/dispatch reject a legal neighbor.
#[test]
fn midpoint_publication_enforces_raw_association_and_garde_rule()
-> Result<(), RelativeMeasurementRefusal> {
    let healthy = publish_reference(raw_midpoint_parts([10, 11, 10, 1]))?;
    assert_eq!(healthy.whole_nanoseconds(), 10);
    assert!(healthy.has_half_nanosecond());
    for (values, accepted) in MIDPOINT_PART_CASES {
        assert_eq!(
            midpoint_rule(&raw_midpoint_parts(values), &()).is_ok(),
            accepted
        );
        assert_eq!(
            MidpointCandidate {
                parts: raw_midpoint_parts(values),
            }
            .validate()
            .is_ok(),
            accepted
        );
        let published = publish_reference(raw_midpoint_parts(values));
        if accepted {
            let reference = published?;
            assert_eq!(
                u128::from(reference.whole_nanoseconds()) * 2
                    + u128::from(reference.has_half_nanosecond()),
                u128::from(values[0]) + u128::from(values[1])
            );
        } else {
            assert!(matches!(
                published,
                Err(RelativeMeasurementRefusal::OutputInvariant)
            ));
        }
    }
    Ok(())
}

/// Duration boundaries need no assumption that a native Instant spans 584 years.
/// # Errors
/// Propagate refusal of the maximum valid duration or checked neighboring setup.
/// # Panics
/// Panic if the first unrepresentable nanosecond is admitted or a valid value changes.
#[test]
fn duration_nanoseconds_admits_full_u64_and_rejects_next_tick()
-> Result<(), RelativeMeasurementRefusal> {
    assert_eq!(duration_nanoseconds(Duration::ZERO)?, 0);
    let maximum = Duration::from_nanos(u64::MAX);
    assert_eq!(duration_nanoseconds(maximum)?, u64::MAX);
    let next = maximum
        .checked_add(Duration::from_nanos(1))
        .ok_or(RelativeMeasurementRefusal::Arithmetic)?;
    for rejected in [next, Duration::new(u64::MAX, 999_999_999)] {
        assert!(matches!(
            duration_nanoseconds(rejected),
            Err(RelativeMeasurementRefusal::ReferenceRange)
        ));
    }
    Ok(())
}

/// Explicit comparisons reject tiny negative differences before duration conversion.
/// # Errors
/// Propagate an unrepresentable small Instant fixture or valid reference refusal.
/// # Panics
/// Panic if before-origin or reversed readings are rounded/clamped into acceptance.
#[test]
fn reference_instants_check_order_before_duration_conversion()
-> Result<(), RelativeMeasurementRefusal> {
    let origin = Instant::now()
        .checked_add(Duration::from_nanos(10))
        .ok_or(RelativeMeasurementRefusal::Arithmetic)?;
    let before = origin
        .checked_sub(Duration::from_nanos(1))
        .ok_or(RelativeMeasurementRefusal::Arithmetic)?;
    let ten = origin
        .checked_add(Duration::from_nanos(10))
        .ok_or(RelativeMeasurementRefusal::Arithmetic)?;
    let eleven = origin
        .checked_add(Duration::from_nanos(11))
        .ok_or(RelativeMeasurementRefusal::Arithmetic)?;
    assert!(before < origin);
    let at_origin = reference_from_instants(origin, origin, origin)?;
    assert_eq!(at_origin.whole_nanoseconds(), 0);
    assert!(!at_origin.has_half_nanosecond());
    let healthy = reference_from_instants(origin, ten, eleven)?;
    assert_eq!(healthy.whole_nanoseconds(), 10);
    assert!(healthy.has_half_nanosecond());
    for (send, read) in [(before, origin), (origin, before)] {
        assert!(matches!(
            reference_from_instants(origin, send, read),
            Err(RelativeMeasurementRefusal::ReferenceBeforeOrigin)
        ));
    }
    assert!(matches!(
        reference_from_instants(origin, eleven, ten),
        Err(RelativeMeasurementRefusal::ReferenceOrder)
    ));
    let equal = reference_from_instants(origin, eleven, eleven)?;
    assert_eq!(equal.whole_nanoseconds(), 11);
    assert!(!equal.has_half_nanosecond());
    Ok(())
}

/// Self-minus-anchor direction and half ticks survive exact reference subtraction.
/// # Errors
/// Propagate refusal of a valid constructed reference or its pure difference.
/// # Panics
/// Panic if sign, exact tuples, rounding or arithmetic bounds change.
#[test]
fn relative_reference_differences_keep_direction_and_half_ticks()
-> Result<(), RelativeMeasurementRefusal> {
    for (row, anchor, sign, magnitude, value, bound) in [
        (
            (11, 11),
            (10, 10),
            Positive,
            2,
            0x3e11_2e0b_e826_d695,
            0x3ac0_0000_0000_0000,
        ),
        (
            (10, 10),
            (11, 11),
            Negative,
            2,
            0xbe11_2e0b_e826_d695,
            0x3ac0_0000_0000_0000,
        ),
        (
            (10, 11),
            (10, 10),
            Positive,
            1,
            0x3e01_2e0b_e826_d695,
            0x3ab0_0000_0000_0000,
        ),
        (
            (10, 10),
            (10, 11),
            Negative,
            1,
            0xbe01_2e0b_e826_d695,
            0x3ab0_0000_0000_0000,
        ),
        ((10, 11), (10, 11), Positive, 0, 0, 0),
        (
            (u64::MAX, u64::MAX),
            (0, 0),
            Positive,
            36_893_488_147_419_103_230,
            0x4211_2e0b_e826_d695,
            0x3ec0_0000_0000_0000,
        ),
        (
            (u64::MAX - 1, u64::MAX),
            (u64::MAX, u64::MAX),
            Negative,
            1,
            0xbe01_2e0b_e826_d695,
            0x3ab0_0000_0000_0000,
        ),
    ] {
        let row = reference_from_bounds(row.0, row.1)?;
        let anchor = reference_from_bounds(anchor.0, anchor.1)?;
        let result = reference_difference(row, anchor)?;
        assert_relative_seconds(&result, sign, magnitude, 2_000_000_000, value, bound);
    }
    Ok(())
}

/// The raw signed helper admits the entire i128 pair domain, including its extremes.
/// These inputs do not construct a nominal phase or claim capture provenance.
/// # Errors
/// Propagate an unexpected checked conversion refusal of a valid raw difference.
/// # Panics
/// Panic if signed subtraction overflows, magnitude truncates or sign/bound changes.
#[test]
fn relative_phase_raw_i128_endpoints_keep_u128_magnitude() -> Result<(), RelativeMeasurementRefusal>
{
    for (row, anchor, sign, value) in [
        (i128::MAX, i128::MIN, Positive, 0x4401_2e0b_e826_d695),
        (i128::MIN, i128::MAX, Negative, 0xc401_2e0b_e826_d695),
    ] {
        let result = phase_difference(row, anchor)?;
        assert_relative_seconds(
            &result,
            sign,
            u128::MAX,
            8_589_934_592_000_000_000,
            value,
            0x40b0_0000_0000_0000,
        );
    }
    Ok(())
}

/// Equal origin values do not merge distinct private run allocations.
/// This exercises only the raw context boundary, without constructing public facts.
/// # Errors
/// Propagate unexpected refusal of genuine clones of one private context.
/// # Panics
/// Panic if pointer-distinct contexts are admitted or an admitted origin changes.
#[test]
fn shared_origin_distinguishes_equal_values_from_run_identity()
-> Result<(), RelativeMeasurementRefusal> {
    let origin = Instant::now();
    let row = Arc::new(RunClockContext { origin });
    let same = Arc::clone(&row);
    assert_eq!(shared_origin(&row, &same)?, origin);
    let different = Arc::new(RunClockContext { origin });
    assert!(!Arc::ptr_eq(&row, &different));
    assert!(matches!(
        shared_origin(&row, &different),
        Err(RelativeMeasurementRefusal::DifferentRun)
    ));
    Ok(())
}

/// Raw scale admission executes before constructing an immutable seconds value.
/// # Errors
/// Propagate unexpected refusal of a positive checked scale or canonical zero.
/// # Panics
/// Panic if zero scale publishes or exact tuple, sign, rounding or bound changes.
#[test]
fn relative_seconds_scale_requires_nonzero_with_healthy_neighbors()
-> Result<(), RelativeMeasurementRefusal> {
    let healthy = convert_seconds(Positive, 1, 8_589_934_592_000_000_000)?;
    assert_relative_seconds(
        &healthy,
        Positive,
        1,
        8_589_934_592_000_000_000,
        0x3c01_2e0b_e826_d695,
        0x38b0_0000_0000_0000,
    );
    assert!(matches!(
        convert_seconds(Positive, 1, 0),
        Err(RelativeMeasurementRefusal::Arithmetic)
    ));
    let zero = convert_seconds(Negative, 0, 8_589_934_592_000_000_000)?;
    assert_relative_seconds(&zero, Positive, 0, 8_589_934_592_000_000_000, 0, 0);
    Ok(())
}

/// Fixed diagnostic text remains meaningful and omits numeric and endpoint data.
/// Original nested measurement causes survive; other fixed refusals have no cause.
/// # Panics
/// Panic if output disappears, changes its declared identity or loses its cause.
#[test]
fn relative_diagnostics_preserve_fixed_text_and_original_causes() {
    for (reason, expected) in [
        (
            RelativeMeasurementRefusal::DifferentRun,
            "relative measurement run mismatch",
        ),
        (
            RelativeMeasurementRefusal::RowMeasurement(MeasurementRefusal::DateScope),
            "relative row measurement unavailable",
        ),
        (
            RelativeMeasurementRefusal::AnchorMeasurement(MeasurementRefusal::CaptureOrder),
            "relative anchor measurement unavailable",
        ),
        (
            RelativeMeasurementRefusal::ReferenceBeforeOrigin,
            "relative reference before origin",
        ),
        (
            RelativeMeasurementRefusal::ReferenceOrder,
            "relative reference order",
        ),
        (
            RelativeMeasurementRefusal::ReferenceRange,
            "relative reference range",
        ),
        (
            RelativeMeasurementRefusal::Arithmetic,
            "relative measurement arithmetic",
        ),
        (
            RelativeMeasurementRefusal::OutputInvariant,
            "relative reference output invariant",
        ),
    ] {
        assert_eq!(reason.to_string(), expected);
        match reason {
            RelativeMeasurementRefusal::RowMeasurement(cause)
            | RelativeMeasurementRefusal::AnchorMeasurement(cause) => assert_eq!(
                reason
                    .source()
                    .and_then(|source| source.downcast_ref::<MeasurementRefusal>()),
                Some(&cause)
            ),
            _ => assert!(reason.source().is_none()),
        }
    }
}

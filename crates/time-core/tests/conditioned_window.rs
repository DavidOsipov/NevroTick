// SPDX-License-Identifier: GPL-2.0-only
//! Independent N1b public preparation facts, frozen before candidate observation.
//!
//! Literal IEEE bits come from exact rational midpoint comparisons in the retained
//! `n1-oracles-v1/generate.py` fixture generator, Python 3.12.3. That generator
//! searches binary64 bit patterns rather than reproducing the product converter.
//! Its 30 input families were frozen before the implementation was observed;
//! the 29 successful families below exclude the separately tested collision.
//! These controls exercise numerical preparation only, with no fit, history,
//! source provenance, current eligibility, physical uncertainty or clock effect.
#![forbid(unsafe_code)]

use std::{error::Error, io, num::NonZeroU64, ptr};

use time_core::{
    ConditionedBatchOutcome, ConditionedCoordinate, ConditionedWindowError,
    DeclaredObservationContext, ExactObservationRow, InsufficientConditionedBatch,
    PreparedConditionedBatch, WindowVarianceError, WindowVarianceStage, prepare_conditioned_batch,
};
use time_domain::{RationalSecondsError, RationalSign, RoundedRationalSeconds};

/// Preserve genuine product errors separately from assertion and fixture failures.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;

/// Exact corrected-phase denominator in seconds, independent of rounded values.
const PHASE_DENOMINATOR: u64 = 8_589_934_592_000_000_000;
/// Exact half-nanosecond reference denominator in seconds.
const REFERENCE_DENOMINATOR: u64 = 2_000_000_000;
/// Smallest legal positive finite scalar input.
const QUANTUM: f64 = f64::from_bits(1);
/// All independently rejected floating scalar classes.
const INVALID_SECONDS: [f64; 7] = [
    0.0,
    -0.0,
    -1.0,
    f64::from_bits(0x8000_0000_0000_0001),
    f64::from_bits(0x7ff8_0000_0000_0042),
    f64::INFINITY,
    f64::NEG_INFINITY,
];

/// Original observable row bits and declared association, without product mutation.
type RowSnapshot = (DeclaredObservationContext, NonZeroU64, u64, bool, i128, u64);

/// One independent exact-ratio and once-rounded/scaled output fact.
struct DeltaFact {
    /// Whether the nonzero exact delta is negative.
    negative: bool,
    /// Unreduced magnitude using the actual interface denominator.
    magnitude: u128,
    /// Independently nearest/ties-even binary64 seconds bits.
    seconds: u64,
    /// Exact original-binade half-ULP bound bits, zero for exact conversion.
    roundoff: u64,
    /// Bit-exact power-of-two scaling of the once-rounded seconds.
    normalized: u64,
    /// Bit-exact power-of-two scaling of its arithmetic-only bound.
    normalized_roundoff: u64,
}

/// One complete independently specified successful three-row numeric window.
struct FrozenCase {
    /// Stable fixture identity used only in test assertion diagnostics.
    name: &'static str,
    /// Literal whole-nanosecond/half-bit reference inputs.
    references: [(u64, bool); 3],
    /// Literal exact corrected-phase numerator inputs.
    phases: [i128; 3],
    /// Distances in original row order, before any scratch sorting.
    distances: [f64; 3],
    /// Whole-population variance divisors independently associated by index.
    divisors: [f64; 3],
    /// Exact oldest-span ceiling exponent.
    time_exponent: i32,
    /// Exact maximum-phase-delta ceiling exponent, zero for constant phases.
    phase_exponent: i32,
    /// Exact and once-rounded reference-delta expectations.
    reference_facts: [DeltaFact; 3],
    /// Exact and once-rounded phase-delta expectations.
    phase_facts: [DeltaFact; 3],
}

/// Construct immutable expected data without calling any product arithmetic.
const fn fact(
    negative: bool,
    magnitude: u128,
    seconds: u64,
    roundoff: u64,
    normalized: u64,
    normalized_roundoff: u64,
) -> DeltaFact {
    DeltaFact {
        negative,
        magnitude,
        seconds,
        roundoff,
        normalized,
        normalized_roundoff,
    }
}

/// Canonical zero is positive at both once-rounded and scaled representations.
const ZERO: DeltaFact = fact(false, 0, 0, 0, 0, 0);

/// Supply one legal association; labels assert no real acquisition authority.
const fn context() -> DeclaredObservationContext {
    DeclaredObservationContext::new(
        NonZeroU64::MIN,
        NonZeroU64::MIN,
        NonZeroU64::MIN,
        NonZeroU64::MIN,
        NonZeroU64::MIN,
    )
}

/// Construct one admitted row from literal input parts through the actual boundary.
/// # Errors
/// Rejects an invalid fixture key or propagates the product row-construction error.
fn row(
    declared: DeclaredObservationContext,
    key: u64,
    reference: (u64, bool),
    phase: i128,
    distance: f64,
) -> TestResult<ExactObservationRow> {
    let key = NonZeroU64::new(key).ok_or("fixture key must be nonzero")?;
    Ok(ExactObservationRow::new(
        declared,
        key,
        reference.0,
        reference.1,
        phase,
        distance,
    )?)
}

/// Construct a complete legal three-row fixture using independent original keys.
/// # Errors
/// Propagates any real row-construction failure.
fn three_rows(
    declared: DeclaredObservationContext,
    references: [(u64, bool); 3],
    phases: [i128; 3],
    distances: [f64; 3],
) -> TestResult<[ExactObservationRow; 3]> {
    Ok([
        row(declared, 1, references[0], phases[0], distances[0])?,
        row(declared, 2, references[1], phases[1], distances[1])?,
        row(declared, 3, references[2], phases[2], distances[2])?,
    ])
}

/// Read all publicly observable original row parts without altering the owner.
fn snapshot(rows: &[ExactObservationRow]) -> Vec<RowSnapshot> {
    rows.iter()
        .map(|value| {
            (
                value.context(),
                value.observation_key(),
                value.reference_whole_ns(),
                value.reference_half(),
                value.corrected_phase_numerator(),
                value.peer_distance_seconds().to_bits(),
            )
        })
        .collect()
}

/// Select the closed successful branch with an ordinary fixture error on mismatch.
/// # Errors
/// Rejects an unexpectedly insufficient result without inventing a prepared value.
fn prepared(outcome: ConditionedBatchOutcome<'_>) -> TestResult<PreparedConditionedBatch<'_>> {
    match outcome {
        ConditionedBatchOutcome::Prepared(batch) => Ok(batch),
        ConditionedBatchOutcome::Insufficient(_) => {
            Err(io::Error::other("expected prepared window").into())
        }
    }
}

/// Select the closed insufficient branch without accepting a fake prepared result.
/// # Errors
/// Rejects an unexpectedly prepared result.
fn insufficient(
    outcome: ConditionedBatchOutcome<'_>,
) -> TestResult<InsufficientConditionedBatch<'_>> {
    match outcome {
        ConditionedBatchOutcome::Insufficient(batch) => Ok(batch),
        ConditionedBatchOutcome::Prepared(_) => {
            Err(io::Error::other("expected insufficient window").into())
        }
    }
}

/// Compare exact source tuple and literal rounding/bound bits; no tolerance fitting.
/// # Panics
/// Requires all independently declared exact and rounded components to agree.
fn assert_delta(actual: RoundedRationalSeconds, expected: &DeltaFact, denominator: u64) {
    let sign = if expected.negative {
        RationalSign::Negative
    } else {
        RationalSign::Positive
    };
    assert_eq!(actual.sign(), sign);
    assert_eq!(actual.numerator_magnitude(), expected.magnitude);
    assert_eq!(actual.denominator().get(), denominator);
    assert_eq!(actual.seconds().to_bits(), expected.seconds);
    assert_eq!(
        actual.absolute_roundoff_seconds().to_bits(),
        expected.roundoff
    );
}

/// Exercise exact subtraction, ceiling scales, conversion ties and transformed bounds.
/// # Errors
/// Propagates any unexpected real row/preparation failure.
/// # Panics
/// Requires every independent tuple/bit fact, association and getter boundary.
#[test]
fn frozen_exact_and_ieee_conditioning_facts() -> TestResult {
    for case in &FROZEN_CASES {
        let declared = context();
        let rows = three_rows(declared, case.references, case.phases, case.distances)?;
        let before = snapshot(&rows);
        let batch = prepared(prepare_conditioned_batch(declared, &rows, 0.25)?)?;
        assert_eq!(batch.declared_context(), declared, "{}", case.name);
        assert_eq!(batch.sample_count(), 3, "{}", case.name);
        assert_eq!(
            batch.current_precision_seconds().to_bits(),
            0x3fd0_0000_0000_0000
        );
        assert_eq!(
            batch.time_scale_exponent(),
            case.time_exponent,
            "{}",
            case.name
        );
        assert_eq!(
            batch.phase_scale_exponent(),
            case.phase_exponent,
            "{}",
            case.name
        );
        assert!(ptr::eq(
            ptr::from_ref(batch.original_rows()),
            ptr::from_ref(rows.as_slice())
        ));
        assert!(ptr::eq(
            ptr::from_ref(batch.newest_row()),
            ptr::from_ref(&rows[2])
        ));
        for (index, original) in rows.iter().enumerate() {
            let point = batch.point(index).ok_or("missing retained point")?;
            let x = &case.reference_facts[index];
            let y = &case.phase_facts[index];
            assert!(ptr::eq(
                ptr::from_ref(point.original_row()),
                ptr::from_ref(original)
            ));
            assert_delta(point.reference_delta(), x, REFERENCE_DENOMINATOR);
            assert_delta(point.phase_delta(), y, PHASE_DENOMINATOR);
            assert_eq!(
                point.normalized_reference().to_bits(),
                x.normalized,
                "{}",
                case.name
            );
            assert_eq!(
                point.normalized_phase().to_bits(),
                y.normalized,
                "{}",
                case.name
            );
            assert_eq!(
                point.normalized_reference_roundoff().to_bits(),
                x.normalized_roundoff
            );
            assert_eq!(
                point.normalized_phase_roundoff().to_bits(),
                y.normalized_roundoff
            );
            assert_eq!(
                point.variance_divisor().to_bits(),
                case.divisors[index].to_bits()
            );
        }
        assert!(batch.point(3).is_none());
        assert!(batch.point(64).is_none());
        assert!(batch.point(usize::MAX).is_none());
        assert_eq!(snapshot(&rows), before);
    }
    Ok(())
}

/// Counts zero/one/two are admitted independently of geometry and exact conversion.
/// # Errors
/// Propagates unexpected rejection of a legal insufficient population.
/// # Panics
/// Requires original rows, optional anchors and complete unchanged quality metadata.
#[test]
fn insufficient_counts_retain_anchors_rows_precision_and_divisors() -> TestResult {
    let declared = context();
    let empty: [ExactObservationRow; 0] = [];
    let result = insufficient(prepare_conditioned_batch(declared, &empty, QUANTUM)?)?;
    assert_eq!(result.declared_context(), declared);
    assert_eq!(result.sample_count(), 0);
    assert_eq!(result.current_precision_seconds().to_bits(), 1);
    assert!(ptr::eq(
        ptr::from_ref(result.original_rows()),
        ptr::from_ref(empty.as_slice())
    ));
    assert!(result.newest_row().is_none());
    assert!(result.variance_divisor(0).is_none());
    assert!(result.variance_divisor(usize::MAX).is_none());

    let one = [row(declared, 1, (u64::MAX, false), i128::MAX, 1.0)?];
    let before = snapshot(&one);
    let result = insufficient(prepare_conditioned_batch(declared, &one, QUANTUM)?)?;
    assert_eq!(result.sample_count(), 1);
    assert_eq!(result.current_precision_seconds().to_bits(), 1);
    assert!(ptr::eq(
        ptr::from_ref(result.original_rows()),
        ptr::from_ref(one.as_slice())
    ));
    assert!(ptr::eq(
        ptr::from_ref(result.newest_row().ok_or("singleton anchor")?),
        ptr::from_ref(&one[0])
    ));
    assert_eq!(
        result.variance_divisor(0).map(f64::to_bits),
        Some(0x3ff0_0000_0000_0000)
    );
    assert!(result.variance_divisor(1).is_none());
    assert!(result.variance_divisor(usize::MAX).is_none());
    assert_eq!(snapshot(&one), before);

    let two = [
        row(declared, 1, (0, false), i128::MIN, 1.0)?,
        row(declared, 2, (0, true), i128::MAX, 3.0)?,
    ];
    let before = snapshot(&two);
    let result = insufficient(prepare_conditioned_batch(declared, &two, 0.25)?)?;
    assert_eq!(result.declared_context(), declared);
    assert_eq!(result.sample_count(), 2);
    assert_eq!(
        result.current_precision_seconds().to_bits(),
        0x3fd0_0000_0000_0000
    );
    assert!(ptr::eq(
        ptr::from_ref(result.original_rows()),
        ptr::from_ref(two.as_slice())
    ));
    assert!(ptr::eq(
        ptr::from_ref(result.newest_row().ok_or("two-row anchor")?),
        ptr::from_ref(&two[1])
    ));
    assert_eq!(
        result.variance_divisor(0).map(f64::to_bits),
        Some(0x3ff0_0000_0000_0000)
    );
    assert_eq!(
        result.variance_divisor(1).map(f64::to_bits),
        Some(0x401e_4000_0000_0000)
    );
    assert!(result.variance_divisor(2).is_none());
    assert!(result.variance_divisor(usize::MAX).is_none());
    assert_eq!(snapshot(&two), before);
    Ok(())
}

/// Sixty-four rows preserve the last nonunit member; 65 rejects before precision.
/// # Errors
/// Propagates any unexpected admitted-row/preparation failure.
/// # Panics
/// Requires complete maximum-size association and the one-over count error.
#[test]
fn maximum_and_one_over_counts_preserve_original_population() -> TestResult {
    let declared = context();
    let rows = (0_u64..65)
        .map(|index| {
            row(
                declared,
                index + 1,
                (index, false),
                0,
                if index == 63 { 2.0 } else { 1.0 },
            )
        })
        .collect::<TestResult<Vec<_>>>()?;
    let before = snapshot(&rows);
    let batch = prepared(prepare_conditioned_batch(declared, &rows[..64], 0.25)?)?;
    assert_eq!(batch.sample_count(), 64);
    assert_eq!(batch.time_scale_exponent(), -23);
    assert_eq!(batch.phase_scale_exponent(), 0);
    assert!(ptr::eq(
        ptr::from_ref(batch.original_rows()),
        ptr::from_ref(&rows[..64])
    ));
    assert!(ptr::eq(
        ptr::from_ref(batch.newest_row()),
        ptr::from_ref(&rows[63])
    ));
    for (index, original) in rows[..64].iter().enumerate() {
        let point = batch.point(index).ok_or("missing maximum-size point")?;
        assert!(ptr::eq(
            ptr::from_ref(point.original_row()),
            ptr::from_ref(original)
        ));
        assert_eq!(
            point.variance_divisor().to_bits(),
            if index == 63 {
                0x4030_0000_0000_0000
            } else {
                0x3ff0_0000_0000_0000
            }
        );
        assert_eq!(point.normalized_phase().to_bits(), 0);
        assert_eq!(point.normalized_phase_roundoff().to_bits(), 0);
    }
    assert!(batch.point(64).is_none());
    assert!(batch.point(usize::MAX).is_none());
    assert_eq!(
        prepare_conditioned_batch(declared, &rows, -0.0).err(),
        Some(ConditionedWindowError::CountOutOfRange { count: 65 })
    );
    assert_eq!(snapshot(&rows), before);
    Ok(())
}

/// Row admission rejects malformed references and every invalid scalar class.
/// # Errors
/// Propagates rejection of legal neighboring references and subnormal distances.
/// # Panics
/// Requires actual receiving constructors to enforce both independent field rules.
#[test]
fn row_constructor_enforces_reference_and_distance_rules() -> TestResult {
    let declared = context();
    assert_eq!(
        ExactObservationRow::new(declared, NonZeroU64::MIN, u64::MAX, true, 0, 1.0).err(),
        Some(ConditionedWindowError::InvalidReference)
    );
    let valid = row(declared, 1, (u64::MAX, false), i128::MIN, QUANTUM)?;
    assert_eq!(valid.reference_whole_ns(), u64::MAX);
    assert!(!valid.reference_half());
    assert_eq!(valid.corrected_phase_numerator(), i128::MIN);
    assert_eq!(valid.peer_distance_seconds().to_bits(), 1);
    for distance in [QUANTUM, f64::MIN_POSITIVE, f64::MAX] {
        let value = row(declared, 1, (0, false), i128::MAX, distance)?;
        assert_eq!(value.peer_distance_seconds().to_bits(), distance.to_bits());
    }
    for invalid in INVALID_SECONDS {
        assert_eq!(
            ExactObservationRow::new(declared, NonZeroU64::MIN, 0, false, 0, invalid).err(),
            Some(ConditionedWindowError::InvalidPeerDistance)
        );
    }
    Ok(())
}

/// Precision validation applies even to empty windows and precedes membership.
/// # Errors
/// Propagates failure to build otherwise legal row fixtures.
/// # Panics
/// Requires rejection of each scalar class and preservation of all input bits.
#[test]
fn invalid_precision_precedes_all_row_membership_checks() -> TestResult {
    let declared = context();
    let different = DeclaredObservationContext::new(
        NonZeroU64::new(2).ok_or("nonzero context")?,
        NonZeroU64::MIN,
        NonZeroU64::MIN,
        NonZeroU64::MIN,
        NonZeroU64::MIN,
    );
    let rows = three_rows(
        different,
        [(0, false), (1, false), (2, false)],
        [0; 3],
        [1.0; 3],
    )?;
    let before = snapshot(&rows);
    for invalid in INVALID_SECONDS {
        assert_eq!(
            prepare_conditioned_batch(declared, &[], invalid).err(),
            Some(ConditionedWindowError::InvalidPrecision)
        );
        assert_eq!(
            prepare_conditioned_batch(declared, &rows, invalid).err(),
            Some(ConditionedWindowError::InvalidPrecision)
        );
    }
    assert_eq!(snapshot(&rows), before);
    Ok(())
}

/// Positive precision endpoints reach the real prepared receiver without narrowing.
/// # Errors
/// Propagates rejection of a legal row or positive finite precision.
/// # Panics
/// Requires original precision bits and all unit quality divisors at every endpoint.
#[test]
fn positive_precision_extremes_reach_the_prepared_receiver() -> TestResult {
    let declared = context();
    let rows = three_rows(
        declared,
        [(0, false), (1, false), (2, false)],
        [0; 3],
        [1.0; 3],
    )?;
    let before = snapshot(&rows);
    for precision in [QUANTUM, f64::MIN_POSITIVE, f64::MAX] {
        let batch = prepared(prepare_conditioned_batch(declared, &rows, precision)?)?;
        assert_eq!(
            batch.current_precision_seconds().to_bits(),
            precision.to_bits()
        );
        for index in 0..3 {
            assert_eq!(
                batch
                    .point(index)
                    .ok_or("precision endpoint point")?
                    .variance_divisor()
                    .to_bits(),
                0x3ff0_0000_0000_0000
            );
        }
    }
    assert_eq!(snapshot(&rows), before);
    Ok(())
}

/// Complete context admission precedes duplicate, exact-order and quality failures.
/// # Errors
/// Propagates rejection of legal declared-label and row fixture construction.
/// # Panics
/// Requires every one of the five context labels and first mismatch index to matter.
#[test]
fn every_context_label_is_checked_before_duplicate_order_and_quality() -> TestResult {
    let declared = context();
    let two = NonZeroU64::new(2).ok_or("nonzero alternate context")?;
    for changed in 0..5 {
        let mut labels = [NonZeroU64::MIN; 5];
        labels[changed] = two;
        let different =
            DeclaredObservationContext::new(labels[0], labels[1], labels[2], labels[3], labels[4]);
        let rows = [
            row(declared, 1, (1, false), 0, 1.0)?,
            row(declared, 1, (0, false), 0, 1.0)?,
            row(different, 3, (2, false), 0, f64::MAX)?,
        ];
        let before = snapshot(&rows);
        assert_eq!(
            prepare_conditioned_batch(declared, &rows, QUANTUM).err(),
            Some(ConditionedWindowError::ContextMismatch { index: 2 })
        );
        assert_eq!(snapshot(&rows), before);
        let singleton = [row(different, 1, (0, false), 0, 1.0)?];
        assert_eq!(
            prepare_conditioned_batch(declared, &singleton, 0.25).err(),
            Some(ConditionedWindowError::ContextMismatch { index: 0 })
        );
    }
    Ok(())
}

/// Original key order precedes exact timestamp order, which precedes quality work.
/// # Errors
/// Propagates any unexpected legal row-construction failure.
/// # Panics
/// Requires first duplicate/order indices and half-tick distinctions without mutation.
#[test]
fn duplicate_and_nonincreasing_reference_errors_have_defined_precedence() -> TestResult {
    let declared = context();
    let duplicate = [
        row(declared, 1, (1, false), 0, 1.0)?,
        row(declared, 1, (0, false), 0, 1.0)?,
        row(declared, 3, (2, false), 0, f64::MAX)?,
    ];
    let before = snapshot(&duplicate);
    assert_eq!(
        prepare_conditioned_batch(declared, &duplicate, QUANTUM).err(),
        Some(ConditionedWindowError::DuplicateObservationKey { index: 1 })
    );
    assert_eq!(snapshot(&duplicate), before);
    let unsorted_keys = [
        row(declared, 9, (0, false), 0, 1.0)?,
        row(declared, 1, (1, false), 0, 1.0)?,
        row(declared, 9, (2, false), 0, 1.0)?,
        row(declared, 1, (3, false), 0, f64::MAX)?,
    ];
    let before = snapshot(&unsorted_keys);
    assert_eq!(
        prepare_conditioned_batch(declared, &unsorted_keys, QUANTUM).err(),
        Some(ConditionedWindowError::DuplicateObservationKey { index: 2 })
    );
    assert_eq!(snapshot(&unsorted_keys), before);
    let two_duplicate = [
        row(declared, 1, (0, false), 0, 1.0)?,
        row(declared, 1, (0, true), 0, 1.0)?,
    ];
    assert_eq!(
        prepare_conditioned_batch(declared, &two_duplicate, 0.25).err(),
        Some(ConditionedWindowError::DuplicateObservationKey { index: 1 })
    );
    let two_reversed = [
        row(declared, 1, (0, true), 0, 1.0)?,
        row(declared, 2, (0, false), 0, 1.0)?,
    ];
    assert_eq!(
        prepare_conditioned_batch(declared, &two_reversed, 0.25).err(),
        Some(ConditionedWindowError::NonIncreasingReference { index: 1 })
    );
    for (references, index) in [
        ([(1, false), (0, false), (2, false)], 1),
        ([(0, false), (0, true), (0, true)], 2),
        ([(0, true), (0, false), (1, false)], 1),
    ] {
        let rows = three_rows(declared, references, [0; 3], [1.0, 1.0, f64::MAX])?;
        let before = snapshot(&rows);
        assert_eq!(
            prepare_conditioned_batch(declared, &rows, QUANTUM).err(),
            Some(ConditionedWindowError::NonIncreasingReference { index })
        );
        assert_eq!(snapshot(&rows), before);
    }
    Ok(())
}

/// Weight derivation retains original-order uncapped, subnormal and reciprocal cases.
/// # Errors
/// Propagates unexpected refusal of any legal numerical population.
/// # Panics
/// Requires literal whole-population divisor bits and unchanged original associations.
#[test]
fn uncapped_and_subnormal_quality_vectors_are_retained_unchanged() -> TestResult {
    let declared = context();
    for (distances, precision, expected) in [
        ([4.0, 1.0, 1.0], 0.25, [144.0, 1.0, 1.0]),
        (
            [QUANTUM, QUANTUM, f64::from_bits(4)],
            QUANTUM,
            [1.0, 1.0, 9.0],
        ),
        (
            [1.0, 1.0, f64::from_bits(0x5fe8_0000_0000_0000)],
            1.0,
            [1.0, 1.0, f64::from_bits(0x7fe2_0000_0000_0000)],
        ),
    ] {
        let rows = three_rows(
            declared,
            [(0, false), (1, false), (2, false)],
            [0; 3],
            distances,
        )?;
        let before = snapshot(&rows);
        let batch = prepared(prepare_conditioned_batch(declared, &rows, precision)?)?;
        assert_eq!(
            batch.current_precision_seconds().to_bits(),
            precision.to_bits()
        );
        for (index, value) in expected.into_iter().enumerate() {
            let point = batch.point(index).ok_or("quality point")?;
            assert_eq!(point.variance_divisor().to_bits(), value.to_bits());
            assert_eq!(
                point.original_row().peer_distance_seconds().to_bits(),
                distances[index].to_bits()
            );
        }
        assert_eq!(snapshot(&rows), before);
    }
    // Literal nearest/even reciprocal of 9*2^1020; this remains N1 admission,
    // not execution or qualification of a future weighted-fit primitive.
    assert_eq!(
        (1.0 / f64::from_bits(0x7fe2_0000_0000_0000)).to_bits(),
        0x0007_1c71_c71c_71c7
    );
    Ok(())
}

/// Actual quality arithmetic failures remain errors even for insufficient counts.
/// # Errors
/// Propagates failure to build the finite positive fixture rows.
/// # Panics
/// Requires original nested stage causes, no partial result and unchanged input bits.
#[test]
fn quality_failures_are_preserved_for_one_two_and_three_rows() -> TestResult {
    let declared = context();
    let cases: [(&[f64], f64, WindowVarianceStage); 4] = [
        (&[f64::MAX], f64::MAX, WindowVarianceStage::Threshold),
        (&[f64::MAX, f64::MAX], 1.0, WindowVarianceStage::MedianSum),
        (
            &[QUANTUM, QUANTUM, 1.0],
            QUANTUM,
            WindowVarianceStage::Ratio,
        ),
        (
            &[1.0, 1.0, f64::from_bits(0x5ff0_0000_0000_0000)],
            1.0,
            WindowVarianceStage::Square,
        ),
    ];
    for (distances, precision, stage) in cases {
        let rows = distances
            .iter()
            .enumerate()
            .map(|(index, distance)| {
                let value = u64::try_from(index)?;
                row(declared, value + 1, (value, false), 0, *distance)
            })
            .collect::<TestResult<Vec<_>>>()?;
        let before = snapshot(&rows);
        let error = prepare_conditioned_batch(declared, &rows, precision)
            .err()
            .ok_or("quality failure expected")?;
        assert_eq!(
            error,
            ConditionedWindowError::Quality {
                cause: WindowVarianceError::Arithmetic { stage }
            }
        );
        assert!(error.source().is_some());
        assert_eq!(snapshot(&rows), before);
    }
    Ok(())
}

/// Exact half-tick order can collide after once rounding; the entire window refuses.
/// # Errors
/// Propagates legal row construction and integer fixture conversion errors.
/// # Panics
/// Requires both an early and a late original-index collision without mutation.
#[test]
fn rounded_reference_collisions_reject_early_and_late_without_partial_payload() -> TestResult {
    let declared = context();
    let early = three_rows(
        declared,
        [(0, false), (0, true), (1 << 63, false)],
        [0; 3],
        [1.0; 3],
    )?;
    let before = snapshot(&early);
    assert_eq!(
        prepare_conditioned_batch(declared, &early, 0.25).err(),
        Some(ConditionedWindowError::RoundedReferenceCollision { index: 1 })
    );
    assert_eq!(snapshot(&early), before);

    let mut half_ticks: Vec<u128> = (0_u128..62).map(|index| index << 58).collect();
    half_ticks.push((61_u128 << 58) + 1);
    half_ticks.push(1_u128 << 64);
    let rows = half_ticks
        .iter()
        .enumerate()
        .map(|(index, h)| {
            row(
                declared,
                u64::try_from(index)? + 1,
                (u64::try_from(h / 2)?, !h.is_multiple_of(2)),
                0,
                1.0,
            )
        })
        .collect::<TestResult<Vec<_>>>()?;
    let before = snapshot(&rows);
    assert_eq!(
        prepare_conditioned_batch(declared, &rows, 0.25).err(),
        Some(ConditionedWindowError::RoundedReferenceCollision { index: 62 })
    );
    assert_eq!(snapshot(&rows), before);
    Ok(())
}

/// Distinct literal labels discriminate constructor/getter swaps and key replacement.
/// # Errors
/// Propagates legal fixture-key, row or population admission failures.
/// # Panics
/// Requires each named label, nonminimum key and returned context to retain its input.
#[test]
fn distinct_declared_labels_and_observation_key_retain_their_exact_fields() -> TestResult {
    let declared = DeclaredObservationContext::new(
        NonZeroU64::new(13).ok_or("logical source")?,
        NonZeroU64::new(29).ok_or("configuration generation")?,
        NonZeroU64::new(43).ok_or("association generation")?,
        NonZeroU64::new(71).ok_or("clock generation")?,
        NonZeroU64::new(u64::MAX).ok_or("mapping generation")?,
    );
    assert_eq!(declared.logical_source().get(), 13);
    assert_eq!(declared.configuration_generation().get(), 29);
    assert_eq!(declared.association_generation().get(), 43);
    assert_eq!(declared.clock_generation().get(), 71);
    assert_eq!(declared.mapping_generation().get(), u64::MAX);
    let rows = [
        row(declared, 97, (0, false), i128::MAX - 2, 1.0)?,
        row(declared, 101, (0, true), i128::MAX - 1, 1.0)?,
        row(declared, 103, (1, false), i128::MAX, 1.0)?,
    ];
    assert_eq!(rows[0].observation_key().get(), 97);
    assert_eq!(rows[0].context(), declared);
    let batch = prepared(prepare_conditioned_batch(declared, &rows, 0.25)?)?;
    assert_eq!(batch.declared_context(), declared);
    assert_eq!(batch.newest_row().observation_key().get(), 103);
    let short = insufficient(prepare_conditioned_batch(declared, &rows[..1], 0.25)?)?;
    assert_eq!(short.declared_context(), declared);
    assert_eq!(
        short
            .newest_row()
            .ok_or("short anchor")?
            .observation_key()
            .get(),
        97
    );
    Ok(())
}

/// Require bounded useful diagnostics without any of the actual fixture payloads.
/// # Panics
/// Rejects empty/oversized text, raw sentinel values or a leaked validation report.
fn assert_redacted_diagnostic(message: &str) {
    assert!((1..=128).contains(&message.len()));
    for payload in [
        "7123456789012345000",
        "-123456789012345678901234567890",
        "31337.25",
        "12345678901",
        "18446744073709551615",
    ] {
        assert!(!message.contains(payload));
    }
    let lower = message.to_ascii_lowercase();
    assert!(!lower.contains("garde"));
    assert!(!lower.contains("report"));
}

/// Check all error families and nested typed sources beside an actual quality error.
/// # Errors
/// Propagates legal row construction or a missing required rejection/cause.
/// # Panics
/// Requires bounded diagnostics, exact causes and terminal cause chains.
fn assert_typed_diagnostic_causes(declared: DeclaredObservationContext) -> TestResult {
    let plain = [
        ConditionedWindowError::InvalidReference,
        ConditionedWindowError::InvalidPeerDistance,
        ConditionedWindowError::CountOutOfRange { count: 65 },
        ConditionedWindowError::InvalidPrecision,
        ConditionedWindowError::ContextMismatch { index: 1 },
        ConditionedWindowError::DuplicateObservationKey { index: 1 },
        ConditionedWindowError::NonIncreasingReference { index: 1 },
        ConditionedWindowError::RoundedReferenceCollision { index: 2 },
        ConditionedWindowError::ArithmeticInvariant,
        ConditionedWindowError::OutputInvariant,
        ConditionedWindowError::ValidationInvariant,
    ];
    for error in plain {
        assert_redacted_diagnostic(&error.to_string());
        assert!(error.source().is_none());
    }
    let maximum = [row(declared, 97, (0, false), 0, f64::MAX)?];
    let quality = prepare_conditioned_batch(declared, &maximum, f64::MAX)
        .err()
        .ok_or("quality rejection")?;
    let expected = WindowVarianceError::Arithmetic {
        stage: WindowVarianceStage::Threshold,
    };
    assert_eq!(quality, ConditionedWindowError::Quality { cause: expected });
    assert_redacted_diagnostic(&quality.to_string());
    let source = quality.source().ok_or("quality cause")?;
    assert_eq!(
        source.downcast_ref::<WindowVarianceError>(),
        Some(&expected)
    );
    assert_redacted_diagnostic(&source.to_string());
    assert!(source.source().is_none());
    // Conversion variants exercise diagnostic projection; their construction
    // does not claim an arithmetic failure is reachable from admitted N1 rows.
    for coordinate in [
        ConditionedCoordinate::Reference,
        ConditionedCoordinate::Phase,
    ] {
        for cause in [
            RationalSecondsError::ArithmeticInvariant,
            RationalSecondsError::OutputInvariant,
        ] {
            let error = ConditionedWindowError::Conversion {
                index: 2,
                coordinate,
                cause,
            };
            assert_redacted_diagnostic(&error.to_string());
            let source = error.source().ok_or("conversion cause")?;
            assert_eq!(source.downcast_ref::<RationalSecondsError>(), Some(&cause));
            assert_redacted_diagnostic(&source.to_string());
            assert!(source.source().is_none());
        }
    }
    Ok(())
}

/// Checked objects redact their payloads and errors preserve bounded typed causes.
/// # Errors
/// Propagates legal fixture construction or a missing required preparation/cause.
/// # Panics
/// Requires useful type families, real quality rejection and both conversion causes.
#[test]
fn diagnostics_redact_payloads_and_preserve_typed_causes() -> TestResult {
    let declared = DeclaredObservationContext::new(
        NonZeroU64::new(12_345_678_901).ok_or("diagnostic label")?,
        NonZeroU64::new(29).ok_or("configuration generation")?,
        NonZeroU64::new(43).ok_or("association generation")?,
        NonZeroU64::new(71).ok_or("clock generation")?,
        NonZeroU64::new(u64::MAX).ok_or("mapping generation")?,
    );
    let rows = three_rows(
        declared,
        [
            (7_123_456_789_012_345_000, false),
            (7_123_456_789_012_345_000, true),
            (7_123_456_789_012_345_001, false),
        ],
        [-123_456_789_012_345_678_901_234_567_890; 3],
        [31_337.25; 3],
    )?;
    let ready_outcome = prepare_conditioned_batch(declared, &rows, 0.25)?;
    let ready_debug = format!("{ready_outcome:?}");
    let batch = prepared(ready_outcome)?;
    let point = batch.point(0).ok_or("diagnostic point")?;
    let short_outcome = prepare_conditioned_batch(declared, &rows[..2], 0.25)?;
    let short_debug = format!("{short_outcome:?}");
    let short = insufficient(short_outcome)?;
    for (rendered, family) in [
        (format!("{declared:?}"), "DeclaredObservationContext"),
        (format!("{:?}", rows[0]), "ExactObservationRow"),
        (format!("{point:?}"), "PreparedConditionedPoint"),
        (format!("{batch:?}"), "PreparedConditionedBatch"),
        (format!("{short:?}"), "InsufficientConditionedBatch"),
        (ready_debug, "ConditionedBatchOutcome::Prepared"),
        (short_debug, "ConditionedBatchOutcome::Insufficient"),
    ] {
        assert!(rendered.contains(family));
        assert_redacted_diagnostic(&rendered);
    }
    assert_typed_diagnostic_causes(declared)
}

/// Twenty-nine successful families from independently retained exact source facts.
const FROZEN_CASES: [FrozenCase; 29] = [
    FrozenCase {
        name: "known_millisecond_window",
        references: [(0, false), (1_000_000_000, false), (2_000_000_000, false)],
        phases: [-8_589_934_592_000_000, 0, 25_769_803_776_000_000],
        distances: [1.0, 1.0, 1.5],
        divisors: [1.0, 1.0, 4.0],
        time_exponent: 1,
        phase_exponent: -7,
        reference_facts: [
            fact(
                true,
                4_000_000_000,
                0xc000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                2_000_000_000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [
            fact(
                true,
                34_359_738_368_000_000,
                0xbf70_624d_d2f1_a9fc,
                0x3c20_0000_0000_0000,
                0xbfe0_624d_d2f1_a9fc,
                0x3c90_0000_0000_0000,
            ),
            fact(
                true,
                25_769_803_776_000_000,
                0xbf68_9374_bc6a_7efa,
                0x3c10_0000_0000_0000,
                0xbfd8_9374_bc6a_7efa,
                0x3c80_0000_0000_0000,
            ),
            ZERO,
        ],
    },
    FrozenCase {
        name: "minimum_reference_span_and_zero_phase",
        references: [(0, false), (0, true), (1, false)],
        phases: [7, 7, 7],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: -29,
        phase_exponent: 0,
        reference_facts: [
            fact(
                true,
                2,
                0xbe11_2e0b_e826_d695,
                0x3ac0_0000_0000_0000,
                0xbfe1_2e0b_e826_d695,
                0x3c90_0000_0000_0000,
            ),
            fact(
                true,
                1,
                0xbe01_2e0b_e826_d695,
                0x3ab0_0000_0000_0000,
                0xbfd1_2e0b_e826_d695,
                0x3c80_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [ZERO, ZERO, ZERO],
    },
    FrozenCase {
        name: "maximum_reference_span",
        references: [
            (0, false),
            (9_223_372_036_854_775_808, false),
            (18_446_744_073_709_551_615, false),
        ],
        phases: [0, 0, 0],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 35,
        phase_exponent: 0,
        reference_facts: [
            fact(
                true,
                36_893_488_147_419_103_230,
                0xc211_2e0b_e826_d695,
                0x3ec0_0000_0000_0000,
                0xbfe1_2e0b_e826_d695,
                0x3c90_0000_0000_0000,
            ),
            fact(
                true,
                18_446_744_073_709_551_614,
                0xc201_2e0b_e826_d695,
                0x3eb0_0000_0000_0000,
                0xbfd1_2e0b_e826_d695,
                0x3c80_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [ZERO, ZERO, ZERO],
    },
    FrozenCase {
        name: "full_negative_phase_difference",
        references: [(0, false), (1_000_000_000, false), (2_000_000_000, false)],
        phases: [i128::MIN, 0, i128::MAX],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 1,
        phase_exponent: 66,
        reference_facts: [
            fact(
                true,
                4_000_000_000,
                0xc000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                2_000_000_000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [
            fact(
                true,
                u128::MAX,
                0xc401_2e0b_e826_d695,
                0x40b0_0000_0000_0000,
                0xbfe1_2e0b_e826_d695,
                0x3c90_0000_0000_0000,
            ),
            fact(
                true,
                170_141_183_460_469_231_731_687_303_715_884_105_727,
                0xc3f1_2e0b_e826_d695,
                0x40a0_0000_0000_0000,
                0xbfd1_2e0b_e826_d695,
                0x3c80_0000_0000_0000,
            ),
            ZERO,
        ],
    },
    FrozenCase {
        name: "full_positive_phase_difference",
        references: [(0, false), (1_000_000_000, false), (2_000_000_000, false)],
        phases: [i128::MAX, 0, i128::MIN],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 1,
        phase_exponent: 66,
        reference_facts: [
            fact(
                true,
                4_000_000_000,
                0xc000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                2_000_000_000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [
            fact(
                false,
                u128::MAX,
                0x4401_2e0b_e826_d695,
                0x40b0_0000_0000_0000,
                0x3fe1_2e0b_e826_d695,
                0x3c90_0000_0000_0000,
            ),
            fact(
                false,
                170_141_183_460_469_231_731_687_303_715_884_105_728,
                0x43f1_2e0b_e826_d695,
                0x40a0_0000_0000_0000,
                0x3fd1_2e0b_e826_d695,
                0x3c80_0000_0000_0000,
            ),
            ZERO,
        ],
    },
    FrozenCase {
        name: "minimum_nonzero_phase",
        references: [(0, false), (1_000_000_000, false), (2_000_000_000, false)],
        phases: [-1, 1, 0],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 1,
        phase_exponent: -62,
        reference_facts: [
            fact(
                true,
                4_000_000_000,
                0xc000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                2_000_000_000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [
            fact(
                true,
                1,
                0xbc01_2e0b_e826_d695,
                0x38b0_0000_0000_0000,
                0xbfe1_2e0b_e826_d695,
                0x3c90_0000_0000_0000,
            ),
            fact(
                false,
                1,
                0x3c01_2e0b_e826_d695,
                0x38b0_0000_0000_0000,
                0x3fe1_2e0b_e826_d695,
                0x3c90_0000_0000_0000,
            ),
            ZERO,
        ],
    },
    FrozenCase {
        name: "positive_even_odd_phase_ties",
        references: [(0, false), (1_000_000_000, false), (2_000_000_000, false)],
        phases: [
            17_592_186_044_416_001_953_125,
            17_592_186_044_416_005_859_375,
            0,
        ],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 1,
        phase_exponent: 12,
        reference_facts: [
            fact(
                true,
                4_000_000_000,
                0xc000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                2_000_000_000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [
            fact(
                false,
                17_592_186_044_416_001_953_125,
                0x40a0_0000_0000_0000,
                0x3d50_0000_0000_0000,
                0x3fe0_0000_0000_0000,
                0x3c90_0000_0000_0000,
            ),
            fact(
                false,
                17_592_186_044_416_005_859_375,
                0x40a0_0000_0000_0002,
                0x3d50_0000_0000_0000,
                0x3fe0_0000_0000_0002,
                0x3c90_0000_0000_0000,
            ),
            ZERO,
        ],
    },
    FrozenCase {
        name: "negative_even_odd_phase_ties",
        references: [(0, false), (1_000_000_000, false), (2_000_000_000, false)],
        phases: [
            -17_592_186_044_416_001_953_125,
            -17_592_186_044_416_005_859_375,
            0,
        ],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 1,
        phase_exponent: 12,
        reference_facts: [
            fact(
                true,
                4_000_000_000,
                0xc000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                2_000_000_000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [
            fact(
                true,
                17_592_186_044_416_001_953_125,
                0xc0a0_0000_0000_0000,
                0x3d50_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x3c90_0000_0000_0000,
            ),
            fact(
                true,
                17_592_186_044_416_005_859_375,
                0xc0a0_0000_0000_0002,
                0x3d50_0000_0000_0000,
                0xbfe0_0000_0000_0002,
                0x3c90_0000_0000_0000,
            ),
            ZERO,
        ],
    },
    FrozenCase {
        name: "phase_tie_carries_binade",
        references: [(0, false), (1_000_000_000, false), (2_000_000_000, false)],
        phases: [35_184_372_088_831_998_046_875, 0, 0],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 1,
        phase_exponent: 12,
        reference_facts: [
            fact(
                true,
                4_000_000_000,
                0xc000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                2_000_000_000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [
            fact(
                false,
                35_184_372_088_831_998_046_875,
                0x40b0_0000_0000_0000,
                0x3d50_0000_0000_0000,
                0x3ff0_0000_0000_0000,
                0x3c90_0000_0000_0000,
            ),
            ZERO,
            ZERO,
        ],
    },
    FrozenCase {
        name: "huge_absolute_phases_tiny_exact_deltas",
        references: [(0, false), (1_000_000_000, false), (2_000_000_000, false)],
        phases: [
            170_141_183_460_469_231_731_687_303_715_884_105_725,
            170_141_183_460_469_231_731_687_303_715_884_105_726,
            i128::MAX,
        ],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 1,
        phase_exponent: -61,
        reference_facts: [
            fact(
                true,
                4_000_000_000,
                0xc000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                2_000_000_000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [
            fact(
                true,
                2,
                0xbc11_2e0b_e826_d695,
                0x38c0_0000_0000_0000,
                0xbfe1_2e0b_e826_d695,
                0x3c90_0000_0000_0000,
            ),
            fact(
                true,
                1,
                0xbc01_2e0b_e826_d695,
                0x38b0_0000_0000_0000,
                0xbfd1_2e0b_e826_d695,
                0x3c80_0000_0000_0000,
            ),
            ZERO,
        ],
    },
    FrozenCase {
        name: "negative_huge_absolute_phases_tiny_exact_deltas",
        references: [(0, false), (1_000_000_000, false), (2_000_000_000, false)],
        phases: [
            -170_141_183_460_469_231_731_687_303_715_884_105_726,
            -170_141_183_460_469_231_731_687_303_715_884_105_727,
            i128::MIN,
        ],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 1,
        phase_exponent: -61,
        reference_facts: [
            fact(
                true,
                4_000_000_000,
                0xc000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                2_000_000_000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [
            fact(
                false,
                2,
                0x3c11_2e0b_e826_d695,
                0x38c0_0000_0000_0000,
                0x3fe1_2e0b_e826_d695,
                0x3c90_0000_0000_0000,
            ),
            fact(
                false,
                1,
                0x3c01_2e0b_e826_d695,
                0x38b0_0000_0000_0000,
                0x3fd1_2e0b_e826_d695,
                0x3c80_0000_0000_0000,
            ),
            ZERO,
        ],
    },
    FrozenCase {
        name: "time_scale_boundary_-10_-1",
        references: [(0, false), (488_281, false), (976_562, false)],
        phases: [0, 0, 0],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: -10,
        phase_exponent: 0,
        reference_facts: [
            fact(
                true,
                1_953_124,
                0xbf4f_fffe_ed1f_417e,
                0x3bf0_0000_0000_0000,
                0xbfef_fffe_ed1f_417e,
                0x3c90_0000_0000_0000,
            ),
            fact(
                true,
                976_562,
                0xbf3f_fffe_ed1f_417e,
                0x3be0_0000_0000_0000,
                0xbfdf_fffe_ed1f_417e,
                0x3c80_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [ZERO, ZERO, ZERO],
    },
    FrozenCase {
        name: "time_scale_boundary_-10_+0",
        references: [(0, false), (488_281, false), (976_562, true)],
        phases: [0, 0, 0],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: -10,
        phase_exponent: 0,
        reference_facts: [
            fact(
                true,
                1_953_125,
                0xbf50_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                976_563,
                0xbf40_0000_8970_5f41,
                0x3bf0_0000_0000_0000,
                0xbfe0_0000_8970_5f41,
                0x3c90_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [ZERO, ZERO, ZERO],
    },
    FrozenCase {
        name: "time_scale_boundary_-10_+1",
        references: [(0, false), (488_281, true), (976_563, false)],
        phases: [0, 0, 0],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: -9,
        phase_exponent: 0,
        reference_facts: [
            fact(
                true,
                1_953_126,
                0xbf50_0000_8970_5f41,
                0x3c00_0000_0000_0000,
                0xbfe0_0000_8970_5f41,
                0x3c90_0000_0000_0000,
            ),
            fact(
                true,
                976_563,
                0xbf40_0000_8970_5f41,
                0x3bf0_0000_0000_0000,
                0xbfd0_0000_8970_5f41,
                0x3c80_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [ZERO, ZERO, ZERO],
    },
    FrozenCase {
        name: "time_scale_boundary_0_-1",
        references: [(0, false), (499_999_999, true), (999_999_999, true)],
        phases: [0, 0, 0],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 0,
        phase_exponent: 0,
        reference_facts: [
            fact(
                true,
                1_999_999_999,
                0xbfef_ffff_ffbb_47d0,
                0x3c90_0000_0000_0000,
                0xbfef_ffff_ffbb_47d0,
                0x3c90_0000_0000_0000,
            ),
            fact(
                true,
                1_000_000_000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [ZERO, ZERO, ZERO],
    },
    FrozenCase {
        name: "time_scale_boundary_0_+0",
        references: [(0, false), (500_000_000, false), (1_000_000_000, false)],
        phases: [0, 0, 0],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 0,
        phase_exponent: 0,
        reference_facts: [
            fact(
                true,
                2_000_000_000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                1_000_000_000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [ZERO, ZERO, ZERO],
    },
    FrozenCase {
        name: "time_scale_boundary_0_+1",
        references: [(0, false), (500_000_000, false), (1_000_000_000, true)],
        phases: [0, 0, 0],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 1,
        phase_exponent: 0,
        reference_facts: [
            fact(
                true,
                2_000_000_001,
                0xbff0_0000_0022_5c18,
                0x3ca0_0000_0000_0000,
                0xbfe0_0000_0022_5c18,
                0x3c90_0000_0000_0000,
            ),
            fact(
                true,
                1_000_000_001,
                0xbfe0_0000_0044_b830,
                0x3c90_0000_0000_0000,
                0xbfd0_0000_0044_b830,
                0x3c80_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [ZERO, ZERO, ZERO],
    },
    FrozenCase {
        name: "time_scale_boundary_34_-1",
        references: [
            (0, false),
            (8_589_934_591_999_999_999, true),
            (17_179_869_183_999_999_999, true),
        ],
        phases: [0, 0, 0],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 34,
        phase_exponent: 0,
        reference_facts: [
            fact(
                true,
                34_359_738_367_999_999_999,
                0xc210_0000_0000_0000,
                0x3eb0_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x3c90_0000_0000_0000,
            ),
            fact(
                true,
                17_179_869_184_000_000_000,
                0xc200_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [ZERO, ZERO, ZERO],
    },
    FrozenCase {
        name: "time_scale_boundary_34_+0",
        references: [
            (0, false),
            (8_589_934_592_000_000_000, false),
            (17_179_869_184_000_000_000, false),
        ],
        phases: [0, 0, 0],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 34,
        phase_exponent: 0,
        reference_facts: [
            fact(
                true,
                34_359_738_368_000_000_000,
                0xc210_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                17_179_869_184_000_000_000,
                0xc200_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [ZERO, ZERO, ZERO],
    },
    FrozenCase {
        name: "time_scale_boundary_34_+1",
        references: [
            (0, false),
            (8_589_934_592_000_000_000, false),
            (17_179_869_184_000_000_000, true),
        ],
        phases: [0, 0, 0],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 35,
        phase_exponent: 0,
        reference_facts: [
            fact(
                true,
                34_359_738_368_000_000_001,
                0xc210_0000_0000_0000,
                0x3ec0_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x3c90_0000_0000_0000,
            ),
            fact(
                true,
                17_179_869_184_000_000_001,
                0xc200_0000_0000_0000,
                0x3eb0_0000_0000_0000,
                0xbfd0_0000_0000_0000,
                0x3c80_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [ZERO, ZERO, ZERO],
    },
    FrozenCase {
        name: "phase_scale_boundary_-42_-1",
        references: [(0, false), (1_000_000_000, false), (2_000_000_000, false)],
        phases: [1_953_124, 0, 0],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 1,
        phase_exponent: -42,
        reference_facts: [
            fact(
                true,
                4_000_000_000,
                0xc000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                2_000_000_000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [
            fact(
                false,
                1_953_124,
                0x3d4f_fffe_ed1f_417e,
                0x39f0_0000_0000_0000,
                0x3fef_fffe_ed1f_417e,
                0x3c90_0000_0000_0000,
            ),
            ZERO,
            ZERO,
        ],
    },
    FrozenCase {
        name: "phase_scale_boundary_-42_+0",
        references: [(0, false), (1_000_000_000, false), (2_000_000_000, false)],
        phases: [1_953_125, 0, 0],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 1,
        phase_exponent: -42,
        reference_facts: [
            fact(
                true,
                4_000_000_000,
                0xc000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                2_000_000_000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [
            fact(
                false,
                1_953_125,
                0x3d50_0000_0000_0000,
                0x0000_0000_0000_0000,
                0x3ff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
            ZERO,
        ],
    },
    FrozenCase {
        name: "phase_scale_boundary_-42_+1",
        references: [(0, false), (1_000_000_000, false), (2_000_000_000, false)],
        phases: [1_953_126, 0, 0],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 1,
        phase_exponent: -41,
        reference_facts: [
            fact(
                true,
                4_000_000_000,
                0xc000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                2_000_000_000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [
            fact(
                false,
                1_953_126,
                0x3d50_0000_8970_5f41,
                0x3a00_0000_0000_0000,
                0x3fe0_0000_8970_5f41,
                0x3c90_0000_0000_0000,
            ),
            ZERO,
            ZERO,
        ],
    },
    FrozenCase {
        name: "phase_scale_boundary_0_-1",
        references: [(0, false), (1_000_000_000, false), (2_000_000_000, false)],
        phases: [8_589_934_591_999_999_999, 0, 0],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 1,
        phase_exponent: 0,
        reference_facts: [
            fact(
                true,
                4_000_000_000,
                0xc000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                2_000_000_000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [
            fact(
                false,
                8_589_934_591_999_999_999,
                0x3ff0_0000_0000_0000,
                0x3c90_0000_0000_0000,
                0x3ff0_0000_0000_0000,
                0x3c90_0000_0000_0000,
            ),
            ZERO,
            ZERO,
        ],
    },
    FrozenCase {
        name: "phase_scale_boundary_0_+0",
        references: [(0, false), (1_000_000_000, false), (2_000_000_000, false)],
        phases: [8_589_934_592_000_000_000, 0, 0],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 1,
        phase_exponent: 0,
        reference_facts: [
            fact(
                true,
                4_000_000_000,
                0xc000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                2_000_000_000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [
            fact(
                false,
                8_589_934_592_000_000_000,
                0x3ff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0x3ff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
            ZERO,
        ],
    },
    FrozenCase {
        name: "phase_scale_boundary_0_+1",
        references: [(0, false), (1_000_000_000, false), (2_000_000_000, false)],
        phases: [8_589_934_592_000_000_001, 0, 0],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 1,
        phase_exponent: 1,
        reference_facts: [
            fact(
                true,
                4_000_000_000,
                0xc000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                2_000_000_000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [
            fact(
                false,
                8_589_934_592_000_000_001,
                0x3ff0_0000_0000_0000,
                0x3ca0_0000_0000_0000,
                0x3fe0_0000_0000_0000,
                0x3c90_0000_0000_0000,
            ),
            ZERO,
            ZERO,
        ],
    },
    FrozenCase {
        name: "phase_scale_boundary_65_-1",
        references: [(0, false), (1_000_000_000, false), (2_000_000_000, false)],
        phases: [
            146_771_466_596_588_118_642_488_497_628_115_894_271,
            i128::MIN,
            i128::MIN,
        ],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 1,
        phase_exponent: 65,
        reference_facts: [
            fact(
                true,
                4_000_000_000,
                0xc000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                2_000_000_000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [
            fact(
                false,
                316_912_650_057_057_350_374_175_801_343_999_999_999,
                0x4400_0000_0000_0000,
                0x40a0_0000_0000_0000,
                0x3ff0_0000_0000_0000,
                0x3c90_0000_0000_0000,
            ),
            ZERO,
            ZERO,
        ],
    },
    FrozenCase {
        name: "phase_scale_boundary_65_+0",
        references: [(0, false), (1_000_000_000, false), (2_000_000_000, false)],
        phases: [
            146_771_466_596_588_118_642_488_497_628_115_894_272,
            i128::MIN,
            i128::MIN,
        ],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 1,
        phase_exponent: 65,
        reference_facts: [
            fact(
                true,
                4_000_000_000,
                0xc000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                2_000_000_000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [
            fact(
                false,
                316_912_650_057_057_350_374_175_801_344_000_000_000,
                0x4400_0000_0000_0000,
                0x0000_0000_0000_0000,
                0x3ff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
            ZERO,
        ],
    },
    FrozenCase {
        name: "phase_scale_boundary_65_+1",
        references: [(0, false), (1_000_000_000, false), (2_000_000_000, false)],
        phases: [
            146_771_466_596_588_118_642_488_497_628_115_894_273,
            i128::MIN,
            i128::MIN,
        ],
        distances: [1.0; 3],
        divisors: [1.0; 3],
        time_exponent: 1,
        phase_exponent: 66,
        reference_facts: [
            fact(
                true,
                4_000_000_000,
                0xc000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            fact(
                true,
                2_000_000_000,
                0xbff0_0000_0000_0000,
                0x0000_0000_0000_0000,
                0xbfe0_0000_0000_0000,
                0x0000_0000_0000_0000,
            ),
            ZERO,
        ],
        phase_facts: [
            fact(
                false,
                316_912_650_057_057_350_374_175_801_344_000_000_001,
                0x4400_0000_0000_0000,
                0x40b0_0000_0000_0000,
                0x3fe0_0000_0000_0000,
                0x3c90_0000_0000_0000,
            ),
            ZERO,
            ZERO,
        ],
    },
];

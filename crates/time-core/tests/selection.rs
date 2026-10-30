// SPDX-License-Identifier: GPL-2.0-only
//! Independent, bounded external consumers of the S1.3 synthetic fixture API.
//!
//! Literal expected facts describe closed real intervals, not integer-lattice
//! adjacency, a production source identity, a clock coordinate or a trust vote.
//! The four test scopes emit their actual precharged public-entry prefixes.
//! No result or execution claim is implied by these authored expectations.
#![forbid(unsafe_code)]

use std::{
    cell::Cell,
    error::Error,
    io::{self, Write},
    num::NonZeroU64,
};

use time_core::{
    BatchError, FixtureBatch, FixtureInterval, FixtureMinimum, FixtureOutcomeKind,
    FixtureSelection, IntervalError, MinimumError, SelectionError, select_fixture,
};

/// Local test failure or a value produced without bypassing an admission error.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Raw fixture label and closed real interval endpoints on the synthetic axis.
type Row = (u64, i64, i64);

/// One scope's attempted public operations, charged before the actual call.
struct Profile {
    /// Fixed scope label used in the single drop-time receipt.
    scope: &'static str,
    /// Attempted interval constructions, charged before each call.
    intervals: Cell<usize>,
    /// Attempted minimum constructions, charged before each call.
    minima: Cell<usize>,
    /// Attempted batch constructions, charged before each call.
    batches: Cell<usize>,
    /// Attempted selections, charged before each call.
    selections: Cell<usize>,
}

impl Profile {
    /// Begin one bounded scope with no attempted public calls.
    const fn new(scope: &'static str) -> Self {
        Self {
            scope,
            intervals: Cell::new(0),
            minima: Cell::new(0),
            batches: Cell::new(0),
            selections: Cell::new(0),
        }
    }

    /// Charge and forward one raw interval to its public constructor.
    ///
    /// # Errors
    /// The outer result rejects a counter overflow or exhausted interval budget.
    /// The inner result preserves the public constructor's admission error.
    fn interval(&self, row: Row) -> TestResult<Result<FixtureInterval, IntervalError>> {
        charge(&self.intervals, 512)?;
        Ok(FixtureInterval::new(row.0, row.1, row.2))
    }

    /// Charge and forward one raw minimum to its public constructor.
    ///
    /// # Errors
    /// The outer result rejects a counter overflow or exhausted minimum budget.
    /// The inner result preserves the public constructor's admission error.
    fn minimum(&self, raw: usize) -> TestResult<Result<FixtureMinimum, MinimumError>> {
        charge(&self.minima, 70)?;
        Ok(FixtureMinimum::new(raw))
    }

    /// Charge and forward one checked slice to the public batch constructor.
    ///
    /// # Errors
    /// The outer result rejects a counter overflow or exhausted batch budget.
    /// The inner result preserves the public receiver's admission error.
    fn batch(&self, rows: &[FixtureInterval]) -> TestResult<Result<FixtureBatch, BatchError>> {
        charge(&self.batches, 70)?;
        Ok(FixtureBatch::new(rows))
    }

    /// Charge and evaluate one checked batch and minimum.
    ///
    /// # Errors
    /// The outer result rejects a counter overflow or exhausted selection budget.
    /// The inner result preserves the evaluator's checked-invariant error.
    fn select(
        &self,
        batch: &FixtureBatch,
        minimum: FixtureMinimum,
    ) -> TestResult<Result<FixtureSelection, SelectionError>> {
        charge(&self.selections, 68)?;
        Ok(select_fixture(batch, minimum))
    }
}

impl Drop for Profile {
    fn drop(&mut self) {
        let record = format!(
            "S13_SELECTION_PROFILE scope={} intervals={} minima={} batches={} selections={}\n",
            self.scope,
            self.intervals.get(),
            self.minima.get(),
            self.batches.get(),
            self.selections.get()
        );
        let _emission_result = io::stderr().lock().write_all(record.as_bytes());
    }
}

/// Checked local ceiling; a rejected charge does not invoke the product owner.
///
/// # Errors
/// Rejects overflow or a next count above the ceiling without changing the count.
fn charge(counter: &Cell<usize>, ceiling: usize) -> TestResult {
    let next = counter
        .get()
        .checked_add(1)
        .ok_or_else(|| io::Error::other("selection profile overflow"))?;
    if next > ceiling {
        return Err(io::Error::other("selection profile ceiling").into());
    }
    counter.set(next);
    Ok(())
}

/// Construct each admitted record through the real public receiving path.
///
/// # Errors
/// Propagates an exhausted call budget or interval/batch admission failure.
fn make_batch(profile: &Profile, rows: &[Row]) -> TestResult<FixtureBatch> {
    let mut checked = Vec::with_capacity(rows.len());
    for row in rows {
        checked.push(profile.interval(*row)??);
    }
    Ok(profile.batch(&checked)??)
}

/// Static independent expectation; no field comes from the returned result.
#[derive(Clone, Copy)]
struct Expected {
    /// Number of distinct fixture intervals in the input.
    population: usize,
    /// Independently configured agreement minimum.
    minimum: usize,
    /// Maximum overlap depth anywhere on the real synthetic axis.
    depth: usize,
    /// Whether maximum depth is a strict majority of the population.
    majority: bool,
    /// Whether maximum depth meets the configured minimum.
    minimum_met: bool,
    /// Expected fixture outcome after the declared precedence rules.
    outcome: FixtureOutcomeKind,
    /// Maximal component count when the outcome reports ambiguity.
    component_count: Option<usize>,
    /// Unique closed component endpoints and ordered supporting labels, if witnessed.
    witness: Option<(i64, i64, &'static [u64])>,
}

/// Assemble manually stated expected facts without consulting selection output.
macro_rules! expected {
    ($n:expr, $min:expr, $depth:expr, $majority:expr, $met:expr, $outcome:ident, $count:expr, $witness:expr) => {
        Expected {
            population: $n,
            minimum: $min,
            depth: $depth,
            majority: $majority,
            minimum_met: $met,
            outcome: FixtureOutcomeKind::$outcome,
            component_count: $count,
            witness: $witness,
        }
    };
}

/// Compare every exposed fixture result against the independent literal facts.
///
/// # Errors
/// Rejects a missing witness when the independent expectation requires one.
///
/// # Panics
/// Panics if any observed result field or supporter differs from its literal fact.
fn check_result(result: &FixtureSelection, expected: Expected) -> TestResult {
    assert_eq!(result.population(), expected.population);
    assert_eq!(result.minimum(), expected.minimum);
    assert_eq!(result.maximum_depth(), expected.depth);
    assert_eq!(result.strict_majority(), expected.majority);
    assert_eq!(result.minimum_met(), expected.minimum_met);
    assert_eq!(result.outcome(), expected.outcome);
    assert_eq!(result.component_count(), expected.component_count);
    match expected.witness {
        Some((lo, hi, labels)) => {
            let witness = result
                .witnessed()
                .ok_or_else(|| io::Error::other("missing fixture witness"))?;
            assert_eq!(
                (witness.component_lower(), witness.component_upper()),
                (lo, hi)
            );
            assert_eq!(witness.witness(), lo);
            assert_eq!(
                (witness.intersection_lower(), witness.intersection_upper()),
                (lo, hi)
            );
            let supporters = witness.supporters();
            assert_eq!(supporters.len(), labels.len());
            let actual: Vec<_> = supporters.map(NonZeroU64::get).collect();
            assert_eq!(actual.as_slice(), labels);
        }
        None => assert!(result.witnessed().is_none()),
    }
    Ok(())
}

/// Execute one precharged literal, checking input preservation and result facts.
///
/// # Errors
/// Propagates call-budget, fixture admission, evaluator or required-witness errors.
///
/// # Panics
/// Panics if a result or preserved input differs from the independent expectation.
fn evaluate(profile: &Profile, rows: &[Row], expected: Expected) -> TestResult {
    let batch = make_batch(profile, rows)?;
    let minimum = profile.minimum(expected.minimum)??;
    assert_eq!(minimum.get(), expected.minimum);
    let before: Vec<Row> = batch
        .iter()
        .map(|value| (value.label().get(), value.lower(), value.upper()))
        .collect();
    assert_eq!(before.as_slice(), rows);
    let result = profile.select(&batch, minimum)??;
    check_result(&result, expected)?;
    assert_eq!(batch.len(), expected.population);
    assert_eq!(batch.is_empty(), expected.population == 0);
    let after: Vec<Row> = batch
        .iter()
        .map(|value| (value.label().get(), value.lower(), value.upper()))
        .collect();
    assert_eq!(after, before);
    Ok(())
}

/// Raw interval admission, error precedence and valid boundary neighbors.
///
/// # Errors
/// Propagates a charge failure or unexpected rejection of a valid neighbor.
///
/// # Panics
/// Panics on an incorrect rejection cause, accepted value or attempt count.
#[test]
fn interval_construction_rejects_invalid_raw_values_and_accepts_neighbors() -> TestResult {
    let profile = Profile::new("interval");
    assert!(matches!(
        profile.interval((0, 0, 1))?,
        Err(IntervalError::ZeroLabel)
    ));
    assert!(matches!(
        profile.interval((1, 4, 3))?,
        Err(IntervalError::ReversedBounds)
    ));
    assert!(matches!(
        profile.interval((1, i64::MAX, i64::MIN))?,
        Err(IntervalError::ReversedBounds)
    ));
    assert!(matches!(
        profile.interval((0, 4, 3))?,
        Err(IntervalError::ZeroLabel)
    ));
    for row in [
        (1, 4, 4),
        (2, 3, 4),
        (3, i64::MIN, i64::MAX),
        (u64::MAX, 7, 7),
    ] {
        let interval = profile.interval(row)??;
        assert_eq!(
            (interval.label().get(), interval.lower(), interval.upper()),
            row
        );
    }
    assert_eq!(profile.intervals.get(), 8);
    Ok(())
}

/// The independently checked minimum is neither zero nor above capacity.
///
/// # Errors
/// Propagates a charge failure or unexpected rejection of a valid minimum.
///
/// # Panics
/// Panics on an incorrect rejection cause, accepted minimum or attempt count.
#[test]
fn minimum_construction_preserves_valid_values() -> TestResult {
    let profile = Profile::new("minimum");
    assert!(matches!(profile.minimum(0)?, Err(MinimumError::OutOfRange)));
    assert!(matches!(
        profile.minimum(17)?,
        Err(MinimumError::OutOfRange)
    ));
    assert!(matches!(
        profile.minimum(usize::MAX)?,
        Err(MinimumError::OutOfRange)
    ));
    for value in [1, 16, 3] {
        assert_eq!(profile.minimum(value)??.get(), value);
    }
    assert_eq!(profile.minima.get(), 6);
    Ok(())
}

/// Invalid batches do not mutate a previously admitted neighboring batch.
///
/// # Errors
/// Propagates call-budget, valid fixture-construction, evaluator or witness errors.
///
/// # Panics
/// Panics on incorrect admission, duplicate order, result facts or preserved input.
#[test]
fn batch_admission_rejects_duplicate_and_capacity_before_recovery() -> TestResult {
    let profile = Profile::new("batch");
    let a = profile.interval((1, 0, 2))??;
    let b = profile.interval((2, 0, 2))??;
    let a_other = profile.interval((1, 9, 10))??;
    let healthy = profile.batch(&[a, b])??;
    let before: Vec<_> = healthy.iter().map(|value| value.label().get()).collect();
    assert_eq!(before, [1, 2]);
    assert!(matches!(
        profile.batch(&[a, b, a])?,
        Err(BatchError::DuplicateLabel { later_index: 2 })
    ));
    assert!(matches!(
        profile.batch(&[a, b, b, a])?,
        Err(BatchError::DuplicateLabel { later_index: 2 })
    ));
    assert!(matches!(
        profile.batch(&[a, a, b])?,
        Err(BatchError::DuplicateLabel { later_index: 1 })
    ));
    assert!(matches!(
        profile.batch(&[a, a_other])?,
        Err(BatchError::DuplicateLabel { later_index: 1 })
    ));
    assert!(matches!(
        profile.batch(&[a; 17])?,
        Err(BatchError::CapacityExceeded)
    ));
    let mut distinct = Vec::with_capacity(16);
    for label in 1_u64..=16 {
        distinct.push(profile.interval((label, 0, 1))??);
    }
    assert_eq!(profile.batch(&distinct[..15])??.len(), 15);
    assert_eq!(profile.batch(&distinct)??.len(), 16);
    assert_eq!(
        healthy
            .iter()
            .map(|value| value.label().get())
            .collect::<Vec<_>>(),
        before
    );
    let minimum = profile.minimum(2)??;
    let selected = profile.select(&healthy, minimum)??;
    check_result(
        &selected,
        expected!(
            2,
            2,
            2,
            true,
            true,
            WitnessedFixture,
            None,
            Some((0, 2, &[1, 2]))
        ),
    )?;
    assert_eq!(
        healthy
            .iter()
            .map(|value| value.label().get())
            .collect::<Vec<_>>(),
        before
    );
    assert_eq!(
        (
            profile.intervals.get(),
            profile.minima.get(),
            profile.batches.get(),
            profile.selections.get()
        ),
        (19, 1, 8, 1)
    );
    Ok(())
}

const UNIQUE: [Row; 3] = [(1, -4, 5), (2, -2, 3), (3, 0, 6)];
const CONTACT: [Row; 3] = [(1, 0, 2), (2, 0, 4), (3, 2, 4)];
const BRIDGE: [Row; 3] = [(1, 0, 10), (2, 1, 2), (3, 4, 5)];
const GAP: [Row; 3] = [(1, 0, 0), (2, 0, 1), (3, 1, 1)];
const MAXIMUM: [Row; 16] = [
    (1, 0, 1),
    (2, 0, 1),
    (3, 0, 1),
    (4, 0, 1),
    (5, 0, 1),
    (6, 0, 1),
    (7, 0, 1),
    (8, 0, 1),
    (9, 0, 1),
    (10, 10, 11),
    (11, 13, 14),
    (12, 16, 17),
    (13, 19, 20),
    (14, 22, 23),
    (15, 25, 26),
    (16, 28, 29),
];
const FIFTEEN: [Row; 15] = [
    (1, 0, 1),
    (2, 0, 1),
    (3, 0, 1),
    (4, 0, 1),
    (5, 0, 1),
    (6, 0, 1),
    (7, 0, 1),
    (8, 0, 1),
    (9, 10, 11),
    (10, 13, 14),
    (11, 16, 17),
    (12, 19, 20),
    (13, 22, 23),
    (14, 25, 26),
    (15, 28, 29),
];

/// Fixed independent facts reused by the literal and permutation controls.
struct LiteralCases {
    /// Unique three-interval intersection.
    unique: Expected,
    /// Three closed intervals meeting at a point.
    contact: Expected,
    /// Two disconnected maximum-depth components.
    ambiguous: Expected,
    /// Majority in the sixteen-interval capacity case.
    maximum: Expected,
    /// The ordered literal inventory and its expected outputs.
    cases: [(&'static [Row], Expected); 24],
}

/// The fixed table is data only; it performs no public fixture calls.
const LITERAL_CASES: LiteralCases = {
    let unique = expected!(
        3,
        3,
        3,
        true,
        true,
        WitnessedFixture,
        None,
        Some((0, 3, &[1, 2, 3]))
    );
    let contact = expected!(
        3,
        2,
        3,
        true,
        true,
        WitnessedFixture,
        None,
        Some((2, 2, &[1, 2, 3]))
    );
    let ambiguous = expected!(
        3,
        2,
        2,
        true,
        true,
        UnsupportedAmbiguousFixture,
        Some(2),
        None
    );
    let maximum = expected!(
        16,
        9,
        9,
        true,
        true,
        WitnessedFixture,
        None,
        Some((0, 1, &[1, 2, 3, 4, 5, 6, 7, 8, 9]))
    );
    let cases: [(&[Row], Expected); 24] = [
        (&[], expected!(0, 1, 0, false, false, Empty, None, None)),
        (
            &[(1, -3, 4)],
            expected!(
                1,
                1,
                1,
                true,
                true,
                WitnessedFixture,
                None,
                Some((-3, 4, &[1]))
            ),
        ),
        (&UNIQUE, unique),
        (
            &[(1, -4, 1), (2, 0, 3), (3, 7, 8)],
            expected!(
                3,
                2,
                2,
                true,
                true,
                WitnessedFixture,
                None,
                Some((0, 1, &[1, 2]))
            ),
        ),
        (
            &[(1, -4, 1), (2, 0, 3), (3, 7, 8)],
            expected!(3, 3, 2, true, false, InsufficientAgreement, None, None),
        ),
        (
            &[(1, 0, 1), (2, 2, 3)],
            expected!(2, 2, 1, false, false, NoMajority, None, None),
        ),
        (
            &[(1, -3, -1), (2, -3, -1), (3, 2, 4), (4, 2, 4)],
            expected!(4, 2, 2, false, true, NoMajority, None, None),
        ),
        (
            &BRIDGE,
            expected!(3, 3, 2, true, false, InsufficientAgreement, None, None),
        ),
        (&BRIDGE, ambiguous),
        (&CONTACT, contact),
        (&GAP, ambiguous),
        (
            &[(1, -2, 0), (2, 0, 7)],
            expected!(
                2,
                2,
                2,
                true,
                true,
                WitnessedFixture,
                None,
                Some((0, 0, &[1, 2]))
            ),
        ),
        (
            &[(1, 5, 5), (2, 5, 5)],
            expected!(
                2,
                2,
                2,
                true,
                true,
                WitnessedFixture,
                None,
                Some((5, 5, &[1, 2]))
            ),
        ),
        (
            &[(1, -1, 1), (2, 0, 2)],
            expected!(2, 3, 2, true, false, InsufficientPopulation, None, None),
        ),
        (
            &[(1, 0, 0), (2, 1, 1)],
            expected!(2, 3, 1, false, false, InsufficientPopulation, None, None),
        ),
        (
            &[
                (1, i64::MIN, i64::MAX),
                (2, i64::MIN, i64::MIN),
                (3, i64::MAX, i64::MAX),
            ],
            expected!(
                3,
                2,
                2,
                true,
                true,
                UnsupportedAmbiguousFixture,
                Some(2),
                None
            ),
        ),
        (
            &[(1, i64::MIN, i64::MAX), (2, i64::MIN, 0), (3, -1, i64::MAX)],
            expected!(
                3,
                3,
                3,
                true,
                true,
                WitnessedFixture,
                None,
                Some((-1, 0, &[1, 2, 3]))
            ),
        ),
        (&MAXIMUM, maximum),
        (
            &MAXIMUM,
            expected!(16, 10, 9, true, false, InsufficientAgreement, None, None),
        ),
        (
            &FIFTEEN,
            expected!(
                15,
                8,
                8,
                true,
                true,
                WitnessedFixture,
                None,
                Some((0, 1, &[1, 2, 3, 4, 5, 6, 7, 8]))
            ),
        ),
        (
            &[(1, 0, 4), (2, 0, 4), (3, 0, 0), (4, 2, 2), (5, 4, 4)],
            expected!(
                5,
                3,
                3,
                true,
                true,
                UnsupportedAmbiguousFixture,
                Some(3),
                None
            ),
        ),
        (
            &[
                (1, -16, 16),
                (2, -15, 15),
                (3, -14, 14),
                (4, -13, 13),
                (5, -12, 12),
                (6, -11, 11),
                (7, -10, 10),
                (8, -9, 9),
                (9, -8, 8),
                (10, -7, 7),
                (11, -6, 6),
                (12, -5, 5),
                (13, -4, 4),
                (14, -3, 3),
                (15, -2, 2),
                (16, -1, 1),
            ],
            expected!(
                16,
                16,
                16,
                true,
                true,
                WitnessedFixture,
                None,
                Some((
                    -1,
                    1,
                    &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
                ))
            ),
        ),
        (
            &[(u64::MAX, 0, 7), (2, 4, 10), (7, -10, -5), (4, 1, 8)],
            expected!(
                4,
                3,
                3,
                true,
                true,
                WitnessedFixture,
                None,
                Some((4, 7, &[2, 4, u64::MAX]))
            ),
        ),
        (
            &[(1, i64::MIN, i64::MAX)],
            expected!(
                1,
                1,
                1,
                true,
                true,
                WitnessedFixture,
                None,
                Some((i64::MIN, i64::MAX, &[1]))
            ),
        ),
    ];
    LiteralCases {
        unique,
        contact,
        ambiguous,
        maximum,
        cases,
    }
};

/// Literal cases and finite input permutations use independently fixed results.
///
/// # Errors
/// Propagates call-budget, fixture admission, evaluator or required-witness errors.
///
/// # Panics
/// Panics if a literal/permutation result, input preservation or total differs.
#[test]
fn interval_majority_literal_cases_and_permutations() -> TestResult {
    let profile = Profile::new("literal_permutations");
    let LiteralCases {
        unique,
        contact,
        ambiguous,
        maximum,
        cases,
    } = LITERAL_CASES;
    for (rows, expected) in cases {
        evaluate(&profile, rows, expected)?;
    }
    let permutations = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    for (rows, expected) in [
        (UNIQUE, unique),
        (CONTACT, contact),
        (BRIDGE, ambiguous),
        (GAP, ambiguous),
    ] {
        for order in permutations {
            evaluate(
                &profile,
                &[rows[order[0]], rows[order[1]], rows[order[2]]],
                expected,
            )?;
        }
    }
    for rotation in 0..16 {
        let mut rows = MAXIMUM;
        rows.rotate_left(rotation);
        evaluate(&profile, &rows, maximum)?;
    }
    evaluate(
        &profile,
        &[(1, 0, 2), (2, 1, 2), (3, 4, 5)],
        expected!(
            3,
            2,
            2,
            true,
            true,
            WitnessedFixture,
            None,
            Some((1, 2, &[1, 2]))
        ),
    )?;
    assert_eq!(
        (
            profile.intervals.get(),
            profile.minima.get(),
            profile.batches.get(),
            profile.selections.get()
        ),
        (446, 65, 65, 65)
    );
    Ok(())
}

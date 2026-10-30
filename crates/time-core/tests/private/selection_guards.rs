// SPDX-License-Identifier: GPL-2.0-only
//! Bounded observations of the actual batch receiver's private copy adapter.
//!
//! The observer counts adapter attempts only. Caller construction, array copies,
//! projections and reads elsewhere are outside its observation. It neither
//! validates a batch nor enforces a global campaign allowance.

use std::{
    cell::Cell,
    error::Error,
    io::{self, Write},
    num::NonZeroU64,
};

use super::{
    BATCH_READ_ATTEMPTS, BatchError, FixtureBatch, FixtureInterval, SelectionError,
    SelectionInvariantStage, cell_depth, read_batch_interval, sort_prefix, supporter_labels,
};

/// A local test or fixture-admission failure.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// The label and inclusive bounds exposed by a checked fixture interval.
type Projection = (u64, i64, i64);

/// Restore the previous thread-local observer state on every exit path.
struct ReadScope {
    /// The inactive state preserved on entry.
    previous: Option<usize>,
}

impl ReadScope {
    /// Activate an initially zero observer; nested observations are rejected.
    ///
    /// # Errors
    /// Rejects an unavailable thread-local observer or an already active scope.
    fn begin() -> TestResult<Self> {
        let previous = BATCH_READ_ATTEMPTS
            .try_with(|attempts| match attempts.get() {
                None => {
                    attempts.set(Some(0));
                    Ok(None)
                }
                Some(_) => Err(io::Error::other("nested batch read observer")),
            })
            .map_err(|_| io::Error::other("batch read observer unavailable"))??;
        Ok(Self { previous })
    }

    /// Read the active thread-local count while a separate RAII scope owns it.
    ///
    /// # Errors
    /// Rejects an inactive or unavailable thread-local observer.
    fn attempts() -> TestResult<usize> {
        Ok(BATCH_READ_ATTEMPTS
            .try_with(Cell::get)
            .map_err(|_| io::Error::other("batch read observer unavailable"))?
            .ok_or_else(|| io::Error::other("batch read observer inactive"))?)
    }
}

impl Drop for ReadScope {
    /// Restore the previous state even when a test unwinds.
    fn drop(&mut self) {
        let _restored = BATCH_READ_ATTEMPTS.try_with(|attempts| attempts.set(self.previous));
    }
}

/// Finite per-case public-call attempts, emitted once even if the case unwinds.
struct Calls {
    /// Fixed, payload-free test case identity.
    case: &'static str,
    /// Precharged public interval-construction attempts.
    intervals: Cell<usize>,
    /// Precharged public batch-construction attempts.
    batches: Cell<usize>,
}

impl Calls {
    /// Start a named scope with no public calls yet attempted.
    const fn new(case: &'static str) -> Self {
        Self {
            case,
            intervals: Cell::new(0),
            batches: Cell::new(0),
        }
    }

    /// Charge one checked interval construction before calling its public owner.
    ///
    /// # Errors
    /// Propagates a charge failure or the public interval constructor's rejection.
    fn interval(&self, label: u64, lo: i64, hi: i64) -> TestResult<FixtureInterval> {
        charge(&self.intervals, 16)?;
        Ok(FixtureInterval::new(label, lo, hi)?)
    }

    /// Charge one public batch construction before passing it the existing slice.
    ///
    /// # Errors
    /// The outer result rejects a counter overflow or exhausted batch budget.
    /// The inner result preserves the public receiver's admission error.
    fn batch(&self, values: &[FixtureInterval]) -> TestResult<Result<FixtureBatch, BatchError>> {
        charge(&self.batches, 2)?;
        Ok(FixtureBatch::new(values))
    }
}

impl Drop for Calls {
    /// Emit one bounded, case-labelled record of attempts, including on unwind.
    fn drop(&mut self) {
        let record = format!(
            "S13_BATCH_GUARD_PROFILE {{\"case\":\"{}\",\"intervals\":{},\"batches\":{}}}\n",
            self.case,
            self.intervals.get(),
            self.batches.get()
        );
        let _emitted = io::stderr().lock().write_all(record.as_bytes());
    }
}

/// Reject over-budget or overflowing charges before the corresponding public call.
///
/// # Errors
/// Rejects overflow or a next count above the limit without changing the count.
fn charge(counter: &Cell<usize>, limit: usize) -> TestResult {
    let next = counter
        .get()
        .checked_add(1)
        .ok_or_else(|| io::Error::other("batch guard count overflow"))?;
    if next > limit {
        return Err(io::Error::other("batch guard count ceiling").into());
    }
    counter.set(next);
    Ok(())
}

/// Project caller-owned checked values for exact preservation comparisons.
fn project(values: &[FixtureInterval]) -> Vec<Projection> {
    values
        .iter()
        .map(|value| (value.label().get(), value.lower(), value.upper()))
        .collect()
}

/// One directly attempted raw helper, excluding calls made inside that helper.
#[derive(Clone, Copy)]
enum HelperKind {
    /// Checked input-copy adapter.
    Read,
    /// Caller-owned prefix sorting.
    Sort,
    /// Checked point or open-span depth.
    Depth,
    /// Raw optional-label prefix projection.
    Labels,
}

/// Finite direct-helper attempts, independent of public construction counters.
struct HelperCalls {
    /// Direct input-copy adapter attempts, at most two.
    reads: Cell<usize>,
    /// Direct prefix-sort attempts, at most three.
    sorts: Cell<usize>,
    /// Direct cell-depth attempts, at most two.
    depths: Cell<usize>,
    /// Direct optional-label projections, at most five.
    labels: Cell<usize>,
}

impl HelperCalls {
    /// Begin one fixed helper recipe without charging a public constructor.
    const fn new() -> Self {
        Self {
            reads: Cell::new(0),
            sorts: Cell::new(0),
            depths: Cell::new(0),
            labels: Cell::new(0),
        }
    }

    /// Charge the next direct attempt before entering the selected helper.
    ///
    /// # Errors
    /// Rejects overflow or a charge beyond that helper's complete recipe.
    fn charge(&self, kind: HelperKind) -> TestResult {
        let (counter, limit) = match kind {
            HelperKind::Read => (&self.reads, 2),
            HelperKind::Sort => (&self.sorts, 3),
            HelperKind::Depth => (&self.depths, 2),
            HelperKind::Labels => (&self.labels, 5),
        };
        charge(counter, limit)
    }

    /// Copy the four direct-attempt totals, without claiming internal operations.
    fn totals(&self) -> (usize, usize, usize, usize) {
        (
            self.reads.get(),
            self.sorts.get(),
            self.depths.get(),
            self.labels.get(),
        )
    }
}

impl Drop for HelperCalls {
    /// Emit one complete native line even when a reached assertion unwinds.
    fn drop(&mut self) {
        let record = format!(
            "SELECTION_HELPER_PROFILE read_adapters={} prefix_sorts={} cell_depths={} label_projections={}\n",
            self.reads.get(),
            self.sorts.get(),
            self.depths.get(),
            self.labels.get()
        );
        let _emitted = io::stderr().lock().write_all(record.as_bytes());
    }
}

/// Challenge the raw input-copy adapter after the caller's observer is restored.
///
/// # Errors
/// Propagates a charge or checked-access error on the healthy path.
///
/// # Panics
/// Panics on an incorrect missing-index result or valid neighboring projection.
fn raw_read_contract(values: &[FixtureInterval], calls: &HelperCalls) -> TestResult {
    calls.charge(HelperKind::Read)?;
    assert!(matches!(
        read_batch_interval(values, values.len()),
        Err(BatchError::Invariant)
    ));
    calls.charge(HelperKind::Read)?;
    let first = read_batch_interval(values, 0)?;
    assert_eq!(
        (first.label().get(), first.lower(), first.upper()),
        (1, -1, 1)
    );
    Ok(())
}

/// Check oversized rejection and exact empty/full-size sorting neighbors.
///
/// # Errors
/// Propagates a charge or admitted-sort error.
///
/// # Panics
/// Panics on wrong rejection, modified rejected input or incorrect sorted output.
fn raw_sort_contract(calls: &HelperCalls) -> TestResult {
    let mut oversized = [
        33_i64, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12,
        11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1,
    ];
    let before = oversized;
    calls.charge(HelperKind::Sort)?;
    assert!(matches!(
        sort_prefix(&mut oversized),
        Err(SelectionError::Invariant {
            stage: SelectionInvariantStage::Slot
        })
    ));
    assert_eq!(oversized, before);
    let mut empty: [i64; 0] = [];
    calls.charge(HelperKind::Sort)?;
    sort_prefix(&mut empty)?;
    assert_eq!(empty, []);
    let mut full = [
        32_i64, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11,
        10, 9, 8, 7, 6, 5, 4, 3, 2, 1,
    ];
    calls.charge(HelperKind::Sort)?;
    sort_prefix(&mut full)?;
    assert_eq!(
        full,
        [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32
        ]
    );
    Ok(())
}

/// Check reversed raw cell bounds beside a known sixteen-row point depth.
///
/// # Errors
/// Propagates a charge or admitted cell-depth error.
///
/// # Panics
/// Panics on an incorrect reversed-bound stage or depth at the shared zero point.
fn raw_depth_contract(batch: &FixtureBatch, calls: &HelperCalls) -> TestResult {
    calls.charge(HelperKind::Depth)?;
    assert!(matches!(
        cell_depth(batch, 1, 0),
        Err(SelectionError::Invariant {
            stage: SelectionInvariantStage::Component
        })
    ));
    calls.charge(HelperKind::Depth)?;
    assert_eq!(cell_depth(batch, 0, 0)?, 16);
    Ok(())
}

/// Require the raw label boundary's declared rejection stage.
///
/// # Panics
/// Panics if a rejected raw prefix succeeds or reports a different stage.
fn require_support_error(result: Result<[NonZeroU64; 16], SelectionError>) {
    assert!(matches!(
        result,
        Err(SelectionError::Invariant {
            stage: SelectionInvariantStage::Support
        })
    ));
}

/// Check raw optional-label guards using labels from the already checked input.
///
/// # Errors
/// Propagates charge, checked input access or the admitted projection's error.
///
/// # Panics
/// Panics on an incorrect support error, sorted prefix, padding or preserved stage.
fn raw_label_contract(values: &[FixtureInterval], calls: &HelperCalls) -> TestResult {
    let first = values
        .get(2)
        .ok_or_else(|| io::Error::other("raw helper input width"))?
        .label();
    let second = values
        .first()
        .ok_or_else(|| io::Error::other("raw helper input width"))?
        .label();
    let third = values
        .get(1)
        .ok_or_else(|| io::Error::other("raw helper input width"))?
        .label();
    let mut staged = [None; 16];
    for (slot, value) in staged.iter_mut().zip([first, second, third]) {
        *slot = Some(value);
    }
    let before = staged;
    calls.charge(HelperKind::Labels)?;
    require_support_error(supporter_labels(&staged, 0));
    calls.charge(HelperKind::Labels)?;
    require_support_error(supporter_labels(&staged, 17));
    let missing_first = [None; 16];
    calls.charge(HelperKind::Labels)?;
    require_support_error(supporter_labels(&missing_first, 1));
    let mut missing_middle = staged;
    *missing_middle
        .get_mut(1)
        .ok_or_else(|| io::Error::other("raw helper stage width"))? = None;
    let missing_before = missing_middle;
    calls.charge(HelperKind::Labels)?;
    require_support_error(supporter_labels(&missing_middle, 3));
    calls.charge(HelperKind::Labels)?;
    let labels = supporter_labels(&staged, 3)?;
    let prefix: Vec<_> = labels
        .get(..3)
        .ok_or_else(|| io::Error::other("raw helper output width"))?
        .iter()
        .copied()
        .map(NonZeroU64::get)
        .collect();
    assert_eq!(prefix, [1, 2, 3]);
    assert!(
        labels
            .get(3..)
            .ok_or_else(|| io::Error::other("raw helper output width"))?
            .iter()
            .all(|label| label.get() == 3)
    );
    assert_eq!(staged, before);
    assert_eq!(missing_first, [None; 16]);
    assert_eq!(missing_middle, missing_before);
    Ok(())
}

/// Exercise wider raw signatures without forging a checked scratch carrier.
///
/// The existing capacity neighbor supplies sixteen checked values and its real
/// immutable batch. Direct helpers add no public construction/evaluation calls.
/// The earlier read observer must already be inactive, and stays inactive.
///
/// # Errors
/// Rejects wrong fixture width, unavailable observer, charge or healthy helper error.
///
/// # Panics
/// Panics on a helper-contract failure, changed input or unexpected observer state.
fn raw_helper_contracts(values: &[FixtureInterval], batch: &FixtureBatch) -> TestResult {
    if values.len() != 16 || batch.len() != 16 {
        return Err(io::Error::other("raw helper fixture width").into());
    }
    let observer = BATCH_READ_ATTEMPTS
        .try_with(Cell::get)
        .map_err(|_| io::Error::other("batch read observer unavailable"))?;
    assert_eq!(observer, None);
    let before = project(values);
    let batch_before: Vec<_> = batch
        .iter()
        .map(|value| (value.label().get(), value.lower(), value.upper()))
        .collect();
    let calls = HelperCalls::new();
    raw_read_contract(values, &calls)?;
    raw_sort_contract(&calls)?;
    raw_depth_contract(batch, &calls)?;
    raw_label_contract(values, &calls)?;
    assert_eq!(calls.totals(), (2, 3, 2, 5));
    assert_eq!(project(values), before);
    assert_eq!(
        batch
            .iter()
            .map(|value| (value.label().get(), value.lower(), value.upper()))
            .collect::<Vec<_>>(),
        batch_before
    );
    assert_eq!(
        BATCH_READ_ATTEMPTS
            .try_with(Cell::get)
            .map_err(|_| io::Error::other("batch read observer unavailable"))?,
        None
    );
    Ok(())
}

/// Capacity rejection occurs before any copy-adapter attempt.
///
/// # Errors
/// Propagates charge, checked interval-construction or observer errors.
///
/// # Panics
/// Panics on an incorrect capacity result, read count, preserved input or total.
#[test]
fn over_capacity_rejects_before_adapter_reads() -> TestResult {
    let calls = Calls::new("capacity");
    let value = calls.interval(1, -2, 3)?;
    let input = [value; 17];
    let before = project(&input);
    let observer = ReadScope::begin()?;
    assert!(matches!(
        calls.batch(&input)?,
        Err(BatchError::CapacityExceeded)
    ));
    assert_eq!(ReadScope::attempts()?, 0);
    drop(observer);
    assert_eq!(project(&input), before);
    assert_eq!((calls.intervals.get(), calls.batches.get()), (1, 1));
    Ok(())
}

/// Sixteen checked values preserve exact input order and require sixteen reads.
///
/// # Errors
/// Propagates charge, checked conversion/construction or observer errors.
///
/// # Panics
/// Panics on incorrect copied values, input preservation, read count or totals.
#[test]
fn capacity_neighbor_copies_all_values_in_order() -> TestResult {
    let calls = Calls::new("capacity_neighbor");
    let mut input = Vec::with_capacity(16);
    for index in 1_i64..=16 {
        input.push(calls.interval(u64::try_from(index)?, -index, index)?);
    }
    let before = project(&input);
    let observer = ReadScope::begin()?;
    let batch = calls.batch(&input)??;
    assert_eq!(ReadScope::attempts()?, 16);
    drop(observer);
    let copied: Vec<Projection> = batch
        .iter()
        .map(|value| (value.label().get(), value.lower(), value.upper()))
        .collect();
    assert_eq!(batch.len(), 16);
    assert_eq!(copied, before);
    assert_eq!(project(&input), before);
    raw_helper_contracts(&input, &batch)?;
    assert_eq!((calls.intervals.get(), calls.batches.get()), (16, 1));
    Ok(())
}

/// The earliest duplicate stops at index two and a valid neighbor still succeeds.
///
/// # Errors
/// Propagates charge, valid fixture-construction or observer errors.
///
/// # Panics
/// Panics on incorrect duplicate order, recovery, input preservation or totals.
#[test]
fn duplicate_stops_at_earliest_index_and_recovers() -> TestResult {
    let calls = Calls::new("duplicate_recovery");
    let a = calls.interval(1, -2, 3)?;
    let b = calls.interval(2, 4, 6)?;
    let input = [a, b, b, a];
    let before = project(&input);
    let observer = ReadScope::begin()?;
    assert!(matches!(
        calls.batch(&input)?,
        Err(BatchError::DuplicateLabel { later_index: 2 })
    ));
    assert_eq!(ReadScope::attempts()?, 3);
    drop(observer);
    assert_eq!(project(&input), before);
    let neighbor = [a, b];
    let neighbor_before = project(&neighbor);
    let observer = ReadScope::begin()?;
    let batch = calls.batch(&neighbor)??;
    assert_eq!(ReadScope::attempts()?, 2);
    drop(observer);
    let copied: Vec<Projection> = batch
        .iter()
        .map(|value| (value.label().get(), value.lower(), value.upper()))
        .collect();
    assert_eq!(copied, neighbor_before);
    assert_eq!(project(&neighbor), neighbor_before);
    assert_eq!(project(&input), before);
    assert_eq!((calls.intervals.get(), calls.batches.get()), (2, 2));
    Ok(())
}

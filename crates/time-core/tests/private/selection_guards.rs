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
};

use super::{BATCH_READ_ATTEMPTS, BatchError, FixtureBatch, FixtureInterval};

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

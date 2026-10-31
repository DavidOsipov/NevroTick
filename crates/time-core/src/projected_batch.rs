// SPDX-License-Identifier: GPL-2.0-only
//! Borrowed compatibility boundary for checked fixture projection enclosures.
//!
//! The intended checked value retains every original wrapper, declared frame,
//! mapping generation and exact evaluation instant, plus the interval owner's
//! completed batch. These assertions establish no real clock origin, current
//! authority, freshness, source independence or physical agreement. Borrowing
//! prevents the retained input from changing or outliving its source storage.
//! No history, ambient clock, network, persistent state or protected effect is
//! accessed. Capacity is checked before entry reads; mandatory contextual garde
//! validation and the same ordered typed checks precede interval construction.

use std::{fmt, num::NonZeroU64};

use garde::Validate;
use time_domain::MonotonicNs;

use crate::{
    BatchError, FixtureBatch, FixtureMinimum, FixtureProjectedInterval, FixtureSelection,
    SelectionError, select_fixture,
};

/// Existing interval-batch capacity, checked before traversing caller storage.
const MAX_PROJECTED_ENTRIES: usize = 16;

/// First differing field within one declared fixture compatibility context.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureProjectionMismatchKind {
    /// The projection's asserted coordinate-frame tag differs.
    Frame,
    /// The projection's asserted fixture-mapping generation differs.
    MappingGeneration,
    /// The supplied monotonic evaluation instant differs exactly.
    EvaluationTime,
}

/// Closed projected-batch failures without retained wrappers or context values.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum FixtureProjectedBatchError {
    /// More than sixteen input wrappers were supplied.
    CapacityExceeded,
    /// First incompatible entry and field in the declared admission order.
    Mismatch {
        /// Zero-based first mismatch position, below sixteen when returned by admission.
        index: usize,
        /// Closed mismatch category; no actual or expected context value is retained.
        kind: FixtureProjectionMismatchKind,
    },
    /// The interval batch owner's actual typed construction failure.
    Batch {
        /// Preserved cause, including the actual first later duplicate position.
        source: BatchError,
    },
    /// A completed representation or mandatory-validation result was inconsistent.
    Invariant,
}

impl fmt::Debug for FixtureProjectedBatchError {
    /// Emit only a closed redacted code and propagate the formatter's failure.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::CapacityExceeded => "FixtureProjectedBatchError::CapacityExceeded",
            Self::Mismatch { .. } => "FixtureProjectedBatchError::Mismatch { .. }",
            Self::Batch { .. } => "FixtureProjectedBatchError::Batch { .. }",
            Self::Invariant => "FixtureProjectedBatchError::Invariant",
        })
    }
}

impl fmt::Display for FixtureProjectedBatchError {
    /// Emit fixed text without raw context or a formatted retained cause.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::CapacityExceeded => "fixture projected batch: capacity exceeded",
            Self::Mismatch { .. } => "fixture projected batch: incompatible context",
            Self::Batch { .. } => "fixture projected batch: interval batch construction",
            Self::Invariant => "fixture projected batch: invariant",
        })
    }
}

impl std::error::Error for FixtureProjectedBatchError {
    /// Borrow only the actual delegated cause; other closed failures are leaves.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Batch { source } => Some(source),
            Self::CapacityExceeded | Self::Mismatch { .. } | Self::Invariant => None,
        }
    }
}

/// Immutable expectation whose scalar shape is owned by the supplied checked types.
#[derive(Clone, Copy)]
struct ProjectionExpectation {
    /// Nonzero asserted frame; it supplies no evidence of a genuine clock origin.
    frame: NonZeroU64,
    /// Nonzero caller assertion for the fixture mapping, not a peer association.
    mapping_generation: NonZeroU64,
    /// Exact supplied monotonic reading, without ambient-time certification.
    at: MonotonicNs,
}

/// Borrowed raw relationship candidate; stable wrapper invariants remain owned.
#[derive(Validate)]
#[garde(context(ProjectionExpectation))]
struct ProjectedBatchCandidate<'a> {
    /// The sole new relationship field is always checked through its shared rule.
    #[garde(custom(projected_entries_rule))]
    values: &'a [FixtureProjectedInterval],
}

/// Immutable bounded prefix whose compatibility was checked by `admit_entries`.
///
/// Only that private enforcing path constructs this carrier. Its complete slice
/// has at most sixteen entries; the interval builder needs no raw caller promise.
struct CompatibleEntries<'a> {
    /// Complete original checked-wrapper slice, with no mutation or retained copy.
    values: &'a [FixtureProjectedInterval],
}

/// Check the one maintained slice-capacity predicate without reading any entry.
fn over_capacity(values: &[FixtureProjectedInterval]) -> bool {
    values.len() > MAX_PROJECTED_ENTRIES
}

#[cfg(test)]
/// Passive test observation of actual adapter attempts and garde invocations.
#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
struct GuardObservations {
    /// Actual entry into the contextual garde callback, not a validation outcome.
    garde_callbacks: u64,
    /// Actual entry into the checked slice adapter, not successful memory reads.
    entry_read_attempts: u64,
}

#[cfg(test)]
std::thread_local! {
    /// Optional per-thread observation, absent in production and never used for admission.
    static GUARD_OBSERVATIONS: std::cell::Cell<Option<GuardObservations>> = const { std::cell::Cell::new(None) };
}

/// Borrow one actual input position; invalid raw indices fail without publication.
///
/// This adapter never copies a wrapper. Its optional test counter observes the
/// attempt before `get`, cannot change the result and saturates instead of wrapping.
/// # Errors
/// Return `Invariant` if the raw index is absent from this immutable slice.
fn read_entry(
    values: &[FixtureProjectedInterval],
    index: usize,
) -> Result<&FixtureProjectedInterval, FixtureProjectedBatchError> {
    #[cfg(test)]
    GUARD_OBSERVATIONS.with(|slot| {
        if let Some(mut observed) = slot.get() {
            observed.entry_read_attempts = observed.entry_read_attempts.saturating_add(1);
            slot.set(Some(observed));
        }
    });
    values
        .get(index)
        .ok_or(FixtureProjectedBatchError::Invariant)
}

/// Find the first raw relationship failure in bounded index/field order.
///
/// Capacity rejects before any entry access. For an admitted length, compare
/// ascending indices and frame, mapping generation, then exact evaluation time.
/// Values and expectation are immutable, so repeated checks cannot refresh data
/// or observe a changed context. No interval batch or partial result is produced.
fn first_issue(
    values: &[FixtureProjectedInterval],
    expected: &ProjectionExpectation,
) -> Option<FixtureProjectedBatchError> {
    if over_capacity(values) {
        return Some(FixtureProjectedBatchError::CapacityExceeded);
    }
    for index in 0..values.len() {
        let entry = match read_entry(values, index) {
            Ok(entry) => entry,
            Err(error) => return Some(error),
        };
        let projection = entry.projection();
        let context = projection.context();
        let kind = if context.frame() != expected.frame {
            Some(FixtureProjectionMismatchKind::Frame)
        } else if context.generation() != expected.mapping_generation {
            Some(FixtureProjectionMismatchKind::MappingGeneration)
        } else if projection.projected_at() != expected.at {
            Some(FixtureProjectionMismatchKind::EvaluationTime)
        } else {
            None
        };
        if let Some(kind) = kind {
            return Some(FixtureProjectedBatchError::Mismatch { index, kind });
        }
    }
    None
}

/// Apply the same raw relationship check inside mandatory contextual garde validation.
///
/// The optional observer records callback entry only and never decides admission.
/// # Errors
/// Return one fixed redacted rule error when the shared ordered check rejects.
fn projected_entries_rule(
    values: &[FixtureProjectedInterval],
    expected: &ProjectionExpectation,
) -> garde::Result {
    #[cfg(test)]
    GUARD_OBSERVATIONS.with(|slot| {
        if let Some(mut observed) = slot.get() {
            observed.garde_callbacks = observed.garde_callbacks.saturating_add(1);
            slot.set(Some(observed));
        }
    });
    if first_issue(values, expected).is_some() {
        Err(garde::Error::new("incompatible projected fixture entries"))
    } else {
        Ok(())
    }
}

/// Enforce all raw relationships before producing a private bounded carrier.
///
/// Capacity is an early resource guard. All potentially successful paths invoke
/// garde and use its result; the same pure check supplies the ordered typed cause.
/// Temporary garde reports contain one fixed field and are dropped on rejection.
/// # Errors
/// Preserve the first capacity/context issue; reject an otherwise unexplained
/// garde failure as `Invariant`. Rejection leaves the original slice unchanged.
fn admit_entries<'a>(
    values: &'a [FixtureProjectedInterval],
    expected: &ProjectionExpectation,
) -> Result<CompatibleEntries<'a>, FixtureProjectedBatchError> {
    if over_capacity(values) {
        return Err(FixtureProjectedBatchError::CapacityExceeded);
    }
    let candidate = ProjectedBatchCandidate { values };
    let validation = candidate.validate_with(expected);
    if let Some(error) = first_issue(values, expected) {
        return Err(error);
    }
    validation.map_err(|_| FixtureProjectedBatchError::Invariant)?;
    Ok(CompatibleEntries { values })
}

/// Copy only checked intervals from one privately admitted occupied prefix.
///
/// The fixed sixteen-slot scratch starts from a genuine first interval; only
/// its occupied prefix is delegated. Empty input delegates an actual empty slice.
/// No wrapper is copied, refit, re-enclosed, reordered or partially published.
/// # Errors
/// Preserve the interval owner's actual `BatchError`; impossible checked input or
/// scratch positions return `Invariant`. All scratch is local and dropped on error.
fn build_interval_batch(
    entries: &CompatibleEntries<'_>,
) -> Result<FixtureBatch, FixtureProjectedBatchError> {
    if entries.values.is_empty() {
        return FixtureBatch::new(&[])
            .map_err(|source| FixtureProjectedBatchError::Batch { source });
    }
    let first = *read_entry(entries.values, 0)?.interval();
    let mut scratch = [first; MAX_PROJECTED_ENTRIES];
    for index in 1..entries.values.len() {
        let interval = *read_entry(entries.values, index)?.interval();
        *scratch
            .get_mut(index)
            .ok_or(FixtureProjectedBatchError::Invariant)? = interval;
    }
    let occupied = scratch
        .get(..entries.values.len())
        .ok_or(FixtureProjectedBatchError::Invariant)?;
    FixtureBatch::new(occupied).map_err(|source| FixtureProjectedBatchError::Batch { source })
}

/// Borrowed original wrappers and an associated completed immutable interval batch.
///
/// Successful construction must establish an occupied input slice of at most
/// sixteen entries, matching expected context at every index, and an interval
/// batch with the same length and original order. The input borrow remains
/// immutable for this value's lifetime; repeated reads or selection refresh no age.
pub struct FixtureProjectedBatch<'a> {
    /// Supplied checked expectation, retained even for the empty batch.
    expected: ProjectionExpectation,
    /// Complete original wrapper slice, with no copies, skipped entries or padding.
    values: &'a [FixtureProjectedInterval],
    /// Interval owner's checked same-order collection with its own private prefix.
    intervals: FixtureBatch,
}

impl fmt::Debug for FixtureProjectedBatch<'_> {
    /// Reveal only the type and propagate formatter failure; retain no formatted data.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureProjectedBatch { .. }")
    }
}

impl<'a> FixtureProjectedBatch<'a> {
    /// Admit original checked wrappers against one declared fixture expectation.
    ///
    /// Capacity rejects before entry reads. Every potentially successful path
    /// invokes shared contextual garde validation, checks ordered compatibility,
    /// then calls the actual interval-batch constructor. Rejection publishes no
    /// partial batch and leaves every original wrapper unchanged. No ambient time,
    /// history update, reprojection, freshness policy or external effect is used.
    /// # Errors
    /// Reject capacity first, then the first mismatching index with frame before
    /// mapping generation before evaluation time, then preserve actual `BatchError`.
    /// Impossible representations or unexplained garde failure return `Invariant`.
    pub fn new(
        expected_frame: NonZeroU64,
        expected_mapping_generation: NonZeroU64,
        expected_at: MonotonicNs,
        values: &'a [FixtureProjectedInterval],
    ) -> Result<Self, FixtureProjectedBatchError> {
        let expected = ProjectionExpectation {
            frame: expected_frame,
            mapping_generation: expected_mapping_generation,
            at: expected_at,
        };
        let admitted = admit_entries(values, &expected)?;
        let intervals = build_interval_batch(&admitted)?;
        Ok(Self {
            expected,
            values: admitted.values,
            intervals,
        })
    }

    /// Copy the unchanged asserted frame without validating a real clock origin.
    #[must_use]
    pub const fn frame(&self) -> NonZeroU64 {
        self.expected.frame
    }

    /// Copy the unchanged fixture-mapping generation without granting current authority.
    #[must_use]
    pub const fn mapping_generation(&self) -> NonZeroU64 {
        self.expected.mapping_generation
    }

    /// Copy the exact supplied evaluation instant without reading an ambient clock.
    #[must_use]
    pub const fn projected_at(&self) -> MonotonicNs {
        self.expected.at
    }

    /// Borrow every original wrapper in input order without copying or refreshing it.
    #[must_use]
    pub const fn entries(&self) -> &[FixtureProjectedInterval] {
        self.values
    }

    /// Return the complete occupied slice length without examining any entry.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.values.len()
    }

    /// Report whether the complete retained slice is empty, with no state change.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Delegate numerical fixture selection using the unchanged checked minimum.
    ///
    /// The private checked interval batch is consumed by the existing selector;
    /// no entry is dropped, no history is refit and no freshness or authority is
    /// created. The detached result remains a numerical fixture result only.
    /// # Errors
    /// Preserve the selector owner's actual typed invariant failure.
    pub fn select(&self, minimum: FixtureMinimum) -> Result<FixtureSelection, SelectionError> {
        select_fixture(&self.intervals, minimum)
    }
}

/// Genuine producers and raw guard observations preserve checked-value privacy.
#[cfg(test)]
#[path = "../tests/private/projected_batch_guards.rs"]
mod projected_batch_guards;

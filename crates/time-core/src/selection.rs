// SPDX-License-Identifier: GPL-2.0-only
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2011-2016, 2018, 2020-2025
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of version 2 of the GNU General Public License as
 * published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 *
 **********************************************************************
*/
//! Checked synthetic intervals and bounded real-axis maximum-depth selection.
//!
//! Modified by David Osipov with AI assistance, 30 October 2026. This adapts
//! selected Chrony 4.9 `sources.c` endpoint-order, maximum-depth and strict-majority
//! ideas at revision 8df4f1263e206585e6c4e61352d042c2dd12916a. Fixed integer
//! endpoint keys and point/open-span cells replace floating tagged events. The
//! fixture requires one maximum component and a complete common witness; it
//! does not reproduce Chrony's membership envelope, source eligibility, trust
//! priorities or configured-minimum-on-survivors behavior. These synthetic
//! values have no network, native clock, identity or source authority.

use std::{fmt, num::NonZeroU64};

use garde::Validate;

/// Bounded, payload-free interval-construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntervalError {
    /// The asserted fixture label was zero.
    ZeroLabel,
    /// The supplied lower bound exceeded the upper bound.
    ReversedBounds,
    /// A validated nonzero label could not be converted to `NonZeroU64`.
    ValidationInvariant,
}

/// Bounded, payload-free fixture-minimum failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MinimumError {
    /// The supplied minimum was not in 1..=16.
    OutOfRange,
}

/// Bounded fixture-batch failure, with only the permitted duplicate index.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BatchError {
    /// More than sixteen records were supplied.
    CapacityExceeded,
    /// First duplicate's later zero-based input position.
    DuplicateLabel {
        /// Bounded later input position of the first duplicate.
        later_index: usize,
    },
    /// A checked input index or staging slot was unexpectedly inconsistent.
    Invariant,
}

/// Stage of a checked selection postcondition failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectionInvariantStage {
    /// Fixed-capacity slot or index consistency.
    Slot,
    /// Maximum-component consistency.
    Component,
    /// Witness support consistency.
    Support,
    /// Supporter intersection consistency.
    Intersection,
}

/// Bounded, payload-free evaluator failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectionError {
    /// An internal checked postcondition failed.
    Invariant {
        /// Output invariant that could not be established.
        stage: SelectionInvariantStage,
    },
}

/// Implement fixed-prefix formatting and leaf error chaining for typed failures.
///
/// Each supplied enum contains bounded metadata only; no validation report or
/// raw fixture input is retained or formatted by these implementations.
macro_rules! display_error {
    ($error:ty, $name:literal) => {
        impl fmt::Display for $error {
            /// Format only bounded typed failure information.
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, concat!($name, ": {:?}"), self)
            }
        }
        impl std::error::Error for $error {}
    };
}

display_error!(IntervalError, "fixture interval");
display_error!(MinimumError, "fixture minimum");
display_error!(BatchError, "fixture batch");
display_error!(SelectionError, "fixture selection");

/// Raw label covered by the one nonzero range rule.
#[derive(Validate)]
struct RawLabel {
    /// An asserted synthetic label; zero is not a vote.
    #[garde(range(min = 1))]
    value: u64,
}

/// Raw endpoint pair covered by the one ordering rule.
#[derive(Validate)]
struct RawBounds {
    /// Inclusive bounds on one exact synthetic real axis.
    #[garde(custom(bounds_rule))]
    value: (i64, i64),
}

/// Admit all ordered inclusive endpoint pairs without coordinate arithmetic.
///
/// Both i64 extrema and singleton intervals are valid; this pure rule neither
/// retains the pair nor gives it a clock frame.
/// # Errors
/// Rejects a reversed pair with fixed text and no raw input in the error.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rules require borrowed fields and a borrowed unit validation context"
)]
fn bounds_rule(value: &(i64, i64), (): &()) -> garde::Result {
    if value.0 <= value.1 {
        Ok(())
    } else {
        Err(garde::Error::new("invalid fixture interval bounds"))
    }
}

/// Raw fixture minimum covered by the one inclusive capacity rule.
#[derive(Validate)]
struct RawMinimum {
    /// Independently asserted count in 1..=16.
    #[garde(range(min = 1, max = 16))]
    value: usize,
}

/// Immutable asserted label and closed bounds on one synthetic real axis.
#[derive(Clone, Copy)]
pub struct FixtureInterval {
    /// Checked asserted fixture label, without identity authority.
    label: NonZeroU64,
    /// Inclusive exact synthetic bounds.
    bounds: (i64, i64),
}

/// Immutable configured fixture minimum, independent of batch size.
#[derive(Clone, Copy)]
pub struct FixtureMinimum {
    /// Unchanged checked minimum.
    value: usize,
}

/// Immutable fixed-capacity collection of already checked fixture intervals.
#[derive(Clone)]
pub struct FixtureBatch {
    /// Occupied prefix of checked values, followed by empty slots.
    values: [Option<FixtureInterval>; 16],
    /// Number of admitted values.
    len: usize,
}

#[cfg(test)]
std::thread_local! {
    /// Optional per-thread observation of actual batch input read attempts.
    /// Saturation prevents overflow from appearing as a smaller passing count;
    /// this observation does not decide admission and is absent in production.
    static BATCH_READ_ATTEMPTS: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
}

/// Copy one checked interval at `index` without changing the caller's slice.
///
/// The test-only counter records the actual access attempt and cannot alter the
/// result. A missing index is an internal invariant failure, not input rejection.
/// # Errors
/// Returns an invariant failure if the index is not present in the slice.
fn read_batch_interval(
    values: &[FixtureInterval],
    index: usize,
) -> Result<FixtureInterval, BatchError> {
    #[cfg(test)]
    BATCH_READ_ATTEMPTS.with(|attempts| {
        if let Some(count) = attempts.get() {
            attempts.set(Some(count.saturating_add(1)));
        }
    });
    values.get(index).copied().ok_or(BatchError::Invariant)
}

/// Closed, non-authoritative fixture outcome kind; it carries no result data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureOutcomeKind {
    /// No intervals were supplied.
    Empty,
    /// Population was below the unchanged minimum.
    InsufficientPopulation,
    /// Maximum depth did not exceed half the population.
    NoMajority,
    /// The strict majority did not meet the configured minimum.
    InsufficientAgreement,
    /// More than one maximum-depth component is unsupported by this fixture.
    UnsupportedAmbiguousFixture,
    /// One maximum component has a complete common witness.
    WitnessedFixture,
}

/// Privately constructed common witness and sorted synthetic supporters.
pub struct FixtureWitness {
    /// Sole maximum-depth component's inclusive bounds.
    component: (i64, i64),
    /// Exact common witness at the lower component endpoint.
    witness: i64,
    /// Complete supporters' inclusive intersection.
    intersection: (i64, i64),
    /// Fixed storage; only the checked prefix contains exposed supporters.
    labels: [NonZeroU64; 16],
    /// Length of the admitted sorted supporter prefix.
    label_count: usize,
}

/// Immutable fixture result with metadata from one evaluated batch.
pub struct FixtureSelection {
    /// Number of admitted intervals.
    population: usize,
    /// Unchanged configured minimum.
    minimum: usize,
    /// Actual global maximum depth.
    maximum_depth: usize,
    /// Whether depth exceeds half of population.
    strict_majority: bool,
    /// Whether depth meets minimum.
    minimum_met: bool,
    /// Closed result kind.
    outcome: FixtureOutcomeKind,
    /// Component count only for unsupported ambiguity.
    component_count: Option<usize>,
    /// Complete common witness only for witnessed success.
    witnessed: Option<FixtureWitness>,
}

/// Format only the supplied type name, without reading its private fixture data.
///
/// The borrowed value and its state remain unchanged; formatter failure is
/// propagated without producing a substitute successful diagnostic.
macro_rules! redacted_debug {
    ($type:ty, $name:literal) => {
        impl fmt::Debug for $type {
            /// Exclude raw fixture values from diagnostics.
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.debug_struct($name).finish_non_exhaustive()
            }
        }
    };
}

redacted_debug!(FixtureInterval, "FixtureInterval");
redacted_debug!(FixtureMinimum, "FixtureMinimum");
redacted_debug!(FixtureBatch, "FixtureBatch");
redacted_debug!(FixtureWitness, "FixtureWitness");
redacted_debug!(FixtureSelection, "FixtureSelection");

impl FixtureInterval {
    /// Validate and retain one asserted label and exact closed interval.
    ///
    /// Garde's label rule runs first, followed by the whole-pair rule. The
    /// immutable result retains the exact inputs and grants no source or clock
    /// authority. Temporary one-rule validation reports are dropped on failure;
    /// validation may allocate. There are no external effects.
    /// # Errors
    /// Rejects zero label before reversed bounds, or an impossible failed
    /// nonzero conversion after successful label validation.
    pub fn new(label: u64, lo: i64, hi: i64) -> Result<Self, IntervalError> {
        let raw_label = RawLabel { value: label };
        raw_label.validate().map_err(|_| IntervalError::ZeroLabel)?;
        let label = NonZeroU64::new(raw_label.value).ok_or(IntervalError::ValidationInvariant)?;
        let raw_bounds = RawBounds { value: (lo, hi) };
        raw_bounds
            .validate()
            .map_err(|_| IntervalError::ReversedBounds)?;
        Ok(Self {
            label,
            bounds: raw_bounds.value,
        })
    }

    /// Copy the checked asserted label; this grants no source authority.
    #[must_use]
    pub const fn label(&self) -> NonZeroU64 {
        self.label
    }
    /// Copy the exact inclusive lower bound, without coordinate conversion.
    #[must_use]
    pub const fn lower(&self) -> i64 {
        self.bounds.0
    }
    /// Copy the exact inclusive upper bound, without coordinate conversion.
    #[must_use]
    pub const fn upper(&self) -> i64 {
        self.bounds.1
    }
}

impl FixtureMinimum {
    /// Validate and retain a fixture minimum independently of batch size.
    ///
    /// The sole range rule is applied inside construction; the temporary report
    /// is dropped on rejection and validation may allocate. No effects occur.
    /// # Errors
    /// Rejects values outside 1..=16 without changing caller state.
    pub fn new(value: usize) -> Result<Self, MinimumError> {
        let raw = RawMinimum { value };
        raw.validate().map_err(|_| MinimumError::OutOfRange)?;
        Ok(Self { value: raw.value })
    }

    /// Copy the admitted configured minimum without reducing it to population.
    #[must_use]
    pub const fn get(&self) -> usize {
        self.value
    }
}

impl FixtureBatch {
    /// Accept already checked intervals without mutating or retaining the slice.
    ///
    /// Capacity is checked before any element read. Each accepted position is
    /// copied once into a local fixed-capacity prefix; duplicate labels are
    /// rejected at the earliest later position. A batch is published only after
    /// all positions pass. No external effects occur.
    /// # Errors
    /// Rejects excess capacity or the first later duplicate label; reports a
    /// checked internal index or staging-slot inconsistency as an invariant.
    pub fn new(values: &[FixtureInterval]) -> Result<Self, BatchError> {
        let len = values.len();
        if len > 16 {
            return Err(BatchError::CapacityExceeded);
        }
        let mut staged = [None; 16];
        for later_index in 0..len {
            let value = read_batch_interval(values, later_index)?;
            let prior = staged.get(..later_index).ok_or(BatchError::Invariant)?;
            for previous in prior {
                let previous: &FixtureInterval = previous.as_ref().ok_or(BatchError::Invariant)?;
                if previous.label() == value.label() {
                    return Err(BatchError::DuplicateLabel { later_index });
                }
            }
            let slot = staged.get_mut(later_index).ok_or(BatchError::Invariant)?;
            if slot.is_some() {
                return Err(BatchError::Invariant);
            }
            *slot = Some(value);
        }
        Ok(Self {
            values: staged,
            len,
        })
    }

    /// Return the admitted record count without changing the batch.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }
    /// Indicate whether the batch contains no admitted records.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// Borrow the checked records in their original input order.
    pub fn iter(&self) -> impl Iterator<Item = &FixtureInterval> {
        self.values.iter().take(self.len).filter_map(Option::as_ref)
    }
}

impl FixtureSelection {
    /// Return the same-batch admitted population.
    #[must_use]
    pub const fn population(&self) -> usize {
        self.population
    }
    /// Return the unchanged configured fixture minimum.
    #[must_use]
    pub const fn minimum(&self) -> usize {
        self.minimum
    }
    /// Return global maximum interval depth.
    #[must_use]
    pub const fn maximum_depth(&self) -> usize {
        self.maximum_depth
    }
    /// Return whether depth is a strict population majority.
    #[must_use]
    pub const fn strict_majority(&self) -> bool {
        self.strict_majority
    }
    /// Return whether depth meets the configured fixture minimum.
    #[must_use]
    pub const fn minimum_met(&self) -> bool {
        self.minimum_met
    }
    /// Return the closed fixture outcome without source or clock authority.
    #[must_use]
    pub const fn outcome(&self) -> FixtureOutcomeKind {
        self.outcome
    }
    /// Return the count only for an unsupported ambiguous fixture.
    #[must_use]
    pub const fn component_count(&self) -> Option<usize> {
        self.component_count
    }
    /// Borrow a completed witness only for the witnessed outcome.
    #[must_use]
    pub const fn witnessed(&self) -> Option<&FixtureWitness> {
        self.witnessed.as_ref()
    }
}

impl FixtureWitness {
    /// Copy the inclusive lower endpoint of the sole maximum component.
    #[must_use]
    pub const fn component_lower(&self) -> i64 {
        self.component.0
    }
    /// Copy the inclusive upper endpoint of the sole maximum component.
    #[must_use]
    pub const fn component_upper(&self) -> i64 {
        self.component.1
    }
    /// Copy the deterministic common witness, the component's lower endpoint.
    #[must_use]
    pub const fn witness(&self) -> i64 {
        self.witness
    }
    /// Copy the inclusive lower endpoint of the supporters' intersection.
    #[must_use]
    pub const fn intersection_lower(&self) -> i64 {
        self.intersection.0
    }
    /// Copy the inclusive upper endpoint of the supporters' intersection.
    #[must_use]
    pub const fn intersection_upper(&self) -> i64 {
        self.intersection.1
    }
    /// Iterate the complete admitted supporter labels in ascending order.
    pub fn supporters(&self) -> impl ExactSizeIterator<Item = NonZeroU64> + '_ {
        self.labels.iter().take(self.label_count).copied()
    }
}

/// Fixed-capacity storage or checked index failed its internal contract.
const SLOT_ERROR: SelectionError = SelectionError::Invariant {
    stage: SelectionInvariantStage::Slot,
};
/// Closed-real maximum-component construction failed its internal contract.
const COMPONENT_ERROR: SelectionError = SelectionError::Invariant {
    stage: SelectionInvariantStage::Component,
};
/// The complete witness support did not match the evaluated maximum depth.
const SUPPORT_ERROR: SelectionError = SelectionError::Invariant {
    stage: SelectionInvariantStage::Support,
};
/// Supporters' common intersection did not equal the sole maximum component.
const INTERSECTION_ERROR: SelectionError = SelectionError::Invariant {
    stage: SelectionInvariantStage::Intersection,
};

/// Sorted distinct supplied endpoint keys in a nonempty initialized prefix.
struct EndpointKeys {
    /// Only the prefix is read as coordinate keys; padding is scratch only.
    values: [i64; 32],
    /// Checked number of distinct keys, in 1..=32.
    len: usize,
}

/// Once-evaluated alternating point/open-span depths in real-axis order.
struct CellDepths {
    /// Point at index 2k and open span after it at index 2k+1.
    values: [usize; 63],
    /// Odd initialized prefix length, twice the key count minus one.
    len: usize,
    /// Actual largest depth among all initialized cells.
    maximum: usize,
}

/// Count of all maximum components and the first one's closed endpoints.
struct MaximumComponents {
    /// Checked count in 1..=32; only a count of one can publish a witness.
    count: usize,
    /// First component in coordinate order, including a possible singleton.
    first: (i64, i64),
}

/// Sort at most 32 initialized values in place without allocating or panicking.
///
/// Insertion sort uses at most n(n-1)/2 order comparisons and moves, plus n-1
/// final writes. The caller owns the slice; failures publish no selection.
/// # Errors
/// Rejects an oversized slice or an inconsistent checked index.
fn sort_prefix<T: Copy + Ord>(values: &mut [T]) -> Result<(), SelectionError> {
    if values.len() > 32 {
        return Err(SLOT_ERROR);
    }
    for index in 1..values.len() {
        let value = *values.get(index).ok_or(SLOT_ERROR)?;
        let mut position = index;
        while position > 0 {
            let prior_index = position.checked_sub(1).ok_or(SLOT_ERROR)?;
            let prior = *values.get(prior_index).ok_or(SLOT_ERROR)?;
            if prior <= value {
                break;
            }
            *values.get_mut(position).ok_or(SLOT_ERROR)? = prior;
            position = prior_index;
        }
        *values.get_mut(position).ok_or(SLOT_ERROR)? = value;
    }
    Ok(())
}

/// Collect, sort and deduplicate the two exact endpoints of each checked row.
///
/// A nonempty immutable batch supplies at most 32 keys; no coordinate arithmetic
/// occurs. Sorting and deduplication read only their initialized prefixes.
/// # Errors
/// Rejects an empty/inconsistent batch, count or scratch index.
fn unique_endpoints(batch: &FixtureBatch) -> Result<EndpointKeys, SelectionError> {
    let mut values = [0; 32];
    let mut len: usize = 0;
    for interval in batch.iter() {
        for endpoint in [interval.lower(), interval.upper()] {
            *values.get_mut(len).ok_or(SLOT_ERROR)? = endpoint;
            len = len.checked_add(1).ok_or(SLOT_ERROR)?;
        }
    }
    if len == 0 || Some(len) != batch.len().checked_mul(2) {
        return Err(SLOT_ERROR);
    }
    sort_prefix(values.get_mut(..len).ok_or(SLOT_ERROR)?)?;
    let mut unique: usize = 0;
    for index in 0..len {
        let key = *values.get(index).ok_or(SLOT_ERROR)?;
        let is_new = match unique.checked_sub(1) {
            Some(previous) => *values.get(previous).ok_or(SLOT_ERROR)? != key,
            None => true,
        };
        if is_new {
            *values.get_mut(unique).ok_or(SLOT_ERROR)? = key;
            unique = unique.checked_add(1).ok_or(SLOT_ERROR)?;
        }
    }
    Ok(EndpointKeys {
        values,
        len: unique,
    })
}

/// Count intervals covering one point or the open cell between adjacent keys.
///
/// Equal bounds describe a closed point. Distinct adjacent endpoint keys describe
/// an open span: with no intervening endpoints, lo<=lower and upper<=hi exactly
/// characterize its constant membership. At most sixteen rows are inspected.
/// # Errors
/// Rejects reversed cell bounds; no input or retained state is changed.
fn cell_depth(batch: &FixtureBatch, lower: i64, upper: i64) -> Result<usize, SelectionError> {
    if lower > upper {
        return Err(COMPONENT_ERROR);
    }
    Ok(batch
        .iter()
        .filter(|interval| interval.lower() <= lower && upper <= interval.upper())
        .count())
}

/// Retain one computed depth and update its same-prefix maximum.
///
/// The scratch owner admits at most 63 cells; only local scratch is modified.
/// # Errors
/// Rejects an unavailable storage slot or an inconsistent count.
fn append_depth(cells: &mut CellDepths, depth: usize) -> Result<(), SelectionError> {
    *cells.values.get_mut(cells.len).ok_or(SLOT_ERROR)? = depth;
    cells.len = cells.len.checked_add(1).ok_or(SLOT_ERROR)?;
    cells.maximum = cells.maximum.max(depth);
    Ok(())
}

/// Evaluate each supplied endpoint and intervening real open span exactly once.
///
/// There are p points and p-1 spans for 1<=p<=32, at most 63 cells and 1,008
/// row/cell membership probes. Endpoint magnitude never changes the workload.
/// # Errors
/// Rejects inconsistent sorted keys, cell counts or scratch indices.
fn depth_cells(batch: &FixtureBatch, keys: &EndpointKeys) -> Result<CellDepths, SelectionError> {
    let mut cells = CellDepths {
        values: [0; 63],
        len: 0,
        maximum: 0,
    };
    for index in 0..keys.len {
        let key = *keys.values.get(index).ok_or(SLOT_ERROR)?;
        append_depth(&mut cells, cell_depth(batch, key, key)?)?;
        let next_index = index.checked_add(1).ok_or(SLOT_ERROR)?;
        if next_index < keys.len {
            let next = *keys.values.get(next_index).ok_or(SLOT_ERROR)?;
            if key >= next {
                return Err(COMPONENT_ERROR);
            }
            append_depth(&mut cells, cell_depth(batch, key, next)?)?;
        }
    }
    let expected = keys.len.checked_mul(2).and_then(|n| n.checked_sub(1));
    if Some(cells.len) != expected || cells.maximum == 0 || cells.maximum > batch.len() {
        return Err(COMPONENT_ERROR);
    }
    Ok(cells)
}

/// Scan retained cell depths for connected runs at the actual global maximum.
///
/// Closed intervals make every maximum span's endpoints maximum points. Each
/// run must therefore begin and end at a point, including singleton components;
/// an intervening nonmaximum open span separates even adjacent integer keys.
/// At most 63 cells and 32 component starts are visited, with no depth recheck.
/// # Errors
/// Rejects inconsistent cell topology, missing maximum points or scratch indices.
fn maximum_components(
    keys: &EndpointKeys,
    cells: &CellDepths,
) -> Result<MaximumComponents, SelectionError> {
    if cells.len.is_multiple_of(2) || cells.maximum == 0 {
        return Err(COMPONENT_ERROR);
    }
    let mut count: usize = 0;
    let mut first = None;
    let mut active = false;
    for (index, depth) in cells
        .values
        .get(..cells.len)
        .ok_or(SLOT_ERROR)?
        .iter()
        .enumerate()
    {
        let is_point = index.is_multiple_of(2);
        if *depth == cells.maximum {
            if !active {
                if !is_point {
                    return Err(COMPONENT_ERROR);
                }
                count = count
                    .checked_add(1)
                    .filter(|n| *n <= 32)
                    .ok_or(COMPONENT_ERROR)?;
                if count == 1 {
                    let key = *keys.values.get(index / 2).ok_or(SLOT_ERROR)?;
                    first = Some((key, key));
                }
            }
            if is_point && count == 1 {
                let key = *keys.values.get(index / 2).ok_or(SLOT_ERROR)?;
                let bounds = first.as_mut().ok_or(COMPONENT_ERROR)?;
                bounds.1 = key;
            }
            active = true;
        } else if active {
            if is_point {
                return Err(COMPONENT_ERROR);
            }
            active = false;
        }
    }
    let first = first.ok_or(COMPONENT_ERROR)?;
    if first.0 > first.1 || count == 0 {
        return Err(COMPONENT_ERROR);
    }
    Ok(MaximumComponents { count, first })
}

/// Project and sort only the populated nonzero supporter-label prefix.
///
/// Padding is initialized from an actual supporter and remains unexposed. There
/// are at most sixteen copies and 120 insertion-sort comparisons and moves.
/// # Errors
/// Rejects an empty/oversized support prefix, missing label or scratch index.
fn supporter_labels(
    staged: &[Option<NonZeroU64>; 16],
    count: usize,
) -> Result<[NonZeroU64; 16], SelectionError> {
    if !(1..=16).contains(&count) {
        return Err(SUPPORT_ERROR);
    }
    let first = staged.first().copied().flatten().ok_or(SUPPORT_ERROR)?;
    let mut labels = [first; 16];
    let prefix = labels.get_mut(..count).ok_or(SLOT_ERROR)?;
    for (slot, value) in prefix
        .iter_mut()
        .zip(staged.get(..count).ok_or(SLOT_ERROR)?)
    {
        *slot = value.ok_or(SUPPORT_ERROR)?;
    }
    sort_prefix(prefix)?;
    Ok(labels)
}

/// Build a witness from all intervals containing the sole component's left end.
///
/// Inspect at most sixteen rows and reduce at most fifteen pairs of original
/// bounds. Before publication require exactly D supporters and their common
/// intersection equal to the complete component. Inputs remain immutable.
/// # Errors
/// Rejects support/count/index inconsistency or a failed intersection postcondition.
fn common_witness(
    batch: &FixtureBatch,
    component: (i64, i64),
    maximum: usize,
) -> Result<FixtureWitness, SelectionError> {
    let witness = component.0;
    let mut staged = [None; 16];
    let mut count: usize = 0;
    let mut intersection: Option<(i64, i64)> = None;
    for interval in batch.iter() {
        if interval.lower() <= witness && witness <= interval.upper() {
            *staged.get_mut(count).ok_or(SLOT_ERROR)? = Some(interval.label());
            count = count.checked_add(1).ok_or(SUPPORT_ERROR)?;
            intersection = Some(match intersection {
                None => (interval.lower(), interval.upper()),
                Some((lower, upper)) => (lower.max(interval.lower()), upper.min(interval.upper())),
            });
        }
    }
    if count != maximum || count == 0 || count > batch.len() {
        return Err(SUPPORT_ERROR);
    }
    let intersection = intersection.ok_or(INTERSECTION_ERROR)?;
    if intersection != component || intersection.0 > witness || witness > intersection.1 {
        return Err(INTERSECTION_ERROR);
    }
    let labels = supporter_labels(&staged, count)?;
    Ok(FixtureWitness {
        component,
        witness,
        intersection,
        labels,
        label_count: count,
    })
}

/// Evaluate an immutable synthetic batch under an unchanged fixture minimum.
///
/// Compute actual global depth even below the population minimum. The declared
/// priority is empty, insufficient population, no strict majority, insufficient
/// agreement, multiple maximum components, then one common witness. At most 32
/// keys and 63 real-axis cells use fixed local scratch, with no allocation,
/// coordinate arithmetic, retained state, network or native clock effects.
/// # Errors
/// Returns only bounded internal index, topology, support or intersection failure;
/// no partial result is published and both admitted inputs remain usable.
pub fn select_fixture(
    batch: &FixtureBatch,
    minimum: FixtureMinimum,
) -> Result<FixtureSelection, SelectionError> {
    let population = batch.len();
    let minimum = minimum.get();
    if batch.is_empty() {
        return Ok(FixtureSelection {
            population,
            minimum,
            maximum_depth: 0,
            strict_majority: false,
            minimum_met: false,
            outcome: FixtureOutcomeKind::Empty,
            component_count: None,
            witnessed: None,
        });
    }
    let keys = unique_endpoints(batch)?;
    let cells = depth_cells(batch, &keys)?;
    let maximum_depth = cells.maximum;
    let strict_majority = maximum_depth > population / 2;
    let minimum_met = maximum_depth >= minimum;
    let mut component_count = None;
    let mut witnessed = None;
    let outcome = if population < minimum {
        FixtureOutcomeKind::InsufficientPopulation
    } else if !strict_majority {
        FixtureOutcomeKind::NoMajority
    } else if !minimum_met {
        FixtureOutcomeKind::InsufficientAgreement
    } else {
        let components = maximum_components(&keys, &cells)?;
        if components.count > 1 {
            component_count = Some(components.count);
            FixtureOutcomeKind::UnsupportedAmbiguousFixture
        } else {
            witnessed = Some(common_witness(batch, components.first, maximum_depth)?);
            FixtureOutcomeKind::WitnessedFixture
        }
    };
    Ok(FixtureSelection {
        population,
        minimum,
        maximum_depth,
        strict_majority,
        minimum_met,
        outcome,
        component_count,
        witnessed,
    })
}

#[cfg(test)]
#[path = "../tests/private/selection_guards.rs"]
mod selection_guards;

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
    SelectionInvariantStage, cell_depth, common_witness, depth_cells, maximum_components,
    read_batch_interval, sort_prefix, supporter_labels, unique_endpoints,
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
        let limit = if self.case == "duplicate_recovery" {
            5
        } else {
            2
        };
        charge(&self.batches, limit)?;
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
    /// Endpoint collection from an admitted batch, including empty rejection.
    Endpoints,
    /// Depth construction from separately admitted keys and batch.
    Cells,
    /// Complete first-component and component-count projection.
    Components,
    /// Raw common-component and declared-maximum witness construction.
    Witness,
}

/// Closed source-owned helper cases; none is a public product input.
#[derive(Clone, Copy)]
enum HelperCase {
    /// Reuse the sixteen checked nested intervals and their admitted batch.
    CapacityNeighbor,
    /// Reuse the two checked disjoint intervals and their recovery batch.
    DuplicateRecovery,
}

impl HelperCase {
    /// Return the fixed payload-free case identity for native transport.
    const fn name(self) -> &'static str {
        match self {
            Self::CapacityNeighbor => "capacity_neighbor",
            Self::DuplicateRecovery => "duplicate_recovery",
        }
    }
}

/// Finite direct-helper attempts, independent of public construction counters.
struct HelperCalls {
    /// Fixed native scope identity and complete per-helper recipe owner.
    case: HelperCase,
    /// Direct input-copy adapter attempts, at most two.
    reads: Cell<usize>,
    /// Direct prefix-sort attempts, at most three.
    sorts: Cell<usize>,
    /// Direct cell-depth attempts, at most two.
    depths: Cell<usize>,
    /// Direct optional-label projections, at most five.
    labels: Cell<usize>,
    /// Direct endpoint-collection attempts, at most three in a case.
    endpoints: Cell<usize>,
    /// Direct depth-construction attempts, at most three in a case.
    cells: Cell<usize>,
    /// Direct component scans, at most three in a case.
    components: Cell<usize>,
    /// Direct witness constructions, at most four in the nested case.
    witnesses: Cell<usize>,
}

impl HelperCalls {
    /// Begin one fixed helper recipe without charging a public constructor.
    const fn new(case: HelperCase) -> Self {
        Self {
            case,
            reads: Cell::new(0),
            sorts: Cell::new(0),
            depths: Cell::new(0),
            labels: Cell::new(0),
            endpoints: Cell::new(0),
            cells: Cell::new(0),
            components: Cell::new(0),
            witnesses: Cell::new(0),
        }
    }

    /// Charge the next direct attempt before entering the selected helper.
    ///
    /// # Errors
    /// Rejects overflow or a charge beyond that helper's complete recipe.
    fn charge(&self, kind: HelperKind) -> TestResult {
        let counter = match kind {
            HelperKind::Read => &self.reads,
            HelperKind::Sort => &self.sorts,
            HelperKind::Depth => &self.depths,
            HelperKind::Labels => &self.labels,
            HelperKind::Endpoints => &self.endpoints,
            HelperKind::Cells => &self.cells,
            HelperKind::Components => &self.components,
            HelperKind::Witness => &self.witnesses,
        };
        let limit = match (self.case, kind) {
            (
                HelperCase::CapacityNeighbor,
                HelperKind::Read | HelperKind::Depth | HelperKind::Endpoints | HelperKind::Cells,
            )
            | (HelperCase::DuplicateRecovery, HelperKind::Components) => 2,
            (HelperCase::CapacityNeighbor, HelperKind::Sort | HelperKind::Components)
            | (HelperCase::DuplicateRecovery, HelperKind::Endpoints | HelperKind::Cells) => 3,
            (HelperCase::CapacityNeighbor, HelperKind::Labels) => 5,
            (HelperCase::CapacityNeighbor, HelperKind::Witness) => 4,
            _ => 0,
        };
        charge(counter, limit)
    }

    /// Copy eight direct-attempt totals, without claiming recursive operations.
    fn totals(&self) -> [usize; 8] {
        [
            self.reads.get(),
            self.sorts.get(),
            self.depths.get(),
            self.labels.get(),
            self.endpoints.get(),
            self.cells.get(),
            self.components.get(),
            self.witnesses.get(),
        ]
    }
}

impl Drop for HelperCalls {
    /// Emit one complete native line even when a reached assertion unwinds.
    fn drop(&mut self) {
        let record = format!(
            "SELECTION_HELPER_PROFILE case={} read_adapters={} prefix_sorts={} cell_depths={} label_projections={} endpoint_collections={} depth_builds={} component_scans={} witness_builds={}\n",
            self.case.name(),
            self.reads.get(),
            self.sorts.get(),
            self.depths.get(),
            self.labels.get(),
            self.endpoints.get(),
            self.cells.get(),
            self.components.get(),
            self.witnesses.get()
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

/// Challenge raw declared maxima and component bounds on real nested intervals.
///
/// The sixteen intervals have labels 1..=16 and bounds [-label,label], so -1
/// has exactly sixteen supporters and their exact intersection is [-1,1].
/// # Errors
/// Propagates a pre-effect charge or the healthy witness construction's error.
/// # Panics
/// Panics on an incorrect rejection stage, witness, intersection or supporter set.
fn raw_witness_contract(batch: &FixtureBatch, calls: &HelperCalls) -> TestResult {
    calls.charge(HelperKind::Witness)?;
    assert!(matches!(
        common_witness(batch, (-1, 1), 17),
        Err(SelectionError::Invariant {
            stage: SelectionInvariantStage::Support
        })
    ));
    calls.charge(HelperKind::Witness)?;
    assert!(matches!(
        common_witness(batch, (17, 17), 0),
        Err(SelectionError::Invariant {
            stage: SelectionInvariantStage::Support
        })
    ));
    calls.charge(HelperKind::Witness)?;
    assert!(matches!(
        common_witness(batch, (-1, 0), 16),
        Err(SelectionError::Invariant {
            stage: SelectionInvariantStage::Intersection
        })
    ));
    calls.charge(HelperKind::Witness)?;
    let witness = common_witness(batch, (-1, 1), 16)?;
    assert_eq!(
        (
            witness.component_lower(),
            witness.component_upper(),
            witness.witness(),
            witness.intersection_lower(),
            witness.intersection_upper(),
        ),
        (-1, 1, -1, -1, 1)
    );
    assert_eq!(witness.supporters().len(), 16);
    assert_eq!(
        witness
            .supporters()
            .map(NonZeroU64::get)
            .collect::<Vec<_>>(),
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    );
    Ok(())
}

/// Challenge actual endpoint/depth/component boundaries without forged carriers.
///
/// The empty batch and batches containing [-2,3], [4,6] or both intervals are
/// admitted by the real public constructor. Keys have no batch-association token;
/// valid keys from the left batch must not admit zero-depth cells for the right.
/// Both disjoint maximum components exist, but only the first is [-2,3].
/// # Errors
/// Rejects the wrong fixture widths, a charge failure or a healthy helper error.
/// # Panics
/// Panics on wrong stages, exact keys/depths/components or changed admitted inputs.
fn raw_pipeline_contracts(
    empty: &FixtureBatch,
    left: &FixtureBatch,
    right: &FixtureBatch,
    disjoint: &FixtureBatch,
) -> TestResult {
    if !empty.is_empty() || left.len() != 1 || right.len() != 1 || disjoint.len() != 2 {
        return Err(io::Error::other("raw pipeline fixture widths").into());
    }
    let before: Vec<Vec<Projection>> = [empty, left, right, disjoint]
        .into_iter()
        .map(|batch| {
            batch
                .iter()
                .map(|value| (value.label().get(), value.lower(), value.upper()))
                .collect()
        })
        .collect();
    let calls = HelperCalls::new(HelperCase::DuplicateRecovery);
    calls.charge(HelperKind::Endpoints)?;
    assert!(matches!(
        unique_endpoints(empty),
        Err(SelectionError::Invariant {
            stage: SelectionInvariantStage::Slot
        })
    ));
    calls.charge(HelperKind::Endpoints)?;
    let left_keys = unique_endpoints(left)?;
    assert_eq!(left_keys.len, 2);
    assert_eq!(
        left_keys.values.get(..left_keys.len),
        Some([-2, 3].as_slice())
    );
    calls.charge(HelperKind::Endpoints)?;
    let disjoint_keys = unique_endpoints(disjoint)?;
    assert_eq!(disjoint_keys.len, 4);
    assert_eq!(
        disjoint_keys.values.get(..disjoint_keys.len),
        Some([-2, 3, 4, 6].as_slice())
    );
    calls.charge(HelperKind::Cells)?;
    let left_cells = depth_cells(left, &left_keys)?;
    assert_eq!((left_cells.len, left_cells.maximum), (3, 1));
    assert_eq!(
        left_cells.values.get(..left_cells.len),
        Some([1, 1, 1].as_slice())
    );
    calls.charge(HelperKind::Cells)?;
    assert!(matches!(
        depth_cells(right, &left_keys),
        Err(SelectionError::Invariant {
            stage: SelectionInvariantStage::Component
        })
    ));
    calls.charge(HelperKind::Cells)?;
    let disjoint_cells = depth_cells(disjoint, &disjoint_keys)?;
    assert_eq!((disjoint_cells.len, disjoint_cells.maximum), (7, 1));
    assert_eq!(
        disjoint_cells.values.get(..disjoint_cells.len),
        Some([1, 1, 1, 0, 1, 1, 1].as_slice())
    );
    calls.charge(HelperKind::Components)?;
    let single = maximum_components(&left_keys, &left_cells)?;
    assert_eq!((single.count, single.first), (1, (-2, 3)));
    calls.charge(HelperKind::Components)?;
    let multiple = maximum_components(&disjoint_keys, &disjoint_cells)?;
    assert_eq!((multiple.count, multiple.first), (2, (-2, 3)));
    assert_eq!(calls.totals(), [0, 0, 0, 0, 3, 3, 2, 0]);
    let after: Vec<Vec<Projection>> = [empty, left, right, disjoint]
        .into_iter()
        .map(|batch| {
            batch
                .iter()
                .map(|value| (value.label().get(), value.lower(), value.upper()))
                .collect()
        })
        .collect();
    assert_eq!(after, before);
    assert_eq!(
        left_keys.values.get(..left_keys.len),
        Some([-2, 3].as_slice())
    );
    assert_eq!(
        disjoint_keys.values.get(..disjoint_keys.len),
        Some([-2, 3, 4, 6].as_slice())
    );
    Ok(())
}

/// Reject mismatched completed topology before any endpoint padding is read.
///
/// Both keys and cells come from their real checked producers. The nested full
/// batch has 32 keys and 63 cells; its first interval has two keys and three
/// cells. Pairing the small keys with the full cells must reject, while both
/// original pairs yield the exact common maximum component [-1,1].
/// # Errors
/// Propagates a pre-effect charge or any healthy producer/consumer error.
/// # Panics
/// Panics on wrong topology, a rejected healthy pair or accepted mismatched pair.
fn raw_topology_contract(
    full: &FixtureBatch,
    small: &FixtureBatch,
    calls: &HelperCalls,
) -> TestResult {
    calls.charge(HelperKind::Endpoints)?;
    let full_keys = unique_endpoints(full)?;
    calls.charge(HelperKind::Endpoints)?;
    let small_keys = unique_endpoints(small)?;
    assert_eq!((full_keys.len, small_keys.len), (32, 2));
    assert_eq!(
        small_keys.values.get(..small_keys.len),
        Some([-1, 1].as_slice())
    );
    calls.charge(HelperKind::Cells)?;
    let full_cells = depth_cells(full, &full_keys)?;
    calls.charge(HelperKind::Cells)?;
    let small_cells = depth_cells(small, &small_keys)?;
    assert_eq!((full_cells.len, full_cells.maximum), (63, 16));
    assert_eq!((small_cells.len, small_cells.maximum), (3, 1));
    calls.charge(HelperKind::Components)?;
    let complete = maximum_components(&full_keys, &full_cells)?;
    assert_eq!((complete.count, complete.first), (1, (-1, 1)));
    calls.charge(HelperKind::Components)?;
    let neighbor = maximum_components(&small_keys, &small_cells)?;
    assert_eq!((neighbor.count, neighbor.first), (1, (-1, 1)));
    calls.charge(HelperKind::Components)?;
    assert!(matches!(
        maximum_components(&small_keys, &full_cells),
        Err(SelectionError::Invariant {
            stage: SelectionInvariantStage::Component
        })
    ));
    assert_eq!((full_keys.len, small_keys.len), (32, 2));
    assert_eq!(
        small_keys.values.get(..small_keys.len),
        Some([-1, 1].as_slice())
    );
    assert_eq!((full_cells.len, full_cells.maximum), (63, 16));
    assert_eq!((small_cells.len, small_cells.maximum), (3, 1));
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
fn raw_helper_contracts(
    values: &[FixtureInterval],
    batch: &FixtureBatch,
    small: &FixtureBatch,
) -> TestResult {
    if values.len() != 16 || batch.len() != 16 || small.len() != 1 {
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
    let calls = HelperCalls::new(HelperCase::CapacityNeighbor);
    raw_read_contract(values, &calls)?;
    raw_sort_contract(&calls)?;
    raw_depth_contract(batch, &calls)?;
    raw_label_contract(values, &calls)?;
    raw_topology_contract(batch, small, &calls)?;
    raw_witness_contract(batch, &calls)?;
    assert_eq!(calls.totals(), [2, 3, 2, 5, 2, 2, 3, 4]);
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
    let first = input
        .get(..1)
        .ok_or_else(|| io::Error::other("capacity neighbor prefix"))?;
    let small = calls.batch(first)??;
    raw_helper_contracts(&input, &batch, &small)?;
    assert_eq!((calls.intervals.get(), calls.batches.get()), (16, 2));
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
    let empty = calls.batch(&[])??;
    let left = calls.batch(&[a])??;
    let right = calls.batch(&[b])??;
    raw_pipeline_contracts(&empty, &left, &right, &batch)?;
    assert_eq!(project(&neighbor), neighbor_before);
    assert_eq!(project(&input), before);
    assert_eq!((calls.intervals.get(), calls.batches.get()), (2, 5));
    Ok(())
}

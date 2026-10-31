// SPDX-License-Identifier: GPL-2.0-only
//! Genuine private decision-consumer relations for CHECK-7/8/14/25.
//!
//! Included only by the actual principal owner under cfg(test).
//! No private State, Selection, Witness or Decision representation is forged.
//! Two real flat histories supply four admitted public decisions. Selections
//! retain their actual checked batch entries as one private pair; independently
//! valid kind/label arguments are mixed only at the complete_decision receiver.
//!
//! Public upper20=[2,6,6,2,3,1,1,96,2,2,2,3,1,1,2,1,4,0,0,0].
//! Nominal complete20 differs only in getters=63. Two wrappers append at lengths
//! 1,2,3: twelve normalized prefix rows and two sufficient fits of size3.
//! Conservative append-based reservation: six fits and384 generated points.
//! Four advances retain their existing internal selector/clone/helper bounds.
//! Separately reserve SEVEN direct actual batch.select calls and SEVEN direct
//! complete_decision calls. Neither operation is labelled getter or advance.
//! Each selection's pair retains exactly the batch's original slice identity.
//! Their local attempt counters enforce7 and are asserted on healthy completion;
//! the unchanged20-field marker intentionally does not claim to observe them.
//! Reserve the complete fixed7 for each even on a failed public-marker prefix.
//! Pair construction adds seven batch.entries accessors; pair identity checks
//! add seven pointer comparisons. Direct complete_decision source upper remains
//! at most112 entry-label comparisons,112 supporter-label comparisons and14
//! selection.outcome probes. The actual
//! positives select original A at entry0; the three negatives reject beforehand.
//! At most two input records, three checked batches and no explicit state clone,
//! whole-state equality or formatting operation occur in this test.
#![forbid(unsafe_code)]

use std::{
    cell::Cell,
    error::Error,
    io::{self, Write},
    num::NonZeroU64,
};

use crate::{
    FixtureContext, FixtureHistory, FixtureMinimum, FixtureObservation, FixtureOutcomeKind,
    FixturePrincipalDecision, FixturePrincipalError, FixturePrincipalOutcomeKind,
    FixturePrincipalParameters, FixturePrincipalState, FixturePrincipalStratum,
    FixtureProjectedBatch, FixtureProjectedInterval, FixtureProjectionTerms,
    enclose_fixture_projection, project_fixture_anchor,
};
use time_domain::{MonotonicNs, PhaseOffsetQ33, UtcCoordinateQ32, exchange_arithmetic};

use super::{SameBatchSelection, complete_decision, select_same_batch};

/// Preserve actual producer/receiver failures separately from harness failures.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// The unchanged principal public-attempt marker field order.
type Recipe = [u64; 20];
/// Complete finite public-attempt reservation; source-only direct calls are above.
const UPPER: Recipe = [2, 6, 6, 2, 3, 1, 1, 96, 2, 2, 2, 3, 1, 1, 2, 1, 4, 0, 0, 0];
/// Literal completed public-attempt recipe, distinct from a failed prefix.
const COMPLETE: Recipe = [2, 6, 6, 2, 3, 1, 1, 63, 2, 2, 2, 3, 1, 1, 2, 1, 4, 0, 0, 0];

/// One test-local public marker and two bounded unlabelled source-only scopes.
struct Calls {
    /// Public attempts precharged in their original20 slots.
    public: [Cell<u64>; 20],
    /// Direct real selector attempts, reserved separately from the public marker.
    selections: Cell<u64>,
    /// Direct actual private decision receiver attempts, also separately reserved.
    decisions: Cell<u64>,
}

impl Calls {
    /// Start one fixed test with no operations or product effects.
    const fn new() -> Self {
        Self {
            public: [const { Cell::new(0) }; 20],
            selections: Cell::new(0),
            decisions: Cell::new(0),
        }
    }

    /// Precharge an actual public operation, preserving its unmodified result.
    /// # Errors
    /// Reject an invalid counter slot or exhaustion before the product call.
    fn call<T>(&self, slot: usize, call: impl FnOnce() -> T) -> TestResult<T> {
        let count = self
            .public
            .get(slot)
            .ok_or_else(|| io::Error::other("principal guard slot"))?;
        let upper = UPPER
            .get(slot)
            .ok_or_else(|| io::Error::other("principal guard upper"))?;
        let next = count
            .get()
            .checked_add(1)
            .ok_or_else(|| io::Error::other("principal guard overflow"))?;
        if next > *upper {
            return Err(io::Error::other("principal guard public budget").into());
        }
        count.set(next);
        Ok(call())
    }

    /// Count an ordinary immutable public accessor in the existing getter slot.
    /// # Errors
    /// Preserve pre-call accounting failure without evaluating the accessor.
    fn get<T>(&self, get: impl FnOnce() -> T) -> TestResult<T> {
        self.call(7, get)
    }

    /// Form one same-batch pair with one actual selector, never a detached result.
    /// # Errors
    /// Preserve pre-call budget failure or the actual paired selector's cause.
    fn select<'batch>(
        &self,
        batch: &'batch FixtureProjectedBatch<'_>,
        minimum: FixtureMinimum,
    ) -> TestResult<SameBatchSelection<'batch>> {
        charge_direct(&self.selections)?;
        Ok(select_same_batch(batch, minimum)?)
    }

    /// Pass independently genuine arguments to the actual private consumer.
    /// # Errors
    /// Preserve accounting failure; retain the consumer Result for exact assertions.
    fn complete<'batch>(
        &self,
        selected: SameBatchSelection<'batch>,
        kind_from: &FixturePrincipalDecision<'_>,
        label_from: &FixturePrincipalDecision<'_>,
    ) -> TestResult<Result<FixturePrincipalDecision<'batch>, FixturePrincipalError>> {
        let label = self.get(|| label_from.principal_label())?;
        let kind = self.get(|| kind_from.kind())?;
        let updated = self.get(|| kind_from.updated_source())?;
        charge_direct(&self.decisions)?;
        Ok(complete_decision(selected, label, kind, updated))
    }

    /// Assert the healthy whole recipe without product state formatting.
    /// # Panics
    /// A successful test must perform all authored public and direct attempts.
    fn assert_complete(&self) {
        assert_eq!(self.public.each_ref().map(Cell::get), COMPLETE);
        assert_eq!(self.selections.get(), 7);
        assert_eq!(self.decisions.get(), 7);
    }
}

impl Drop for Calls {
    /// Emit actual public prefixes on both normal return and assertion unwind.
    /// Missing/failed emission remains an observation gap, not a zero-work pass.
    fn drop(&mut self) {
        let c = self.public.each_ref().map(Cell::get);
        let _emission = writeln!(
            io::stderr().lock(),
            "CORE_PRINCIPAL_NEXT_PROFILE case=principal_private_decision_relations histories={} appends={} observations={} contexts={} monotonic={} coordinates={} exchanges={} getters={} terms={} projections={} enclosures={} batches={} minima={} parameters={} strata={} seeds={} advances={} state_clones={} value_equalities={} state_formats={}",
            c[0],
            c[1],
            c[2],
            c[3],
            c[4],
            c[5],
            c[6],
            c[7],
            c[8],
            c[9],
            c[10],
            c[11],
            c[12],
            c[13],
            c[14],
            c[15],
            c[16],
            c[17],
            c[18],
            c[19]
        );
    }
}

/// Bound each separately reserved direct receiver to seven actual attempts.
/// # Errors
/// Reject overflow/exhaustion before invoking the product receiver.
fn charge_direct(counter: &Cell<u64>) -> TestResult {
    let next = counter
        .get()
        .checked_add(1)
        .ok_or_else(|| io::Error::other("direct guard overflow"))?;
    if next > 7 {
        return Err(io::Error::other("direct guard budget").into());
    }
    counter.set(next);
    Ok(())
}

/// Admit one nonzero literal harness identity without introducing product trust.
/// # Errors
/// Reject zero rather than substituting an identity.
fn tag(raw: u64) -> TestResult<NonZeroU64> {
    NonZeroU64::new(raw).ok_or_else(|| io::Error::other("zero guard tag").into())
}

/// Produce one real three-row history, projection and enclosure in fixed space.
/// # Errors
/// Preserve actual producer/accounting failures without retry or replacement.
fn projected(
    p: &Calls,
    context: FixtureContext,
    phase: PhaseOffsetQ33,
    times: [MonotonicNs; 3],
    distance: f64,
) -> TestResult<FixtureProjectedInterval> {
    let mut history = p.call(0, || FixtureHistory::new(context))?;
    for at in times {
        let row = p.call(2, || FixtureObservation::new(context, at, phase, 1.0))??;
        p.call(1, || history.try_append(row))??;
    }
    let terms = p.call(8, || FixtureProjectionTerms::new(distance, 0.0, 0.0))??;
    let projection = p.call(9, || {
        project_fixture_anchor(&history, context, 0, times[0], times[2], terms)
    })??;
    Ok(p.call(10, || enclose_fixture_projection(&projection))??)
}

/// Observe a matching successful private result against independent trace facts.
/// # Errors
/// Preserve observation accounting failure.
/// # Panics
/// Kind, selected label, readiness, event, selector outcome and pointer must agree.
fn observe(
    p: &Calls,
    value: &FixturePrincipalDecision<'_>,
    kind: FixturePrincipalOutcomeKind,
    label: Option<NonZeroU64>,
    outcome: FixtureOutcomeKind,
    original_a: &FixtureProjectedInterval,
) -> TestResult {
    assert_eq!(p.get(|| value.kind())?, kind);
    assert_eq!(p.get(|| value.principal_label())?, label);
    assert_eq!(
        p.get(|| value.reference_ready())?,
        kind == FixturePrincipalOutcomeKind::ReferenceReady
    );
    assert_eq!(p.get(|| value.updated_source())?, None);
    let selection = p.get(|| value.selection())?;
    assert_eq!(p.get(|| selection.outcome())?, outcome);
    match p.get(|| value.principal_entry())? {
        Some(entry) => {
            assert!(label.is_some());
            assert!(std::ptr::eq(entry, original_a));
        }
        None => assert!(label.is_none()),
    }
    Ok(())
}

/// Keep the four accepted relations beside three independent mixed-value errors.
/// # Errors
/// Preserve actual selection/admission/observer errors in the healthy controls.
/// # Panics
/// Each mismatched relation must fail at the real private receiving boundary.
fn check_relations(
    p: &Calls,
    batches: [&FixtureProjectedBatch<'_>; 3],
    donors: [&FixturePrincipalDecision<'_>; 4],
    minimum: FixtureMinimum,
    a: NonZeroU64,
    original_a: &FixtureProjectedInterval,
) -> TestResult {
    let [full, only_b, empty] = batches;
    let [ready, retained, waiting, unavailable] = donors;
    for (batch, donor, kind, label, outcome) in [
        (
            full,
            ready,
            FixturePrincipalOutcomeKind::ReferenceReady,
            Some(a),
            FixtureOutcomeKind::WitnessedFixture,
        ),
        (
            full,
            retained,
            FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate,
            Some(a),
            FixtureOutcomeKind::WitnessedFixture,
        ),
        (
            only_b,
            waiting,
            FixturePrincipalOutcomeKind::WaitingForUpdate,
            None,
            FixtureOutcomeKind::WitnessedFixture,
        ),
        (
            empty,
            unavailable,
            FixturePrincipalOutcomeKind::Unavailable,
            None,
            FixtureOutcomeKind::Empty,
        ),
    ] {
        let selected = p.select(batch, minimum)?;
        let entries = p.get(|| batch.entries())?;
        assert!(std::ptr::eq(selected.entries, entries));
        let matched = p.complete(selected, donor, donor)??;
        observe(p, &matched, kind, label, outcome, original_a)?;
    }
    // A-only: Ready paired with a genuine unavailable None label.
    // B-only: Unavailable paired with an independently witnessed selection.
    // C-only: Waiting paired with an independently empty selection.
    for (batch, kind_from, label_from) in [
        (full, ready, unavailable),
        (empty, waiting, waiting),
        (full, unavailable, unavailable),
    ] {
        let selected = p.select(batch, minimum)?;
        let entries = p.get(|| batch.entries())?;
        assert!(std::ptr::eq(selected.entries, entries));
        let rejected = p.complete(selected, kind_from, label_from)?;
        assert!(matches!(rejected, Err(FixturePrincipalError::Invariant)));
    }
    Ok(())
}

/// Genuine decisions own their valid values; the private receiver owns relations.
/// # Errors
/// Preserve actual public producer/receiver errors and counter failures.
/// # Panics
/// Fixed-minimum trace facts, matching relations and cross-input errors must hold.
#[test]
fn principal_private_decision_relations() -> TestResult {
    let p = Calls::new();
    let roster = [tag(101)?, tag(202)?];
    let frame = tag(7)?;
    let generation = tag(11)?;
    let times = [
        p.call(4, || MonotonicNs::new(0))?,
        p.call(4, || MonotonicNs::new(1_000_000_000))?,
        p.call(4, || MonotonicNs::new(2_000_000_000))?,
    ];
    let zero = p.call(5, || UtcCoordinateQ32::new(0))??;
    let exchange = p.call(6, || exchange_arithmetic(zero, zero, zero, zero))??;
    let phase = p.get(|| exchange.offset())?;
    assert_eq!(p.get(|| phase.numerator())?, 0);
    let a = p.call(3, || FixtureContext::new(roster[0], frame, generation))?;
    let b = p.call(3, || FixtureContext::new(roster[1], frame, generation))?;
    let entries = [
        projected(&p, a, phase, times, 1.0)?,
        projected(&p, b, phase, times, 2.0)?,
    ];
    let strata = [
        p.call(14, || FixturePrincipalStratum::new(roster[0], 1))??,
        p.call(14, || FixturePrincipalStratum::new(roster[1], 1))??,
    ];
    let full = p.call(11, || {
        FixtureProjectedBatch::new(frame, generation, times[2], &entries)
    })??;
    let only_b = p.call(11, || {
        FixtureProjectedBatch::new(frame, generation, times[2], &entries[1..])
    })??;
    let empty = p.call(11, || {
        FixtureProjectedBatch::new(frame, generation, times[2], &[])
    })??;
    let minimum = p.call(12, || FixtureMinimum::new(1))??;
    let parameters = p.call(13, || FixturePrincipalParameters::new(0.25, 0.0))??;
    let mut state = p.call(15, || {
        FixturePrincipalState::seed(&roster, parameters, minimum, &full, &strata)
    })??;
    let ready = p.call(16, || state.advance(&full, &strata))??;
    let source_b = p
        .get(|| state.source_state(roster[1]))?
        .ok_or_else(|| io::Error::other("missing genuine B state"))?;
    assert_eq!(p.get(|| source_b.score())?.to_bits(), 1.0_f64.to_bits());
    assert!(!p.get(|| source_b.pending())?);
    let retained = p.call(16, || state.advance(&full, &strata))??;
    let waiting = p.call(16, || state.advance(&only_b, &strata[1..]))??;
    let unavailable = p.call(16, || state.advance(&empty, &[]))??;
    let held_minimum = p.get(|| state.minimum())?;
    assert_eq!(p.get(|| held_minimum.get())?, 1);
    check_relations(
        &p,
        [&full, &only_b, &empty],
        [&ready, &retained, &waiting, &unavailable],
        minimum,
        roster[0],
        &entries[0],
    )?;
    p.assert_complete();
    Ok(())
}

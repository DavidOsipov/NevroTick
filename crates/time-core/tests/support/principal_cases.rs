// SPDX-License-Identifier: GPL-2.0-only
//! Bounded independent CHECK-7/8/14/25 principal receiver cases.
//!
//! Authored and independently source-reviewed before execution. The existing
//! S1.3 task records current compiler, formatter and native observations.
//! This module supplies separate cases alongside the original first test.
//! Same genuine producer path: UTC/exchange -> exact observation -> constant
//! three-row history -> projection -> enclosure -> checked batch -> principal.
//! Every wrapper uses append occupied lengths 1,2,3: six prefix rows, one fit
//! of size3. At most sixteen wrappers per batch; finite literal loops only.
//!
//! Complete per-case public-call UPPER recipes appear at each Budget::new.
//! They are source reservations, not measured attempts; getter bounds include
//! deliberate slack. Actual-attempt counters emit on unwind if later admitted.
//! Source-work ledger (wrappers/builds/advance calls): threshold22/11/11,
//! pending24/8/8, lifecycle28/14/20, agreement29/10/15, cached11/4/4,
//! scalar0/0/0, metadata4/2/11, arithmetic6/3/4, cardinality36/5/5.
//! Total160 wrappers,480 append attempts, <=960 normalized-prefix rows,
//! 160 sufficient fits of size3, and78 advances. Distinct-value equality controls
//! add no histories, state clones, advances or protected effects.
//! New case upper20=[36,108,108,36,149,72,36,256,36,36,36,5,5,5,36,5,5,0,0,0],
//! nominal231 getters; this remains a source calculation, not an observation.
//! Cumulative upper20=[160,480,480,160,697,320,160,1348,160,160,160,58,18,38,
//! 166,22,78,27,37,12]. The new case has five cold seeds, no state clones,
//! comparisons or format calls, and no unobserved pre-reset score claim.
//! Each advance reserves existing selector work and <=82 scoring primitives;
//! seed makes no selector call. At most18 state clones/24 value comparisons in
//! a case, separately counted from ordinary getters. State clone/equality work
//! is bounded by all16 slots; source/parameter comparisons use their own bounded
//! fields. Parameter/source floating equality preserves exact bits. Distinct
//! legitimate values challenge always-true Params/SourceState/State equality.
//! Explicit state Debug controls are separately counted and print one fixed
//! redacted string; assertions never implicitly format complete product state.
//! These are unmeasured body bounds, not product CPU/memory observations.
//! At authorship, compilation/format/lint, actual runtime counts, private garde/read
//! observers, 16-source execution, arithmetic underflow branches, external privacy
//! consumers and coverage/mutation/fuzz were not established.
//! Admission must include this new target and its20-field marker parser; copying
//! the draft is not execution authorization or proof. Error priority assertions
//! were reconciled with current receiver source, not executed.
//!
//! No production defaults, freshness/quorum, authentication or clock authority.
#![forbid(unsafe_code)]

use std::{
    cell::Cell,
    error::Error,
    io::{self, Write},
    num::NonZeroU64,
};

use time_core::{
    ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitError, FixtureContext,
    FixtureHistory, FixtureMinimum, FixtureObservation, FixtureOutcomeKind,
    FixturePrincipalContextField, FixturePrincipalDecision, FixturePrincipalError,
    FixturePrincipalOutcomeKind, FixturePrincipalParameterField, FixturePrincipalParameters,
    FixturePrincipalScalarIssue, FixturePrincipalState, FixturePrincipalStep,
    FixturePrincipalStratum, FixtureProjectedBatch, FixtureProjectedInterval,
    FixtureProjectionTerms, enclose_fixture_projection, project_fixture_anchor,
};
use time_domain::{MonotonicNs, UtcCoordinateQ32, exchange_arithmetic};

/// Harness errors remain distinct from returned product admission errors.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Original17 fields plus state clones, value equalities and state Debug formats.
type Recipe = [u64; 20];

/// Bounded observed attempts, with explicit source-owned ceilings and case label.
struct Budget {
    /// Fixed authored marker label, never derived from raw/product error data.
    case: &'static str,
    /// Per-case public-call upper, not an assertion that this work executed.
    ceiling: Recipe,
    /// Actual attempts precharged before the corresponding operation.
    actual: [Cell<u64>; 20],
}

impl Budget {
    /// Start a local case; the caller supplies the complete finite upper recipe.
    const fn new(case: &'static str, ceiling: Recipe) -> Self {
        Self {
            case,
            ceiling,
            actual: [const { Cell::new(0) }; 20],
        }
    }

    /// Precharge one source-owned field, then preserve the real operation result.
    /// # Errors
    /// Invalid slots, overflow and exhaustion reject before the product call.
    fn call<T>(&self, slot: usize, call: impl FnOnce() -> T) -> TestResult<T> {
        let count = self
            .actual
            .get(slot)
            .ok_or_else(|| io::Error::other("draft counter"))?;
        let upper = self
            .ceiling
            .get(slot)
            .ok_or_else(|| io::Error::other("draft upper"))?;
        let next = count
            .get()
            .checked_add(1)
            .ok_or_else(|| io::Error::other("draft overflow"))?;
        if next > *upper {
            return Err(io::Error::other("draft budget exhausted").into());
        }
        count.set(next);
        Ok(call())
    }

    /// Precharge an ordinary immutable public accessor.
    /// # Errors
    /// Preserve pre-call accounting failure.
    fn get<T>(&self, get: impl FnOnce() -> T) -> TestResult<T> {
        self.call(7, get)
    }

    /// Clone the complete actual state, separately from ordinary getter counts.
    /// # Errors
    /// Preserve pre-call accounting failure.
    fn clone_state(&self, value: &FixturePrincipalState) -> TestResult<FixturePrincipalState> {
        self.call(17, || value.clone())
    }

    /// Compare complete bitwise product state, including hidden caches/metadata.
    /// # Errors
    /// Preserve pre-call accounting failure.
    /// # Panics
    /// Complete product state must equal the independent saved/control state.
    fn same(&self, value: &FixturePrincipalState, expected: &FixturePrincipalState) -> TestResult {
        assert!(self.call(18, || value == expected)?);
        Ok(())
    }

    /// Challenge actual equality using distinct legitimately produced values.
    /// # Errors
    /// Preserve pre-call accounting failure without invoking the comparison.
    /// # Panics
    /// Independent transition/bit facts require the actual values to be unequal.
    fn different<T: PartialEq + ?Sized>(&self, value: &T, previous: &T) -> TestResult {
        assert!(self.call(18, || value != previous)?);
        Ok(())
    }

    /// Reach actual product Debug under its own source-owned work field.
    /// # Errors
    /// Preserve pre-call accounting failure.
    /// # Panics
    /// Formatting must reveal only the admitted fixed type string.
    fn redacted(&self, value: &FixturePrincipalState) -> TestResult {
        assert_eq!(
            self.call(19, || format!("{value:?}"))?,
            "FixturePrincipalState { .. }"
        );
        Ok(())
    }
}

impl Drop for Budget {
    /// Emit actual attempted prefixes even when assertions unwind.
    /// Missing emission is an evidence gap, never a successful zero-work result.
    fn drop(&mut self) {
        let c = &self.actual;
        let line = format!(
            "CORE_PRINCIPAL_NEXT_PROFILE case={} histories={} appends={} observations={} contexts={} monotonic={} coordinates={} exchanges={} getters={} terms={} projections={} enclosures={} batches={} minima={} parameters={} strata={} seeds={} advances={} state_clones={} value_equalities={} state_formats={}\n",
            self.case,
            c[0].get(),
            c[1].get(),
            c[2].get(),
            c[3].get(),
            c[4].get(),
            c[5].get(),
            c[6].get(),
            c[7].get(),
            c[8].get(),
            c[9].get(),
            c[10].get(),
            c[11].get(),
            c[12].get(),
            c[13].get(),
            c[14].get(),
            c[15].get(),
            c[16].get(),
            c[17].get(),
            c[18].get(),
            c[19].get()
        );
        let _emission_result = io::stderr().lock().write_all(line.as_bytes());
    }
}

/// Admit one nonzero literal test tag without product trust or fallback.
/// # Errors
/// Reject zero before constructing any product fixture context.
fn tag(raw: u64) -> TestResult<NonZeroU64> {
    NonZeroU64::new(raw).ok_or_else(|| io::Error::other("zero draft fixture tag").into())
}

/// Literal same-history source facts, not product outputs used as an oracle.
#[derive(Clone, Copy)]
struct Source {
    /// Asserted source label.
    label: u64,
    /// Constant phase numerator over2^33, within the admitted exact phase range.
    phase: i128,
    /// Explicit nonnegative projected fixture distance.
    distance: f64,
    /// Actual final row time; must be at least two seconds for this helper.
    latest: u64,
    /// Explicit fixture stratum.
    stratum: u8,
}

/// Author one literal source record without product calls or validation bypass.
const fn src(label: u64, phase: i128, distance: f64, latest: u64, stratum: u8) -> Source {
    Source {
        label,
        phase,
        distance,
        latest,
        stratum,
    }
}

/// Checked wrappers/metadata storage; its later batch borrows these exact values.
struct Built {
    /// Complete bounded original wrapper prefix.
    entries: Vec<FixtureProjectedInterval>,
    /// One checked stratum item for every wrapper.
    metadata: Vec<FixturePrincipalStratum>,
    /// Caller-asserted context and exact common evaluation time.
    header: (NonZeroU64, NonZeroU64, MonotonicNs),
}

impl Built {
    /// Construct the real checked borrowed batch against its exact asserted header.
    /// # Errors
    /// Preserve actual receiving-boundary/accounting failure.
    fn batch(&self, p: &Budget) -> TestResult<FixtureProjectedBatch<'_>> {
        Ok(p.call(11, || {
            FixtureProjectedBatch::new(self.header.0, self.header.1, self.header.2, &self.entries)
        })??)
    }
}

/// Build <=16 genuine constant histories and their exact common-time wrappers.
/// # Errors
/// Preserve actual producer errors; reject impossible helper chronology.
/// # Panics
/// Helper capacity and exact phase facts are independent harness assumptions.
fn build(
    p: &Budget,
    sources: &[Source],
    frame: u64,
    generation: u64,
    at: u64,
) -> TestResult<Built> {
    assert!(sources.len() <= 16);
    let frame = tag(frame)?;
    let generation = tag(generation)?;
    let at = p.call(4, || MonotonicNs::new(at))?;
    let mut entries = Vec::with_capacity(sources.len());
    let mut metadata = Vec::with_capacity(sources.len());
    for source in sources {
        let label = tag(source.label)?;
        let context = p.call(3, || FixtureContext::new(label, frame, generation))?;
        let zero = p.call(5, || UtcCoordinateQ32::new(0))??;
        let shifted = p.call(5, || UtcCoordinateQ32::new(source.phase))??;
        let exchange = p.call(6, || exchange_arithmetic(zero, shifted, zero, zero))??;
        let phase = p.get(|| exchange.offset())?;
        assert_eq!(p.get(|| phase.numerator())?, source.phase);
        let oldest = source
            .latest
            .checked_sub(2_000_000_000)
            .ok_or_else(|| io::Error::other("draft oldest time"))?;
        let middle = source
            .latest
            .checked_sub(1_000_000_000)
            .ok_or_else(|| io::Error::other("draft middle time"))?;
        let mut history = p.call(0, || FixtureHistory::new(context))?;
        for raw in [oldest, middle, source.latest] {
            let observed = p.call(4, || MonotonicNs::new(raw))?;
            let row = p.call(2, || FixtureObservation::new(context, observed, phase, 1.0))??;
            p.call(1, || history.try_append(row))??;
        }
        let anchor = p.call(4, || MonotonicNs::new(oldest))?;
        let terms = p.call(8, || FixtureProjectionTerms::new(source.distance, 0.0, 0.0))??;
        let projection = p.call(9, || {
            project_fixture_anchor(&history, context, 0, anchor, at, terms)
        })??;
        entries.push(p.call(10, || enclose_fixture_projection(&projection))??);
        metadata.push(p.call(14, || FixturePrincipalStratum::new(label, source.stratum))??);
    }
    Ok(Built {
        entries,
        metadata,
        header: (frame, generation, at),
    })
}

/// Independent expected view of a completed decision and committed principal.
#[derive(Clone, Copy)]
struct View {
    /// Expected current principal, or none for an unavailable/waiting result.
    principal: Option<NonZeroU64>,
    /// Closed actual state-transition outcome.
    kind: FixturePrincipalOutcomeKind,
    /// Exactly one actual observation event, or none.
    updated: Option<NonZeroU64>,
    /// Real selector reason, retained without caller-supplied replacement.
    selection: FixtureOutcomeKind,
}

/// Observe real outcome/source-association fields using independent literal facts.
/// # Errors
/// Preserve observer budget failure.
/// # Panics
/// Actual completed fields must match the declared independent outcome.
fn view(
    p: &Budget,
    state: &FixturePrincipalState,
    value: &FixturePrincipalDecision<'_>,
    fact: View,
) -> TestResult {
    assert_eq!(p.get(|| state.principal())?, fact.principal);
    assert_eq!(p.get(|| value.principal_label())?, fact.principal);
    assert_eq!(p.get(|| value.kind())?, fact.kind);
    assert_eq!(p.get(|| value.updated_source())?, fact.updated);
    assert_eq!(
        p.get(|| value.reference_ready())?,
        fact.kind == FixturePrincipalOutcomeKind::ReferenceReady
    );
    let selection = p.get(|| value.selection())?;
    assert_eq!(p.get(|| selection.outcome())?, fact.selection);
    Ok(())
}

/// Observe one exact score without recomputing the algorithm as its oracle.
/// # Errors
/// Reject an unexpectedly missing registered source or exhausted budget.
/// # Panics
/// Returned scalar bits must match the independent dyadic score/reset fact.
fn score(
    p: &Budget,
    state: &FixturePrincipalState,
    label: NonZeroU64,
    expected: f64,
) -> TestResult {
    let source = p
        .get(|| state.source_state(label))?
        .ok_or_else(|| io::Error::other("draft source"))?;
    assert_eq!(p.get(|| source.score())?.to_bits(), expected.to_bits());
    Ok(())
}

/// Observe retained unconsumed credit independently of pair-event multiplication.
/// # Errors
/// Reject a missing source or exhausted budget.
/// # Panics
/// Actual pending state must match the independent consumption fact.
fn pending(
    p: &Budget,
    state: &FixturePrincipalState,
    label: NonZeroU64,
    expected: bool,
) -> TestResult {
    let source = p
        .get(|| state.source_state(label))?
        .ok_or_else(|| io::Error::other("draft source"))?;
    assert_eq!(p.get(|| source.pending())?, expected);
    Ok(())
}

/// Fixed explicit starting policy/assertions for one inert scenario.
#[derive(Clone, Copy)]
struct Start<'a> {
    /// Frozen ordered roster, including any deliberately unseen registered label.
    roster: &'a [NonZeroU64],
    /// Unchanged fixture minimum.
    minimum: usize,
    /// Explicit fixture parameters; no Default exists.
    parameters: [f64; 2],
    /// Literal initial evaluation instant.
    at: u64,
    /// Independently chosen initial principal label.
    principal: NonZeroU64,
}

/// Seed and select a healthy neighboring control through actual entry points.
/// # Errors
/// Preserve all product/observer errors without retry or fixture substitution.
fn start(
    p: &Budget,
    sources: &[Source],
    fact: Start<'_>,
) -> TestResult<(FixturePrincipalState, Built)> {
    let built = build(p, sources, 7, 11, fact.at)?;
    let batch = built.batch(p)?;
    let minimum = p.call(12, || FixtureMinimum::new(fact.minimum))??;
    let parameters = p.call(13, || {
        FixturePrincipalParameters::new(fact.parameters[0], fact.parameters[1])
    })??;
    let mut state = p.call(15, || {
        FixturePrincipalState::seed(fact.roster, parameters, minimum, &batch, &built.metadata)
    })??;
    let result = p.call(16, || state.advance(&batch, &built.metadata))??;
    view(
        p,
        &state,
        &result,
        View {
            principal: Some(fact.principal),
            kind: FixturePrincipalOutcomeKind::ReferenceReady,
            updated: None,
            selection: FixtureOutcomeKind::WitnessedFixture,
        },
    )?;
    Ok((state, built))
}

/// Independent exact10 and neighboring scores challenge strict reselection.
/// # Errors
/// Preserve real producer/consumer rejection and finite observer-budget failures.
/// # Panics
/// Threshold, update-credit and exact score/reset observations must match facts.
#[test]
fn principal_threshold_exact_and_neighbors() -> TestResult {
    let p = Budget::new(
        "principal_threshold_exact_and_neighbors",
        [
            22, 66, 66, 22, 99, 44, 22, 192, 22, 22, 22, 11, 3, 3, 22, 3, 11, 0, 0, 0,
        ],
    );
    let roster = [tag(101)?, tag(202)?];
    for (distance_a, candidate_score, switches) in [
        (79.0 / 16.0, 79.0 / 8.0, false),
        (5.0, 10.0, false),
        (81.0 / 16.0, 81.0 / 8.0, true),
    ] {
        let initial = [
            src(101, 0, distance_a, 2_000_000_000, 1),
            src(202, 0, 6.0, 2_000_000_000, 1),
        ];
        let (mut state, _) = start(
            &p,
            &initial,
            Start {
                roster: &roster,
                minimum: 2,
                parameters: [0.25, 0.0],
                at: 2_000_000_000,
                principal: roster[0],
            },
        )?;
        let first = [initial[0], src(202, 0, 0.25, 3_000_000_000, 1)];
        let built = build(&p, &first, 7, 11, 3_000_000_000)?;
        let batch = built.batch(&p)?;
        let result = p.call(16, || state.advance(&batch, &built.metadata))??;
        let principal = if switches { roster[1] } else { roster[0] };
        view(
            &p,
            &state,
            &result,
            View {
                principal: Some(principal),
                kind: if switches {
                    FixturePrincipalOutcomeKind::ReferenceReady
                } else {
                    FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
                },
                updated: Some(roster[1]),
                selection: FixtureOutcomeKind::WitnessedFixture,
            },
        )?;
        score(
            &p,
            &state,
            roster[1],
            if switches { 1.0 } else { candidate_score },
        )?;
        pending(&p, &state, roster[1], !switches)?;
        let replay = build(&p, &first, 7, 11, 3_500_000_000)?;
        let batch = replay.batch(&p)?;
        let result = p.call(16, || state.advance(&batch, &replay.metadata))??;
        view(
            &p,
            &state,
            &result,
            View {
                principal: Some(principal),
                kind: FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate,
                updated: None,
                selection: FixtureOutcomeKind::WitnessedFixture,
            },
        )?;
        score(
            &p,
            &state,
            roster[1],
            if switches { 1.0 } else { candidate_score },
        )?;
        pending(&p, &state, roster[1], !switches)?;
        if !switches {
            let next = [initial[0], src(202, 0, 0.25, 4_000_000_000, 1)];
            let built = build(&p, &next, 7, 11, 4_000_000_000)?;
            let batch = built.batch(&p)?;
            let result = p.call(16, || state.advance(&batch, &built.metadata))??;
            view(
                &p,
                &state,
                &result,
                View {
                    principal: Some(roster[1]),
                    kind: FixturePrincipalOutcomeKind::ReferenceReady,
                    updated: Some(roster[1]),
                    selection: FixtureOutcomeKind::WitnessedFixture,
                },
            )?;
            score(&p, &state, roster[1], 1.0)?;
            pending(&p, &state, roster[1], false)?;
        }
    }
    Ok(())
}

/// Unrelated source events cannot compound pair scores or forge pending credit.
/// # Errors
/// Preserve real producer/consumer rejection and finite observer-budget failures.
/// # Panics
/// Each literal event's principal, score and pending-credit state must match.
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "The fixed seven-event pending-credit trace stays together with its literal state expectations."
)]
fn principal_unrelated_event_and_pending_wait() -> TestResult {
    let p = Budget::new(
        "principal_unrelated_event_and_pending_wait",
        [
            24, 72, 72, 24, 104, 48, 24, 192, 24, 24, 24, 8, 1, 1, 24, 1, 8, 0, 0, 0,
        ],
    );
    let roster = [tag(101)?, tag(202)?, tag(303)?];
    let mut sources = [
        src(101, 0, 1.0, 2_000_000_000, 1),
        src(202, 0, 2.0, 2_000_000_000, 1),
        src(303, 0, 4.0, 2_000_000_000, 1),
    ];
    let (mut state, _) = start(
        &p,
        &sources,
        Start {
            roster: &roster,
            minimum: 2,
            parameters: [0.25, 0.0],
            at: 2_000_000_000,
            principal: roster[0],
        },
    )?;
    // Source index, new watermark, expected principal, expected outcome, optional B score.
    for (index, at, principal, kind, b_score) in [
        (
            1,
            3_000_000_000,
            Some(roster[0]),
            FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate,
            Some(2.0),
        ),
        (
            2,
            4_000_000_000,
            Some(roster[0]),
            FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate,
            Some(2.0),
        ),
        (
            0,
            5_000_000_000,
            Some(roster[0]),
            FixturePrincipalOutcomeKind::ReferenceReady,
            Some(4.0),
        ),
        (
            0,
            6_000_000_000,
            Some(roster[0]),
            FixturePrincipalOutcomeKind::ReferenceReady,
            Some(8.0),
        ),
        (
            0,
            7_000_000_000,
            None,
            FixturePrincipalOutcomeKind::WaitingForUpdate,
            None,
        ),
        (
            3,
            7_500_000_000,
            None,
            FixturePrincipalOutcomeKind::WaitingForUpdate,
            None,
        ),
        (
            1,
            8_000_000_000,
            Some(roster[1]),
            FixturePrincipalOutcomeKind::ReferenceReady,
            Some(1.0),
        ),
    ] {
        let updated = if index < 3 {
            sources[index].latest = at;
            if index == 1 {
                sources[index].distance = 0.25;
            }
            Some(roster[index])
        } else {
            None
        };
        let built = build(&p, &sources, 7, 11, at)?;
        let batch = built.batch(&p)?;
        let result = p.call(16, || state.advance(&batch, &built.metadata))??;
        view(
            &p,
            &state,
            &result,
            View {
                principal,
                kind,
                updated,
                selection: FixtureOutcomeKind::WitnessedFixture,
            },
        )?;
        if let Some(expected) = b_score {
            score(&p, &state, roster[1], expected)?;
        }
        pending(&p, &state, roster[1], at < 5_000_000_000)?;
    }
    // No pre-reset16 claim: waiting asserts no principal; final selected state resets1.
    Ok(())
}

/// Reject six stale/context/event cases atomically and preserve a useful neighbor.
/// # Errors
/// Preserve actual producer/observer failure; each intended receiver error is asserted.
/// # Panics
/// Exact rejection causes, unchanged state and valid recovery must match facts.
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "The six literal lifecycle rejections and their atomicity/recovery controls share one fixed trace."
)]
fn principal_lifecycle_rejection_is_atomic_and_recovers() -> TestResult {
    let p = Budget::new(
        "principal_lifecycle_rejection_is_atomic_and_recovers",
        [
            28, 84, 84, 28, 126, 56, 28, 204, 28, 28, 28, 14, 1, 1, 28, 1, 20, 18, 24, 6,
        ],
    );
    let roster = [tag(101)?, tag(202)?];
    let initial = [
        src(101, 0, 1.0, 4_000_000_000, 1),
        src(202, 0, 2.0, 4_000_000_000, 1),
    ];
    let (mut base, _) = start(
        &p,
        &initial,
        Start {
            roster: &roster,
            minimum: 2,
            parameters: [0.25, 0.0],
            at: 4_000_000_000,
            principal: roster[0],
        },
    )?;
    let later = build(&p, &initial, 7, 11, 5_000_000_000)?;
    let batch = later.batch(&p)?;
    let result = p.call(16, || base.advance(&batch, &later.metadata))??;
    view(
        &p,
        &base,
        &result,
        View {
            principal: Some(roster[0]),
            kind: FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate,
            updated: None,
            selection: FixtureOutcomeKind::WitnessedFixture,
        },
    )?;
    for case in 0..6 {
        let mut inputs = initial;
        let mut frame = 7;
        let mut generation = 11;
        let mut at = 6_000_000_000;
        let expected = match case {
            0 => {
                frame = 8;
                FixturePrincipalError::ContextMismatch {
                    field: FixturePrincipalContextField::Frame,
                }
            }
            1 => {
                generation = 12;
                FixturePrincipalError::ContextMismatch {
                    field: FixturePrincipalContextField::MappingGeneration,
                }
            }
            2 => {
                at = 4_500_000_000;
                FixturePrincipalError::BackwardEvaluation
            }
            3 => {
                inputs[0].latest = 3_000_000_000;
                FixturePrincipalError::BackwardObservation { index: 0 }
            }
            4 => {
                inputs[0].stratum = 2;
                FixturePrincipalError::StratumWithoutObservation { index: 0 }
            }
            _ => {
                inputs[0].latest = 6_000_000_000;
                inputs[1].latest = 6_000_000_000;
                FixturePrincipalError::MultipleObservations
            }
        };
        let mut state = p.clone_state(&base)?;
        let before = p.clone_state(&state)?;
        let mut control = p.clone_state(&state)?;
        p.redacted(&before)?;
        let bad = build(&p, &inputs, frame, generation, at)?;
        let batch = bad.batch(&p)?;
        let rejected = p.call(16, || state.advance(&batch, &bad.metadata))?;
        assert!(matches!(rejected,Err(error) if error==expected));
        p.same(&state, &before)?;
        let recovery = [initial[0], src(202, 0, 2.0, 6_000_000_000, 1)];
        let good = build(&p, &recovery, 7, 11, 6_000_000_000)?;
        let batch = good.batch(&p)?;
        let result = p.call(16, || state.advance(&batch, &good.metadata))??;
        let _control_decision = p.call(16, || control.advance(&batch, &good.metadata))??;
        p.same(&state, &control)?;
        view(
            &p,
            &state,
            &result,
            View {
                principal: Some(roster[0]),
                kind: FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate,
                updated: Some(roster[1]),
                selection: FixtureOutcomeKind::WitnessedFixture,
            },
        )?;
        pending(&p, &state, roster[1], true)?;
        // Recovery accepts B at6s after saved B at4s and saved evaluation at5s.
        // Compare those real values; no expected state or private field is forged.
        p.different(&state, &before)?;
        let recovered_b = p
            .get(|| state.source_state(roster[1]))?
            .ok_or_else(|| io::Error::other("missing recovered equality-control source"))?;
        let previous_b = p
            .get(|| before.source_state(roster[1]))?
            .ok_or_else(|| io::Error::other("missing saved equality-control source"))?;
        p.different(recovered_b, previous_b)?;
    }
    Ok(())
}

/// Five real selector-negative reasons clear principal without reducing minimum.
/// # Errors
/// Preserve real producer/consumer rejection and finite observer-budget failures.
/// # Panics
/// Selector reasons, fixed minimum, observation identity and recovery must match.
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "The five literal agreement-loss scenarios retain their expected reasons and recovery neighbors together."
)]
fn principal_agreement_loss_preserves_minimum_and_observation_identity() -> TestResult {
    let p = Budget::new(
        "principal_agreement_loss_preserves_minimum_and_observation_identity",
        [
            29, 87, 87, 29, 126, 58, 29, 224, 29, 29, 29, 10, 5, 5, 29, 5, 15, 0, 0, 0,
        ],
    );
    let roster = [tag(101)?, tag(202)?, tag(303)?, tag(404)?];
    for case in 0..5 {
        let common = [
            src(101, 0, 0.125, 2_000_000_000, 1),
            src(202, 0, 0.125, 2_000_000_000, 1),
            src(303, 0, 0.125, 2_000_000_000, 1),
            src(404, 0, 0.125, 2_000_000_000, 1),
        ];
        let (initial, next, minimum, expected, principal, event) = match case {
            0 => {
                let mut initial = common;
                initial[0].phase = -(1_i128 << 32);
                initial[1].phase = -(1_i128 << 32);
                initial[2].phase = 1_i128 << 32;
                initial[3].phase = -(1_i128 << 32);
                let mut next = initial;
                next[3].phase = 1_i128 << 32;
                next[3].latest = 3_000_000_000;
                (
                    initial.to_vec(),
                    next.to_vec(),
                    2,
                    FixtureOutcomeKind::NoMajority,
                    roster[0],
                    Some(roster[3]),
                )
            }
            1 => (
                common[..3].to_vec(),
                common[..2].to_vec(),
                3,
                FixtureOutcomeKind::InsufficientPopulation,
                roster[0],
                None,
            ),
            2 => {
                let mut next = common;
                next[3].phase = 1_i128 << 32;
                next[3].latest = 3_000_000_000;
                (
                    common.to_vec(),
                    next.to_vec(),
                    4,
                    FixtureOutcomeKind::InsufficientAgreement,
                    roster[0],
                    Some(roster[3]),
                )
            }
            3 => {
                let initial = [
                    src(101, 1_i128 << 31, 0.25, 2_000_000_000, 1),
                    src(202, 1_i128 << 29, 0.0625, 2_000_000_000, 1),
                    src(303, 1_i128 << 29, 0.0625, 2_000_000_000, 1),
                ];
                let mut next = initial;
                next[2].phase = 7_i128 << 29;
                next[2].latest = 3_000_000_000;
                (
                    initial.to_vec(),
                    next.to_vec(),
                    2,
                    FixtureOutcomeKind::UnsupportedAmbiguousFixture,
                    roster[1],
                    Some(roster[2]),
                )
            }
            _ => (
                common[..2].to_vec(),
                Vec::new(),
                2,
                FixtureOutcomeKind::Empty,
                roster[0],
                None,
            ),
        };
        let (mut state, _) = start(
            &p,
            &initial,
            Start {
                roster: &roster[..initial.len()],
                minimum,
                parameters: [0.25, 0.0],
                at: 2_000_000_000,
                principal,
            },
        )?;
        let bad = build(&p, &next, 7, 11, 3_000_000_000)?;
        let batch = bad.batch(&p)?;
        let first = p.call(16, || state.advance(&batch, &bad.metadata))??;
        view(
            &p,
            &state,
            &first,
            View {
                principal: None,
                kind: FixturePrincipalOutcomeKind::Unavailable,
                updated: event,
                selection: expected,
            },
        )?;
        let held_minimum = p.get(|| state.minimum())?;
        assert_eq!(p.get(|| held_minimum.get())?, minimum);
        if let Some(label) = event {
            pending(&p, &state, label, true)?;
        }
        let repeated = p.call(16, || state.advance(&batch, &bad.metadata))??;
        view(
            &p,
            &state,
            &repeated,
            View {
                principal: None,
                kind: FixturePrincipalOutcomeKind::Unavailable,
                updated: None,
                selection: expected,
            },
        )?;
    }
    Ok(())
}

/// Cached signed numerator changes forced replacement versus premature clearing.
/// # Errors
/// Preserve real producer/consumer rejection and finite observer-budget failures.
/// # Panics
/// Cached/present source ordering and the resulting state must match exact facts.
#[test]
fn principal_cached_absence_and_present_nonmember_keep_score_order() -> TestResult {
    let p = Budget::new(
        "principal_cached_absence_and_present_nonmember_keep_score_order",
        [
            11, 33, 33, 11, 48, 22, 11, 96, 11, 11, 11, 4, 1, 1, 11, 1, 4, 2, 0, 0,
        ],
    );
    let roster = [tag(101)?, tag(202)?, tag(303)?];
    let initial = [
        src(101, 1_i128 << 32, 0.0625, 2_000_000_000, 1),
        src(202, 1_i128 << 32, 0.25, 2_000_000_000, 3),
        src(303, 1_i128 << 32, 0.125, 2_000_000_000, 3),
    ];
    let (mut base, _) = start(
        &p,
        &initial,
        Start {
            roster: &roster,
            minimum: 2,
            parameters: [0.25, 0.125],
            at: 2_000_000_000,
            principal: roster[0],
        },
    )?;
    let mut moved = initial;
    moved[1].phase = -(1_i128 << 32);
    moved[1].latest = 3_000_000_000;
    let built = build(&p, &moved, 7, 11, 3_000_000_000)?;
    let batch = built.batch(&p)?;
    let result = p.call(16, || base.advance(&batch, &built.metadata))??;
    view(
        &p,
        &base,
        &result,
        View {
            principal: Some(roster[0]),
            kind: FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate,
            updated: Some(roster[1]),
            selection: FixtureOutcomeKind::WitnessedFixture,
        },
    )?;
    pending(&p, &base, roster[1], true)?;
    moved[2].phase = -(1_i128 << 32);
    moved[2].latest = 4_000_000_000;
    for absent in [false, true] {
        let mut state = p.clone_state(&base)?;
        // Reverse the absent branch's wrappers/metadata: fixed-roster B still
        // wins the score tie even though C is first in the actual input batch.
        let reversed = [moved[2], moved[1]];
        let sources = if absent { &reversed[..] } else { &moved[..] };
        let built = build(&p, sources, 7, 11, 4_000_000_000)?;
        let batch = built.batch(&p)?;
        let result = p.call(16, || state.advance(&batch, &built.metadata))??;
        // Old A numerator 1/16 +(1-3)/8 = -3/16. Only C updates;
        // C floors1 and B already1, so roster-first B wins with pending credit.
        // Premature clearing yields inverse distances B=2,C=8/3 and chooses C.
        view(
            &p,
            &state,
            &result,
            View {
                principal: Some(roster[1]),
                kind: FixturePrincipalOutcomeKind::ReferenceReady,
                updated: Some(roster[2]),
                selection: FixtureOutcomeKind::WitnessedFixture,
            },
        )?;
        score(&p, &state, roster[1], 1.0)?;
        score(&p, &state, roster[2], 1.0)?;
        pending(&p, &state, roster[1], false)?;
        pending(&p, &state, roster[2], false)?;
    }
    Ok(())
}

/// Raw scalar/stratum admission has useful valid neighbors and fixed precedence.
/// # Errors
/// Preserve observer-budget failures and unexpected rejection of valid neighbors.
/// # Panics
/// Invalid scalar/stratum causes and preserved valid values must match literal facts.
#[test]
fn principal_raw_scalar_and_stratum_neighbors() -> TestResult {
    let p = Budget::new(
        "principal_raw_scalar_and_stratum_neighbors",
        [0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 20, 5, 0, 0, 0, 1, 0],
    );
    let r = FixturePrincipalParameterField::ReselectDistanceSeconds;
    let w = FixturePrincipalParameterField::StratumWeightSeconds;
    for (raw_r, raw_w, field, issue) in [
        (f64::NAN, 0.0, r, FixturePrincipalScalarIssue::NonFinite),
        (
            f64::INFINITY,
            0.0,
            r,
            FixturePrincipalScalarIssue::NonFinite,
        ),
        (
            f64::from_bits(1),
            0.0,
            r,
            FixturePrincipalScalarIssue::Subnormal,
        ),
        (-1.0, 0.0, r, FixturePrincipalScalarIssue::Negative),
        (0.0, 0.0, r, FixturePrincipalScalarIssue::NonPositive),
        (-0.0, 0.0, r, FixturePrincipalScalarIssue::NonPositive),
        (
            0.25,
            f64::from_bits(0x8000_0000_0000_0001),
            w,
            FixturePrincipalScalarIssue::Subnormal,
        ),
        (0.25, -1.0, w, FixturePrincipalScalarIssue::Negative),
        (0.25, f64::NAN, w, FixturePrincipalScalarIssue::NonFinite),
    ] {
        let rejected = p.call(13, || FixturePrincipalParameters::new(raw_r, raw_w))?;
        assert!(
            matches!(rejected,Err(FixturePrincipalError::InvalidParameter {field:actual_field,issue:actual_issue}) if actual_field==field && actual_issue==issue)
        );
        let valid = p.call(13, || FixturePrincipalParameters::new(0.25, 0.0))??;
        assert_eq!(
            p.get(|| valid.reselect_distance_seconds())?.to_bits(),
            0.25_f64.to_bits()
        );
        assert_eq!(
            p.get(|| valid.stratum_weight_seconds())?.to_bits(),
            0.0_f64.to_bits()
        );
    }
    let signed = p.call(13, || FixturePrincipalParameters::new(0.25, -0.0))??;
    assert_eq!(
        p.get(|| signed.reselect_distance_seconds())?.to_bits(),
        0.25_f64.to_bits()
    );
    assert_eq!(
        p.get(|| signed.stratum_weight_seconds())?.to_bits(),
        (-0.0_f64).to_bits()
    );
    let positive_zero = p.call(13, || FixturePrincipalParameters::new(0.25, 0.0))??;
    p.different(&positive_zero, &signed)?;
    let label = tag(u64::MAX)?;
    for raw in [0, 16, u8::MAX] {
        assert!(matches!(
            p.call(14, || FixturePrincipalStratum::new(label, raw))?,
            Err(FixturePrincipalError::InvalidStratum)
        ));
    }
    for raw in [1, 15] {
        let valid = p.call(14, || FixturePrincipalStratum::new(label, raw))??;
        assert_eq!(p.get(|| valid.source_id())?, label);
        assert_eq!(p.get(|| valid.stratum())?, raw);
    }
    Ok(())
}

/// Actual raw relationship receivers reject malformed maps/rosters atomically.
/// # Errors
/// Preserve producer/observer failure and unexpected rejection of healthy controls.
/// # Panics
/// Exact metadata/roster errors, state preservation and seed credit must match facts.
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "The bounded malformed roster/map inventory stays with its unchanged-state and accepted-seed neighbors."
)]
fn principal_metadata_and_roster_rejections_preserve_state() -> TestResult {
    let p = Budget::new(
        "principal_metadata_and_roster_rejections_preserve_state",
        [
            4, 12, 12, 4, 18, 8, 4, 96, 4, 4, 4, 3, 1, 1, 5, 5, 11, 5, 10, 5,
        ],
    );
    let roster = [tag(101)?, tag(202)?, tag(303)?];
    let initial = [
        src(101, 0, 1.0, 2_000_000_000, 1),
        src(202, 0, 2.0, 2_000_000_000, 1),
    ];
    let (mut state, good) = start(
        &p,
        &initial,
        Start {
            roster: &roster,
            minimum: 2,
            parameters: [0.25, 0.0],
            at: 2_000_000_000,
            principal: roster[0],
        },
    )?;
    let batch = good.batch(&p)?;
    let unknown = build(
        &p,
        &[initial[0], src(u64::MAX, 0, 2.0, 2_000_000_000, 1)],
        7,
        11,
        2_000_000_000,
    )?;
    let unknown_batch = unknown.batch(&p)?;
    let extra = p.call(14, || FixturePrincipalStratum::new(roster[2], 1))??;
    for case in 0..5 {
        let before = p.clone_state(&state)?;
        p.redacted(&before)?;
        let (candidate, metadata, expected) = match case {
            0 => (
                &batch,
                vec![good.metadata[0], good.metadata[0]],
                FixturePrincipalError::DuplicateMetadata { index: 1 },
            ),
            1 => (
                &batch,
                vec![good.metadata[0]],
                FixturePrincipalError::MetadataCountMismatch,
            ),
            2 => (
                &batch,
                vec![good.metadata[0], extra],
                FixturePrincipalError::MissingMetadata { index: 1 },
            ),
            3 => (
                &batch,
                vec![good.metadata[0]; 17],
                FixturePrincipalError::MetadataCapacityExceeded,
            ),
            _ => (
                &unknown_batch,
                unknown.metadata.clone(),
                FixturePrincipalError::UnknownSource { index: 1 },
            ),
        };
        let rejected = p.call(16, || state.advance(candidate, &metadata))?;
        assert!(matches!(rejected,Err(error) if error==expected));
        p.same(&state, &before)?;
        let control = p.call(16, || state.advance(&batch, &good.metadata))??;
        view(
            &p,
            &state,
            &control,
            View {
                principal: Some(roster[0]),
                kind: FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate,
                updated: None,
                selection: FixtureOutcomeKind::WitnessedFixture,
            },
        )?;
        p.same(&state, &before)?;
    }
    let parameters = p.get(|| state.parameters())?;
    let minimum = p.get(|| state.minimum())?;
    for (bad_roster, expected) in [
        (Vec::new(), FixturePrincipalError::EmptyRoster),
        (
            vec![roster[0]; 17],
            FixturePrincipalError::RosterCapacityExceeded,
        ),
        (
            vec![roster[0], roster[0]],
            FixturePrincipalError::DuplicateRoster { index: 1 },
        ),
    ] {
        let rejected = p.call(15, || {
            FixturePrincipalState::seed(&bad_roster, parameters, minimum, &batch, &good.metadata)
        })?;
        assert!(matches!(rejected,Err(error) if error==expected));
    }
    let seeded = p.call(15, || {
        FixturePrincipalState::seed(&roster, parameters, minimum, &batch, &good.metadata)
    })??;
    assert_eq!(p.get(|| seeded.principal())?, None);
    pending(&p, &seeded, roster[0], true)?;
    pending(&p, &seeded, roster[1], true)?;
    Ok(())
}

/// Real finite inputs reach principal overflow; failure preserves every field.
/// # Errors
/// Preserve producer/observer failure and unexpected rejection of recovery controls.
/// # Panics
/// The exact numeric cause, whole-state atomicity and valid recovery must match facts.
#[test]
fn principal_numeric_rejection_is_atomic_and_recovers() -> TestResult {
    let p = Budget::new(
        "principal_numeric_rejection_is_atomic_and_recovers",
        [
            6, 18, 18, 6, 27, 12, 6, 64, 6, 6, 6, 3, 1, 1, 6, 1, 4, 2, 2, 1,
        ],
    );
    let roster = [tag(101)?, tag(202)?];
    let initial = [
        src(101, 0, 1.0, 2_000_000_000, 1),
        src(202, 0, 2.0, 2_000_000_000, 1),
    ];
    // Both initial strata1 => zero penalty despite the explicit large weight.
    let (mut state, _) = start(
        &p,
        &initial,
        Start {
            roster: &roster,
            minimum: 2,
            parameters: [0.25, 1.0e308],
            at: 2_000_000_000,
            principal: roster[0],
        },
    )?;
    let before = p.clone_state(&state)?;
    let mut control = p.clone_state(&state)?;
    p.redacted(&before)?;
    // Root distances remain valid/enclosable. New B stratum3 makes the actual
    // receiving scorer evaluate (3-1)*1e308, whose multiplication is nonfinite.
    let bad = build(
        &p,
        &[initial[0], src(202, 0, 2.0, 3_000_000_000, 3)],
        7,
        11,
        3_000_000_000,
    )?;
    let batch = bad.batch(&p)?;
    let rejected = p.call(16, || state.advance(&batch, &bad.metadata))?;
    assert!(matches!(
        rejected,
        Err(FixturePrincipalError::Arithmetic {
            step: FixturePrincipalStep::StratumPenalty,
            source: FitError::Arithmetic {
                stage: ArithmeticStage::Principal,
                operation: ArithmeticOperation::Multiply,
                issue: ArithmeticIssue::NonFinite
            },
        })
    ));
    p.same(&state, &before)?;
    let good = build(
        &p,
        &[initial[0], src(202, 0, 2.0, 3_000_000_000, 1)],
        7,
        11,
        3_000_000_000,
    )?;
    let batch = good.batch(&p)?;
    let result = p.call(16, || state.advance(&batch, &good.metadata))??;
    let _control_result = p.call(16, || control.advance(&batch, &good.metadata))??;
    p.same(&state, &control)?;
    view(
        &p,
        &state,
        &result,
        View {
            principal: Some(roster[0]),
            kind: FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate,
            updated: Some(roster[1]),
            selection: FixtureOutcomeKind::WitnessedFixture,
        },
    )?;
    pending(&p, &state, roster[1], true)?;
    Ok(())
}

/// Exercise a genuine positive cardinality neighbor without fake slot padding.
/// # Errors
/// Preserve real producer/consumer errors or impossible harness prefix admission.
/// # Panics
/// Real selection, retained minimum and original zero distance must match facts.
fn cardinality_neighbor(p: &Budget, roster: &[NonZeroU64], count: usize) -> TestResult {
    let occupied = roster
        .get(..count)
        .ok_or_else(|| io::Error::other("cardinality prefix"))?;
    let first = *occupied
        .first()
        .ok_or_else(|| io::Error::other("empty cardinality prefix"))?;
    let sources: Vec<Source> = occupied
        .iter()
        .enumerate()
        .map(|(index, label)| {
            src(
                label.get(),
                0,
                if index == 0 { 0.0 } else { 1.0 },
                2_000_000_000,
                1,
            )
        })
        .collect();
    let built = build(p, &sources, 7, 11, 2_000_000_000)?;
    let batch = built.batch(p)?;
    let minimum = p.call(12, || FixtureMinimum::new(count))??;
    let parameters = p.call(13, || FixturePrincipalParameters::new(0.25, 0.0))??;
    let mut state = p.call(15, || {
        FixturePrincipalState::seed(occupied, parameters, minimum, &batch, &built.metadata)
    })??;
    let result = p.call(16, || state.advance(&batch, &built.metadata))??;
    view(
        p,
        &state,
        &result,
        View {
            principal: Some(first),
            kind: FixturePrincipalOutcomeKind::ReferenceReady,
            updated: None,
            selection: FixtureOutcomeKind::WitnessedFixture,
        },
    )?;
    let selected = p.get(|| result.selection())?;
    assert_eq!(p.get(|| selected.population())?, count);
    assert_eq!(p.get(|| selected.maximum_depth())?, count);
    assert_eq!(p.get(|| selected.minimum())?, count);
    assert!(p.get(|| selected.strict_majority())?);
    assert!(p.get(|| selected.minimum_met())?);
    assert_eq!(p.get(|| state.roster_len())?, count);
    let held_minimum = p.get(|| state.minimum())?;
    assert_eq!(p.get(|| held_minimum.get())?, count);
    let original = p
        .get(|| result.principal_entry())?
        .ok_or_else(|| io::Error::other("missing zero-distance principal"))?;
    assert!(std::ptr::eq(original, &raw const built.entries[0]));
    let projection = p.get(|| original.projection())?;
    assert_eq!(
        p.get(|| projection.distance_seconds())?.to_bits(),
        0.0_f64.to_bits()
    );
    let cached = p
        .get(|| state.source_state(first))?
        .ok_or_else(|| io::Error::other("missing cached zero-distance principal"))?;
    assert_eq!(
        p.get(|| cached.distance_seconds())?.map(f64::to_bits),
        Some(0.0_f64.to_bits())
    );
    for &label in occupied {
        pending(p, &state, label, false)?;
    }
    Ok(())
}

/// Genuine1/15/16 admission and paired finite weight semantics use cold seeds.
/// No Default, artificial padding, caller-selected survivors or unchecked values.
/// # Errors
/// Preserve real producer/consumer rejection and finite observer-budget failures.
/// # Panics
/// Cardinality, zero-distance, fixed-parameter and stratum-weight facts must match.
#[test]
fn principal_cardinality_zero_distance_and_stratum_weight_neighbors() -> TestResult {
    let p = Budget::new(
        "principal_cardinality_zero_distance_and_stratum_weight_neighbors",
        [
            36, 108, 108, 36, 149, 72, 36, 256, 36, 36, 36, 5, 5, 5, 36, 5, 5, 0, 0, 0,
        ],
    );
    let roster = [
        tag(101)?,
        tag(102)?,
        tag(103)?,
        tag(104)?,
        tag(105)?,
        tag(106)?,
        tag(107)?,
        tag(108)?,
        tag(109)?,
        tag(110)?,
        tag(111)?,
        tag(112)?,
        tag(113)?,
        tag(114)?,
        tag(115)?,
        tag(116)?,
    ];
    for count in [1_usize, 15, 16] {
        cardinality_neighbor(&p, &roster, count)?;
    }
    let paired = [
        src(101, 0, 0.25, 2_000_000_000, 3),
        src(102, 0, 0.75, 2_000_000_000, 1),
    ];
    for (weight, principal) in [(0.0_f64, roster[0]), (0.5_f64, roster[1])] {
        // w0: denominators A=1/2,B=1 -> A. w1/2: A=3/2,B=1 -> B.
        // Input observations/distances/strata/minimum/reselection remain identical.
        let (state, _built) = start(
            &p,
            &paired,
            Start {
                roster: &roster[..2],
                minimum: 2,
                parameters: [0.25, weight],
                at: 2_000_000_000,
                principal,
            },
        )?;
        let parameters = p.get(|| state.parameters())?;
        assert_eq!(
            p.get(|| parameters.reselect_distance_seconds())?.to_bits(),
            0.25_f64.to_bits()
        );
        assert_eq!(
            p.get(|| parameters.stratum_weight_seconds())?.to_bits(),
            weight.to_bits()
        );
        for (label, distance, stratum) in [(roster[0], 0.25_f64, 3_u8), (roster[1], 0.75_f64, 1_u8)]
        {
            let held = p
                .get(|| state.source_state(label))?
                .ok_or_else(|| io::Error::other("missing weighted control source"))?;
            assert_eq!(
                p.get(|| held.distance_seconds())?.map(f64::to_bits),
                Some(distance.to_bits())
            );
            assert_eq!(p.get(|| held.stratum())?, Some(stratum));
        }
    }
    Ok(())
}

// Added for the observed principal mutation gaps; execution is recorded in S1.3.

/// Public error variants expose their exact embedded typed cause by reference.
/// This is Error-API coverage, not a claim that a valid selector can fail.
/// # Errors
/// Preserve counter exhaustion and reject an unexpectedly missing/wrong cause.
/// # Panics
/// Delegates must return the same borrow; every nondelegated family has no cause.
#[test]
fn principal_error_sources_preserve_embedded_identity() -> TestResult {
    use time_core::{SelectionError, SelectionInvariantStage};

    let p = Budget::new(
        "principal_error_sources_preserve_embedded_identity",
        [0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    );
    let arithmetic = FixturePrincipalError::Arithmetic {
        step: FixturePrincipalStep::UpdatedScore,
        source: FitError::Arithmetic {
            stage: ArithmeticStage::Principal,
            operation: ArithmeticOperation::Multiply,
            issue: ArithmeticIssue::NonFinite,
        },
    };
    let FixturePrincipalError::Arithmetic {
        source: expected, ..
    } = &arithmetic
    else {
        return Err(io::Error::other("arithmetic variant fixture").into());
    };
    let actual = p
        .get(|| arithmetic.source())?
        .ok_or_else(|| io::Error::other("missing arithmetic cause"))?;
    let actual = actual
        .downcast_ref::<FitError>()
        .ok_or_else(|| io::Error::other("wrong arithmetic cause type"))?;
    assert!(std::ptr::eq(actual, expected));

    let selection = FixturePrincipalError::Selection {
        source: SelectionError::Invariant {
            stage: SelectionInvariantStage::Slot,
        },
    };
    let FixturePrincipalError::Selection { source: expected } = &selection else {
        return Err(io::Error::other("selection variant fixture").into());
    };
    let actual = p
        .get(|| selection.source())?
        .ok_or_else(|| io::Error::other("missing selection cause"))?;
    let actual = actual
        .downcast_ref::<SelectionError>()
        .ok_or_else(|| io::Error::other("wrong selection cause type"))?;
    assert!(std::ptr::eq(actual, expected));

    for error in [
        FixturePrincipalError::InvalidParameter {
            field: FixturePrincipalParameterField::ReselectDistanceSeconds,
            issue: FixturePrincipalScalarIssue::NonPositive,
        },
        FixturePrincipalError::InvalidStratum,
        FixturePrincipalError::EmptyRoster,
        FixturePrincipalError::RosterCapacityExceeded,
        FixturePrincipalError::DuplicateRoster { index: 1 },
        FixturePrincipalError::MetadataCapacityExceeded,
        FixturePrincipalError::MetadataCountMismatch,
        FixturePrincipalError::UnknownSource { index: 1 },
        FixturePrincipalError::DuplicateMetadata { index: 1 },
        FixturePrincipalError::MissingMetadata { index: 1 },
        FixturePrincipalError::ContextMismatch {
            field: FixturePrincipalContextField::Frame,
        },
        FixturePrincipalError::BackwardEvaluation,
        FixturePrincipalError::BackwardObservation { index: 1 },
        FixturePrincipalError::MultipleObservations,
        FixturePrincipalError::StratumWithoutObservation { index: 1 },
        FixturePrincipalError::Invariant,
    ] {
        assert!(p.get(|| error.source())?.is_none());
    }
    Ok(())
}

/// Context belongs to whole state; source equality still owns pending credit.
/// # Errors
/// Preserve genuine producer/receiver rejection and bounded observer failures.
/// # Panics
/// Only frame differs at seed; after one advance only source pending differs.
#[test]
fn principal_equality_distinguishes_context_and_pending_credit() -> TestResult {
    let p = Budget::new(
        "principal_equality_distinguishes_context_and_pending_credit",
        [2, 6, 6, 2, 10, 4, 2, 40, 2, 2, 2, 2, 1, 1, 2, 2, 1, 1, 4, 0],
    );
    let roster = [tag(101)?];
    let sources = [src(101, 0, 1.0, 2_000_000_000, 1)];
    let left_built = build(&p, &sources, 7, 11, 2_000_000_000)?;
    let right_built = build(&p, &sources, 8, 11, 2_000_000_000)?;
    let left_batch = left_built.batch(&p)?;
    let right_batch = right_built.batch(&p)?;
    let minimum = p.call(12, || FixtureMinimum::new(1))??;
    let parameters = p.call(13, || FixturePrincipalParameters::new(0.25, 0.0))??;
    let mut left = p.call(15, || {
        FixturePrincipalState::seed(
            &roster,
            parameters,
            minimum,
            &left_batch,
            &left_built.metadata,
        )
    })??;
    let right = p.call(15, || {
        FixturePrincipalState::seed(
            &roster,
            parameters,
            minimum,
            &right_batch,
            &right_built.metadata,
        )
    })??;
    assert_eq!(p.get(|| left.principal())?, None);
    assert_eq!(p.get(|| right.principal())?, None);
    let saved = p.clone_state(&left)?;
    p.same(&left, &saved)?;
    p.different(&left, &right)?;
    let right_source = p
        .get(|| right.source_state(roster[0]))?
        .ok_or_else(|| io::Error::other("missing right source"))?;
    let left_source = p
        .get(|| left.source_state(roster[0]))?
        .ok_or_else(|| io::Error::other("missing left source"))?;
    assert!(p.call(18, || left_source == right_source)?);

    // Same original observation, scalar metadata and evaluation time: no event.
    let result = p.call(16, || left.advance(&left_batch, &left_built.metadata))??;
    view(
        &p,
        &left,
        &result,
        View {
            principal: Some(roster[0]),
            kind: FixturePrincipalOutcomeKind::ReferenceReady,
            updated: None,
            selection: FixtureOutcomeKind::WitnessedFixture,
        },
    )?;
    let after = p
        .get(|| left.source_state(roster[0]))?
        .ok_or_else(|| io::Error::other("missing advanced source"))?;
    for source in [after, right_source] {
        assert_eq!(p.get(|| source.source_id())?, roster[0]);
        assert_eq!(p.get(|| source.score())?.to_bits(), 1.0_f64.to_bits());
        assert_eq!(
            p.get(|| source.latest_observed_at())?,
            Some(left_built.header.2)
        );
        assert_eq!(
            p.get(|| source.distance_seconds())?.map(f64::to_bits),
            Some(1.0_f64.to_bits())
        );
        assert_eq!(p.get(|| source.stratum())?, Some(1));
    }
    assert!(!p.get(|| after.pending())?);
    assert!(p.get(|| right_source.pending())?);
    p.different(after, right_source)?;
    Ok(())
}

/// A fresh inferior challenger floors its exact one-half score back to one.
/// # Errors
/// Preserve genuine producer/receiver rejection and bounded observer failures.
/// # Panics
/// Literal event, floor, principal and same-observation preservation must agree.
#[test]
fn principal_inferior_update_floors_score_without_reprojection_credit() -> TestResult {
    let p = Budget::new(
        "principal_inferior_update_floors_score_without_reprojection_credit",
        [
            4, 12, 12, 4, 18, 8, 4, 64, 4, 4, 4, 2, 1, 1, 4, 1, 3, 1, 1, 0,
        ],
    );
    let roster = [tag(101)?, tag(202)?];
    let mut sources = [
        src(101, 0, 1.0, 2_000_000_000, 1),
        src(202, 0, 1.75, 2_000_000_000, 1),
    ];
    let (mut state, _initial) = start(
        &p,
        &sources,
        Start {
            roster: &roster,
            minimum: 2,
            parameters: [0.25, 0.0],
            at: 2_000_000_000,
            principal: roster[0],
        },
    )?;
    score(&p, &state, roster[1], 1.0)?;
    pending(&p, &state, roster[1], false)?;

    // A numerator1, B denominator(7/4+1/4)=2: product1*(1/2), floor1.
    sources[1].latest = 3_000_000_000;
    let built = build(&p, &sources, 7, 11, 3_000_000_000)?;
    let batch = built.batch(&p)?;
    let result = p.call(16, || state.advance(&batch, &built.metadata))??;
    view(
        &p,
        &state,
        &result,
        View {
            principal: Some(roster[0]),
            kind: FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate,
            updated: Some(roster[1]),
            selection: FixtureOutcomeKind::WitnessedFixture,
        },
    )?;
    score(&p, &state, roster[1], 1.0)?;
    pending(&p, &state, roster[0], false)?;
    pending(&p, &state, roster[1], true)?;
    let saved = p.clone_state(&state)?;

    let repeated = p.call(16, || state.advance(&batch, &built.metadata))??;
    view(
        &p,
        &state,
        &repeated,
        View {
            principal: Some(roster[0]),
            kind: FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate,
            updated: None,
            selection: FixtureOutcomeKind::WitnessedFixture,
        },
    )?;
    score(&p, &state, roster[1], 1.0)?;
    pending(&p, &state, roster[1], true)?;
    p.same(&state, &saved)?;
    Ok(())
}

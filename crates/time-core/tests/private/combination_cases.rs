// SPDX-License-Identifier: GPL-2.0-only
//! Independent CHECK-7/8/14/25 combination-kernel consumers.
//!
//! Bounded cfg(test) unit module at tests/private/combination_cases.rs; this file
//! uses genuine checked producers and never constructs private representations.
//! Genuine three-row histories supply actual latest-anchor projections, exact
//! enclosures, checked batches and principal decisions. Explicit uncertainty
//! assertions are at that same instant, separate from projection growth skew.
//!
//! Closed22-field order is the existing principal20 fields followed by
//! uncertainties and combinations. Per-case UPPER recipes precede every call.
//! Twelve complete authored recipes (not execution observations) are the literal
//!22-field arrays at each case entry. Their aggregate is
//! [43,129,129,43,145,516,129,977,43,43,43,14,14,14,43,14,15,0,0,5,132,60].
//! Forty-three histories/129 appends yield258 normalized prefix rows and43
//! sufficient fits of size3. Conservative append-based upper129 fits/8256 points.
//! Fifteen principal advances retain selector/implicit-clone/source bounds;
//! the kernel owns separate fixed<=16 arithmetic/metadata bounds. These callers
//! invoke no public low-level square-root API, explicit state clone or formatting.
//! Setup loops have at most16 sources and3 rows; metadata inspection has16
//! entries. Raw constructor cases have3fields*7invalid and3fields*5valid inputs,
//! then4mixed-field checks. Arithmetic square checks have3fields*3faults and
//!3healthy neighbors. Attempt records emit on assertion unwind and normal return.
//! Separately precharged direct private attempts are2 record admissions,3 estimate
//! completions and1 singleton copy; these are source-owned, outside Closed22.
//! Genuine principal B from a real singleton decision challenges decision A's
//! association guard. Raw publication counts0/17 reject beside a genuine count1.
//! One legal immutable entry copy challenges address custody; no private value is
//! forged. Successful constructor/singleton paths preserve three negative-zero
//! uncertainty bits. Genuine integer-produced phase/rate zero bits remain+0.
//! Negative/overflow/underflow square roots and final inverse-root magnitude
//! failures are not forced: supported nonnegative radicands produce supported
//! roots, and finite positive supported sums bound their reciprocals to normal
//! magnitudes. Reachable earlier square/sum/weighted/mean failures are exercised.
//! No execution, calibrated confidence, compatibility lifecycle or effect claim.
#![forbid(unsafe_code)]

use std::{
    cell::Cell,
    error::Error,
    io::{self, Write},
    num::NonZeroU64,
};

use crate::{
    ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitError,
    FixtureCombinationContextField, FixtureCombinationDenominatorField, FixtureCombinationError,
    FixtureCombinationKernelKind, FixtureCombinationKernelOutcome, FixtureCombinationScalarField,
    FixtureCombinationScalarIssue, FixtureCombinationStep, FixtureCombinedEstimate, FixtureContext,
    FixtureHistory, FixtureMinimum, FixtureObservation, FixtureOutcomeKind,
    FixturePrincipalDecision, FixturePrincipalOutcomeKind, FixturePrincipalParameters,
    FixturePrincipalState, FixturePrincipalStratum, FixtureProjectedBatch,
    FixtureProjectedInterval, FixtureProjectionTerms, FixtureSupporterUncertainty,
    combine_fixture_supporters, enclose_fixture_projection, project_fixture_anchor,
};
use time_domain::{MonotonicNs, PhaseOffsetQ33, UtcCoordinateQ32, exchange_arithmetic};

/// Harness errors stay separate from returned product admission failures.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Principal20 followed by uncertainty-constructor and kernel-call attempts.
type Recipe = [u64; 22];

/// Precharge a separately source-owned private attempt before its operation.
/// # Errors
/// Reject an exhausted fixed local allowance without invoking the receiver.
fn private_attempt<T>(attempts: &Cell<u8>, ceiling: u8, call: impl FnOnce() -> T) -> TestResult<T> {
    let next = attempts
        .get()
        .checked_add(1)
        .ok_or_else(|| io::Error::other("combination private counter overflow"))?;
    if next > ceiling {
        return Err(io::Error::other("combination private counter ceiling").into());
    }
    attempts.set(next);
    Ok(call())
}

/// One bounded local actual-attempt scope with no product-derived allowance.
struct Calls {
    /// Fixed authored case identity.
    case: &'static str,
    /// Complete finite source reservation, never a post-failure retry allowance.
    ceiling: Recipe,
    /// Actual attempts precharged before the corresponding public operation.
    actual: [Cell<u64>; 22],
}

impl Calls {
    /// Begin one authored case without invoking any product operation.
    const fn new(case: &'static str, ceiling: Recipe) -> Self {
        Self {
            case,
            ceiling,
            actual: [const { Cell::new(0) }; 22],
        }
    }

    /// Precharge one real call and preserve its original return value.
    /// # Errors
    /// Reject invalid slots, overflow and exhausted allowances before the call.
    fn call<T>(&self, slot: usize, call: impl FnOnce() -> T) -> TestResult<T> {
        let count = self
            .actual
            .get(slot)
            .ok_or_else(|| io::Error::other("combination counter slot"))?;
        let upper = self
            .ceiling
            .get(slot)
            .ok_or_else(|| io::Error::other("combination counter upper"))?;
        let next = count
            .get()
            .checked_add(1)
            .ok_or_else(|| io::Error::other("combination counter overflow"))?;
        if next > *upper {
            return Err(io::Error::other("combination counter ceiling").into());
        }
        count.set(next);
        Ok(call())
    }

    /// Charge one ordinary immutable product accessor.
    /// # Errors
    /// Preserve accounting failure without invoking the accessor.
    fn get<T>(&self, get: impl FnOnce() -> T) -> TestResult<T> {
        self.call(7, get)
    }

    /// Confirm the independent whole healthy recipe, not a failed prefix.
    /// # Panics
    /// The successful case must perform every predeclared public attempt.
    fn assert_complete(&self) {
        assert_eq!(self.actual.each_ref().map(Cell::get), self.ceiling);
    }
}

impl Drop for Calls {
    /// Preserve attempted prefixes even when the intended behavioral RED unwinds.
    /// Failed or absent emission remains an observation gap.
    fn drop(&mut self) {
        let c = self.actual.each_ref().map(Cell::get);
        let _emission = writeln!(
            io::stderr().lock(),
            "CORE_COMBINATION_PROFILE case={} histories={} appends={} observations={} contexts={} monotonic={} coordinates={} exchanges={} getters={} terms={} projections={} enclosures={} batches={} minima={} parameters={} strata={} seeds={} advances={} state_clones={} value_equalities={} state_formats={} uncertainties={} combinations={}",
            self.case,
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
            c[19],
            c[20],
            c[21]
        );
    }
}

/// Admit a nonzero literal tag without product identity or fallback authority.
/// # Errors
/// Reject zero rather than substituting a fixture identity.
fn tag(raw: u64) -> TestResult<NonZeroU64> {
    NonZeroU64::new(raw).ok_or_else(|| io::Error::other("zero combination fixture tag").into())
}

/// One independent literal three-point model and its common-time expected facts.
#[derive(Clone, Copy)]
struct Source {
    /// Fixed source label assertion.
    label: u64,
    /// Exact Q33 phase numerators at0/1/2 seconds.
    phases: [i128; 3],
    /// Explicit root-distance term at the latest anchor, without physical claims.
    distance: f64,
    /// Independent literal latest phase bits, not a product-derived oracle.
    phase_bits: u64,
    /// Independent literal fitted positive-behind derivative bits.
    rate_bits: u64,
}

/// Genuine original entries and their separately checked principal metadata.
struct Built {
    /// Original immutable projections/enclosures, never reconstructed from results.
    entries: Vec<FixtureProjectedInterval>,
    /// Explicit checked stratum1 mapping for every original source.
    strata: Vec<FixturePrincipalStratum>,
    /// Shared literal frame assertion.
    frame: NonZeroU64,
    /// Shared literal mapping-generation assertion.
    generation: NonZeroU64,
    /// Actual common projection instant2s; every chosen anchor is latest.
    at: MonotonicNs,
}

impl Built {
    /// Borrow these exact original entries into their real checked batch.
    /// # Errors
    /// Preserve actual receiver or accounting failure.
    fn batch(&self, p: &Calls) -> TestResult<FixtureProjectedBatch<'_>> {
        Ok(p.call(11, || {
            FixtureProjectedBatch::new(self.frame, self.generation, self.at, &self.entries)
        })??)
    }
}

/// Produce exact Q33 phase through real checked UTC and exchange owners.
/// # Errors
/// Preserve constructor, arithmetic and pre-call counter failures.
/// # Panics
/// Actual legal construction must retain the authored exact numerator.
fn phase(p: &Calls, numerator: i128) -> TestResult<PhaseOffsetQ33> {
    let t1 = p.call(5, || UtcCoordinateQ32::new(0))??;
    let t2 = p.call(5, || UtcCoordinateQ32::new(numerator))??;
    let t3 = p.call(5, || UtcCoordinateQ32::new(0))??;
    let t4 = p.call(5, || UtcCoordinateQ32::new(0))??;
    let exchange = p.call(6, || exchange_arithmetic(t1, t2, t3, t4))??;
    let phase = p.get(|| exchange.offset())?;
    assert_eq!(p.get(|| phase.numerator())?, numerator);
    Ok(phase)
}

/// Build at most16 real histories and their actual latest-anchor wrappers.
/// # Errors
/// Reject excessive setup scope and preserve actual producer/accounting errors.
/// # Panics
/// The real projected phase/rate must match the independent literal source facts.
fn build(p: &Calls, sources: &[Source]) -> TestResult<Built> {
    build_in_context(p, sources, 7, 11, 2_000_000_000)
}

/// Build genuinely independent entries under an explicit alternate context.
/// # Errors
/// Preserve source-capacity, timestamp, checked-producer and accounting errors.
/// # Panics
/// Real projections must retain the independent supplied center/rate facts.
fn build_in_context(
    p: &Calls,
    sources: &[Source],
    raw_frame: u64,
    raw_generation: u64,
    latest: u64,
) -> TestResult<Built> {
    if sources.len() > 16 {
        return Err(io::Error::other("combination source recipe capacity").into());
    }
    let frame = tag(raw_frame)?;
    let generation = tag(raw_generation)?;
    let at = p.call(4, || MonotonicNs::new(latest))?;
    let first = latest
        .checked_sub(2_000_000_000)
        .ok_or_else(|| io::Error::other("combination fixture timestamp"))?;
    let middle = latest
        .checked_sub(1_000_000_000)
        .ok_or_else(|| io::Error::other("combination fixture timestamp"))?;
    let mut entries = Vec::with_capacity(sources.len());
    let mut strata = Vec::with_capacity(sources.len());
    for source in sources {
        let label = tag(source.label)?;
        let context = p.call(3, || FixtureContext::new(label, frame, generation))?;
        let mut history = p.call(0, || FixtureHistory::new(context))?;
        for (raw_at, numerator) in [first, middle, latest].into_iter().zip(source.phases) {
            let observed = p.call(4, || MonotonicNs::new(raw_at))?;
            let phase = phase(p, numerator)?;
            let row = p.call(2, || FixtureObservation::new(context, observed, phase, 1.0))??;
            p.call(1, || history.try_append(row))??;
        }
        // Latest anchor has age0: supplied growth skew cannot alter distance.
        let terms = p.call(8, || FixtureProjectionTerms::new(source.distance, 0.0, 0.0))??;
        let projection = p.call(9, || {
            project_fixture_anchor(&history, context, 2, at, at, terms)
        })??;
        assert_eq!(
            p.get(|| projection.center_seconds())?.to_bits(),
            source.phase_bits
        );
        assert_eq!(
            p.get(|| projection.phase_rate_seconds_per_second())?
                .to_bits(),
            source.rate_bits
        );
        entries.push(p.call(10, || enclose_fixture_projection(&projection))??);
        strata.push(p.call(14, || FixturePrincipalStratum::new(label, 1))??);
    }
    Ok(Built {
        entries,
        strata,
        frame,
        generation,
        at,
    })
}

/// Seed explicit fixture parameters and an independently fixed minimum.
/// # Errors
/// Preserve actual checked construction or accounting rejection.
fn state(
    p: &Calls,
    built: &Built,
    batch: &FixtureProjectedBatch<'_>,
    roster: &[NonZeroU64],
    minimum: usize,
) -> TestResult<FixturePrincipalState> {
    let minimum = p.call(12, || FixtureMinimum::new(minimum))??;
    let parameters = p.call(13, || FixturePrincipalParameters::new(0.25, 0.0))??;
    Ok(p.call(15, || {
        FixturePrincipalState::seed(roster, parameters, minimum, batch, &built.strata)
    })??)
}

/// Observe the actual input principal transition against independent facts.
/// # Errors
/// Preserve pre-call getter accounting failure.
/// # Panics
/// Kind, label and inert ready-credit provenance must match the literal trace.
fn principal(
    p: &Calls,
    decision: &FixturePrincipalDecision<'_>,
    kind: FixturePrincipalOutcomeKind,
    label: Option<NonZeroU64>,
) -> TestResult {
    assert_eq!(p.get(|| decision.kind())?, kind);
    assert_eq!(p.get(|| decision.principal_label())?, label);
    assert_eq!(
        p.get(|| decision.reference_ready())?,
        kind == FixturePrincipalOutcomeKind::ReferenceReady
    );
    Ok(())
}

/// Inspect every supplied uncertainty scalar and its actual original entry borrow.
/// # Errors
/// Preserve pre-call getter accounting failure.
/// # Panics
/// Constructor output must preserve all supplied bits and exact entry identity.
fn uncertainty(
    p: &Calls,
    value: &FixtureSupporterUncertainty<'_>,
    original: &FixtureProjectedInterval,
    raw: [f64; 3],
) -> TestResult {
    assert!(std::ptr::eq(p.get(|| value.entry())?, original));
    assert_eq!(
        p.get(|| value.phase_standard_deviation_seconds())?
            .to_bits(),
        raw[0].to_bits()
    );
    assert_eq!(
        p.get(|| value.rate_standard_deviation_seconds_per_second())?
            .to_bits(),
        raw[1].to_bits()
    );
    assert_eq!(
        p.get(|| value.tracking_skew_seconds_per_second())?
            .to_bits(),
        raw[2].to_bits()
    );
    Ok(())
}

/// Inspect full combined scalar/context/supporter output against independent bits.
/// # Errors
/// Preserve observer failure or reject an unexpectedly absent combined estimate.
/// # Panics
/// All five scalar results, original principal context and complete labels must match.
fn combined(p: &Calls, outcome: &FixtureCombinationKernelOutcome, built: &Built) -> TestResult {
    observe_combined(
        p,
        outcome,
        built,
        FixturePrincipalOutcomeKind::ReferenceReady,
        101,
        &[101, 202],
        [
            0x3f50_0000_0000_0000,
            0x3ed0_0000_0000_0000,
            0x3ff0_0001_7fff_ee00,
            0x3fdc_9f25_c5bf_edd9,
            0x3fe6_a09e_667f_3bcc,
        ],
    )
}

/// Observe every public output field against independent scalar/context facts.
/// # Errors
/// Preserve accounting failure and reject an absent computed estimate.
/// # Panics
/// The complete estimate must retain all independently declared facts.
fn observe_combined(
    p: &Calls,
    outcome: &FixtureCombinationKernelOutcome,
    built: &Built,
    kind: FixturePrincipalOutcomeKind,
    label: u64,
    labels: &[u64],
    bits: [u64; 5],
) -> TestResult {
    assert_eq!(
        p.get(|| outcome.kind())?,
        FixtureCombinationKernelKind::Combined
    );
    assert_eq!(p.get(|| outcome.principal_kind())?, kind);
    let estimate = p
        .get(|| outcome.combined())?
        .ok_or_else(|| io::Error::other("missing actual combined estimate"))?;
    observe_estimate(p, estimate, built, label, labels, bits)
}

/// Observe a real estimate's scalars, complete provenance and occupied labels.
/// # Errors
/// Preserve the public-getter precharge failure without calling its accessor.
/// # Panics
/// Every field must retain the independently declared scalar and source facts.
fn observe_estimate(
    p: &Calls,
    estimate: &FixtureCombinedEstimate,
    built: &Built,
    label: u64,
    labels: &[u64],
    bits: [u64; 5],
) -> TestResult {
    // Exact dyadic means; roots/reciprocals follow independently derived
    // round-to-nearest-even execution controls, not a production-formula oracle.
    assert_eq!(p.get(|| estimate.phase_seconds())?.to_bits(), bits[0]);
    assert_eq!(
        p.get(|| estimate.phase_rate_seconds_per_second())?
            .to_bits(),
        bits[1]
    );
    assert_eq!(
        p.get(|| estimate.phase_standard_deviation_seconds())?
            .to_bits(),
        bits[2]
    );
    assert_eq!(
        p.get(|| estimate.rate_standard_deviation_seconds_per_second())?
            .to_bits(),
        bits[3]
    );
    assert_eq!(
        p.get(|| estimate.tracking_skew_seconds_per_second())?
            .to_bits(),
        bits[4]
    );
    assert_eq!(p.get(|| estimate.principal_label())?, tag(label)?);
    assert_eq!(p.get(|| estimate.frame())?, built.frame);
    assert_eq!(p.get(|| estimate.mapping_generation())?, built.generation);
    assert_eq!(p.get(|| estimate.projected_at())?, built.at);
    assert_eq!(p.get(|| estimate.anchor_observed_at())?, built.at);
    assert_eq!(p.get(|| estimate.latest_observed_at())?, built.at);
    assert_eq!(p.get(|| estimate.fit_reference())?, built.at);
    assert_eq!(p.get(|| estimate.supporter_count())?, labels.len());
    let actual_labels: Vec<u64> = p
        .get(|| estimate.supporters())?
        .map(NonZeroU64::get)
        .collect();
    assert_eq!(actual_labels, labels);
    Ok(())
}

/// Challenge raw prefix counts using a genuine singleton's checked quantities.
/// # Errors
/// Preserve setup, private receiver and precharged observer failures.
/// # Panics
/// Counts0/17 must reject; the count1 neighbor preserves all scalar/provenance bits.
fn singleton_publication_counts(
    p: &Calls,
    decision: &FixturePrincipalDecision<'_>,
    record: &FixtureSupporterUncertainty<'_>,
    built: &Built,
    label: NonZeroU64,
) -> TestResult {
    let principal = p
        .get(|| decision.principal_entry())?
        .ok_or_else(|| io::Error::other("missing singleton publication principal"))?;
    let copies = Cell::new(0);
    let quantities = private_attempt(&copies, 1, || super::singleton_quantities(record))?;
    let attempts = Cell::new(0);
    let estimate = private_attempt(&attempts, 3, || {
        super::complete_estimate(principal, [label; 16], 1, &quantities)
    })??;
    observe_estimate(
        p,
        &estimate,
        built,
        101,
        &[101],
        [
            0x3fc0_0000_0000_0000,
            0,
            0x8000_0000_0000_0000,
            0x8000_0000_0000_0000,
            0x8000_0000_0000_0000,
        ],
    )?;
    for count in [0, 17] {
        assert_eq!(
            private_attempt(&attempts, 3, || {
                super::complete_estimate(principal, [label; 16], count, &quantities)
            })?
            .err(),
            Some(FixtureCombinationError::Invariant)
        );
    }
    assert_eq!(copies.get(), 1);
    assert_eq!(attempts.get(), 3);
    Ok(())
}

/// Challenge cross-decision principal association without forging checked values.
/// # Errors
/// Preserve genuine producer, private receiver and precharged accessor failures.
/// # Panics
/// Decision A accepts its own principal and rejects genuine witnessed principal B.
fn principal_association_rejections(
    p: &Calls,
    built: &Built,
    decision: &FixturePrincipalDecision<'_>,
    records: &[FixtureSupporterUncertainty<'_>],
) -> TestResult {
    let selection = p.get(|| decision.selection())?;
    let witness = p
        .get(|| selection.witnessed())?
        .ok_or_else(|| io::Error::other("missing association witness"))?;
    let labels: Vec<_> = p.get(|| witness.supporters())?.collect();
    assert_eq!(labels, [tag(101)?, tag(202)?]);
    let batch = p.call(11, || {
        FixtureProjectedBatch::new(
            built.frame,
            built.generation,
            built.at,
            &built.entries[1..2],
        )
    })??;
    let minimum = p.call(12, || FixtureMinimum::new(1))??;
    let parameters = p.call(13, || FixturePrincipalParameters::new(0.25, 0.0))??;
    let roster = [tag(202)?];
    let mut state = p.call(15, || {
        FixturePrincipalState::seed(&roster, parameters, minimum, &batch, &built.strata[1..2])
    })??;
    let other = p.call(16, || state.advance(&batch, &built.strata[1..2]))??;
    principal(
        p,
        &other,
        FixturePrincipalOutcomeKind::ReferenceReady,
        Some(tag(202)?),
    )?;
    let actual = p
        .get(|| decision.principal_entry())?
        .ok_or_else(|| io::Error::other("missing association principal A"))?;
    let other = p
        .get(|| other.principal_entry())?
        .ok_or_else(|| io::Error::other("missing association principal B"))?;
    assert!(std::ptr::eq(actual, &raw const built.entries[0]));
    assert!(std::ptr::eq(other, &raw const built.entries[1]));
    let attempts = Cell::new(0);
    private_attempt(&attempts, 2, || {
        super::admit_records(decision, records, actual)
    })??;
    assert_eq!(
        private_attempt(&attempts, 2, || {
            super::admit_records(decision, records, other)
        })?,
        Err(FixtureCombinationError::Invariant)
    );
    assert_eq!(attempts.get(), 2);
    Ok(())
}

/// Describe a literal exact zero line without any product operation.
const fn zero_source(label: u64, distance: f64) -> Source {
    Source {
        label,
        phases: [0; 3],
        distance,
        phase_bits: 0,
        rate_bits: 0,
    }
}

/// Exact constant-line inputs exposing ascending-label binary64 reduction order.
const fn cancellation_sources() -> [Source; 3] {
    [
        Source {
            label: 101,
            phases: [8_589_934_592; 3],
            distance: 1.0,
            phase_bits: 0x3ff0_0000_0000_0000,
            rate_bits: 0,
        },
        Source {
            label: 202,
            phases: [1; 3],
            distance: 1_048_576.0,
            phase_bits: 0x3de0_0000_0000_0000,
            rate_bits: 0,
        },
        Source {
            label: 303,
            phases: [-8_589_934_592; 3],
            distance: 1.0,
            phase_bits: 0xbff0_0000_0000_0000,
            rate_bits: 0,
        },
    ]
}

/// Preserve a real returned receiver error rather than formatting an outcome.
/// # Errors
/// A successful operation is a failed negative control.
fn rejection(
    result: Result<FixtureCombinationKernelOutcome, FixtureCombinationError>,
) -> TestResult<FixtureCombinationError> {
    result
        .err()
        .ok_or_else(|| io::Error::other("combination negative accepted").into())
}

/// Challenge the original-address, count, membership and context guards.
/// # Errors
/// Preserve an unexpected acceptance or bounded accounting failure.
/// # Panics
/// Each malformed association must retain its independently declared first error.
fn metadata_rejections(
    p: &Calls,
    decision: &FixturePrincipalDecision<'_>,
    records: [FixtureSupporterUncertainty<'_>; 7],
) -> TestResult {
    let [a, b, c, copied, frame, generation, time] = records;
    for supplied in [&[][..], &[a][..], &[a, b, c][..]] {
        assert_eq!(
            rejection(p.call(21, || combine_fixture_supporters(decision, supplied))?)?,
            FixtureCombinationError::MetadataCountMismatch
        );
    }
    for (supplied, expected) in [
        (
            [a, a],
            FixtureCombinationError::DuplicateUncertainty { index: 1 },
        ),
        (
            [a, c],
            FixtureCombinationError::UnexpectedUncertainty { index: 1 },
        ),
        (
            [copied, b],
            FixtureCombinationError::EntryMismatch { index: 0 },
        ),
        (
            [frame, b],
            FixtureCombinationError::ContextMismatch {
                index: 0,
                field: FixtureCombinationContextField::Frame,
            },
        ),
        (
            [generation, b],
            FixtureCombinationError::ContextMismatch {
                index: 0,
                field: FixtureCombinationContextField::MappingGeneration,
            },
        ),
        (
            [time, b],
            FixtureCombinationError::ContextMismatch {
                index: 0,
                field: FixtureCombinationContextField::ProjectedAt,
            },
        ),
    ] {
        assert_eq!(
            rejection(p.call(21, || combine_fixture_supporters(decision, &supplied))?)?,
            expected
        );
    }
    Ok(())
}

/// Compare the real checked arithmetic error with the independently named step.
/// # Errors
/// A successful operation fails this rejection control.
/// # Panics
/// Exact owner stage, primitive and first issue must match.
fn arithmetic_rejection(
    result: Result<FixtureCombinationKernelOutcome, FixtureCombinationError>,
    step: FixtureCombinationStep,
    operation: ArithmeticOperation,
    issue: ArithmeticIssue,
) -> TestResult<FixtureCombinationError> {
    let error = rejection(result)?;
    assert_eq!(
        error,
        FixtureCombinationError::Arithmetic {
            step,
            source: FitError::Arithmetic {
                stage: ArithmeticStage::Combination,
                operation,
                issue
            },
        }
    );
    Ok(error)
}

/// Observe an actual product Debug dispatch against its fixed redaction contract.
/// # Errors
/// Preserve the precharged formatting-attempt bound.
/// # Panics
/// The complete output must contain only the independently named opaque type.
fn opaque_debug(p: &Calls, value: &impl std::fmt::Debug, expected: &str) -> TestResult {
    assert_eq!(p.call(19, || format!("{value:?}"))?, expected);
    Ok(())
}

/// Require the arithmetic error trait source to borrow its original typed cause.
/// # Errors
/// Reject a missing/wrong cause or a precharged getter-accounting failure.
/// # Panics
/// Downcasting must retain the address of the actual original `FitError` field.
fn borrowed_arithmetic_source(p: &Calls, error: &FixtureCombinationError) -> TestResult {
    let FixtureCombinationError::Arithmetic {
        source: expected, ..
    } = error
    else {
        return Err(io::Error::other("missing combination arithmetic cause").into());
    };
    let source = p
        .get(|| Error::source(error))?
        .ok_or_else(|| io::Error::other("missing borrowed combination cause"))?;
    let actual = source
        .downcast_ref::<FitError>()
        .ok_or_else(|| io::Error::other("wrong borrowed combination cause type"))?;
    assert!(std::ptr::eq(actual, expected));
    Ok(())
}

/// A neighboring arithmetic input must produce a real combined zero line.
/// # Errors
/// Preserve actual absence and accounting failures.
/// # Panics
/// Success must include a computed estimate with the independently exact zero means.
fn accepted_zero_means(p: &Calls, outcome: &FixtureCombinationKernelOutcome) -> TestResult {
    assert_eq!(
        p.get(|| outcome.kind())?,
        FixtureCombinationKernelKind::Combined
    );
    let estimate = p
        .get(|| outcome.combined())?
        .ok_or_else(|| io::Error::other("missing combination neighbor"))?;
    assert_eq!(p.get(|| estimate.phase_seconds())?.to_bits(), 0);
    assert_eq!(
        p.get(|| estimate.phase_rate_seconds_per_second())?
            .to_bits(),
        0
    );
    Ok(())
}

/// Unequal offset/rate weighting retains disagreement about ORIGINAL principal A.
/// # Errors
/// Preserve actual setup/admission errors; the reached computed path must succeed.
/// # Panics
/// The five independent exact rounded results and their provenance must match.
#[test]
fn combination_distinct_weights_and_principal_disagreement() -> TestResult {
    let p = Calls::new(
        "combination_distinct_weights_and_principal_disagreement",
        [
            2, 6, 6, 2, 7, 24, 6, 45, 2, 2, 2, 1, 1, 1, 2, 1, 1, 0, 0, 2, 2, 1,
        ],
    );
    // Supplied entry/metadata order B,A differs from ascending witness order A,B.
    // B's phases are (3062,3067,3072)*2^-20, exactly representable in Q33.
    let built = build(
        &p,
        &[
            Source {
                label: 202,
                phases: [25_083_904, 25_124_864, 25_165_824],
                distance: 2.0,
                phase_bits: 0x3f68_0000_0000_0000,
                rate_bits: 0x3ed4_0000_0000_0000,
            },
            Source {
                label: 101,
                phases: [0; 3],
                distance: 1.0,
                phase_bits: 0,
                rate_bits: 0,
            },
        ],
    )?;
    let batch = built.batch(&p)?;
    let roster = [tag(101)?, tag(202)?];
    let mut state = state(&p, &built, &batch, &roster, 2)?;
    let decision = p.call(16, || state.advance(&batch, &built.strata))??;
    principal(
        &p,
        &decision,
        FixturePrincipalOutcomeKind::ReferenceReady,
        Some(roster[0]),
    )?;
    let records = [
        p.call(20, || {
            FixtureSupporterUncertainty::new(&built.entries[0], 1.0, 0.5, 1.0)
        })??,
        p.call(20, || {
            FixtureSupporterUncertainty::new(&built.entries[1], 1.0, 1.0, 1.0)
        })??,
    ];
    for (index, raw) in [[1.0, 0.5, 1.0], [1.0, 1.0, 1.0]].into_iter().enumerate() {
        uncertainty(&p, &records[index], &built.entries[index], raw)?;
    }
    let result = p.call(21, || combine_fixture_supporters(&decision, &records))?;
    assert!(
        result.is_ok(),
        "genuine two-source combination computed path must succeed"
    );
    let outcome = result?;
    opaque_debug(&p, &outcome, "FixtureCombinationKernelOutcome { .. }")?;
    let estimate = p
        .get(|| outcome.combined())?
        .ok_or_else(|| io::Error::other("missing diagnostic combination estimate"))?;
    opaque_debug(&p, estimate, "FixtureCombinedEstimate { .. }")?;
    combined(&p, &outcome, &built)?;
    p.assert_complete();
    Ok(())
}

/// Checked uncertainty accepts signed zero; lost minimum yields no kernel estimate.
/// # Errors
/// Preserve actual constructor/public-consumer or bounded accounting failure.
/// # Panics
/// The healthy constructor and actual unavailable outcome must retain their facts.
#[test]
fn combination_unavailable_and_checked_uncertainty() -> TestResult {
    let p = Calls::new(
        "combination_unavailable_and_checked_uncertainty",
        [
            1, 3, 3, 1, 4, 12, 3, 22, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 3, 1, 2,
        ],
    );
    let built = build(
        &p,
        &[Source {
            label: 101,
            phases: [1_073_741_824; 3],
            distance: 1.0 / 64.0,
            phase_bits: 0x3fc0_0000_0000_0000,
            rate_bits: 0,
        }],
    )?;
    let batch = built.batch(&p)?;
    let record = p.call(20, || {
        FixtureSupporterUncertainty::new(&built.entries[0], -0.0, 0.5, 1.0)
    })??;
    uncertainty(&p, &record, &built.entries[0], [-0.0, 0.5, 1.0])?;
    opaque_debug(&p, &record, "FixtureSupporterUncertainty { .. }")?;
    let roster = [tag(101)?, tag(202)?];
    let mut state = state(&p, &built, &batch, &roster, 2)?;
    let decision = p.call(16, || state.advance(&batch, &built.strata))??;
    principal(
        &p,
        &decision,
        FixturePrincipalOutcomeKind::Unavailable,
        None,
    )?;
    let selection = p.get(|| decision.selection())?;
    assert_eq!(
        p.get(|| selection.outcome())?,
        FixtureOutcomeKind::InsufficientPopulation
    );
    assert_eq!(p.get(|| selection.minimum())?, 2);
    let error = rejection(p.call(21, || combine_fixture_supporters(&decision, &[record]))?)?;
    assert_eq!(error, FixtureCombinationError::MetadataCountMismatch);
    opaque_debug(&p, &error, "FixtureCombinationError { .. }")?;
    assert_eq!(
        p.call(19, || error.to_string())?,
        "combination metadata count mismatch"
    );
    assert!(p.get(|| Error::source(&error))?.is_none());
    let outcome = p.call(21, || combine_fixture_supporters(&decision, &[]))??;
    assert_eq!(
        p.get(|| outcome.kind())?,
        FixtureCombinationKernelKind::Unavailable
    );
    assert_eq!(
        p.get(|| outcome.principal_kind())?,
        FixturePrincipalOutcomeKind::Unavailable
    );
    assert!(p.get(|| outcome.combined())?.is_none());
    p.assert_complete();
    Ok(())
}

/// Singleton copy preserves every bit and retained kind creates no new credit.
/// # Errors
/// Preserve genuine setup, receiver and bounded accounting errors.
/// # Panics
/// Zero uncertainty denominators are legal only in this true singleton branch.
#[test]
fn combination_singleton_signed_zero_and_retained() -> TestResult {
    let p = Calls::new(
        "combination_singleton_signed_zero_and_retained",
        [
            1, 3, 3, 1, 4, 12, 3, 67, 1, 1, 1, 1, 1, 1, 1, 1, 2, 0, 0, 0, 1, 2,
        ],
    );
    let built = build(
        &p,
        &[Source {
            label: 101,
            phases: [1_073_741_824; 3],
            distance: 0.0,
            phase_bits: 0x3fc0_0000_0000_0000,
            rate_bits: 0,
        }],
    )?;
    let batch = built.batch(&p)?;
    let roster = [tag(101)?];
    let mut state = state(&p, &built, &batch, &roster, 1)?;
    let record = p.call(20, || {
        FixtureSupporterUncertainty::new(&built.entries[0], -0.0, -0.0, -0.0)
    })??;
    uncertainty(&p, &record, &built.entries[0], [-0.0; 3])?;
    for kind in [
        FixturePrincipalOutcomeKind::ReferenceReady,
        FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate,
    ] {
        let decision = p.call(16, || state.advance(&batch, &built.strata))??;
        principal(&p, &decision, kind, Some(roster[0]))?;
        if kind == FixturePrincipalOutcomeKind::ReferenceReady {
            singleton_publication_counts(&p, &decision, &record, &built, roster[0])?;
        }
        let outcome = p.call(21, || combine_fixture_supporters(&decision, &[record]))??;
        observe_combined(
            &p,
            &outcome,
            &built,
            kind,
            101,
            &[101],
            [
                0x3fc0_0000_0000_0000,
                0,
                0x8000_0000_0000_0000,
                0x8000_0000_0000_0000,
                0x8000_0000_0000_0000,
            ],
        )?;
    }
    p.assert_complete();
    Ok(())
}

/// Every raw scalar field uses its actual mandatory constructor and consumer.
/// # Errors
/// Preserve fixture/accounting errors and reject an absent expected scalar error.
/// # Panics
/// Classifier order, signed zeros and adjacent legal normal values must hold.
#[test]
fn combination_raw_uncertainty_boundaries() -> TestResult {
    let p = Calls::new(
        "combination_raw_uncertainty_boundaries",
        [
            1, 3, 3, 1, 4, 12, 3, 326, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 40, 15,
        ],
    );
    let built = build(&p, &[zero_source(101, 1.0)])?;
    let batch = built.batch(&p)?;
    let roster = [tag(101)?];
    let mut state = state(&p, &built, &batch, &roster, 1)?;
    let decision = p.call(16, || state.advance(&batch, &built.strata))??;
    principal(
        &p,
        &decision,
        FixturePrincipalOutcomeKind::ReferenceReady,
        Some(roster[0]),
    )?;
    let fields = [
        FixtureCombinationScalarField::PhaseStandardDeviation,
        FixtureCombinationScalarField::RateStandardDeviation,
        FixtureCombinationScalarField::TrackingSkew,
    ];
    let invalid = [
        (f64::NAN, FixtureCombinationScalarIssue::NonFinite),
        (f64::INFINITY, FixtureCombinationScalarIssue::NonFinite),
        (f64::NEG_INFINITY, FixtureCombinationScalarIssue::NonFinite),
        (f64::from_bits(1), FixtureCombinationScalarIssue::Subnormal),
        (-f64::from_bits(1), FixtureCombinationScalarIssue::Subnormal),
        (-1.0, FixtureCombinationScalarIssue::Negative),
        (-f64::MIN_POSITIVE, FixtureCombinationScalarIssue::Negative),
    ];
    for (index, field) in fields.into_iter().enumerate() {
        for (value, issue) in invalid {
            let mut raw = [1.0; 3];
            raw[index] = value;
            let result = p.call(20, || {
                FixtureSupporterUncertainty::new(&built.entries[0], raw[0], raw[1], raw[2])
            })?;
            let error = result
                .err()
                .ok_or_else(|| io::Error::other("raw uncertainty accepted"))?;
            assert_eq!(
                error,
                FixtureCombinationError::InvalidUncertainty { field, issue }
            );
        }
        for value in [0.0, -0.0, f64::MIN_POSITIVE, 1.0, f64::MAX] {
            let mut raw = [1.0; 3];
            raw[index] = value;
            let record = p.call(20, || {
                FixtureSupporterUncertainty::new(&built.entries[0], raw[0], raw[1], raw[2])
            })??;
            uncertainty(&p, &record, &built.entries[0], raw)?;
            let outcome = p.call(21, || combine_fixture_supporters(&decision, &[record]))??;
            observe_combined(
                &p,
                &outcome,
                &built,
                FixturePrincipalOutcomeKind::ReferenceReady,
                101,
                &[101],
                [0, 0, raw[0].to_bits(), raw[1].to_bits(), raw[2].to_bits()],
            )?;
        }
    }
    for (raw, field, issue) in [
        (
            [-1.0, f64::NAN, 1.0],
            FixtureCombinationScalarField::PhaseStandardDeviation,
            FixtureCombinationScalarIssue::Negative,
        ),
        (
            [f64::from_bits(1), f64::NAN, 1.0],
            FixtureCombinationScalarField::PhaseStandardDeviation,
            FixtureCombinationScalarIssue::Subnormal,
        ),
        (
            [1.0, -f64::MIN_POSITIVE, f64::NAN],
            FixtureCombinationScalarField::RateStandardDeviation,
            FixtureCombinationScalarIssue::Negative,
        ),
        (
            [1.0, 1.0, f64::NAN],
            FixtureCombinationScalarField::TrackingSkew,
            FixtureCombinationScalarIssue::NonFinite,
        ),
    ] {
        let result = p.call(20, || {
            FixtureSupporterUncertainty::new(&built.entries[0], raw[0], raw[1], raw[2])
        })?;
        assert_eq!(
            result
                .err()
                .ok_or_else(|| io::Error::other("mixed raw accepted"))?,
            FixtureCombinationError::InvalidUncertainty { field, issue }
        );
    }
    p.assert_complete();
    Ok(())
}

/// Three genuine opposing lines give exact cancellation and distinct weights.
/// # Errors
/// Preserve real source/receiver and accounting failures.
/// # Panics
/// Metadata permutations must reproduce all fields against the same independent oracle.
#[test]
fn combination_three_source_permutation_and_zero_phase_sd() -> TestResult {
    let p = Calls::new(
        "combination_three_source_permutation_and_zero_phase_sd",
        [
            3, 9, 9, 3, 10, 36, 9, 73, 3, 3, 3, 1, 1, 1, 3, 1, 1, 0, 0, 0, 3, 2,
        ],
    );
    // A contributes phase variance1/64; B and C contribute disagreement1/64
    // each. Equal phase weights give (3/64)/3=1/64, hence SD1/8 exactly.
    // Rate/skew inverse variances are1,4,4, so sqrt9=3 then reciprocal1/3.
    let built = build(
        &p,
        &[
            Source {
                label: 303,
                phases: [-805_306_368, -939_524_096, -1_073_741_824],
                distance: 1.0,
                phase_bits: 0xbfc0_0000_0000_0000,
                rate_bits: 0xbf90_0000_0000_0000,
            },
            zero_source(101, 1.0),
            Source {
                label: 202,
                phases: [805_306_368, 939_524_096, 1_073_741_824],
                distance: 1.0,
                phase_bits: 0x3fc0_0000_0000_0000,
                rate_bits: 0x3f90_0000_0000_0000,
            },
        ],
    )?;
    let batch = built.batch(&p)?;
    let roster = [tag(101)?, tag(202)?, tag(303)?];
    let mut state = state(&p, &built, &batch, &roster, 3)?;
    let decision = p.call(16, || state.advance(&batch, &built.strata))??;
    principal(
        &p,
        &decision,
        FixturePrincipalOutcomeKind::ReferenceReady,
        Some(roster[0]),
    )?;
    let raw = [[0.0, 0.5, 0.5], [0.125, 1.0, 1.0], [0.0, 0.5, 0.5]];
    let records = [
        p.call(20, || {
            FixtureSupporterUncertainty::new(&built.entries[0], 0.0, 0.5, 0.5)
        })??,
        p.call(20, || {
            FixtureSupporterUncertainty::new(&built.entries[1], 0.125, 1.0, 1.0)
        })??,
        p.call(20, || {
            FixtureSupporterUncertainty::new(&built.entries[2], 0.0, 0.5, 0.5)
        })??,
    ];
    for index in 0..3 {
        uncertainty(&p, &records[index], &built.entries[index], raw[index])?;
    }
    for supplied in [records, [records[2], records[0], records[1]]] {
        let outcome = p.call(21, || combine_fixture_supporters(&decision, &supplied))??;
        observe_combined(
            &p,
            &outcome,
            &built,
            FixturePrincipalOutcomeKind::ReferenceReady,
            101,
            &[101, 202, 303],
            [
                0,
                0,
                0x3fc0_0000_0000_0000,
                0x3fd5_5555_5555_5555,
                0x3fd5_5555_5555_5555,
            ],
        )?;
    }
    p.assert_complete();
    Ok(())
}

/// Metadata must match original addresses, current context and actual support.
/// # Errors
/// Preserve genuine checked setup, unexpected acceptance and accounting failures.
/// # Panics
/// Missing/extra/duplicate/foreign/nonmember metadata must fail at its declared guard.
#[test]
fn combination_metadata_association_and_context() -> TestResult {
    let p = Calls::new(
        "combination_metadata_association_and_context",
        [
            6, 18, 18, 6, 22, 72, 18, 76, 6, 6, 6, 2, 2, 2, 6, 2, 2, 0, 0, 0, 7, 10,
        ],
    );
    let built = build(
        &p,
        &[
            zero_source(101, 1.0 / 64.0),
            zero_source(202, 1.0 / 64.0),
            Source {
                label: 303,
                phases: [4_294_967_296; 3],
                distance: 1.0 / 64.0,
                phase_bits: 0x3fe0_0000_0000_0000,
                rate_bits: 0,
            },
        ],
    )?;
    let wrong_frame = build_in_context(&p, &[zero_source(101, 1.0 / 64.0)], 8, 11, 2_000_000_000)?;
    let wrong_generation =
        build_in_context(&p, &[zero_source(101, 1.0 / 64.0)], 7, 12, 2_000_000_000)?;
    let wrong_time = build_in_context(&p, &[zero_source(101, 1.0 / 64.0)], 7, 11, 3_000_000_000)?;
    // This is a legal immutable public value copy, not a private representation
    // forgery; identical context/contents must not substitute its new address.
    let copied_entry = built.entries[0];
    let batch = built.batch(&p)?;
    let roster = [tag(101)?, tag(202)?, tag(303)?];
    let mut state = state(&p, &built, &batch, &roster, 2)?;
    let decision = p.call(16, || state.advance(&batch, &built.strata))??;
    principal(
        &p,
        &decision,
        FixturePrincipalOutcomeKind::ReferenceReady,
        Some(roster[0]),
    )?;
    let a = p.call(20, || {
        FixtureSupporterUncertainty::new(&built.entries[0], 0.0, 1.0, 1.0)
    })??;
    let b = p.call(20, || {
        FixtureSupporterUncertainty::new(&built.entries[1], 0.0, 1.0, 1.0)
    })??;
    let c = p.call(20, || {
        FixtureSupporterUncertainty::new(&built.entries[2], 0.0, 1.0, 1.0)
    })??;
    let copied = p.call(20, || {
        FixtureSupporterUncertainty::new(&copied_entry, 0.0, 1.0, 1.0)
    })??;
    let frame = p.call(20, || {
        FixtureSupporterUncertainty::new(&wrong_frame.entries[0], 0.0, 1.0, 1.0)
    })??;
    let generation = p.call(20, || {
        FixtureSupporterUncertainty::new(&wrong_generation.entries[0], 0.0, 1.0, 1.0)
    })??;
    let time = p.call(20, || {
        FixtureSupporterUncertainty::new(&wrong_time.entries[0], 0.0, 1.0, 1.0)
    })??;
    principal_association_rejections(&p, &built, &decision, &[a, b])?;
    metadata_rejections(&p, &decision, [a, b, c, copied, frame, generation, time])?;
    let outcome = p.call(21, || combine_fixture_supporters(&decision, &[a, b]))??;
    observe_combined(
        &p,
        &outcome,
        &built,
        FixturePrincipalOutcomeKind::ReferenceReady,
        101,
        &[101, 202],
        [0, 0, 0, 0x3fe6_a09e_667f_3bcc, 0x3fe6_a09e_667f_3bcc],
    )?;
    p.assert_complete();
    Ok(())
}

/// Multisource inverse denominators reject both zero signs without a fallback.
/// # Errors
/// Preserve real source, receiver and accounting errors.
/// # Panics
/// Zero phase SD remains a successful neighbor, unlike zero reciprocal denominators.
#[test]
fn combination_zero_denominators_and_phase_neighbor() -> TestResult {
    let p = Calls::new(
        "combination_zero_denominators_and_phase_neighbor",
        [
            4, 12, 12, 4, 14, 48, 12, 55, 4, 4, 4, 2, 2, 2, 4, 2, 2, 0, 0, 0, 8, 6,
        ],
    );
    let zero_distance = build(&p, &[zero_source(101, 0.0), zero_source(202, 1.0)])?;
    let normal = build(&p, &[zero_source(101, 1.0), zero_source(202, 1.0)])?;
    let roster = [tag(101)?, tag(202)?];
    let zero_batch = zero_distance.batch(&p)?;
    let mut zero_state = state(&p, &zero_distance, &zero_batch, &roster, 2)?;
    let zero_decision = p.call(16, || {
        zero_state.advance(&zero_batch, &zero_distance.strata)
    })??;
    principal(
        &p,
        &zero_decision,
        FixturePrincipalOutcomeKind::ReferenceReady,
        Some(roster[0]),
    )?;
    let zero_records = [
        p.call(20, || {
            FixtureSupporterUncertainty::new(&zero_distance.entries[0], 0.0, 1.0, 1.0)
        })??,
        p.call(20, || {
            FixtureSupporterUncertainty::new(&zero_distance.entries[1], 0.0, 1.0, 1.0)
        })??,
    ];
    assert_eq!(
        rejection(p.call(21, || combine_fixture_supporters(
            &zero_decision,
            &zero_records
        ))?)?,
        FixtureCombinationError::ZeroDenominator {
            index: 0,
            field: FixtureCombinationDenominatorField::RootDistance
        }
    );
    let batch = normal.batch(&p)?;
    let mut state = state(&p, &normal, &batch, &roster, 2)?;
    let decision = p.call(16, || state.advance(&batch, &normal.strata))??;
    principal(
        &p,
        &decision,
        FixturePrincipalOutcomeKind::ReferenceReady,
        Some(roster[0]),
    )?;
    let a = p.call(20, || {
        FixtureSupporterUncertainty::new(&normal.entries[0], -0.0, 1.0, 1.0)
    })??;
    let b = p.call(20, || {
        FixtureSupporterUncertainty::new(&normal.entries[1], 0.0, 1.0, 1.0)
    })??;
    for zero in [0.0, -0.0] {
        for (raw, field) in [
            (
                [0.0, zero, 1.0],
                FixtureCombinationDenominatorField::RateStandardDeviation,
            ),
            (
                [0.0, 1.0, zero],
                FixtureCombinationDenominatorField::TrackingSkew,
            ),
        ] {
            let invalid = p.call(20, || {
                FixtureSupporterUncertainty::new(&normal.entries[1], raw[0], raw[1], raw[2])
            })??;
            assert_eq!(
                rejection(p.call(21, || combine_fixture_supporters(&decision, &[invalid, a]))?)?,
                FixtureCombinationError::ZeroDenominator { index: 1, field }
            );
        }
    }
    let outcome = p.call(21, || combine_fixture_supporters(&decision, &[b, a]))??;
    observe_combined(
        &p,
        &outcome,
        &normal,
        FixturePrincipalOutcomeKind::ReferenceReady,
        101,
        &[101, 202],
        [0, 0, 0, 0x3fe6_a09e_667f_3bcc, 0x3fe6_a09e_667f_3bcc],
    )?;
    p.assert_complete();
    Ok(())
}

/// Finite admitted uncertainties can fail their actual checked square operation.
/// # Errors
/// Preserve checked setup, neighboring consumer and accounting failures.
/// # Panics
/// Overflow, subnormal and nonzero-to-zero results retain the actual square step.
#[test]
fn combination_finite_square_failures_and_neighbors() -> TestResult {
    let p = Calls::new(
        "combination_finite_square_failures_and_neighbors",
        [
            2, 6, 6, 2, 7, 24, 6, 31, 2, 2, 2, 1, 1, 1, 2, 1, 1, 0, 0, 0, 13, 12,
        ],
    );
    let built = build(&p, &[zero_source(101, 1.0), zero_source(202, 1.0)])?;
    let batch = built.batch(&p)?;
    let roster = [tag(101)?, tag(202)?];
    let mut state = state(&p, &built, &batch, &roster, 2)?;
    let decision = p.call(16, || state.advance(&batch, &built.strata))??;
    principal(
        &p,
        &decision,
        FixturePrincipalOutcomeKind::ReferenceReady,
        Some(roster[0]),
    )?;
    let b = p.call(20, || {
        FixtureSupporterUncertainty::new(&built.entries[1], 0.0, 1.0, 1.0)
    })??;
    // Exact powers: (2^512)^2 overflows; (2^-520)^2=2^-1040 is
    // subnormal; (2^-600)^2 rounds to zero although both factors are nonzero.
    for (index, step) in [
        (0, FixtureCombinationStep::PhaseVariance),
        (1, FixtureCombinationStep::RateVariance),
        (2, FixtureCombinationStep::TrackingSkewVariance),
    ] {
        for (value, issue) in [
            (
                f64::from_bits(0x5ff0_0000_0000_0000),
                ArithmeticIssue::NonFinite,
            ),
            (
                f64::from_bits(0x1f70_0000_0000_0000),
                ArithmeticIssue::Subnormal,
            ),
            (
                f64::from_bits(0x1a70_0000_0000_0000),
                ArithmeticIssue::ZeroUnderflow,
            ),
        ] {
            let mut raw = [0.0, 1.0, 1.0];
            raw[index] = value;
            let a = p.call(20, || {
                FixtureSupporterUncertainty::new(&built.entries[0], raw[0], raw[1], raw[2])
            })??;
            arithmetic_rejection(
                p.call(21, || combine_fixture_supporters(&decision, &[a, b]))?,
                step,
                ArithmeticOperation::Multiply,
                issue,
            )?;
        }
    }
    // The phase neighbor keeps mean variance normal: 2^-1020/2=2^-1021.
    // Rate/skew neighbors square exactly to MIN_POSITIVE, whose inverse2^1022
    // remains finite with this second unit-weight participant.
    for raw in [
        [f64::from_bits(0x2010_0000_0000_0000), 1.0, 1.0],
        [0.0, f64::from_bits(0x2000_0000_0000_0000), 1.0],
        [0.0, 1.0, f64::from_bits(0x2000_0000_0000_0000)],
    ] {
        let a = p.call(20, || {
            FixtureSupporterUncertainty::new(&built.entries[0], raw[0], raw[1], raw[2])
        })??;
        let outcome = p.call(21, || combine_fixture_supporters(&decision, &[b, a]))??;
        accepted_zero_means(&p, &outcome)?;
    }
    p.assert_complete();
    Ok(())
}

/// Sixteen actual supporters succeed; seventeen records reject before duplicates.
/// # Errors
/// Preserve real setup, receiver, expected rejection and accounting failures.
/// # Panics
/// Capacity boundaries and finite-input weight-sum overflow remain distinct.
#[test]
fn combination_capacity_and_weight_sum_overflow() -> TestResult {
    let p = Calls::new(
        "combination_capacity_and_weight_sum_overflow",
        [
            16, 48, 48, 16, 49, 192, 48, 148, 16, 16, 16, 1, 1, 1, 16, 1, 1, 0, 0, 0, 48, 4,
        ],
    );
    let sources: Vec<Source> = (101..117)
        .rev()
        .map(|label| zero_source(label, 1.0))
        .collect();
    let built = build(&p, &sources)?;
    let batch = built.batch(&p)?;
    let roster: Vec<NonZeroU64> = (101..117).map(tag).collect::<TestResult<_>>()?;
    let mut state = state(&p, &built, &batch, &roster, 16)?;
    let decision = p.call(16, || state.advance(&batch, &built.strata))??;
    principal(
        &p,
        &decision,
        FixturePrincipalOutcomeKind::ReferenceReady,
        Some(roster[0]),
    )?;
    let mut records = Vec::with_capacity(16);
    for entry in &built.entries {
        records.push(p.call(20, || {
            FixtureSupporterUncertainty::new(entry, 0.0, 1.0, 1.0)
        })??);
    }
    assert_eq!(
        rejection(p.call(21, || combine_fixture_supporters(
            &decision,
            &[records[0]; 17]
        ))?)?,
        FixtureCombinationError::CapacityExceeded
    );
    let outcome = p.call(21, || combine_fixture_supporters(&decision, &records))??;
    let labels: Vec<u64> = (101..117).collect();
    // All sixteen unit inverse variances sum16; sqrt16=4 exactly.
    observe_combined(
        &p,
        &outcome,
        &built,
        FixturePrincipalOutcomeKind::ReferenceReady,
        101,
        &labels,
        [0, 0, 0, 0x3fd0_0000_0000_0000, 0x3fd0_0000_0000_0000],
    )?;
    for step in [
        FixtureCombinationStep::RateWeightSum,
        FixtureCombinationStep::TrackingSkewWeightSum,
    ] {
        let mut excessive = Vec::with_capacity(16);
        for (index, entry) in built.entries.iter().enumerate() {
            // Entries are descending: indices12..16 are labels104..101.
            // Canonical labels101..104 each contribute2^1022. Their fourth
            // addition overflows, with no unsupported individual factor.
            let tiny = if index >= 12 {
                f64::from_bits(0x2000_0000_0000_0000)
            } else {
                1.0
            };
            let (rate, skew) = if step == FixtureCombinationStep::RateWeightSum {
                (tiny, 1.0)
            } else {
                (1.0, tiny)
            };
            excessive.push(p.call(20, || {
                FixtureSupporterUncertainty::new(entry, 0.0, rate, skew)
            })??);
        }
        arithmetic_rejection(
            p.call(21, || combine_fixture_supporters(&decision, &excessive))?,
            step,
            ArithmeticOperation::Add,
            ArithmeticIssue::NonFinite,
        )?;
    }
    p.assert_complete();
    Ok(())
}

/// A supported weighted variance can underflow only at its final mean division.
/// # Errors
/// Preserve checked setup, accepted-neighbor and accounting failures.
/// # Panics
/// The rejected division must not silently become a zero uncertainty estimate.
#[test]
fn combination_variance_mean_underflow_and_neighbor() -> TestResult {
    let p = Calls::new(
        "combination_variance_mean_underflow_and_neighbor",
        [
            2, 6, 6, 2, 7, 24, 6, 36, 2, 2, 2, 1, 1, 1, 2, 1, 1, 0, 0, 0, 3, 2,
        ],
    );
    let built = build(
        &p,
        &[zero_source(101, f64::MIN_POSITIVE), zero_source(202, 1.0)],
    )?;
    let batch = built.batch(&p)?;
    let roster = [tag(101)?, tag(202)?];
    let mut state = state(&p, &built, &batch, &roster, 2)?;
    let decision = p.call(16, || state.advance(&batch, &built.strata))??;
    principal(
        &p,
        &decision,
        FixturePrincipalOutcomeKind::ReferenceReady,
        Some(roster[0]),
    )?;
    let a = p.call(20, || {
        FixtureSupporterUncertainty::new(&built.entries[0], 0.0, 1.0, 1.0)
    })??;
    let b = p.call(20, || {
        FixtureSupporterUncertainty::new(
            &built.entries[1],
            f64::from_bits(0x2000_0000_0000_0000),
            1.0,
            1.0,
        )
    })??;
    // Phase weights2^1022 and1 round to2^1022. Weighted variance is
    // MIN_POSITIVE, so its division by2^1022 rounds nonzero real2^-2044 to0.
    arithmetic_rejection(
        p.call(21, || combine_fixture_supporters(&decision, &[b, a]))?,
        FixtureCombinationStep::PhaseVarianceMean,
        ArithmeticOperation::Divide,
        ArithmeticIssue::ZeroUnderflow,
    )?;
    let neighbor = p.call(20, || {
        FixtureSupporterUncertainty::new(&built.entries[1], 1.0, 1.0, 1.0)
    })??;
    let outcome = p.call(21, || combine_fixture_supporters(&decision, &[a, neighbor]))??;
    // Neighbor mean variance1/2^1022=MIN_POSITIVE, sqrt=2^-511 exactly.
    observe_combined(
        &p,
        &outcome,
        &built,
        FixturePrincipalOutcomeKind::ReferenceReady,
        101,
        &[101, 202],
        [
            0,
            0,
            0x2000_0000_0000_0000,
            0x3fe6_a09e_667f_3bcc,
            0x3fe6_a09e_667f_3bcc,
        ],
    )?;
    p.assert_complete();
    Ok(())
}

/// A legal nonzero fitted rate can underflow at its weighted multiplication.
/// # Errors
/// Preserve actual producer, accepted-neighbor and accounting failures.
/// # Panics
/// The first tiny weighted rate must reject, while its normal neighbor is exact.
#[test]
fn combination_weighted_rate_subnormal_and_neighbor() -> TestResult {
    let p = Calls::new(
        "combination_weighted_rate_subnormal_and_neighbor",
        [
            2, 6, 6, 2, 7, 24, 6, 37, 2, 2, 2, 1, 1, 1, 2, 1, 1, 0, 0, 0, 3, 2,
        ],
    );
    let built = build(
        &p,
        &[
            zero_source(101, 1.0),
            Source {
                label: 202,
                phases: [25_083_904, 25_124_864, 25_165_824],
                distance: 2.0,
                phase_bits: 0x3f68_0000_0000_0000,
                rate_bits: 0x3ed4_0000_0000_0000,
            },
        ],
    )?;
    let batch = built.batch(&p)?;
    let roster = [tag(101)?, tag(202)?];
    let mut state = state(&p, &built, &batch, &roster, 2)?;
    let decision = p.call(16, || state.advance(&batch, &built.strata))??;
    principal(
        &p,
        &decision,
        FixturePrincipalOutcomeKind::ReferenceReady,
        Some(roster[0]),
    )?;
    let a = p.call(20, || {
        FixtureSupporterUncertainty::new(&built.entries[0], 1.0, 1.0, 1.0)
    })??;
    let b = p.call(20, || {
        FixtureSupporterUncertainty::new(
            &built.entries[1],
            1.0,
            f64::from_bits(0x5fe0_0000_0000_0000),
            1.0,
        )
    })??;
    // rateSD2^511 squares to2^1022; inverse2^-1022 times5*2^-20 is
    // the nonzero subnormal5*2^-1042. Phase/disagreement terms remain normal.
    let error = arithmetic_rejection(
        p.call(21, || combine_fixture_supporters(&decision, &[b, a]))?,
        FixtureCombinationStep::WeightedRate,
        ArithmeticOperation::Multiply,
        ArithmeticIssue::Subnormal,
    )?;
    borrowed_arithmetic_source(&p, &error)?;
    let neighbor = p.call(20, || {
        FixtureSupporterUncertainty::new(
            &built.entries[1],
            1.0,
            f64::from_bits(0x5f30_0000_0000_0000),
            1.0,
        )
    })??;
    let outcome = p.call(21, || combine_fixture_supporters(&decision, &[a, neighbor]))??;
    // NeighborSD2^500 gives weight2^-1000, weighted rate5*2^-1020;
    // total rate weight rounds to1, so this normal dyadic rate is exact.
    observe_combined(
        &p,
        &outcome,
        &built,
        FixturePrincipalOutcomeKind::ReferenceReady,
        101,
        &[101, 202],
        [
            0x3f50_0000_0000_0000,
            0x0054_0000_0000_0000,
            0x3ff0_0001_7fff_ee00,
            0x3ff0_0000_0000_0000,
            0x3fe6_a09e_667f_3bcc,
        ],
    )?;
    p.assert_complete();
    Ok(())
}

/// A half-ULP contribution discriminates sorted reduction from metadata order.
/// # Errors
/// Preserve genuine checked setup, consumer and bounded accounting failures.
/// # Panics
/// Both metadata permutations must match the complete independent rounded view.
#[test]
fn combination_canonical_reduction_rounding_order() -> TestResult {
    let p = Calls::new(
        "combination_canonical_reduction_rounding_order",
        [
            3, 9, 9, 3, 10, 36, 9, 61, 3, 3, 3, 1, 1, 1, 3, 1, 1, 0, 0, 0, 3, 2,
        ],
    );
    let built = build(&p, &cancellation_sources())?;
    let batch = built.batch(&p)?;
    let roster = [tag(101)?, tag(202)?, tag(303)?];
    let mut state = state(&p, &built, &batch, &roster, 3)?;
    let decision = p.call(16, || state.advance(&batch, &built.strata))??;
    principal(
        &p,
        &decision,
        FixturePrincipalOutcomeKind::ReferenceReady,
        Some(roster[0]),
    )?;
    let records = [
        p.call(20, || {
            FixtureSupporterUncertainty::new(&built.entries[0], 0.0, 1.0, 1.0)
        })??,
        p.call(20, || {
            FixtureSupporterUncertainty::new(&built.entries[1], 0.0, 1.0, 1.0)
        })??,
        p.call(20, || {
            FixtureSupporterUncertainty::new(&built.entries[2], 0.0, 1.0, 1.0)
        })??,
    ];
    // Canonical terms1,2^-53,-1 give(1+2^-53)-1=+0 under RNE;
    // supplied order1,-1,2^-53 would leave a positive nonzero numerator.
    // Independent exact-rational RNE and squared-midpoint root bracketing:
    // weighted variance RN4+2^-20; weight sum2+2^-20;
    // divided mean bits3fffffff80000400, square-root bits3ff6a09e393e003c.
    // Rate/skew sums3; RN sqrt3=3ffbb67ae8584caa, whose reciprocal is
    // bits3fe279a74590331d. No host sqrt or production result is the oracle.
    for supplied in [records, [records[0], records[2], records[1]]] {
        let outcome = p.call(21, || combine_fixture_supporters(&decision, &supplied))??;
        observe_combined(
            &p,
            &outcome,
            &built,
            FixturePrincipalOutcomeKind::ReferenceReady,
            101,
            &[101, 202, 303],
            [
                0,
                0,
                0x3ff6_a09e_393e_003c,
                0x3fe2_79a7_4590_331d,
                0x3fe2_79a7_4590_331d,
            ],
        )?;
    }
    p.assert_complete();
    Ok(())
}

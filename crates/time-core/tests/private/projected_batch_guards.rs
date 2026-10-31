// SPDX-License-Identifier: GPL-2.0-only
//! Genuine producer, direct raw-rule and private projected-batch guard observations.
//!
//! The public-attempt row retains actual producer work even on early failure.
//! The separate guard row is emitted only after its assertions and reports real
//! callback/adapter observations. Neither observer performs a checked transition.

use std::{
    error::Error,
    io::{self, Write},
    num::NonZeroU64,
};

use garde::Validate;
use time_domain::{MonotonicNs, UtcCoordinateQ32, exchange_arithmetic};

use crate::{
    FixtureContext, FixtureHistory, FixtureObservation, FixtureProjectionTerms,
    enclose_fixture_projection, project_fixture_anchor,
};

use super::FixtureProjectionMismatchKind::{EvaluationTime, Frame, MappingGeneration};
use super::{
    BatchError, FixtureProjectedBatch, FixtureProjectedBatchError, FixtureProjectedInterval,
    GUARD_OBSERVATIONS, GuardObservations, ProjectedBatchCandidate, ProjectionExpectation,
    first_issue, read_entry,
};

/// Preserve actual product, harness and observer failures at their owners.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;

/// Four closed private scopes, each with separately frozen finite source work.
#[derive(Clone, Copy)]
enum GuardCase {
    /// Public capacity/empty/singleton admission and actual raw/adapter neighbors.
    Admission,
    /// Direct raw validation and ordered index/field compatibility checks.
    Priority,
    /// Actual delegated duplicate cause and borrowed original collection.
    Delegation,
    /// Closed redacted errors and typed source chaining, without producer calls.
    Errors,
}

impl GuardCase {
    /// Return the fixed non-input-bearing marker name without effects.
    const fn name(self) -> &'static str {
        match self {
            Self::Admission => "projected_batch_private_admission",
            Self::Priority => "projected_batch_private_priority",
            Self::Delegation => "projected_batch_private_delegation",
            Self::Errors => "projected_batch_private_errors",
        }
    }
}

/// Stable public attempt families in the declared fourteen-field marker order.
#[derive(Clone, Copy)]
enum Call {
    /// Actual fixture-history construction attempts.
    Histories,
    /// Actual history append attempts.
    Appends,
    /// Actual fixture-observation construction attempts.
    Observations,
    /// Actual fixture-context construction attempts.
    Contexts,
    /// Actual unit-tagged monotonic construction attempts.
    Monotonic,
    /// Actual checked UTC-coordinate construction attempts.
    Coordinates,
    /// Actual exchange arithmetic attempts.
    Exchanges,
    /// Actual explicit public data getter requests made by this test scope.
    Getters,
    /// Actual projection-term construction attempts.
    Terms,
    /// Actual checked projection attempts.
    Projections,
    /// Actual checked enclosure attempts.
    Enclosures,
    /// Actual public projected-batch construction attempts.
    Batches,
}

impl Call {
    /// Return the fixed counter slot without conversion or product effects.
    const fn slot(self) -> usize {
        match self {
            Self::Histories => 0,
            Self::Appends => 1,
            Self::Observations => 2,
            Self::Contexts => 3,
            Self::Monotonic => 4,
            Self::Coordinates => 5,
            Self::Exchanges => 6,
            Self::Getters => 7,
            Self::Terms => 8,
            Self::Projections => 9,
            Self::Enclosures => 10,
            Self::Batches => 11,
        }
    }
}

/// Actual public attempts; unused minimum/selection slots remain zero here.
struct Profile {
    /// Closed scope identity, never derived from a fixture value.
    case: GuardCase,
    /// Fourteen nonwrapping attempt totals in the shared marker order.
    counts: [u64; 14],
    /// Whether the one allowed output attempt has begun.
    attempted: bool,
}

impl Profile {
    /// Start empty test-owned observation without any product operation.
    const fn new(case: GuardCase) -> Self {
        Self {
            case,
            counts: [0; 14],
            attempted: false,
        }
    }

    /// Charge an actual public attempt immediately before calling its owner.
    /// # Errors
    /// Reject a counter invariant or overflow rather than emitting a smaller count.
    fn attempt(&mut self, call: Call) -> io::Result<()> {
        let counter = self
            .counts
            .get_mut(call.slot())
            .ok_or_else(|| io::Error::other("projected-batch counter slot"))?;
        *counter = counter
            .checked_add(1)
            .ok_or_else(|| io::Error::other("projected-batch counter overflow"))?;
        Ok(())
    }

    /// Format the actual complete or failed-prefix attempts with no product work.
    /// # Errors
    /// Preserve stderr failure; partial or missing rows remain unobserved evidence.
    fn emit(&self) -> io::Result<()> {
        let [
            histories,
            appends,
            observations,
            contexts,
            monotonic,
            coordinates,
            exchanges,
            getters,
            terms,
            projections,
            enclosures,
            batches,
            minima,
            selections,
        ] = self.counts;
        let case = self.case.name();
        writeln!(
            io::stderr().lock(),
            "CORE_PROJECTED_BATCH_PROFILE case={case} histories={histories} appends={appends} observations={observations} contexts={contexts} monotonic={monotonic} coordinates={coordinates} exchanges={exchanges} getters={getters} terms={terms} projections={projections} enclosures={enclosures} batches={batches} minima={minima} selections={selections}"
        )
    }

    /// Finish the one output attempt and propagate its failure on a healthy path.
    /// # Errors
    /// Preserve stderr failure without retrying or changing any product result.
    fn finish(mut self) -> io::Result<()> {
        self.attempted = true;
        self.emit()
    }
}

impl Drop for Profile {
    /// Retain best-effort prefix evidence on unwind or an earlier returned failure.
    ///
    /// Healthy paths call `finish` and propagate I/O errors. This fallback cannot
    /// mask the original failure; absent/partial rows do not become zero or success.
    fn drop(&mut self) {
        if !self.attempted {
            self.attempted = true;
            drop(self.emit());
        }
    }
}

/// Test-owned lifetime of the passive per-thread garde/adapter observer.
struct ObservationScope {
    /// Whether this scope must still disable its own observer at drop.
    active: bool,
}

impl ObservationScope {
    /// Begin a fresh observation without erasing a previously active scope.
    /// # Errors
    /// Reject nested observation rather than resetting already recorded attempts.
    fn begin() -> io::Result<Self> {
        GUARD_OBSERVATIONS.with(|slot| {
            if slot.get().is_some() {
                return Err(io::Error::other("projected-batch observer already active"));
            }
            slot.set(Some(GuardObservations::default()));
            Ok(Self { active: true })
        })
    }

    /// Disable observation and return actual counts, never a fabricated default.
    /// # Errors
    /// Reject an absent observation as a harness invariant failure.
    fn finish(mut self) -> io::Result<GuardObservations> {
        let observed = GUARD_OBSERVATIONS.with(|slot| slot.replace(None));
        self.active = false;
        observed.ok_or_else(|| io::Error::other("projected-batch observation missing"))
    }
}

impl Drop for ObservationScope {
    /// Disable only this test-owned observer during ordinary drop or unwinding.
    fn drop(&mut self) {
        if self.active {
            GUARD_OBSERVATIONS.with(|slot| slot.set(None));
        }
    }
}

/// Emit asserted actual guard observations after checks, with no input-bearing text.
/// # Errors
/// Preserve a stderr failure; no missing row is substituted with zero.
fn guard_profile(case: GuardCase, actual: GuardObservations) -> io::Result<()> {
    let name = case.name();
    writeln!(
        io::stderr().lock(),
        "CORE_PROJECTED_BATCH_GUARD_PROFILE case={name} garde_callbacks={} entry_read_attempts={}",
        actual.garde_callbacks,
        actual.entry_read_attempts
    )
}

/// Admit one fixed nonzero fixture label without silently substituting a default.
/// # Errors
/// Reject zero as a harness error; this creates no product state or authority.
fn tag(value: u64) -> TestResult<NonZeroU64> {
    NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero projected-batch guard tag").into())
}

/// Count and execute one checked UTC construction before passing its result onward.
/// # Errors
/// Preserve actual counter or coordinate rejection without charging later calls.
fn coordinate(value: i128, profile: &mut Profile) -> TestResult<UtcCoordinateQ32> {
    profile.attempt(Call::Coordinates)?;
    Ok(UtcCoordinateQ32::new(value)?)
}

/// Build a genuine constant one-eighth-second projection and enclosure.
///
/// Exactly three real exchange observations at zero/one/two seconds enter the
/// actual history. Supplied tags and `at` stay fixture assertions. No checked
/// value is forged, no ambient clock is read and no history is retained here.
/// # Errors
/// Preserve actual counted producer errors; later calls are never precharged.
/// # Panics
/// Fails if the literal exchange phase differs from the independent Q33 fact.
fn genuine_constant(
    source: NonZeroU64,
    frame: NonZeroU64,
    generation: NonZeroU64,
    at: MonotonicNs,
    profile: &mut Profile,
) -> TestResult<FixtureProjectedInterval> {
    profile.attempt(Call::Contexts)?;
    let context = FixtureContext::new(source, frame, generation);
    profile.attempt(Call::Histories)?;
    let mut history = FixtureHistory::new(context);
    for raw_at in [0, 1_000_000_000, 2_000_000_000] {
        let t1 = coordinate(0, profile)?;
        let t2 = coordinate(1_i128 << 30, profile)?;
        let t3 = coordinate(0, profile)?;
        let t4 = coordinate(0, profile)?;
        profile.attempt(Call::Exchanges)?;
        let exchange = exchange_arithmetic(t1, t2, t3, t4)?;
        profile.attempt(Call::Getters)?;
        let phase = exchange.offset();
        profile.attempt(Call::Getters)?;
        assert_eq!(phase.numerator(), 1_i128 << 30);
        profile.attempt(Call::Monotonic)?;
        let row_at = MonotonicNs::new(raw_at);
        profile.attempt(Call::Observations)?;
        let observation = FixtureObservation::new(context, row_at, phase, 1.0)?;
        profile.attempt(Call::Appends)?;
        history.try_append(observation)?;
    }
    profile.attempt(Call::Terms)?;
    let terms = FixtureProjectionTerms::new(1.0 / 64.0, 0.0, 0.0)?;
    profile.attempt(Call::Monotonic)?;
    let anchor_at = MonotonicNs::new(0);
    profile.attempt(Call::Projections)?;
    let projection = project_fixture_anchor(&history, context, 0, anchor_at, at, terms)?;
    profile.attempt(Call::Enclosures)?;
    Ok(enclose_fixture_projection(&projection)?)
}

/// Public capacity rejection precedes callback/read work; raw validation stays bounded.
/// # Errors
/// Preserve genuine producer, observer or unexpected legal-neighbor rejection.
/// # Panics
/// Fails on capacity/read/callback or direct raw-adapter contract changes.
#[test]
fn projected_batch_private_admission() -> TestResult {
    let mut profile = Profile::new(GuardCase::Admission);
    profile.attempt(Call::Monotonic)?;
    let at = MonotonicNs::new(3_000_000_000);
    let frame = tag(43)?;
    let generation = tag(47)?;
    let expected = ProjectionExpectation {
        frame,
        mapping_generation: generation,
        at,
    };
    let entry = genuine_constant(tag(41)?, frame, generation, at, &mut profile)?;
    let excess = [entry; 17];
    let singleton = [entry];
    let observer = ObservationScope::begin()?;

    profile.attempt(Call::Batches)?;
    assert!(FixtureProjectedBatch::new(frame, generation, at, &[]).is_ok());
    profile.attempt(Call::Batches)?;
    assert!(matches!(
        FixtureProjectedBatch::new(frame, generation, at, &excess),
        Err(FixtureProjectedBatchError::CapacityExceeded)
    ));
    assert_eq!(
        first_issue(&excess, &expected),
        Some(FixtureProjectedBatchError::CapacityExceeded)
    );
    assert!(
        ProjectedBatchCandidate { values: &excess }
            .validate_with(&expected)
            .is_err()
    );
    profile.attempt(Call::Batches)?;
    assert!(FixtureProjectedBatch::new(frame, generation, at, &singleton).is_ok());
    assert!(read_entry(&singleton, 0).is_ok());
    assert!(matches!(
        read_entry(&singleton, usize::MAX),
        Err(FixtureProjectedBatchError::Invariant)
    ));

    let actual = observer.finish()?;
    assert_eq!(
        actual,
        GuardObservations {
            garde_callbacks: 3,
            entry_read_attempts: 5
        }
    );
    guard_profile(GuardCase::Admission, actual)?;
    profile.finish()?;
    Ok(())
}

/// Raw rule and typed predicate share field/index order, including valid neighbors.
/// # Errors
/// Preserve genuine producer and observation failures; no raw wrapper is forged.
/// # Panics
/// Fails if raw validation or the ordered typed issue differs from the case facts.
#[test]
fn projected_batch_private_priority() -> TestResult {
    let mut profile = Profile::new(GuardCase::Priority);
    profile.attempt(Call::Monotonic)?;
    let at = MonotonicNs::new(3_000_000_000);
    profile.attempt(Call::Monotonic)?;
    let later = MonotonicNs::new(3_000_000_001);
    let frame = tag(43)?;
    let generation = tag(47)?;
    let expected = ProjectionExpectation {
        frame,
        mapping_generation: generation,
        at,
    };
    let base = genuine_constant(tag(41)?, frame, generation, at, &mut profile)?;
    let other_generation = genuine_constant(tag(53)?, frame, tag(48)?, at, &mut profile)?;
    let other_frame = genuine_constant(tag(59)?, tag(44)?, generation, at, &mut profile)?;
    let one = [base];
    let generation_then_frame = [other_generation, other_frame];
    let frame_then_generation = [other_frame, other_generation];
    let time_then_generation = [base, other_generation];
    let generation_then_time = [other_generation, base];
    let later_frame = [base, other_frame];
    let wrong_all = ProjectionExpectation {
        frame: tag(44)?,
        mapping_generation: tag(48)?,
        at: later,
    };
    let wrong_generation_time = ProjectionExpectation {
        frame,
        mapping_generation: tag(48)?,
        at: later,
    };
    let wrong_time = ProjectionExpectation {
        frame,
        mapping_generation: generation,
        at: later,
    };
    let mismatch = |index, kind| Some(FixtureProjectedBatchError::Mismatch { index, kind });
    let observer = ObservationScope::begin()?;
    for (values, context, issue) in [
        (one.as_slice(), wrong_all, mismatch(0, Frame)),
        (
            one.as_slice(),
            wrong_generation_time,
            mismatch(0, MappingGeneration),
        ),
        (one.as_slice(), wrong_time, mismatch(0, EvaluationTime)),
        (
            generation_then_frame.as_slice(),
            expected,
            mismatch(0, MappingGeneration),
        ),
        (
            frame_then_generation.as_slice(),
            expected,
            mismatch(0, Frame),
        ),
        (
            time_then_generation.as_slice(),
            wrong_time,
            mismatch(0, EvaluationTime),
        ),
        (
            generation_then_time.as_slice(),
            wrong_time,
            mismatch(0, MappingGeneration),
        ),
        (later_frame.as_slice(), expected, mismatch(1, Frame)),
        (one.as_slice(), expected, None),
    ] {
        assert_eq!(first_issue(values, &context), issue);
        assert_eq!(
            ProjectedBatchCandidate { values }
                .validate_with(&context)
                .is_err(),
            issue.is_some()
        );
    }
    let actual = observer.finish()?;
    assert_eq!(
        actual,
        GuardObservations {
            garde_callbacks: 9,
            entry_read_attempts: 20
        }
    );
    guard_profile(GuardCase::Priority, actual)?;
    profile.finish()?;
    Ok(())
}

/// Accepted borrowed storage and an actual delegated duplicate preserve their owners.
/// # Errors
/// Preserve genuine producer, constructor, cause-access and observer failures.
/// # Panics
/// Fails if original storage, interval population, cause or observations change.
#[test]
fn projected_batch_private_delegation() -> TestResult {
    let mut profile = Profile::new(GuardCase::Delegation);
    profile.attempt(Call::Monotonic)?;
    let at = MonotonicNs::new(3_000_000_000);
    let frame = tag(43)?;
    let generation = tag(47)?;
    let first = genuine_constant(tag(41)?, frame, generation, at, &mut profile)?;
    let second = genuine_constant(tag(53)?, frame, generation, at, &mut profile)?;
    let values = [first, second];
    let duplicate = [first, first];
    let observer = ObservationScope::begin()?;
    profile.attempt(Call::Batches)?;
    let batch = FixtureProjectedBatch::new(frame, generation, at, &values)?;
    profile.attempt(Call::Getters)?;
    assert!(std::ptr::eq(batch.entries(), values.as_slice()));
    profile.attempt(Call::Getters)?;
    assert_eq!(batch.intervals.len(), 2);
    profile.attempt(Call::Batches)?;
    let Err(error) = FixtureProjectedBatch::new(frame, generation, at, &duplicate) else {
        return Err(io::Error::other("duplicate projected batch admitted").into());
    };
    let source = BatchError::DuplicateLabel { later_index: 1 };
    assert_eq!(error, FixtureProjectedBatchError::Batch { source });
    assert_eq!(
        error
            .source()
            .and_then(|cause| cause.downcast_ref::<BatchError>()),
        Some(&source)
    );
    let actual = observer.finish()?;
    assert_eq!(
        actual,
        GuardObservations {
            garde_callbacks: 2,
            entry_read_attempts: 12
        }
    );
    guard_profile(GuardCase::Delegation, actual)?;
    profile.finish()?;
    Ok(())
}

/// Closed formatting redacts raw-sized metadata while explicit source access preserves it.
/// # Errors
/// Preserve observer and output failures without inventing a passing marker.
/// # Panics
/// Fails on redaction, source chaining or unexpected observer activity.
#[test]
fn projected_batch_private_errors() -> TestResult {
    let profile = Profile::new(GuardCase::Errors);
    let observer = ObservationScope::begin()?;
    for (error, debug) in [
        (
            FixtureProjectedBatchError::CapacityExceeded,
            "FixtureProjectedBatchError::CapacityExceeded",
        ),
        (
            FixtureProjectedBatchError::Mismatch {
                index: usize::MAX,
                kind: EvaluationTime,
            },
            "FixtureProjectedBatchError::Mismatch { .. }",
        ),
        (
            FixtureProjectedBatchError::Invariant,
            "FixtureProjectedBatchError::Invariant",
        ),
    ] {
        assert_eq!(format!("{error:?}"), debug);
        assert!(error.to_string().len() < 64);
        assert!(error.source().is_none());
    }
    for source in [
        BatchError::CapacityExceeded,
        BatchError::DuplicateLabel {
            later_index: usize::MAX,
        },
        BatchError::Invariant,
    ] {
        let error = FixtureProjectedBatchError::Batch { source };
        assert_eq!(
            format!("{error:?}"),
            "FixtureProjectedBatchError::Batch { .. }"
        );
        assert_eq!(
            error.to_string(),
            "fixture projected batch: interval batch construction"
        );
        assert_eq!(
            error
                .source()
                .and_then(|cause| cause.downcast_ref::<BatchError>()),
            Some(&source)
        );
    }
    let actual = observer.finish()?;
    assert_eq!(actual, GuardObservations::default());
    guard_profile(GuardCase::Errors, actual)?;
    profile.finish()?;
    Ok(())
}

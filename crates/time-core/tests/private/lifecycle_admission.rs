// SPDX-License-Identifier: GPL-2.0-only

//! Actual private bridge rejection controls built from genuine public producers.
//!
//! Recorded subset is appends/admissions/masks/direct Garde calls, not every
//! fixture accessor or internal primitive. One fixed case builds two three-row
//! histories, two projections/enclosures, one batch/principal seed/advance and
//! three uncertainty constructors. Reserve six append fits/384 point admissions,
//! twelve full receiving passes, twelve sixteen-position mask scans, at most64
//! reduction binaries/six roots, plus the ordinary two-source fixture setup.
//! No checked decision, token, witness or state representation is forged.

use std::{
    cell::Cell,
    error::Error,
    io::{self, Write},
    num::NonZeroU64,
};

use garde::Validate;

use crate::{
    FixtureContext, FixtureHistory, FixtureMinimum, FixtureObservation, FixturePrincipalParameters,
    FixturePrincipalState, FixturePrincipalStratum, FixtureProjectedBatch,
    FixtureProjectedInterval, FixtureProjectionTerms, enclose_fixture_projection,
    project_fixture_anchor,
};
use time_domain::{MonotonicNs, UtcCoordinateQ32, exchange_arithmetic};

use super::{
    AdmittedFixtureSupporters, FixtureCombinationError, FixtureSupporterUncertainty,
    MAX_SUPPORTERS, RawContributors, RawSupporterAdmission, admit_lifecycle_supporters,
    combine_lifecycle_supporters,
};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

/// Checked pre-call counters record only the explicitly named subset.
#[derive(Default)]
struct Work {
    appends: Cell<u64>,
    admissions: Cell<u64>,
    masks: Cell<u64>,
    garde: Cell<u64>,
}

/// Precharge before entering the real operation; never refund a rejected prefix.
/// # Errors
/// Reject overflow or exhaustion before executing the callback.
fn attempt<T>(count: &Cell<u64>, maximum: u64, call: impl FnOnce() -> T) -> TestResult<T> {
    let next = count
        .get()
        .checked_add(1)
        .ok_or_else(|| io::Error::other("bridge counter overflow"))?;
    if next > maximum {
        return Err(io::Error::other("bridge counter exhausted").into());
    }
    count.set(next);
    Ok(call())
}

impl Drop for Work {
    /// Retain the actual observed subset on success or assertion unwind.
    fn drop(&mut self) {
        let _emission = writeln!(
            io::stderr().lock(),
            "CORE_LIFECYCLE_BRIDGE_PROFILE case=bridge_receiving_and_mask_boundaries appends={} admissions={} masks={} garde={}",
            self.appends.get(),
            self.admissions.get(),
            self.masks.get(),
            self.garde.get()
        );
    }
}

/// Produce a checked fixture label from a literal nonzero test identifier.
/// # Errors
/// Reject a mistaken zero without creating a private domain representation.
fn tag(raw: u64) -> TestResult<NonZeroU64> {
    Ok(NonZeroU64::new(raw).ok_or_else(|| io::Error::other("zero fixture label"))?)
}

/// Produce an original flat three-row projection/enclosure, with no retained history borrow.
/// # Errors
/// Preserve actual history, numerical and precharge failures.
fn entry(work: &Work, label: NonZeroU64, distance: f64) -> TestResult<FixtureProjectedInterval> {
    let context = FixtureContext::new(label, tag(7)?, tag(11)?);
    let mut history = FixtureHistory::new(context);
    let zero = UtcCoordinateQ32::new(0)?;
    let phase = exchange_arithmetic(zero, zero, zero, zero)?.offset();
    for raw in [0, 1_000_000_000, 2_000_000_000] {
        let row = FixtureObservation::new(context, MonotonicNs::new(raw), phase, 1.0)?;
        attempt(&work.appends, 6, || history.try_append(row))??;
    }
    let terms = FixtureProjectionTerms::new(distance, 0.0, 0.0)?;
    let projection = project_fixture_anchor(
        &history,
        context,
        0,
        MonotonicNs::new(0),
        MonotonicNs::new(2_000_000_000),
        terms,
    )?;
    Ok(enclose_fixture_projection(&projection)?)
}

/// Challenge masks only after the original witness has passed full admission.
/// # Errors
/// Preserve actual reduction and precharge failures on healthy masks.
/// # Panics
/// Mandatory mask rules, contributor membership and ordinary-reduction zero sign hold.
fn check_contributor_masks(
    work: &Work,
    admitted: &AdmittedFixtureSupporters<'_, '_>,
) -> TestResult {
    let mut one = [false; MAX_SUPPORTERS];
    one[0] = true;
    let mut all = one;
    all[1] = true;
    let raw_good = RawContributors {
        inputs: (admitted, one),
    };
    let raw_bad = RawContributors {
        inputs: (admitted, [false; MAX_SUPPORTERS]),
    };
    assert!(attempt(&work.garde, 4, || raw_good.validate())?.is_ok());
    assert!(attempt(&work.garde, 4, || raw_bad.validate())?.is_err());
    let combined = attempt(&work.masks, 5, || {
        combine_lifecycle_supporters(admitted, all)
    })??;
    assert_eq!(combined.supporter_count(), 2);
    let combined = attempt(&work.masks, 5, || {
        combine_lifecycle_supporters(admitted, one)
    })??;
    assert_eq!(combined.supporter_count(), 1);
    assert_eq!(combined.phase_standard_deviation_seconds().to_bits(), 0);
    let mut nonprincipal = [false; MAX_SUPPORTERS];
    nonprincipal[1] = true;
    let mut suffix = one;
    suffix[15] = true;
    for invalid in [[false; MAX_SUPPORTERS], nonprincipal, suffix] {
        assert!(matches!(
            attempt(&work.masks, 5, || combine_lifecycle_supporters(
                admitted, invalid
            ))?,
            Err(FixtureCombinationError::Invariant)
        ));
    }
    Ok(())
}

/// Challenge the actual private receivers with genuine values and valid neighbors.
/// # Errors
/// Preserve producer/receiver/precharge failures on all expected successful paths.
/// # Panics
/// Full admission, mandatory rules, exact contributor membership and zero sign must hold.
#[test]
fn bridge_receiving_and_mask_boundaries() -> TestResult {
    let work = Work::default();
    let roster = [tag(101)?, tag(202)?];
    let entries = [entry(&work, roster[0], 1.0)?, entry(&work, roster[1], 2.0)?];
    let batch =
        FixtureProjectedBatch::new(tag(7)?, tag(11)?, MonotonicNs::new(2_000_000_000), &entries)?;
    let strata = [
        FixturePrincipalStratum::new(roster[0], 1)?,
        FixturePrincipalStratum::new(roster[1], 1)?,
    ];
    let mut principal = FixturePrincipalState::seed(
        &roster,
        FixturePrincipalParameters::new(0.25, 0.0)?,
        FixtureMinimum::new(2)?,
        &batch,
        &strata,
    )?;
    let decision = principal.advance(&batch, &strata)?;
    let records = [
        FixtureSupporterUncertainty::new(&entries[0], -0.0, 1.0, 1.0)?,
        FixtureSupporterUncertainty::new(&entries[1], 0.0, 1.0, 1.0)?,
    ];
    let admitted = attempt(&work.admissions, 5, || {
        admit_lifecycle_supporters(&decision, &records)
    })??
    .ok_or_else(|| io::Error::other("missing genuine admitted witness"))?;
    assert_eq!(admitted.count(), 2);
    let mut labels = admitted.labels();
    assert_eq!(labels.next(), Some(roster[0]));
    assert_eq!(labels.next(), Some(roster[1]));
    assert_eq!(labels.next(), None);
    assert!(admitted.record(tag(999)?).is_err());
    assert!(matches!(
        attempt(&work.admissions, 5, || admit_lifecycle_supporters(
            &decision,
            &records[..1]
        ))?,
        Err(FixtureCombinationError::MetadataCountMismatch)
    ));
    let too_many = [records[0]; 17];
    assert!(matches!(
        attempt(&work.admissions, 5, || admit_lifecycle_supporters(
            &decision, &too_many
        ))?,
        Err(FixtureCombinationError::CapacityExceeded)
    ));
    let copied_entry = entries[0];
    let foreign = [
        FixtureSupporterUncertainty::new(&copied_entry, 0.0, 1.0, 1.0)?,
        records[1],
    ];
    assert!(matches!(
        attempt(&work.admissions, 5, || admit_lifecycle_supporters(
            &decision, &foreign
        ))?,
        Err(FixtureCombinationError::EntryMismatch { index: 0 })
    ));
    let duplicate = [records[0], records[0]];
    assert!(matches!(
        attempt(&work.admissions, 5, || admit_lifecycle_supporters(
            &decision, &duplicate
        ))?,
        Err(FixtureCombinationError::DuplicateUncertainty { index: 1 })
    ));

    let raw_good = RawSupporterAdmission {
        inputs: (&decision, &records, admitted.principal()),
    };
    let raw_bad = RawSupporterAdmission {
        inputs: (&decision, &records[..1], admitted.principal()),
    };
    assert!(attempt(&work.garde, 4, || raw_good.validate())?.is_ok());
    assert!(attempt(&work.garde, 4, || raw_bad.validate())?.is_err());
    check_contributor_masks(&work, &admitted)?;
    assert_eq!(
        (
            work.appends.get(),
            work.admissions.get(),
            work.masks.get(),
            work.garde.get()
        ),
        (6, 5, 5, 4)
    );
    Ok(())
}

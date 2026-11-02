// SPDX-License-Identifier: GPL-2.0-only
//! Independent retained-prefix fixture controls; CHECK-7/20/25, not source policy.
//!
//! Four bounded scopes reserve at most 125 point constructions, seven input
//! constructions and fourteen selector entries in total. Each scope additionally
//! caps attempts before calling a product owner; actual prefixes are emitted even
//! when an assertion unwinds. Internal candidate fits/scans belong to the selector
//! and are not counted as direct calls here. No histories, clocks or I/O effects.
#![forbid(unsafe_code)]

use std::{
    cell::Cell,
    error::Error,
    io::{self, Write},
    num::NonZeroU64,
};

use time_core::{
    FitInput, FitPoint, FixtureRunsError, FixtureRunsSelection, FixtureRunsStage, FixtureRunsStop,
    select_fixture_runs, select_fixture_runs_with_retained,
};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

/// Exact-real oracle numerators over 512, with the independent second-round row.
const VALUES: [i32; 13] = [512, -21, 27, 27, 27, -27, -27, -27, -27, 16, 16, 16, 32];
/// Fixed absolute bound 2^-40 for these small well-conditioned binary64 fixtures.
const COEFFICIENT_TOLERANCE: f64 = 1.0 / 1_099_511_627_776.0;

/// Fixed local attempt caps; no count is inferred from a successful return.
struct Profile {
    /// Fixed source test identity for distinguishing native attempt records.
    case: &'static str,
    /// Attempts to construct checked fit points in this scope.
    points: Cell<usize>,
    /// Attempts to construct complete checked fit inputs in this scope.
    inputs: Cell<usize>,
    /// Calls to either public selector in this scope.
    selections: Cell<usize>,
}

impl Profile {
    /// Start a bounded local attempt record without invoking a product owner.
    const fn new(case: &'static str) -> Self {
        Self {
            case,
            points: Cell::new(0),
            inputs: Cell::new(0),
            selections: Cell::new(0),
        }
    }

    /// Construct one checked complete input from literal chronological values.
    /// # Errors
    /// Preserve local cap, exact index conversion and checked owner failures.
    fn input(&self, values: &[i32]) -> TestResult<FitInput> {
        if !(3..=64).contains(&values.len()) {
            return Err(io::Error::other("fixture length outside checked input range").into());
        }
        let last = i32::try_from(
            values
                .len()
                .checked_sub(1)
                .ok_or_else(|| io::Error::other("empty fixture"))?,
        )?;
        let mut points = Vec::with_capacity(values.len());
        for (position, value) in values.iter().copied().enumerate() {
            charge(&self.points, 64)?;
            let x = f64::from(i32::try_from(position)? - last);
            points.push(FitPoint::new(x, f64::from(value) / 512.0, 1.0)?);
        }
        charge(&self.inputs, 3)?;
        let frame = NonZeroU64::new(17).ok_or_else(|| io::Error::other("zero fixture frame"))?;
        Ok(FitInput::new(frame, &points)?)
    }

    /// Enter the unchanged m=0 owner after charging this scope.
    /// # Errors
    /// Preserve local attempt-cap and original selector failures.
    fn old(&self, input: &FitInput, minimum: usize) -> TestResult<FixtureRunsSelection> {
        charge(&self.selections, 8)?;
        Ok(select_fixture_runs(input, minimum)?)
    }

    /// Enter the new retained-prefix owner without changing its actual error.
    /// # Errors
    /// Preserve the local attempt cap; the inner result is the actual selector result.
    fn retained(
        &self,
        input: &FitInput,
        prefix: usize,
        minimum: usize,
    ) -> TestResult<Result<FixtureRunsSelection, FixtureRunsError>> {
        charge(&self.selections, 8)?;
        Ok(select_fixture_runs_with_retained(input, prefix, minimum))
    }
}

impl Drop for Profile {
    /// Emit only fixed attempt counts, including on an assertion unwind.
    fn drop(&mut self) {
        let _emission = writeln!(
            io::stderr().lock(),
            "CORE_RETAINED_RUNS_PROFILE case={} points={} inputs={} selections={}",
            self.case,
            self.points.get(),
            self.inputs.get(),
            self.selections.get()
        );
    }
}

/// Charge before the actual owner call and reject counter overflow/exhaustion.
/// # Errors
/// Fail before invoking a product owner when the local ceiling would be exceeded.
fn charge(counter: &Cell<usize>, maximum: usize) -> TestResult {
    let next = counter
        .get()
        .checked_add(1)
        .ok_or_else(|| io::Error::other("attempt overflow"))?;
    if next > maximum {
        return Err(io::Error::other("attempt ceiling").into());
    }
    counter.set(next);
    Ok(())
}

/// Independently compare this fixture's finite coefficient to exact-real facts.
/// # Panics
/// A nonfinite coefficient or deviation above 2^-40 rejects the observation.
fn near(actual: f64, expected: f64) {
    assert!(actual.is_finite());
    assert!((actual - expected).abs() <= COEFFICIENT_TOLERANCE);
}

/// Check the line from the actual selected suffix and its own report.
/// # Panics
/// A missing or different fit count, frame, phase, or rate rejects the observation.
fn line(selection: &FixtureRunsSelection, count: usize, phase: f64, rate: f64) {
    let statistics = selection.statistics();
    assert_eq!(statistics.sample_count(), count);
    assert_eq!(statistics.degrees_of_freedom(), count - 2);
    assert_eq!(statistics.frame(), selection.frame());
    near(statistics.line().phase_seconds(), phase);
    near(statistics.line().phase_rate_seconds_per_second(), rate);
}

/// The retained row changes the next decision without entering the fitted suffix.
/// # Errors
/// Propagate real fixture and selector rejections, including the intentional RED.
/// # Panics
/// Both rounds and the no-retained counterexample must match independent facts.
#[test]
fn retained_prefix_preserves_second_round_information() -> TestResult {
    let p = Profile::new("retained_prefix_preserves_second_round_information");
    let first = p.input(&VALUES[..12])?;
    let old = p.old(&first, 3)?;
    assert_eq!(
        (
            old.start(),
            old.sample_count(),
            old.decision_start(),
            old.decision_count()
        ),
        (1, 11, 0, 12)
    );
    assert_eq!(
        (old.decision_runs(), old.fit_runs(), old.stop()),
        (5, 4, FixtureRunsStop::RunsPassed)
    );
    line(&old, 11, 0.0, 0.0);

    let combined = p.input(&VALUES)?;
    let retained = p.retained(&combined, 1, 3)??;
    assert_eq!(
        (
            retained.start(),
            retained.sample_count(),
            retained.decision_start(),
            retained.decision_count()
        ),
        (1, 12, 0, 13)
    );
    assert_eq!(
        (
            retained.decision_runs(),
            retained.fit_runs(),
            retained.stop()
        ),
        (5, 4, FixtureRunsStop::RunsPassed)
    );
    line(&retained, 12, 23.0 / 1248.0, 1.0 / 416.0);

    let without_extra = p.input(&VALUES[1..])?;
    let lost = p.old(&without_extra, 3)?;
    assert_eq!(
        (
            lost.start(),
            lost.sample_count(),
            lost.decision_start(),
            lost.decision_count()
        ),
        (4, 8, 0, 12)
    );
    assert_eq!(
        (lost.decision_runs(), lost.fit_runs(), lost.stop()),
        (5, 5, FixtureRunsStop::RunsPassed)
    );
    line(&lost, 8, 179.0 / 3072.0, 25.0 / 1344.0);
    assert_eq!(
        (p.points.get(), p.inputs.get(), p.selections.get()),
        (37, 3, 3)
    );
    Ok(())
}

/// Zero retained rows preserve all old selection and associated statistics bits.
/// # Errors
/// Preserve checked fixture and either selector's actual rejection.
/// # Panics
/// Reject any metadata or numerical drift for the unchanged old m=0 contract.
#[test]
fn zero_prefix_preserves_legacy_selection() -> TestResult {
    let p = Profile::new("zero_prefix_preserves_legacy_selection");
    let input = p.input(&VALUES)?;
    let old = p.old(&input, 3)?;
    let new = p.retained(&input, 0, 3)??;
    assert_eq!(
        (
            new.start(),
            new.sample_count(),
            new.frame(),
            new.stop(),
            new.decision_start(),
            new.decision_count(),
            new.decision_runs(),
            new.fit_runs()
        ),
        (
            old.start(),
            old.sample_count(),
            old.frame(),
            old.stop(),
            old.decision_start(),
            old.decision_count(),
            old.decision_runs(),
            old.fit_runs()
        )
    );
    let (a, b) = (new.statistics(), old.statistics());
    assert_eq!(
        (a.frame(), a.sample_count(), a.degrees_of_freedom()),
        (b.frame(), b.sample_count(), b.degrees_of_freedom())
    );
    assert_eq!(
        a.line().phase_seconds().to_bits(),
        b.line().phase_seconds().to_bits()
    );
    assert_eq!(
        a.line().phase_rate_seconds_per_second().to_bits(),
        b.line().phase_rate_seconds_per_second().to_bits()
    );
    assert_eq!(
        a.weighted_sse_seconds_squared().to_bits(),
        b.weighted_sse_seconds_squared().to_bits()
    );
    assert_eq!(
        a.residual_scale_seconds_squared().to_bits(),
        b.residual_scale_seconds_squared().to_bits()
    );
    assert_eq!(
        a.normalized_data_variance_seconds_squared().to_bits(),
        b.normalized_data_variance_seconds_squared().to_bits()
    );
    assert_eq!(
        (p.points.get(), p.inputs.get(), p.selections.get()),
        (13, 1, 2)
    );
    Ok(())
}

/// Prefix geometry is rejected before minimum; a true three-point fit is legal.
/// # Errors
/// Preserve actual input construction and local attempt-accounting failures.
/// # Panics
/// Each bad boundary must expose its exact closed stage and preserve input facts.
#[test]
fn invalid_prefix_and_minimum_boundaries() -> TestResult {
    let p = Profile::new("invalid_prefix_and_minimum_boundaries");
    let four = p.input(&[0; 4])?;
    let seven = p.input(&[0; 7])?;
    for (input, prefix, minimum, stage) in [
        (&four, usize::MAX, 0, FixtureRunsStage::Suffix),
        (&four, 4, 3, FixtureRunsStage::Suffix),
        (&four, 2, 3, FixtureRunsStage::Suffix),
        (&seven, 4, 3, FixtureRunsStage::Suffix),
        (&four, 1, 2, FixtureRunsStage::Minimum),
        (&four, 1, 4, FixtureRunsStage::Minimum),
    ] {
        let observed = p.retained(input, prefix, minimum)?;
        let error = observed
            .err()
            .ok_or_else(|| io::Error::other("invalid selector accepted"))?;
        assert_eq!(error.stage(), stage);
        assert_eq!(error.candidate_start(), None);
        assert_eq!(error.point_index(), None);
        assert!(error.source().is_none());
    }
    let legal = p.retained(&four, 1, 3)??;
    assert_eq!(
        (
            legal.start(),
            legal.sample_count(),
            legal.decision_start(),
            legal.decision_count()
        ),
        (1, 3, 0, 4)
    );
    assert_eq!((four.sample_count(), seven.sample_count()), (4, 7));
    assert_eq!(
        (p.points.get(), p.inputs.get(), p.selections.get()),
        (11, 2, 7)
    );
    Ok(())
}

/// Exactly 64 combined rows admit 32 fit and 32 auxiliary rows, but not 33/31.
/// # Errors
/// Preserve checked input and local attempt-accounting failures.
/// # Panics
/// Reject an off-by-one count, changed window or wrong failure stage.
#[test]
fn maximum_combined_window_and_prefix_boundary() -> TestResult {
    let p = Profile::new("maximum_combined_window_and_prefix_boundary");
    let input = p.input(&[0; 64])?;
    let selected = p.retained(&input, 32, 32)??;
    assert_eq!(
        (
            selected.start(),
            selected.sample_count(),
            selected.decision_start(),
            selected.decision_count()
        ),
        (32, 32, 0, 64)
    );
    assert_eq!(
        (
            selected.decision_runs(),
            selected.fit_runs(),
            selected.stop()
        ),
        (64, 32, FixtureRunsStop::RunsPassed)
    );
    line(&selected, 32, 0.0, 0.0);
    let error = p
        .retained(&input, 33, 3)?
        .err()
        .ok_or_else(|| io::Error::other("excess prefix accepted"))?;
    assert_eq!(error.stage(), FixtureRunsStage::Suffix);
    assert_eq!(error.candidate_start(), None);
    assert_eq!(error.point_index(), None);
    assert!(error.source().is_none());
    assert_eq!(input.sample_count(), 64);
    assert_eq!(
        (p.points.get(), p.inputs.get(), p.selections.get()),
        (64, 1, 2)
    );
    Ok(())
}

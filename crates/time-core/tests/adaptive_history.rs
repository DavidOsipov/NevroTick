// SPDX-License-Identifier: GPL-2.0-only
//! Independent sequential fixture controls for CHECK-7/20/25, not source policy.
//!
//! Fixed profiles cap each test at 65 observations and append calls. Across the
//! eight tests there are at most 200 of each and thirteen history constructions. Each
//! append can admit at most 64 rows and try at most 62 suffix fits of at most 64
//! rows; these are upper bounds on owner work, not measured operation counts.
//! The exact-real trace first prunes at t=8, retaining t=0 for later decisions.
#![forbid(unsafe_code)]

use std::{
    cell::Cell,
    error::Error,
    io::{self, Write},
    num::NonZeroU64,
};
use time_core::{
    AdaptiveHistoryError, FitError, FixtureAdaptiveHistory, FixtureContext, FixtureObservation,
    FixtureRunsError, FixtureRunsStage, FixtureRunsStop, NormalizationError,
};
use time_domain::{MonotonicNs, UtcCoordinateQ32, exchange_arithmetic};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;
const VALUES: [i32; 13] = [512, -21, 27, 27, 27, -27, -27, -27, -27, 16, 16, 16, 32];
const TOLERANCE: f64 = 1.0 / 1_099_511_627_776.0;
const SECOND: u64 = 1_000_000_000;

struct Profile {
    /// Fixed source test identity for distinguishing native attempt records.
    case: &'static str,
    observations: Cell<usize>,
    appends: Cell<usize>,
    histories: Cell<usize>,
}
impl Profile {
    /// Start zeroed local attempt counters without calling an owner or allocating.
    const fn new(case: &'static str) -> Self {
        Self {
            case,
            observations: Cell::new(0),
            appends: Cell::new(0),
            histories: Cell::new(0),
        }
    }
    /// Charge and invoke the real history constructor; preserve its actual result.
    /// # Errors
    /// Reject an exhausted local counter before invoking the constructor.
    fn history(
        &self,
        context: FixtureContext,
        minimum: usize,
    ) -> TestResult<Result<FixtureAdaptiveHistory, AdaptiveHistoryError>> {
        charge(&self.histories, 6)?;
        Ok(FixtureAdaptiveHistory::new(context, minimum))
    }
    /// Build one checked observation from a context, second and numerator over 512.
    /// # Errors
    /// Preserve the attempt cap, time overflow and actual domain/admission errors;
    /// no history or external state changes.
    fn observation(
        &self,
        context: FixtureContext,
        second: u64,
        numerator: i32,
    ) -> TestResult<FixtureObservation> {
        let nanos = second
            .checked_mul(SECOND)
            .ok_or_else(|| io::Error::other("time overflow"))?;
        self.observation_at_ns(context, nanos, numerator)
    }
    /// Build one checked observation from an exact monotonic nanosecond value.
    /// # Errors
    /// Preserve the local cap and real domain/admission errors; no history or
    /// external state changes, and one observation is charged at most once.
    fn observation_at_ns(
        &self,
        context: FixtureContext,
        nanos: u64,
        numerator: i32,
    ) -> TestResult<FixtureObservation> {
        self.observation_at_ns_q32(context, nanos, i128::from(numerator) * (1_i128 << 23))
    }
    /// Build one observation from exact monotonic nanoseconds and signed Q32 ticks.
    /// # Errors
    /// Preserve the local cap and actual domain/admission errors; no history or
    /// external state changes, and one observation is charged at most once.
    fn observation_at_ns_q32(
        &self,
        context: FixtureContext,
        nanos: u64,
        phase_ticks: i128,
    ) -> TestResult<FixtureObservation> {
        charge(&self.observations, 65)?;
        let zero = UtcCoordinateQ32::new(0)?;
        let shifted = UtcCoordinateQ32::new(phase_ticks)?;
        let phase = exchange_arithmetic(zero, shifted, shifted, zero)?.offset();
        Ok(FixtureObservation::new(
            context,
            MonotonicNs::new(nanos),
            phase,
            1.0,
        )?)
    }
    /// Charge and attempt one append, returning its actual owner result unchanged.
    /// # Errors
    /// Reject an exhausted counter before calling the owner; the owner controls
    /// whether the supplied history changes.
    fn append(
        &self,
        history: &mut FixtureAdaptiveHistory,
        row: FixtureObservation,
    ) -> TestResult<Result<(), AdaptiveHistoryError>> {
        charge(&self.appends, 65)?;
        Ok(history.try_append(row))
    }
}
impl Drop for Profile {
    /// Emit only fixed attempt counts to stderr on ordinary drop or unwinding;
    /// ignore output failure and do not alter any product state.
    fn drop(&mut self) {
        let _result = writeln!(
            io::stderr().lock(),
            "CORE_ADAPTIVE_HISTORY_PROFILE case={} observations={} appends={} histories={}",
            self.case,
            self.observations.get(),
            self.appends.get(),
            self.histories.get()
        );
    }
}
/// Charge a bounded attempt before invoking its owner, with no product effects.
/// # Errors
/// Reject overflow or exhaustion before changing the counter.
fn charge(counter: &Cell<usize>, cap: usize) -> TestResult {
    let next = counter
        .get()
        .checked_add(1)
        .ok_or_else(|| io::Error::other("attempt overflow"))?;
    if next > cap {
        return Err(io::Error::other("attempt ceiling").into());
    }
    counter.set(next);
    Ok(())
}
/// Build three nonzero fixture labels without acquiring trust or external state.
/// # Errors
/// Reject a zero supplied generation; fixed source and frame labels are nonzero.
fn context(generation: u64) -> TestResult<FixtureContext> {
    let nonzero = |value| NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero tag"));
    Ok(FixtureContext::new(
        nonzero(11)?,
        nonzero(17)?,
        nonzero(generation)?,
    ))
}
/// Compare a finite coefficient to the fixed independent tolerance without effects.
/// # Panics
/// Reject a nonfinite value or a deviation greater than 2^-40.
fn near(actual: f64, expected: f64) {
    assert!(actual.is_finite());
    assert!((actual - expected).abs() <= TOLERANCE);
}
/// Copy the public immutable identity and exact scalar bits without effects.
fn row(row: &FixtureObservation) -> (FixtureContext, u64, i128, u64) {
    (
        row.context(),
        row.observed_at().nanoseconds(),
        row.phase().numerator(),
        row.variance_divisor().to_bits(),
    )
}

#[derive(Debug, PartialEq)]
struct Snapshot {
    context: FixtureContext,
    minimum: usize,
    count: usize,
    auxiliary: usize,
    reference: Option<u64>,
    fitted: Vec<(FixtureContext, u64, i128, u64)>,
    decision: Vec<(FixtureContext, u64, i128, u64)>,
    report: Option<ReportSnapshot>,
}
#[derive(Debug, PartialEq)]
struct ReportSnapshot {
    context: FixtureContext,
    reference: u64,
    configured: usize,
    effective: usize,
    start: usize,
    count: usize,
    frame: NonZeroU64,
    stop: FixtureRunsStop,
    decision_start: usize,
    decision_count: usize,
    decision_runs: usize,
    fit_runs: usize,
    stats_count: usize,
    degrees: usize,
    stats_frame: NonZeroU64,
    phase: u64,
    rate: u64,
    weighted_sse: u64,
    residual_scale: u64,
    variance: u64,
}
/// Copy the complete bounded public view, including both row views and report bits.
/// # Errors
/// Reject inconsistent or excessive counts and absent required rows before
/// publishing a snapshot; only the test-owned vectors may allocate.
/// # Panics
/// Reject exposed rows outside either declared index range.
fn snapshot(history: &FixtureAdaptiveHistory) -> TestResult<Snapshot> {
    let count = history.sample_count();
    let auxiliary = history.auxiliary_count();
    let total = count
        .checked_add(auxiliary)
        .ok_or_else(|| io::Error::other("count overflow"))?;
    if count > 64 || auxiliary > 64 || auxiliary > count || total > 64 {
        return Err(io::Error::other("history count outside fixture bounds").into());
    }
    let fitted = (0..count)
        .map(|i| {
            history
                .observation(i)
                .map(row)
                .ok_or_else(|| io::Error::other("missing fitted row"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let decision = (0..total)
        .map(|i| {
            history
                .decision_observation(i)
                .map(row)
                .ok_or_else(|| io::Error::other("missing decision row"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    assert!(history.observation(count).is_none());
    assert!(history.observation(usize::MAX).is_none());
    assert!(history.decision_observation(total).is_none());
    assert!(history.decision_observation(usize::MAX).is_none());
    let report = history.report().map(|report| {
        let selection = report.selection();
        let stats = selection.statistics();
        ReportSnapshot {
            context: report.context(),
            reference: report.reference().nanoseconds(),
            configured: report.configured_minimum(),
            effective: report.effective_minimum(),
            start: selection.start(),
            count: selection.sample_count(),
            frame: selection.frame(),
            stop: selection.stop(),
            decision_start: selection.decision_start(),
            decision_count: selection.decision_count(),
            decision_runs: selection.decision_runs(),
            fit_runs: selection.fit_runs(),
            stats_count: stats.sample_count(),
            degrees: stats.degrees_of_freedom(),
            stats_frame: stats.frame(),
            phase: stats.line().phase_seconds().to_bits(),
            rate: stats.line().phase_rate_seconds_per_second().to_bits(),
            weighted_sse: stats.weighted_sse_seconds_squared().to_bits(),
            residual_scale: stats.residual_scale_seconds_squared().to_bits(),
            variance: stats.normalized_data_variance_seconds_squared().to_bits(),
        }
    });
    Ok(Snapshot {
        context: history.context(),
        minimum: history.configured_minimum(),
        count,
        auxiliary,
        reference: history.reference().map(MonotonicNs::nanoseconds),
        fitted,
        decision,
        report,
    })
}
/// Append a bounded literal prefix in order, preserving every real owner result.
/// # Errors
/// Reject local caps, index conversion or a constructor/append failure; successful
/// earlier appends remain committed and no external state is changed.
fn append_trace(
    profile: &Profile,
    history: &mut FixtureAdaptiveHistory,
    context: FixtureContext,
    through: usize,
) -> TestResult {
    for (index, value) in VALUES.iter().copied().enumerate().take(through) {
        let second = u64::try_from(index)?;
        profile.append(history, profile.observation(context, second, value)?)??;
    }
    Ok(())
}

/// Check actual Debug projections of the nonzero bounded fixture history/report.
/// The caller has already checked their numerical line; this formats local strings
/// only, changes no product state and publishes no diagnostic payload.
/// # Panics
/// Reject exposed observation/fit fields or the actual phase/rate sentinels.
fn assert_redacted_debug(
    history: &FixtureAdaptiveHistory,
    report: &time_core::FixtureAdaptiveHistoryReport,
) {
    let debug_history = format!("{history:?}");
    let debug_report = format!("{report:?}");
    assert!(debug_history.contains("FixtureAdaptiveHistory"));
    assert!(debug_report.contains("FixtureAdaptiveHistoryReport"));
    let phase_sentinel = report
        .selection()
        .statistics()
        .line()
        .phase_seconds()
        .to_string();
    let rate_sentinel = report
        .selection()
        .statistics()
        .line()
        .phase_rate_seconds_per_second()
        .to_string();
    for text in [&debug_history, &debug_report] {
        for forbidden in [
            "phase",
            "rate",
            "observations",
            "statistics",
            "variance_divisor",
            "observed_at",
            "weighted_sse",
        ] {
            assert!(!text.contains(forbidden));
        }
        assert!(!text.contains(&phase_sentinel));
        assert!(!text.contains(&rate_sentinel));
    }
}

/// # Errors
/// Preserve real constructor and append failures.
/// # Panics
/// Reject any mismatch against the independently derived two-round trace.
#[test]
fn adaptive_retains_decision_prefix_across_two_rounds() -> TestResult {
    let p = Profile::new("adaptive_retains_decision_prefix_across_two_rounds");
    let ctx = context(1)?;
    let mut history = p.history(ctx, 3)??;
    append_trace(&p, &mut history, ctx, 8)?;
    assert_eq!((history.auxiliary_count(), history.sample_count()), (0, 8));
    p.append(&mut history, p.observation(ctx, 8, VALUES[8])?)??;
    assert_eq!((history.auxiliary_count(), history.sample_count()), (1, 8));
    for (index, value) in VALUES.iter().copied().enumerate().skip(9).take(3) {
        p.append(
            &mut history,
            p.observation(ctx, u64::try_from(index)?, value)?,
        )??;
    }
    let first = history
        .report()
        .ok_or_else(|| io::Error::other("missing first report"))?;
    let s = first.selection();
    assert_eq!(
        (
            s.start(),
            s.sample_count(),
            s.decision_start(),
            s.decision_count(),
            s.decision_runs(),
            s.fit_runs()
        ),
        (1, 11, 0, 12, 5, 4)
    );
    near(s.statistics().line().phase_seconds(), 0.0);
    near(s.statistics().line().phase_rate_seconds_per_second(), 0.0);
    p.append(&mut history, p.observation(ctx, 12, VALUES[12])?)??;
    let report = history
        .report()
        .ok_or_else(|| io::Error::other("missing second report"))?;
    assert_eq!(
        (
            report.context(),
            report.reference().nanoseconds(),
            report.configured_minimum(),
            report.effective_minimum()
        ),
        (ctx, 12 * SECOND, 3, 3)
    );
    let s = report.selection();
    assert_eq!(
        (
            s.start(),
            s.sample_count(),
            s.decision_start(),
            s.decision_count(),
            s.decision_runs(),
            s.fit_runs(),
            s.stop()
        ),
        (1, 12, 0, 13, 5, 4, FixtureRunsStop::RunsPassed)
    );
    near(s.statistics().line().phase_seconds(), 23.0 / 1248.0);
    near(
        s.statistics().line().phase_rate_seconds_per_second(),
        1.0 / 416.0,
    );
    assert_redacted_debug(&history, report);
    let held = snapshot(&history)?;
    assert_eq!(
        (held.count, held.auxiliary, held.decision.len()),
        (12, 1, 13)
    );
    assert_eq!(held.decision[0].1, 0);
    assert_eq!(held.fitted[0], held.decision[1]);
    assert_eq!(held.fitted[11].1, 12 * SECOND);
    for (index, value) in VALUES.iter().copied().enumerate() {
        assert_eq!(
            held.decision[index],
            (
                ctx,
                u64::try_from(index)? * SECOND,
                i128::from(value) * (1_i128 << 24),
                1.0_f64.to_bits()
            )
        );
    }
    Ok(())
}

/// # Errors
/// Preserve all actual constructor and observation errors.
/// # Panics
/// Reject incorrect minima, warmup behavior, or effective minimum.
#[test]
fn configured_minimum_warms_up_without_claiming_eligibility() -> TestResult {
    let p = Profile::new("configured_minimum_warms_up_without_claiming_eligibility");
    let ctx = context(1)?;
    for (error, expected) in [
        (
            AdaptiveHistoryError::InvalidMinimum,
            "adaptive history minimum",
        ),
        (
            AdaptiveHistoryError::ContextMismatch,
            "adaptive history context mismatch",
        ),
        (
            AdaptiveHistoryError::DuplicateTime,
            "adaptive history duplicate time",
        ),
        (
            AdaptiveHistoryError::OutOfOrderTime,
            "adaptive history out-of-order time",
        ),
        (
            AdaptiveHistoryError::StateInvariant,
            "adaptive history state invariant",
        ),
    ] {
        assert_eq!(error.to_string(), expected);
        assert!(error.source().is_none());
    }
    for minimum in [0, 2, 65, usize::MAX] {
        assert!(matches!(
            p.history(ctx, minimum)?,
            Err(AdaptiveHistoryError::InvalidMinimum)
        ));
    }
    let low = p.history(ctx, 3)??;
    assert_eq!(low.configured_minimum(), 3);
    let mut history = p.history(ctx, 64)??;
    assert_eq!(
        (
            history.context(),
            history.sample_count(),
            history.auxiliary_count(),
            history.reference()
        ),
        (ctx, 0, 0, None)
    );
    assert!(history.report().is_none());
    for index in 0..3_u64 {
        p.append(&mut history, p.observation(ctx, index, 0)?)??;
        assert_eq!(history.sample_count(), usize::try_from(index + 1)?);
        if index < 2 {
            assert!(history.report().is_none());
        }
    }
    let report = history
        .report()
        .ok_or_else(|| io::Error::other("missing warmup report"))?;
    assert_eq!(
        (
            report.configured_minimum(),
            report.effective_minimum(),
            report.selection().sample_count()
        ),
        (64, 3, 3)
    );
    let _snapshot = snapshot(&history)?;
    Ok(())
}

/// Verify a genuine successful fallback without manufacturing a checked report.
///
/// At t=0..8, phase=t²/128 and unit divisors give the independent exact-real
/// newest phase 41/96 and rate 1/16. The terminal residuals have signs ++-----++
/// and magnitudes at least 7/384, so three runs equal the pinned critical value
/// of three at nine rows. The explicit minimum therefore retains all nine rows.
/// At eight rows the neighboring three-run decision exceeds its critical two.
/// # Errors
/// Preserve actual context, observation, history, append and snapshot failures.
/// # Panics
/// Reject a missing fallback, changed fit, lost row or mismatched report context.
#[test]
fn adaptive_minimum_fallback_commits_full_checked_window() -> TestResult {
    let p = Profile::new("adaptive_minimum_fallback_commits_full_checked_window");
    let ctx = context(1)?;
    let mut history = p.history(ctx, 9)??;
    let numerators = [0, 4, 16, 36, 64, 100, 144, 196, 256];
    for (index, numerator) in numerators.iter().copied().enumerate() {
        let second = u64::try_from(index)?;
        p.append(&mut history, p.observation(ctx, second, numerator)?)??;
        assert_eq!(history.sample_count(), index + 1);
        assert_eq!(history.auxiliary_count(), 0);
        assert_eq!(history.reference(), Some(MonotonicNs::new(second * SECOND)));
        if index < 2 {
            assert!(history.report().is_none());
        }
        if index == 7 {
            let report = history
                .report()
                .ok_or_else(|| io::Error::other("missing fallback neighbor report"))?;
            assert_eq!(report.effective_minimum(), 8);
            assert_eq!(report.selection().sample_count(), 8);
            assert_eq!(report.selection().decision_runs(), 3);
            assert_eq!(report.selection().stop(), FixtureRunsStop::RunsPassed);
            near(
                report.selection().statistics().line().phase_seconds(),
                21.0 / 64.0,
            );
            near(
                report
                    .selection()
                    .statistics()
                    .line()
                    .phase_rate_seconds_per_second(),
                7.0 / 128.0,
            );
        }
    }
    let report = history
        .report()
        .ok_or_else(|| io::Error::other("missing minimum fallback report"))?;
    assert_eq!(
        (
            report.context(),
            report.reference(),
            report.configured_minimum(),
            report.effective_minimum(),
        ),
        (ctx, MonotonicNs::new(8 * SECOND), 9, 9)
    );
    let selection = report.selection();
    assert_eq!(
        (
            selection.start(),
            selection.sample_count(),
            selection.decision_start(),
            selection.decision_count(),
            selection.decision_runs(),
            selection.fit_runs(),
            selection.stop(),
        ),
        (0, 9, 0, 9, 3, 3, FixtureRunsStop::MinimumFallback)
    );
    let statistics = selection.statistics();
    assert_eq!(
        (
            selection.frame(),
            statistics.frame(),
            statistics.sample_count(),
            statistics.degrees_of_freedom(),
        ),
        (ctx.frame(), ctx.frame(), 9, 7)
    );
    near(statistics.line().phase_seconds(), 41.0 / 96.0);
    near(
        statistics.line().phase_rate_seconds_per_second(),
        1.0 / 16.0,
    );
    let held = snapshot(&history)?;
    assert_eq!((held.count, held.auxiliary, held.decision.len()), (9, 0, 9));
    assert_eq!(held.fitted, held.decision);
    for (index, numerator) in numerators.iter().copied().enumerate() {
        assert_eq!(
            held.fitted[index],
            (
                ctx,
                u64::try_from(index)? * SECOND,
                i128::from(numerator) * (1_i128 << 24),
                1.0_f64.to_bits(),
            )
        );
    }
    assert_eq!(
        (p.observations.get(), p.appends.get(), p.histories.get()),
        (9, 9, 1)
    );
    Ok(())
}

/// # Errors
/// Preserve the real fixture construction and append errors.
/// # Panics
/// Every rejected append must preserve the complete observable state and recover.
#[test]
fn rejected_appends_preserve_complete_state_and_recover() -> TestResult {
    let p = Profile::new("rejected_appends_preserve_complete_state_and_recover");
    let ctx = context(1)?;
    let mut history = p.history(ctx, 3)??;
    append_trace(&p, &mut history, ctx, 13)?;
    let before = snapshot(&history)?;
    assert!(matches!(
        p.append(&mut history, p.observation(context(2)?, 13, 0)?)?,
        Err(AdaptiveHistoryError::ContextMismatch)
    ));
    assert_eq!(snapshot(&history)?, before);
    let wrong_source = FixtureContext::new(
        NonZeroU64::new(12).ok_or_else(|| io::Error::other("zero source"))?,
        ctx.frame(),
        ctx.generation(),
    );
    assert!(matches!(
        p.append(&mut history, p.observation(wrong_source, 0, 0)?)?,
        Err(AdaptiveHistoryError::ContextMismatch)
    ));
    assert_eq!(snapshot(&history)?, before);
    let wrong_frame = FixtureContext::new(
        ctx.source_id(),
        NonZeroU64::new(18).ok_or_else(|| io::Error::other("zero frame"))?,
        ctx.generation(),
    );
    assert!(matches!(
        p.append(&mut history, p.observation(wrong_frame, 0, 0)?)?,
        Err(AdaptiveHistoryError::ContextMismatch)
    ));
    assert_eq!(snapshot(&history)?, before);
    assert!(matches!(
        p.append(&mut history, p.observation(ctx, 12, 0)?)?,
        Err(AdaptiveHistoryError::DuplicateTime)
    ));
    assert_eq!(snapshot(&history)?, before);
    assert!(matches!(
        p.append(&mut history, p.observation(ctx, 11, 0)?)?,
        Err(AdaptiveHistoryError::OutOfOrderTime)
    ));
    assert_eq!(snapshot(&history)?, before);
    // Auxiliary t=0 exceeds 4096 s; fitted t=1 is exactly on the bound.
    let Err(normalization_error) = p.append(&mut history, p.observation(ctx, 4097, 0)?)? else {
        return Err(io::Error::other("expected normalization failure").into());
    };
    assert_eq!(
        normalization_error.to_string(),
        "adaptive history normalization: normalization elapsed range at 0"
    );
    let normalization_source = normalization_error
        .source()
        .and_then(|cause| cause.downcast_ref::<NormalizationError>())
        .ok_or_else(|| io::Error::other("missing normalization cause"))?;
    assert_eq!(
        *normalization_source,
        NormalizationError::ElapsedRange { index: 0 }
    );
    assert!(normalization_source.source().is_none());
    assert_eq!(snapshot(&history)?, before);
    p.append(&mut history, p.observation(ctx, 13, -64)?)??;
    assert_eq!(
        (
            history.auxiliary_count(),
            history.sample_count(),
            history.reference().map(MonotonicNs::nanoseconds)
        ),
        (1, 13, Some(13 * SECOND))
    );
    Ok(())
}

/// # Errors
/// Preserve actual fixture construction and unexpected append errors.
/// # Panics
/// A suffix failure must retain its typed cause, preserve all state, and recover.
#[test]
fn selector_failure_preserves_cause_and_committed_history() -> TestResult {
    let p = Profile::new("selector_failure_preserves_cause_and_committed_history");
    let ctx = context(1)?;
    let mut history = p.history(ctx, 3)??;
    append_trace(&p, &mut history, ctx, 9)?;
    for offset in [1_u64, 2] {
        p.append(
            &mut history,
            p.observation_at_ns(ctx, 8 * SECOND + offset, -512)?,
        )??;
    }
    let before = snapshot(&history)?;
    let failure = p.append(
        &mut history,
        p.observation_at_ns(ctx, 8 * SECOND + 3, -512)?,
    )?;
    let Err(outer) = failure else {
        return Err(io::Error::other("expected selector failure").into());
    };
    assert_eq!(
        outer.to_string(),
        "adaptive history runs: fixture runs: Suffix"
    );
    assert!(
        outer
            .source()
            .and_then(|cause| cause.downcast_ref::<FixtureRunsError>())
            .is_some()
    );
    let AdaptiveHistoryError::Runs { source } = outer else {
        return Err(io::Error::other("expected wrapped selector cause").into());
    };
    assert_eq!(source.stage(), FixtureRunsStage::Suffix);
    assert_eq!(source.candidate_start(), Some(8));
    assert_eq!(source.point_index(), None);
    assert!(matches!(
        source
            .source()
            .and_then(|cause| cause.downcast_ref::<FitError>()),
        Some(FitError::SpanOutOfRange)
    ));
    assert_eq!(snapshot(&history)?, before);
    p.append(&mut history, p.observation(ctx, 9, 0)?)??;
    assert_eq!(
        history.reference().map(MonotonicNs::nanoseconds),
        Some(9 * SECOND)
    );
    assert_eq!(
        (p.observations.get(), p.appends.get(), p.histories.get()),
        (13, 13, 1)
    );
    Ok(())
}

/// # Errors
/// Preserve checked observations and actual owner errors for the fixed trace.
/// # Panics
/// Reject changed pre-prune indices, line, or compacted public row views.
#[test]
fn adaptive_successful_compaction_preserves_decision_rows() -> TestResult {
    let p = Profile::new("adaptive_successful_compaction_preserves_decision_rows");
    let ctx = context(1)?;
    let mut history = p.history(ctx, 3)??;
    for second in 0..12_u64 {
        let t = i128::from(second);
        let sign = if second % 2 == 0 { 1 } else { -1 };
        let numerator = 512 * t * t + sign;
        let observation =
            p.observation_at_ns_q32(ctx, second * SECOND, numerator * (1_i128 << 16))?;
        p.append(&mut history, observation)??;
        if second == 8 {
            let report = history
                .report()
                .ok_or_else(|| io::Error::other("missing compaction report"))?;
            let selected = report.selection();
            assert_eq!(
                (
                    selected.start(),
                    selected.sample_count(),
                    selected.decision_start(),
                    selected.decision_count(),
                    selected.decision_runs()
                ),
                (5, 4, 1, 8, 3)
            );
            near(
                selected.statistics().line().phase_seconds(),
                161_283.0 / 327_680.0,
            );
            near(
                selected.statistics().line().phase_rate_seconds_per_second(),
                16_641.0 / 163_840.0,
            );
        }
    }
    let held = snapshot(&history)?;
    assert_eq!((held.auxiliary, held.count, held.decision.len()), (4, 4, 8));
    for (index, observed) in held.decision.iter().enumerate() {
        let second = u64::try_from(index)? + 4;
        let t = i128::from(second);
        let sign = if second % 2 == 0 { 1 } else { -1 };
        let numerator = 512 * t * t + sign;
        assert_eq!(
            *observed,
            (
                ctx,
                second * SECOND,
                numerator * (1_i128 << 17),
                1.0_f64.to_bits()
            )
        );
    }
    assert_eq!(held.fitted.as_slice(), &held.decision[4..]);
    let report = held
        .report
        .ok_or_else(|| io::Error::other("missing final compaction report"))?;
    assert_eq!(
        (
            report.start,
            report.count,
            report.decision_start,
            report.decision_count,
            report.decision_runs,
            report.fit_runs
        ),
        (5, 4, 1, 8, 3, 3)
    );
    assert_eq!(
        (p.observations.get(), p.appends.get(), p.histories.get()),
        (12, 12, 1)
    );
    Ok(())
}

/// # Errors
/// Preserve actual construction and append errors across the bounded trace.
/// # Panics
/// A full store must discard the genuinely retained auxiliary row first.
#[test]
fn full_history_evicts_retained_auxiliary_first() -> TestResult {
    let p = Profile::new("full_history_evicts_retained_auxiliary_first");
    let ctx = context(1)?;
    let mut history = p.history(ctx, 3)??;
    append_trace(&p, &mut history, ctx, 13)?;
    for second in 13..64_u64 {
        let value = if second % 2 == 0 { 64 } else { -64 };
        p.append(&mut history, p.observation(ctx, second, value)?)??;
    }
    let full = snapshot(&history)?;
    assert_eq!(
        (full.auxiliary, full.count, full.decision.len()),
        (1, 63, 64)
    );
    assert_eq!((full.decision[0].1, full.fitted[0].1), (0, SECOND));
    p.append(&mut history, p.observation(ctx, 64, 64)?)??;
    let after = snapshot(&history)?;
    assert_eq!(
        (after.auxiliary, after.count, after.decision.len()),
        (0, 64, 64)
    );
    assert_eq!(
        (after.decision[0].1, after.decision[63].1),
        (SECOND, 64 * SECOND)
    );
    let report = after
        .report
        .ok_or_else(|| io::Error::other("missing report"))?;
    assert_eq!(
        (
            report.start,
            report.decision_start,
            report.count,
            report.decision_count,
            report.decision_runs
        ),
        (0, 0, 64, 64, 56)
    );
    Ok(())
}

/// # Errors
/// Preserve actual construction and append errors across the bounded trace.
/// # Panics
/// A full fitted-only store must discard its oldest fitted row.
#[test]
fn full_fitted_history_evicts_oldest_fitted_row() -> TestResult {
    let p = Profile::new("full_fitted_history_evicts_oldest_fitted_row");
    let ctx = context(1)?;
    let mut history = p.history(ctx, 64)??;
    for second in 0..64_u64 {
        p.append(&mut history, p.observation(ctx, second, 0)?)??;
    }
    let full = snapshot(&history)?;
    assert_eq!((full.auxiliary, full.count, full.fitted[0].1), (0, 64, 0));
    p.append(&mut history, p.observation(ctx, 64, 0)?)??;
    let after = snapshot(&history)?;
    assert_eq!(
        (
            after.auxiliary,
            after.count,
            after.fitted[0].1,
            after.fitted[63].1
        ),
        (0, 64, SECOND, 64 * SECOND)
    );
    let report = after
        .report
        .ok_or_else(|| io::Error::other("missing report"))?;
    assert_eq!(
        (
            report.configured,
            report.effective,
            report.count,
            report.decision_count
        ),
        (64, 64, 64, 64)
    );
    Ok(())
}

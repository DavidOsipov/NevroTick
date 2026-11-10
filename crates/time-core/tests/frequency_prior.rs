// SPDX-License-Identifier: GPL-2.0-only
//! Independent frequency lifecycle controls over genuine exchange-derived fits.
//!
//! All rates, clock labels and budgets are explicit fixture assertions. A learned
//! model supplies neither a vote nor an acknowledged active-time base rate. These
//! tests make no physical, native, service, package or release claim.
#![forbid(unsafe_code)]

use std::{error::Error, fmt, io, num::NonZeroU64};

use time_core::{
    ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitError, FixtureAdaptiveHistory,
    FixtureAffineChart, FixtureBaselineCompletion, FixtureBaselineError, FixtureBaselineFeedback,
    FixtureBaselineRetirement, FixtureBaselineSettlement, FixtureChartBatch, FixtureChartDomain,
    FixtureChartFrames, FixtureChartInterval, FixtureChartOutcome, FixtureCombinationParameters,
    FixtureCombinationState, FixtureContext, FixtureControllerClockMap,
    FixtureControllerClockMapInput, FixtureFrequencyAdmission, FixtureFrequencyClockAssociation,
    FixtureFrequencyError, FixtureFrequencyHistory, FixtureFrequencyNumericError,
    FixtureFrequencyNumericField, FixtureFrequencyNumericIssue, FixtureFrequencyNumericStep,
    FixtureFrequencyProfile, FixtureMinimum, FixtureObservation, FixturePhaseBaseline,
    FixturePhaseError, FixturePhaseNow, FixturePhasePlanOwner, FixturePhaseProfile,
    FixturePhaseReferenceAdmission, FixturePhaseResidualPermission, FixturePhaseRetirement,
    FixturePhaseRetirementReason, FixturePhaseSettlement, FixturePrincipalParameters,
    FixturePrincipalStratum, FixtureProjectedBatch, FixtureProjectionTerms, FixtureRateClock,
    FixtureRateClockContext, FixtureRateCursorStatus, FixtureRateSettlement,
    FixtureRateSettlementCursor, FixtureRateSpan, FixtureReferenceInputError,
    FixtureSettledRateInterval, enclose_fixture_chart_projection, enclose_fixture_projection,
};
use time_domain::{ActiveAwakeNs, MonotonicNs, UtcCoordinateQ32, exchange_arithmetic};

/// Genuine fixture setup and checked product errors remain explicit.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Integer nanoseconds in the independently selected second.
const SECOND: u64 = 1_000_000_000;
/// Initial exact phase one thirty-second, expressed in Q32.
const INITIAL_PHASE: i128 = 1 << 27;
/// Nonconstant symmetric residual one sixty-fourth, expressed in Q32.
const RESIDUAL: i128 = 1 << 26;
/// Exact fitted slope one five-hundred-and-twelfth, expressed in Q32.
const FRESH_RATE: i128 = 1 << 23;

/// Require a known-valid pure product call before extracting its typed result.
/// # Errors
/// Preserve the actual bounded error rather than substituting a fixture value.
/// # Panics
/// Reject an unexpected refusal of an independently valid literal trajectory.
#[track_caller]
fn product_ok<T, E: fmt::Debug + fmt::Display>(value: Result<T, E>) -> TestResult<T> {
    assert!(
        value.is_ok(),
        "the independent frequency fixture must succeed; bounded refusal: {:?}",
        value.as_ref().err().map(ToString::to_string)
    );
    value.map_err(|error| io::Error::other(format!("frequency product: {error:?}")).into())
}

/// Admit a fixed nonzero label without silently replacing zero.
/// # Errors
/// Reject an invalid fixture label before calling the product.
fn label(value: u64) -> TestResult<NonZeroU64> {
    NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero frequency fixture label").into())
}

/// Keep active and monotonic fixture origins distinct.
/// # Errors
/// Preserve the fixed label checks.
fn clocks() -> TestResult<FixtureRateClockContext> {
    Ok(FixtureRateClockContext::new(
        label(101)?,
        label(103)?,
        label(107)?,
    ))
}

/// Declare the explicit original-source to monotonic fixture association.
/// # Errors
/// Preserve the fixed label checks; no native relationship is inferred.
fn association() -> TestResult<FixtureFrequencyClockAssociation> {
    Ok(FixtureFrequencyClockAssociation::new(
        label(17)?,
        label(23)?,
        label(101)?,
        label(107)?,
    ))
}

/// Supply independently selected readings, with no active-to-monotonic conversion.
/// # Errors
/// Reject a fixture coordinate outside the finite sixty-four-second table.
fn at(active: u64, monotonic: u64) -> TestResult<FixturePhaseNow> {
    if active > 64 || monotonic > 64 {
        return Err(io::Error::other("frequency fixture time outside table").into());
    }
    Ok(FixturePhaseNow::new(
        clocks()?,
        ActiveAwakeNs::new(active * SECOND),
        MonotonicNs::new(monotonic * SECOND),
    ))
}

/// Build one genuine immutable affine chart with distinct source/target tags.
/// # Errors
/// Preserve mandatory descriptor admission.
fn chart() -> TestResult<FixtureAffineChart> {
    product_ok(FixtureAffineChart::new(
        FixtureChartFrames::new(label(17)?, label(23)?, label(19)?, label(29)?)?,
        FixtureChartDomain::new(
            MonotonicNs::new(0),
            MonotonicNs::new(2 * SECOND),
            MonotonicNs::new(64 * SECOND),
        )?,
        0.0,
        0.0,
    ))
}

/// Original immutable observations and the actual adaptive fitted history.
struct Source {
    /// Actual source/frame/generation, independent of target chart tags.
    context: FixtureContext,
    /// Three checked exchange-derived observations; no combined record is forged.
    history: FixtureAdaptiveHistory,
}

impl Source {
    /// Fit `P+f*(t-last)+[c,-2c,c]` at three equally spaced exact instants.
    ///
    /// Independently, the final mean is P, slope f, SSE is 6c², fitted slope
    /// variance is 3c² and fitted phase variance at the last instant is 5c².
    /// # Errors
    /// Preserve coordinate/exchange/history admission; reject unbounded fixtures.
    fn new(first: u64, phase: i128, rate: i128, residual: i128) -> TestResult<Self> {
        if first > 12
            || !(-(1_i128 << 30)..=(1_i128 << 30)).contains(&phase)
            || !(-(1_i128 << 27)..=(1_i128 << 27)).contains(&rate)
            || !(1..=(1_i128 << 27)).contains(&residual)
        {
            return Err(io::Error::other("frequency source outside finite table").into());
        }
        let context = FixtureContext::new(label(11)?, label(17)?, label(23)?);
        let mut history = product_ok(FixtureAdaptiveHistory::new(context, 3))?;
        for (index, noise) in [residual, -2 * residual, residual].into_iter().enumerate() {
            let position = i128::try_from(index)? - 2;
            let coordinate = product_ok(UtcCoordinateQ32::new(phase + rate * position + noise))?;
            let zero = product_ok(UtcCoordinateQ32::new(0))?;
            let observed = product_ok(exchange_arithmetic(zero, coordinate, coordinate, zero))?;
            let second = first + u64::try_from(index)?;
            product_ok(history.try_append(product_ok(FixtureObservation::new(
                context,
                MonotonicNs::new(second * SECOND),
                observed.offset(),
                1.0,
            ))?))?;
        }
        Ok(Self { context, history })
    }

    /// Project the actual fitted tracking attachment through this exact chart.
    /// # Errors
    /// Preserve real projection/enclosure failures and absent-history setup errors.
    fn entry<'chart>(
        &self,
        chart: &'chart FixtureAffineChart,
        second: u64,
        skew: f64,
    ) -> TestResult<FixtureChartInterval<'chart>> {
        let view = product_ok(
            self.history
                .prepare_chart(self.context, FixtureChartOutcome::Known(chart)),
        )?
        .ok_or_else(|| io::Error::other("missing genuine frequency chart view"))?;
        let reference = self
            .history
            .reference()
            .ok_or_else(|| io::Error::other("missing genuine frequency fit reference"))?;
        let projected = product_ok(view.project_tracking(
            view.context(),
            2,
            reference,
            MonotonicNs::new(second * SECOND),
            product_ok(FixtureProjectionTerms::new(0.25, 0.0, 0.125))?,
            skew,
        ))?;
        product_ok(enclose_fixture_chart_projection(&projected))
    }
}

/// One explicit fixture source; this selects no production quorum/default.
/// # Errors
/// Preserve the fixed source-label admission.
fn strata() -> TestResult<[FixturePrincipalStratum; 1]> {
    Ok([product_ok(FixturePrincipalStratum::new(label(11)?, 2))?])
}

/// Seed the real producer from its actual original projection enclosure.
/// # Errors
/// Preserve checked minimum, parameters, batch and producer admission.
fn seed(entry: &FixtureChartInterval<'_>, second: u64) -> TestResult<FixtureCombinationState> {
    let original = [product_ok(enclose_fixture_projection(
        entry.projection().source_projection(),
    ))?];
    let batch = product_ok(FixtureProjectedBatch::new(
        label(17)?,
        label(23)?,
        MonotonicNs::new(second * SECOND),
        &original,
    ))?;
    product_ok(FixtureCombinationState::seed(
        &[label(11)?],
        product_ok(FixturePrincipalParameters::new(1.0 / 1024.0, 0.0))?,
        product_ok(FixtureCombinationParameters::new(2.0, 0.0))?,
        product_ok(FixtureMinimum::new(1))?,
        &batch,
        &strata()?,
    ))
}

/// Select only the previously qualified finite phase fixture parameters.
/// # Errors
/// Preserve the distinct exact-duration and private profile guards.
fn phase_profile() -> TestResult<FixturePhaseProfile> {
    product_ok(FixturePhaseProfile::new(
        ActiveAwakeNs::new(8 * SECOND).duration_since(ActiveAwakeNs::new(0))?,
        MonotonicNs::new(16 * SECOND).duration_since(MonotonicNs::new(0))?,
        1.0 / 16.0,
        1.0 / 128.0,
        1.0 / 64.0,
    ))
}

/// Explicit conditional skew/growth/age fixture, never a shipping default.
/// # Errors
/// Preserve exact duration and mandatory parameter admission.
fn frequency_profile(
    max_skew: f64,
    clock_error: f64,
    growth: f64,
    age: u64,
) -> TestResult<FixtureFrequencyProfile> {
    product_ok(FixtureFrequencyProfile::new(
        max_skew,
        clock_error,
        growth,
        MonotonicNs::new(age * SECOND).duration_since(MonotonicNs::new(0))?,
    ))
}

/// Lease the actual producer and enable mandatory frequency intake.
/// # Errors
/// Preserve all checked constructor failures; no prior is attached afterward.
fn enabled<'producer, 'chart>(
    producer: &'producer mut FixtureCombinationState,
    chart: &'chart FixtureAffineChart,
    profile: FixtureFrequencyProfile,
) -> TestResult<FixturePhasePlanOwner<'producer, 'chart>> {
    product_ok(FixturePhasePlanOwner::new_with_frequency(
        producer,
        chart,
        FixtureRateSettlementCursor::new(chart, clocks()?, label(1)?),
        at(0, 4)?,
        phase_profile()?,
        product_ok(FixturePhaseBaseline::new(1.0 / 1024.0))?,
        profile,
        association()?,
    ))
}

/// Admit one actual batch and preserve the returned phase branch separately.
/// # Errors
/// Preserve the genuine batch/receiver error rather than forging an outcome.
fn receive(
    owner: &mut FixturePhasePlanOwner<'_, '_>,
    source: &Source,
    second: u64,
    skew: f64,
    readings: FixturePhaseNow,
) -> TestResult<FixturePhaseReferenceAdmission> {
    let entries = [source.entry(owner.chart(), second, skew)?];
    let batch = product_ok(FixtureChartBatch::new(
        owner.chart(),
        MonotonicNs::new(second * SECOND),
        &entries,
    ))?;
    product_ok(owner.receive_reference(&batch, &strata()?, readings))
}

/// Complete observational model snapshot; copying it grants no intake authority.
#[derive(Debug, Eq, PartialEq)]
struct HistorySnapshot {
    /// Stored mean point and deterministic error bits.
    mean: [u64; 2],
    /// Conditional skew point and its separate arithmetic error bits.
    skew: [u64; 2],
    /// Residual point/error, distinct from a learned active-time base.
    residual: [u64; 2],
    /// Genuine phase anchor and inherited/current arithmetic error bits.
    anchor: [u64; 2],
    /// Original fit phase SD and rate SD bits.
    statistics: [u64; 2],
    /// Actual source emission ordinal, never a caller-created credit token.
    round: NonZeroU64,
    /// Original genuine source identity/frame/generation.
    source: FixtureContext,
    /// Actual mathematical chart association of this model.
    frames: FixtureChartFrames,
    /// Original anchor/latest/fit/projected instants, in that fixed order.
    times: [u64; 4],
}

/// Read a complete public historical record without constructing product state.
fn snapshot(history: &FixtureFrequencyHistory) -> HistorySnapshot {
    HistorySnapshot {
        mean: [
            history.mean_seconds_per_second().to_bits(),
            history.mean_roundoff_seconds_per_second().to_bits(),
        ],
        skew: [
            history.skew_seconds_per_second().to_bits(),
            history.skew_roundoff_seconds_per_second().to_bits(),
        ],
        residual: [
            history.residual_seconds_per_second().to_bits(),
            history.residual_roundoff_seconds_per_second().to_bits(),
        ],
        anchor: [
            history.anchor_phase_seconds().to_bits(),
            history.anchor_roundoff_seconds().to_bits(),
        ],
        statistics: [
            history.phase_standard_deviation_seconds().to_bits(),
            history
                .rate_standard_deviation_seconds_per_second()
                .to_bits(),
        ],
        round: history.emitted_round(),
        source: history.source_context(),
        frames: history.frames(),
        times: [
            history.anchor_observed_at().nanoseconds(),
            history.latest_observed_at().nanoseconds(),
            history.fit_reference().nanoseconds(),
            history.projected_at().nanoseconds(),
        ],
    }
}

/// Check fit statistics against independent three-row residual geometry.
///
/// At two seconds after the final fit reference, phase SD is
/// `(sqrt(5)+2*sqrt(3))*c`, while rate SD is `sqrt(3)*c`. The dyadic bounds
/// below enclose those radicals independently and distinguish SD from skew.
/// # Panics
/// Reject a copied wrong statistic, hidden NaN or numerical/physical conflation.
fn check_fit_statistics(history: &FixtureFrequencyHistory, residual: f64) {
    let rate_sd = history.rate_standard_deviation_seconds_per_second();
    assert!(rate_sd > (7094.0 / 4096.0) * residual);
    assert!(rate_sd < (7095.0 / 4096.0) * residual);
    let phase_sd = history.phase_standard_deviation_seconds();
    assert!(phase_sd > (23346.0 / 4096.0) * residual);
    assert!(phase_sd < (23349.0 / 4096.0) * residual);
}

/// Build one genuine modeled completed-rate input, not a completion/report factory.
/// # Errors
/// Preserve exact interval, context, descriptor and rate admission.
fn interval(
    chart: &FixtureAffineChart,
    ordinal: u64,
    active: (u64, u64),
    monotonic: (u64, u64),
    rate: f64,
) -> TestResult<FixtureSettledRateInterval> {
    let frames = chart.frames();
    product_ok(FixtureSettledRateInterval::new(
        clocks()?,
        label(ordinal)?,
        product_ok(FixtureRateSpan::new(
            (
                ActiveAwakeNs::new(active.0 * SECOND),
                ActiveAwakeNs::new(active.1 * SECOND),
            ),
            (
                MonotonicNs::new(monotonic.0 * SECOND),
                MonotonicNs::new(monotonic.1 * SECOND),
            ),
        ))?,
        rate,
        product_ok(FixtureChartFrames::new(
            frames.target_frame(),
            frames.target_generation(),
            label(31)?,
            label(7)?,
        ))?,
        chart.domain(),
    ))
}

/// Recover only the actual known receiving branch after its primary assertion.
/// # Errors
/// Reject a different ownership branch without manufacturing a retirement.
/// # Panics
/// Reject failure to commit an independently valid known historical interval.
fn known<'producer>(
    owner: FixturePhasePlanOwner<'producer, '_>,
    event: FixtureRateSettlement,
    readings: FixturePhaseNow,
) -> TestResult<FixturePhaseRetirement<'producer>> {
    let result = owner.settle(
        event,
        Some(product_ok(FixturePhaseBaseline::new(1.0 / 1024.0))?),
        readings,
    );
    assert!(matches!(&result, FixturePhaseSettlement::Known(_)));
    match result {
        FixturePhaseSettlement::Known(retired) => Ok(retired),
        _ => Err(io::Error::other("missing actual known frequency retirement").into()),
    }
}

/// Require an observable genuine historical record before inspecting it.
/// # Errors
/// Preserve the absent-record observation after its primary semantic assertion.
/// # Panics
/// Reject a model lost after an independently admitted actual reference.
fn history(owner: &FixturePhasePlanOwner<'_, '_>) -> TestResult<FixtureFrequencyHistory> {
    let value = owner.frequency_history();
    assert!(
        value.is_some(),
        "the actual admitted frequency history must remain observable"
    );
    value.ok_or_else(|| io::Error::other("missing admitted frequency history").into())
}

/// Require both independent admission channels to name the actual emitted round.
/// # Panics
/// Reject a missing phase/frequency admission or a manufactured ordinal.
fn accepted(
    owner: &FixturePhasePlanOwner<'_, '_>,
    phase: &FixturePhaseReferenceAdmission,
    round: u64,
) {
    assert!(
        matches!(phase, FixturePhaseReferenceAdmission::Accepted { emitted_round } if emitted_round.get() == round)
    );
    assert!(
        matches!(owner.frequency_admission(), FixtureFrequencyAdmission::Accepted { emitted_round } if emitted_round.get() == round)
    );
}

/// One public value per refusal family; these are diagnostic fixtures only.
fn frequency_diagnostic_families() -> [FixtureFrequencyError; 18] {
    [
        FixtureFrequencyError::InvalidProfile,
        FixtureFrequencyError::Disabled,
        FixtureFrequencyError::ClockContextMismatch,
        FixtureFrequencyError::ClockAssociationMismatch,
        FixtureFrequencyError::TimeRegression {
            clock: FixtureRateClock::ActiveAwake,
        },
        FixtureFrequencyError::CursorUnavailable,
        FixtureFrequencyError::QueryBeforeReference,
        FixtureFrequencyError::QueryOutsideDomain,
        FixtureFrequencyError::NoModel,
        FixtureFrequencyError::Expired,
        FixtureFrequencyError::OutstandingOperation,
        FixtureFrequencyError::Stopped,
        FixtureFrequencyError::Generation,
        FixtureFrequencyError::Unknown,
        FixtureFrequencyError::MeasuredSkew,
        FixtureFrequencyError::AnchorRebaseUnavailable,
        FixtureFrequencyError::Numeric {
            source: FixtureFrequencyNumericError::IndeterminateWeights,
        },
        FixtureFrequencyError::ValidationInvariant,
    ]
}

/// Preserve the exact typed cause while keeping payload details out of text.
/// # Panics
/// Reject a lost, copied, mistyped or invented source in a public refusal.
fn check_frequency_error_source(error: &FixtureFrequencyError) {
    match error {
        FixtureFrequencyError::Numeric { source } => assert!(error.source().is_some_and(|cause| {
            cause
                .downcast_ref::<FixtureFrequencyNumericError>()
                .is_some_and(|actual| std::ptr::eq(actual, source))
        })),
        _ => assert!(error.source().is_none()),
    }
}

/// Check useful refusal families and exact explicit limits without English tables.
/// # Errors
/// Preserve checked literal profile and nonzero association admission failures.
/// # Panics
/// Reject unbounded or merged families, leaked nested roles or wrong readouts.
#[test]
fn frequency_diagnostics_and_declared_limits_keep_bounded_private_readouts() -> TestResult {
    let mut families = std::collections::BTreeSet::new();
    for error in frequency_diagnostic_families() {
        let display = error.to_string();
        assert!(display.starts_with("fixture frequency: "));
        assert!(display.is_ascii() && display.len() <= 96);
        assert!(families.insert(display));
        assert_eq!(format!("{error:?}"), "FixtureFrequencyError { .. }");
        check_frequency_error_source(&error);
    }
    assert_eq!(families.len(), 18);
    for error in [
        FixtureFrequencyError::TimeRegression {
            clock: FixtureRateClock::Monotonic,
        },
        FixtureFrequencyError::Numeric {
            source: FixtureFrequencyNumericError::Input {
                field: FixtureFrequencyNumericField::ResidualError,
                issue: FixtureFrequencyNumericIssue::Negative,
            },
        },
    ] {
        let display = error.to_string();
        assert!(families.contains(&display));
        assert!(!display.contains("Monotonic"));
        assert!(!display.contains("ResidualError"));
        assert!(!display.contains("Negative"));
        assert_eq!(format!("{error:?}"), "FixtureFrequencyError { .. }");
        check_frequency_error_source(&error);
    }
    let profile = frequency_profile(1.0 / 1024.0, 1.0 / 2048.0, 1.0 / 8.0, 32)?;
    assert_eq!(profile.maximum_measured_skew(), 1.0 / 1024.0);
    assert_eq!(profile.maximum_clock_error_rate(), 1.0 / 2048.0);
    assert_eq!(profile.growth_cap(), 1.0 / 8.0);
    assert_eq!(profile.maximum_age().nanoseconds(), 32 * SECOND);
    assert_eq!(format!("{profile:?}"), "FixtureFrequencyProfile { .. }");
    assert_eq!(
        format!("{:?}", association()?),
        "FixtureFrequencyClockAssociation { .. }"
    );

    // Direct public profile admission is independent of the diagnostic checks.
    // The largest positive subnormal and MIN_POSITIVE are adjacent f64 values.
    let age = MonotonicNs::new(32 * SECOND).duration_since(MonotonicNs::new(0))?;
    let max_subnormal = f64::from_bits(0x000f_ffff_ffff_ffff);
    for rejected in [
        -f64::MIN_POSITIVE,
        f64::from_bits(1),
        max_subnormal,
        -f64::from_bits(1),
        -max_subnormal,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
    ] {
        assert_eq!(
            FixtureFrequencyProfile::new(rejected, 0.0, 1.0 / 8.0, age).err(),
            Some(FixtureFrequencyError::InvalidProfile)
        );
        assert_eq!(
            FixtureFrequencyProfile::new(0.0, rejected, 1.0 / 8.0, age).err(),
            Some(FixtureFrequencyError::InvalidProfile)
        );
    }
    for admitted in [0.0_f64, -0.0, f64::MIN_POSITIVE, f64::MAX] {
        let skew = frequency_profile(admitted, 0.0, 1.0 / 8.0, 32)?;
        assert_eq!(skew.maximum_measured_skew().to_bits(), admitted.to_bits());
        assert_eq!(skew.maximum_clock_error_rate().to_bits(), 0.0_f64.to_bits());
        let clock_error = frequency_profile(0.0, admitted, 1.0 / 8.0, 32)?;
        assert_eq!(
            clock_error.maximum_measured_skew().to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(
            clock_error.maximum_clock_error_rate().to_bits(),
            admitted.to_bits()
        );
    }
    Ok(())
}

/// Check original evidence times independently of current intake/projection time.
/// # Panics
/// Reject an age reset, relabelled source, wrong chart or missing mean error.
fn check_original(history: &FixtureFrequencyHistory, first: u64, projected: u64, round: u64) {
    assert_eq!(history.emitted_round().get(), round);
    assert_eq!(history.source_context().source_id().get(), 11);
    assert_eq!(history.source_context().frame().get(), 17);
    assert_eq!(history.source_context().generation().get(), 23);
    assert_eq!(
        history.anchor_observed_at(),
        MonotonicNs::new((first + 2) * SECOND)
    );
    assert_eq!(
        history.latest_observed_at(),
        MonotonicNs::new((first + 2) * SECOND)
    );
    assert_eq!(
        history.fit_reference(),
        MonotonicNs::new((first + 2) * SECOND)
    );
    assert_eq!(history.projected_at(), MonotonicNs::new(projected * SECOND));
    assert!(history.mean_roundoff_seconds_per_second().is_finite());
    assert!(history.mean_roundoff_seconds_per_second() >= 0.0);
    assert!(history.anchor_roundoff_seconds().is_finite());
    assert!(history.anchor_roundoff_seconds() >= 0.0);
}

/// Real original evidence initializes once; later evaluation grants no credit.
/// # Errors
/// Preserve literal setup and genuine receiving/projection failures.
/// # Panics
/// Reject wrong literal facts, observation-age reset or replayed model learning.
#[test]
fn actual_intake_replay_and_monotonic_prediction_preserve_original_evidence() -> TestResult {
    let source = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
    let chart = chart()?;
    let entry = source.entry(&chart, 4, 1.0 / 1024.0)?;
    let mut producer = seed(&entry, 4)?;
    let mut owner = enabled(
        &mut producer,
        &chart,
        frequency_profile(1.0 / 1024.0, 0.0, 0.125, 32)?,
    )?;
    assert!(owner.frequency_history().is_none());
    let phase = receive(&mut owner, &source, 4, 1.0 / 1024.0, at(0, 4)?)?;
    accepted(&owner, &phase, 1);
    let first = product_ok(owner.project_model(at(0, 4)?))?;
    assert_eq!(first.nominal_phase_seconds(), 1.0 / 32.0);
    assert_eq!(first.added_growth_seconds(), 0.0);
    assert_eq!(first.evaluated_at(), MonotonicNs::new(4 * SECOND));
    assert!(first.phase_roundoff_seconds().is_finite());
    assert!(first.phase_roundoff_seconds() >= first.history().anchor_roundoff_seconds());
    assert!(first.history().anchor_roundoff_seconds() > 0.0);
    assert_eq!(first.evidence_age().nanoseconds(), 2 * SECOND);
    assert_eq!(format!("{first:?}"), "FixtureFrequencyProjection { .. }");
    assert_eq!(
        format!("{:?}", first.history()),
        "FixtureFrequencyHistory { .. }"
    );
    assert_eq!(first.history().reference_frames(), chart.frames());
    assert_eq!(first.history().frames(), chart.frames());
    assert_eq!(first.history().domain().start(), MonotonicNs::new(0));
    assert_eq!(
        first.history().domain().common_reference(),
        MonotonicNs::new(2 * SECOND)
    );
    assert_eq!(
        first.history().domain().end(),
        MonotonicNs::new(64 * SECOND)
    );
    assert_eq!(
        format!("{:?}", owner.frequency_admission()),
        "FixtureFrequencyAdmission { .. }"
    );
    check_original(first.history(), 0, 4, 1);
    check_fit_statistics(first.history(), 1.0 / 64.0);
    assert_eq!(first.history().mean_seconds_per_second(), 0.0);
    assert_eq!(first.history().skew_seconds_per_second(), 1.0 / 1024.0);
    let original = snapshot(first.history());
    let later = product_ok(owner.project_model(at(0, 6)?))?;
    assert_eq!(later.nominal_phase_seconds(), 1.0 / 32.0);
    assert_eq!(later.added_growth_seconds(), 1.0 / 512.0);
    assert_eq!(later.evidence_age().nanoseconds(), 4 * SECOND);
    assert_eq!(snapshot(later.history()), original);
    assert_eq!(owner.current_time().active(), ActiveAwakeNs::new(0));
    let replay = receive(&mut owner, &source, 6, 1.0 / 1024.0, at(0, 6)?)?;
    assert!(matches!(
        replay,
        FixturePhaseReferenceAdmission::AdmittedNoPlan {
            emitted_round: None,
            ..
        }
    ));
    assert!(matches!(
        owner.frequency_admission(),
        FixtureFrequencyAdmission::NoReady {
            emitted_round: None
        }
    ));
    assert_eq!(snapshot(&history(&owner)?), original);
    assert_eq!(owner.cursor().next_ordinal(), Some(label(1)?));
    assert!(owner.proposal().is_none());
    Ok(())
}

/// New real rows replace the full record and exercise the reviewed weighted law.
/// # Errors
/// Preserve actual source, owner and current-time admission.
/// # Panics
/// Reject stale anchor/statistics, wrong weights or collapsed growth terms.
#[test]
fn fresh_singleton_updates_the_whole_record_and_real_growth_interval() -> TestResult {
    let old = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
    // At fit reference6: phase3/256; adding two seconds of rate1/512 gives1/64.
    let fresh = Source::new(4, 3_i128 << 24, FRESH_RATE, RESIDUAL / 2)?;
    let chart = chart()?;
    let mut producer = seed(&old.entry(&chart, 4, 1.0 / 1024.0)?, 4)?;
    let mut owner = enabled(
        &mut producer,
        &chart,
        frequency_profile(1.0 / 1024.0, 1.0 / 1024.0, 0.125, 32)?,
    )?;
    let first = receive(&mut owner, &old, 4, 1.0 / 1024.0, at(0, 4)?)?;
    accepted(&owner, &first, 1);
    let original_statistics = snapshot(&history(&owner)?).statistics;
    let next = receive(&mut owner, &fresh, 8, 1.0 / 1024.0, at(0, 8)?)?;
    accepted(&owner, &next, 2);
    let updated = product_ok(owner.project_model(at(0, 8)?))?;
    let model = updated.history();
    assert_eq!(model.anchor_phase_seconds(), 1.0 / 64.0);
    assert_eq!(model.mean_seconds_per_second(), 3.0 / 2048.0);
    assert_eq!(model.residual_seconds_per_second(), 1.0 / 2048.0);
    check_original(model, 4, 8, 2);
    check_fit_statistics(model, 1.0 / 128.0);
    assert_ne!(snapshot(model).statistics, original_statistics);
    let sqrt3_low = 7094.0 / 4096.0;
    let sqrt3_high = 7095.0 / 4096.0;
    assert!(model.skew_seconds_per_second() > (2.0 + sqrt3_low) / 2048.0);
    assert!(model.skew_seconds_per_second() < (2.0 + sqrt3_high) / 2048.0);
    let aged = product_ok(owner.project_model(at(0, 16)?))?;
    assert_eq!(aged.nominal_phase_seconds(), 7.0 / 256.0);
    // This is the actual C02 model, not the unrelated standalone S=1/1024 case.
    assert!(aged.added_growth_seconds() > (5.0 + sqrt3_low) / 256.0);
    assert!(aged.added_growth_upper_seconds() < (5.0 + sqrt3_high) / 256.0);
    assert!(aged.added_growth_upper_seconds() >= aged.added_growth_seconds());
    assert_eq!(aged.evidence_age().nanoseconds(), 10 * SECOND);
    Ok(())
}

/// Frequency and phase have independent post-emission admission/retention.
/// # Errors
/// Preserve actual ready references and finite issuance failures.
/// # Panics
/// Reject false rollback, lost old-model facts or baseline mutation by learning.
#[test]
fn independent_phase_and_frequency_commits_preserve_their_actual_records() -> TestResult {
    let outside = Source::new(0, 1_i128 << 29, 0, RESIDUAL)?;
    let phase_only = Source::new(4, INITIAL_PHASE, 0, RESIDUAL)?;
    let chart = chart()?;
    let mut producer = seed(&outside.entry(&chart, 4, 1.0 / 1024.0)?, 4)?;
    let mut owner = enabled(
        &mut producer,
        &chart,
        frequency_profile(1.0 / 1024.0, 0.0, 0.125, 32)?,
    )?;
    let first = receive(&mut owner, &outside, 4, 1.0 / 1024.0, at(0, 4)?)?;
    assert!(
        matches!(first, FixturePhaseReferenceAdmission::AdmittedNoPlan { emitted_round: Some(round), reason: FixturePhaseError::PhaseEnvelope } if round.get() == 1)
    );
    assert!(
        matches!(owner.frequency_admission(), FixtureFrequencyAdmission::Accepted { emitted_round } if emitted_round.get() == 1)
    );
    assert_eq!(owner.remaining_phase_seconds(), None);
    let original = snapshot(&history(&owner)?);
    let next = receive(&mut owner, &phase_only, 8, 1.0 / 512.0, at(0, 8)?)?;
    assert!(
        matches!(next, FixturePhaseReferenceAdmission::Accepted { emitted_round } if emitted_round.get() == 2)
    );
    assert!(
        matches!(owner.frequency_admission(), FixtureFrequencyAdmission::Refused { emitted_round, reason: FixtureFrequencyError::MeasuredSkew } if emitted_round.get() == 2)
    );
    assert_eq!(snapshot(&history(&owner)?), original);
    assert_eq!(owner.remaining_phase_seconds(), Some(1.0 / 32.0));
    assert_eq!(
        product_ok(owner.project_model(at(0, 8)?))?.nominal_phase_seconds(),
        1.0 / 8.0
    );
    let issued = product_ok(owner.issue(at(0, 8)?, FixturePhaseResidualPermission::Deny))?;
    assert_eq!(issued.aggregate_rate(), 5.0 / 1024.0);
    Ok(())
}

/// A zero-gain successful update refreshes real metadata; both-zero refuses.
/// # Errors
/// Preserve genuine source/consumer failures for both explicit fixture rows.
/// # Panics
/// Reject conflation of a successful zero-gain observation with no assimilation.
#[test]
fn gain_zero_success_and_both_zero_refusal_have_distinct_whole_records() -> TestResult {
    for measured_skew in [0.0, 1.0 / 1024.0] {
        let old = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
        let fresh = Source::new(4, 3_i128 << 24, FRESH_RATE, RESIDUAL / 2)?;
        let chart = chart()?;
        let mut producer = seed(&old.entry(&chart, 4, 0.0)?, 4)?;
        let mut owner = enabled(
            &mut producer,
            &chart,
            frequency_profile(1.0 / 1024.0, 0.0, 0.125, 32)?,
        )?;
        let initial = receive(&mut owner, &old, 4, 0.0, at(0, 4)?)?;
        accepted(&owner, &initial, 1);
        let original = snapshot(&history(&owner)?);
        let next = receive(&mut owner, &fresh, 8, measured_skew, at(0, 8)?)?;
        assert!(
            matches!(next, FixturePhaseReferenceAdmission::Accepted { emitted_round } if emitted_round.get() == 2)
        );
        if measured_skew == 0.0 {
            assert!(
                matches!(owner.frequency_admission(), FixtureFrequencyAdmission::Refused { emitted_round, reason: FixtureFrequencyError::Numeric { source: FixtureFrequencyNumericError::IndeterminateWeights } } if emitted_round.get() == 2)
            );
            assert_eq!(snapshot(&history(&owner)?), original);
        } else {
            assert!(
                matches!(owner.frequency_admission(), FixtureFrequencyAdmission::Accepted { emitted_round } if emitted_round.get() == 2)
            );
            let projection = product_ok(owner.project_model(at(0, 10)?))?;
            assert_eq!(projection.nominal_phase_seconds(), 1.0 / 64.0);
            assert_eq!(projection.history().mean_seconds_per_second(), 0.0);
            assert_eq!(projection.history().skew_seconds_per_second(), 0.0);
            assert_eq!(
                projection.history().residual_seconds_per_second(),
                1.0 / 512.0
            );
            check_original(projection.history(), 4, 8, 2);
            check_fit_statistics(projection.history(), 1.0 / 128.0);
            assert_ne!(snapshot(projection.history()).anchor, original.anchor);
        }
    }
    Ok(())
}

/// A gain-one actual observation supplies its current slope and a fresh anchor.
/// # Errors
/// Preserve genuine producer and checked model operations.
/// # Panics
/// Reject a retained old mean or old anchor in the gain-one neighbor.
#[test]
fn gain_one_uses_the_genuine_current_rate_without_changing_active_baseline() -> TestResult {
    let old = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
    let fresh = Source::new(4, 3_i128 << 24, FRESH_RATE, RESIDUAL / 2)?;
    let chart = chart()?;
    let mut producer = seed(&old.entry(&chart, 4, 1.0 / 1024.0)?, 4)?;
    let mut owner = enabled(
        &mut producer,
        &chart,
        frequency_profile(1.0 / 1024.0, 0.0, 0.125, 32)?,
    )?;
    let first = receive(&mut owner, &old, 4, 1.0 / 1024.0, at(0, 4)?)?;
    accepted(&owner, &first, 1);
    let next = receive(&mut owner, &fresh, 8, 0.0, at(0, 8)?)?;
    accepted(&owner, &next, 2);
    let projection = product_ok(owner.project_model(at(0, 10)?))?;
    assert_eq!(projection.nominal_phase_seconds(), 5.0 / 256.0);
    assert_eq!(projection.history().mean_seconds_per_second(), 1.0 / 512.0);
    assert_eq!(projection.history().skew_seconds_per_second(), 0.0);
    assert_eq!(projection.history().residual_seconds_per_second(), 0.0);
    assert_eq!(projection.added_growth_seconds(), 0.0);
    let issued = product_ok(owner.issue(at(0, 10)?, FixturePhaseResidualPermission::Deny))?;
    // The phase is1/64 and its active allowance is still8: q1/512 plus b1/1024.
    assert_eq!(issued.aggregate_rate(), 3.0 / 1024.0);
    Ok(())
}

/// Disabled frequency cannot advance current time or enable itself after intake.
/// # Errors
/// Preserve the original finite-only construction and actual source admission.
/// # Panics
/// Reject implicit learning, time mutation or changed finite baseline behavior.
#[test]
fn finite_only_constructor_stays_disabled_without_projection_time_effects() -> TestResult {
    let source = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
    let chart = chart()?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 1024.0)?, 4)?;
    let mut owner = product_ok(FixturePhasePlanOwner::new(
        &mut producer,
        &chart,
        FixtureRateSettlementCursor::new(&chart, clocks()?, label(1)?),
        at(0, 4)?,
        phase_profile()?,
        product_ok(FixturePhaseBaseline::new(1.0 / 1024.0))?,
    ))?;
    let phase = receive(&mut owner, &source, 4, 1.0 / 1024.0, at(0, 4)?)?;
    assert!(matches!(
        phase,
        FixturePhaseReferenceAdmission::Accepted { .. }
    ));
    assert!(matches!(
        owner.frequency_admission(),
        FixtureFrequencyAdmission::Disabled
    ));
    assert!(owner.frequency_history().is_none());
    assert_eq!(
        owner.project_model(at(1, 6)?).err(),
        Some(FixtureFrequencyError::Disabled)
    );
    assert_eq!(owner.current_time().active(), ActiveAwakeNs::new(0));
    assert_eq!(
        owner.current_time().monotonic(),
        MonotonicNs::new(4 * SECOND)
    );
    assert_eq!(
        product_ok(owner.issue(at(0, 4)?, FixturePhaseResidualPermission::Deny))?.aggregate_rate(),
        5.0 / 1024.0
    );
    Ok(())
}

/// Independent clocks reject regression without converting one axis to another.
/// # Errors
/// Preserve actual fixture and model admission.
/// # Panics
/// Reject wrong clock priority, mutation before admission or age inferred from active time.
#[test]
fn clock_context_and_each_time_order_guard_preserve_the_model() -> TestResult {
    let source = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
    let chart = chart()?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 1024.0)?, 4)?;
    let mut owner = enabled(
        &mut producer,
        &chart,
        frequency_profile(1.0 / 1024.0, 0.0, 0.125, 32)?,
    )?;
    let phase = receive(&mut owner, &source, 4, 1.0 / 1024.0, at(0, 4)?)?;
    accepted(&owner, &phase, 1);
    let original = snapshot(&history(&owner)?);
    let foreign = FixturePhaseNow::new(
        FixtureRateClockContext::new(label(999)?, label(103)?, label(107)?),
        ActiveAwakeNs::new(0),
        MonotonicNs::new(6 * SECOND),
    );
    assert_eq!(
        owner.project_model(foreign).err(),
        Some(FixtureFrequencyError::ClockContextMismatch)
    );
    assert_eq!(owner.current_time().active(), ActiveAwakeNs::new(0));
    assert_eq!(
        owner.current_time().monotonic(),
        MonotonicNs::new(4 * SECOND)
    );
    product_ok(owner.project_model(at(0, 6)?))?;
    assert_eq!(
        owner.project_model(at(0, 5)?).err(),
        Some(FixtureFrequencyError::TimeRegression {
            clock: FixtureRateClock::Monotonic
        })
    );
    product_ok(owner.project_model(at(1, 6)?))?;
    assert_eq!(
        owner.project_model(at(0, 7)?).err(),
        Some(FixtureFrequencyError::TimeRegression {
            clock: FixtureRateClock::ActiveAwake
        })
    );
    assert_eq!(owner.current_time().active(), ActiveAwakeNs::new(SECOND));
    assert_eq!(
        owner.current_time().monotonic(),
        MonotonicNs::new(6 * SECOND)
    );
    let later = product_ok(owner.project_model(at(1, 7)?))?;
    assert_eq!(later.evidence_age().nanoseconds(), 5 * SECOND);
    assert_eq!(later.added_growth_seconds(), 3.0 / 1024.0);
    assert_eq!(snapshot(later.history()), original);
    Ok(())
}

/// A pending projection advances time but retains the real late-event receiver.
/// # Errors
/// Preserve actual issuance, interval, settlement and take-once transfer failures.
/// # Panics
/// Reject lost custody, double displacement or confusion of total anchor and debt.
#[test]
fn pending_highwater_allows_late_known_total_anchor_rebase_once() -> TestResult {
    let source = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
    let chart = chart()?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 1024.0)?, 4)?;
    let mut owner = enabled(
        &mut producer,
        &chart,
        frequency_profile(1.0 / 1024.0, 0.0, 0.125, 32)?,
    )?;
    let phase = receive(&mut owner, &source, 4, 1.0 / 1024.0, at(0, 4)?)?;
    accepted(&owner, &phase, 1);
    let original = snapshot(&history(&owner)?);
    assert_eq!(
        product_ok(owner.issue(at(0, 4)?, FixturePhaseResidualPermission::Deny))?.aggregate_rate(),
        5.0 / 1024.0
    );
    assert_eq!(
        owner.project_model(at(3, 7)?).err(),
        Some(FixtureFrequencyError::OutstandingOperation)
    );
    assert_eq!(
        owner.current_time().active(),
        ActiveAwakeNs::new(3 * SECOND)
    );
    assert_eq!(
        owner.current_time().monotonic(),
        MonotonicNs::new(7 * SECOND)
    );
    assert_eq!(owner.cursor().next_ordinal(), Some(label(1)?));
    assert_eq!(owner.proposal().map(|p| p.ordinal()), Some(label(1)?));
    assert_eq!(snapshot(&history(&owner)?), original);
    let event =
        FixtureRateSettlement::Completed(interval(owner.chart(), 1, (0, 2), (4, 6), 5.0 / 1024.0)?);
    let mut retired = known(owner, event, at(4, 8)?)?;
    assert_eq!(retired.remaining_phase_seconds(), Some(3.0 / 128.0));
    let rebased = retired.frequency_history();
    assert!(rebased.is_some());
    let rebased =
        rebased.ok_or_else(|| io::Error::other("missing known rebased frequency history"))?;
    assert_eq!(rebased.anchor_phase_seconds(), 11.0 / 512.0);
    assert_eq!(rebased.mean_seconds_per_second(), 0.0);
    assert_eq!(rebased.skew_seconds_per_second(), 1.0 / 1024.0);
    assert_eq!(rebased.reference_frames(), chart.frames());
    assert_eq!(rebased.frames().source_frame(), label(17)?);
    assert_eq!(rebased.frames().source_generation(), label(23)?);
    assert_eq!(rebased.frames().target_frame(), label(31)?);
    assert_eq!(rebased.frames().target_generation(), label(7)?);
    assert_eq!(snapshot(&rebased).times, original.times);
    assert_eq!(snapshot(&rebased).statistics, original.statistics);
    assert_eq!(
        retired
            .cursor()
            .and_then(FixtureRateSettlementCursor::next_ordinal),
        Some(label(2)?)
    );
    {
        let mut resumed =
            product_ok(retired.resume(at(4, 8)?, FixturePhaseResidualPermission::Allow))?;
        let projection = product_ok(resumed.project_model(at(4, 8)?))?;
        assert_eq!(projection.nominal_phase_seconds(), 11.0 / 512.0);
        assert_eq!(projection.history().anchor_phase_seconds(), 11.0 / 512.0);
        assert_eq!(projection.history().frames(), resumed.chart().frames());
    }
    assert!(matches!(
        retired.resume_fresh(
            at(4, 8)?,
            product_ok(FixturePhaseBaseline::new(1.0 / 1024.0))?
        ),
        Err(FixturePhaseError::TransferConsumed)
    ));
    Ok(())
}

/// Real `NoEffect` ages the same record; Unknown consumes custody without zero inference.
/// # Errors
/// Preserve actual producer, issuance and terminal-input admission.
/// # Panics
/// Reject invented delivery, lost history, reused ordinals or false current quality.
#[test]
fn no_effect_then_unknown_preserve_history_but_have_distinct_cursor_results() -> TestResult {
    let source = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
    let chart = chart()?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 1024.0)?, 4)?;
    let mut owner = enabled(
        &mut producer,
        &chart,
        frequency_profile(1.0 / 1024.0, 0.0, 0.125, 32)?,
    )?;
    let phase = receive(&mut owner, &source, 4, 1.0 / 1024.0, at(0, 4)?)?;
    accepted(&owner, &phase, 1);
    let original = snapshot(&history(&owner)?);
    product_ok(owner.issue(at(0, 4)?, FixturePhaseResidualPermission::Deny))?;
    let result = owner.settle(
        FixtureRateSettlement::ProvenNoEffect {
            context: clocks()?,
            ordinal: label(1)?,
        },
        None,
        at(2, 6)?,
    );
    assert!(matches!(&result, FixturePhaseSettlement::ProvenNoEffect(_)));
    let FixturePhaseSettlement::ProvenNoEffect(mut owner) = result else {
        return Err(io::Error::other("missing actual no-effect owner").into());
    };
    assert_eq!(owner.cursor().next_ordinal(), Some(label(2)?));
    assert_eq!(owner.cursor().last_active_end(), None);
    assert_eq!(owner.cursor().last_monotonic_end(), None);
    assert_eq!(snapshot(&history(&owner)?), original);
    let aged = product_ok(owner.project_model(at(2, 6)?))?;
    assert_eq!(aged.nominal_phase_seconds(), 1.0 / 32.0);
    assert_eq!(aged.added_growth_seconds(), 1.0 / 512.0);
    assert_eq!(aged.evidence_age().nanoseconds(), 4 * SECOND);
    assert_eq!(
        product_ok(owner.issue(at(2, 6)?, FixturePhaseResidualPermission::Allow))?.ordinal(),
        label(2)?
    );
    let result = owner.settle(
        FixtureRateSettlement::Unknown {
            context: clocks()?,
            ordinal: label(2)?,
        },
        None,
        at(2, 6)?,
    );
    assert!(matches!(&result, FixturePhaseSettlement::Unknown(_)));
    match result {
        FixturePhaseSettlement::Unknown(blocked) => {
            let retained = blocked.frequency_history();
            assert!(retained.is_some());
            assert_eq!(
                snapshot(&retained.ok_or_else(|| io::Error::other("missing unknown history"))?),
                original
            );
            assert_eq!(
                blocked.cursor().status(),
                FixtureRateCursorStatus::Unresolved
            );
            assert_eq!(blocked.cursor().next_ordinal(), None);
            assert_eq!(blocked.cursor().last_active_end(), None);
            assert_eq!(blocked.cursor().last_monotonic_end(), None);
        }
        _ => return Err(io::Error::other("missing actual unknown receiver").into()),
    }
    Ok(())
}

/// Expiration is inclusive and only genuinely new evidence initializes a new model.
/// # Errors
/// Preserve actual source, profile, clock and receiving failures.
/// # Panics
/// Reject rewind recovery, replay refresh or weighted updating of an expired model.
#[test]
fn age_equality_replay_and_fresh_recovery_use_actual_observation_age() -> TestResult {
    let old = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
    let fresh = Source::new(4, 3_i128 << 24, FRESH_RATE, RESIDUAL / 2)?;
    let chart = chart()?;
    let mut producer = seed(&old.entry(&chart, 4, 1.0 / 1024.0)?, 4)?;
    let mut owner = enabled(
        &mut producer,
        &chart,
        frequency_profile(1.0 / 1024.0, 0.0, 0.125, 6)?,
    )?;
    let first = receive(&mut owner, &old, 4, 1.0 / 1024.0, at(0, 4)?)?;
    accepted(&owner, &first, 1);
    let original = snapshot(&history(&owner)?);
    assert_eq!(
        product_ok(owner.project_model(at(0, 7)?))?
            .evidence_age()
            .nanoseconds(),
        5 * SECOND
    );
    assert_eq!(
        owner.project_model(at(0, 8)?).err(),
        Some(FixtureFrequencyError::Expired)
    );
    assert_eq!(
        owner.project_model(at(0, 7)?).err(),
        Some(FixtureFrequencyError::TimeRegression {
            clock: FixtureRateClock::Monotonic
        })
    );
    let replay = receive(&mut owner, &old, 8, 1.0 / 1024.0, at(0, 8)?)?;
    assert!(matches!(
        replay,
        FixturePhaseReferenceAdmission::AdmittedNoPlan {
            emitted_round: None,
            ..
        }
    ));
    assert_eq!(snapshot(&history(&owner)?), original);
    assert_eq!(
        owner.project_model(at(0, 8)?).err(),
        Some(FixtureFrequencyError::Expired)
    );
    let next = receive(&mut owner, &fresh, 8, 1.0 / 1024.0, at(0, 8)?)?;
    accepted(&owner, &next, 2);
    let recovered = product_ok(owner.project_model(at(0, 10)?))?;
    assert_eq!(recovered.history().mean_seconds_per_second(), 1.0 / 512.0);
    assert_eq!(recovered.history().skew_seconds_per_second(), 1.0 / 1024.0);
    assert_eq!(recovered.history().residual_seconds_per_second(), 0.0);
    assert_eq!(recovered.nominal_phase_seconds(), 5.0 / 256.0);
    assert_eq!(recovered.evidence_age().nanoseconds(), 4 * SECOND);
    check_original(recovered.history(), 4, 8, 2);
    Ok(())
}

/// A known outward growth budget and an earlier valid neighbor cannot be rewound.
/// # Errors
/// Preserve actual source/model calls and explicit clock labels.
/// # Panics
/// Reject a point-only growth test, missing expiry or quality restored by rewind.
#[test]
fn growth_upper_cap_expiry_keeps_the_previous_model_historical() -> TestResult {
    let source = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
    let chart = chart()?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 1024.0)?, 4)?;
    let mut owner = enabled(
        &mut producer,
        &chart,
        frequency_profile(1.0 / 1024.0, 0.0, 1.0 / 1024.0, 32)?,
    )?;
    let phase = receive(&mut owner, &source, 4, 1.0 / 1024.0, at(0, 4)?)?;
    accepted(&owner, &phase, 1);
    let original = snapshot(&history(&owner)?);
    let just_before = FixturePhaseNow::new(
        clocks()?,
        ActiveAwakeNs::new(0),
        MonotonicNs::new(5 * SECOND - 1),
    );
    let before = product_ok(owner.project_model(just_before))?;
    assert!(before.added_growth_upper_seconds() < 1.0 / 1024.0);
    assert!(before.added_growth_upper_seconds() >= before.added_growth_seconds());
    assert_eq!(
        owner.project_model(at(0, 5)?).err(),
        Some(FixtureFrequencyError::Expired)
    );
    assert_eq!(
        owner.project_model(just_before).err(),
        Some(FixtureFrequencyError::TimeRegression {
            clock: FixtureRateClock::Monotonic
        })
    );
    assert_eq!(snapshot(&history(&owner)?), original);
    Ok(())
}

/// Generation/Stop model latches remain reachable after the phase Conflict latch.
/// # Errors
/// Preserve independent actual source setup and conditional projection calls.
/// # Panics
/// Reject first-cause masking, newly granted operations or lost historical facts.
#[test]
fn conflict_then_generation_or_stop_blocks_models_without_erasing_history() -> TestResult {
    for (reason, expected) in [
        (
            FixturePhaseRetirementReason::Generation,
            FixtureFrequencyError::Generation,
        ),
        (
            FixturePhaseRetirementReason::Stop,
            FixtureFrequencyError::Stopped,
        ),
    ] {
        let source = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
        let chart = chart()?;
        let mut producer = seed(&source.entry(&chart, 4, 1.0 / 1024.0)?, 4)?;
        let mut owner = enabled(
            &mut producer,
            &chart,
            frequency_profile(1.0 / 1024.0, 0.0, 0.125, 32)?,
        )?;
        let phase = receive(&mut owner, &source, 4, 1.0 / 1024.0, at(0, 4)?)?;
        accepted(&owner, &phase, 1);
        let original = snapshot(&history(&owner)?);
        owner.retire(FixturePhaseRetirementReason::Conflict);
        assert_eq!(
            product_ok(owner.project_model(at(0, 6)?))?.nominal_phase_seconds(),
            1.0 / 32.0
        );
        owner.retire(reason);
        assert_eq!(owner.project_model(at(0, 7)?).err(), Some(expected));
        assert_eq!(
            owner.current_time().monotonic(),
            MonotonicNs::new(7 * SECOND)
        );
        assert_eq!(snapshot(&history(&owner)?), original);
        assert!(matches!(
            owner.issue(at(0, 7)?, FixturePhaseResidualPermission::Allow),
            Err(FixturePhaseError::Retired {
                reason: FixturePhaseRetirementReason::Conflict
            })
        ));
        assert!(owner.proposal().is_none());
    }
    Ok(())
}

/// Equal-valued foreign charts/rounds cannot supply a ready model to this owner.
/// # Errors
/// Preserve both real producer paths and their independently checked projections.
/// # Panics
/// Reject foreign batch substitution or learning from coincident round numbers.
#[test]
fn foreign_real_producer_round_and_equal_chart_do_not_replace_owned_model() -> TestResult {
    let source = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
    let other = Source::new(0, 7_i128 << 24, FRESH_RATE, RESIDUAL)?;
    let own_chart = chart()?;
    let foreign_chart = chart()?;
    let mut producer = seed(&source.entry(&own_chart, 4, 1.0 / 1024.0)?, 4)?;
    let mut foreign_producer = seed(&other.entry(&foreign_chart, 4, 1.0 / 1024.0)?, 4)?;
    let mut owner = enabled(
        &mut producer,
        &own_chart,
        frequency_profile(1.0 / 1024.0, 0.0, 0.125, 32)?,
    )?;
    let mut foreign = enabled(
        &mut foreign_producer,
        &foreign_chart,
        frequency_profile(1.0 / 1024.0, 0.0, 0.125, 32)?,
    )?;
    let own = receive(&mut owner, &source, 4, 1.0 / 1024.0, at(0, 4)?)?;
    let other_admission = receive(&mut foreign, &other, 4, 1.0 / 1024.0, at(0, 4)?)?;
    accepted(&owner, &own, 1);
    accepted(&foreign, &other_admission, 1);
    assert_eq!(history(&foreign)?.mean_seconds_per_second(), 1.0 / 512.0);
    let original = snapshot(&history(&owner)?);
    let entries = [other.entry(&foreign_chart, 4, 1.0 / 1024.0)?];
    let batch = product_ok(FixtureChartBatch::new(
        &foreign_chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    assert!(matches!(
        owner.receive_reference(&batch, &strata()?, at(0, 4)?),
        Err(FixturePhaseError::Reference {
            source: FixtureReferenceInputError::ForeignChartView
        })
    ));
    assert_eq!(snapshot(&history(&owner)?), original);
    assert_eq!(
        owner.current_time().monotonic(),
        MonotonicNs::new(4 * SECOND)
    );
    assert_eq!(owner.cursor().next_ordinal(), Some(label(1)?));
    Ok(())
}

/// Fresh-context transfer carries the old model without claiming a new observation.
/// # Errors
/// Preserve actual known settlement, new observations and take-once transfer.
/// # Panics
/// Reject double debt integration, a replayed remap or restored transfer capacity.
#[test]
fn fresh_context_transfer_retains_history_until_actual_new_observation() -> TestResult {
    let old = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
    // Actual raw phase13/512 minus known total5/512 gives current phase1/64.
    let fresh = Source::new(4, 13_i128 << 23, 0, RESIDUAL)?;
    let chart = chart()?;
    let mut producer = seed(&old.entry(&chart, 4, 1.0 / 1024.0)?, 4)?;
    let mut owner = enabled(
        &mut producer,
        &chart,
        frequency_profile(1.0 / 1024.0, 0.0, 0.125, 32)?,
    )?;
    let phase = receive(&mut owner, &old, 4, 1.0 / 1024.0, at(0, 4)?)?;
    accepted(&owner, &phase, 1);
    product_ok(owner.issue(at(0, 4)?, FixturePhaseResidualPermission::Deny))?;
    let event =
        FixtureRateSettlement::Completed(interval(owner.chart(), 1, (0, 2), (4, 6), 5.0 / 1024.0)?);
    let mut retired = known(owner, event, at(2, 6)?)?;
    {
        let mut current = product_ok(retired.resume_fresh(
            at(2, 6)?,
            product_ok(FixturePhaseBaseline::new(1.0 / 1024.0))?,
        ))?;
        assert_eq!(current.remaining_phase_seconds(), None);
        let retained = product_ok(current.project_model(at(2, 6)?))?;
        assert_eq!(retained.nominal_phase_seconds(), 11.0 / 512.0);
        assert_eq!(retained.history().emitted_round().get(), 1);
        let original = snapshot(retained.history());
        let replay = receive(&mut current, &old, 6, 1.0 / 1024.0, at(2, 6)?)?;
        assert!(matches!(
            replay,
            FixturePhaseReferenceAdmission::AdmittedNoPlan {
                emitted_round: None,
                ..
            }
        ));
        assert_eq!(snapshot(&history(&current)?), original);
        let next = receive(&mut current, &fresh, 8, 1.0 / 1024.0, at(2, 8)?)?;
        accepted(&current, &next, 2);
        assert_eq!(current.remaining_phase_seconds(), Some(1.0 / 64.0));
        let projection = product_ok(current.project_model(at(2, 8)?))?;
        assert_eq!(projection.nominal_phase_seconds(), 1.0 / 64.0);
        check_original(projection.history(), 4, 8, 2);
        assert_eq!(
            product_ok(current.issue(at(2, 8)?, FixturePhaseResidualPermission::Deny))?
                .aggregate_rate(),
            3.0 / 1024.0
        );
    }
    assert!(matches!(
        retired.resume(at(2, 8)?, FixturePhaseResidualPermission::Allow),
        Err(FixturePhaseError::TransferConsumed)
    ));
    Ok(())
}

/// Exhaustion prevents another operation without inventing model invalidation.
/// # Errors
/// Preserve the public terminal-ordinal cursor and actual no-effect receiver.
/// # Panics
/// Reject wraparound, lost historical observations or a read-only Pending gate.
#[test]
fn exhausted_no_effect_cursor_still_allows_only_readonly_model_projection() -> TestResult {
    let source = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
    let chart = chart()?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 1024.0)?, 4)?;
    let mut owner = product_ok(FixturePhasePlanOwner::new_with_frequency(
        &mut producer,
        &chart,
        FixtureRateSettlementCursor::new(&chart, clocks()?, label(u64::MAX)?),
        at(0, 4)?,
        phase_profile()?,
        product_ok(FixturePhaseBaseline::new(1.0 / 1024.0))?,
        frequency_profile(1.0 / 1024.0, 0.0, 0.125, 32)?,
        association()?,
    ))?;
    let phase = receive(&mut owner, &source, 4, 1.0 / 1024.0, at(0, 4)?)?;
    accepted(&owner, &phase, 1);
    let original = snapshot(&history(&owner)?);
    assert_eq!(
        product_ok(owner.issue(at(0, 4)?, FixturePhaseResidualPermission::Deny))?
            .ordinal()
            .get(),
        u64::MAX
    );
    let result = owner.settle(
        FixtureRateSettlement::ProvenNoEffect {
            context: clocks()?,
            ordinal: label(u64::MAX)?,
        },
        None,
        at(2, 6)?,
    );
    assert!(matches!(&result, FixturePhaseSettlement::ProvenNoEffect(_)));
    let FixturePhaseSettlement::ProvenNoEffect(mut owner) = result else {
        return Err(io::Error::other("missing terminal no-effect owner").into());
    };
    assert_eq!(owner.cursor().status(), FixtureRateCursorStatus::Exhausted);
    assert_eq!(owner.cursor().next_ordinal(), None);
    let projection = product_ok(owner.project_model(at(2, 6)?))?;
    assert_eq!(projection.nominal_phase_seconds(), 1.0 / 32.0);
    assert_eq!(snapshot(projection.history()), original);
    assert!(matches!(
        owner.issue(at(2, 6)?, FixturePhaseResidualPermission::Allow),
        Err(FixturePhaseError::CursorUnavailable)
    ));
    Ok(())
}

/// A rejected source/clock association returns the actual untouched cursor.
/// # Errors
/// Preserve fixture setup and the healthy constructor/intake neighbor.
/// # Panics
/// Reject an accepted foreign association or consumed constructor custody.
#[test]
fn association_constructor_refusal_preserves_cursor_for_a_valid_neighbor() -> TestResult {
    let source = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
    let chart = chart()?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 1024.0)?, 4)?;
    let cursor = FixtureRateSettlementCursor::new(&chart, clocks()?, label(1)?);
    let wrong =
        FixtureFrequencyClockAssociation::new(label(999)?, label(23)?, label(101)?, label(107)?);
    let result = FixturePhasePlanOwner::new_with_frequency(
        &mut producer,
        &chart,
        cursor,
        at(0, 4)?,
        phase_profile()?,
        product_ok(FixturePhaseBaseline::new(1.0 / 1024.0))?,
        frequency_profile(1.0 / 1024.0, 0.0, 0.125, 32)?,
        wrong,
    );
    assert!(result.is_err());
    let (cursor, error) = match result {
        Err(rejected) => rejected.into_parts(),
        Ok(_) => return Err(io::Error::other("foreign association was admitted").into()),
    };
    assert_eq!(
        error,
        FixturePhaseError::Frequency {
            source: FixtureFrequencyError::ClockAssociationMismatch
        }
    );
    let actual_source = error
        .source()
        .and_then(|cause| cause.downcast_ref::<FixtureFrequencyError>());
    assert_eq!(
        actual_source,
        Some(&FixtureFrequencyError::ClockAssociationMismatch)
    );
    assert!(matches!(
        &error,
        FixturePhaseError::Frequency { source }
            if actual_source.is_some_and(|actual| std::ptr::eq(actual, source))
    ));
    assert_eq!(cursor.status(), FixtureRateCursorStatus::Pending);
    assert_eq!(cursor.next_ordinal(), Some(label(1)?));
    assert_eq!(cursor.last_active_end(), None);
    assert_eq!(cursor.last_monotonic_end(), None);
    let mut owner = product_ok(FixturePhasePlanOwner::new_with_frequency(
        &mut producer,
        &chart,
        cursor,
        at(0, 4)?,
        phase_profile()?,
        product_ok(FixturePhaseBaseline::new(1.0 / 1024.0))?,
        frequency_profile(1.0 / 1024.0, 0.0, 0.125, 32)?,
        association()?,
    ))?;
    assert_eq!(
        owner.frequency_admission(),
        FixtureFrequencyAdmission::NoReady {
            emitted_round: None
        }
    );
    let phase = receive(&mut owner, &source, 4, 1.0 / 1024.0, at(0, 4)?)?;
    accepted(&owner, &phase, 1);
    assert_eq!(
        product_ok(owner.project_model(at(0, 4)?))?.nominal_phase_seconds(),
        1.0 / 32.0
    );
    Ok(())
}

/// Each source/clock coordinate refuses without consuming valid constructor custody.
/// # Errors
/// Preserve checked fixtures and the healthy retry after each precise refusal.
/// # Panics
/// Reject a wrong cause, changed cursor or missing initial admission.
#[test]
fn constructor_coordinate_refusals_keep_cursor_and_exact_cause() -> TestResult {
    let source = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
    let chart = chart()?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 1024.0)?, 4)?;

    // Each association coordinate fails independently before any source credit.
    for (source_generation, monotonic_origin, clock_generation, expected) in [
        (
            999,
            101,
            107,
            FixtureFrequencyError::ClockAssociationMismatch,
        ),
        (23, 999, 107, FixtureFrequencyError::ClockContextMismatch),
        (23, 101, 999, FixtureFrequencyError::ClockContextMismatch),
    ] {
        let cursor = FixtureRateSettlementCursor::new(&chart, clocks()?, label(1)?);
        let wrong = FixtureFrequencyClockAssociation::new(
            label(17)?,
            label(source_generation)?,
            label(monotonic_origin)?,
            label(clock_generation)?,
        );
        let result = FixturePhasePlanOwner::new_with_frequency(
            &mut producer,
            &chart,
            cursor,
            at(0, 4)?,
            phase_profile()?,
            product_ok(FixturePhaseBaseline::new(1.0 / 1024.0))?,
            frequency_profile(1.0 / 1024.0, 0.0, 0.125, 32)?,
            wrong,
        );
        assert!(result.is_err());
        let (cursor, error) = match result {
            Err(rejected) => rejected.into_parts(),
            Ok(_) => {
                return Err(
                    io::Error::other("single-coordinate association mismatch admitted").into(),
                );
            }
        };
        assert_eq!(error, FixturePhaseError::Frequency { source: expected });
        assert_eq!(cursor.status(), FixtureRateCursorStatus::Pending);
        assert_eq!(cursor.next_ordinal(), Some(label(1)?));
        assert_eq!(cursor.last_active_end(), None);
        assert_eq!(cursor.last_monotonic_end(), None);
        {
            let recovered = product_ok(FixturePhasePlanOwner::new_with_frequency(
                &mut producer,
                &chart,
                cursor,
                at(0, 4)?,
                phase_profile()?,
                product_ok(FixturePhaseBaseline::new(1.0 / 1024.0))?,
                frequency_profile(1.0 / 1024.0, 0.0, 0.125, 32)?,
                association()?,
            ))?;
            assert_eq!(
                recovered.frequency_admission(),
                FixtureFrequencyAdmission::NoReady {
                    emitted_round: None
                }
            );
            assert!(recovered.frequency_history().is_none());
        }
        assert_eq!(producer.emitted_rounds(), 0);
    }

    Ok(())
}

/// Checked foreign-target cursors retain the frequency owner's failure family.
/// # Errors
/// Preserve checked chart/cursor construction and the matching-chart retry.
/// # Panics
/// Reject an admitted mismatch or substitution of a later generic error.
#[test]
fn foreign_target_cursor_refusals_keep_frequency_cause_and_retry() -> TestResult {
    let source = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
    let chart = chart()?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 1024.0)?, 4)?;
    // These are genuine cursors for separate checked charts, not retargeted fields.
    for (target_frame, target_generation) in [(999, 29), (19, 999)] {
        let other = product_ok(FixtureAffineChart::new(
            FixtureChartFrames::new(
                label(17)?,
                label(23)?,
                label(target_frame)?,
                label(target_generation)?,
            )?,
            chart.domain(),
            0.0,
            0.0,
        ))?;
        let cursor = FixtureRateSettlementCursor::new(&other, clocks()?, label(1)?);
        let result = FixturePhasePlanOwner::new_with_frequency(
            &mut producer,
            &chart,
            cursor,
            at(0, 4)?,
            phase_profile()?,
            product_ok(FixturePhaseBaseline::new(1.0 / 1024.0))?,
            frequency_profile(1.0 / 1024.0, 0.0, 0.125, 32)?,
            association()?,
        );
        assert!(result.is_err());
        let (cursor, error) = match result {
            Err(rejected) => rejected.into_parts(),
            Ok(_) => {
                return Err(io::Error::other("single-coordinate cursor mismatch admitted").into());
            }
        };
        // A later generic mapping rejection cannot substitute for this owner.
        assert_eq!(
            error,
            FixturePhaseError::Frequency {
                source: FixtureFrequencyError::CursorUnavailable
            }
        );
        assert_eq!(cursor.status(), FixtureRateCursorStatus::Pending);
        assert_eq!(cursor.next_ordinal(), Some(label(1)?));
        assert_eq!(cursor.last_active_end(), None);
        assert_eq!(cursor.last_monotonic_end(), None);
        {
            let recovered = product_ok(FixturePhasePlanOwner::new_with_frequency(
                &mut producer,
                &other,
                cursor,
                at(0, 4)?,
                phase_profile()?,
                product_ok(FixturePhaseBaseline::new(1.0 / 1024.0))?,
                frequency_profile(1.0 / 1024.0, 0.0, 0.125, 32)?,
                association()?,
            ))?;
            assert_eq!(
                recovered.frequency_admission(),
                FixtureFrequencyAdmission::NoReady {
                    emitted_round: None
                }
            );
            assert!(recovered.frequency_history().is_none());
        }
        assert_eq!(producer.emitted_rounds(), 0);
    }
    Ok(())
}

/// Pending guards cannot erase expiry or historical Known accounting.
/// # Errors
/// Preserve the real issued operation, late matching terminal and transfer.
/// # Panics
/// Reject an expiry refresh during known rebasing or discarded issued custody.
#[test]
fn expiry_masked_by_pending_operation_stays_expired_after_historical_known() -> TestResult {
    let source = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
    let chart = chart()?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 1024.0)?, 4)?;
    let mut owner = enabled(
        &mut producer,
        &chart,
        frequency_profile(1.0 / 1024.0, 0.0, 0.125, 6)?,
    )?;
    let phase = receive(&mut owner, &source, 4, 1.0 / 1024.0, at(0, 4)?)?;
    accepted(&owner, &phase, 1);
    product_ok(owner.issue(at(0, 4)?, FixturePhaseResidualPermission::Deny))?;
    assert_eq!(
        owner.project_model(at(3, 8)?).err(),
        Some(FixtureFrequencyError::OutstandingOperation)
    );
    assert_eq!(owner.cursor().next_ordinal(), Some(label(1)?));
    let event =
        FixtureRateSettlement::Completed(interval(owner.chart(), 1, (0, 2), (4, 6), 5.0 / 1024.0)?);
    let mut retired = known(owner, event, at(4, 9)?)?;
    assert_eq!(retired.remaining_phase_seconds(), Some(3.0 / 128.0));
    let retained = retired.frequency_history();
    assert!(retained.is_some());
    let retained = retained.ok_or_else(|| io::Error::other("missing expired historical model"))?;
    assert_eq!(retained.anchor_phase_seconds(), 11.0 / 512.0);
    check_original(&retained, 0, 4, 1);
    let mut current = product_ok(retired.resume(at(4, 9)?, FixturePhaseResidualPermission::Allow))?;
    assert_eq!(
        current.project_model(at(4, 9)?).err(),
        Some(FixtureFrequencyError::Expired)
    );
    assert_eq!(
        current.project_model(at(4, 8)?).err(),
        Some(FixtureFrequencyError::TimeRegression {
            clock: FixtureRateClock::Monotonic
        })
    );
    assert_eq!(current.cursor().next_ordinal(), Some(label(2)?));
    assert_eq!(history(&current)?.anchor_phase_seconds(), 11.0 / 512.0);
    Ok(())
}

/// Genuine minimum-normal skew reaches a model-only numeric loss and fresh recovery.
///
/// At the first projected instant elapsed time is exactly zero. One nanosecond
/// later, `MIN_POSITIVE * RN(1/1e9)` is nonzero subnormal and must be refused by
/// the existing point-domain guard. New actual source rows then initialize a
/// supported model on the same chart, without resetting producer/cursor history.
/// # Errors
/// Preserve genuine input, constructor, source receiving and projection failures.
/// # Panics
/// Reject wrong failure family, lost history or recovery without actual new credit.
#[test]
fn genuine_numeric_growth_loss_recovers_only_from_new_ready_evidence() -> TestResult {
    let old = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
    let fresh = Source::new(4, 1_i128 << 26, 0, RESIDUAL / 2)?;
    let chart = chart()?;
    let mut producer = seed(&old.entry(&chart, 4, f64::MIN_POSITIVE)?, 4)?;
    let mut owner = enabled(
        &mut producer,
        &chart,
        frequency_profile(1.0 / 1024.0, 0.0, 0.125, 32)?,
    )?;
    let first = receive(&mut owner, &old, 4, f64::MIN_POSITIVE, at(2, 4)?)?;
    accepted(&owner, &first, 1);
    let initial = product_ok(owner.project_model(at(2, 4)?))?;
    assert_eq!(initial.added_growth_seconds(), 0.0);
    assert_eq!(
        initial.history().skew_seconds_per_second(),
        f64::MIN_POSITIVE
    );
    check_fit_statistics(initial.history(), 1.0 / 64.0);
    let original = snapshot(initial.history());
    let one_nanosecond = FixturePhaseNow::new(
        clocks()?,
        ActiveAwakeNs::new(2 * SECOND),
        MonotonicNs::new(4 * SECOND + 1),
    );
    assert!(matches!(
        owner.project_model(one_nanosecond),
        Err(FixtureFrequencyError::Numeric {
            source: FixtureFrequencyNumericError::Arithmetic {
                step: FixtureFrequencyNumericStep::Growth,
                ..
            }
        })
    ));
    assert_eq!(
        owner.current_time().active(),
        ActiveAwakeNs::new(2 * SECOND)
    );
    assert_eq!(
        owner.current_time().monotonic(),
        MonotonicNs::new(4 * SECOND + 1)
    );
    assert_eq!(snapshot(&history(&owner)?), original);
    assert_eq!(owner.cursor().next_ordinal(), Some(label(1)?));
    assert_eq!(owner.cursor().last_active_end(), None);
    assert_eq!(owner.cursor().last_monotonic_end(), None);
    let next = receive(&mut owner, &fresh, 8, 1.0 / 1024.0, at(2, 8)?)?;
    accepted(&owner, &next, 2);
    let recovered = product_ok(owner.project_model(at(2, 9)?))?;
    assert_eq!(recovered.nominal_phase_seconds(), 1.0 / 64.0);
    assert_eq!(recovered.history().mean_seconds_per_second(), 0.0);
    assert_eq!(recovered.history().skew_seconds_per_second(), 1.0 / 1024.0);
    assert_eq!(recovered.history().residual_seconds_per_second(), 0.0);
    assert_eq!(recovered.added_growth_seconds(), 1.0 / 1024.0);
    assert_eq!(recovered.evidence_age().nanoseconds(), 3 * SECOND);
    check_original(recovered.history(), 4, 8, 2);
    check_fit_statistics(recovered.history(), 1.0 / 128.0);
    assert_eq!(recovered.history().frames(), original.frames);
    assert_eq!(owner.cursor().next_ordinal(), Some(label(1)?));
    assert_eq!(owner.cursor().last_active_end(), None);
    assert_eq!(owner.cursor().last_monotonic_end(), None);
    assert!(owner.proposal().is_none());
    Ok(())
}

/// Rounded companion equality, distinct from the exact-real growth boundary.
///
/// With initial S=2^-10, residual=M=0 and t=8, exact growth is2^-7.
/// The duration enclosure's next-up at8 scales exactly to one ULP at2^-7;
/// outward multiplication and final upper admission each add one more ULP.
/// Thus this independently derived execution control has upper bits base+3.
/// It checks the current companion's admission boundary, not a physical error.
/// # Errors
/// Preserve actual producer, checked profile and model operations.
/// # Panics
/// Reject point-only admission, a non-strict cap or an incorrect valid neighbor.
#[test]
fn outward_growth_cap_equality_and_adjacent_caps_use_the_real_receiver() -> TestResult {
    let exact_bits = (1.0_f64 / 128.0).to_bits();
    let expected_upper_bits = exact_bits + 3;
    for cap_bits in [exact_bits + 2, expected_upper_bits, exact_bits + 4] {
        let source = Source::new(0, INITIAL_PHASE, 0, RESIDUAL)?;
        let chart = chart()?;
        let mut producer = seed(&source.entry(&chart, 4, 1.0 / 1024.0)?, 4)?;
        let mut owner = enabled(
            &mut producer,
            &chart,
            frequency_profile(1.0 / 1024.0, 0.0, f64::from_bits(cap_bits), 32)?,
        )?;
        let phase = receive(&mut owner, &source, 4, 1.0 / 1024.0, at(0, 4)?)?;
        accepted(&owner, &phase, 1);
        let before = snapshot(&history(&owner)?);
        let result = owner.project_model(at(0, 12)?);
        if cap_bits <= expected_upper_bits {
            assert_eq!(result.err(), Some(FixtureFrequencyError::Expired));
        } else {
            let prediction = product_ok(result)?;
            assert_eq!(prediction.added_growth_seconds().to_bits(), exact_bits);
            assert_eq!(
                prediction.added_growth_upper_seconds().to_bits(),
                expected_upper_bits
            );
        }
        assert_eq!(snapshot(&history(&owner)?), before);
        assert_eq!(owner.cursor().next_ordinal(), Some(label(1)?));
    }
    Ok(())
}

/// Explicit physical fixture coefficients; no shipped profile or issuer is chosen.
struct BaseTrace {
    /// Reference-minus-raw-clock gradient in exact Q32 per monotonic second.
    raw_gradient: i128,
    /// Mathematical chart displacement slope.
    chart_rate: f64,
    /// Known old achieved baseline per active second.
    baseline: f64,
    /// Actual source-to-clock displacement slope per monotonic second.
    actual_rate: f64,
    /// Explicit dA/dM; the nonunit witness is one half.
    ratio: f64,
    /// Explicit fixture aggregate cap, shared by phase and base proposals.
    cap: f64,
}
/// Keep the actual public owner and all source producers on one explicit contract.
/// # Errors
/// Preserve checked profiles, association, cursor and old baseline admission.
fn base_owner_at<'p, 'c>(
    producer: &'p mut FixtureCombinationState,
    chart: &'c FixtureAffineChart,
    trace: &BaseTrace,
    initial: FixturePhaseNow,
) -> TestResult<FixturePhasePlanOwner<'p, 'c>> {
    product_ok(FixturePhasePlanOwner::new_with_frequency(
        producer,
        chart,
        FixtureRateSettlementCursor::new(chart, clocks()?, label(1)?),
        initial,
        product_ok(FixturePhaseProfile::new(
            ActiveAwakeNs::new(8 * SECOND).duration_since(ActiveAwakeNs::new(0))?,
            MonotonicNs::new(16 * SECOND).duration_since(MonotonicNs::new(0))?,
            1.0,
            1.0 / 128.0,
            trace.cap,
        ))?,
        product_ok(FixturePhaseBaseline::new(trace.baseline))?,
        frequency_profile(1.0, 0.0, 1.0, 16)?,
        association()?,
    ))
}
/// Keep the established nonunit fixture's initial pair; no conversion is inferred.
/// # Errors
/// Preserve the same checked owner admission as the explicit-initial variant.
fn base_owner<'p, 'c>(
    producer: &'p mut FixtureCombinationState,
    chart: &'c FixtureAffineChart,
    trace: &BaseTrace,
) -> TestResult<FixturePhasePlanOwner<'p, 'c>> {
    base_owner_at(producer, chart, trace, at(2, 4)?)
}
/// Bind actual declared coefficients to this full chart and exact clock readings.
/// # Errors
/// Preserve the mandatory map rule; zero inherited error means an exact fixture.
fn base_map(
    chart: &FixtureAffineChart,
    trace: &BaseTrace,
    now: FixturePhaseNow,
) -> TestResult<FixtureControllerClockMap> {
    product_ok(FixtureControllerClockMap::new(
        chart,
        product_ok(FixturePhaseBaseline::new(trace.baseline))?,
        FixtureControllerClockMapInput {
            reference: now,
            valid_until: MonotonicNs::new(32 * SECOND),
            active_ratio: trace.ratio,
            active_ratio_error: 0.0,
            actual_phase: trace.actual_rate
                * f64::from(u32::try_from(now.monotonic().nanoseconds() / SECOND)?),
            actual_phase_error: 0.0,
            actual_rate: trace.actual_rate,
            actual_rate_error: 0.0,
        },
    ))
}
/// Checked source chart used by the analytic fixture; the common reference is M=2.
/// # Errors
/// Preserve immutable chart/domain construction.
fn base_chart(rate: f64) -> TestResult<FixtureAffineChart> {
    base_chart_at_reference(rate, MonotonicNs::new(2 * SECOND))
}
/// Select a checked coefficient epoch without changing the source/history domain.
/// # Errors
/// Preserve the actual chart's frame, domain and numerical constructor refusals.
fn base_chart_at_reference(rate: f64, reference: MonotonicNs) -> TestResult<FixtureAffineChart> {
    let original = chart()?;
    product_ok(FixtureAffineChart::new(
        original.frames(),
        product_ok(FixtureChartDomain::new(
            original.domain().start(),
            reference,
            original.domain().end(),
        ))?,
        0.0,
        rate,
    ))
}
/// Construct a typed matched achieved receipt, keeping requested and actual separate.
/// # Errors
/// Preserve real spans, the explicit final frame/generation chain and actual baseline admission.
fn base_receipt(
    chart: &FixtureAffineChart,
    ordinal: u64,
    span: ((u64, u64), (u64, u64)),
    rates: (f64, f64),
    target: (u64, u64),
) -> TestResult<FixtureBaselineFeedback> {
    let delivery = interval(chart, ordinal, span.0, span.1, rates.0)?;
    let final_frames = product_ok(FixtureChartFrames::new(
        label(31)?,
        label(7)?,
        label(target.0)?,
        label(target.1)?,
    ))?;
    Ok(FixtureBaselineFeedback::Known(product_ok(
        FixtureBaselineCompletion::new(
            delivery,
            product_ok(FixturePhaseBaseline::new(rates.1))?,
            final_frames,
        ),
    )?))
}
/// Extract only a semantically asserted committed known receipt.
/// # Errors
/// Preserve unexpected branch as a fixture failure, never a fabricated completion.
/// # Panics
/// Reject absence of the independently expected known-custody branch.
fn base_known<'p>(
    result: FixtureBaselineSettlement<'p, '_>,
) -> TestResult<FixtureBaselineRetirement<'p>> {
    assert!(matches!(result, FixtureBaselineSettlement::Known(_)));
    match result {
        FixtureBaselineSettlement::Known(value) => Ok(value),
        _ => Err(io::Error::other("missing committed achieved-base receipt").into()),
    }
}
/// The independently derived nonunit case, with no request→achievement shortcut.
fn nonunit_trace() -> BaseTrace {
    BaseTrace {
        raw_gradient: 1 << 27,
        chart_rate: 1.0 / 128.0,
        baseline: 1.0 / 32.0,
        actual_rate: 1.0 / 64.0,
        ratio: 0.5,
        cap: 1.0 / 8.0,
    }
}

/// Check signed and clipped requests from actual fitted source observations.
/// # Errors
/// Preserve actual source, profile, map and proposal admission failures.
/// # Panics
/// Reject wrong literal rates, clipping, source credit or a premature modeled effect.
#[test]
fn actual_frequency_producer_converts_zero_fast_slow_and_clipped_requests() -> TestResult {
    // Xs=M, T=M+INITIAL_PHASE+q*(M−2): q>0 means a slow local oscillator.
    // Four rows anchor A=2+(M−4), dA/dM=1 and c=k=b0=0; fifth A=M/2.
    for (raw, c, b, k, a, cap, desired, requested, clipped) in [
        (0, 0.0, 0.0, 0.0, 1.0, 0.125, 0.0, 0.0, false),
        (
            1 << 27,
            0.0,
            0.0,
            0.0,
            1.0,
            0.125,
            1.0 / 32.0,
            1.0 / 32.0,
            false,
        ),
        (
            -(1 << 27),
            0.0,
            0.0,
            0.0,
            1.0,
            0.125,
            -1.0 / 32.0,
            -1.0 / 32.0,
            false,
        ),
        (
            1 << 27,
            0.0,
            0.0,
            0.0,
            1.0,
            1.0 / 64.0,
            1.0 / 32.0,
            1.0 / 64.0,
            true,
        ),
        (
            1 << 27,
            1.0 / 128.0,
            1.0 / 32.0,
            1.0 / 64.0,
            0.5,
            0.125,
            1.0 / 16.0,
            1.0 / 16.0,
            false,
        ),
    ] {
        let trace = BaseTrace {
            raw_gradient: raw,
            chart_rate: c,
            baseline: b,
            actual_rate: k,
            ratio: a,
            cap,
        };
        let chart = base_chart(c)?;
        let source = Source::new(0, INITIAL_PHASE, raw, 1 << 20)?;
        let mut producer = seed(&source.entry(&chart, 4, 1.0 / 32.0)?, 4)?;
        let mut owner = base_owner(&mut producer, &chart, &trace)?;
        let now = at(2, 4)?;
        let admitted = receive(&mut owner, &source, 4, 1.0 / 32.0, now)?;
        accepted(&owner, &admitted, 1);
        let before = snapshot(&history(&owner)?);
        let proposal = product_ok(owner.issue_baseline(base_map(&chart, &trace, now)?, now))?;
        assert_eq!(proposal.desired_rate(), desired);
        assert_eq!(proposal.requested_rate(), requested);
        assert_eq!(proposal.clipped(), clipped);
        assert_eq!(proposal.emitted_round(), label(1)?);
        assert_eq!(proposal.ordinal(), label(1)?);
        assert!(proposal.desired_error().is_finite());
        assert_eq!(
            snapshot(&history(&owner)?),
            before,
            "prediction cannot apply the requested base"
        );
        assert!(matches!(
            owner.issue(now, FixturePhaseResidualPermission::Allow),
            Err(FixturePhaseError::OutstandingOperation)
        ));
    }
    Ok(())
}

/// Check the exact nonunit physical witness against the actual committed outputs.
/// # Errors
/// Preserve absent actual model, map or replacement diagnostics.
/// # Panics
/// Reject requested-as-achieved, wrong epoch/sign, age reset or double constant charge.
fn check_quantized_feedback(
    retired: &FixtureBaselineRetirement<'_>,
    before: &FixtureFrequencyHistory,
) -> TestResult<FixtureControllerClockMap> {
    assert_eq!(retired.achieved_baseline().rate(), 3.0 / 64.0);
    assert_eq!(
        retired
            .cursor()
            .and_then(FixtureRateSettlementCursor::next_ordinal),
        Some(label(2)?)
    );
    let after = product_ok(
        retired
            .frequency_history()
            .ok_or_else(|| io::Error::other("lost real frequency model")),
    )?;
    assert_eq!(after.mean_seconds_per_second(), 1.0 / 64.0); // m1=3/128−1/128.
    assert_eq!(
        after.skew_seconds_per_second(),
        before.skew_seconds_per_second()
    );
    assert_eq!(
        after.residual_seconds_per_second(),
        before.residual_seconds_per_second()
    );
    assert_eq!(snapshot(&after).times, snapshot(before).times);
    assert_eq!(after.emitted_round(), before.emitted_round());
    // Old delivery=b0*(Aack−Aissue)=1/32. E(2)=−1/32 and E(4)=−1/64.
    // Therefore combined effect at the common reference M=2 is zero; at Mp=4 it is1/64.
    assert_eq!(
        product_ok(retired.replacement())?.phase_at_reference_seconds(),
        0.0
    );
    assert_eq!(
        product_ok(retired.replacement())?.rate_seconds_per_second(),
        1.0 / 64.0
    );
    assert_eq!(
        after.anchor_phase_seconds(),
        before.anchor_phase_seconds() - 1.0 / 64.0
    );
    let map = *product_ok(retired.clock_map())?;
    assert_eq!(map.actual_rate(), 3.0 / 128.0);
    assert_eq!(map.actual_phase(), 3.0 / 32.0); // Cold(6), with NO second old-delivery constant.
    Ok(map)
}

/// Follow actual quantized achievement through rebase, fresh intake and replay refusal.
/// # Errors
/// Preserve genuine source, map, terminal and custody-transfer admission failures.
/// # Panics
/// Reject wrong achieved-coordinate feedback, minted phase debt or reused operation credit.
#[test]
fn quantized_achieved_base_rebases_actual_frequency_then_requires_fresh_phase_evidence()
-> TestResult {
    let trace = nonunit_trace();
    let chart = base_chart(trace.chart_rate)?;
    let source = Source::new(0, INITIAL_PHASE, trace.raw_gradient, 1 << 20)?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 32.0)?, 4)?;
    let mut owner = base_owner(&mut producer, &chart, &trace)?;
    let admitted = receive(&mut owner, &source, 4, 1.0 / 32.0, at(2, 4)?)?;
    accepted(&owner, &admitted, 1);
    let before = history(&owner)?;
    let map = base_map(&chart, &trace, at(2, 4)?)?;
    assert_eq!(
        product_ok(owner.issue_baseline(map, at(2, 4)?))?.requested_rate(),
        1.0 / 16.0
    );
    let receipt = base_receipt(
        &chart,
        1,
        ((2, 3), (4, 6)),
        (trace.baseline, 3.0 / 64.0),
        (37, 9),
    )?;
    let mut retired = base_known(owner.settle_baseline(receipt, at(3, 6)?))?;
    let map = check_quantized_feedback(&retired, &before)?;
    let mut next = product_ok(retired.resume(at(3, 6)?, FixturePhaseResidualPermission::Allow))?;
    let stale_base = product_ok(FixtureControllerClockMap::new(
        next.chart(),
        product_ok(FixturePhaseBaseline::new(trace.baseline))?,
        FixtureControllerClockMapInput {
            reference: at(3, 6)?,
            valid_until: MonotonicNs::new(32 * SECOND),
            active_ratio: 0.5,
            active_ratio_error: 0.0,
            actual_phase: 3.0 / 32.0,
            actual_phase_error: 0.0,
            actual_rate: 3.0 / 128.0,
            actual_rate_error: 0.0,
        },
    ))?;
    assert!(matches!(
        next.issue_baseline(stale_base, at(3, 6)?),
        Err(FixtureBaselineError::Map)
    ));
    assert!(matches!(
        next.issue(at(3, 6)?, FixturePhaseResidualPermission::Allow),
        Err(FixturePhaseError::NoReference)
    ));
    assert!(
        product_ok(next.project_model(at(3, 6)?))?
            .nominal_phase_seconds()
            .is_finite()
    );
    assert!(matches!(
        next.issue_baseline(map, at(3, 6)?),
        Err(FixtureBaselineError::ReusedEmission)
    ));
    // New genuine raw observations share the original q; assimilation must leave the same physical target.
    let recovered = Source::new(2, INITIAL_PHASE + (1 << 28), trace.raw_gradient, 1 << 20)?;
    let admitted = receive(&mut next, &recovered, 6, 1.0 / 32.0, at(3, 6)?)?;
    accepted(&next, &admitted, 2);
    assert_eq!(
        product_ok(next.issue_baseline(map, at(3, 6)?))?.desired_rate(),
        1.0 / 16.0
    );
    let replay = next.settle_baseline(receipt, at(3, 6)?);
    assert!(matches!(
        replay,
        FixtureBaselineSettlement::OuterRejected {
            error: FixtureBaselineError::Receipt,
            ..
        }
    ));
    let FixtureBaselineSettlement::OuterRejected { owner: next, .. } = replay else {
        return Err(io::Error::other("lost replay-safe owner").into());
    };
    assert_eq!(
        next.baseline_proposal().map(|value| value.ordinal()),
        Some(label(2)?)
    );
    let healthy = FixtureBaselineFeedback::ProvenNoEffect {
        context: clocks()?,
        ordinal: label(2)?,
    };
    assert!(matches!(
        next.settle_baseline(healthy, at(3, 6)?),
        FixtureBaselineSettlement::ProvenNoEffect(_)
    ));
    Ok(())
}

/// Retain the original phase budget when actual achievement is bit-identical.
/// # Errors
/// Preserve genuine source, map, known receipt and residual-transfer setup failures.
/// # Panics
/// Reject changed original deadlines, stale map admission or reused base/source credit.
#[test]
fn known_unchanged_base_keeps_original_phase_deadlines_and_consumes_emission_credit() -> TestResult
{
    let trace = nonunit_trace();
    let chart = base_chart(trace.chart_rate)?;
    let source = Source::new(0, INITIAL_PHASE, trace.raw_gradient, 1 << 20)?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 32.0)?, 4)?;
    let mut owner = base_owner(&mut producer, &chart, &trace)?;
    let admitted = receive(&mut owner, &source, 4, 1.0 / 32.0, at(2, 4)?)?;
    accepted(&owner, &admitted, 1);
    let deadlines = (owner.active_deadline(), owner.monotonic_deadline());
    let before = history(&owner)?;
    let stale_map = base_map(&chart, &trace, at(2, 4)?)?;
    product_ok(owner.issue_baseline(stale_map, at(2, 4)?))?;
    let receipt = base_receipt(
        &chart,
        1,
        ((2, 3), (4, 6)),
        (trace.baseline, trace.baseline),
        (37, 9),
    )?;
    let mut retired = base_known(owner.settle_baseline(receipt, at(3, 6)?))?;
    assert_eq!(retired.achieved_baseline().rate(), trace.baseline);
    let after = product_ok(
        retired
            .frequency_history()
            .ok_or_else(|| io::Error::other("lost unchanged model")),
    )?;
    assert_eq!(
        after.mean_seconds_per_second(),
        before.mean_seconds_per_second()
    );
    assert_eq!(
        after.anchor_phase_seconds(),
        before.anchor_phase_seconds() - 1.0 / 32.0
    );
    let map = *product_ok(retired.clock_map())?;
    let mut next = product_ok(retired.resume(at(3, 6)?, FixturePhaseResidualPermission::Allow))?;
    assert_eq!(
        (next.active_deadline(), next.monotonic_deadline()),
        deadlines
    );
    assert!(matches!(
        next.issue_baseline(stale_map, at(3, 6)?),
        Err(FixtureBaselineError::Map)
    ));
    assert!(matches!(
        next.issue_baseline(map, at(3, 6)?),
        Err(FixtureBaselineError::ReusedEmission)
    ));
    assert_eq!(
        product_ok(next.issue(at(3, 6)?, FixturePhaseResidualPermission::Allow))?.ordinal(),
        label(2)?
    );
    Ok(())
}

/// Distinguish no effect, uncertain effect and rejected replay on the actual cursor.
/// # Errors
/// Preserve real source/map setup and report an unexpected terminal custody branch.
/// # Panics
/// Reject lost pending custody, invented modeled effects or an unblocked uncertain cursor.
#[test]
fn no_effect_unknown_wrong_receipt_and_replay_keep_the_real_shared_cursor() -> TestResult {
    for unknown in [false, true] {
        let trace = nonunit_trace();
        let chart = base_chart(trace.chart_rate)?;
        let source = Source::new(0, INITIAL_PHASE, trace.raw_gradient, 1 << 20)?;
        let mut producer = seed(&source.entry(&chart, 4, 1.0 / 32.0)?, 4)?;
        let mut owner = base_owner(&mut producer, &chart, &trace)?;
        let admitted = receive(&mut owner, &source, 4, 1.0 / 32.0, at(2, 4)?)?;
        accepted(&owner, &admitted, 1);
        let before = snapshot(&history(&owner)?);
        let map = base_map(&chart, &trace, at(2, 4)?)?;
        product_ok(owner.issue_baseline(map, at(2, 4)?))?;
        let wrong = FixtureBaselineFeedback::ProvenNoEffect {
            context: clocks()?,
            ordinal: label(2)?,
        };
        let rejected = owner.settle_baseline(wrong, at(2, 4)?);
        assert!(matches!(
            rejected,
            FixtureBaselineSettlement::OuterRejected {
                error: FixtureBaselineError::Receipt,
                ..
            }
        ));
        let FixtureBaselineSettlement::OuterRejected { owner, .. } = rejected else {
            return Err(io::Error::other("lost unchanged pending owner").into());
        };
        let event = if unknown {
            FixtureBaselineFeedback::Unknown {
                context: clocks()?,
                ordinal: label(1)?,
            }
        } else {
            FixtureBaselineFeedback::ProvenNoEffect {
                context: clocks()?,
                ordinal: label(1)?,
            }
        };
        match owner.settle_baseline(event, at(2, 4)?) {
            FixtureBaselineSettlement::Unknown(blocked) if unknown => {
                assert_eq!(
                    blocked.cursor().status(),
                    FixtureRateCursorStatus::Unresolved
                );
                assert_eq!(
                    snapshot(
                        &blocked
                            .frequency_history()
                            .ok_or_else(|| io::Error::other("lost unknown history"))?
                    ),
                    before
                );
            }
            FixtureBaselineSettlement::ProvenNoEffect(mut next) if !unknown => {
                assert_eq!(snapshot(&history(&next)?), before);
                assert_eq!(next.chart().rate_seconds_per_second(), trace.chart_rate);
                assert!(matches!(
                    next.issue_baseline(map, at(2, 4)?),
                    Err(FixtureBaselineError::ReusedEmission)
                ));
                assert_eq!(
                    product_ok(next.issue(at(2, 4)?, FixturePhaseResidualPermission::Allow))?
                        .ordinal(),
                    label(2)?
                );
                // Replaying the base event cannot settle the now-issued finite phase operation.
                assert!(matches!(
                    next.settle_baseline(event, at(2, 4)?),
                    FixtureBaselineSettlement::OuterRejected { .. }
                ));
            }
            _ => return Err(io::Error::other("wrong actual base terminal branch").into()),
        }
    }
    Ok(())
}

/// Change exactly one declared public numeric operand for a bounded admission case.
/// # Errors
/// Reject a fixture field outside the six numeric map operands.
fn changed_map_scalar(
    mut input: FixtureControllerClockMapInput,
    field: FixtureFrequencyNumericField,
    value: f64,
) -> TestResult<FixtureControllerClockMapInput> {
    match field {
        FixtureFrequencyNumericField::ActiveRatio => input.active_ratio = value,
        FixtureFrequencyNumericField::ActiveRatioError => input.active_ratio_error = value,
        FixtureFrequencyNumericField::MapPhase => input.actual_phase = value,
        FixtureFrequencyNumericField::MapPhaseError => input.actual_phase_error = value,
        FixtureFrequencyNumericField::ActualRate => input.actual_rate = value,
        FixtureFrequencyNumericField::ActualRateError => input.actual_rate_error = value,
        _ => return Err(io::Error::other("unexpected bounded map scalar field").into()),
    }
    Ok(input)
}

/// Challenge every public map scalar through its mandatory checked constructor.
/// # Errors
/// Preserve valid descriptor setup and reject an unexpected fixture field.
/// # Panics
/// Reject missing scalar validation, a wrong typed cause or loss of a legal neighbor.
fn check_public_map_scalar_admission(chart: &FixtureAffineChart, baseline: f64) -> TestResult {
    let baseline = product_ok(FixturePhaseBaseline::new(baseline))?;
    let input = FixtureControllerClockMapInput {
        reference: at(2, 4)?,
        valid_until: MonotonicNs::new(32 * SECOND),
        active_ratio: 0.5,
        active_ratio_error: -0.0,
        actual_phase: -1.0 / 16.0,
        actual_phase_error: -0.0,
        actual_rate: -1.0 / 64.0,
        actual_rate_error: -0.0,
    };
    let healthy = product_ok(FixtureControllerClockMap::new(chart, baseline, input))?;
    assert_eq!(healthy.actual_phase(), -1.0 / 16.0);
    assert_eq!(healthy.actual_rate(), -1.0 / 64.0);
    for field in [
        FixtureFrequencyNumericField::ActiveRatio,
        FixtureFrequencyNumericField::ActiveRatioError,
        FixtureFrequencyNumericField::MapPhase,
        FixtureFrequencyNumericField::MapPhaseError,
        FixtureFrequencyNumericField::ActualRate,
        FixtureFrequencyNumericField::ActualRateError,
    ] {
        let negative_issue = if field == FixtureFrequencyNumericField::ActiveRatio {
            Some(FixtureFrequencyNumericIssue::NonPositive)
        } else if matches!(
            field,
            FixtureFrequencyNumericField::ActiveRatioError
                | FixtureFrequencyNumericField::MapPhaseError
                | FixtureFrequencyNumericField::ActualRateError
        ) {
            Some(FixtureFrequencyNumericIssue::Negative)
        } else {
            None
        };
        let malformed = [
            Some((f64::NAN, FixtureFrequencyNumericIssue::NonFinite)),
            Some((f64::INFINITY, FixtureFrequencyNumericIssue::NonFinite)),
            Some((f64::from_bits(1), FixtureFrequencyNumericIssue::Subnormal)),
            negative_issue.map(|issue| (-1.0, issue)),
        ];
        for (value, issue) in malformed.into_iter().flatten() {
            let changed = changed_map_scalar(input, field, value)?;
            assert!(
                matches!(FixtureControllerClockMap::new(chart, baseline, changed),
                Err(FixtureBaselineError::Numeric {
                    source: FixtureFrequencyNumericError::Input { field: actual_field, issue: actual_issue }
                }) if actual_field == field && actual_issue == issue),
                "{field:?}/{issue:?}"
            );
            assert!(FixtureControllerClockMap::new(chart, baseline, input).is_ok());
        }
    }
    Ok(())
}

/// Check public map refusals and a legal neighboring actual proposal.
/// # Errors
/// Preserve genuine source, checked baseline, legal map and no-effect setup failures.
/// # Panics
/// Reject malformed maps, a contradictory clock pair or refreshed original evidence age.
#[test]
fn public_map_bad_inputs_stale_model_and_affine_segment_discontinuity_refuse_with_legal_neighbor()
-> TestResult {
    let trace = nonunit_trace();
    let chart = base_chart(trace.chart_rate)?;
    check_public_map_scalar_admission(&chart, trace.baseline)?;
    for (ratio, ratio_error) in [(0.0, 0.0), (-0.5, 0.0), (f64::NAN, 0.0), (0.5, 0.5)] {
        assert!(
            FixtureControllerClockMap::new(
                &chart,
                product_ok(FixturePhaseBaseline::new(trace.baseline))?,
                FixtureControllerClockMapInput {
                    reference: at(2, 4)?,
                    valid_until: MonotonicNs::new(32 * SECOND),
                    active_ratio: ratio,
                    active_ratio_error: ratio_error,
                    actual_phase: 1.0 / 16.0,
                    actual_phase_error: 0.0,
                    actual_rate: trace.actual_rate,
                    actual_rate_error: 0.0
                }
            )
            .is_err()
        );
    }
    let source = Source::new(0, INITIAL_PHASE, trace.raw_gradient, 1 << 20)?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 32.0)?, 4)?;
    let mut owner = base_owner(&mut producer, &chart, &trace)?;
    let admitted = receive(&mut owner, &source, 4, 1.0 / 32.0, at(2, 4)?)?;
    accepted(&owner, &admitted, 1);
    let map = base_map(&chart, &trace, at(2, 4)?)?;
    assert!(
        matches!(
            owner.issue_baseline(map, at(3, 5)?),
            Err(FixtureBaselineError::Numeric { .. })
        ),
        "A advanced1 while M advanced1 violates a=1/2"
    );
    // A failed pair is not accepted as current readings; the exact legal neighbor can still issue.
    product_ok(owner.issue_baseline(map, at(3, 6)?))?;
    let event = FixtureBaselineFeedback::ProvenNoEffect {
        context: clocks()?,
        ordinal: label(1)?,
    };
    let FixtureBaselineSettlement::ProvenNoEffect(mut owner) =
        owner.settle_baseline(event, at(3, 6)?)
    else {
        return Err(io::Error::other("no-effect neighbor refused").into());
    };
    assert!(matches!(
        owner.issue_baseline(map, at(9, 18)?),
        Err(FixtureBaselineError::Frequency {
            source: FixtureFrequencyError::Expired
        })
    ));
    Ok(())
}

/// Exercise known facts that cannot become useful continuation authority.
/// # Errors
/// Preserve real source/chart/receipt admission and reject an unknown literal row.
/// # Panics
/// Reject lost achieved facts, consumed custody or bypassed deadline/cap retirement.
fn known_base_boundary_case(row: u8) -> TestResult {
    let trace = if row == 2 {
        BaseTrace {
            raw_gradient: 0,
            chart_rate: 0.0,
            baseline: 0.0,
            actual_rate: 0.0,
            ratio: 0.5,
            cap: 0.125,
        }
    } else {
        nonunit_trace()
    };
    let chart = base_chart(trace.chart_rate)?;
    let source = Source::new(0, INITIAL_PHASE, trace.raw_gradient, 1 << 20)?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 32.0)?, 4)?;
    let mut owner = base_owner(&mut producer, &chart, &trace)?;
    let admitted = receive(&mut owner, &source, 4, 1.0 / 32.0, at(2, 4)?)?;
    accepted(&owner, &admitted, 1);
    product_ok(owner.issue_baseline(base_map(&chart, &trace, at(2, 4)?)?, at(2, 4)?))?;
    let (ack_active, ack_mono, actual) = match row {
        0 => (3, 6, 0.25), // Actual clipped/quantized output remains fact even above the declared cap.
        1 => (10, 20, 3.0 / 64.0), // Exact original active AND monotonic deadlines.
        2 => (3, 6, -0.0), // gpoint=0 is not bit-identical baseline identity.
        _ => return Err(io::Error::other("unknown bounded achieved-base row").into()),
    };
    let receipt = base_receipt(
        &chart,
        1,
        ((2, ack_active), (4, ack_mono)),
        (trace.baseline, actual),
        (37, 9),
    )?;
    let mut retired = base_known(owner.settle_baseline(receipt, at(ack_active, ack_mono)?))?;
    assert_eq!(
        retired.achieved_baseline().rate().to_bits(),
        actual.to_bits()
    );
    assert_eq!(
        retired
            .cursor()
            .and_then(FixtureRateSettlementCursor::next_ordinal),
        Some(label(2)?)
    );
    assert!(
        retired.replacement().is_ok(),
        "known numeric geometry must remain observable"
    );
    if row == 2 {
        let mut next =
            product_ok(retired.resume(at(3, 6)?, FixturePhaseResidualPermission::Allow))?;
        assert!(matches!(
            next.issue(at(3, 6)?, FixturePhaseResidualPermission::Allow),
            Err(FixturePhaseError::NoReference)
        ));
    } else {
        assert!(matches!(
            (row, retired.continuation_error()),
            (0, Some(FixturePhaseError::DeliveredOutOfEnvelope))
                | (1, Some(FixturePhaseError::Expired))
        ));
        assert!(
            retired
                .resume(
                    at(ack_active, ack_mono)?,
                    FixturePhaseResidualPermission::Allow
                )
                .is_err()
        );
        assert!(
            retired.cursor().is_some(),
            "refused resume cannot erase consumed custody"
        );
    }
    Ok(())
}

/// Exercise literal cap, deadline and signed-zero known-feedback boundaries.
/// # Errors
/// Preserve each real fixture and actual known terminal transition failure.
/// # Panics
/// Reject wrong achieved facts or continuation authority in the assertion-bearing cases.
#[test]
fn known_base_cap_deadline_and_signed_zero_have_distinct_lifecycle_outcomes() -> TestResult {
    for row in 0..=2 {
        known_base_boundary_case(row)?;
    }
    Ok(())
}

/// Check actual handler custody before a healthy matching known receipt.
/// # Errors
/// Preserve real source, checked map and matching receipt setup failures.
/// # Panics
/// Reject substituted geometry, wrong old delivery or loss of the pending ordinal.
#[test]
fn real_base_handler_rechecks_substituted_geometry_and_old_rate_before_consumption() -> TestResult {
    let trace = nonunit_trace();
    let chart = base_chart(trace.chart_rate)?;
    let substituted = base_chart(trace.chart_rate + 1.0 / 128.0)?;
    let source = Source::new(0, INITIAL_PHASE, trace.raw_gradient, 1 << 20)?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 32.0)?, 4)?;
    let mut owner = base_owner(&mut producer, &chart, &trace)?;
    let admitted = receive(&mut owner, &source, 4, 1.0 / 32.0, at(2, 4)?)?;
    accepted(&owner, &admitted, 1);
    let wrong_map = base_map(&substituted, &trace, at(2, 4)?)?;
    assert!(matches!(
        owner.issue_baseline(wrong_map, at(2, 4)?),
        Err(FixtureBaselineError::Map)
    ));
    product_ok(owner.issue_baseline(base_map(&chart, &trace, at(2, 4)?)?, at(2, 4)?))?;
    let wrong = base_receipt(
        &chart,
        1,
        ((2, 3), (4, 6)),
        (1.0 / 64.0, 3.0 / 64.0),
        (37, 9),
    )?;
    let outcome = owner.settle_baseline(wrong, at(3, 6)?);
    assert!(matches!(
        outcome,
        FixtureBaselineSettlement::OuterRejected {
            error: FixtureBaselineError::Receipt,
            ..
        }
    ));
    let FixtureBaselineSettlement::OuterRejected { owner, .. } = outcome else {
        return Err(io::Error::other("lost real pending base").into());
    };
    assert_eq!(
        owner.baseline_proposal().map(|value| value.ordinal()),
        Some(label(1)?)
    );
    let healthy = base_receipt(
        &chart,
        1,
        ((2, 3), (4, 6)),
        (trace.baseline, 3.0 / 64.0),
        (37, 9),
    )?;
    let retired = base_known(owner.settle_baseline(healthy, at(3, 6)?))?;
    assert_eq!(retired.achieved_baseline().rate(), 3.0 / 64.0);
    Ok(())
}

/// Retain late known achievement after explicit useful-authority retirement.
/// # Errors
/// Preserve genuine source, map, known receipt and historical custody admission failures.
/// # Panics
/// Reject lost achieved facts/cursor custody or revived stopped authority.
#[test]
fn stopped_base_request_still_records_matched_achievement_without_reviving_authority() -> TestResult
{
    let trace = nonunit_trace();
    let chart = base_chart(trace.chart_rate)?;
    let source = Source::new(0, INITIAL_PHASE, trace.raw_gradient, 1 << 20)?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 32.0)?, 4)?;
    let mut owner = base_owner(&mut producer, &chart, &trace)?;
    let admitted = receive(&mut owner, &source, 4, 1.0 / 32.0, at(2, 4)?)?;
    accepted(&owner, &admitted, 1);
    product_ok(owner.issue_baseline(base_map(&chart, &trace, at(2, 4)?)?, at(2, 4)?))?;
    owner.retire(FixturePhaseRetirementReason::Stop);
    assert!(owner.baseline_proposal().is_none());
    let receipt = base_receipt(
        &chart,
        1,
        ((2, 3), (4, 6)),
        (trace.baseline, 3.0 / 64.0),
        (37, 9),
    )?;
    let mut retired = base_known(owner.settle_baseline(receipt, at(3, 6)?))?;
    assert_eq!(retired.achieved_baseline().rate(), 3.0 / 64.0);
    assert_eq!(
        retired
            .cursor()
            .and_then(FixtureRateSettlementCursor::next_ordinal),
        Some(label(2)?)
    );
    assert!(matches!(
        retired.resume(at(3, 6)?, FixturePhaseResidualPermission::Allow),
        Err(FixtureBaselineError::Phase {
            source: FixturePhaseError::Retired {
                reason: FixturePhaseRetirementReason::Stop
            }
        })
    ));
    Ok(())
}

/// Declare a bounded uncertain ratio for the exact zero-base affine fixture.
/// # Errors
/// Preserve private map admission; no calibration is inferred from construction.
fn uncertain_map(
    chart: &FixtureAffineChart,
    reference: FixturePhaseNow,
) -> TestResult<FixtureControllerClockMap> {
    product_ok(FixtureControllerClockMap::new(
        chart,
        product_ok(FixturePhaseBaseline::new(0.0))?,
        FixtureControllerClockMapInput {
            reference,
            valid_until: MonotonicNs::new(32 * SECOND),
            active_ratio: 1.0,
            active_ratio_error: 1.0 / 8.0,
            actual_phase: 0.0,
            actual_phase_error: 0.0,
            actual_rate: 0.0,
            actual_rate_error: 0.0,
        },
    ))
}

/// Establish one real affine witness through actual owner/handler observations.
/// # Errors
/// Preserve real producer, immutable map and terminal admission failures.
/// # Panics
/// Reject a non-affine known receipt or loss of its healthy original pending owner.
fn uncertain_ratio_case(
    issue_active: u64,
    bad_active: u64,
    healthy: (u64, u64),
    at_reference: bool,
) -> TestResult {
    let trace = BaseTrace {
        raw_gradient: 0,
        chart_rate: 0.0,
        baseline: 0.0,
        actual_rate: 0.0,
        ratio: 1.0,
        cap: 0.125,
    };
    let chart = base_chart(0.0)?;
    let source = Source::new(0, INITIAL_PHASE, 0, 1 << 20)?;
    let mut producer = seed(&source.entry(&chart, 8, 1.0 / 32.0)?, 8)?;
    let mut owner = base_owner_at(&mut producer, &chart, &trace, at(0, 0)?)?;
    let admitted = receive(&mut owner, &source, 8, 1.0 / 32.0, at(issue_active, 8)?)?;
    accepted(&owner, &admitted, 1);
    let map = uncertain_map(
        &chart,
        if at_reference {
            at(issue_active, 8)?
        } else {
            at(0, 0)?
        },
    )?;
    product_ok(owner.issue_baseline(map, at(issue_active, 8)?))?;
    let owner = if at_reference {
        owner
    } else {
        let bad = base_receipt(
            &chart,
            1,
            ((issue_active, bad_active), (8, 16)),
            (0.0, 0.0),
            (37, 9),
        )?;
        let rejected = owner.settle_baseline(bad, at(bad_active, 16)?);
        assert!(matches!(
            rejected,
            FixtureBaselineSettlement::OuterRejected {
                error: FixtureBaselineError::Map | FixtureBaselineError::Numeric { .. },
                ..
            }
        ));
        let FixtureBaselineSettlement::OuterRejected { owner, .. } = rejected else {
            return Err(io::Error::other("lost ratio-rejected pending owner").into());
        };
        assert_eq!(
            owner.baseline_proposal().map(|value| value.ordinal()),
            Some(label(1)?)
        );
        owner
    };
    let event = base_receipt(
        &chart,
        1,
        ((issue_active, healthy.0), (8, healthy.1)),
        (0.0, 0.0),
        (37, 9),
    )?;
    let mut retired = base_known(owner.settle_baseline(event, at(healthy.0, healthy.1)?))?;
    assert_eq!(
        retired
            .cursor()
            .and_then(FixtureRateSettlementCursor::next_ordinal),
        Some(label(2)?)
    );
    if issue_active == 8 && healthy == (12, 12) {
        check_rebound_ratio_custody(&mut retired)?;
    }
    if healthy == (16, 16) {
        assert!(matches!(
            retired.continuation_error(),
            Some(FixturePhaseError::Expired)
        ));
    }
    Ok(())
}

/// A rebound changes the coefficient epoch, not the exact A/M ratio witness.
/// # Errors
/// Preserve take-once custody, real fresh intake and healthy terminal admission.
/// # Panics
/// Reject a newly chosen slope after rebinding or lost pending ordinal on refusal.
fn check_rebound_ratio_custody(retired: &mut FixtureBaselineRetirement<'_>) -> TestResult {
    let map = *product_ok(retired.clock_map())?;
    let mut owner = product_ok(retired.resume(at(12, 12)?, FixturePhaseResidualPermission::Allow))?;
    let source = Source::new(10, INITIAL_PHASE, 0, 1 << 20)?;
    let admitted = receive(&mut owner, &source, 20, 1.0 / 32.0, at(20, 20)?)?;
    accepted(&owner, &admitted, 2);
    product_ok(owner.issue_baseline(map, at(20, 20)?))?;
    // New epoch12/12: both endpoint17/16 and operation9/8 fit the coefficient
    // interval, but neither is the retained exact ratio1 established before ack1.
    // Known1 targets37/generation9; Known2 needs a distinct final generation11.
    let bad = base_receipt(owner.chart(), 2, ((20, 29), (20, 28)), (0.0, 0.0), (41, 11))?;
    let rejected = owner.settle_baseline(bad, at(29, 28)?);
    assert!(matches!(
        rejected,
        FixtureBaselineSettlement::OuterRejected {
            error: FixtureBaselineError::Map,
            ..
        }
    ));
    let FixtureBaselineSettlement::OuterRejected { owner, .. } = rejected else {
        return Err(io::Error::other("lost rebound-ratio pending owner").into());
    };
    assert_eq!(
        owner.baseline_proposal().map(|value| value.ordinal()),
        Some(label(2)?)
    );
    let healthy = base_receipt(owner.chart(), 2, ((20, 27), (20, 27)), (0.0, 0.0), (41, 11))?;
    let retired = base_known(owner.settle_baseline(healthy, at(27, 27)?))?;
    assert_eq!(
        retired
            .cursor()
            .and_then(FixtureRateSettlementCursor::next_ordinal),
        Some(label(3)?)
    );
    Ok(())
}

/// Require exact affine clock custody through actual issued and rebound operations.
/// # Errors
/// Preserve the literal fixture, actual terminal and rebound-control setup failures.
/// # Panics
/// Reject inconsistent clock triples or loss of their healthy shared-cursor neighbors.
#[test]
fn matched_base_handler_requires_one_affine_ratio_across_issue_ack_and_rebound() -> TestResult {
    // Reference0/0, a=1±1/8: each endpoint is allowed, but operation11/8 is not.
    uncertain_ratio_case(7, 18, (14, 16), false)?;
    // Every pair enclosure passes here; exact8/8 differs from17/16 nonetheless.
    // The true affine neighbor12/12 also leaves time for the cross-rebound case.
    uncertain_ratio_case(8, 17, (16, 16), false)?;
    uncertain_ratio_case(8, 17, (12, 12), false)?;
    // At issue==reference, ack1 is the first nonzero observation; custody must survive.
    uncertain_ratio_case(8, 17, (12, 12), true)?;
    Ok(())
}

/// `ProvenNoEffect` retains clock-only evidence without refilling source/base credit.
/// # Errors
/// Preserve real source admission and typed map/receipt construction.
/// # Panics
/// Reject a new affine slope hidden behind an original or newly declared Copy map.
fn no_effect_ratio_case(at_reference: bool) -> TestResult {
    let trace = BaseTrace {
        raw_gradient: 0,
        chart_rate: 0.0,
        baseline: 0.0,
        actual_rate: 0.0,
        ratio: 1.0,
        cap: 0.125,
    };
    let chart = base_chart(0.0)?;
    let source = Source::new(0, INITIAL_PHASE, 0, 1 << 20)?;
    let mut producer = seed(&source.entry(&chart, 8, 1.0 / 32.0)?, 8)?;
    let mut owner = base_owner_at(&mut producer, &chart, &trace, at(0, 0)?)?;
    let admitted = receive(&mut owner, &source, 8, 1.0 / 32.0, at(7, 8)?)?;
    accepted(&owner, &admitted, 1);
    let original_map = uncertain_map(&chart, if at_reference { at(7, 8)? } else { at(0, 0)? })?;
    product_ok(owner.issue_baseline(original_map, at(7, 8)?))?;
    let event = FixtureBaselineFeedback::ProvenNoEffect {
        context: clocks()?,
        ordinal: label(1)?,
    };
    let FixtureBaselineSettlement::ProvenNoEffect(mut owner) =
        owner.settle_baseline(event, if at_reference { at(14, 16)? } else { at(7, 8)? })
    else {
        return Err(io::Error::other("no-effect relation custody lost").into());
    };
    let source = Source::new(10, INITIAL_PHASE, 0, 1 << 20)?;
    let admitted = receive(&mut owner, &source, 16, 1.0 / 32.0, at(14, 16)?)?;
    accepted(&owner, &admitted, 2);
    // Original Copy map(None), or a new current-epoch declaration, cannot
    // erase the original clock relation7/8 after genuine new source emission.
    let incoming = if at_reference {
        // This new declaration fits current14/16 with local slope1, but its
        // origin13/15 contradicts the retained exact global7/8 relation.
        let forged = uncertain_map(&chart, at(13, 15)?)?;
        assert!(matches!(
            owner.issue_baseline(forged, at(14, 16)?),
            Err(FixtureBaselineError::Numeric {
                source: FixtureFrequencyNumericError::ActiveRatioMismatch
            })
        ));
        uncertain_map(&chart, at(14, 16)?)?
    } else {
        original_map
    };
    product_ok(owner.issue_baseline(incoming, at(14, 16)?))?;
    let bad = base_receipt(&chart, 2, ((14, 22), (16, 24)), (0.0, 0.0), (37, 9))?;
    let rejected = owner.settle_baseline(bad, at(22, 24)?);
    assert!(matches!(
        rejected,
        FixtureBaselineSettlement::OuterRejected {
            error: FixtureBaselineError::Map,
            ..
        }
    ));
    let FixtureBaselineSettlement::OuterRejected { owner, .. } = rejected else {
        return Err(io::Error::other("lost ratio-safe pending ordinal2").into());
    };
    let healthy = base_receipt(&chart, 2, ((14, 21), (16, 24)), (0.0, 0.0), (37, 9))?;
    let retired = base_known(owner.settle_baseline(healthy, at(21, 24)?))?;
    assert_eq!(
        retired
            .cursor()
            .and_then(FixtureRateSettlementCursor::next_ordinal),
        Some(label(3)?)
    );
    Ok(())
}

/// Retain accepted clock evidence across no effect and genuine new source intake.
/// # Errors
/// Preserve both real no-effect, map and new-emission fixture failures.
/// # Panics
/// Reject a changed clock origin/slope or loss of the healthy pending operation.
#[test]
fn no_effect_keeps_validated_clock_ratio_across_real_new_emission() -> TestResult {
    no_effect_ratio_case(false)?;
    no_effect_ratio_case(true)?;
    Ok(())
}

/// Recover from source loss through context-only custody after old phase expiry.
/// # Errors
/// Preserve real source, known receipt, take-once transfer and fresh intake failures.
/// # Panics
/// Reject renewed old debt, refreshed stale history or failure to recover from genuine evidence.
#[test]
fn unchanged_known_after_old_phase_expiry_can_transfer_context_for_real_source_recovery()
-> TestResult {
    let trace = BaseTrace {
        raw_gradient: 0,
        chart_rate: 0.0,
        baseline: 0.0,
        actual_rate: 0.0,
        ratio: 0.5,
        cap: 0.125,
    };
    let chart = base_chart(0.0)?;
    let source = Source::new(0, INITIAL_PHASE, 0, 1 << 20)?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 32.0)?, 4)?;
    let mut owner = base_owner(&mut producer, &chart, &trace)?;
    let admitted = receive(&mut owner, &source, 4, 1.0 / 32.0, at(2, 4)?)?;
    accepted(&owner, &admitted, 1);
    let old_deadlines = (owner.active_deadline(), owner.monotonic_deadline());
    let map = base_map(&chart, &trace, at(2, 4)?)?;
    product_ok(owner.issue_baseline(map, at(8, 16)?))?;
    let receipt = base_receipt(&chart, 1, ((8, 11), (16, 22)), (0.0, 0.0), (37, 9))?;
    let mut retired = base_known(owner.settle_baseline(receipt, at(11, 22)?))?;
    assert_eq!(retired.original_phase_deadlines(), old_deadlines);
    assert!(
        retired.continuation_error().is_none(),
        "base operation is within its own original cutoffs"
    );
    assert!(matches!(
        retired.resume(at(11, 22)?, FixturePhaseResidualPermission::Allow),
        Err(FixtureBaselineError::Phase {
            source: FixturePhaseError::Expired
        })
    ));
    let mut next = product_ok(retired.resume_fresh(at(11, 22)?))?;
    assert!(matches!(
        next.issue(at(11, 22)?, FixturePhaseResidualPermission::Allow),
        Err(FixturePhaseError::NoReference)
    ));
    assert!(matches!(
        next.project_model(at(11, 22)?),
        Err(FixtureFrequencyError::Expired)
    ));
    let recovered = Source::new(12, INITIAL_PHASE, 0, 1 << 20)?;
    let admitted = receive(&mut next, &recovered, 22, 1.0 / 32.0, at(11, 22)?)?;
    accepted(&next, &admitted, 2);
    assert_eq!(
        product_ok(next.issue(at(11, 22)?, FixturePhaseResidualPermission::Allow))?.ordinal(),
        label(2)?
    );
    Ok(())
}

/// Admit the original real source and reserve one base operation on the short map.
/// # Errors
/// Preserve actual source, checked map and proposal admission failures.
/// # Panics
/// Reject a missing first actual emission or refusal of the legal pending fixture.
fn prepare_base_map_expiry<'p, 'c>(
    producer: &'p mut FixtureCombinationState,
    chart: &'c FixtureAffineChart,
    source: &Source,
    trace: &BaseTrace,
) -> TestResult<(FixturePhasePlanOwner<'p, 'c>, HistorySnapshot)> {
    let mut owner = base_owner(producer, chart, trace)?;
    let admitted = receive(&mut owner, source, 4, 1.0 / 32.0, at(2, 4)?)?;
    accepted(&owner, &admitted, 1);
    let before = snapshot(&history(&owner)?);
    let map = product_ok(FixtureControllerClockMap::new(
        chart,
        product_ok(FixturePhaseBaseline::new(trace.baseline))?,
        FixtureControllerClockMapInput {
            reference: at(2, 4)?,
            valid_until: MonotonicNs::new(8 * SECOND),
            active_ratio: 0.5,
            active_ratio_error: 0.0,
            actual_phase: 1.0 / 16.0,
            actual_phase_error: 0.0,
            actual_rate: 1.0 / 64.0,
            actual_rate_error: 0.0,
        },
    ))?;
    product_ok(owner.issue_baseline(map, at(2, 4)?))?;
    Ok((owner, before))
}

/// Check actual known delivery/cursor facts before its map/history availability.
/// # Errors
/// Preserve an absent actual cursor or historical model as a fixture failure.
/// # Panics
/// Reject lost achieved facts, changed endpoints or extrapolated map/model history.
fn check_base_map_expiry_known(
    retired: &FixtureBaselineRetirement<'_>,
    before: &HistorySnapshot,
    row: u8,
    ack: (u64, u64),
) -> TestResult {
    assert_eq!(retired.achieved_baseline().rate(), 3.0 / 64.0);
    let cursor = product_ok(
        retired
            .cursor()
            .ok_or_else(|| io::Error::other("missing committed expired-map cursor")),
    )?;
    assert_eq!(cursor.next_ordinal(), Some(label(2)?));
    assert_eq!(
        cursor.last_active_end(),
        Some(ActiveAwakeNs::new(ack.0 * SECOND))
    );
    assert_eq!(
        cursor.last_monotonic_end(),
        Some(MonotonicNs::new(ack.1 * SECOND))
    );
    assert!(matches!(
        retired.event(),
        FixtureRateSettlement::Completed(_)
    ));
    if row == 3 {
        assert!(matches!(
            retired.feedback_error(),
            Some(FixtureBaselineError::Map)
        ));
        assert!(matches!(
            retired.replacement(),
            Err(FixtureBaselineError::Map)
        ));
        assert!(matches!(
            retired.clock_map(),
            Err(FixtureBaselineError::Map)
        ));
        assert_eq!(
            &snapshot(&product_ok(retired.frequency_history().ok_or_else(|| {
                io::Error::other("lost pre-expiry historical model")
            }))?),
            before
        );
    } else {
        assert!(
            retired.replacement().is_ok(),
            "inclusive/timely actual ack permits its actual chart effect"
        );
        assert!(retired.feedback_error().is_none());
        assert_eq!(retired.clock_map().is_ok(), row != 1);
    }
    Ok(())
}

/// Exercise actual context transfer only after the literal knowledge checks.
/// # Errors
/// Preserve the healthy actual context-only transfer's original refusal.
/// # Panics
/// Reject a wrong expiry status or any useful transfer from an unavailable map.
fn check_base_map_expiry_continuation(
    retired: &mut FixtureBaselineRetirement<'_>,
    row: u8,
    published: (u64, u64),
) -> TestResult {
    if row == 0 {
        assert!(retired.continuation_error().is_none());
        product_ok(retired.resume_fresh(at(published.0, published.1)?))?;
    } else {
        assert!(matches!(
            retired.continuation_error(),
            Some(FixturePhaseError::Expired)
        ));
        assert!(retired.resume_fresh(at(published.0, published.1)?).is_err());
    }
    Ok(())
}

/// Match an actual base receipt across mathematical-map expiry without extrapolation.
/// # Errors
/// Preserve genuine source, fixed map, known interval and branch admission.
/// # Panics
/// Reject loss of known achieved facts, old delivery/cursor custody or expired-map gating.
fn base_map_expiry_case(row: u8) -> TestResult {
    let trace = nonunit_trace();
    let chart = base_chart(trace.chart_rate)?;
    let source = Source::new(0, INITIAL_PHASE, trace.raw_gradient, 1 << 20)?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 32.0)?, 4)?;
    let (owner, before) = prepare_base_map_expiry(&mut producer, &chart, &source, &trace)?;
    if row == 4 {
        let result = owner.settle_baseline(
            FixtureBaselineFeedback::Unknown {
                context: clocks()?,
                ordinal: label(1)?,
            },
            at(6, 10)?,
        );
        assert!(matches!(&result, FixtureBaselineSettlement::Unknown(_)));
        let FixtureBaselineSettlement::Unknown(blocked) = result else {
            return Err(io::Error::other("expired Unknown custody lost").into());
        };
        assert_eq!(
            blocked.cursor().status(),
            FixtureRateCursorStatus::Unresolved
        );
        return Ok(());
    }
    let (ack, published) = match row {
        0 => ((3, 6), (3, 6)), // Timely actual boundary and publication; positive future map remains.
        1 => ((4, 8), (4, 8)), // Inclusive map endpoint: known effect, no positive new map segment.
        2 => ((3, 6), (5, 10)), // Timely actual boundary, late publication beyond the original map end.
        3 => ((6, 10), (6, 10)), // Outside map domain: do not extrapolate even to reject A/M=2/3.
        _ => return Err(io::Error::other("unknown map-expiry row").into()),
    };
    let event = base_receipt(
        &chart,
        1,
        ((2, ack.0), (4, ack.1)),
        (trace.baseline, 3.0 / 64.0),
        (37, 9),
    )?;
    let mut retired = base_known(owner.settle_baseline(event, at(published.0, published.1)?))?;
    check_base_map_expiry_known(&retired, &before, row, ack)?;
    check_base_map_expiry_continuation(&mut retired, row, published)
}

/// Keep actual knowledge and map availability distinct at five literal expiry rows.
/// # Errors
/// Preserve real fixture and actual terminal setup failures in each boundary case.
/// # Panics
/// Reject lost actual facts/custody, affine extrapolation or an incorrect continuation outcome.
#[test]
fn actual_base_knowledge_survives_map_expiry_without_affine_extrapolation() -> TestResult {
    for row in 0..=4 {
        base_map_expiry_case(row)?;
    }
    Ok(())
}

/// Observe actual commitment independently of the later model-feedback result.
/// # Errors
/// Preserve absent-cursor, replacement and rebound-map diagnostics after assertions.
/// # Panics
/// Reject lost actual endpoints, an unconsumed ordinal or an earlier chart/map failure.
fn check_anchor_rebase_custody(
    retired: &FixtureBaselineRetirement<'_>,
    achieved: f64,
    gradient: f64,
) -> TestResult {
    assert_eq!(
        retired.achieved_baseline().rate().to_bits(),
        achieved.to_bits()
    );
    assert!(matches!(
        retired.event(),
        FixtureRateSettlement::Completed(_)
    ));
    assert!(retired.cursor().is_some());
    let cursor = retired
        .cursor()
        .ok_or_else(|| io::Error::other("lost committed base cursor"))?;
    assert_eq!(cursor.status(), FixtureRateCursorStatus::Pending);
    assert_eq!(cursor.next_ordinal(), Some(label(2)?));
    assert_eq!(
        cursor.last_active_end(),
        Some(ActiveAwakeNs::new(8 * SECOND))
    );
    assert_eq!(
        cursor.last_monotonic_end(),
        Some(MonotonicNs::new(16 * SECOND))
    );
    assert_eq!(
        retired.original_phase_deadlines(),
        (
            Some(ActiveAwakeNs::new(10 * SECOND)),
            Some(MonotonicNs::new(20 * SECOND))
        )
    );
    assert!(retired.continuation_error().is_none());
    assert!(retired.replacement().is_ok());
    let replacement = product_ok(retired.replacement())?;
    assert_eq!(replacement.phase_at_reference_seconds(), 0.0);
    assert_eq!(replacement.rate_seconds_per_second(), gradient);
    assert_eq!(
        replacement.domain().common_reference(),
        MonotonicNs::new(16 * SECOND)
    );
    assert_eq!(
        replacement.frames(),
        product_ok(FixtureChartFrames::new(
            label(17)?,
            label(23)?,
            label(37)?,
            label(9)?,
        ))?
    );
    assert!(retired.clock_map().is_ok());
    let map = product_ok(retired.clock_map())?;
    assert_eq!(map.reference().monotonic(), MonotonicNs::new(16 * SECOND));
    assert_eq!(map.actual_phase(), 0.0);
    assert_eq!(map.actual_rate(), gradient);
    Ok(())
}

/// Keep a post-consumption numerical refusal distinct from cap/deadline refusal.
/// # Errors
/// Preserve the fixed current-clock fixture admission.
/// # Panics
/// Reject a lost exact arithmetic cause, a changed old model or either useful transfer.
fn check_anchor_rebase_failure(
    retired: &mut FixtureBaselineRetirement<'_>,
    before: &HistorySnapshot,
    after: &FixtureFrequencyHistory,
) -> TestResult {
    assert!(matches!(
        retired.feedback_error(),
        Some(FixtureBaselineError::Frequency {
            source: FixtureFrequencyError::Numeric {
                source: FixtureFrequencyNumericError::Arithmetic {
                    step: FixtureFrequencyNumericStep::AffineAnchorRebase,
                    source: FitError::Arithmetic {
                        stage: ArithmeticStage::Coefficients,
                        operation: ArithmeticOperation::Multiply,
                        issue: ArithmeticIssue::NonFinite,
                    },
                },
            },
        })
    ));
    assert_eq!(&snapshot(after), before);
    assert!(matches!(
        retired.resume(at(8, 16)?, FixturePhaseResidualPermission::Allow),
        Err(FixtureBaselineError::Receipt)
    ));
    assert!(matches!(
        retired.resume_fresh(at(8, 16)?),
        Err(FixtureBaselineError::Receipt)
    ));
    assert_eq!(
        retired
            .cursor()
            .and_then(FixtureRateSettlementCursor::next_ordinal),
        Some(label(2)?)
    );
    Ok(())
}

/// Exercise useful context transfer only for the independently healthy small rate.
/// # Errors
/// Preserve the actual context-only receiver transfer and fixture clock checks.
/// # Panics
/// Reject a wrong model transformation, refreshed history or recreated phase debt.
fn check_anchor_rebase_healthy(
    retired: &mut FixtureBaselineRetirement<'_>,
    before: &HistorySnapshot,
    after: &FixtureFrequencyHistory,
) -> TestResult {
    assert!(retired.feedback_error().is_none());
    assert_eq!(after.mean_seconds_per_second(), -1.0 / 128.0);
    assert_eq!(after.anchor_phase_seconds(), 1.0 / 8.0);
    let changed = snapshot(after);
    assert_eq!(changed.times, before.times);
    assert_eq!(changed.source, before.source);
    assert_eq!(changed.round, before.round);
    assert_eq!(changed.skew, before.skew);
    assert_eq!(changed.residual, before.residual);
    assert_eq!(changed.statistics, before.statistics);
    assert_eq!(changed.frames, product_ok(retired.replacement())?.frames());
    let current = product_ok(retired.resume_fresh(at(8, 16)?))?;
    assert_eq!(current.active_deadline(), None);
    assert_eq!(current.monotonic_deadline(), None);
    assert_eq!(current.cursor().next_ordinal(), Some(label(2)?));
    Ok(())
}

/// Run one finite public trajectory with a separately specified achieved rate.
/// # Errors
/// Preserve genuine source, checked map/receipt and actual transfer failures.
/// # Panics
/// Reject a wrong branch, lost historical identity or an incorrect healthy rebase.
fn anchor_rebase_receiver_case(achieved: f64, gradient: f64, overflow: bool) -> TestResult {
    let trace = BaseTrace {
        raw_gradient: 0,
        chart_rate: 0.0,
        baseline: 0.0,
        actual_rate: 0.0,
        ratio: 0.5,
        cap: f64::from_bits(0x7fe0_0000_0000_0000), // Exactly 2^1023, finite and explicit.
    };
    let chart = base_chart_at_reference(0.0, MonotonicNs::new(16 * SECOND))?;
    let source = Source::new(0, INITIAL_PHASE, 0, 1 << 20)?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 32.0)?, 4)?;
    let mut owner = base_owner(&mut producer, &chart, &trace)?;
    let admitted = receive(&mut owner, &source, 4, 1.0 / 32.0, at(2, 4)?)?;
    accepted(&owner, &admitted, 1);
    let original = history(&owner)?;
    let before = snapshot(&original);
    assert_eq!(original.mean_seconds_per_second(), 0.0);
    assert_eq!(original.anchor_phase_seconds(), 1.0 / 32.0);
    assert_eq!(original.projected_at(), MonotonicNs::new(4 * SECOND));
    {
        let proposal =
            product_ok(owner.issue_baseline(base_map(&chart, &trace, at(2, 4)?)?, at(2, 4)?))?;
        assert_eq!(proposal.requested_rate(), 0.0);
        assert_eq!(proposal.ordinal(), label(1)?);
        assert_eq!(proposal.emitted_round(), label(1)?);
    }
    let receipt = base_receipt(&chart, 1, ((2, 8), (4, 16)), (0.0, achieved), (37, 9))?;
    let mut retired = base_known(owner.settle_baseline(receipt, at(8, 16)?))?;
    check_anchor_rebase_custody(&retired, achieved, gradient)?;
    assert!(retired.frequency_history().is_some());
    let after = retired
        .frequency_history()
        .ok_or_else(|| io::Error::other("lost anchor feedback history"))?;
    if overflow {
        check_anchor_rebase_failure(&mut retired, &before, &after)?;
    } else {
        check_anchor_rebase_healthy(&mut retired, &before, &after)?;
    }
    Ok(())
}

/// Preserve Known facts when a finite public input overflows model-anchor feedback.
///
/// The coefficient epoch equals the acknowledgement M=16, so E(16)=0 while
/// g=2^1021 stays finite. At the real model epoch Mp=4, E(4)=-12*2^1021
/// exceeds binary64's finite range. The small neighbor gives E(4)=-3/32,
/// anchor 1/32-(-3/32)=1/8 and mean 0-1/128=-1/128.
/// These explicit finite extremes select no physical or native clock profile.
/// # Errors
/// Preserve checked public fixture/receiver admission in both independent cases.
/// # Panics
/// Reject an early refusal, lost actual facts, wrong error or useful failed-model transfer.
#[test]
fn known_base_anchor_overflow_preserves_receipt_and_blocks_both_resume_paths() -> TestResult {
    anchor_rebase_receiver_case(
        f64::from_bits(0x7fd0_0000_0000_0000), // Exactly 2^1022 achieved per active second.
        f64::from_bits(0x7fc0_0000_0000_0000), // Independently g=2^1021 per monotonic second.
        true,
    )?;
    anchor_rebase_receiver_case(1.0 / 64.0, 1.0 / 128.0, false)
}

/// Observe the genuine initial state without creating intake or issuance authority.
/// Fixed source/clock/ordinal facts belong to the A2/M4 fixture in the caller.
/// # Errors
/// Preserve checked literal labels or an unexpected missing genuine public history.
/// # Panics
/// Reject partial state publication, a consumed credit or changed source provenance.
fn check_unissued_base_issue_custody(
    owner: &FixturePhasePlanOwner<'_, '_>,
    before: &HistorySnapshot,
    phase: [Option<u64>; 2],
) -> TestResult {
    assert_eq!(&snapshot(&history(owner)?), before);
    assert_eq!(
        owner.current_time().active(),
        ActiveAwakeNs::new(2 * SECOND)
    );
    assert_eq!(
        owner.current_time().monotonic(),
        MonotonicNs::new(4 * SECOND)
    );
    assert_eq!(owner.cursor().next_ordinal(), Some(label(1)?));
    assert_eq!(owner.cursor().last_active_end(), None);
    assert_eq!(owner.cursor().last_monotonic_end(), None);
    assert_eq!(owner.cursor().status(), FixtureRateCursorStatus::Pending);
    assert_eq!(
        [
            owner.remaining_phase_seconds().map(f64::to_bits),
            owner.remaining_phase_error_seconds().map(f64::to_bits)
        ],
        phase
    );
    assert_eq!(
        owner.active_deadline(),
        Some(ActiveAwakeNs::new(10 * SECOND))
    );
    assert_eq!(
        owner.monotonic_deadline(),
        Some(MonotonicNs::new(20 * SECOND))
    );
    let provenance = product_ok(owner.provenance().ok_or("missing genuine phase provenance"))?;
    assert_eq!(provenance.source_context(), before.source);
    assert_eq!(provenance.emitted_round(), before.round);
    assert_eq!(provenance.frames(), before.frames);
    assert_eq!(
        provenance.latest_observed_at().nanoseconds(),
        before.times[1]
    );
    assert_eq!(provenance.projected_at().nanoseconds(), before.times[3]);
    assert!(owner.baseline_proposal().is_none());
    Ok(())
}

/// A new map must account for retained readings already inside its segment.
/// The supplied issue pair A8/M8 fits a=1±1/8 from0/0, but retained A2/M4
/// cannot fit that same segment. Refusal must precede credit or clock publication.
/// # Errors
/// Preserve genuine source/owner/map construction and the healthy retry failure.
/// # Panics
/// Reject a changed cause, partial admission or loss of the original once-only credit.
#[test]
fn baseline_issue_rechecks_retained_current_reading_before_reserving_credit() -> TestResult {
    let trace = BaseTrace {
        raw_gradient: 0,
        chart_rate: 0.0,
        baseline: 0.0,
        actual_rate: 0.0,
        ratio: 1.0,
        cap: 0.125,
    };
    let chart = base_chart(0.0)?;
    let source = Source::new(0, INITIAL_PHASE, 0, 1 << 20)?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 32.0)?, 4)?;
    let mut owner = base_owner(&mut producer, &chart, &trace)?;
    let admitted = receive(&mut owner, &source, 4, 1.0 / 32.0, at(2, 4)?)?;
    accepted(&owner, &admitted, 1);
    let before = snapshot(&history(&owner)?);
    let phase = [
        owner.remaining_phase_seconds().map(f64::to_bits),
        owner.remaining_phase_error_seconds().map(f64::to_bits),
    ];
    check_unissued_base_issue_custody(&owner, &before, phase)?;
    assert!(owner.baseline_proposal().is_none());
    assert!(matches!(
        owner.issue_baseline(uncertain_map(&chart, at(0, 0)?)?, at(8, 8)?),
        Err(FixtureBaselineError::Numeric {
            source: FixtureFrequencyNumericError::ActiveRatioMismatch,
        })
    ));
    check_unissued_base_issue_custody(&owner, &before, phase)?;
    accepted(&owner, &admitted, 1);
    let retry = at(8, 10)?;
    let proposal = product_ok(owner.issue_baseline(uncertain_map(&chart, at(2, 4)?)?, retry))?;
    assert_eq!(proposal.ordinal(), label(1)?);
    assert_eq!(proposal.emitted_round(), label(1)?);
    assert_eq!(proposal.desired_rate(), 0.0);
    assert_eq!(proposal.requested_rate(), 0.0);
    assert!(!proposal.clipped());
    assert_eq!(snapshot(&history(&owner)?), before);
    assert_eq!(owner.current_time().active(), retry.active());
    assert_eq!(owner.current_time().monotonic(), retry.monotonic());
    assert_eq!(owner.cursor().next_ordinal(), Some(label(1)?));
    assert!(matches!(
        owner.issue_baseline(uncertain_map(&chart, retry)?, retry),
        Err(FixtureBaselineError::Phase {
            source: FixturePhaseError::OutstandingOperation
        })
    ));
    Ok(())
}

/// A fresh declared segment may begin after the owner's older retained reading.
/// A3/M6→A4/M8 is exactly a=1/2; retained A2/M4 is outside that new segment
/// and must not be extrapolated as though it were an admitted map observation.
/// # Errors
/// Preserve actual source/owner admission and the checked useful baseline request.
/// # Panics
/// Reject a refused legal segment, renewed source/debt or invented issuance credit.
#[test]
fn baseline_issue_future_map_origin_does_not_extrapolate_older_reading() -> TestResult {
    let trace = BaseTrace {
        raw_gradient: 0,
        chart_rate: 0.0,
        baseline: 0.0,
        actual_rate: 0.0,
        ratio: 0.5,
        cap: 0.125,
    };
    let chart = base_chart(0.0)?;
    let source = Source::new(0, INITIAL_PHASE, 0, 1 << 20)?;
    let mut producer = seed(&source.entry(&chart, 4, 1.0 / 32.0)?, 4)?;
    let mut owner = base_owner(&mut producer, &chart, &trace)?;
    let admitted = receive(&mut owner, &source, 4, 1.0 / 32.0, at(2, 4)?)?;
    accepted(&owner, &admitted, 1);
    let before = snapshot(&history(&owner)?);
    let phase = (
        owner.remaining_phase_seconds(),
        owner.remaining_phase_error_seconds(),
        owner.active_deadline(),
        owner.monotonic_deadline(),
    );
    let map = base_map(&chart, &trace, at(3, 6)?)?;
    assert_eq!(map.reference().active(), ActiveAwakeNs::new(3 * SECOND));
    assert_eq!(map.reference().monotonic(), MonotonicNs::new(6 * SECOND));
    let now = at(4, 8)?;
    let proposal = product_ok(owner.issue_baseline(map, now))?;
    assert_eq!(proposal.ordinal(), label(1)?);
    assert_eq!(proposal.emitted_round(), label(1)?);
    assert_eq!(proposal.desired_rate(), 0.0);
    assert_eq!(proposal.requested_rate(), 0.0);
    assert!(!proposal.clipped());
    assert_eq!(snapshot(&history(&owner)?), before);
    assert_eq!(owner.current_time().active(), now.active());
    assert_eq!(owner.current_time().monotonic(), now.monotonic());
    assert_eq!(
        (
            owner.remaining_phase_seconds(),
            owner.remaining_phase_error_seconds(),
            owner.active_deadline(),
            owner.monotonic_deadline()
        ),
        phase
    );
    assert_eq!(owner.cursor().next_ordinal(), Some(label(1)?));
    assert!(matches!(
        owner.issue_baseline(map, now),
        Err(FixtureBaselineError::Phase {
            source: FixturePhaseError::OutstandingOperation
        })
    ));
    Ok(())
}

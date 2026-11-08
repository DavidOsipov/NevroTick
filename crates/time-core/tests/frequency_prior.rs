// SPDX-License-Identifier: GPL-2.0-only
//! Independent frequency lifecycle controls over genuine exchange-derived fits.
//!
//! All rates, clock labels and budgets are explicit fixture assertions. A learned
//! model supplies neither a vote nor an acknowledged active-time base rate. These
//! tests make no physical, native, service, package or release claim.
#![forbid(unsafe_code)]

use std::{error::Error, fmt, io, num::NonZeroU64};

use time_core::{
    FixtureAdaptiveHistory, FixtureAffineChart, FixtureChartBatch, FixtureChartDomain,
    FixtureChartFrames, FixtureChartInterval, FixtureChartOutcome, FixtureCombinationParameters,
    FixtureCombinationState, FixtureContext, FixtureFrequencyAdmission,
    FixtureFrequencyClockAssociation, FixtureFrequencyError, FixtureFrequencyHistory,
    FixtureFrequencyNumericError, FixtureFrequencyNumericField, FixtureFrequencyNumericIssue,
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
                step: time_core::FixtureFrequencyNumericStep::Growth,
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

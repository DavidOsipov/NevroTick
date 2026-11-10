// SPDX-License-Identifier: GPL-2.0-only
//! Private fixture expiry, Unknown reception and construction-error checks.
//!
//! Expiry and Unknown cases use actual checked histories, producer emissions and
//! owning lifecycle operations before observing private state. The construction
//! error case tests a pure conversion law without constructing an owner or event.
//! Clocks remain fixture assertions; no native completion or physical clock is
//! supplied.

use std::{error::Error, fmt, io, num::NonZeroU64};

use garde::Validate;
use time_domain::{ActiveAwakeNs, MonotonicNs, UtcCoordinateQ32, exchange_arithmetic};

use super::{
    BaselineIssued, BaselineTerminal, ControllerBasis, ControllerClockRelation,
    ControllerRatioWitness, FixtureBaselineCompletion, FixtureBaselineError,
    FixtureBaselineFeedback, FixtureBaselineSettlement, FixtureControllerClockMap,
    FixtureControllerClockMapInput, FixturePhaseBaseline, FixturePhaseError, FixturePhaseNow,
    FixturePhasePlanOwner, FixturePhaseProfile, FixturePhaseReferenceAdmission,
    FixturePhaseResidualPermission, FixturePhaseSettlement, PendingOperation,
    RawBaselineCompletion, RawBaselineTerminal, RawControllerMap, baseline_completion_rule,
    baseline_terminal, controller_map_rule,
};
use crate::{
    FixtureAdaptiveHistory, FixtureAffineChart, FixtureChartBatch, FixtureChartDomain,
    FixtureChartError, FixtureChartFrames, FixtureChartInterval, FixtureChartOutcome,
    FixtureCombinationParameters, FixtureCombinationState, FixtureContext, FixtureFrequencyHistory,
    FixtureFrequencyNumericError, FixtureMinimum, FixtureObservation, FixturePrincipalParameters,
    FixturePrincipalStratum, FixtureProjectedBatch, FixtureProjectionTerms, FixtureRateClock,
    FixtureRateClockContext, FixtureRateCursorStatus, FixtureRateDeliveryError,
    FixtureRateSettlement, FixtureRateSettlementCompletion, FixtureRateSettlementCursor,
    FixtureRateSettlementKind, FixtureRateSpan, FixtureReferenceInputError,
    FixtureReferenceReadiness, FixtureSettledRateInterval, enclose_fixture_chart_projection,
    enclose_fixture_projection,
};

/// Retain an unexpected checked fixture error without inventing fallback values.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Explicit nanoseconds per fixture second, independent of elapsed host time.
const SECOND: u64 = 1_000_000_000;

/// Assert a known-valid pure product trajectory before preserving its refusal.
/// # Errors
/// Preserve the typed failure as fixture context after its semantic assertion.
/// # Panics
/// Reject refusal of the independently specified supported inputs.
#[track_caller]
fn product_ok<T, E: fmt::Debug + fmt::Display>(value: Result<T, E>) -> TestResult<T> {
    assert!(
        value.is_ok(),
        "valid NoEffect fixture was refused: {:?}",
        value.as_ref().err().map(ToString::to_string)
    );
    value.map_err(|error| io::Error::other(format!("NoEffect fixture: {error:?}")).into())
}

/// Admit one explicitly nonzero source, frame, clock or ordinal label.
/// # Errors
/// Reject zero rather than replacing it with another fixture identity.
fn label(value: u64) -> TestResult<NonZeroU64> {
    NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero NoEffect fixture label").into())
}

/// Declare independent active and monotonic origins and one shared generation.
/// # Errors
/// Preserve the fixed labels' nonzero admission failures.
fn clocks() -> TestResult<FixtureRateClockContext> {
    Ok(FixtureRateClockContext::new(
        label(101)?,
        label(103)?,
        label(107)?,
    ))
}

/// Construct the genuine identity-valued map used by every actual source view.
/// # Errors
/// Preserve checked frame/domain/chart refusals.
/// # Panics
/// Reject refusal of the independently supported fixed chart.
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

/// Build one genuine fitted source with literal phases [3/64, 0, 3/64].
///
/// Their independent exact mean is 1/32 and slope is zero at times 0/1/2.
/// The nonzero residual spread supplies a genuine positive fitted rate SD.
/// # Errors
/// Preserve checked exchange, observation, history and projection failures.
/// # Panics
/// Reject missing fitted evidence or refusal of this supported pure fixture.
fn entry(chart: &FixtureAffineChart, id: u64) -> TestResult<FixtureChartInterval<'_>> {
    if ![11, 22].contains(&id) {
        return Err(io::Error::other("source outside fixed NoEffect pair").into());
    }
    let context = FixtureContext::new(label(id)?, label(17)?, label(23)?);
    let mut history = product_ok(FixtureAdaptiveHistory::new(context, 3))?;
    for (second, offset) in [(0, 3_i128 << 26), (1, 0), (2, 3_i128 << 26)] {
        let zero = UtcCoordinateQ32::new(0)?;
        let shifted = UtcCoordinateQ32::new(offset)?;
        let phase = product_ok(exchange_arithmetic(zero, shifted, shifted, zero))?.offset();
        product_ok(history.try_append(FixtureObservation::new(
            context,
            MonotonicNs::new(second * SECOND),
            phase,
            1.0,
        )?))?;
    }
    let prepared = product_ok(history.prepare_chart(context, FixtureChartOutcome::Known(chart)))?;
    assert!(
        prepared.is_some(),
        "genuine fitted history has no chart view"
    );
    let Some(view) = prepared else {
        return Err(io::Error::other("missing fitted chart view").into());
    };
    let reference = history.reference();
    assert_eq!(reference, Some(MonotonicNs::new(2 * SECOND)));
    let Some(reference) = reference else {
        return Err(io::Error::other("missing fitted reference").into());
    };
    let projection = product_ok(view.project_tracking(
        view.context(),
        2,
        reference,
        MonotonicNs::new(4 * SECOND),
        FixtureProjectionTerms::new(0.25, 0.0, 0.125)?,
        0.125,
    ))?;
    product_ok(enclose_fixture_chart_projection(&projection))
}

/// Genuine producer paired with actual chart projections and fixed metadata.
struct ProducerFixture<'chart> {
    /// Actual checked producer state; no local emitted-round payload is created.
    state: FixtureCombinationState,
    /// Original checked source views borrowing the supplied actual chart.
    entries: [FixtureChartInterval<'chart>; 2],
    /// Fixed equal stratum declarations for the two genuine source labels.
    strata: [FixturePrincipalStratum; 2],
}

/// Seed from the actual original projections before the lifecycle advances it.
/// # Errors
/// Preserve checked source, roster, minimum and producer admission failures.
/// # Panics
/// Reject refusal of the supported two-source seeded state.
fn producer(chart: &FixtureAffineChart) -> TestResult<ProducerFixture<'_>> {
    let entries = [entry(chart, 11)?, entry(chart, 22)?];
    let strata = [
        FixturePrincipalStratum::new(label(11)?, 2)?,
        FixturePrincipalStratum::new(label(22)?, 2)?,
    ];
    let raw = [
        enclose_fixture_projection(entries[0].projection().source_projection())?,
        enclose_fixture_projection(entries[1].projection().source_projection())?,
    ];
    let batch = product_ok(FixtureProjectedBatch::new(
        label(17)?,
        label(23)?,
        MonotonicNs::new(4 * SECOND),
        &raw,
    ))?;
    let state = product_ok(FixtureCombinationState::seed(
        &[label(11)?, label(22)?],
        FixturePrincipalParameters::new(1.0 / 1024.0, 0.0)?,
        FixtureCombinationParameters::new(2.0, 0.0)?,
        FixtureMinimum::new(2)?,
        &batch,
        &strata,
    ))?;
    Ok(ProducerFixture {
        state,
        entries,
        strata,
    })
}

/// Admit the explicit eight/sixteen-second fixture policy without a default.
/// # Errors
/// Preserve typed durations and complete mandatory profile admission failures.
/// # Panics
/// Reject refusal of the literal supported profile.
fn profile() -> TestResult<FixturePhaseProfile> {
    product_ok(FixturePhaseProfile::new(
        ActiveAwakeNs::new(8 * SECOND).duration_since(ActiveAwakeNs::new(0))?,
        MonotonicNs::new(16 * SECOND).duration_since(MonotonicNs::new(0))?,
        1.0 / 16.0,
        1.0 / 128.0,
        1.0 / 64.0,
    ))
}

/// Observe genuine returned state before another admission can repair a bad latch.
/// # Errors
/// Retain impossible missing-obligation context after its semantic assertion.
/// # Panics
/// Reject incorrect latching, debt, deadlines, chart identity or cursor history.
fn inspect_no_effect(
    returned: &FixturePhasePlanOwner<'_, '_>,
    chart: &FixtureAffineChart,
    error: Option<f64>,
    current: FixturePhaseNow,
    expired: bool,
) -> TestResult {
    assert!(
        returned.obligation.is_some(),
        "actual NoEffect lost its finite obligation"
    );
    let Some(obligation) = returned.obligation.as_ref() else {
        return Err(io::Error::other("missing NoEffect obligation").into());
    };
    // This is the detector: no issue/current-time check has run after settlement.
    assert_eq!(
        obligation.expired, expired,
        "NoEffect did not latch either reached deadline"
    );
    assert!(obligation.residual);
    assert_eq!(obligation.debt.point(), 1.0 / 32.0);
    assert_eq!(Some(obligation.debt.error()), error);
    assert_eq!(obligation.active_deadline, ActiveAwakeNs::new(8 * SECOND));
    assert_eq!(obligation.monotonic_deadline, MonotonicNs::new(20 * SECOND));
    assert!(returned.issued.is_none());
    assert!(returned.proposal().is_none());
    assert!(std::ptr::eq(returned.chart(), chart));
    assert_eq!(returned.cursor().status(), FixtureRateCursorStatus::Pending);
    assert_eq!(returned.cursor().next_ordinal(), Some(label(2)?));
    assert_eq!(returned.cursor().last_active_end(), None);
    assert_eq!(returned.cursor().last_monotonic_end(), None);
    assert_eq!(returned.current_time().active(), current.active());
    assert_eq!(returned.current_time().monotonic(), current.monotonic());
    Ok(())
}

/// Reach one `NoEffect` branch and inspect the latch before a later check masks it.
/// # Errors
/// Preserve genuine fixture construction and actual owning-operation refusals.
/// # Panics
/// Reject incorrect private expiry, lost custody, invented delivery or rewind.
fn no_effect_case(active: u64, monotonic: u64, clock: Option<FixtureRateClock>) -> TestResult {
    let chart = chart()?;
    let mut fixture = producer(&chart)?;
    let context = clocks()?;
    let initial =
        FixturePhaseNow::new(context, ActiveAwakeNs::new(0), MonotonicNs::new(4 * SECOND));
    let current = FixturePhaseNow::new(
        context,
        ActiveAwakeNs::new(active),
        MonotonicNs::new(monotonic),
    );
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &fixture.entries,
    ))?;
    let mut owner = product_ok(FixturePhasePlanOwner::new(
        &mut fixture.state,
        &chart,
        FixtureRateSettlementCursor::new(&chart, context, label(1)?),
        initial,
        profile()?,
        product_ok(FixturePhaseBaseline::new(1.0 / 1024.0))?,
    ))?;
    let admitted = product_ok(owner.receive_reference(&batch, &fixture.strata, initial))?;
    assert!(
        matches!(admitted, FixturePhaseReferenceAdmission::Accepted { emitted_round }
        if emitted_round == label(1)?)
    );
    assert_eq!(owner.remaining_phase_seconds(), Some(1.0 / 32.0));
    let error = owner.remaining_phase_error_seconds();
    assert!(error.is_some_and(|value| value.is_finite() && value >= 0.0));
    {
        let proposal = product_ok(owner.issue(initial, FixturePhaseResidualPermission::Deny))?;
        assert_eq!(proposal.ordinal(), label(1)?);
        assert_eq!(proposal.transient_rate(), 1.0 / 256.0);
        assert_eq!(proposal.aggregate_rate(), 5.0 / 1024.0);
    }
    let result = owner.settle(
        FixtureRateSettlement::ProvenNoEffect {
            context,
            ordinal: label(1)?,
        },
        None,
        current,
    );
    assert!(matches!(&result, FixturePhaseSettlement::ProvenNoEffect(_)));
    let FixturePhaseSettlement::ProvenNoEffect(mut returned) = result else {
        return Err(io::Error::other("missing actual NoEffect owner").into());
    };
    inspect_no_effect(&returned, &chart, error, current, clock.is_some())?;
    if let Some(expected_clock) = clock {
        assert!(
            matches!(returned.issue(initial, FixturePhaseResidualPermission::Allow),
            Err(FixturePhaseError::TimeRegression { clock: actual }) if actual == expected_clock)
        );
        assert!(matches!(
            returned.issue(current, FixturePhaseResidualPermission::Allow),
            Err(FixturePhaseError::Expired)
        ));
        assert!(returned.proposal().is_none());
    } else {
        assert!(matches!(
            returned.issue(current, FixturePhaseResidualPermission::Deny),
            Err(FixturePhaseError::ResidualPermission)
        ));
        let proposal = product_ok(returned.issue(current, FixturePhaseResidualPermission::Allow))?;
        assert_eq!(proposal.ordinal(), label(2)?);
        assert_eq!(proposal.transient_rate(), 1.0 / 256.0);
        assert_eq!(proposal.aggregate_rate(), 5.0 / 1024.0);
    }
    assert_eq!(returned.remaining_phase_seconds(), Some(1.0 / 32.0));
    assert_eq!(returned.remaining_phase_error_seconds(), error);
    assert_eq!(returned.cursor().next_ordinal(), Some(label(2)?));
    assert_eq!(returned.cursor().last_active_end(), None);
    assert_eq!(returned.cursor().last_monotonic_end(), None);
    Ok(())
}

/// Independent one-clock deadlines must latch during actual `NoEffect` reception.
/// # Errors
/// Preserve actual fixture construction and genuine owning lifecycle failures.
/// # Panics
/// Reject missed latches, unconditional expiry, invented delivery or clock rewind.
#[test]
fn no_effect_latches_each_deadline_before_later_issue_can_mask_it() -> TestResult {
    for (active, monotonic, clock) in [
        (0, 4 * SECOND, None),
        (8 * SECOND, 4 * SECOND, Some(FixtureRateClock::ActiveAwake)),
        (0, 20 * SECOND, Some(FixtureRateClock::Monotonic)),
    ] {
        no_effect_case(active, monotonic, clock)?;
    }
    Ok(())
}

/// Projection of valid construction causes preserves their typed frequency family.
///
/// This calls a pure private classifier directly. It fabricates no owner, event
/// or constructor result and claims no public constructor reaches this arm.
/// # Panics
/// Reject cause erasure in the bounded projection or its nonfrequency neighbor.
#[test]
fn construction_error_projection_preserves_frequency_causes_without_owner_state() {
    for source in [
        crate::FixtureFrequencyError::ClockContextMismatch,
        crate::FixtureFrequencyError::ClockAssociationMismatch,
        crate::FixtureFrequencyError::CursorUnavailable,
        crate::FixtureFrequencyError::ValidationInvariant,
    ] {
        let original = FixturePhaseError::Frequency { source };
        let restored = super::ConstructionFailure::from_error(&original).into_error();
        assert_eq!(restored, original);
    }
    let neighbor = FixturePhaseError::ClockContextMismatch;
    assert_eq!(
        super::ConstructionFailure::from_error(&neighbor).into_error(),
        neighbor
    );
}

/// Enable the existing genuine fixture without attaching a model or source credit.
/// # Errors
/// Preserve checked fixture labels, profiles, cursor and constructor refusals.
/// # Panics
/// Reject refusal of these explicit supported scalar and association inputs.
fn frequency_owner_for_unknown<'producer, 'chart>(
    state: &'producer mut FixtureCombinationState,
    chart: &'chart FixtureAffineChart,
    initial: FixturePhaseNow,
) -> TestResult<FixturePhasePlanOwner<'producer, 'chart>> {
    let frequency = product_ok(crate::FixtureFrequencyProfile::new(
        1.0,
        0.0,
        1.0,
        MonotonicNs::new(32 * SECOND).duration_since(MonotonicNs::new(0))?,
    ))?;
    product_ok(FixturePhasePlanOwner::new_with_frequency(
        state,
        chart,
        FixtureRateSettlementCursor::new(chart, clocks()?, label(1)?),
        initial,
        profile()?,
        product_ok(FixturePhaseBaseline::new(1.0 / 1024.0))?,
        frequency,
        crate::FixtureFrequencyClockAssociation::new(
            label(17)?,
            label(23)?,
            label(101)?,
            label(107)?,
        ),
    ))
}

/// Compare every exposed historical field without granting current model quality.
/// # Panics
/// Reject changed statistics, numerical bounds, identity, chart support or ages.
fn assert_unknown_frequency_history(
    actual: &FixtureFrequencyHistory,
    original: &FixtureFrequencyHistory,
) {
    assert_eq!(
        [
            actual.mean_seconds_per_second().to_bits(),
            actual.mean_roundoff_seconds_per_second().to_bits(),
            actual.skew_seconds_per_second().to_bits(),
            actual.skew_roundoff_seconds_per_second().to_bits(),
            actual.residual_seconds_per_second().to_bits(),
            actual.residual_roundoff_seconds_per_second().to_bits(),
            actual.anchor_phase_seconds().to_bits(),
            actual.anchor_roundoff_seconds().to_bits(),
            actual.phase_standard_deviation_seconds().to_bits(),
            actual
                .rate_standard_deviation_seconds_per_second()
                .to_bits(),
        ],
        [
            original.mean_seconds_per_second().to_bits(),
            original.mean_roundoff_seconds_per_second().to_bits(),
            original.skew_seconds_per_second().to_bits(),
            original.skew_roundoff_seconds_per_second().to_bits(),
            original.residual_seconds_per_second().to_bits(),
            original.residual_roundoff_seconds_per_second().to_bits(),
            original.anchor_phase_seconds().to_bits(),
            original.anchor_roundoff_seconds().to_bits(),
            original.phase_standard_deviation_seconds().to_bits(),
            original
                .rate_standard_deviation_seconds_per_second()
                .to_bits(),
        ]
    );
    assert_eq!(actual.emitted_round(), original.emitted_round());
    assert_eq!(actual.source_context(), original.source_context());
    assert_eq!(actual.frames(), original.frames());
    assert_eq!(actual.reference_frames(), original.reference_frames());
    let domain = actual.domain();
    let original_domain = original.domain();
    assert_eq!(
        [domain.start(), domain.common_reference(), domain.end()],
        [
            original_domain.start(),
            original_domain.common_reference(),
            original_domain.end(),
        ]
    );
    assert_eq!(
        [
            actual.anchor_observed_at(),
            actual.latest_observed_at(),
            actual.fit_reference(),
            actual.projected_at(),
        ],
        [
            original.anchor_observed_at(),
            original.latest_observed_at(),
            original.fit_reference(),
            original.projected_at(),
        ]
    );
}

/// Actual parent Unknown reception must latch the retained frequency component.
///
/// The final component query is a private state observation. The public blocked
/// owner still has no projection or recovery path, and this grants neither one.
/// # Errors
/// Preserve genuine fixture setup and actual constructor/intake/issue failures.
/// # Panics
/// Reject a lost Unknown latch, invented history or reusable terminal custody.
#[test]
fn actual_unknown_reception_latches_frequency_before_private_projection() -> TestResult {
    let chart = chart()?;
    let mut fixture = producer(&chart)?;
    let context = clocks()?;
    let initial =
        FixturePhaseNow::new(context, ActiveAwakeNs::new(0), MonotonicNs::new(4 * SECOND));
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &fixture.entries,
    ))?;
    let mut owner = frequency_owner_for_unknown(&mut fixture.state, &chart, initial)?;
    let admitted = product_ok(owner.receive_reference(&batch, &fixture.strata, initial))?;
    assert!(matches!(
        admitted,
        FixturePhaseReferenceAdmission::Accepted { emitted_round }
            if emitted_round == label(1)?
    ));
    assert!(matches!(
        owner.frequency_admission(),
        crate::FixtureFrequencyAdmission::Accepted { emitted_round }
            if emitted_round == label(1)?
    ));
    let healthy = product_ok(owner.project_model(initial))?;
    assert_eq!(healthy.nominal_phase_seconds(), 1.0 / 32.0);
    assert_eq!(healthy.evidence_age().nanoseconds(), 2 * SECOND);
    let original = *healthy.history();
    {
        let proposal = product_ok(owner.issue(initial, FixturePhaseResidualPermission::Deny))?;
        assert_eq!(proposal.ordinal(), label(1)?);
    }
    let result = owner.settle(
        FixtureRateSettlement::Unknown {
            context,
            ordinal: label(1)?,
        },
        None,
        initial,
    );
    assert!(matches!(&result, FixturePhaseSettlement::Unknown(_)));
    let FixturePhaseSettlement::Unknown(mut blocked) = result else {
        return Err(io::Error::other("missing actual Unknown parent result").into());
    };
    assert_eq!(
        blocked.cursor().status(),
        FixtureRateCursorStatus::Unresolved
    );
    assert_eq!(blocked.cursor().next_ordinal(), None);
    assert_eq!(blocked.cursor().last_active_end(), None);
    assert_eq!(blocked.cursor().last_monotonic_end(), None);
    // Same chart/time was healthy before the real terminal call. Only the
    // component's Unknown latch should prevent this private query afterward.
    let projected =
        blocked
            .continuation
            .frequency
            .project(&chart, context, initial.monotonic(), false);
    assert!(matches!(
        projected,
        Err(crate::FixtureFrequencyError::Unknown)
    ));
    let retained = blocked.frequency_history();
    assert!(retained.is_some());
    let Some(retained) = retained else {
        return Err(io::Error::other("actual Unknown lost its historical model").into());
    };
    assert_unknown_frequency_history(&retained, &original);
    assert_eq!(
        blocked.cursor().status(),
        FixtureRateCursorStatus::Unresolved
    );
    assert_eq!(blocked.cursor().next_ordinal(), None);
    Ok(())
}

/// Private full-u64 helper-domain precision control, outside public span admission.
/// A unit difference at MAX is below f64 ratio resolution; integer cross-products
/// must preserve it. This supplies no physical trace, mapping or source credit.
/// # Panics
/// Require the exact unit ratio and zero-span neighbors; reject the unequal ratio.
#[test]
fn exact_clock_ratio_witness_distinguishes_full_u64_precision_neighbor() {
    let witness = ControllerRatioWitness {
        active: u64::MAX,
        monotonic: u64::MAX,
    };
    assert!(
        witness.admits(u64::MAX, u64::MAX).is_ok(),
        "MAX squared fits checked u128"
    );
    assert!(matches!(
        witness.admits(u64::MAX - 1, u64::MAX),
        Err(FixtureBaselineError::Map)
    ));
    assert!(
        witness.admits(0, 0).is_ok(),
        "zero span preserves an existing ratio without creating one"
    );
}

/// Obtain distinct phase/rate points and both inherited coefficient bounds.
/// # Errors
/// Preserve checked chart, frame and ordinary-composition fixture failures.
/// # Panics
/// Reject refusal of the supported dyadic operands.
fn controller_geometry() -> TestResult<FixtureAffineChart> {
    let support = chart()?.domain();
    let first = product_ok(FixtureAffineChart::new(
        FixtureChartFrames::new(label(17)?, label(23)?, label(31)?, label(7)?)?,
        support,
        1.0 / 64.0,
        1.0 / 2048.0,
    ))?;
    let second = product_ok(FixtureAffineChart::new(
        FixtureChartFrames::new(label(31)?, label(7)?, label(19)?, label(29)?)?,
        support,
        1.0 / 64.0,
        1.0 / 2048.0,
    ))?;
    product_ok(first.compose(&second))
}

/// Change one private captured operand, without changing a checked chart.
/// # Errors
/// Preserve checked frame/domain construction or reject an unknown row.
fn changed_controller_basis(mut basis: ControllerBasis, row: usize) -> TestResult<ControllerBasis> {
    match row {
        0..=3 => {
            let mut tags = [17, 23, 19, 29];
            tags[row] = [41, 43, 47, 53][row];
            basis.frames = FixtureChartFrames::new(
                label(tags[0])?,
                label(tags[1])?,
                label(tags[2])?,
                label(tags[3])?,
            )?;
        }
        4..=6 => {
            let mut times = [0, 2 * SECOND, 64 * SECOND];
            times[row - 4] = [SECOND, 3 * SECOND, 63 * SECOND][row - 4];
            basis.domain = FixtureChartDomain::new(
                MonotonicNs::new(times[0]),
                MonotonicNs::new(times[1]),
                MonotonicNs::new(times[2]),
            )?;
        }
        7..=10 => basis.coefficients[row - 7] ^= 1,
        _ => return Err(io::Error::other("unknown controller basis component").into()),
    }
    Ok(basis)
}

/// Observe every captured component against fixed independent fixture facts.
/// # Panics
/// Reject reordered fields, reset error bounds or altered literal geometry.
fn assert_controller_basis_capture(actual: &FixtureAffineChart, basis: &ControllerBasis) {
    assert_eq!(
        [
            basis.frames.source_frame().get(),
            basis.frames.source_generation().get(),
            basis.frames.target_frame().get(),
            basis.frames.target_generation().get()
        ],
        [17, 23, 19, 29]
    );
    assert_eq!(
        [
            basis.domain.start().nanoseconds(),
            basis.domain.common_reference().nanoseconds(),
            basis.domain.end().nanoseconds()
        ],
        [0, 2 * SECOND, 64 * SECOND]
    );
    assert!(actual.phase_coefficient_roundoff_seconds() > 0.0);
    assert!(actual.rate_coefficient_roundoff_seconds_per_second() > 0.0);
    assert_ne!(
        actual.phase_coefficient_roundoff_seconds(),
        actual.rate_coefficient_roundoff_seconds_per_second()
    );
    assert_eq!(
        basis.coefficients,
        [
            (1.0_f64 / 32.0).to_bits(),
            (1.0_f64 / 1024.0).to_bits(),
            actual.phase_coefficient_roundoff_seconds().to_bits(),
            actual
                .rate_coefficient_roundoff_seconds_per_second()
                .to_bits()
        ]
    );
}

/// This is the phase owner's `ControllerBasis`, not the frequency state's `Basis`.
/// Private operand faults supply no public chart mutation or physical identity.
/// # Errors
/// Preserve genuine composition and checked neighboring geometry failures.
/// # Panics
/// Reject an omitted captured component, loose matching or lost signed-zero identity.
#[test]
fn controller_basis_captures_and_checks_every_geometry_component_bit() -> TestResult {
    let actual = controller_geometry()?;
    let basis = ControllerBasis::from_chart(&actual);
    assert_controller_basis_capture(&actual, &basis);
    for row in 0..=10 {
        assert!(basis.matches(&actual));
        assert!(
            !changed_controller_basis(basis, row)?.matches(&actual),
            "captured controller component must distinguish its neighbor"
        );
        assert!(ControllerBasis::from_chart(&actual).matches(&actual));
    }
    let zero = product_ok(FixtureAffineChart::new(
        actual.frames(),
        actual.domain(),
        0.0,
        0.0,
    ))?;
    let zero_basis = ControllerBasis::from_chart(&zero);
    for (phase, rate) in [(-0.0, 0.0), (0.0, -0.0)] {
        let signed = product_ok(FixtureAffineChart::new(
            actual.frames(),
            actual.domain(),
            phase,
            rate,
        ))?;
        assert!(!zero_basis.matches(&signed));
        assert!(ControllerBasis::from_chart(&signed).matches(&signed));
        assert!(zero_basis.matches(&zero));
    }
    Ok(())
}

/// Declare a checked uncertain mathematical segment, with no native authority.
/// # Errors
/// Preserve checked baseline/map admission and the fixed fixture's labels.
/// # Panics
/// Reject refusal of the supported map declaration.
fn private_controller_map(
    chart: &FixtureAffineChart,
    reference: FixturePhaseNow,
) -> TestResult<FixtureControllerClockMap> {
    product_ok(FixtureControllerClockMap::new(
        chart,
        product_ok(FixturePhaseBaseline::new(1.0 / 1024.0))?,
        FixtureControllerClockMapInput {
            reference,
            valid_until: MonotonicNs::new(32 * SECOND),
            active_ratio: 0.5,
            active_ratio_error: 1.0 / 8.0,
            actual_phase: 0.0,
            actual_phase_error: 0.0,
            actual_rate: 0.0,
            actual_rate_error: 0.0,
        },
    ))
}

/// Raw adapter operands are deliberately private relationship faults, not maps.
/// # Errors
/// Preserve checked chart/map and neighboring frame/domain admission failures.
/// # Panics
/// Reject a missing Garde callback or disagreement with its maintained predicate.
#[test]
fn raw_controller_map_validation_rejects_each_relationship_with_healthy_restoration() -> TestResult
{
    let original = chart()?;
    let chart = product_ok(FixtureAffineChart::new(
        original.frames(),
        FixtureChartDomain::new(
            MonotonicNs::new(SECOND),
            MonotonicNs::new(2 * SECOND),
            MonotonicNs::new(64 * SECOND),
        )?,
        0.0,
        0.0,
    ))?;
    let reference = FixturePhaseNow::new(
        clocks()?,
        ActiveAwakeNs::new(0),
        MonotonicNs::new(2 * SECOND),
    );
    let valid = private_controller_map(&chart, reference)?;
    for row in 0..5 {
        assert!(
            RawControllerMap {
                value: (&chart, &valid)
            }
            .validate()
            .is_ok()
        );
        let mut invalid = valid;
        match row {
            0 => invalid.basis = changed_controller_basis(valid.basis, 0)?,
            1 => {
                invalid.reference =
                    FixturePhaseNow::new(clocks()?, ActiveAwakeNs::new(0), MonotonicNs::new(0));
            }
            2 => invalid.valid_until = MonotonicNs::new(65 * SECOND),
            3 => invalid.valid_until = reference.monotonic(),
            _ => invalid.valid_until = MonotonicNs::new(2 * SECOND - 1),
        }
        assert!(
            RawControllerMap {
                value: (&chart, &invalid)
            }
            .validate()
            .is_err()
        );
        assert!(matches!(
            controller_map_rule(&chart, &invalid),
            Err(FixtureBaselineError::Map)
        ));
        assert!(
            RawControllerMap {
                value: (&chart, &valid)
            }
            .validate()
            .is_ok()
        );
        product_ok(controller_map_rule(&chart, &valid))?;
    }
    Ok(())
}

/// Build typed completion operands; only a receiving owner can admit the event.
/// # Errors
/// Preserve actual frame, rate-interval, baseline and complete-chain construction.
/// # Panics
/// Reject refusal of this supported standalone fixture receipt.
fn private_baseline_completion(
    chart: &FixtureAffineChart,
    context: FixtureRateClockContext,
    ordinal: NonZeroU64,
    span: FixtureRateSpan,
    rate: f64,
) -> TestResult<FixtureBaselineCompletion> {
    let frames = FixtureChartFrames::new(
        chart.frames().target_frame(),
        chart.frames().target_generation(),
        label(31)?,
        label(7)?,
    )?;
    let delivery = product_ok(FixtureSettledRateInterval::new(
        context,
        ordinal,
        span,
        rate,
        frames,
        chart.domain(),
    ))?;
    product_ok(FixtureBaselineCompletion::new(
        delivery,
        product_ok(FixturePhaseBaseline::new(3.0 / 1024.0))?,
        FixtureChartFrames::new(label(31)?, label(7)?, label(37)?, label(9)?)?,
    ))
}

/// Direct derived callback controls complement the executed external constructor.
/// Every changed final descriptor remains individually constructor-valid.
/// # Errors
/// Preserve checked span, completion and individual frame admission failures.
/// # Panics
/// Reject callback omission or erased intermediate/cycle distinctions.
#[test]
fn raw_baseline_completion_validation_checks_intermediate_and_endpoint_chains() -> TestResult {
    let chart = chart()?;
    let span = product_ok(FixtureRateSpan::new(
        (ActiveAwakeNs::new(0), ActiveAwakeNs::new(SECOND)),
        (MonotonicNs::new(4 * SECOND), MonotonicNs::new(6 * SECOND)),
    ))?;
    let valid = private_baseline_completion(&chart, clocks()?, label(1)?, span, 1.0 / 1024.0)?;
    for (tags, cycle) in [
        ([41, 7, 37, 9], false),
        ([31, 11, 37, 9], false),
        ([31, 7, 19, 9], true),
        ([31, 7, 37, 29], true),
    ] {
        assert!(RawBaselineCompletion { value: valid }.validate().is_ok());
        let invalid = FixtureBaselineCompletion {
            final_frames: FixtureChartFrames::new(
                label(tags[0])?,
                label(tags[1])?,
                label(tags[2])?,
                label(tags[3])?,
            )?,
            ..valid
        };
        assert!(RawBaselineCompletion { value: invalid }.validate().is_err());
        let rejected = baseline_completion_rule(&invalid);
        if cycle {
            assert!(matches!(
                rejected,
                Err(FixtureBaselineError::Phase {
                    source: FixturePhaseError::Chart {
                        source: FixtureChartError::InvalidFrames
                    }
                })
            ));
        } else {
            assert!(matches!(rejected, Err(FixtureBaselineError::Receipt)));
        }
        assert!(RawBaselineCompletion { value: valid }.validate().is_ok());
        product_ok(baseline_completion_rule(&valid))?;
    }
    Ok(())
}

/// Change one terminal relationship using individually checked typed operands.
/// The uncertain positive ratio keeps one-nanosecond start/end faults otherwise
/// inside the numeric enclosure; only row8 deliberately violates that enclosure.
/// # Errors
/// Preserve checked labels, spans and complete receipt construction failures.
/// # Panics
/// Reject refusal of the supported typed malformed-neighbor operands.
fn private_terminal_receipt(
    chart: &FixtureAffineChart,
    issued: &BaselineIssued,
    row: u8,
) -> TestResult<FixtureBaselineCompletion> {
    let mut context = issued.started.context;
    let mut ordinal = issued.ordinal;
    let mut rate = issued.old_baseline.rate();
    let (mut active_start, mut active_end, mut mono_start, mut mono_end) =
        (0, SECOND, 4 * SECOND, 6 * SECOND);
    match row {
        0 => {}
        1 => context = FixtureRateClockContext::new(label(201)?, label(103)?, label(107)?),
        2 => ordinal = label(2)?,
        3 => rate = 2.0 / 1024.0,
        4 => active_start = 1,
        5 => mono_start += 1,
        6 => active_end += 1,
        7 => mono_end += 1,
        8 => mono_end = 5 * SECOND,
        _ => return Err(io::Error::other("unknown private terminal row").into()),
    }
    let span = product_ok(FixtureRateSpan::new(
        (
            ActiveAwakeNs::new(active_start),
            ActiveAwakeNs::new(active_end),
        ),
        (MonotonicNs::new(mono_start), MonotonicNs::new(mono_end)),
    ))?;
    private_baseline_completion(chart, context, ordinal, span, rate)
}

/// Check the generated adapter independently before observing its direct rule.
/// This does not call settlement, consume an ordinal or manufacture completion.
/// # Errors
/// Return the exact maintained terminal-relationship refusal.
/// # Panics
/// Reject a derived callback accepting a malformed relationship or refusing its neighbor.
fn check_terminal_adapter(
    issued: &BaselineIssued,
    event: FixtureBaselineFeedback,
    now: FixturePhaseNow,
    accepted: bool,
) -> Result<(), FixtureBaselineError> {
    let raw = RawBaselineTerminal {
        value: BaselineTerminal {
            issued: *issued,
            event,
            now,
        },
    };
    assert_eq!(
        raw.validate().is_ok(),
        accepted,
        "raw terminal callback must enforce its relationship"
    );
    baseline_terminal(&raw.value)
}

/// Known callbacks preserve the real owner's pending slot while operands vary.
/// # Errors
/// Preserve individually checked receipt setup and the healthy direct predicate.
/// # Panics
/// Reject a missing callback, wrong cause, consumed cursor or altered pending owner.
fn check_known_terminal_adapters(
    chart: &FixtureAffineChart,
    issued: &BaselineIssued,
    now: FixturePhaseNow,
    owner: &FixturePhasePlanOwner<'_, '_>,
) -> TestResult {
    let healthy = FixtureBaselineFeedback::Known(private_terminal_receipt(chart, issued, 0)?);
    for row in 1..=8 {
        product_ok(check_terminal_adapter(issued, healthy, now, true))?;
        let malformed =
            FixtureBaselineFeedback::Known(private_terminal_receipt(chart, issued, row)?);
        let rejected = check_terminal_adapter(issued, malformed, now, false);
        if row == 8 {
            assert!(matches!(
                rejected,
                Err(FixtureBaselineError::Numeric {
                    source: FixtureFrequencyNumericError::ActiveRatioMismatch
                })
            ));
        } else {
            assert!(matches!(rejected, Err(FixtureBaselineError::Receipt)));
        }
        product_ok(check_terminal_adapter(issued, healthy, now, true))?;
        assert_eq!(
            owner.baseline_proposal().map(|proposal| proposal.ordinal()),
            Some(label(1)?)
        );
        assert_eq!(owner.cursor().next_ordinal(), Some(label(1)?));
        assert_eq!(owner.cursor().last_active_end(), None);
        assert_eq!(owner.cursor().last_monotonic_end(), None);
    }
    Ok(())
}

/// Non-Known branches validate identity without inventing an endpoint span.
/// # Errors
/// Preserve fixed context labels and healthy direct predicate failures.
/// # Panics
/// Reject missing callbacks or a different refusal family for an identity fault.
fn check_nonknown_terminal_adapters(issued: &BaselineIssued, now: FixturePhaseNow) -> TestResult {
    for unknown in [false, true] {
        for (context, ordinal, accepted) in [
            (clocks()?, label(1)?, true),
            (
                FixtureRateClockContext::new(label(201)?, label(103)?, label(107)?),
                label(1)?,
                false,
            ),
            (clocks()?, label(2)?, false),
        ] {
            let event = if unknown {
                FixtureBaselineFeedback::Unknown { context, ordinal }
            } else {
                FixtureBaselineFeedback::ProvenNoEffect { context, ordinal }
            };
            let result = check_terminal_adapter(issued, event, now, accepted);
            if accepted {
                product_ok(result)?;
            } else {
                assert!(matches!(result, Err(FixtureBaselineError::Receipt)));
            }
        }
    }
    Ok(())
}

/// Genuine issuance supplies pending custody; only private raw operands vary.
/// These checks claim no actual public settlement or physical acknowledgement.
/// # Errors
/// Preserve genuine producer/intake/issue and checked malformed-neighbor setup.
/// # Panics
/// Reject a missing callback, wrong cause, consumed cursor or altered pending owner.
#[test]
fn raw_baseline_terminal_validation_preserves_genuine_pending_custody() -> TestResult {
    let chart = chart()?;
    let mut fixture = producer(&chart)?;
    let initial = FixturePhaseNow::new(
        clocks()?,
        ActiveAwakeNs::new(0),
        MonotonicNs::new(4 * SECOND),
    );
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        initial.monotonic(),
        &fixture.entries,
    ))?;
    let mut owner = frequency_owner_for_unknown(&mut fixture.state, &chart, initial)?;
    assert!(
        matches!(product_ok(owner.receive_reference(&batch, &fixture.strata, initial))?,
        FixturePhaseReferenceAdmission::Accepted { emitted_round } if emitted_round == label(1)?)
    );
    {
        let proposal =
            product_ok(owner.issue_baseline(private_controller_map(&chart, initial)?, initial))?;
        assert_eq!(proposal.ordinal(), label(1)?);
    }
    assert!(
        matches!(owner.issued, Some(PendingOperation::Baseline(_))),
        "actual baseline issue must retain its pending slot"
    );
    let Some(PendingOperation::Baseline(issued)) = owner.issued else {
        return Err(io::Error::other("missing genuinely issued baseline").into());
    };
    let now = FixturePhaseNow::new(
        clocks()?,
        ActiveAwakeNs::new(SECOND),
        MonotonicNs::new(6 * SECOND),
    );
    check_known_terminal_adapters(&chart, &issued, now, &owner)?;
    check_nonknown_terminal_adapters(&issued, now)?;
    assert_eq!(
        owner.baseline_proposal().map(|proposal| proposal.ordinal()),
        Some(label(1)?)
    );
    assert_eq!(owner.cursor().status(), FixtureRateCursorStatus::Pending);
    assert_eq!(owner.cursor().next_ordinal(), Some(label(1)?));
    Ok(())
}

/// Prepare actual source/issue custody for the private ancestor receiver only.
/// # Errors
/// Preserve genuine producer intake, checked map and actual baseline issuance.
/// # Panics
/// Reject a missing accepted emission or wrong pending operation identity.
fn prepared_private_baseline_receiver<'producer, 'chart>(
    fixture: &'producer mut ProducerFixture<'chart>,
    chart: &'chart FixtureAffineChart,
) -> TestResult<FixturePhasePlanOwner<'producer, 'chart>> {
    let initial = FixturePhaseNow::new(
        clocks()?,
        ActiveAwakeNs::new(0),
        MonotonicNs::new(4 * SECOND),
    );
    let batch = product_ok(FixtureChartBatch::new(
        chart,
        initial.monotonic(),
        &fixture.entries,
    ))?;
    let mut owner = frequency_owner_for_unknown(&mut fixture.state, chart, initial)?;
    assert!(
        matches!(product_ok(owner.receive_reference(&batch, &fixture.strata, initial))?,
        FixturePhaseReferenceAdmission::Accepted { emitted_round } if emitted_round == label(1)?)
    );
    {
        let proposal =
            product_ok(owner.issue_baseline(private_controller_map(chart, initial)?, initial))?;
        assert_eq!(proposal.ordinal(), label(1)?);
    }
    Ok(owner)
}

/// Observe direct receiver accounting without claiming parent phase completion.
/// Parent model/proposal/source facts must stay historical at this boundary.
/// # Errors
/// Preserve fixed labels and missing history after its semantic assertion.
/// # Panics
/// Reject early consumption, duplicate consumption or an altered source/model.
fn assert_private_receiver_state(
    owner: &FixturePhasePlanOwner<'_, '_>,
    original: &FixtureFrequencyHistory,
    consumed: bool,
) -> TestResult {
    assert_eq!(
        owner.readiness.readiness(),
        if consumed {
            FixtureReferenceReadiness::CoordinateChanged
        } else {
            FixtureReferenceReadiness::Ready
        }
    );
    assert_eq!(owner.cursor().status(), FixtureRateCursorStatus::Pending);
    assert_eq!(
        owner.cursor().next_ordinal(),
        Some(label(if consumed { 2 } else { 1 })?)
    );
    assert_eq!(
        owner.cursor.target_frame,
        label(if consumed { 37 } else { 19 })?
    );
    assert_eq!(
        owner.cursor.target_generation,
        label(if consumed { 9 } else { 29 })?
    );
    assert_eq!(owner.cursor.context, clocks()?);
    assert_eq!(
        owner.cursor().last_active_end(),
        consumed.then_some(ActiveAwakeNs::new(SECOND))
    );
    assert_eq!(
        owner.cursor().last_monotonic_end(),
        consumed.then_some(MonotonicNs::new(6 * SECOND))
    );
    assert_eq!(owner.current_time().active(), ActiveAwakeNs::new(0));
    assert_eq!(
        owner.current_time().monotonic(),
        MonotonicNs::new(4 * SECOND)
    );
    assert_eq!(owner.current_time().context, clocks()?);
    assert_eq!(owner.producer.emitted_rounds(), 1);
    assert_eq!(
        owner
            .provenance()
            .map(super::FixturePhaseProvenance::emitted_round),
        Some(label(1)?)
    );
    assert_eq!(
        owner.baseline_proposal().map(|proposal| proposal.ordinal()),
        Some(label(1)?)
    );
    assert_eq!(owner.baseline.rate(), 1.0 / 1024.0);
    assert!(owner.frequency_history().is_some());
    let actual = owner
        .frequency_history()
        .ok_or_else(|| io::Error::other("private receiver lost actual source history"))?;
    assert_unknown_frequency_history(&actual, original);
    Ok(())
}

/// Observe every immutable checked interval operand against literal fixture truth.
/// # Errors
/// Preserve exact label and descriptor admission for the independent expectation.
/// # Panics
/// Reject a changed interval, span, context, rate, identity or mathematical domain.
fn assert_private_receiver_input(
    interval: &FixtureSettledRateInterval,
    chart: &FixtureAffineChart,
) -> TestResult {
    assert_eq!(interval.context, clocks()?);
    assert_eq!(interval.ordinal, label(1)?);
    assert_eq!(interval.rate.to_bits(), (1.0_f64 / 1024.0).to_bits());
    assert_eq!(
        interval.span.active,
        (ActiveAwakeNs::new(0), ActiveAwakeNs::new(SECOND))
    );
    assert_eq!(
        interval.span.effective,
        (MonotonicNs::new(4 * SECOND), MonotonicNs::new(6 * SECOND))
    );
    assert_eq!(interval.span.active_elapsed.nanoseconds(), SECOND);
    assert_eq!(
        interval.frames,
        FixtureChartFrames::new(label(19)?, label(29)?, label(31)?, label(7)?)?
    );
    assert_eq!(interval.domain, chart.domain());
    Ok(())
}

/// The returned result owns only the intermediate constant-rate composition.
/// The parent has not yet performed its separate final affine/model feedback.
/// # Errors
/// Preserve the real replacement's bounded arithmetic or missing-result cause.
/// # Panics
/// Reject an invented final chart or an absent admitted constant completion.
fn assert_private_constant_completion(completion: &FixtureRateSettlementCompletion) -> TestResult {
    assert_eq!(completion.kind(), FixtureRateSettlementKind::Completed);
    let replacement = product_ok(completion.replacement())?;
    assert!(
        replacement.is_some(),
        "actual private completion must retain its intermediate replacement"
    );
    let replacement =
        replacement.ok_or_else(|| io::Error::other("missing actual intermediate replacement"))?;
    assert_eq!(
        replacement.frames(),
        FixtureChartFrames::new(label(17)?, label(23)?, label(31)?, label(7)?)?
    );
    assert_eq!(replacement.phase_at_reference_seconds(), 1.0 / 1024.0);
    assert_eq!(replacement.rate_seconds_per_second(), 0.0);
    Ok(())
}

/// Each checked mismatched component must reject at the actual private handler.
/// # Errors
/// Preserve the independent frame labels and unchanged-state observations.
/// # Panics
/// Reject a later admission, wrong error or mutation before complete-chain validation.
fn check_private_chain_rejections(
    owner: &mut FixturePhasePlanOwner<'_, '_>,
    chart: &FixtureAffineChart,
    interval: &FixtureSettledRateInterval,
    original: &FixtureFrequencyHistory,
) -> TestResult {
    for (frame, generation) in [(41, 7), (31, 11)] {
        let wrong =
            FixtureChartFrames::new(label(frame)?, label(generation)?, label(37)?, label(9)?)?;
        let rejected =
            owner
                .readiness
                .receive_baseline_settlement(&mut owner.cursor, chart, *interval, wrong);
        assert!(
            matches!(
                rejected,
                Err(FixtureRateDeliveryError::Chart {
                    source: FixtureChartError::CompositionMappingMismatch
                })
            ),
            "one mismatched chain component must reject before admission"
        );
        assert_private_receiver_state(owner, original, false)?;
        assert_private_receiver_input(interval, chart)?;
        assert_eq!(wrong.source_frame(), label(frame)?);
        assert_eq!(wrong.source_generation(), label(generation)?);
        assert_eq!(wrong.target_frame(), label(37)?);
        assert_eq!(wrong.target_generation(), label(9)?);
    }
    Ok(())
}

/// Exercise the private handler's full input contract beyond public receipt filtering.
/// Direct accounting is observed; no public phase/frequency completion is claimed.
/// # Errors
/// Preserve genuinely admitted source/issue state and individually checked operands.
/// # Panics
/// Reject either one-component chain mismatch consuming state or a duplicate commit.
#[test]
fn private_baseline_receiver_rejects_each_chain_component_before_admission() -> TestResult {
    let chart = chart()?;
    let mut fixture = producer(&chart)?;
    let mut owner = prepared_private_baseline_receiver(&mut fixture, &chart)?;
    assert!(owner.frequency_history().is_some());
    let original = owner
        .frequency_history()
        .ok_or_else(|| io::Error::other("genuine private receiver has no model"))?;
    let span = product_ok(FixtureRateSpan::new(
        (ActiveAwakeNs::new(0), ActiveAwakeNs::new(SECOND)),
        (MonotonicNs::new(4 * SECOND), MonotonicNs::new(6 * SECOND)),
    ))?;
    let frames = FixtureChartFrames::new(label(19)?, label(29)?, label(31)?, label(7)?)?;
    let interval = product_ok(FixtureSettledRateInterval::new(
        clocks()?,
        label(1)?,
        span,
        1.0 / 1024.0,
        frames,
        chart.domain(),
    ))?;
    assert_private_receiver_state(&owner, &original, false)?;
    check_private_chain_rejections(&mut owner, &chart, &interval, &original)?;
    let final_frames = FixtureChartFrames::new(label(31)?, label(7)?, label(37)?, label(9)?)?;
    let completion = product_ok(owner.readiness.receive_baseline_settlement(
        &mut owner.cursor,
        &chart,
        interval,
        final_frames,
    ))?;
    assert_private_constant_completion(&completion)?;
    assert_private_receiver_state(&owner, &original, true)?;
    assert_private_receiver_input(&interval, &chart)?;
    let repeated = owner.readiness.receive_baseline_settlement(
        &mut owner.cursor,
        &chart,
        interval,
        final_frames,
    );
    assert!(matches!(
        repeated,
        Err(FixtureRateDeliveryError::Reference {
            source: FixtureReferenceInputError::Unavailable {
                readiness: FixtureReferenceReadiness::CoordinateChanged
            }
        })
    ));
    assert_private_receiver_state(&owner, &original, true)?;
    assert_private_receiver_input(&interval, &chart)?;
    Ok(())
}

/// The public error contract exposes the original typed nested cause, not a copy
/// or a formatted replacement. Leaf errors have no invented cause.
/// # Panics
/// Reject a missing, substituted or fabricated error source.
#[test]
fn baseline_errors_expose_each_original_nested_cause_and_leaf_neighbors() {
    let phase = FixtureBaselineError::Phase {
        source: FixturePhaseError::CursorUnavailable,
    };
    let actual = phase
        .source()
        .and_then(|cause| cause.downcast_ref::<FixturePhaseError>());
    assert!(matches!((&phase, actual),
        (FixtureBaselineError::Phase { source }, Some(actual)) if std::ptr::eq(source, actual)));
    let frequency = FixtureBaselineError::Frequency {
        source: crate::FixtureFrequencyError::Expired,
    };
    let actual = frequency
        .source()
        .and_then(|cause| cause.downcast_ref::<crate::FixtureFrequencyError>());
    assert!(matches!((&frequency, actual),
        (FixtureBaselineError::Frequency { source }, Some(actual)) if std::ptr::eq(source, actual)));
    let numeric = FixtureBaselineError::Numeric {
        source: FixtureFrequencyNumericError::ActiveRatioMismatch,
    };
    let actual = numeric
        .source()
        .and_then(|cause| cause.downcast_ref::<FixtureFrequencyNumericError>());
    assert!(matches!((&numeric, actual),
        (FixtureBaselineError::Numeric { source }, Some(actual)) if std::ptr::eq(source, actual)));
    for leaf in [
        FixtureBaselineError::Map,
        FixtureBaselineError::Receipt,
        FixtureBaselineError::ReusedEmission,
    ] {
        assert!(leaf.source().is_none());
    }
}

/// Declare literal whole-second fixture readings; u32 seconds fit u64 nanoseconds.
/// # Errors
/// Preserve the independently checked nonzero context labels.
fn controller_clock_point(active: u32, monotonic: u32) -> TestResult<FixturePhaseNow> {
    Ok(FixturePhaseNow::new(
        clocks()?,
        ActiveAwakeNs::new(u64::from(active) * SECOND),
        MonotonicNs::new(u64::from(monotonic) * SECOND),
    ))
}

/// The map's declared nonunit coefficient and the chart's inclusive end are
/// independently specified literals, not values inferred from a proposal.
/// # Errors
/// Preserve actual checked chart, baseline and mandatory map construction.
/// # Panics
/// Reject a changed getter, excluded inclusive end or missing validation callback.
#[test]
fn checked_controller_map_keeps_nonunit_ratio_and_inclusive_chart_end() -> TestResult {
    let chart = product_ok(chart())?;
    assert_eq!(chart.domain().end(), MonotonicNs::new(64 * SECOND));
    let map = product_ok(FixtureControllerClockMap::new(
        &chart,
        product_ok(FixturePhaseBaseline::new(1.0 / 1024.0))?,
        FixtureControllerClockMapInput {
            reference: controller_clock_point(0, 4)?,
            valid_until: MonotonicNs::new(64 * SECOND),
            active_ratio: 0.5,
            active_ratio_error: 1.0 / 8.0,
            actual_phase: 0.0,
            actual_phase_error: 0.0,
            actual_rate: 0.0,
            actual_rate_error: 0.0,
        },
    ))?;
    assert_eq!(map.active_ratio().to_bits(), 0.5_f64.to_bits());
    assert!(
        RawControllerMap {
            value: (&chart, &map)
        }
        .validate()
        .is_ok()
    );
    product_ok(controller_map_rule(&chart, &map))?;
    assert!(map.observe(controller_clock_point(30, 64)?).is_ok());
    Ok(())
}

/// Direct helper inputs retain their independent context/order contract even
/// when current public callers have earlier guards. No physical map is issued.
/// # Errors
/// Preserve checked fixture clocks and their distinct context labels.
/// # Panics
/// Reject an admitted context mismatch, regression or lost healthy observation.
#[test]
fn private_clock_relation_rejects_each_order_fault_before_subtraction() -> TestResult {
    let relation = ControllerClockRelation {
        reference: controller_clock_point(10, 10)?,
        witness: None,
    };
    let healthy = controller_clock_point(20, 20)?;
    assert!(relation.observe(healthy, 1.0, 0.0).is_ok());
    let wrong_context = FixturePhaseNow::new(
        FixtureRateClockContext::new(label(109)?, label(103)?, label(107)?),
        healthy.active(),
        healthy.monotonic(),
    );
    assert!(matches!(
        relation.observe(wrong_context, 1.0, 0.0),
        Err(FixtureBaselineError::Map)
    ));
    for wrong in [
        controller_clock_point(9, 11)?,
        controller_clock_point(11, 9)?,
    ] {
        // Preserve the default panic hook. A removed ordering guard can panic in
        // mutant subtraction; the independent assertion still checks the exact
        // promised return, rather than crediting that auxiliary production panic.
        let returned = std::panic::catch_unwind(|| relation.observe(wrong, 1.0, 0.0));
        assert!(
            matches!(returned, Ok(Err(FixtureBaselineError::Map))),
            "clock regression must return Map before subtraction"
        );
        assert!(relation.observe(healthy, 1.0, 0.0).is_ok());
    }
    Ok(())
}

/// These are checked-map private consumer inputs, with isolated wrong context
/// and an otherwise affine observation just outside the declared domain.
/// # Errors
/// Preserve actual checked map and distinct context admission failures.
/// # Panics
/// Reject either bypassed guard or loss of a healthy in-domain observation.
#[test]
fn private_controller_map_observe_rechecks_context_and_expiry() -> TestResult {
    let chart = product_ok(chart())?;
    let map = product_ok(private_controller_map(
        &chart,
        controller_clock_point(10, 10)?,
    ))?;
    let healthy = controller_clock_point(20, 30)?;
    assert!(map.observe(healthy).is_ok());
    let wrong_context = FixturePhaseNow::new(
        FixtureRateClockContext::new(label(109)?, label(103)?, label(107)?),
        healthy.active(),
        healthy.monotonic(),
    );
    assert!(matches!(
        map.observe(wrong_context),
        Err(FixtureBaselineError::Map)
    ));
    // (22-10)/(34-10)=1/2, so only the named M=32 domain end refuses it.
    assert!(matches!(
        map.observe(controller_clock_point(22, 34)?),
        Err(FixtureBaselineError::Map)
    ));
    assert!(map.observe(healthy).is_ok());
    Ok(())
}

/// Inspect the actual base `NoEffect` return before later issuance can repair an
/// omitted expiry latch. Each row has one fixed affine ratio in [3/8,5/8].
/// # Errors
/// Preserve genuine source, pending base, checked clocks and original model state.
/// # Panics
/// Reject wrong latching, changed source/model facts or invented finite delivery.
fn baseline_no_effect_deadline_case(active: u32, monotonic: u32, expired: bool) -> TestResult {
    let chart = product_ok(chart())?;
    let mut fixture = product_ok(producer(&chart))?;
    let owner = product_ok(prepared_private_baseline_receiver(&mut fixture, &chart))?;
    let error = owner.remaining_phase_error_seconds();
    let before = product_ok(
        owner
            .frequency_history()
            .ok_or_else(|| io::Error::other("missing genuine model")),
    )?;
    let current = controller_clock_point(active, monotonic)?;
    let result = owner.settle_baseline(
        FixtureBaselineFeedback::ProvenNoEffect {
            context: clocks()?,
            ordinal: label(1)?,
        },
        current,
    );
    assert!(matches!(
        &result,
        FixtureBaselineSettlement::ProvenNoEffect(_)
    ));
    let FixtureBaselineSettlement::ProvenNoEffect(returned) = result else {
        return Err(io::Error::other("missing actual base NoEffect return").into());
    };
    inspect_no_effect(&returned, &chart, error, current, expired)?;
    assert_eq!(
        returned.baseline.rate.to_bits(),
        (1.0_f64 / 1024.0).to_bits()
    );
    assert_eq!(returned.producer.emitted_rounds(), 1);
    let after = product_ok(
        returned
            .frequency_history()
            .ok_or_else(|| io::Error::other("lost genuine model")),
    )?;
    assert_unknown_frequency_history(&after, &before);
    assert!(returned.baseline_proposal().is_none());
    Ok(())
}

/// The original active cutoff is A=8 and monotonic cutoff M=20. These four
/// literal affine rows cover neither, active only, monotonic only and both.
/// # Errors
/// Preserve actual fixture, source and base-terminal admission failures.
/// # Panics
/// Reject loss of either independent expiry condition or unconditional expiry.
#[test]
fn baseline_no_effect_latches_each_original_deadline_before_reissue() -> TestResult {
    for (active, monotonic, expired) in
        [(7, 18, false), (8, 18, true), (7, 20, true), (8, 20, true)]
    {
        baseline_no_effect_deadline_case(active, monotonic, expired)?;
    }
    Ok(())
}

/// Observe the actual untouched pending slot after read-only helper calls.
/// # Errors
/// Preserve the fixed context/ordinal labels and checked original clock readings.
/// # Panics
/// Reject accidental consumption, invented endpoints or changed relation custody.
fn assert_pending_base_without_ratio(owner: &FixturePhasePlanOwner<'_, '_>) -> TestResult {
    assert_eq!(
        owner.baseline_proposal().map(|proposal| proposal.ordinal()),
        Some(label(1)?)
    );
    assert_eq!(owner.cursor().status(), FixtureRateCursorStatus::Pending);
    assert_eq!(owner.cursor().next_ordinal(), Some(label(1)?));
    assert_eq!(owner.cursor().last_active_end(), None);
    assert_eq!(owner.cursor().last_monotonic_end(), None);
    assert_eq!(owner.current_time().active(), ActiveAwakeNs::new(0));
    assert_eq!(
        owner.current_time().monotonic(),
        MonotonicNs::new(4 * SECOND)
    );
    assert!(matches!(owner.clock_relation, Some(relation) if relation.witness.is_none()));
    Ok(())
}

/// Known and `NoEffect` terminals can establish the first nonzero clock witness.
/// These direct helper observations consume no ordinal and create no clock effect.
/// # Errors
/// Preserve actual source/issue state and individually checked terminal operands.
/// # Panics
/// Reject missing returned relation evidence or mutation of the actual pending slot.
#[test]
fn baseline_terminal_relation_retains_first_known_and_no_effect_observations() -> TestResult {
    let chart = product_ok(chart())?;
    let mut fixture = product_ok(producer(&chart))?;
    let owner = product_ok(prepared_private_baseline_receiver(&mut fixture, &chart))?;
    assert_pending_base_without_ratio(&owner)?;
    let Some(PendingOperation::Baseline(issued)) = owner.issued else {
        return Err(io::Error::other("missing genuine pending base").into());
    };
    let span = product_ok(FixtureRateSpan::new(
        (ActiveAwakeNs::new(0), ActiveAwakeNs::new(SECOND)),
        (MonotonicNs::new(4 * SECOND), MonotonicNs::new(6 * SECOND)),
    ))?;
    let known = product_ok(private_baseline_completion(
        &chart,
        clocks()?,
        label(1)?,
        span,
        1.0 / 1024.0,
    ))?;
    let now = controller_clock_point(1, 6)?;
    for event in [
        FixtureBaselineFeedback::Known(known),
        FixtureBaselineFeedback::ProvenNoEffect {
            context: clocks()?,
            ordinal: label(1)?,
        },
    ] {
        let observed = product_ok(owner.baseline_terminal_relation(&issued, event, now))?;
        assert!(
            matches!(observed, Some(relation) if relation.witness.is_some_and(|witness|
            witness.active == SECOND && witness.monotonic == 2 * SECOND)),
            "terminal must retain the first exact 1/2 ratio witness"
        );
        assert_pending_base_without_ratio(&owner)?;
    }
    Ok(())
}

/// A late `NoEffect` observation lies outside the admitted affine segment and
/// cannot supply extrapolated clock evidence, even if its pair contradicts it.
/// # Errors
/// Preserve actual source/issue state, checked clocks and the same pending identity.
/// # Panics
/// Reject an extrapolated refusal, lost ordinal or fabricated ratio/model evidence.
#[test]
fn baseline_no_effect_after_map_expiry_consumes_without_extrapolating_ratio() -> TestResult {
    let chart = product_ok(chart())?;
    let mut fixture = product_ok(producer(&chart))?;
    let owner = product_ok(prepared_private_baseline_receiver(&mut fixture, &chart))?;
    assert_pending_base_without_ratio(&owner)?;
    let error = owner.remaining_phase_error_seconds();
    let before = product_ok(
        owner
            .frequency_history()
            .ok_or_else(|| io::Error::other("missing genuine model")),
    )?;
    let Some(PendingOperation::Baseline(issued)) = owner.issued else {
        return Err(io::Error::other("missing genuine pending base").into());
    };
    let event = FixtureBaselineFeedback::ProvenNoEffect {
        context: clocks()?,
        ordinal: label(1)?,
    };
    // M=34 exceeds map end32; 7/(34-4)=7/30 is deliberately outside [3/8,5/8].
    let now = controller_clock_point(7, 34)?;
    let observed = product_ok(owner.baseline_terminal_relation(&issued, event, now))?;
    assert!(matches!(observed, Some(relation) if relation.witness.is_none()));
    assert_pending_base_without_ratio(&owner)?;
    let result = owner.settle_baseline(event, now);
    assert!(matches!(
        &result,
        FixtureBaselineSettlement::ProvenNoEffect(_)
    ));
    let FixtureBaselineSettlement::ProvenNoEffect(returned) = result else {
        return Err(io::Error::other("late NoEffect must retain actual consumption").into());
    };
    inspect_no_effect(&returned, &chart, error, now, true)?;
    assert!(matches!(returned.clock_relation, Some(relation) if relation.witness.is_none()));
    let after = product_ok(
        returned
            .frequency_history()
            .ok_or_else(|| io::Error::other("lost genuine model")),
    )?;
    assert_unknown_frequency_history(&after, &before);
    assert_eq!(returned.producer.emitted_rounds(), 1);
    Ok(())
}

/// Inspect a real matched achieved baseline independently of its strict cap gate.
/// The model/map remain available while out-of-envelope achievement stays known.
/// # Errors
/// Preserve genuine source/issue state and checked actual completion operands.
/// # Panics
/// Reject cap equality refusal, missing over-cap refusal or erased achieved facts.
fn baseline_achieved_cap_case(rate: f64, refused: bool) -> TestResult {
    let chart = product_ok(chart())?;
    let mut fixture = product_ok(producer(&chart))?;
    let owner = product_ok(prepared_private_baseline_receiver(&mut fixture, &chart))?;
    let span = product_ok(FixtureRateSpan::new(
        (ActiveAwakeNs::new(0), ActiveAwakeNs::new(SECOND)),
        (MonotonicNs::new(4 * SECOND), MonotonicNs::new(6 * SECOND)),
    ))?;
    let original = product_ok(private_baseline_completion(
        &chart,
        clocks()?,
        label(1)?,
        span,
        1.0 / 1024.0,
    ))?;
    let receipt = product_ok(FixtureBaselineCompletion::new(
        original.delivery,
        product_ok(FixturePhaseBaseline::new(rate))?,
        original.final_frames,
    ))?;
    let result = owner.settle_baseline(
        FixtureBaselineFeedback::Known(receipt),
        controller_clock_point(1, 6)?,
    );
    assert!(matches!(&result, FixtureBaselineSettlement::Known(_)));
    let FixtureBaselineSettlement::Known(retired) = result else {
        return Err(io::Error::other("missing actual known base retirement").into());
    };
    assert_eq!(retired.achieved_baseline().rate().to_bits(), rate.to_bits());
    assert!(retired.feedback_error().is_none());
    assert!(retired.clock_map().is_ok());
    assert!(retired.replacement().is_ok());
    assert!(retired.frequency_history().is_some());
    let cursor = product_ok(
        retired
            .cursor()
            .ok_or_else(|| io::Error::other("lost actual cursor")),
    )?;
    assert_eq!(cursor.next_ordinal(), Some(label(2)?));
    assert_eq!(cursor.last_active_end(), Some(ActiveAwakeNs::new(SECOND)));
    assert_eq!(
        cursor.last_monotonic_end(),
        Some(MonotonicNs::new(6 * SECOND))
    );
    if refused {
        assert!(matches!(
            retired.continuation_error(),
            Some(FixturePhaseError::DeliveredOutOfEnvelope)
        ));
    } else {
        assert!(
            retired.continuation_error().is_none(),
            "inclusive achieved cap permits continuation"
        );
    }
    Ok(())
}

/// Positive and negative exact caps are legal, with literal below/above neighbors.
/// # Errors
/// Preserve actual fixture, source and known-terminal construction failures.
/// # Panics
/// Reject a non-strict cap, asymmetric signed cap or lost achieved knowledge.
#[test]
fn known_baseline_cap_is_inclusive_for_both_signs_without_losing_achievement() -> TestResult {
    for (rate, refused) in [
        (1.0 / 128.0, false),
        (-1.0 / 128.0, false),
        (1.0 / 64.0, false),
        (-1.0 / 64.0, false),
        (1.0 / 32.0, true),
        (-1.0 / 32.0, true),
    ] {
        baseline_achieved_cap_case(rate, refused)?;
    }
    Ok(())
}

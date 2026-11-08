// SPDX-License-Identifier: GPL-2.0-only
//! Private expiry-invariant observations after genuine modeled `NoEffect` reception.
//!
//! Actual checked histories, producer emissions and owning lifecycle operations
//! precede the private observation. Clocks are explicit fixture assertions; no
//! native completion, physical clock or fabricated owner/debt is supplied.

use std::{error::Error, fmt, io, num::NonZeroU64};

use time_domain::{ActiveAwakeNs, MonotonicNs, UtcCoordinateQ32, exchange_arithmetic};

use super::{
    FixturePhaseBaseline, FixturePhaseError, FixturePhaseNow, FixturePhasePlanOwner,
    FixturePhaseProfile, FixturePhaseReferenceAdmission, FixturePhaseResidualPermission,
    FixturePhaseSettlement,
};
use crate::{
    FixtureAdaptiveHistory, FixtureAffineChart, FixtureChartBatch, FixtureChartDomain,
    FixtureChartFrames, FixtureChartInterval, FixtureChartOutcome, FixtureCombinationParameters,
    FixtureCombinationState, FixtureContext, FixtureMinimum, FixtureObservation,
    FixturePrincipalParameters, FixturePrincipalStratum, FixtureProjectedBatch,
    FixtureProjectionTerms, FixtureRateClock, FixtureRateClockContext, FixtureRateCursorStatus,
    FixtureRateSettlement, FixtureRateSettlementCursor, enclose_fixture_chart_projection,
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

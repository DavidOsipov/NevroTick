// SPDX-License-Identifier: GPL-2.0-only
//! Matched-step arithmetic-failure custody and exact terminal admission controls.
//!
//! Every owner comes from genuine fitted source histories. Large numeric inputs
//! are explicit model assertions, not physical rates or native clock effects.

use std::io;

use time_core::{
    FixtureChartFrames, FixturePhaseError, FixturePhaseNow, FixturePhaseNumericError,
    FixturePhaseNumericStep, FixturePhasePlanOwner, FixturePhaseResidualPermission,
    FixtureRateClock, FixtureRateCursorStatus, FixtureRateSettlement, FixtureRateSettlementCursor,
    FixtureStepCompletion, FixtureStepDirections, FixtureStepError, FixtureStepFeedback,
    FixtureStepRetirement, FixtureStepSettlement,
};
use time_domain::{ActiveAwakeNs, MonotonicNs};

use super::{
    SECOND, TestResult, clocks, cursor_snapshot, interval, label, now, product_ok, step_limits,
    step_receipt, with_step_owner,
};

/// Observe one committed identity and original interval through its public custody.
/// # Errors
/// Preserve nonzero fixture-label construction failures.
/// # Panics
/// Detect an erased actual step, unconsumed ordinal, missing interval or lost endpoints.
fn check_consumed_rate_case(retired: &FixtureStepRetirement<'_>) -> TestResult {
    assert_eq!(retired.ordinal(), label(1)?);
    assert_eq!(retired.achieved_step_seconds(), 1.0 / 32.0);
    assert!(matches!(
        retired.event(),
        FixtureRateSettlement::Completed(_)
    ));
    assert_eq!(
        retired.completion().final_frames(),
        FixtureChartFrames::new(label(31)?, label(37)?, label(41)?, label(43)?)?
    );
    assert!(
        retired.cursor().is_some(),
        "known interval lost its committed cursor"
    );
    let cursor = retired
        .cursor()
        .ok_or_else(|| io::Error::other("known arithmetic result lost cursor custody"))?;
    assert_eq!(cursor.status(), FixtureRateCursorStatus::Pending);
    assert_eq!(cursor.next_ordinal(), Some(label(2)?));
    assert_eq!(
        cursor.last_active_end(),
        Some(ActiveAwakeNs::new(2 * SECOND))
    );
    assert_eq!(
        cursor.last_monotonic_end(),
        Some(MonotonicNs::new(8 * SECOND))
    );
    Ok(())
}

/// Inspect numerical results independently of the already consumed operation.
/// # Errors
/// Preserve checked retained chart access failures on the finite control.
/// # Panics
/// Detect erased partial results or mistaking a policy refusal for arithmetic failure.
fn check_rate_arithmetic(retired: &FixtureStepRetirement<'_>, overflow: bool) -> TestResult {
    if overflow {
        assert!(retired.rate_replacement().is_err());
        assert!(retired.replacement().is_err());
        assert!(retired.feedback_error().is_some());
        assert_eq!(retired.pre_step_phase_seconds(), None);
        assert_eq!(retired.remaining_phase_seconds(), None);
        assert_eq!(retired.pre_step_phase_error_seconds(), None);
        assert_eq!(retired.remaining_phase_error_seconds(), None);
        assert!(matches!(
            retired.debt_error(),
            Some(FixturePhaseError::Numeric {
                source: FixturePhaseNumericError::Arithmetic {
                    step: FixturePhaseNumericStep::AggregateDelivery,
                    ..
                }
            })
        ));
    } else {
        // Binary64 MAX/2 is exact. Adding or subtracting the small fixed
        // debt, baseline and step cannot change that rounded point.
        let half_max = f64::from_bits(0x7fdf_ffff_ffff_ffff);
        assert_eq!(
            retired.replacement()?.phase_at_reference_seconds(),
            half_max
        );
        assert!(retired.rate_replacement()?.is_some());
        assert_eq!(retired.pre_step_phase_seconds(), Some(-half_max));
        assert_eq!(retired.remaining_phase_seconds(), Some(-half_max));
        assert!(
            retired
                .remaining_phase_error_seconds()
                .is_some_and(f64::is_finite)
        );
        assert!(retired.feedback_error().is_none());
        assert!(retired.debt_error().is_none());
    }
    Ok(())
}

/// Distinguish actual rate overflow from its finite quarter-magnitude neighbor.
/// # Errors
/// Preserve actual source, request, receipt and checked chart access failures.
/// # Panics
/// Detect losing Known custody or confusing a policy refusal with arithmetic failure.
fn known_rate_case(actual: f64, overflow: bool) -> TestResult {
    with_step_owner(1, |mut owner| {
        product_ok(owner.issue_step(
            now(0, 4 * SECOND)?,
            FixturePhaseResidualPermission::Allow,
            step_limits(FixtureStepDirections::Both)?,
        ))?;
        let receipt = step_receipt(&owner, 1, actual, 1.0 / 32.0)?;
        let result = owner.settle_step(
            FixtureStepFeedback::Known(receipt),
            now(2 * SECOND, 8 * SECOND)?,
        );
        assert!(matches!(&result, FixtureStepSettlement::Known(_)));
        let FixtureStepSettlement::Known(mut retired) = result else {
            return Err(io::Error::other("actual rate arithmetic lost Known settlement").into());
        };
        check_consumed_rate_case(&retired)?;
        check_rate_arithmetic(&retired, overflow)?;
        assert!(matches!(
            retired.continuation_error(),
            Some(FixtureStepError::Phase {
                source: FixturePhaseError::BaselineChanged
            })
        ));
        assert!(retired.resume_fresh(now(2 * SECOND, 8 * SECOND)?).is_err());
        check_consumed_rate_case(&retired)
    })
}

/// Unsupported actual integration consumes before failure; a finite neighbor still computes.
/// # Errors
/// Preserve both actual owning trajectories and checked fixture setup failures.
#[test]
fn matched_step_rate_overflow_retains_known_effect_beside_finite_neighbor() -> TestResult {
    known_rate_case(f64::MAX, true)?;
    known_rate_case(f64::from_bits(0x7fcf_ffff_ffff_ffff), false)
}

/// Retain genuine numerical bounds, including the independently rounded step subtraction.
/// # Errors
/// Preserve genuine source, explicit request, receipt and historical-bound failures.
/// # Panics
/// Reject fabricated zero/unit bounds, lost partial debt or erased Known custody.
#[test]
fn matched_step_error_getters_retain_positive_bounded_numerical_evidence() -> TestResult {
    with_step_owner(1, |mut owner| {
        let original = owner.remaining_phase_error_seconds();
        assert!(original.is_some_and(|error| error > 0.0 && error < 1.0 / 8.0));
        let original =
            original.ok_or_else(|| io::Error::other("source lost its numerical bound"))?;
        let proposal = product_ok(owner.issue_step(
            now(0, 4 * SECOND)?,
            FixturePhaseResidualPermission::Allow,
            step_limits(FixtureStepDirections::Both)?,
        ))?;
        assert_eq!(proposal.requested_phase_seconds(), 1.0 / 8.0);
        // Stored binary64 0.1, scaled exactly by eight. The exact dyadic
        // subtraction from 1/8 differs from its rounded point by exactly
        // 2^-58 seconds, independently of inherited numerical uncertainty.
        let achieved = f64::from_bits(0x3fb9_9999_9999_999a) / 8.0;
        let receipt = product_ok(step_receipt(&owner, 1, 1.0 / 32.0, achieved))?;
        let result = owner.settle_step(
            FixtureStepFeedback::Known(receipt),
            now(2 * SECOND, 8 * SECOND)?,
        );
        assert!(matches!(&result, FixtureStepSettlement::Known(_)));
        let FixtureStepSettlement::Known(retired) = result else {
            return Err(io::Error::other("bounded numerical case lost Known custody").into());
        };
        assert_eq!(
            retired.achieved_step_seconds().to_bits(),
            achieved.to_bits()
        );
        assert_eq!(retired.pre_step_phase_seconds(), Some(1.0 / 8.0));
        assert!(
            retired
                .pre_step_phase_error_seconds()
                .is_some_and(|error| error.is_finite() && error >= original && error < 1.0 / 4.0)
        );
        assert_eq!(
            retired.remaining_phase_seconds(),
            Some(f64::from_bits(0x3fec_cccc_cccc_cccd) / 8.0)
        );
        let subtraction_error = f64::from_bits(0x3c50_0000_0000_0000);
        assert!(
            retired
                .remaining_phase_error_seconds()
                .is_some_and(|error| error.is_finite()
                    && error >= original
                    && error >= subtraction_error
                    && error < 1.0 / 4.0)
        );
        assert!(retired.debt_error().is_none());
        assert!(retired.cursor().is_some());
        assert_eq!(
            retired
                .cursor()
                .and_then(FixtureRateSettlementCursor::next_ordinal),
            Some(label(2)?)
        );
        Ok(())
    })
}

/// Construct an independently selected actual interval on the pending source chart.
/// # Errors
/// Preserve typed span, frame-chain and known-step admission failures.
fn terminal_receipt(
    owner: &FixturePhasePlanOwner<'_, '_>,
    ordinal: u64,
    active: (u64, u64),
    monotonic: (u64, u64),
) -> TestResult<FixtureStepCompletion> {
    let delivery = interval(
        owner.chart(),
        clocks()?,
        ordinal,
        active,
        monotonic,
        1.0 / 32.0,
        (31, 37),
    )?;
    product_ok(FixtureStepCompletion::new(
        delivery,
        1.0 / 32.0,
        FixtureChartFrames::new(label(31)?, label(37)?, label(41)?, label(43)?)?,
    ))
}

/// Return the same pending owner only after observing its unchanged public state.
/// # Errors
/// Reject a consuming result instead of reconstructing an owner after refusal.
/// # Panics
/// Detect wrong refusal meaning, premature consumption, altered time/chart or lost debt.
fn reject_unchanged<'producer, 'chart>(
    owner: FixturePhasePlanOwner<'producer, 'chart>,
    receipt: FixtureStepCompletion,
    current: FixturePhaseNow,
    expected: &FixtureStepError,
) -> TestResult<FixturePhasePlanOwner<'producer, 'chart>> {
    let cursor = cursor_snapshot(owner.cursor());
    let time = owner.current_time();
    let debt = (
        owner.remaining_phase_seconds(),
        owner.remaining_phase_error_seconds(),
    );
    let chart = (
        owner.chart().frames(),
        owner.chart().phase_at_reference_seconds().to_bits(),
    );
    let ordinal = owner.step_proposal().map(|value| value.ordinal());
    let result = owner.settle_step(FixtureStepFeedback::Known(receipt), current);
    assert!(matches!(
        &result,
        FixtureStepSettlement::OuterRejected { .. }
    ));
    let FixtureStepSettlement::OuterRejected { owner, error } = result else {
        return Err(io::Error::other("terminal guard consumed the original step owner").into());
    };
    assert_eq!(&error, expected);
    assert_eq!(cursor_snapshot(owner.cursor()), cursor);
    assert_eq!(owner.current_time().active(), time.active());
    assert_eq!(owner.current_time().monotonic(), time.monotonic());
    assert_eq!(
        (
            owner.remaining_phase_seconds(),
            owner.remaining_phase_error_seconds()
        ),
        debt
    );
    assert_eq!(
        (
            owner.chart().frames(),
            owner.chart().phase_at_reference_seconds().to_bits()
        ),
        chart
    );
    assert_eq!(owner.step_proposal().map(|value| value.ordinal()), ordinal);
    Ok(owner)
}

/// A legal one-active/two-monotonic-second neighbor commits actual independent facts.
/// # Errors
/// Preserve actual receipt, checked chart access and expected ordinal construction.
/// # Panics
/// Detect blanket rejection after an invalid neighbor or accounting requested displacement.
fn valid_guard_neighbor(owner: FixturePhasePlanOwner<'_, '_>, ordinal: u64) -> TestResult {
    let next = ordinal
        .checked_add(1)
        .ok_or_else(|| io::Error::other("guard neighbor ordinal overflow"))?;
    let receipt = terminal_receipt(
        &owner,
        ordinal,
        (SECOND, 2 * SECOND),
        (6 * SECOND, 8 * SECOND),
    )?;
    let FixtureStepSettlement::Known(retired) = owner.settle_step(
        FixtureStepFeedback::Known(receipt),
        now(2 * SECOND, 8 * SECOND)?,
    ) else {
        return Err(io::Error::other("legal terminal neighbor failed after guard refusal").into());
    };
    assert_eq!(
        retired.replacement()?.phase_at_reference_seconds(),
        1.0 / 16.0
    );
    assert_eq!(retired.remaining_phase_seconds(), Some(3.0 / 32.0));
    assert_eq!(
        retired
            .cursor()
            .and_then(FixtureRateSettlementCursor::next_ordinal),
        Some(label(next)?)
    );
    Ok(())
}

/// Independently selected actual endpoints, current readings and expected refusal.
type TerminalCase = ((u64, u64), (u64, u64), (u64, u64), FixtureStepError);

/// Change one issued-start or current upper-end relationship at a time.
fn span_guard_cases() -> [TerminalCase; 4] {
    [
        (
            (SECOND + 1, 2 * SECOND),
            (6 * SECOND, 8 * SECOND),
            (2 * SECOND, 8 * SECOND),
            FixtureStepError::Receipt,
        ),
        (
            (SECOND, 2 * SECOND),
            (6 * SECOND + 1, 8 * SECOND),
            (2 * SECOND, 8 * SECOND),
            FixtureStepError::Receipt,
        ),
        (
            (SECOND, 2 * SECOND + 1),
            (6 * SECOND, 8 * SECOND),
            (2 * SECOND, 8 * SECOND),
            FixtureStepError::Receipt,
        ),
        (
            (SECOND, 2 * SECOND),
            (6 * SECOND, 8 * SECOND + 1),
            (2 * SECOND, 8 * SECOND),
            FixtureStepError::Receipt,
        ),
    ]
}

/// Regress one independently ordered current clock below the actual issue reading.
fn current_guard_cases() -> [TerminalCase; 2] {
    [
        (
            (SECOND, 2 * SECOND),
            (6 * SECOND, 8 * SECOND),
            (SECOND - 1, 8 * SECOND),
            FixtureStepError::Phase {
                source: FixturePhaseError::TimeRegression {
                    clock: FixtureRateClock::ActiveAwake,
                },
            },
        ),
        (
            (SECOND, 2 * SECOND),
            (6 * SECOND, 8 * SECOND),
            (2 * SECOND, 6 * SECOND - 1),
            FixtureStepError::Phase {
                source: FixturePhaseError::TimeRegression {
                    clock: FixtureRateClock::Monotonic,
                },
            },
        ),
    ]
}

/// Independent start/end/read-now guards reject without consuming the pending step.
/// # Errors
/// Preserve genuine source, typed intervals and the healthy terminal neighbor.
#[test]
fn matched_step_terminal_span_and_current_readings_preserve_pending_owner() -> TestResult {
    with_step_owner(1, |mut owner| {
        product_ok(owner.issue_step(
            now(SECOND, 6 * SECOND)?,
            FixturePhaseResidualPermission::Allow,
            step_limits(FixtureStepDirections::Both)?,
        ))?;
        for (active, monotonic, current, error) in
            span_guard_cases().into_iter().chain(current_guard_cases())
        {
            let receipt = terminal_receipt(&owner, 1, active, monotonic)?;
            owner = reject_unchanged(owner, receipt, now(current.0, current.1)?, &error)?;
        }
        valid_guard_neighbor(owner, 1)
    })
}

/// A lower consumed ordinal cannot replace the next genuine pending step.
/// # Errors
/// Preserve actual `NoEffect` consumption, repeated issuance and neighboring completion.
/// # Panics
/// Detect replay acceptance or loss of the next operation's original identity.
#[test]
fn matched_step_lower_consumed_ordinal_rejects_beside_current_completion() -> TestResult {
    with_step_owner(1, |mut owner| {
        let current = now(SECOND, 6 * SECOND)?;
        product_ok(owner.issue_step(
            current,
            FixturePhaseResidualPermission::Allow,
            step_limits(FixtureStepDirections::Both)?,
        ))?;
        let FixtureStepSettlement::ProvenNoEffect(mut owner) = owner.settle_step(
            FixtureStepFeedback::ProvenNoEffect {
                context: clocks()?,
                ordinal: label(1)?,
            },
            current,
        ) else {
            return Err(io::Error::other("NoEffect did not consume the first real step").into());
        };
        let proposal = product_ok(owner.issue_step(
            current,
            FixturePhaseResidualPermission::Allow,
            step_limits(FixtureStepDirections::Both)?,
        ))?;
        assert_eq!(proposal.ordinal(), label(2)?);
        let replay = terminal_receipt(&owner, 1, (SECOND, 2 * SECOND), (6 * SECOND, 8 * SECOND))?;
        let owner = reject_unchanged(
            owner,
            replay,
            now(2 * SECOND, 8 * SECOND)?,
            &FixtureStepError::Receipt,
        )?;
        valid_guard_neighbor(owner, 2)
    })
}

/// A genuine neutral fitted phase cannot reserve a useful signed step.
/// # Errors
/// Preserve neutral and healthy actual source construction and request failures.
/// # Panics
/// Detect zero-phase issuance, consumed credit or a blanket no-step implementation.
#[test]
fn matched_step_neutral_reference_refuses_beside_sign_certain_neighbor() -> TestResult {
    with_step_owner(0, |mut owner| {
        let before = cursor_snapshot(owner.cursor());
        let debt = (
            owner.remaining_phase_seconds(),
            owner.remaining_phase_error_seconds(),
        );
        assert_eq!(debt.0, Some(0.0));
        assert_eq!(
            owner
                .issue_step(
                    now(0, 4 * SECOND)?,
                    FixturePhaseResidualPermission::Allow,
                    step_limits(FixtureStepDirections::Both)?,
                )
                .err(),
            Some(FixtureStepError::NoUsefulStep)
        );
        assert_eq!(cursor_snapshot(owner.cursor()), before);
        assert_eq!(
            (
                owner.remaining_phase_seconds(),
                owner.remaining_phase_error_seconds()
            ),
            debt
        );
        assert!(owner.step_proposal().is_none());
        Ok(())
    })?;
    with_step_owner(1, |mut owner| {
        let proposal = product_ok(owner.issue_step(
            now(0, 4 * SECOND)?,
            FixturePhaseResidualPermission::Allow,
            step_limits(FixtureStepDirections::Both)?,
        ))?;
        assert_eq!(proposal.ordinal(), label(1)?);
        assert_eq!(proposal.requested_phase_seconds(), 1.0 / 8.0);
        Ok(())
    })
}

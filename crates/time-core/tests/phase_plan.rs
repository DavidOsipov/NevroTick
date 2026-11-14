// SPDX-License-Identifier: GPL-2.0-only
//! Independent finite phase-obligation journeys over genuine fitted histories.
//!
//! All clocks, rates and phase values below are explicit fixture inputs. These
//! tests establish no production source eligibility, physical baseline, native
//! delivery, durable broker ledger or shipping correction policy. Root owns
//! execution; no expected value is obtained from the phase-plan implementation.
#![forbid(unsafe_code)]

use std::{error::Error, fmt, io, num::NonZeroU64};

use time_core::{
    FixtureAdaptiveHistory, FixtureAdaptiveHorizonError, FixtureAdaptiveHorizonProfile,
    FixtureAffineChart, FixtureChartBatch, FixtureChartCombinationError, FixtureChartDomain,
    FixtureChartError, FixtureChartFrames, FixtureChartInterval, FixtureChartOutcome,
    FixtureChartPrincipalError, FixtureCombinationParameters, FixtureCombinationState,
    FixtureContext, FixtureMinimum, FixtureObservation, FixturePhaseBaseline,
    FixturePhaseConstructionRejected, FixturePhaseError, FixturePhaseNow, FixturePhaseNumericError,
    FixturePhasePlanOwner, FixturePhaseProfile, FixturePhaseProposal, FixturePhaseProvenance,
    FixturePhaseReferenceAdmission, FixturePhaseResidualPermission, FixturePhaseRetirement,
    FixturePhaseRetirementReason, FixturePhaseSettlement, FixturePrincipalError,
    FixturePrincipalParameters, FixturePrincipalStratum, FixtureProjectedBatch,
    FixtureProjectionTerms, FixtureRateClock, FixtureRateClockContext, FixtureRateCursorStatus,
    FixtureRateDeliveryError, FixtureRateSettlement, FixtureRateSettlementCursor, FixtureRateSpan,
    FixtureReferenceInputError, FixtureReferenceInputState, FixtureSettledRateInterval,
    FixtureStepCompletion, FixtureStepDirections, FixtureStepError, FixtureStepFeedback,
    FixtureStepLimits, FixtureStepSettlement, enclose_fixture_chart_projection,
    enclose_fixture_projection,
};
use time_domain::{ActiveAwakeNs, MonotonicNs, UtcCoordinateQ32, exchange_arithmetic};

/// Independent negative controls for the same actual matched-step owner.
#[path = "private/step_contracts.rs"]
mod step_contracts;

/// Qualification controls exercise actual post-commit failures and terminal guards.
#[path = "private/step_qualification.rs"]
mod step_qualification;

#[path = "private/step_diagnostics.rs"]
mod step_diagnostics;

/// Lease genuine source history and first reference for one bounded step journey.
/// No synthetic completion or combined estimate is created by setup.
/// # Errors
/// Preserve actual history, chart, profile, producer and admission failures.
/// # Panics
/// Detect rewriting original source rows or fitted diagnostics during the journey.
fn with_step_owner(
    sign: i8,
    action: impl for<'producer, 'chart> FnOnce(FixturePhasePlanOwner<'producer, 'chart>) -> TestResult,
) -> TestResult {
    with_step_owner_policy(sign, 1.0 / 4.0, f64::from(sign) / 32.0, action)
}

/// Admit explicit point-cap and baseline neighbors through the genuine source journey.
/// The source rows supply `sign / 8` seconds, including the zero-debt control.
/// # Errors
/// Preserve checked profile, baseline, history, chart and reference admission failures.
/// # Panics
/// Detect mutation of the original source rows while the actual owner is leased.
fn with_step_owner_policy(
    sign: i8,
    phase_cap: f64,
    baseline: f64,
    action: impl for<'producer, 'chart> FnOnce(FixturePhasePlanOwner<'producer, 'chart>) -> TestResult,
) -> TestResult {
    if !matches!(sign, -1..=1) {
        return Err(
            io::Error::other("step fixture sign must be negative, zero or positive unit").into(),
        );
    }
    let a = Source::new(11, 0, i128::from(sign) * (1_i128 << 29))?;
    let b = Source::new(22, 0, i128::from(sign) * (1_i128 << 29))?;
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let explicit = product_ok(FixturePhaseProfile::new(
        adaptive_duration(8 * SECOND)?,
        MonotonicNs::new(16 * SECOND).duration_since(MonotonicNs::new(0))?,
        phase_cap,
        1.0 / 4.0,
        1.0 / 2.0,
    ))?;
    let raw_before = [source_snapshot(&a)?, source_snapshot(&b)?];
    let owner = start_with_profile(&mut producer, &chart, &batch, &strata, explicit, baseline)?;
    action(owner)?;
    assert_eq!([source_snapshot(&a)?, source_snapshot(&b)?], raw_before);
    Ok(())
}

/// Admit one explicit per-operation fixture limit without selecting a product policy.
/// # Errors
/// Preserve the checked magnitude/direction constructor's actual refusal.
fn step_limits(directions: FixtureStepDirections) -> TestResult<FixtureStepLimits> {
    product_ok(FixtureStepLimits::new(1.0 / 4.0, directions))
}

/// Construct independent actual delivery on the same pending operation's chart.
/// The interval has two active and four monotonic seconds; the step stays separate.
/// # Errors
/// Preserve checked interval, frame-chain, scalar and completion construction.
fn step_receipt(
    owner: &FixturePhasePlanOwner<'_, '_>,
    ordinal: u64,
    aggregate: f64,
    actual_step: f64,
) -> TestResult<FixtureStepCompletion> {
    let delivery = interval(
        owner.chart(),
        clocks()?,
        ordinal,
        (0, 2 * SECOND),
        (4 * SECOND, 8 * SECOND),
        aggregate,
        (31, 37),
    )?;
    product_ok(FixtureStepCompletion::new(
        delivery,
        actual_step,
        FixtureChartFrames::new(label(31)?, label(37)?, label(41)?, label(43)?)?,
    ))
}

/// Actual aggregate delivery and actual step retain distinct signs and units.
/// # Errors
/// Preserve genuine producer, request, checked completion and take-once recovery.
/// # Panics
/// Detect requested-as-applied correction, wrong elapsed clock or reused old debt.
#[test]
fn matched_step_accounts_actual_effect_and_requires_fresh_evidence_for_both_signs() -> TestResult {
    for sign in [-1_i8, 1] {
        with_step_owner(sign, |mut owner| {
            let signed = f64::from(sign);
            let before = owner
                .provenance()
                .map(FixturePhaseProvenance::emitted_round);
            let proposal = product_ok(owner.issue_step(
                now(0, 4 * SECOND)?,
                FixturePhaseResidualPermission::Allow,
                step_limits(FixtureStepDirections::Both)?,
            ))?;
            assert_eq!(proposal.ordinal(), label(1)?);
            assert_eq!(proposal.requested_phase_seconds(), signed / 8.0);
            assert_eq!(proposal.known_baseline().rate(), signed / 32.0);
            assert_eq!(proposal.active_deadline(), ActiveAwakeNs::new(8 * SECOND));
            assert_eq!(proposal.monotonic_deadline(), MonotonicNs::new(20 * SECOND));
            assert!(matches!(
                owner.issue(now(0, 4 * SECOND)?, FixturePhaseResidualPermission::Allow),
                Err(FixturePhaseError::OutstandingOperation)
            ));
            let actual = step_receipt(&owner, 1, signed / 32.0, signed / 32.0)?;
            let result = owner.settle_step(
                FixtureStepFeedback::Known(actual),
                now(2 * SECOND, 8 * SECOND)?,
            );
            let FixtureStepSettlement::Known(mut retired) = result else {
                return Err(io::Error::other("matched step must retain Known completion").into());
            };
            assert_eq!(retired.requested_phase_seconds(), signed / 8.0);
            assert_eq!(retired.achieved_step_seconds(), signed / 32.0);
            assert_eq!(retired.pre_step_phase_seconds(), Some(signed / 8.0));
            assert_eq!(retired.remaining_phase_seconds(), Some(signed * 3.0 / 32.0));
            assert_eq!(
                retired.replacement()?.phase_at_reference_seconds(),
                signed * 3.0 / 32.0
            );
            assert!(retired.feedback_error().is_none());
            assert!(retired.continuation_error().is_none());
            assert!(retired.debt_error().is_none());
            let mut fresh = product_ok(retired.resume_fresh(now(2 * SECOND, 8 * SECOND)?))?;
            assert_eq!(fresh.remaining_phase_seconds(), None);
            assert_eq!(fresh.cursor().next_ordinal(), Some(label(2)?));
            assert_eq!(
                fresh.cursor().last_active_end(),
                Some(ActiveAwakeNs::new(2 * SECOND))
            );
            assert_eq!(
                fresh.cursor().last_monotonic_end(),
                Some(MonotonicNs::new(8 * SECOND))
            );
            assert_eq!(fresh.chart().frames().target_frame(), label(41)?);
            assert_eq!(fresh.chart().frames().target_generation(), label(43)?);
            assert!(matches!(
                fresh.issue(
                    now(2 * SECOND, 8 * SECOND)?,
                    FixturePhaseResidualPermission::Allow
                ),
                Err(FixturePhaseError::NoReference)
            ));
            assert!(matches!(
                fresh.issue_step(
                    now(2 * SECOND, 8 * SECOND)?,
                    FixturePhaseResidualPermission::Allow,
                    step_limits(FixtureStepDirections::Both)?
                ),
                Err(FixtureStepError::Phase {
                    source: FixturePhaseError::NoReference
                })
            ));
            assert_eq!(before, Some(label(1)?));
            Ok(())
        })?;
    }
    Ok(())
}

/// An unexpected actual aggregate still changes both chart and carried debt.
/// # Errors
/// Preserve actual request, completion and historical-chart failures.
/// # Panics
/// Detect using only the step or erasing known effects on baseline disagreement.
#[test]
fn matched_step_accounts_unexpected_actual_rate_before_refusing_continuation() -> TestResult {
    with_step_owner(1, |mut owner| {
        product_ok(owner.issue_step(
            now(0, 4 * SECOND)?,
            FixturePhaseResidualPermission::Allow,
            step_limits(FixtureStepDirections::Both)?,
        ))?;
        let actual = step_receipt(&owner, 1, 3.0 / 64.0, 1.0 / 32.0)?;
        let result = owner.settle_step(
            FixtureStepFeedback::Known(actual),
            now(2 * SECOND, 8 * SECOND)?,
        );
        let FixtureStepSettlement::Known(mut retired) = result else {
            return Err(io::Error::other("unexpected rate remains a Known step fact").into());
        };
        assert_eq!(retired.pre_step_phase_seconds(), Some(3.0 / 32.0));
        assert_eq!(retired.remaining_phase_seconds(), Some(1.0 / 16.0));
        assert_eq!(
            retired.replacement()?.phase_at_reference_seconds(),
            1.0 / 8.0
        );
        assert_eq!(retired.achieved_step_seconds(), 1.0 / 32.0);
        assert!(matches!(
            retired.continuation_error(),
            Some(FixtureStepError::Phase {
                source: FixturePhaseError::BaselineChanged
            })
        ));
        assert_eq!(
            retired
                .cursor()
                .and_then(FixtureRateSettlementCursor::next_ordinal),
            Some(label(2)?)
        );
        assert!(retired.resume_fresh(now(2 * SECOND, 8 * SECOND)?).is_err());
        Ok(())
    })
}

/// A known zero step still accounts elapsed baseline and consumes its new target.
/// # Errors
/// Preserve actual request and original completion/frame construction failures.
/// # Panics
/// Detect collapsing known zero into unchanged-state feedback.
#[test]
fn matched_known_zero_step_accounts_elapsed_baseline_and_final_target() -> TestResult {
    with_step_owner(1, |mut owner| {
        product_ok(owner.issue_step(
            now(0, 4 * SECOND)?,
            FixturePhaseResidualPermission::Allow,
            step_limits(FixtureStepDirections::Positive)?,
        ))?;
        let actual = step_receipt(&owner, 1, 1.0 / 32.0, -0.0)?;
        let result = owner.settle_step(
            FixtureStepFeedback::Known(actual),
            now(2 * SECOND, 8 * SECOND)?,
        );
        let FixtureStepSettlement::Known(mut retired) = result else {
            return Err(io::Error::other("zero-valued Known step must retain its effects").into());
        };
        assert_eq!(
            retired.achieved_step_seconds().to_bits(),
            (-0.0_f64).to_bits()
        );
        assert_eq!(retired.remaining_phase_seconds(), Some(1.0 / 8.0));
        assert_eq!(
            retired.replacement()?.phase_at_reference_seconds(),
            1.0 / 16.0
        );
        assert_eq!(retired.replacement()?.frames().target_frame(), label(41)?);
        let fresh = product_ok(retired.resume_fresh(now(2 * SECOND, 8 * SECOND)?))?;
        assert_eq!(fresh.remaining_phase_seconds(), None);
        assert_eq!(fresh.cursor().next_ordinal(), Some(label(2)?));
        Ok(())
    })
}

/// Stop forbids new useful work while preserving a matching historical step.
/// # Errors
/// Preserve actual issuance, original receipt and checked current-clock failures.
/// # Panics
/// Detect erased known delivery or restored authority after stop.
#[test]
fn matched_step_after_stop_preserves_historical_effects_without_recovery() -> TestResult {
    with_step_owner(1, |mut owner| {
        product_ok(owner.issue_step(
            now(0, 4 * SECOND)?,
            FixturePhaseResidualPermission::Allow,
            step_limits(FixtureStepDirections::Both)?,
        ))?;
        let actual = step_receipt(&owner, 1, 1.0 / 32.0, 1.0 / 32.0)?;
        assert!(owner.step_proposal().is_some());
        owner.retire(FixturePhaseRetirementReason::Stop);
        assert!(owner.step_proposal().is_none());
        let result = owner.settle_step(
            FixtureStepFeedback::Known(actual),
            now(2 * SECOND, 8 * SECOND)?,
        );
        let FixtureStepSettlement::Known(mut retired) = result else {
            return Err(io::Error::other("stop must preserve matched known completion").into());
        };
        assert_eq!(retired.remaining_phase_seconds(), Some(3.0 / 32.0));
        assert_eq!(
            retired.replacement()?.phase_at_reference_seconds(),
            3.0 / 32.0
        );
        assert!(matches!(
            retired.continuation_error(),
            Some(FixtureStepError::Phase {
                source: FixturePhaseError::Retired {
                    reason: FixturePhaseRetirementReason::Stop
                }
            })
        ));
        assert!(retired.resume_fresh(now(2 * SECOND, 8 * SECOND)?).is_err());
        Ok(())
    })
}

/// A known step drops old live debt while retaining the original accepted interval.
///
/// New raw source mean is 1/8; the actual step/rate chart displacement is 3/32,
/// so fresh debt is 1/32. The independent SD factor remains k=sqrt(5)+2sqrt(3).
/// Eight active seconds since Accepted give area k/4 and a time constant 8*k.
/// Resetting the stamp at Step A=2 instead would produce 6*k; clearing it gives
/// the first-reference minimum. Neither is the admitted rule.
/// # Errors
/// Preserve genuine source, reference, step, chart and fresh-admission failures.
/// # Panics
/// Detect reused live debt/area, a renewed accepted stamp or requested-as-delivered data.
#[test]
fn adaptive_step_fresh_reference_uses_original_accepted_interval() -> TestResult {
    let a = Source::new(11, 0, 1_i128 << 29)?;
    let b = Source::new(22, 0, 1_i128 << 29)?;
    let next_a = Source::new(11, 4, 1_i128 << 29)?;
    let next_b = Source::new(22, 4, 1_i128 << 29)?;
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let mut owner = start_with_profile(
        &mut producer,
        &chart,
        &batch,
        &strata,
        adaptive_profile(32, 64, 0.0)?,
        1.0 / 32.0,
    )?;
    product_ok(owner.issue_step(
        now(0, 4 * SECOND)?,
        FixturePhaseResidualPermission::Allow,
        step_limits(FixtureStepDirections::Both)?,
    ))?;
    let actual = step_receipt(&owner, 1, 1.0 / 32.0, 1.0 / 32.0)?;
    let result = owner.settle_step(
        FixtureStepFeedback::Known(actual),
        now(2 * SECOND, 8 * SECOND)?,
    );
    let FixtureStepSettlement::Known(mut retired) = result else {
        return Err(io::Error::other("adaptive step must retain actual Known feedback").into());
    };
    assert_eq!(retired.remaining_phase_seconds(), Some(3.0 / 32.0));
    let mut fresh = product_ok(retired.resume_fresh(now(2 * SECOND, 8 * SECOND)?))?;
    assert_eq!(fresh.remaining_phase_seconds(), None);
    assert!(matches!(
        fresh.issue(
            now(2 * SECOND, 8 * SECOND)?,
            FixturePhaseResidualPermission::Allow
        ),
        Err(FixturePhaseError::NoReference)
    ));
    receive_serialized_refresh(
        &mut fresh,
        &a,
        &next_a,
        &next_b,
        &strata,
        (8 * SECOND, 20 * SECOND),
    )?;
    assert_eq!(fresh.remaining_phase_seconds(), Some(1.0 / 32.0));
    let proposal = product_ok(fresh.issue(
        now(8 * SECOND, 20 * SECOND)?,
        FixturePhaseResidualPermission::Allow,
    ))?;
    let k = 5.0_f64.sqrt() + 2.0 * 3.0_f64.sqrt();
    check_adaptive_horizon(&proposal, 8.0 * k)?;
    assert_eq!(
        proposal.requested_active_duration().nanoseconds(),
        32 * SECOND
    );
    assert_eq!(proposal.active_deadline(), ActiveAwakeNs::new(40 * SECOND));
    Ok(())
}

/// Consume real serialized source updates whose admitted active interval is unsupported.
/// # Errors
/// Preserve actual batch, source, clock and reference admission failures.
/// # Panics
/// Detect rejected source credit being restored or an unsupported interval becoming a plan.
fn receive_adaptive_interval_refusal(
    owner: &mut FixturePhasePlanOwner<'_, '_>,
    old_a: &Source,
    new_a: &Source,
    new_b: &Source,
    projected_second: u64,
    readings: (u64, u64),
    expected_round: u64,
) -> TestResult {
    let strata = strata()?;
    let pending = [
        old_a.entry(owner.chart(), projected_second)?,
        new_b.entry(owner.chart(), projected_second)?,
    ];
    let pending_batch = product_ok(FixtureChartBatch::new(
        owner.chart(),
        MonotonicNs::new(projected_second * SECOND),
        &pending,
    ))?;
    assert!(matches!(
        product_ok(owner.receive_reference(&pending_batch, &strata, now(readings.0, readings.1)?))?,
        FixturePhaseReferenceAdmission::AdmittedNoPlan {
            emitted_round: None,
            reason: FixturePhaseError::Reference {
                source: FixtureReferenceInputError::NotReady
            }
        }
    ));
    let entries = [
        new_a.entry(owner.chart(), projected_second)?,
        new_b.entry(owner.chart(), projected_second)?,
    ];
    let batch = product_ok(FixtureChartBatch::new(
        owner.chart(),
        MonotonicNs::new(projected_second * SECOND),
        &entries,
    ))?;
    assert!(matches!(
        product_ok(owner.receive_reference(&batch, &strata, now(readings.0, readings.1)?))?,
        FixturePhaseReferenceAdmission::AdmittedNoPlan { emitted_round: Some(round), reason: FixturePhaseError::AdaptiveHorizon { source: FixtureAdaptiveHorizonError::UnsupportedInterval } } if round == label(expected_round)?
    ));
    assert_eq!(owner.remaining_phase_seconds(), None);
    assert_eq!(
        owner
            .provenance()
            .map(FixturePhaseProvenance::emitted_round),
        Some(label(expected_round)?)
    );
    Ok(())
}

/// A genuine consumed-but-unplanned reference cannot advance the accepted-area stamp.
///
/// These are explicit independent clock-domain fixture readings, with no affine
/// or physical clock-mapping claim. The unsupported active delta exceeds 2^53 ns.
/// A second fresh row pair only one active second later must still be refused:
/// updating the accepted stamp on the first refusal would incorrectly admit it.
/// # Errors
/// Preserve genuine histories, producer, checked profiles and reference operations.
/// # Panics
/// Detect restored source credit, renewal of the stamp or a fabricated live obligation.
#[test]
fn adaptive_admitted_no_plan_preserves_accepted_stamp_across_fresh_rows() -> TestResult {
    let a = Source::new(11, 0, 1_i128 << 29)?;
    let b = Source::new(22, 0, 1_i128 << 29)?;
    let next_a = Source::new(11, 4, 1_i128 << 29)?;
    let next_b = Source::new(22, 4, 1_i128 << 29)?;
    let third_a = Source::new(11, 8, 1_i128 << 29)?;
    let third_b = Source::new(22, 8, 1_i128 << 29)?;
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let mut owner = start_with_profile(
        &mut producer,
        &chart,
        &batch,
        &strata,
        adaptive_profile(32, 64, 0.0)?,
        0.0,
    )?;
    let unsupported = (1_u64 << 53) + 1;
    receive_adaptive_interval_refusal(
        &mut owner,
        &a,
        &next_a,
        &next_b,
        8,
        (unsupported, 20 * SECOND),
        2,
    )?;
    receive_adaptive_interval_refusal(
        &mut owner,
        &next_a,
        &third_a,
        &third_b,
        12,
        (unsupported + SECOND, 24 * SECOND),
        3,
    )?;
    assert!(matches!(
        owner.issue(
            now(unsupported + SECOND, 24 * SECOND)?,
            FixturePhaseResidualPermission::Allow
        ),
        Err(FixturePhaseError::NoReference)
    ));
    Ok(())
}

/// Construct an exact active duration without assigning any monotonic meaning.
/// # Errors
/// Preserve the domain's checked duration conversion.
fn adaptive_duration(nanoseconds: u64) -> TestResult<time_domain::ActiveAwakeDurationNs> {
    Ok(ActiveAwakeNs::new(nanoseconds).duration_since(ActiveAwakeNs::new(0))?)
}

/// Explicit numerical controller parameters; no shipped policy is selected.
/// # Errors
/// Preserve the same checked public construction used by callers.
fn adaptive_profile(
    active_seconds: u64,
    maximum_seconds: u64,
    small_debt: f64,
) -> TestResult<FixturePhaseProfile> {
    let active = active_seconds
        .checked_mul(SECOND)
        .ok_or_else(|| io::Error::other("adaptive fixture duration overflow"))?;
    let policy = product_ok(FixtureAdaptiveHorizonProfile::new(
        4.0,
        adaptive_duration(SECOND)?,
        adaptive_duration(
            maximum_seconds
                .checked_mul(SECOND)
                .ok_or_else(|| io::Error::other("adaptive fixture maximum overflow"))?,
        )?,
        small_debt,
    ))?;
    let profile = product_ok(FixturePhaseProfile::new(
        adaptive_duration(active)?,
        MonotonicNs::new(64 * SECOND).duration_since(MonotonicNs::new(0))?,
        1.0 / 4.0,
        1.0 / 4.0,
        1.0 / 2.0,
    ))?;
    product_ok(profile.with_adaptive_horizon(policy))
}

/// Lease the actual producer and install only its genuine first reference.
/// # Errors
/// Preserve public construction and source/clock/reference admission failures.
/// # Panics
/// Reject an obligation without the independently expected first emission.
fn start_with_profile<'producer, 'chart>(
    producer: &'producer mut FixtureCombinationState,
    chart: &'chart FixtureAffineChart,
    batch: &FixtureChartBatch<'_, 'chart>,
    strata: &[FixturePrincipalStratum; 2],
    profile: FixturePhaseProfile,
    base: f64,
) -> TestResult<FixturePhasePlanOwner<'producer, 'chart>> {
    let mut owner = product_ok(FixturePhasePlanOwner::new(
        producer,
        chart,
        FixtureRateSettlementCursor::new(chart, clocks()?, label(1)?),
        now(0, 4 * SECOND)?,
        profile,
        baseline(base)?,
    ))?;
    assert!(matches!(
        product_ok(owner.receive_reference(batch, strata, now(0, 4 * SECOND)?))?,
        FixturePhaseReferenceAdmission::Accepted { emitted_round } if emitted_round == label(1)?
    ));
    Ok(owner)
}

/// Raw caller profiles must reject malformed values before any controller exists.
/// # Errors
/// Preserve the explicit valid duration and profile checks.
/// # Panics
/// Reject a malformed ratio, threshold or duration interval that bypasses admission.
#[test]
fn adaptive_horizon_profile_rejects_raw_bypasses_and_keeps_explicit_zero_threshold() -> TestResult {
    let minimum = adaptive_duration(SECOND)?;
    let maximum = adaptive_duration(16 * SECOND)?;
    for ratio in [0.0, -0.0, -1.0, f64::INFINITY, f64::NAN, f64::from_bits(1)] {
        assert!(FixtureAdaptiveHorizonProfile::new(ratio, minimum, maximum, 0.0).is_err());
    }
    for threshold in [-1.0, f64::INFINITY, f64::NAN] {
        assert!(FixtureAdaptiveHorizonProfile::new(4.0, minimum, maximum, threshold).is_err());
    }
    for (lower, upper) in [(0, SECOND), (2 * SECOND, SECOND), (1, (1_u64 << 53) + 1)] {
        assert!(
            FixtureAdaptiveHorizonProfile::new(
                4.0,
                adaptive_duration(lower)?,
                adaptive_duration(upper)?,
                0.0,
            )
            .is_err()
        );
    }
    for threshold in [0.0, -0.0, f64::from_bits(1), 1.0 / 4.0] {
        let policy = product_ok(FixtureAdaptiveHorizonProfile::new(
            4.0, minimum, maximum, threshold,
        ))?;
        product_ok(profile()?.with_adaptive_horizon(policy))?;
    }
    Ok(())
}

/// Compare a typed controller horizon with the independent analytic seconds.
/// # Errors
/// Reject a whole/fraction part outside this finite test's integer representation.
/// # Panics
/// Detect more than two nanoseconds of arithmetic/ceiling discrepancy.
fn check_adaptive_horizon(proposal: &FixturePhaseProposal<'_>, expected: f64) -> TestResult {
    let nanos = proposal.correction_horizon().nanoseconds();
    let whole = u32::try_from(nanos / SECOND)?;
    let fraction = u32::try_from(nanos % SECOND)?;
    let actual = f64::from(whole) + f64::from(fraction) / 1_000_000_000.0;
    assert!(
        (actual - expected).abs() <= 2.0e-9,
        "adaptive horizon: {actual} versus {expected}"
    );
    Ok(())
}

/// Issue the first genuine reference with zero adaptive area and original limits.
/// # Errors
/// Preserve the actual pending-operation and checked clock refusals.
/// # Panics
/// Detect a fabricated first interval or changed original permission deadline.
fn check_initial_adaptive_issue(owner: &mut FixturePhasePlanOwner<'_, '_>) -> TestResult {
    let first =
        product_ok(owner.issue(now(0, 4 * SECOND)?, FixturePhaseResidualPermission::Allow))?;
    assert_eq!(first.correction_horizon().nanoseconds(), SECOND);
    assert_eq!(first.requested_active_duration().nanoseconds(), SECOND);
    assert_eq!(
        first.requested_active_deadline(),
        ActiveAwakeNs::new(SECOND)
    );
    assert_eq!(first.transient_rate(), 1.0 / 8.0);
    assert_eq!(first.active_deadline(), ActiveAwakeNs::new(32 * SECOND));
    Ok(())
}

/// A real fresh-reference interval uses active time and survives intervening `NoEffect`.
///
/// Two independent three-row fits have residuals [c,-2c,c], c=1/64 second.
/// Residual scale is 6c²; newest-time phase SE is sqrt(5)c and rate SE is sqrt(3)c.
/// Projection by two seconds gives SD k*c, k=sqrt(5)+2sqrt(3). Identical-source
/// phase-variance averaging retains that SD. Ratio 4 and eight active seconds give
/// area k/4; phase 1/8 gives horizon 2k.
/// A fixed 2-ns allowance covers upward ns quantization and far smaller binary64
/// reduction error; it is not a physical timing or statistical confidence bound.
/// # Errors
/// Preserve actual history/selection/proposal/settlement and continuation failures.
/// # Panics
/// Detect using monotonic elapsed time, `last_now`, requested delivery or expiry as T.
#[test]
fn adaptive_horizon_tracks_real_reference_and_actual_partial_delivery() -> TestResult {
    let a = Source::new(11, 0, 1_i128 << 29)?;
    let b = Source::new(22, 0, 1_i128 << 29)?;
    let next_a = Source::new(11, 4, 1_i128 << 29)?;
    let next_b = Source::new(22, 4, 1_i128 << 29)?;
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let mut owner = start_with_profile(
        &mut producer,
        &chart,
        &batch,
        &strata,
        adaptive_profile(32, 64, 0.0)?,
        1.0 / 128.0,
    )?;
    check_initial_adaptive_issue(&mut owner)?;
    let mut owner = no_effect(owner, 1, now(SECOND, 6 * SECOND)?)?;

    // Fresh intake has eight active seconds but sixteen monotonic seconds.
    // Neither the intervening NoEffect nor the helper's NotReady intake may
    // shorten that accepted-obligation interval.
    receive_serialized_refresh(
        &mut owner,
        &a,
        &next_a,
        &next_b,
        &strata,
        (8 * SECOND, 20 * SECOND),
    )?;
    let proposal = product_ok(owner.issue(
        now(8 * SECOND, 20 * SECOND)?,
        FixturePhaseResidualPermission::Allow,
    ))?;
    // Independent OLS/projection equations; no product SD or horizon helper
    // supplies the expected value.
    let k = 5.0_f64.sqrt() + 2.0 * 3.0_f64.sqrt();
    check_adaptive_horizon(&proposal, 2.0 * k)?;
    assert!((proposal.transient_rate() - 1.0 / (16.0 * k)).abs() <= 1.0e-10);
    assert_eq!(proposal.active_deadline(), ActiveAwakeNs::new(40 * SECOND));
    assert_eq!(proposal.monotonic_deadline(), MonotonicNs::new(84 * SECOND));

    // The independent modeled actuator returns the exact aggregate 9/128;
    // no requested rate is recycled as evidence of actual completion.
    let mut retired = settle_interval(
        owner,
        2,
        ((8 * SECOND, 9 * SECOND), (20 * SECOND, 22 * SECOND)),
        9.0 / 128.0,
        (31, 37),
        true,
        1.0 / 128.0,
    )?;
    assert_eq!(retired.last_known_phase_seconds(), Some(1.0 / 16.0));
    let mut owner = product_ok(retired.resume(
        now(9 * SECOND, 22 * SECOND)?,
        FixturePhaseResidualPermission::Allow,
    ))?;
    let proposal = product_ok(owner.issue(
        now(9 * SECOND, 22 * SECOND)?,
        FixturePhaseResidualPermission::Allow,
    ))?;
    check_adaptive_horizon(&proposal, 4.0 * k)?;
    assert!((proposal.transient_rate() - 1.0 / (64.0 * k)).abs() <= 1.0e-10);
    assert!((proposal.aggregate_rate() - (1.0 / 128.0 + 1.0 / (64.0 * k))).abs() <= 1.0e-10);
    let mut owner = no_effect(owner, 3, now(38 * SECOND, 60 * SECOND)?)?;
    let proposal = product_ok(owner.issue(
        now(38 * SECOND, 60 * SECOND)?,
        FixturePhaseResidualPermission::Allow,
    ))?;
    check_adaptive_horizon(&proposal, 4.0 * k)?;
    assert_eq!(
        proposal.requested_active_duration().nanoseconds(),
        2 * SECOND
    );
    assert_eq!(
        proposal.requested_active_deadline(),
        ActiveAwakeNs::new(40 * SECOND)
    );
    assert!((proposal.transient_rate() - 1.0 / (64.0 * k)).abs() <= 1.0e-10);
    let mut retired = settle_interval(
        owner,
        4,
        ((38 * SECOND, 40 * SECOND), (60 * SECOND, 64 * SECOND)),
        3.0 / 128.0,
        (41, 43),
        false,
        1.0 / 128.0,
    )?;
    assert_eq!(retired.last_known_phase_seconds(), Some(1.0 / 32.0));
    assert!(matches!(
        retired.resume(
            now(40 * SECOND, 64 * SECOND)?,
            FixturePhaseResidualPermission::Allow
        ),
        Err(FixturePhaseError::Expired)
    ));
    Ok(())
}

/// Small signed debt chooses a long time constant, rather than silently zeroing it.
/// # Errors
/// Preserve actual source, profile, proposal and immutable fixture checks.
/// # Panics
/// Detect a fabricated zero request, sign reversal or using permission as denominator.
#[test]
fn adaptive_small_debt_keeps_signed_rate_and_independent_permission_window() -> TestResult {
    for sign in [-1_i128, 1] {
        let a = Source::new(11, 0, sign * (1_i128 << 29))?;
        let b = Source::new(22, 0, sign * (1_i128 << 29))?;
        let chart = initial_chart()?;
        let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
        let strata = strata()?;
        let batch = product_ok(FixtureChartBatch::new(
            &chart,
            MonotonicNs::new(4 * SECOND),
            &entries,
        ))?;
        let mut producer = seed(&entries, &strata, 4)?;
        let mut owner = start_with_profile(
            &mut producer,
            &chart,
            &batch,
            &strata,
            adaptive_profile(2, 16, 1.0 / 4.0)?,
            0.0,
        )?;
        let proposal =
            product_ok(owner.issue(now(0, 4 * SECOND)?, FixturePhaseResidualPermission::Allow))?;
        assert_eq!(proposal.correction_horizon().nanoseconds(), 16 * SECOND);
        assert_eq!(
            proposal.requested_active_duration().nanoseconds(),
            2 * SECOND
        );
        assert_eq!(
            proposal.requested_active_deadline(),
            ActiveAwakeNs::new(2 * SECOND)
        );
        assert_eq!(
            proposal.transient_rate(),
            if sign < 0 { -1.0 / 128.0 } else { 1.0 / 128.0 }
        );
        assert!(proposal.has_useful_transient());
    }
    Ok(())
}

/// Propagate fixture setup errors without replacing them with invented values.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Exact integer nanoseconds in one independently supplied second.
const SECOND: u64 = 1_000_000_000;
/// Original positive phase: exactly one thirty-second of a second in Q32.
const ORIGINAL_PHASE_Q32: i128 = 1 << 27;
/// Fixed nonconstant source residual: exactly one sixty-fourth of a second.
const SOURCE_RESIDUAL_Q32: i128 = 1 << 26;

/// Turn an expected product success into an explicit semantic assertion first.
/// # Errors
/// Preserve a bounded debug rendering of the unexpected typed product failure.
/// # Panics
/// Reject failure of a literal known-valid product trajectory before propagation.
#[track_caller]
fn product_ok<T, E: fmt::Debug + fmt::Display>(value: Result<T, E>) -> TestResult<T> {
    assert!(
        value.is_ok(),
        "the independently specified valid phase-plan trajectory must succeed; bounded refusal: {:?}",
        value.as_ref().err().map(ToString::to_string)
    );
    value.map_err(|error| io::Error::other(format!("phase-plan product: {error:?}")).into())
}

/// Admit one fixed nonzero fixture label without a fallback identity.
/// # Errors
/// Reject an accidental zero in this finite source table.
fn label(value: u64) -> TestResult<NonZeroU64> {
    NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero phase fixture label").into())
}

/// Declare the original independent active and monotonic clock assertions.
/// # Errors
/// Preserve the fixed nonzero-label setup checks.
fn clocks() -> TestResult<FixtureRateClockContext> {
    Ok(FixtureRateClockContext::new(
        label(101)?,
        label(103)?,
        label(107)?,
    ))
}

/// Build one inert identity-valued affine map with genuine distinct frame tags.
/// # Errors
/// Preserve mandatory frame, domain and coefficient admission failures.
fn initial_chart() -> TestResult<FixtureAffineChart> {
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

/// Sole owner of three real exchange-derived observations and their fitted row.
struct Source {
    /// Raw original source/frame/generation, never relabelled as a target map.
    context: FixtureContext,
    /// Actual immutable-after-setup history; no combined payload is constructed.
    history: FixtureAdaptiveHistory,
}

impl Source {
    /// Build a nonconstant symmetric three-row source with an exact ideal mean.
    ///
    /// At equally spaced instants the literal residuals are `[c,-2c,c]`, so the
    /// exact slope is zero and the exact mean is the independently supplied Q32
    /// centre. Nonzero spread avoids a manufactured zero-SD combination input.
    /// This file uses source labels11/22, starts through8 seconds and Q32 centres
    /// inside the literal +/-1/8-second fixture envelope.
    /// # Errors
    /// Preserve genuine coordinate, exchange, observation and fitted-history errors.
    fn new(id: u64, first_second: u64, centre_q32: i128) -> TestResult<Self> {
        if ![11, 22].contains(&id)
            || first_second > 8
            || !(-(1_i128 << 29)..=(1_i128 << 29)).contains(&centre_q32)
        {
            return Err(io::Error::other("source outside finite phase fixture").into());
        }
        let context = FixtureContext::new(label(id)?, label(17)?, label(23)?);
        let mut history = product_ok(FixtureAdaptiveHistory::new(context, 3))?;
        for (index, residual) in [
            SOURCE_RESIDUAL_Q32,
            -2 * SOURCE_RESIDUAL_Q32,
            SOURCE_RESIDUAL_Q32,
        ]
        .into_iter()
        .enumerate()
        {
            let offset = centre_q32 + residual;
            let zero = UtcCoordinateQ32::new(0)?;
            let shifted = UtcCoordinateQ32::new(offset)?;
            let phase = exchange_arithmetic(zero, shifted, shifted, zero)?.offset();
            let second = first_second + u64::try_from(index)?;
            product_ok(history.try_append(FixtureObservation::new(
                context,
                MonotonicNs::new(second * SECOND),
                phase,
                1.0,
            )?))?;
        }
        Ok(Self { context, history })
    }

    /// Project the genuine last fitted row through the actual supplied map.
    /// # Errors
    /// Preserve original history, tracking, checked projection and enclosure errors.
    fn entry<'chart>(
        &self,
        chart: &'chart FixtureAffineChart,
        at_second: u64,
    ) -> TestResult<FixtureChartInterval<'chart>> {
        let view = product_ok(
            self.history
                .prepare_chart(self.context, FixtureChartOutcome::Known(chart)),
        )?
        .ok_or_else(|| io::Error::other("missing genuine phase-plan chart view"))?;
        let reference = self
            .history
            .reference()
            .ok_or_else(|| io::Error::other("missing genuine phase-plan reference"))?;
        let projection = product_ok(view.project_tracking(
            view.context(),
            2,
            reference,
            MonotonicNs::new(at_second * SECOND),
            FixtureProjectionTerms::new(0.25, 0.0, 0.125)?,
            0.125,
        ))?;
        product_ok(enclose_fixture_chart_projection(&projection))
    }
}

/// Bind exactly two independent fixture labels to their explicit equal strata.
/// # Errors
/// Preserve nonzero-label and stratum admission failures.
fn strata() -> TestResult<[FixturePrincipalStratum; 2]> {
    Ok([
        FixturePrincipalStratum::new(label(11)?, 2)?,
        FixturePrincipalStratum::new(label(22)?, 2)?,
    ])
}

/// Construct the original producer from actual raw projection enclosures.
/// # Errors
/// Preserve genuine enclosures, roster, minimum and producer admission failures.
fn seed(
    entries: &[FixtureChartInterval<'_>; 2],
    strata: &[FixturePrincipalStratum; 2],
    at_second: u64,
) -> TestResult<FixtureCombinationState> {
    let raw = [
        enclose_fixture_projection(entries[0].projection().source_projection())?,
        enclose_fixture_projection(entries[1].projection().source_projection())?,
    ];
    let batch = FixtureProjectedBatch::new(
        label(17)?,
        label(23)?,
        MonotonicNs::new(at_second * SECOND),
        &raw,
    )?;
    product_ok(FixtureCombinationState::seed(
        &[label(11)?, label(22)?],
        FixturePrincipalParameters::new(1.0 / 1024.0, 0.0)?,
        FixtureCombinationParameters::new(2.0, 0.0)?,
        FixtureMinimum::new(2)?,
        &batch,
        strata,
    ))
}

/// Retain the complete observable cursor state around an outer rejection.
type CursorSnapshot = (
    FixtureRateCursorStatus,
    Option<NonZeroU64>,
    Option<ActiveAwakeNs>,
    Option<MonotonicNs>,
);

/// Copy public cursor observations without a raw state reconstruction path.
fn cursor_snapshot(cursor: &FixtureRateSettlementCursor) -> CursorSnapshot {
    (
        cursor.status(),
        cursor.next_ordinal(),
        cursor.last_active_end(),
        cursor.last_monotonic_end(),
    )
}

/// Build one checked actual aggregate interval for the original operation.
/// # Errors
/// Preserve checked labels, spans, frames, domain and rate failures.
fn interval(
    chart: &FixtureAffineChart,
    context: FixtureRateClockContext,
    ordinal: u64,
    active: (u64, u64),
    monotonic: (u64, u64),
    aggregate: f64,
    target: (u64, u64),
) -> TestResult<FixtureSettledRateInterval> {
    let old = chart.frames();
    product_ok(FixtureSettledRateInterval::new(
        context,
        label(ordinal)?,
        FixtureRateSpan::new(
            (ActiveAwakeNs::new(active.0), ActiveAwakeNs::new(active.1)),
            (MonotonicNs::new(monotonic.0), MonotonicNs::new(monotonic.1)),
        )?,
        aggregate,
        FixtureChartFrames::new(
            old.target_frame(),
            old.target_generation(),
            label(target.0)?,
            label(target.1)?,
        )?,
        chart.domain(),
    ))
}

/// Exact raw source state; target-coordinate changes must not rewrite it.
#[derive(Debug, Eq, PartialEq)]
struct SourceSnapshot {
    /// Actual context and all three original observation instants/phases/divisors.
    rows: [(FixtureContext, u64, i128, u64); 3],
    /// Original stored coefficient bits, independent of subsequent projection maps.
    coefficients: (u64, u64),
}

/// Inspect the real immutable evidence without constructing a replacement report.
/// # Errors
/// Preserve an absent original observation or fitted report as fixture failure.
fn source_snapshot(source: &Source) -> TestResult<SourceSnapshot> {
    let mut rows = [(source.context, 0, 0, 0); 3];
    for (index, output) in rows.iter_mut().enumerate() {
        let row = source
            .history
            .decision_observation(index)
            .ok_or_else(|| io::Error::other("missing original phase-plan row"))?;
        *output = (
            row.context(),
            row.observed_at().nanoseconds(),
            row.phase().numerator(),
            row.variance_divisor().to_bits(),
        );
    }
    let line = source
        .history
        .report()
        .ok_or_else(|| io::Error::other("missing original phase-plan fitted row"))?
        .selection()
        .statistics()
        .line();
    Ok(SourceSnapshot {
        rows,
        coefficients: (
            line.phase_seconds().to_bits(),
            line.phase_rate_seconds_per_second().to_bits(),
        ),
    })
}

/// A real healthy producer emits one literal phase; old evaluation gives no credit.
/// # Errors
/// Preserve actual fixture, batch, source, producer and reference-token failures.
/// # Panics
/// Reject wrong literal mean/rate, counterfeit fresh rounds or changed source rows.
#[test]
fn genuine_literal_sources_emit_once_and_repeated_evaluation_is_not_ready() -> TestResult {
    let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
    let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
    let original = [source_snapshot(&a)?, source_snapshot(&b)?];
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let first = product_ok(producer.advance_chart(&chart, &batch, &strata))?;
    assert_eq!(first.emitted_round(), Some(label(1)?));
    let input_owner = FixtureReferenceInputState::new(&chart);
    let input = product_ok(input_owner.admit_reference_input(&first))?;
    assert_eq!(input.combined().phase_seconds(), 1.0 / 32.0);
    assert_eq!(input.combined().phase_rate_seconds_per_second(), 0.0);
    assert!(input.combined().phase_mean_roundoff_seconds().is_finite());
    assert!(input.combined().phase_mean_roundoff_seconds() >= 0.0);
    let retained = product_ok(producer.advance_chart(&chart, &batch, &strata))?;
    assert_eq!(retained.emitted_round(), None);
    assert!(matches!(
        input_owner.admit_reference_input(&retained),
        Err(FixtureReferenceInputError::NotReady)
    ));
    // This immutable genuine emission supplies the forwarding oracle, not a
    // recomputation of its numerical bound through the phase-plan getter.
    let original_error = input.combined().phase_mean_roundoff_seconds();
    assert!(original_error.is_finite() && original_error > 0.0 && original_error < 1.0);
    let mut phase_producer = seed(&entries, &strata, 4)?;
    let mut owner = start(&mut phase_producer, &chart, &batch, &strata, 1.0 / 1024.0)?;
    check_original_error(&owner, original_error.to_bits());
    check_issue(&mut owner, (0, 4 * SECOND), 1, (1.0 / 256.0, 5.0 / 1024.0))?;
    let mut retired = settle_interval(
        owner,
        1,
        ((0, 2 * SECOND), (4 * SECOND, 6 * SECOND)),
        5.0 / 1024.0,
        (31, 7),
        false,
        1.0 / 1024.0,
    )?;
    assert_eq!(
        retired
            .provenance()
            .map(|value| value.phase_error_seconds().to_bits()),
        Some(original_error.to_bits())
    );
    let accounted_error = retired.remaining_phase_error_seconds();
    assert!(accounted_error.is_some_and(|value| value.is_finite() && value >= original_error));
    let resumed = product_ok(retired.resume(
        now(2 * SECOND, 6 * SECOND)?,
        FixturePhaseResidualPermission::Allow,
    ))?;
    check_original_error(&resumed, original_error.to_bits());
    assert_eq!(
        resumed.last_known_phase_error_seconds().map(f64::to_bits),
        accounted_error.map(f64::to_bits)
    );
    assert_eq!([source_snapshot(&a)?, source_snapshot(&b)?], original);
    Ok(())
}

/// Admit the exact finite dyadic profile selected for this fixture only.
/// # Errors
/// Preserve duration and mandatory profile admission failures.
fn profile() -> TestResult<FixturePhaseProfile> {
    product_ok(FixturePhaseProfile::new(
        ActiveAwakeNs::new(8 * SECOND).duration_since(ActiveAwakeNs::new(0))?,
        MonotonicNs::new(16 * SECOND).duration_since(MonotonicNs::new(0))?,
        1.0 / 16.0,
        1.0 / 128.0,
        1.0 / 64.0,
    ))
}

/// Retain explicitly supplied independent nanosecond readings in one context.
/// # Errors
/// Preserve the literal context's nonzero-label admission.
fn now(active: u64, monotonic: u64) -> TestResult<FixturePhaseNow> {
    Ok(FixturePhaseNow::new(
        clocks()?,
        ActiveAwakeNs::new(active),
        MonotonicNs::new(monotonic),
    ))
}

/// Admit one independently declared effective baseline, never a requested value.
/// # Errors
/// Preserve the baseline's maintained rate-domain refusal.
fn baseline(rate: f64) -> TestResult<FixturePhaseBaseline> {
    product_ok(FixturePhaseBaseline::new(rate))
}

/// Lease the real producer and admit its first real emitted reference.
/// # Errors
/// Preserve owner, time, profile, producer and reference admission failures.
/// # Panics
/// Reject a failed first emission or a plan created without its actual round.
fn start<'producer, 'chart>(
    producer: &'producer mut FixtureCombinationState,
    chart: &'chart FixtureAffineChart,
    batch: &FixtureChartBatch<'_, 'chart>,
    strata: &[FixturePrincipalStratum; 2],
    effective_base: f64,
) -> TestResult<FixturePhasePlanOwner<'producer, 'chart>> {
    let mut owner = product_ok(FixturePhasePlanOwner::new(
        producer,
        chart,
        FixtureRateSettlementCursor::new(chart, clocks()?, label(1)?),
        now(0, 4 * SECOND)?,
        profile()?,
        baseline(effective_base)?,
    ))?;
    assert_eq!(owner.last_known_phase_error_seconds(), None);
    let admitted = product_ok(owner.receive_reference(batch, strata, now(0, 4 * SECOND)?))?;
    assert_eq!(format!("{owner:?}"), "FixturePhasePlanOwner { .. }");
    assert_eq!(
        format!("{admitted:?}"),
        "FixturePhaseReferenceAdmission { .. }"
    );
    assert_eq!(
        owner.provenance().map(|value| format!("{value:?}")),
        Some("FixturePhaseProvenance { .. }".to_owned())
    );
    assert!(matches!(
        admitted,
        FixturePhaseReferenceAdmission::Accepted { emitted_round } if emitted_round == label(1)?
    ));
    assert!(owner.proposal().is_none());
    assert_eq!(
        owner.last_known_phase_error_seconds().map(f64::to_bits),
        owner.remaining_phase_error_seconds().map(f64::to_bits)
    );
    Ok(owner)
}

/// Inspect independently specified debt while keeping its bound separate.
/// # Panics
/// Reject missing/wrong phase or a missing, negative or nonfinite numerical bound.
fn check_debt(point: Option<f64>, error: Option<f64>, expected: f64) {
    assert_eq!(point, Some(expected));
    assert!(error.is_some_and(|value| value.is_finite() && value >= 0.0));
}

/// Reject two new observations as one event without changing the current owner.
/// # Panics
/// Reject any other failure, lost debt/provenance/time/cursor or hidden issuance.
fn check_multiple_updates_rejected<'chart>(
    owner: &mut FixturePhasePlanOwner<'_, 'chart>,
    batch: &FixtureChartBatch<'_, 'chart>,
    strata: &[FixturePrincipalStratum; 2],
    at: FixturePhaseNow,
) {
    let snapshot = |owner: &FixturePhasePlanOwner<'_, '_>| {
        (
            cursor_snapshot(owner.cursor()),
            (
                owner.remaining_phase_seconds().map(f64::to_bits),
                owner.remaining_phase_error_seconds().map(f64::to_bits),
                owner.last_known_phase_seconds().map(f64::to_bits),
                owner.last_known_phase_error_seconds().map(f64::to_bits),
            ),
            (
                owner.active_deadline(),
                owner.monotonic_deadline(),
                owner.current_time().active(),
                owner.current_time().monotonic(),
            ),
            owner.provenance().map(|p| {
                (
                    p.phase_seconds().to_bits(),
                    p.phase_error_seconds().to_bits(),
                    p.emitted_round(),
                    p.source_context(),
                    p.frames(),
                    p.anchor_observed_at(),
                    p.latest_observed_at(),
                    p.fit_reference(),
                    p.projected_at(),
                )
            }),
        )
    };
    let before = snapshot(owner);
    assert!(owner.proposal().is_none());
    assert!(matches!(
        owner.receive_reference(batch, strata, at),
        Err(FixturePhaseError::Combination {
            source: FixtureChartCombinationError::Principal {
                source: FixtureChartPrincipalError::Principal {
                    source: FixturePrincipalError::MultipleObservations
                }
            }
        })
    ));
    assert_eq!(snapshot(owner), before);
    assert!(owner.proposal().is_none());
}

/// Receive two genuine source events in the producer's serialized event order.
///
/// Label 11 is the retained principal. Updating nonprincipal 22 first commits its
/// pending observation without emitting; updating 11 then emits round 2 using both
/// new histories. Updating 11 first would consume its credit before 22 arrives.
/// No operation is issued between these two actual reference transactions.
/// # Errors
/// Preserve genuine projections, checked batches and actual receiving failures.
/// # Panics
/// Reject minted intermediate credit, a lost pending update or a fabricated round.
fn receive_serialized_refresh(
    owner: &mut FixturePhasePlanOwner<'_, '_>,
    original_a: &Source,
    new_a: &Source,
    new_b: &Source,
    strata: &[FixturePrincipalStratum; 2],
    readings: (u64, u64),
) -> TestResult {
    assert_eq!(
        owner.provenance().map(|p| p.source_context().source_id()),
        Some(label(11)?)
    );
    let pending_entries = [
        original_a.entry(owner.chart(), 8)?,
        new_b.entry(owner.chart(), 8)?,
    ];
    let pending_batch = product_ok(FixtureChartBatch::new(
        owner.chart(),
        MonotonicNs::new(8 * SECOND),
        &pending_entries,
    ))?;
    assert!(matches!(
        product_ok(owner.receive_reference(&pending_batch, strata, now(readings.0, readings.1)?,))?,
        FixturePhaseReferenceAdmission::AdmittedNoPlan {
            emitted_round: None,
            reason: FixturePhaseError::Reference {
                source: FixtureReferenceInputError::NotReady
            }
        }
    ));
    assert!(owner.proposal().is_none());
    assert_eq!(owner.remaining_phase_seconds(), None);
    let entries = [
        new_a.entry(owner.chart(), 8)?,
        new_b.entry(owner.chart(), 8)?,
    ];
    let batch = product_ok(FixtureChartBatch::new(
        owner.chart(),
        MonotonicNs::new(8 * SECOND),
        &entries,
    ))?;
    assert!(
        matches!(product_ok(owner.receive_reference(&batch, strata, now(readings.0, readings.1)?))?, FixturePhaseReferenceAdmission::Accepted { emitted_round } if emitted_round == label(2)?)
    );
    assert!(owner.proposal().is_none());
    Ok(())
}

/// Inspect the actual issued view against literal rates and original deadlines.
/// # Errors
/// Preserve the independently supplied nonzero operation-label check.
/// # Panics
/// Reject wrong identity, rate, missing errors or a restarted finite horizon.
fn check_proposal(
    proposal: &FixturePhaseProposal<'_>,
    ordinal: u64,
    transient: f64,
    aggregate: f64,
) -> TestResult {
    assert_eq!(format!("{proposal:?}"), "FixturePhaseProposal { .. }");
    assert_eq!(proposal.ordinal(), label(ordinal)?);
    assert_eq!(proposal.transient_rate(), transient);
    assert_eq!(proposal.aggregate_rate(), aggregate);
    assert!(proposal.transient_error().is_finite() && proposal.transient_error() >= 0.0);
    assert!(proposal.aggregate_error().is_finite() && proposal.aggregate_error() >= 0.0);
    assert_eq!(proposal.active_deadline(), ActiveAwakeNs::new(8 * SECOND));
    assert_eq!(proposal.monotonic_deadline(), MonotonicNs::new(20 * SECOND));
    Ok(())
}

/// Require a known terminal without a fallback old owner or fabricated chart.
/// # Errors
/// Reject a different real receiving branch after its semantic assertion.
/// # Panics
/// Reject failure to commit an independently valid known terminal.
fn known<'producer>(
    owner: FixturePhasePlanOwner<'producer, '_>,
    event: FixtureRateSettlement,
    effective_base: Option<FixturePhaseBaseline>,
    at: FixturePhaseNow,
) -> TestResult<FixturePhaseRetirement<'producer>> {
    let result = owner.settle(event, effective_base, at);
    assert_eq!(format!("{result:?}"), "FixturePhaseSettlement { .. }");
    assert!(
        matches!(&result, FixturePhaseSettlement::Known(_)),
        "the independently specified known interval must commit its historical result"
    );
    match result {
        FixturePhaseSettlement::Known(retirement) => {
            assert_eq!(format!("{retirement:?}"), "FixturePhaseRetirement { .. }");
            Ok(retirement)
        }
        _ => Err(io::Error::other("missing known phase retirement").into()),
    }
}

/// Issue one known-valid operation and inspect its independent literal rates.
/// # Errors
/// Preserve genuine current-time, issuance and view-check failures.
fn check_issue(
    owner: &mut FixturePhasePlanOwner<'_, '_>,
    readings: (u64, u64),
    ordinal: u64,
    rates: (f64, f64),
) -> TestResult {
    let proposal = product_ok(owner.issue(
        now(readings.0, readings.1)?,
        FixturePhaseResidualPermission::Allow,
    ))?;
    check_proposal(&proposal, ordinal, rates.0, rates.1)
}

/// Settle one real completed/cancelled interval without constructing an outcome.
/// # Errors
/// Preserve the real interval and consuming historical receiver failures.
fn settle_interval<'producer>(
    owner: FixturePhasePlanOwner<'producer, '_>,
    ordinal: u64,
    spans: ((u64, u64), (u64, u64)),
    aggregate: f64,
    target: (u64, u64),
    cancelled: bool,
    effective_base: f64,
) -> TestResult<FixturePhaseRetirement<'producer>> {
    let interval = interval(
        owner.chart(),
        clocks()?,
        ordinal,
        spans.0,
        spans.1,
        aggregate,
        target,
    )?;
    let event = if cancelled {
        FixtureRateSettlement::Cancelled(interval)
    } else {
        FixtureRateSettlement::Completed(interval)
    };
    known(
        owner,
        event,
        Some(baseline(effective_base)?),
        now(spans.0.1, spans.1.1)?,
    )
}

/// Recover an unchanged owner only from its real outer-rejection branch.
/// # Errors
/// Reject a different ownership outcome without fabricating an owner.
/// # Panics
/// Reject changed receiving priority or a consumed operation on structural refusal.
fn rejected<'producer, 'chart>(
    result: FixturePhaseSettlement<'producer, 'chart>,
    expected: &FixturePhaseError,
) -> TestResult<FixturePhasePlanOwner<'producer, 'chart>> {
    assert!(matches!(
        &result,
        FixturePhaseSettlement::OuterRejected { .. }
    ));
    match result {
        FixturePhaseSettlement::OuterRejected { owner, error } => {
            assert_eq!(&error, expected);
            Ok(owner)
        }
        _ => Err(io::Error::other("missing unchanged outer-rejected owner").into()),
    }
}

/// Recover the same chart owner only from an actual proven-no-effect terminal.
/// # Errors
/// Reject a different real terminal result after its semantic assertion.
/// # Panics
/// Reject a no-effect event that invents a known interval or loses ownership.
fn no_effect<'producer, 'chart>(
    owner: FixturePhasePlanOwner<'producer, 'chart>,
    ordinal: u64,
    at: FixturePhaseNow,
) -> TestResult<FixturePhasePlanOwner<'producer, 'chart>> {
    let original_error = owner.remaining_phase_error_seconds().map(f64::to_bits);
    assert!(original_error.is_some());
    let result = owner.settle(
        FixtureRateSettlement::ProvenNoEffect {
            context: clocks()?,
            ordinal: label(ordinal)?,
        },
        None,
        at,
    );
    assert!(matches!(&result, FixturePhaseSettlement::ProvenNoEffect(_)));
    match result {
        FixturePhaseSettlement::ProvenNoEffect(owner) => {
            assert_eq!(
                owner.last_known_phase_error_seconds().map(f64::to_bits),
                original_error
            );
            Ok(owner)
        }
        _ => Err(io::Error::other("missing same-chart no-effect owner").into()),
    }
}

/// Reject the first interval's replay before settling the second real interval.
/// # Errors
/// Preserve genuine issuance, replay reception, settlement and label failures.
/// # Panics
/// Reject a lost proposal or a changed replay-refusal cause.
fn second_retirement_after_replay<'producer>(
    mut owner: FixturePhasePlanOwner<'producer, '_>,
    replay: FixtureRateSettlement,
) -> TestResult<FixturePhaseRetirement<'producer>> {
    assert!(owner.proposal().is_none());
    check_issue(
        &mut owner,
        (2 * SECOND, 6 * SECOND),
        2,
        (1.0 / 256.0, 5.0 / 1024.0),
    )?;
    // The genuine old interval also predates the second operation's start.
    owner = rejected(
        owner.settle(
            replay,
            Some(baseline(1.0 / 1024.0)?),
            now(2 * SECOND, 6 * SECOND)?,
        ),
        &FixturePhaseError::TimeRegression {
            clock: FixtureRateClock::ActiveAwake,
        },
    )?;
    assert_eq!(
        owner.proposal().map(|value| value.ordinal()),
        Some(label(2)?)
    );
    settle_interval(
        owner,
        2,
        ((2 * SECOND, 4 * SECOND), (6 * SECOND, 8 * SECOND)),
        5.0 / 1024.0,
        (37, 11),
        true,
        1.0 / 1024.0,
    )
}

/// Two real retirements carry the updated phase and original time allowance.
/// # Errors
/// Preserve genuine producer, owner, issue, settlement and resume failures.
/// # Panics
/// Reject repeated integration, total/base confusion, lost error or lifetime reset.
#[test]
fn successive_two_second_retirements_carry_debt_error_and_original_deadlines() -> TestResult {
    let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
    let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
    let raw_before = [source_snapshot(&a)?, source_snapshot(&b)?];
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let mut owner0 = start(&mut producer, &chart, &batch, &strata, 1.0 / 1024.0)?;
    check_debt(
        owner0.remaining_phase_seconds(),
        owner0.remaining_phase_error_seconds(),
        1.0 / 32.0,
    );
    let original_error = owner0.remaining_phase_error_seconds();
    check_issue(&mut owner0, (0, 4 * SECOND), 1, (1.0 / 256.0, 5.0 / 1024.0))?;
    let mut retirement1 = settle_interval(
        owner0,
        1,
        ((0, 2 * SECOND), (4 * SECOND, 6 * SECOND)),
        5.0 / 1024.0,
        (31, 7),
        false,
        1.0 / 1024.0,
    )?;
    check_debt(
        retirement1.remaining_phase_seconds(),
        retirement1.remaining_phase_error_seconds(),
        3.0 / 128.0,
    );
    assert!(retirement1.remaining_phase_error_seconds() >= original_error);
    let first_error = retirement1.remaining_phase_error_seconds();
    let replay = *retirement1.event();
    assert_eq!(
        retirement1
            .cursor()
            .and_then(FixtureRateSettlementCursor::next_ordinal),
        Some(label(2)?)
    );
    let owner1 = product_ok(retirement1.resume(
        now(2 * SECOND, 6 * SECOND)?,
        FixturePhaseResidualPermission::Allow,
    ))?;
    let mut retirement2 = second_retirement_after_replay(owner1, replay)?;
    check_original_retirement_deadlines(&retirement2);
    check_debt(
        retirement2.remaining_phase_seconds(),
        retirement2.remaining_phase_error_seconds(),
        1.0 / 64.0,
    );
    assert!(retirement2.remaining_phase_error_seconds() >= first_error);
    assert_eq!(
        retirement2
            .cursor()
            .and_then(FixtureRateSettlementCursor::last_active_end),
        Some(ActiveAwakeNs::new(4 * SECOND))
    );
    assert!(retirement1.cursor().is_none());
    check_original_retirement_deadlines(&retirement1);
    assert!(matches!(
        retirement1.resume(
            now(4 * SECOND, 8 * SECOND)?,
            FixturePhaseResidualPermission::Allow
        ),
        Err(FixturePhaseError::TransferConsumed)
    ));
    assert!(matches!(
        retirement1.resume_fresh(now(4 * SECOND, 8 * SECOND)?, baseline(1.0 / 1024.0)?),
        Err(FixturePhaseError::TransferConsumed)
    ));
    let mut owner2 = product_ok(retirement2.resume(
        now(4 * SECOND, 8 * SECOND)?,
        FixturePhaseResidualPermission::Allow,
    ))?;
    check_issue(
        &mut owner2,
        (4 * SECOND, 8 * SECOND),
        3,
        (1.0 / 256.0, 5.0 / 1024.0),
    )?;
    assert_eq!(
        owner2
            .provenance()
            .map(FixturePhaseProvenance::phase_seconds),
        Some(1.0 / 32.0)
    );
    assert_eq!(
        owner2
            .provenance()
            .map(FixturePhaseProvenance::emitted_round),
        Some(label(1)?)
    );
    assert_eq!([source_snapshot(&a)?, source_snapshot(&b)?], raw_before);
    Ok(())
}

/// Actual quantized delivery, rather than the requested rate, reduces the debt.
/// # Errors
/// Preserve actual history, admission, interval and known-retirement failures.
/// # Panics
/// Reject requested-rate substitution or an erased non-dyadic division bound.
#[test]
fn quantized_actual_rate_has_the_literal_residual_and_division_bound() -> TestResult {
    let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
    let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let mut owner = start(&mut producer, &chart, &batch, &strata, 1.0 / 1024.0)?;
    check_issue(&mut owner, (0, 4 * SECOND), 1, (1.0 / 256.0, 5.0 / 1024.0))?;
    let mut retired = settle_interval(
        owner,
        1,
        ((0, 2 * SECOND), (4 * SECOND, 6 * SECOND)),
        9.0 / 2048.0,
        (31, 7),
        false,
        1.0 / 1024.0,
    )?;
    check_debt(
        retired.remaining_phase_seconds(),
        retired.remaining_phase_error_seconds(),
        25.0 / 1024.0,
    );
    let mut next = product_ok(retired.resume(
        now(2 * SECOND, 6 * SECOND)?,
        FixturePhaseResidualPermission::Allow,
    ))?;
    let proposal = product_ok(next.issue(
        now(2 * SECOND, 6 * SECOND)?,
        FixturePhaseResidualPermission::Allow,
    ))?;
    assert_eq!(proposal.transient_rate(), 25.0 / 6144.0);
    assert!(proposal.transient_error() > 0.0);
    assert_ne!(proposal.transient_rate(), 1.0 / 256.0);
    assert_eq!(proposal.ordinal(), label(2)?);
    Ok(())
}

/// A negative transient can require a positive aggregate without being opposite.
/// # Errors
/// Preserve genuine source, admission, delivery and continuation failures.
/// # Panics
/// Reject checking aggregate sign in place of the baseline-relative contribution.
#[test]
fn negative_transient_with_positive_aggregate_is_a_healthy_continuation() -> TestResult {
    let a = Source::new(11, 0, -(1 << 24))?;
    let b = Source::new(22, 0, -(1 << 24))?;
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let mut owner = start(&mut producer, &chart, &batch, &strata, 1.0 / 1024.0)?;
    check_debt(
        owner.remaining_phase_seconds(),
        owner.remaining_phase_error_seconds(),
        -1.0 / 256.0,
    );
    check_issue(
        &mut owner,
        (0, 4 * SECOND),
        1,
        (-1.0 / 2048.0, 1.0 / 2048.0),
    )?;
    let mut retired = settle_interval(
        owner,
        1,
        ((0, 2 * SECOND), (4 * SECOND, 6 * SECOND)),
        1.0 / 2048.0,
        (31, 7),
        false,
        1.0 / 1024.0,
    )?;
    check_debt(
        retired.remaining_phase_seconds(),
        retired.remaining_phase_error_seconds(),
        -3.0 / 1024.0,
    );
    let mut next = product_ok(retired.resume(
        now(2 * SECOND, 6 * SECOND)?,
        FixturePhaseResidualPermission::Allow,
    ))?;
    check_issue(
        &mut next,
        (2 * SECOND, 6 * SECOND),
        2,
        (-1.0 / 2048.0, 1.0 / 2048.0),
    )?;
    Ok(())
}

/// Proven no effect consumes exactly its ordinal, without inventing delivery.
/// # Errors
/// Preserve actual owner, issue, no-effect, rejection and useful-neighbor failures.
/// # Panics
/// Reject hidden reissuance, invented endpoints, changed debt or repeated credit.
#[test]
fn no_effect_clears_only_the_operation_and_duplicate_terminal_is_outer_rejected() -> TestResult {
    let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
    let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let mut owner = start(&mut producer, &chart, &batch, &strata, 1.0 / 1024.0)?;
    let before = cursor_snapshot(owner.cursor());
    check_issue(&mut owner, (0, 4 * SECOND), 1, (1.0 / 256.0, 5.0 / 1024.0))?;
    assert!(matches!(
        owner.issue(now(0, 4 * SECOND)?, FixturePhaseResidualPermission::Allow),
        Err(FixturePhaseError::OutstandingOperation)
    ));
    owner = no_effect(owner, 1, now(0, 4 * SECOND)?)?;
    assert!(owner.proposal().is_none());
    assert_eq!(owner.cursor().next_ordinal(), Some(label(2)?));
    assert_eq!(owner.cursor().last_active_end(), before.2);
    assert_eq!(owner.cursor().last_monotonic_end(), before.3);
    check_debt(
        owner.remaining_phase_seconds(),
        owner.remaining_phase_error_seconds(),
        1.0 / 32.0,
    );
    assert_eq!(
        owner.active_deadline(),
        Some(ActiveAwakeNs::new(8 * SECOND))
    );
    assert_eq!(
        owner.monotonic_deadline(),
        Some(MonotonicNs::new(20 * SECOND))
    );
    let duplicate = FixtureRateSettlement::ProvenNoEffect {
        context: clocks()?,
        ordinal: label(1)?,
    };
    owner = rejected(
        owner.settle(duplicate, None, now(0, 4 * SECOND)?),
        &FixturePhaseError::NoOutstandingOperation,
    )?;
    check_issue(&mut owner, (0, 4 * SECOND), 2, (1.0 / 256.0, 5.0 / 1024.0))?;
    owner = rejected(
        owner.settle(duplicate, None, now(0, 4 * SECOND)?),
        &FixturePhaseError::Delivery {
            source: FixtureRateDeliveryError::OrdinalMismatch,
        },
    )?;
    let retired = settle_interval(
        owner,
        2,
        ((0, 2 * SECOND), (4 * SECOND, 6 * SECOND)),
        5.0 / 1024.0,
        (31, 7),
        false,
        1.0 / 1024.0,
    )?;
    check_debt(
        retired.remaining_phase_seconds(),
        retired.remaining_phase_error_seconds(),
        3.0 / 128.0,
    );
    Ok(())
}

/// Foreign and replayed terminals preserve the actual outstanding operation.
/// # Errors
/// Preserve genuine setup, rejection recovery and the matching healthy terminal.
/// # Panics
/// Reject changed cursor/debt, wrong refusal identity or a refunded operation.
#[test]
fn foreign_context_and_wrong_ordinal_leave_a_matching_terminal_usable() -> TestResult {
    let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
    let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let mut owner = start(&mut producer, &chart, &batch, &strata, 1.0 / 1024.0)?;
    check_issue(&mut owner, (0, 4 * SECOND), 1, (1.0 / 256.0, 5.0 / 1024.0))?;
    let before = cursor_snapshot(owner.cursor());
    let foreign = FixtureRateClockContext::new(label(109)?, label(103)?, label(107)?);
    let event = FixtureRateSettlement::Unknown {
        context: foreign,
        ordinal: label(1)?,
    };
    owner = rejected(
        owner.settle(event, None, now(0, 4 * SECOND)?),
        &FixturePhaseError::Delivery {
            source: FixtureRateDeliveryError::ClockContextMismatch,
        },
    )?;
    let event = FixtureRateSettlement::Unknown {
        context: clocks()?,
        ordinal: label(2)?,
    };
    owner = rejected(
        owner.settle(event, None, now(0, 4 * SECOND)?),
        &FixturePhaseError::Delivery {
            source: FixtureRateDeliveryError::OrdinalMismatch,
        },
    )?;
    assert_eq!(cursor_snapshot(owner.cursor()), before);
    assert_eq!(owner.proposal().map(|p| p.ordinal()), Some(label(1)?));
    check_debt(
        owner.remaining_phase_seconds(),
        owner.remaining_phase_error_seconds(),
        1.0 / 32.0,
    );
    let retired = settle_interval(
        owner,
        1,
        ((0, 2 * SECOND), (4 * SECOND, 6 * SECOND)),
        5.0 / 1024.0,
        (31, 7),
        false,
        1.0 / 1024.0,
    )?;
    check_debt(
        retired.remaining_phase_seconds(),
        retired.remaining_phase_error_seconds(),
        3.0 / 128.0,
    );
    Ok(())
}

/// Current Unknown retains the event and unresolved custody without zero inference.
/// # Errors
/// Preserve genuine setup and current operation issuance failures.
/// # Panics
/// Reject an outer refusal, fabricated endpoint/debt result or refunded cursor.
#[test]
fn current_unknown_is_a_blocked_historical_owner_not_zero_delivery() -> TestResult {
    let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
    let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let mut owner = start(&mut producer, &chart, &batch, &strata, 1.0 / 1024.0)?;
    let original_error = owner.remaining_phase_error_seconds();
    assert!(original_error.is_some_and(|value| value.is_finite() && value >= 0.0));
    check_issue(&mut owner, (0, 4 * SECOND), 1, (1.0 / 256.0, 5.0 / 1024.0))?;
    let event = FixtureRateSettlement::Unknown {
        context: clocks()?,
        ordinal: label(1)?,
    };
    let settled = owner.settle(event, None, now(0, 5 * SECOND)?);
    assert!(matches!(&settled, FixturePhaseSettlement::Unknown(_)));
    let FixturePhaseSettlement::Unknown(blocked) = settled else {
        return Err(io::Error::other("missing actual blocked owner").into());
    };
    assert_eq!(
        blocked.cursor().status(),
        FixtureRateCursorStatus::Unresolved
    );
    assert_eq!(blocked.cursor().next_ordinal(), None);
    assert_eq!(blocked.cursor().last_active_end(), None);
    assert_eq!(blocked.cursor().last_monotonic_end(), None);
    assert!(
        matches!(blocked.event(), FixtureRateSettlement::Unknown { ordinal, .. } if *ordinal == label(1)?)
    );
    assert_eq!(blocked.last_known_phase_seconds(), Some(1.0 / 32.0));
    assert_eq!(blocked.last_known_phase_error_seconds(), original_error);
    assert_eq!(
        blocked
            .provenance()
            .map(FixturePhaseProvenance::phase_seconds),
        Some(1.0 / 32.0)
    );
    assert_eq!(
        blocked
            .provenance()
            .map(FixturePhaseProvenance::emitted_round),
        Some(label(1)?)
    );
    assert_eq!(
        blocked.provenance().map(|value| (
            value.source_context(),
            value.frames(),
            value.anchor_observed_at(),
            value.latest_observed_at(),
            value.fit_reference(),
            value.projected_at(),
        )),
        Some((
            a.context,
            chart.frames(),
            MonotonicNs::new(2 * SECOND),
            MonotonicNs::new(2 * SECOND),
            MonotonicNs::new(2 * SECOND),
            MonotonicNs::new(4 * SECOND),
        ))
    );
    assert_eq!(format!("{blocked:?}"), "FixturePhaseBlocked { .. }");
    Ok(())
}

/// A retained remap earns no reference; genuinely new rows replace debt directly.
/// # Errors
/// Preserve real histories, chart batches, producer admission and context transfer.
/// # Panics
/// Reject invented credit, old-debt addition, delivery subtraction twice or reset cursors.
#[test]
fn retained_remap_is_not_ready_but_fresh_observations_replace_the_obligation() -> TestResult {
    let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
    let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
    let before = [source_snapshot(&a)?, source_snapshot(&b)?];
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let mut owner = start(&mut producer, &chart, &batch, &strata, 1.0 / 1024.0)?;
    check_issue(&mut owner, (0, 4 * SECOND), 1, (1.0 / 256.0, 5.0 / 1024.0))?;
    let mut retired = settle_interval(
        owner,
        1,
        ((0, 2 * SECOND), (4 * SECOND, 6 * SECOND)),
        5.0 / 1024.0,
        (31, 7),
        false,
        1.0 / 1024.0,
    )?;
    let accounted_error = retired.remaining_phase_error_seconds().map(f64::to_bits);
    assert!(accounted_error.is_some());
    let mut fresh =
        product_ok(retired.resume_fresh(now(2 * SECOND, 6 * SECOND)?, baseline(1.0 / 1024.0)?))?;
    assert_eq!(fresh.remaining_phase_seconds(), None);
    assert_eq!(
        fresh.last_known_phase_error_seconds().map(f64::to_bits),
        accounted_error
    );
    let remapped = [a.entry(fresh.chart(), 4)?, b.entry(fresh.chart(), 4)?];
    let old_batch = product_ok(FixtureChartBatch::new(
        fresh.chart(),
        MonotonicNs::new(4 * SECOND),
        &remapped,
    ))?;
    let retained =
        product_ok(fresh.receive_reference(&old_batch, &strata, now(2 * SECOND, 6 * SECOND)?))?;
    assert!(matches!(
        retained,
        FixturePhaseReferenceAdmission::AdmittedNoPlan {
            emitted_round: None,
            reason: FixturePhaseError::Reference {
                source: FixtureReferenceInputError::NotReady
            },
        }
    ));
    assert!(fresh.proposal().is_none());
    assert_eq!(fresh.remaining_phase_error_seconds(), None);
    assert_eq!(
        fresh.last_known_phase_error_seconds().map(f64::to_bits),
        accounted_error
    );
    assert_eq!(fresh.cursor().next_ordinal(), Some(label(2)?));
    // The original aggregate delivered 5/512. New raw mean13/512 therefore
    // projects to the independently chosen fresh phase8/512 =1/64.
    let new_a = Source::new(11, 4, 13 << 23)?;
    let new_b = Source::new(22, 4, 13 << 23)?;
    receive_serialized_refresh(
        &mut fresh,
        &a,
        &new_a,
        &new_b,
        &strata,
        (2 * SECOND, 8 * SECOND),
    )?;
    check_debt(
        fresh.remaining_phase_seconds(),
        fresh.remaining_phase_error_seconds(),
        1.0 / 64.0,
    );
    let proposal = product_ok(fresh.issue(
        now(2 * SECOND, 8 * SECOND)?,
        FixturePhaseResidualPermission::Deny,
    ))?;
    assert_eq!(proposal.transient_rate(), 1.0 / 512.0);
    assert_eq!(proposal.aggregate_rate(), 3.0 / 1024.0);
    assert_eq!(proposal.ordinal(), label(2)?);
    assert_eq!(proposal.active_deadline(), ActiveAwakeNs::new(10 * SECOND));
    assert_eq!(proposal.monotonic_deadline(), MonotonicNs::new(24 * SECOND));
    assert_eq!([source_snapshot(&a)?, source_snapshot(&b)?], before);
    Ok(())
}

/// Each deadline is independently exact and a later rewind cannot restore old debt.
/// # Errors
/// Preserve genuine setup, settlement and the subsequent fresh-context neighbor.
/// # Panics
/// Reject deadline extension, cursor loss or renewal of an expired obligation.
#[test]
fn exact_expiry_then_clock_rewind_cannot_restore_residual_permission() -> TestResult {
    for (active, monotonic, rewind_clock) in [
        (8 * SECOND, 12 * SECOND, FixtureRateClock::ActiveAwake),
        (2 * SECOND, 20 * SECOND, FixtureRateClock::Monotonic),
    ] {
        let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
        let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
        let chart = initial_chart()?;
        let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
        let strata = strata()?;
        let batch = product_ok(FixtureChartBatch::new(
            &chart,
            MonotonicNs::new(4 * SECOND),
            &entries,
        ))?;
        let mut producer = seed(&entries, &strata, 4)?;
        let mut owner = start(&mut producer, &chart, &batch, &strata, 1.0 / 1024.0)?;
        check_issue(&mut owner, (0, 4 * SECOND), 1, (1.0 / 256.0, 5.0 / 1024.0))?;
        let mut retired = settle_interval(
            owner,
            1,
            ((0, 2 * SECOND), (4 * SECOND, 6 * SECOND)),
            5.0 / 1024.0,
            (31, 7),
            false,
            1.0 / 1024.0,
        )?;
        assert!(matches!(
            retired.resume(
                now(active, monotonic)?,
                FixturePhaseResidualPermission::Allow
            ),
            Err(FixturePhaseError::Expired)
        ));
        assert!(
            matches!(retired.resume(now(2 * SECOND, 6 * SECOND)?, FixturePhaseResidualPermission::Allow), Err(FixturePhaseError::TimeRegression { clock }) if clock == rewind_clock)
        );
        check_debt(
            retired.remaining_phase_seconds(),
            retired.remaining_phase_error_seconds(),
            3.0 / 128.0,
        );
        assert_eq!(
            retired
                .cursor()
                .and_then(FixtureRateSettlementCursor::next_ordinal),
            Some(label(2)?)
        );
        let mut context =
            product_ok(retired.resume_fresh(now(active, monotonic)?, baseline(1.0 / 1024.0)?))?;
        assert!(context.proposal().is_none());
        assert_eq!(context.remaining_phase_seconds(), None);
        assert!(matches!(
            context.issue(
                now(active, monotonic)?,
                FixturePhaseResidualPermission::Allow
            ),
            Err(FixturePhaseError::NoReference)
        ));
    }
    Ok(())
}

/// Retired proposal authority still admits matching late historical delivery.
/// # Errors
/// Preserve actual setup and known-rate receiving failures.
/// # Panics
/// Reject restoration through either transfer path or loss of known delivery.
#[test]
fn stop_conflict_and_generation_retirement_preserve_late_known_settlement() -> TestResult {
    for reason in [
        FixturePhaseRetirementReason::Stop,
        FixturePhaseRetirementReason::Conflict,
        FixturePhaseRetirementReason::Generation,
    ] {
        let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
        let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
        let chart = initial_chart()?;
        let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
        let strata = strata()?;
        let batch = product_ok(FixtureChartBatch::new(
            &chart,
            MonotonicNs::new(4 * SECOND),
            &entries,
        ))?;
        let mut producer = seed(&entries, &strata, 4)?;
        let mut owner = start(&mut producer, &chart, &batch, &strata, 1.0 / 1024.0)?;
        check_issue(&mut owner, (0, 4 * SECOND), 1, (1.0 / 256.0, 5.0 / 1024.0))?;
        owner.retire(reason);
        owner.retire(reason);
        assert!(owner.proposal().is_none());
        assert!(
            matches!(owner.issue(now(0, 4 * SECOND)?, FixturePhaseResidualPermission::Allow), Err(FixturePhaseError::Retired { reason: actual }) if actual == reason)
        );
        let mut retired = settle_interval(
            owner,
            1,
            ((0, 2 * SECOND), (4 * SECOND, 6 * SECOND)),
            5.0 / 1024.0,
            (31, 7),
            false,
            1.0 / 1024.0,
        )?;
        check_debt(
            retired.remaining_phase_seconds(),
            retired.remaining_phase_error_seconds(),
            3.0 / 128.0,
        );
        assert!(matches!(
            retired.event(),
            FixtureRateSettlement::Completed(_)
        ));
        assert_eq!(
            retired
                .cursor()
                .and_then(FixtureRateSettlementCursor::next_ordinal),
            Some(label(2)?)
        );
        assert!(
            matches!(retired.resume(now(2 * SECOND, 6 * SECOND)?, FixturePhaseResidualPermission::Allow), Err(FixturePhaseError::Retired { reason: actual }) if actual == reason)
        );
        assert!(
            matches!(retired.resume_fresh(now(2 * SECOND, 6 * SECOND)?, baseline(1.0 / 1024.0)?), Err(FixturePhaseError::Retired { reason: actual }) if actual == reason)
        );
    }
    Ok(())
}

/// An admitted unrepresentable duration consumes historical delivery before failure.
/// # Errors
/// Preserve genuine input construction and known terminal admission failures.
/// # Panics
/// Reject outer rollback, endpoint loss, default debt or a fabricated replacement.
#[test]
fn admitted_conversion_failure_retains_event_cursor_and_original_provenance() -> TestResult {
    let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
    let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let mut owner = start(&mut producer, &chart, &batch, &strata, 1.0 / 1024.0)?;
    check_issue(&mut owner, (0, 4 * SECOND), 1, (1.0 / 256.0, 5.0 / 1024.0))?;
    let mut first = settle_interval(
        owner,
        1,
        ((0, 2 * SECOND), (4 * SECOND, 6 * SECOND)),
        5.0 / 1024.0,
        (31, 7),
        false,
        1.0 / 1024.0,
    )?;
    let carried_error = first.remaining_phase_error_seconds();
    let mut next = product_ok(first.resume(
        now(2 * SECOND, 6 * SECOND)?,
        FixturePhaseResidualPermission::Allow,
    ))?;
    check_issue(
        &mut next,
        (2 * SECOND, 6 * SECOND),
        2,
        (1.0 / 256.0, 5.0 / 1024.0),
    )?;
    let huge = 2 * SECOND + (1_u64 << 53) + 1;
    let event = interval(
        next.chart(),
        clocks()?,
        2,
        (2 * SECOND, huge),
        (6 * SECOND, 8 * SECOND),
        5.0 / 1024.0,
        (37, 11),
    )?;
    let mut retired = known(
        next,
        FixtureRateSettlement::Completed(event),
        Some(baseline(1.0 / 1024.0)?),
        now(huge, 8 * SECOND)?,
    )?;
    assert!(matches!(
        retired.replacement(),
        Err(FixtureChartError::Normalization { .. })
    ));
    assert!(retired.debt_error().is_some());
    assert_eq!(retired.remaining_phase_seconds(), None);
    assert_eq!(
        retired
            .cursor()
            .and_then(FixtureRateSettlementCursor::next_ordinal),
        Some(label(3)?)
    );
    assert_eq!(
        retired
            .cursor()
            .and_then(FixtureRateSettlementCursor::last_active_end),
        Some(ActiveAwakeNs::new(huge))
    );
    assert_eq!(
        retired
            .cursor()
            .and_then(FixtureRateSettlementCursor::last_monotonic_end),
        Some(MonotonicNs::new(8 * SECOND))
    );
    assert_eq!(retired.last_known_phase_seconds(), Some(3.0 / 128.0));
    assert_eq!(retired.last_known_phase_error_seconds(), carried_error);
    assert_eq!(
        retired
            .provenance()
            .map(FixturePhaseProvenance::phase_seconds),
        Some(1.0 / 32.0)
    );
    assert!(matches!(
        retired.event(),
        FixtureRateSettlement::Completed(_)
    ));
    assert!(matches!(
        retired.resume(
            now(huge, 8 * SECOND)?,
            FixturePhaseResidualPermission::Allow
        ),
        Err(FixturePhaseError::Chart {
            source: FixtureChartError::Normalization { .. }
        })
    ));
    Ok(())
}

/// The same active span has identical debt despite independently longer monotonic time.
/// # Errors
/// Preserve real construction, known settlement and useful continuation failures.
/// # Panics
/// Reject integrating monotonic elapsed or restarting either original deadline.
#[test]
fn active_delivery_is_independent_of_monotonic_elapsed_and_utc_mapping() -> TestResult {
    for monotonic_end in [6 * SECOND, 18 * SECOND] {
        let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
        let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
        let chart = initial_chart()?;
        let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
        let strata = strata()?;
        let batch = product_ok(FixtureChartBatch::new(
            &chart,
            MonotonicNs::new(4 * SECOND),
            &entries,
        ))?;
        let mut producer = seed(&entries, &strata, 4)?;
        let mut owner = start(&mut producer, &chart, &batch, &strata, 1.0 / 1024.0)?;
        check_issue(&mut owner, (0, 4 * SECOND), 1, (1.0 / 256.0, 5.0 / 1024.0))?;
        let mut retired = settle_interval(
            owner,
            1,
            ((0, 2 * SECOND), (4 * SECOND, monotonic_end)),
            5.0 / 1024.0,
            (31, 7),
            false,
            1.0 / 1024.0,
        )?;
        check_debt(
            retired.remaining_phase_seconds(),
            retired.remaining_phase_error_seconds(),
            3.0 / 128.0,
        );
        let mut next = product_ok(retired.resume(
            now(2 * SECOND, monotonic_end)?,
            FixturePhaseResidualPermission::Allow,
        ))?;
        check_issue(
            &mut next,
            (2 * SECOND, monotonic_end),
            2,
            (1.0 / 256.0, 5.0 / 1024.0),
        )?;
    }
    Ok(())
}

/// Valid current readings remain accepted facts even when residual permission fails.
/// # Errors
/// Preserve real setup and the later permitted non-rewound continuation.
/// # Panics
/// Reject rollback of accepted clocks, lost history or accepting regressed time.
#[test]
fn rejected_permission_keeps_clock_highwaters_but_not_an_issued_operation() -> TestResult {
    let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
    let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let mut owner = start(&mut producer, &chart, &batch, &strata, 1.0 / 1024.0)?;
    check_issue(&mut owner, (0, 4 * SECOND), 1, (1.0 / 256.0, 5.0 / 1024.0))?;
    let mut retired = settle_interval(
        owner,
        1,
        ((0, 2 * SECOND), (4 * SECOND, 6 * SECOND)),
        5.0 / 1024.0,
        (31, 7),
        false,
        1.0 / 1024.0,
    )?;
    assert!(matches!(
        retired.resume(
            now(3 * SECOND, 7 * SECOND)?,
            FixturePhaseResidualPermission::Deny
        ),
        Err(FixturePhaseError::ResidualPermission)
    ));
    assert_eq!(
        retired.current_time().map(|value| value.active()),
        Some(ActiveAwakeNs::new(3 * SECOND))
    );
    assert_eq!(
        retired.current_time().map(|value| value.monotonic()),
        Some(MonotonicNs::new(7 * SECOND))
    );
    assert!(matches!(
        retired.resume(
            now(2 * SECOND, 7 * SECOND)?,
            FixturePhaseResidualPermission::Allow
        ),
        Err(FixturePhaseError::TimeRegression {
            clock: FixtureRateClock::ActiveAwake
        })
    ));
    assert!(matches!(
        retired.resume(
            now(3 * SECOND, 6 * SECOND)?,
            FixturePhaseResidualPermission::Allow
        ),
        Err(FixturePhaseError::TimeRegression {
            clock: FixtureRateClock::Monotonic
        })
    ));
    let foreign = FixtureRateClockContext::new(label(109)?, label(103)?, label(107)?);
    assert!(matches!(
        retired.resume(
            FixturePhaseNow::new(
                foreign,
                ActiveAwakeNs::new(4 * SECOND),
                MonotonicNs::new(8 * SECOND)
            ),
            FixturePhaseResidualPermission::Allow
        ),
        Err(FixturePhaseError::ClockContextMismatch)
    ));
    assert_eq!(
        retired.current_time().map(|value| value.active()),
        Some(ActiveAwakeNs::new(3 * SECOND))
    );
    let mut next = product_ok(retired.resume(
        now(3 * SECOND, 7 * SECOND)?,
        FixturePhaseResidualPermission::Allow,
    ))?;
    assert!(next.proposal().is_none());
    let proposal = product_ok(next.issue(
        now(3 * SECOND, 7 * SECOND)?,
        FixturePhaseResidualPermission::Allow,
    ))?;
    assert_eq!(proposal.transient_rate(), 3.0 / 640.0);
    assert_eq!(proposal.ordinal(), label(2)?);
    Ok(())
}

/// Unknown/changed baselines preserve aggregate settlement and allow only explicit fresh context.
/// # Errors
/// Preserve real setup, known settlement and the explicit current-baseline transfer.
/// # Panics
/// Reject a fake debt, discarded interval or restoration of an unavailable old baseline.
#[test]
fn missing_or_changed_effective_baseline_retains_known_history_without_debt() -> TestResult {
    for changed in [false, true] {
        let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
        let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
        let chart = initial_chart()?;
        let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
        let strata = strata()?;
        let batch = product_ok(FixtureChartBatch::new(
            &chart,
            MonotonicNs::new(4 * SECOND),
            &entries,
        ))?;
        let mut producer = seed(&entries, &strata, 4)?;
        let mut owner = start(&mut producer, &chart, &batch, &strata, 1.0 / 1024.0)?;
        check_issue(&mut owner, (0, 4 * SECOND), 1, (1.0 / 256.0, 5.0 / 1024.0))?;
        let event = interval(
            &chart,
            clocks()?,
            1,
            (0, 2 * SECOND),
            (4 * SECOND, 6 * SECOND),
            5.0 / 1024.0,
            (31, 7),
        )?;
        let supplied = if changed {
            Some(baseline(3.0 / 2048.0)?)
        } else {
            None
        };
        let mut retired = known(
            owner,
            FixtureRateSettlement::Completed(event),
            supplied,
            now(2 * SECOND, 6 * SECOND)?,
        )?;
        assert_eq!(retired.remaining_phase_seconds(), None);
        assert_eq!(retired.last_known_phase_seconds(), Some(1.0 / 32.0));
        assert_eq!(
            retired.replacement()?.phase_at_reference_seconds(),
            5.0 / 512.0
        );
        assert_eq!(
            retired
                .cursor()
                .and_then(FixtureRateSettlementCursor::next_ordinal),
            Some(label(2)?)
        );
        assert_eq!(
            retired.debt_error(),
            Some(&if changed {
                FixturePhaseError::BaselineChanged
            } else {
                FixturePhaseError::BaselineUnavailable
            })
        );
        assert!(matches!(
            retired.resume(
                now(2 * SECOND, 6 * SECOND)?,
                FixturePhaseResidualPermission::Allow
            ),
            Err(FixturePhaseError::NoReference)
        ));
        let mut fresh = product_ok(
            retired.resume_fresh(now(2 * SECOND, 6 * SECOND)?, baseline(3.0 / 2048.0)?),
        )?;
        assert_eq!(fresh.remaining_phase_seconds(), None);
        assert!(matches!(
            fresh.issue(
                now(2 * SECOND, 6 * SECOND)?,
                FixturePhaseResidualPermission::Allow
            ),
            Err(FixturePhaseError::NoReference)
        ));
        assert_eq!(
            fresh.cursor().last_active_end(),
            Some(ActiveAwakeNs::new(2 * SECOND))
        );
        let new_a = Source::new(11, 4, 13 << 23)?;
        let new_b = Source::new(22, 4, 13 << 23)?;
        receive_serialized_refresh(
            &mut fresh,
            &a,
            &new_a,
            &new_b,
            &strata,
            (2 * SECOND, 8 * SECOND),
        )?;
        let proposal = product_ok(fresh.issue(
            now(2 * SECOND, 8 * SECOND)?,
            FixturePhaseResidualPermission::Deny,
        ))?;
        assert_eq!(proposal.transient_rate(), 1.0 / 512.0);
        assert_eq!(proposal.aggregate_rate(), 7.0 / 2048.0);
    }
    Ok(())
}

/// One finite actual-delivery case with a separate expected continuation result.
struct DeliveryCase {
    /// Actual aggregate, independent of the requested transient cap.
    aggregate: f64,
    /// Actual active duration and independently supplied monotonic endpoint.
    spans: ((u64, u64), (u64, u64)),
    /// Exact baseline-relative remainder from the declared rational facts.
    phase: f64,
    /// A policy retirement after committed history, or a useful neighboring result.
    refusal: Option<FixturePhaseError>,
}

/// Actual aggregate violations and overshoot remain historical; qmax is a request cap.
/// # Errors
/// Preserve genuine setup, typed event and useful-neighbor continuation failures.
/// # Panics
/// Reject rollback, automatic reversal or applying a requested-rate cap to actual delivery.
#[test]
fn delivery_retirements_preserve_debt_and_actual_transient_need_not_equal_request() -> TestResult {
    for case in [
        DeliveryCase {
            aggregate: 0.0,
            spans: ((0, 2 * SECOND), (4 * SECOND, 6 * SECOND)),
            phase: 17.0 / 512.0,
            refusal: Some(FixturePhaseError::DeliveredOppositeDirection),
        },
        DeliveryCase {
            aggregate: 10.0 / 1024.0,
            spans: ((0, 4 * SECOND), (4 * SECOND, 8 * SECOND)),
            phase: -1.0 / 256.0,
            refusal: Some(FixturePhaseError::DeliveredOvershoot),
        },
        DeliveryCase {
            aggregate: 1.0 / 32.0,
            spans: ((0, 15_625_000), (4 * SECOND, 4 * SECOND + 15_625_000)),
            phase: 2017.0 / 65536.0,
            refusal: Some(FixturePhaseError::DeliveredOutOfEnvelope),
        },
        DeliveryCase {
            aggregate: 12.0 / 1024.0,
            spans: ((0, 15_625_000), (4 * SECOND, 4 * SECOND + 15_625_000)),
            phase: 2037.0 / 65536.0,
            refusal: None,
        },
    ] {
        let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
        let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
        let chart = initial_chart()?;
        let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
        let strata = strata()?;
        let batch = product_ok(FixtureChartBatch::new(
            &chart,
            MonotonicNs::new(4 * SECOND),
            &entries,
        ))?;
        let mut producer = seed(&entries, &strata, 4)?;
        let mut owner = start(&mut producer, &chart, &batch, &strata, 1.0 / 1024.0)?;
        check_issue(&mut owner, (0, 4 * SECOND), 1, (1.0 / 256.0, 5.0 / 1024.0))?;
        let mut retired = settle_interval(
            owner,
            1,
            case.spans,
            case.aggregate,
            (31, 7),
            false,
            1.0 / 1024.0,
        )?;
        check_debt(
            retired.remaining_phase_seconds(),
            retired.remaining_phase_error_seconds(),
            case.phase,
        );
        assert_eq!(
            retired
                .cursor()
                .and_then(FixtureRateSettlementCursor::last_active_end),
            Some(ActiveAwakeNs::new(case.spans.0.1))
        );
        if let Some(refusal) = case.refusal {
            assert_eq!(
                retired
                    .resume(
                        now(case.spans.0.1, case.spans.1.1)?,
                        FixturePhaseResidualPermission::Allow
                    )
                    .err(),
                Some(refusal)
            );
        } else {
            let mut next = product_ok(retired.resume(
                now(case.spans.0.1, case.spans.1.1)?,
                FixturePhaseResidualPermission::Allow,
            ))?;
            let proposal = product_ok(next.issue(
                now(case.spans.0.1, case.spans.1.1)?,
                FixturePhaseResidualPermission::Allow,
            ))?;
            assert_eq!(proposal.ordinal(), label(2)?);
            assert!(proposal.transient_rate() > 0.0 && proposal.transient_rate() <= 1.0 / 128.0);
        }
    }
    Ok(())
}

/// Completing the phase early preserves the baseline's negative-zero bits.
/// # Errors
/// Preserve real setup, actual-rate settlement and no-useful-transient issuance.
/// # Panics
/// Reject a zero-error claim, new transient or normalization of the known baseline.
#[test]
fn zero_nominal_debt_keeps_its_bound_and_the_original_signed_zero_baseline() -> TestResult {
    let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
    let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let mut owner = start(&mut producer, &chart, &batch, &strata, -0.0)?;
    let original_error = owner.remaining_phase_error_seconds();
    check_issue(&mut owner, (0, 4 * SECOND), 1, (1.0 / 256.0, 1.0 / 256.0))?;
    let mut retired = settle_interval(
        owner,
        1,
        ((0, 4 * SECOND), (4 * SECOND, 8 * SECOND)),
        1.0 / 128.0,
        (31, 7),
        false,
        -0.0,
    )?;
    check_debt(
        retired.remaining_phase_seconds(),
        retired.remaining_phase_error_seconds(),
        0.0,
    );
    assert!(retired.remaining_phase_error_seconds() >= original_error);
    let mut next = product_ok(retired.resume(
        now(4 * SECOND, 8 * SECOND)?,
        FixturePhaseResidualPermission::Allow,
    ))?;
    let proposal = product_ok(next.issue(
        now(4 * SECOND, 8 * SECOND)?,
        FixturePhaseResidualPermission::Allow,
    ))?;
    assert!(!proposal.has_useful_transient());
    assert_eq!(proposal.aggregate_rate().to_bits(), (-0.0_f64).to_bits());
    assert_eq!(proposal.ordinal(), label(2)?);
    Ok(())
}

/// Foreign chart and outstanding-operation rejection cannot consume fresh producer credit.
/// # Errors
/// Preserve real histories, batches, producer admission and the healthy fresh neighbor.
/// # Panics
/// Reject foreign instance admission, changed clocks or preemptive emission consumption.
#[test]
fn foreign_chart_and_outstanding_reference_reject_before_producer_advance() -> TestResult {
    let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
    let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
    let chart = initial_chart()?;
    let foreign = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let foreign_entries = [a.entry(&foreign, 4)?, b.entry(&foreign, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let foreign_batch = product_ok(FixtureChartBatch::new(
        &foreign,
        MonotonicNs::new(4 * SECOND),
        &foreign_entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let mut owner = start(&mut producer, &chart, &batch, &strata, 1.0 / 1024.0)?;
    let before = cursor_snapshot(owner.cursor());
    assert!(matches!(
        owner.receive_reference(&foreign_batch, &strata, now(0, 4 * SECOND)?),
        Err(FixturePhaseError::Reference {
            source: FixtureReferenceInputError::ForeignChartView
        })
    ));
    check_debt(
        owner.remaining_phase_seconds(),
        owner.remaining_phase_error_seconds(),
        1.0 / 32.0,
    );
    check_issue(&mut owner, (0, 4 * SECOND), 1, (1.0 / 256.0, 5.0 / 1024.0))?;
    let new_a = Source::new(11, 4, 1 << 26)?;
    let new_b = Source::new(22, 4, 1 << 26)?;
    let new_entries = [new_a.entry(&chart, 8)?, new_b.entry(&chart, 8)?];
    let new_batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(8 * SECOND),
        &new_entries,
    ))?;
    assert!(matches!(
        owner.receive_reference(&new_batch, &strata, now(0, 8 * SECOND)?),
        Err(FixturePhaseError::OutstandingOperation)
    ));
    assert_eq!(
        owner.current_time().monotonic(),
        MonotonicNs::new(4 * SECOND)
    );
    assert_eq!(cursor_snapshot(owner.cursor()), before);
    owner = no_effect(owner, 1, now(0, 4 * SECOND)?)?;
    assert!(owner.proposal().is_none());
    check_multiple_updates_rejected(&mut owner, &new_batch, &strata, now(0, 8 * SECOND)?);
    receive_serialized_refresh(&mut owner, &a, &new_a, &new_b, &strata, (0, 8 * SECOND))?;
    check_debt(
        owner.remaining_phase_seconds(),
        owner.remaining_phase_error_seconds(),
        1.0 / 64.0,
    );
    assert_eq!(owner.cursor().next_ordinal(), Some(label(2)?));
    Ok(())
}

/// Initial constructor failure returns its actual cursor; a valid neighbor emits round one.
/// # Errors
/// Preserve genuine setup and valid construction/reference admission failures.
/// # Panics
/// Reject cursor loss, producer advancement or a useful owner from foreign time context.
#[test]
fn rejected_construction_returns_the_cursor_and_leaves_the_producer_unadvanced() -> TestResult {
    let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
    let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let cursor = FixtureRateSettlementCursor::new(&chart, clocks()?, label(1)?);
    let before = cursor_snapshot(&cursor);
    let foreign = FixtureRateClockContext::new(label(109)?, label(103)?, label(107)?);
    let attempted = FixturePhasePlanOwner::new(
        &mut producer,
        &chart,
        cursor,
        FixturePhaseNow::new(foreign, ActiveAwakeNs::new(0), MonotonicNs::new(4 * SECOND)),
        profile()?,
        baseline(1.0 / 1024.0)?,
    );
    assert!(attempted.is_err());
    let Err(failure) = attempted else {
        return Err(io::Error::other("foreign construction unexpectedly admitted").into());
    };
    check_construction_diagnostics(&failure);
    let (cursor, error) = failure.into_parts();
    assert_eq!(error, FixturePhaseError::ClockContextMismatch);
    assert_eq!(cursor_snapshot(&cursor), before);
    let mut owner = product_ok(FixturePhasePlanOwner::new(
        &mut producer,
        &chart,
        cursor,
        now(0, 4 * SECOND)?,
        profile()?,
        baseline(1.0 / 1024.0)?,
    ))?;
    assert!(matches!(
        owner.issue(now(0, 4 * SECOND)?, FixturePhaseResidualPermission::Allow),
        Err(FixturePhaseError::NoReference)
    ));
    assert!(
        matches!(product_ok(owner.receive_reference(&batch, &strata, now(0, 4 * SECOND)?))?, FixturePhaseReferenceAdmission::Accepted { emitted_round } if emitted_round == label(1)?)
    );
    check_issue(&mut owner, (0, 4 * SECOND), 1, (1.0 / 256.0, 5.0 / 1024.0))?;
    Ok(())
}

/// A phase-envelope refusal commits its real emission but grants no useful plan.
/// # Errors
/// Preserve genuine fixture, construction, producer and fresh-neighbor failures.
/// # Panics
/// Reject manufactured rollback, reused emission credit or admission of oversized debt.
#[test]
fn admitted_no_plan_retains_credit_until_genuinely_new_eligible_rows_arrive() -> TestResult {
    let a = Source::new(11, 0, 1 << 29)?;
    let b = Source::new(22, 0, 1 << 29)?;
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let mut owner = product_ok(FixturePhasePlanOwner::new(
        &mut producer,
        &chart,
        FixtureRateSettlementCursor::new(&chart, clocks()?, label(1)?),
        now(0, 4 * SECOND)?,
        profile()?,
        baseline(1.0 / 1024.0)?,
    ))?;
    let refused = product_ok(owner.receive_reference(&batch, &strata, now(0, 4 * SECOND)?))?;
    assert!(
        matches!(refused, FixturePhaseReferenceAdmission::AdmittedNoPlan { emitted_round: Some(round), reason: FixturePhaseError::PhaseEnvelope } if round == label(1)?)
    );
    assert!(owner.proposal().is_none());
    assert_eq!(owner.remaining_phase_seconds(), None);
    assert_eq!(owner.last_known_phase_seconds(), Some(1.0 / 8.0));
    assert!(matches!(
        product_ok(owner.receive_reference(&batch, &strata, now(0, 4 * SECOND)?))?,
        FixturePhaseReferenceAdmission::AdmittedNoPlan {
            emitted_round: None,
            reason: FixturePhaseError::Reference {
                source: FixtureReferenceInputError::NotReady
            }
        }
    ));
    let new_a = Source::new(11, 4, ORIGINAL_PHASE_Q32)?;
    let new_b = Source::new(22, 4, ORIGINAL_PHASE_Q32)?;
    receive_serialized_refresh(&mut owner, &a, &new_a, &new_b, &strata, (0, 8 * SECOND))?;
    check_debt(
        owner.remaining_phase_seconds(),
        owner.remaining_phase_error_seconds(),
        1.0 / 32.0,
    );
    assert_eq!(owner.cursor().next_ordinal(), Some(label(1)?));
    Ok(())
}

/// The last local ordinal is consumed without wrapping or refunding issuance.
/// # Errors
/// Preserve genuine construction, producer admission and no-effect receiving failures.
/// # Panics
/// Reject a reset cursor, invented endpoint or a proposal after terminal exhaustion.
#[test]
fn ordinal_exhaustion_has_a_useful_last_neighbor_and_no_reset_path() -> TestResult {
    for ordinal in [u64::MAX - 1, u64::MAX] {
        let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
        let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
        let chart = initial_chart()?;
        let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
        let strata = strata()?;
        let batch = product_ok(FixtureChartBatch::new(
            &chart,
            MonotonicNs::new(4 * SECOND),
            &entries,
        ))?;
        let mut producer = seed(&entries, &strata, 4)?;
        let mut owner = product_ok(FixturePhasePlanOwner::new(
            &mut producer,
            &chart,
            FixtureRateSettlementCursor::new(&chart, clocks()?, label(ordinal)?),
            now(0, 4 * SECOND)?,
            profile()?,
            baseline(1.0 / 1024.0)?,
        ))?;
        assert!(
            matches!(product_ok(owner.receive_reference(&batch, &strata, now(0, 4 * SECOND)?))?, FixturePhaseReferenceAdmission::Accepted { emitted_round } if emitted_round == label(1)?)
        );
        check_issue(
            &mut owner,
            (0, 4 * SECOND),
            ordinal,
            (1.0 / 256.0, 5.0 / 1024.0),
        )?;
        owner = no_effect(owner, ordinal, now(0, 4 * SECOND)?)?;
        assert!(owner.proposal().is_none());
        assert_eq!(owner.cursor().last_active_end(), None);
        assert_eq!(owner.cursor().last_monotonic_end(), None);
        if ordinal == u64::MAX {
            assert_eq!(owner.cursor().status(), FixtureRateCursorStatus::Exhausted);
            assert_eq!(owner.cursor().next_ordinal(), None);
            assert!(matches!(
                owner.issue(now(0, 4 * SECOND)?, FixturePhaseResidualPermission::Allow),
                Err(FixturePhaseError::CursorUnavailable)
            ));
        } else {
            assert_eq!(owner.cursor().status(), FixtureRateCursorStatus::Pending);
            check_issue(
                &mut owner,
                (0, 4 * SECOND),
                u64::MAX,
                (1.0 / 256.0, 5.0 / 1024.0),
            )?;
        }
    }
    Ok(())
}

/// Raw finite-profile and baseline admission remains mandatory with useful neighbors.
/// # Errors
/// Preserve actual same-domain duration and supported neighboring construction failures.
/// # Panics
/// Reject zero/unsupported bounds, nonfinite rates or normalized signed-zero inputs.
#[test]
fn raw_profile_and_baseline_refusals_keep_supported_boundary_neighbors() -> TestResult {
    let active = ActiveAwakeNs::new(8 * SECOND).duration_since(ActiveAwakeNs::new(0))?;
    let monotonic = MonotonicNs::new(16 * SECOND).duration_since(MonotonicNs::new(0))?;
    for bad in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::MIN_POSITIVE / 2.0] {
        for caps in [
            [bad, 1.0 / 128.0, 1.0 / 64.0],
            [1.0 / 16.0, bad, 1.0 / 64.0],
            [1.0 / 16.0, 1.0 / 128.0, bad],
        ] {
            assert!(matches!(
                FixturePhaseProfile::new(active, monotonic, caps[0], caps[1], caps[2]),
                Err(FixturePhaseError::InvalidProfile)
            ));
        }
    }
    for nanoseconds in [0, (1_u64 << 53) + 1] {
        let unsupported = ActiveAwakeNs::new(nanoseconds).duration_since(ActiveAwakeNs::new(0))?;
        assert!(matches!(
            FixturePhaseProfile::new(unsupported, monotonic, 1.0 / 16.0, 1.0 / 128.0, 1.0 / 64.0),
            Err(FixturePhaseError::InvalidProfile)
        ));
    }
    for nanoseconds in [1, 1_u64 << 53] {
        let supported = ActiveAwakeNs::new(nanoseconds).duration_since(ActiveAwakeNs::new(0))?;
        let admitted = product_ok(FixturePhaseProfile::new(
            supported,
            monotonic,
            1.0 / 16.0,
            1.0 / 128.0,
            1.0 / 64.0,
        ))?;
        assert_eq!(format!("{admitted:?}"), "FixturePhaseProfile { .. }");
    }
    let zero_monotonic = MonotonicNs::new(0).duration_since(MonotonicNs::new(0))?;
    assert!(matches!(
        FixturePhaseProfile::new(active, zero_monotonic, 1.0 / 16.0, 1.0 / 128.0, 1.0 / 64.0),
        Err(FixturePhaseError::InvalidProfile)
    ));
    for bad in [f64::NAN, f64::INFINITY, f64::MIN_POSITIVE / 2.0] {
        assert!(matches!(
            FixturePhaseBaseline::new(bad),
            Err(FixturePhaseError::Chart { .. })
        ));
    }
    for value in [0.0_f64, -0.0_f64, 1.0 / 1024.0] {
        let admitted = baseline(value)?;
        assert_eq!(admitted.rate().to_bits(), value.to_bits());
        assert_eq!(format!("{admitted:?}"), "FixturePhaseBaseline { .. }");
    }
    Ok(())
}

/// Historical getters retain both original independently declared deadlines.
/// # Panics
/// Reject reset deadlines on a genuine original-obligation retirement.
fn check_original_retirement_deadlines(retirement: &FixturePhaseRetirement<'_>) {
    assert_eq!(
        retirement.active_deadline(),
        Some(ActiveAwakeNs::new(8 * SECOND))
    );
    assert_eq!(
        retirement.monotonic_deadline(),
        Some(MonotonicNs::new(20 * SECOND))
    );
}

/// Construction diagnostics expose a family without cursor, source or clock data.
/// # Panics
/// Reject unbounded text, leaked state or a fabricated delegated cause.
fn check_construction_diagnostics(failure: &FixturePhaseConstructionRejected) {
    assert_eq!(
        format!("{failure:?}"),
        "FixturePhaseConstructionRejected { .. }"
    );
    let display = failure.to_string();
    assert!(display.starts_with("fixture phase: "));
    assert!(display.is_ascii() && display.len() <= 96);
    assert!(failure.source().is_none());
}

/// Recover only an actual constructor rejection and its unchanged owned cursor.
/// # Errors
/// Preserve unexpected success without fabricating returned custody.
/// # Panics
/// Reject acceptance, the wrong typed refusal or revealing diagnostics.
fn rejected_construction(
    result: Result<FixturePhasePlanOwner<'_, '_>, FixturePhaseConstructionRejected>,
    expected: &FixturePhaseError,
) -> TestResult<FixtureRateSettlementCursor> {
    assert!(result.is_err());
    let Err(failure) = result else {
        return Err(io::Error::other("invalid phase construction succeeded").into());
    };
    check_construction_diagnostics(&failure);
    let (cursor, error) = failure.into_parts();
    assert_eq!(&error, expected);
    Ok(cursor)
}

/// Closed error values are diagnostic fixtures, not fabricated runtime outcomes.
fn diagnostic_families() -> [FixturePhaseError; 25] {
    [
        FixturePhaseError::InvalidProfile,
        FixturePhaseError::ValidationInvariant,
        FixturePhaseError::ClockContextMismatch,
        FixturePhaseError::TimeRegression {
            clock: FixtureRateClock::ActiveAwake,
        },
        FixturePhaseError::ClockOverflow,
        FixturePhaseError::CursorUnavailable,
        FixturePhaseError::CursorMappingMismatch,
        FixturePhaseError::OutstandingOperation,
        FixturePhaseError::NoOutstandingOperation,
        FixturePhaseError::NoReference,
        FixturePhaseError::Expired,
        FixturePhaseError::Retired {
            reason: FixturePhaseRetirementReason::Stop,
        },
        FixturePhaseError::ResidualPermission,
        FixturePhaseError::BaselineUnavailable,
        FixturePhaseError::BaselineChanged,
        FixturePhaseError::PhaseEnvelope,
        FixturePhaseError::DeliveredOutOfEnvelope,
        FixturePhaseError::DeliveredOppositeDirection,
        FixturePhaseError::DeliveredOvershoot,
        FixturePhaseError::TransferConsumed,
        FixturePhaseError::Chart {
            source: FixtureChartError::Invariant,
        },
        FixturePhaseError::Reference {
            source: FixtureReferenceInputError::NotReady,
        },
        FixturePhaseError::Delivery {
            source: FixtureRateDeliveryError::OrdinalMismatch,
        },
        FixturePhaseError::Combination {
            source: FixtureChartCombinationError::Invariant,
        },
        FixturePhaseError::Numeric {
            source: FixturePhaseNumericError::AggregateLimit,
        },
    ]
}

/// Delegation preserves the exact typed cause rather than a flattened family.
/// # Panics
/// Reject a lost, foreign or spuriously attached source in the public error trait.
fn check_phase_error_source(error: &FixturePhaseError) {
    let actual = error.source();
    match error {
        FixturePhaseError::Chart { source } => assert!(
            actual
                .is_some_and(|value| { value.downcast_ref::<FixtureChartError>() == Some(source) })
        ),
        FixturePhaseError::Reference { source } => assert!(actual.is_some_and(|value| {
            value.downcast_ref::<FixtureReferenceInputError>() == Some(source)
        })),
        FixturePhaseError::Delivery { source } => assert!(actual.is_some_and(|value| {
            value.downcast_ref::<FixtureRateDeliveryError>() == Some(source)
        })),
        FixturePhaseError::Combination { source } => assert!(actual.is_some_and(|value| {
            value.downcast_ref::<FixtureChartCombinationError>() == Some(source)
        })),
        FixturePhaseError::Numeric { source } => assert!(actual.is_some_and(|value| {
            value.downcast_ref::<FixturePhaseNumericError>() == Some(source)
        })),
        _ => assert!(actual.is_none()),
    }
}

/// Distinguish bounded refusal families without selecting their English wording.
/// # Panics
/// Reject leaked payloads, merged families, unbounded diagnostics or wrong causes.
#[test]
fn lifecycle_error_families_keep_bounded_redacted_diagnostics_and_typed_causes() {
    let mut rendered = std::collections::BTreeSet::new();
    for error in diagnostic_families() {
        assert_eq!(format!("{error:?}"), "FixturePhaseError { .. }");
        let display = error.to_string();
        assert!(display.starts_with("fixture phase: "));
        assert!(display.is_ascii() && display.len() <= 96);
        assert!(rendered.insert(display));
        check_phase_error_source(&error);
    }
    assert_eq!(rendered.len(), 25);
    // A public nested diagnostic may contain an arbitrary index, never wire data
    // or authority. The parent keeps it in the typed source, not rendered text.
    let indexed = FixturePhaseError::Combination {
        source: FixtureChartCombinationError::MissingTracking { index: usize::MAX },
    };
    let indexed_display = indexed.to_string();
    assert!(indexed_display.starts_with("fixture phase: "));
    assert!(indexed_display.is_ascii() && indexed_display.len() <= 96);
    assert!(!indexed_display.contains(&usize::MAX.to_string()));
    assert_eq!(format!("{indexed:?}"), "FixturePhaseError { .. }");
    check_phase_error_source(&indexed);
}

/// Receive an actual known interval and retain its real replacement plus cursor.
/// # Errors
/// Preserve genuine clock, interval, terminal and replacement admission failures.
/// # Panics
/// Reject a successful known terminal that omits its replacement.
fn settled_cursor_and_chart(
    chart: &FixtureAffineChart,
) -> TestResult<(FixtureRateSettlementCursor, FixtureAffineChart)> {
    let mut cursor = FixtureRateSettlementCursor::new(chart, clocks()?, label(1)?);
    let mut readiness = FixtureReferenceInputState::new(chart);
    let event = FixtureRateSettlement::Completed(interval(
        chart,
        clocks()?,
        1,
        (0, 2 * SECOND),
        (4 * SECOND, 6 * SECOND),
        5.0 / 1024.0,
        (31, 7),
    )?);
    let completed = product_ok(readiness.receive_rate_settlement(&mut cursor, chart, event))?;
    let replacement = product_ok(completed.into_replacement())?;
    assert!(replacement.is_some());
    let Some(replacement) = replacement else {
        return Err(io::Error::other("known rate omitted its genuine replacement").into());
    };
    Ok((cursor, replacement))
}

/// Literal independent clock/chart/baseline constructor refusal vector.
type ConstructorCase = (u64, u64, bool, f64, FixturePhaseError);

/// Each row changes one precondition of the genuinely settled cursor.
fn constructor_refusal_cases() -> [ConstructorCase; 4] {
    [
        (
            SECOND,
            6 * SECOND,
            false,
            1.0 / 1024.0,
            FixturePhaseError::TimeRegression {
                clock: FixtureRateClock::ActiveAwake,
            },
        ),
        (
            2 * SECOND,
            5 * SECOND,
            false,
            1.0 / 1024.0,
            FixturePhaseError::TimeRegression {
                clock: FixtureRateClock::Monotonic,
            },
        ),
        (
            2 * SECOND,
            6 * SECOND,
            true,
            1.0 / 1024.0,
            FixturePhaseError::CursorMappingMismatch,
        ),
        (
            2 * SECOND,
            6 * SECOND,
            false,
            1.0 / 32.0,
            FixturePhaseError::DeliveredOutOfEnvelope,
        ),
    ]
}

/// Real settled endpoints constrain new owners before any producer emission.
/// # Errors
/// Preserve real setup, settlement, cursor recovery and the healthy construction.
/// # Panics
/// Reject clock/mapping/base bypasses, changed custody or consumed source credit.
#[test]
fn settled_cursor_constructor_refusals_preserve_custody_and_first_reference_credit() -> TestResult {
    let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
    let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let mut producer = seed(&entries, &strata, 4)?;
    let (mut cursor, replacement) = settled_cursor_and_chart(&chart)?;
    let before = cursor_snapshot(&cursor);
    for (active, monotonic, old_chart, base, expected) in constructor_refusal_cases() {
        let attempted = FixturePhasePlanOwner::new(
            &mut producer,
            if old_chart { &chart } else { &replacement },
            cursor,
            now(active, monotonic)?,
            profile()?,
            baseline(base)?,
        );
        cursor = rejected_construction(attempted, &expected)?;
        assert_eq!(cursor_snapshot(&cursor), before);
    }
    let entries = [a.entry(&replacement, 6)?, b.entry(&replacement, 6)?];
    let batch = product_ok(FixtureChartBatch::new(
        &replacement,
        MonotonicNs::new(6 * SECOND),
        &entries,
    ))?;
    let mut owner = product_ok(FixturePhasePlanOwner::new(
        &mut producer,
        &replacement,
        cursor,
        now(2 * SECOND, 6 * SECOND)?,
        profile()?,
        baseline(1.0 / 1024.0)?,
    ))?;
    assert_eq!(cursor_snapshot(owner.cursor()), before);
    assert!(matches!(product_ok(owner.receive_reference(
        &batch, &strata, now(2 * SECOND, 6 * SECOND)?,
    ))?, FixturePhaseReferenceAdmission::Accepted { emitted_round } if emitted_round == label(1)?));
    check_debt(
        owner.remaining_phase_seconds(),
        owner.remaining_phase_error_seconds(),
        11.0 / 512.0,
    );
    Ok(())
}

/// Checked deadline addition accepts the exact endpoint and rejects its neighbor.
/// # Errors
/// Preserve genuine setup, cursor recovery and exact-boundary construction.
/// # Panics
/// Reject overflow, changed cursor data or an invented initial reference.
#[test]
fn constructor_deadline_overflow_has_independent_exact_clock_neighbors() -> TestResult {
    let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
    let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let mut producer = seed(&entries, &strata, 4)?;
    for active_clock in [true, false] {
        let cursor = FixtureRateSettlementCursor::new(&chart, clocks()?, label(1)?);
        let before = cursor_snapshot(&cursor);
        let (active, monotonic) = if active_clock {
            (u64::MAX - 8 * SECOND, 4 * SECOND)
        } else {
            (0, u64::MAX - 16 * SECOND)
        };
        let rejected_at = now(
            active + u64::from(active_clock),
            monotonic + u64::from(!active_clock),
        )?;
        let attempted = FixturePhasePlanOwner::new(
            &mut producer,
            &chart,
            cursor,
            rejected_at,
            profile()?,
            baseline(1.0 / 1024.0)?,
        );
        let cursor = rejected_construction(attempted, &FixturePhaseError::ClockOverflow)?;
        assert_eq!(cursor_snapshot(&cursor), before);
        let owner = product_ok(FixturePhasePlanOwner::new(
            &mut producer,
            &chart,
            cursor,
            now(active, monotonic)?,
            profile()?,
            baseline(1.0 / 1024.0)?,
        ))?;
        assert!(owner.provenance().is_none());
        assert!(owner.proposal().is_none());
        assert_eq!(owner.active_deadline(), None);
        assert_eq!(owner.monotonic_deadline(), None);
        assert_eq!(cursor_snapshot(owner.cursor()), before);
    }
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let outcome = product_ok(producer.advance_chart(&chart, &batch, &strata))?;
    assert_eq!(outcome.emitted_round(), Some(label(1)?));
    Ok(())
}

/// `Unknown` and exhausted cursors arise only from their actual terminal receiver.
/// # Errors
/// Preserve genuine setup, terminal execution and rejected-cursor recovery.
/// # Panics
/// Reject renewed cursor authority, invented endpoints or consumed source credit.
#[test]
fn terminal_cursor_construction_cannot_restore_unknown_or_exhausted_authority() -> TestResult {
    for unknown in [true, false] {
        let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
        let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
        let chart = initial_chart()?;
        let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
        let strata = strata()?;
        let mut producer = seed(&entries, &strata, 4)?;
        let mut cursor = FixtureRateSettlementCursor::new(&chart, clocks()?, label(u64::MAX)?);
        let mut readiness = FixtureReferenceInputState::new(&chart);
        let event = if unknown {
            FixtureRateSettlement::Unknown {
                context: clocks()?,
                ordinal: label(u64::MAX)?,
            }
        } else {
            FixtureRateSettlement::ProvenNoEffect {
                context: clocks()?,
                ordinal: label(u64::MAX)?,
            }
        };
        let completion = product_ok(readiness.receive_rate_settlement(&mut cursor, &chart, event))?;
        assert_eq!(
            completion.kind(),
            if unknown {
                time_core::FixtureRateSettlementKind::Unknown
            } else {
                time_core::FixtureRateSettlementKind::ProvenNoEffect
            }
        );
        assert!(product_ok(completion.replacement())?.is_none());
        assert_eq!(
            cursor.status(),
            if unknown {
                FixtureRateCursorStatus::Unresolved
            } else {
                FixtureRateCursorStatus::Exhausted
            }
        );
        let before = cursor_snapshot(&cursor);
        let attempted = FixturePhasePlanOwner::new(
            &mut producer,
            &chart,
            cursor,
            now(0, 4 * SECOND)?,
            profile()?,
            baseline(1.0 / 1024.0)?,
        );
        let cursor = rejected_construction(attempted, &FixturePhaseError::CursorUnavailable)?;
        assert_eq!(cursor_snapshot(&cursor), before);
        assert_eq!(cursor.next_ordinal(), None);
        assert_eq!(cursor.last_active_end(), None);
        assert_eq!(cursor.last_monotonic_end(), None);
        let batch = product_ok(FixtureChartBatch::new(
            &chart,
            MonotonicNs::new(4 * SECOND),
            &entries,
        ))?;
        let outcome = product_ok(producer.advance_chart(&chart, &batch, &strata))?;
        assert_eq!(outcome.emitted_round(), Some(label(1)?));
    }
    Ok(())
}

/// Exact issue expiry and no-effect permission never silently renew the budget.
/// # Errors
/// Preserve real producer, operation, no-effect and fixed-clock setup failures.
/// # Panics
/// Reject deadline equality, rewind, hidden issuance or fabricated known delivery.
#[test]
fn issue_deadlines_and_no_effect_permission_preserve_original_operation_budget() -> TestResult {
    for (near, expiry, clock, transient, aggregate) in [
        (
            (7 * SECOND, 4 * SECOND),
            (8 * SECOND, 4 * SECOND),
            FixtureRateClock::ActiveAwake,
            1.0 / 128.0,
            9.0 / 1024.0,
        ),
        (
            (0, 19 * SECOND),
            (0, 20 * SECOND),
            FixtureRateClock::Monotonic,
            1.0 / 256.0,
            5.0 / 1024.0,
        ),
    ] {
        let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
        let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
        let chart = initial_chart()?;
        let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
        let strata = strata()?;
        let batch = product_ok(FixtureChartBatch::new(
            &chart,
            MonotonicNs::new(4 * SECOND),
            &entries,
        ))?;
        let mut producer = seed(&entries, &strata, 4)?;
        let owner = start(&mut producer, &chart, &batch, &strata, 1.0 / 1024.0)?;
        check_issue_budget(owner, near, expiry, clock, (transient, aggregate))?;
    }
    Ok(())
}

/// Exercise real no-effect permission and exact expiry on one actual owner.
/// # Errors
/// Preserve actual issuance, terminal, typed-time and fixed-label setup failures.
/// # Panics
/// Reject new delivery endpoints, lost ordinal history or expired authority reuse.
fn check_issue_budget(
    mut owner: FixturePhasePlanOwner<'_, '_>,
    near: (u64, u64),
    expiry: (u64, u64),
    clock: FixtureRateClock,
    rates: (f64, f64),
) -> TestResult {
    assert_eq!(
        format!("{:?}", now(near.0, near.1)?),
        "FixturePhaseNow { .. }"
    );
    check_issue(&mut owner, near, 1, rates)?;
    owner = no_effect(owner, 1, now(near.0, near.1)?)?;
    let before = cursor_snapshot(owner.cursor());
    assert!(matches!(
        owner.issue(now(near.0, near.1)?, FixturePhaseResidualPermission::Deny),
        Err(FixturePhaseError::ResidualPermission)
    ));
    assert!(owner.proposal().is_none());
    assert_eq!(cursor_snapshot(owner.cursor()), before);
    check_issue(&mut owner, near, 2, rates)?;
    owner = no_effect(owner, 2, now(near.0, near.1)?)?;
    assert_eq!(owner.cursor().next_ordinal(), Some(label(3)?));
    assert!(matches!(
        owner.issue(
            now(expiry.0, expiry.1)?,
            FixturePhaseResidualPermission::Allow
        ),
        Err(FixturePhaseError::Expired)
    ));
    assert!(matches!(
        owner.issue(now(near.0, near.1)?, FixturePhaseResidualPermission::Allow),
        Err(FixturePhaseError::TimeRegression { clock: actual }) if actual == clock
    ));
    assert_eq!(owner.current_time().active(), ActiveAwakeNs::new(expiry.0));
    assert_eq!(owner.current_time().monotonic(), MonotonicNs::new(expiry.1));
    assert!(owner.proposal().is_none());
    assert_eq!(owner.cursor().next_ordinal(), Some(label(3)?));
    assert_eq!(owner.cursor().last_active_end(), None);
    assert_eq!(owner.cursor().last_monotonic_end(), None);
    check_debt(
        owner.remaining_phase_seconds(),
        owner.remaining_phase_error_seconds(),
        1.0 / 32.0,
    );
    Ok(())
}

/// Aggregate-cap refusal leaves no operation; an exact-cap neighbor is useful.
/// # Errors
/// Preserve real source, phase admission and successful boundary issuance.
/// # Panics
/// Reject cap bypass, request/debt mutation or consumption by a failed proposal.
#[test]
fn aggregate_cap_refusal_has_an_exact_cap_neighbor_without_hidden_issuance() -> TestResult {
    for (base, refused) in [(1.0 / 64.0, true), (3.0 / 256.0, false)] {
        let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
        let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
        let chart = initial_chart()?;
        let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
        let strata = strata()?;
        let batch = product_ok(FixtureChartBatch::new(
            &chart,
            MonotonicNs::new(4 * SECOND),
            &entries,
        ))?;
        let mut producer = seed(&entries, &strata, 4)?;
        let mut owner = start(&mut producer, &chart, &batch, &strata, base)?;
        let before = cursor_snapshot(owner.cursor());
        if refused {
            assert!(matches!(
                owner.issue(now(0, 4 * SECOND)?, FixturePhaseResidualPermission::Allow),
                Err(FixturePhaseError::Numeric {
                    source: FixturePhaseNumericError::AggregateLimit
                })
            ));
            assert!(owner.proposal().is_none());
        } else {
            let proposal = product_ok(
                owner.issue(now(0, 4 * SECOND)?, FixturePhaseResidualPermission::Allow),
            )?;
            assert_eq!(proposal.ordinal(), label(1)?);
            assert_eq!(proposal.transient_rate(), 1.0 / 256.0);
            assert_eq!(proposal.aggregate_rate(), 1.0 / 64.0);
            assert_eq!(proposal.active_deadline(), ActiveAwakeNs::new(8 * SECOND));
            assert_eq!(proposal.monotonic_deadline(), MonotonicNs::new(20 * SECOND));
        }
        assert_eq!(cursor_snapshot(owner.cursor()), before);
        check_debt(
            owner.remaining_phase_seconds(),
            owner.remaining_phase_error_seconds(),
            1.0 / 32.0,
        );
    }
    Ok(())
}

/// Original provenance is a bitwise copy of the immutable emitted error.
/// # Panics
/// Reject a constant, accumulated or absent original-error projection.
fn check_original_error(owner: &FixturePhasePlanOwner<'_, '_>, expected: u64) {
    assert_eq!(
        owner
            .provenance()
            .map(|value| value.phase_error_seconds().to_bits()),
        Some(expected)
    );
}

/// Construct an explicit finite test profile without changing the shared defaults.
/// # Errors
/// Preserve literal-time conversion and genuine checked profile admission.
/// # Panics
/// Reject failure of these independently supplied valid fixture parameters.
fn profile_with_horizons(
    active_seconds: u64,
    monotonic_seconds: u64,
    phase_cap: f64,
) -> TestResult<FixturePhaseProfile> {
    let active = active_seconds
        .checked_mul(SECOND)
        .ok_or_else(|| io::Error::other("fixture active horizon overflow"))?;
    let monotonic = monotonic_seconds
        .checked_mul(SECOND)
        .ok_or_else(|| io::Error::other("fixture monotonic horizon overflow"))?;
    product_ok(FixturePhaseProfile::new(
        ActiveAwakeNs::new(active).duration_since(ActiveAwakeNs::new(0))?,
        MonotonicNs::new(monotonic).duration_since(MonotonicNs::new(0))?,
        phase_cap,
        1.0 / 128.0,
        1.0 / 64.0,
    ))
}

/// Lease a genuine producer without presuming its next reference is admissible.
/// # Errors
/// Preserve actual checked constructor, cursor and fixture-time failures.
/// # Panics
/// Reject failure of the explicitly valid construction inputs.
fn unadmitted_owner<'producer, 'chart>(
    producer: &'producer mut FixtureCombinationState,
    chart: &'chart FixtureAffineChart,
    profile: FixturePhaseProfile,
    base: f64,
) -> TestResult<FixturePhasePlanOwner<'producer, 'chart>> {
    product_ok(FixturePhasePlanOwner::new(
        producer,
        chart,
        FixtureRateSettlementCursor::new(chart, clocks()?, label(1)?),
        now(0, 4 * SECOND)?,
        profile,
        baseline(base)?,
    ))
}

/// Either target coordinate alone can invalidate a real pending cursor mapping.
/// # Errors
/// Preserve real charts, settlement, rejected custody and useful source admission.
/// # Panics
/// Reject independent frame/generation bypasses or loss of the producer's credit.
#[test]
fn independent_target_tag_mismatches_preserve_cursor_and_source_credit() -> TestResult {
    let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
    let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let mut producer = seed(&entries, &strata, 4)?;
    let (mut cursor, replacement) = settled_cursor_and_chart(&chart)?;
    let before = cursor_snapshot(&cursor);
    for target in [(19, 7), (31, 29)] {
        let foreign = product_ok(FixtureAffineChart::new(
            FixtureChartFrames::new(label(17)?, label(23)?, label(target.0)?, label(target.1)?)?,
            chart.domain(),
            0.0,
            0.0,
        ))?;
        let attempted = FixturePhasePlanOwner::new(
            &mut producer,
            &foreign,
            cursor,
            now(2 * SECOND, 6 * SECOND)?,
            profile()?,
            baseline(1.0 / 1024.0)?,
        );
        cursor = rejected_construction(attempted, &FixturePhaseError::CursorMappingMismatch)?;
        assert_eq!(cursor_snapshot(&cursor), before);
    }
    let entries = [a.entry(&replacement, 6)?, b.entry(&replacement, 6)?];
    let batch = product_ok(FixtureChartBatch::new(
        &replacement,
        MonotonicNs::new(6 * SECOND),
        &entries,
    ))?;
    let mut owner = product_ok(FixturePhasePlanOwner::new(
        &mut producer,
        &replacement,
        cursor,
        now(2 * SECOND, 6 * SECOND)?,
        profile()?,
        baseline(1.0 / 1024.0)?,
    ))?;
    assert!(matches!(product_ok(owner.receive_reference(
        &batch, &strata, now(2 * SECOND, 6 * SECOND)?,
    ))?, FixturePhaseReferenceAdmission::Accepted { emitted_round } if emitted_round == label(1)?));
    assert_eq!(cursor_snapshot(owner.cursor()), before);
    check_debt(
        owner.remaining_phase_seconds(),
        owner.remaining_phase_error_seconds(),
        11.0 / 512.0,
    );
    Ok(())
}

/// Admission observes the two adjacent caps and exact equality independently.
/// # Errors
/// Preserve real fixture construction and current owner/reference operations.
/// # Panics
/// Reject wrong cap direction, rolled-back credit or a refused exact-cap plan.
fn check_phase_cap_admission(cap: f64, accepted: bool) -> TestResult {
    let a = Source::new(11, 0, 1 << 28)?;
    let b = Source::new(22, 0, 1 << 28)?;
    let before = [source_snapshot(&a)?, source_snapshot(&b)?];
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let mut owner = unadmitted_owner(
        &mut producer,
        &chart,
        profile_with_horizons(8, 16, cap)?,
        1.0 / 1024.0,
    )?;
    let cursor = cursor_snapshot(owner.cursor());
    let admission = product_ok(owner.receive_reference(&batch, &strata, now(0, 4 * SECOND)?))?;
    assert_eq!(
        owner
            .provenance()
            .map(FixturePhaseProvenance::phase_seconds),
        Some(1.0 / 16.0)
    );
    assert_eq!(
        owner
            .provenance()
            .map(FixturePhaseProvenance::emitted_round),
        Some(label(1)?)
    );
    assert_eq!(owner.last_known_phase_seconds(), Some(1.0 / 16.0));
    if accepted {
        assert!(
            matches!(admission, FixturePhaseReferenceAdmission::Accepted { emitted_round } if emitted_round == label(1)?)
        );
        check_debt(
            owner.remaining_phase_seconds(),
            owner.remaining_phase_error_seconds(),
            1.0 / 16.0,
        );
        let proposal =
            product_ok(owner.issue(now(0, 4 * SECOND)?, FixturePhaseResidualPermission::Deny))?;
        check_proposal(&proposal, 1, 1.0 / 128.0, 9.0 / 1024.0)?;
    } else {
        assert!(
            matches!(admission, FixturePhaseReferenceAdmission::AdmittedNoPlan {
            emitted_round: Some(round), reason: FixturePhaseError::PhaseEnvelope,
        } if round == label(1)?)
        );
        check_refused_cap_owner(&mut owner, &batch, &strata)?;
    }
    assert_eq!(cursor_snapshot(owner.cursor()), cursor);
    assert_eq!([source_snapshot(&a)?, source_snapshot(&b)?], before);
    Ok(())
}

/// A committed oversized phase leaves history but no useful plan or new credit.
/// # Errors
/// Preserve genuine receiving and literal-time setup failures.
/// # Panics
/// Reject hidden issuance, residual deadlines or a second reference emission.
fn check_refused_cap_owner<'chart>(
    owner: &mut FixturePhasePlanOwner<'_, 'chart>,
    batch: &FixtureChartBatch<'_, 'chart>,
    strata: &[FixturePrincipalStratum; 2],
) -> TestResult {
    assert_eq!(owner.remaining_phase_seconds(), None);
    assert_eq!(owner.remaining_phase_error_seconds(), None);
    assert_eq!(owner.active_deadline(), None);
    assert_eq!(owner.monotonic_deadline(), None);
    assert!(owner.proposal().is_none());
    assert!(matches!(
        owner.issue(now(0, 4 * SECOND)?, FixturePhaseResidualPermission::Deny),
        Err(FixturePhaseError::NoReference)
    ));
    assert!(matches!(
        product_ok(owner.receive_reference(batch, strata, now(0, 4 * SECOND)?))?,
        FixturePhaseReferenceAdmission::AdmittedNoPlan {
            emitted_round: None,
            reason: FixturePhaseError::Reference {
                source: FixtureReferenceInputError::NotReady
            },
        }
    ));
    Ok(())
}

/// Policy-cap neighbors leave the genuine emitted mean fixed at exactly 1/16.
/// # Errors
/// Preserve each real source/producer/owner trajectory's checked failures.
/// # Panics
/// Reject an equality refusal or incorrect outside/inside boundary behavior.
#[test]
fn genuine_phase_envelope_has_adjacent_declared_cap_neighbors() -> TestResult {
    for (bits, accepted) in [
        (0x3faf_ffff_ffff_ffff, false),
        (0x3fb0_0000_0000_0000, true),
        (0x3fb0_0000_0000_0001, true),
    ] {
        check_phase_cap_admission(f64::from_bits(bits), accepted)?;
    }
    Ok(())
}

/// Known modeled interval and independent expected point/refusal, not policy.
struct SettlementCase {
    /// Literal Q32 input centre of each genuine symmetric source history.
    phase_q32: i128,
    /// Independently chosen exact initial mean in seconds.
    phase: f64,
    /// Explicit known effective baseline for this whole interval.
    base: f64,
    /// Actual aggregate applied rate, independent of the issued proposal.
    actual: f64,
    /// Actual active interval, never inferred from receiving current time.
    elapsed: u64,
    /// Optional later receiving readings; absence uses the actual endpoints.
    received: Option<(u64, u64)>,
    /// Independently calculated separately rounded accounted point.
    expected: f64,
    /// Exact immediate residual refusal, also required from resume.
    refusal: Option<FixturePhaseError>,
}

/// Read the actual interval ends without substituting the optional receiving time.
/// # Errors
/// Reject overflow in a literal fixture endpoint instead of wrapping it.
fn case_end(case: &SettlementCase) -> TestResult<(u64, u64)> {
    let monotonic = (4 * SECOND)
        .checked_add(case.elapsed)
        .ok_or_else(|| io::Error::other("fixture known endpoint overflow"))?;
    Ok((case.elapsed, monotonic))
}

/// Use only checked public producer/owner paths for one fixed known trajectory.
/// # Errors
/// Preserve real fixture, reference, issuance and known-settlement failures.
/// # Panics
/// Reject fabricated credit, wrong initial phase or absent real issuance.
fn run_settlement_case(case: SettlementCase, horizons: (u64, u64)) -> TestResult {
    let a = Source::new(11, 0, case.phase_q32)?;
    let b = Source::new(22, 0, case.phase_q32)?;
    let chart = initial_chart()?;
    let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
    let strata = strata()?;
    let batch = product_ok(FixtureChartBatch::new(
        &chart,
        MonotonicNs::new(4 * SECOND),
        &entries,
    ))?;
    let mut producer = seed(&entries, &strata, 4)?;
    let mut owner = unadmitted_owner(
        &mut producer,
        &chart,
        profile_with_horizons(horizons.0, horizons.1, 1.0 / 16.0)?,
        case.base,
    )?;
    assert!(
        matches!(product_ok(owner.receive_reference(&batch, &strata, now(0, 4 * SECOND)?))?,
        FixturePhaseReferenceAdmission::Accepted { emitted_round } if emitted_round == label(1)?)
    );
    check_debt(
        owner.remaining_phase_seconds(),
        owner.remaining_phase_error_seconds(),
        case.phase,
    );
    {
        let proposal =
            product_ok(owner.issue(now(0, 4 * SECOND)?, FixturePhaseResidualPermission::Allow))?;
        assert_eq!(proposal.ordinal(), label(1)?);
        if case.phase_q32 == 0 {
            assert!(!proposal.has_useful_transient());
            assert_eq!(proposal.transient_rate().to_bits(), 0);
            assert_eq!(proposal.aggregate_rate().to_bits(), case.base.to_bits());
        } else {
            assert!(proposal.has_useful_transient());
        }
    }
    let ends = case_end(&case)?;
    let current = case.received.unwrap_or(ends);
    let event = FixtureRateSettlement::Completed(interval(
        owner.chart(),
        clocks()?,
        1,
        (0, ends.0),
        (4 * SECOND, ends.1),
        case.actual,
        (31, 7),
    )?);
    let retired = known(
        owner,
        event,
        Some(baseline(case.base)?),
        now(current.0, current.1)?,
    )?;
    check_settlement_case(retired, case, horizons)
}

/// Inspect immediate classification before resume can independently latch expiry.
/// # Errors
/// Preserve actual transfer and fixed-label/time setup failures.
/// # Panics
/// Reject lost delivery/debt, wrong typed refusal or restarted finite deadlines.
fn check_settlement_case(
    mut retired: FixturePhaseRetirement<'_>,
    case: SettlementCase,
    horizons: (u64, u64),
) -> TestResult {
    let ends = case_end(&case)?;
    let current = case.received.unwrap_or(ends);
    check_debt(
        retired.remaining_phase_seconds(),
        retired.remaining_phase_error_seconds(),
        case.expected,
    );
    assert_eq!(retired.debt_error(), case.refusal.as_ref());
    assert_eq!(
        retired.cursor().map(cursor_snapshot),
        Some((
            FixtureRateCursorStatus::Pending,
            Some(label(2)?),
            Some(ActiveAwakeNs::new(ends.0)),
            Some(MonotonicNs::new(ends.1)),
        ))
    );
    if let Some(expected) = case.refusal {
        assert_eq!(
            retired
                .resume(
                    now(current.0, current.1)?,
                    FixturePhaseResidualPermission::Allow
                )
                .err(),
            Some(expected)
        );
        check_debt(
            retired.remaining_phase_seconds(),
            retired.remaining_phase_error_seconds(),
            case.expected,
        );
        assert!(retired.cursor().is_some());
    } else {
        let owner = product_ok(retired.resume(
            now(current.0, current.1)?,
            FixturePhaseResidualPermission::Allow,
        ))?;
        check_debt(
            owner.remaining_phase_seconds(),
            owner.remaining_phase_error_seconds(),
            case.expected,
        );
        assert!(owner.proposal().is_none());
        assert_eq!(owner.cursor().next_ordinal(), Some(label(2)?));
        assert_eq!(
            owner.active_deadline(),
            Some(ActiveAwakeNs::new(horizons.0 * SECOND))
        );
        assert_eq!(
            owner.monotonic_deadline(),
            Some(MonotonicNs::new((4 + horizons.1) * SECOND))
        );
    }
    Ok(())
}

/// Assemble a literal case; receiving time normally equals the actual endpoints.
fn settlement_case(
    phase: (i128, f64),
    rates: (f64, f64),
    elapsed: u64,
    expected: f64,
    refusal: Option<FixturePhaseError>,
) -> SettlementCase {
    SettlementCase {
        phase_q32: phase.0,
        phase: phase.1,
        base: rates.0,
        actual: rates.1,
        elapsed,
        received: None,
        expected,
        refusal,
    }
}

/// Actual aggregate-cap equality and zero transient are valid for either debt sign.
/// # Errors
/// Preserve real fixture and exact typed known-delivery/transfer failures.
/// # Panics
/// Reject equality as an excess or baseline-only actual delivery as opposite.
#[test]
fn actual_rate_cap_and_zero_transient_keep_both_debt_signs() -> TestResult {
    for case in [
        settlement_case(
            (ORIGINAL_PHASE_Q32, 1.0 / 32.0),
            (1.0 / 1024.0, 1.0 / 64.0),
            15_625_000,
            2033.0 / 65_536.0,
            None,
        ),
        settlement_case(
            (-ORIGINAL_PHASE_Q32, -1.0 / 32.0),
            (-1.0 / 1024.0, -1.0 / 64.0),
            15_625_000,
            -2033.0 / 65_536.0,
            None,
        ),
        settlement_case(
            (ORIGINAL_PHASE_Q32, 1.0 / 32.0),
            (0.0, 0.0),
            SECOND,
            1.0 / 32.0,
            None,
        ),
        settlement_case(
            (-ORIGINAL_PHASE_Q32, -1.0 / 32.0),
            (0.0, 0.0),
            SECOND,
            -1.0 / 32.0,
            None,
        ),
    ] {
        run_settlement_case(case, (8, 16))?;
    }
    Ok(())
}

/// Negative-debt direction and overshoot have a useful exact-zero neighbor.
/// # Errors
/// Preserve real source, operation, known interval and residual-transfer failures.
/// # Panics
/// Reject wrong-direction/overshoot admission or refusal of a true zero residual.
#[test]
fn negative_debt_direction_and_zero_residual_have_typed_neighbors() -> TestResult {
    for case in [
        settlement_case(
            (-ORIGINAL_PHASE_Q32, -1.0 / 32.0),
            (0.0, 1.0 / 1024.0),
            2 * SECOND,
            -17.0 / 512.0,
            Some(FixturePhaseError::DeliveredOppositeDirection),
        ),
        settlement_case(
            (-ORIGINAL_PHASE_Q32, -1.0 / 32.0),
            (0.0, -1.0 / 128.0),
            6 * SECOND,
            1.0 / 64.0,
            Some(FixturePhaseError::DeliveredOvershoot),
        ),
        settlement_case(
            (-ORIGINAL_PHASE_Q32, -1.0 / 32.0),
            (0.0, -1.0 / 128.0),
            4 * SECOND,
            0.0,
            None,
        ),
    ] {
        run_settlement_case(case, (8, 16))?;
    }
    Ok(())
}

/// A genuine zero mean can issue baseline-only work and receive either actual sign.
/// # Errors
/// Preserve real zero-reference, checked issuance and known-delivery failures.
/// # Panics
/// Reject a fabricated direction or overshoot from nominal zero debt.
#[test]
fn zero_phase_baseline_only_issuance_accepts_both_actual_rate_signs() -> TestResult {
    for (actual, expected) in [(1.0 / 1024.0, -1.0 / 512.0), (-1.0 / 1024.0, 1.0 / 512.0)] {
        run_settlement_case(
            settlement_case((0, 0.0), (0.0, actual), 2 * SECOND, expected, None),
            (8, 16),
        )?;
    }
    Ok(())
}

/// A declared point cap admits equality and refuses the independently known RN excess.
/// # Errors
/// Preserve real capped-reference, interval and exact residual-error failures.
/// # Panics
/// Reject a cap-equality refusal, reassociated point or wrong phase-envelope cause.
#[test]
fn postsettlement_phase_cap_distinguishes_equal_and_rounded_excess() -> TestResult {
    run_settlement_case(
        settlement_case(
            (1 << 28, 1.0 / 16.0),
            (0.0, 0.0),
            8 * SECOND,
            1.0 / 16.0,
            None,
        ),
        (16, 64),
    )?;
    // r=b=-2^-6+2^-59; products=-2^-3+2^-56. RN(P0-d)=3/16
    // by ties-to-even, then +a yields exactly P0+2^-56, not net-rate P0.
    let actual = f64::from_bits(0xbf8f_ffff_ffff_ffff);
    run_settlement_case(
        settlement_case(
            (1 << 28, 1.0 / 16.0),
            (actual, actual),
            8 * SECOND,
            f64::from_bits(0x3fb0_0000_0000_0001),
            Some(FixturePhaseError::PhaseEnvelope),
        ),
        (16, 64),
    )?;
    Ok(())
}

/// Either current clock can expire a known two-second interval's continuation.
/// # Errors
/// Preserve real setup, known settlement and exact typed expiry refusals.
/// # Panics
/// Reject missing immediate expiry or a generic replacement for its exact cause.
#[test]
fn late_known_delivery_reports_each_expiry_before_transfer() -> TestResult {
    for current in [(8 * SECOND, 6 * SECOND), (2 * SECOND, 20 * SECOND)] {
        let mut case = settlement_case(
            (ORIGINAL_PHASE_Q32, 1.0 / 32.0),
            (1.0 / 1024.0, 5.0 / 1024.0),
            2 * SECOND,
            3.0 / 128.0,
            Some(FixturePhaseError::Expired),
        );
        case.received = Some(current);
        run_settlement_case(case, (8, 16))?;
    }
    Ok(())
}

/// Both signs of the new baseline use an inclusive cap and one retained transfer slot.
/// # Errors
/// Preserve genuine setup, checked baselines, known delivery and context transfer.
/// # Panics
/// Reject adjacent outside values, equality refusal or changed chart/cursor custody.
#[test]
fn fresh_transfer_baseline_cap_has_signed_adjacent_neighbors() -> TestResult {
    for negative in [false, true] {
        let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
        let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
        let chart = initial_chart()?;
        let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
        let strata = strata()?;
        let batch = product_ok(FixtureChartBatch::new(
            &chart,
            MonotonicNs::new(4 * SECOND),
            &entries,
        ))?;
        let mut producer = seed(&entries, &strata, 4)?;
        let mut owner = start(&mut producer, &chart, &batch, &strata, 1.0 / 1024.0)?;
        check_issue(&mut owner, (0, 4 * SECOND), 1, (1.0 / 256.0, 5.0 / 1024.0))?;
        let retired = settle_interval(
            owner,
            1,
            ((0, 2 * SECOND), (4 * SECOND, 6 * SECOND)),
            5.0 / 1024.0,
            (31, 7),
            false,
            1.0 / 1024.0,
        )?;
        check_fresh_baseline_boundary(retired, negative)?;
    }
    Ok(())
}

/// Reject the closest outside baseline without taking the actual transfer custody.
/// # Errors
/// Preserve checked fixture baselines and the exact-cap healthy transfer.
/// # Panics
/// Reject consumed slot, generation substitution or refreshed source provenance.
fn check_fresh_baseline_boundary(
    mut retired: FixturePhaseRetirement<'_>,
    negative: bool,
) -> TestResult {
    let cap = 1.0_f64 / 64.0;
    let exact = if negative { -cap } else { cap };
    let outside = if negative {
        -cap.next_up()
    } else {
        cap.next_up()
    };
    let before = retired.cursor().map(cursor_snapshot);
    assert!(before.is_some());
    let historical_error = retired.remaining_phase_error_seconds().map(f64::to_bits);
    assert!(historical_error.is_some());
    assert_eq!(
        retired
            .resume_fresh(now(2 * SECOND, 6 * SECOND)?, baseline(outside)?)
            .err(),
        Some(FixturePhaseError::DeliveredOutOfEnvelope)
    );
    assert_eq!(retired.cursor().map(cursor_snapshot), before);
    assert_eq!(
        retired.remaining_phase_error_seconds().map(f64::to_bits),
        historical_error
    );
    let frames = retired.replacement()?.frames();
    {
        let context =
            product_ok(retired.resume_fresh(now(2 * SECOND, 6 * SECOND)?, baseline(exact)?))?;
        assert_eq!(context.chart().frames(), frames);
        assert_eq!(context.chart().frames().target_frame(), label(31)?);
        assert_eq!(context.chart().frames().target_generation(), label(7)?);
        assert_eq!(Some(cursor_snapshot(context.cursor())), before);
        assert_eq!(context.remaining_phase_seconds(), None);
        assert_eq!(context.remaining_phase_error_seconds(), None);
        assert_eq!(
            context.last_known_phase_error_seconds().map(f64::to_bits),
            historical_error
        );
        assert!(context.proposal().is_none());
        assert_eq!(
            context
                .provenance()
                .map(|value| value.frames().target_generation()),
            Some(label(29)?)
        );
    }
    assert!(retired.cursor().is_none());
    assert_eq!(
        retired
            .resume_fresh(now(2 * SECOND, 6 * SECOND)?, baseline(exact)?)
            .err(),
        Some(FixturePhaseError::TransferConsumed)
    );
    assert_eq!(
        retired
            .resume(
                now(2 * SECOND, 6 * SECOND)?,
                FixturePhaseResidualPermission::Allow
            )
            .err(),
        Some(FixturePhaseError::TransferConsumed)
    );
    Ok(())
}

/// A future monotonic terminal end rejects without changing any exposed custody.
/// # Errors
/// Preserve real rejected-owner recovery and fixed proposal inspection failures.
/// # Panics
/// Reject early endpoint admission, a lost proposal or changed historical state.
fn reject_future_monotonic_end<'producer, 'chart>(
    owner: FixturePhasePlanOwner<'producer, 'chart>,
    event: FixtureRateSettlement,
) -> TestResult<FixturePhasePlanOwner<'producer, 'chart>> {
    let snapshot = |value: &FixturePhasePlanOwner<'_, '_>| {
        (
            cursor_snapshot(value.cursor()),
            (
                value.remaining_phase_seconds().map(f64::to_bits),
                value.remaining_phase_error_seconds().map(f64::to_bits),
                value.last_known_phase_seconds().map(f64::to_bits),
                value.last_known_phase_error_seconds().map(f64::to_bits),
            ),
            (
                value.current_time().active(),
                value.current_time().monotonic(),
            ),
            (value.active_deadline(), value.monotonic_deadline()),
            value.provenance().map(|p| {
                (
                    p.phase_seconds().to_bits(),
                    p.phase_error_seconds().to_bits(),
                    p.emitted_round(),
                    p.source_context(),
                    p.frames(),
                    p.anchor_observed_at(),
                    p.latest_observed_at(),
                    p.fit_reference(),
                    p.projected_at(),
                )
            }),
            value.proposal().map(|p| {
                (
                    p.ordinal(),
                    p.transient_rate().to_bits(),
                    p.transient_error().to_bits(),
                    p.aggregate_rate().to_bits(),
                    p.aggregate_error().to_bits(),
                    p.active_deadline(),
                    p.monotonic_deadline(),
                )
            }),
        )
    };
    let before = snapshot(&owner);
    let owner = rejected(
        owner.settle(
            event,
            Some(baseline(1.0 / 1024.0)?),
            now(3 * SECOND, 6 * SECOND - 1)?,
        ),
        &FixturePhaseError::TimeRegression {
            clock: FixtureRateClock::Monotonic,
        },
    )?;
    assert_eq!(snapshot(&owner), before);
    let proposal = owner.proposal();
    assert!(proposal.is_some());
    let proposal =
        proposal.ok_or_else(|| io::Error::other("lost issued terminal after refusal"))?;
    check_proposal(&proposal, 1, 1.0 / 256.0, 5.0 / 1024.0)?;
    Ok(owner)
}

/// Inspect actual delayed delivery rather than integrating idle current time.
/// # Errors
/// Preserve exact-label checks and the actual replacement-chart result.
/// # Panics
/// Reject wrong consumed endpoints, known debt, current readings or event kind.
fn check_delayed_terminal(retired: &FixturePhaseRetirement<'_>, cancelled: bool) -> TestResult {
    check_debt(
        retired.remaining_phase_seconds(),
        retired.remaining_phase_error_seconds(),
        7.0 / 256.0,
    );
    assert_eq!(
        retired.cursor().map(cursor_snapshot),
        Some((
            FixtureRateCursorStatus::Pending,
            Some(label(2)?),
            Some(ActiveAwakeNs::new(2 * SECOND)),
            Some(MonotonicNs::new(6 * SECOND)),
        ))
    );
    let current = retired.current_time();
    assert_eq!(
        current.as_ref().map(FixturePhaseNow::active),
        Some(ActiveAwakeNs::new(3 * SECOND))
    );
    assert_eq!(
        current.as_ref().map(FixturePhaseNow::monotonic),
        Some(MonotonicNs::new(6 * SECOND))
    );
    check_original_retirement_deadlines(retired);
    assert_eq!(
        retired.replacement()?.phase_at_reference_seconds(),
        5.0 / 1024.0
    );
    assert_eq!(retired.debt_error(), None);
    assert_eq!(
        matches!(retired.event(), FixtureRateSettlement::Cancelled(_)),
        cancelled
    );
    Ok(())
}

/// Actual endpoints are independently contained by the supplied current clocks.
/// # Errors
/// Preserve genuine setup, typed interval and consuming receiver failures.
/// # Panics
/// Reject a future endpoint or refusal of a later-starting valid delivery retry.
#[test]
fn terminal_current_clocks_contain_delayed_known_delivery_independently() -> TestResult {
    for cancelled in [false, true] {
        let a = Source::new(11, 0, ORIGINAL_PHASE_Q32)?;
        let b = Source::new(22, 0, ORIGINAL_PHASE_Q32)?;
        let chart = initial_chart()?;
        let entries = [a.entry(&chart, 4)?, b.entry(&chart, 4)?];
        let strata = strata()?;
        let batch = product_ok(FixtureChartBatch::new(
            &chart,
            MonotonicNs::new(4 * SECOND),
            &entries,
        ))?;
        let mut producer = seed(&entries, &strata, 4)?;
        let mut owner = start(&mut producer, &chart, &batch, &strata, 1.0 / 1024.0)?;
        check_issue(&mut owner, (0, 4 * SECOND), 1, (1.0 / 256.0, 5.0 / 1024.0))?;
        let delivered = interval(
            owner.chart(),
            clocks()?,
            1,
            (SECOND, 2 * SECOND),
            (5 * SECOND, 6 * SECOND),
            5.0 / 1024.0,
            (31, 7),
        )?;
        let event = if cancelled {
            FixtureRateSettlement::Cancelled(delivered)
        } else {
            FixtureRateSettlement::Completed(delivered)
        };
        let owner = reject_future_monotonic_end(owner, event)?;
        let retired = known(
            owner,
            event,
            Some(baseline(1.0 / 1024.0)?),
            now(3 * SECOND, 6 * SECOND)?,
        )?;
        check_delayed_terminal(&retired, cancelled)?;
    }
    Ok(())
}

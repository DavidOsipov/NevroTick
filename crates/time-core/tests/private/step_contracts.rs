// SPDX-License-Identifier: GPL-2.0-only
//! Matched-step refusal and terminal custody over the parent's genuine source owner.
//! All limits, readings and completions are explicit inert fixture inputs.

use std::io;

use super::{
    ActiveAwakeNs, FixtureChartFrames, FixturePhaseError, FixturePhasePlanOwner,
    FixturePhaseProvenance, FixturePhaseResidualPermission, FixturePhaseSettlement,
    FixtureRateClockContext, FixtureRateCursorStatus, FixtureRateSettlement,
    FixtureRateSettlementCursor, FixtureStepCompletion, FixtureStepDirections, FixtureStepError,
    FixtureStepFeedback, FixtureStepLimits, FixtureStepSettlement, MonotonicNs, SECOND, TestResult,
    clocks, cursor_snapshot, interval, label, now, product_ok, step_limits, step_receipt,
    with_step_owner, with_step_owner_policy,
};

/// Preserve an unchanged actual owner only through its typed outer-refusal branch.
/// # Errors
/// Reject an unexpected consuming settlement without reconstructing an owner.
fn outer_owner<'producer, 'chart>(
    result: FixtureStepSettlement<'producer, 'chart>,
) -> TestResult<(FixturePhasePlanOwner<'producer, 'chart>, FixtureStepError)> {
    match result {
        FixtureStepSettlement::OuterRejected { owner, error } => Ok((owner, error)),
        _ => Err(io::Error::other("step structural refusal consumed the original owner").into()),
    }
}

/// Independently selected dyadic facts for one genuine matched completion.
struct KnownStepExpectation {
    /// Signed source debt supplied by the two original fitted histories.
    requested: f64,
    /// Explicit original baseline, also supplied as the actual aggregate rate.
    baseline: f64,
    /// Actual acknowledged constant displacement, independent of the request.
    achieved: f64,
    /// Expected two-second baseline displacement plus the actual constant step.
    chart: f64,
    /// Expected original source debt minus the actual constant step.
    debt: f64,
    /// Exact post-accounting policy refusal, absent for the healthy neighbor.
    refusal: Option<FixtureStepError>,
}

/// Follow actual issue, matched Known custody and take-once recovery.
/// # Errors
/// Preserve genuine request, receipt, replacement and fresh transfer failures.
/// # Panics
/// Detect changed policy classification, requested-as-applied accounting or refunded custody.
fn check_known_step(
    mut owner: FixturePhasePlanOwner<'_, '_>,
    limits: FixtureStepLimits,
    expected: &KnownStepExpectation,
) -> TestResult {
    assert_eq!(owner.remaining_phase_seconds(), Some(expected.requested));
    let proposal = product_ok(owner.issue_step(
        now(0, 4 * SECOND)?,
        FixturePhaseResidualPermission::Allow,
        limits,
    ))?;
    assert_eq!(proposal.ordinal(), label(1)?);
    assert_eq!(proposal.requested_phase_seconds(), expected.requested);
    assert_eq!(proposal.known_baseline().rate(), expected.baseline);
    let receipt = step_receipt(&owner, 1, expected.baseline, expected.achieved)?;
    let result = owner.settle_step(
        FixtureStepFeedback::Known(receipt),
        now(2 * SECOND, 8 * SECOND)?,
    );
    assert!(matches!(&result, FixtureStepSettlement::Known(_)));
    let FixtureStepSettlement::Known(mut retired) = result else {
        return Err(io::Error::other("a matched actual step must retain Known custody").into());
    };
    assert_eq!(retired.feedback_error(), None);
    assert_eq!(retired.continuation_error(), expected.refusal.as_ref());
    assert_eq!(retired.pre_step_phase_seconds(), Some(expected.requested));
    assert_eq!(retired.achieved_step_seconds(), expected.achieved);
    assert_eq!(retired.remaining_phase_seconds(), Some(expected.debt));
    assert_eq!(
        retired.replacement()?.phase_at_reference_seconds(),
        expected.chart
    );
    let cursor = retired
        .cursor()
        .ok_or_else(|| io::Error::other("lost committed cursor"))?;
    assert_eq!(cursor.next_ordinal(), Some(label(2)?));
    assert_eq!(
        cursor.last_active_end(),
        Some(ActiveAwakeNs::new(2 * SECOND))
    );
    assert_eq!(
        cursor.last_monotonic_end(),
        Some(MonotonicNs::new(8 * SECOND))
    );
    if expected.refusal.is_none() {
        let fresh = product_ok(retired.resume_fresh(now(2 * SECOND, 8 * SECOND)?))?;
        assert_eq!(fresh.remaining_phase_seconds(), None);
        assert_eq!(fresh.cursor().next_ordinal(), Some(label(2)?));
    } else {
        assert!(retired.resume_fresh(now(2 * SECOND, 8 * SECOND)?).is_err());
        assert_eq!(retired.remaining_phase_seconds(), Some(expected.debt));
        assert_eq!(
            retired
                .cursor()
                .and_then(FixtureRateSettlementCursor::next_ordinal),
            Some(label(2)?)
        );
    }
    assert_eq!(retired.achieved_step_seconds(), expected.achieved);
    Ok(())
}

/// The original point cap is inclusive at request, actual-step and residual boundaries.
/// # Errors
/// Preserve genuine cap-equality admission and matched completion failures.
/// # Panics
/// Detect rejection of an admitted endpoint or inventing an overshoot at exact zero.
#[test]
fn matched_step_original_phase_cap_is_inclusive_at_request_and_retirement() -> TestResult {
    for sign in [-1_i8, 1] {
        let direction = f64::from(sign);
        for (achieved, chart, debt) in [
            (0.0, direction / 16.0, direction / 8.0),
            (direction / 8.0, direction * 3.0 / 16.0, 0.0),
        ] {
            let expected = KnownStepExpectation {
                requested: direction / 8.0,
                baseline: direction / 32.0,
                achieved,
                chart,
                debt,
                refusal: None,
            };
            with_step_owner_policy(sign, 1.0 / 8.0, expected.baseline, |owner| {
                check_known_step(owner, step_limits(FixtureStepDirections::Both)?, &expected)
            })?;
        }
    }
    Ok(())
}

/// Per-operation magnitude and original phase magnitude remain independent authorities.
/// # Errors
/// Preserve checked distinct limits and actual matched completion failures.
/// # Panics
/// Detect one cap masking the other, or misclassifying known excess as only overshoot.
#[test]
fn matched_step_limit_and_original_cap_remain_independent() -> TestResult {
    for (limit, achieved, chart, debt, refusal) in [
        (
            1.0 / 8.0,
            3.0 / 16.0,
            1.0 / 4.0,
            -1.0 / 16.0,
            Some(FixtureStepError::Magnitude),
        ),
        (
            1.0 / 2.0,
            3.0 / 8.0,
            7.0 / 16.0,
            -1.0 / 4.0,
            Some(FixtureStepError::Magnitude),
        ),
        (1.0 / 8.0, 1.0 / 8.0, 3.0 / 16.0, 0.0, None),
    ] {
        let expected = KnownStepExpectation {
            requested: 1.0 / 8.0,
            baseline: 1.0 / 32.0,
            achieved,
            chart,
            debt,
            refusal,
        };
        let limits = product_ok(FixtureStepLimits::new(limit, FixtureStepDirections::Both))?;
        with_step_owner(1, |owner| check_known_step(owner, limits, &expected))?;
    }
    Ok(())
}

/// A baseline at either aggregate endpoint remains admitted beside an interior neighbor.
/// # Errors
/// Preserve genuine baseline construction, reference and actual delivery failures.
/// # Panics
/// Detect an inclusive baseline cap becoming a known-effect refusal.
#[test]
fn matched_step_aggregate_baseline_cap_is_inclusive_for_both_signs() -> TestResult {
    for sign in [-1_i8, 1] {
        let direction = f64::from(sign);
        for (baseline, chart) in [(1.0 / 4.0, 9.0 / 16.0), (1.0 / 2.0, 17.0 / 16.0)] {
            let expected = KnownStepExpectation {
                requested: direction / 8.0,
                baseline: direction * baseline,
                achieved: direction / 16.0,
                chart: direction * chart,
                debt: direction / 16.0,
                refusal: None,
            };
            with_step_owner_policy(sign, 1.0 / 4.0, expected.baseline, |owner| {
                check_known_step(owner, step_limits(FixtureStepDirections::Both)?, &expected)
            })?;
        }
    }
    Ok(())
}

/// Bidirectional permission does not permit an actual step opposite the established debt.
/// # Errors
/// Preserve genuine signed sources, explicit limits and actual completion failures.
/// # Panics
/// Detect direction authority being lost when the enum itself permits both signs.
#[test]
fn matched_step_bidirectional_permission_still_refuses_opposite_actual_sign() -> TestResult {
    for sign in [-1_i8, 1] {
        let direction = f64::from(sign);
        let expected = KnownStepExpectation {
            requested: direction / 8.0,
            baseline: direction / 32.0,
            achieved: -direction / 32.0,
            chart: direction / 32.0,
            debt: direction * 5.0 / 32.0,
            refusal: Some(FixtureStepError::Direction),
        };
        with_step_owner(sign, |owner| {
            check_known_step(owner, step_limits(FixtureStepDirections::Both)?, &expected)
        })?;
    }
    Ok(())
}

/// Zero, complete and overshooting actual steps have distinct signed policy outcomes.
/// # Errors
/// Preserve genuine source, acknowledgement, chart and sole-transfer failures.
/// # Panics
/// Detect zero treated as opposite, exact completion as overshoot or lost negative overshoot.
#[test]
fn matched_step_zero_full_and_overshoot_are_distinct_for_both_signs() -> TestResult {
    for sign in [-1_i8, 1] {
        let direction = f64::from(sign);
        for (achieved, chart, debt, refusal) in [
            (0.0, direction / 16.0, direction / 8.0, None),
            (direction / 8.0, direction * 3.0 / 16.0, 0.0, None),
            (
                direction * 3.0 / 16.0,
                direction / 4.0,
                -direction / 16.0,
                Some(FixtureStepError::Phase {
                    source: FixturePhaseError::DeliveredOvershoot,
                }),
            ),
        ] {
            let expected = KnownStepExpectation {
                requested: direction / 8.0,
                baseline: direction / 32.0,
                achieved,
                chart,
                debt,
                refusal,
            };
            with_step_owner(sign, |owner| {
                check_known_step(owner, step_limits(FixtureStepDirections::Both)?, &expected)
            })?;
        }
    }
    Ok(())
}

/// Either original clock deadline independently closes new step admission.
/// # Errors
/// Preserve genuine time/limit construction and the interior healthy neighbor.
/// # Panics
/// Detect single-clock expiry issuing an ordinal or renewing the accepted obligation.
#[test]
fn matched_step_each_original_clock_deadline_refuses_new_issuance() -> TestResult {
    for readings in [(8 * SECOND, 12 * SECOND), (2 * SECOND, 20 * SECOND)] {
        with_step_owner(1, |mut owner| {
            let before = cursor_snapshot(owner.cursor());
            let debt = owner.remaining_phase_seconds();
            let result = owner.issue_step(
                now(readings.0, readings.1)?,
                FixturePhaseResidualPermission::Allow,
                step_limits(FixtureStepDirections::Both)?,
            );
            assert!(matches!(
                result,
                Err(FixtureStepError::Phase {
                    source: FixturePhaseError::Expired
                })
            ));
            assert_eq!(cursor_snapshot(owner.cursor()), before);
            assert_eq!(owner.remaining_phase_seconds(), debt);
            assert_eq!(
                owner.active_deadline(),
                Some(ActiveAwakeNs::new(8 * SECOND))
            );
            assert_eq!(
                owner.monotonic_deadline(),
                Some(MonotonicNs::new(20 * SECOND))
            );
            assert!(owner.step_proposal().is_none());
            Ok(())
        })?;
    }
    with_step_owner(1, |mut owner| {
        let proposal = product_ok(owner.issue_step(
            now(7 * SECOND, 19 * SECOND)?,
            FixturePhaseResidualPermission::Allow,
            step_limits(FixtureStepDirections::Both)?,
        ))?;
        assert_eq!(proposal.ordinal(), label(1)?);
        assert_eq!(proposal.requested_phase_seconds(), 1.0 / 8.0);
        Ok(())
    })
}

/// Malformed scalar/chain construction cannot bypass the checked request or receipt.
/// # Errors
/// Preserve the genuine source, interval and healthy request setup failures.
/// # Panics
/// Detect an invalid raw value, direction or magnitude admitted before its valid neighbor.
#[test]
fn matched_step_raw_limits_and_receipts_reject_bypasses_before_valid_issue() -> TestResult {
    for cap in [
        0.0,
        -0.0,
        -1.0,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::from_bits(1),
    ] {
        assert!(matches!(
            FixtureStepLimits::new(cap, FixtureStepDirections::Both),
            Err(FixtureStepError::InvalidLimits)
        ));
    }
    with_step_owner(1, |mut owner| {
        let before = cursor_snapshot(owner.cursor());
        let original_debt = owner.remaining_phase_seconds();
        for (limits, expected) in [
            (
                product_ok(FixtureStepLimits::new(
                    1.0 / 16.0,
                    FixtureStepDirections::Both,
                ))?,
                FixtureStepError::Magnitude,
            ),
            (
                step_limits(FixtureStepDirections::Negative)?,
                FixtureStepError::Direction,
            ),
        ] {
            let result = owner.issue_step(
                now(0, 4 * SECOND)?,
                FixturePhaseResidualPermission::Allow,
                limits,
            );
            assert_eq!(result.err(), Some(expected));
            assert_eq!(cursor_snapshot(owner.cursor()), before);
            assert_eq!(owner.remaining_phase_seconds(), original_debt);
            assert!(owner.step_proposal().is_none());
        }
        for actual in [
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::from_bits(1),
        ] {
            let delivery = interval(
                owner.chart(),
                clocks()?,
                1,
                (0, 2 * SECOND),
                (4 * SECOND, 8 * SECOND),
                1.0 / 32.0,
                (31, 37),
            )?;
            assert!(matches!(
                FixtureStepCompletion::new(
                    delivery,
                    actual,
                    FixtureChartFrames::new(label(31)?, label(37)?, label(41)?, label(43)?)?
                ),
                Err(FixtureStepError::Phase {
                    source: FixturePhaseError::Chart { .. }
                })
            ));
        }
        let delivery = interval(
            owner.chart(),
            clocks()?,
            1,
            (0, 2 * SECOND),
            (4 * SECOND, 8 * SECOND),
            1.0 / 32.0,
            (31, 37),
        )?;
        assert!(matches!(
            FixtureStepCompletion::new(
                delivery,
                1.0 / 32.0,
                FixtureChartFrames::new(label(59)?, label(61)?, label(41)?, label(43)?)?
            ),
            Err(FixtureStepError::Receipt)
        ));
        let proposal = product_ok(owner.issue_step(
            now(0, 4 * SECOND)?,
            FixturePhaseResidualPermission::Allow,
            product_ok(FixtureStepLimits::new(
                1.0 / 8.0,
                FixtureStepDirections::Positive,
            ))?,
        ))?;
        assert_eq!(proposal.ordinal(), label(1)?);
        assert_eq!(proposal.requested_phase_seconds(), 1.0 / 8.0);
        assert_eq!(owner.remaining_phase_seconds(), original_debt);
        Ok(())
    })
}

/// Foreign context and ordinal refusals leave the actual pending step intact.
/// # Errors
/// Preserve genuine request, foreign-but-structural receipt and valid terminal setup.
/// # Panics
/// Detect changed cursor, time, chart, debt or pending operation before the valid neighbor.
#[test]
fn matched_step_foreign_clock_and_wrong_ordinal_preserve_outstanding_owner() -> TestResult {
    with_step_owner(1, |mut owner| {
        product_ok(owner.issue_step(
            now(0, 4 * SECOND)?,
            FixturePhaseResidualPermission::Allow,
            step_limits(FixtureStepDirections::Both)?,
        ))?;
        let before = cursor_snapshot(owner.cursor());
        let old_time = owner.current_time();
        let old_debt = owner.remaining_phase_seconds();
        let old_chart = (
            owner.chart().frames(),
            owner.chart().phase_at_reference_seconds().to_bits(),
        );
        let foreign = FixtureRateClockContext::new(label(109)?, label(103)?, label(107)?);
        let delivery = interval(
            owner.chart(),
            foreign,
            1,
            (0, 2 * SECOND),
            (4 * SECOND, 8 * SECOND),
            1.0 / 32.0,
            (31, 37),
        )?;
        let foreign = product_ok(FixtureStepCompletion::new(
            delivery,
            1.0 / 32.0,
            FixtureChartFrames::new(label(31)?, label(37)?, label(41)?, label(43)?)?,
        ))?;
        let (returned, error) = outer_owner(owner.settle_step(
            FixtureStepFeedback::Known(foreign),
            now(2 * SECOND, 8 * SECOND)?,
        ))?;
        owner = returned;
        assert!(matches!(error, FixtureStepError::Receipt));
        assert_eq!(cursor_snapshot(owner.cursor()), before);
        assert_eq!(owner.current_time().active(), old_time.active());
        assert_eq!(owner.current_time().monotonic(), old_time.monotonic());
        assert_eq!(owner.remaining_phase_seconds(), old_debt);
        assert_eq!(
            (
                owner.chart().frames(),
                owner.chart().phase_at_reference_seconds().to_bits()
            ),
            old_chart
        );
        let wrong_ordinal = step_receipt(&owner, 2, 1.0 / 32.0, 1.0 / 32.0)?;
        let (returned, error) = outer_owner(owner.settle_step(
            FixtureStepFeedback::Known(wrong_ordinal),
            now(2 * SECOND, 8 * SECOND)?,
        ))?;
        owner = returned;
        assert!(matches!(error, FixtureStepError::Receipt));
        assert_eq!(cursor_snapshot(owner.cursor()), before);
        assert_eq!(owner.current_time().active(), old_time.active());
        assert_eq!(owner.current_time().monotonic(), old_time.monotonic());
        assert_eq!(owner.remaining_phase_seconds(), old_debt);
        assert_eq!(
            owner.step_proposal().map(|value| value.ordinal()),
            Some(label(1)?)
        );
        assert_eq!(
            (
                owner.chart().frames(),
                owner.chart().phase_at_reference_seconds().to_bits()
            ),
            old_chart
        );
        let actual = step_receipt(&owner, 1, 1.0 / 32.0, 1.0 / 32.0)?;
        let FixtureStepSettlement::Known(retired) = owner.settle_step(
            FixtureStepFeedback::Known(actual),
            now(2 * SECOND, 8 * SECOND)?,
        ) else {
            return Err(io::Error::other("valid matched step neighbor did not commit").into());
        };
        assert_eq!(retired.remaining_phase_seconds(), Some(3.0 / 32.0));
        assert_eq!(
            retired.replacement()?.phase_at_reference_seconds(),
            3.0 / 32.0
        );
        Ok(())
    })
}

/// Consume exactly the intervening phase operation after the prior step's no-effect fact.
/// # Errors
/// Preserve phase issuance, wrong-kind refusal and matching terminal setup failures.
/// # Panics
/// Detect mismatched pending-kind admission or a lost ordinal-two phase operation.
fn phase_no_effect_after_step<'producer, 'chart>(
    mut owner: FixturePhasePlanOwner<'producer, 'chart>,
) -> TestResult<FixturePhasePlanOwner<'producer, 'chart>> {
    let proposal =
        product_ok(owner.issue(now(0, 4 * SECOND)?, FixturePhaseResidualPermission::Allow))?;
    assert_eq!(proposal.ordinal(), label(2)?);
    assert!(matches!(
        owner.issue_step(
            now(0, 4 * SECOND)?,
            FixturePhaseResidualPermission::Allow,
            step_limits(FixtureStepDirections::Both)?
        ),
        Err(FixtureStepError::Phase {
            source: FixturePhaseError::OutstandingOperation
        })
    ));
    let (returned, error) = outer_owner(owner.settle_step(
        FixtureStepFeedback::ProvenNoEffect {
            context: clocks()?,
            ordinal: label(2)?,
        },
        now(0, 4 * SECOND)?,
    ))?;
    owner = returned;
    assert!(matches!(
        error,
        FixtureStepError::Phase {
            source: FixturePhaseError::NoOutstandingOperation
        }
    ));
    assert_eq!(
        owner.proposal().map(|value| value.ordinal()),
        Some(label(2)?)
    );
    let FixturePhaseSettlement::ProvenNoEffect(owner) = owner.settle(
        FixtureRateSettlement::ProvenNoEffect {
            context: clocks()?,
            ordinal: label(2)?,
        },
        None,
        now(0, 4 * SECOND)?,
    ) else {
        return Err(io::Error::other("matching phase NoEffect did not consume ordinal two").into());
    };
    Ok(owner)
}

/// `NoEffect` spends the shared ordinal while preserving the original finite allowance.
/// # Errors
/// Preserve actual source, request, no-effect and competing-phase setup failures.
/// # Panics
/// Detect refunded operations, changed coordinates, invented endpoints or lost residual permission.
#[test]
fn matched_step_no_effect_consumes_shared_ordinal_without_renewing_permission() -> TestResult {
    with_step_owner(1, |mut owner| {
        let deadlines = (owner.active_deadline(), owner.monotonic_deadline());
        let original_error = owner.remaining_phase_error_seconds().map(f64::to_bits);
        let original_chart = (
            owner.chart().frames(),
            owner.chart().phase_at_reference_seconds().to_bits(),
        );
        product_ok(owner.issue_step(
            now(0, 4 * SECOND)?,
            FixturePhaseResidualPermission::Allow,
            step_limits(FixtureStepDirections::Both)?,
        ))?;
        assert!(matches!(
            owner.issue(now(0, 4 * SECOND)?, FixturePhaseResidualPermission::Allow),
            Err(FixturePhaseError::OutstandingOperation)
        ));
        let result = owner.settle_step(
            FixtureStepFeedback::ProvenNoEffect {
                context: clocks()?,
                ordinal: label(1)?,
            },
            now(0, 4 * SECOND)?,
        );
        let FixtureStepSettlement::ProvenNoEffect(mut owner) = result else {
            return Err(io::Error::other("step NoEffect lost its unchanged owner").into());
        };
        assert_eq!(owner.cursor().next_ordinal(), Some(label(2)?));
        assert_eq!(owner.cursor().last_active_end(), None);
        assert_eq!(owner.cursor().last_monotonic_end(), None);
        assert_eq!(
            (
                owner.chart().frames(),
                owner.chart().phase_at_reference_seconds().to_bits()
            ),
            original_chart
        );
        assert_eq!(owner.remaining_phase_seconds(), Some(1.0 / 8.0));
        assert_eq!(
            owner.remaining_phase_error_seconds().map(f64::to_bits),
            original_error
        );
        assert_eq!(
            (owner.active_deadline(), owner.monotonic_deadline()),
            deadlines
        );
        assert!(matches!(
            owner.issue_step(
                now(0, 4 * SECOND)?,
                FixturePhaseResidualPermission::Deny,
                step_limits(FixtureStepDirections::Both)?
            ),
            Err(FixtureStepError::Phase {
                source: FixturePhaseError::ResidualPermission
            })
        ));
        assert_eq!(owner.cursor().next_ordinal(), Some(label(2)?));
        assert!(owner.step_proposal().is_none());
        owner = phase_no_effect_after_step(owner)?;
        let proposal = product_ok(owner.issue_step(
            now(0, 4 * SECOND)?,
            FixturePhaseResidualPermission::Allow,
            step_limits(FixtureStepDirections::Both)?,
        ))?;
        assert_eq!(proposal.ordinal(), label(3)?);
        assert_eq!(proposal.provenance().emitted_round(), label(1)?);
        assert_eq!(
            (proposal.active_deadline(), proposal.monotonic_deadline()),
            (
                ActiveAwakeNs::new(8 * SECOND),
                MonotonicNs::new(20 * SECOND)
            )
        );
        Ok(())
    })
}

/// Unknown retains only historical debt and permanently unresolved cursor custody.
/// # Errors
/// Preserve the actual request and typed current-Unknown observation failures.
/// # Panics
/// Detect a zero-delivery inference, fabricated endpoints or restored operation credit.
#[test]
fn matched_step_unknown_blocks_without_inventing_delivery() -> TestResult {
    with_step_owner(1, |mut owner| {
        let original_error = owner.remaining_phase_error_seconds().map(f64::to_bits);
        product_ok(owner.issue_step(
            now(0, 4 * SECOND)?,
            FixturePhaseResidualPermission::Allow,
            step_limits(FixtureStepDirections::Both)?,
        ))?;
        let FixtureStepSettlement::Unknown(blocked) = owner.settle_step(
            FixtureStepFeedback::Unknown {
                context: clocks()?,
                ordinal: label(1)?,
            },
            now(0, 5 * SECOND)?,
        ) else {
            return Err(io::Error::other("matching step Unknown did not block its owner").into());
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
        assert_eq!(blocked.last_known_phase_seconds(), Some(1.0 / 8.0));
        assert_eq!(
            blocked.last_known_phase_error_seconds().map(f64::to_bits),
            original_error
        );
        assert_eq!(
            blocked
                .provenance()
                .map(FixturePhaseProvenance::emitted_round),
            Some(label(1)?)
        );
        Ok(())
    })
}

/// Actual forbidden direction, magnitude and overshoot remain acknowledged facts.
/// # Errors
/// Preserve valid request, actual receipt, chart and retirement construction failures.
/// # Panics
/// Detect refused historical accounting or useful continuation after a policy violation.
#[test]
fn matched_step_known_actual_violations_remain_owned_historical_facts() -> TestResult {
    for (actual, chart, debt, kind) in [
        (-1.0 / 32.0, 1.0 / 32.0, 5.0 / 32.0, 0),
        (5.0 / 16.0, 3.0 / 8.0, -3.0 / 16.0, 1),
        (3.0 / 16.0, 1.0 / 4.0, -1.0 / 16.0, 2),
    ] {
        with_step_owner(1, |mut owner| {
            product_ok(owner.issue_step(
                now(0, 4 * SECOND)?,
                FixturePhaseResidualPermission::Allow,
                step_limits(FixtureStepDirections::Positive)?,
            ))?;
            let actual_receipt = step_receipt(&owner, 1, 1.0 / 32.0, actual)?;
            let FixtureStepSettlement::Known(mut retired) = owner.settle_step(
                FixtureStepFeedback::Known(actual_receipt),
                now(2 * SECOND, 8 * SECOND)?,
            ) else {
                return Err(
                    io::Error::other("actual out-of-policy step lost Known custody").into(),
                );
            };
            assert_eq!(retired.achieved_step_seconds(), actual);
            assert_eq!(retired.requested_phase_seconds(), 1.0 / 8.0);
            assert_eq!(retired.remaining_phase_seconds(), Some(debt));
            assert!(
                retired
                    .remaining_phase_error_seconds()
                    .is_some_and(|value| value.is_finite() && value >= 0.0)
            );
            assert_eq!(retired.replacement()?.phase_at_reference_seconds(), chart);
            assert_eq!(
                retired
                    .cursor()
                    .and_then(FixtureRateSettlementCursor::next_ordinal),
                Some(label(2)?)
            );
            let expected = match kind {
                0 => matches!(
                    retired.continuation_error(),
                    Some(FixtureStepError::Direction)
                ),
                1 => matches!(
                    retired.continuation_error(),
                    Some(FixtureStepError::Magnitude)
                ),
                _ => matches!(
                    retired.continuation_error(),
                    Some(FixtureStepError::Phase {
                        source: FixturePhaseError::DeliveredOvershoot
                    })
                ),
            };
            assert!(
                expected,
                "step policy classification after actual effect accounting"
            );
            assert!(retired.resume_fresh(now(2 * SECOND, 8 * SECOND)?).is_err());
            assert_eq!(retired.achieved_step_seconds(), actual);
            Ok(())
        })?;
    }
    Ok(())
}

/// Exact expiry on either clock cannot erase a matching earlier actual completion.
/// # Errors
/// Preserve genuine request, original receipt and explicit current-clock setup failures.
/// # Panics
/// Detect erased effects, refunded ordinal or useful continuation at deadline equality.
#[test]
fn matched_step_known_deadline_equality_preserves_effects_and_blocks_recovery() -> TestResult {
    for readings in [(8 * SECOND, 20 * SECOND - 1), (2 * SECOND, 20 * SECOND)] {
        with_step_owner(1, |mut owner| {
            product_ok(owner.issue_step(
                now(0, 4 * SECOND)?,
                FixturePhaseResidualPermission::Allow,
                step_limits(FixtureStepDirections::Both)?,
            ))?;
            let actual = step_receipt(&owner, 1, 1.0 / 32.0, 1.0 / 32.0)?;
            let result = owner.settle_step(
                FixtureStepFeedback::Known(actual),
                now(readings.0, readings.1)?,
            );
            assert!(matches!(&result, FixtureStepSettlement::Known(_)));
            let FixtureStepSettlement::Known(mut retired) = result else {
                return Err(io::Error::other("late matching step lost acknowledged facts").into());
            };
            assert_eq!(retired.remaining_phase_seconds(), Some(3.0 / 32.0));
            assert_eq!(
                retired.replacement()?.phase_at_reference_seconds(),
                3.0 / 32.0
            );
            assert_eq!(retired.achieved_step_seconds(), 1.0 / 32.0);
            assert_eq!(
                retired
                    .cursor()
                    .and_then(FixtureRateSettlementCursor::next_ordinal),
                Some(label(2)?)
            );
            assert_eq!(
                retired
                    .cursor()
                    .and_then(FixtureRateSettlementCursor::last_active_end),
                Some(ActiveAwakeNs::new(2 * SECOND))
            );
            assert_eq!(
                retired
                    .cursor()
                    .and_then(FixtureRateSettlementCursor::last_monotonic_end),
                Some(MonotonicNs::new(8 * SECOND))
            );
            assert!(matches!(
                retired.continuation_error(),
                Some(FixtureStepError::Phase {
                    source: FixturePhaseError::Expired
                })
            ));
            assert!(retired.resume_fresh(now(readings.0, readings.1)?).is_err());
            assert_eq!(retired.achieved_step_seconds(), 1.0 / 32.0);
            Ok(())
        })?;
    }
    Ok(())
}

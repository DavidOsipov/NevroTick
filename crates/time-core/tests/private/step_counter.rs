// SPDX-License-Identifier: GPL-2.0-only
//! Genuine matched-step custody preserves elapsed-counter evidence, not UTC mapping.
//!
//! The actual baseline NoEffect receiver establishes the counter witness. The
//! actual step receiver and fresh transfer must retain it while changing chart
//! basis. All clock and mapping values remain explicit mathematical fixtures.

use std::io;

use super::super::{
    FixtureBaselineError, FixtureBaselineFeedback, FixtureBaselineSettlement,
    FixturePhasePlanOwner, FixturePhaseResidualPermission, FixtureStepCompletion,
    FixtureStepDirections, FixtureStepFeedback, FixtureStepLimits, FixtureStepSettlement,
    controller_map_rule,
};
use super::{
    ProducerFixture, SECOND, TestResult, chart, clocks, controller_clock_point, label,
    prepared_private_baseline_receiver, private_controller_map, producer, product_ok,
};
use crate::{FixtureAffineChart, FixtureChartFrames, FixtureRateSpan, FixtureSettledRateInterval};

/// Establish the first exact counter ratio through the real baseline receiver.
/// # Errors
/// Preserve genuine fixture, baseline settlement and clock admission failures.
/// # Panics
/// Reject a different terminal kind or loss of the original fitted debt.
fn witnessed_owner<'producer, 'chart>(
    fixture: &'producer mut ProducerFixture<'chart>,
    chart: &'chart FixtureAffineChart,
) -> TestResult<FixturePhasePlanOwner<'producer, 'chart>> {
    let owner = prepared_private_baseline_receiver(fixture, chart)?;
    let returned = owner.settle_baseline(
        FixtureBaselineFeedback::ProvenNoEffect {
            context: clocks()?,
            ordinal: label(1)?,
        },
        controller_clock_point(1, 6)?,
    );
    assert!(matches!(
        returned,
        FixtureBaselineSettlement::ProvenNoEffect(_)
    ));
    let FixtureBaselineSettlement::ProvenNoEffect(owner) = returned else {
        return Err(io::Error::other("baseline NoEffect did not return its actual owner").into());
    };
    assert_eq!(owner.remaining_phase_seconds(), Some(1.0 / 32.0));
    check_counter_witness(&owner)?;
    Ok(owner)
}

/// Observe the exact original elapsed-clock relation without creating one.
/// # Errors
/// Report absent custody instead of constructing a replacement witness.
/// # Panics
/// Detect changed reference, forgotten evidence or a ratio inferred from labels.
fn check_counter_witness(owner: &FixturePhasePlanOwner<'_, '_>) -> TestResult {
    assert!(
        owner.clock_relation.is_some(),
        "lost actual counter relation"
    );
    let relation = owner
        .clock_relation
        .ok_or_else(|| io::Error::other("missing actual counter relation"))?;
    let witness = relation
        .witness
        .ok_or_else(|| io::Error::other("missing actual counter witness"))?;
    assert_eq!(relation.reference.active().nanoseconds(), 0);
    assert_eq!(relation.reference.monotonic().nanoseconds(), 4 * SECOND);
    assert_eq!(witness.active.nanoseconds(), SECOND);
    assert_eq!(witness.monotonic.nanoseconds(), 2 * SECOND);
    Ok(())
}

/// Supply an independent known interval and step for the genuinely pending ordinal.
/// # Errors
/// Preserve typed span, complete frame-chain and actual-step construction failures.
fn known_step(owner: &FixturePhasePlanOwner<'_, '_>) -> TestResult<FixtureStepCompletion> {
    let span = product_ok(FixtureRateSpan::new(
        (
            controller_clock_point(1, 6)?.active(),
            controller_clock_point(2, 8)?.active(),
        ),
        (
            controller_clock_point(1, 6)?.monotonic(),
            controller_clock_point(2, 8)?.monotonic(),
        ),
    ))?;
    let before = owner.chart().frames();
    let delivery = product_ok(FixtureSettledRateInterval::new(
        clocks()?,
        label(2)?,
        span,
        1.0 / 1024.0,
        FixtureChartFrames::new(
            before.target_frame(),
            before.target_generation(),
            label(31)?,
            label(7)?,
        )?,
        owner.chart().domain(),
    ))?;
    product_ok(FixtureStepCompletion::new(
        delivery,
        1.0 / 64.0,
        FixtureChartFrames::new(label(31)?, label(7)?, label(37)?, label(9)?)?,
    ))
}

/// Challenge the real future map consumer beside a compatible private control.
/// # Errors
/// Preserve checked map/control construction and actual retained-witness failures.
/// # Panics
/// Detect clearing counter evidence or admitting an old absolute chart basis.
fn check_new_map_guards(
    owner: &mut FixturePhasePlanOwner<'_, '_>,
    old_map: super::super::FixtureControllerClockMap,
) -> TestResult {
    assert!(matches!(
        controller_map_rule(owner.chart(), &old_map),
        Err(FixtureBaselineError::Map)
    ));
    let healthy_now = controller_clock_point(3, 10)?;
    let healthy = private_controller_map(owner.chart(), healthy_now)?;
    product_ok(controller_map_rule(owner.chart(), &healthy))?;
    let relation = owner
        .clock_relation
        .ok_or_else(|| io::Error::other("fresh transfer lost its counter relation"))?;
    product_ok(relation.admit_map(healthy, healthy_now))?;

    // Three active seconds over five monotonic seconds fits the declared
    // [3/8,5/8] enclosure, but contradicts the already observed exact 1/2.
    let conflicting_now = controller_clock_point(3, 9)?;
    let conflicting = private_controller_map(owner.chart(), conflicting_now)?;
    let before = owner.current_time();
    assert!(matches!(
        owner.issue_baseline(conflicting, conflicting_now),
        Err(FixtureBaselineError::Map)
    ));
    assert_eq!(owner.current_time().active(), before.active());
    assert_eq!(owner.current_time().monotonic(), before.monotonic());
    assert_eq!(owner.cursor().next_ordinal(), Some(label(3)?));
    assert_eq!(owner.producer.emitted_rounds(), 1);
    assert_eq!(owner.remaining_phase_seconds(), None);
    check_counter_witness(owner)
}

/// Known step recovery retains counter history while requiring a new absolute map.
/// # Errors
/// Preserve genuine source, matched operations, fresh transfer and mapping failures.
/// # Panics
/// Detect requested-as-applied feedback, lost counter evidence or revived old debt.
#[test]
fn known_step_fresh_recovery_retains_counter_witness_and_rejects_old_absolute_map() -> TestResult {
    let chart = chart()?;
    let mut fixture = producer(&chart)?;
    let mut owner = witnessed_owner(&mut fixture, &chart)?;
    let old_map = private_controller_map(&chart, controller_clock_point(0, 4)?)?;
    let limits = product_ok(FixtureStepLimits::new(
        1.0 / 16.0,
        FixtureStepDirections::Both,
    ))?;
    let issued = product_ok(owner.issue_step(
        controller_clock_point(1, 6)?,
        FixturePhaseResidualPermission::Allow,
        limits,
    ))?;
    assert_eq!(issued.ordinal(), label(2)?);
    assert_eq!(issued.requested_phase_seconds(), 1.0 / 32.0);
    let receipt = known_step(&owner)?;
    let settled = owner.settle_step(
        FixtureStepFeedback::Known(receipt),
        controller_clock_point(2, 8)?,
    );
    assert!(matches!(settled, FixtureStepSettlement::Known(_)));
    let FixtureStepSettlement::Known(mut retired) = settled else {
        return Err(io::Error::other("matched known step lost its retirement").into());
    };
    assert_eq!(
        retired.replacement()?.phase_at_reference_seconds(),
        17.0 / 1024.0
    );
    assert_eq!(retired.remaining_phase_seconds(), Some(1.0 / 64.0));
    let mut current = product_ok(retired.resume_fresh(controller_clock_point(2, 8)?))?;
    assert_eq!(current.remaining_phase_seconds(), None);
    check_counter_witness(&current)?;
    check_new_map_guards(&mut current, old_map)
}

/// Arithmetic failure occurs after the final mathematical target and endpoints commit.
/// # Errors
/// Preserve genuine source, counter witness, step request and scalar construction.
/// # Panics
/// Detect refunding the ordinal/target or erasing a separately known actual step.
#[test]
fn known_step_arithmetic_failure_commits_final_cursor_target_and_actual_facts() -> TestResult {
    let chart = product_ok(chart())?;
    let mut fixture = product_ok(producer(&chart))?;
    let mut owner = witnessed_owner(&mut fixture, &chart)?;
    product_ok(owner.issue_step(
        controller_clock_point(1, 6)?,
        FixturePhaseResidualPermission::Allow,
        product_ok(FixtureStepLimits::new(
            1.0 / 16.0,
            FixtureStepDirections::Both,
        ))?,
    ))?;
    let before = owner.chart().frames();
    let delivery = product_ok(FixtureSettledRateInterval::new(
        clocks()?,
        label(2)?,
        product_ok(FixtureRateSpan::new(
            (
                controller_clock_point(1, 6)?.active(),
                controller_clock_point(3, 10)?.active(),
            ),
            (
                controller_clock_point(1, 6)?.monotonic(),
                controller_clock_point(3, 10)?.monotonic(),
            ),
        ))?,
        f64::MAX,
        FixtureChartFrames::new(
            before.target_frame(),
            before.target_generation(),
            label(31)?,
            label(7)?,
        )?,
        owner.chart().domain(),
    ))?;
    let receipt = product_ok(FixtureStepCompletion::new(
        delivery,
        1.0 / 64.0,
        FixtureChartFrames::new(label(31)?, label(7)?, label(37)?, label(9)?)?,
    ))?;
    let result = owner.settle_step(
        FixtureStepFeedback::Known(receipt),
        controller_clock_point(3, 10)?,
    );
    assert!(matches!(&result, FixtureStepSettlement::Known(_)));
    let FixtureStepSettlement::Known(mut retired) = result else {
        return Err(io::Error::other("numeric failure cannot undo a Known step").into());
    };
    assert!(
        retired.cursor().is_some(),
        "lost committed final step cursor"
    );
    let cursor = retired
        .cursor()
        .ok_or_else(|| io::Error::other("lost committed cursor"))?;
    assert_eq!(cursor.next_ordinal(), Some(label(3)?));
    assert_eq!(cursor.target_frame, label(37)?);
    assert_eq!(cursor.target_generation, label(9)?);
    assert_eq!(
        cursor.last_active_end(),
        Some(controller_clock_point(3, 10)?.active())
    );
    assert_eq!(
        cursor.last_monotonic_end(),
        Some(controller_clock_point(3, 10)?.monotonic())
    );
    assert_eq!(retired.requested_phase_seconds(), 1.0 / 32.0);
    assert_eq!(retired.achieved_step_seconds(), 1.0 / 64.0);
    assert_eq!(retired.pre_step_phase_seconds(), None);
    assert_eq!(retired.remaining_phase_seconds(), None);
    assert!(retired.replacement().is_err());
    assert!(retired.debt_error().is_some());
    assert!(
        retired
            .resume_fresh(controller_clock_point(3, 10)?)
            .is_err()
    );
    Ok(())
}

/// Observe Step expiry before another operation can independently repair its latch.
/// # Errors
/// Preserve genuine source, baseline witness, checked issue and terminal identity failures.
/// # Panics
/// Reject a missing independent expiry, refunded ordinal or changed original debt/budget.
#[test]
fn step_no_effect_latches_each_original_deadline_before_later_admission() -> TestResult {
    for (active, monotonic, expired) in [(1, 6, false), (8, 6, true), (1, 20, true)] {
        let chart = product_ok(chart())?;
        let mut fixture = product_ok(producer(&chart))?;
        let mut owner = product_ok(witnessed_owner(&mut fixture, &chart))?;
        let error = owner.remaining_phase_error_seconds();
        let limits = product_ok(FixtureStepLimits::new(
            1.0 / 16.0,
            FixtureStepDirections::Both,
        ))?;
        let proposal = product_ok(owner.issue_step(
            controller_clock_point(1, 6)?,
            FixturePhaseResidualPermission::Allow,
            limits,
        ))?;
        assert_eq!(proposal.ordinal(), label(2)?);
        let current = controller_clock_point(active, monotonic)?;
        let result = owner.settle_step(
            FixtureStepFeedback::ProvenNoEffect {
                context: clocks()?,
                ordinal: label(2)?,
            },
            current,
        );
        assert!(matches!(&result, FixtureStepSettlement::ProvenNoEffect(_)));
        let FixtureStepSettlement::ProvenNoEffect(returned) = result else {
            return Err(io::Error::other("Step NoEffect lost its real owner").into());
        };
        assert!(
            returned.obligation.is_some(),
            "Step NoEffect lost original obligation"
        );
        let obligation = returned
            .obligation
            .as_ref()
            .ok_or_else(|| io::Error::other("lost original Step obligation"))?;
        // No issue, query or admission has run since the actual settlement.
        assert_eq!(obligation.expired, expired);
        assert!(obligation.residual);
        assert_eq!(obligation.debt.point(), 1.0 / 32.0);
        assert_eq!(Some(obligation.debt.error()), error);
        assert_eq!(
            obligation.active_deadline,
            controller_clock_point(8, 20)?.active()
        );
        assert_eq!(
            obligation.monotonic_deadline,
            controller_clock_point(8, 20)?.monotonic()
        );
        assert!(returned.issued.is_none());
        assert!(returned.step_proposal().is_none());
        assert!(std::ptr::eq(returned.chart(), &raw const chart));
        assert_eq!(returned.cursor().next_ordinal(), Some(label(3)?));
        assert_eq!(returned.cursor().last_active_end(), None);
        assert_eq!(returned.cursor().last_monotonic_end(), None);
        assert_eq!(returned.producer.emitted_rounds(), 1);
        assert_eq!(returned.current_time().active(), current.active());
        assert_eq!(returned.current_time().monotonic(), current.monotonic());
    }
    Ok(())
}

/// Preserve a checked negative-zero rate through both actual Step chart links.
/// # Errors
/// Preserve genuine fitting, operation, receipt and replacement-chart admission failures.
/// # Panics
/// Reject a positive-zero replacement or requested-as-applied phase/debt geometry.
#[test]
fn known_step_preserves_negative_zero_rate_in_actual_replacement_geometry() -> TestResult {
    let original = product_ok(chart())?;
    let chart = product_ok(FixtureAffineChart::new(
        original.frames(),
        original.domain(),
        0.0,
        -0.0,
    ))?;
    let mut fixture = product_ok(producer(&chart))?;
    let mut owner = product_ok(witnessed_owner(&mut fixture, &chart))?;
    product_ok(owner.issue_step(
        controller_clock_point(1, 6)?,
        FixturePhaseResidualPermission::Allow,
        product_ok(FixtureStepLimits::new(
            1.0 / 16.0,
            FixtureStepDirections::Both,
        ))?,
    ))?;
    let receipt = product_ok(known_step(&owner))?;
    let result = owner.settle_step(
        FixtureStepFeedback::Known(receipt),
        controller_clock_point(2, 8)?,
    );
    assert!(matches!(&result, FixtureStepSettlement::Known(_)));
    let FixtureStepSettlement::Known(retired) = result else {
        return Err(io::Error::other("signed-zero geometry lost Known retirement").into());
    };
    let replacement = product_ok(retired.replacement())?;
    assert_eq!(
        replacement.rate_seconds_per_second().to_bits(),
        (-0.0_f64).to_bits()
    );
    // One active second at 1/1024 plus actual S=1/64: total 17/1024.
    assert_eq!(replacement.phase_at_reference_seconds(), 17.0 / 1024.0);
    assert_eq!(retired.achieved_step_seconds(), 1.0 / 64.0);
    assert_eq!(retired.remaining_phase_seconds(), Some(1.0 / 64.0));
    assert!(retired.cursor().is_some());
    assert_eq!(
        retired.cursor().map(|cursor| cursor.target_frame),
        Some(label(37)?)
    );
    Ok(())
}

/// Retain the actual frequency model separately from the sole context-only transfer.
/// # Errors
/// Preserve genuine model, Step, historical projection and fresh-transfer failures.
/// # Panics
/// Reject erased/relabelled history, lost counter evidence or renewed old phase credit.
#[test]
fn known_step_frequency_history_remains_historical_after_fresh_owner_is_dropped() -> TestResult {
    let chart = product_ok(chart())?;
    let mut fixture = product_ok(producer(&chart))?;
    let mut owner = product_ok(witnessed_owner(&mut fixture, &chart))?;
    assert!(
        owner.frequency_history().is_some(),
        "genuine reference lost its fitted model"
    );
    let original = owner
        .frequency_history()
        .ok_or_else(|| io::Error::other("missing genuine frequency model"))?;
    assert_eq!(original.anchor_phase_seconds(), 1.0 / 32.0);
    assert_eq!(original.mean_seconds_per_second(), 0.0);
    product_ok(owner.issue_step(
        controller_clock_point(1, 6)?,
        FixturePhaseResidualPermission::Allow,
        product_ok(FixtureStepLimits::new(
            1.0 / 16.0,
            FixtureStepDirections::Both,
        ))?,
    ))?;
    let receipt = product_ok(known_step(&owner))?;
    let result = owner.settle_step(
        FixtureStepFeedback::Known(receipt),
        controller_clock_point(2, 8)?,
    );
    assert!(matches!(&result, FixtureStepSettlement::Known(_)));
    let FixtureStepSettlement::Known(mut retired) = result else {
        return Err(io::Error::other("frequency model lost Known Step retirement").into());
    };
    assert!(
        retired.frequency_history().is_some(),
        "Step erased actual model history"
    );
    let history = retired
        .frequency_history()
        .ok_or_else(|| io::Error::other("missing reexpressed frequency history"))?;
    // Original anchor 32/1024 minus actual baseline1/1024 and Step16/1024.
    assert_eq!(history.anchor_phase_seconds(), 15.0 / 1024.0);
    assert_eq!(history.mean_seconds_per_second(), 0.0);
    assert_eq!(history.emitted_round(), label(1)?);
    assert_eq!(history.source_context(), original.source_context());
    assert_eq!(history.reference_frames(), original.reference_frames());
    assert_eq!(
        history.frames(),
        product_ok(retired.replacement())?.frames()
    );
    assert_eq!(history.anchor_observed_at(), original.anchor_observed_at());
    assert_eq!(history.latest_observed_at(), original.latest_observed_at());
    assert_eq!(history.fit_reference(), original.fit_reference());
    assert_eq!(history.projected_at(), original.projected_at());
    assert_eq!(
        history.phase_standard_deviation_seconds(),
        original.phase_standard_deviation_seconds()
    );
    assert_eq!(
        history.rate_standard_deviation_seconds_per_second(),
        original.rate_standard_deviation_seconds_per_second()
    );
    assert_eq!(retired.remaining_phase_seconds(), Some(1.0 / 64.0));
    {
        let fresh = product_ok(retired.resume_fresh(controller_clock_point(2, 8)?))?;
        assert_eq!(fresh.remaining_phase_seconds(), None);
        assert_eq!(fresh.producer.emitted_rounds(), 1);
        check_counter_witness(&fresh)?;
    }
    assert!(retired.cursor().is_none());
    assert!(
        retired.frequency_history().is_some(),
        "dropping fresh owner erased history"
    );
    let retained = retired
        .frequency_history()
        .ok_or_else(|| io::Error::other("consumed transfer lost historical model"))?;
    super::assert_unknown_frequency_history(&retained, &history);
    Ok(())
}

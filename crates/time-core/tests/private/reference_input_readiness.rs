// SPDX-License-Identifier: GPL-2.0-only
//! Independent genuine-outcome controls for the modeled reference-input gate.
//!
//! The four original tests reuse two genuine three-row histories; one later
//! observation supplies round two. Five finite achieved-phase families add exact
//! dyadic, rejection, monotonicity, post-admission failure and retained controls.
//! No checked outcome or reference credit is manufactured. Foreign bindings
//! reuse one genuine original-chart outcome.

use time_core::{
    ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitError, FixtureChartError,
    FixtureChartField, FixtureChartIssue, FixtureChartStep, FixtureCorrectionFeedback,
    FixtureReferenceInput, FixtureReferenceInputError, FixtureReferenceInputState,
    FixtureReferenceReadiness,
};

use super::{
    FixtureAffineChart, FixtureChartBatch, FixtureChartCombinationOutcome, FixtureChartDomain,
    FixtureChartFrames, FixtureContext, FixturePrincipalOutcomeKind, MonotonicNs, SECOND, Source,
    TestResult, chart, entries, io, label, ptr, seed, strata,
};

/// Require the exact public rejection without constructing a substitute outcome.
/// # Errors
/// Reject unexpected acceptance rather than manufacturing the expected error.
/// # Panics
/// Reject an error different from the independently selected contract branch.
fn rejected<'chart>(
    state: &FixtureReferenceInputState<'chart>,
    outcome: &FixtureChartCombinationOutcome<'_, '_, 'chart>,
    expected: FixtureReferenceInputError,
) -> TestResult {
    let error = state
        .admit_reference_input(outcome)
        .err()
        .ok_or_else(|| io::Error::other("reference input unexpectedly admitted"))?;
    assert_eq!(error, expected);
    Ok(())
}

/// Observe admission after a literal truth-table transition, preserving round1.
/// # Errors
/// Preserve healthy admission errors or reject an unexpected invalid-state success.
/// # Panics
/// Reject changed readiness, estimate identity or original emitted round.
fn check_readiness<'chart>(
    state: &FixtureReferenceInputState<'chart>,
    outcome: &FixtureChartCombinationOutcome<'_, '_, 'chart>,
    expected: FixtureReferenceReadiness,
) -> TestResult {
    assert_eq!(state.readiness(), expected);
    if expected == FixtureReferenceReadiness::Ready {
        let original = outcome
            .combined()
            .ok_or_else(|| io::Error::other("missing genuine ready estimate"))?;
        let input = state.admit_reference_input(outcome)?;
        assert!(ptr::eq(input.combined(), original));
        assert_eq!(input.emitted_round(), label(1)?);
    } else {
        rejected(
            state,
            outcome,
            FixtureReferenceInputError::Unavailable {
                readiness: expected,
            },
        )?;
    }
    assert_eq!(state.readiness(), expected);
    Ok(())
}

/// Check bounded, distinguishable error families and metadata-only debug output.
/// # Panics
/// Reject empty or conflated diagnostics, an unbounded payload or raw chart data.
fn check_diagnostics(
    state: &FixtureReferenceInputState<'_>,
    input: &FixtureReferenceInput<'_, '_, '_, '_>,
) {
    let errors = [
        FixtureReferenceInputError::Unavailable {
            readiness: FixtureReferenceReadiness::Ready,
        },
        FixtureReferenceInputError::Unavailable {
            readiness: FixtureReferenceReadiness::CoordinateChanged,
        },
        FixtureReferenceInputError::Unavailable {
            readiness: FixtureReferenceReadiness::Unknown,
        },
        FixtureReferenceInputError::NotReady,
        FixtureReferenceInputError::ForeignChartView,
        FixtureReferenceInputError::ValidationInvariant,
    ];
    let messages = errors.map(|error| error.to_string());
    for message in &messages {
        assert_ne!(message, "");
        assert!(message.len() <= 128);
    }
    let families = [&messages[0], &messages[3], &messages[4], &messages[5]];
    for (index, family) in families.iter().enumerate() {
        assert!(families.iter().take(index).all(|earlier| earlier != family));
    }
    let owner_debug = format!("{state:?}");
    for text in [
        owner_debug,
        format!("{input:?}"),
        format!("{:?}", FixtureReferenceInputError::NotReady),
    ] {
        assert_ne!(text, "");
        assert!(text.len() <= 128);
        for raw in ["17", "23", "19", "29", "0.00390625", "0.0009765625"] {
            assert!(!text.contains(raw));
        }
    }
}

/// Reading a real Ready outcome preserves its provenance and upstream credit.
/// # Errors
/// Preserve actual fixture production, combination and readiness admission failures.
/// # Panics
/// Reject substituted metadata, another estimate or new credit from read-only reuse.
#[test]
fn genuine_ready_input_is_readonly_and_replay_has_no_new_credit() -> TestResult {
    let mut a = Source::new(11, [0, 2, 0])?;
    let b = Source::new(22, [1, 3, 1])?;
    let map = chart()?;
    let strata = strata()?;
    let original = entries(&a, &b, &map, 4, 0.25, true)?;
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &original)?;
    let mut owner = seed(&original, &strata)?;
    let ready = owner.advance_chart(&map, &batch, &strata)?;
    assert_eq!(
        ready.principal().kind(),
        FixturePrincipalOutcomeKind::ReferenceReady
    );
    assert_eq!(ready.emitted_round(), Some(label(1)?));
    let before = [
        owner.source_state(label(11)?).copied(),
        owner.source_state(label(22)?).copied(),
    ];
    let state = FixtureReferenceInputState::new(&map);
    let input = state.admit_reference_input(&ready)?;
    check_diagnostics(&state, &input);
    let checked = input.combined();
    assert!(ptr::eq(checked.chart(), batch.chart()));
    assert!(ptr::eq(checked.batch(), ptr::from_ref(&batch)));
    assert_eq!(
        checked.source_context(),
        FixtureContext::new(label(11)?, label(17)?, label(23)?)
    );
    assert_eq!(
        checked.context(),
        FixtureContext::new(label(11)?, label(19)?, label(29)?)
    );
    assert_eq!(checked.anchor_observed_at(), MonotonicNs::new(2 * SECOND));
    assert_eq!(checked.latest_observed_at(), MonotonicNs::new(2 * SECOND));
    assert_eq!(checked.projected_at(), MonotonicNs::new(4 * SECOND));
    assert_eq!(input.emitted_round(), label(1)?);
    for _ in 0..3 {
        check_readiness(&state, &ready, FixtureReferenceReadiness::Ready)?;
    }
    assert_eq!(owner.emitted_rounds(), 1);
    assert_eq!(
        [
            owner.source_state(label(11)?).copied(),
            owner.source_state(label(22)?).copied(),
        ],
        before
    );
    for id in [11, 22] {
        let source = owner
            .principal_state()
            .source_state(label(id)?)
            .ok_or_else(|| io::Error::other("missing original source credit"))?;
        assert_eq!(
            source.latest_observed_at(),
            Some(MonotonicNs::new(2 * SECOND))
        );
        assert!(!source.pending());
    }
    let retained = owner.advance_chart(&map, &batch, &strata)?;
    assert_eq!(
        retained.principal().kind(),
        FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    );
    assert!(retained.combined().is_none());
    assert_eq!(retained.emitted_round(), None);
    rejected(&state, &retained, FixtureReferenceInputError::NotReady)?;
    assert_eq!(state.readiness(), FixtureReferenceReadiness::Ready);
    assert_eq!(owner.emitted_rounds(), 1);

    // A later genuine emission distinguishes round preservation from constant one.
    a.append(3, 0, 1.0)?;
    let fresh = entries(&a, &b, &map, 5, 0.25, true)?;
    let fresh_batch = FixtureChartBatch::new(&map, MonotonicNs::new(5 * SECOND), &fresh)?;
    let second = owner.advance_chart(&map, &fresh_batch, &strata)?;
    assert_eq!(second.emitted_round(), Some(label(2)?));
    let second_input = state.admit_reference_input(&second)?;
    assert_eq!(second_input.emitted_round(), label(2)?);
    assert_eq!(
        second_input.combined().latest_observed_at(),
        MonotonicNs::new(3 * SECOND)
    );
    assert_eq!(input.emitted_round(), label(1)?);
    assert_eq!(
        input.combined().latest_observed_at(),
        MonotonicNs::new(2 * SECOND)
    );
    assert_eq!(owner.emitted_rounds(), 2);
    Ok(())
}

/// An actual empty selection and exhausted pending credit cannot become input.
/// # Errors
/// Preserve genuine chart, seed and lifecycle errors beside the healthy neighbor.
/// # Panics
/// Reject fabricated readiness or error precedence that revives an unknown owner.
#[test]
fn unavailable_and_waiting_outcomes_reject_without_reviving_readiness() -> TestResult {
    let a = Source::new(11, [0, 2, 0])?;
    let b = Source::new(22, [1, 3, 1])?;
    let map = chart()?;
    let strata = strata()?;
    let original = entries(&a, &b, &map, 4, 0.25, true)?;
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &original)?;
    let mut owner = seed(&original, &strata)?;
    let ready = owner.advance_chart(&map, &batch, &strata)?;
    let mut state = FixtureReferenceInputState::new(&map);
    check_readiness(&state, &ready, FixtureReferenceReadiness::Ready)?;
    let empty = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &[])?;
    let unavailable = owner.advance_chart(&map, &empty, &[])?;
    assert_eq!(
        unavailable.principal().kind(),
        FixturePrincipalOutcomeKind::Unavailable
    );
    assert!(unavailable.combined().is_none());
    assert_eq!(unavailable.emitted_round(), None);
    rejected(&state, &unavailable, FixtureReferenceInputError::NotReady)?;
    let waiting = owner.advance_chart(&map, &batch, &strata)?;
    assert_eq!(
        waiting.principal().kind(),
        FixturePrincipalOutcomeKind::WaitingForUpdate
    );
    assert!(waiting.combined().is_none());
    assert_eq!(waiting.emitted_round(), None);
    rejected(&state, &waiting, FixtureReferenceInputError::NotReady)?;
    assert_eq!(state.readiness(), FixtureReferenceReadiness::Ready);
    assert_eq!(owner.emitted_rounds(), 1);
    assert_eq!(
        state.receive_feedback(FixtureCorrectionFeedback::Unknown),
        FixtureReferenceReadiness::Unknown
    );
    for outcome in [&unavailable, &waiting] {
        rejected(
            &state,
            outcome,
            FixtureReferenceInputError::Unavailable {
                readiness: FixtureReferenceReadiness::Unknown,
            },
        )?;
    }
    assert_eq!(state.readiness(), FixtureReferenceReadiness::Unknown);
    assert_eq!(owner.emitted_rounds(), 1);
    Ok(())
}

/// Construct all eight foreign instances without changing the genuine outcome.
/// # Errors
/// Preserve complete checked descriptor admission; no invalid source is projected.
fn foreign_charts(map: &FixtureAffineChart) -> TestResult<[FixtureAffineChart; 8]> {
    let source_frame = FixtureChartFrames::new(label(31)?, label(23)?, label(19)?, label(29)?)?;
    let source_generation =
        FixtureChartFrames::new(label(17)?, label(31)?, label(19)?, label(29)?)?;
    let target_frame = FixtureChartFrames::new(label(17)?, label(23)?, label(31)?, label(29)?)?;
    let target_generation =
        FixtureChartFrames::new(label(17)?, label(23)?, label(19)?, label(31)?)?;
    let wider = FixtureChartDomain::new(
        MonotonicNs::new(0),
        MonotonicNs::new(2 * SECOND),
        MonotonicNs::new(9 * SECOND),
    )?;
    Ok([
        chart()?,
        FixtureAffineChart::new(map.frames(), map.domain(), 5.0 / 1024.0, 1.0 / 1024.0)?,
        FixtureAffineChart::new(map.frames(), map.domain(), 4.0 / 1024.0, 2.0 / 1024.0)?,
        FixtureAffineChart::new(map.frames(), wider, 4.0 / 1024.0, 1.0 / 1024.0)?,
        FixtureAffineChart::new(source_frame, map.domain(), 4.0 / 1024.0, 1.0 / 1024.0)?,
        FixtureAffineChart::new(source_generation, map.domain(), 4.0 / 1024.0, 1.0 / 1024.0)?,
        FixtureAffineChart::new(target_frame, map.domain(), 4.0 / 1024.0, 1.0 / 1024.0)?,
        FixtureAffineChart::new(target_generation, map.domain(), 4.0 / 1024.0, 1.0 / 1024.0)?,
    ])
}

/// Equal labels/values and every changed descriptor remain foreign instances.
/// # Errors
/// Preserve genuine fixture construction and healthy original-instance admissions.
/// # Panics
/// Reject value-based identity, masked readiness precedence or a reject-all gate.
#[test]
fn foreign_chart_instances_reject_beside_the_bound_original() -> TestResult {
    let a = Source::new(11, [0, 2, 0])?;
    let b = Source::new(22, [1, 3, 1])?;
    let map = chart()?;
    let strata = strata()?;
    let original = entries(&a, &b, &map, 4, 0.25, true)?;
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &original)?;
    let mut owner = seed(&original, &strata)?;
    let ready = owner.advance_chart(&map, &batch, &strata)?;
    let bound = FixtureReferenceInputState::new(&map);
    let foreign = foreign_charts(&map)?;
    assert_eq!(foreign[0].frames(), map.frames());
    assert_eq!(foreign[0].domain(), map.domain());
    assert_eq!(
        foreign[0].phase_at_reference_seconds().to_bits(),
        map.phase_at_reference_seconds().to_bits()
    );
    assert_eq!(
        foreign[0].rate_seconds_per_second().to_bits(),
        map.rate_seconds_per_second().to_bits()
    );
    assert_ne!(
        foreign[1].phase_at_reference_seconds().to_bits(),
        map.phase_at_reference_seconds().to_bits()
    );
    assert_ne!(
        foreign[2].rate_seconds_per_second().to_bits(),
        map.rate_seconds_per_second().to_bits()
    );
    assert_ne!(foreign[3].domain(), map.domain());
    for candidate in &foreign {
        check_readiness(&bound, &ready, FixtureReferenceReadiness::Ready)?;
        let mut different = FixtureReferenceInputState::new(candidate);
        rejected(
            &different,
            &ready,
            FixtureReferenceInputError::ForeignChartView,
        )?;
        assert_eq!(different.readiness(), FixtureReferenceReadiness::Ready);
        assert_eq!(
            different.receive_feedback(FixtureCorrectionFeedback::CoordinateChanged),
            FixtureReferenceReadiness::CoordinateChanged
        );
        rejected(
            &different,
            &ready,
            FixtureReferenceInputError::Unavailable {
                readiness: FixtureReferenceReadiness::CoordinateChanged,
            },
        )?;
        check_readiness(&bound, &ready, FixtureReferenceReadiness::Ready)?;
    }
    assert_eq!(owner.emitted_rounds(), 1);
    Ok(())
}

/// All nine ordered feedback pairs implement the literal monotone truth table.
/// # Errors
/// Preserve genuine production or fail an admission inconsistent with the table.
/// # Panics
/// Reject reset by no-effect, loss of Unknown dominance, or non-idempotent repeats.
#[test]
fn feedback_pairs_and_repeats_never_restore_an_invalidated_input() -> TestResult {
    use FixtureCorrectionFeedback::{CoordinateChanged as C, ProvenNoEffect as N, Unknown as U};
    use FixtureReferenceReadiness::{CoordinateChanged as Changed, Ready, Unknown};

    let a = Source::new(11, [0, 2, 0])?;
    let b = Source::new(22, [1, 3, 1])?;
    let map = chart()?;
    let strata = strata()?;
    let original = entries(&a, &b, &map, 4, 0.25, true)?;
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &original)?;
    let mut owner = seed(&original, &strata)?;
    let ready = owner.advance_chart(&map, &batch, &strata)?;
    // Independent literals, rather than the production transition or a rank helper.
    for (first, after_first, second, expected) in [
        (N, Ready, N, Ready),
        (N, Ready, C, Changed),
        (N, Ready, U, Unknown),
        (C, Changed, N, Changed),
        (C, Changed, C, Changed),
        (C, Changed, U, Unknown),
        (U, Unknown, N, Unknown),
        (U, Unknown, C, Unknown),
        (U, Unknown, U, Unknown),
    ] {
        let mut state = FixtureReferenceInputState::new(&map);
        assert_eq!(state.readiness(), Ready);
        assert_eq!(state.receive_feedback(first), after_first);
        assert_eq!(state.receive_feedback(first), after_first);
        check_readiness(&state, &ready, after_first)?;
        assert_eq!(state.receive_feedback(second), expected);
        assert_eq!(state.receive_feedback(second), expected);
        check_readiness(&state, &ready, expected)?;
        assert_eq!(state.receive_feedback(N), expected);
        check_readiness(&state, &ready, expected)?;
    }
    assert_eq!(owner.emitted_rounds(), 1);
    Ok(())
}

/// Admit a distinct B→C mapping with deliberately unordered generation labels.
/// # Errors
/// Preserve nonzero-label and checked relationship rejection.
fn achieved_frames() -> TestResult<FixtureChartFrames> {
    Ok(FixtureChartFrames::new(
        label(19)?,
        label(29)?,
        label(31)?,
        label(7)?,
    )?)
}

/// Exact rows, source context, counts and coefficient bits of one three-row owner.
type AchievedSourceSnapshot = (
    FixtureContext,
    [(u64, i128, u64); 3],
    usize,
    usize,
    u64,
    u64,
);

/// Capture only genuine retained rows and the original fitted report.
/// # Errors
/// Reject unexpected fixture shape or a missing actual row/report.
fn achieved_source_snapshot(source: &Source) -> TestResult<AchievedSourceSnapshot> {
    let sample_count = source.history.sample_count();
    let auxiliary_count = source.history.auxiliary_count();
    if sample_count != 3 || auxiliary_count != 0 {
        return Err(io::Error::other("achieved phase source shape").into());
    }
    let mut rows = [(0, 0, 0); 3];
    for (index, slot) in rows.iter_mut().enumerate() {
        let row = source
            .history
            .decision_observation(index)
            .ok_or_else(|| io::Error::other("missing achieved phase source row"))?;
        *slot = (
            row.observed_at().nanoseconds(),
            row.phase().numerator(),
            row.variance_divisor().to_bits(),
        );
    }
    let line = source
        .history
        .report()
        .ok_or_else(|| io::Error::other("missing achieved phase fitted report"))?
        .selection()
        .statistics()
        .line();
    Ok((
        source.context,
        rows,
        sample_count,
        auxiliary_count,
        line.phase_seconds().to_bits(),
        line.phase_rate_seconds_per_second().to_bits(),
    ))
}

/// Four literal achieved quantities add phase and preserve rate, including ±0.
/// # Errors
/// Preserve genuine history, projection and checked receiving failures.
/// # Panics
/// Reject wrong correction sign, requested-value substitution or zero-as-no-effect.
#[test]
fn achieved_phase_dyadics_change_coordinates_and_preserve_rate() -> TestResult {
    let source = Source::new(11, [0, 2, 0])?;
    let map = chart()?;
    let frames = achieved_frames()?;
    let domain = map.domain();
    let original = source.entry(&map, 4, 0.25, false, 0.125)?;
    // Weighted slope is 1/24: a zero latest phase extrapolates to ideal
    // raw center 1/12 at t=4. Its actual checked point is the transformation
    // input; this test makes no new fit-rounding promise. These bounds keep
    // all literal subtractions on one exact binary64 grid.
    let raw_center = original.projection().source_projection().center_seconds();
    assert!((9.0 / 128.0..15.0 / 128.0).contains(&raw_center));
    assert_eq!(
        original.projection().center_seconds(),
        raw_center - 3.0 / 512.0
    );
    for (achieved, replacement_phase, replacement_displacement) in [
        (1.0 / 512.0, 3.0 / 512.0, 4.0 / 512.0),
        (-1.0 / 512.0, 1.0 / 512.0, 2.0 / 512.0),
        (0.0, 1.0 / 256.0, 3.0 / 512.0),
        (-0.0, 1.0 / 256.0, 3.0 / 512.0),
    ] {
        let mut state = FixtureReferenceInputState::new(&map);
        let completion = state.receive_achieved_phase(&map, frames, domain, achieved)?;
        assert_eq!(
            state.readiness(),
            FixtureReferenceReadiness::CoordinateChanged
        );
        let replacement = completion.into_replacement()?;
        assert_eq!(replacement.frames().target_frame(), label(31)?);
        assert_eq!(replacement.frames().target_generation(), label(7)?);
        assert_eq!(replacement.domain(), domain);
        assert_eq!(replacement.phase_at_reference_seconds(), replacement_phase);
        assert_eq!(replacement.rate_seconds_per_second(), 1.0 / 1024.0);
        let remapped = source.entry(&replacement, 4, 0.25, false, 0.125)?;
        assert_eq!(
            remapped.projection().center_seconds(),
            raw_center - replacement_displacement
        );
        assert_eq!(
            remapped.projection().center_seconds(),
            original.projection().center_seconds() - achieved
        );
    }
    let negative_zero_rate = FixtureAffineChart::new(map.frames(), domain, 1.0 / 256.0, -0.0)?;
    let mut negative_zero_state = FixtureReferenceInputState::new(&negative_zero_rate);
    let preserved = negative_zero_state
        .receive_achieved_phase(&negative_zero_rate, frames, domain, 1.0 / 512.0)?
        .into_replacement()?;
    assert_eq!(
        preserved.rate_seconds_per_second().to_bits(),
        (-0.0_f64).to_bits()
    );
    Ok(())
}

/// One literal rejection and its independently selected accepted neighbor.
type AchievedRejectionCase<'chart> = (
    &'chart FixtureAffineChart,
    FixtureChartFrames,
    FixtureChartDomain,
    f64,
    FixtureReferenceInputError,
    f64,
);

/// Eight checked context cases; no receiver or history operation occurs here.
/// # Errors
/// Preserve the same four frame and three domain constructor failures.
fn achieved_context_cases<'chart>(
    map: &'chart FixtureAffineChart,
    equal_foreign: &'chart FixtureAffineChart,
    frames: FixtureChartFrames,
    domain: FixtureChartDomain,
) -> TestResult<[AchievedRejectionCase<'chart>; 8]> {
    let wrong_frame = FixtureChartFrames::new(label(41)?, label(29)?, label(31)?, label(7)?)?;
    let wrong_generation = FixtureChartFrames::new(label(19)?, label(41)?, label(31)?, label(7)?)?;
    let frame_cycle = FixtureChartFrames::new(label(19)?, label(29)?, label(17)?, label(7)?)?;
    let generation_cycle = FixtureChartFrames::new(label(19)?, label(29)?, label(31)?, label(23)?)?;
    let later_start = FixtureChartDomain::new(
        MonotonicNs::new(SECOND),
        MonotonicNs::new(2 * SECOND),
        MonotonicNs::new(8 * SECOND),
    )?;
    let later_reference = FixtureChartDomain::new(
        MonotonicNs::new(0),
        MonotonicNs::new(3 * SECOND),
        MonotonicNs::new(8 * SECOND),
    )?;
    let later_end = FixtureChartDomain::new(
        MonotonicNs::new(0),
        MonotonicNs::new(2 * SECOND),
        MonotonicNs::new(9 * SECOND),
    )?;
    let chart_error = |source| FixtureReferenceInputError::Chart { source };
    let mapping = chart_error(FixtureChartError::CompositionMappingMismatch);
    let different_domain = chart_error(FixtureChartError::CompositionDomainMismatch);
    let endpoint = chart_error(FixtureChartError::InvalidFrames);
    Ok([
        (
            equal_foreign,
            frames,
            domain,
            1.0 / 512.0,
            FixtureReferenceInputError::ForeignChartView,
            1.0 / 512.0,
        ),
        (map, wrong_frame, domain, 1.0 / 512.0, mapping, 1.0 / 512.0),
        (
            map,
            wrong_generation,
            domain,
            1.0 / 512.0,
            mapping,
            1.0 / 512.0,
        ),
        (
            map,
            frames,
            later_start,
            1.0 / 512.0,
            different_domain,
            1.0 / 512.0,
        ),
        (
            map,
            frames,
            later_reference,
            1.0 / 512.0,
            different_domain,
            1.0 / 512.0,
        ),
        (
            map,
            frames,
            later_end,
            1.0 / 512.0,
            different_domain,
            1.0 / 512.0,
        ),
        (map, frame_cycle, domain, 1.0 / 512.0, endpoint, 1.0 / 512.0),
        (
            map,
            generation_cycle,
            domain,
            1.0 / 512.0,
            endpoint,
            1.0 / 512.0,
        ),
    ])
}

/// Five unsupported scalar literals paired with supported signed neighbors.
fn achieved_scalar_cases(
    map: &FixtureAffineChart,
    frames: FixtureChartFrames,
    domain: FixtureChartDomain,
) -> [AchievedRejectionCase<'_>; 5] {
    let chart_error = |source| FixtureReferenceInputError::Chart { source };
    let nonfinite = chart_error(FixtureChartError::InvalidScalar {
        field: FixtureChartField::Displacement,
        issue: FixtureChartIssue::NonFinite,
    });
    let subnormal = chart_error(FixtureChartError::InvalidScalar {
        field: FixtureChartField::Displacement,
        issue: FixtureChartIssue::Subnormal,
    });
    [
        (map, frames, domain, f64::NAN, nonfinite, 1.0 / 512.0),
        (map, frames, domain, f64::INFINITY, nonfinite, 1.0 / 512.0),
        (
            map,
            frames,
            domain,
            f64::NEG_INFINITY,
            nonfinite,
            -1.0 / 512.0,
        ),
        (
            map,
            frames,
            domain,
            f64::from_bits(1),
            subnormal,
            f64::MIN_POSITIVE,
        ),
        (
            map,
            frames,
            domain,
            -f64::from_bits(1),
            subnormal,
            -f64::MIN_POSITIVE,
        ),
    ]
}

/// Thirteen literal relationship/scalar failures leave a valid neighbor usable.
/// # Errors
/// Preserve checked descriptors and each same-owner accepted neighboring effect.
/// # Panics
/// Reject value-based identity, incomplete context checks, premature commit or reject-all.
#[test]
fn achieved_phase_context_and_scalar_rejections_preserve_ready_owner() -> TestResult {
    let map = chart()?;
    let equal_foreign = chart()?;
    let frames = achieved_frames()?;
    let domain = map.domain();
    // Invalid B→C endpoints cannot cross the checked frame constructor either.
    assert_eq!(
        FixtureChartFrames::new(label(19)?, label(29)?, label(19)?, label(7)?).err(),
        Some(FixtureChartError::InvalidFrames)
    );
    assert_eq!(
        FixtureChartFrames::new(label(19)?, label(29)?, label(31)?, label(29)?).err(),
        Some(FixtureChartError::InvalidFrames)
    );
    // Exactly eight context plus five scalar rows; each retains its healthy call.
    let cases = achieved_context_cases(&map, &equal_foreign, frames, domain)?
        .into_iter()
        .chain(achieved_scalar_cases(&map, frames, domain));
    for (predecessor, candidate_frames, candidate_domain, phase, expected, neighbor) in cases {
        let mut state = FixtureReferenceInputState::new(&map);
        assert_eq!(
            state
                .receive_achieved_phase(predecessor, candidate_frames, candidate_domain, phase)
                .err(),
            Some(expected)
        );
        assert_eq!(state.readiness(), FixtureReferenceReadiness::Ready);
        let accepted = state
            .receive_achieved_phase(&map, frames, domain, neighbor)?
            .into_replacement()?;
        assert_eq!(accepted.rate_seconds_per_second(), 1.0 / 1024.0);
        assert_eq!(
            state.readiness(),
            FixtureReferenceReadiness::CoordinateChanged
        );
    }
    Ok(())
}

/// No-effect is identity; achieved repetition rejects and Unknown stays dominant.
/// # Errors
/// Preserve checked chart construction and the single admitted achieved phase.
/// # Panics
/// Reject readiness restoration or local replay admission after invalidation.
#[test]
fn achieved_phase_noeffect_unknown_and_repetition_are_monotone() -> TestResult {
    let map = chart()?;
    let frames = achieved_frames()?;
    let domain = map.domain();
    let mut state = FixtureReferenceInputState::new(&map);
    assert_eq!(
        state.receive_feedback(FixtureCorrectionFeedback::ProvenNoEffect),
        FixtureReferenceReadiness::Ready
    );
    assert_eq!(
        state.receive_feedback(FixtureCorrectionFeedback::ProvenNoEffect),
        FixtureReferenceReadiness::Ready
    );
    let replacement = state
        .receive_achieved_phase(&map, frames, domain, 1.0 / 512.0)?
        .into_replacement()?;
    assert_eq!(replacement.phase_at_reference_seconds(), 3.0 / 512.0);
    assert_eq!(
        state
            .receive_achieved_phase(&map, frames, domain, 1.0 / 512.0)
            .err(),
        Some(FixtureReferenceInputError::Unavailable {
            readiness: FixtureReferenceReadiness::CoordinateChanged
        })
    );
    assert_eq!(
        state.receive_feedback(FixtureCorrectionFeedback::ProvenNoEffect),
        FixtureReferenceReadiness::CoordinateChanged
    );
    assert_eq!(
        state.receive_feedback(FixtureCorrectionFeedback::Unknown),
        FixtureReferenceReadiness::Unknown
    );
    assert_eq!(
        state
            .receive_achieved_phase(&map, frames, domain, 1.0 / 512.0)
            .err(),
        Some(FixtureReferenceInputError::Unavailable {
            readiness: FixtureReferenceReadiness::Unknown
        })
    );
    assert_eq!(
        state.receive_feedback(FixtureCorrectionFeedback::CoordinateChanged),
        FixtureReferenceReadiness::Unknown
    );
    assert_eq!(
        state.receive_feedback(FixtureCorrectionFeedback::ProvenNoEffect),
        FixtureReferenceReadiness::Unknown
    );
    assert_eq!(
        state
            .receive_achieved_phase(&map, frames, domain, 1.0 / 512.0)
            .err(),
        Some(FixtureReferenceInputError::Unavailable {
            readiness: FixtureReferenceReadiness::Unknown
        })
    );
    assert_eq!(state.readiness(), FixtureReferenceReadiness::Unknown);
    Ok(())
}

/// Finite achieved input is admitted before replacement overflow is reported.
/// # Errors
/// Preserve legitimate predecessor and effect context admission failures.
/// # Panics
/// Reject unchanged-state rejection, a lost exact arithmetic cause or revived readiness.
#[test]
fn achieved_phase_arithmetic_failure_is_admitted_and_invalidates_old_owner() -> TestResult {
    let ordinary = chart()?;
    let map = FixtureAffineChart::new(ordinary.frames(), ordinary.domain(), f64::MAX, 0.0)?;
    let frames = achieved_frames()?;
    let domain = map.domain();
    let mut state = FixtureReferenceInputState::new(&map);
    let completion = state.receive_achieved_phase(&map, frames, domain, f64::MAX)?;
    let expected = FixtureChartError::Arithmetic {
        step: FixtureChartStep::ComposeDisplacement,
        source: FitError::Arithmetic {
            stage: ArithmeticStage::Response,
            operation: ArithmeticOperation::Add,
            issue: ArithmeticIssue::NonFinite,
        },
    };
    assert_eq!(
        format!("{completion:?}"),
        "FixtureAchievedPhaseCompletion { .. }"
    );
    let receiving_error = FixtureReferenceInputError::Chart { source: expected };
    assert_eq!(
        format!("{receiving_error}"),
        "fixture reference input: achieved phase rejected"
    );
    assert_eq!(
        format!("{receiving_error:?}"),
        "FixtureReferenceInputError { .. }"
    );
    assert_eq!(
        std::error::Error::source(&receiving_error)
            .and_then(|source| source.downcast_ref::<FixtureChartError>()),
        Some(&expected)
    );
    assert!(std::error::Error::source(&FixtureReferenceInputError::NotReady).is_none());
    assert_eq!(completion.replacement().err(), Some(expected));
    assert_eq!(completion.into_replacement().err(), Some(expected));
    assert_eq!(
        state.readiness(),
        FixtureReferenceReadiness::CoordinateChanged
    );
    assert_eq!(
        state
            .receive_achieved_phase(&map, frames, domain, 0.0)
            .err(),
        Some(FixtureReferenceInputError::Unavailable {
            readiness: FixtureReferenceReadiness::CoordinateChanged
        })
    );
    Ok(())
}

/// Compare one genuine projection pair without producing or remapping either value.
/// # Errors
/// Preserve label admission or a missing genuine tracking attachment.
/// # Panics
/// Reject changed provenance, timestamps, source statistics or phase arithmetic.
fn check_achieved_projection_pair(
    old: &time_core::FixtureChartProjection<'_>,
    new: &time_core::FixtureChartProjection<'_>,
    id: u64,
    old_center: f64,
    new_center: f64,
) -> TestResult {
    assert_eq!(old.center_seconds(), old_center);
    assert_eq!(new.center_seconds(), new_center);
    assert_eq!(
        new.context(),
        FixtureContext::new(label(id)?, label(31)?, label(7)?)
    );
    let old_raw = old.source_projection();
    let new_raw = new.source_projection();
    assert_eq!(new_raw.context(), old_raw.context());
    assert_eq!(new_raw.anchor_observed_at(), old_raw.anchor_observed_at());
    assert_eq!(new_raw.latest_observed_at(), old_raw.latest_observed_at());
    assert_eq!(new_raw.fit_reference(), old_raw.fit_reference());
    assert_eq!(new_raw.projected_at(), old_raw.projected_at());
    assert_eq!(
        new_raw.center_seconds().to_bits(),
        old_raw.center_seconds().to_bits()
    );
    assert_eq!(
        new_raw.distance_seconds().to_bits(),
        old_raw.distance_seconds().to_bits()
    );
    assert_eq!(
        new.phase_rate_seconds_per_second().to_bits(),
        old.phase_rate_seconds_per_second().to_bits()
    );
    assert!(new.center_roundoff_seconds() > old.center_roundoff_seconds());
    let old_tracking = old
        .tracking()
        .ok_or_else(|| io::Error::other("missing original achieved tracking"))?;
    let new_tracking = new
        .tracking()
        .ok_or_else(|| io::Error::other("missing remapped achieved tracking"))?;
    assert!(old_tracking.phase_standard_deviation_seconds() > 0.0);
    assert_eq!(
        new_tracking.phase_standard_deviation_seconds().to_bits(),
        old_tracking.phase_standard_deviation_seconds().to_bits()
    );
    assert_eq!(
        new_tracking
            .rate_standard_deviation_seconds_per_second()
            .to_bits(),
        old_tracking
            .rate_standard_deviation_seconds_per_second()
            .to_bits()
    );
    assert_eq!(
        new_tracking.tracking_skew_seconds_per_second().to_bits(),
        old_tracking.tracking_skew_seconds_per_second().to_bits()
    );
    Ok(())
}

/// Remap two genuine three-row histories through the same retained combination owner.
/// # Errors
/// Preserve genuine histories, projections, enclosures, batch and lifecycle failures.
/// # Panics
/// Reject rewritten evidence, altered statistical uncertainty or remap-created credit.
#[test]
fn achieved_phase_remapped_histories_retain_without_new_estimate() -> TestResult {
    let a = Source::new(11, [0, 2, 0])?;
    let b = Source::new(22, [1, 3, 1])?;
    let before = [achieved_source_snapshot(&a)?, achieved_source_snapshot(&b)?];
    let map = chart()?;
    let frames = achieved_frames()?;
    let domain = map.domain();
    let strata = strata()?;
    let original = entries(&a, &b, &map, 4, 0.25, true)?;
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &original)?;
    let mut owner = seed(&original, &strata)?;
    let first = owner.advance_chart(&map, &batch, &strata)?;
    assert_eq!(first.emitted_round(), Some(label(1)?));
    let mut state = FixtureReferenceInputState::new(&map);
    assert_eq!(
        state.admit_reference_input(&first)?.emitted_round(),
        label(1)?
    );
    let principal_before = owner.principal_state().clone();
    let source_credit_before = [
        owner.source_state(label(11)?).copied(),
        owner.source_state(label(22)?).copied(),
    ];
    let replacement = state
        .receive_achieved_phase(&map, frames, domain, 1.0 / 512.0)?
        .into_replacement()?;
    rejected(
        &state,
        &first,
        FixtureReferenceInputError::Unavailable {
            readiness: FixtureReferenceReadiness::CoordinateChanged,
        },
    )?;
    assert!(replacement.phase_coefficient_roundoff_seconds() > 0.0);
    assert!(replacement.rate_coefficient_roundoff_seconds_per_second() > 0.0);
    let remapped = entries(&a, &b, &replacement, 4, 0.25, true)?;
    for (index, id, lower, upper) in [
        (0, 11, 9.0 / 128.0, 15.0 / 128.0),
        (1, 22, 17.0 / 128.0, 31.0 / 128.0),
    ] {
        let old = original[index].projection();
        let new = remapped[index].projection();
        let raw_center = old.source_projection().center_seconds();
        assert!((lower..upper).contains(&raw_center));
        check_achieved_projection_pair(
            old,
            new,
            id,
            raw_center - 3.0 / 512.0,
            raw_center - 4.0 / 512.0,
        )?;
        assert_eq!(new.center_seconds(), old.center_seconds() - 1.0 / 512.0);
    }
    let remapped_batch =
        FixtureChartBatch::new(&replacement, MonotonicNs::new(4 * SECOND), &remapped)?;
    let retained = owner.advance_chart(&replacement, &remapped_batch, &strata)?;
    assert_eq!(
        retained.principal().kind(),
        FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    );
    assert_eq!(retained.principal().updated_source(), None);
    assert!(retained.combined().is_none());
    assert_eq!(retained.emitted_round(), None);
    assert_eq!(owner.emitted_rounds(), 1);
    assert_eq!(owner.principal_state(), &principal_before);
    assert_eq!(
        [
            owner.source_state(label(11)?).copied(),
            owner.source_state(label(22)?).copied(),
        ],
        source_credit_before
    );
    let replacement_state = FixtureReferenceInputState::new(&replacement);
    rejected(
        &replacement_state,
        &retained,
        FixtureReferenceInputError::NotReady,
    )?;
    assert_eq!(
        [achieved_source_snapshot(&a)?, achieved_source_snapshot(&b)?],
        before
    );
    Ok(())
}

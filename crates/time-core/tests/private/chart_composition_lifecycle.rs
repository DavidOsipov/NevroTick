// SPDX-License-Identifier: GPL-2.0-only
//! S1.4b coefficient-error identity through actual batch and lifecycle consumers.
//!
//! Two genuine three-row histories are reused across two independent mismatch
//! families. Every rejected operation is followed by a useful matching neighbor
//! and replay; no fabricated principal decision or correction receipt is used.

use std::io;

use super::{
    FixtureAffineChart, FixtureChartBatch, FixtureChartCombinationError, FixtureChartDomain,
    FixtureChartFrames, FixturePrincipalOutcomeKind, MonotonicNs, SECOND, Source, TestResult,
    entries, label, seed, strata,
};
use time_core::{
    FixtureChartBatchError, FixtureChartMismatchKind, FixtureChartPrincipalError,
    FixturePrincipalState, FixturePrincipalStratum,
};

/// Three original rows followed by the same report's phase/rate representations.
type SourceSnapshot = ([(u64, i128, u64); 3], u64, u64);

/// Preserve original rows and fitted coefficient bits without retaining a live view.
/// # Errors
/// Reject an unexpected fixture shape or unavailable original row/report.
fn source_snapshot(source: &Source) -> TestResult<SourceSnapshot> {
    if source.history.sample_count() != 3 || source.history.auxiliary_count() != 0 {
        return Err(io::Error::other("composition source shape").into());
    }
    let mut rows = [(0, 0, 0); 3];
    for (index, slot) in rows.iter_mut().enumerate() {
        let row = source
            .history
            .decision_observation(index)
            .ok_or_else(|| io::Error::other("missing composition source row"))?;
        *slot = (
            row.observed_at().nanoseconds(),
            row.phase().numerator(),
            row.variance_divisor().to_bits(),
        );
    }
    let line = source
        .history
        .report()
        .ok_or_else(|| io::Error::other("missing composition fitted report"))?
        .selection()
        .statistics()
        .line();
    Ok((
        rows,
        line.phase_seconds().to_bits(),
        line.phase_rate_seconds_per_second().to_bits(),
    ))
}

/// Build a zero-point chain with independently varied rate-roundoff magnitude.
/// # Errors
/// Preserve checked labels, domains, scalar and composition admission failures.
fn zero_composition(
    common: FixtureChartDomain,
    cancellation_rate: bool,
) -> TestResult<FixtureAffineChart> {
    let rate = if cancellation_rate { 1.0 } else { 0.0 };
    let left = FixtureAffineChart::new(
        FixtureChartFrames::new(label(17)?, label(23)?, label(19)?, label(29)?)?,
        common,
        0.0,
        rate,
    )?;
    let right = FixtureAffineChart::new(
        FixtureChartFrames::new(label(19)?, label(29)?, label(31)?, label(37)?)?,
        common,
        0.0,
        -rate,
    )?;
    Ok(left.compose(&right)?)
}

/// Reject an incompatible expected chart before a real principal consumes credit.
/// # Errors
/// Preserve the accepted same-event and replay transition's genuine errors.
/// # Panics
/// Reject a changed mismatch cause, partial state commit or reused observation credit.
fn principal_control(
    principal: &mut FixturePrincipalState,
    actual: &FixtureAffineChart,
    expected: &FixtureAffineChart,
    batch: &FixtureChartBatch<'_, '_>,
    strata: &[FixturePrincipalStratum; 2],
    kind: FixtureChartMismatchKind,
) -> TestResult {
    let before = principal.clone();
    assert!(matches!(
        principal.advance_chart(expected, batch, strata),
        Err(FixtureChartPrincipalError::ChartMismatch { kind: rejected }) if rejected == kind
    ));
    assert_eq!(*principal, before);
    let ready = principal.advance_chart(actual, batch, strata)?;
    assert_eq!(ready.kind(), FixturePrincipalOutcomeKind::ReferenceReady);
    let retained = principal.advance_chart(actual, batch, strata)?;
    assert_eq!(
        retained.kind(),
        FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    );
    assert_eq!(retained.updated_source(), None);
    Ok(())
}

/// Reject the same mismatch before a real combination owner can emit a round.
/// # Errors
/// Preserve genuine matching-event and replay failures.
/// # Panics
/// Reject partial lifecycle state changes or replay-created emission.
fn combination_control(
    combined: &mut super::FixtureCombinationState,
    control: &super::FixtureCombinationState,
    actual: &FixtureAffineChart,
    expected: &FixtureAffineChart,
    batch: &FixtureChartBatch<'_, '_>,
    strata: &[FixturePrincipalStratum; 2],
    kind: FixtureChartMismatchKind,
) -> TestResult {
    assert_eq!(&*combined, control);
    assert!(matches!(
        combined.advance_chart(expected, batch, strata),
        Err(FixtureChartCombinationError::Principal {
            source: FixtureChartPrincipalError::ChartMismatch { kind: rejected }
        }) if rejected == kind
    ));
    assert_eq!(&*combined, control);
    assert_eq!(combined.emitted_rounds(), 0);
    let ready = combined.advance_chart(actual, batch, strata)?;
    assert_eq!(
        ready.principal().kind(),
        FixturePrincipalOutcomeKind::ReferenceReady
    );
    assert_eq!(ready.emitted_round(), Some(label(1)?));
    assert!(ready.combined().is_some());
    let retained = combined.advance_chart(actual, batch, strata)?;
    assert_eq!(
        retained.principal().kind(),
        FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    );
    assert_eq!(retained.emitted_round(), None);
    assert!(retained.combined().is_none());
    assert_eq!(combined.emitted_rounds(), 1);
    Ok(())
}

/// Changed coefficient-error bits reject before consuming real pending credit.
/// # Errors
/// Preserve genuine history, projection, batch, seed and accepted-transition errors.
/// # Panics
/// Reject lost bound identity, partial owner mutation or replay-created emission.
#[test]
fn coefficient_error_identity_precedes_batch_principal_and_combination_updates() -> TestResult {
    let a = Source::new(11, [0, 2, 0])?;
    let b = Source::new(22, [1, 3, 1])?;
    let before = [source_snapshot(&a)?, source_snapshot(&b)?];
    let common = FixtureChartDomain::new(
        MonotonicNs::new(0),
        MonotonicNs::new(2 * SECOND),
        MonotonicNs::new(8 * SECOND),
    )?;
    let direct = FixtureAffineChart::new(
        FixtureChartFrames::new(label(17)?, label(23)?, label(31)?, label(37)?)?,
        common,
        0.0,
        0.0,
    )?;
    let composed_zero = zero_composition(common, false)?;
    let composed_rate = zero_composition(common, true)?;
    assert_eq!(
        composed_zero.phase_at_reference_seconds().to_bits(),
        direct.phase_at_reference_seconds().to_bits()
    );
    assert_eq!(
        composed_zero.rate_seconds_per_second().to_bits(),
        direct.rate_seconds_per_second().to_bits()
    );
    assert!(composed_zero.phase_coefficient_roundoff_seconds() > 0.0);
    assert_eq!(
        composed_zero.phase_coefficient_roundoff_seconds().to_bits(),
        composed_rate.phase_coefficient_roundoff_seconds().to_bits()
    );
    assert!(
        composed_rate.rate_coefficient_roundoff_seconds_per_second()
            > composed_zero.rate_coefficient_roundoff_seconds_per_second()
    );
    let strata = strata()?;
    let at = MonotonicNs::new(4 * SECOND);
    for (actual, expected, kind) in [
        (
            &direct,
            &composed_zero,
            FixtureChartMismatchKind::PhaseCoefficientRoundoff,
        ),
        (
            &composed_zero,
            &composed_rate,
            FixtureChartMismatchKind::RateCoefficientRoundoff,
        ),
    ] {
        let current = entries(&a, &b, actual, 4, 0.25, true)?;
        let batch = FixtureChartBatch::new(actual, at, &current)?;
        assert!(matches!(
            FixtureChartBatch::new(expected, at, &current),
            Err(FixtureChartBatchError::Mismatch { index: 0, kind: rejected }) if rejected == kind
        ));
        let mut combined = seed(&current, &strata)?;
        let control = seed(&current, &strata)?;
        let mut principal = combined.principal_state().clone();
        principal_control(&mut principal, actual, expected, &batch, &strata, kind)?;
        combination_control(
            &mut combined,
            &control,
            actual,
            expected,
            &batch,
            &strata,
            kind,
        )?;
        assert_eq!([source_snapshot(&a)?, source_snapshot(&b)?], before);
    }
    Ok(())
}

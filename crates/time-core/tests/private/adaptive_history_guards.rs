// SPDX-License-Identifier: GPL-2.0-only
//! Direct minimum/count admission and genuine history postcondition witnesses.
//! Checked histories, reports and selections are never forged or modified.
//! Unpublished candidates test missing reports and cross-associated intact ones.

use std::{error::Error, io, num::NonZeroU64};

use garde::Validate;
use time_domain::{MonotonicNs, UtcCoordinateQ32, exchange_arithmetic};

use super::{
    AdaptiveHistoryError, CommittedAdaptiveState, FixtureAdaptiveHistory, FixtureContext,
    FixtureObservation, MAX_POINTS, RawMinimum, combined_prefix, validate_complete,
};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

/// Construct one genuine checked observation from bounded fixture coordinates.
/// # Errors
/// Reject second conversion/phase admission failure; perform no append or fit.
fn checked_observation(
    context: FixtureContext,
    second: u64,
    numerator: i32,
) -> TestResult<FixtureObservation> {
    let nanos = second
        .checked_mul(1_000_000_000)
        .ok_or_else(|| io::Error::other("fixture time overflow"))?;
    let zero = UtcCoordinateQ32::new(0)?;
    let shifted = UtcCoordinateQ32::new(i128::from(numerator) * (1_i128 << 23))?;
    let phase = exchange_arithmetic(zero, shifted, shifted, zero)?.offset();
    Ok(FixtureObservation::new(
        context,
        MonotonicNs::new(nanos),
        phase,
        1.0,
    )?)
}

/// Build at most nine chronological rows solely through the real history owner.
/// # Errors
/// Reject an unsupported fixture count or any actual constructor/append failure.
/// Across its five callers this constructs five histories and nineteen rows,
/// and performs nineteen appends; it never installs a private state or report.
fn checked_history(values: &[i32]) -> TestResult<FixtureAdaptiveHistory> {
    if !(1..=9).contains(&values.len()) {
        return Err(io::Error::other("fixture count outside one through nine").into());
    }
    let context = FixtureContext::new(NonZeroU64::MIN, NonZeroU64::MIN, NonZeroU64::MIN);
    let mut history = FixtureAdaptiveHistory::new(context, 3)?;
    for (index, numerator) in values.iter().copied().enumerate() {
        history.try_append(checked_observation(
            context,
            u64::try_from(index)?,
            numerator,
        )?)?;
    }
    Ok(history)
}

/// Construct a genuine three-row donor with explicit context and first second.
/// Two callers produce six observations/appends and two one-candidate fits.
/// # Errors
/// Preserve time overflow, checked observation and real history owner failures;
/// no report fields or private state are supplied by this fixture constructor.
fn checked_donor(context: FixtureContext, first_second: u64) -> TestResult<FixtureAdaptiveHistory> {
    let mut history = FixtureAdaptiveHistory::new(context, 3)?;
    for elapsed in 0..3_u64 {
        let second = first_second
            .checked_add(elapsed)
            .ok_or_else(|| io::Error::other("donor fixture time overflow"))?;
        history.try_append(checked_observation(context, second, 0)?)?;
    }
    Ok(history)
}

/// Borrow the committed state of a genuinely constructed nonempty test owner.
/// # Errors
/// Reject an empty owner; never manufacture, mutate or publish checked state.
fn checked_state(history: &FixtureAdaptiveHistory) -> TestResult<&CommittedAdaptiveState> {
    Ok(history
        .state
        .as_ref()
        .ok_or_else(|| io::Error::other("missing genuine committed fixture state"))?)
}

/// Check the actual derived field rule independently of constructor predicates.
/// Six literal candidates call Garde at most six times and construct no history,
/// observation, fit or report; there are no state, native or external effects.
/// # Panics
/// Reject an omitted rule, wrong bound or rejection of either valid endpoint.
#[test]
fn adaptive_raw_minimum_semantic_neighbors() {
    for (value, expected) in [
        (0, false),
        (2, false),
        (3, true),
        (64, true),
        (65, false),
        (usize::MAX, false),
    ] {
        assert_eq!(RawMinimum { value }.validate().is_ok(), expected);
    }
}

/// Challenge the actual count receiver with eleven literal boundary tuples.
/// One checked observation initializes the raw array; no row ordering, history,
/// normalization or fit is claimed by this count-only borrowing helper.
/// # Errors
/// Preserve checked observation construction and unexpected accepted-case errors.
/// # Panics
/// Reject invalid counts and retain the original borrowed prefix for valid ones.
#[test]
fn adaptive_combined_prefix_semantic_neighbors() -> TestResult {
    let context = FixtureContext::new(NonZeroU64::MIN, NonZeroU64::MIN, NonZeroU64::MIN);
    let observations = [checked_observation(context, 0, 0)?; MAX_POINTS];
    for (auxiliary, fitted, expected) in [
        (0, 0, None),
        (0, 1, Some(1)),
        (1, 1, Some(2)),
        (2, 1, None),
        (0, 64, Some(64)),
        (32, 32, Some(64)),
        (33, 32, None),
        (0, 65, None),
        (usize::MAX, 1, None),
        (0, usize::MAX, None),
        (usize::MAX, usize::MAX, None),
    ] {
        let actual = combined_prefix(&observations, auxiliary, fitted);
        if let Some(count) = expected {
            let rows = actual?;
            assert_eq!(rows.len(), count);
            assert!(std::ptr::eq(rows.as_ptr(), observations.as_ptr()));
        } else {
            assert!(matches!(actual, Err(AdaptiveHistoryError::StateInvariant)));
        }
    }
    Ok(())
}

/// Check nine context/time/provenance inputs against one unchanged warmup owner.
/// One genuine observation and append perform no fit; only raw validator
/// arguments vary, and the final healthy neighbor reuses the same borrowed state.
/// # Errors
/// Preserve actual owner construction and unexpected healthy validation failure.
/// # Panics
/// Every incorrect parameter must reject without requiring invalid checked data.
#[test]
fn adaptive_warmup_postconditions_reject_wrong_arguments() -> TestResult {
    let history = checked_history(&[0])?;
    let state = checked_state(&history)?;
    let context = history.context();
    let other = FixtureContext::new(
        NonZeroU64::new(2).ok_or_else(|| io::Error::other("zero fixture source"))?,
        context.frame(),
        context.generation(),
    );
    assert_eq!((state.sample_count, state.auxiliary_count), (1, 0));
    assert!(state.report.is_none());
    for (claimed_context, newest, original_count, original_auxiliary, expected) in [
        (context, 0, 1, 0, true),
        (other, 0, 1, 0, false),
        (context, 1, 1, 0, false),
        (context, 0, 0, 0, false),
        (context, 0, 2, 0, false),
        (context, 0, 1, 1, false),
        (context, 0, usize::MAX, 0, false),
        (context, 0, 1, usize::MAX, false),
        (context, 0, 1, 0, true),
    ] {
        let actual = validate_complete(
            state,
            claimed_context,
            3,
            original_count,
            original_auxiliary,
            MonotonicNs::new(newest),
        );
        if expected {
            actual?;
        } else {
            assert!(matches!(actual, Err(AdaptiveHistoryError::StateInvariant)));
        }
    }
    Ok(())
}

/// Exercise four raw minimum claims beside one genuine three-row report.
/// Three checked appends produce one actual selector/fit; the report is borrowed
/// unchanged and both wrong claims are individually valid configured minima.
/// # Errors
/// Preserve actual owner work and healthy postcondition validation failures.
/// # Panics
/// A mismatched configured minimum must reject even when the effective minimum
/// remains three, independently of the matching report and fit statistics.
#[test]
fn adaptive_complete_postconditions_bind_configured_minimum() -> TestResult {
    let history = checked_history(&[0, 0, 0])?;
    let state = checked_state(&history)?;
    let context = history.context();
    let newest = MonotonicNs::new(2_000_000_000);
    assert_eq!((state.sample_count, state.auxiliary_count), (3, 0));
    assert!(state.report.is_some());
    for (minimum, expected) in [(3, true), (4, false), (64, false), (3, true)] {
        let actual = validate_complete(state, context, minimum, 3, 0, newest);
        if expected {
            actual?;
        } else {
            assert!(matches!(actual, Err(AdaptiveHistoryError::StateInvariant)));
        }
    }
    Ok(())
}

/// Check five raw auxiliary-prefix claims beside the actual first-prune state.
/// Nine checked rows/appends produce the retained t=0 prefix and fitted t=1..8
/// suffix. This fixed trace needs eight candidate fits; no checked state changes.
/// # Errors
/// Preserve genuine owner construction and unexpected healthy validation failure.
/// # Panics
/// A claimed old auxiliary prefix cannot extend past the selected fit start.
#[test]
fn adaptive_complete_postconditions_bind_original_auxiliary() -> TestResult {
    let history = checked_history(&[512, -21, 27, 27, 27, -27, -27, -27, -27])?;
    let state = checked_state(&history)?;
    let context = history.context();
    let newest = MonotonicNs::new(8_000_000_000);
    let report = history
        .report()
        .ok_or_else(|| io::Error::other("missing genuine first-prune report"))?;
    assert_eq!((state.sample_count, state.auxiliary_count), (8, 1));
    assert_eq!(
        (
            report.selection().start(),
            report.selection().decision_start(),
            report.selection().decision_count(),
        ),
        (1, 0, 9)
    );
    for (auxiliary, expected) in [
        (0, true),
        (1, true),
        (2, false),
        (usize::MAX, false),
        (0, true),
    ] {
        let actual = validate_complete(state, context, 3, 9, auxiliary, newest);
        if expected {
            actual?;
        } else {
            assert!(matches!(actual, Err(AdaptiveHistoryError::StateInvariant)));
        }
    }
    Ok(())
}

/// Check an unpublished report-free candidate at two and three genuine rows.
/// Three checked observations initialize one raw array. Each candidate reaches
/// only final validation: neither is installed in a history, and no checked
/// report/selection, normalization or fit is manufactured or executed.
/// # Errors
/// Preserve checked observation construction and healthy warmup admission.
/// # Panics
/// Three fitted rows require a report before publication; two do not.
#[test]
fn adaptive_unpublished_three_rows_require_report() -> TestResult {
    let context = FixtureContext::new(NonZeroU64::MIN, NonZeroU64::MIN, NonZeroU64::MIN);
    let first = checked_observation(context, 0, 0)?;
    let mut observations = [first; MAX_POINTS];
    observations[1] = checked_observation(context, 1, 0)?;
    observations[2] = checked_observation(context, 2, 0)?;
    for (sample_count, newest, expected) in [(2, 1_000_000_000, true), (3, 2_000_000_000, false)] {
        let candidate = CommittedAdaptiveState {
            observations,
            auxiliary_count: 0,
            sample_count,
            report: None,
        };
        let actual = validate_complete(
            &candidate,
            context,
            3,
            sample_count,
            0,
            MonotonicNs::new(newest),
        );
        if expected {
            actual?;
        } else {
            assert!(matches!(actual, Err(AdaptiveHistoryError::StateInvariant)));
        }
    }
    Ok(())
}

/// Check a fresh candidate assembled with an intact report from another source.
/// Two genuine three-row histories perform six appends/two fits. The donor is
/// consumed to move its whole checked report; no report/selection field changes,
/// and the candidate never enters a published history. Two validator calls
/// compare the healthy original association with the wrong-source association.
/// # Errors
/// Preserve real fixture work and healthy complete-state validation failures.
/// # Panics
/// Matching frame, time, counts and minima cannot excuse a different source tag.
#[test]
fn adaptive_unpublished_report_requires_same_context() -> TestResult {
    let history = checked_history(&[0, 0, 0])?;
    let state = checked_state(&history)?;
    let context = history.context();
    let other = FixtureContext::new(
        NonZeroU64::new(2).ok_or_else(|| io::Error::other("zero donor source"))?,
        context.frame(),
        context.generation(),
    );
    let newest = MonotonicNs::new(2_000_000_000);
    validate_complete(state, context, 3, 3, 0, newest)?;
    let donor = checked_donor(other, 0)?;
    let report = donor
        .state
        .ok_or_else(|| io::Error::other("missing genuine donor state"))?
        .report
        .ok_or_else(|| io::Error::other("missing genuine donor report"))?;
    assert_eq!(report.context(), other);
    assert_eq!(report.reference(), newest);
    let candidate = CommittedAdaptiveState {
        observations: state.observations,
        auxiliary_count: state.auxiliary_count,
        sample_count: state.sample_count,
        report: Some(report),
    };
    assert!(matches!(
        validate_complete(&candidate, context, 3, 3, 0, newest),
        Err(AdaptiveHistoryError::StateInvariant)
    ));
    Ok(())
}

/// Check a fresh candidate assembled with an intact report from a later window.
/// Two genuine three-row histories perform six appends/two fits. The donor
/// starts one second later with the same context, minima and zero-phase fit.
/// Its whole report moves into an unpublished candidate without field changes;
/// two validator calls contrast healthy and mismatched time associations.
/// # Errors
/// Preserve genuine fixture work and healthy complete-state validation failures.
/// # Panics
/// A same-frame report must retain the exact newest time of its actual rows.
#[test]
fn adaptive_unpublished_report_requires_same_reference() -> TestResult {
    let history = checked_history(&[0, 0, 0])?;
    let state = checked_state(&history)?;
    let context = history.context();
    let newest = MonotonicNs::new(2_000_000_000);
    validate_complete(state, context, 3, 3, 0, newest)?;
    let donor = checked_donor(context, 1)?;
    let report = donor
        .state
        .ok_or_else(|| io::Error::other("missing genuine shifted donor state"))?
        .report
        .ok_or_else(|| io::Error::other("missing genuine shifted donor report"))?;
    assert_eq!(report.context(), context);
    assert_eq!(report.reference().nanoseconds(), 3_000_000_000);
    let candidate = CommittedAdaptiveState {
        observations: state.observations,
        auxiliary_count: state.auxiliary_count,
        sample_count: state.sample_count,
        report: Some(report),
    };
    assert!(matches!(
        validate_complete(&candidate, context, 3, 3, 0, newest),
        Err(AdaptiveHistoryError::StateInvariant)
    ));
    Ok(())
}

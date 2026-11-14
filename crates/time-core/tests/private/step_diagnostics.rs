// SPDX-License-Identifier: GPL-2.0-only
//! Bounded Step diagnostics and historical custody through genuine source owners.
//!
//! Diagnostic strings expose no rates, timestamps or source labels. The owning
//! paths below distinguish actual effects, explicit absence and unresolved work;
//! no formatter or getter establishes native completion or fresh source credit.

use std::{error::Error, io};

use time_core::FixtureStepRetirement;

use super::{
    ActiveAwakeNs, FixtureChartFrames, FixturePhaseError, FixturePhaseProvenance,
    FixturePhaseResidualPermission, FixtureRateCursorStatus, FixtureStepCompletion,
    FixtureStepDirections, FixtureStepError, FixtureStepFeedback, FixtureStepLimits,
    FixtureStepSettlement, MonotonicNs, SECOND, TestResult, clocks, cursor_snapshot, label, now,
    product_ok, step_limits, step_receipt, with_step_owner,
};

/// Observe the fixed public family while retaining the exact original typed cause.
/// # Panics
/// Detect payload disclosure, an unrelated delegated cause or a fabricated leaf cause.
fn check_error(
    error: &FixtureStepError,
    display: &str,
    expected_source: Option<&FixturePhaseError>,
) {
    assert_eq!(format!("{error:?}"), "FixtureStepError { .. }");
    assert_eq!(error.to_string(), display);
    assert!(display.is_ascii() && display.len() <= 64);
    match expected_source {
        Some(expected) => assert_eq!(
            error
                .source()
                .and_then(|actual| actual.downcast_ref::<FixturePhaseError>()),
            Some(expected)
        ),
        None => assert!(error.source().is_none()),
    }
}

/// Keep checked limits/receipts opaque while using their actual copied interval.
/// # Errors
/// Preserve genuine source, checked construction and unexpected rejection failures.
/// # Panics
/// Detect leaked payloads, malformed-chain acceptance or replacement of receipt fields.
#[test]
fn matched_step_constructor_diagnostics_are_opaque_and_preserve_checked_receipts() -> TestResult {
    let invalid_limits = FixtureStepLimits::new(0.0, FixtureStepDirections::Both);
    assert!(matches!(
        &invalid_limits,
        Err(FixtureStepError::InvalidLimits)
    ));
    let invalid = invalid_limits
        .err()
        .ok_or_else(|| io::Error::other("zero step cap was admitted"))?;
    assert_eq!(invalid, FixtureStepError::InvalidLimits);
    check_error(&invalid, "fixture step: invalid explicit limits", None);
    let limits = step_limits(FixtureStepDirections::Both)?;
    assert_eq!(limits.maximum_abs_seconds(), 1.0 / 4.0);
    assert_eq!(limits.directions(), FixtureStepDirections::Both);
    assert_eq!(format!("{limits:?}"), "FixtureStepLimits { .. }");
    with_step_owner(1, |mut owner| {
        product_ok(owner.issue_step(
            now(0, 4 * SECOND)?,
            FixturePhaseResidualPermission::Allow,
            limits,
        ))?;
        let original = step_receipt(&owner, 1, 1.0 / 32.0, 1.0 / 32.0)?;
        assert_eq!(format!("{original:?}"), "FixtureStepCompletion { .. }");
        let wrong = FixtureChartFrames::new(label(47)?, label(37)?, label(41)?, label(43)?)?;
        let foreign = FixtureStepCompletion::new(original.delivery(), 1.0 / 32.0, wrong);
        assert!(matches!(&foreign, Err(FixtureStepError::Receipt)));
        let refused = foreign
            .err()
            .ok_or_else(|| io::Error::other("foreign intermediate step frame was admitted"))?;
        assert_eq!(refused, FixtureStepError::Receipt);
        check_error(&refused, "fixture step: unmatched receipt", None);
        // The copied getters must retain a receipt usable by the real pending
        // receiver, not only produce values that compare with themselves.
        let copied = product_ok(FixtureStepCompletion::new(
            original.delivery(),
            original.achieved_step_seconds(),
            original.final_frames(),
        ))?;
        let feedback = FixtureStepFeedback::Known(copied);
        assert_eq!(format!("{feedback:?}"), "FixtureStepFeedback { .. }");
        let result = owner.settle_step(feedback, now(2 * SECOND, 8 * SECOND)?);
        assert_eq!(format!("{result:?}"), "FixtureStepSettlement { .. }");
        assert!(matches!(&result, FixtureStepSettlement::Known(_)));
        let FixtureStepSettlement::Known(retired) = result else {
            return Err(io::Error::other("copied checked receipt lost Known custody").into());
        };
        assert_eq!(retired.requested_phase_seconds(), 1.0 / 8.0);
        assert_eq!(retired.achieved_step_seconds(), 1.0 / 32.0);
        assert_eq!(retired.remaining_phase_seconds(), Some(3.0 / 32.0));
        assert_eq!(
            retired.replacement()?.phase_at_reference_seconds(),
            3.0 / 32.0
        );
        Ok(())
    })
}

/// Real request refusals preserve the same source owner and its subsequent pending slot.
/// # Errors
/// Preserve genuine source, limits, request and expected refusal extraction failures.
/// # Panics
/// Detect changed refusal meaning, lost delegation or consumption before healthy issue.
#[test]
fn matched_step_request_diagnostics_keep_exact_refusals_and_pending_custody() -> TestResult {
    with_step_owner(1, |mut owner| {
        let before = cursor_snapshot(owner.cursor());
        for (limits, expected, display) in [
            (
                product_ok(FixtureStepLimits::new(
                    1.0 / 16.0,
                    FixtureStepDirections::Both,
                ))?,
                FixtureStepError::Magnitude,
                "fixture step: magnitude exceeded",
            ),
            (
                step_limits(FixtureStepDirections::Negative)?,
                FixtureStepError::Direction,
                "fixture step: direction refused",
            ),
        ] {
            let result = owner.issue_step(
                now(0, 4 * SECOND)?,
                FixturePhaseResidualPermission::Allow,
                limits,
            );
            assert!(matches!(&result, Err(error) if error == &expected));
            let refused = result
                .err()
                .ok_or_else(|| io::Error::other("forbidden step request was admitted"))?;
            assert_eq!(refused, expected);
            check_error(&refused, display, None);
            assert_eq!(cursor_snapshot(owner.cursor()), before);
            assert!(owner.step_proposal().is_none());
        }
        product_ok(owner.issue_step(
            now(0, 4 * SECOND)?,
            FixturePhaseResidualPermission::Allow,
            step_limits(FixtureStepDirections::Both)?,
        ))?;
        let result = owner.issue_step(
            now(0, 4 * SECOND)?,
            FixturePhaseResidualPermission::Allow,
            step_limits(FixtureStepDirections::Both)?,
        );
        assert!(matches!(
            &result,
            Err(FixtureStepError::Phase {
                source: FixturePhaseError::OutstandingOperation
            })
        ));
        let refused = result
            .err()
            .ok_or_else(|| io::Error::other("second step replaced the pending request"))?;
        let source = FixturePhaseError::OutstandingOperation;
        assert!(matches!(
            &refused,
            FixtureStepError::Phase {
                source: FixturePhaseError::OutstandingOperation
            }
        ));
        check_error(&refused, "fixture step: phase owner refused", Some(&source));
        assert_eq!(cursor_snapshot(owner.cursor()), before);
        assert_eq!(
            owner.step_proposal().map(|value| value.ordinal()),
            Some(label(1)?)
        );
        Ok(())
    })
}

/// A genuinely neutral fitted reference has a bounded leaf refusal beside signed neighbors.
/// # Errors
/// Preserve genuine source and checked request/refusal construction failures.
/// # Panics
/// Detect invented nonzero phase, delegated cause or consumed neutral request credit.
#[test]
fn matched_step_neutral_diagnostic_has_no_delegated_cause() -> TestResult {
    with_step_owner(0, |mut owner| {
        let before = cursor_snapshot(owner.cursor());
        let result = owner.issue_step(
            now(0, 4 * SECOND)?,
            FixturePhaseResidualPermission::Allow,
            step_limits(FixtureStepDirections::Both)?,
        );
        assert!(matches!(&result, Err(FixtureStepError::NoUsefulStep)));
        let refused = result
            .err()
            .ok_or_else(|| io::Error::other("neutral source acquired a signed request"))?;
        assert_eq!(refused, FixtureStepError::NoUsefulStep);
        check_error(&refused, "fixture step: no established nonzero debt", None);
        assert_eq!(cursor_snapshot(owner.cursor()), before);
        assert_eq!(owner.remaining_phase_seconds(), Some(0.0));
        Ok(())
    })
}

/// Compare retained original emission fields without treating them as fresh authority.
/// # Panics
/// Detect any relabelled source, renewed observation age or replaced original estimate.
fn check_provenance(actual: &FixturePhaseProvenance, expected: &FixturePhaseProvenance) {
    assert_eq!(
        actual.phase_seconds().to_bits(),
        expected.phase_seconds().to_bits()
    );
    assert_eq!(
        actual.phase_error_seconds().to_bits(),
        expected.phase_error_seconds().to_bits()
    );
    assert_eq!(actual.emitted_round(), expected.emitted_round());
    assert_eq!(actual.source_context(), expected.source_context());
    assert_eq!(actual.frames(), expected.frames());
    assert_eq!(actual.anchor_observed_at(), expected.anchor_observed_at());
    assert_eq!(actual.latest_observed_at(), expected.latest_observed_at());
    assert_eq!(actual.fit_reference(), expected.fit_reference());
    assert_eq!(actual.projected_at(), expected.projected_at());
}

/// Historical request/effect metadata survives taking away useful owner custody.
/// # Errors
/// Report missing genuine source custody instead of manufacturing replacement metadata.
/// # Panics
/// Detect refreshed deadlines, requested-as-applied feedback or fabricated model history.
fn check_retired_facts(
    retired: &FixtureStepRetirement<'_>,
    original: &FixturePhaseProvenance,
) -> TestResult {
    assert_eq!(format!("{retired:?}"), "FixtureStepRetirement { .. }");
    assert_eq!(retired.requested_phase_seconds(), 1.0 / 8.0);
    assert_eq!(
        retired.requested_phase_error_seconds().to_bits(),
        original.phase_error_seconds().to_bits()
    );
    assert_eq!(retired.achieved_step_seconds(), 1.0 / 32.0);
    assert_eq!(retired.pre_step_phase_seconds(), Some(1.0 / 8.0));
    assert!(
        retired
            .pre_step_phase_error_seconds()
            .is_some_and(|error| error >= original.phase_error_seconds())
    );
    assert_eq!(retired.remaining_phase_seconds(), Some(3.0 / 32.0));
    assert_eq!(
        retired.replacement()?.phase_at_reference_seconds(),
        3.0 / 32.0
    );
    assert_eq!(retired.active_deadline(), ActiveAwakeNs::new(8 * SECOND));
    assert_eq!(retired.monotonic_deadline(), MonotonicNs::new(20 * SECOND));
    assert!(retired.frequency_history().is_none());
    assert!(retired.feedback_error().is_none());
    assert!(retired.continuation_error().is_none());
    assert!(retired.debt_error().is_none());
    assert!(
        retired.provenance().is_some(),
        "lost original source provenance"
    );
    let retained = retired
        .provenance()
        .ok_or_else(|| io::Error::other("retirement lost its original source metadata"))?;
    check_provenance(retained, original);
    Ok(())
}

/// Dropping the sole recovered owner cannot recreate the consumed transfer slot.
/// # Errors
/// Preserve genuine source, request, Known receipt and recovery failures.
/// # Panics
/// Detect concurrent or sequential duplication, lost historical facts or reused phase credit.
#[test]
fn matched_step_retirement_diagnostics_preserve_provenance_after_sole_transfer() -> TestResult {
    with_step_owner(1, |mut owner| {
        assert!(
            owner.provenance().is_some(),
            "genuine owner lost source provenance"
        );
        let original = owner
            .provenance()
            .copied()
            .ok_or_else(|| io::Error::other("genuine step owner has no admitted source"))?;
        let proposal = product_ok(owner.issue_step(
            now(0, 4 * SECOND)?,
            FixturePhaseResidualPermission::Allow,
            step_limits(FixtureStepDirections::Both)?,
        ))?;
        assert_eq!(format!("{proposal:?}"), "FixtureStepProposal { .. }");
        assert_eq!(
            proposal.requested_phase_error_seconds().to_bits(),
            original.phase_error_seconds().to_bits()
        );
        assert_eq!(proposal.known_baseline().rate(), 1.0 / 32.0);
        check_provenance(proposal.provenance(), &original);
        let receipt = step_receipt(&owner, 1, 1.0 / 32.0, 1.0 / 32.0)?;
        let result = owner.settle_step(
            FixtureStepFeedback::Known(receipt),
            now(2 * SECOND, 8 * SECOND)?,
        );
        assert!(matches!(&result, FixtureStepSettlement::Known(_)));
        let FixtureStepSettlement::Known(mut retired) = result else {
            return Err(io::Error::other("actual step did not retain Known retirement").into());
        };
        check_retired_facts(&retired, &original)?;
        {
            let mut fresh = product_ok(retired.resume_fresh(now(2 * SECOND, 8 * SECOND)?))?;
            assert_eq!(fresh.cursor().next_ordinal(), Some(label(2)?));
            assert_eq!(fresh.remaining_phase_seconds(), None);
            assert!(
                fresh.provenance().is_some(),
                "fresh owner lost historical provenance"
            );
            let retained = fresh
                .provenance()
                .ok_or_else(|| io::Error::other("fresh owner lost historical source metadata"))?;
            check_provenance(retained, &original);
            assert_eq!(
                fresh
                    .issue(
                        now(2 * SECOND, 8 * SECOND)?,
                        FixturePhaseResidualPermission::Allow
                    )
                    .err(),
                Some(FixturePhaseError::NoReference)
            );
        }
        assert!(retired.cursor().is_none());
        let transferred = retired.resume_fresh(now(2 * SECOND, 8 * SECOND)?);
        assert!(matches!(
            &transferred,
            Err(FixtureStepError::Phase {
                source: FixturePhaseError::TransferConsumed
            })
        ));
        let refused = transferred
            .err()
            .ok_or_else(|| io::Error::other("dropped owner restored consumed step transfer"))?;
        let source = FixturePhaseError::TransferConsumed;
        assert!(matches!(
            &refused,
            FixtureStepError::Phase {
                source: FixturePhaseError::TransferConsumed
            }
        ));
        check_error(&refused, "fixture step: phase owner refused", Some(&source));
        assert!(retired.cursor().is_none());
        check_retired_facts(&retired, &original)
    })
}

/// Identically opaque terminal formatting cannot conflate absence with uncertainty.
/// # Errors
/// Preserve genuine source, checked issuance and exact terminal identity failures.
/// # Panics
/// Detect leaked context/ordinal, refunded identities or fabricated delivered endpoints.
#[test]
fn matched_step_terminal_diagnostics_do_not_conflate_no_effect_and_unknown() -> TestResult {
    for unknown in [false, true] {
        with_step_owner(1, |mut owner| {
            product_ok(owner.issue_step(
                now(0, 4 * SECOND)?,
                FixturePhaseResidualPermission::Allow,
                step_limits(FixtureStepDirections::Both)?,
            ))?;
            let feedback = if unknown {
                FixtureStepFeedback::Unknown {
                    context: clocks()?,
                    ordinal: label(1)?,
                }
            } else {
                FixtureStepFeedback::ProvenNoEffect {
                    context: clocks()?,
                    ordinal: label(1)?,
                }
            };
            assert_eq!(format!("{feedback:?}"), "FixtureStepFeedback { .. }");
            let result = owner.settle_step(feedback, now(0, 5 * SECOND)?);
            assert_eq!(format!("{result:?}"), "FixtureStepSettlement { .. }");
            assert!(
                matches!(&result, FixtureStepSettlement::Unknown(_) if unknown)
                    || matches!(&result, FixtureStepSettlement::ProvenNoEffect(_) if !unknown)
            );
            match result {
                FixtureStepSettlement::ProvenNoEffect(owner) if !unknown => {
                    assert_eq!(owner.cursor().next_ordinal(), Some(label(2)?));
                    assert_eq!(owner.cursor().last_active_end(), None);
                    assert_eq!(owner.cursor().last_monotonic_end(), None);
                    assert_eq!(owner.remaining_phase_seconds(), Some(1.0 / 8.0));
                }
                FixtureStepSettlement::Unknown(blocked) if unknown => {
                    assert_eq!(
                        blocked.cursor().status(),
                        FixtureRateCursorStatus::Unresolved
                    );
                    assert_eq!(blocked.cursor().next_ordinal(), None);
                    assert_eq!(blocked.cursor().last_active_end(), None);
                    assert_eq!(blocked.cursor().last_monotonic_end(), None);
                    assert_eq!(blocked.last_known_phase_seconds(), Some(1.0 / 8.0));
                }
                _ => {
                    return Err(
                        io::Error::other("opaque diagnostic changed terminal semantics").into(),
                    );
                }
            }
            Ok(())
        })?;
    }
    Ok(())
}

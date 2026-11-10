// SPDX-License-Identifier: GPL-2.0-only
//! Deterministic boundary controls for the real private progress API.
//! These controls perform no socket I/O and make no production path claim.

use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

use super::{AddressRoundConfig, Progress};
use crate::PilotError;

/// Preserve typed product and fixture failures without unwrapping or suppression.
type Check<T> = Result<T, Box<dyn std::error::Error>>;

/// Assert the independently known-valid product result before propagating it.
/// # Errors
/// Retain the original typed refusal.
/// # Panics
/// Reject only the unexpected product refusal named by the static fixture fact.
fn product_ok<T, E>(value: Result<T, E>, fact: &'static str) -> Result<T, E> {
    assert!(value.is_ok(), "{fact}");
    value
}

/// Require the independently expected available position without inventing one.
/// # Errors
/// An absent claim retains a fixed fixture cause after its assertion.
/// # Panics
/// Reject the unexpected absence named by the independent static fixture fact.
fn product_some<T>(value: Result<Option<T>, PilotError>, fact: &'static str) -> Check<T> {
    let value = product_ok(value, fact)?;
    assert!(value.is_some(), "{fact}");
    value.ok_or_else(|| fact.into())
}

/// Three distinct endpoints include both families in an intentionally fixed order.
fn peers() -> [SocketAddr; 3] {
    [
        SocketAddr::from(([127, 0, 0, 1], 41001)),
        SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 1], 41002)),
        SocketAddr::from(([127, 0, 0, 1], 41003)),
    ]
}

/// Use the actual parent constructor, including its mandatory Garde admission.
/// # Errors
/// Preserve rejection after asserting the independent fixture's valid admission.
fn config(peers: &[SocketAddr], attempt: u64, whole: u64) -> Check<AddressRoundConfig> {
    Ok(product_ok(
        AddressRoundConfig::new(
            peers,
            Duration::from_secs(attempt),
            Duration::from_secs(whole),
        ),
        "the independent fixture has distinct loopback peers and valid finite budgets",
    )?)
}

/// Consume three independent endpoint expectations in their original order.
/// # Errors
/// Propagate configuration, checked index conversion and completion failures.
/// # Panics
/// Reject changed order/cutoffs, refused legal claims or reopened exhaustion.
#[test]
fn original_order_is_consumed_once_without_deadline_renewal() -> Check<()> {
    let endpoints = peers();
    let configuration = config(&endpoints, 2, 9)?;
    let origin = Instant::now();
    let cutoff = origin + Duration::from_secs(9);
    let mut progress = Progress::new(&configuration, cutoff);
    for (index, peer) in endpoints.into_iter().enumerate() {
        let now = origin + Duration::from_secs(u64::try_from(index)?);
        let claim = product_some(progress.claim_next(now), "one remaining position")?;
        let expected_index = u8::try_from(index)?;
        assert_eq!(
            claim.parts(),
            (expected_index, peer, now + Duration::from_secs(2))
        );
        product_ok(claim.finish(expected_index), "exact active completion")?;
    }
    assert!(
        product_ok(
            progress.claim_next(origin + Duration::from_secs(3)),
            "exhausted round"
        )?
        .is_none()
    );
    assert!(
        product_ok(
            progress.claim_next(origin),
            "exhaustion cannot reopen the round"
        )?
        .is_none()
    );
    Ok(())
}

/// Exercise the one- and two-address neighbors below the maximum capacity.
/// # Errors
/// Propagate configuration, checked index conversion and completion failures.
/// # Panics
/// Reject a changed endpoint/index, missing legal claim or extra position.
#[test]
fn one_and_two_candidate_rounds_stop_at_their_actual_lengths() -> Check<()> {
    let endpoints = peers();
    let origin = Instant::now();
    for count in 1..=2 {
        let configuration = config(&endpoints[..count], 1, 3)?;
        let mut progress = Progress::new(&configuration, origin + Duration::from_secs(3));
        for (index, peer) in endpoints[..count].iter().copied().enumerate() {
            let claim = product_some(progress.claim_next(origin), "admitted remaining peer")?;
            let position = u8::try_from(index)?;
            assert_eq!(claim.parts().0, position);
            assert_eq!(claim.parts().1, peer);
            product_ok(claim.finish(position), "matching position")?;
        }
        assert!(
            product_ok(
                progress.claim_next(origin),
                "actual admitted length is exhausted"
            )?
            .is_none()
        );
    }
    Ok(())
}

/// Require a late-issued attempt to retain the earlier whole-round cutoff.
/// # Errors
/// Propagate configuration, claim and matching-completion failures.
/// # Panics
/// Reject a changed endpoint/cutoff, refused legal claim or renewed round.
#[test]
fn final_attempt_is_clipped_to_the_original_whole_cutoff() -> Check<()> {
    let endpoints = peers();
    let configuration = config(&endpoints[..1], 3, 3)?;
    let origin = Instant::now();
    let cutoff = origin + Duration::from_secs(3);
    let mut progress = Progress::new(&configuration, cutoff);
    let claim = product_some(
        progress.claim_next(origin + Duration::from_secs(2)),
        "before original cutoff",
    )?;
    assert_eq!(claim.parts(), (0, endpoints[0], cutoff));
    product_ok(claim.finish(0), "actual position")?;
    assert!(
        product_ok(
            progress.claim_next(cutoff),
            "whole cutoff is a normal deferral"
        )?
        .is_none()
    );
    Ok(())
}

/// Contrast the exact cutoff and its immediate predecessor/successor.
/// # Errors
/// Propagate configuration/completion failures and an unsupported fixture instant.
/// # Panics
/// Reject late admission, consumption by a refused query or a changed cutoff.
#[test]
fn just_before_cutoff_is_legal_but_exact_and_later_cutoffs_refuse() -> Check<()> {
    let endpoints = peers();
    let configuration = config(&endpoints[..1], 1, 1)?;
    let origin = Instant::now();
    let cutoff = origin + Duration::from_secs(1);
    let mut progress = Progress::new(&configuration, cutoff);
    assert!(product_ok(progress.claim_next(cutoff), "exact cutoff")?.is_none());
    assert!(
        product_ok(
            progress.claim_next(cutoff + Duration::from_nanos(1)),
            "after cutoff"
        )?
        .is_none()
    );
    let before = cutoff
        .checked_sub(Duration::from_nanos(1))
        .ok_or_else(|| std::io::Error::other("fixture cutoff predecessor unavailable"))?;
    let claim = product_some(progress.claim_next(before), "one nanosecond remains")?;
    assert_eq!(claim.parts(), (0, endpoints[0], cutoff));
    product_ok(
        claim.finish(0),
        "refused late queries did not consume the first position",
    )?;
    Ok(())
}

/// Abandon one real claim without manufacturing its completion.
/// # Errors
/// Propagate configuration and initial legal-claim failures.
/// # Panics
/// Reject a missing/wrong initial claim or any subsequent admission after drop.
#[test]
fn dropping_an_unfinished_claim_cannot_admit_a_successor() -> Check<()> {
    let endpoints = peers();
    let configuration = config(&endpoints, 1, 3)?;
    let origin = Instant::now();
    let mut progress = Progress::new(&configuration, origin + Duration::from_secs(3));
    {
        let claim = product_some(progress.claim_next(origin), "first claim")?;
        assert_eq!(claim.parts().0, 0);
    }
    assert!(
        matches!(progress.claim_next(origin), Err(PilotError::InternalState)),
        "abandonment is not completion"
    );
    assert!(matches!(
        progress.claim_next(origin + Duration::from_secs(1)),
        Err(PilotError::InternalState)
    ));
    Ok(())
}

/// Supply a wrong completion index beside a genuinely issued first claim.
/// # Errors
/// Propagate configuration and initial legal-claim failures.
/// # Panics
/// Reject acceptance of the wrong index or release of its still-active position.
#[test]
fn wrong_completion_consumes_the_claim_but_keeps_the_cursor_occupied() -> Check<()> {
    let endpoints = peers();
    let configuration = config(&endpoints, 1, 3)?;
    let origin = Instant::now();
    let mut progress = Progress::new(&configuration, origin + Duration::from_secs(3));
    let claim = product_some(progress.claim_next(origin), "first claim")?;
    assert!(matches!(claim.finish(1), Err(PilotError::InternalState)));
    assert!(matches!(
        progress.claim_next(origin),
        Err(PilotError::InternalState)
    ));
    Ok(())
}

/// Replay the previous index against the genuinely issued successor claim.
/// # Errors
/// Propagate configuration, legal claims and the first matching completion failure.
/// # Panics
/// Reject the wrong successor identity or acceptance/release by the stale index.
#[test]
fn stale_previous_index_cannot_complete_the_next_active_claim() -> Check<()> {
    let endpoints = peers();
    let configuration = config(&endpoints, 1, 3)?;
    let origin = Instant::now();
    let mut progress = Progress::new(&configuration, origin + Duration::from_secs(3));
    let first = product_some(progress.claim_next(origin), "first claim")?;
    product_ok(first.finish(0), "first completion")?;
    let successor = product_some(progress.claim_next(origin), "second claim")?;
    assert_eq!(successor.parts().0, 1);
    assert_eq!(successor.parts().1, endpoints[1]);
    assert!(matches!(
        successor.finish(0),
        Err(PilotError::InternalState)
    ));
    assert!(
        matches!(progress.claim_next(origin), Err(PilotError::InternalState)),
        "stale completion must not release position one"
    );
    Ok(())
}

/// Release completed ownership without renewing the original whole-round cutoff.
/// # Errors
/// Propagate configuration, legal-claim and matching-completion failures.
/// # Panics
/// Reject a missing/wrong initial position or a claim at/after the original cutoff.
#[test]
fn completion_after_the_global_cutoff_does_not_enable_more_work() -> Check<()> {
    let endpoints = peers();
    let configuration = config(&endpoints, 1, 3)?;
    let origin = Instant::now();
    let cutoff = origin + Duration::from_secs(3);
    let mut progress = Progress::new(&configuration, cutoff);
    let claim = product_some(progress.claim_next(origin), "before deadline")?;
    assert_eq!(claim.parts().0, 0);
    // Completion releases custody even when late; it grants no next attempt.
    product_ok(claim.finish(0), "exact completion")?;
    assert!(
        product_ok(
            progress.claim_next(cutoff),
            "late completion does not renew cutoff"
        )?
        .is_none()
    );
    assert!(
        product_ok(
            progress.claim_next(cutoff + Duration::from_nanos(1)),
            "late completion does not renew cutoff"
        )?
        .is_none()
    );
    Ok(())
}

// SPDX-License-Identifier: GPL-2.0-only
//! Same-transition disclosure controls using actual signed recovery evidence.

use super::{
    ConflictEvidence, ConflictProofError, Current, DisclosureAction, DisclosureError,
    DisclosureView, LIMIT, State, base64,
};
use crate::roughtime::{
    exchange::Receipt, round::recovery::proof::tests::signed_recovery_evidence,
};
use std::{
    error::Error,
    sync::{
        MutexGuard,
        mpsc::{self, Receiver, RecvTimeoutError, Sender},
    },
    thread::{self, ScopedJoinHandle},
    time::{Duration, Instant},
};

/// Independent bound on each observation of the controlled worker.
const WATCHDOG: Duration = Duration::from_secs(3);
/// Controlled disclosure accepts the exact 30-second laboratory lifetime.
const MAX_LIFETIME: Duration = Duration::from_secs(30);

/// Check standard Base64 literals across complete and padded groups.
///
/// # Panics
/// Panics when the encoder differs from a literal expected value.
#[test]
fn base64_literal_padding_cases() {
    for (input, expected) in [
        (&b""[..], &b""[..]),
        (&b"f"[..], &b"Zg=="[..]),
        (&b"fo"[..], &b"Zm8="[..]),
        (&b"foo"[..], &b"Zm9v"[..]),
        (&b"foob"[..], &b"Zm9vYg=="[..]),
    ] {
        let mut output: Vec<u8> = Vec::new();
        base64(&mut output, input);
        assert_eq!(output, expected);
    }
}

/// Independently construct the largest inert controlled report profile.
///
/// # Panics
/// Panics if the literal report exceeds its owner limit or differs in length.
#[test]
fn maximal_inert_report_profile_fits_the_output_limit() {
    let root: String = format!("{}=", "A".repeat(43));
    let request: String = format!("{}==", "A".repeat(1382));
    let response: String = format!("{}==", "A".repeat(5462));
    let random: String = format!("{}=", "A".repeat(43));
    let mut report: String = String::from("{\"responses\":[");
    for position in 0..9 {
        if position != 0 {
            report.push(',');
        }
        report.push_str("{\"publicKey\":\"");
        report.push_str(&root);
        report.push_str("\",\"request\":\"");
        report.push_str(&request);
        report.push_str("\",\"response\":\"");
        report.push_str(&response);
        if position != 0 {
            report.push_str("\",\"rand\":\"");
            report.push_str(&random);
        }
        report.push_str("\"}");
    }
    report.push_str("]}");
    assert_eq!(report.len(), 62_871);
    assert!(report.len() <= LIMIT);
}

/// Retire the original session after encoding but before its transfer guard.
///
/// # Errors
/// Returns a fixture, confirmation or recipient observation refusal.
///
/// # Panics
/// Panics if retirement fails or the retired action publishes any bytes.
#[test]
fn session_retirement_before_commit_refuses_publication() -> Result<(), Box<dyn Error>> {
    let evidence: ConflictEvidence = signed_recovery_evidence()?;
    let action: DisclosureAction<'_> = evidence
        .confirm_disclosure(Duration::from_secs(3))
        .map_err(|error| format!("{error:?}"))?;
    let result: Result<(), DisclosureError> = action.submit_using(
        || Some(Instant::now()),
        || {
            assert!(evidence.retire_disclosure().is_ok());
        },
        LIMIT,
        Vec::try_reserve_exact,
    );
    assert!(matches!(result, Err(DisclosureError::Stale)));
    assert!(
        evidence
            .disclosure_report()
            .map_err(|error| format!("{error:?}"))?
            .is_none()
    );
    assert!(matches!(
        evidence.confirm_disclosure(Duration::from_secs(3)),
        Err(DisclosureError::Unavailable)
    ));
    assert_eq!(evidence.conflict().positions(), (0, 2));
    Ok(())
}

/// Replace an action while another thread owns its prepared report and reservation.
///
/// # Errors
/// Returns an unexpected admission, watchdog, worker, or recipient error.
///
/// # Panics
/// Panics if the replacement can submit while the reservation is held, the stale
/// worker publishes, or the original reservation survives worker completion.
fn check_replaced_reservation(evidence: &ConflictEvidence) -> Result<(), Box<dyn Error>> {
    thread::scope(|scope| -> Result<(), Box<dyn Error>> {
        let original: DisclosureAction<'_> = evidence
            .confirm_disclosure(MAX_LIFETIME)
            .map_err(|error| format!("{error:?}"))?;
        let (prepared, preparation): (Sender<()>, Receiver<()>) = mpsc::channel();
        let (resume, resumed): (Sender<()>, Receiver<()>) = mpsc::channel();
        let worker: ScopedJoinHandle<'_, Result<(), DisclosureError>> = scope.spawn(move || {
            original.submit_using(
                || Some(Instant::now()),
                || {
                    assert!(prepared.send(()).is_ok());
                    assert!(resumed.recv_timeout(WATCHDOG).is_ok());
                },
                LIMIT,
                Vec::try_reserve_exact,
            )
        });
        preparation.recv_timeout(WATCHDOG)?;

        let replacement: DisclosureAction<'_> = evidence
            .confirm_disclosure(MAX_LIFETIME)
            .map_err(|error| format!("{error:?}"))?;
        assert!(matches!(replacement.submit(), Err(DisclosureError::Busy)));
        resume.send(())?;
        let outcome: Result<(), DisclosureError> =
            worker.join().map_err(|_| "disclosure worker panicked")?;
        assert!(matches!(outcome, Err(DisclosureError::Stale)));
        assert!(
            evidence
                .disclosure_report()
                .map_err(|error| format!("{error:?}"))?
                .is_none()
        );
        evidence
            .confirm_disclosure(MAX_LIFETIME)
            .and_then(DisclosureAction::submit)
            .map_err(|error| format!("{error:?}"))?;
        assert!(
            evidence
                .disclosure_report()
                .map_err(|error| format!("{error:?}"))?
                .is_some()
        );
        evidence
            .clear_disclosure_report()
            .map_err(|error| format!("{error:?}"))?;
        Ok(())
    })
}

/// Require an overlapping clear to refuse while the recipient view holds its guard.
///
/// # Errors
/// Returns an unexpected observation, watchdog, worker, or clear error.
///
/// # Panics
/// Panics if the contender clears the held report, or the report loses its bytes.
fn check_held_recipient(
    evidence: &ConflictEvidence,
    exact_size: usize,
) -> Result<(), Box<dyn Error>> {
    let view: DisclosureView<'_> = evidence
        .disclosure_report()
        .map_err(|error| format!("{error:?}"))?
        .ok_or("missing report before contention")?;
    thread::scope(move |scope| -> Result<(), Box<dyn Error>> {
        type Disposition = Result<(), DisclosureError>;
        let (finished, completion): (Sender<Disposition>, Receiver<Disposition>) = mpsc::channel();
        let contender: ScopedJoinHandle<'_, ()> = scope.spawn(move || {
            assert!(finished.send(evidence.clear_disclosure_report()).is_ok());
        });
        let outcome: Result<Result<(), DisclosureError>, RecvTimeoutError> =
            completion.recv_timeout(WATCHDOG);
        assert_eq!(view.bytes().map(<[u8]>::len), Some(exact_size));
        // Release the view before joining, even if a faulty contender blocked on it.
        drop(view);
        contender
            .join()
            .map_err(|_| "disclosure contender panicked")?;
        assert!(matches!(outcome?, Err(DisclosureError::Unavailable)));
        Ok(())
    })?;
    evidence
        .clear_disclosure_report()
        .map_err(|error| format!("{error:?}"))?;
    assert!(
        evidence
            .disclosure_report()
            .map_err(|error| format!("{error:?}"))?
            .is_none()
    );
    Ok(())
}

/// Refuse the output reservation before publication while retaining the original conflict.
///
/// # Errors
/// Returns an unexpected confirmation or recipient observation error.
///
/// # Panics
/// Panics if the request exceeds its bound, allocation refusal is lost, or a
/// report or changed conflict is visible after the refusal.
fn check_allocation_refusal(
    evidence: &ConflictEvidence,
    exact_size: usize,
) -> Result<(), Box<dyn Error>> {
    let action: DisclosureAction<'_> = evidence
        .confirm_disclosure(MAX_LIFETIME)
        .map_err(|error| format!("{error:?}"))?;
    let mut requested: Option<usize> = None;
    let result: Result<(), DisclosureError> = action.submit_using(
        || Some(Instant::now()),
        || {},
        LIMIT,
        |output: &mut Vec<u8>, required: usize| {
            assert!(output.is_empty());
            requested = Some(required);
            // A Vec<u8> capacity above isize::MAX is refused before allocation.
            output.try_reserve_exact(usize::MAX)
        },
    );
    assert!(matches!(result, Err(DisclosureError::Allocation)));
    assert_eq!(requested, Some(exact_size));
    assert!((1..=LIMIT).contains(&exact_size));
    assert!(
        evidence
            .disclosure_report()
            .map_err(|error| format!("{error:?}"))?
            .is_none()
    );
    assert_eq!(evidence.conflict().positions(), (0, 2));
    Ok(())
}

/// Exercise clock, withdrawal, capacity and incomplete-evidence refusals.
///
/// # Errors
/// Returns a fixture, clock-range or unexpected disclosure refusal.
///
/// # Panics
/// Panics when an invalidated action publishes or a valid neighbor fails.
#[test]
fn current_action_is_rechecked_at_the_actual_transfer() -> Result<(), Box<dyn Error>> {
    let mut evidence: ConflictEvidence = signed_recovery_evidence()?;
    let lifetime: Duration = Duration::from_secs(3);
    let mut exact_size: usize = 0;
    for case in 0..4 {
        let action: DisclosureAction<'_> = evidence
            .confirm_disclosure(lifetime)
            .map_err(|e| format!("{e:?}"))?;
        let (issued, cutoff): (Instant, Instant) = {
            let state: MutexGuard<'_, State> =
                evidence.disclosure.lock().map_err(|e| format!("{e:?}"))?;
            let current: &Current = state.current.as_ref().ok_or("missing current action")?;
            (current.issued, current.cutoff)
        };
        let timely: Instant = cutoff
            .checked_sub(Duration::from_nanos(2))
            .ok_or("clock range")?;
        let later: Instant = cutoff
            .checked_sub(Duration::from_nanos(1))
            .ok_or("clock range")?;
        assert!(issued <= timely);
        let mut sequence: std::array::IntoIter<Option<Instant>, 2> = match case {
            0 => [Some(timely), Some(cutoff)].into_iter(),
            1 => [Some(later), Some(timely)].into_iter(),
            2 => [Some(timely), None].into_iter(),
            _ => [Some(timely), Some(later)].into_iter(),
        };
        let result: Result<(), DisclosureError> = action.submit_using(
            || sequence.next().flatten(),
            || {},
            LIMIT,
            Vec::try_reserve_exact,
        );
        match case {
            0 => assert!(matches!(result, Err(DisclosureError::Stale))),
            1 | 2 => assert!(matches!(result, Err(DisclosureError::Clock))),
            _ => assert!(result.is_ok()),
        }
        assert_eq!(sequence.len(), 0);
        let present: bool = evidence
            .disclosure_report()
            .map_err(|e| format!("{e:?}"))?
            .is_some();
        assert_eq!(present, case == 3);
        if case == 3 {
            let view: DisclosureView<'_> = evidence
                .disclosure_report()
                .map_err(|e| format!("{e:?}"))?
                .ok_or("missing report")?;
            exact_size = view.bytes().ok_or("missing report bytes")?.len();
        }
        evidence
            .clear_disclosure_report()
            .map_err(|e| format!("{e:?}"))?;
    }
    let action: DisclosureAction<'_> = evidence
        .confirm_disclosure(lifetime)
        .map_err(|e| format!("{e:?}"))?;
    let result: Result<(), DisclosureError> = action.submit_using(
        || Some(Instant::now()),
        || {
            assert!(evidence.dismiss_disclosure().is_ok());
        },
        LIMIT,
        Vec::try_reserve_exact,
    );
    assert!(matches!(result, Err(DisclosureError::Stale)));
    assert!(
        evidence
            .disclosure_report()
            .map_err(|e| format!("{e:?}"))?
            .is_none()
    );
    check_replaced_reservation(&evidence)?;
    let too_small: DisclosureAction<'_> = evidence
        .confirm_disclosure(lifetime)
        .map_err(|e| format!("{e:?}"))?;
    assert!(matches!(
        too_small.submit_using(
            || Some(Instant::now()),
            || {},
            exact_size - 1,
            Vec::try_reserve_exact,
        ),
        Err(DisclosureError::Size)
    ));
    assert!(
        evidence
            .disclosure_report()
            .map_err(|e| format!("{e:?}"))?
            .is_none()
    );
    check_allocation_refusal(&evidence, exact_size)?;
    let exact: DisclosureAction<'_> = evidence
        .confirm_disclosure(lifetime)
        .map_err(|e| format!("{e:?}"))?;
    exact
        .submit_using(
            || Some(Instant::now()),
            || {},
            exact_size,
            Vec::try_reserve_exact,
        )
        .map_err(|e| format!("{e:?}"))?;
    let size: Option<usize> = evidence
        .disclosure_report()
        .map_err(|error| format!("{error:?}"))?
        .as_ref()
        .and_then(DisclosureView::bytes)
        .map(<[u8]>::len);
    assert_eq!(size, Some(exact_size));
    evidence
        .clear_disclosure_report()
        .map_err(|e| format!("{e:?}"))?;
    let maximum: DisclosureAction<'_> = evidence
        .confirm_disclosure(MAX_LIFETIME)
        .map_err(|e| format!("{e:?}"))?;
    for invalid in [Duration::ZERO, Duration::new(30, 1)] {
        assert!(matches!(
            evidence.confirm_disclosure(invalid),
            Err(DisclosureError::Configuration)
        ));
    }
    maximum.submit().map_err(|e| format!("{e:?}"))?;
    check_held_recipient(&evidence, exact_size)?;
    // This mutation is possible only in this descendant test module.
    let _removed: Receipt = evidence.receipts.pop().ok_or("missing last receipt")?;
    let incomplete: DisclosureAction<'_> = evidence
        .confirm_disclosure(lifetime)
        .map_err(|e| format!("{e:?}"))?;
    assert!(matches!(
        incomplete.submit(),
        Err(DisclosureError::Proof(ConflictProofError::Incomplete))
    ));
    assert!(
        evidence
            .disclosure_report()
            .map_err(|e| format!("{e:?}"))?
            .is_none()
    );
    assert_eq!(evidence.conflict().positions(), (0, 2));
    Ok(())
}

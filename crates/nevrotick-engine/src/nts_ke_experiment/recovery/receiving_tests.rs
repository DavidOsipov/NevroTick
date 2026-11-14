// SPDX-License-Identifier: GPL-2.0-only
//! Genuine admitted receiving owners challenge the consuming history boundary.

use std::cell::Cell;
use std::io;
use std::sync::Arc;

use zeroize::Zeroizing;

use super::{ActiveReceiver, Admission, Control, End, Failure, Invocation, Producer};
use crate::nts_ke_experiment::association::ledger::{
    LedgerError, Receipt, RequestInput, SendOutcome,
};
use crate::nts_ke_experiment::association::{AssociationCandidate, AssociationOwner};
use crate::nts_ke_experiment::datagram::tests::recovery_tests::{ExchangeCase, with_fixture};
use crate::nts_ke_experiment::process::ProviderScope;
use crate::nts_ke_experiment::profile::StrictProfile;
use crate::nts_ke_experiment::request::DirectionalKeys;
use crate::nts_ke_experiment::{
    CancelOutcome, Config, ExchangeMode, InvocationCompletion, IoPhase, StopDisposition,
};

/// Obtain the same charged Control/context reservation used by the actual driver.
/// # Errors
/// Preserve admission failure or an unexpected finished fixture before any exchange.
fn admitted(invocation: &mut Invocation<'_>) -> Result<(Control, u8), Failure> {
    match invocation.admit().map_err(|failure| failure.primary)? {
        Admission::Ready { control, contexts } => Ok((control, contexts)),
        Admission::Finished(_) => Err(Failure::Control),
    }
}

/// Publish a genuinely separate checked owner with distinguishable initial counts.
/// These public fixed keys are never used to assert TLS or numerical authority.
/// # Errors
/// Preserve the actual guarded candidate publication refusal.
fn foreign_association<'scope>(
    config: &'scope Config,
    control: &'scope Control,
    provider: &'scope ProviderScope<'scope>,
) -> Result<AssociationOwner<'scope>, Failure> {
    let candidate = AssociationCandidate::new(
        config,
        control,
        provider,
        std::net::SocketAddr::new(config.peer.ip(), 123),
        StrictProfile::component(),
        DirectionalKeys::new(Zeroizing::new([0x91; 32]), Zeroizing::new([0x92; 32])),
        vec![Zeroizing::new(vec![0x33; 8]), Zeroizing::new(vec![0x44; 8])],
    );
    AssociationOwner::receive(config, control, provider, candidate)
}

/// Burn one actual checked cookie through a definitive failed synchronous send.
/// A distinct retired UID leaves the maintained healthy reply's correlation fresh.
/// # Errors
/// Preserve checked reservation/preparation failures before the expected send refusal.
/// # Panics
/// Fail if the actual send outcome is hidden or any charged attempt is refunded.
fn burn_first_cookie(association: &mut AssociationOwner<'_>) -> Result<(), Failure> {
    let before = association.status();
    let ticket = association.reserve(RequestInput::Observed)?;
    association.prepare_observed(&ticket, |output| {
        output[..8].copy_from_slice(&1_u64.to_be_bytes());
        output[8..40].fill(0xee);
        output[40..].fill(0xaa);
        Ok(())
    })?;
    assert!(matches!(
        association.invoke_sender(ticket, |_, _| SendOutcome::Failed(
            io::ErrorKind::BrokenPipe
        )),
        Err(Failure::Ledger(LedgerError::Send(
            io::ErrorKind::BrokenPipe
        )))
    ));
    assert_eq!(
        association.status().attempts.checked_add(1),
        Some(before.attempts)
    );
    assert_eq!(association.status().used.checked_sub(1), Some(before.used));
    Ok(())
}

/// Equal origins/configuration cannot pair a committed fact with a foreign receiver.
/// Both the direct projection and actual acceptance entry enforce the same binding.
/// # Errors
/// Preserve genuine admission, protected exchange and independently checked owners.
/// # Panics
/// Fail if rejection alters either owner, substitutes its counts/chart or refunds work.
#[test]
fn consuming_history_rejects_foreign_receiver_context_and_population() -> Result<(), Failure> {
    for through_accept in [false, true] {
        with_fixture(
            ExchangeMode::ProtectedDatagram,
            None,
            vec![Ok(())],
            ExchangeCase::Healthy,
            |invocation, provider, producer| {
                let (control, contexts) = admitted(invocation)?;
                let config = invocation.config;
                let mut association = producer
                    .establish(config, &control, provider)
                    .map_err(|failure| failure.primary)?;
                invocation.contexts = contexts;
                let completion = producer.exchange(&mut association, &control)?;
                let committed = completion.commit(&mut association)?;
                let original = invocation.snapshot();
                let original_status = association.status();
                let original_receipt = association.last_receipt().ok_or(Failure::Control)?;
                let mut foreign =
                    Invocation::one_shot(config, provider, Control::new(original.origin)?)?;
                let (foreign_control, foreign_contexts) = admitted(&mut foreign)?;
                let foreign_owner = foreign_association(config, &foreign_control, provider)?;
                foreign.contexts = foreign_contexts;
                assert!(!Arc::ptr_eq(
                    invocation.run_context(),
                    foreign.run_context()
                ));
                assert_eq!(foreign.origin, original.origin);
                assert_eq!(foreign_owner.cookie_population()?, (2, 16));
                assert_eq!(association.cookie_population()?, (1, 12));
                let before = foreign.snapshot();
                let foreign_status = foreign_owner.status();
                let mut receiving = ActiveReceiver {
                    invocation: &mut foreign,
                    control: &foreign_control,
                };
                let result = {
                    let binding = receiving.bind_association(&foreign_owner)?;
                    if through_accept {
                        binding.accept(committed).map(|_| ())
                    } else {
                        committed.into_history(&binding).map(|_| ())
                    }
                };
                assert!(matches!(result, Err(Failure::Control)));
                assert_eq!(receiving.invocation.snapshot(), before);
                assert_eq!(foreign_owner.status(), foreign_status);
                assert!(foreign_owner.last_receipt().is_none());
                foreign_control.remaining(IoPhase::Exchange)?;
                assert_eq!(invocation.snapshot(), original);
                assert_eq!(association.status(), original_status);
                assert_eq!(
                    association.last_receipt().map(Receipt::serial),
                    Some(original_receipt.serial())
                );
                assert_eq!(control.cancel()?, CancelOutcome::AlreadyCommitted);
                Ok(())
            },
        )?;
    }
    Ok(())
}

/// A completed decision remains history after local cancellation or invocation Stop.
/// # Errors
/// Preserve genuine admission, protected exchange and the actual historical projection.
/// # Panics
/// Fail if late stop discards/rebinds history, changes original counts or renews time.
#[test]
fn consuming_history_preserves_committed_facts_after_late_cancel_and_stop() -> Result<(), Failure> {
    for stop_scope in [false, true] {
        with_fixture(
            ExchangeMode::ProtectedDatagram,
            None,
            vec![Ok(())],
            ExchangeCase::MultipleCookies,
            |invocation, provider, producer| {
                let stop = invocation.stop_handle();
                let (control, contexts) = admitted(invocation)?;
                let config = invocation.config;
                let original_chart = Arc::downgrade(invocation.run_context());
                let mut association = producer
                    .establish(config, &control, provider)
                    .map_err(|failure| failure.primary)?;
                invocation.contexts = contexts;
                let original = invocation.snapshot();
                let mut receiving = ActiveReceiver {
                    invocation,
                    control: &control,
                };
                let binding = receiving.bind_association(&association)?;
                let completion = producer.exchange(&mut association, &control)?;
                let committed = completion.commit(&mut association)?;
                let receipt = association.last_receipt().ok_or(Failure::Control)?;
                let status = association.status();
                if stop_scope {
                    assert_eq!(stop.request_stop()?, StopDisposition::Latched);
                }
                assert_eq!(control.cancel()?, CancelOutcome::AlreadyCommitted);
                let InvocationCompletion::Protected(history) = binding.accept(committed)? else {
                    return Err(Failure::Control);
                };
                assert_eq!(history.summary().cookies, 1);
                assert_eq!(history.summary().cookie_bytes, 12);
                assert_eq!(history.population(), (2, 24));
                assert_eq!(
                    history.receipt().fields().header(),
                    receipt.fields().header()
                );
                assert_eq!(history.receipt().serial(), receipt.serial());
                assert_eq!(history.receipt().accepted_at(), receipt.accepted_at());
                assert!(std::sync::Weak::ptr_eq(
                    &original_chart,
                    &Arc::downgrade(history.run_context())
                ));
                assert_eq!(association.status(), status);
                let after = receiving.invocation.snapshot();
                assert_eq!(
                    (after.origin, after.cutoff),
                    (original.origin, original.cutoff)
                );
                assert_eq!(after.end, Some(End::Completed));
                assert_eq!(after.transactions, original.transactions);
                assert_eq!(after.observations, original.observations);
                Ok(())
            },
        )?;
    }
    Ok(())
}

/// The actual history summary keeps pre-operation counts after a genuine cookie burn.
/// Original KE count two, pre-operation count one and post-reply count one are distinct
/// facts; the paired multiple-cookie control distinguishes pre-operation from post-reply.
/// # Errors
/// Preserve actual admitted owners, failed-send burn and protected history refusal.
/// # Panics
/// Fail if creation metadata replaces the existing pre-operation summary contract.
#[test]
fn bound_history_preserves_preoperation_population_after_prior_burn_and_refill()
-> Result<(), Failure> {
    with_fixture(
        ExchangeMode::ProtectedDatagram,
        None,
        vec![Ok(())],
        ExchangeCase::Healthy,
        |invocation, provider, producer| {
            producer.supply_two_initial_cookies();
            let (control, contexts) = admitted(invocation)?;
            let config = invocation.config;
            let mut association = producer
                .establish(config, &control, provider)
                .map_err(|failure| failure.primary)?;
            invocation.contexts = contexts;
            assert_eq!(association.cookie_population()?, (2, 24));
            burn_first_cookie(&mut association)?;
            assert_eq!(association.cookie_population()?, (1, 12));
            assert_eq!(
                (association.status().used, association.status().attempts),
                (1, 31)
            );
            let mut receiving = ActiveReceiver {
                invocation,
                control: &control,
            };
            let completion = receiving
                .receive_protected(&mut association, |owner| producer.exchange(owner, &control))?;
            let InvocationCompletion::Protected(history) = completion else {
                return Err(Failure::Control);
            };
            assert_eq!(
                (history.summary().cookies, history.summary().cookie_bytes),
                (1, 12)
            );
            assert_eq!(history.population(), (1, 12));
            assert_eq!(association.cookie_population()?, (1, 12));
            assert_eq!(
                (association.status().used, association.status().attempts),
                (2, 30)
            );
            assert_eq!(history.receipt().serial(), 2);
            assert_eq!(receiving.invocation.snapshot().end, Some(End::Completed));
            assert_eq!(control.cancel()?, CancelOutcome::AlreadyCommitted);
            Ok(())
        },
    )
}

/// An exhausted pool remains a valid bounded snapshot; the real producer owns refusal.
/// # Errors
/// Preserve admitted setup and actual failed-send burn before the empty-pool neighbor.
/// # Panics
/// Fail if binding invents a nonempty rule, bypasses the producer or refunds its charge.
#[test]
fn bound_history_empty_population_preserves_actual_producer_refusal() -> Result<(), Failure> {
    with_fixture(
        ExchangeMode::ProtectedDatagram,
        None,
        vec![Ok(())],
        ExchangeCase::Healthy,
        |invocation, provider, producer| {
            let (control, contexts) = admitted(invocation)?;
            let config = invocation.config;
            let mut association = producer
                .establish(config, &control, provider)
                .map_err(|failure| failure.primary)?;
            invocation.contexts = contexts;
            burn_first_cookie(&mut association)?;
            assert_eq!(association.cookie_population()?, (0, 0));
            let before = invocation.snapshot();
            let entered = Cell::new(false);
            let mut receiving = ActiveReceiver {
                invocation,
                control: &control,
            };
            let result = receiving.receive_protected(&mut association, |owner| {
                entered.set(true);
                producer.exchange(owner, &control)
            });
            assert!(entered.get());
            assert!(matches!(result, Err(Failure::Ledger(LedgerError::Cookies))));
            assert_eq!(receiving.invocation.snapshot(), before);
            assert_eq!(association.cookie_population()?, (0, 0));
            assert_eq!(
                (association.status().used, association.status().attempts),
                (1, 30)
            );
            assert!(association.last_receipt().is_none());
            control.remaining(IoPhase::Exchange)?;
            Ok(())
        },
    )
}

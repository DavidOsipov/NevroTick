// SPDX-License-Identifier: GPL-2.0-only
//! Real checked ledger transitions with public component material.
//!
//! Existing independent tagged packets isolate history and receiver semantics.
//! Fixed keys/entropy and synchronous callbacks establish no TLS, RNG, native
//! completion or production-context freshness.

use std::cell::Cell;
use std::ffi::OsString;
use std::io;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use zeroize::Zeroizing;

use super::{PastRequests, check_history_bounds};
use crate::nts_ke_experiment::association::ledger::{
    ATTEMPTS, FRAMES, LedgerError, RECEIVE_BYTES, Receipt, RecoveryReason, RequestInput,
    RequestPhase, SendOutcome,
};
use crate::nts_ke_experiment::association::{AssociationCandidate, AssociationOwner};
use crate::nts_ke_experiment::process::{ProcessAdmission, ProviderScope};
use crate::nts_ke_experiment::profile::StrictProfile;
use crate::nts_ke_experiment::request::DirectionalKeys;
use crate::nts_ke_experiment::response::{ReplyError, fixtures};
use crate::nts_ke_experiment::{COOKIE_BYTES_LIMIT, CancelOutcome, Config, Control, Failure};

/// Public independently tagged fixtures use this exact UID and outgoing key.
const UID: [u8; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31,
];
/// The independent `WRONG_UID` packet is a valid neighbor for this different UID.
const NEXT_UID: [u8; 32] = [
    1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31,
];
/// Independent packet fixtures' public server-to-client direction.
const S2C: [u8; 32] = [
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f,
    0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f,
];
/// Exact public Cookie body carried by `HEALTHY` and `WRONG_UID`.
const REFILL: [u8; 12] = [
    0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab,
];
/// Fixed wire token, distinct from all actual local monotonic observations.
const WIRE: u64 = 0x1122_3344_5566_7788;
/// Distinct declared successor exporters and UID match the existing tagged fixtures.
const SUCCESSOR_C2S: [u8; 32] = [
    0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f,
    0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f,
];
const SUCCESSOR_S2C: [u8; 32] = [
    0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf,
    0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf,
];
const SUCCESSOR_UID: [u8; 32] = [
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f,
];
const SUCCESSOR_WIRE: u64 = 0x8899_aabb_ccdd_eeff;

/// Construct the protected mode only through the existing admitted argument API.
/// # Errors
/// Preserve unexpected refusal of the fixed valid component arguments.
fn config() -> Result<Config, Failure> {
    Config::from_arguments(
        ["127.0.0.1:4460", "Time.Example", "--protected-datagram"]
            .map(OsString::from)
            .into_iter(),
    )
}

/// Hold one real checked configuration/claim/component-provider for the exercise.
/// # Errors
/// Preserve actual construction, publication and exercised boundary failures.
fn with_provider(
    exercise: impl FnOnce(&Config, &ProviderScope<'_>) -> Result<(), Failure>,
) -> Result<(), Failure> {
    let config = config()?;
    let admission = ProcessAdmission::new();
    let claim = admission.claim()?;
    let provider = ProviderScope::component(claim);
    exercise(&config, &provider)
}

/// Publish complete component material through the actual checked owner.
/// # Errors
/// Preserve binding, cookie-budget and Control publication refusal.
fn owner<'a>(
    config: &'a Config,
    control: &'a Control,
    provider: &'a ProviderScope<'a>,
    cookies: Vec<Zeroizing<Vec<u8>>>,
) -> Result<AssociationOwner<'a>, Failure> {
    let candidate = AssociationCandidate::new(
        config,
        control,
        provider,
        SocketAddr::new(config.peer.ip(), 123),
        StrictProfile::component(),
        DirectionalKeys::new(Zeroizing::new(UID), Zeroizing::new(S2C)),
        cookies,
    );
    AssociationOwner::receive(config, control, provider, candidate)
}

/// Reserve, prepare and complete the real synchronous adapter transition.
/// # Errors
/// Preserve original admission, entropy, encoding and submission refusal.
fn pending(owner: &mut AssociationOwner<'_>, uid: &[u8; 32]) -> Result<(), Failure> {
    let ticket = owner.reserve(RequestInput::Observed)?;
    owner.prepare_observed(&ticket, |output| {
        output[..8].copy_from_slice(&WIRE.to_be_bytes());
        output[8..40].copy_from_slice(uid);
        output[40..].fill(0x66);
        Ok(())
    })?;
    owner.invoke_sender(ticket, |packet, _| SendOutcome::Completed(packet.len()))
}

/// Issue at most thirty-one distinct public UIDs through unused checked requests.
/// Returning each real ticket preserves its cookie but refunds no checkout or UID.
/// # Errors
/// Refuse excessive count before work; preserve actual admission or partial progress.
fn prepare_and_return(owner: &mut AssociationOwner<'_>, count: u8) -> Result<(), Failure> {
    if usize::from(count) >= ATTEMPTS {
        return Err(Failure::Limits);
    }
    for index in 0..count {
        let ticket = owner.reserve(RequestInput::Observed)?;
        owner.prepare_observed(&ticket, |output| {
            output[..8].copy_from_slice(&WIRE.to_be_bytes());
            output[8..40].fill(index);
            output[40..].fill(0x66);
            Ok(())
        })?;
        owner.return_unused(ticket)?;
    }
    Ok(())
}

/// Challenge both nonlocking recovery helpers before genuine local exhaustion.
/// This observer invokes only those actual helpers and never constructs an intent.
/// # Panics
/// Fail if a fresh or occupied healthy owner gains an intent or changes custody.
fn assert_premature_helpers_are_inert(owner: &mut AssociationOwner<'_>) {
    let before = owner.status();
    let expiry = owner.material.control.expiry;
    assert!(before.frames > 0 && before.bytes > 0);
    assert!(owner.ledger.recovery.is_none());
    assert!(!owner.ledger.recovery_issued);
    owner.recover_checkout();
    assert_eq!(owner.status(), before, "premature-checkout-recovery-state");
    assert!(
        owner.ledger.recovery.is_none(),
        "premature-checkout-recovery-intent"
    );
    assert!(!owner.ledger.recovery_issued);
    owner.retire_exhausted();
    assert_eq!(owner.status(), before, "premature-receive-retirement-state");
    assert!(
        owner.ledger.recovery.is_none(),
        "premature-receive-retirement-intent"
    );
    assert!(!owner.ledger.recovery_issued);
    assert_eq!(owner.material.control.expiry, expiry);
}

/// Premature helper calls cannot preempt the later genuine receive-work reason.
/// # Errors
/// Preserve checked owner, request and actual authenticated rejection setup.
/// # Panics
/// Fail if early calls stick a wrong reason, retire Pending or replenish work.
fn assert_premature_helpers_preserve_exhaustion(
    config: &Config,
    provider: &ProviderScope<'_>,
) -> Result<(), Failure> {
    let control = Control::new(Instant::now())?;
    let mut original = owner(
        config,
        &control,
        provider,
        vec![Zeroizing::new(vec![0x11; 12])],
    )?;
    assert_premature_helpers_are_inert(&mut original);
    pending(&mut original, &UID)?;
    assert_eq!(original.status().phase, RequestPhase::Pending);
    assert_premature_helpers_are_inert(&mut original);
    for remaining in (0..FRAMES).rev() {
        assert_eq!(
            original
                .receive_reply(
                    fixtures::ZERO_COOKIES,
                    original.material.udp,
                    Instant::now()
                )
                .err(),
            Some(Failure::Ledger(LedgerError::Reply(ReplyError::Cookies))),
        );
        assert_eq!(original.status().frames, remaining);
    }
    assert_eq!(original.status().phase, RequestPhase::Idle);
    assert!(original.last_receipt().is_none());
    assert_eq!(
        original
            .ledger
            .recovery
            .as_ref()
            .map(|intent| (intent.reason, intent.not_before)),
        Some((RecoveryReason::ReceiveWork, control.expiry)),
        "premature-helpers-preserve-actual-receive-reason",
    );
    assert!(original.ledger.recovery_issued);
    Ok(())
}

/// A healthy final reply reaches both real budget boundaries without issuing intent.
/// # Errors
/// Preserve actual pending submission, rejected replies and the final healthy receipt.
/// # Panics
/// Fail if the final allowance cannot commit or rejection refreshes expiry/work.
fn healthy_final_allowance(
    owner: &mut AssociationOwner<'_>,
    control: &Control,
) -> Result<(), Failure> {
    pending(owner, &UID)?;
    for remaining in (1..FRAMES).rev() {
        assert_eq!(
            owner
                .receive_reply(fixtures::ZERO_COOKIES, owner.material.udp, Instant::now())
                .err(),
            Some(Failure::Ledger(LedgerError::Reply(ReplyError::Cookies))),
        );
        assert_eq!(owner.status().frames, remaining);
        assert!(owner.ledger.recovery.is_none());
    }
    owner.receive_reply(fixtures::HEALTHY, owner.material.udp, Instant::now())?;
    assert_eq!(owner.status().frames, 0);
    assert_eq!(owner.status().bytes, 127_088);
    assert_eq!(owner.status().phase, RequestPhase::Idle);
    assert!(owner.last_receipt().is_some());
    assert!(owner.ledger.recovery.is_none());
    assert!(!owner.ledger.recovery_issued);
    assert_eq!(owner.material.control.expiry, control.expiry);
    Ok(())
}

/// Compare the actual precharge priority at thirty versus thirty-one unused tickets.
/// # Errors
/// Preserve checked setup, real distinct-UID preparation and healthy final replies.
/// # Panics
/// Fail if the final reserve chooses a postcharge reason or refunds an allowance.
fn assert_checkout_priority_at_real_boundaries(
    config: &Config,
    provider: &ProviderScope<'_>,
) -> Result<(), Failure> {
    for count in [30, 31] {
        let control = Control::new(Instant::now())?;
        let mut original = owner(
            config,
            &control,
            provider,
            vec![Zeroizing::new(vec![0x11; 12])],
        )?;
        prepare_and_return(&mut original, count)?;
        let prepared = original.status();
        assert_eq!(
            (prepared.attempts, prepared.uids, prepared.used),
            (32 - usize::from(count), usize::from(count), 0)
        );
        assert_eq!(
            (prepared.frames, prepared.bytes, prepared.available),
            (32, RECEIVE_BYTES, 1)
        );
        healthy_final_allowance(&mut original, &control)?;
        let before = original.status();
        assert_eq!(
            (before.attempts, before.uids, before.used),
            (31 - usize::from(count), usize::from(count) + 1, 1)
        );
        let (error, reason) = if count == 30 {
            (LedgerError::ReceiveWork, RecoveryReason::ReceiveWork)
        } else {
            (LedgerError::Attempts, RecoveryReason::Attempts)
        };
        assert_eq!(
            original.reserve(RequestInput::Observed).err(),
            Some(Failure::Ledger(error)),
            "checkout-priority-before-final-charge"
        );
        let after = original.status();
        assert_eq!(
            (after.attempts, after.frames, after.uids),
            (0, 0, before.uids)
        );
        assert_eq!(
            original
                .ledger
                .recovery
                .as_ref()
                .map(|intent| (intent.reason, intent.not_before)),
            Some((reason, control.expiry)),
            "checkout-priority-original-reason",
        );
        assert_eq!(
            original.reserve(RequestInput::Observed).err(),
            Some(Failure::Ledger(LedgerError::Attempts))
        );
        assert_eq!(original.status(), after);
        assert_eq!(
            original
                .ledger
                .recovery
                .as_ref()
                .map(|intent| intent.reason),
            Some(reason)
        );
    }
    Ok(())
}

/// A failed real send archives its cookie before the last checkout selects Cookies.
/// # Errors
/// Preserve actual distinct-UID preparation and checked ticket transitions.
/// # Panics
/// Fail if postcharge Attempts overwrites the original empty-pool disposition.
fn assert_empty_pool_priority_survives_last_attempt(
    config: &Config,
    provider: &ProviderScope<'_>,
) -> Result<(), Failure> {
    let control = Control::new(Instant::now())?;
    let mut original = owner(
        config,
        &control,
        provider,
        vec![Zeroizing::new(vec![0x11; 12])],
    )?;
    prepare_and_return(&mut original, 30)?;
    let ticket = original.reserve(RequestInput::Observed)?;
    original.prepare_observed(&ticket, |output| {
        output[..8].copy_from_slice(&WIRE.to_be_bytes());
        output[8..40].copy_from_slice(&UID);
        output[40..].fill(0x66);
        Ok(())
    })?;
    let sends = Cell::new(0);
    assert_eq!(
        original.invoke_sender(ticket, |_, _| {
            sends.set(sends.get() + 1);
            SendOutcome::Failed(io::ErrorKind::BrokenPipe)
        }),
        Err(Failure::Ledger(LedgerError::Send(
            io::ErrorKind::BrokenPipe
        ))),
    );
    assert_eq!(sends.get(), 1);
    let sent = original.status();
    assert_eq!(
        (sent.attempts, sent.frames, sent.bytes),
        (1, 32, RECEIVE_BYTES)
    );
    assert_eq!(
        (sent.available, sent.used, sent.uids, sent.phase),
        (0, 1, 31, RequestPhase::Idle)
    );
    assert!(original.ledger.recovery.is_none());
    assert!(!original.ledger.recovery_issued);
    assert_eq!(
        original.reserve(RequestInput::Observed).err(),
        Some(Failure::Ledger(LedgerError::Cookies)),
        "empty-pool-precharge-priority"
    );
    let exhausted = original.status();
    assert_eq!(
        (exhausted.attempts, exhausted.frames, exhausted.uids),
        (0, 32, 31)
    );
    assert_eq!(
        original
            .ledger
            .recovery
            .as_ref()
            .map(|intent| (intent.reason, intent.not_before)),
        Some((RecoveryReason::Cookies, control.expiry)),
    );
    assert_eq!(
        original.reserve(RequestInput::Observed).err(),
        Some(Failure::Ledger(LedgerError::Attempts))
    );
    assert_eq!(original.status(), exhausted);
    assert_eq!(
        original
            .ledger
            .recovery
            .as_ref()
            .map(|intent| intent.reason),
        Some(RecoveryReason::Cookies),
        "first-cookie-intent-remains-sticky"
    );
    Ok(())
}

/// Charge the complete original allowance with actual authenticated count refusals.
/// # Errors
/// Preserve unexpected setup failure; assertions retain the exact response cause.
/// # Panics
/// Fail if a zero-cookie response succeeds, changes cause or renews the deadline.
fn exhaust(owner: &mut AssociationOwner<'_>, control: &Control) -> Result<(), Failure> {
    pending(owner, &UID)?;
    let expiry = control.expiry;
    for remaining in (0..FRAMES).rev() {
        assert_eq!(
            owner
                .receive_reply(fixtures::ZERO_COOKIES, owner.material.udp, Instant::now())
                .err(),
            Some(Failure::Ledger(LedgerError::Reply(ReplyError::Cookies))),
        );
        assert_eq!(owner.status().frames, remaining);
        assert_eq!(control.expiry, expiry);
    }
    assert_eq!(owner.status().phase, RequestPhase::Idle);
    assert!(owner.last_receipt().is_none());
    Ok(())
}

/// Receive a genuine owned history projection, then close its old admission once.
/// # Errors
/// Preserve checked transitions and actual cancellation failure.
/// # Panics
/// Fail if retiring the original context changed its not-before or closed it early.
fn history<'scope>(
    config: &'scope Config,
    control: &Control,
    provider: &'scope ProviderScope<'scope>,
    cookie: Vec<u8>,
) -> Result<PastRequests<'scope>, Failure> {
    let mut original = owner(config, control, provider, vec![Zeroizing::new(cookie)])?;
    exhaust(&mut original, control)?;
    let retired = original.retire_for_recovery(config, control, provider)?;
    let (not_before, history) = retired.receive(config, control, provider)?;
    assert_eq!(not_before, control.expiry);
    assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
    Ok(history)
}

/// Publish distinct component material through the same checked receiving owner.
/// This fixture supplies no real TLS exporter or entropy provenance.
/// # Errors
/// Preserve actual candidate binding, publication and cookie-capacity refusals.
fn successor_owner<'a>(
    config: &'a Config,
    control: &'a Control,
    provider: &'a ProviderScope<'a>,
) -> Result<AssociationOwner<'a>, Failure> {
    let candidate = AssociationCandidate::new(
        config,
        control,
        provider,
        SocketAddr::new(config.peer.ip(), 123),
        StrictProfile::component(),
        DirectionalKeys::new(Zeroizing::new(SUCCESSOR_C2S), Zeroizing::new(SUCCESSOR_S2C)),
        vec![Zeroizing::new(vec![0x22; 12])],
    );
    AssociationOwner::receive(config, control, provider, candidate)
}

/// Spend the successor's real local request and receive allowances once.
/// All packets use the existing independently tagged successor fixture.
/// # Errors
/// Preserve actual reserve, entropy, preparation and synchronous submission refusal.
/// # Panics
/// Fail if a count refusal renews expiry, refunds a frame or produces a receipt.
fn exhaust_successor(owner: &mut AssociationOwner<'_>, control: &Control) -> Result<(), Failure> {
    let ticket = owner.reserve(RequestInput::Observed)?;
    owner.prepare_observed(&ticket, |output| {
        output[..8].copy_from_slice(&SUCCESSOR_WIRE.to_be_bytes());
        output[8..40].copy_from_slice(&SUCCESSOR_UID);
        output[40..].fill(0x99);
        Ok(())
    })?;
    owner.invoke_sender(ticket, |packet, _| SendOutcome::Completed(packet.len()))?;
    let expiry = control.expiry;
    for remaining in (0..FRAMES).rev() {
        assert_eq!(
            owner
                .receive_reply(
                    fixtures::SUCCESSOR_ZERO_COOKIES,
                    owner.material.udp,
                    Instant::now()
                )
                .err(),
            Some(Failure::Ledger(LedgerError::Reply(ReplyError::Cookies))),
        );
        assert_eq!(owner.status().frames, remaining);
        assert_eq!(control.expiry, expiry);
    }
    assert!(owner.last_receipt().is_none());
    Ok(())
}

/// A real second exhausted context cannot manufacture a third predecessor chain.
/// # Errors
/// Preserve genuine history projection, checked attachment and actual exhaustion.
/// # Panics
/// Fail if the inherited archive is ignored when all other retirement facts hold.
fn assert_predecessor_retirement_refusal(
    config: &Config,
    provider: &ProviderScope<'_>,
) -> Result<(), Failure> {
    let first_control = Control::new(Instant::now())?;
    let past = history(config, &first_control, provider, vec![0x11; 12])?;
    let control = Control::new(Instant::now())?;
    let replacement = successor_owner(config, &control, provider)?;
    let mut replacement = replacement.install_previous(config, &control, provider, past)?;
    exhaust_successor(&mut replacement, &control)?;
    let status = replacement.status();
    assert_eq!(
        (status.attempts, status.frames, status.used, status.uids),
        (31, 0, 1, 1)
    );
    assert_eq!((status.phase, status.available), (RequestPhase::Idle, 0));
    assert!(replacement.ledger.previous.is_some());
    assert!(replacement.ledger.exhausted_response.is_some());
    assert!(replacement.ledger.recovery.as_ref().is_some_and(|intent| {
        intent.reason == RecoveryReason::ReceiveWork && intent.not_before == control.expiry
    }));
    assert_eq!(
        replacement
            .retire_for_recovery(config, &control, provider)
            .err(),
        Some(Failure::Ledger(LedgerError::Phase)),
        "retirement-retire-predecessor",
    );
    Ok(())
}

/// Exercise the real ledger under an independently checked Preview configuration.
/// # Errors
/// Preserve checked Preview construction, publication and authenticated exhaustion.
/// # Panics
/// Fail if a mode-ineligible owner gains retirement despite otherwise genuine facts.
fn assert_preview_retirement_refusal(provider: &ProviderScope<'_>) -> Result<(), Failure> {
    let config = Config::from_arguments(
        ["127.0.0.1:4460", "Time.Example"]
            .map(OsString::from)
            .into_iter(),
    )?;
    let control = Control::new(Instant::now())?;
    let mut original = owner(
        &config,
        &control,
        provider,
        vec![Zeroizing::new(vec![0x11; 12])],
    )?;
    exhaust(&mut original, &control)?;
    assert!(original.ledger.exhausted_response.is_some());
    assert_eq!(
        original
            .retire_for_recovery(&config, &control, provider)
            .err(),
        Some(Failure::Control),
        "retirement-retire-preview-mode",
    );
    Ok(())
}

/// Supply one foreign identity at a time while both actual Controls are Published.
/// # Errors
/// Preserve real claims, checked owners and consumed genuine retirement facts.
/// # Panics
/// Fail if equal expiry or another valid owner substitutes for actual provenance.
fn assert_projection_binding_refusals(
    config: &Config,
    provider: &ProviderScope<'_>,
) -> Result<(), Failure> {
    for case in 0..3 {
        let origin = Instant::now();
        let control = Control::new(origin)?;
        let foreign_control = Control::new(origin)?;
        let foreign_owner = owner(
            config,
            &foreign_control,
            provider,
            vec![Zeroizing::new(vec![0x22; 12])],
        )?;
        assert_eq!(foreign_owner.status().frames, FRAMES);
        assert_eq!(control.expiry, foreign_control.expiry);
        foreign_control.with_published(|_| Ok(()))?;
        let other_config = self::config()?;
        let other_admission = ProcessAdmission::new();
        let other_claim = other_admission.claim()?;
        let other_provider = ProviderScope::component(other_claim);
        let mut original = owner(
            config,
            &control,
            provider,
            vec![Zeroizing::new(vec![0x11; 12])],
        )?;
        exhaust(&mut original, &control)?;
        let retired = original.retire_for_recovery(config, &control, provider)?;
        control.with_published(|_| Ok(()))?;
        let (result, marker) = match case {
            0 => (
                retired.receive(config, &control, &other_provider),
                "retirement-receive-provider-binding",
            ),
            1 => (
                retired.receive(&other_config, &control, provider),
                "retirement-receive-config-binding",
            ),
            _ => (
                retired.receive(config, &foreign_control, provider),
                "retirement-receive-published-control-binding",
            ),
        };
        assert_eq!(result.err(), Some(Failure::Control), "{marker}");
    }
    Ok(())
}

/// Keep self and history provider affinity distinct from the supplied provider argument.
/// # Errors
/// Preserve the second actual claim, genuine history and pristine replacement.
/// # Panics
/// Fail if a provider matching history can override the material's real provider.
fn assert_install_provider_argument_binding(
    config: &Config,
    provider: &ProviderScope<'_>,
) -> Result<(), Failure> {
    let other_admission = ProcessAdmission::new();
    let other_claim = other_admission.claim()?;
    let other_provider = ProviderScope::component(other_claim);
    let first_control = Control::new(Instant::now())?;
    let past = history(config, &first_control, &other_provider, vec![0x11; 12])?;
    let control = Control::new(Instant::now())?;
    let replacement = owner(
        config,
        &control,
        provider,
        vec![Zeroizing::new(vec![0x22; 12])],
    )?;
    assert_eq!(
        replacement
            .install_previous(config, &control, &other_provider, past)
            .err(),
        Some(Failure::Control),
        "retirement-install-provider-binding",
    );
    Ok(())
}

/// Separately publish a foreign Control whose complete expiry equals the actual one.
/// # Errors
/// Preserve genuine history, checked construction and real publication.
/// # Panics
/// Fail if a published equal-valued Control can replace actual receiving custody.
fn assert_install_published_control_binding(
    config: &Config,
    provider: &ProviderScope<'_>,
) -> Result<(), Failure> {
    let first_control = Control::new(Instant::now())?;
    let past = history(config, &first_control, provider, vec![0x11; 12])?;
    let origin = Instant::now();
    let control = Control::new(origin)?;
    let foreign_control = Control::new(origin)?;
    let replacement = owner(
        config,
        &control,
        provider,
        vec![Zeroizing::new(vec![0x22; 12])],
    )?;
    let foreign_owner = owner(
        config,
        &foreign_control,
        provider,
        vec![Zeroizing::new(vec![0x33; 12])],
    )?;
    assert_eq!(foreign_owner.status().frames, FRAMES);
    assert_eq!(control.expiry, foreign_control.expiry);
    foreign_control.with_published(|_| Ok(()))?;
    assert_eq!(
        replacement
            .install_previous(config, &foreign_control, provider, past)
            .err(),
        Some(Failure::Control),
        "retirement-install-published-control-binding",
    );
    Ok(())
}

/// Bind self material independently when the argument and genuine history agree.
/// # Errors
/// Preserve separately checked configuration, history and pristine replacement.
/// # Panics
/// Fail if an argument matching history can override the material's actual source.
fn assert_install_config_argument_binding(
    config: &Config,
    provider: &ProviderScope<'_>,
) -> Result<(), Failure> {
    let other_config = self::config()?;
    let first_control = Control::new(Instant::now())?;
    let past = history(&other_config, &first_control, provider, vec![0x11; 12])?;
    let control = Control::new(Instant::now())?;
    let replacement = owner(
        config,
        &control,
        provider,
        vec![Zeroizing::new(vec![0x22; 12])],
    )?;
    assert_eq!(
        replacement
            .install_previous(&other_config, &control, provider, past)
            .err(),
        Some(Failure::Control),
        "retirement-install-config-binding",
    );
    Ok(())
}

/// Use only a genuine retired archive to block both issued UIDs and the sent cookie.
/// # Errors
/// Preserve actual attachment, explicit return, preparation and sender failures.
/// # Panics
/// Fail if a returned UID is forgotten, old cookie reappears or refusal refunds work.
fn genuine_archive_blocks_returned_and_sent_values<'a>(
    config: &'a Config,
    provider: &'a ProviderScope<'a>,
    past: PastRequests<'a>,
) -> Result<(), Failure> {
    let control = Control::new(Instant::now())?;
    let replacement = owner(
        config,
        &control,
        provider,
        vec![
            Zeroizing::new(vec![0x11; 12]),
            Zeroizing::new(vec![0x22; 12]),
        ],
    )?;
    let mut replacement = replacement.install_previous(config, &control, provider, past)?;
    assert_eq!(replacement.cookie_population()?, (1, 12));
    for (used, attempts) in [(&NEXT_UID, 31), (&UID, 30)] {
        let ticket = replacement.reserve(RequestInput::Observed)?;
        let fills = Cell::new(0);
        assert_eq!(
            replacement
                .prepare_observed(&ticket, |output| {
                    fills.set(fills.get() + 1);
                    output[..8].copy_from_slice(&WIRE.to_be_bytes());
                    output[8..40].copy_from_slice(used);
                    output[40..].fill(0x77);
                    Ok(())
                })
                .err(),
            Some(Failure::Ledger(LedgerError::RepeatedUid))
        );
        assert_eq!(fills.get(), 1);
        replacement.return_unused(ticket)?;
        assert_eq!(replacement.status().attempts, attempts);
        assert_eq!(replacement.status().uids, 0);
        assert_eq!(replacement.cookie_population()?, (1, 12));
    }
    let ticket = replacement.reserve(RequestInput::Observed)?;
    replacement.prepare_observed(&ticket, |output| {
        output[..8].copy_from_slice(&WIRE.to_be_bytes());
        output[8..40].fill(0x55);
        output[40..].fill(0x77);
        Ok(())
    })?;
    replacement.invoke_sender(ticket, |packet, _| {
        assert_eq!(&packet[52..84], &[0x55; 32]);
        assert_eq!(&packet[88..100], &[0x22; 12]);
        SendOutcome::Completed(packet.len())
    })?;
    assert_eq!(replacement.status().attempts, 29);
    assert_eq!(
        (replacement.status().used, replacement.status().uids),
        (1, 1)
    );
    Ok(())
}

/// Consuming real exhaustion moves exact archive storage and original expiry.
/// # Errors
/// Preserve genuine construction, transition and projection refusal.
/// # Panics
/// Fail if bytes are cloned, history is omitted or another marker is minted.
#[test]
fn genuine_retirement_moves_archive_and_preserves_original_not_before() -> Result<(), Failure> {
    with_provider(|config, provider| {
        let control = Control::new(Instant::now())?;
        let mut original = owner(
            config,
            &control,
            provider,
            vec![Zeroizing::new(vec![0x11; 12])],
        )?;
        let address = original.material.cookies[0].as_ptr();
        let returned = original.reserve(RequestInput::Observed)?;
        original.prepare_observed(&returned, |output| {
            output[..8].copy_from_slice(&WIRE.to_be_bytes());
            output[8..40].copy_from_slice(&NEXT_UID);
            output[40..].fill(0x77);
            Ok(())
        })?;
        original.return_unused(returned)?;
        exhaust(&mut original, &control)?;
        let retired = original.retire_for_recovery(config, &control, provider)?;
        let (not_before, past) = retired.receive(config, &control, provider)?;
        assert_eq!(not_before, control.expiry);
        assert_eq!(past.cookies[0].as_ptr(), address);
        assert!(past.cookie_was_used(&[0x11; 12]));
        assert!(!past.cookie_was_used(&[0x22; 12]));
        assert!(past.uid_was_issued(&UID));
        assert!(past.uid_was_issued(&NEXT_UID));
        assert!(!past.uid_was_issued(&[0x55; 32]));
        assert_eq!((past.cookies.len(), past.uid_count), (1, 2));
        assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
        genuine_archive_blocks_returned_and_sent_values(config, provider, past)
    })
}

/// A retired count refusal may contain earlier malformed charged traffic.
/// # Errors
/// Preserve real checked transitions and receiving projection failure.
/// # Panics
/// Fail if the broader admitted final-count predicate requires an invented history.
#[test]
fn malformed_history_with_final_authenticated_count_refusal_can_retire() -> Result<(), Failure> {
    with_provider(|config, provider| {
        let control = Control::new(Instant::now())?;
        let mut original = owner(
            config,
            &control,
            provider,
            vec![Zeroizing::new(vec![0x11; 12])],
        )?;
        pending(&mut original, &UID)?;
        for _ in 1..FRAMES {
            assert!(
                original
                    .receive_reply(&[0], original.material.udp, Instant::now())
                    .is_err()
            );
        }
        assert_eq!(
            original
                .receive_reply(
                    fixtures::NINE_COOKIES,
                    original.material.udp,
                    Instant::now()
                )
                .err(),
            Some(Failure::Ledger(LedgerError::Reply(ReplyError::Cookies))),
        );
        let (not_before, _) = original
            .retire_for_recovery(config, &control, provider)?
            .receive(config, &control, provider)?;
        assert_eq!(not_before, control.expiry);
        Ok(())
    })
}

/// Actual exhaustion is insufficient when the final producer class is different.
/// # Errors
/// Preserve genuine owner, sender and consumed-unknown setup refusal.
/// # Panics
/// Fail if a missing marker, still-live request or prior success yields retirement.
#[test]
fn missing_final_marker_or_prior_receipt_refuses_retirement() -> Result<(), Failure> {
    with_provider(|config, provider| {
        for case in 0..4 {
            let control = Control::new(Instant::now())?;
            let mut original = owner(
                config,
                &control,
                provider,
                vec![Zeroizing::new(vec![0x11; 12])],
            )?;
            pending(&mut original, &UID)?;
            for _ in 1..FRAMES {
                assert!(
                    original
                        .receive_reply(
                            fixtures::ZERO_COOKIES,
                            original.material.udp,
                            Instant::now()
                        )
                        .is_err()
                );
            }
            match case {
                0 => {}
                1 => {
                    assert!(
                        original
                            .receive_reply(&[0], original.material.udp, Instant::now())
                            .is_err()
                    );
                }
                2 => {
                    assert!(original.charge_consumed_unknown().is_err());
                }
                _ => {
                    original.receive_reply(
                        fixtures::HEALTHY,
                        original.material.udp,
                        Instant::now(),
                    )?;
                }
            }
            assert!(matches!(
                original.retire_for_recovery(config, &control, provider),
                Err(Failure::Ledger(LedgerError::Phase)),
            ));
        }
        assert_predecessor_retirement_refusal(config, provider)?;
        assert_premature_helpers_preserve_exhaustion(config, provider)?;
        Ok(())
    })
}

/// Marker chronology remains separate from its actual authenticated error class.
/// # Errors
/// Preserve genuine owner and preparation failure.
/// # Panics
/// Fail if a future or presubmission arrival can supply a retirement fact.
#[test]
fn final_count_marker_rechecks_receive_chronology_at_retirement() -> Result<(), Failure> {
    with_provider(|config, provider| {
        for future in [false, true] {
            let control = Control::new(Instant::now())?;
            let mut original = owner(
                config,
                &control,
                provider,
                vec![Zeroizing::new(vec![0x11; 12])],
            )?;
            pending(&mut original, &UID)?;
            for _ in 1..FRAMES {
                assert!(
                    original
                        .receive_reply(
                            fixtures::ZERO_COOKIES,
                            original.material.udp,
                            Instant::now()
                        )
                        .is_err()
                );
            }
            let arrival = if future {
                control.expiry
            } else {
                control
                    .origin
                    .checked_sub(Duration::from_nanos(1))
                    .ok_or(Failure::Control)?
            };
            assert_eq!(
                original
                    .receive_reply(fixtures::ZERO_COOKIES, original.material.udp, arrival)
                    .err(),
                Some(Failure::Ledger(LedgerError::Reply(ReplyError::Cookies))),
            );
            assert!(matches!(
                original.retire_for_recovery(config, &control, provider),
                Err(Failure::Ledger(LedgerError::Time)),
            ));
        }
        Ok(())
    })
}

/// Taking an intent or changing actual owner identity cannot mint another fact.
/// # Errors
/// Preserve genuine checked owners, exhaustion and cancellation failure.
/// # Panics
/// Fail if a drained, foreign or cancelled source can retire successfully.
#[test]
fn retirement_refuses_drained_foreign_and_cancelled_sources() -> Result<(), Failure> {
    with_provider(|config, provider| {
        for case in 0..5 {
            let control = Control::new(Instant::now())?;
            let other_control = Control::new(Instant::now())?;
            let other_config = self::config()?;
            let other_admission = ProcessAdmission::new();
            let other_claim = other_admission.claim()?;
            let other_provider = ProviderScope::component(other_claim);
            let mut original = owner(
                config,
                &control,
                provider,
                vec![Zeroizing::new(vec![0x11; 12])],
            )?;
            exhaust(&mut original, &control)?;
            if case == 0 {
                assert!(original.take_recovery_intent().is_some());
            }
            if case == 4 {
                control.cancel()?;
            }
            let result = match case {
                1 => original.retire_for_recovery(&other_config, &control, provider),
                2 => original.retire_for_recovery(config, &other_control, provider),
                3 => original.retire_for_recovery(config, &control, &other_provider),
                _ => original.retire_for_recovery(config, &control, provider),
            };
            assert!(matches!(
                result,
                Err(Failure::Control | Failure::Cancelled | Failure::Ledger(LedgerError::Phase))
            ));
        }
        assert_preview_retirement_refusal(provider)?;
        Ok(())
    })
}

/// The receiving projection uses actual live references after secret retirement.
/// # Errors
/// Preserve real checked retirement and cancellation failure.
/// # Panics
/// Fail if a foreign or newly cancelled receiver can retain history.
#[test]
fn receiving_projection_refuses_foreign_or_cancelled_actual_control() -> Result<(), Failure> {
    with_provider(|config, provider| {
        for cancelled in [false, true] {
            let control = Control::new(Instant::now())?;
            let other = Control::new(Instant::now())?;
            let mut original = owner(
                config,
                &control,
                provider,
                vec![Zeroizing::new(vec![0x11; 12])],
            )?;
            exhaust(&mut original, &control)?;
            let retired = original.retire_for_recovery(config, &control, provider)?;
            if cancelled {
                control.cancel()?;
                assert!(matches!(
                    retired.receive(config, &control, provider),
                    Err(Failure::Cancelled)
                ));
            } else {
                assert!(matches!(
                    retired.receive(config, &other, provider),
                    Err(Failure::Control)
                ));
            }
        }
        assert_projection_binding_refusals(config, provider)?;
        Ok(())
    })
}

/// Cookie filtering and duplicate UID refusal leave all local budgets irreversible.
/// # Errors
/// Preserve real checked transfer, preparation, explicit return and submission.
/// # Panics
/// Fail if old wire values escape, entropy redraws or a return refunds allowance.
#[test]
fn installed_history_filters_cookie_and_refuses_uid_before_fresh_send() -> Result<(), Failure> {
    with_provider(|config, provider| {
        let original_control = Control::new(Instant::now())?;
        let past = history(config, &original_control, provider, vec![0x11; 12])?;
        let control = Control::new(Instant::now())?;
        let replacement = owner(
            config,
            &control,
            provider,
            vec![
                Zeroizing::new(vec![0x11; 12]),
                Zeroizing::new(vec![0x22; 12]),
            ],
        )?;
        let mut replacement = replacement.install_previous(config, &control, provider, past)?;
        assert_eq!(replacement.cookie_population()?, (1, 12));
        let before = replacement.status();
        assert_eq!(
            (
                before.attempts,
                before.frames,
                before.bytes,
                before.used,
                before.uids
            ),
            (ATTEMPTS, FRAMES, RECEIVE_BYTES, 0, 0)
        );
        assert_eq!(before.available, 1);
        let ticket = replacement.reserve(RequestInput::Observed)?;
        let fills = Cell::new(0);
        assert_eq!(
            replacement
                .prepare_observed(&ticket, |output| {
                    fills.set(fills.get() + 1);
                    output[..8].copy_from_slice(&WIRE.to_be_bytes());
                    output[8..40].copy_from_slice(&UID);
                    output[40..].fill(0x66);
                    Ok(())
                })
                .err(),
            Some(Failure::Ledger(LedgerError::RepeatedUid))
        );
        assert_eq!(fills.get(), 1);
        assert_eq!(
            (replacement.status().attempts, replacement.status().uids),
            (31, 0)
        );
        replacement.return_unused(ticket)?;
        assert_eq!(replacement.status().attempts, 31);
        let ticket = replacement.reserve(RequestInput::Observed)?;
        replacement.prepare_observed(&ticket, |output| {
            output[..8].copy_from_slice(&WIRE.to_be_bytes());
            output[8..40].copy_from_slice(&NEXT_UID);
            output[40..].fill(0x77);
            Ok(())
        })?;
        let sends = Cell::new(0);
        replacement.invoke_sender(ticket, |packet, _| {
            sends.set(sends.get() + 1);
            assert_eq!(&packet[52..84], &NEXT_UID);
            assert_eq!(&packet[88..100], &[0x22; 12]);
            SendOutcome::Completed(packet.len())
        })?;
        assert_eq!(sends.get(), 1);
        assert_eq!(replacement.status().attempts, 30);
        assert_eq!(
            (replacement.status().used, replacement.status().uids),
            (1, 1)
        );
        Ok(())
    })
}

/// A genuine earlier accepted reply independently forbids exhausted-context recovery.
/// # Errors
/// Preserve checked owner, fresh second request and authenticated admission.
/// # Panics
/// Fail if a real final count marker can hide the original accepted receipt.
#[test]
fn prior_receipt_refuses_retirement_with_genuine_final_count_marker() -> Result<(), Failure> {
    with_provider(|config, provider| {
        let control = Control::new(Instant::now())?;
        let mut original = owner(
            config,
            &control,
            provider,
            vec![Zeroizing::new(vec![0x11; 12])],
        )?;
        pending(&mut original, &NEXT_UID)?;
        let first =
            original.receive_reply(fixtures::WRONG_UID, original.material.udp, Instant::now())?;
        assert_eq!(first.cookie_counts(), (1, 1));
        assert_eq!(original.status().frames, 31);

        pending(&mut original, &UID)?;
        for remaining in (0..31).rev() {
            assert_eq!(
                original
                    .receive_reply(
                        fixtures::ZERO_COOKIES,
                        original.material.udp,
                        Instant::now(),
                    )
                    .err(),
                Some(Failure::Ledger(LedgerError::Reply(ReplyError::Cookies))),
            );
            assert_eq!(original.status().frames, remaining);
        }
        let status = original.status();
        assert_eq!(
            (status.attempts, status.frames, status.used, status.uids),
            (30, 0, 2, 2)
        );
        assert_eq!(status.phase, RequestPhase::Idle);
        assert!(original.ledger.exhausted_response.is_some());
        assert!(original.ledger.recovery.as_ref().is_some_and(|intent| {
            intent.reason == RecoveryReason::ReceiveWork && intent.not_before == control.expiry
        }));
        assert_eq!(
            original.last_receipt().map(Receipt::serial),
            Some(first.serial()),
        );
        assert_eq!(
            original
                .retire_for_recovery(config, &control, provider)
                .err(),
            Some(Failure::Ledger(LedgerError::Phase)),
        );
        Ok(())
    })
}

/// An old UID refuses; a valid different UID may complete with no usable refill.
/// # Errors
/// Preserve genuine checked source/history, submission and authenticated response.
/// # Panics
/// Fail if predecessor cookies reenter inventory or a rejection refunds work.
#[test]
fn authenticated_replacement_filters_predecessor_cookie_after_fresh_uid_reply()
-> Result<(), Failure> {
    with_provider(|config, provider| {
        let original_control = Control::new(Instant::now())?;
        let past = history(config, &original_control, provider, REFILL.to_vec())?;
        let control = Control::new(Instant::now())?;
        let replacement = owner(
            config,
            &control,
            provider,
            vec![Zeroizing::new(vec![0x22; 12])],
        )?;
        let mut replacement = replacement.install_previous(config, &control, provider, past)?;
        pending(&mut replacement, &NEXT_UID)?;
        assert_eq!(
            replacement
                .receive_reply(fixtures::HEALTHY, replacement.material.udp, Instant::now())
                .err(),
            Some(Failure::Ledger(LedgerError::Reply(
                ReplyError::UniqueIdentifier
            )))
        );
        assert_eq!(replacement.status().frames, 31);
        let receipt = replacement.receive_reply(
            fixtures::WRONG_UID,
            replacement.material.udp,
            Instant::now(),
        )?;
        assert_eq!(receipt.cookie_counts(), (1, 0));
        assert_eq!(replacement.cookie_population()?, (0, 0));
        assert_eq!(replacement.status().available, 0);
        assert_eq!(replacement.status().frames, 30);
        assert_eq!(replacement.status().phase, RequestPhase::Idle);
        let before = replacement.status();
        assert_eq!(
            replacement.reserve(RequestInput::Observed).err(),
            Some(Failure::Ledger(LedgerError::Cookies))
        );
        assert_eq!(replacement.status().attempts, before.attempts - 1);
        assert_eq!(replacement.status().frames, before.frames);
        assert_eq!(replacement.status().bytes, before.bytes);
        assert_eq!(
            replacement.last_receipt().map(Receipt::serial),
            Some(receipt.serial())
        );
        Ok(())
    })
}

/// Empty usable inventory, prior work and foreign source histories refuse attachment.
/// # Errors
/// Preserve real checked construction, retirement and explicit ticket return.
/// # Panics
/// Fail if history is discarded to admit an unusable or previously used context.
#[test]
fn attachment_refuses_known_only_pool_nonpristine_and_foreign_source() -> Result<(), Failure> {
    with_provider(|config, provider| {
        for case in 0..4 {
            let original_control = Control::new(Instant::now())?;
            let past = history(config, &original_control, provider, vec![0x11; 12])?;
            let control = Control::new(Instant::now())?;
            let other_config = self::config()?;
            let selected_config = if case == 3 { &other_config } else { config };
            let cookie = if case == 0 { 0x11 } else { 0x22 };
            let mut replacement = owner(
                selected_config,
                &control,
                provider,
                vec![Zeroizing::new(vec![cookie; 12])],
            )?;
            if case == 1 {
                let ticket = replacement.reserve(RequestInput::Observed)?;
                replacement.return_unused(ticket)?;
            }
            if case == 2 {
                let _ticket = replacement.reserve(RequestInput::Observed)?;
            }
            let result = replacement.install_previous(selected_config, &control, provider, past);
            assert!(matches!(
                result,
                Err(Failure::Control | Failure::Ledger(LedgerError::Phase | LedgerError::Cookies))
            ));
        }
        assert_install_config_argument_binding(config, provider)?;
        Ok(())
    })
}

/// A second predecessor and another actual provider/control never replace custody.
/// # Errors
/// Preserve genuine checked contexts and history transfer.
/// # Panics
/// Fail if a previous history or original source/provider binding can be replaced.
#[test]
fn attachment_refuses_second_history_foreign_provider_and_foreign_control() -> Result<(), Failure> {
    with_provider(|config, provider| {
        for case in 0..3 {
            let first_control = Control::new(Instant::now())?;
            let past = history(config, &first_control, provider, vec![0x11; 12])?;
            let control = Control::new(Instant::now())?;
            let other_control = Control::new(Instant::now())?;
            let other_admission = ProcessAdmission::new();
            let other_claim = other_admission.claim()?;
            let other_provider = ProviderScope::component(other_claim);
            let selected_provider = if case == 1 { &other_provider } else { provider };
            let mut replacement = owner(
                config,
                &control,
                selected_provider,
                vec![Zeroizing::new(vec![0x22; 12])],
            )?;
            let result = if case == 0 {
                replacement = replacement.install_previous(config, &control, provider, past)?;
                let another_control = Control::new(Instant::now())?;
                let another = history(config, &another_control, provider, vec![0x33; 12])?;
                replacement.install_previous(config, &control, provider, another)
            } else {
                let expected_control = if case == 2 { &other_control } else { &control };
                replacement.install_previous(config, expected_control, selected_provider, past)
            };
            assert!(matches!(
                result,
                Err(Failure::Control | Failure::Ledger(LedgerError::Phase))
            ));
        }
        assert_install_provider_argument_binding(config, provider)?;
        assert_install_published_control_binding(config, provider)?;
        Ok(())
    })
}

/// Arbitrary raw dimensions receive only a bounds result, never archive authority.
/// # Errors
/// Preserve actual checked fixture setup and valid raw boundary checks.
/// # Panics
/// Fail if empty or over-capacity data passes the finite dimension contract.
#[test]
fn checked_history_limits_have_exact_neighbor_and_reject_invalid_dimensions() -> Result<(), Failure>
{
    with_provider(|config, provider| {
        let exact = vec![Zeroizing::new(vec![0x11; COOKIE_BYTES_LIMIT])];
        check_history_bounds(&exact, 1)?;
        assert_eq!(exact[0].len(), COOKIE_BYTES_LIMIT);
        let cookies: Vec<_> = (0_u8..32)
            .map(|value| Zeroizing::new(vec![value; 12]))
            .collect();
        check_history_bounds(&cookies, 32)?;
        assert_eq!(cookies.len(), 32);
        for (entries, bytes, uid_count) in [
            (0, 0, 1),
            (1, 1, 0),
            (1, 1, ATTEMPTS + 1),
            (ATTEMPTS + 1, 1, ATTEMPTS),
            (2, 1, 1),
            (1, COOKIE_BYTES_LIMIT + 1, 1),
        ] {
            let cookies: Vec<_> = (0..entries)
                .map(|_| Zeroizing::new(vec![0x22; bytes]))
                .collect();
            assert_eq!(
                check_history_bounds(&cookies, uid_count),
                Err(Failure::Ledger(LedgerError::Capacity))
            );
        }
        assert_checkout_priority_at_real_boundaries(config, provider)?;
        assert_empty_pool_priority_survives_last_attempt(config, provider)?;
        Ok(())
    })
}

/// Construct a genuine short Control before any checked owner publication.
/// # Errors
/// Preserve monotonic arithmetic or real Control construction refusal.
fn short_control() -> Result<Control, Failure> {
    let origin = Instant::now();
    let cutoff = origin
        .checked_add(Duration::from_millis(250))
        .ok_or(Failure::Deadline)?;
    Control::clamped(origin, cutoff)
}

/// Park once at the real original expiry, without changing Control or clock state.
/// # Panics
/// Refuse an accidentally long wait or an early return before actual expiry.
fn expire(control: &Control) {
    let remaining = control.expiry.saturating_duration_since(Instant::now());
    assert!(remaining <= Duration::from_millis(250));
    std::thread::sleep(remaining);
    assert!(Instant::now() >= control.expiry);
}

/// Every new consuming guard rechecks real expiry after earlier healthy setup.
/// # Errors
/// Preserve genuine construction, exhaustion, retirement and history projection.
/// # Panics
/// Fail if actual expiry admits retirement, projection or replacement attachment.
#[test]
fn actual_expiry_refuses_retirement_projection_and_attachment() -> Result<(), Failure> {
    with_provider(|config, provider| {
        let control = short_control()?;
        let mut original = owner(
            config,
            &control,
            provider,
            vec![Zeroizing::new(vec![0x11; 12])],
        )?;
        exhaust(&mut original, &control)?;
        expire(&control);
        assert_eq!(
            original
                .retire_for_recovery(config, &control, provider)
                .err(),
            Some(Failure::Deadline),
        );

        let control = short_control()?;
        let mut original = owner(
            config,
            &control,
            provider,
            vec![Zeroizing::new(vec![0x11; 12])],
        )?;
        exhaust(&mut original, &control)?;
        let retired = original.retire_for_recovery(config, &control, provider)?;
        expire(&control);
        assert_eq!(
            retired.receive(config, &control, provider).err(),
            Some(Failure::Deadline),
        );

        let original_control = Control::new(Instant::now())?;
        let past = history(config, &original_control, provider, vec![0x11; 12])?;
        let control = short_control()?;
        let replacement = owner(
            config,
            &control,
            provider,
            vec![Zeroizing::new(vec![0x22; 12])],
        )?;
        expire(&control);
        assert_eq!(
            replacement
                .install_previous(config, &control, provider, past)
                .err(),
            Some(Failure::Deadline),
        );
        Ok(())
    })
}

/// The projected archive outlives only the original local Control, never its provider.
/// # Errors
/// Preserve actual retirement, cancellation, replacement and authenticated admission.
/// # Panics
/// Fail if history loses membership or prevents the fresh tagged neighbor's completion.
#[test]
fn projected_history_survives_old_control_scope_before_replacement_use() -> Result<(), Failure> {
    with_provider(|config, provider| {
        let past = {
            let original_control = Control::new(Instant::now())?;
            history(config, &original_control, provider, vec![0x11; 12])?
        };
        assert!(past.cookie_was_used(&[0x11; 12]));
        assert!(past.uid_was_issued(&UID));
        let control = Control::new(Instant::now())?;
        let replacement = owner(
            config,
            &control,
            provider,
            vec![Zeroizing::new(vec![0x22; 12])],
        )?;
        let mut replacement = replacement.install_previous(config, &control, provider, past)?;
        pending(&mut replacement, &NEXT_UID)?;
        let receipt = replacement.receive_reply(
            fixtures::WRONG_UID,
            replacement.material.udp,
            Instant::now(),
        )?;
        assert_eq!(receipt.cookie_counts(), (1, 1));
        assert_eq!(replacement.status().phase, RequestPhase::Idle);
        assert_eq!(
            (replacement.status().used, replacement.status().uids),
            (1, 1)
        );
        Ok(())
    })
}

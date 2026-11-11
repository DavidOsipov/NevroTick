// SPDX-License-Identifier: GPL-2.0-only
//! Actual ledger component controls with public fixed keys and valid-tag packets.
//! These callbacks supply no TLS, UDP, native completion or fresh-time evidence.

use std::cell::Cell;
use std::net::SocketAddr;
use std::num::NonZeroU64;
use std::time::{Duration, Instant};

use zeroize::Zeroizing;

use super::super::{AssociationCandidate, AssociationOwner, Config, Control, DirectionalKeys};
use super::{
    Failure, LedgerError, Receipt, RecoveryIntent, RecoveryReason, RequestInput, RequestPhase,
    SendOutcome, check_receive_times, refused,
};
use crate::nts_ke_experiment::process::{ProcessAdmission, ProviderScope};
use crate::nts_ke_experiment::profile::StrictProfile;
use crate::nts_ke_experiment::request::RequestError;
use crate::nts_ke_experiment::response::{ReplyError, fixtures, stage_reply};
use crate::nts_ke_experiment::{CancelOutcome, Phase};

/// Independent public outgoing key and fixture UID.
const C2S: [u8; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31,
];
/// Independent public response-fixture key; it differs from C2S in both halves.
const S2C: [u8; 32] = [
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f,
    0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f,
];
/// The valid-tag response's literal wire correlation, not local T1.
const WIRE: u64 = 0x1122_3344_5566_7788;
/// Public opaque body carried by the independent healthy response.
const REFILL: [u8; 12] = [
    0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab,
];

/// Exercise the actual private association receiver with component-only material.
/// # Errors
/// Preserve real constructor, publication or exercised consumer refusals.
fn with_owner(
    cookies: Vec<Zeroizing<Vec<u8>>>,
    exercise: impl FnOnce(&mut AssociationOwner<'_>, &Control) -> Result<(), Failure>,
) -> Result<(), Failure> {
    let config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    let control = Control::new(Instant::now())?;
    let admission = ProcessAdmission::new();
    let claim = admission.claim()?;
    let provider = ProviderScope::component(&claim);
    let candidate = AssociationCandidate::new(
        &config,
        &control,
        &provider,
        SocketAddr::new(config.peer.ip(), 123),
        StrictProfile::component(),
        DirectionalKeys::new(Zeroizing::new(C2S), Zeroizing::new(S2C)),
        cookies,
    );
    let mut owner = AssociationOwner::receive(&config, &control, &provider, candidate)?;
    exercise(&mut owner, &control)
}

/// One representable public initial cookie, distinct from response refill.
fn cookies() -> Vec<Zeroizing<Vec<u8>>> {
    vec![Zeroizing::new(vec![0x11; 12])]
}

/// Supply public UID/nonce fixture bytes; no randomness claim follows.
fn entropy(output: &mut [u8; 48]) {
    output[..32].copy_from_slice(&C2S);
    output[32..].fill(0x66);
}

/// Keep a literal local T1 distinct from the request's saved wire token.
/// # Errors
/// Preserve actual fixed-header or nonzero-token construction refusal.
fn packet_input() -> Result<RequestInput, Failure> {
    let local_t1 = ntp_wire::decode_fixed_header(&[0; 48])
        .map_err(|_| Failure::Control)?
        .transmit_timestamp();
    Ok(RequestInput::Packet {
        wire: NonZeroU64::new(WIRE).ok_or(Failure::Control)?,
        local_t1,
    })
}

/// Complete the actual component adapter while observing its protected bytes.
/// # Errors
/// Preserve actual reservation, encoding and synchronous completion refusal.
/// # Panics
/// Fail if correlation or destination differs from its independent facts.
fn pending(owner: &mut AssociationOwner<'_>) -> Result<(), Failure> {
    let ticket = owner.reserve(packet_input()?)?;
    owner.prepare(&ticket, |output| {
        entropy(output);
        Ok(())
    })?;
    owner.invoke_sender(ticket, |packet, remote| {
        assert_eq!(remote.port(), 123);
        assert_eq!(&packet[40..48], &WIRE.to_be_bytes());
        SendOutcome::Completed(packet.len())
    })
}

/// Check the receiver's inclusive pool-byte contract, without TLS provenance.
/// # Errors
/// Preserve real component construction, reservation and explicit-return errors.
/// # Panics
/// Fail if equality rejects the untouched cookie or copies its owned body.
fn explicit_return_at_pool_byte_limit() -> Result<(), Failure> {
    let population = vec![
        Zeroizing::new(vec![0x11; 12]),
        Zeroizing::new(vec![0x22; 65_530]),
        Zeroizing::new(vec![0x33; 65_530]),
    ];
    with_owner(population, |owner, control| {
        let vector = owner.material.cookies.as_ptr();
        let body = owner.material.cookies[0].as_ptr();
        let ticket = owner.reserve(RequestInput::Preview)?;
        owner.return_unused(ticket)?;
        assert_eq!(owner.cookie_population()?, (3, 131_072));
        assert_eq!(owner.material.cookies.as_ptr(), vector);
        assert_eq!(owner.material.cookies[0].as_ptr(), body);
        assert_eq!(owner.status().attempts, 31);
        assert_eq!(owner.status().uids, 0);
        assert!(owner.last_receipt().is_none());
        assert!(matches!(
            *control.phase.lock().map_err(|_| Failure::Control)?,
            Phase::Published
        ));
        Ok(())
    })
}

/// Exercise actual finite capacity fallback and the zero-placeholder refusal.
/// # Errors
/// Preserve genuine owner, encoder and synchronous sender refusals.
/// # Panics
/// Fail if layout fallback reacquires entropy or records the wrong allowance.
fn preparation_capacity_controls() -> Result<(), Failure> {
    for (cookie_length, packet_length) in [(512, Some(3736)), (4096, None)] {
        with_owner(
            vec![Zeroizing::new(vec![0x11; cookie_length])],
            |owner, _| {
                let ticket = owner.reserve(packet_input()?)?;
                let calls = Cell::new(0);
                let result = owner.prepare(&ticket, |output| {
                    calls.set(calls.get() + 1);
                    entropy(output);
                    Ok(())
                });
                assert_eq!(calls.get(), 1);
                assert_eq!(owner.status().attempts, 31);
                assert_eq!(owner.status().uids, 1);
                if let Some(packet_length) = packet_length {
                    // Seven placeholders require 4252 bytes; six fit in 3736.
                    result?;
                    assert_eq!(
                        owner
                            .ledger
                            .active
                            .as_ref()
                            .map(|active| active.maximum_cookies),
                        Some(7)
                    );
                    owner.invoke_sender(ticket, |packet, _| {
                        assert_eq!(packet.len(), packet_length);
                        SendOutcome::Completed(packet.len())
                    })?;
                    assert_eq!(owner.status().phase, RequestPhase::Pending);
                } else {
                    assert_eq!(
                        result,
                        Err(Failure::Packet(RequestError::Format(
                            ntp_wire::NtsRequestEncodeError::FrameLimit(
                                ntp_wire::FrameError::ByteLimit
                            )
                        )))
                    );
                    assert_eq!(owner.status().phase, RequestPhase::Preparing);
                    assert_eq!(owner.status().used, 0);
                    owner.return_unused(ticket)?;
                    assert_eq!(owner.status().available, 1);
                }
                Ok(())
            },
        )?;
    }
    Ok(())
}

/// Refuse above-capacity bytes, then accept the genuine exact-size neighbor.
/// # Errors
/// Preserve actual pending construction and authenticated packet admission.
/// # Panics
/// Fail if byte equality refuses, an oversized packet advances state, or work resets.
fn actual_packet_capacity_controls() -> Result<(), Failure> {
    with_owner(cookies(), |owner, control| {
        pending(owner)?;
        let expiry = control.expiry;
        assert!(matches!(
            owner.receive_reply(
                fixtures::TOO_MANY_PACKET_BYTES,
                owner.material.udp,
                Instant::now()
            ),
            Err(Failure::Ledger(LedgerError::Capacity))
        ));
        assert_eq!(owner.status().phase, RequestPhase::Pending);
        assert_eq!(owner.status().frames, 31);
        assert_eq!(owner.status().bytes, 131_072 - 4100);
        assert!(owner.last_receipt().is_none());
        let receipt = owner.receive_reply(
            fixtures::MAX_PACKET_BYTES,
            owner.material.udp,
            Instant::now(),
        )?;
        assert_eq!(receipt.serial(), 1);
        assert_eq!(receipt.cookie_counts(), (1, 1));
        assert_eq!(owner.status().phase, RequestPhase::Idle);
        assert_eq!(owner.status().frames, 30);
        assert_eq!(owner.status().bytes, 131_072 - 4100 - 4096);
        assert_eq!(control.expiry, expiry);
        Ok(())
    })
}

/// Spend raw work through real oversized refusals until the requested remainder.
///
/// The setup requires a Pending request with its original byte allowance and a
/// remainder in 2..=61440, so both charged refusals are bounded oversized inputs.
/// # Panics
/// Fail on invalid setup, changed request custody or refunded raw bytes.
fn leave_reply_bytes(owner: &mut AssociationOwner<'_>, remaining: usize) {
    assert!((2..=61_440).contains(&remaining));
    let first = vec![0_u8; 65_535];
    let second_length = 131_072 - first.len() - remaining;
    assert!(second_length <= 65_535 && second_length > 4096);
    let second = vec![0_u8; second_length];
    for raw in [&first, &second] {
        let outcome = owner.receive_reply(raw, owner.material.udp, Instant::now());
        assert!(matches!(
            outcome,
            Err(Failure::Ledger(LedgerError::Capacity))
        ));
        assert_eq!(owner.status().phase, RequestPhase::Pending);
    }
    assert_eq!(owner.status().bytes, remaining);
}

/// Exercise inclusive time boundaries both at the receiver and pure helper.
/// # Errors
/// Preserve real pending construction, timestamp arithmetic and healthy receipt.
/// # Panics
/// Fail if an early arrival publishes or either inclusive equality refuses.
fn inclusive_receive_time_controls() -> Result<(), Failure> {
    with_owner(cookies(), |owner, control| {
        pending(owner)?;
        let sent = owner
            .ledger
            .active
            .as_ref()
            .and_then(|active| active.sent_at)
            .ok_or(Failure::Control)?;
        let early = sent
            .checked_sub(Duration::from_nanos(1))
            .ok_or(Failure::Control)?;
        let before = owner.status();
        let expiry = control.expiry;
        assert!(matches!(
            owner.receive_reply(fixtures::HEALTHY, owner.material.udp, early),
            Err(Failure::Ledger(LedgerError::Time))
        ));
        assert_eq!(owner.status().phase, RequestPhase::Pending);
        assert_eq!(owner.status().frames, before.frames - 1);
        assert_eq!(owner.status().bytes, before.bytes - fixtures::HEALTHY.len());
        assert!(owner.last_receipt().is_none());
        let receipt = owner.receive_reply(fixtures::HEALTHY, owner.material.udp, sent)?;
        assert!(receipt.accepted_at() >= sent);
        assert_eq!(control.expiry, expiry);
        Ok(())
    })?;
    let sent = Instant::now();
    let processing = sent
        .checked_add(Duration::from_nanos(1))
        .ok_or(Failure::Control)?;
    let future = processing
        .checked_add(Duration::from_nanos(1))
        .ok_or(Failure::Control)?;
    assert_eq!(check_receive_times(sent, sent, sent), Ok(()));
    assert_eq!(check_receive_times(sent, sent, processing), Ok(()));
    assert_eq!(check_receive_times(sent, processing, processing), Ok(()));
    assert_eq!(
        check_receive_times(sent, future, processing),
        Err(refused(LedgerError::Time))
    );
    Ok(())
}

/// Check only inert replacement-helper domains with genuinely opened replies.
///
/// These old pools obey the association's component bounds but do not represent
/// a Pending request's refill vacancies or establish TLS-producer provenance.
/// # Errors
/// Preserve real component construction, AEAD staging and healthy helper errors.
/// # Panics
/// Fail if count/byte limits weaken or staging mutates the original pool or receipt.
fn refill_staging_domain_controls() -> Result<(), Failure> {
    for count in [7_u8, 8] {
        let population = (0..count)
            .map(|index| Zeroizing::new(vec![0x30 + index; 12]))
            .collect();
        assert_refill_staging_domain(population, count == 7)?;
    }
    for extra in [0, 1] {
        let population = vec![
            Zeroizing::new(vec![0x44; 65_530 + extra]),
            Zeroizing::new(vec![0x55; 65_530]),
        ];
        assert_refill_staging_domain(population, extra == 0)?;
    }
    Ok(())
}

/// Keep authenticated staging inert at the helper's aggregate contract boundary.
/// # Errors
/// Preserve healthy component construction and S2C authentication failures.
/// # Panics
/// Fail if the expected capacity disposition, custody or publication differs.
fn assert_refill_staging_domain(
    population: Vec<Zeroizing<Vec<u8>>>,
    accepted: bool,
) -> Result<(), Failure> {
    with_owner(population, |owner, control| {
        let before = owner.status();
        let vector = owner.material.cookies.as_ptr();
        let bodies: Vec<_> = owner
            .material
            .cookies
            .iter()
            .map(|body| (body.as_ptr(), body.len()))
            .collect();
        let reply = stage_reply(&owner.material.keys, fixtures::HEALTHY, &C2S, WIRE, 1)
            .map_err(|error| refused(LedgerError::Reply(error)))?;
        let staged = owner.stage_replacement(reply);
        if accepted {
            let staged = staged?;
            assert_eq!(staged.received_cookies, 1);
            assert_eq!(staged.retained, 1);
            assert_eq!(staged.cookies[0].as_slice(), REFILL);
            // Project metadata before an assertion can format any cookie body.
            let staged_entries = staged.next.len();
            assert_eq!(staged_entries, 0);
            assert!(staged.next.capacity() >= 8);
        } else {
            assert!(matches!(
                staged,
                Err(Failure::Ledger(LedgerError::Capacity))
            ));
        }
        assert_eq!(owner.status(), before);
        assert_eq!(owner.material.cookies.as_ptr(), vector);
        assert_eq!(owner.material.cookies.len(), bodies.len());
        for (body, (address, length)) in owner.material.cookies.iter().zip(bodies) {
            assert_eq!(body.as_ptr(), address);
            assert_eq!(body.len(), length);
        }
        assert!(owner.last_receipt().is_none());
        assert!(matches!(
            *control.phase.lock().map_err(|_| Failure::Control)?,
            Phase::Published
        ));
        Ok(())
    })
}

/// Busy is effect-free; only explicit return restores the original allocation.
/// # Errors
/// Preserve healthy reservation/preparation/return refusal.
/// # Panics
/// Fail if Busy calls entropy, return copies the body, or credit/deadline renews.
#[test]
fn reservation_busy_and_explicit_return_preserve_global_credit() -> Result<(), Failure> {
    with_owner(cookies(), |owner, control| {
        let original_expiry = control.expiry;
        let original_body = owner.material.cookies[0].as_ptr();
        let original_vector = owner.material.cookies.as_ptr();
        let ticket = owner.reserve(packet_input()?)?;
        assert!(matches!(
            owner.reserve(packet_input()?),
            Err(Failure::Ledger(LedgerError::Busy))
        ));
        assert_eq!(owner.status().attempts, 31);
        owner.prepare(&ticket, |output| {
            entropy(output);
            Ok(())
        })?;
        owner.return_unused(ticket)?;
        assert_eq!(owner.material.cookies[0].as_ptr(), original_body);
        assert_eq!(owner.material.cookies.as_ptr(), original_vector);
        assert_eq!(owner.status().phase, RequestPhase::Idle);
        assert_eq!(owner.status().attempts, 31);
        assert_eq!(owner.status().uids, 1);
        assert_eq!(control.expiry, original_expiry);
        assert!(matches!(
            *control.phase.lock().map_err(|_| Failure::Control)?,
            Phase::Published
        ));
        Ok(())
    })?;
    explicit_return_at_pool_byte_limit()
}

/// Ticket destruction and forgetting cannot manufacture rollback.
/// # Errors
/// Preserve first reservation refusal.
/// # Panics
/// Fail if either abandoned capability frees the occupied slot or cookie.
#[test]
fn dropped_and_forgotten_tickets_leave_busy() -> Result<(), Failure> {
    for forget in [false, true] {
        with_owner(cookies(), |owner, _| {
            {
                let ticket = owner.reserve(RequestInput::Preview)?;
                if forget {
                    let _forgotten = std::mem::ManuallyDrop::new(ticket);
                }
            }
            assert!(matches!(
                owner.reserve(RequestInput::Preview),
                Err(Failure::Ledger(LedgerError::Busy))
            ));
            assert_eq!(owner.status().available, 0);
            assert_eq!(owner.status().attempts, 31);
            Ok(())
        })?;
    }
    Ok(())
}

/// Failed preparation and repeated UID cannot buy another entropy call.
/// # Errors
/// Preserve healthy preparation and explicit returns.
/// # Panics
/// Fail if a retired UID or a repeated preparation reaches encoding again.
#[test]
fn preparation_once_and_retired_uid_are_enforced() -> Result<(), Failure> {
    with_owner(cookies(), |owner, _| {
        let first = owner.reserve(packet_input()?)?;
        owner.prepare(&first, |output| {
            entropy(output);
            Ok(())
        })?;
        let calls = Cell::new(0);
        assert_eq!(
            owner.prepare(&first, |_| {
                calls.set(calls.get() + 1);
                Ok(())
            }),
            Err(refused(LedgerError::PreparationSpent))
        );
        assert_eq!(calls.get(), 0);
        owner.return_unused(first)?;
        let second = owner.reserve(packet_input()?)?;
        assert_eq!(
            owner.prepare(&second, |output| {
                entropy(output);
                Ok(())
            }),
            Err(refused(LedgerError::RepeatedUid))
        );
        assert_eq!(owner.status().uids, 1);
        assert_eq!(owner.status().attempts, 30);
        owner.return_unused(second)?;
        let third = owner.reserve(packet_input()?)?;
        owner.prepare(&third, |output| {
            entropy(output);
            output[0] = 0xff;
            Ok(())
        })?;
        assert_eq!(owner.status().uids, 2);
        Ok(())
    })?;
    preparation_capacity_controls()
}

/// Foreign return cannot mutate either association or retire another ticket.
/// # Errors
/// Preserve genuine owner construction and reservation refusal.
/// # Panics
/// Fail if equal serials substitute for actual owner identity.
#[test]
fn foreign_return_does_not_restore_a_cookie() -> Result<(), Failure> {
    with_owner(cookies(), |first, _| {
        let ticket = first.reserve(RequestInput::Preview)?;
        with_owner(cookies(), |second, _| {
            let own_ticket = second.reserve(RequestInput::Preview)?;
            assert_eq!(ticket.serial, own_ticket.serial);
            let before = second.status();
            let calls = Cell::new(0);
            assert_eq!(
                second.prepare(&ticket, |_| {
                    calls.set(calls.get() + 1);
                    Ok(())
                }),
                Err(refused(LedgerError::Ticket))
            );
            assert_eq!(calls.get(), 0);
            assert_eq!(
                second.return_unused(ticket),
                Err(refused(LedgerError::Ticket))
            );
            assert_eq!(second.status(), before);
            second.return_unused(own_ticket)?;
            assert_eq!(second.status().phase, RequestPhase::Idle);
            assert_eq!(second.status().available, 1);
            Ok(())
        })?;
        assert_eq!(first.status().phase, RequestPhase::Reserved);
        assert_eq!(first.status().available, 0);
        Ok(())
    })
}

/// Every callback exposure archives value first, including short/error/unknown.
/// # Errors
/// Preserve the actual protected encoding and owner construction.
/// # Panics
/// Fail if a callback outcome restores used values or clears unknown custody.
#[test]
fn send_outcomes_keep_value_retirement_and_distinct_custody() -> Result<(), Failure> {
    for outcome in [
        SendOutcome::Completed(1),
        SendOutcome::Failed(std::io::ErrorKind::BrokenPipe),
        SendOutcome::Unknown,
    ] {
        with_owner(cookies(), |owner, _| {
            let address = owner.material.cookies[0].as_ptr();
            let ticket = owner.reserve(packet_input()?)?;
            owner.prepare(&ticket, |output| {
                entropy(output);
                Ok(())
            })?;
            assert!(owner.invoke_sender(ticket, |_, _| outcome).is_err());
            assert_eq!(owner.ledger.used[0].as_ptr(), address);
            assert_eq!(owner.status().used, 1);
            assert_eq!(owner.status().available, 0);
            assert_eq!(
                owner.status().phase,
                if outcome == SendOutcome::Unknown {
                    RequestPhase::Invoked
                } else {
                    RequestPhase::Idle
                }
            );
            Ok(())
        })?;
    }
    Ok(())
}

/// Callback unwinding preserves the same archived cookie and packet custody.
/// # Errors
/// Preserve healthy construction/encoding refusal.
/// # Panics
/// Fail if ordinary unwinding turns unresolved invocation into reusable capacity.
#[test]
#[expect(
    clippy::panic,
    reason = "intentional adapter fault proves sticky custody"
)]
fn callback_panic_keeps_invoked_busy() -> Result<(), Failure> {
    with_owner(cookies(), |owner, _| {
        let ticket = owner.reserve(packet_input()?)?;
        owner.prepare(&ticket, |output| {
            entropy(output);
            Ok(())
        })?;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _outcome =
                owner.invoke_sender(ticket, |_, _| panic!("public component callback control"));
        }));
        assert!(result.is_err());
        assert_eq!(owner.status().phase, RequestPhase::Invoked);
        assert_eq!(owner.status().used, 1);
        assert!(matches!(
            owner.reserve(packet_input()?),
            Err(Failure::Ledger(LedgerError::Busy))
        ));
        Ok(())
    })
}

/// Pre-invocation cancellation has no adapter effect; post-invocation never reuses.
/// # Errors
/// Preserve actual construction, preparation and cancellation failures.
/// # Panics
/// Fail if a late full return manufactures Pending or an earlier stop calls send.
#[test]
fn cancellation_before_and_inside_adapter_preserves_original_facts() -> Result<(), Failure> {
    with_owner(cookies(), |owner, control| {
        let ticket = owner.reserve(packet_input()?)?;
        owner.prepare(&ticket, |output| {
            entropy(output);
            Ok(())
        })?;
        control.cancel()?;
        let calls = Cell::new(0);
        assert_eq!(
            owner.invoke_sender(ticket, |_, _| {
                calls.set(calls.get() + 1);
                SendOutcome::Completed(0)
            }),
            Err(Failure::Cancelled)
        );
        assert_eq!(calls.get(), 0);
        assert_eq!(owner.status().used, 0);
        Ok(())
    })?;
    with_owner(cookies(), |owner, control| {
        let ticket = owner.reserve(packet_input()?)?;
        owner.prepare(&ticket, |output| {
            entropy(output);
            Ok(())
        })?;
        assert_eq!(
            owner.invoke_sender(ticket, |packet, _| {
                assert_eq!(control.cancel(), Ok(CancelOutcome::Cancelled));
                SendOutcome::Completed(packet.len())
            }),
            Err(Failure::Cancelled)
        );
        assert_eq!(owner.status().used, 1);
        assert_eq!(owner.status().phase, RequestPhase::Idle);
        assert!(owner.last_receipt().is_none());
        Ok(())
    })
}

/// Same-value duplicate inventory is skipped after the first possible exposure.
/// # Errors
/// Preserve healthy checkout/encoding and explicit return.
/// # Panics
/// Fail if a distinct allocation with already used bytes can be sent again.
#[test]
fn duplicate_cookie_values_are_skipped_at_checkout() -> Result<(), Failure> {
    for (extra_fresh, initial, after_send) in [(false, 1, 0), (true, 2, 1)] {
        let mut population = vec![
            Zeroizing::new(vec![0x11; 12]),
            Zeroizing::new(vec![0x11; 12]),
        ];
        if extra_fresh {
            population.push(Zeroizing::new(vec![0x22; 12]));
        }
        with_owner(population, |owner, _| {
            let fresh = owner.material.cookies.get(2).map(|cookie| cookie.as_ptr());
            assert_eq!(owner.status().available, initial);
            let first = owner.reserve(packet_input()?)?;
            owner.prepare(&first, |output| {
                entropy(output);
                Ok(())
            })?;
            assert_eq!(
                owner.invoke_sender(first, |_, _| SendOutcome::Completed(0)),
                Err(refused(LedgerError::ShortSend))
            );
            assert_eq!(owner.status().available, after_send);
            if let Some(fresh) = fresh {
                let second = owner.reserve(packet_input()?)?;
                assert_eq!(
                    owner
                        .ledger
                        .active
                        .as_ref()
                        .and_then(|active| active.cookie.as_ref())
                        .map(|cookie| cookie.as_ptr()),
                    Some(fresh)
                );
                assert_eq!(owner.status().available, 0);
                owner.return_unused(second)?;
                assert_eq!(owner.status().available, 1);
            } else {
                assert!(matches!(
                    owner.reserve(packet_input()?),
                    Err(Failure::Ledger(LedgerError::Cookies))
                ));
                assert_eq!(owner.status().available, 0);
            }
            assert_eq!(owner.status().used, 1);
            Ok(())
        })?;
    }
    Ok(())
}

/// Explicit returns cannot replenish aggregate attempt/UID allowances or intents.
/// # Errors
/// Preserve all original healthy reservations and explicit returns.
/// # Panics
/// Fail if repeated no-send returns permit attempt33 or a second recovery intent.
#[test]
fn returned_cookies_never_renew_attempts_or_recovery_intent() -> Result<(), Failure> {
    with_owner(cookies(), |owner, control| {
        for value in 0_u8..32 {
            let ticket = owner.reserve(RequestInput::Preview)?;
            owner.prepare(&ticket, |output| {
                entropy(output);
                output[0] = value;
                Ok(())
            })?;
            owner.return_unused(ticket)?;
        }
        assert_eq!(owner.status().uids, 32);
        assert_eq!(owner.status().attempts, 0);
        assert!(matches!(
            owner.reserve(RequestInput::Preview),
            Err(Failure::Ledger(LedgerError::Attempts))
        ));
        let recovery = owner.take_recovery_intent().ok_or(Failure::Control)?;
        assert_eq!(recovery.reason(), RecoveryReason::Attempts);
        assert_eq!(recovery.not_before(), control.expiry);
        assert!(matches!(
            owner.reserve(RequestInput::Preview),
            Err(Failure::Ledger(LedgerError::Attempts))
        ));
        assert!(owner.take_recovery_intent().is_none());
        Ok(())
    })
}

/// Actual valid-tag zero-cookie rejection preserves decreasing budget to neighbor32.
/// # Errors
/// Preserve real encoding, AEAD and healthy final publication refusal.
/// # Panics
/// Fail if rejection renews work/deadline, publishes a receipt, or blocks frame32.
#[test]
fn zero_cookie_rejections_keep_remaining_budget_and_last_healthy_neighbor() -> Result<(), Failure> {
    with_owner(cookies(), |owner, control| {
        pending(owner)?;
        let expiry = control.expiry;
        for count in 1..32 {
            let before = owner.status();
            assert!(matches!(
                owner.receive_reply(fixtures::ZERO_COOKIES, owner.material.udp, Instant::now()),
                Err(Failure::Ledger(LedgerError::Reply(ReplyError::Cookies)))
            ));
            assert_eq!(owner.status().frames, 32 - count);
            assert_eq!(
                owner.status().bytes,
                before.bytes - fixtures::ZERO_COOKIES.len()
            );
            assert_eq!(owner.status().phase, RequestPhase::Pending);
            assert!(owner.last_receipt().is_none());
            assert_eq!(control.expiry, expiry);
        }
        let receipt = owner.receive_reply(fixtures::HEALTHY, owner.material.udp, Instant::now())?;
        assert_eq!(receipt.cookie_counts(), (1, 1));
        assert_eq!(
            receipt.local_t1().map(|timestamp| timestamp.bits()),
            Some(0)
        );
        assert_eq!(receipt.fields().origin().bits(), WIRE);
        assert_eq!(owner.status().frames, 0);
        assert_eq!(owner.status().phase, RequestPhase::Idle);
        assert!(matches!(
            *control.phase.lock().map_err(|_| Failure::Control)?,
            Phase::Published
        ));
        Ok(())
    })
}

/// Last rejected charged frame retires instead of creating another allowance.
/// # Errors
/// Preserve actual encoding and original publication guards.
/// # Panics
/// Fail if zero-cookie traffic keeps a request alive after all32 charged frames.
#[test]
fn zero_cookie_exhaustion_retires_request_without_refund() -> Result<(), Failure> {
    with_owner(cookies(), |owner, _| {
        pending(owner)?;
        for _ in 0..32 {
            assert!(
                owner
                    .receive_reply(fixtures::ZERO_COOKIES, owner.material.udp, Instant::now())
                    .is_err()
            );
        }
        assert_eq!(owner.status().frames, 0);
        assert_eq!(owner.status().phase, RequestPhase::Idle);
        assert!(owner.last_receipt().is_none());
        assert!(matches!(
            owner.reserve(packet_input()?),
            Err(Failure::Ledger(LedgerError::ReceiveWork))
        ));
        assert_eq!(
            owner.take_recovery_intent().map(RecoveryIntent::reason),
            Some(RecoveryReason::ReceiveWork)
        );
        Ok(())
    })
}

/// Raw semantic/shape refusals consume work before a genuine accepted neighbor.
/// # Errors
/// Preserve the real encoder and independent healthy response consumer.
/// # Panics
/// Fail if endpoint/correlation/shape failures alter success or refund work.
#[test]
fn raw_refusals_are_charged_before_healthy_reply() -> Result<(), Failure> {
    with_owner(cookies(), |owner, _| {
        pending(owner)?;
        let remote = owner.material.udp;
        let wrong_remote = SocketAddr::new(remote.ip(), 124);
        let oversized = [0_u8; 4097];
        for (raw, endpoint) in [
            (fixtures::HEALTHY, wrong_remote),
            (fixtures::WRONG_UID, remote),
            (fixtures::WRONG_ORIGIN, remote),
            (oversized.as_slice(), remote),
        ] {
            let before = owner.status();
            assert!(owner.receive_reply(raw, endpoint, Instant::now()).is_err());
            assert_eq!(owner.status().frames, before.frames - 1);
            assert_eq!(owner.status().bytes, before.bytes - raw.len());
            assert_eq!(owner.status().phase, RequestPhase::Pending);
            assert!(owner.last_receipt().is_none());
        }
        let receipt = owner.receive_reply(fixtures::HEALTHY, remote, Instant::now())?;
        let accepted_at = receipt.accepted_at();
        assert_eq!(receipt.serial(), 1);
        assert!(
            owner
                .receive_reply(fixtures::HEALTHY, remote, Instant::now())
                .is_err()
        );
        assert_eq!(
            owner.last_receipt().map(Receipt::accepted_at),
            Some(accepted_at)
        );
        let remaining = owner.status();
        let second = owner.reserve(packet_input()?)?;
        owner.prepare(&second, |output| {
            entropy(output);
            output[0] = 0xff;
            Ok(())
        })?;
        owner.invoke_sender(second, |packet, _| SendOutcome::Completed(packet.len()))?;
        assert_eq!(owner.status().frames, remaining.frames);
        assert_eq!(owner.status().bytes, remaining.bytes);
        // ZERO_COOKIES carries the retired UID here: this second-attempt control
        // tests correlation refusal; the separate valid-UID cases test policy.
        for raw in [&[0_u8; 3][..], fixtures::ZERO_COOKIES] {
            let before = owner.status();
            assert!(owner.receive_reply(raw, remote, Instant::now()).is_err());
            assert_eq!(owner.status().phase, RequestPhase::Pending);
            assert_eq!(owner.status().frames, before.frames - 1);
            assert_eq!(owner.status().bytes, before.bytes - raw.len());
            let last = owner.last_receipt().ok_or(Failure::Control)?;
            assert_eq!(last.serial(), receipt.serial());
            assert_eq!(last.accepted_at(), receipt.accepted_at());
            assert_eq!(last.local_t1(), receipt.local_t1());
            assert_eq!(last.fields().origin(), receipt.fields().origin());
            assert_eq!(last.fields().receive(), receipt.fields().receive());
            assert_eq!(last.fields().transmit(), receipt.fields().transmit());
            assert_eq!(last.cookie_counts(), receipt.cookie_counts());
        }
        Ok(())
    })?;
    actual_packet_capacity_controls()
}

/// Cookie presence is separate from whether any returned value is never-used.
/// # Errors
/// Preserve actual protected request and authenticated reply admission.
/// # Panics
/// Fail if duplicate used values are reissued or mistaken for zero Cookie fields.
#[test]
fn used_only_cookie_reply_completes_without_usable_refill() -> Result<(), Failure> {
    with_owner(vec![Zeroizing::new(REFILL.to_vec())], |owner, _| {
        pending(owner)?;
        let receipt = owner.receive_reply(fixtures::HEALTHY, owner.material.udp, Instant::now())?;
        assert_eq!(receipt.cookie_counts(), (1, 0));
        assert_eq!(owner.status().available, 0);
        assert_eq!(owner.status().used, 1);
        assert!(matches!(
            owner.reserve(packet_input()?),
            Err(Failure::Ledger(LedgerError::Cookies))
        ));
        assert_eq!(
            owner.take_recovery_intent().map(RecoveryIntent::reason),
            Some(RecoveryReason::Cookies)
        );
        Ok(())
    })?;
    refill_staging_domain_controls()
}

/// The normal preview takes and explicitly returns through the same ledger.
/// # Errors
/// Preserve actual preview entropy, codec and explicit-return failure.
/// # Panics
/// Fail if opaque preview Drop commits control or refunds attempt/UID history.
#[test]
fn opaque_preview_uses_real_reservation_and_explicit_return() -> Result<(), Failure> {
    with_owner(cookies(), |owner, control| {
        let preview = owner.ledger_preview(|output| {
            entropy(output);
            Ok(())
        })?;
        drop(preview);
        assert_eq!(owner.status().phase, RequestPhase::Idle);
        assert_eq!(owner.status().available, 1);
        assert_eq!(owner.status().attempts, 31);
        assert_eq!(owner.status().uids, 1);
        assert!(matches!(
            *control.phase.lock().map_err(|_| Failure::Control)?,
            Phase::Published
        ));
        Ok(())
    })
}

/// Preview encoding never authorizes even a component callback byte exposure.
/// # Errors
/// Preserve actual preview preparation and owner construction.
/// # Panics
/// Fail if the sender receives the zero-T1 preview or archives its cookie.
#[test]
fn preview_cannot_enter_sender_callback() -> Result<(), Failure> {
    with_owner(cookies(), |owner, _| {
        let ticket = owner.reserve(RequestInput::Preview)?;
        owner.prepare(&ticket, |output| {
            entropy(output);
            Ok(())
        })?;
        let calls = Cell::new(0);
        assert_eq!(
            owner.invoke_sender(ticket, |_, _| {
                calls.set(calls.get() + 1);
                SendOutcome::Completed(0)
            }),
            Err(refused(LedgerError::Preview))
        );
        assert_eq!(calls.get(), 0);
        assert_eq!(owner.status().used, 0);
        assert_eq!(owner.status().phase, RequestPhase::Prepared);
        Ok(())
    })
}

/// Exactly the last raw-byte allowance remains usable by a healthy packet.
/// # Errors
/// Preserve actual encoding and the independently authenticated final neighbor.
/// # Panics
/// Fail if oversized refusals refund raw bytes or equality rejects the last grant.
#[test]
fn raw_byte_allowance_has_an_exact_healthy_neighbor() -> Result<(), Failure> {
    with_owner(cookies(), |owner, _| {
        pending(owner)?;
        leave_reply_bytes(owner, fixtures::HEALTHY.len());
        let receipt = owner.receive_reply(fixtures::HEALTHY, owner.material.udp, Instant::now())?;
        assert_eq!(receipt.cookie_counts(), (1, 1));
        assert_eq!(owner.status().bytes, 0);
        assert_eq!(owner.status().frames, 29);
        Ok(())
    })?;
    with_owner(cookies(), |owner, control| {
        pending(owner)?;
        leave_reply_bytes(owner, 140);
        let expiry = control.expiry;
        assert!(matches!(
            owner.receive_reply(&[0; 141], owner.material.udp, Instant::now()),
            Err(Failure::Ledger(LedgerError::ReceiveWork))
        ));
        assert_eq!(owner.status().frames, 29);
        assert_eq!(owner.status().bytes, 0);
        assert_eq!(owner.status().phase, RequestPhase::Idle);
        assert!(owner.last_receipt().is_none());
        assert_eq!(control.expiry, expiry);
        Ok(())
    })
}

/// An impossible arrival cannot mutate cookies/last success after real AEAD work.
/// # Errors
/// Preserve the original healthy pending and accepted neighboring response.
/// # Panics
/// Fail if original-expiry arrival publishes, renews a deadline or refunds work.
#[test]
fn future_arrival_preserves_pending_and_charged_work() -> Result<(), Failure> {
    with_owner(cookies(), |owner, control| {
        pending(owner)?;
        let before = owner.status();
        let expiry = control.expiry;
        assert!(
            owner
                .receive_reply(fixtures::HEALTHY, owner.material.udp, expiry)
                .is_err()
        );
        assert!(owner.last_receipt().is_none());
        assert_eq!(owner.status().phase, RequestPhase::Pending);
        assert_eq!(owner.status().frames, before.frames - 1);
        assert_eq!(owner.status().bytes, before.bytes - fixtures::HEALTHY.len());
        assert_eq!(control.expiry, expiry);
        let _receipt =
            owner.receive_reply(fixtures::HEALTHY, owner.material.udp, Instant::now())?;
        Ok(())
    })?;
    inclusive_receive_time_controls()
}

/// Entropy failure spends preparation but permits only an explicit unused return.
/// # Errors
/// Preserve healthy construction, reservation and explicit return refusal.
/// # Panics
/// Fail if failure refunds credit, repeats entropy or automatically returns bytes.
#[test]
fn entropy_failure_requires_explicit_return_and_keeps_spent_credit() -> Result<(), Failure> {
    with_owner(cookies(), |owner, _| {
        let ticket = owner.reserve(packet_input()?)?;
        assert_eq!(
            owner.prepare(&ticket, |output| {
                output.fill(0x55);
                Err(Failure::Entropy)
            }),
            Err(Failure::Entropy)
        );
        assert_eq!(owner.status().available, 0);
        assert_eq!(owner.status().phase, RequestPhase::Preparing);
        assert_eq!(owner.status().attempts, 31);
        let calls = Cell::new(0);
        assert_eq!(
            owner.prepare(&ticket, |_| {
                calls.set(calls.get() + 1);
                Ok(())
            }),
            Err(refused(LedgerError::PreparationSpent))
        );
        assert_eq!(calls.get(), 0);
        owner.return_unused(ticket)?;
        assert_eq!(owner.status().available, 1);
        assert_eq!(owner.status().uids, 0);
        assert_eq!(owner.status().attempts, 31);
        Ok(())
    })
}

/// One observed batch issues wire correlation without a declared local timestamp.
/// # Errors
/// Preserve actual checked owner, entropy, encoder, sender and reply failures.
/// # Panics
/// Fail if Busy/mode mismatch calls entropy, the split changes or T1 is fabricated.
#[test]
fn observed_request_retains_authenticated_header_without_declared_t1() -> Result<(), Failure> {
    with_owner(cookies(), |owner, control| {
        assert_eq!(owner.check_context(control)?, owner.material.udp);
        let foreign = Control::new(control.origin)?;
        assert_eq!(owner.check_context(&foreign), Err(Failure::Control));
        let ticket = owner.reserve(RequestInput::Observed)?;
        let calls = Cell::new(0);
        assert!(matches!(
            owner.reserve(RequestInput::Observed),
            Err(Failure::Ledger(LedgerError::Busy))
        ));
        assert_eq!(
            owner.prepare(&ticket, |_| {
                calls.set(calls.get() + 1);
                Ok(())
            }),
            Err(refused(LedgerError::Phase))
        );
        assert_eq!(calls.get(), 0);
        assert_eq!(owner.status().phase, RequestPhase::Reserved);
        owner.prepare_observed(&ticket, |output| {
            calls.set(calls.get() + 1);
            output[..8].copy_from_slice(&WIRE.to_be_bytes());
            entropy(
                (&mut output[8..])
                    .try_into()
                    .map_err(|_| Failure::Control)?,
            );
            Ok(())
        })?;
        assert_eq!(calls.get(), 1);
        owner.invoke_sender(ticket, |packet, _| {
            assert_eq!(&packet[40..48], &WIRE.to_be_bytes());
            assert_eq!(&packet[52..84], &C2S);
            SendOutcome::Completed(packet.len())
        })?;
        let receipt = owner.receive_reply(fixtures::HEALTHY, owner.material.udp, Instant::now())?;
        assert_eq!(receipt.local_t1(), None);
        let expected_header = ntp_wire::decode_fixed_header(&fixtures::HEALTHY[..48])
            .map_err(|_| Failure::Control)?;
        assert_eq!(receipt.fields().header(), expected_header);
        assert_eq!(receipt.fields().origin().bits(), WIRE);
        assert_eq!(owner.status().used, 1);
        assert_eq!(owner.status().uids, 1);
        assert_eq!(owner.status().phase, RequestPhase::Idle);
        Ok(())
    })
}

/// An unusable random token cannot trigger another draw or restore preparation.
/// # Errors
/// Preserve healthy component construction and explicit uninvoked return.
/// # Panics
/// Fail if zero entropy permits encoding, an automatic refund or a second fill.
#[test]
fn observed_zero_token_spends_preparation_without_resampling() -> Result<(), Failure> {
    with_owner(cookies(), |owner, _| {
        let ticket = owner.reserve(RequestInput::Observed)?;
        let calls = Cell::new(0);
        assert_eq!(
            owner.prepare_observed(&ticket, |output| {
                calls.set(calls.get() + 1);
                output.fill(0);
                Ok(())
            }),
            Err(refused(LedgerError::ZeroWireToken))
        );
        assert_eq!(calls.get(), 1);
        assert_eq!(owner.status().attempts, 31);
        assert_eq!(owner.status().uids, 0);
        assert_eq!(owner.status().used, 0);
        assert_eq!(owner.status().available, 0);
        assert_eq!(owner.status().phase, RequestPhase::Preparing);
        assert_eq!(
            owner.prepare_observed(&ticket, |_| {
                calls.set(calls.get() + 1);
                Ok(())
            }),
            Err(refused(LedgerError::PreparationSpent))
        );
        assert_eq!(calls.get(), 1);
        owner.return_unused(ticket)?;
        assert_eq!(owner.status().attempts, 31);
        assert_eq!(owner.status().available, 1);
        assert!(owner.last_receipt().is_none());
        Ok(())
    })
}

/// A consumed unknown extent retires Pending without manufactured parser bytes.
/// # Errors
/// Preserve genuine first completion, second preparation and owner controls.
/// # Panics
/// Fail if an unknown extent refunds work, overwrites receipt or charges twice.
#[test]
fn unknown_extent_exhausts_bytes_once_and_preserves_previous_receipt() -> Result<(), Failure> {
    with_owner(cookies(), |owner, control| {
        let original = owner.status();
        assert_eq!(
            owner.charge_consumed_unknown(),
            Err(refused(LedgerError::Phase))
        );
        assert_eq!(owner.status(), original);
        pending(owner)?;
        let first = owner.receive_reply(fixtures::HEALTHY, owner.material.udp, Instant::now())?;
        let ticket = owner.reserve(RequestInput::Observed)?;
        owner.prepare_observed(&ticket, |output| {
            output[..8].copy_from_slice(&WIRE.to_be_bytes());
            entropy(
                (&mut output[8..])
                    .try_into()
                    .map_err(|_| Failure::Control)?,
            );
            output[8] = 0xff;
            Ok(())
        })?;
        let prepared = owner.status();
        assert_eq!(
            owner.charge_consumed_unknown(),
            Err(refused(LedgerError::Phase))
        );
        assert_eq!(owner.status(), prepared);
        owner.invoke_sender(ticket, |packet, _| SendOutcome::Completed(packet.len()))?;
        let before = owner.status();
        let expiry = control.expiry;
        assert_eq!(
            owner.charge_consumed_unknown(),
            Err(refused(LedgerError::ReceiveWork))
        );
        assert_eq!(owner.status().bytes, 0);
        assert_eq!(owner.status().frames, before.frames - 1);
        assert_eq!(owner.status().phase, RequestPhase::Idle);
        assert_eq!(owner.status().used, 2);
        assert_eq!(owner.status().attempts, before.attempts);
        let last = owner.last_receipt().ok_or(Failure::Control)?;
        assert_eq!(last.serial(), first.serial());
        assert_eq!(last.accepted_at(), first.accepted_at());
        assert_eq!(last.fields().header(), first.fields().header());
        assert_eq!(control.expiry, expiry);
        let terminal = owner.status();
        assert_eq!(
            owner.charge_consumed_unknown(),
            Err(refused(LedgerError::Phase))
        );
        assert_eq!(owner.status(), terminal);
        assert_eq!(
            owner.take_recovery_intent().map(RecoveryIntent::reason),
            Some(RecoveryReason::ReceiveWork)
        );
        assert!(owner.take_recovery_intent().is_none());
        Ok(())
    })
}

/// A genuine Reserved request cannot skip the helper's preparation admission.
/// # Errors
/// Preserve real owner construction, reservation and the valid preparing neighbor.
/// # Panics
/// Fail if premature finishing issues a UID, publishes Ready or changes spent credit.
#[test]
fn reserved_request_cannot_finish_preparation_before_begin() -> Result<(), Failure> {
    with_owner(cookies(), |owner, _| {
        let ticket = owner.reserve(packet_input()?)?;
        let reserved = owner.status();
        let mut public_entropy = [0; 48];
        entropy(&mut public_entropy);
        assert_eq!(
            owner.finish_preparation(&public_entropy),
            Err(refused(LedgerError::PreparationSpent))
        );
        assert_eq!(owner.status(), reserved);
        let active = owner.ledger.active.as_ref().ok_or(Failure::Control)?;
        assert_eq!(active.uid, None);
        assert!(active.ready.is_none());
        owner.begin_preparation(&ticket, false)?;
        owner.finish_preparation(&public_entropy)?;
        assert_eq!(owner.status().phase, RequestPhase::Prepared);
        assert_eq!(owner.status().uids, 1);
        assert_eq!(owner.status().attempts, reserved.attempts);
        assert_eq!(owner.status().frames, reserved.frames);
        assert_eq!(owner.status().bytes, reserved.bytes);
        assert_eq!(owner.status().used, reserved.used);
        let active = owner.ledger.active.as_ref().ok_or(Failure::Control)?;
        assert_eq!(active.uid, Some(0));
        assert!(active.ready.is_some());
        owner.return_unused(ticket)?;
        Ok(())
    })
}

/// A genuine Preparing request cannot encode before its UID is retired.
/// # Errors
/// Preserve real owner construction, admission and the valid UID-retired neighbor.
/// # Panics
/// Fail if missing UID custody yields a staged packet or changes any allowance.
#[test]
fn preparing_request_cannot_stage_before_uid_retirement() -> Result<(), Failure> {
    with_owner(cookies(), |owner, _| {
        let ticket = owner.reserve(packet_input()?)?;
        owner.begin_preparation(&ticket, false)?;
        let preparing = owner.status();
        let mut public_entropy = [0; 48];
        entropy(&mut public_entropy);
        assert!(matches!(
            owner.stage_ready(&public_entropy),
            Err(Failure::Ledger(LedgerError::Phase))
        ));
        assert_eq!(owner.status(), preparing);
        let active = owner.ledger.active.as_ref().ok_or(Failure::Control)?;
        assert_eq!(active.uid, None);
        assert!(active.ready.is_none());
        owner.finish_preparation(&public_entropy)?;
        assert_eq!(owner.status().phase, RequestPhase::Prepared);
        assert_eq!(owner.status().uids, 1);
        assert_eq!(owner.status().attempts, preparing.attempts);
        assert_eq!(owner.status().frames, preparing.frames);
        assert_eq!(owner.status().bytes, preparing.bytes);
        assert_eq!(owner.status().used, preparing.used);
        let active = owner.ledger.active.as_ref().ok_or(Failure::Control)?;
        assert_eq!(active.uid, Some(0));
        assert!(active.ready.is_some());
        owner.return_unused(ticket)?;
        Ok(())
    })
}

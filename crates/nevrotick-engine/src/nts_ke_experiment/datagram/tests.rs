// SPDX-License-Identifier: GPL-2.0-only
//! Public-material deterministic cases, with no UDP/TLS/native qualification.

#[path = "../recovery/tests.rs"]
pub(in crate::nts_ke_experiment) mod recovery_tests;

use std::collections::VecDeque;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use nevrotick_s115_aes_siv_fixture::staged::StagedDecryptError;
use zeroize::Zeroizing;

use super::DatagramError;
use super::reached::{
    Backend, Interest, Io, Read, check_capture_order, classify_read, exchange_with, operation,
};
use crate::nts_ke_experiment::association::ledger::{
    LedgerError, Receipt, RequestInput, RequestPhase, SendOutcome,
};
use crate::nts_ke_experiment::association::{AssociationCandidate, AssociationOwner};
use crate::nts_ke_experiment::process::{ProcessAdmission, ProviderScope};
use crate::nts_ke_experiment::profile::StrictProfile;
use crate::nts_ke_experiment::request::DirectionalKeys;
use crate::nts_ke_experiment::response::{ReplyError, fixtures};
use crate::nts_ke_experiment::{
    CancelOutcome, Config, Control, Failure, IoPhase, StopDisposition, StopHandle, preview_once,
    protected_once,
};

/// Public independent request/UID bytes, not randomness or exporter evidence.
const C2S: [u8; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31,
];
/// Public response key used by the maintained independent packet fixtures.
const S2C: [u8; 32] = [
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f,
    0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f,
];
/// The independently generated packet's wire correlation, unrelated to local UTC.
const WIRE: u64 = 0x1122_3344_5566_7788;

/// A closed finite readiness response; it cannot run an arbitrary callback.
pub(super) enum ScriptReady {
    /// A ready event may still be followed by a `WouldBlock` return.
    Ready,
    /// Cancellation happens outside the final syscall guard.
    Cancel,
    /// Park until the independently driven control wake/deadline.
    Pending,
    /// A same-scope sibling closes only its own phase after this wait registered.
    SiblingCancelPending(Arc<Control>),
    /// Readiness itself failed, without consuming any datagram.
    Failed(io::ErrorKind),
}

/// Fixed syscall outcomes; no callback executes inside the control mutex.
#[derive(Clone, Copy)]
enum ScriptSend {
    Full,
    /// Complete the send, then inject a synthetic UTC step into its after capture.
    AfterWallStep,
    Short,
    Failed(io::ErrorKind),
    Unknown,
    CancelBeforeGuard,
    ScopeStopBeforeGuard,
    CancelAfterFull,
}

/// One selected public payload or a non-payload native return.
enum ScriptRead {
    Packet(&'static [u8], SocketAddr),
    WouldBlock,
    UnknownExtent,
    Failed(io::ErrorKind),
}

/// Bounded scripted facts. This type is available only in unit-test builds.
pub(super) struct Script {
    /// Actual component backend lifetime, separate from retained socket identity.
    lifetime: Arc<()>,
    ready: VecDeque<ScriptReady>,
    reads: VecDeque<ScriptRead>,
    send: ScriptSend,
    send_calls: usize,
    receive_calls: usize,
    packet: Zeroizing<[u8; 4096]>,
    packet_bytes: usize,
}

impl Script {
    /// Create a finite public case before entering the guarded operation.
    fn new(send: ScriptSend, reads: Vec<ScriptRead>) -> Self {
        Self {
            lifetime: Arc::new(()),
            ready: VecDeque::new(),
            reads: reads.into(),
            send,
            send_calls: 0,
            receive_calls: 0,
            packet: Zeroizing::new([0; 4096]),
            packet_bytes: 0,
        }
    }

    /// Latch the exact archive-to-syscall race outside the cancellation mutex.
    /// # Errors
    /// Preserve the actual cancellation failure rather than pretending to send.
    /// # Panics
    /// Fail if the selected first invocation stop had already been delivered.
    pub(super) fn before_guard(&mut self, control: &Control) -> Result<(), Failure> {
        if matches!(self.send, ScriptSend::CancelBeforeGuard) {
            control.cancel()?;
        }
        if matches!(self.send, ScriptSend::ScopeStopBeforeGuard) {
            let stop = StopHandle {
                scope: Arc::clone(&control.stop_scope),
            };
            assert_eq!(stop.request_stop()?, StopDisposition::Latched);
        }
        Ok(())
    }

    /// Supply a fixed ready/error/control event without performing any I/O.
    /// # Errors
    /// Preserve the selected closed readiness/control failure.
    pub(super) async fn ready(
        &mut self,
        _interest: Interest,
        control: &Control,
    ) -> Result<(), Failure> {
        match self.ready.pop_front().unwrap_or(ScriptReady::Ready) {
            ScriptReady::Ready => Ok(()),
            ScriptReady::Cancel => {
                control.cancel()?;
                Ok(())
            }
            ScriptReady::Pending => std::future::pending().await,
            ScriptReady::SiblingCancelPending(sibling) => {
                sibling.cancel()?;
                std::future::pending().await
            }
            ScriptReady::Failed(kind) => Err(Failure::Datagram(DatagramError::Readiness(kind))),
        }
    }

    /// Return fixed facts; the only copy is into preallocated bounded storage.
    pub(super) fn send(&mut self, packet: &[u8]) -> (SendOutcome, bool) {
        self.send_calls += 1;
        let Some(destination) = self.packet.get_mut(..packet.len()) else {
            return (SendOutcome::Failed(io::ErrorKind::InvalidInput), false);
        };
        destination.copy_from_slice(packet);
        self.packet_bytes = packet.len();
        match self.send {
            ScriptSend::Full
            | ScriptSend::AfterWallStep
            | ScriptSend::CancelBeforeGuard
            | ScriptSend::ScopeStopBeforeGuard => (SendOutcome::Completed(packet.len()), false),
            ScriptSend::Short => (
                SendOutcome::Completed(packet.len().saturating_sub(1)),
                false,
            ),
            ScriptSend::Failed(kind) => (SendOutcome::Failed(kind), false),
            ScriptSend::Unknown => (SendOutcome::Unknown, false),
            ScriptSend::CancelAfterFull => (SendOutcome::Completed(packet.len()), true),
        }
    }

    /// Inject only the closed UTC fault; retain the actual captured monotonic bracket.
    /// This changes neither the host clock nor syscall, receipt or join ownership.
    /// # Errors
    /// Refuse an unrepresentable synthetic UTC coordinate rather than hiding the fault.
    pub(super) fn observe_send_after(
        &self,
        captured: crate::SoftwareClock,
    ) -> Result<crate::SoftwareClock, Failure> {
        if matches!(self.send, ScriptSend::AfterWallStep) {
            let utc = captured
                .utc()
                .checked_add(Duration::from_secs(60))
                .ok_or(Failure::Control)?;
            Ok(crate::SoftwareClock { utc, ..captured })
        } else {
            Ok(captured)
        }
    }

    /// Copy one complete public datagram, never a usable truncated prefix.
    pub(super) fn receive(&mut self, buffer: &mut [u8; 4097]) -> Read {
        self.receive_calls += 1;
        match self.reads.pop_front() {
            Some(ScriptRead::Packet(packet, remote)) if packet.len() < buffer.len() => {
                let Some(destination) = buffer.get_mut(..packet.len()) else {
                    return Read::Failed(io::ErrorKind::InvalidInput);
                };
                destination.copy_from_slice(packet);
                Read::Datagram(packet.len(), remote)
            }
            Some(ScriptRead::Packet(_, _) | ScriptRead::UnknownExtent) => Read::ConsumedUnknown,
            Some(ScriptRead::WouldBlock) => Read::WouldBlock,
            Some(ScriptRead::Failed(kind)) => Read::Failed(kind),
            None => Read::Failed(io::ErrorKind::UnexpectedEof),
        }
    }
}

/// Supply the known wire/UID/nonce batch once to the actual preparation owner.
/// # Errors
/// The maintained callback signature permits refusal; this fixed batch succeeds.
#[expect(
    clippy::unnecessary_wraps,
    reason = "The actual entropy consumer requires a fallible callback; this public fixed batch is its healthy test neighbor."
)]
fn entropy(output: &mut [u8; 56]) -> Result<(), Failure> {
    output[..8].copy_from_slice(&WIRE.to_be_bytes());
    output[8..40].copy_from_slice(&C2S);
    output[40..].fill(0x66);
    Ok(())
}

/// One private current-thread driver, without sockets, jobs or paused-time features.
/// # Errors
/// Preserve the actual runtime setup error category.
fn runtime() -> Result<tokio::runtime::Runtime, Failure> {
    tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .max_blocking_threads(2)
        .build()
        .map_err(|error| Failure::Datagram(DatagramError::Runtime(error.kind())))
}

/// Construct the actual private checked receiving owner with public component bytes.
/// # Errors
/// Preserve checked construction, publication and the exercised consumer failure.
fn with_owner(
    response_key: [u8; 32],
    exercise: impl FnOnce(&mut AssociationOwner<'_>, &Control, SocketAddr) -> Result<(), Failure>,
) -> Result<(), Failure> {
    with_owner_start(Instant::now(), response_key, exercise)
}

/// Retain the genuine original-origin deadline while varying only its age.
/// # Errors
/// Preserve original deadline construction, publication or exercised refusal.
fn with_owner_start(
    start: Instant,
    response_key: [u8; 32],
    exercise: impl FnOnce(&mut AssociationOwner<'_>, &Control, SocketAddr) -> Result<(), Failure>,
) -> Result<(), Failure> {
    let config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    let control = Control::new(start)?;
    let admission = ProcessAdmission::new();
    let claim = admission.claim()?;
    let provider = ProviderScope::component(claim);
    let remote = SocketAddr::new(config.peer.ip(), 123);
    let candidate = AssociationCandidate::new(
        &config,
        &control,
        &provider,
        remote,
        StrictProfile::component(),
        DirectionalKeys::new(Zeroizing::new(C2S), Zeroizing::new(response_key)),
        vec![Zeroizing::new(vec![0x11; 12])],
    );
    let mut owner = AssociationOwner::receive(&config, &control, &provider, candidate)?;
    exercise(&mut owner, &control, remote)
}

/// Keep script socket identity separate from the actual association identity.
fn backend(remote: SocketAddr, script: Script) -> Backend {
    Backend {
        io: Io::Script(Box::new(script)),
        local: SocketAddr::new(remote.ip(), 49000),
        remote,
        identity: Arc::new(()),
        send: None,
    }
}

/// Inspect only this test build's closed backend.
/// # Errors
/// Refuse accidental native ownership instead of panicking or touching its socket.
#[cfg_attr(
    not(windows),
    expect(
        clippy::unnecessary_wraps,
        reason = "Windows has a fallible native-backend branch; GNU component builds contain only the scripted backend."
    )
)]
fn script(backend: &Backend) -> Result<&Script, Failure> {
    match &backend.io {
        Io::Script(script) => Ok(script),
        #[cfg(windows)]
        Io::Native(_) => Err(Failure::Control),
    }
}

/// A genuine fixed-tag neighbor retains header/captures/identity and completes once.
/// # Errors
/// Preserve actual component construction, packet and response refusal.
/// # Panics
/// Fail if any raw coordinate, complete-header bit, credit or capture is fabricated.
#[test]
fn healthy_operation_retains_full_header_and_distinct_capture_provenance() -> Result<(), Failure> {
    use std::cell::Cell;

    with_owner(S2C, |owner, control, remote| {
        let retained = Arc::new(());
        let task_lifetime = Arc::downgrade(&retained);
        let mut pending = None;
        let mut socket_lifetime = None;
        let completion = exchange_with(owner, control, entropy, |admitted| {
            assert_eq!(admitted, remote);
            let backend = backend(
                admitted,
                Script::new(
                    ScriptSend::Full,
                    vec![ScriptRead::Packet(fixtures::HEALTHY, admitted)],
                ),
            );
            socket_lifetime = Some(Arc::downgrade(&backend.identity));
            pending = Some(tokio::spawn(async move {
                std::future::pending::<()>().await;
                drop(retained);
            }));
            Ok(backend)
        })?;
        let history = completion.history();
        let fixture_header = fixtures::HEALTHY.get(..48).ok_or(Failure::Control)?;
        let expected =
            ntp_wire::decode_fixed_header(fixture_header).map_err(|_| Failure::Control)?;
        let header = history.receipt.fields().header();
        assert_eq!(header.root_delay_bits(), expected.root_delay_bits());
        assert_eq!(
            header.root_dispersion_bits(),
            expected.root_dispersion_bits()
        );
        assert_eq!(header.reference_id(), expected.reference_id());
        assert_eq!(
            header.reference_timestamp().bits(),
            expected.reference_timestamp().bits()
        );
        assert_eq!(header.origin_timestamp().bits(), WIRE);
        assert_eq!(header.poll(), expected.poll());
        assert_eq!(header.precision(), expected.precision());
        assert!(history.receipt.local_t1().is_none());
        assert!(history.send_before.monotonic_after() <= history.send_after.monotonic());
        assert!(history.send_after.monotonic_after() <= history.read_return.monotonic());
        assert_eq!(history.local, SocketAddr::new(remote.ip(), 49000));
        assert_eq!(history.remote, remote);
        assert_eq!(owner.status().frames, 31);
        assert_eq!(owner.status().bytes, 131_072 - fixtures::HEALTHY.len());
        assert_eq!(owner.status().used, 1);
        let socket_lifetime = socket_lifetime.ok_or(Failure::Control)?;
        assert!(Arc::ptr_eq(
            &history.identity,
            &socket_lifetime.upgrade().ok_or(Failure::Control)?
        ));
        assert_eq!(socket_lifetime.strong_count(), 1);
        assert_eq!(task_lifetime.strong_count(), 0);
        // The wrapper, not the observer, retired its backend and runtime task.
        drop(pending);
        let committed = completion.commit(owner)?;
        let summary = committed.into_summary();
        assert!(summary.transmitted);
        assert_eq!((summary.cookies, summary.cookie_bytes), (1, 12));
        assert_eq!(socket_lifetime.strong_count(), 0);
        assert_eq!(control.cancel()?, CancelOutcome::AlreadyCommitted);
        let before = owner.status();
        let expiry = control.expiry;
        let receipt = owner.last_receipt().map(Receipt::serial);
        let backend_calls = Cell::new(0);
        let entropy_calls = Cell::new(0);
        assert!(matches!(
            exchange_with(
                owner,
                control,
                |output| {
                    entropy_calls.set(entropy_calls.get() + 1);
                    entropy(output)
                },
                |admitted| {
                    backend_calls.set(backend_calls.get() + 1);
                    Ok(backend(admitted, Script::new(ScriptSend::Full, Vec::new())))
                }
            ),
            Err(Failure::Control)
        ));
        assert_eq!((backend_calls.get(), entropy_calls.get()), (0, 0));
        assert_eq!(owner.status(), before);
        assert_eq!(owner.last_receipt().map(Receipt::serial), receipt);
        assert_eq!(control.expiry, expiry);
        assert_eq!(control.cancel()?, CancelOutcome::AlreadyCommitted);
        Ok(())
    })
}

/// An entered runtime refuses the synchronous wrapper before any setup.
/// # Errors
/// Preserve checked owner/runtime construction and unexpected currentness refusal.
/// # Panics
/// Fail if refusal enters backend/entropy, changes ledger history or commits control.
#[test]
fn wrapper_nested_runtime_refusal_precedes_backend_and_entropy() -> Result<(), Failure> {
    use std::cell::Cell;

    with_owner(S2C, |owner, control, _remote| {
        let before = owner.status();
        let expiry = control.expiry;
        let backend_calls = Cell::new(0);
        let entropy_calls = Cell::new(0);
        let driver = runtime()?;
        let result = driver.block_on(async {
            exchange_with(
                owner,
                control,
                |output| {
                    entropy_calls.set(entropy_calls.get() + 1);
                    entropy(output)
                },
                |admitted| {
                    backend_calls.set(backend_calls.get() + 1);
                    Ok(backend(admitted, Script::new(ScriptSend::Full, Vec::new())))
                },
            )
        });
        assert!(matches!(
            result,
            Err(Failure::Datagram(DatagramError::EnteredRuntime))
        ));
        drop(driver);
        assert_eq!((backend_calls.get(), entropy_calls.get()), (0, 0));
        assert_eq!(owner.status(), before);
        assert!(owner.last_receipt().is_none());
        assert_eq!(control.expiry, expiry);
        control.remaining(IoPhase::Exchange)?;
        Ok(())
    })
}

/// Foreign and cancelled controls refuse the wrapper before any backend setup.
/// # Errors
/// Preserve checked owner construction and unexpected currentness refusal.
/// # Panics
/// Fail if refusal enters backend/entropy, changes ledger history or either Control.
#[test]
fn wrapper_context_refusals_precede_backend_and_entropy() -> Result<(), Failure> {
    use std::cell::Cell;

    with_owner(S2C, |owner, control, _remote| {
        with_owner(S2C, |_foreign, foreign_control, _foreign_remote| {
            let before = owner.status();
            let expiry = control.expiry;
            let foreign_expiry = foreign_control.expiry;
            let backend_calls = Cell::new(0);
            let entropy_calls = Cell::new(0);
            assert!(matches!(
                exchange_with(
                    owner,
                    foreign_control,
                    |output| {
                        entropy_calls.set(entropy_calls.get() + 1);
                        entropy(output)
                    },
                    |admitted| {
                        backend_calls.set(backend_calls.get() + 1);
                        Ok(backend(admitted, Script::new(ScriptSend::Full, Vec::new())))
                    }
                ),
                Err(Failure::Control)
            ));
            assert_eq!((backend_calls.get(), entropy_calls.get()), (0, 0));
            assert_eq!(owner.status(), before);
            assert!(owner.last_receipt().is_none());
            assert_eq!(
                (control.expiry, foreign_control.expiry),
                (expiry, foreign_expiry)
            );
            control.remaining(IoPhase::Exchange)?;
            foreign_control.remaining(IoPhase::Exchange)?;
            Ok(())
        })
    })?;
    with_owner(S2C, |owner, control, _remote| {
        assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
        let before = owner.status();
        let expiry = control.expiry;
        let backend_calls = Cell::new(0);
        let entropy_calls = Cell::new(0);
        assert!(matches!(
            exchange_with(
                owner,
                control,
                |output| {
                    entropy_calls.set(entropy_calls.get() + 1);
                    entropy(output)
                },
                |admitted| {
                    backend_calls.set(backend_calls.get() + 1);
                    Ok(backend(admitted, Script::new(ScriptSend::Full, Vec::new())))
                }
            ),
            Err(Failure::Cancelled)
        ));
        assert_eq!((backend_calls.get(), entropy_calls.get()), (0, 0));
        assert_eq!(owner.status(), before);
        assert!(owner.last_receipt().is_none());
        assert_eq!(control.expiry, expiry);
        assert_eq!(control.cancel()?, CancelOutcome::AlreadyCancelled);
        Ok(())
    })
}

/// A refused backend factory retires its actual driver without entering entropy.
/// This is not an injected `Runtime::Builder` failure or native socket observation.
/// # Errors
/// Preserve checked owner setup and unexpected final currentness refusal.
/// # Panics
/// Fail if setup changes category/state or its runtime-owned capture survives return.
#[test]
fn wrapper_setup_refusal_retires_runtime_before_return() -> Result<(), Failure> {
    use std::cell::Cell;

    with_owner(S2C, |owner, control, remote| {
        let before = owner.status();
        let expiry = control.expiry;
        let backend_calls = Cell::new(0);
        let entropy_calls = Cell::new(0);
        let retained = Arc::new(());
        let task_lifetime = Arc::downgrade(&retained);
        let mut pending = None;
        let result = exchange_with(
            owner,
            control,
            |output| {
                entropy_calls.set(entropy_calls.get() + 1);
                entropy(output)
            },
            |admitted| {
                assert_eq!(admitted, remote);
                assert!(tokio::runtime::Handle::try_current().is_ok());
                backend_calls.set(backend_calls.get() + 1);
                pending = Some(tokio::spawn(async move {
                    std::future::pending::<()>().await;
                    drop(retained);
                }));
                Err(Failure::Endpoint)
            },
        );
        assert!(matches!(result, Err(Failure::Endpoint)));
        assert_eq!((backend_calls.get(), entropy_calls.get()), (1, 0));
        assert_eq!(task_lifetime.strong_count(), 0);
        drop(pending);
        assert_eq!(owner.status(), before);
        assert!(owner.last_receipt().is_none());
        assert_eq!(control.expiry, expiry);
        assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
        Ok(())
    })
}

/// A historical completion cannot finalize over a newer charged reservation.
/// This challenges the new private receiving contract, not current CLI behavior.
/// # Errors
/// Preserve genuine owner, packet, runtime, reservation or currentness refusal.
/// # Panics
/// Fail if stale completion commits, alters the newer slot or refunds its work.
#[test]
fn stale_completion_cannot_commit_over_newer_reservation() -> Result<(), Failure> {
    with_owner(S2C, |owner, control, remote| {
        let completion = exchange_with(owner, control, entropy, |admitted| {
            Ok(backend(
                admitted,
                Script::new(
                    ScriptSend::Full,
                    vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
                ),
            ))
        })?;
        let serial = completion.history().receipt.serial();
        let _ticket = owner.reserve(RequestInput::Observed)?;
        let reserved = owner.status();
        assert_eq!(reserved.phase, RequestPhase::Reserved);
        assert_eq!(reserved.attempts, 30);
        assert_eq!(owner.last_receipt().map(Receipt::serial), Some(serial));
        control.remaining(IoPhase::Exchange)?;
        assert!(
            matches!(
                completion.commit(owner),
                Err(Failure::Ledger(LedgerError::Ticket))
            ),
            "stale completion committed over a newer reservation"
        );
        assert_eq!(owner.status(), reserved);
        assert_eq!(owner.last_receipt().map(Receipt::serial), Some(serial));
        assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
        Ok(())
    })
}

/// Returning a newer reservation cannot make the older completion current again.
/// The original last receipt remains; the charged attempt must still stale it.
/// # Errors
/// Preserve actual owner, packet, runtime, explicit-return or currentness refusal.
/// # Panics
/// Fail if last-receipt-only validation accepts stale completion or restores work.
#[test]
fn stale_completion_cannot_commit_after_newer_reservation_return() -> Result<(), Failure> {
    with_owner(S2C, |owner, control, remote| {
        let completion = exchange_with(owner, control, entropy, |admitted| {
            Ok(backend(
                admitted,
                Script::new(
                    ScriptSend::Full,
                    vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
                ),
            ))
        })?;
        let serial = completion.history().receipt.serial();
        let ticket = owner.reserve(RequestInput::Observed)?;
        owner.return_unused(ticket)?;
        let returned = owner.status();
        assert_eq!(returned.phase, RequestPhase::Idle);
        assert_eq!(returned.attempts, 30);
        assert_eq!(owner.last_receipt().map(Receipt::serial), Some(serial));
        control.remaining(IoPhase::Exchange)?;
        assert!(
            matches!(
                completion.commit(owner),
                Err(Failure::Ledger(LedgerError::Ticket))
            ),
            "returned reservation revived stale completion"
        );
        assert_eq!(owner.status(), returned);
        assert_eq!(owner.last_receipt().map(Receipt::serial), Some(serial));
        assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
        Ok(())
    })
}

/// Rejected authenticated packets preserve the last permitted healthy neighbor.
/// # Errors
/// Preserve genuine setup, preparation, opening or completion failure.
/// # Panics
/// Fail if rejection refunds processing credit, renews expiry or hides a real reply.
#[test]
fn thirty_one_zero_cookie_refusals_preserve_final_healthy_packet() -> Result<(), Failure> {
    with_owner(S2C, |owner, control, remote| {
        let expiry = control.expiry;
        let mut reads: Vec<_> = (0..31)
            .map(|_| ScriptRead::Packet(fixtures::ZERO_COOKIES, remote))
            .collect();
        reads.push(ScriptRead::Packet(fixtures::HEALTHY, remote));
        let mut backend = backend(remote, Script::new(ScriptSend::Full, reads));
        let completion = runtime()?.block_on(operation(owner, control, entropy, &mut backend))?;
        assert_eq!(script(&backend)?.receive_calls, 32);
        assert_eq!(owner.status().frames, 0);
        assert_eq!(
            owner.status().bytes,
            131_072 - 31 * fixtures::ZERO_COOKIES.len() - fixtures::HEALTHY.len()
        );
        assert_eq!(control.expiry, expiry);
        assert_eq!(completion.receipt.cookie_counts(), (1, 1));
        Ok(())
    })
}

/// Endpoint/UID refusals and spurious readiness do not consume completion.
/// # Errors
/// Preserve valid-neighbor failure through the actual receiver.
/// # Panics
/// Fail on refunded bytes, a changed endpoint or a second send.
#[test]
fn false_readiness_wrong_endpoint_and_uid_remain_bounded() -> Result<(), Failure> {
    with_owner(S2C, |owner, control, remote| {
        let wrong = SocketAddr::new(remote.ip(), 124);
        let reads = vec![
            ScriptRead::WouldBlock,
            ScriptRead::Packet(fixtures::HEALTHY, wrong),
            ScriptRead::Packet(fixtures::WRONG_UID, remote),
            ScriptRead::Packet(fixtures::HEALTHY, remote),
        ];
        let mut backend = backend(remote, Script::new(ScriptSend::Full, reads));
        runtime()?.block_on(operation(owner, control, entropy, &mut backend))?;
        assert_eq!(script(&backend)?.send_calls, 1);
        assert_eq!(script(&backend)?.receive_calls, 4);
        assert_eq!(owner.status().frames, 29);
        assert_eq!(
            owner.status().bytes,
            131_072 - 2 * fixtures::HEALTHY.len() - fixtures::WRONG_UID.len()
        );
        Ok(())
    })
}

/// Every invoked non-full outcome burns the cookie and never reaches receiving.
/// # Errors
/// Preserve only unexpected fixture setup failure.
/// # Panics
/// Fail if `WouldBlock` rearms a send or an unknown result falsely releases custody.
#[test]
fn short_failed_wouldblock_and_unknown_sends_never_reissue() -> Result<(), Failure> {
    for outcome in [
        ScriptSend::Short,
        ScriptSend::Failed(io::ErrorKind::WouldBlock),
        ScriptSend::Failed(io::ErrorKind::PermissionDenied),
        ScriptSend::Unknown,
    ] {
        with_owner(S2C, |owner, control, remote| {
            let mut backend = backend(remote, Script::new(outcome, Vec::new()));
            assert!(
                runtime()?
                    .block_on(operation(owner, control, entropy, &mut backend))
                    .is_err()
            );
            assert_eq!(script(&backend)?.send_calls, 1);
            assert_eq!(script(&backend)?.receive_calls, 0);
            assert_eq!(owner.status().used, 1);
            assert_eq!(owner.status().available, 0);
            assert_eq!(
                owner.status().phase,
                if matches!(outcome, ScriptSend::Unknown) {
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

/// Cancellation before versus after concrete exposure keeps truthful custody.
/// # Errors
/// Preserve unexpected component construction failure.
/// # Panics
/// Fail if pre-invocation cancellation submits, or late cancellation publishes.
#[test]
fn cancellation_before_and_after_exposure_preserves_send_facts() -> Result<(), Failure> {
    for after in [false, true] {
        with_owner(S2C, |owner, control, remote| {
            let mut selected = Script::new(
                if after {
                    ScriptSend::CancelAfterFull
                } else {
                    ScriptSend::Full
                },
                Vec::new(),
            );
            if !after {
                selected.ready.push_back(ScriptReady::Cancel);
            }
            let mut backend = backend(remote, selected);
            assert!(matches!(
                runtime()?.block_on(operation(owner, control, entropy, &mut backend)),
                Err(Failure::Cancelled)
            ));
            assert_eq!(script(&backend)?.send_calls, usize::from(after));
            assert_eq!(owner.status().used, usize::from(after));
            assert!(owner.last_receipt().is_none());
            Ok(())
        })?;
    }
    Ok(())
}

/// Stop after archival still wins the final concrete syscall guard.
/// # Errors
/// Preserve unexpected component setup failure.
/// # Panics
/// Fail if the stale adapter submits, publishes or hides the original cancellation.
#[test]
fn cancellation_after_archive_before_syscall_blocks_actual_submission() -> Result<(), Failure> {
    with_owner(S2C, |owner, control, remote| {
        let mut backend = backend(
            remote,
            Script::new(ScriptSend::CancelBeforeGuard, Vec::new()),
        );
        assert!(matches!(
            runtime()?.block_on(operation(owner, control, entropy, &mut backend)),
            Err(Failure::Cancelled)
        ));
        assert_eq!(owner.status().used, 1);
        assert_eq!(script(&backend)?.send_calls, 0);
        assert_eq!(script(&backend)?.receive_calls, 0);
        assert_eq!(owner.status().phase, RequestPhase::Invoked);
        assert!(owner.last_receipt().is_none());
        Ok(())
    })
}

/// Stop inside real preparation entropy cannot publish a packet or refund work.
/// # Errors
/// Preserve checked owner/runtime setup and the healthy response's actual refusal.
/// # Panics
/// Fail if a callback holds either gate, stop permits an effect or work is renewed.
#[test]
fn invocation_stop_inside_protected_entropy_preserves_charges_without_send() -> Result<(), Failure>
{
    use std::cell::Cell;

    for stop_in_entropy in [false, true] {
        with_owner(S2C, |owner, control, remote| {
            let stop = StopHandle {
                scope: Arc::clone(&control.stop_scope),
            };
            let expiry = control.expiry;
            let original = owner.status();
            let entropy_calls = Cell::new(0);
            let mut backend = backend(
                remote,
                Script::new(
                    ScriptSend::Full,
                    vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
                ),
            );
            let driver = runtime()?;
            let result = driver.block_on(operation(
                owner,
                control,
                |output| {
                    entropy_calls.set(entropy_calls.get() + 1);
                    // Nonblocking observations make a wrongly retained guard fail
                    // before a synchronous stop request could wait on that guard.
                    assert!(control.stop_scope.stopped.try_lock().is_ok());
                    assert!(control.phase.try_lock().is_ok());
                    entropy(output)?;
                    if stop_in_entropy {
                        assert_eq!(stop.request_stop()?, StopDisposition::Latched);
                    }
                    Ok(())
                },
                &mut backend,
            ));
            assert_eq!(entropy_calls.get(), 1);
            assert_eq!(control.expiry, expiry);
            assert_eq!(owner.status().attempts, original.attempts - 1);
            assert_eq!(owner.status().uids, 1);
            if stop_in_entropy {
                assert!(matches!(result, Err(Failure::Cancelled)));
                let stopped = owner.status();
                assert_eq!(stopped.phase, RequestPhase::Preparing);
                assert_eq!((stopped.available, stopped.used), (0, 0));
                assert_eq!(
                    (stopped.frames, stopped.bytes),
                    (original.frames, original.bytes)
                );
                assert!(owner.last_receipt().is_none());
                assert_eq!(stop.request_stop()?, StopDisposition::AlreadyLatched);
                let repeated = driver.block_on(operation(
                    owner,
                    control,
                    |output| {
                        entropy_calls.set(entropy_calls.get() + 1);
                        entropy(output)
                    },
                    &mut backend,
                ));
                assert!(matches!(repeated, Err(Failure::Cancelled)));
                assert_eq!(entropy_calls.get(), 1);
                assert_eq!(owner.status(), stopped);
                assert_eq!(script(&backend)?.send_calls, 0);
                assert_eq!(script(&backend)?.receive_calls, 0);
            } else {
                let completion = result?;
                assert_eq!(completion.receipt.serial(), 1);
                assert_eq!(script(&backend)?.send_calls, 1);
                assert_eq!(script(&backend)?.receive_calls, 1);
                assert_eq!(owner.status().used, 1);
            }
            Ok(())
        })?;
    }
    Ok(())
}

/// KE-STOP-06: invocation stop wins after archive and before the actual send guard.
/// # Errors
/// Preserve checked setup and the healthy neighboring packet's real refusal.
/// # Panics
/// Fail if stop permits a send or refunds the archived cookie/request charge.
#[test]
fn invocation_stop_before_syscall_preserves_archive_and_blocks_send() -> Result<(), Failure> {
    for stop_first in [false, true] {
        with_owner(S2C, |owner, control, remote| {
            let mut backend = backend(
                remote,
                Script::new(
                    if stop_first {
                        ScriptSend::ScopeStopBeforeGuard
                    } else {
                        ScriptSend::Full
                    },
                    vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
                ),
            );
            let result = runtime()?.block_on(operation(owner, control, entropy, &mut backend));
            if stop_first {
                assert!(matches!(result, Err(Failure::Cancelled)));
                assert_eq!(script(&backend)?.send_calls, 0);
                assert_eq!(script(&backend)?.receive_calls, 0);
                assert_eq!(owner.status().phase, RequestPhase::Invoked);
                assert!(owner.last_receipt().is_none());
            } else {
                let completion = result?;
                assert_eq!(completion.receipt.serial(), 1);
                assert_eq!(script(&backend)?.send_calls, 1);
                assert_eq!(script(&backend)?.receive_calls, 1);
            }
            assert_eq!(owner.status().attempts, 31);
            assert_eq!(owner.status().used, 1);
            Ok(())
        })?;
    }
    Ok(())
}

/// Consumed unknown extents cannot be parsed or charged as fabricated prefixes.
/// # Errors
/// Preserve only unexpected setup failure.
/// # Panics
/// Fail if any original byte allowance survives unknown-extent consumption.
#[test]
fn consumed_unknown_extent_exhausts_bytes_and_retires_pending() -> Result<(), Failure> {
    with_owner(S2C, |owner, control, remote| {
        let mut backend = backend(
            remote,
            Script::new(ScriptSend::Full, vec![ScriptRead::UnknownExtent]),
        );
        assert!(matches!(
            runtime()?.block_on(operation(owner, control, entropy, &mut backend)),
            Err(Failure::Ledger(LedgerError::ReceiveWork))
        ));
        assert_eq!(owner.status().frames, 31);
        assert_eq!(owner.status().bytes, 0);
        assert_eq!(owner.status().phase, RequestPhase::Idle);
        assert!(owner.last_receipt().is_none());
        Ok(())
    })
}

/// A ready source that never supplies a datagram cannot spin indefinitely.
/// # Errors
/// Preserve only unexpected setup failure.
/// # Panics
/// Fail if spurious readiness consumes datagram credit or exceeds the shared cap.
#[test]
fn shared_attempt_limit_stops_spurious_readiness_without_refund() -> Result<(), Failure> {
    with_owner(S2C, |owner, control, remote| {
        let reads = (0..64).map(|_| ScriptRead::WouldBlock).collect();
        let mut backend = backend(remote, Script::new(ScriptSend::Full, reads));
        assert!(matches!(
            runtime()?.block_on(operation(owner, control, entropy, &mut backend)),
            Err(Failure::Datagram(DatagramError::Attempts))
        ));
        assert_eq!(script(&backend)?.receive_calls, 31);
        assert_eq!((owner.status().frames, owner.status().bytes), (32, 131_072));
        assert_eq!(script(&backend)?.send_calls, 1);
        Ok(())
    })
}

/// Actual non-payload error categories do not become fictitious UDP messages.
/// # Errors
/// Preserve setup failures beside the selected I/O refusal.
/// # Panics
/// Fail if non-consumed failures charge a frame or overwrite their original kind.
#[test]
fn readiness_receive_and_wrong_key_failures_preserve_original_categories() -> Result<(), Failure> {
    with_owner(S2C, |owner, control, remote| {
        let mut selected = Script::new(ScriptSend::Full, Vec::new());
        selected
            .ready
            .push_back(ScriptReady::Failed(io::ErrorKind::PermissionDenied));
        let mut backend = backend(remote, selected);
        assert!(matches!(
            runtime()?.block_on(operation(owner, control, entropy, &mut backend)),
            Err(Failure::Datagram(DatagramError::Readiness(
                io::ErrorKind::PermissionDenied
            )))
        ));
        assert_eq!(script(&backend)?.send_calls, 0);
        Ok(())
    })?;
    with_owner(S2C, |owner, control, remote| {
        let mut backend = backend(
            remote,
            Script::new(
                ScriptSend::Full,
                vec![ScriptRead::Failed(io::ErrorKind::ConnectionReset)],
            ),
        );
        assert!(matches!(
            runtime()?.block_on(operation(owner, control, entropy, &mut backend)),
            Err(Failure::Datagram(DatagramError::Receive(
                io::ErrorKind::ConnectionReset
            )))
        ));
        assert_eq!(owner.status().frames, 32);
        Ok(())
    })?;
    with_owner([0x55; 32], |owner, control, remote| {
        let reads = (0..32)
            .map(|_| ScriptRead::Packet(fixtures::HEALTHY, remote))
            .collect();
        let mut backend = backend(remote, Script::new(ScriptSend::Full, reads));
        assert!(
            runtime()?
                .block_on(operation(owner, control, entropy, &mut backend))
                .is_err()
        );
        assert_eq!(owner.status().frames, 0);
        assert!(owner.last_receipt().is_none());
        Ok(())
    })
}

/// Cancellation wakes a registered pending wait without timeout polling.
/// # Errors
/// Preserve unexpected setup or cancellation failure.
/// # Panics
/// Fail if a latched stop is lost or the parked readiness branch submits bytes.
#[test]
fn cancellation_wakes_registered_pending_readiness() -> Result<(), Failure> {
    with_owner(S2C, |owner, control, remote| {
        let mut selected = Script::new(ScriptSend::Full, Vec::new());
        selected.ready.push_back(ScriptReady::Pending);
        let mut backend = backend(remote, selected);
        let result = runtime()?.block_on(async {
            let stopping = async {
                tokio::task::yield_now().await;
                control.cancel()
            };
            let (result, stopped) = tokio::time::timeout(Duration::from_secs(2), async {
                tokio::join!(operation(owner, control, entropy, &mut backend), stopping)
            })
            .await
            .map_err(|_| Failure::Control)?;
            stopped?;
            result
        });
        assert!(matches!(result, Err(Failure::Cancelled)));
        assert_eq!(script(&backend)?.send_calls, 0);
        Ok(())
    })
}

/// KE-STOP-05: the real invocation handle wakes an already parked operation.
/// # Errors
/// Preserve component setup, scope-stop or bounded watchdog refusal.
/// # Panics
/// Fail if registration is bypassed, stop is lost, or the parked operation sends.
#[test]
fn invocation_stop_wakes_registered_pending_readiness() -> Result<(), Failure> {
    use std::future::Future;
    use std::task::Poll;

    with_owner(S2C, |owner, control, remote| {
        let stop = StopHandle {
            scope: Arc::clone(&control.stop_scope),
        };
        let expiry = control.expiry;
        let mut selected = Script::new(ScriptSend::Full, Vec::new());
        selected.ready.push_back(ScriptReady::Pending);
        let mut backend = backend(remote, selected);
        let result = runtime()?.block_on(async {
            let pending = operation(owner, control, entropy, &mut backend);
            tokio::pin!(pending);
            std::future::poll_fn(|context| {
                assert!(pending.as_mut().poll(context).is_pending());
                Poll::Ready(())
            })
            .await;
            let stopping = async { stop.request_stop() };
            let (result, stopped) = tokio::time::timeout(Duration::from_secs(2), async {
                tokio::join!(pending, stopping)
            })
            .await
            .map_err(|_| Failure::Control)?;
            assert_eq!(stopped?, StopDisposition::Latched);
            result
        });
        assert!(matches!(result, Err(Failure::Cancelled)));
        assert_eq!(stop.request_stop()?, StopDisposition::AlreadyLatched);
        assert_eq!(control.expiry, expiry);
        assert_eq!(script(&backend)?.send_calls, 0);
        assert_eq!(script(&backend)?.receive_calls, 0);
        assert_eq!(owner.status().phase, RequestPhase::Prepared);
        assert_eq!(owner.status().attempts, 31);
        assert!(owner.last_receipt().is_none());
        Ok(())
    })
}

/// KE-STOP-05: a sibling's local cancellation is only a wake for a live owner.
/// # Errors
/// Preserve checked same-scope construction, actual packet or watchdog refusal.
/// # Panics
/// Fail if a harmless wake cancels current authority or prevents the healthy receipt.
#[test]
fn sibling_local_cancel_rewaits_and_preserves_healthy_receipt() -> Result<(), Failure> {
    with_owner(S2C, |owner, control, remote| {
        let sibling = Arc::new(Control::clamped_in(
            control.origin,
            control.expiry,
            Arc::clone(&control.stop_scope),
        )?);
        let mut selected = Script::new(
            ScriptSend::Full,
            vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
        );
        selected
            .ready
            .push_back(ScriptReady::SiblingCancelPending(Arc::clone(&sibling)));
        let mut backend = backend(remote, selected);
        let completion = runtime()?.block_on(async {
            tokio::time::timeout(
                Duration::from_secs(2),
                operation(owner, control, entropy, &mut backend),
            )
            .await
            .map_err(|_| Failure::Control)?
        })?;
        control.remaining(IoPhase::Exchange)?;
        assert_eq!(sibling.cancel()?, CancelOutcome::AlreadyCancelled);
        assert_eq!(completion.receipt.serial(), 1);
        assert_eq!(owner.last_receipt().map(Receipt::serial), Some(1));
        assert_eq!(script(&backend)?.send_calls, 1);
        assert_eq!(script(&backend)?.receive_calls, 1);
        assert!(script(&backend)?.ready.is_empty());
        assert_eq!(owner.status().frames, 31);
        assert_eq!(owner.status().used, 1);
        Ok(())
    })
}

/// KE-STOP-05: finite harmless wakes spend the unchanged readiness allowance.
/// # Errors
/// Preserve same-scope setup or the two-second watchdog refusal.
/// # Panics
/// Fail if wakes become cancellation, refill Attempts, submit bytes or loop forever.
#[test]
fn sibling_local_wakes_exhaust_existing_attempts_without_cancelling() -> Result<(), Failure> {
    with_owner(S2C, |owner, control, remote| {
        let sibling = Arc::new(Control::clamped_in(
            control.origin,
            control.expiry,
            Arc::clone(&control.stop_scope),
        )?);
        let mut selected = Script::new(ScriptSend::Full, Vec::new());
        selected.ready = (0..64)
            .map(|_| ScriptReady::SiblingCancelPending(Arc::clone(&sibling)))
            .collect();
        let mut backend = backend(remote, selected);
        let result = runtime()?.block_on(async {
            tokio::time::timeout(
                Duration::from_secs(2),
                operation(owner, control, entropy, &mut backend),
            )
            .await
            .map_err(|_| Failure::Control)?
        });
        assert!(matches!(
            result,
            Err(Failure::Datagram(DatagramError::Attempts))
        ));
        control.remaining(IoPhase::Exchange)?;
        assert_eq!(sibling.cancel()?, CancelOutcome::AlreadyCancelled);
        assert!(script(&backend)?.ready.is_empty());
        assert_eq!(script(&backend)?.send_calls, 0);
        assert_eq!(script(&backend)?.receive_calls, 0);
        assert_eq!(owner.status().phase, RequestPhase::Prepared);
        assert_eq!(owner.status().attempts, 31);
        assert_eq!((owner.status().frames, owner.status().bytes), (32, 131_072));
        assert!(owner.last_receipt().is_none());
        Ok(())
    })
}

/// The complete buffer sentinel is never passed to the packet reader.
/// # Panics
/// Fail if a boundary/error return is classified as usable truncated data.
#[test]
fn complete_receive_sentinel_and_platform_error_classification_are_closed() {
    let remote = SocketAddr::from(([127, 0, 0, 1], 123));
    assert!(
        matches!(classify_read(Ok((4096, remote))), Read::Datagram(4096, source) if source == remote)
    );
    assert!(matches!(
        classify_read(Ok((4097, remote))),
        Read::ConsumedUnknown
    ));
    assert!(matches!(
        classify_read(Err(io::ErrorKind::WouldBlock.into())),
        Read::WouldBlock
    ));
    let native = if cfg!(windows) { 10040 } else { 90 };
    assert!(matches!(
        classify_read(Err(io::Error::from_raw_os_error(native))),
        Read::ConsumedUnknown
    ));
    assert!(matches!(
        classify_read(Err(io::ErrorKind::PermissionDenied.into())),
        Read::Failed(io::ErrorKind::PermissionDenied)
    ));
    let foreign = if cfg!(windows) { 90 } else { 10040 };
    assert!(matches!(
        classify_read(Err(io::Error::from_raw_os_error(foreign))),
        Read::Failed(_)
    ));
}

/// One helper enforces both actual capture boundaries over its complete input domain.
/// # Errors
/// Preserve checked construction of the adjacent monotonic observation.
/// # Panics
/// Fail if equality is refused or any reversed observation is accepted.
#[test]
fn capture_order_has_inclusive_equal_and_adjacent_neighbors() -> Result<(), Failure> {
    let earlier = Instant::now();
    let later = earlier
        .checked_add(Duration::from_nanos(1))
        .ok_or(Failure::Control)?;
    assert_eq!(check_capture_order(earlier, later), Ok(()));
    assert_eq!(check_capture_order(earlier, earlier), Ok(()));
    assert_eq!(
        check_capture_order(later, earlier),
        Err(Failure::Datagram(DatagramError::Capture))
    );
    Ok(())
}

/// The actual concrete sender boundary refuses wrong destinations and repeat calls.
/// # Errors
/// Preserve actual preparation/publication and the independent healthy neighbor.
/// # Panics
/// Fail if either misuse exposes bytes to the scripted syscall branch.
#[test]
fn concrete_sender_refuses_wrong_endpoint_and_repeated_submission() -> Result<(), Failure> {
    with_owner(S2C, |owner, control, remote| {
        let mut backend = backend(remote, Script::new(ScriptSend::Full, Vec::new()));
        let ticket = owner.reserve(RequestInput::Observed)?;
        owner.prepare_observed(&ticket, entropy)?;
        let wrong = SocketAddr::new(remote.ip(), 124);
        let mut refused = None;
        let result = owner.invoke_sender(ticket, |packet, _| {
            match backend.send_guarded(control, packet, wrong) {
                Ok(outcome) => outcome,
                Err(error) => {
                    refused = Some(error);
                    SendOutcome::Unknown
                }
            }
        });
        assert_eq!(refused, Some(Failure::Endpoint));
        assert!(matches!(result, Err(Failure::Ledger(LedgerError::Unknown))));
        assert_eq!(script(&backend)?.send_calls, 0);
        assert_eq!(owner.status().used, 1);
        assert_eq!(owner.status().phase, RequestPhase::Invoked);
        assert!(owner.last_receipt().is_none());
        Ok(())
    })?;
    with_owner(S2C, |owner, control, remote| {
        let mut backend = backend(
            remote,
            Script::new(
                ScriptSend::Full,
                vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
            ),
        );
        let driver = runtime()?;
        let completion = driver.block_on(operation(owner, control, entropy, &mut backend))?;
        let captured = script(&backend)?;
        assert_eq!(&captured.packet[40..48], &WIRE.to_be_bytes());
        let packet = Zeroizing::new(
            captured
                .packet
                .get(..captured.packet_bytes)
                .ok_or(Failure::Control)?
                .to_vec(),
        );
        assert_eq!(
            backend.send_guarded(control, &packet, remote),
            Err(Failure::Endpoint)
        );
        assert_eq!(script(&backend)?.send_calls, 1);
        assert!(owner.last_receipt().is_some());
        // Raw operation history grants no final commit, even after manual cleanup.
        drop(completion);
        drop(backend);
        drop(driver);
        assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
        Ok(())
    })
}

/// Historical packet completion cannot defeat a stop before the final CLI commit.
/// # Errors
/// Preserve only unexpected fixture construction or packet refusal.
/// # Panics
/// Fail if stop is acknowledged then success reported, or committed history rewritten.
#[test]
fn outer_commit_distinguishes_stop_before_and_after_historical_completion() -> Result<(), Failure> {
    for stop_first in [true, false] {
        with_owner(S2C, |owner, control, remote| {
            let completion = exchange_with(owner, control, entropy, |admitted| {
                Ok(backend(
                    admitted,
                    Script::new(
                        ScriptSend::Full,
                        vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
                    ),
                ))
            })?;
            assert!(owner.last_receipt().is_some());
            let before = owner.status();
            if stop_first {
                assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
                assert!(matches!(
                    protected_once(owner, |_| Ok(completion)),
                    Err(Failure::Cancelled)
                ));
                assert_eq!(owner.status(), before);
            } else {
                let committed = completion.commit(owner)?;
                assert_eq!(control.cancel()?, CancelOutcome::AlreadyCommitted);
                assert!(committed.into_summary().transmitted);
            }
            Ok(())
        })?;
    }
    Ok(())
}

/// KE-STOP-06: scope stop cannot forge or erase a genuinely retired completion.
/// # Errors
/// Preserve checked setup, genuine protected exchange and actual stop refusal.
/// # Panics
/// Fail if stop-before-commit succeeds or a prior commit loses historical counts.
#[test]
fn invocation_stop_orders_retired_commit_without_rewriting_history() -> Result<(), Failure> {
    for stop_first in [true, false] {
        with_owner(S2C, |owner, control, remote| {
            let completion = exchange_with(owner, control, entropy, |admitted| {
                Ok(backend(
                    admitted,
                    Script::new(
                        ScriptSend::Full,
                        vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
                    ),
                ))
            })?;
            let stop = StopHandle {
                scope: Arc::clone(&control.stop_scope),
            };
            let before = owner.status();
            let receipt = owner.last_receipt().map(Receipt::serial);
            assert_eq!(receipt, Some(1));
            if stop_first {
                assert_eq!(stop.request_stop()?, StopDisposition::Latched);
                assert!(matches!(
                    protected_once(owner, |_| Ok(completion)),
                    Err(Failure::Cancelled)
                ));
            } else {
                let committed = completion.commit(owner)?;
                assert_eq!(stop.request_stop()?, StopDisposition::Latched);
                assert_eq!(control.cancel()?, CancelOutcome::AlreadyCommitted);
                let summary = committed.into_summary();
                assert!(summary.transmitted);
                assert_eq!(summary.cookies, 1);
                assert_eq!(summary.cookie_bytes, 12);
            }
            assert_eq!(stop.request_stop()?, StopDisposition::AlreadyLatched);
            assert_eq!(owner.status(), before);
            assert_eq!(owner.last_receipt().map(Receipt::serial), receipt);
            Ok(())
        })?;
    }
    Ok(())
}

/// A genuinely retired completion loses authority at its unchanged original expiry.
/// # Errors
/// Preserve checked construction, genuine packet completion or bounded-wait refusal.
/// # Panics
/// Fail if expiry enters setup/entropy, changes history/budgets or commits control.
#[test]
fn retired_completion_expiry_refuses_wrapper_and_final_commit() -> Result<(), Failure> {
    use std::cell::Cell;

    // Leave five seconds for actual checked construction and the healthy Script
    // operation; the clock is never changed and no trusted field is rewritten.
    let age = crate::nts_ke_experiment::EXCHANGE_TIME
        .checked_sub(Duration::from_secs(5))
        .ok_or(Failure::Deadline)?;
    let start = Instant::now().checked_sub(age).ok_or(Failure::Deadline)?;
    with_owner_start(start, S2C, |owner, control, _remote| {
        let completion = exchange_with(owner, control, entropy, |admitted| {
            Ok(backend(
                admitted,
                Script::new(
                    ScriptSend::Full,
                    vec![ScriptRead::Packet(fixtures::HEALTHY, admitted)],
                ),
            ))
        })?;
        let receipt = completion.history().receipt;
        let before = owner.status();
        let expiry = control.expiry;
        assert_eq!(receipt.serial(), 1);
        runtime()?
            .block_on(async {
                tokio::time::timeout(
                    Duration::from_secs(6),
                    tokio::time::sleep_until(tokio::time::Instant::from_std(expiry)),
                )
                .await
            })
            .map_err(|_| Failure::Control)?;
        assert!(Instant::now() >= expiry);
        assert!(tokio::runtime::Handle::try_current().is_err());
        let backend_calls = Cell::new(0);
        let entropy_calls = Cell::new(0);
        assert!(matches!(
            exchange_with(
                owner,
                control,
                |output| {
                    entropy_calls.set(entropy_calls.get() + 1);
                    entropy(output)
                },
                |admitted| {
                    backend_calls.set(backend_calls.get() + 1);
                    Ok(backend(admitted, Script::new(ScriptSend::Full, Vec::new())))
                }
            ),
            Err(Failure::Deadline)
        ));
        assert_eq!((backend_calls.get(), entropy_calls.get()), (0, 0));
        assert!(matches!(completion.commit(owner), Err(Failure::Deadline)));
        assert_eq!(owner.status(), before);
        let after = owner.last_receipt().ok_or(Failure::Control)?;
        assert_eq!(after.serial(), receipt.serial());
        assert_eq!(after.accepted_at(), receipt.accepted_at());
        assert_eq!(after.cookie_counts(), receipt.cookie_counts());
        assert_eq!(control.expiry, expiry);
        assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
        Ok(())
    })
}

/// Equal endpoints and genuine equal receipt ordinals do not establish ownership.
/// The independent ledger boundary also refuses a foreign receipt after a local return.
/// # Errors
/// Preserve checked setup and genuine operation failures for either association.
/// # Panics
/// Fail if the foreign completion commits, changes either ledger or closes either Control.
#[test]
fn foreign_completion_with_equal_endpoint_and_serial_cannot_commit() -> Result<(), Failure> {
    with_owner(S2C, |original, original_control, original_remote| {
        let completion = exchange_with(original, original_control, entropy, |admitted| {
            Ok(backend(
                admitted,
                Script::new(
                    ScriptSend::Full,
                    vec![ScriptRead::Packet(fixtures::HEALTHY, admitted)],
                ),
            ))
        })?;
        let serial = completion.history().receipt.serial();
        assert_eq!(serial, 1);
        let original_before = original.status();
        with_owner(S2C, |other, other_control, other_remote| {
            assert_eq!(original_remote, other_remote);
            let other_completion = exchange_with(other, other_control, entropy, |admitted| {
                Ok(backend(
                    admitted,
                    Script::new(
                        ScriptSend::Full,
                        vec![ScriptRead::Packet(fixtures::HEALTHY, admitted)],
                    ),
                ))
            })?;
            assert_eq!(other_completion.history().receipt.serial(), serial);
            with_owner(S2C, |idle, idle_control, idle_remote| {
                assert_eq!(idle_remote, other_remote);
                let ticket = idle.reserve(RequestInput::Observed)?;
                idle.return_unused(ticket)?;
                let before = idle.status();
                assert_eq!(before.phase, RequestPhase::Idle);
                assert_eq!(before.attempts, 31);
                assert!(idle.last_receipt().is_none());
                // A genuine foreign receipt matches the local charged ordinal,
                // but cannot supply the local owner's missing completion history.
                assert!(
                    matches!(
                        idle.check_completed(idle_control, &other_completion.history().receipt),
                        Err(Failure::Ledger(LedgerError::Ticket))
                    ),
                    "foreign receipt bypassed the local completion-history guard"
                );
                assert_eq!(idle.status(), before);
                assert!(idle.last_receipt().is_none());
                idle_control.remaining(IoPhase::Exchange)?;
                assert_eq!(idle_control.cancel()?, CancelOutcome::Cancelled);
                Ok(())
            })?;
            let other_before = other.status();
            assert!(matches!(
                protected_once(other, |_| Ok(completion)),
                Err(Failure::Control)
            ));
            assert_eq!(original.status(), original_before);
            assert_eq!(other.status(), other_before);
            assert_eq!(original.last_receipt().map(Receipt::serial), Some(serial));
            assert_eq!(other.last_receipt().map(Receipt::serial), Some(serial));
            original_control.remaining(IoPhase::Exchange)?;
            other_control.remaining(IoPhase::Exchange)?;
            // A genuine completion still commits at its own receiving owner.
            assert!(other_completion.commit(other)?.into_summary().transmitted);
            assert_eq!(other_control.cancel()?, CancelOutcome::AlreadyCommitted);
            assert_eq!(original_control.cancel()?, CancelOutcome::Cancelled);
            Ok(())
        })
    })
}

/// Dropping a retired completion leaves genuine receipt history without a fact.
/// # Errors
/// Preserve checked setup, operation and still-published currentness failures.
/// # Panics
/// Fail if Drop commits, changes budgets or removes the accepted historical receipt.
#[test]
fn dropped_retired_completion_preserves_history_without_commit() -> Result<(), Failure> {
    with_owner(S2C, |owner, control, _remote| {
        let completion = exchange_with(owner, control, entropy, |admitted| {
            Ok(backend(
                admitted,
                Script::new(
                    ScriptSend::Full,
                    vec![ScriptRead::Packet(fixtures::HEALTHY, admitted)],
                ),
            ))
        })?;
        let serial = completion.history().receipt.serial();
        let before = owner.status();
        drop(completion);
        assert_eq!(owner.status(), before);
        assert_eq!(owner.last_receipt().map(Receipt::serial), Some(serial));
        control.remaining(IoPhase::Exchange)?;
        assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
        Ok(())
    })
}

/// The production preview decision projects initial counts without a protected fact.
/// # Errors
/// Preserve checked setup, actual preview sealing and final currentness failures.
/// # Panics
/// Fail if preview reports transmission, changes initial counts or refunds its attempt.
#[test]
fn one_shot_preview_preserves_counts_and_never_reports_transmission() -> Result<(), Failure> {
    with_owner(S2C, |owner, control, _remote| {
        let summary = preview_once(owner, |output| {
            output[..32].copy_from_slice(&C2S);
            output[32..].fill(0x66);
            Ok(())
        })?;
        assert!(!summary.transmitted);
        assert_eq!(
            (summary.cookies, summary.cookie_bytes, summary.packet_bytes),
            (1, 12, 140)
        );
        assert_eq!(owner.status().attempts, 31);
        assert_eq!((owner.status().frames, owner.status().bytes), (32, 131_072));
        assert_eq!((owner.status().used, owner.status().available), (0, 1));
        assert!(owner.last_receipt().is_none());
        assert_eq!(control.cancel()?, CancelOutcome::AlreadyCommitted);
        Ok(())
    })
}

/// Preview refusal preserves both currentness and its irreversibly charged work.
/// # Errors
/// Preserve checked owner setup and unexpected cancellation/currentness refusal.
/// # Panics
/// Fail if cancellation enters entropy, or a failed fill permits another attempt.
#[test]
fn one_shot_preview_refusals_preserve_currentness_and_spent_work() -> Result<(), Failure> {
    use std::cell::Cell;

    with_owner(S2C, |owner, control, _remote| {
        assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
        let before = owner.status();
        let expiry = control.expiry;
        let calls = Cell::new(0);
        assert!(matches!(
            preview_once(owner, |_| {
                calls.set(calls.get() + 1);
                Ok(())
            }),
            Err(Failure::Cancelled)
        ));
        assert_eq!(calls.get(), 0);
        assert_eq!(owner.status(), before);
        assert!(owner.last_receipt().is_none());
        assert_eq!(control.expiry, expiry);
        assert_eq!(control.cancel()?, CancelOutcome::AlreadyCancelled);
        Ok(())
    })?;
    with_owner(S2C, |owner, control, _remote| {
        let expiry = control.expiry;
        let calls = Cell::new(0);
        assert!(matches!(
            preview_once(owner, |_| {
                calls.set(calls.get() + 1);
                Err(Failure::Entropy)
            }),
            Err(Failure::Entropy)
        ));
        assert_eq!(calls.get(), 1);
        let spent = owner.status();
        assert_eq!(spent.phase, RequestPhase::Preparing);
        assert_eq!(spent.attempts, 31);
        assert_eq!((spent.frames, spent.bytes), (32, 131_072));
        assert_eq!((spent.available, spent.used, spent.uids), (0, 0, 0));
        let second_calls = Cell::new(0);
        assert!(matches!(
            preview_once(owner, |_| {
                second_calls.set(second_calls.get() + 1);
                Ok(())
            }),
            Err(Failure::PreviewSpent)
        ));
        assert_eq!(second_calls.get(), 0);
        assert_eq!(owner.status(), spent);
        assert!(owner.last_receipt().is_none());
        assert_eq!(control.expiry, expiry);
        control.remaining(IoPhase::Exchange)?;
        assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
        Ok(())
    })
}

/// The actual receiving decision keeps initial KE counts after a larger replacement.
/// # Errors
/// Preserve checked setup, shared-wrapper operation and receiving commit failures.
/// # Panics
/// Fail if terminal counts follow replacement cookies or a different packet extent.
#[test]
fn one_shot_protected_decision_preserves_initial_cookie_counts() -> Result<(), Failure> {
    use std::cell::Cell;

    with_owner(S2C, |owner, control, _remote| {
        let packet_bytes = Cell::new(0);
        let summary = protected_once(owner, |receiving| {
            let completion = exchange_with(receiving, control, entropy, |admitted| {
                Ok(backend(
                    admitted,
                    Script::new(
                        ScriptSend::Full,
                        vec![ScriptRead::Packet(fixtures::TWO_COOKIES, admitted)],
                    ),
                ))
            })?;
            assert_eq!(completion.history().receipt.cookie_counts(), (2, 2));
            packet_bytes.set(completion.history().packet_bytes);
            Ok(completion)
        })?;
        assert!(summary.transmitted);
        assert_eq!((summary.cookies, summary.cookie_bytes), (1, 12));
        assert_eq!(owner.cookie_population()?, (2, 24));
        assert_eq!(summary.packet_bytes, packet_bytes.get());
        assert_eq!(owner.status().attempts, 31);
        assert_eq!(owner.status().frames, 31);
        assert_eq!(owner.status().bytes, 131_072 - fixtures::TWO_COOKIES.len());
        assert_eq!(control.cancel()?, CancelOutcome::AlreadyCommitted);
        Ok(())
    })
}

/// Failure retires the shared wrapper's resources without refunding charged work.
/// # Errors
/// Preserve unexpected checked setup or final cancellation failures.
/// # Panics
/// Fail if an error changes category, leaks the backend/runtime, commits or resets budgets.
#[test]
fn wrapper_failures_retire_resources_and_preserve_spent_budgets() -> Result<(), Failure> {
    for (key, send, packet, expected, frames, bytes) in [
        (
            S2C,
            ScriptSend::Failed(io::ErrorKind::PermissionDenied),
            None,
            Failure::Ledger(LedgerError::Send(io::ErrorKind::PermissionDenied)),
            32,
            131_072,
        ),
        (
            S2C,
            ScriptSend::Full,
            Some(fixtures::ZERO_COOKIES),
            Failure::Ledger(LedgerError::Reply(ReplyError::Cookies)),
            0,
            131_072 - 32 * fixtures::ZERO_COOKIES.len(),
        ),
        (
            [0x55; 32],
            ScriptSend::Full,
            Some(fixtures::HEALTHY),
            Failure::Ledger(LedgerError::Reply(ReplyError::Open(
                StagedDecryptError::Authentication,
            ))),
            0,
            131_072 - 32 * fixtures::HEALTHY.len(),
        ),
    ] {
        with_owner(key, |owner, control, _remote| {
            let expiry = control.expiry;
            let retained = Arc::new(());
            let task_lifetime = Arc::downgrade(&retained);
            let mut socket_lifetime = None;
            let mut pending = None;
            let result = protected_once(owner, |receiving| {
                exchange_with(receiving, control, entropy, |admitted| {
                    let reads = packet.map_or_else(Vec::new, |packet| {
                        (0..32)
                            .map(|_| ScriptRead::Packet(packet, admitted))
                            .collect()
                    });
                    let backend = backend(admitted, Script::new(send, reads));
                    socket_lifetime = Some(Arc::downgrade(&backend.identity));
                    pending = Some(tokio::spawn(async move {
                        std::future::pending::<()>().await;
                        drop(retained);
                    }));
                    Ok(backend)
                })
            });
            assert!(matches!(result, Err(error) if error == expected));
            assert_eq!(socket_lifetime.ok_or(Failure::Control)?.strong_count(), 0);
            assert_eq!(task_lifetime.strong_count(), 0);
            drop(pending);
            assert_eq!(control.expiry, expiry);
            assert_eq!(owner.status().attempts, 31);
            assert_eq!(
                (owner.status().frames, owner.status().bytes),
                (frames, bytes)
            );
            assert_eq!((owner.status().used, owner.status().available), (1, 0));
            assert_eq!(owner.status().phase, RequestPhase::Idle);
            assert!(owner.last_receipt().is_none());
            assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
            Ok(())
        })?;
    }
    Ok(())
}

/// A refusal before ordinal consumption cannot stale the last genuine completion.
/// # Errors
/// Preserve actual bounded reserve/return, shared operation and final commit failures.
/// # Panics
/// Fail if exhaustion refunds an attempt or invalidates a still-current completion.
#[test]
fn uncharged_attempt_exhaustion_does_not_stale_current_completion() -> Result<(), Failure> {
    with_owner(S2C, |owner, control, _remote| {
        for _ in 0..31 {
            let ticket = owner.reserve(RequestInput::Observed)?;
            owner.return_unused(ticket)?;
        }
        let completion = exchange_with(owner, control, entropy, |admitted| {
            Ok(backend(
                admitted,
                Script::new(
                    ScriptSend::Full,
                    vec![ScriptRead::Packet(fixtures::HEALTHY, admitted)],
                ),
            ))
        })?;
        assert_eq!(completion.history().receipt.serial(), 32);
        let before = owner.status();
        assert_eq!(before.attempts, 0);
        assert!(matches!(
            owner.reserve(RequestInput::Observed),
            Err(Failure::Ledger(LedgerError::Attempts))
        ));
        assert_eq!(owner.status(), before);
        assert!(completion.commit(owner)?.into_summary().transmitted);
        assert_eq!(control.cancel()?, CancelOutcome::AlreadyCommitted);
        Ok(())
    })
}

/// Dropping a parked component operation cannot undo its invocation custody.
/// # Errors
/// Preserve unexpected component construction or runtime failure.
/// # Panics
/// Fail if future destruction reissues a cookie, frees the slot or enters entropy again.
#[test]
fn dropped_pending_operation_keeps_cookie_and_exclusive_slot() -> Result<(), Failure> {
    use std::cell::Cell;
    use std::future::Future;
    use std::task::Poll;

    with_owner(S2C, |owner, control, remote| {
        let mut selected = Script::new(ScriptSend::Full, Vec::new());
        selected.ready.push_back(ScriptReady::Ready);
        selected.ready.push_back(ScriptReady::Pending);
        let mut backend = backend(remote, selected);
        let executor = runtime()?;
        let mut pending = Box::pin(operation(owner, control, entropy, &mut backend));
        let parked = executor.block_on(std::future::poll_fn(|context| {
            Poll::Ready(pending.as_mut().poll(context).is_pending())
        }));
        assert!(parked);
        // This owns the future itself; dropping a Pin<&mut _> would only drop
        // its borrow and would not exercise operation destruction here.
        drop(pending);
        assert_eq!(owner.status().phase, RequestPhase::Pending);
        assert_eq!(owner.status().used, 1);
        assert_eq!(script(&backend)?.send_calls, 1);
        assert!(owner.last_receipt().is_none());
        let calls = Cell::new(0);
        let retried = executor.block_on(operation(
            owner,
            control,
            |output| {
                calls.set(calls.get() + 1);
                entropy(output)
            },
            &mut backend,
        ));
        assert!(matches!(retried, Err(Failure::Ledger(LedgerError::Busy))));
        assert_eq!(calls.get(), 0);
        assert_eq!(script(&backend)?.send_calls, 1);
        assert_eq!((owner.status().frames, owner.status().bytes), (32, 131_072));
        Ok(())
    })
}

/// The real absolute deadline releases a parked wait without a fake time source.
/// # Errors
/// Preserve unexpected setup failure or checked monotonic subtraction refusal.
/// # Panics
/// Fail if readiness parking renews expiry, falsely completes or exceeds its watchdog.
#[test]
fn original_absolute_expiry_ends_parked_receive_without_publication() -> Result<(), Failure> {
    // Healthy neighboring reception uses the exact same operation and script
    // arrangement, differing only in its ready packet rather than parked input.
    with_owner(S2C, |owner, control, remote| {
        let mut backend = backend(
            remote,
            Script::new(
                ScriptSend::Full,
                vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
            ),
        );
        runtime()?.block_on(operation(owner, control, entropy, &mut backend))?;
        Ok(())
    })?;
    let start = Instant::now()
        .checked_sub(Duration::from_secs(59))
        .ok_or(Failure::Deadline)?;
    with_owner_start(start, S2C, |owner, control, remote| {
        let expiry = control.expiry;
        let mut selected = Script::new(ScriptSend::Full, Vec::new());
        selected.ready.push_back(ScriptReady::Ready);
        selected.ready.push_back(ScriptReady::Pending);
        let mut backend = backend(remote, selected);
        let outcome = runtime()?.block_on(async {
            tokio::time::timeout(
                Duration::from_secs(2),
                operation(owner, control, entropy, &mut backend),
            )
            .await
        });
        assert!(matches!(outcome, Ok(Err(Failure::Deadline))));
        assert_eq!(control.expiry, expiry);
        assert_eq!(owner.status().used, 1);
        assert_eq!(script(&backend)?.send_calls, 1);
        assert_eq!(script(&backend)?.receive_calls, 0);
        assert!(owner.last_receipt().is_none());
        Ok(())
    })
}

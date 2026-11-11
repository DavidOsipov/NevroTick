// SPDX-License-Identifier: GPL-2.0-only
//! Public-material deterministic cases, with no UDP/TLS/native qualification.

use std::collections::VecDeque;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use zeroize::Zeroizing;

use super::DatagramError;
use super::reached::{Backend, Interest, Io, Read, check_capture_order, classify_read, operation};
use crate::nts_ke_experiment::association::ledger::{
    LedgerError, RequestInput, RequestPhase, SendOutcome,
};
use crate::nts_ke_experiment::association::{AssociationCandidate, AssociationOwner};
use crate::nts_ke_experiment::process::{ProcessAdmission, ProviderScope};
use crate::nts_ke_experiment::profile::StrictProfile;
use crate::nts_ke_experiment::request::DirectionalKeys;
use crate::nts_ke_experiment::response::fixtures;
use crate::nts_ke_experiment::{Config, Control, Failure};

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
    /// Readiness itself failed, without consuming any datagram.
    Failed(io::ErrorKind),
}

/// Fixed syscall outcomes; no callback executes inside the control mutex.
#[derive(Clone, Copy)]
enum ScriptSend {
    Full,
    Short,
    Failed(io::ErrorKind),
    Unknown,
    CancelBeforeGuard,
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
    pub(super) fn before_guard(&mut self, control: &Control) -> Result<(), Failure> {
        if matches!(self.send, ScriptSend::CancelBeforeGuard) {
            control.cancel()?;
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
            ScriptSend::Full | ScriptSend::CancelBeforeGuard => {
                (SendOutcome::Completed(packet.len()), false)
            }
            ScriptSend::Short => (
                SendOutcome::Completed(packet.len().saturating_sub(1)),
                false,
            ),
            ScriptSend::Failed(kind) => (SendOutcome::Failed(kind), false),
            ScriptSend::Unknown => (SendOutcome::Unknown, false),
            ScriptSend::CancelAfterFull => (SendOutcome::Completed(packet.len()), true),
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
    let provider = ProviderScope::component(&claim);
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
    with_owner(S2C, |owner, control, remote| {
        let mut backend = backend(
            remote,
            Script::new(
                ScriptSend::Full,
                vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
            ),
        );
        let completion = runtime()?.block_on(operation(owner, control, entropy, &mut backend))?;
        let fixture_header = fixtures::HEALTHY.get(..48).ok_or(Failure::Control)?;
        let expected =
            ntp_wire::decode_fixed_header(fixture_header).map_err(|_| Failure::Control)?;
        let header = completion.receipt.fields().header();
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
        assert!(completion.receipt.local_t1().is_none());
        assert!(completion.send_before.monotonic_after() <= completion.send_after.monotonic());
        assert!(completion.send_after.monotonic_after() <= completion.read_return.monotonic());
        assert_eq!(completion.local, backend.local);
        assert_eq!(completion.remote, remote);
        assert!(Arc::ptr_eq(&completion.identity, &backend.identity));
        assert_eq!(script(&backend)?.send_calls, 1);
        assert_eq!(&script(&backend)?.packet[40..48], &WIRE.to_be_bytes());
        assert_eq!(owner.status().frames, 31);
        assert_eq!(owner.status().bytes, 131_072 - fixtures::HEALTHY.len());
        assert_eq!(owner.status().used, 1);
        drop(backend);
        let summary = completion.finish()?;
        assert!(summary.transmitted);
        assert_eq!((summary.cookies, summary.cookie_bytes), (1, 12));
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
        let completion = runtime()?.block_on(operation(owner, control, entropy, &mut backend))?;
        let captured = script(&backend)?;
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
        drop(backend);
        assert!(completion.finish()?.transmitted);
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
            let mut backend = backend(
                remote,
                Script::new(
                    ScriptSend::Full,
                    vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
                ),
            );
            let completion =
                runtime()?.block_on(operation(owner, control, entropy, &mut backend))?;
            assert!(owner.last_receipt().is_some());
            drop(backend);
            if stop_first {
                assert_eq!(
                    control.cancel()?,
                    crate::nts_ke_experiment::CancelOutcome::Cancelled
                );
                assert!(matches!(completion.finish(), Err(Failure::Cancelled)));
            } else {
                let summary = completion.finish()?;
                assert!(summary.transmitted);
                assert_eq!(
                    control.cancel()?,
                    crate::nts_ke_experiment::CancelOutcome::AlreadyCommitted
                );
            }
            Ok(())
        })?;
    }
    Ok(())
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

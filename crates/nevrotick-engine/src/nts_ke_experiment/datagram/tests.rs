// SPDX-License-Identifier: GPL-2.0-only
//! Protected datagram component, loopback, and controlled native integration cases.

#[path = "../recovery/tests.rs"]
pub(in crate::nts_ke_experiment) mod recovery_tests;

use std::collections::VecDeque;
use std::io;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, TryLockError};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use nevrotick_s115_aes_siv_fixture::staged::StagedDecryptError;
use tokio::sync::watch;
use zeroize::Zeroizing;

#[cfg(windows)]
use windows_platform_native::ClockReadError;

use super::super::ProtectedControlKind;
use super::DatagramError;
use super::reached::{
    Backend, Interest, Io, OperationCompletion, Read, RetiredProtectedResponse,
    check_capture_order, classify_read, exchange_successor_mixed, exchange_successor_observing,
    exchange_successor_with, exchange_with, operation,
};
use crate::SoftwareClock;
use crate::nts_ke_experiment::association::ledger::{
    AppliedProtectedControl, CookieReplacement, LedgerError, PendingSubmission, Receipt,
    RequestInput, RequestPhase, SendOutcome, StagedProtectedSuccessor, Status, Ticket,
};
use crate::nts_ke_experiment::association::{AssociationCandidate, AssociationOwner};
use crate::nts_ke_experiment::process::{ProcessAdmission, ProcessClaim, ProviderScope};
use crate::nts_ke_experiment::profile::StrictProfile;
use crate::nts_ke_experiment::request::DirectionalKeys;
use crate::nts_ke_experiment::response::{ControlProtection, ReplyError, fixtures};
use crate::nts_ke_experiment::{
    CancelOutcome, Config, Control, Failure, IoPhase, StopDisposition, StopHandle, preview_once,
    protected_once,
};
use crate::{
    KePreviewTarget, SourceConfigurationSnapshot, SourcePolicyAuthority, SourceRestrictionSnapshot,
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
    /// Latch the invocation's Stop after a completed send enters receiving.
    ScopeStop,
    /// Park until the independently driven control wake/deadline.
    Pending,
    /// Wait at the actual source/endpoint floor under the original outer deadline.
    After(Instant),
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

/// Independently revoke authority after the original selected receive capture.
enum ScriptReceiveRevocation {
    /// Latch the original invocation's real Stop before ledger entry.
    Stop {
        handle: StopHandle,
        expiry: Instant,
        wait_for_expiry: bool,
    },
    /// Wait for the invocation's existing, finite real cutoff.
    Expiry(Instant),
}

impl ScriptReceiveRevocation {
    /// Apply exactly the fixture's selected action outside the Control gates.
    /// # Panics
    /// Fail if authority had already expired or the scope was not freshly latched.
    fn apply(&self, captured: SoftwareClock) {
        let expiry: Instant = match self {
            Self::Stop {
                handle,
                expiry,
                wait_for_expiry,
            } => {
                assert!(handle.scope.stopped.try_lock().is_ok());
                assert_eq!(handle.request_stop(), Ok(StopDisposition::Latched));
                assert!(Instant::now() < *expiry);
                if !wait_for_expiry {
                    return;
                }
                *expiry
            }
            Self::Expiry(expiry) => *expiry,
        };
        assert!(captured.monotonic_after() < expiry);
        std::thread::sleep(expiry.saturating_duration_since(Instant::now()));
        assert!(Instant::now() >= expiry);
    }
}

/// Bounded scripted facts. This type is available only in unit-test builds.
pub(super) struct Script {
    /// Actual component backend lifetime, separate from retained socket identity.
    lifetime: Arc<()>,
    ready: VecDeque<ScriptReady>,
    /// Keep read readiness parked across repeated polls until the fixture releases it.
    read_release: Option<watch::Receiver<bool>>,
    reads: VecDeque<ScriptRead>,
    /// Optional observation of queue depth immediately after each actual read.
    read_residue: Option<Arc<AtomicUsize>>,
    /// Optional record of the readings returned by the bounded scripted adapter.
    read_captures: Option<Arc<Mutex<Vec<SoftwareClock>>>>,
    /// Override the leading receive-return reading at the selected queue residue.
    read_before_at_residue: Option<(usize, Instant)>,
    /// Optional trailing reading at the actual receive-return capture position.
    read_after: Option<Instant>,
    /// Optional substitute for the trailing reading of the final scripted datagram.
    read_after_final: Option<Instant>,
    /// Apply the closed event on the selected completed receive capture.
    revoke_after_read: Option<(usize, ScriptReceiveRevocation)>,
    send: ScriptSend,
    /// Optional fixed sample at the final pre-send capture position.
    send_before: Option<SoftwareClock>,
    /// Optional closed fixture date, independent of the actual host clock.
    fixed_utc: Option<SystemTime>,
    /// Scripted read refusals for the original pre-send, send-return and receive-return points.
    #[cfg(windows)]
    pub(super) clock_refusals: [Option<ClockReadError>; 3],
    /// Optional genuine same-byte replacement after readiness and before final custody.
    replace_before_guard: Option<GuardReplacement>,
    send_calls: usize,
    receive_calls: usize,
    packet: Zeroizing<[u8; 4096]>,
    packet_bytes: usize,
}

/// Closed configuration replacement using the actual source authority and snapshot.
struct GuardReplacement {
    /// Authority for this configured source's existing revision.
    authority: SourcePolicyAuthority,
    /// Exact prior definition that must still be current.
    expected: SourceConfigurationSnapshot,
    /// Original TLS input, reused alongside the original endpoint set.
    target: KePreviewTarget,
}

impl Script {
    /// Create a finite public case before entering the guarded operation.
    fn new(send: ScriptSend, reads: Vec<ScriptRead>) -> Self {
        Self {
            lifetime: Arc::new(()),
            ready: VecDeque::new(),
            read_release: None,
            reads: reads.into(),
            read_residue: None,
            read_captures: None,
            read_before_at_residue: None,
            read_after: None,
            read_after_final: None,
            revoke_after_read: None,
            send,
            send_before: None,
            fixed_utc: None,
            #[cfg(windows)]
            clock_refusals: [None; 3],
            replace_before_guard: None,
            send_calls: 0,
            receive_calls: 0,
            packet: Zeroizing::new([0; 4096]),
            packet_bytes: 0,
        }
    }

    /// Apply the selected test action after readiness and before archive-and-send custody.
    /// # Errors
    /// Preserve the actual configuration or cancellation failure before any send.
    /// # Panics
    /// Fail if the selected first invocation stop had already been delivered.
    pub(super) fn before_guard(&mut self, control: &Control) -> Result<(), Failure> {
        if let Some(mut replacement) = self.replace_before_guard.take() {
            let newer: SourceConfigurationSnapshot = replacement
                .authority
                .replace_ke_preview_configuration(
                    &replacement.expected,
                    replacement.expected.addresses().clone(),
                    replacement.target,
                )
                .map_err(Failure::SourcePolicy)?;
            assert!(!newer.same_configuration(&replacement.expected));
        }
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
    /// # Panics
    /// Panics if the scripted scope Stop was already latched.
    pub(super) async fn ready(
        &mut self,
        interest: Interest,
        control: &Control,
    ) -> Result<(), Failure> {
        if let Interest::Read = interest
            && let Some(release) = &mut self.read_release
        {
            drop(
                release
                    .wait_for(|ready: &bool| *ready)
                    .await
                    .map_err(|_| Failure::Control)?,
            );
        }
        match self.ready.pop_front().unwrap_or(ScriptReady::Ready) {
            ScriptReady::Ready => Ok(()),
            ScriptReady::Cancel => {
                control.cancel()?;
                Ok(())
            }
            ScriptReady::ScopeStop => {
                let stop: StopHandle = StopHandle {
                    scope: Arc::clone(&control.stop_scope),
                };
                assert_eq!(stop.request_stop()?, StopDisposition::Latched);
                Ok(())
            }
            ScriptReady::Pending => std::future::pending().await,
            ScriptReady::After(due) => {
                tokio::time::sleep_until(tokio::time::Instant::from_std(due)).await;
                Ok(())
            }
            ScriptReady::SiblingCancelPending(sibling) => {
                sibling.cancel()?;
                std::future::pending().await
            }
            ScriptReady::Failed(kind) => Err(Failure::Datagram(DatagramError::Readiness(kind))),
        }
    }

    /// Return fixed facts; the only copy is into preallocated bounded storage.
    /// # Panics
    /// Fail if either genuine authority gate is unlocked at adapter entry.
    pub(super) fn send(&mut self, packet: &[u8], control: &Control) -> (SendOutcome, bool) {
        assert!(matches!(
            control.stop_scope.stopped.try_lock(),
            Err(TryLockError::WouldBlock)
        ));
        assert!(matches!(
            control.phase.try_lock(),
            Err(TryLockError::WouldBlock)
        ));
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

    /// Return the fixed pre-send sample when that fixture selected one.
    pub(super) const fn observe_send_before(&self, captured: SoftwareClock) -> SoftwareClock {
        match self.send_before {
            Some(before) => before,
            None => self.fixture_wall(captured),
        }
    }

    /// Inject only the closed UTC fault; retain the actual captured monotonic bracket.
    /// This changes neither the host clock nor syscall, receipt or join ownership.
    /// # Errors
    /// Refuse an unrepresentable synthetic UTC coordinate rather than hiding the fault.
    pub(super) fn observe_send_after(
        &self,
        captured: SoftwareClock,
    ) -> Result<SoftwareClock, Failure> {
        let captured: SoftwareClock = self.fixture_wall(captured);
        if matches!(self.send, ScriptSend::AfterWallStep) {
            let utc = captured
                .utc()
                .checked_add(Duration::from_secs(60))
                .ok_or(Failure::Control)?;
            Ok(SoftwareClock { utc, ..captured })
        } else {
            Ok(captured)
        }
    }

    /// Apply the fixed event and substitute only selected readings at the actual capture site.
    /// # Panics
    /// Fail if a selected fixture event cannot reach its intended boundary.
    pub(super) fn observe_read_return(&self, captured: SoftwareClock) -> SoftwareClock {
        if let Some((ordinal, revocation)) = &self.revoke_after_read
            && self.receive_calls == *ordinal
        {
            revocation.apply(captured);
        }
        let captured: SoftwareClock = match self.read_before_at_residue {
            Some((residue, mono)) if self.reads.len() == residue => {
                SoftwareClock { mono, ..captured }
            }
            _ => captured,
        };
        let trailing: Option<Instant> = self
            .read_after
            .or_else(|| self.read_after_final.filter(|_| self.reads.is_empty()));
        let captured: SoftwareClock = match trailing {
            Some(mono_after) => SoftwareClock {
                mono_after,
                ..captured
            },
            None => captured,
        };
        let captured: SoftwareClock = self.fixture_wall(captured);
        if let Some(captures) = &self.read_captures {
            assert!(
                captures
                    .lock()
                    .map(|mut readings: MutexGuard<'_, Vec<SoftwareClock>>| readings.push(captured))
                    .is_ok()
            );
        }
        captured
    }

    /// Substitute only wall readings, preserving the actual monotonic brackets.
    const fn fixture_wall(&self, captured: SoftwareClock) -> SoftwareClock {
        match self.fixed_utc {
            Some(utc) => SoftwareClock { utc, ..captured },
            None => captured,
        }
    }

    /// Copy one complete public datagram, never a usable truncated prefix.
    pub(super) fn receive(&mut self, buffer: &mut [u8; 4097]) -> Read {
        self.receive_calls += 1;
        let read: Option<ScriptRead> = self.reads.pop_front();
        if let Some(residue) = &self.read_residue {
            residue.store(self.reads.len(), Ordering::Relaxed);
        }
        match read {
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
    with_owner_cookies(start, response_key, &[(0x11, 12)], exercise)
}

/// Admit distinct initial test cookies independently of a replacement reply.
/// # Errors
/// Preserve actual original deadline, KE publication and exercised refusal.
fn with_owner_cookies(
    start: Instant,
    response_key: [u8; 32],
    cookie_values: &[(u8, usize)],
    exercise: impl FnOnce(&mut AssociationOwner<'_>, &Control, SocketAddr) -> Result<(), Failure>,
) -> Result<(), Failure> {
    let config: Config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    with_config_owner(&config, start, response_key, cookie_values, exercise)
}

/// Publish a checked component association under either actual configuration.
/// # Errors
/// Preserve issuer, provider, lease or original Control refusal.
fn with_config_owner(
    config: &Config,
    start: Instant,
    response_key: [u8; 32],
    cookie_values: &[(u8, usize)],
    exercise: impl FnOnce(&mut AssociationOwner<'_>, &Control, SocketAddr) -> Result<(), Failure>,
) -> Result<(), Failure> {
    let control: Control = Control::new(start)?;
    with_config_control(config, &control, response_key, cookie_values, exercise)
}

/// Publish a component association with the caller's checked original cutoff.
/// # Errors
/// Retain actual provider, endpoint publication and operation refusals.
fn with_config_control(
    config: &Config,
    control: &Control,
    response_key: [u8; 32],
    cookie_values: &[(u8, usize)],
    exercise: impl FnOnce(&mut AssociationOwner<'_>, &Control, SocketAddr) -> Result<(), Failure>,
) -> Result<(), Failure> {
    let admission: ProcessAdmission = ProcessAdmission::new();
    let claim: ProcessClaim<'_> = admission.claim()?;
    let provider: ProviderScope<'_> = ProviderScope::component(claim);
    let remote: SocketAddr = SocketAddr::new(config.peer.ip(), 123);
    let candidate: AssociationCandidate<'_> = AssociationCandidate::new(
        config,
        control,
        &provider,
        remote,
        StrictProfile::component(),
        DirectionalKeys::new(Zeroizing::new(C2S), Zeroizing::new(response_key)),
        cookie_values
            .iter()
            .map(|&(value, length): &(u8, usize)| Zeroizing::new(vec![value; length]))
            .collect::<Vec<Zeroizing<Vec<u8>>>>(),
    );
    let mut owner: AssociationOwner<'_> =
        AssociationOwner::receive(config, control, &provider, candidate)?;
    exercise(&mut owner, control, remote)
}

/// Keep script socket identity separate from the actual association identity.
pub(super) fn backend(remote: SocketAddr, script: Script) -> Backend {
    Backend {
        io: Io::Script(Box::new(script)),
        local: SocketAddr::new(remote.ip(), 49000),
        remote,
        identity: Arc::new(()),
        send: None,
    }
}

/// Component-only directional keys matching the independent protected reply.
pub(in crate::nts_ke_experiment) fn managed_component_keys() -> DirectionalKeys {
    DirectionalKeys::new(Zeroizing::new(C2S), Zeroizing::new(S2C))
}

/// Exercise the ordinary protected adapter with one reply and an optional preceding refusal.
/// # Errors
/// Preserve the actual owner, entropy or finite operation refusal.
pub(in crate::nts_ke_experiment) fn managed_component_exchange<'a>(
    association: &mut AssociationOwner<'a>,
    control: &'a Control,
    preceding: Option<&'static [u8]>,
    reply: &'static [u8],
    pre_send: Option<[Instant; 2]>,
    before_send: impl FnOnce() -> Result<(), Failure>,
) -> Result<(super::ProtectedCompletion<'a>, [SoftwareClock; 3]), Failure> {
    let completed: super::ProtectedCompletion<'a> = exchange_with(
        association,
        control,
        |output: &mut [u8; 56]| {
            before_send()?;
            entropy(output)
        },
        |peer: SocketAddr| {
            let reads: Vec<ScriptRead> = preceding
                .into_iter()
                .chain(std::iter::once(reply))
                .map(|packet: &'static [u8]| ScriptRead::Packet(packet, peer))
                .collect();
            let mut script: Script = Script::new(ScriptSend::Full, reads);
            if let Some([mono, mono_after]) = pre_send {
                script.send = ScriptSend::AfterWallStep;
                script.fixed_utc = Some(
                    UNIX_EPOCH
                        .checked_sub(Duration::from_secs(60))
                        .ok_or(Failure::Control)?,
                );
                script.send_before = Some(SoftwareClock {
                    mono,
                    utc: UNIX_EPOCH
                        .checked_sub(Duration::from_secs(120))
                        .ok_or(Failure::Control)?,
                    mono_after,
                });
                #[cfg(windows)]
                {
                    script.clock_refusals = [
                        Some(ClockReadError::Frequency(7)),
                        Some(ClockReadError::Before(5)),
                        Some(ClockReadError::After(9)),
                    ];
                }
            } else {
                // Fixed era-zero date shared by the admitted and refused model cases.
                script.fixed_utc = Some(
                    UNIX_EPOCH
                        .checked_add(Duration::from_secs(1_700_000_000))
                        .ok_or(Failure::Control)?,
                );
            }
            Ok(backend(peer, script))
        },
    )?;
    let original: &super::reached::OperationHistory<'a> = completed.history();
    let captures: [SoftwareClock; 3] = [
        original.send_before,
        original.send_after,
        original.read_return,
    ];
    Ok((completed, captures))
}

/// Inspect only this test build's closed backend.
/// # Errors
/// Refuse accidental native ownership instead of panicking or touching its socket.
#[cfg_attr(
    not(any(windows, target_os = "linux")),
    expect(
        clippy::unnecessary_wraps,
        reason = "Other targets retain the fallible signature of the Windows and Linux script inspector."
    )
)]
fn script(backend: &Backend) -> Result<&Script, Failure> {
    match &backend.io {
        Io::Script(script) => Ok(script),
        #[cfg(any(windows, target_os = "linux"))]
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
        assert_eq!((owner.status().attempts, owner.status().uids), (31, 1));
        assert_eq!(owner.status().available, 0);
        assert_eq!(owner.cookie_population()?, (0, 0));
        let admitted: Receipt = history.receipt;
        assert!(owner.last_receipt() == Some(admitted));
        let socket_lifetime = socket_lifetime.ok_or(Failure::Control)?;
        assert!(Arc::ptr_eq(
            completion.identity(),
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
        assert_eq!(owner.cookie_population()?, (1, 12));
        assert_eq!(owner.status().available, 1);
        assert!(owner.last_receipt() == Some(admitted));
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

/// Check ordinary protected receipt and cookie custody at the receive tail boundaries.
/// # Errors
/// Preserve owner setup, checked time and healthy final completion failures.
/// # Panics
/// Fail if a bad tail publishes history, changes spent work, or bypasses retirement.
#[test]
fn ordinary_protected_reversed_receive_bracket_cannot_commit() -> Result<(), Failure> {
    #[derive(Clone, Copy, Debug)]
    enum ReceiveTail {
        Observed,
        Reversed,
        AtExpiry,
        FutureBeforeExpiry,
    }

    for tail in [
        ReceiveTail::Observed,
        ReceiveTail::Reversed,
        ReceiveTail::AtExpiry,
        ReceiveTail::FutureBeforeExpiry,
    ] {
        with_owner(S2C, |owner, control, remote| {
            let expiry: Instant = control.expiry;
            let future: Instant = expiry
                .checked_sub(Duration::from_nanos(1))
                .ok_or(Failure::Control)?;
            let earlier: Instant = control
                .origin
                .checked_sub(Duration::from_nanos(1))
                .ok_or(Failure::Control)?;
            let receive_failure: Failure =
                Failure::Datagram(DatagramError::Receive(io::ErrorKind::ConnectionReset));
            let (trailing, expected, expected_residue): (
                Option<Instant>,
                Result<(), Failure>,
                usize,
            ) = match tail {
                ReceiveTail::Observed => (None, Ok(()), 1),
                ReceiveTail::Reversed => (Some(earlier), Err(receive_failure), 0),
                ReceiveTail::AtExpiry => (Some(expiry), Err(Failure::Deadline), 1),
                ReceiveTail::FutureBeforeExpiry => (Some(future), Err(receive_failure), 0),
            };
            let residue: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(usize::MAX));
            let mut lifetime: Option<std::sync::Weak<()>> = None;
            let result: Result<(), Failure> = exchange_with(owner, control, entropy, |peer| {
                assert_eq!(peer, remote);
                let mut script: Script = Script::new(
                    ScriptSend::Full,
                    vec![
                        ScriptRead::Packet(fixtures::HEALTHY, peer),
                        ScriptRead::Failed(io::ErrorKind::ConnectionReset),
                    ],
                );
                script.read_after = trailing;
                script.read_residue = Some(Arc::clone(&residue));
                lifetime = Some(Arc::downgrade(&script.lifetime));
                Ok(backend(peer, script))
            })
            .and_then(|completion: super::ProtectedCompletion<'_>| {
                completion.commit(owner).map(drop)
            });

            assert!(
                Instant::now() < future,
                "fixture processing reached the modeled future reading"
            );
            let status: Status = owner.status();
            assert_eq!((status.attempts, status.used, status.uids), (31, 1, 1));
            assert_eq!(
                (status.frames, status.bytes),
                (31, 131_072 - fixtures::HEALTHY.len())
            );
            assert_eq!(control.expiry, expiry);
            assert_eq!(lifetime.ok_or(Failure::Control)?.strong_count(), 0);
            let committed: bool = expected.is_ok();
            assert_eq!(
                owner.cookie_population()?,
                if committed { (1, 12) } else { (0, 0) },
                "tail={tail:?}"
            );
            assert_eq!(result, expected, "tail={tail:?}");
            assert_eq!(
                residue.load(Ordering::Relaxed),
                expected_residue,
                "tail={tail:?}"
            );
            if committed {
                assert_eq!(status.phase, RequestPhase::Idle);
                assert_eq!(owner.last_receipt().map(Receipt::serial), Some(1));
                assert_eq!(control.cancel()?, CancelOutcome::AlreadyCommitted);
            } else {
                assert_eq!(status.phase, RequestPhase::Pending);
                assert!(owner.last_receipt().is_none());
                drop(control.published_guard()?);
            }
            Ok(())
        })?;
    }
    Ok(())
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
    with_owner_cookies(
        Instant::now(),
        S2C,
        &[(0x11, 12), (0x22, 16)],
        |owner, control, remote| {
            let completion = exchange_with(owner, control, entropy, |admitted| {
                Ok(backend(
                    admitted,
                    Script::new(
                        ScriptSend::Full,
                        vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
                    ),
                ))
            })?;
            let admitted: Receipt = completion.history().receipt;
            assert_eq!(owner.cookie_population()?, (1, 16));
            assert!(owner.last_receipt() == Some(admitted));
            let _ticket = owner.reserve(RequestInput::Observed)?;
            let reserved = owner.status();
            assert_eq!(reserved.phase, RequestPhase::Reserved);
            assert_eq!(reserved.attempts, 30);
            let population: (usize, usize) = owner.cookie_population()?;
            assert_eq!(population, (0, 0));
            control.remaining(IoPhase::Exchange)?;
            assert!(
                matches!(
                    completion.commit(owner),
                    Err(Failure::Ledger(LedgerError::Ticket))
                ),
                "stale completion committed over a newer reservation"
            );
            assert_eq!(owner.status(), reserved);
            assert_eq!(owner.cookie_population()?, population);
            assert!(owner.last_receipt() == Some(admitted));
            assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
            Ok(())
        },
    )?;
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
        let admitted: Receipt = completion.history().receipt;
        assert_eq!(owner.cookie_population()?, (0, 0));
        assert!(matches!(
            owner.reserve(RequestInput::Observed),
            Err(Failure::Ledger(LedgerError::Cookies))
        ));
        let charged: Status = owner.status();
        assert_eq!((charged.phase, charged.attempts), (RequestPhase::Idle, 30));
        assert!(matches!(
            completion.commit(owner),
            Err(Failure::Ledger(LedgerError::Ticket))
        ));
        assert_eq!(owner.status(), charged);
        assert_eq!(owner.cookie_population()?, (0, 0));
        assert!(owner.last_receipt() == Some(admitted));
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
    with_owner_cookies(
        Instant::now(),
        S2C,
        &[(0x11, 12), (0x22, 16)],
        |owner, control, remote| {
            let completion = exchange_with(owner, control, entropy, |admitted| {
                Ok(backend(
                    admitted,
                    Script::new(
                        ScriptSend::Full,
                        vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
                    ),
                ))
            })?;
            let admitted: Receipt = completion.history().receipt;
            let population: (usize, usize) = owner.cookie_population()?;
            assert_eq!(population, (1, 16));
            let ticket = owner.reserve(RequestInput::Observed)?;
            owner.return_unused(ticket)?;
            let returned = owner.status();
            assert_eq!(returned.phase, RequestPhase::Idle);
            assert_eq!(returned.attempts, 30);
            assert!(owner.last_receipt() == Some(admitted));
            control.remaining(IoPhase::Exchange)?;
            assert!(
                matches!(
                    completion.commit(owner),
                    Err(Failure::Ledger(LedgerError::Ticket))
                ),
                "returned reservation revived stale completion"
            );
            assert_eq!(owner.status(), returned);
            assert_eq!(owner.cookie_population()?, population);
            assert!(owner.last_receipt() == Some(admitted));
            assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
            Ok(())
        },
    )
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
        assert_eq!(completion.history.receipt.cookie_counts(), (1, 1));
        Ok(())
    })
}

/// The two packets differ only in the final tag bit.
enum LastMixedFrame {
    /// The independently authenticated successor DENY.
    Deny,
    /// The same request coordinates with an invalid tag.
    BadTag,
}

/// Drive the shared receive owner to its final charged frame on the successor.
/// # Errors
/// Preserve real fixture setup, control and unexpected receive failures.
/// # Panics
/// Fail on an early retirement, lost cause, refunded work or premature effect.
fn check_mixed_last_frame(last: &LastMixedFrame) -> Result<(), Failure> {
    use crate::SourcePolicySnapshot;
    use crate::nts_ke_experiment::association::ledger::{RecoveryIntent, RecoveryReason};

    with_managed_successor(|owner, control, remote, _authority, source| {
        let expiry: Instant = control.expiry;
        let first: super::ProtectedCompletion<'_> = first_managed_exchange(owner, control, 0)?;
        let first_receipt: Receipt = first.history().receipt;
        first.commit_for_successor(owner)?;
        assert_eq!((owner.status().frames, owner.status().bytes), (31, 130_932));
        assert_eq!(owner.cookie_population()?, (1, 12));
        assert!(owner.take_recovery_intent().is_none());

        let packet: &'static [u8; 124] = match last {
            LastMixedFrame::Deny => &PROTECTED_DENY,
            LastMixedFrame::BadTag => &DENY_BAD_TAG,
        };
        // The single-thread driver only publishes the depth immediately after a real read.
        let residue: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(32));
        let selected: Result<RetiredProtectedResponse<'_>, Failure> =
            exchange_successor_mixed(owner, control, successor_entropy, |peer: SocketAddr| {
                assert_eq!(peer, remote);
                let mut reads: Vec<ScriptRead> = (0..30)
                    .map(|_| ScriptRead::Packet(&PROTECTED_UNKNOWN, peer))
                    .collect();
                reads.push(ScriptRead::Packet(packet, peer));
                reads.push(ScriptRead::Packet(fixtures::SUCCESSOR_OLD_S2C, peer));
                let mut script: Script = Script::new(ScriptSend::Full, reads);
                script.read_residue = Some(Arc::clone(&residue));
                script
            });
        let phase: RequestPhase = match last {
            LastMixedFrame::Deny => {
                let RetiredProtectedResponse::Control(retired) = selected? else {
                    return Err(Failure::Control);
                };
                retired.check_current(owner)?;
                RequestPhase::Pending
            }
            LastMixedFrame::BadTag => {
                assert_eq!(
                    selected.err(),
                    Some(Failure::Ledger(LedgerError::Reply(ReplyError::Open(
                        StagedDecryptError::Authentication,
                    ))))
                );
                let recovery: RecoveryIntent =
                    owner.take_recovery_intent().ok_or(Failure::Control)?;
                assert_eq!(recovery.reason(), RecoveryReason::ReceiveWork);
                assert_eq!(recovery.not_before(), expiry);
                RequestPhase::Idle
            }
        };
        assert_eq!(residue.load(Ordering::Relaxed), 1);
        assert!(owner.take_recovery_intent().is_none());
        // One 140-byte first reply and thirty-one 124-byte successor reads.
        assert_eq!(
            owner.status(),
            Status {
                phase,
                attempts: 30,
                frames: 0,
                bytes: 127_088,
                available: 0,
                used: 2,
                uids: 2,
            }
        );
        assert!(owner.last_receipt() == Some(first_receipt));
        assert_eq!(owner.cookie_population()?, (0, 0));
        assert_eq!(control.expiry, expiry);
        let policy: SourcePolicySnapshot = source
            .policy_snapshot(remote)
            .map_err(Failure::SourcePolicy)?;
        assert!(!policy.source().refused() && !policy.endpoint().refused());
        assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
        Ok(())
    })
}

/// A genuine DENY still stages when the final frame spends the last credit.
/// # Errors
/// Preserve actual boundary, receive, authentication and fixture failures.
#[test]
fn managed_mixed_receive_admits_deny_on_final_frame() -> Result<(), Failure> {
    check_mixed_last_frame(&LastMixedFrame::Deny)
}

/// A failed final tag retires receive work and leaves the healthy neighbor queued.
/// # Errors
/// Preserve actual boundary, receive, authentication and fixture failures.
#[test]
fn managed_mixed_receive_exhausts_on_invalid_final_frame() -> Result<(), Failure> {
    check_mixed_last_frame(&LastMixedFrame::BadTag)
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
            assert_eq!(backend.send.is_some(), after);
            let status: Status = owner.status();
            assert_eq!((status.used, status.available), (usize::from(after), 0));
            let phase: RequestPhase = if after {
                RequestPhase::Idle
            } else {
                RequestPhase::Prepared
            };
            assert_eq!(status.phase, phase);
            assert!(owner.last_receipt().is_none());
            Ok(())
        })?;
    }
    Ok(())
}

/// Check capture order and cutoff at the actual protected sender boundary.
/// # Errors
/// Preserve unexpected fixture setup or healthy-path failure.
/// # Panics
/// Fail on a wrong refusal, a forbidden send, or lost ledger/capture custody.
#[test]
fn protected_send_requires_an_ordered_bracket_before_the_original_deadline() -> Result<(), Failure>
{
    enum Case {
        Ordered,
        Touching,
        Reversed,
        Cutoff,
    }

    let open: Instant = Instant::now();
    let origin: Instant = open
        .checked_sub(Duration::from_millis(1))
        .ok_or(Failure::Deadline)?;
    for case in [Case::Ordered, Case::Touching, Case::Reversed, Case::Cutoff] {
        with_owner_start(origin, S2C, |owner, control, remote| {
            let deadline: Instant = control.expiry;
            let (mono, mono_after, refusal): (Instant, Instant, Option<Failure>) = match case {
                Case::Ordered => (origin, open, None),
                Case::Touching => (open, open, None),
                Case::Reversed => (
                    deadline,
                    open,
                    Some(Failure::Datagram(DatagramError::Capture)),
                ),
                Case::Cutoff => (
                    deadline
                        .checked_sub(Duration::from_nanos(1))
                        .ok_or(Failure::Deadline)?,
                    deadline,
                    Some(Failure::Deadline),
                ),
            };
            let before: SoftwareClock = SoftwareClock {
                mono,
                utc: UNIX_EPOCH,
                mono_after,
            };
            let mut selected: Script = Script::new(
                ScriptSend::Full,
                vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
            );
            selected.send_before = Some(before);
            let mut backend: Backend = backend(remote, selected);
            let original: Status = owner.status();
            let result: Result<OperationCompletion<'_>, Failure> =
                runtime()?.block_on(operation(owner, control, entropy, &mut backend));
            let script: &Script = script(&backend)?;
            let status: Status = owner.status();

            assert_eq!(control.expiry, deadline);
            assert_eq!(status.used, 1);
            if let Some(expected) = refusal {
                assert_eq!(
                    (script.send_calls, script.receive_calls),
                    (0, 0),
                    "a refused pre-send capture must never enter the synchronous adapter"
                );
                assert_eq!(result.err(), Some(expected));
                assert!(backend.send.is_none());
                assert!(owner.last_receipt().is_none());
                assert_eq!(status.phase, RequestPhase::Invoked);
                assert_eq!(status.available, 0);
                assert_eq!(status.frames, original.frames);
                assert_eq!(status.bytes, original.bytes);
            } else {
                let completion: OperationCompletion<'_> = result?;
                assert_eq!((script.send_calls, script.receive_calls), (1, 1));
                let receipt: Receipt = completion.history.receipt;
                assert_eq!(receipt.fields().header().origin_timestamp().bits(), WIRE);
                assert_eq!(completion.history.send_before.monotonic(), mono);
                assert_eq!(completion.history.send_before.monotonic_after(), mono_after);
                assert_eq!(completion.history.send_before.utc(), UNIX_EPOCH);
            }
            Ok(())
        })?;
    }
    Ok(())
}

/// Use independent before/exact floor observations at the actual guarded adapter.
/// # Errors
/// Preserve setup refusal or an unexpected adapter/packet failure.
/// # Panics
/// Reject a premature syscall, missing exact-neighbor send, or floor double debit.
#[test]
fn managed_protected_send_uses_the_exact_shared_floor() -> Result<(), Failure> {
    use crate::source_policy::{FixtureRequest, SourceBinding, fixture_send, isolated_authority};
    use crate::{
        SourceAddressSet, SourceHandle, SourcePolicyError, SourcePolicyLimits, SourcePolicySnapshot,
    };

    let interval: Duration = Duration::from_millis(4);
    let tick: Duration = Duration::from_nanos(1);
    let now: Instant = Instant::now();
    let floor: Instant = now.checked_sub(interval).ok_or(Failure::Deadline)?;
    let sent_at: Instant = floor.checked_sub(interval).ok_or(Failure::Deadline)?;
    let start: Instant = sent_at.checked_sub(interval).ok_or(Failure::Deadline)?;
    for exact in [false, true] {
        let limits: SourcePolicyLimits =
            SourcePolicyLimits::new(2, 1, interval, interval).map_err(Failure::SourcePolicy)?;
        let mut authority: SourcePolicyAuthority =
            isolated_authority(limits).map_err(Failure::SourcePolicy)?;
        let source: SourceHandle = authority.source(0).map_err(Failure::SourcePolicy)?;
        let alias: SourceHandle = authority.source(1).map_err(Failure::SourcePolicy)?;
        let remote: SocketAddr = "127.0.0.1:123".parse().map_err(|_| Failure::Endpoint)?;
        let peer: SocketAddr = "127.0.0.1:4460".parse().map_err(|_| Failure::Endpoint)?;
        let target: KePreviewTarget =
            KePreviewTarget::new(peer, "Time.Example").map_err(|_| Failure::Arguments)?;
        let addresses: SourceAddressSet =
            SourceAddressSet::new(&[remote]).map_err(Failure::SourcePolicy)?;
        let expected: SourceConfigurationSnapshot = authority
            .install_ke_preview_configuration(&source, addresses, target)
            .map_err(Failure::SourcePolicy)?;
        let binding: SourceBinding = alias.bind(remote).map_err(Failure::SourcePolicy)?;
        let earlier: Arc<()> = Arc::new(());
        let _earlier_request: FixtureRequest =
            fixture_send(&binding, &earlier, sent_at).map_err(Failure::SourcePolicy)?;
        let config: Config = Config::from_managed_protected(
            authority
                .authorize_protected_ke(&expected)
                .map_err(Failure::SourcePolicy)?,
        );
        with_config_owner(
            &config,
            start,
            S2C,
            &[(0x11, 12)],
            |owner, control, negotiated| {
                let observation: Instant = if exact {
                    floor
                } else {
                    floor.checked_sub(tick).ok_or(Failure::Deadline)?
                };
                let mut selected: Script = Script::new(
                    ScriptSend::Full,
                    vec![ScriptRead::Packet(fixtures::HEALTHY, negotiated)],
                );
                selected.send_before = Some(SoftwareClock {
                    mono: observation,
                    mono_after: observation,
                    utc: UNIX_EPOCH,
                });
                let mut backend: Backend = backend(negotiated, selected);
                let outcome: Result<OperationCompletion<'_>, Failure> =
                    runtime()?.block_on(operation(owner, control, entropy, &mut backend));
                let actual: SourcePolicySnapshot = source
                    .policy_snapshot(negotiated)
                    .map_err(Failure::SourcePolicy)?;
                let endpoint_floor: Instant = if exact { now } else { floor };
                let source_floor: Option<Instant> = if exact { Some(now) } else { None };
                let alias_snapshot: SourcePolicySnapshot = alias
                    .policy_snapshot(negotiated)
                    .map_err(Failure::SourcePolicy)?;
                assert_eq!(alias_snapshot.source().floor(), Some(floor));
                assert_eq!(actual.source().floor(), source_floor);
                assert_eq!(actual.endpoint().floor(), Some(endpoint_floor));
                let observed: &Script = script(&backend)?;
                if exact {
                    assert!(outcome.is_ok());
                    assert_eq!((observed.send_calls, observed.receive_calls), (1, 1));
                } else {
                    assert!(matches!(
                        outcome,
                        Err(Failure::SourcePolicy(SourcePolicyError::NotDue))
                    ));
                    assert_eq!((observed.send_calls, observed.receive_calls), (0, 0));
                    assert_eq!(owner.status().phase, RequestPhase::Invoked);
                }
                assert_eq!(owner.status().used, 1);
                assert_eq!(owner.cookie_population()?, (0, 0));
                Ok(())
            },
        )?;
    }
    Ok(())
}

/// Let a separate alias debit the shared record at the first eligible observation.
/// # Errors
/// Preserve independent policy reservation or floor arithmetic refusal.
async fn debit_alias_at_floor(
    binding: &crate::source_policy::SourceBinding,
    floor: Instant,
    interval: Duration,
) -> Result<Instant, Failure> {
    use crate::source_policy::{FixtureRequest, fixture_send};

    tokio::time::sleep_until(tokio::time::Instant::from_std(floor)).await;
    let attempt: Instant = Instant::now();
    let _request: FixtureRequest =
        fixture_send(binding, &Arc::new(()), attempt).map_err(Failure::SourcePolicy)?;
    attempt.checked_add(interval).ok_or(Failure::Deadline)
}

/// Park the real managed driver and recheck an independently extended endpoint floor.
/// # Errors
/// Preserve setup, original time, budget or packet-admission refusal.
/// # Panics
/// Fail if the test starts after its floor, the driver prepares before parking,
/// or the resumed operation does not debit and invoke exactly once.
#[test]
fn managed_protected_policy_wait_checks_out_cookie_only_when_due() -> Result<(), Failure> {
    use std::cell::Cell;
    use std::future::Future;
    use std::task::Poll;

    use crate::source_policy::SourceBinding;
    use crate::{SourceHandle, SourcePolicySnapshot};

    let start: Instant = Instant::now();
    let interval: Duration = Duration::from_millis(200);
    let maximum: Duration = Duration::from_millis(400);
    let (config, source, authority): (Config, SourceHandle, SourcePolicyAuthority) =
        managed_attempt_config(start, interval, maximum)?;
    with_config_owner(
        &config,
        start,
        S2C,
        &[(0x11, 12)],
        |owner, control, remote| {
            let before: SourcePolicySnapshot = source
                .policy_snapshot(remote)
                .map_err(Failure::SourcePolicy)?;
            let floor: Instant = start.checked_add(maximum).ok_or(Failure::Deadline)?;
            assert_eq!(before.endpoint().floor(), Some(floor));
            assert_eq!(owner.cookie_population()?, (1, 12));
            let alias: SourceHandle = authority.source(1).map_err(Failure::SourcePolicy)?;
            let binding: SourceBinding = alias.bind(remote).map_err(Failure::SourcePolicy)?;
            let next_floor: Cell<Instant> = Cell::new(floor);
            let entropy_calls: Cell<usize> = Cell::new(0);
            let mut backend: Backend = backend(
                remote,
                Script::new(
                    ScriptSend::Full,
                    vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
                ),
            );
            runtime()?.block_on(async {
                let pending = operation(
                    owner,
                    control,
                    |output| {
                        assert!(Instant::now() >= next_floor.get());
                        entropy_calls.set(entropy_calls.get() + 1);
                        entropy(output)
                    },
                    &mut backend,
                );
                tokio::pin!(pending);
                std::future::poll_fn(|context| {
                    assert!(Instant::now() < floor);
                    assert!(pending.as_mut().poll(context).is_pending());
                    assert_eq!(entropy_calls.get(), 0);
                    Poll::Ready(())
                })
                .await;
                next_floor.set(debit_alias_at_floor(&binding, floor, maximum).await?);
                tokio::time::timeout(Duration::from_secs(2), pending)
                    .await
                    .map_err(|_| Failure::Control)??;
                Ok::<(), Failure>(())
            })?;
            let after: SourcePolicySnapshot = source
                .policy_snapshot(remote)
                .map_err(Failure::SourcePolicy)?;
            let adapter: &Script = script(&backend)?;
            assert_eq!(
                (
                    entropy_calls.get(),
                    adapter.send_calls,
                    adapter.receive_calls
                ),
                (1, 1, 1)
            );
            assert_eq!(owner.status().used, 1);
            assert_eq!(owner.cookie_population()?, (0, 0));
            assert_eq!(
                after
                    .source()
                    .revision()
                    .checked_sub(before.source().revision()),
                Some(1)
            );
            assert_eq!(
                after
                    .endpoint()
                    .revision()
                    .checked_sub(before.endpoint().revision()),
                Some(2)
            );
            let (send_before, _, _): (
                super::clock_capture::ApplicationClockCapture,
                super::clock_capture::ApplicationClockCapture,
                usize,
            ) = backend.send.ok_or(Failure::Control)?;
            let sent: Instant = send_before.software().monotonic_after();
            assert!(sent >= next_floor.get() && sent < control.expiry);
            assert_eq!(after.source().floor(), sent.checked_add(interval));
            assert_eq!(after.endpoint().floor(), sent.checked_add(maximum));
            Ok(())
        },
    )
}

/// Actual events independent of the other's refusal and the future floor timer.
#[derive(Clone, Copy)]
enum ManagedWaitRefusal {
    Stop,
    ReplaceConfiguration,
    Expiry,
    StopLatchedBeforeExpiry,
}

/// Replace the original definition with equal values while the driver is parked.
/// # Errors
/// Preserve missing ownership or actual current revision refusal.
/// # Panics
/// Fail if replacement does not issue a different configuration revision.
fn replace_policy_wait_configuration(
    authority: &mut SourcePolicyAuthority,
    original: &Config,
) -> Result<(), Failure> {
    let definition: &crate::source_policy::ConfiguredKePreview =
        original.mode.source_definition().ok_or(Failure::Control)?;
    let expected: &SourceConfigurationSnapshot = definition.snapshot();
    let replacement: SourceConfigurationSnapshot = authority
        .replace_ke_preview_configuration(
            expected,
            expected.addresses().clone(),
            definition.target().clone(),
        )
        .map_err(Failure::SourcePolicy)?;
    assert!(!replacement.same_configuration(expected));
    Ok(())
}

/// Hold the Stop request between its actual latch write and its notification.
/// # Errors
/// Preserve original scope lock failure.
/// # Panics
/// Fail if the latch was already set before this fixture event.
fn latch_wait_stop_without_wake(control: &Control) -> Result<(), Failure> {
    let mut stopped = control
        .stop_scope
        .stopped
        .lock()
        .map_err(|_| Failure::Control)?;
    assert!(!*stopped);
    *stopped = true;
    Ok(())
}

/// Compare policy records without comparing their time-dependent diagnostic wait.
/// # Panics
/// Fail if pre-send waiting or refusal changed either record.
fn assert_policy_wait_kept_records(
    before: &crate::SourcePolicySnapshot,
    after: &crate::SourcePolicySnapshot,
) {
    for (old, current) in [
        (before.source(), after.source()),
        (before.endpoint(), after.endpoint()),
    ] {
        assert_eq!(
            (current.floor(), current.interval(), current.revision()),
            (old.floor(), old.interval(), old.revision())
        );
    }
}

/// Refuse at the actual pre-cookie await after a parked operation observes one event.
/// # Errors
/// Preserve setup, expiry arithmetic, event or watchdog refusal.
/// # Panics
/// Fail on an incorrect named reason, changed policy or any pre-eligibility work.
fn check_managed_policy_wait_refusal(cause: ManagedWaitRefusal) -> Result<(), Failure> {
    use std::cell::Cell;
    use std::future::Future;
    use std::task::Poll;

    use crate::{SourceHandle, SourcePolicyError, SourcePolicySnapshot};

    let start: Instant = Instant::now();
    let (config, source, mut authority): (Config, SourceHandle, SourcePolicyAuthority) =
        managed_attempt_config(start, Duration::from_secs(2), Duration::from_secs(4))?;
    let cutoff: Duration = match cause {
        ManagedWaitRefusal::Expiry | ManagedWaitRefusal::StopLatchedBeforeExpiry => {
            Duration::from_millis(250)
        }
        ManagedWaitRefusal::Stop | ManagedWaitRefusal::ReplaceConfiguration => {
            Duration::from_secs(6)
        }
    };
    let expiry: Instant = start.checked_add(cutoff).ok_or(Failure::Deadline)?;
    let control: Control = Control::clamped(start, expiry)?;
    with_config_control(
        &config,
        &control,
        S2C,
        &[(0x11, 12)],
        |owner, control, remote| {
            let status: Status = owner.status();
            let before: SourcePolicySnapshot = source
                .policy_snapshot(remote)
                .map_err(Failure::SourcePolicy)?;
            let calls: Cell<usize> = Cell::new(0);
            let mut backend: Backend = backend(remote, Script::new(ScriptSend::Full, Vec::new()));
            let result: Result<OperationCompletion<'_>, Failure> = runtime()?.block_on(async {
                let pending = operation(
                    owner,
                    control,
                    |output| {
                        calls.set(calls.get() + 1);
                        entropy(output)
                    },
                    &mut backend,
                );
                tokio::pin!(pending);
                std::future::poll_fn(|context| {
                    assert!(pending.as_mut().poll(context).is_pending());
                    assert_eq!(calls.get(), 0);
                    Poll::Ready(())
                })
                .await;
                match cause {
                    ManagedWaitRefusal::Stop => {
                        let stop: StopHandle = StopHandle {
                            scope: Arc::clone(&control.stop_scope),
                        };
                        assert_eq!(stop.request_stop()?, StopDisposition::Latched);
                    }
                    ManagedWaitRefusal::ReplaceConfiguration => {
                        replace_policy_wait_configuration(&mut authority, &config)?;
                    }
                    ManagedWaitRefusal::Expiry => {}
                    ManagedWaitRefusal::StopLatchedBeforeExpiry => {
                        latch_wait_stop_without_wake(control)?;
                    }
                }
                tokio::time::timeout(Duration::from_secs(2), pending)
                    .await
                    .map_err(|_| Failure::Control)?
            });
            let expected: Failure = match cause {
                ManagedWaitRefusal::Stop | ManagedWaitRefusal::StopLatchedBeforeExpiry => {
                    Failure::Cancelled
                }
                ManagedWaitRefusal::ReplaceConfiguration => {
                    Failure::SourcePolicy(SourcePolicyError::ConfigurationChanged)
                }
                ManagedWaitRefusal::Expiry => Failure::Deadline,
            };
            assert_eq!(result.err(), Some(expected));
            if matches!(cause, ManagedWaitRefusal::StopLatchedBeforeExpiry) {
                assert!(Instant::now() >= expiry);
            }
            assert_eq!(control.expiry, expiry);
            assert_eq!(owner.status(), status);
            assert_eq!(owner.cookie_population()?, (1, 12));
            let adapter: &Script = script(&backend)?;
            assert_eq!(
                (calls.get(), adapter.send_calls, adapter.receive_calls),
                (0, 0, 0)
            );
            let after: SourcePolicySnapshot = source
                .policy_snapshot(remote)
                .map_err(Failure::SourcePolicy)?;
            assert_policy_wait_kept_records(&before, &after);
            Ok(())
        },
    )
}

/// Check each independent terminal event after the original driver has parked.
/// # Errors
/// Preserve the first unexpected bounded refusal history.
#[test]
fn managed_protected_policy_wait_refuses_before_cookie_checkout() -> Result<(), Failure> {
    for cause in [
        ManagedWaitRefusal::Stop,
        ManagedWaitRefusal::ReplaceConfiguration,
        ManagedWaitRefusal::Expiry,
        ManagedWaitRefusal::StopLatchedBeforeExpiry,
    ] {
        check_managed_policy_wait_refusal(cause)?;
    }
    Ok(())
}

/// Refuse when a real first exchange leaves only enough work for one policy park.
/// # Errors
/// Preserve ordinary reply, current association, clock or policy refusal.
/// # Panics
/// Fail if a park on the original send floor can reserve a cookie or enter entropy
/// when the carried allowance can no longer reach the required write wait.
#[test]
fn managed_protected_policy_wait_last_credit_refuses_before_cookie_checkout() -> Result<(), Failure>
{
    use std::cell::Cell;

    use crate::{SourceHandle, SourcePolicySnapshot};

    let start: Instant = Instant::now()
        .checked_sub(Duration::from_secs(2))
        .ok_or(Failure::Deadline)?;
    let (config, source, _authority): (Config, SourceHandle, SourcePolicyAuthority) =
        managed_attempt_config(
            start,
            Duration::from_millis(250),
            Duration::from_millis(500),
        )?;
    with_config_owner(
        &config,
        start,
        S2C,
        &[(0x11, 12)],
        |owner, control, remote| {
            let first: super::ProtectedCompletion<'_> =
                exchange_with(owner, control, entropy, |peer: SocketAddr| {
                    // Write, read, 30 WouldBlock/read-ready pairs and one valid datagram spend 63.
                    let mut reads: Vec<ScriptRead> =
                        (0..30).map(|_| ScriptRead::WouldBlock).collect();
                    reads.push(ScriptRead::Packet(fixtures::HEALTHY, peer));
                    Ok(backend(peer, Script::new(ScriptSend::Full, reads)))
                })?;
            assert_eq!(first.remaining_attempts(), 1);
            first.commit_for_successor(owner)?;
            let before: Status = owner.status();
            let receipt: Option<Receipt> = owner.last_receipt();
            let before_policy: SourcePolicySnapshot = source
                .policy_snapshot(remote)
                .map_err(Failure::SourcePolicy)?;
            let floor: Instant = before_policy.endpoint().floor().ok_or(Failure::Control)?;
            assert!(Instant::now() < floor && floor < control.expiry);
            assert_eq!(owner.cookie_population()?, (1, 12));
            let entropy_calls: Cell<usize> = Cell::new(0);
            let result: Result<RetiredProtectedResponse<'_>, Failure> = exchange_successor_mixed(
                owner,
                control,
                |output| {
                    entropy_calls.set(entropy_calls.get() + 1);
                    successor_entropy(output)
                },
                |_| Script::new(ScriptSend::Full, Vec::new()),
            );
            assert!(matches!(
                result,
                Err(Failure::Datagram(DatagramError::Attempts))
            ));
            assert!(Instant::now() >= floor);
            assert_eq!(entropy_calls.get(), 0);
            assert_eq!(owner.status(), before);
            assert!(owner.last_receipt() == receipt);
            assert_eq!(owner.cookie_population()?, (1, 12));
            let after: SourcePolicySnapshot = source
                .policy_snapshot(remote)
                .map_err(Failure::SourcePolicy)?;
            assert_policy_wait_kept_records(&before_policy, &after);
            Ok(())
        },
    )
}

/// Retain both independently advanced managed floors across every attempted send.
/// # Errors
/// Preserve the first unsuccessful component history.
#[test]
fn managed_protected_attempts_keep_floors_after_failure_and_stop() -> Result<(), Failure> {
    let cases: [(ScriptSend, bool, Option<Failure>, RequestPhase); 8] = [
        (ScriptSend::Full, false, None, RequestPhase::Idle),
        (
            ScriptSend::Short,
            false,
            Some(Failure::Ledger(LedgerError::ShortSend)),
            RequestPhase::Idle,
        ),
        (
            ScriptSend::Failed(io::ErrorKind::WouldBlock),
            false,
            Some(Failure::Ledger(LedgerError::Send(
                io::ErrorKind::WouldBlock,
            ))),
            RequestPhase::Idle,
        ),
        (
            ScriptSend::Failed(io::ErrorKind::PermissionDenied),
            false,
            Some(Failure::Ledger(LedgerError::Send(
                io::ErrorKind::PermissionDenied,
            ))),
            RequestPhase::Idle,
        ),
        (
            ScriptSend::Unknown,
            false,
            Some(Failure::Ledger(LedgerError::Unknown)),
            RequestPhase::Invoked,
        ),
        (
            ScriptSend::ScopeStopBeforeGuard,
            false,
            Some(Failure::Cancelled),
            RequestPhase::Prepared,
        ),
        (
            ScriptSend::CancelAfterFull,
            false,
            Some(Failure::Cancelled),
            RequestPhase::Idle,
        ),
        (
            ScriptSend::Full,
            true,
            Some(Failure::Cancelled),
            RequestPhase::Pending,
        ),
    ];
    for (send, stop_on_read, expected_failure, phase) in cases {
        check_managed_attempt(send, stop_on_read, expected_failure, phase)?;
    }
    Ok(())
}

/// Issue one protected configuration with an independent alias RATE control.
/// # Errors
/// Preserve genuine issuer, binding, allowance and control refusal.
pub(in crate::nts_ke_experiment) fn managed_attempt_config(
    earlier: Instant,
    interval: Duration,
    maximum: Duration,
) -> Result<(Config, crate::SourceHandle, SourcePolicyAuthority), Failure> {
    use crate::source_policy::{
        FixtureRequest, SourceBinding, fixture_control, fixture_send, isolated_authority,
    };
    use crate::{SourceAddressSet, SourceHandle, SourcePolicyLimits};

    let limits: SourcePolicyLimits =
        SourcePolicyLimits::new(2, 2, interval, maximum).map_err(Failure::SourcePolicy)?;
    let mut authority: SourcePolicyAuthority =
        isolated_authority(limits).map_err(Failure::SourcePolicy)?;
    let source: SourceHandle = authority.source(0).map_err(Failure::SourcePolicy)?;
    let alias: SourceHandle = authority.source(1).map_err(Failure::SourcePolicy)?;
    let remote: SocketAddr = "127.0.0.1:123".parse().map_err(|_| Failure::Endpoint)?;
    let tls: SocketAddr = "127.0.0.1:4460".parse().map_err(|_| Failure::Endpoint)?;
    let target: KePreviewTarget =
        KePreviewTarget::new(tls, "Time.Example").map_err(|_| Failure::Arguments)?;
    let addresses: SourceAddressSet =
        SourceAddressSet::new(&[remote]).map_err(Failure::SourcePolicy)?;
    let original: SourceConfigurationSnapshot = authority
        .install_ke_preview_configuration(&source, addresses, target)
        .map_err(Failure::SourcePolicy)?;
    let binding: SourceBinding = alias.bind(remote).map_err(Failure::SourcePolicy)?;
    let request: Arc<()> = Arc::new(());
    let mut alias_send: FixtureRequest =
        fixture_send(&binding, &request, earlier).map_err(Failure::SourcePolicy)?;
    fixture_control(
        &binding,
        &mut alias_send,
        u32::from_be_bytes(*b"RATE"),
        earlier,
    )
    .map_err(Failure::SourcePolicy)?;
    let config: Config = Config::from_managed_protected(
        authority
            .authorize_protected_ke(&original)
            .map_err(Failure::SourcePolicy)?,
    );
    Ok((config, source, authority))
}

/// Observe actual adapter entry, receive custody and retained policy after one attempt.
/// # Errors
/// Preserve fixture, timing, runtime and policy inspection refusal.
/// # Panics
/// Reject any unexpected adapter result, duplicate entry, receipt or debit.
fn check_managed_attempt(
    send: ScriptSend,
    stop_on_read: bool,
    expected_failure: Option<Failure>,
    phase: RequestPhase,
) -> Result<(), Failure> {
    use crate::{SourceHandle, SourcePolicySnapshot};

    let interval: Duration = Duration::from_millis(4);
    let doubled: Duration = Duration::from_millis(8);
    let observation: Instant = Instant::now();
    let earlier: Instant = observation
        .checked_sub(Duration::from_millis(20))
        .ok_or(Failure::Deadline)?;
    let source_floor: Instant = observation.checked_add(interval).ok_or(Failure::Deadline)?;
    let endpoint_floor: Instant = observation.checked_add(doubled).ok_or(Failure::Deadline)?;
    let prior_endpoint_floor: Instant = earlier.checked_add(doubled).ok_or(Failure::Deadline)?;
    let (config, source, _authority): (Config, SourceHandle, SourcePolicyAuthority) =
        managed_attempt_config(earlier, interval, doubled)?;
    with_config_owner(
        &config,
        earlier,
        S2C,
        &[(0x11, 12)],
        |owner, control, negotiated| {
            let before: SourcePolicySnapshot = source
                .policy_snapshot(negotiated)
                .map_err(Failure::SourcePolicy)?;
            assert_eq!(before.source().floor(), None);
            assert_eq!(before.endpoint().floor(), Some(prior_endpoint_floor));

            let mut selected: Script = Script::new(
                send,
                vec![ScriptRead::Packet(fixtures::HEALTHY, negotiated)],
            );
            selected.send_before = Some(SoftwareClock {
                mono: observation,
                mono_after: observation,
                utc: UNIX_EPOCH,
            });
            if stop_on_read {
                selected
                    .ready
                    .extend([ScriptReady::Ready, ScriptReady::ScopeStop]);
            }
            let mut backend: Backend = backend(negotiated, selected);
            let outcome: Result<OperationCompletion<'_>, Failure> =
                runtime()?.block_on(operation(owner, control, entropy, &mut backend));
            assert_eq!(outcome.err(), expected_failure);

            let entered: bool = !matches!(send, ScriptSend::ScopeStopBeforeGuard);
            let received: bool = expected_failure.is_none();
            let script: &Script = script(&backend)?;
            assert_eq!(
                (script.send_calls, script.receive_calls),
                (usize::from(entered), usize::from(received))
            );
            assert!(script.ready.is_empty());
            assert_eq!(backend.send.is_some(), entered);
            drop(backend);

            assert_eq!(
                owner.status(),
                Status {
                    phase,
                    attempts: 31,
                    frames: 32 - usize::from(received),
                    bytes: 131_072 - usize::from(received) * fixtures::HEALTHY.len(),
                    available: 0,
                    used: usize::from(entered),
                    uids: 1,
                }
            );
            assert_eq!(owner.last_receipt().is_some(), received);
            assert_eq!(owner.cookie_population()?, (0, 0));
            if matches!(send, ScriptSend::Unknown) {
                assert!(matches!(
                    owner.reserve(RequestInput::Observed),
                    Err(Failure::Ledger(LedgerError::Busy))
                ));
            }

            let after: SourcePolicySnapshot = source
                .policy_snapshot(negotiated)
                .map_err(Failure::SourcePolicy)?;
            assert_eq!(after.source().floor(), entered.then_some(source_floor));
            assert_eq!(
                after.source().revision(),
                before.source().revision() + u64::from(entered)
            );
            assert_eq!(
                after.endpoint().floor(),
                Some(if entered {
                    endpoint_floor
                } else {
                    prior_endpoint_floor
                })
            );
            assert_eq!(
                after.endpoint().revision(),
                before.endpoint().revision() + u64::from(entered)
            );
            Ok(())
        },
    )
}

/// Select exactly one record for a real administrative epoch advance.
#[derive(Clone, Copy)]
enum ManagedEpochTarget {
    /// The logical source that sent the protected request.
    Source,
    /// The negotiated endpoint, reached by a different wire representation.
    MappedEndpoint,
    /// A different logical source within the same authority.
    OtherSource,
    /// A different port on the same address.
    OtherPort,
}

/// Stage one exact-endpoint reply after charging the wrong-wire alias.
/// # Errors
/// Preserve the actual checked exchange or retirement failure.
/// # Panics
/// Reject a substituted wire peer, refunded work or premature cookie installation.
fn stage_exact_managed_reply<'a>(
    owner: &mut AssociationOwner<'_>,
    control: &'a Control,
    remote: SocketAddr,
    mapped: SocketAddr,
) -> Result<super::ProtectedCompletion<'a>, Failure> {
    let completion: super::ProtectedCompletion<'_> =
        exchange_with(owner, control, entropy, |peer: SocketAddr| {
            assert_eq!(peer, remote);
            Ok(backend(
                peer,
                Script::new(
                    ScriptSend::Full,
                    vec![
                        ScriptRead::Packet(fixtures::HEALTHY, mapped),
                        ScriptRead::Packet(fixtures::HEALTHY, remote),
                    ],
                ),
            ))
        })?;
    let charged: Status = owner.status();
    assert_eq!(charged.phase, RequestPhase::Idle);
    assert_eq!((charged.attempts, charged.used, charged.uids), (31, 1, 1));
    assert_eq!(
        charged.frames, 30,
        "mapped wire peer must spend work and be refused"
    );
    assert_eq!(charged.bytes, 131_072 - 2 * fixtures::HEALTHY.len());
    assert_eq!(owner.cookie_population()?, (0, 0));
    Ok(completion)
}

/// Stage the exact reply, then advance only the selected policy record.
/// # Errors
/// Preserve unexpected issuer, adapter, administrative or final-commit refusal.
/// # Panics
/// Reject wire alias substitution, epoch leakage or premature publication.
fn check_managed_protected_reply_epoch(
    change: ManagedEpochTarget,
    invalidates: bool,
) -> Result<(), Failure> {
    use std::net::Ipv4Addr;

    use crate::source_policy::{SourceBinding, isolated_authority};
    use crate::{
        SourceAddressSet, SourceHandle, SourcePolicyError, SourcePolicyLimits,
        SourcePolicySnapshot, SourcePolicyTarget,
    };

    let interval: Duration = Duration::from_secs(10);
    let limits: SourcePolicyLimits =
        SourcePolicyLimits::new(2, 2, interval, interval).map_err(Failure::SourcePolicy)?;
    let mut authority: SourcePolicyAuthority =
        isolated_authority(limits).map_err(Failure::SourcePolicy)?;
    let source: SourceHandle = authority.source(0).map_err(Failure::SourcePolicy)?;
    let other: SourceHandle = authority.source(1).map_err(Failure::SourcePolicy)?;
    let remote: SocketAddr = SocketAddr::from((Ipv4Addr::LOCALHOST, 123));
    let mapped: SocketAddr = SocketAddr::from((Ipv4Addr::LOCALHOST.to_ipv6_mapped(), 123));
    let other_port: SocketAddr = SocketAddr::from((Ipv4Addr::LOCALHOST, 124));
    let tls: SocketAddr = SocketAddr::from((Ipv4Addr::LOCALHOST, 4460));
    let target: KePreviewTarget =
        KePreviewTarget::new(tls, "Time.Example").map_err(|_| Failure::Arguments)?;
    let addresses: SourceAddressSet =
        SourceAddressSet::new(&[remote]).map_err(Failure::SourcePolicy)?;
    let original: SourceConfigurationSnapshot = authority
        .install_ke_preview_configuration(&source, addresses, target)
        .map_err(Failure::SourcePolicy)?;
    let _other_binding: SourceBinding = other.bind(other_port).map_err(Failure::SourcePolicy)?;
    let config: Config = Config::from_managed_protected(
        authority
            .authorize_protected_ke(&original)
            .map_err(Failure::SourcePolicy)?,
    );
    with_config_owner(
        &config,
        Instant::now(),
        S2C,
        &[(0x11, 12)],
        |owner, control, negotiated| {
            assert_eq!(negotiated, remote);
            let completion: super::ProtectedCompletion<'_> =
                stage_exact_managed_reply(owner, control, remote, mapped)?;
            let charged: Status = owner.status();
            let receipt: Receipt = owner.last_receipt().ok_or(Failure::Control)?;
            let before: SourcePolicySnapshot = source
                .policy_snapshot(remote)
                .map_err(Failure::SourcePolicy)?;
            assert!(before.source().floor().is_some());
            assert_eq!(before.source().floor(), before.endpoint().floor());
            let selection: SourcePolicyTarget = match change {
                ManagedEpochTarget::Source => authority.source_target(&source),
                ManagedEpochTarget::MappedEndpoint => authority.endpoint_target(mapped),
                ManagedEpochTarget::OtherSource => authority.source_target(&other),
                ManagedEpochTarget::OtherPort => authority.endpoint_target(other_port),
            }
            .map_err(Failure::SourcePolicy)?;
            authority
                .reenable(&[selection])
                .map_err(Failure::SourcePolicy)?;
            let result: Result<super::CommittedProtected<'_>, Failure> = completion.commit(owner);
            let (available, control_after): (usize, Result<(), Failure>) = if invalidates {
                assert!(matches!(
                    result,
                    Err(Failure::SourcePolicy(SourcePolicyError::Stale))
                ));
                assert_eq!(owner.cookie_population()?, (0, 0));
                (0, Ok(()))
            } else {
                assert!(result?.into_summary().transmitted);
                assert_eq!(owner.cookie_population()?, (1, 12));
                (1, Err(Failure::Control))
            };
            assert_eq!(control.with_published(|_| Ok(())), control_after);
            assert_eq!(
                owner.last_receipt().map(Receipt::serial),
                Some(receipt.serial())
            );
            assert_eq!(
                owner.status(),
                Status {
                    available,
                    ..charged
                }
            );

            let after: SourcePolicySnapshot = source
                .policy_snapshot(remote)
                .map_err(Failure::SourcePolicy)?;
            assert!(!after.source().refused() && !after.endpoint().refused());
            assert_eq!(after.source().floor(), before.source().floor());
            assert_eq!(after.endpoint().floor(), before.endpoint().floor());
            assert_eq!(
                after.source().revision(),
                before.source().revision()
                    + u64::from(matches!(change, ManagedEpochTarget::Source))
            );
            assert_eq!(
                after.endpoint().revision(),
                before.endpoint().revision()
                    + u64::from(matches!(change, ManagedEpochTarget::MappedEndpoint))
            );
            Ok(())
        },
    )
}

/// Check two genuine backend identities sharing the same numeric addresses.
/// # Errors
/// Preserve actual owner, packet preparation, adapter and good-reply failure.
/// # Panics
/// Reject substitutions followed by receipt publication or refunded receive work.
fn check_managed_pending_receiving_backend() -> Result<(), Failure> {
    let interval: Duration = Duration::from_millis(4);
    let doubled: Duration = Duration::from_millis(8);
    let start: Instant = Instant::now()
        .checked_sub(Duration::from_millis(20))
        .ok_or(Failure::Deadline)?;
    let (config, _, _): (Config, crate::SourceHandle, SourcePolicyAuthority) =
        managed_attempt_config(start, interval, doubled)?;
    with_config_owner(
        &config,
        start,
        S2C,
        &[(0x11, 12)],
        |owner, control, remote| {
            let mut sender: Backend = backend(remote, Script::new(ScriptSend::Full, Vec::new()));
            let other: Backend = backend(remote, Script::new(ScriptSend::Full, Vec::new()));
            assert_eq!((sender.local, sender.remote), (other.local, other.remote));
            assert!(!Arc::ptr_eq(&sender.identity, &other.identity));
            let ticket: Ticket<'_> = owner.reserve(RequestInput::Observed)?;
            owner.prepare_observed(&ticket, entropy)?;
            sender.send_prepared(owner, ticket, control)?;
            let pending: Status = owner.status();
            assert_eq!(pending.phase, RequestPhase::Pending);

            let refused: Result<(Receipt, CookieReplacement, PendingSubmission), Failure> = owner
                .stage_observed_reply(
                    fixtures::HEALTHY,
                    remote,
                    &SoftwareClock::capture(),
                    &other.identity,
                );
            assert!(matches!(refused, Err(Failure::Ledger(LedgerError::Ticket))));
            assert_eq!(
                owner.status(),
                Status {
                    frames: pending.frames - 1,
                    bytes: pending.bytes - fixtures::HEALTHY.len(),
                    ..pending
                }
            );
            assert!(owner.last_receipt().is_none());
            assert_eq!(owner.cookie_population()?, (0, 0));

            let (receipt, replacement, submission): (
                Receipt,
                CookieReplacement,
                PendingSubmission,
            ) = owner.stage_observed_reply(
                fixtures::HEALTHY,
                remote,
                &SoftwareClock::capture(),
                &sender.identity,
            )?;
            assert!(Arc::ptr_eq(submission.identity(), &sender.identity));
            assert_eq!(replacement.population(), (1, 12));
            assert_eq!(
                owner.last_receipt().map(Receipt::serial),
                Some(receipt.serial())
            );
            assert_eq!(
                owner.status(),
                Status {
                    phase: RequestPhase::Idle,
                    frames: pending.frames - 2,
                    bytes: pending.bytes - 2 * fixtures::HEALTHY.len(),
                    ..pending
                }
            );
            assert_eq!(owner.cookie_population()?, (0, 0));
            Ok(())
        },
    )
}

/// Re-enable affects only the selected protected correlation record.
/// Reply staging consumes the submission from the concrete Pending owner.
/// # Errors
/// Preserve the first unexpected effect or healthy-neighbor failure.
#[test]
fn managed_protected_reply_distinguishes_wire_peer_and_policy_epochs() -> Result<(), Failure> {
    for (change, invalidates) in [
        (ManagedEpochTarget::OtherSource, false),
        (ManagedEpochTarget::OtherPort, false),
        (ManagedEpochTarget::Source, true),
        (ManagedEpochTarget::MappedEndpoint, true),
    ] {
        check_managed_protected_reply_epoch(change, invalidates)?;
    }
    check_managed_pending_receiving_backend()
}

/// A source-policy action or invocation Stop issued after the protected send.
#[derive(Clone, Copy)]
enum PendingReadEvent {
    /// Retain a server refusal at the exact protected endpoint.
    Deny,
    /// Increase the next-send floor at the exact protected endpoint.
    Rate,
    /// Retain a server refusal at a separate endpoint on the same IP.
    DenyOtherPort,
    /// Replace the installed source with equal values and a fresh revision.
    ReplaceConfiguration,
    /// Request Stop through the original invocation scope.
    Stop,
}

impl PendingReadEvent {
    /// Specify the independently expected retirement result for this event.
    const fn refusal(self) -> Option<Failure> {
        use crate::SourcePolicyError;

        match self {
            Self::Deny => Some(Failure::SourcePolicy(SourcePolicyError::Disabled)),
            Self::ReplaceConfiguration => Some(Failure::SourcePolicy(
                SourcePolicyError::ConfigurationChanged,
            )),
            Self::Stop => Some(Failure::Cancelled),
            Self::Rate | Self::DenyOtherPort => None,
        }
    }
}

/// Sample notifications from the parked operation independently of polling it.
#[derive(Default)]
struct PendingReadWake {
    /// Notifications delivered to the registered future since the first poll.
    notifications: AtomicUsize,
}

impl std::task::Wake for PendingReadWake {
    fn wake(self: Arc<Self>) {
        self.notifications.fetch_add(1, Ordering::Relaxed);
    }
}

/// Apply the external event only after the real driver parks at read readiness.
/// # Errors
/// Preserve a setup or event refusal separately from the observed driver result.
/// # Panics
/// Reject missing or unexpected notifications and early or absent readiness.
fn poll_pending_read_event<'control>(
    operation: impl Future<Output = Result<OperationCompletion<'control>, Failure>>,
    apply: impl FnOnce() -> Result<(), Failure>,
    release: &watch::Sender<bool>,
    event: PendingReadEvent,
) -> Result<Result<OperationCompletion<'control>, Failure>, Failure> {
    use std::task::{Context, Poll, Waker};

    let observed: Arc<PendingReadWake> = Arc::new(PendingReadWake::default());
    let waker: Waker = Waker::from(Arc::clone(&observed));
    let mut context: Context<'_> = Context::from_waker(&waker);
    let mut operation = std::pin::pin!(operation);
    assert!(operation.as_mut().poll(&mut context).is_pending());
    let before: usize = observed.notifications.load(Ordering::Relaxed);
    apply()?;
    assert_eq!(
        observed.notifications.load(Ordering::Relaxed) > before,
        !matches!(event, PendingReadEvent::DenyOtherPort),
        "only an event applicable to the request should wake the protected receive",
    );
    if event.refusal().is_none() {
        assert!(operation.as_mut().poll(&mut context).is_pending());
        assert!(!release.send_replace(true));
    }
    let polled: Poll<Result<OperationCompletion<'_>, Failure>> =
        operation.as_mut().poll(&mut context);
    assert!(
        polled.is_ready(),
        "the selected event did not resolve the pending receive",
    );
    match polled {
        Poll::Ready(result) => Ok(result),
        Poll::Pending => Err(Failure::Control),
    }
}

/// Verify the actual policy transition and its exact protected-endpoint scope.
/// # Panics
/// Reject missing protected send debit, cross-source effects or a lost floor update.
fn assert_pending_read_policy(
    event: PendingReadEvent,
    submitted: &crate::SourcePolicySnapshot,
    updated: &crate::SourcePolicySnapshot,
) {
    assert!(
        submitted.source().floor().is_some(),
        "the request must have sent",
    );
    assert_eq!(updated.source().floor(), submitted.source().floor());
    assert!(!updated.source().refused());
    assert_eq!(
        updated.endpoint().refused(),
        matches!(event, PendingReadEvent::Deny),
    );
    if matches!(event, PendingReadEvent::Rate) {
        assert_eq!(updated.endpoint().interval(), Duration::from_secs(1));
        assert!(updated.endpoint().floor() > submitted.endpoint().floor());
    } else {
        assert_eq!(updated.endpoint().floor(), submitted.endpoint().floor());
    }
}

/// Deliver a real owner event independently of protected packet readiness.
/// # Errors
/// Preserve original Stop, revision replacement or alias admission failure.
/// # Panics
/// Fail if the first Stop did not latch.
fn apply_pending_read_event(
    event: PendingReadEvent,
    authority: &mut SourcePolicyAuthority,
    config: &Config,
    control: &Control,
    alias: &crate::source_policy::SourceBinding,
    alias_reply: &mut crate::source_policy::FixtureRequest,
) -> Result<(), Failure> {
    match event {
        PendingReadEvent::ReplaceConfiguration => {
            replace_policy_wait_configuration(authority, config)
        }
        PendingReadEvent::Stop => {
            let stop: StopHandle = StopHandle {
                scope: Arc::clone(&control.stop_scope),
            };
            assert_eq!(stop.request_stop()?, StopDisposition::Latched);
            Ok(())
        }
        PendingReadEvent::Deny | PendingReadEvent::DenyOtherPort | PendingReadEvent::Rate => {
            let code: [u8; 4] = if matches!(event, PendingReadEvent::Rate) {
                *b"RATE"
            } else {
                *b"DENY"
            };
            crate::source_policy::fixture_control(
                alias,
                alias_reply,
                u32::from_be_bytes(code),
                Instant::now(),
            )
            .map_err(Failure::SourcePolicy)
        }
    }
}

/// Apply each event while the actual driver is parked on read readiness.
/// # Errors
/// Preserve issuer, fixture control, run or observation refusal.
/// # Panics
/// Reject an early send or reply, charge reversal, or misapplied policy.
fn check_managed_protected_pending_read_event(event: PendingReadEvent) -> Result<(), Failure> {
    use crate::source_policy::{FixtureRequest, SourceBinding, fixture_send};
    use crate::{SourceHandle, SourcePolicySnapshot};

    let initial: Duration = Duration::from_millis(250);
    let earlier: Instant = Instant::now()
        .checked_sub(Duration::from_secs(2))
        .ok_or(Failure::Deadline)?;
    let (config, source, mut authority): (Config, SourceHandle, SourcePolicyAuthority) =
        managed_attempt_config(earlier, initial, Duration::from_secs(2))?;
    let alias: SourceHandle = authority.source(1).map_err(Failure::SourcePolicy)?;
    with_config_owner(
        &config,
        earlier,
        S2C,
        &[(0x11, 12)],
        |owner, control, remote| {
            let alias_peer: SocketAddr = if matches!(event, PendingReadEvent::DenyOtherPort) {
                SocketAddr::new(remote.ip(), 124)
            } else {
                remote
            };
            let binding: SourceBinding = alias.bind(alias_peer).map_err(Failure::SourcePolicy)?;
            let prior_send: Instant = earlier
                .checked_add(Duration::from_millis(500))
                .ok_or(Failure::Deadline)?;
            let mut alias_reply: FixtureRequest =
                fixture_send(&binding, &Arc::new(()), prior_send).map_err(Failure::SourcePolicy)?;
            let (release, pending_read): (watch::Sender<bool>, watch::Receiver<bool>) =
                watch::channel(false);
            let receive: bool = event.refusal().is_none();
            let reads: Vec<ScriptRead> = if receive {
                vec![ScriptRead::Packet(fixtures::HEALTHY, remote)]
            } else {
                Vec::new()
            };
            let mut selected: Script = Script::new(ScriptSend::Full, reads);
            selected.read_release = Some(pending_read);
            let mut backend: Backend = backend(remote, selected);
            let expiry: Instant = control.expiry;
            let result: Result<OperationCompletion<'_>, Failure> = runtime()?.block_on(async {
                poll_pending_read_event(
                    operation(owner, control, entropy, &mut backend),
                    || {
                        let submitted: SourcePolicySnapshot = source
                            .policy_snapshot(remote)
                            .map_err(Failure::SourcePolicy)?;
                        apply_pending_read_event(
                            event,
                            &mut authority,
                            &config,
                            control,
                            &binding,
                            &mut alias_reply,
                        )?;
                        let updated: SourcePolicySnapshot = source
                            .policy_snapshot(remote)
                            .map_err(Failure::SourcePolicy)?;
                        assert_pending_read_policy(event, &submitted, &updated);
                        Ok(())
                    },
                    &release,
                    event,
                )
            })?;
            assert_eq!(control.expiry, expiry);
            assert!(Instant::now() < expiry);
            let adapter: &Script = script(&backend)?;
            assert_eq!(
                (adapter.send_calls, adapter.receive_calls),
                (1, usize::from(receive)),
            );
            let status: Status = owner.status();
            assert_eq!((status.attempts, status.used, status.uids), (31, 1, 1));
            assert_eq!(status.frames, 32 - usize::from(receive));
            assert_eq!(
                status.bytes,
                131_072 - usize::from(receive) * fixtures::HEALTHY.len(),
            );
            assert_eq!(owner.cookie_population()?, (0, 0));
            if let Some(expected) = event.refusal() {
                assert_eq!(result.err(), Some(expected));
                assert!(owner.last_receipt().is_none());
            } else {
                let completion: OperationCompletion<'_> = result?;
                let header: ntp_wire::RawHeader = completion.history.receipt.fields().header();
                assert_eq!(header.origin_timestamp().bits(), WIRE);
                assert_eq!(owner.last_receipt().map(Receipt::serial), Some(1));
                assert_eq!(
                    (completion.history.cookies, completion.history.cookie_bytes),
                    (1, 12),
                );
            }
            Ok(())
        },
    )
}

/// The exact endpoint's refusal wakes the receiver without a datagram or timeout.
/// # Errors
/// Preserve fixture and driver refusal.
#[test]
fn managed_protected_pending_endpoint_deny_wakes_silent_receive() -> Result<(), Failure> {
    check_managed_protected_pending_read_event(PendingReadEvent::Deny)
}

/// A RATE-only update preserves the actual request and its separately ready reply.
/// # Errors
/// Preserve fixture and driver refusal.
#[test]
fn managed_protected_pending_rate_preserves_readiness_and_reply() -> Result<(), Failure> {
    check_managed_protected_pending_read_event(PendingReadEvent::Rate)
}

/// A DENY received at another port does not invalidate the protected exchange.
/// # Errors
/// Preserve fixture and driver refusal.
#[test]
fn managed_protected_pending_ignores_deny_for_another_port() -> Result<(), Failure> {
    check_managed_protected_pending_read_event(PendingReadEvent::DenyOtherPort)
}

/// Stop and equal-value source replacement retire a quiet pending receive.
/// # Errors
/// Preserve fixture and driver refusal.
#[test]
fn managed_protected_pending_stop_and_revision_replacement_wake_receive() -> Result<(), Failure> {
    check_managed_protected_pending_read_event(PendingReadEvent::Stop)?;
    check_managed_protected_pending_read_event(PendingReadEvent::ReplaceConfiguration)
}

/// Park after one managed send and use the driver's own expiry timer.
/// # Errors
/// Preserve fixture, policy, runtime or unexpected protected receive refusal.
/// # Panics
/// Fail if the read did not park, expiry missed a latched Stop, or work was refunded.
#[test]
fn managed_protected_pending_expiry_observes_already_latched_stop() -> Result<(), Failure> {
    use std::task::Poll;

    let cases: [(bool, Option<Failure>); 3] = [
        (true, Some(Failure::Cancelled)),
        (false, Some(Failure::Deadline)),
        (false, None),
    ];
    for (latch_stop, expected) in cases {
        let earlier: Instant = Instant::now()
            .checked_sub(Duration::from_secs(2))
            .ok_or(Failure::Deadline)?;
        let (config, source, _authority): (Config, crate::SourceHandle, SourcePolicyAuthority) =
            managed_attempt_config(earlier, Duration::from_millis(250), Duration::from_secs(2))?;
        let driver: tokio::runtime::Runtime = runtime()?;
        let start: Instant = Instant::now();
        let expiry: Instant = start
            .checked_add(Duration::from_millis(250))
            .ok_or(Failure::Deadline)?;
        let control: Control = Control::clamped(start, expiry)?;
        with_config_control(
            &config,
            &control,
            S2C,
            &[(0x11, 12)],
            |owner, control, remote| {
                let (release, pending_read): (watch::Sender<bool>, watch::Receiver<bool>) =
                    watch::channel(false);
                let mut selected: Script = Script::new(
                    ScriptSend::Full,
                    vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
                );
                selected.read_release = Some(pending_read);
                let mut backend: Backend = backend(remote, selected);
                let result: Result<OperationCompletion<'_>, Failure> = driver.block_on(async {
                    let pending = operation(owner, control, entropy, &mut backend);
                    tokio::pin!(pending);
                    std::future::poll_fn(|context| {
                        assert!(pending.as_mut().poll(context).is_pending());
                        Poll::Ready(())
                    })
                    .await;
                    let submitted: crate::SourcePolicySnapshot = source
                        .policy_snapshot(remote)
                        .map_err(Failure::SourcePolicy)?;
                    assert!(
                        submitted.source().floor().is_some(),
                        "the request must have sent"
                    );
                    assert!(
                        Instant::now() < expiry,
                        "the operation must park before expiry"
                    );
                    if latch_stop {
                        latch_wait_stop_without_wake(control)?;
                    }
                    if expected.is_none() {
                        assert!(!release.send_replace(true));
                    }
                    Ok::<_, Failure>(pending.await)
                })?;
                assert_eq!(control.expiry, expiry);
                let observed: &Script = script(&backend)?;
                let receive: usize = usize::from(expected.is_none());
                assert_eq!((observed.send_calls, observed.receive_calls), (1, receive));
                let status: Status = owner.status();
                assert_eq!((status.attempts, status.used, status.uids), (31, 1, 1));
                assert_eq!(status.frames, 32 - receive);
                assert_eq!(status.bytes, 131_072 - receive * fixtures::HEALTHY.len());
                assert_eq!(owner.cookie_population()?, (0, 0));
                if let Some(failure) = expected {
                    assert!(Instant::now() >= expiry);
                    assert_eq!(
                        result.err(),
                        Some(failure),
                        "expiry must recheck the original Stop"
                    );
                    assert!(owner.last_receipt().is_none());
                } else {
                    let completed: OperationCompletion<'_> = result?;
                    assert_eq!(completed.history.receipt.serial(), 1);
                }
                Ok(())
            },
        )?;
    }
    Ok(())
}

/// Retire the first genuine managed operation at its checked alias floor.
/// # Errors
/// Preserve the original driver, send, timing or authentication failure.
fn first_managed_exchange<'a>(
    association: &mut AssociationOwner<'_>,
    control: &'a Control,
    spurious_reads: usize,
) -> Result<super::ProtectedCompletion<'a>, Failure> {
    let due: Instant = control
        .origin
        .checked_add(Duration::from_millis(8))
        .ok_or(Failure::Deadline)?;
    exchange_with(association, control, entropy, |remote: SocketAddr| {
        let mut reads: Vec<ScriptRead> = (0..spurious_reads)
            .map(|_| ScriptRead::WouldBlock)
            .collect();
        reads.push(ScriptRead::Packet(fixtures::HEALTHY, remote));
        let mut script: Script = Script::new(ScriptSend::Full, reads);
        script.send_before = Some(SoftwareClock {
            mono: due,
            mono_after: due,
            utc: UNIX_EPOCH,
        });
        Ok(backend(remote, script))
    })
}

/// Exercise the real issuer, original Control and independently fixed response keys.
/// # Errors
/// Preserve expiry arithmetic, checked issuer or actual consumer refusal.
fn with_managed_successor(
    exercise: impl FnOnce(
        &mut AssociationOwner<'_>,
        &Control,
        SocketAddr,
        &mut SourcePolicyAuthority,
        &crate::SourceHandle,
    ) -> Result<(), Failure>,
) -> Result<(), Failure> {
    let start: Instant = Instant::now()
        .checked_sub(Duration::from_millis(20))
        .ok_or(Failure::Deadline)?;
    let (config, source, mut authority): (Config, crate::SourceHandle, SourcePolicyAuthority) =
        managed_attempt_config(start, Duration::from_millis(4), Duration::from_millis(8))?;
    with_config_owner(
        &config,
        start,
        S2C,
        &[(0x11, 12)],
        |owner, control, remote| exercise(owner, control, remote, &mut authority, &source),
    )
}

/// Challenge continuation without a viable producer or backend entry.
/// # Panics
/// Fail if the scope is admitted, visits a fixture callback or changes ledger work.
fn assert_successor_refused(
    owner: &mut AssociationOwner<'_>,
    control: &Control,
    expected: Failure,
) {
    let before: Status = owner.status();
    let receipt: Option<Receipt> = owner.last_receipt();
    let called: std::cell::Cell<bool> = std::cell::Cell::new(false);
    let result: Result<super::ProtectedCompletion<'_>, Failure> = exchange_successor_with(
        owner,
        control,
        |_| {
            called.set(true);
            Err(Failure::Entropy)
        },
        |_| {
            called.set(true);
            Err(Failure::Endpoint)
        },
    );
    assert_eq!(result.err(), Some(expected));
    assert!(!called.get());
    assert_eq!(owner.status(), before);
    assert!(owner.last_receipt() == receipt);
}

/// Supply the independently sealed successor's wire/UID/nonce inputs once.
/// # Errors
/// The protected private runner requires a fallible entropy callback.
#[expect(
    clippy::unnecessary_wraps,
    reason = "This fixed fixture supplies the legal half of the real fallible entropy boundary."
)]
fn successor_entropy(output: &mut [u8; 56]) -> Result<(), Failure> {
    output[..8].copy_from_slice(&0x8899_aabb_ccdd_eeff_u64.to_be_bytes());
    output[8..40].copy_from_slice(&[
        0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e,
        0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d,
        0x3e, 0x3f,
    ]);
    output[40..].fill(0x99);
    Ok(())
}

/// Submit the chosen scripted control after the actual original send floor.
/// The common driver owns eligibility, original Stop and expiry.
/// # Errors
/// Preserve floor, original time, actual adapter and mixed receive refusals.
pub(in crate::nts_ke_experiment) fn scripted_managed_successor<'control>(
    association: &mut AssociationOwner<'_>,
    control: &'control Control,
    kind: ProtectedControlKind,
) -> Result<RetiredProtectedResponse<'control>, Failure> {
    exchange_successor_mixed(
        association,
        control,
        successor_entropy,
        |peer: SocketAddr| {
            let reads: Vec<ScriptRead> = match kind {
                ProtectedControlKind::Rate => vec![
                    ScriptRead::Packet(&RATE_BAD_TAG, peer),
                    ScriptRead::Packet(&PROTECTED_RATE, peer),
                ],
                ProtectedControlKind::Deny => vec![ScriptRead::Packet(&PROTECTED_DENY, peer)],
                ProtectedControlKind::Restrict => {
                    vec![ScriptRead::Packet(&PROTECTED_RESTRICTION, peer)]
                }
            };
            Script::new(ScriptSend::Full, reads)
        },
    )
}

/// Drive the selected independently sealed restriction against the first request.
/// # Errors
/// Preserve the actual bounded driver, packet and retirement refusal.
#[cfg(any(windows, target_os = "linux"))]
pub(in crate::nts_ke_experiment) fn scripted_managed_initial_control<'control>(
    association: &mut AssociationOwner<'_>,
    control: &'control Control,
    kind: ProtectedControlKind,
) -> Result<RetiredProtectedResponse<'control>, Failure> {
    super::reached::exchange_initial_with(
        association,
        control,
        successor_entropy,
        |peer: SocketAddr| {
            let packet: &'static [u8; 124] = match kind {
                ProtectedControlKind::Rate => &PROTECTED_RATE,
                ProtectedControlKind::Deny => &PROTECTED_DENY,
                ProtectedControlKind::Restrict => &PROTECTED_RESTRICTION,
            };
            let reads: Vec<ScriptRead> = vec![
                ScriptRead::Packet(&CORRELATED_NTSN, peer),
                ScriptRead::Packet(&CORRELATED_DENY, peer),
                ScriptRead::Packet(&RATE_BAD_TAG, peer),
                ScriptRead::Packet(packet, peer),
                ScriptRead::Failed(io::ErrorKind::UnexpectedEof),
            ];
            Ok(backend(peer, Script::new(ScriptSend::Full, reads)))
        },
    )
}

/// Run the bounded first leg through the actual socket and retirement.
/// # Errors
/// Preserve the original admission, socket, driver and receiving refusal.
#[cfg(any(windows, target_os = "linux"))]
pub(in crate::nts_ke_experiment) fn loopback_managed_initial<'control>(
    association: &mut AssociationOwner<'_>,
    control: &'control Control,
) -> Result<RetiredProtectedResponse<'control>, Failure> {
    super::reached::exchange_initial_with(association, control, entropy, Backend::bind)
}

/// Run the first ordinary leg through the actual socket and retirement.
/// # Errors
/// Preserve the original admission, socket, driver and receiving refusal.
#[cfg(any(windows, target_os = "linux"))]
pub(in crate::nts_ke_experiment) fn loopback_managed_first<'control>(
    association: &mut AssociationOwner<'_>,
    control: &'control Control,
) -> Result<super::ProtectedCompletion<'control>, Failure> {
    exchange_with(association, control, entropy, Backend::bind)
}

/// Run the admitted second leg through the exact same socket path.
/// # Errors
/// Preserve the continuation, adapter, budget and charged receive refusals.
#[cfg(any(windows, target_os = "linux"))]
pub(in crate::nts_ke_experiment) fn loopback_managed_successor<'control>(
    association: &mut AssociationOwner<'_>,
    control: &'control Control,
) -> Result<RetiredProtectedResponse<'control>, Failure> {
    super::reached::exchange_successor_mixed_with(
        association,
        control,
        successor_entropy,
        Backend::bind,
    )
}

/// Keep the actual socket adapter and original NAK observation in the joined control.
/// # Errors
/// Preserve the charged receiver or original adapter refusal.
#[cfg(any(windows, target_os = "linux"))]
pub(in crate::nts_ke_experiment) fn observe_loopback_managed_successor<'control>(
    association: &mut AssociationOwner<'_>,
    control: &'control Control,
) -> Result<super::ObservedRetiredSuccessor<'control>, super::SuccessorFailure> {
    exchange_successor_observing(association, control, successor_entropy, Backend::bind)
}

/// Inspect the two authenticated responses from the fixed native fixture peer.
/// Both datagrams use the actual adapter, original association and carried work.
/// # Errors
/// Preserve missing first receipt, continuation, socket and receive refusals.
/// # Panics
/// Fail on a wrong packet disposition, lost custody or refunded packet work.
#[cfg(windows)]
pub(in crate::nts_ke_experiment) fn loopback_native_managed_successor<'control>(
    association: &mut AssociationOwner<'_>,
    control: &'control Control,
    fill: impl FnOnce(&mut [u8; 56]) -> Result<(), Failure>,
) -> Result<RetiredProtectedResponse<'control>, Failure> {
    let first: Receipt = association.last_receipt().ok_or(Failure::Control)?;
    assert_eq!((first.serial(), first.cookie_counts()), (1, (1, 1)));
    assert_eq!(association.cookie_population()?, (1, 12));
    assert_eq!(
        association.status(),
        Status {
            phase: RequestPhase::Idle,
            attempts: 31,
            frames: 31,
            bytes: 130_932, // 131,072 original, less one 140-byte reply.
            available: 1,
            used: 1,
            uids: 1,
        }
    );

    let retired: RetiredProtectedResponse<'control> =
        super::reached::exchange_successor_mixed_with(association, control, fill, Backend::bind)?;
    assert!(matches!(&retired, RetiredProtectedResponse::Control(_)));
    assert_eq!(
        association.status(),
        Status {
            phase: RequestPhase::Pending,
            attempts: 30,
            frames: 30,
            bytes: 130_808, // 124-byte control also charged to the original budget.
            available: 0,
            used: 2,
            uids: 2,
        }
    );
    assert!(association.last_receipt() == Some(first));
    assert_eq!(association.cookie_population()?, (0, 0));
    Ok(retired)
}

/// Read-only observations owned by the peer at the boundaries of one exchange.
#[cfg(any(windows, target_os = "linux"))]
#[derive(Clone, Copy)]
pub(in crate::nts_ke_experiment) struct LoopbackPeerLeg<T> {
    /// Peer observation immediately after the successful request receive.
    pub(in crate::nts_ke_experiment) received: T,
    /// Peer observation immediately before the selected response send.
    pub(in crate::nts_ke_experiment) sending: T,
}

/// Receive and check an actual protected client packet against fixed wire facts.
/// The peer owns its configured read timeout and accepts only its loopback client.
/// # Errors
/// Preserve the actual receive or observation error, or refuse a mismatched fixed request.
/// # Panics
/// Fail if an actual request is incomplete, plain or has the wrong correlation.
#[cfg(any(windows, target_os = "linux"))]
fn loopback_request<T>(
    peer: &std::net::UdpSocket,
    wire: u64,
    uid: &[u8],
    cookie: &[u8; 12],
    placeholder_count: usize,
    observe: impl FnOnce() -> Result<T, Failure>,
) -> Result<(SocketAddr, T), Failure> {
    let mut packet: [u8; 253] = [0; 253];
    let (length, client): (usize, SocketAddr) = peer
        .recv_from(&mut packet)
        .map_err(|error: io::Error| Failure::Datagram(DatagramError::Receive(error.kind())))?;
    let received: T = observe()?;
    assert!(client.ip().is_loopback());
    assert_ne!(client.port(), 0);
    let authentication_offset: usize = 100 + 16 * placeholder_count;
    assert_eq!(length, authentication_offset + 40);
    assert_eq!(packet.first(), Some(&0x23));
    assert_eq!(packet.get(40..48), Some(wire.to_be_bytes().as_slice()));
    assert_eq!(packet.get(48..52), Some([1, 4, 0, 36].as_slice()));
    assert_eq!(packet.get(52..84), Some(uid));
    assert_eq!(packet.get(84..88), Some([2, 4, 0, 16].as_slice()));
    assert_eq!(packet.get(88..100), Some(cookie.as_slice()));
    let placeholders: &[u8] = packet
        .get(100..authentication_offset)
        .ok_or(Failure::Control)?;
    let placeholder: [u8; 16] = [3, 4, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let (fields, remainder): (&[[u8; 16]], &[u8]) = placeholders.as_chunks();
    assert_eq!(remainder, []);
    assert!(fields.iter().all(|field: &[u8; 16]| *field == placeholder));
    assert_eq!(
        packet.get(authentication_offset..authentication_offset + 4),
        Some([4, 4, 0, 40].as_slice())
    );
    Ok((client, received))
}

/// Send an independently sealed public vector on the admitted peer socket.
/// # Errors
/// Preserve the actual observation or send error.
/// # Panics
/// Fail if an actual send completes only part of the fixed vector.
#[cfg(any(windows, target_os = "linux"))]
fn send_loopback_reply<T>(
    peer: &std::net::UdpSocket,
    client: SocketAddr,
    response: &[u8],
    observe: impl FnOnce() -> Result<T, Failure>,
) -> Result<T, Failure> {
    let sending: T = observe()?;
    let sent: usize = peer
        .send_to(response, client)
        .map_err(|error: io::Error| Failure::Ledger(LedgerError::Send(error.kind())))?;
    assert_eq!(sent, response.len());
    Ok(sending)
}

/// Run the independent peer for exactly two requests under finite socket timeouts.
/// Return the peer's observations for the two selected authenticated replies.
/// # Errors
/// Preserve either leg's peer I/O or observation failure.
#[cfg(any(windows, target_os = "linux"))]
pub(in crate::nts_ke_experiment) fn serve_managed_loopback<T>(
    peer: &std::net::UdpSocket,
    first_reply: &'static [u8],
    ordinary_successor: bool,
    sends_nak: bool,
    mut observe: impl FnMut() -> Result<T, Failure>,
) -> Result<[LoopbackPeerLeg<T>; 2], Failure> {
    let (first, first_received): (SocketAddr, T) =
        loopback_request(peer, WIRE, &C2S, &[0x11; 12], 7, &mut observe)?;
    let first_sending: T = send_loopback_reply(peer, first, first_reply, &mut observe)?;
    let second_uid: &[u8] = PROTECTED_DENY.get(52..84).ok_or(Failure::Control)?;
    let replacement: [u8; 12] = [
        0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab,
    ];
    let second_placeholders: usize = if ordinary_successor { 6 } else { 7 };
    let (second, second_received): (SocketAddr, T) = loopback_request(
        peer,
        0x8899_aabb_ccdd_eeff,
        second_uid,
        &replacement,
        second_placeholders,
        &mut observe,
    )?;
    if sends_nak {
        send_loopback_reply(peer, second, &CORRELATED_NTSN, || Ok(()))?;
    }
    let terminal: &[u8] = if ordinary_successor {
        fixtures::LOOPBACK_SUCCESSOR_QUALITY_REPLY
    } else {
        &PROTECTED_DENY
    };
    let second_sending: T = send_loopback_reply(peer, second, terminal, observe)?;
    Ok([
        LoopbackPeerLeg {
            received: first_received,
            sending: first_sending,
        },
        LoopbackPeerLeg {
            received: second_received,
            sending: second_sending,
        },
    ])
}

/// Compare the retained first operation with its observations before commit.
/// # Panics
/// Fail if the receipt, socket, captures, commit interval or issued revision changes.
fn assert_first_commit(
    committed: &super::CommittedFirstProtected<'_>,
    receipt: Receipt,
    backend: &Arc<()>,
    captures: &[(Instant, Instant, SystemTime); 3],
    original_configuration: &SourceConfigurationSnapshot,
    commit_interval: &std::ops::RangeInclusive<Instant>,
) {
    let (history, identity, committed_at, configuration): (
        &super::reached::OperationHistory<'_>,
        &Arc<()>,
        Instant,
        Option<&SourceConfigurationSnapshot>,
    ) = committed.observation();
    assert!(history.receipt == receipt);
    assert!(Arc::ptr_eq(identity, backend));
    assert_eq!(
        &[history.send_before, history.send_after, history.read_return]
            .map(|clock: SoftwareClock| (clock.monotonic(), clock.monotonic_after(), clock.utc())),
        captures
    );
    assert!(commit_interval.contains(&committed_at));
    assert!(
        configuration.is_some_and(|actual: &SourceConfigurationSnapshot| {
            actual.same_configuration(original_configuration)
        })
    );
}

/// Two separately retired requests use exactly one worker-local association allowance.
/// # Errors
/// Preserve actual issuer, both exchange paths or the terminal commit refusal.
/// # Panics
/// Fail if staging creates authority, either budget resets or a third operation enters.
#[test]
fn managed_retired_first_request_allows_one_same_association_successor() -> Result<(), Failure> {
    for terminal in [true, false] {
        with_managed_successor(|owner, control, _remote, _authority, source| {
            assert_successor_refused(owner, control, Failure::Control);
            let expiry: Instant = control.expiry;
            // A normal first leg spends three units. Each WouldBlock costs its
            // read and a second readiness wait, leaving exactly three units.
            let first: super::ProtectedCompletion<'_> = first_managed_exchange(owner, control, 29)?;
            assert_eq!(first.remaining_attempts(), 3);
            assert_eq!(first.history().receipt.serial(), 1);
            assert!(tokio::runtime::Handle::try_current().is_err());
            assert_successor_refused(owner, control, Failure::Control);
            assert_eq!(owner.cookie_population()?, (0, 0));
            let first_backend: Arc<()> = Arc::clone(first.identity());
            let first_status: Status = owner.status();
            let first_receipt: Receipt = first.history().receipt;
            let original_captures: [(Instant, Instant, SystemTime); 3] = [
                first.history().send_before,
                first.history().send_after,
                first.history().read_return,
            ]
            .map(|clock: SoftwareClock| (clock.monotonic(), clock.monotonic_after(), clock.utc()));
            let original_configuration: SourceConfigurationSnapshot = source
                .configuration_snapshot()
                .map_err(Failure::SourcePolicy)?
                .ok_or(Failure::Control)?;
            let before_commit: Instant = Instant::now();
            let committed: super::CommittedFirstProtected<'_> =
                first.commit_first_for_successor(owner)?;
            drop(control.published_guard()?);
            assert_eq!(owner.cookie_population()?, (1, 12));
            assert_eq!(
                owner.status(),
                Status {
                    available: 1,
                    ..first_status
                }
            );
            assert_eq!(control.expiry, expiry);
            let reset: Result<super::ProtectedCompletion<'_>, Failure> = exchange_with(
                owner,
                control,
                |_| Err(Failure::Entropy),
                |_| Err(Failure::Endpoint),
            );
            assert_eq!(reset.err(), Some(Failure::Control));
            let second: super::ProtectedCompletion<'_> = exchange_successor_with(
                owner,
                control,
                successor_entropy,
                |remote: SocketAddr| {
                    Ok(backend(
                        remote,
                        Script::new(
                            ScriptSend::Full,
                            vec![ScriptRead::Packet(fixtures::SUCCESSOR_OLD_S2C, remote)],
                        ),
                    ))
                },
            )?;
            assert_eq!(second.remaining_attempts(), 0);
            assert_eq!(second.history().receipt.serial(), 2);
            assert!(!Arc::ptr_eq(second.identity(), &first_backend));
            assert_first_commit(
                &committed,
                first_receipt,
                &first_backend,
                &original_captures,
                &original_configuration,
                &(before_commit..=second.history().send_before.monotonic()),
            );
            assert_successor_refused(owner, control, Failure::Ledger(LedgerError::Ticket));
            assert_eq!(
                owner.status(),
                Status {
                    phase: RequestPhase::Idle,
                    attempts: 30,
                    frames: 30,
                    bytes: 130_792,
                    available: 0,
                    used: 2,
                    uids: 2,
                }
            );
            if terminal {
                assert!(second.commit(owner)?.into_summary().transmitted);
                assert_eq!(control.cancel()?, CancelOutcome::AlreadyCommitted);
            } else {
                let before: Status = owner.status();
                assert_eq!(second.commit_for_successor(owner), Err(Failure::Control));
                assert_eq!(owner.status(), before);
                assert_eq!(owner.cookie_population()?, (0, 0));
                assert_successor_refused(owner, control, Failure::Ledger(LedgerError::Ticket));
            }
            assert_eq!(control.expiry, expiry);
            Ok(())
        })?;
    }
    Ok(())
}

/// A returned reservation spends its ordinal without establishing authentication.
/// # Errors
/// Preserve actual reservation, retirement and healthy terminal commit failures.
/// # Panics
/// Fail if ordinal two can install a first successor or its refusal changes custody.
#[test]
fn managed_retired_ordinal_two_cannot_open_first_successor() -> Result<(), Failure> {
    for terminal in [false, true] {
        with_managed_successor(|owner, control, _remote, _authority, _source| {
            let expiry: Instant = control.expiry;
            let ticket: Ticket<'_> = owner.reserve(RequestInput::Observed)?;
            owner.return_unused(ticket)?;
            assert_eq!(
                owner.status(),
                Status {
                    phase: RequestPhase::Idle,
                    attempts: 31,
                    frames: 32,
                    bytes: 131_072,
                    available: 1,
                    used: 0,
                    uids: 0,
                }
            );
            assert!(owner.last_receipt().is_none());
            assert_eq!(owner.cookie_population()?, (1, 12));

            let completion: super::ProtectedCompletion<'_> =
                first_managed_exchange(owner, control, 0)?;
            let receipt: Receipt = completion.history().receipt;
            let retired: Status = owner.status();
            assert_eq!(receipt.serial(), 2);
            assert_eq!(completion.remaining_attempts(), 61);
            assert!(tokio::runtime::Handle::try_current().is_err());
            assert_eq!(
                retired,
                Status {
                    phase: RequestPhase::Idle,
                    attempts: 30,
                    frames: 31,
                    bytes: 131_072 - fixtures::HEALTHY.len(),
                    available: 0,
                    used: 1,
                    uids: 1,
                }
            );
            assert_eq!(owner.cookie_population()?, (0, 0));
            assert_successor_refused(owner, control, Failure::Control);

            if terminal {
                assert!(completion.commit(owner)?.into_summary().transmitted);
                assert_eq!(
                    owner.status(),
                    Status {
                        available: 1,
                        ..retired
                    }
                );
                assert_eq!(owner.cookie_population()?, (1, 12));
                assert_eq!(control.cancel()?, CancelOutcome::AlreadyCommitted);
            } else {
                assert_eq!(
                    completion.commit_for_successor(owner),
                    Err(Failure::Control)
                );
                assert_eq!(owner.status(), retired);
                assert_eq!(owner.cookie_population()?, (0, 0));
                assert_successor_refused(owner, control, Failure::Control);
                assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
            }
            assert!(owner.last_receipt() == Some(receipt));
            assert_eq!(control.expiry, expiry);
            Ok(())
        })?;
    }
    Ok(())
}

/// A component action changes current authority independently of the witness.
#[derive(Clone, Copy)]
enum AuthenticationInvalidation {
    /// The actual invocation stop scope closes both requests.
    Stop,
    /// Administrative re-enable advances the original source epoch.
    SourceEpoch,
    /// The same negotiated address set now belongs to a different definition.
    Configuration,
}

/// Apply a real Stop, admin epoch or configuration change.
/// # Errors
/// Preserve genuine owner and authorised configuration failures.
/// # Panics
/// Fails if the fixture Stop request is not latched.
fn invalidate_authentication(
    change: AuthenticationInvalidation,
    control: &Control,
    remote: SocketAddr,
    authority: &mut SourcePolicyAuthority,
    source: &crate::SourceHandle,
) -> Result<Failure, Failure> {
    use crate::{SourceAddressSet, SourcePolicyError, SourcePolicyTarget};

    match change {
        AuthenticationInvalidation::Stop => {
            let stop: StopHandle = StopHandle {
                scope: Arc::clone(&control.stop_scope),
            };
            assert_eq!(stop.request_stop()?, StopDisposition::Latched);
            Ok(Failure::Cancelled)
        }
        AuthenticationInvalidation::SourceEpoch => {
            let target: SourcePolicyTarget = authority
                .source_target(source)
                .map_err(Failure::SourcePolicy)?;
            authority
                .reenable(&[target])
                .map_err(Failure::SourcePolicy)?;
            Ok(Failure::SourcePolicy(SourcePolicyError::Stale))
        }
        AuthenticationInvalidation::Configuration => {
            let original: SourceConfigurationSnapshot = source
                .configuration_snapshot()
                .map_err(Failure::SourcePolicy)?
                .ok_or(Failure::Control)?;
            let addresses: SourceAddressSet =
                SourceAddressSet::new(&[remote]).map_err(Failure::SourcePolicy)?;
            let tls: SocketAddr = "127.0.0.1:4460".parse().map_err(|_| Failure::Endpoint)?;
            let target: KePreviewTarget =
                KePreviewTarget::new(tls, "Time.Example").map_err(|_| Failure::Arguments)?;
            let replacement: SourceConfigurationSnapshot = authority
                .replace_ke_preview_configuration(&original, addresses, target)
                .map_err(Failure::SourcePolicy)?;
            assert!(!original.same_configuration(&replacement));
            Ok(Failure::SourcePolicy(
                SourcePolicyError::ConfigurationChanged,
            ))
        }
    }
}

/// Staged history and effective invalidation grant no continuation.
/// # Errors
/// Preserve real issuer, reply, component or policy operation failure.
/// # Panics
/// Fail if invalidation reaches a backend or stopped work installs a cookie.
#[test]
fn managed_retired_authentication_refuses_uncommitted_and_invalidated_history()
-> Result<(), Failure> {
    with_managed_successor(|owner, control, _remote, _authority, _source| {
        let first: super::ProtectedCompletion<'_> = first_managed_exchange(owner, control, 0)?;
        assert!(owner.last_receipt().is_some());
        drop(first);
        assert_successor_refused(owner, control, Failure::Control);
        assert_eq!(owner.cookie_population()?, (0, 0));
        Ok(())
    })?;
    with_managed_successor(|owner, control, _remote, _authority, _source| {
        first_managed_exchange(owner, control, 0)?.commit_for_successor(owner)?;
        with_managed_successor(
            |_foreign_owner, foreign_control, _remote, _authority, _source| {
                assert_successor_refused(owner, foreign_control, Failure::Control);
                Ok(())
            },
        )?;
        let before: Status = owner.status();
        let result: Result<super::ProtectedCompletion<'_>, Failure> = exchange_successor_with(
            owner,
            control,
            |_| Err(Failure::Entropy),
            |_| Err(Failure::Endpoint),
        );
        assert_eq!(result.err(), Some(Failure::Endpoint));
        assert_eq!(owner.status(), before);
        assert_eq!(owner.cookie_population()?, (1, 12));
        assert_successor_refused(owner, control, Failure::Ledger(LedgerError::Ticket));
        Ok(())
    })?;
    for change in [
        AuthenticationInvalidation::Stop,
        AuthenticationInvalidation::SourceEpoch,
        AuthenticationInvalidation::Configuration,
    ] {
        for committed in [false, true] {
            with_managed_successor(|owner, control, remote, authority, source| {
                let first: super::ProtectedCompletion<'_> =
                    first_managed_exchange(owner, control, 0)?;
                let before: Status = owner.status();
                if committed {
                    first.commit_for_successor(owner)?;
                    let expected: Failure =
                        invalidate_authentication(change, control, remote, authority, source)?;
                    assert_successor_refused(owner, control, expected);
                    assert_eq!(owner.cookie_population()?, (1, 12));
                } else {
                    let expected: Failure =
                        invalidate_authentication(change, control, remote, authority, source)?;
                    assert_eq!(first.commit_for_successor(owner), Err(expected));
                    assert_eq!(owner.status(), before);
                    assert_eq!(owner.cookie_population()?, (0, 0));
                }
                Ok(())
            })?;
        }
    }
    Ok(())
}

/// Independently sealed RATE with the second request coordinates and an ignored poll of 127.
const PROTECTED_RATE: [u8; 124] = [
    0x24, 0x00, 0x7f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x52, 0x41, 0x54, 0x45,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x04, 0x00, 0x24, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
    0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b,
    0x3c, 0x3d, 0x3e, 0x3f, 0x04, 0x04, 0x00, 0x28, 0x00, 0x10, 0x00, 0x10, 0x70, 0x71, 0x72, 0x73,
    0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0xac, 0x79, 0x43, 0xfc,
    0x61, 0x72, 0x4f, 0x12, 0xed, 0x43, 0xab, 0x1c, 0x36, 0x3a, 0xf2, 0x06,
];

/// A first RATE candidate with only one authenticator tag bit changed.
const RATE_BAD_TAG: [u8; 124] = {
    let mut packet: [u8; 124] = PROTECTED_RATE;
    packet[123] ^= 1;
    packet
};

/// Valid S2C tag, literal DENY and exact second request coordinates.
const PROTECTED_DENY: [u8; 124] = [
    0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x44, 0x45, 0x4e, 0x59,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x04, 0x00, 0x24, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
    0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b,
    0x3c, 0x3d, 0x3e, 0x3f, 0x04, 0x04, 0x00, 0x28, 0x00, 0x10, 0x00, 0x10, 0x70, 0x71, 0x72, 0x73,
    0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x88, 0xd3, 0xca, 0xb3,
    0x90, 0xfb, 0xc2, 0x47, 0x7a, 0xf9, 0x7e, 0xa4, 0x03, 0xbf, 0x73, 0xfc,
];

/// Literal DENY with the second request coordinates and only the UID extension.
const CORRELATED_DENY: [u8; 84] = [
    0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x44, 0x45, 0x4e, 0x59,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x04, 0x00, 0x24, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
    0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b,
    0x3c, 0x3d, 0x3e, 0x3f,
];

/// Public NTSN fixture with the second request's exact UID and origin.
const CORRELATED_NTSN: [u8; 84] = [
    0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4e, 0x54, 0x53, 0x4e,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x04, 0x00, 0x24, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
    0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b,
    0x3c, 0x3d, 0x3e, 0x3f,
];

/// The same NTSN followed by a well-framed forbidden Cookie extension.
const NTSN_WITH_COOKIE: [u8; 112] = {
    let mut packet: [u8; 112] = [0x5a; 112];
    let (prefix, suffix): (&mut [u8], &mut [u8]) = packet.split_at_mut(CORRELATED_NTSN.len());
    prefix.copy_from_slice(&CORRELATED_NTSN);
    suffix[0] = 0x02;
    suffix[1] = 0x04;
    suffix[2] = 0;
    suffix[3] = 28;
    packet
};

/// Independently sealed RSTR with the exact second request coordinates and public S2C key.
const PROTECTED_RESTRICTION: [u8; 124] = [
    0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x52, 0x53, 0x54, 0x52,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x04, 0x00, 0x24, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
    0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b,
    0x3c, 0x3d, 0x3e, 0x3f, 0x04, 0x04, 0x00, 0x28, 0x00, 0x10, 0x00, 0x10, 0x70, 0x71, 0x72, 0x73,
    0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x7c, 0x37, 0xbc, 0xdc,
    0x4f, 0x80, 0x01, 0xb7, 0x0a, 0x7f, 0x10, 0xb0, 0x47, 0x2d, 0x7f, 0xc6,
];

/// Valid S2C tag, exact second request coordinates and an unselected control code.
const PROTECTED_UNKNOWN: [u8; 124] = [
    0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x54, 0x45, 0x53, 0x54,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x04, 0x00, 0x24, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
    0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b,
    0x3c, 0x3d, 0x3e, 0x3f, 0x04, 0x04, 0x00, 0x28, 0x00, 0x10, 0x00, 0x10, 0x70, 0x71, 0x72, 0x73,
    0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x91, 0x4b, 0xa8, 0x7c,
    0x5d, 0x65, 0x97, 0x74, 0xd5, 0xd0, 0xb2, 0x37, 0xe3, 0x5c, 0x8d, 0xa7,
];

/// Valid S2C tag with the first request UID.
const DENY_PREDECESSOR_UID: [u8; 124] = [
    0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x44, 0x45, 0x4e, 0x59,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x04, 0x00, 0x24, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b,
    0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
    0x1c, 0x1d, 0x1e, 0x1f, 0x04, 0x04, 0x00, 0x28, 0x00, 0x10, 0x00, 0x10, 0x70, 0x71, 0x72, 0x73,
    0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x21, 0x2b, 0x8b, 0x59,
    0x85, 0x86, 0x20, 0x8b, 0x8e, 0x38, 0x80, 0xf2, 0x30, 0xba, 0x22, 0x3e,
];

/// Valid S2C tag with just the original wire token changed.
const DENY_WRONG_ORIGIN: [u8; 124] = [
    0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x44, 0x45, 0x4e, 0x59,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xfe,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x04, 0x00, 0x24, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
    0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b,
    0x3c, 0x3d, 0x3e, 0x3f, 0x04, 0x04, 0x00, 0x28, 0x00, 0x10, 0x00, 0x10, 0x70, 0x71, 0x72, 0x73,
    0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x53, 0x97, 0xc5, 0x43,
    0x23, 0x8c, 0x95, 0x70, 0x47, 0x74, 0xc9, 0x91, 0xc7, 0x39, 0xdf, 0x8c,
];

/// The same literal DENY with a tag bit flipped.
const DENY_BAD_TAG: [u8; 124] = {
    let mut bad: [u8; 124] = PROTECTED_DENY;
    bad[123] ^= 1;
    bad
};

/// The same authenticated prefix followed by one well-framed unknown extension.
const DENY_WITH_SUFFIX: [u8; 152] = {
    let mut packet: [u8; 152] = [0; 152];
    let (prefix, suffix): (&mut [u8], &mut [u8]) = packet.split_at_mut(PROTECTED_DENY.len());
    prefix.copy_from_slice(&PROTECTED_DENY);
    suffix[0] = 0xd0;
    suffix[1] = 0x0d;
    suffix[3] = 28;
    packet
};

/// Check that the extra-field control reaches semantic admission as a complete frame.
/// # Errors
/// Preserve an invalid framing limit or malformed fixture.
/// # Panics
/// Fail if the intended third field is absent.
fn assert_deny_suffix_framed() -> Result<(), Failure> {
    use ntp_wire::{FrameLimits, RawDatagram, frame_no_legacy_mac_datagram};

    let limits: FrameLimits = FrameLimits::new(4096, 64)
        .map_err(|error| Failure::Ledger(LedgerError::Reply(ReplyError::Frame(error))))?;
    let framed: RawDatagram<'_> = frame_no_legacy_mac_datagram(&DENY_WITH_SUFFIX, limits)
        .map_err(|error| Failure::Ledger(LedgerError::Reply(ReplyError::Frame(error))))?;
    assert_eq!(framed.extension_count(), 3);
    Ok(())
}

/// The charged DENY retires its adapter, leaving the request to the final owner.
/// # Errors
/// Preserve original managed fixture, request, packet or policy failures.
/// # Panics
/// Fail if any rejected datagram bypasses charging, retirement consumes the
/// outstanding request, or a stale retired ticket changes either policy record.
#[test]
fn managed_protected_deny_retires_only_the_exact_authenticated_successor() -> Result<(), Failure> {
    use crate::{SourcePolicyError, SourcePolicySnapshot, SourcePolicyTarget};

    assert_deny_suffix_framed()?;
    with_managed_successor(|owner, control, remote, authority, source| {
        let clean: Status = owner.status();
        let mut entered: bool = false;
        assert!(matches!(
            exchange_successor_mixed(owner, control, successor_entropy, |_| {
                entered = true;
                Script::new(ScriptSend::Full, Vec::new())
            }),
            Err(Failure::Control)
        ));
        assert!(!entered);
        assert_eq!(owner.status(), clean);
        let expiry: Instant = control.expiry;
        let first: super::ProtectedCompletion<'_> = first_managed_exchange(owner, control, 0)?;
        let first_receipt: Receipt = first.history().receipt;
        first.commit_for_successor(owner)?;
        let other_port: u16 = remote.port().checked_add(1).ok_or(Failure::Endpoint)?;
        let other_peer: SocketAddr = SocketAddr::new(remote.ip(), other_port);
        let mut backend_lifetime: Option<std::sync::Weak<()>> = None;
        let selected: RetiredProtectedResponse<'_> =
            exchange_successor_mixed(owner, control, successor_entropy, |peer: SocketAddr| {
                assert_eq!(peer, remote);
                let script: Script = Script::new(
                    ScriptSend::Full,
                    vec![
                        ScriptRead::Packet(&PROTECTED_DENY, other_peer),
                        ScriptRead::Packet(&DENY_PREDECESSOR_UID, peer),
                        ScriptRead::Packet(&DENY_WRONG_ORIGIN, peer),
                        ScriptRead::Packet(&DENY_BAD_TAG, peer),
                        ScriptRead::Packet(&DENY_WITH_SUFFIX, peer),
                        ScriptRead::Packet(&PROTECTED_DENY, peer),
                    ],
                );
                backend_lifetime = Some(Arc::downgrade(&script.lifetime));
                script
            })?;
        let RetiredProtectedResponse::Control(retired) = selected else {
            return Err(Failure::Control);
        };
        let lifetime: std::sync::Weak<()> = backend_lifetime.ok_or(Failure::Control)?;
        assert_eq!(lifetime.strong_count(), 0);
        assert!(tokio::runtime::Handle::try_current().is_err());
        assert_eq!(
            owner.status(),
            Status {
                phase: RequestPhase::Pending,
                attempts: 30,
                frames: 25,
                bytes: 130_160,
                available: 0,
                used: 2,
                uids: 2,
            }
        );
        assert!(owner.last_receipt() == Some(first_receipt));
        assert_eq!(owner.cookie_population()?, (0, 0));
        assert_eq!(control.expiry, expiry);
        retired.check_current(owner)?;
        let policy: SourcePolicySnapshot = source
            .policy_snapshot(remote)
            .map_err(Failure::SourcePolicy)?;
        assert!(!policy.source().refused() && !policy.endpoint().refused());
        let before: Status = owner.status();
        let target: SourcePolicyTarget = authority
            .source_target(source)
            .map_err(Failure::SourcePolicy)?;
        authority
            .reenable(&[target])
            .map_err(Failure::SourcePolicy)?;
        let reenabled: SourcePolicySnapshot = source
            .policy_snapshot(remote)
            .map_err(Failure::SourcePolicy)?;
        assert_eq!(
            reenabled
                .source()
                .revision()
                .checked_sub(policy.source().revision()),
            Some(1)
        );
        assert_deny_record(policy.endpoint(), reenabled.endpoint(), false);
        assert_eq!(
            retired.commit(owner).err(),
            Some(Failure::SourcePolicy(SourcePolicyError::Stale))
        );
        let after: SourcePolicySnapshot = source
            .policy_snapshot(remote)
            .map_err(Failure::SourcePolicy)?;
        assert_deny_record(reenabled.source(), after.source(), false);
        assert_deny_record(reenabled.endpoint(), after.endpoint(), false);
        drop(control.published_guard()?);
        assert_eq!(owner.status(), before);
        assert!(owner.last_receipt() == Some(first_receipt));
        assert_eq!(owner.cookie_population()?, (0, 0));
        Ok(())
    })
}

/// Compare the retained record fields independently of snapshot capture time.
/// # Panics
/// Fail if denial changes an unrelated field, or if refusal and revision differ
/// from the exact expected policy effect.
fn assert_deny_record(
    before: &SourceRestrictionSnapshot,
    after: &SourceRestrictionSnapshot,
    deny: bool,
) {
    assert!(!before.refused() && !before.unavailable());
    assert_eq!(after.refused(), deny);
    assert!(!after.unavailable());
    assert_eq!(
        after.revision().checked_sub(before.revision()),
        Some(u64::from(deny))
    );
    assert_eq!(after.floor(), before.floor());
    assert_eq!(after.interval(), before.interval());
    assert_eq!(after.maximum_interval(), before.maximum_interval());
}

/// A charged, retired DENY cannot act on a receive bracket that reaches cutoff.
/// # Errors
/// Preserve original fixture, authentication or policy observation failures.
/// # Panics
/// Fail if retirement spends other work, the original cutoff does not refuse,
/// or the refusal changes either restriction or the original receipt and pool.
#[test]
fn managed_retired_deny_refuses_receive_capture_at_cutoff() -> Result<(), Failure> {
    use crate::SourcePolicySnapshot;

    with_managed_successor(|owner, control, remote, _, source| {
        let expiry: Instant = control.expiry;
        let first: super::ProtectedCompletion<'_> = first_managed_exchange(owner, control, 0)?;
        let first_receipt: Receipt = first.history().receipt;
        first.commit_for_successor(owner)?;
        let mut backend_lifetime: Option<std::sync::Weak<()>> = None;
        let selected: RetiredProtectedResponse<'_> =
            exchange_successor_mixed(owner, control, successor_entropy, |peer: SocketAddr| {
                let mut script: Script = Script::new(
                    ScriptSend::Full,
                    vec![ScriptRead::Packet(&PROTECTED_DENY, peer)],
                );
                script.read_after = Some(expiry);
                backend_lifetime = Some(Arc::downgrade(&script.lifetime));
                script
            })?;
        let RetiredProtectedResponse::Control(retired) = selected else {
            return Err(Failure::Control);
        };
        let lifetime: std::sync::Weak<()> = backend_lifetime.ok_or(Failure::Control)?;
        assert_eq!(lifetime.strong_count(), 0);
        assert!(tokio::runtime::Handle::try_current().is_err());
        let pending: Status = owner.status();
        assert_eq!(
            pending,
            Status {
                phase: RequestPhase::Pending,
                attempts: 30,
                frames: 30,
                bytes: 131_072 - fixtures::HEALTHY.len() - PROTECTED_DENY.len(),
                available: 0,
                used: 2,
                uids: 2,
            }
        );
        let before: SourcePolicySnapshot = source
            .policy_snapshot(remote)
            .map_err(Failure::SourcePolicy)?;
        let outcome: Result<AppliedProtectedControl, Failure> = retired.commit(owner);
        assert!(
            Instant::now() < expiry,
            "the real processing clock must stay inside the modeled cutoff"
        );
        assert_eq!(
            outcome.err(),
            Some(Failure::Deadline),
            "the original receive bracket ending at cutoff cannot authorize DENY"
        );
        let after: SourcePolicySnapshot = source
            .policy_snapshot(remote)
            .map_err(Failure::SourcePolicy)?;
        assert_deny_record(before.source(), after.source(), false);
        assert_deny_record(before.endpoint(), after.endpoint(), false);
        assert_eq!(owner.status(), pending);
        assert!(owner.last_receipt() == Some(first_receipt));
        assert_eq!(owner.cookie_population()?, (0, 0));
        assert_eq!(control.expiry, expiry);
        drop(control.published_guard()?);
        Ok(())
    })
}

/// The two disjoint invalid positions for an otherwise in-window receive tail.
enum InvalidReceiveTail {
    /// The trailing reading precedes the actual leading reading.
    BeforeArrival,
    /// The trailing reading follows actual processing and remains before expiry.
    AfterProcessing,
}

/// Exercise the real retired control effect with one changed trailing reading.
///
/// # Errors
/// Preserve actual successor, fixture clock and policy observation failures.
/// # Panics
/// Fail if receive chronology changes retained authority, work or policy.
fn check_retired_deny_receive_order(invalid: &InvalidReceiveTail) -> Result<(), Failure> {
    use crate::SourcePolicySnapshot;

    with_managed_successor(|owner, control, remote, _, source| {
        let expiry: Instant = control.expiry;
        let trailing: Instant = match invalid {
            // A valid leading reading follows a send within this invocation.
            InvalidReceiveTail::BeforeArrival => {
                control.origin.checked_sub(Duration::from_nanos(1))
            }
            InvalidReceiveTail::AfterProcessing => expiry.checked_sub(Duration::from_nanos(1)),
        }
        .ok_or(Failure::Control)?;
        let first: super::ProtectedCompletion<'_> = first_managed_exchange(owner, control, 0)?;
        let first_receipt: Receipt = first.history().receipt;
        first.commit_for_successor(owner)?;
        let mut backend_lifetime: Option<std::sync::Weak<()>> = None;
        let selected: RetiredProtectedResponse<'_> =
            exchange_successor_mixed(owner, control, successor_entropy, |peer: SocketAddr| {
                let mut script: Script = Script::new(
                    ScriptSend::Full,
                    vec![ScriptRead::Packet(&PROTECTED_DENY, peer)],
                );
                script.read_after = Some(trailing);
                backend_lifetime = Some(Arc::downgrade(&script.lifetime));
                script
            })?;
        let RetiredProtectedResponse::Control(retired) = selected else {
            return Err(Failure::Control);
        };
        let lifetime: std::sync::Weak<()> = backend_lifetime.ok_or(Failure::Control)?;
        assert_eq!(lifetime.strong_count(), 0);
        assert!(tokio::runtime::Handle::try_current().is_err());
        let pending: Status = owner.status();
        assert_eq!(
            pending,
            Status {
                phase: RequestPhase::Pending,
                attempts: 30,
                frames: 30,
                bytes: 131_072 - fixtures::HEALTHY.len() - PROTECTED_DENY.len(),
                available: 0,
                used: 2,
                uids: 2,
            }
        );
        let cookies: (usize, usize) = owner.cookie_population()?;
        assert_eq!(cookies, (0, 0));
        let before: SourcePolicySnapshot = source
            .policy_snapshot(remote)
            .map_err(Failure::SourcePolicy)?;
        let outcome: Result<AppliedProtectedControl, Failure> = retired.commit(owner);
        let upper_bound: Instant = match invalid {
            InvalidReceiveTail::BeforeArrival => expiry,
            InvalidReceiveTail::AfterProcessing => trailing,
        };
        assert!(
            Instant::now() < upper_bound,
            "actual processing must stay inside the modeled receive boundary"
        );
        assert_eq!(
            outcome.err(),
            Some(Failure::Ledger(LedgerError::Time)),
            "invalid receive chronology must not authorize protected DENY"
        );
        let after: SourcePolicySnapshot = source
            .policy_snapshot(remote)
            .map_err(Failure::SourcePolicy)?;
        assert_deny_record(before.source(), after.source(), false);
        assert_deny_record(before.endpoint(), after.endpoint(), false);
        assert_eq!(owner.status(), pending);
        assert!(owner.last_receipt() == Some(first_receipt));
        assert_eq!(owner.cookie_population()?, cookies);
        assert_eq!(control.expiry, expiry);
        drop(control.published_guard()?);
        Ok(())
    })
}

/// Refuse a trailing receive reading before the original leading reading.
///
/// # Errors
/// Propagate setup errors from the actual retired control fixture.
#[test]
fn managed_retired_deny_refuses_reversed_receive_capture() -> Result<(), Failure> {
    check_retired_deny_receive_order(&InvalidReceiveTail::BeforeArrival)
}

/// Refuse a trailing receive reading after processing and before expiry.
///
/// # Errors
/// Propagate setup errors from the actual retired control fixture.
#[test]
fn managed_retired_deny_refuses_future_receive_tail() -> Result<(), Failure> {
    check_retired_deny_receive_order(&InvalidReceiveTail::AfterProcessing)
}

/// Charge the complete datagram before refusing its inverted send-to-read order.
/// # Errors
/// Preserve actual setup, successor and policy observation failures.
/// # Panics
/// Fail if the receive debit, original Capture result, retirement or records differ.
#[test]
fn known_protected_datagram_with_invalid_capture_charges_frame_and_bytes() -> Result<(), Failure> {
    use crate::SourcePolicySnapshot;
    use crate::nts_ke_experiment::association::ledger::{RecoveryIntent, RecoveryReason};

    for final_frame in [false, true] {
        with_managed_successor(|owner, control, remote, _, source| {
            let expiry: Instant = control.expiry;
            // The actual successor send must follow the original invocation origin.
            let read_before: Instant = control
                .origin
                .checked_sub(Duration::from_nanos(1))
                .ok_or(Failure::Control)?;
            let first: super::ProtectedCompletion<'_> = first_managed_exchange(owner, control, 0)?;
            let first_receipt: Receipt = first.history().receipt;
            first.commit_for_successor(owner)?;
            let before: Status = owner.status();
            assert!(before.frames > 1);
            let preceding: usize = if final_frame { before.frames - 1 } else { 0 };
            let before_policy: SourcePolicySnapshot = source
                .policy_snapshot(remote)
                .map_err(Failure::SourcePolicy)?;
            let read_residue: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(usize::MAX));
            let mut backend_lifetime: Option<std::sync::Weak<()>> = None;
            let result: Result<RetiredProtectedResponse<'_>, Failure> =
                exchange_successor_mixed(owner, control, successor_entropy, |peer: SocketAddr| {
                    let mut reads: Vec<ScriptRead> = (0..preceding)
                        .map(|_| ScriptRead::Packet(&PROTECTED_UNKNOWN, peer))
                        .collect();
                    reads.push(ScriptRead::Packet(&PROTECTED_DENY, peer));
                    reads.push(ScriptRead::Packet(fixtures::SUCCESSOR_OLD_S2C, peer));
                    let mut script: Script = Script::new(ScriptSend::Full, reads);
                    script.read_before_at_residue = Some((1, read_before));
                    script.read_residue = Some(Arc::clone(&read_residue));
                    backend_lifetime = Some(Arc::downgrade(&script.lifetime));
                    script
                });
            assert_eq!(
                result.err(),
                Some(Failure::Datagram(DatagramError::Capture))
            );
            assert_eq!(read_residue.load(Ordering::Relaxed), 1);
            let lifetime: std::sync::Weak<()> = backend_lifetime.ok_or(Failure::Control)?;
            assert_eq!(lifetime.strong_count(), 0);
            assert!(tokio::runtime::Handle::try_current().is_err());
            let pending: Status = owner.status();
            assert_eq!(
                (pending.frames, pending.bytes),
                (
                    before.frames - preceding - 1,
                    before.bytes - preceding * PROTECTED_UNKNOWN.len() - PROTECTED_DENY.len()
                ),
                "a consumed complete protected datagram must charge one frame and its exact bytes"
            );
            if final_frame {
                assert_eq!(pending.phase, RequestPhase::Idle);
                let intent: RecoveryIntent =
                    owner.take_recovery_intent().ok_or(Failure::Control)?;
                assert_eq!(intent.reason(), RecoveryReason::ReceiveWork);
                assert_eq!(intent.not_before(), expiry);
            } else {
                assert_eq!(pending.phase, RequestPhase::Pending);
            }
            assert!(owner.take_recovery_intent().is_none());
            assert_eq!(
                (
                    pending.attempts,
                    pending.available,
                    pending.used,
                    pending.uids
                ),
                (before.attempts - 1, 0, before.used + 1, before.uids + 1)
            );
            assert!(owner.last_receipt() == Some(first_receipt));
            assert_eq!(owner.cookie_population()?, (0, 0));
            assert_eq!(control.expiry, expiry);
            drop(control.published_guard()?);
            let after_policy: SourcePolicySnapshot = source
                .policy_snapshot(remote)
                .map_err(Failure::SourcePolicy)?;
            for (prior, current) in [
                (before_policy.source(), after_policy.source()),
                (before_policy.endpoint(), after_policy.endpoint()),
            ] {
                assert!(!current.refused() && !current.unavailable());
                assert_eq!(current.revision().checked_sub(prior.revision()), Some(1));
                assert!(current.floor() > prior.floor());
                assert_eq!(current.interval(), prior.interval());
                assert_eq!(current.maximum_interval(), prior.maximum_interval());
            }
            Ok(())
        })?;
    }
    Ok(())
}

/// Preserve the authenticated DENY path as the valid neighboring case.
/// # Errors
/// Propagate actual fixture, exchange, retirement or policy refusal.
#[test]
fn managed_retired_deny_commits_only_its_source_and_exact_endpoint() -> Result<(), Failure> {
    assert_managed_retired_deny(&PROTECTED_DENY, ControlProtection::Authenticated)
}

/// Apply a correlated-only DENY through the same retired successor owner.
/// # Errors
/// Propagate actual fixture, exchange, retirement or policy refusal.
#[test]
fn managed_retired_correlated_only_deny_has_exact_policy_scope() -> Result<(), Failure> {
    assert_managed_retired_deny(&CORRELATED_DENY, ControlProtection::CorrelatedOnly)
}

/// Consume the selected exact DENY and observe its committed protection and policy.
/// # Errors
/// Preserve fixture, exchange, retirement and policy failures.
/// # Panics
/// Fail on incorrect protection, scope, history, cookie or budget handling.
fn assert_managed_retired_deny(
    packet: &'static [u8],
    protection: ControlProtection,
) -> Result<(), Failure> {
    use crate::source_policy::SourceBinding;
    use crate::{SourceHandle, SourcePolicySnapshot};

    with_managed_successor(|owner, control, remote, authority, source| {
        let alias: SourceHandle = authority.source(1).map_err(Failure::SourcePolicy)?;
        let other_port: u16 = remote.port().checked_add(1).ok_or(Failure::Endpoint)?;
        let other_peer: SocketAddr = SocketAddr::new(remote.ip(), other_port);
        let _alias_other: SourceBinding = alias.bind(other_peer).map_err(Failure::SourcePolicy)?;
        let first: super::ProtectedCompletion<'_> = first_managed_exchange(owner, control, 0)?;
        let first_receipt: Receipt = first.history().receipt;
        first.commit_for_successor(owner)?;
        assert_eq!(first_receipt.serial(), 1);
        assert_eq!(owner.cookie_population()?, (1, 12));
        let before: Status = owner.status();
        let selected: RetiredProtectedResponse<'_> =
            exchange_successor_mixed(owner, control, successor_entropy, |peer: SocketAddr| {
                Script::new(ScriptSend::Full, vec![ScriptRead::Packet(packet, peer)])
            })?;
        let RetiredProtectedResponse::Control(retired) = selected else {
            return Err(Failure::Control);
        };
        let pending: Status = owner.status();
        assert_eq!(
            pending,
            Status {
                phase: RequestPhase::Pending,
                attempts: before.attempts - 1,
                frames: before.frames - 1,
                bytes: before.bytes - packet.len(),
                available: 0,
                used: before.used + 1,
                uids: before.uids + 1,
            }
        );
        let original: SourcePolicySnapshot = source
            .policy_snapshot(remote)
            .map_err(Failure::SourcePolicy)?;
        let unrelated: SourcePolicySnapshot = alias
            .policy_snapshot(other_peer)
            .map_err(Failure::SourcePolicy)?;
        let earliest: Instant = Instant::now();
        let applied: AppliedProtectedControl = retired.commit(owner)?;
        assert_eq!(applied.kind(), ProtectedControlKind::Deny);
        assert_eq!(applied.protection(), protection);
        assert_eq!(applied.remote(), remote);
        assert_eq!(applied.serial(), 2);
        assert!((earliest..=Instant::now()).contains(&applied.decided_at()));
        assert!(applied.decided_at() < control.expiry);
        assert_eq!(
            owner.status(),
            Status {
                phase: RequestPhase::Idle,
                ..pending
            }
        );
        assert!(owner.last_receipt() == Some(first_receipt));
        assert_eq!(owner.cookie_population()?, (0, 0));
        assert_eq!(control.cancel()?, CancelOutcome::AlreadyCommitted);

        for (handle, peer, denied_source, denied_endpoint) in [
            (source, remote, true, true),
            (&alias, remote, false, true),
            (source, other_peer, true, false),
            (&alias, other_peer, false, false),
        ] {
            let observed: SourcePolicySnapshot = handle
                .policy_snapshot(peer)
                .map_err(Failure::SourcePolicy)?;
            let before_source: &SourceRestrictionSnapshot = if denied_source {
                original.source()
            } else {
                unrelated.source()
            };
            let before_endpoint: &SourceRestrictionSnapshot = if denied_endpoint {
                original.endpoint()
            } else {
                unrelated.endpoint()
            };
            assert_deny_record(before_source, observed.source(), denied_source);
            assert_deny_record(before_endpoint, observed.endpoint(), denied_endpoint);
        }
        Ok(())
    })
}

/// Check interval, cap and the original send-debit floor at the actual policy owner.
/// # Panics
/// Fail if the record changes availability or the expected retained values differ.
fn assert_protected_rate_record(
    record: &SourceRestrictionSnapshot,
    expected_ms: u64,
    floor: Option<Instant>,
    revision: u64,
) {
    assert!(!record.refused() && !record.unavailable());
    assert_eq!(record.interval(), Duration::from_millis(expected_ms));
    assert_eq!(record.maximum_interval(), Duration::from_millis(10));
    assert_eq!(record.floor(), floor);
    assert_eq!(record.revision(), revision);
}

/// Try independent observations just before and at the retained exact endpoint floor.
/// The healthy attempt supplies a real ordinary protected reply after the send.
/// # Errors
/// Preserve checked association, observation or runtime refusal.
/// # Panics
/// Fail if a premature observation enters the adapter or an at-cap probe is withheld.
fn check_protected_rate_send_floor(
    config: &Config,
    source: &crate::SourceHandle,
    start: Instant,
    floor: Instant,
) -> Result<(), Failure> {
    use crate::{SourcePolicyError, SourcePolicySnapshot};

    let early: Instant = floor
        .checked_sub(Duration::from_nanos(1))
        .ok_or(Failure::Deadline)?;
    for (at, should_send) in [(early, false), (floor, true)] {
        with_config_owner(
            config,
            start,
            S2C,
            &[(0x11, 12)],
            |owner, control, remote| {
                let before: SourcePolicySnapshot = source
                    .policy_snapshot(remote)
                    .map_err(Failure::SourcePolicy)?;
                let mut selected: Script = Script::new(
                    ScriptSend::Full,
                    vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
                );
                selected.ready.push_back(ScriptReady::After(floor));
                selected.send_before = Some(SoftwareClock {
                    mono: at,
                    mono_after: at,
                    utc: UNIX_EPOCH,
                });
                let mut backend: Backend = backend(remote, selected);
                let outcome: Result<OperationCompletion<'_>, Failure> =
                    runtime()?.block_on(operation(owner, control, entropy, &mut backend));
                let observed: &Script = script(&backend)?;
                let after: SourcePolicySnapshot = source
                    .policy_snapshot(remote)
                    .map_err(Failure::SourcePolicy)?;
                if should_send {
                    assert!(outcome.is_ok());
                    assert_eq!((observed.send_calls, observed.receive_calls), (1, 1));
                    assert_protected_rate_record(
                        after.source(),
                        8,
                        at.checked_add(Duration::from_millis(8)),
                        before.source().revision() + 1,
                    );
                    assert_protected_rate_record(
                        after.endpoint(),
                        10,
                        at.checked_add(Duration::from_millis(10)),
                        before.endpoint().revision() + 1,
                    );
                } else {
                    assert!(matches!(
                        outcome,
                        Err(Failure::SourcePolicy(SourcePolicyError::NotDue))
                    ));
                    assert_eq!((observed.send_calls, observed.receive_calls), (0, 0));
                    assert_protected_rate_record(
                        after.source(),
                        8,
                        before.source().floor(),
                        before.source().revision(),
                    );
                    assert_protected_rate_record(
                        after.endpoint(),
                        10,
                        before.endpoint().floor(),
                        before.endpoint().revision(),
                    );
                }
                Ok(())
            },
        )?;
    }
    Ok(())
}

/// Submit the same bytes again at the actual closed receiver boundary.
/// # Panics
/// Fail if replay enters the consumed invocation or changes ledger accounting.
fn assert_protected_rate_replay_refused(
    owner: &mut AssociationOwner<'_>,
    control: &Control,
    remote: SocketAddr,
) {
    let settled: Status = owner.status();
    let replay: Result<StagedProtectedSuccessor<'_>, Failure> = owner.stage_protected_successor(
        control,
        &PROTECTED_RATE,
        remote,
        &SoftwareClock::capture(),
        &Arc::new(()),
    );
    assert!(matches!(replay, Err(Failure::Control)));
    assert_eq!(owner.status(), settled);
}

/// Consume one correlated RATE under real policy, with a forged neighbor and replay.
/// # Errors
/// Preserve fixture, deadline, original receive and resulting send refusals.
/// # Panics
/// Fail if the authenticated control ignores exact authority, lowers an existing
/// floor, publishes a time/cookie result, or prevents a later at-cap probe.
#[test]
fn managed_retired_rate_preserves_independent_intervals_and_exact_next_send() -> Result<(), Failure>
{
    use crate::source_policy::{FixtureRequest, SourceBinding, fixture_send};
    use crate::{SourceHandle, SourcePolicySnapshot};

    for keep_later_floor in [false, true] {
        let start: Instant = Instant::now()
            .checked_sub(Duration::from_millis(20))
            .ok_or(Failure::Deadline)?;
        let (config, source, authority): (Config, SourceHandle, SourcePolicyAuthority) =
            managed_attempt_config(start, Duration::from_millis(4), Duration::from_millis(10))?;
        with_config_owner(
            &config,
            start,
            S2C,
            &[(0x11, 12)],
            |owner, control, remote| {
                let first: super::ProtectedCompletion<'_> =
                    first_managed_exchange(owner, control, 0)?;
                let first_receipt: Receipt = first.history().receipt;
                first.commit_for_successor(owner)?;
                let before: Status = owner.status();
                let selected: RetiredProtectedResponse<'_> =
                    scripted_managed_successor(owner, control, ProtectedControlKind::Rate)?;
                let RetiredProtectedResponse::Control(retired) = selected else {
                    return Err(Failure::Control);
                };
                let pending: Status = owner.status();
                assert_eq!(pending.phase, RequestPhase::Pending);
                assert_eq!(pending.frames, before.frames - 2);
                assert_eq!(pending.bytes, before.bytes - 2 * PROTECTED_RATE.len());

                if keep_later_floor {
                    // The separate alias uses the policy fixture clock to reserve a later
                    // endpoint floor without waiting or directly modifying the record.
                    let later: Instant = control
                        .expiry
                        .checked_sub(Duration::from_secs(5))
                        .ok_or(Failure::Deadline)?;
                    let alias: SourceHandle = authority.source(1).map_err(Failure::SourcePolicy)?;
                    let binding: SourceBinding =
                        alias.bind(remote).map_err(Failure::SourcePolicy)?;
                    let _competing: FixtureRequest = fixture_send(&binding, &Arc::new(()), later)
                        .map_err(Failure::SourcePolicy)?;
                }
                let original: SourcePolicySnapshot = source
                    .policy_snapshot(remote)
                    .map_err(Failure::SourcePolicy)?;
                assert_eq!(original.source().interval(), Duration::from_millis(4));
                assert_eq!(original.endpoint().interval(), Duration::from_millis(8));
                let applied: AppliedProtectedControl = retired.commit(owner)?;
                assert_eq!(applied.kind(), ProtectedControlKind::Rate);
                assert_eq!(applied.serial(), 2);
                assert_eq!(
                    owner.status(),
                    Status {
                        phase: RequestPhase::Idle,
                        ..pending
                    }
                );
                assert!(owner.last_receipt() == Some(first_receipt));
                assert_eq!(owner.cookie_population()?, (0, 0));

                let source_floor: Option<Instant> =
                    applied.decided_at().checked_add(Duration::from_millis(8));
                let revised_endpoint: Instant = applied
                    .decided_at()
                    .checked_add(Duration::from_millis(10))
                    .ok_or(Failure::Deadline)?;
                let retained: Instant = original.endpoint().floor().ok_or(Failure::Control)?;
                let endpoint_floor: Instant = if keep_later_floor {
                    assert!(retained > revised_endpoint);
                    retained
                } else {
                    assert!(retained < revised_endpoint);
                    revised_endpoint
                };
                assert_protected_rate_replay_refused(owner, control, remote);
                let after: SourcePolicySnapshot = source
                    .policy_snapshot(remote)
                    .map_err(Failure::SourcePolicy)?;
                assert_protected_rate_record(
                    after.source(),
                    8,
                    source_floor,
                    original.source().revision() + 1,
                );
                assert_protected_rate_record(
                    after.endpoint(),
                    10,
                    Some(endpoint_floor),
                    original.endpoint().revision() + 1,
                );
                if !keep_later_floor {
                    check_protected_rate_send_floor(&config, &source, start, endpoint_floor)?;
                }
                Ok(())
            },
        )?;
    }
    Ok(())
}

/// The mixed second receive keeps the original budgets and normal cookie commit.
/// # Errors
/// Preserve original managed fixture, request, packet or policy failures.
/// # Panics
/// Fail if an unselected control wins, a rejected read avoids its charge, or
/// the selected ordinary response changes live cookies before its final commit.
#[test]
fn managed_protected_ordinary_follows_rejected_controls_without_renewal() -> Result<(), Failure> {
    use crate::SourcePolicySnapshot;

    with_managed_successor(|owner, control, remote, _authority, source| {
        let expiry: Instant = control.expiry;
        first_managed_exchange(owner, control, 0)?.commit_for_successor(owner)?;
        let mut backend_lifetime: Option<std::sync::Weak<()>> = None;
        let selected: RetiredProtectedResponse<'_> =
            exchange_successor_mixed(owner, control, successor_entropy, |peer: SocketAddr| {
                assert_eq!(peer, remote);
                let script: Script = Script::new(
                    ScriptSend::Full,
                    vec![
                        ScriptRead::Packet(&PROTECTED_UNKNOWN, peer),
                        ScriptRead::Packet(&DENY_WRONG_ORIGIN, peer),
                        ScriptRead::Packet(fixtures::SUCCESSOR_HEALTHY, peer),
                        ScriptRead::Packet(fixtures::SUCCESSOR_OLD_S2C, peer),
                    ],
                );
                backend_lifetime = Some(Arc::downgrade(&script.lifetime));
                script
            })?;
        let RetiredProtectedResponse::Ordinary(retired) = selected else {
            return Err(Failure::Control);
        };
        let lifetime: std::sync::Weak<()> = backend_lifetime.ok_or(Failure::Control)?;
        assert_eq!(lifetime.strong_count(), 0);
        assert!(tokio::runtime::Handle::try_current().is_err());
        assert_eq!(retired.remaining_attempts(), 55);
        assert_eq!(retired.history().receipt.serial(), 2);
        assert_eq!(
            owner.status(),
            Status {
                phase: RequestPhase::Idle,
                attempts: 30,
                frames: 27,
                bytes: 130_404,
                available: 0,
                used: 2,
                uids: 2,
            }
        );
        assert!(owner.last_receipt() == Some(retired.history().receipt));
        assert_eq!(owner.cookie_population()?, (0, 0));
        assert_eq!(control.expiry, expiry);
        let policy: SourcePolicySnapshot = source
            .policy_snapshot(remote)
            .map_err(Failure::SourcePolicy)?;
        assert!(!policy.source().refused() && !policy.endpoint().refused());
        assert!(retired.commit(owner)?.into_summary().transmitted);
        assert_eq!(owner.cookie_population()?, (1, 12));
        assert_eq!(control.cancel()?, CancelOutcome::AlreadyCommitted);
        Ok(())
    })
}

/// Confirm both policy records changed only by the one protected send.
/// # Panics
/// Fail on refusal, unavailability, a second revision, or a changed interval.
fn assert_nak_only_send_changed_policy(
    before: &crate::SourcePolicySnapshot,
    after: &crate::SourcePolicySnapshot,
) {
    for (prior, current) in [
        (before.source(), after.source()),
        (before.endpoint(), after.endpoint()),
    ] {
        assert!(!current.refused() && !current.unavailable());
        assert_eq!(current.revision().checked_sub(prior.revision()), Some(1));
        assert!(current.floor() > prior.floor());
        assert_eq!(current.interval(), prior.interval());
        assert_eq!(current.maximum_interval(), prior.maximum_interval());
    }
}

/// Admit the protected reply on the same request after charging the correlated NTSN.
/// # Errors
/// Preserve the fixture, original receive and authenticated commit refusals.
/// # Panics
/// Fail if NTSN ends the receive, refreshes policy, or changes receipt or cookie custody.
#[test]
fn managed_successor_nak_leaves_ordinary_reply_opportunity() -> Result<(), Failure> {
    with_managed_successor(|owner, control, remote, _, source| {
        let expiry: Instant = control.expiry;
        first_managed_exchange(owner, control, 0)?.commit_for_successor(owner)?;
        let before: crate::SourcePolicySnapshot = source
            .policy_snapshot(remote)
            .map_err(Failure::SourcePolicy)?;
        assert_eq!(owner.cookie_population()?, (1, 12));
        let residue: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(3));
        let captures: Arc<Mutex<Vec<SoftwareClock>>> = Arc::new(Mutex::new(Vec::with_capacity(2)));
        let mut lifetime: Option<std::sync::Weak<()>> = None;
        let (selected, first_nak_at): (RetiredProtectedResponse<'_>, Option<Instant>) =
            exchange_successor_observing(owner, control, successor_entropy, |peer: SocketAddr| {
                assert_eq!(peer, remote);
                let mut script: Script = Script::new(
                    ScriptSend::Full,
                    vec![
                        ScriptRead::Packet(&CORRELATED_NTSN, peer),
                        ScriptRead::Packet(fixtures::SUCCESSOR_OLD_S2C, peer),
                        ScriptRead::Failed(io::ErrorKind::UnexpectedEof),
                    ],
                );
                script.read_residue = Some(Arc::clone(&residue));
                script.read_captures = Some(Arc::clone(&captures));
                lifetime = Some(Arc::downgrade(&script.lifetime));
                Ok(backend(peer, script))
            })
            .map_err(|failure| failure.cause())?
            .into_parts();
        let RetiredProtectedResponse::Ordinary(retired) = selected else {
            return Err(Failure::Control);
        };
        assert_eq!(residue.load(Ordering::Relaxed), 1);
        assert_eq!(lifetime.ok_or(Failure::Control)?.strong_count(), 0);
        assert!(tokio::runtime::Handle::try_current().is_err());
        {
            let readings: MutexGuard<'_, Vec<SoftwareClock>> =
                captures.lock().map_err(|_| Failure::Control)?;
            assert_eq!(readings.len(), 2);
            let first_at: Instant = readings.first().ok_or(Failure::Control)?.monotonic();
            assert_eq!(first_nak_at, Some(first_at));
        }
        let receipt: Receipt = retired.history().receipt;
        assert_eq!((receipt.serial(), receipt.cookie_counts()), (2, (1, 1)));
        assert_eq!(retired.history().remote, remote);
        assert_eq!(retired.history().packet_bytes, 252);
        assert_eq!(
            owner.status(),
            Status {
                phase: RequestPhase::Idle,
                attempts: 30,
                frames: 29,
                bytes: 130_932 - CORRELATED_NTSN.len() - fixtures::SUCCESSOR_OLD_S2C.len(),
                available: 0,
                used: 2,
                uids: 2,
            }
        );
        assert!(owner.last_receipt() == Some(receipt));
        assert_eq!(owner.cookie_population()?, (0, 0));
        assert!(retired.commit(owner)?.into_summary().transmitted);
        assert_eq!(owner.cookie_population()?, (1, 12));
        assert!(owner.take_recovery_intent().is_none());
        assert_eq!(control.expiry, expiry);
        let after: crate::SourcePolicySnapshot = source
            .policy_snapshot(remote)
            .map_err(Failure::SourcePolicy)?;
        assert_nak_only_send_changed_policy(&before, &after);
        Ok(())
    })
}

/// The selected terminal histories of the charged NTSN receiver.
enum NakTerminal {
    /// Repeated matching NTSN consumes the entire original frame allowance.
    ReceiveWork,
    /// A forbidden Cookie precedes a distinct backend EOF.
    InvalidThenEof,
    /// A matching NTSN precedes the same distinct backend EOF.
    ValidThenEof,
}

/// Follow a genuine first commit into the selected terminal successor history.
/// # Errors
/// Preserve fixture, ledger and capture-observation refusals.
/// # Panics
/// Fail on lost terminal cause, changed policy, or incorrect NTSN observation or debit.
fn assert_nak_terminal_history(terminal: &NakTerminal) -> Result<(), Failure> {
    use crate::nts_ke_experiment::association::ledger::{RecoveryIntent, RecoveryReason};

    with_managed_successor(|owner, control, remote, _, source| {
        let expiry: Instant = control.expiry;
        let first: super::ProtectedCompletion<'_> = first_managed_exchange(owner, control, 0)?;
        let first_receipt: Receipt = first.history().receipt;
        first.commit_for_successor(owner)?;
        assert_eq!(owner.cookie_population()?, (1, 12));
        assert!(owner.take_recovery_intent().is_none());
        let before: crate::SourcePolicySnapshot = source
            .policy_snapshot(remote)
            .map_err(Failure::SourcePolicy)?;
        let (packet, count, admitted): (&'static [u8], usize, bool) = match terminal {
            NakTerminal::ReceiveWork => (&CORRELATED_NTSN, 31, true),
            NakTerminal::InvalidThenEof => (&NTSN_WITH_COOKIE, 1, false),
            NakTerminal::ValidThenEof => (&CORRELATED_NTSN, 1, true),
        };
        let residue: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(usize::MAX));
        let captures: Arc<Mutex<Vec<SoftwareClock>>> =
            Arc::new(Mutex::new(Vec::with_capacity(count + 1)));
        let mut lifetime: Option<std::sync::Weak<()>> = None;
        let failure =
            exchange_successor_observing(owner, control, successor_entropy, |peer: SocketAddr| {
                assert_eq!(peer, remote);
                let sentinel: ScriptRead = match terminal {
                    NakTerminal::ReceiveWork => {
                        ScriptRead::Packet(fixtures::SUCCESSOR_OLD_S2C, peer)
                    }
                    NakTerminal::InvalidThenEof | NakTerminal::ValidThenEof => {
                        ScriptRead::Failed(io::ErrorKind::UnexpectedEof)
                    }
                };
                let reads: Vec<ScriptRead> = (0..count)
                    .map(|_| ScriptRead::Packet(packet, peer))
                    .chain([sentinel])
                    .collect();
                let mut script: Script = Script::new(ScriptSend::Full, reads);
                script.read_residue = Some(Arc::clone(&residue));
                script.read_captures = Some(Arc::clone(&captures));
                lifetime = Some(Arc::downgrade(&script.lifetime));
                Ok(backend(peer, script))
            })
            .err()
            .ok_or(Failure::Control)?;
        assert_eq!(lifetime.ok_or(Failure::Control)?.strong_count(), 0);
        assert!(tokio::runtime::Handle::try_current().is_err());
        let readings: MutexGuard<'_, Vec<SoftwareClock>> =
            captures.lock().map_err(|_| Failure::Control)?;
        let first_at: Instant = readings.first().ok_or(Failure::Control)?.monotonic();
        assert_eq!(failure.first_nak_at(), admitted.then_some(first_at));
        let phase: RequestPhase = match terminal {
            NakTerminal::ReceiveWork => {
                assert_eq!(failure.cause(), Failure::Ledger(LedgerError::ReceiveWork));
                assert_eq!(residue.load(Ordering::Relaxed), 1);
                assert_eq!(readings.len(), 31);
                let last_at: Instant = readings.last().ok_or(Failure::Control)?.monotonic();
                assert!(first_at < last_at);
                let recovery: RecoveryIntent =
                    owner.take_recovery_intent().ok_or(Failure::Control)?;
                assert_eq!(recovery.reason(), RecoveryReason::ReceiveWork);
                assert_eq!(recovery.not_before(), expiry);
                RequestPhase::Idle
            }
            NakTerminal::InvalidThenEof | NakTerminal::ValidThenEof => {
                assert_eq!(
                    failure.cause(),
                    Failure::Datagram(DatagramError::Receive(io::ErrorKind::UnexpectedEof))
                );
                assert_eq!(residue.load(Ordering::Relaxed), 0);
                assert_eq!(readings.len(), 2);
                RequestPhase::Pending
            }
        };
        assert_eq!(
            owner.status(),
            Status {
                phase,
                frames: 31 - count,
                bytes: 130_932 - count * packet.len(),
                attempts: 30,
                available: 0,
                used: 2,
                uids: 2
            }
        );
        assert!(owner.take_recovery_intent().is_none());
        assert!(owner.last_receipt() == Some(first_receipt));
        assert_eq!(owner.cookie_population()?, (0, 0));
        assert_eq!(control.expiry, expiry);
        drop(control.published_guard()?);
        let after: crate::SourcePolicySnapshot = source
            .policy_snapshot(remote)
            .map_err(Failure::SourcePolicy)?;
        assert_nak_only_send_changed_policy(&before, &after);
        Ok(())
    })
}

/// Keep the first admitted mark alongside the original exhausted-work failure.
/// # Errors
/// Preserve fixture, ledger and capture-observation refusals.
#[test]
fn managed_successor_nak_exhaustion_retains_first_mark_and_receive_work() -> Result<(), Failure> {
    assert_nak_terminal_history(&NakTerminal::ReceiveWork)
}

/// A forbidden NTSN Cookie must not mark the subsequent ordinary receive failure.
/// # Errors
/// Preserve fixture, ledger and capture-observation refusals.
#[test]
fn managed_successor_forbidden_nak_field_leaves_original_eof() -> Result<(), Failure> {
    assert_nak_terminal_history(&NakTerminal::InvalidThenEof)
}

/// Keep an admitted NTSN mark alongside the original backend EOF.
/// # Errors
/// Preserve fixture, ledger and capture-observation refusals.
#[test]
fn managed_successor_admitted_nak_retains_original_eof() -> Result<(), Failure> {
    assert_nak_terminal_history(&NakTerminal::ValidThenEof)
}

/// Replace the exact negotiated authorization after actual readiness.
/// # Errors
/// Preserve fixture and actual association publication refusal.
/// # Panics
/// Reject archive, send or floor debit after a same-byte revision replacement.
#[test]
fn managed_protected_rechecks_definition_after_readiness() -> Result<(), Failure> {
    use crate::source_policy::isolated_authority;
    use crate::{
        SourceAddressSet, SourceHandle, SourcePolicyError, SourcePolicyLimits, SourcePolicySnapshot,
    };

    let interval: Duration = Duration::from_millis(4);
    let limits: SourcePolicyLimits =
        SourcePolicyLimits::new(1, 1, interval, interval).map_err(Failure::SourcePolicy)?;
    let mut authority: SourcePolicyAuthority =
        isolated_authority(limits).map_err(Failure::SourcePolicy)?;
    let source: SourceHandle = authority.source(0).map_err(Failure::SourcePolicy)?;
    let remote: SocketAddr = "127.0.0.1:123".parse().map_err(|_| Failure::Endpoint)?;
    let peer: SocketAddr = "127.0.0.1:4460".parse().map_err(|_| Failure::Endpoint)?;
    let target: KePreviewTarget =
        KePreviewTarget::new(peer, "Time.Example").map_err(|_| Failure::Arguments)?;
    let original: SourceConfigurationSnapshot = authority
        .install_ke_preview_configuration(
            &source,
            SourceAddressSet::new(&[remote]).map_err(Failure::SourcePolicy)?,
            target.clone(),
        )
        .map_err(Failure::SourcePolicy)?;
    let config: Config = Config::from_managed_protected(
        authority
            .authorize_protected_ke(&original)
            .map_err(Failure::SourcePolicy)?,
    );
    with_config_owner(
        &config,
        Instant::now(),
        S2C,
        &[(0x11, 12)],
        |owner, control, negotiated| {
            let mut selected: Script = Script::new(
                ScriptSend::Full,
                vec![ScriptRead::Packet(fixtures::HEALTHY, negotiated)],
            );
            selected.ready.push_back(ScriptReady::Ready);
            selected.replace_before_guard = Some(GuardReplacement {
                authority,
                expected: original.clone(),
                target,
            });
            let mut backend: Backend = backend(negotiated, selected);
            let outcome: Result<OperationCompletion<'_>, Failure> =
                runtime()?.block_on(operation(owner, control, entropy, &mut backend));
            assert!(matches!(
                outcome,
                Err(Failure::SourcePolicy(
                    SourcePolicyError::ConfigurationChanged
                ))
            ));
            let observed: &Script = script(&backend)?;
            assert!(observed.ready.is_empty() && observed.replace_before_guard.is_none());
            assert_eq!((observed.send_calls, observed.receive_calls), (0, 0));
            assert_eq!(owner.status().phase, RequestPhase::Prepared);
            assert_eq!(owner.status().attempts, 31);
            assert_eq!(owner.status().used, 0);
            assert!(owner.last_receipt().is_none());
            let current: SourceConfigurationSnapshot = source
                .configuration_snapshot()
                .map_err(Failure::SourcePolicy)?
                .ok_or(Failure::Control)?;
            assert!(!current.same_configuration(&original));
            let retained: SourcePolicySnapshot = source
                .policy_snapshot(negotiated)
                .map_err(Failure::SourcePolicy)?;
            assert!(retained.source().floor().is_none() && retained.endpoint().floor().is_none());
            Ok(())
        },
    )
}

/// Local cancellation after readiness can win before the shared send guard.
/// # Errors
/// Preserve unexpected component setup failure.
/// # Panics
/// Fail if cancellation permits submission, archival or a cookie refund.
#[test]
fn cancellation_before_shared_send_guard_blocks_archive_and_submission() -> Result<(), Failure> {
    with_owner(S2C, |owner, control, remote| {
        let mut backend: Backend = backend(
            remote,
            Script::new(ScriptSend::CancelBeforeGuard, Vec::new()),
        );
        assert!(matches!(
            runtime()?.block_on(operation(owner, control, entropy, &mut backend)),
            Err(Failure::Cancelled)
        ));
        let status: Status = owner.status();
        assert_eq!((status.used, status.available), (0, 0));
        assert_eq!((status.attempts, status.uids), (31, 1));
        assert_eq!(script(&backend)?.send_calls, 0);
        assert_eq!(script(&backend)?.receive_calls, 0);
        assert!(backend.send.is_none());
        assert_eq!(status.phase, RequestPhase::Prepared);
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
                assert_eq!(completion.history.receipt.serial(), 1);
                assert_eq!(script(&backend)?.send_calls, 1);
                assert_eq!(script(&backend)?.receive_calls, 1);
                assert_eq!(owner.status().used, 1);
            }
            Ok(())
        })?;
    }
    Ok(())
}

/// KE-STOP-06: invocation Stop wins before the shared archive and send guard.
/// # Errors
/// Preserve checked setup and the healthy neighboring packet's real refusal.
/// # Panics
/// Fail if early Stop archives or sends, or refunds the spent request work.
#[test]
fn invocation_stop_before_shared_send_guard_preserves_spent_work() -> Result<(), Failure> {
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
                assert_eq!(owner.status().phase, RequestPhase::Prepared);
                assert_eq!(owner.status().available, 0);
                assert!(backend.send.is_none());
                assert!(owner.last_receipt().is_none());
            } else {
                let completion = result?;
                assert_eq!(completion.history.receipt.serial(), 1);
                assert_eq!(script(&backend)?.send_calls, 1);
                assert_eq!(script(&backend)?.receive_calls, 1);
            }
            let status: Status = owner.status();
            assert_eq!((status.attempts, status.uids), (31, 1));
            assert_eq!(status.used, usize::from(!stop_first));
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

/// Exercise exact consumed extent after either post-read Control refusal.
///
/// The fixture consumes and captures the selected datagram before changing
/// Control. The second ordinary datagram stays queued under either refusal.
/// # Errors
/// Preserve genuine fixture, policy, runtime or original Control setup failures.
/// # Panics
/// Reject changed debit, lost failure precedence, staging, or an extra read.
fn check_protected_consumed_receive_revocation(
    unknown_extent: bool,
    stop: bool,
) -> Result<(), Failure> {
    use crate::nts_ke_experiment::association::ledger::RecoveryReason;

    let earlier: Instant = Instant::now()
        .checked_sub(Duration::from_secs(2))
        .ok_or(Failure::Deadline)?;
    let (config, _source, _authority): (Config, crate::SourceHandle, SourcePolicyAuthority) =
        managed_attempt_config(earlier, Duration::from_millis(250), Duration::from_secs(2))?;
    let driver: tokio::runtime::Runtime = runtime()?;
    let start: Instant = Instant::now();
    let expiry: Instant = start
        .checked_add(Duration::from_secs(1))
        .ok_or(Failure::Deadline)?;
    let control: Control = Control::clamped(start, expiry)?;
    with_config_control(
        &config,
        &control,
        S2C,
        &[(0x11, 12)],
        |owner, control, remote| {
            owner.select_next_receipt_at_read_tail_for_test()?;
            let first: ScriptRead = if unknown_extent {
                ScriptRead::UnknownExtent
            } else {
                ScriptRead::Packet(fixtures::HEALTHY, remote)
            };
            let mut selected: Script = Script::new(
                ScriptSend::Full,
                vec![first, ScriptRead::Packet(fixtures::HEALTHY, remote)],
            );
            selected.revoke_after_read = Some((
                1,
                if stop {
                    ScriptReceiveRevocation::Stop {
                        handle: StopHandle {
                            scope: Arc::clone(&control.stop_scope),
                        },
                        expiry,
                        wait_for_expiry: false,
                    }
                } else {
                    ScriptReceiveRevocation::Expiry(expiry)
                },
            ));
            let mut backend: Backend = backend(remote, selected);
            let result: Result<OperationCompletion<'_>, Failure> =
                driver.block_on(operation(owner, control, entropy, &mut backend));
            let expected: Failure = if stop {
                Failure::Cancelled
            } else {
                Failure::Deadline
            };
            assert_eq!(result.err(), Some(expected));
            assert_eq!(control.expiry, expiry);
            let script: &Script = script(&backend)?;
            assert_eq!(
                (script.send_calls, script.receive_calls, script.reads.len()),
                (1, 1, 1)
            );
            let status: Status = owner.status();
            assert_eq!((status.attempts, status.used, status.uids), (31, 1, 1));
            let bytes: usize = if unknown_extent {
                0
            } else {
                131_072 - fixtures::HEALTHY.len()
            };
            assert_eq!(
                (status.frames, status.bytes),
                (31, bytes),
                "consumed protected receive must debit the original allowance after {expected:?}",
            );
            assert!(owner.has_receipt_tail_selection_for_test());
            assert!(owner.last_receipt().is_none());
            assert_eq!(owner.cookie_population()?, (0, 0));
            if unknown_extent {
                assert_eq!(status.phase, RequestPhase::Idle);
                let recovery = owner.take_recovery_intent().ok_or(Failure::Control)?;
                assert_eq!(recovery.reason(), RecoveryReason::ReceiveWork);
                assert_eq!(recovery.not_before(), expiry);
            }
            assert!(owner.take_recovery_intent().is_none());
            Ok(())
        },
    )
}

/// Protect exact byte and frame accounting when known data precedes Stop.
/// # Errors
/// Preserve unexpected fixture or Control refusal.
#[test]
fn protected_consumed_known_debits_after_stop() -> Result<(), Failure> {
    check_protected_consumed_receive_revocation(false, true)
}

/// Protect exact byte and frame accounting when known data precedes expiry.
/// # Errors
/// Preserve unexpected fixture or Control refusal.
#[test]
fn protected_consumed_known_debits_after_expiry() -> Result<(), Failure> {
    check_protected_consumed_receive_revocation(false, false)
}

/// Conservatively exhaust unknown bytes when consumption precedes Stop.
/// # Errors
/// Preserve unexpected fixture or Control refusal.
#[test]
fn protected_consumed_unknown_retires_after_stop() -> Result<(), Failure> {
    check_protected_consumed_receive_revocation(true, true)
}

/// Conservatively exhaust unknown bytes when consumption precedes expiry.
/// # Errors
/// Preserve unexpected fixture or Control refusal.
#[test]
fn protected_consumed_unknown_retires_after_expiry() -> Result<(), Failure> {
    check_protected_consumed_receive_revocation(true, false)
}

/// Which terminal observations are visible after capturing the final known frame.
enum AfterFinalRead {
    Stop,
    Expiry,
    StopThenExpiry,
}

/// Exercise the final receive credit with the original finite cutoff.
/// # Errors
/// Preserve unexpected fixture construction or owner failures.
/// # Panics
/// Fail if the final frame is refunded, published, or leaves exhausted custody pending.
fn check_final_known_frame_retirement(after: &AfterFinalRead) -> Result<(), Failure> {
    use crate::nts_ke_experiment::association::ledger::{RecoveryIntent, RecoveryReason};

    let earlier: Instant = Instant::now()
        .checked_sub(Duration::from_secs(2))
        .ok_or(Failure::Deadline)?;
    let (config, _source, _authority): (Config, crate::SourceHandle, SourcePolicyAuthority) =
        managed_attempt_config(earlier, Duration::from_millis(250), Duration::from_secs(2))?;
    let driver: tokio::runtime::Runtime = runtime()?;
    let start: Instant = Instant::now();
    let expiry: Instant = start
        .checked_add(Duration::from_secs(1))
        .ok_or(Failure::Deadline)?;
    let control: Control = Control::clamped(start, expiry)?;
    with_config_control(
        &config,
        &control,
        S2C,
        &[(0x11, 12)],
        |owner, control, remote| {
            let (event, expected): (ScriptReceiveRevocation, Failure) = match after {
                AfterFinalRead::Expiry => {
                    (ScriptReceiveRevocation::Expiry(expiry), Failure::Deadline)
                }
                AfterFinalRead::Stop | AfterFinalRead::StopThenExpiry => (
                    ScriptReceiveRevocation::Stop {
                        handle: StopHandle {
                            scope: Arc::clone(&control.stop_scope),
                        },
                        expiry,
                        wait_for_expiry: matches!(after, AfterFinalRead::StopThenExpiry),
                    },
                    Failure::Cancelled,
                ),
            };
            let mut reads: Vec<ScriptRead> = (0..31)
                .map(|_| ScriptRead::Packet(fixtures::ZERO_COOKIES, remote))
                .collect();
            reads.extend([
                ScriptRead::Packet(fixtures::HEALTHY, remote),
                ScriptRead::Packet(fixtures::HEALTHY, remote),
            ]);
            let mut selected: Script = Script::new(ScriptSend::Full, reads);
            selected.revoke_after_read = Some((32, event));
            let mut backend: Backend = backend(remote, selected);
            let result: Result<OperationCompletion<'_>, Failure> =
                driver.block_on(operation(owner, control, entropy, &mut backend));

            assert_eq!(result.err(), Some(expected));
            let script: &Script = script(&backend)?;
            assert_eq!(
                (script.send_calls, script.receive_calls, script.reads.len()),
                (1, 32, 1)
            );
            assert_eq!(
                owner.status(),
                Status {
                    phase: RequestPhase::Idle,
                    attempts: 31,
                    frames: 0,
                    bytes: 131_072 - 31 * fixtures::ZERO_COOKIES.len() - fixtures::HEALTHY.len(),
                    available: 0,
                    used: 1,
                    uids: 1,
                },
                "exhausted final known frame must retire after {expected:?}",
            );
            assert!(owner.last_receipt().is_none());
            assert_eq!(owner.cookie_population()?, (0, 0));
            assert_eq!(control.expiry, expiry);
            let recovery: RecoveryIntent = owner.take_recovery_intent().ok_or(Failure::Control)?;
            assert_eq!(recovery.reason(), RecoveryReason::ReceiveWork);
            assert_eq!(recovery.not_before(), expiry);
            assert!(owner.take_recovery_intent().is_none());
            Ok(())
        },
    )
}

/// Spend and retire the final known frame when Stop follows capture.
/// # Errors
/// Preserve unexpected fixture or owner failures.
#[test]
fn protected_final_known_frame_retires_after_stop() -> Result<(), Failure> {
    check_final_known_frame_retirement(&AfterFinalRead::Stop)
}

/// Spend and retire the final known frame when the original cutoff passes after capture.
/// # Errors
/// Preserve unexpected fixture or owner failures.
#[test]
fn protected_final_known_frame_retires_after_expiry() -> Result<(), Failure> {
    check_final_known_frame_retirement(&AfterFinalRead::Expiry)
}

/// Preserve a latched Stop when the cutoff also passes after the final capture.
/// # Errors
/// Preserve unexpected fixture or owner failures.
#[test]
fn protected_final_known_frame_retires_after_stop_and_expiry() -> Result<(), Failure> {
    check_final_known_frame_retirement(&AfterFinalRead::StopThenExpiry)
}

/// A genuine Stop during read readiness preserves both receive budgets.
/// # Errors
/// Preserve unexpected fixture or Control refusal.
/// # Panics
/// Reject consumption, staging or lost original Stop.
#[test]
fn protected_stop_before_receive_preserves_datagram_credit() -> Result<(), Failure> {
    with_owner(S2C, |owner, control, remote| {
        owner.select_next_receipt_at_read_tail_for_test()?;
        let expiry: Instant = control.expiry;
        let mut selected: Script = Script::new(
            ScriptSend::Full,
            vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
        );
        selected
            .ready
            .extend([ScriptReady::Ready, ScriptReady::ScopeStop]);
        let mut backend: Backend = backend(remote, selected);
        let result: Result<OperationCompletion<'_>, Failure> =
            runtime()?.block_on(operation(owner, control, entropy, &mut backend));
        assert_eq!(result.err(), Some(Failure::Cancelled));
        let script: &Script = script(&backend)?;
        assert_eq!(
            (script.send_calls, script.receive_calls, script.reads.len()),
            (1, 0, 1)
        );
        let status: Status = owner.status();
        assert_eq!((status.frames, status.bytes), (32, 131_072));
        assert_eq!(control.expiry, expiry);
        assert!(owner.has_receipt_tail_selection_for_test());
        assert!(owner.last_receipt().is_none());
        Ok(())
    })
}

/// A rejected origin spends bytes and leaves the matching origin usable once.
/// # Errors
/// Preserve unexpected fixture, runtime or healthy reply refusal.
/// # Panics
/// Reject uncharged rejected data, altered original deadline or spurious acceptance.
#[test]
fn protected_known_live_reply_charges_rejected_and_matching_origins() -> Result<(), Failure> {
    with_owner(S2C, |owner, control, remote| {
        let expiry: Instant = control.expiry;
        let mut backend: Backend = backend(
            remote,
            Script::new(
                ScriptSend::Full,
                vec![
                    ScriptRead::Packet(fixtures::WRONG_ORIGIN, remote),
                    ScriptRead::Packet(fixtures::HEALTHY, remote),
                ],
            ),
        );
        let result: OperationCompletion<'_> =
            runtime()?.block_on(operation(owner, control, entropy, &mut backend))?;
        assert_eq!(result.history.receipt.fields().origin().bits(), WIRE);
        assert_eq!(owner.last_receipt().map(Receipt::serial), Some(1));
        let script: &Script = script(&backend)?;
        assert_eq!(
            (script.send_calls, script.receive_calls, script.reads.len()),
            (1, 2, 0)
        );
        let status: Status = owner.status();
        assert_eq!(
            (status.frames, status.bytes),
            (
                30,
                131_072 - fixtures::WRONG_ORIGIN.len() - fixtures::HEALTHY.len()
            ),
        );
        assert_eq!(control.expiry, expiry);
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
        assert_eq!(completion.history.receipt.serial(), 1);
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

/// The complete buffer sentinel and the Windows oversize return expose no packet.
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
    assert!(matches!(
        classify_read(Err(io::Error::from_raw_os_error(10040))),
        Read::ConsumedUnknown
    ));
    assert!(matches!(
        classify_read(Err(io::ErrorKind::PermissionDenied.into())),
        Read::Failed(io::ErrorKind::PermissionDenied)
    ));
    assert!(matches!(
        classify_read(Err(io::Error::from_raw_os_error(90))),
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

/// A foreign request or Control cannot reach the private pre-guard Stop fixture.
/// # Errors
/// Preserve either association's checked construction or request preparation refusal.
/// # Panics
/// Fail if a rejected attempt stops either owner or changes request or socket custody.
#[test]
fn concrete_sender_refuses_foreign_owner_before_fixture_stop() -> Result<(), Failure> {
    for (foreign_control, stop) in [
        (true, ScriptSend::CancelBeforeGuard),
        (false, ScriptSend::ScopeStopBeforeGuard),
    ] {
        with_owner(S2C, |owner, control, remote| {
            let mut backend: Backend = backend(remote, Script::new(stop, Vec::new()));
            let ticket: Ticket<'_> = owner.reserve(RequestInput::Observed)?;
            owner.prepare_observed(&ticket, entropy)?;
            with_owner(S2C, |other, foreign, _| {
                let foreign_ticket: Ticket<'_> = other.reserve(RequestInput::Observed)?;
                other.prepare_observed(&foreign_ticket, entropy)?;
                let actual_state: Status = owner.status();
                let foreign_state: Status = other.status();
                if foreign_control {
                    assert_eq!(
                        backend.send_prepared(owner, ticket, foreign).err(),
                        Some(Failure::Control)
                    );
                } else {
                    assert_eq!(
                        backend.send_prepared(owner, foreign_ticket, control).err(),
                        Some(Failure::Ledger(LedgerError::Ticket))
                    );
                }
                drop(control.published_guard()?);
                drop(foreign.published_guard()?);
                assert_eq!(owner.status(), actual_state);
                assert_eq!(other.status(), foreign_state);
                assert!(owner.last_receipt().is_none());
                assert!(other.last_receipt().is_none());
                Ok(())
            })?;
            assert_eq!(script(&backend)?.send_calls, 0);
            assert_eq!(script(&backend)?.receive_calls, 0);
            assert!(backend.send.is_none());
            Ok(())
        })?;
    }
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
        let wrong: SocketAddr = SocketAddr::new(remote.ip(), 124);
        let mut backend: Backend = backend(wrong, Script::new(ScriptSend::Full, Vec::new()));
        let ticket: Ticket<'_> = owner.reserve(RequestInput::Observed)?;
        owner.prepare_observed(&ticket, entropy)?;
        assert_eq!(
            backend.send_prepared(owner, ticket, control).err(),
            Some(Failure::Endpoint)
        );
        assert_eq!(script(&backend)?.send_calls, 0);
        assert_eq!(owner.status().used, 1);
        assert_eq!(owner.status().phase, RequestPhase::Invoked);
        assert!(owner.last_receipt().is_none());
        Ok(())
    })?;
    with_owner_cookies(
        Instant::now(),
        S2C,
        &[(0x11, 12), (0x22, 16)],
        |owner, control, remote| {
            let mut backend: Backend = backend(
                remote,
                Script::new(
                    ScriptSend::Full,
                    vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
                ),
            );
            let driver: tokio::runtime::Runtime = runtime()?;
            let completion: OperationCompletion<'_> =
                driver.block_on(operation(owner, control, entropy, &mut backend))?;
            assert_eq!(owner.cookie_population()?, (1, 16));
            let captured: &Script = script(&backend)?;
            assert_eq!(&captured.packet[40..48], &WIRE.to_be_bytes());
            let ticket: Ticket<'_> = owner.reserve(RequestInput::Observed)?;
            owner.prepare_observed(&ticket, |output: &mut [u8; 56]| {
                entropy(output)?;
                output[8..].fill(0x77);
                Ok(())
            })?;
            assert_eq!(
                backend.send_prepared(owner, ticket, control).err(),
                Some(Failure::Endpoint)
            );
            assert_eq!(script(&backend)?.send_calls, 1);
            let status: Status = owner.status();
            assert_eq!((status.used, status.available), (2, 0));
            assert_eq!(status.phase, RequestPhase::Invoked);
            assert_eq!(owner.last_receipt().map(Receipt::serial), Some(1));
            // Raw operation history grants no final commit, even after manual cleanup.
            drop(completion);
            drop(backend);
            drop(driver);
            assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
            Ok(())
        },
    )
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
            let admitted: Receipt = completion.history().receipt;
            assert!(owner.last_receipt() == Some(admitted));
            assert_eq!(owner.cookie_population()?, (0, 0));
            let before = owner.status();
            if stop_first {
                assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
                assert!(matches!(
                    protected_once(owner, |_| Ok(completion)),
                    Err(Failure::Cancelled)
                ));
                assert_eq!(owner.status(), before);
                assert_eq!(owner.cookie_population()?, (0, 0));
            } else {
                let committed = completion.commit(owner)?;
                assert_eq!(control.cancel()?, CancelOutcome::AlreadyCommitted);
                assert!(committed.into_summary().transmitted);
                assert_eq!(owner.cookie_population()?, (1, 12));
            }
            assert!(owner.last_receipt() == Some(admitted));
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
            let receipt: Receipt = completion.history().receipt;
            assert!(owner.last_receipt() == Some(receipt));
            assert_eq!(receipt.serial(), 1);
            assert_eq!(owner.cookie_population()?, (0, 0));
            if stop_first {
                assert_eq!(stop.request_stop()?, StopDisposition::Latched);
                assert!(matches!(
                    protected_once(owner, |_| Ok(completion)),
                    Err(Failure::Cancelled)
                ));
                assert_eq!(owner.status(), before);
                assert_eq!(owner.cookie_population()?, (0, 0));
            } else {
                let committed = completion.commit(owner)?;
                assert_eq!(stop.request_stop()?, StopDisposition::Latched);
                assert_eq!(control.cancel()?, CancelOutcome::AlreadyCommitted);
                let summary = committed.into_summary();
                assert!(summary.transmitted);
                assert_eq!(summary.cookies, 1);
                assert_eq!(summary.cookie_bytes, 12);
                assert_eq!(
                    owner.status(),
                    Status {
                        available: 1,
                        ..before
                    }
                );
                assert_eq!(owner.cookie_population()?, (1, 12));
            }
            assert_eq!(stop.request_stop()?, StopDisposition::AlreadyLatched);
            assert!(owner.last_receipt() == Some(receipt));
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
        assert_eq!(owner.cookie_population()?, (0, 0));
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
        assert_eq!(owner.cookie_population()?, (0, 0));
        let after = owner.last_receipt().ok_or(Failure::Control)?;
        assert_eq!(after.serial(), receipt.serial());
        assert_eq!(after.accepted_at(), receipt.accepted_at());
        assert_eq!(after.cookie_counts(), receipt.cookie_counts());
        assert_eq!(control.expiry, expiry);
        assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
        Ok(())
    })
}

/// Pass one exact fixture reading to the real retired commit and historical receiver.
/// # Errors
/// Preserve the actual guarded ledger refusal.
pub(in crate::nts_ke_experiment) fn commit_at_fixture_reading<'control>(
    completion: super::ProtectedCompletion<'control>,
    association: &mut AssociationOwner<'_>,
    sample: Instant,
) -> Result<super::CommittedProtected<'control>, Failure> {
    completion.commit_observing(association, || sample)
}

/// Exercise the real retired commit at its inclusive acceptance and exclusive expiry.
/// # Errors
/// Preserve checked owner construction, fixture admission and state observation.
/// # Panics
/// Fail if a refused reading publishes cookies or rewrites authenticated history.
#[test]
fn retired_completion_commit_respects_acceptance_and_expiry() -> Result<(), Failure> {
    /// Change only the final reading after genuine backend retirement.
    #[derive(Debug)]
    enum Reading {
        /// The exact recorded packet acceptance is admissible.
        AtAcceptance,
        /// The original deadline is exclusive.
        AtExpiry,
        /// One nanosecond before packet acceptance must refuse.
        BeforeAcceptance,
    }

    for reading in [
        Reading::AtAcceptance,
        Reading::AtExpiry,
        Reading::BeforeAcceptance,
    ] {
        let start: Instant = Instant::now()
            .checked_sub(Duration::from_secs(1))
            .ok_or(Failure::Control)?;
        with_owner_start(start, S2C, |owner, control, remote| {
            let mut backend_lifetime: Option<std::sync::Weak<()>> = None;
            let completion: super::ProtectedCompletion<'_> =
                exchange_with(owner, control, entropy, |peer| {
                    assert_eq!(peer, remote);
                    let script: Script = Script::new(
                        ScriptSend::Full,
                        vec![ScriptRead::Packet(fixtures::HEALTHY, peer)],
                    );
                    backend_lifetime = Some(Arc::downgrade(&script.lifetime));
                    Ok(backend(peer, script))
                })?;
            assert_eq!(backend_lifetime.ok_or(Failure::Control)?.strong_count(), 0);
            let receipt: Receipt = completion.history().receipt;
            let accepted_at: Instant = receipt.accepted_at();
            let before_acceptance: Instant = accepted_at
                .checked_sub(Duration::from_nanos(1))
                .ok_or(Failure::Control)?;
            let expiry: Instant = control.expiry;
            assert!(start < before_acceptance && before_acceptance < accepted_at);
            assert!(accepted_at < expiry);
            assert_eq!(receipt.serial(), 1);
            assert!(owner.last_receipt() == Some(receipt));
            assert_eq!(owner.cookie_population()?, (0, 0));
            let before: Status = owner.status();
            assert_eq!(before.phase, RequestPhase::Idle);

            let (sample, expected): (Instant, Result<(), Failure>) = match reading {
                Reading::AtAcceptance => (accepted_at, Ok(())),
                Reading::AtExpiry => (expiry, Err(Failure::Deadline)),
                Reading::BeforeAcceptance => {
                    (before_acceptance, Err(Failure::Ledger(LedgerError::Time)))
                }
            };
            let result: Result<(), Failure> =
                commit_at_fixture_reading(completion, owner, sample).map(drop);
            let (available, expected_pool, cancellation): (usize, (usize, usize), CancelOutcome) =
                if expected.is_ok() {
                    (1, (1, 12), CancelOutcome::AlreadyCommitted)
                } else {
                    (0, (0, 0), CancelOutcome::Cancelled)
                };
            assert_eq!(
                (result, owner.cookie_population()?),
                (expected, expected_pool),
                "retired commit at {reading:?} changed the decision or live cookie pool"
            );
            assert_eq!(
                owner.status(),
                Status {
                    available,
                    ..before
                }
            );
            assert!(owner.last_receipt() == Some(receipt));
            assert_eq!(control.expiry, expiry);
            assert_eq!(control.cancel()?, cancellation);
            Ok(())
        })?;
    }
    Ok(())
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
        assert_eq!(original.cookie_population()?, (0, 0));
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
            assert_eq!(other.cookie_population()?, (0, 0));
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
            assert_eq!(original.cookie_population()?, (0, 0));
            assert_eq!(other.cookie_population()?, (0, 0));
            assert_eq!(original.last_receipt().map(Receipt::serial), Some(serial));
            assert_eq!(other.last_receipt().map(Receipt::serial), Some(serial));
            original_control.remaining(IoPhase::Exchange)?;
            other_control.remaining(IoPhase::Exchange)?;
            // A genuine completion still commits at its own receiving owner.
            assert!(other_completion.commit(other)?.into_summary().transmitted);
            assert_eq!(other.cookie_population()?, (1, 12));
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
        assert_eq!(owner.cookie_population()?, (0, 0));
        drop(completion);
        assert_eq!(owner.status(), before);
        assert_eq!(owner.cookie_population()?, (0, 0));
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
            assert_eq!(receiving.cookie_population()?, (0, 0));
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

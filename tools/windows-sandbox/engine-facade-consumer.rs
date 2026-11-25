// SPDX-License-Identifier: GPL-2.0-only
//! Fixed Windows production-facade consumer for refusal and loopback observations.
//!
//! Link the normal Engine and Tokio artifacts from the admitted Root graph.
//! The supervisor admits this v3 image and owns its process/output custody.
//! Exactly one closed mode selects an Engine operation. The managed successor
//! modes obtain canonical IPv4 loopback endpoints from the private harness
//! environment and install them with a fixed TLS identity. The pair runs two
//! separately configured acquisitions through one owner and one cutoff.
//! Missing arguments reach admission refusal. Valid fixed loopback arguments
//! under an entered current-thread runtime reach caller-context refusal before
//! Engine preparation. The EOF mode owns one loopback listener and presents no
//! certificate or application protocol. It keeps the Engine exit and diagnostic
//! and separately requires the peer to observe `ClientHello` and join.

// Match the standalone launcher and CPU diagnostic's deny policy.
#![deny(
    warnings,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_unsafe,
    unused_must_use,
    let_underscore_drop,
    unused_macro_rules,
    unused_lifetimes,
    unfulfilled_lint_expectations,
    unknown_lints,
    renamed_and_removed_lints,
    improper_ctypes,
    improper_ctypes_definitions,
    unexpected_cfgs,
    missing_docs,
    unreachable_pub,
    unused_qualifications,
    clippy::all,
    clippy::pedantic,
    clippy::missing_docs_in_private_items,
    clippy::undocumented_unsafe_blocks,
    clippy::allow_attributes_without_reason,
    clippy::allow_attributes,
    clippy::multiple_unsafe_ops_per_block,
    clippy::unused_result_ok,
    clippy::string_slice,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::todo,
    clippy::unimplemented,
    clippy::unreachable,
    clippy::dbg_macro,
    clippy::let_underscore_must_use,
    clippy::as_conversions,
    clippy::mem_forget,
    clippy::exit,
    rustdoc::all,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links
)]

#[cfg(not(all(windows, target_arch = "x86_64")))]
compile_error!("This production-facade consumer is exclusively for x64 Windows.");

use std::{
    env,
    ffi::{OsStr, OsString},
    io::{self, Read, Write},
    net::{Ipv4Addr, Shutdown, SocketAddr, SocketAddrV4, TcpListener, TcpStream, UdpSocket},
    num::NonZeroU16,
    process::ExitCode,
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use nevrotick_engine::{
    CertificateTimeProvenance, KePreviewTarget, ManagedProtectedPairReport, ManagedProtectedPlan,
    ManagedProtectedReceipt, ManagedProtectedReport, ManagedProtectedRestriction,
    ManagedProtectedTerminal, ProtectedNativeCaptureSpan, ProtectedNativeClockObservation,
    SoftwareClock, SourceAddressSet, SourceConfigurationSnapshot, SourceHandle,
    SourcePolicyAuthority, SourcePolicyError, SourcePolicyLimits, SourcePolicySnapshot,
    SourceProtectedProbeFailure,
};

/// Fixed program atom consumed by the Engine's actual CLI entrypoint.
const PROGRAM: &str = "engine-facade-consumer";

/// Maximum TLS record payload accepted from the actual client.
const PEER_BYTES: usize = 4096;
/// One cumulative deadline covers accept, `ClientHello` and peer shutdown.
const PEER_WINDOW: Duration = Duration::from_secs(8);
/// Keep an idle nonblocking socket within the same finite peer window.
const PEER_POLL: Duration = Duration::from_millis(2);

/// The private harness supplies only the owned numeric TCP endpoint here.
const SUCCESSOR_KE_PEER: &str = "NV_NTS_SUCCESSOR_KE_PEER";
/// The private harness supplies only the owned numeric UDP endpoint here.
const SUCCESSOR_UDP_PEER: &str = "NV_NTS_SUCCESSOR_UDP_PEER";
/// Separate owned TCP endpoint for the second job in the pair.
const PAIR_SECOND_KE_PEER: &str = "NV_NTS_PAIR_SECOND_KE_PEER";
/// Separate owned UDP endpoint for the second job in the pair.
const PAIR_SECOND_UDP_PEER: &str = "NV_NTS_PAIR_SECOND_UDP_PEER";
/// One fixture deadline for both jobs, within the standalone peers' lifetime.
const PAIR_WINDOW: Duration = Duration::from_secs(45);
/// The identity verified by the Engine against the owned KE peer's certificate.
const SUCCESSOR_TLS_IDENTITY: &str = "ke-a7e40ec4eade477dbaba6ca9e9f975a4.invalid";
/// The complete fixed address and delimiter preceding a canonical port.
const SUCCESSOR_PEER_PREFIX: &str = "127.0.0.1:";
/// Bound the original environment bytes by the prefix and five port digits.
const SUCCESSOR_PEER_BYTES: usize = SUCCESSOR_PEER_PREFIX.len() + 5;

/// Closed setup failures, distinct from Engine refusal exits one and two.
enum SetupFailure {
    /// Missing, unknown or extra consumer mode; no Engine call is permitted.
    Arguments,
    /// The current-thread runtime could not be built; no Engine call occurred.
    Runtime,
    /// The owned listener or peer thread could not start; no Engine call occurred.
    PeerStart,
    /// The joined peer refused the required accept, read, half-close or drain.
    Peer(PeerFailure),
    /// The peer thread terminated abnormally.
    PeerPanic,
    /// The joined peer's bounded witness could not be written.
    PeerWitness,
    /// The real managed source definition could not be installed or replaced.
    ManagedPolicy,
    /// An obsolete definition did not refuse before spending the invocation.
    ManagedStale,
    /// The original elapsed cutoff did not refuse as preparation.
    ManagedDeadline,
    /// The actual managed failure or independent cleanup differed from EOF.
    ManagedOutcome,
    /// A protected or fallback UDP datagram reached the owned endpoint.
    ManagedDatagram,
    /// A required private successor endpoint did not pass admission.
    ManagedSuccessorPeers,
    /// Successor history did not retain the original revision or chronological order.
    ManagedSuccessorHistory,
    /// The timing witness did not receive an intact ordinary two-leg exchange.
    ManagedSuccessorTiming,
    /// The pair did not return two joined ordinary outcomes with original work.
    ManagedPairOutcome,
    /// The receipts or their adjacent application captures are inconsistent.
    ManagedPairOrder,
    /// The pair's single observer retirement returned a native error.
    ManagedPairClose,
}

/// Closed peer refusal facts. Neither socket addresses nor bytes enter diagnostics.
enum PeerFailure {
    /// The expected loopback stream was not admitted.
    Accept,
    /// Its TLS record did not contain one complete `ClientHello`.
    ClientHello,
    /// The peer could not signal ordinary EOF to the Engine.
    HalfClose,
    /// The Engine stream did not close within the bounded read.
    Drain,
}

impl SetupFailure {
    /// Attempt one bounded fixed diagnostic and preserve its distinct setup exit.
    /// Diagnostic failure never replaces the original setup classification.
    fn report(self) -> ExitCode {
        let (stage, code) = match self {
            Self::Arguments => ("arguments", 126),
            Self::Runtime => ("runtime", 125),
            Self::PeerStart => ("peer-start", 124),
            Self::Peer(PeerFailure::Accept) => ("peer-accept", 124),
            Self::Peer(PeerFailure::ClientHello) => ("peer-clienthello", 124),
            Self::Peer(PeerFailure::HalfClose) => ("peer-half-close", 124),
            Self::Peer(PeerFailure::Drain) => ("peer-drain", 124),
            Self::PeerPanic => ("peer-panic", 124),
            Self::PeerWitness => ("peer-witness", 124),
            Self::ManagedPolicy => ("managed-policy", 124),
            Self::ManagedStale => ("managed-stale", 124),
            Self::ManagedDeadline => ("managed-deadline", 124),
            Self::ManagedOutcome => ("managed-outcome", 124),
            Self::ManagedDatagram => ("managed-datagram", 124),
            Self::ManagedSuccessorPeers => ("managed-successor-peers", 124),
            Self::ManagedSuccessorHistory => ("managed-successor-history", 124),
            Self::ManagedSuccessorTiming => ("managed-successor-timing", 124),
            Self::ManagedPairOutcome => ("managed-pair-outcome", 124),
            Self::ManagedPairOrder => ("managed-pair-order", 124),
            Self::ManagedPairClose => ("managed-pair-close", 124),
        };
        let _diagnostic_result = writeln!(io::stderr().lock(), "facade-consumer-stage={stage}");
        ExitCode::from(code)
    }
}

/// Closed calls to the normal production facade with mode-specific fixture custody.
enum Mode {
    /// Supply only a program atom, leaving required Engine arguments absent.
    Arguments,
    /// Supply valid fixed arguments while a current-thread runtime is entered.
    EnteredRuntime,
    /// Observe one real `ClientHello`, then signal EOF without providing TLS material.
    LoopbackEof,
    /// Refuse a stale managed definition, then observe one current managed EOF.
    ManagedProtectedEof,
    /// Observe one explicit managed protected invocation with an earned successor.
    ManagedProtectedSuccessor(SuccessorWitness),
    /// Observe two managed protected invocations in the same acquisition owner.
    ManagedProtectedPair,
}

/// Select the output of the explicitly invoked protected successor fixture.
#[derive(Clone, Copy)]
enum SuccessorWitness {
    /// Write the ordinary joined summary.
    Summary,
    /// Also export the original native coordinates of a healthy two-leg exchange.
    Timing,
}

impl Mode {
    /// Read at most two consumer atoms and admit exactly one closed mode.
    /// No input text is decoded, logged or forwarded to the Engine.
    /// # Errors
    /// Refuse missing, unknown and extra modes before runtime or Engine work.
    fn from_arguments() -> Result<Self, SetupFailure> {
        let mut arguments = env::args_os().skip(1);
        let mode = match arguments.next().as_deref() {
            Some(value) if value == OsStr::new("arguments") => Self::Arguments,
            Some(value) if value == OsStr::new("entered-runtime") => Self::EnteredRuntime,
            Some(value) if value == OsStr::new("loopback-eof") => Self::LoopbackEof,
            Some(value) if value == OsStr::new("managed-protected-eof") => {
                Self::ManagedProtectedEof
            }
            Some(value) if value == OsStr::new("managed-protected-successor") => {
                Self::ManagedProtectedSuccessor(SuccessorWitness::Summary)
            }
            Some(value) if value == OsStr::new("managed-protected-successor-timing") => {
                Self::ManagedProtectedSuccessor(SuccessorWitness::Timing)
            }
            Some(value) if value == OsStr::new("managed-protected-pair") => {
                Self::ManagedProtectedPair
            }
            _ => return Err(SetupFailure::Arguments),
        };
        if arguments.next().is_some() {
            return Err(SetupFailure::Arguments);
        }
        Ok(mode)
    }
}

/// Fill the caller's fixed buffer or report how many bytes preceded peer EOF.
/// # Errors
/// Refuse timeout, other socket failures and a count beyond the supplied buffer.
fn read_peer(stream: &mut TcpStream, buffer: &mut [u8], deadline: Instant) -> io::Result<usize> {
    let capacity: usize = buffer.len();
    let mut remaining: &mut [u8] = buffer;
    while !remaining.is_empty() {
        if Instant::now() >= deadline {
            return Err(io::ErrorKind::TimedOut.into());
        }
        match stream.read(remaining) {
            Ok(0) => break,
            Ok(length) => {
                remaining = remaining
                    .split_at_mut_checked(length)
                    .ok_or(io::ErrorKind::InvalidData)?
                    .1;
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => thread::sleep(PEER_POLL),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(error),
        }
    }
    Ok(capacity - remaining.len())
}

/// Admit one loopback client and retire the listener before consuming its bytes.
/// # Errors
/// Refuse timeout, unexpected peer, or a socket that cannot be made nonblocking.
fn accept_peer(listener: TcpListener, deadline: Instant) -> Result<TcpStream, PeerFailure> {
    while Instant::now() < deadline {
        match listener.accept() {
            Ok((stream, SocketAddr::V4(peer))) if peer.ip().is_loopback() => {
                drop(listener);
                stream
                    .set_nonblocking(true)
                    .map_err(|_| PeerFailure::Accept)?;
                return Ok(stream);
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => thread::sleep(PEER_POLL),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            _ => return Err(PeerFailure::Accept),
        }
    }
    Err(PeerFailure::Accept)
}

/// Confirm a complete first `ClientHello` record, then send EOF and observe close.
/// # Errors
/// Refuse wrong or short framing, half-close refusal, and unconfirmed peer EOF.
fn observe_peer(listener: TcpListener, deadline: Instant) -> Result<(), PeerFailure> {
    let mut stream: TcpStream = accept_peer(listener, deadline)?;
    let mut header: [u8; 5] = [0; 5];
    if read_peer(&mut stream, &mut header, deadline).map_err(|_| PeerFailure::ClientHello)?
        != header.len()
    {
        return Err(PeerFailure::ClientHello);
    }
    let [22, 3, 1..=3, high, low]: [u8; 5] = header else {
        return Err(PeerFailure::ClientHello);
    };
    let length: usize = usize::from(u16::from_be_bytes([high, low]));
    let mut storage: [u8; PEER_BYTES] = [0; PEER_BYTES];
    let payload: &mut [u8] = storage.get_mut(..length).ok_or(PeerFailure::ClientHello)?;
    if read_peer(&mut stream, payload, deadline).map_err(|_| PeerFailure::ClientHello)? != length {
        return Err(PeerFailure::ClientHello);
    }
    let [1, high, middle, low, hello @ ..] = payload else {
        return Err(PeerFailure::ClientHello);
    };
    let body_length: usize =
        (usize::from(*high) << 16) | (usize::from(*middle) << 8) | usize::from(*low);
    // Two legacy version bytes and 32 random bytes are always present.
    if body_length != hello.len() || body_length < 34 {
        return Err(PeerFailure::ClientHello);
    }
    if Instant::now() >= deadline {
        return Err(PeerFailure::HalfClose);
    }
    stream
        .shutdown(Shutdown::Write)
        .map_err(|_| PeerFailure::HalfClose)?;
    if read_peer(&mut stream, &mut storage, deadline).map_err(|_| PeerFailure::Drain)?
        == storage.len()
    {
        return Err(PeerFailure::Drain);
    }
    Ok(())
}

/// Call the normal never-send facade with the owned peer's actual bound address.
/// The peer thread joins even if the Engine exits before attempting transport.
/// # Errors
/// Distinguish fixture setup or completion refusal from the actual Engine exit.
fn loopback_eof() -> Result<ExitCode, SetupFailure> {
    let listener: TcpListener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
        .map_err(|_| SetupFailure::PeerStart)?;
    listener
        .set_nonblocking(true)
        .map_err(|_| SetupFailure::PeerStart)?;
    let address: SocketAddr = listener.local_addr().map_err(|_| SetupFailure::PeerStart)?;
    let deadline: Instant = Instant::now()
        .checked_add(PEER_WINDOW)
        .ok_or(SetupFailure::PeerStart)?;
    let peer: thread::JoinHandle<Result<(), PeerFailure>> = thread::Builder::new()
        .spawn(move || observe_peer(listener, deadline))
        .map_err(|_| SetupFailure::PeerStart)?;
    let code: ExitCode = nevrotick_engine::experimental_nts_ke_probe([
        OsString::from(PROGRAM),
        OsString::from(address.to_string()),
        OsString::from("localhost"),
    ]);
    peer.join()
        .map_err(|_| SetupFailure::PeerPanic)?
        .map_err(SetupFailure::Peer)?;
    writeln!(io::stderr().lock(), "facade-peer=clienthello-eof-joined")
        .map_err(|_| SetupFailure::PeerWitness)?;
    Ok(code)
}

/// Require the real managed failure without discarding independent cleanup facts.
/// # Errors
/// Refuse success, another primary cause, retained history or failed observer close.
fn managed_eof_failure(
    result: Result<ManagedProtectedReport, SourceProtectedProbeFailure>,
) -> Result<&'static str, SetupFailure> {
    let report: ManagedProtectedReport = result.map_err(|_| SetupFailure::ManagedOutcome)?;
    let ManagedProtectedTerminal::Refused(failure) = report.terminal() else {
        return Err(SetupFailure::ManagedOutcome);
    };
    if report.first().is_some()
        || report.first_nak_at().is_some()
        || report.remaining_association_work().is_some()
        || report.power_unregistration_error().is_some()
        || (
            report.attempted_sends(),
            report.fully_submitted_sends(),
            report.remaining_driver_work(),
        ) != (0, 0, 64)
        || failure.labels() != ("handshake", "unexpected-eof")
        || failure.primary_labels() != Some(("handshake", "unexpected-eof"))
        || failure.admission_cleanup_labels().is_some()
        || failure.source_policy_error().is_some()
        || failure.retained_configuration().is_some()
        || failure.power_stop_failed()
        || failure.power_unregistration_error().is_some()
    {
        return Err(SetupFailure::ManagedOutcome);
    }
    match failure.cleanup_labels() {
        None => Ok("none"),
        Some(("response", "unexpected-eof")) => Ok("response/unexpected-eof"),
        Some(_) => Err(SetupFailure::ManagedOutcome),
    }
}

/// Prepare one checked source and its separate owned UDP and TLS destinations.
/// # Errors
/// Refuse source, endpoint or TLS target setup before invoking the Engine.
fn managed_definition_inputs(
    address: SocketAddr,
    udp_address: SocketAddr,
    tls_identity: &str,
    sources: usize,
    endpoints: usize,
) -> Result<
    (
        SourcePolicyAuthority,
        SourceHandle,
        SourceAddressSet,
        KePreviewTarget,
    ),
    SetupFailure,
> {
    let limits: SourcePolicyLimits = SourcePolicyLimits::new(
        sources,
        endpoints,
        Duration::from_millis(1),
        Duration::from_millis(4),
    )
    .map_err(|_| SetupFailure::ManagedPolicy)?;
    let authority: SourcePolicyAuthority =
        SourcePolicyAuthority::install(limits).map_err(|_| SetupFailure::ManagedPolicy)?;
    let source: SourceHandle = authority
        .source(0)
        .map_err(|_| SetupFailure::ManagedPolicy)?;
    let addresses: SourceAddressSet =
        SourceAddressSet::new(&[udp_address]).map_err(|_| SetupFailure::ManagedPolicy)?;
    let target: KePreviewTarget =
        KePreviewTarget::new(address, tls_identity).map_err(|_| SetupFailure::ManagedPolicy)?;
    Ok((authority, source, addresses, target))
}

/// Install equal-value revisions and reject the old one through the public caller.
/// # Errors
/// Refuse setup, unchanged revision identity or any failure other than stale policy.
fn managed_definition(
    address: SocketAddr,
    udp_address: SocketAddr,
) -> Result<(SourcePolicyAuthority, SourceConfigurationSnapshot), SetupFailure> {
    let (mut authority, source, addresses, target): (
        SourcePolicyAuthority,
        SourceHandle,
        SourceAddressSet,
        KePreviewTarget,
    ) = managed_definition_inputs(address, udp_address, "localhost", 1, 1)?;
    let stale: SourceConfigurationSnapshot = authority
        .install_ke_preview_configuration(&source, addresses.clone(), target.clone())
        .map_err(|_| SetupFailure::ManagedPolicy)?;
    let current: SourceConfigurationSnapshot = authority
        .replace_ke_preview_configuration(&stale, addresses, target)
        .map_err(|_| SetupFailure::ManagedPolicy)?;
    let stale_failure: SourceProtectedProbeFailure = authority
        .probe_nts_protected(&stale)
        .err()
        .ok_or(SetupFailure::ManagedStale)?;
    if stale.same_configuration(&current)
        || stale_failure.source_policy_error() != Some(SourcePolicyError::ConfigurationChanged)
        || stale_failure.labels() != ("source-policy", "refused")
        || stale_failure.primary_labels() != Some(("source-policy", "refused"))
        || stale_failure.cleanup_labels().is_some()
        || stale_failure.admission_cleanup_labels().is_some()
        || stale_failure.power_stop_failed()
        || stale_failure.power_unregistration_error().is_some()
        || stale_failure.retained_configuration().is_some()
    {
        return Err(SetupFailure::ManagedStale);
    }
    Ok((authority, current))
}

/// Check the real bounded public API before the following managed EOF neighbor.
/// # Errors
/// Reject a result other than the typed, pre-registration deadline refusal.
fn reject_elapsed_managed_cutoff(
    authority: &SourcePolicyAuthority,
    expected: &SourceConfigurationSnapshot,
) -> Result<(), SetupFailure> {
    let cutoff: Instant = Instant::now();
    let failure: SourceProtectedProbeFailure = authority
        .invoke_nts_protected_before(
            expected,
            ManagedProtectedPlan::OneAuthenticatedSuccessor,
            cutoff,
        )
        .err()
        .ok_or(SetupFailure::ManagedDeadline)?;
    if failure.labels() != ("currentness", "deadline")
        || failure.primary_labels() != Some(("currentness", "deadline"))
        || failure.cleanup_labels().is_some()
        || failure.admission_cleanup_labels().is_some()
        || failure.power_stop_failed()
        || failure.power_unregistration_error().is_some()
        || failure.retained_configuration().is_some()
    {
        return Err(SetupFailure::ManagedDeadline);
    }
    Ok(())
}

/// Reach one current managed EOF after the stale-definition and deadline refusals.
/// The same fresh process owns one policy root, observer and worker invocation.
/// # Errors
/// Preserve source setup, peer retirement, managed EOF and UDP-observation failures.
fn managed_protected_eof() -> Result<ExitCode, SetupFailure> {
    let listener: TcpListener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
        .map_err(|_| SetupFailure::PeerStart)?;
    listener
        .set_nonblocking(true)
        .map_err(|_| SetupFailure::PeerStart)?;
    let address: SocketAddr = listener.local_addr().map_err(|_| SetupFailure::PeerStart)?;
    let udp: UdpSocket = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
        .map_err(|_| SetupFailure::ManagedPolicy)?;
    udp.set_nonblocking(true)
        .map_err(|_| SetupFailure::ManagedPolicy)?;
    let udp_address: SocketAddr = udp.local_addr().map_err(|_| SetupFailure::ManagedPolicy)?;
    let (authority, current): (SourcePolicyAuthority, SourceConfigurationSnapshot) =
        managed_definition(address, udp_address)?;
    reject_elapsed_managed_cutoff(&authority, &current)?;
    if !matches!(listener.accept(), Err(error) if error.kind() == io::ErrorKind::WouldBlock) {
        return Err(SetupFailure::ManagedStale);
    }
    let deadline: Instant = Instant::now()
        .checked_add(PEER_WINDOW)
        .ok_or(SetupFailure::PeerStart)?;
    let peer: thread::JoinHandle<Result<(), PeerFailure>> = thread::Builder::new()
        .spawn(move || observe_peer(listener, deadline))
        .map_err(|_| SetupFailure::PeerStart)?;
    let result: Result<ManagedProtectedReport, SourceProtectedProbeFailure> = authority
        .invoke_nts_protected_before(
            &current,
            ManagedProtectedPlan::OneAuthenticatedSuccessor,
            deadline,
        );
    peer.join()
        .map_err(|_| SetupFailure::PeerPanic)?
        .map_err(SetupFailure::Peer)?;
    let cleanup: &'static str = managed_eof_failure(result)?;
    let mut buffer: [u8; 1] = [0];
    if !matches!(udp.recv_from(&mut buffer), Err(error) if error.kind() == io::ErrorKind::WouldBlock)
    {
        return Err(SetupFailure::ManagedDatagram);
    }
    writeln!(io::stderr().lock(),
        "scope=managed-protected-native eligible=0 fresh_sync=0 stale=configuration-changed current=handshake/unexpected-eof cleanup={cleanup} admission=none retained=none power_stop_failed=0 observer_close_error=none peer_joined=1 owned_udp_received=0")
        .map_err(|_| SetupFailure::PeerWitness)?;
    Ok(ExitCode::SUCCESS)
}

/// Admit one required harness endpoint as a literal IPv4 loopback socket address.
/// # Errors
/// Refuse missing, oversized or noncanonical input before any source setup.
fn successor_peer(variable: &'static str) -> Result<SocketAddr, SetupFailure> {
    let raw: OsString = env::var_os(variable).ok_or(SetupFailure::ManagedSuccessorPeers)?;
    if raw.as_encoded_bytes().len() > SUCCESSOR_PEER_BYTES {
        return Err(SetupFailure::ManagedSuccessorPeers);
    }
    let port_text: &str = raw
        .to_str()
        .and_then(|value: &str| value.strip_prefix(SUCCESSOR_PEER_PREFIX))
        .ok_or(SetupFailure::ManagedSuccessorPeers)?;
    if port_text.starts_with('0') || !port_text.bytes().all(|byte: u8| byte.is_ascii_digit()) {
        return Err(SetupFailure::ManagedSuccessorPeers);
    }
    let port: NonZeroU16 = port_text
        .parse()
        .map_err(|_| SetupFailure::ManagedSuccessorPeers)?;
    Ok(SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::LOCALHOST,
        port.get(),
    )))
}

/// Check the original native observations against their corresponding software brackets.
///
/// The protected socket takes all three captures on its current-thread worker.
/// Verify local chronology and exact representation of its original Windows wall reads.
fn successor_clock_captures_match(receipt: &ManagedProtectedReceipt) -> bool {
    let [before_send, after_send, receive]: [SoftwareClock; 3] =
        receipt.application_clock_captures();
    let native: [Option<ProtectedNativeClockObservation>; 3] =
        receipt.application_native_clock_captures();
    let [
        Some(ProtectedNativeClockObservation::Recorded(before_coordinates)),
        Some(ProtectedNativeClockObservation::Recorded(after_coordinates)),
        Some(ProtectedNativeClockObservation::Recorded(receive_coordinates)),
    ] = native
    else {
        return false;
    };
    // From 1601-01-01 through 1970-01-01 the Gregorian calendar spans
    // 134,774 days. Compare elapsed time from that epoch with each whole raw value.
    let Some(filetime_epoch): Option<SystemTime> =
        UNIX_EPOCH.checked_sub(Duration::from_hours(134_774 * 24))
    else {
        return false;
    };
    if before_send.monotonic_after() > after_send.monotonic()
        || after_send.monotonic_after() > receive.monotonic()
        || receive.monotonic_after() != receipt.received_at()
        || receipt.received_at() > receipt.committed_at()
        || before_coordinates.qpc_after() > after_coordinates.qpc_before()
        || after_coordinates.qpc_after() > receive_coordinates.qpc_before()
    {
        return false;
    }
    [
        (before_send, before_coordinates),
        (after_send, after_coordinates),
        (receive, receive_coordinates),
    ]
    .into_iter()
    .all(|(capture, coordinates)| {
        let wall: SystemTime = capture.utc();
        let raw_elapsed_ns: u128 = u128::from(coordinates.filetime_100ns()) * 100;
        capture.monotonic() <= capture.monotonic_after()
            && coordinates.frequency_hz() == before_coordinates.frequency_hz()
            && coordinates.qpc_before() <= coordinates.qpc_after()
            && wall
                .duration_since(filetime_epoch)
                .is_ok_and(|elapsed: Duration| elapsed.as_nanos() == raw_elapsed_ns)
    })
}

/// Check the issued definition, native capture triplet and fixed authenticated peer wire values.
fn successor_receipt_matches(
    receipt: &ManagedProtectedReceipt,
    expected: &SourceConfigurationSnapshot,
) -> bool {
    let peer_timestamps: [u64; 2] = receipt.peer_timestamps().map(|timestamp| timestamp.bits());
    receipt.configuration().same_configuration(expected)
        && peer_timestamps == [0x0123_4567_89ab_d000, 0x0123_4567_89ab_e000]
        && successor_clock_captures_match(receipt)
}

/// Check any retained history against the definition issued for this invocation.
/// # Errors
/// Refuse a different retained revision, unavailable clock observations, or inconsistent
/// native or software chronology or wall representation.
fn check_successor_failure(
    failure: &SourceProtectedProbeFailure,
    expected: &SourceConfigurationSnapshot,
) -> Result<(), SetupFailure> {
    if failure
        .retained_first()
        .is_some_and(|receipt: ManagedProtectedReceipt| {
            !successor_receipt_matches(&receipt, expected)
        })
        || failure.retained_configuration().is_some_and(
            |configuration: &SourceConfigurationSnapshot| {
                !configuration.same_configuration(expected)
            },
        )
    {
        return Err(SetupFailure::ManagedSuccessorHistory);
    }
    Ok(())
}

/// Inspect the policy and alias records after the joined restriction passes report checks.
/// # Errors
/// Refuse a mismatched endpoint, effect, or shared policy state.
fn check_successor_restriction(
    restriction: &ManagedProtectedRestriction,
    source: &SourceHandle,
    alias: &SourceHandle,
    remote: SocketAddr,
) -> Result<(), SetupFailure> {
    let actual: SourcePolicySnapshot = source
        .policy_snapshot(remote)
        .map_err(|_| SetupFailure::ManagedPolicy)?;
    let shared: SourcePolicySnapshot = alias
        .policy_snapshot(remote)
        .map_err(|_| SetupFailure::ManagedPolicy)?;
    let (refused, interval): (bool, Duration) = match restriction.code() {
        "RATE" => (false, Duration::from_millis(2)),
        "DENY" | "RSTR" => (true, Duration::from_millis(1)),
        _ => return Err(SetupFailure::ManagedSuccessorHistory),
    };
    if restriction.remote() != remote
        || [actual.source(), actual.endpoint(), shared.endpoint()]
            .into_iter()
            .any(|record| {
                record.refused() != refused
                    || record.interval() != interval
                    || record.floor().is_none()
                    || record.unavailable()
            })
        || shared.source().refused()
        || shared.source().unavailable()
        || shared.source().interval() != Duration::from_millis(1)
        || shared.source().floor().is_some()
        || shared.endpoint().revision() != actual.endpoint().revision()
        || shared.endpoint().floor() != actual.endpoint().floor()
    {
        return Err(SetupFailure::ManagedSuccessorHistory);
    }
    Ok(())
}

/// Require original receipt identity and order before emitting the joined witness.
/// # Errors
/// Refuse foreign revisions, unavailable or inconsistent clock evidence, inconsistent
/// successor order, or an initial restriction without authentication and exactly one
/// submitted send.
fn check_successor_report(
    report: &ManagedProtectedReport,
    expected: &SourceConfigurationSnapshot,
) -> Result<(), SetupFailure> {
    let first: Option<&ManagedProtectedReceipt> = report.first();
    let first_commit: Option<Instant> = first.map(ManagedProtectedReceipt::committed_at);
    let nak: Option<Instant> = report.first_nak_at();
    if first.is_some_and(|receipt: &ManagedProtectedReceipt| {
        !successor_receipt_matches(receipt, expected)
    }) || nak.is_some_and(|observed: Instant| {
        first_commit.is_none_or(|committed: Instant| observed < committed)
    }) {
        return Err(SetupFailure::ManagedSuccessorHistory);
    }
    let successor_terminal_at: Option<Instant> = match report.terminal() {
        ManagedProtectedTerminal::Ordinary(receipt)
            if successor_receipt_matches(receipt, expected) =>
        {
            Some(receipt.received_at())
        }
        ManagedProtectedTerminal::Restriction(restriction)
            if restriction.configuration().same_configuration(expected) =>
        {
            match restriction.serial() {
                1 if restriction.authenticated()
                    && first.is_none()
                    && report.attempted_sends() == 1
                    && report.fully_submitted_sends() == 1 =>
                {
                    None
                }
                2 => Some(restriction.decided_at()),
                _ => return Err(SetupFailure::ManagedSuccessorHistory),
            }
        }
        ManagedProtectedTerminal::Refused(failure) => {
            check_successor_failure(failure, expected)?;
            None
        }
        ManagedProtectedTerminal::Ordinary(_) | ManagedProtectedTerminal::Restriction(_) => {
            return Err(SetupFailure::ManagedSuccessorHistory);
        }
    };
    if successor_terminal_at.is_some_and(|observed: Instant| {
        first_commit.is_none_or(|committed: Instant| observed < committed)
            || nak.is_some_and(|received: Instant| observed < received)
    }) {
        return Err(SetupFailure::ManagedSuccessorHistory);
    }
    Ok(())
}

/// Project only the local request extent and authenticated peer leap claim.
fn successor_receipt_values(receipt: Option<&ManagedProtectedReceipt>) -> (usize, String) {
    receipt.map_or_else(
        || (0, String::from("none")),
        |value: &ManagedProtectedReceipt| {
            (value.packet_bytes(), value.leap_indicator().to_string())
        },
    )
}

/// Render only a reviewed failure label pair, with absence explicit.
fn successor_labels(labels: Option<(&'static str, &'static str)>) -> String {
    labels.map_or_else(
        || String::from("none"),
        |(stage, cause)| format!("{stage}/{cause}"),
    )
}

/// Write one fixed line from an outcome already checked against its invocation.
/// Report work and NAK observations only when the worker normally joined.
/// # Errors
/// Propagate sink failure without formatting raw endpoints, time, packets or errors.
fn write_successor_report(
    sink: &mut impl Write,
    outcome: Result<&ManagedProtectedReport, &SourceProtectedProbeFailure>,
) -> io::Result<()> {
    let report: Option<&ManagedProtectedReport> = outcome.ok();
    let first: Option<&ManagedProtectedReceipt> = report.and_then(ManagedProtectedReport::first);
    let terminal_result: Result<&ManagedProtectedTerminal, &SourceProtectedProbeFailure> =
        outcome.map(ManagedProtectedReport::terminal);
    let (first_state, configuration, order): (&str, &str, &str) = match (first, terminal_result) {
        (Some(_), _) => ("committed", "original", "valid"),
        (None, Ok(ManagedProtectedTerminal::Restriction(_))) => ("none", "original", "none"),
        (None, _) => ("none", "none", "none"),
    };
    let (terminal, ordinary, restriction, authenticated, failure): (
        &str,
        Option<&ManagedProtectedReceipt>,
        &str,
        &str,
        Option<&SourceProtectedProbeFailure>,
    ) = match terminal_result {
        Ok(ManagedProtectedTerminal::Ordinary(receipt)) => {
            ("ordinary", Some(receipt), "none", "1", None)
        }
        Ok(ManagedProtectedTerminal::Restriction(restriction)) => (
            "restriction",
            None,
            restriction.code(),
            if restriction.authenticated() {
                "1"
            } else {
                "0"
            },
            None,
        ),
        Ok(ManagedProtectedTerminal::Refused(failure)) => {
            ("refused", None, "none", "none", Some(failure))
        }
        Err(failure) => ("invocation-refused", None, "none", "none", Some(failure)),
    };
    let (first_bytes, first_li): (usize, String) = successor_receipt_values(first);
    let (terminal_bytes, terminal_li): (usize, String) = successor_receipt_values(ordinary);
    let failure_labels: String = successor_labels(failure.map(SourceProtectedProbeFailure::labels));
    let primary: String =
        successor_labels(failure.and_then(SourceProtectedProbeFailure::primary_labels));
    let cleanup: String =
        successor_labels(failure.and_then(SourceProtectedProbeFailure::cleanup_labels));
    let admission: String =
        successor_labels(failure.and_then(SourceProtectedProbeFailure::admission_cleanup_labels));
    let retained_history: Option<ManagedProtectedReceipt> =
        failure.and_then(SourceProtectedProbeFailure::retained_first);
    let (retained_first_bytes, retained_first_li): (usize, String) =
        successor_receipt_values(retained_history.as_ref());
    let retained_first: &str = if retained_history.is_some() {
        "original"
    } else {
        "none"
    };
    let retained_nak: u8 = u8::from(
        failure
            .and_then(SourceProtectedProbeFailure::retained_first_nak_at)
            .is_some(),
    );
    let retained: &str = if failure
        .and_then(SourceProtectedProbeFailure::retained_configuration)
        .is_some()
    {
        "original"
    } else {
        "none"
    };
    let power_stop_failed: u8 =
        u8::from(failure.is_some_and(SourceProtectedProbeFailure::power_stop_failed));
    let observer_close_failed: bool = match outcome {
        Ok(report) => report.power_unregistration_error().is_some(),
        Err(failure) => failure.power_unregistration_error().is_some(),
    };
    let observer_close_error: &str = if observer_close_failed {
        "present"
    } else {
        "none"
    };
    write!(
        sink,
        "scope=managed-protected-successor-native measurement=mapping-unavailable eligible=0 fresh_sync=0 \
         first={first_state} configuration={configuration} receipt_order={order} first_bytes={first_bytes} first_li={first_li} \
         terminal={terminal} restriction={restriction} authenticated={authenticated} terminal_bytes={terminal_bytes} terminal_li={terminal_li} \
         failure={failure_labels} primary={primary} cleanup={cleanup} admission={admission} retained_first={retained_first} retained_first_bytes={retained_first_bytes} retained_first_li={retained_first_li} retained_nak={retained_nak} retained={retained} \
         power_stop_failed={power_stop_failed} observer_close_error={observer_close_error}"
    )?;
    if let Some(report) = report {
        write_successor_work(sink, report)?;
    }
    writeln!(sink)
}

/// Write the actual local adapter work and original bounded recovery disposition.
/// # Errors
/// Preserve the first write refusal.
fn write_successor_work(sink: &mut impl Write, report: &ManagedProtectedReport) -> io::Result<()> {
    let nak_seen: u8 = u8::from(report.first_nak_at().is_some());
    let recovery_unavailable: u8 = u8::from(report.recovery_unavailable());
    write!(
        sink,
        " nak_seen={nak_seen} recovery_unavailable={recovery_unavailable} attempted_sends={} fully_submitted_sends={} remaining_driver_work={}",
        report.attempted_sends(),
        report.fully_submitted_sends(),
        report.remaining_driver_work()
    )?;
    if let Some((checkouts, frames, bytes)) = report.remaining_association_work() {
        write!(
            sink,
            " remaining_checkouts={checkouts} remaining_frames={frames} remaining_bytes={bytes}"
        )?;
    }
    Ok(())
}

/// Project the validated loopback tuple from the original joined receipt.
/// # Errors
/// Refuse an unassigned port, a nonfixture local address or an unexpected peer.
fn successor_timing_ports(
    receipt: &ManagedProtectedReceipt,
    expected_peer: SocketAddr,
) -> Result<(NonZeroU16, NonZeroU16), SetupFailure> {
    let local: SocketAddr = receipt.local_endpoint();
    let peer: SocketAddr = receipt.peer_endpoint();
    if local.ip() != Ipv4Addr::LOCALHOST
        || peer.ip() != Ipv4Addr::LOCALHOST
        || peer != expected_peer
    {
        return Err(SetupFailure::ManagedSuccessorTiming);
    }
    let client_port: NonZeroU16 =
        NonZeroU16::new(local.port()).ok_or(SetupFailure::ManagedSuccessorTiming)?;
    let peer_port: NonZeroU16 =
        NonZeroU16::new(peer.port()).ok_or(SetupFailure::ManagedSuccessorTiming)?;
    Ok((client_port, peer_port))
}

/// Export both original native triplets and their loopback tuples from the joined job.
///
/// The six rows carry loopback ports and application-boundary coordinates for the lab reader.
/// They grant no calibrated mapping, sample eligibility or clock authority.
///
/// # Errors
/// Refuse incomplete or mismatched history, missing or incompatible timing observations,
/// invalid native capture spans, mismatched frequencies, overlapping or reversed legs,
/// a wrong endpoint, or a failed write.
fn write_successor_timing(
    report: &ManagedProtectedReport,
    expected_peer: SocketAddr,
) -> Result<(), SetupFailure> {
    let (Some(first), ManagedProtectedTerminal::Ordinary(successor)) =
        (report.first(), report.terminal())
    else {
        return Err(SetupFailure::ManagedSuccessorTiming);
    };
    if !first.same_invocation(successor)
        || !first.same_ke_association(successor)
        || first.same_exchange(successor)
        || [first, successor]
            .into_iter()
            .any(|receipt: &ManagedProtectedReceipt| {
                receipt.certificate_time_provenance()
                    != CertificateTimeProvenance::WindowsLaboratoryLocalWall
            })
        || report.first_nak_at().is_some()
        || report.power_unregistration_error().is_some()
        || (report.attempted_sends(), report.fully_submitted_sends()) != (2, 2)
    {
        return Err(SetupFailure::ManagedSuccessorTiming);
    }
    report
        .check_timing_observations()
        .map_err(|_| SetupFailure::ManagedSuccessorTiming)?;
    let first_ports: (NonZeroU16, NonZeroU16) = successor_timing_ports(first, expected_peer)?;
    let next_ports: (NonZeroU16, NonZeroU16) = successor_timing_ports(successor, expected_peer)?;
    let first_span: ProtectedNativeCaptureSpan = first
        .native_capture_span()
        .map_err(|_| SetupFailure::ManagedSuccessorTiming)?;
    let next_span: ProtectedNativeCaptureSpan = successor
        .native_capture_span()
        .map_err(|_| SetupFailure::ManagedSuccessorTiming)?;
    let [_, first_end]: [i64; 2] = first_span.qpc_bounds();
    let [next_start, _]: [i64; 2] = next_span.qpc_bounds();
    if first_end > next_start || first_span.frequency_hz() != next_span.frequency_hz() {
        return Err(SetupFailure::ManagedSuccessorTiming);
    }
    let mut sink: io::StdoutLock<'_> = io::stdout().lock();
    for (leg, (client_port, peer_port), captures) in [
        ("first", first_ports, first_span.captures()),
        ("successor", next_ports, next_span.captures()),
    ] {
        for (point, coordinates) in ["pre-send", "send-return", "receive-return"]
            .into_iter()
            .zip(captures)
        {
            writeln!(
                sink,
                "scope=managed-protected-successor-timing leg={leg} point={point} \
                 client_port={client_port} peer_port={peer_port} \
                 qpc_before={} filetime_100ns={} qpc_after={} frequency_hz={}",
                coordinates.qpc_before(),
                coordinates.filetime_100ns(),
                coordinates.qpc_after(),
                coordinates.frequency_hz(),
            )
            .map_err(|_| SetupFailure::PeerWitness)?;
        }
    }
    sink.flush().map_err(|_| SetupFailure::PeerWitness)
}

/// Invoke the actual protected successor facade once with both admitted peers.
/// A joined observation exits zero, including a typed terminal refusal. An outer
/// refusal exits with the managed setup code after its separate typed witness.
/// The timing selector additionally requires a healthy ordinary exchange.
/// # Errors
/// Preserve endpoint admission, installation, witness continuity and output refusals.
fn managed_protected_successor(witness: SuccessorWitness) -> Result<ExitCode, SetupFailure> {
    let ke_peer: SocketAddr = successor_peer(SUCCESSOR_KE_PEER)?;
    let udp_peer: SocketAddr = successor_peer(SUCCESSOR_UDP_PEER)?;
    let (mut authority, source, addresses, target): (
        SourcePolicyAuthority,
        SourceHandle,
        SourceAddressSet,
        KePreviewTarget,
    ) = managed_definition_inputs(ke_peer, udp_peer, SUCCESSOR_TLS_IDENTITY, 2, 1)?;
    let alias: SourceHandle = authority
        .source(1)
        .map_err(|_| SetupFailure::ManagedPolicy)?;
    let expected: SourceConfigurationSnapshot = authority
        .install_ke_preview_configuration(&source, addresses, target)
        .map_err(|_| SetupFailure::ManagedPolicy)?;
    let outcome: Result<ManagedProtectedReport, SourceProtectedProbeFailure> =
        authority.invoke_nts_protected(&expected, ManagedProtectedPlan::OneAuthenticatedSuccessor);
    let code: ExitCode = match outcome.as_ref() {
        Ok(report) => {
            check_successor_report(report, &expected)?;
            if let ManagedProtectedTerminal::Restriction(restriction) = report.terminal() {
                check_successor_restriction(restriction, &source, &alias, udp_peer)?;
            }
            ExitCode::SUCCESS
        }
        Err(failure) => {
            check_successor_failure(failure, &expected)?;
            ExitCode::from(124)
        }
    };
    write_successor_report(&mut io::stderr().lock(), outcome.as_ref())
        .map_err(|_| SetupFailure::PeerWitness)?;
    if matches!(witness, SuccessorWitness::Timing) {
        let report: &ManagedProtectedReport = outcome
            .as_ref()
            .map_err(|_| SetupFailure::ManagedSuccessorTiming)?;
        write_successor_timing(report, udp_peer)?;
    }
    Ok(code)
}

/// Require the two original ordinary receipts and admitted work for one paired job.
///
/// # Errors
/// Refuse absent or foreign receipts, unexpected NAKs or mismatched work bounds.
fn checked_pair_receipts<'a>(
    report: &'a ManagedProtectedReport,
    expected: &SourceConfigurationSnapshot,
    expected_peer: SocketAddr,
) -> Result<[&'a ManagedProtectedReceipt; 2], SetupFailure> {
    let (Some(first), ManagedProtectedTerminal::Ordinary(successor)): (
        Option<&ManagedProtectedReceipt>,
        &ManagedProtectedTerminal,
    ) = (report.first(), report.terminal()) else {
        return Err(SetupFailure::ManagedPairOutcome);
    };
    if !first.same_invocation(successor)
        || !first.same_ke_association(successor)
        || first.same_exchange(successor)
        || report.first_nak_at().is_some()
        || report.power_unregistration_error().is_some()
        || (report.attempted_sends(), report.fully_submitted_sends()) != (2, 2)
        || report.remaining_driver_work() > 58
        || report.remaining_association_work() != Some((30, 30, 130_792))
        || [first, successor]
            .into_iter()
            .any(|receipt: &ManagedProtectedReceipt| {
                receipt.certificate_time_provenance()
                    != CertificateTimeProvenance::WindowsLaboratoryLocalWall
                    || receipt.packet_bytes() != 252
                    || receipt.leap_indicator() != 0
                    || receipt.peer_endpoint() != expected_peer
                    || !receipt.local_endpoint().ip().is_loopback()
                    || receipt.local_endpoint().port() == 0
                    || !successor_receipt_matches(receipt, expected)
            })
    {
        return Err(SetupFailure::ManagedPairOutcome);
    }
    Ok([first, successor])
}

/// Compare adjacent original captures without creating a clock mapping.
fn pair_receipt_precedes(prior: &ManagedProtectedReceipt, next: &ManagedProtectedReceipt) -> bool {
    use ProtectedNativeClockObservation::Recorded;

    let prior_native: [Option<ProtectedNativeClockObservation>; 3] =
        prior.application_native_clock_captures();
    let next_native: [Option<ProtectedNativeClockObservation>; 3] =
        next.application_native_clock_captures();
    let [_, _, Some(Recorded(receive))]: [Option<ProtectedNativeClockObservation>; 3] =
        prior_native
    else {
        return false;
    };
    let [Some(Recorded(before_send)), ..]: [Option<ProtectedNativeClockObservation>; 3] =
        next_native
    else {
        return false;
    };
    let [next_start, ..]: [SoftwareClock; 3] = next.application_clock_captures();
    prior.committed_at() <= next_start.monotonic()
        && receive.frequency_hz() == before_send.frequency_hz()
        && receive.qpc_after() <= before_send.qpc_before()
}

/// Run the two separately installed sources through one managed pair and cutoff.
///
/// # Errors
/// Refuse malformed or crossed endpoints, incomplete joined custody, inconsistent
/// original captures, or failed observer retirement before reporting success.
fn managed_protected_pair() -> Result<ExitCode, SetupFailure> {
    let first_ke: SocketAddr = successor_peer(SUCCESSOR_KE_PEER)?;
    let first_udp: SocketAddr = successor_peer(SUCCESSOR_UDP_PEER)?;
    let second_ke: SocketAddr = successor_peer(PAIR_SECOND_KE_PEER)?;
    let second_udp: SocketAddr = successor_peer(PAIR_SECOND_UDP_PEER)?;
    if first_ke == second_ke || first_udp == second_udp {
        return Err(SetupFailure::ManagedSuccessorPeers);
    }

    let (mut authority, first, addresses, target): (
        SourcePolicyAuthority,
        SourceHandle,
        SourceAddressSet,
        KePreviewTarget,
    ) = managed_definition_inputs(first_ke, first_udp, SUCCESSOR_TLS_IDENTITY, 2, 2)?;
    let first_expected: SourceConfigurationSnapshot = authority
        .install_ke_preview_configuration(&first, addresses, target)
        .map_err(|_| SetupFailure::ManagedPolicy)?;
    let second: SourceHandle = authority
        .source(1)
        .map_err(|_| SetupFailure::ManagedPolicy)?;
    let second_addresses: SourceAddressSet =
        SourceAddressSet::new(&[second_udp]).map_err(|_| SetupFailure::ManagedPolicy)?;
    let second_target: KePreviewTarget = KePreviewTarget::new(second_ke, SUCCESSOR_TLS_IDENTITY)
        .map_err(|_| SetupFailure::ManagedPolicy)?;
    let second_expected: SourceConfigurationSnapshot = authority
        .install_ke_preview_configuration(&second, second_addresses, second_target)
        .map_err(|_| SetupFailure::ManagedPolicy)?;
    let cutoff: Instant = Instant::now()
        .checked_add(PAIR_WINDOW)
        .ok_or(SetupFailure::ManagedDeadline)?;
    let pair: ManagedProtectedPairReport = authority
        .invoke_nts_protected_pair_before(&first_expected, &second_expected, cutoff)
        .map_err(|_| SetupFailure::ManagedPairOutcome)?;
    let second_report: &ManagedProtectedReport = pair
        .second()
        .ok_or(SetupFailure::ManagedPairOutcome)?
        .map_err(|_| SetupFailure::ManagedPairOutcome)?;
    let [first_initial, first_terminal]: [&ManagedProtectedReceipt; 2] =
        checked_pair_receipts(pair.first(), &first_expected, first_udp)?;
    let [second_initial, second_terminal]: [&ManagedProtectedReceipt; 2] =
        checked_pair_receipts(second_report, &second_expected, second_udp)?;

    if first_initial.same_invocation(second_initial)
        || first_initial.same_ke_association(second_initial)
        || first_terminal.committed_at() >= cutoff
        || second_terminal.committed_at() >= cutoff
        || !pair_receipt_precedes(first_initial, first_terminal)
        || !pair_receipt_precedes(first_terminal, second_initial)
        || !pair_receipt_precedes(second_initial, second_terminal)
    {
        return Err(SetupFailure::ManagedPairOrder);
    }
    if pair.power_unregistration_error().is_some() {
        return Err(SetupFailure::ManagedPairClose);
    }
    writeln!(
        io::stderr().lock(),
        "scope=managed-protected-pair-native measurement=mapping-unavailable eligible=0 fresh_sync=0 \
         jobs=2 configuration=original receipt_order=valid authenticated=1 attempted_sends=4 \
         fully_submitted_sends=4 observer_close_error=none"
    )
    .map_err(|_| SetupFailure::PeerWitness)?;
    Ok(ExitCode::SUCCESS)
}

/// Invoke only the production facade and retain its operation and fixture outcomes.
/// # Errors
/// Preserve setup or peer completion refusal alongside any Engine diagnostic.
fn run(mode: &Mode) -> Result<ExitCode, SetupFailure> {
    match mode {
        Mode::Arguments => Ok(nevrotick_engine::experimental_nts_ke_probe([
            OsString::from(PROGRAM),
        ])),
        Mode::EnteredRuntime => {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .build()
                .map_err(|_| SetupFailure::Runtime)?;
            let entered = runtime.enter();
            let code = nevrotick_engine::experimental_nts_ke_probe([
                OsString::from(PROGRAM),
                OsString::from("127.0.0.1:4460"),
                OsString::from("localhost"),
            ]);
            drop(entered);
            drop(runtime);
            Ok(code)
        }
        Mode::LoopbackEof => loopback_eof(),
        Mode::ManagedProtectedEof => managed_protected_eof(),
        Mode::ManagedProtectedSuccessor(witness) => managed_protected_successor(*witness),
        Mode::ManagedProtectedPair => managed_protected_pair(),
    }
}

/// Preserve Engine exits after the selected fixture completes.
fn main() -> ExitCode {
    match Mode::from_arguments().and_then(|mode| run(&mode)) {
        Ok(code) => code,
        Err(failure) => failure.report(),
    }
}

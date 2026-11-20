// SPDX-License-Identifier: GPL-2.0-only
//! Fixed Windows production-facade consumer for refusal and loopback observations.
//!
//! Link the normal Engine and Tokio artifacts from the admitted Root graph.
//! The supervisor admits this v3 image and owns its process/output custody.
//! Exactly one closed mode selects an Engine operation. The managed successor
//! mode obtains two canonical IPv4 loopback endpoints from the private harness
//! environment and installs them with a fixed TLS identity.
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
    time::{Duration, Instant},
};

use nevrotick_engine::{
    KePreviewTarget, ManagedProtectedPlan, ManagedProtectedReceipt, ManagedProtectedReport,
    ManagedProtectedRestriction, ManagedProtectedTerminal, SourceAddressSet,
    SourceConfigurationSnapshot, SourceHandle, SourcePolicyAuthority, SourcePolicyError,
    SourcePolicyLimits, SourcePolicySnapshot, SourceProtectedProbeFailure,
    SourceProtectedProbeReport,
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
    /// The actual managed failure or independent cleanup differed from EOF.
    ManagedOutcome,
    /// A protected or fallback UDP datagram reached the owned endpoint.
    ManagedDatagram,
    /// One of the two required private successor endpoints did not pass admission.
    ManagedSuccessorPeers,
    /// Successor history did not retain the original revision or chronological order.
    ManagedSuccessorHistory,
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
            Self::ManagedOutcome => ("managed-outcome", 124),
            Self::ManagedDatagram => ("managed-datagram", 124),
            Self::ManagedSuccessorPeers => ("managed-successor-peers", 124),
            Self::ManagedSuccessorHistory => ("managed-successor-history", 124),
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
    ManagedProtectedSuccessor,
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
                Self::ManagedProtectedSuccessor
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
    result: Result<SourceProtectedProbeReport, SourceProtectedProbeFailure>,
) -> Result<&'static str, SetupFailure> {
    let failure: SourceProtectedProbeFailure = result.err().ok_or(SetupFailure::ManagedOutcome)?;
    if failure.labels() != ("handshake", "unexpected-eof")
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
        1,
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
    ) = managed_definition_inputs(address, udp_address, "localhost", 1)?;
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

/// Reach one current managed EOF after the exact stale-definition refusal.
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
    if !matches!(listener.accept(), Err(error) if error.kind() == io::ErrorKind::WouldBlock) {
        return Err(SetupFailure::ManagedStale);
    }
    let deadline: Instant = Instant::now()
        .checked_add(PEER_WINDOW)
        .ok_or(SetupFailure::PeerStart)?;
    let peer: thread::JoinHandle<Result<(), PeerFailure>> = thread::Builder::new()
        .spawn(move || observe_peer(listener, deadline))
        .map_err(|_| SetupFailure::PeerStart)?;
    let result: Result<SourceProtectedProbeReport, SourceProtectedProbeFailure> =
        authority.probe_nts_protected(&current);
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

/// Check any retained history against the definition issued for this invocation.
/// # Errors
/// Refuse a retained revision that differs from the original installation.
fn check_successor_failure(
    failure: &SourceProtectedProbeFailure,
    expected: &SourceConfigurationSnapshot,
) -> Result<(), SetupFailure> {
    if failure
        .retained_first()
        .is_some_and(|receipt: ManagedProtectedReceipt| {
            !receipt.configuration().same_configuration(expected)
                || receipt.received_at() > receipt.committed_at()
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

/// Inspect the actual policy and alias records after a joined restriction effect.
/// # Errors
/// Refuse a mismatched ordinal, endpoint, effect, or shared policy state.
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
    if restriction.serial() != 2
        || restriction.remote() != remote
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
/// Refuse foreign revisions or a terminal/NAK observation inconsistent with the
/// original receipt, including success or an admitted NAK without a first receipt.
fn check_successor_report(
    report: &ManagedProtectedReport,
    expected: &SourceConfigurationSnapshot,
) -> Result<(), SetupFailure> {
    let first: Option<&ManagedProtectedReceipt> = report.first();
    let first_commit: Option<Instant> = first.map(ManagedProtectedReceipt::committed_at);
    let nak: Option<Instant> = report.first_nak_at();
    if first.is_some_and(|receipt: &ManagedProtectedReceipt| {
        !receipt.configuration().same_configuration(expected)
            || receipt.received_at() > receipt.committed_at()
    }) || nak.is_some_and(|observed: Instant| {
        first_commit.is_none_or(|committed: Instant| observed < committed)
    }) {
        return Err(SetupFailure::ManagedSuccessorHistory);
    }
    let terminal: Option<Instant> = match report.terminal() {
        ManagedProtectedTerminal::Ordinary(receipt)
            if receipt.configuration().same_configuration(expected)
                && receipt.received_at() <= receipt.committed_at() =>
        {
            Some(receipt.received_at())
        }
        ManagedProtectedTerminal::Restriction(restriction)
            if restriction.configuration().same_configuration(expected) =>
        {
            Some(restriction.decided_at())
        }
        ManagedProtectedTerminal::Refused(failure) => {
            check_successor_failure(failure, expected)?;
            None
        }
        ManagedProtectedTerminal::Ordinary(_) | ManagedProtectedTerminal::Restriction(_) => {
            return Err(SetupFailure::ManagedSuccessorHistory);
        }
    };
    if terminal.is_some_and(|observed: Instant| {
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
    let (first_state, configuration, order): (&str, &str, &str) = if first.is_some() {
        ("committed", "original", "valid")
    } else {
        ("none", "none", "none")
    };
    let (terminal, ordinary, restriction, authenticated, failure): (
        &str,
        Option<&ManagedProtectedReceipt>,
        &str,
        &str,
        Option<&SourceProtectedProbeFailure>,
    ) = match outcome.map(ManagedProtectedReport::terminal) {
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

/// Invoke the actual protected successor facade once with both admitted peers.
/// A joined observation exits zero, including a typed terminal refusal. An outer
/// refusal exits with the managed setup code after its separate typed witness.
/// # Errors
/// Preserve endpoint admission, installation, witness continuity and output refusals.
fn managed_protected_successor() -> Result<ExitCode, SetupFailure> {
    let ke_peer: SocketAddr = successor_peer(SUCCESSOR_KE_PEER)?;
    let udp_peer: SocketAddr = successor_peer(SUCCESSOR_UDP_PEER)?;
    let (mut authority, source, addresses, target): (
        SourcePolicyAuthority,
        SourceHandle,
        SourceAddressSet,
        KePreviewTarget,
    ) = managed_definition_inputs(ke_peer, udp_peer, SUCCESSOR_TLS_IDENTITY, 2)?;
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
    Ok(code)
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
        Mode::ManagedProtectedSuccessor => managed_protected_successor(),
    }
}

/// Preserve Engine exits after the selected fixture completes.
fn main() -> ExitCode {
    match Mode::from_arguments().and_then(|mode| run(&mode)) {
        Ok(code) => code,
        Err(failure) => failure.report(),
    }
}

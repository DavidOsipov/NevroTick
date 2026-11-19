// SPDX-License-Identifier: GPL-2.0-only
//! Fixed Windows production-facade refusal and loopback EOF consumer.
//!
//! Link the normal Engine and Tokio artifacts from the admitted Root graph.
//! The supervisor admits this v3 image and owns its process/output custody.
//! Exactly one closed mode selects fixed Engine arguments; caller text never
//! becomes a destination, identity, operation flag or Engine configuration.
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
    process::ExitCode,
    thread,
    time::{Duration, Instant},
};

use nevrotick_engine::{
    KePreviewTarget, SourceAddressSet, SourceConfigurationSnapshot, SourceHandle,
    SourcePolicyAuthority, SourcePolicyError, SourcePolicyLimits, SourceProtectedProbeFailure,
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
        };
        let _diagnostic_result = writeln!(io::stderr().lock(), "facade-consumer-stage={stage}");
        ExitCode::from(code)
    }
}

/// Closed calls to the normal production facade, with one owned loopback peer.
enum Mode {
    /// Supply only a program atom, leaving required Engine arguments absent.
    Arguments,
    /// Supply valid fixed arguments while a current-thread runtime is entered.
    EnteredRuntime,
    /// Observe one real `ClientHello`, then signal EOF without providing TLS material.
    LoopbackEof,
    /// Refuse a stale managed definition, then observe one current managed EOF.
    ManagedProtectedEof,
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

/// Install equal-value revisions and reject the old one through the public caller.
/// # Errors
/// Refuse setup, unchanged revision identity or any failure other than stale policy.
fn managed_definition(
    address: SocketAddr,
    udp_address: SocketAddr,
) -> Result<(SourcePolicyAuthority, SourceConfigurationSnapshot), SetupFailure> {
    let limits: SourcePolicyLimits =
        SourcePolicyLimits::new(1, 1, Duration::from_millis(1), Duration::from_millis(4))
            .map_err(|_| SetupFailure::ManagedPolicy)?;
    let mut authority: SourcePolicyAuthority =
        SourcePolicyAuthority::install(limits).map_err(|_| SetupFailure::ManagedPolicy)?;
    let source: SourceHandle = authority
        .source(0)
        .map_err(|_| SetupFailure::ManagedPolicy)?;
    let addresses: SourceAddressSet =
        SourceAddressSet::new(&[udp_address]).map_err(|_| SetupFailure::ManagedPolicy)?;
    let target: KePreviewTarget =
        KePreviewTarget::new(address, "localhost").map_err(|_| SetupFailure::ManagedPolicy)?;
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
    }
}

/// Preserve Engine exits after the selected fixture completes.
fn main() -> ExitCode {
    match Mode::from_arguments().and_then(|mode| run(&mode)) {
        Ok(code) => code,
        Err(failure) => failure.report(),
    }
}

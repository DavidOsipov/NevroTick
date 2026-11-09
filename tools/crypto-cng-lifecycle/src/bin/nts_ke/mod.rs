//! One bounded laboratory KE job; this module grants no product authority.
//!
//! CLI construction checks immutable endpoint/identity inputs before effects.
//! Response helpers produce only inert negotiation/cookie data. Only the Windows
//! transport child can complete the real TLS/exporter operation. Cancellation
//! and the final time observation share a serialized, consume-once commit.

use std::ffi::OsString;
use std::io::{self, Read};
use std::net::{IpAddr, SocketAddr};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use ntp_wire::{
    NtsKeAccumulator, NtsKeAccumulatorError, NtsKeExporterContext, NtsKeExporterDirection,
    NtsKeFrameLimits, NtsKeNegotiationError, NtsKeNtpv4Offer, NtsKeProgress,
    NtsKeRequestEncodeError, encode_nts_ke_ntpv4_request, negotiate_nts_ke_ntpv4,
};
use rustls_pki_types::ServerName;
use zeroize::Zeroizing;

mod handoff;
mod request;
#[cfg(test)]
mod tests;
#[cfg(windows)]
mod transport;
mod transport_fault;

/// Maximum complete plaintext response, including record framing.
const RESPONSE_LIMIT: usize = 131_072;
/// Maximum record count, including the sole final EOM.
const RECORD_LIMIT: usize = 32_768;
/// Maximum application-owned read scratch, wiped on ordinary drop.
const SCRATCH_LIMIT: usize = 4_096;
/// Maximum opaque initial cookie count; even empty cookies consume a slot.
const COOKIE_LIMIT: usize = 8;
/// Independent defensive aggregate limit on copied cookie payload.
const COOKIE_BYTES_LIMIT: usize = 131_072;
/// Connect and full handshake share this duration from the original origin.
const HANDSHAKE_TIME: Duration = Duration::from_secs(20);
/// KE publication and ephemeral expiry share this original-origin duration.
const EXCHANGE_TIME: Duration = Duration::from_secs(60);

/// Closed failures contain no identity, endpoint, packet or library-error text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Failure {
    /// Raw CLI input fails exact admission.
    Arguments,
    /// A bounded allocation was refused.
    Allocation,
    /// Fresh monotonic time is outside a strict-before deadline.
    Deadline,
    /// Cancellation won before commit.
    Cancelled,
    /// The control lock or consume-once state is unavailable.
    Control,
    /// Fixed encoder parameters or the actual request were rejected.
    Offer(NtsKeRequestEncodeError),
    /// Fixed framing limits were rejected before parsing.
    Limits,
    /// A response read failed with this payload-free typed category.
    Response(io::ErrorKind),
    /// The actual incremental framing operation refused its input.
    Frame(NtsKeAccumulatorError),
    /// Actual offer/response negotiation refused its input.
    Negotiation(NtsKeNegotiationError),
    /// Cookie count, aggregate size or a checked copy exceeded its budget.
    Cookies,
    /// The actual provider refused the one bounded UID/nonce entropy request.
    Entropy,
    /// The private fixed-shape request receiver refused its exact input.
    Packet(request::RequestError),
    /// An unadmitted endpoint hint or zero port was supplied.
    Endpoint,
    /// Connecting the one numeric peer failed.
    #[cfg(windows)]
    Connect(io::ErrorKind),
    /// The actual connected endpoint differs from the admitted endpoint.
    #[cfg(windows)]
    Peer,
    /// Explicit provider, verifier or TLS configuration construction failed.
    #[cfg(windows)]
    TlsSetup,
    /// The real handshake failed; arbitrary error text is not retained.
    #[cfg(windows)]
    Handshake(io::ErrorKind),
    /// Negotiated protocol, ALPN or full-handshake requirements did not hold.
    #[cfg(windows)]
    TlsProfile,
    /// Complete request write or its mandatory flush failed.
    #[cfg(windows)]
    Request(io::ErrorKind),
    /// A regular same-connection exporter failed.
    #[cfg(windows)]
    Exporter,
    /// Required local close-notify flush failed.
    #[cfg(windows)]
    Shutdown(io::ErrorKind),
}

/// Admitted immutable inputs for one numeric, loopback-only laboratory job.
struct Config {
    /// Actual connection destination; no DNS or address alternative is selected.
    peer: SocketAddr,
    /// Separate, bounded original verification identity and admitted hint mapping.
    identity: String,
    /// The standard checked DNS/IP identity used by the actual TLS constructor.
    server_name: ServerName<'static>,
}

impl Config {
    /// Parse exact bounded inputs before any provider or connection is touched.
    ///
    /// Invalid Unicode, names, ports, non-loopback peers and scoped IPv6 refuse.
    ///
    /// # Errors
    /// Refuses invalid Unicode, lengths, address/name syntax, port zero, non-loopback peers
    /// and every scoped IPv6 spelling before effects.
    fn new(peer: OsString, identity: OsString) -> Result<Self, Failure> {
        if peer.len() > 128 || identity.len() > 253 {
            return Err(Failure::Arguments);
        }
        let peer = peer.into_string().map_err(|_| Failure::Arguments)?;
        if peer.contains('%') {
            return Err(Failure::Arguments);
        }
        let peer: SocketAddr = peer.parse().map_err(|_| Failure::Arguments)?;
        if !peer.ip().is_loopback() || peer.port() == 0 {
            return Err(Failure::Arguments);
        }
        if let SocketAddr::V6(address) = peer
            && (address.scope_id() != 0 || address.flowinfo() != 0)
        {
            return Err(Failure::Arguments);
        }
        let identity = identity.into_string().map_err(|_| Failure::Arguments)?;
        if identity.is_empty() || identity.len() > 253 || !identity.is_ascii() {
            return Err(Failure::Arguments);
        }
        let server_name = ServerName::try_from(identity.clone()).map_err(|_| Failure::Arguments)?;
        Ok(Self {
            peer,
            identity,
            server_name,
        })
    }
}

/// Socket phase chooses a deadline, never a fresh relative timeout budget.
#[derive(Clone, Copy)]
enum IoPhase {
    /// Connect plus TLS must both finish before origin plus twenty seconds.
    Handshake,
    /// All subsequent KE work remains before origin plus sixty seconds.
    Exchange,
}

/// Serialized terminal phase; historical commit cannot be rewritten by stop.
#[derive(Clone, Copy)]
enum Phase {
    /// The sole job still has permission to prepare and commit.
    Open,
    /// Cancellation has irrevocably closed publication.
    Cancelled,
    /// Complete publication already won its final guard.
    Committed,
}

/// Explicit result of cancellation competing with the one commit point.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CancelOutcome {
    /// This call closed publication.
    Cancelled,
    /// Publication was already closed by an earlier cancellation.
    AlreadyCancelled,
    /// Historical completion already committed; this stop is too late.
    AlreadyCommitted,
}

/// One invocation's immutable time bounds and serialized publication authority.
struct Control {
    /// Original monotonic origin; earlier supplied times cannot gain lifetime.
    origin: Instant,
    /// Connect/handshake deadline from the original invocation origin.
    handshake_deadline: Instant,
    /// Full-job deadline and candidate expiry, never reset by progress.
    expiry: Instant,
    /// Cancellation and commit share this short, non-I/O critical section.
    phase: Mutex<Phase>,
}

impl Control {
    /// Establish both finite deadlines once; arithmetic failure creates no job.
    ///
    /// # Errors
    /// Refuses monotonic deadline arithmetic overflow; creates no usable control on
    /// failure.
    fn new(start: Instant) -> Result<Self, Failure> {
        Ok(Self {
            origin: start,
            handshake_deadline: start.checked_add(HANDSHAKE_TIME).ok_or(Failure::Deadline)?,
            expiry: start.checked_add(EXCHANGE_TIME).ok_or(Failure::Deadline)?,
            phase: Mutex::new(Phase::Open),
        })
    }

    /// Observe actual fresh time while holding the current-publication guard.
    ///
    /// # Errors
    /// Refuses a poisoned lock, cancelled/committed phase or expired deadline using fresh
    /// time sampled under the lock.
    fn remaining(&self, phase: IoPhase) -> Result<Duration, Failure> {
        let state = self.phase.lock().map_err(|_| Failure::Control)?;
        self.remaining_locked(*state, phase, Instant::now())
    }

    /// Testable time boundary with the same mandatory phase guard and rule.
    #[cfg(test)]
    ///
    /// # Errors
    /// Refuses unavailable publication authority, pre-origin time and time at or after the
    /// selected deadline.
    fn remaining_at(&self, phase: IoPhase, now: Instant) -> Result<Duration, Failure> {
        let state = self.phase.lock().map_err(|_| Failure::Control)?;
        self.remaining_locked(*state, phase, now)
    }

    /// Check the immutable deadline only within an already-held phase guard.
    ///
    /// # Errors
    /// Refuses closed authority, time before the origin and a zero or negative remaining
    /// lifetime.
    fn remaining_locked(
        &self,
        state: Phase,
        phase: IoPhase,
        now: Instant,
    ) -> Result<Duration, Failure> {
        match state {
            Phase::Cancelled => return Err(Failure::Cancelled),
            Phase::Committed => return Err(Failure::Control),
            Phase::Open => {}
        }
        if now < self.origin {
            return Err(Failure::Deadline);
        }
        let deadline = match phase {
            IoPhase::Handshake => self.handshake_deadline,
            IoPhase::Exchange => self.expiry,
        };
        deadline
            .checked_duration_since(now)
            .filter(|value| !value.is_zero())
            .ok_or(Failure::Deadline)
    }

    /// Commit after a fresh time observation under the cancellation lock.
    ///
    /// # Errors
    /// Refuses poisoned, cancelled, already committed or expired state; no commit occurs on
    /// refusal.
    fn commit(&self) -> Result<(), Failure> {
        let mut state = self.phase.lock().map_err(|_| Failure::Control)?;
        self.commit_locked(&mut state, Instant::now())
    }

    /// Exercise the identical commit transition at an independent boundary time.
    #[cfg(test)]
    ///
    /// # Errors
    /// Refuses closed authority or a time outside the strict original-origin lifetime,
    /// without changing the phase.
    fn commit_at(&self, now: Instant) -> Result<(), Failure> {
        let mut state = self.phase.lock().map_err(|_| Failure::Control)?;
        self.commit_locked(&mut state, now)
    }

    /// Apply the one commit rule under the held lock and sampled time.
    ///
    /// # Errors
    /// Refuses the same mandatory phase/time rule before mutating the held state.
    fn commit_locked(&self, state: &mut Phase, now: Instant) -> Result<(), Failure> {
        self.remaining_locked(*state, IoPhase::Exchange, now)?;
        *state = Phase::Committed;
        Ok(())
    }

    /// Close new publication, explicitly distinguishing a stop after commit.
    ///
    /// # Errors
    /// Refuses a poisoned control lock; an existing commit remains explicitly distinguished
    /// from cancellation.
    fn cancel(&self) -> Result<CancelOutcome, Failure> {
        let mut state = self.phase.lock().map_err(|_| Failure::Control)?;
        match *state {
            Phase::Open => {
                *state = Phase::Cancelled;
                Ok(CancelOutcome::Cancelled)
            }
            Phase::Cancelled => Ok(CancelOutcome::AlreadyCancelled),
            Phase::Committed => Ok(CancelOutcome::AlreadyCommitted),
        }
    }
}

/// Inert owned negotiation result; this type proves no successful TLS operation.
struct Reply {
    /// Opaque initial cookies, copied only within the count/aggregate limits.
    cookies: Vec<Zeroizing<Vec<u8>>>,
    /// Prospective admitted UDP endpoint; no UDP operation occurs here.
    udp: SocketAddr,
    /// Literal protocol/AEAD/direction metadata for the actual C2S exporter.
    c2s_context: NtsKeExporterContext,
    /// Separate direction metadata for the actual S2C exporter.
    s2c_context: NtsKeExporterContext,
}

/// Encode the actual sole offer into exactly sixteen caller-owned bytes.
///
/// # Errors
/// Propagates rejection of fixed limits or the actual request encoder; no unchecked offer
/// is returned.
fn encode_offer(storage: &mut [u8; 16]) -> Result<NtsKeNtpv4Offer<'_>, Failure> {
    let limits = NtsKeFrameLimits::new(16, 3).map_err(|_| Failure::Limits)?;
    encode_nts_ke_ntpv4_request(&[15], storage, limits).map_err(Failure::Offer)
}

/// Admit hints only against this invocation's existing identity-to-peer mapping.
///
/// # Errors
/// Refuses an unmapped server hint or explicit zero port; returns no alternative address or
/// DNS effect.
fn endpoint(
    config: &Config,
    server: Option<&str>,
    port: Option<u16>,
) -> Result<SocketAddr, Failure> {
    if let Some(server) = server {
        let admitted = match server.parse::<IpAddr>() {
            Ok(address) => address == config.peer.ip(),
            Err(_) => server.eq_ignore_ascii_case(&config.identity),
        };
        if !admitted {
            return Err(Failure::Endpoint);
        }
    }
    let port = port.unwrap_or(123);
    if port == 0 {
        return Err(Failure::Endpoint);
    }
    Ok(SocketAddr::new(config.peer.ip(), port))
}

/// Copy at most eight cookies with checked aggregate size and zeroizing owners.
///
/// # Errors
/// Refuses a ninth cookie, aggregate overflow/excess or allocation failure; partial copies
/// are wiped on ordinary drop.
fn copy_cookies<'a>(
    cookies: impl Iterator<Item = &'a [u8]>,
) -> Result<Vec<Zeroizing<Vec<u8>>>, Failure> {
    let mut owned = Vec::new();
    owned
        .try_reserve_exact(COOKIE_LIMIT)
        .map_err(|_| Failure::Allocation)?;
    let mut total = 0_usize;
    for cookie in cookies {
        if owned.len() == COOKIE_LIMIT {
            return Err(Failure::Cookies);
        }
        total = total.checked_add(cookie.len()).ok_or(Failure::Cookies)?;
        if total > COOKIE_BYTES_LIMIT {
            return Err(Failure::Cookies);
        }
        let mut copy = Zeroizing::new(Vec::new());
        copy.try_reserve_exact(cookie.len())
            .map_err(|_| Failure::Allocation)?;
        copy.extend_from_slice(cookie);
        owned.push(copy);
    }
    Ok(owned)
}

/// Read one real response without conflating EOM with EOF or reading past EOM.
///
/// The reader owns live time/cancellation admission. Every I/O error, including
/// interruption, refuses without retry. This helper never creates TLS authority.
///
/// # Errors
/// Refuses allocation, limits, read/interruption, incomplete or invalid framing,
/// negotiation, cookie-copy and endpoint failures.
fn read_response(
    reader: &mut impl Read,
    config: &Config,
    offer: &NtsKeNtpv4Offer<'_>,
) -> Result<Reply, Failure> {
    let mut storage = Zeroizing::new(Vec::new());
    storage
        .try_reserve_exact(RESPONSE_LIMIT)
        .map_err(|_| Failure::Allocation)?;
    storage.resize(RESPONSE_LIMIT, 0);
    let mut scratch = Zeroizing::new([0_u8; SCRATCH_LIMIT]);
    let limits =
        NtsKeFrameLimits::new(RESPONSE_LIMIT, RECORD_LIMIT).map_err(|_| Failure::Limits)?;
    let mut accumulator = NtsKeAccumulator::new(&mut storage, limits).map_err(Failure::Frame)?;
    loop {
        let count = reader
            .read(scratch.as_mut())
            .map_err(|error| Failure::Response(error.kind()))?;
        if count == 0 {
            break;
        }
        let bytes = scratch.get(..count).ok_or(Failure::Control)?;
        if accumulator.push(bytes).map_err(Failure::Frame)? == NtsKeProgress::EndOfMessage {
            break;
        }
    }
    let message = accumulator.finish().map_err(Failure::Frame)?;
    let negotiation = negotiate_nts_ke_ntpv4(offer, message).map_err(Failure::Negotiation)?;
    Ok(Reply {
        cookies: copy_cookies(negotiation.cookies())?,
        udp: endpoint(config, negotiation.server_hint(), negotiation.port_hint())?,
        c2s_context: negotiation.exporter_context(NtsKeExporterDirection::ClientToServer),
        s2c_context: negotiation.exporter_context(NtsKeExporterDirection::ServerToClient),
    })
}

/// Payload-free historical completion after all secret/native owners drop.
struct Summary {
    /// Actual bounded initial cookie count.
    cookies: usize,
    /// Actual aggregate opaque cookie payload bytes.
    cookie_bytes: usize,
    /// Complete untransmitted laboratory request length after final commit.
    packet_bytes: usize,
}

/// Primary operation failure and a separately retained cancellation failure.
#[cfg(windows)]
struct Failed {
    /// Original operation failure, never replaced by cleanup.
    primary: Failure,
    /// Secondary failure flushing TLS closure after a primary operation failed.
    cleanup: Option<Failure>,
    /// A separate failure closing publication admission after the operation.
    admission_cleanup: Option<Failure>,
}

#[cfg(windows)]
impl From<Failure> for Failed {
    /// Retain one original failure before any additional cleanup was attempted.
    fn from(primary: Failure) -> Self {
        Self {
            primary,
            cleanup: None,
            admission_cleanup: None,
        }
    }
}

/// Drive the sole Windows job and close admission after any primary failure.
#[cfg(windows)]
///
/// # Errors
/// Preserves the original setup/operation failure plus separate TLS and admission-cleanup
/// failures; returns no success on rejection.
fn run(config: Config) -> Result<Summary, Failed> {
    let control = Control::new(Instant::now()).map_err(Failed::from)?;
    let result = transport::exchange(&config, &control);
    // Consume this one-shot input before reporting; it cannot start a second job
    // after the process-global CNG shutdown owner has finished.
    drop(config);
    match result {
        Ok(summary) => Ok(summary),
        Err(mut failure) => {
            failure.admission_cleanup = control.cancel().err();
            Err(failure)
        }
    }
}

/// Project only closed, bounded failure labels; payload-bearing errors stay out.
#[cfg(windows)]
fn failure_labels(failure: Failure) -> (&'static str, &'static str) {
    match failure {
        Failure::Arguments => ("admission", "arguments"),
        Failure::Allocation => ("resources", "allocation"),
        Failure::Deadline => ("currentness", "deadline"),
        Failure::Cancelled => ("currentness", "cancelled"),
        Failure::Control => ("currentness", "control"),
        Failure::Offer(_) | Failure::Limits => ("request", "parameters"),
        Failure::Response(kind) => ("response", io_label(kind)),
        Failure::Frame(_) => ("response", "framing"),
        Failure::Negotiation(_) => ("response", "negotiation"),
        Failure::Cookies => ("response", "cookie-budget"),
        Failure::Entropy => ("request-sealing", "entropy"),
        Failure::Packet(_) => ("request-sealing", "encoding"),
        Failure::Endpoint => ("response", "endpoint"),
        Failure::Connect(kind) => ("connect", io_label(kind)),
        Failure::Peer => ("connect", "peer-mismatch"),
        Failure::TlsSetup => ("tls", "setup"),
        Failure::Handshake(kind) => ("handshake", io_label(kind)),
        Failure::TlsProfile => ("handshake", "profile"),
        Failure::Request(kind) => ("request", io_label(kind)),
        Failure::Exporter => ("exporter", "export"),
        Failure::Shutdown(kind) => ("shutdown", io_label(kind)),
    }
}

/// Classify a typed I/O cause without exposing its original error or source.
#[cfg(windows)]
fn io_label(kind: io::ErrorKind) -> &'static str {
    match kind {
        io::ErrorKind::TimedOut => "timeout",
        io::ErrorKind::Interrupted => "interrupted",
        io::ErrorKind::ConnectionRefused => "refused",
        io::ErrorKind::UnexpectedEof => "unexpected-eof",
        io::ErrorKind::InvalidData => "invalid-data",
        _ => "io",
    }
}

/// Attempt the bounded failure report while preserving its original exit class.
#[cfg(windows)]
fn report_failure(failure: &Failed, code: std::process::ExitCode) -> std::process::ExitCode {
    use std::io::Write;
    let (stage, cause) = failure_labels(failure.primary);
    let (cleanup_stage, cleanup_cause) = failure.cleanup.map_or(("none", "none"), failure_labels);
    let (admission_stage, admission_cause) = failure
        .admission_cleanup
        .map_or(("none", "none"), failure_labels);
    // Diagnostic failure cannot replace the primary operation failure/exit class.
    let _diagnostic_result = writeln!(
        io::stderr().lock(),
        "stage={stage} cause={cause} cleanup_stage={cleanup_stage} cleanup_cause={cleanup_cause} admission_stage={admission_stage} admission_cause={admission_cause}"
    );
    code
}

/// Admit exactly two bounded CLI arguments, then run the one Windows job.
///
/// Only safe historical counts survive the job. Output occurs after all TLS,
/// secret and shutdown owners have returned; native cleanup remains unqualified.
#[cfg(windows)]
pub(super) fn entry(mut arguments: impl Iterator<Item = OsString>) -> std::process::ExitCode {
    use std::io::Write;
    use std::process::ExitCode;
    let _program = arguments.next();
    let (Some(peer), Some(identity), None) = (arguments.next(), arguments.next(), arguments.next())
    else {
        return report_failure(&Failed::from(Failure::Arguments), ExitCode::from(2));
    };
    let config = match Config::new(peer, identity) {
        Ok(config) => config,
        Err(primary) => return report_failure(&Failed::from(primary), ExitCode::from(2)),
    };
    match run(config) {
        Ok(summary) => {
            let mut output = io::stdout().lock();
            if writeln!(output, "scope=laboratory-ke completed=1 cookies={} cookie_bytes={} sealed_packet_bytes={} transmitted=0 native_cleanup_qualified=0", summary.cookies, summary.cookie_bytes, summary.packet_bytes).is_err() || output.flush().is_err() {
                return ExitCode::FAILURE;
            }
            ExitCode::SUCCESS
        }
        Err(failure) => report_failure(&failure, ExitCode::FAILURE),
    }
}

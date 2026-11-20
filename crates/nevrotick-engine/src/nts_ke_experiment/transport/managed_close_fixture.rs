// SPDX-License-Identifier: GPL-2.0-only
//! Native loopback controls for managed KE retirement and the joined Protected report.

use std::env;
use std::error::Error;
use std::io::{self, Read, Write};
use std::net::{Ipv4Addr, Shutdown, SocketAddr, SocketAddrV4, TcpListener, TcpStream, UdpSocket};
use std::sync::OnceLock;
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::thread;
use std::time::{Duration, Instant};

use super::super::{Config, PROCESS_WORKER, job::Observation, run_with_power_observer};
use crate::{
    KePreviewTarget, MeasurementRefusal, SourceAddressSet, SourceConfigurationSnapshot,
    SourceHandle, SourcePolicyAuthority, SourcePolicyError, SourcePolicyLimits,
    SourcePolicySnapshot, SourceProtectedProbeFailure, SourceProtectedProbeReport, SourceScale,
};

/// This process owns one probe and one bounded peer; no production deadline changes.
const PEER_WINDOW: Duration = Duration::from_secs(10);
/// Poll only the fixture's owned nonblocking listener and accepted socket.
const PEER_POLL: Duration = Duration::from_millis(1);
/// Fixed maximum TLS plaintext record body.
const TLS_RECORD_BYTES: usize = 16_384;
/// The one exact loopback connection used in the separately selected test process.
static READ_STARTED: OnceLock<(SocketAddr, SyncSender<()>)> = OnceLock::new();

/// Errors can cross the bounded peer thread without discarding the original cause.
type FixtureResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

/// Notify the owned peer after both native-read admission checks have passed.
/// The native socket read follows immediately and completes on the peer's half-close.
///
/// # Errors
/// Refuse a full or disconnected fixture channel at the selected endpoint.
pub(super) fn signal_native_read(socket: &TcpStream) -> io::Result<()> {
    if let Some((peer, signal)) = READ_STARTED.get()
        && socket.peer_addr().is_ok_and(|address| address == *peer)
    {
        signal
            .try_send(())
            .map_err(|_| io::Error::other("managed fixture read notification failed"))?;
    }
    Ok(())
}

/// Read at most the supplied bytes against the original fixture deadline.
///
/// # Errors
/// Return the native error or a timeout, preserving short EOF as a short read.
fn read_peer(
    stream: &mut TcpStream,
    mut output: &mut [u8],
    deadline: Instant,
) -> io::Result<usize> {
    let capacity: usize = output.len();
    while !output.is_empty() {
        if Instant::now() >= deadline {
            return Err(io::ErrorKind::TimedOut.into());
        }
        match stream.read(output) {
            Ok(0) => break,
            Ok(count) => output = &mut output[count..],
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => thread::sleep(PEER_POLL),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(error),
        }
    }
    Ok(capacity - output.len())
}

/// Accept the single loopback connection from the owned listener.
///
/// # Errors
/// Refuse timeout, unexpected peer or socket setup failure.
fn accept_peer(listener: &TcpListener, deadline: Instant) -> io::Result<TcpStream> {
    while Instant::now() < deadline {
        match listener.accept() {
            Ok((stream, SocketAddr::V4(peer))) if peer.ip().is_loopback() => {
                stream.set_nonblocking(true)?;
                return Ok(stream);
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => thread::sleep(PEER_POLL),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(error),
            Ok(_) => return Err(io::ErrorKind::InvalidData.into()),
        }
    }
    Err(io::ErrorKind::TimedOut.into())
}

/// Confirm the complete first TLS record from the live client before replacing its revision.
///
/// # Errors
/// Refuse an incomplete or invalid `ClientHello` or a native read error.
fn read_client_hello(stream: &mut TcpStream, deadline: Instant) -> io::Result<()> {
    let mut header: [u8; 5] = [0; 5];
    if read_peer(stream, &mut header, deadline)? != header.len() {
        return Err(io::ErrorKind::UnexpectedEof.into());
    }
    let [22, 3, 1..=3, high, low]: [u8; 5] = header else {
        return Err(io::ErrorKind::InvalidData.into());
    };
    let length: usize = usize::from(u16::from_be_bytes([high, low]));
    let mut storage: [u8; TLS_RECORD_BYTES] = [0; TLS_RECORD_BYTES];
    let payload: &mut [u8] = storage
        .get_mut(..length)
        .ok_or(io::ErrorKind::InvalidData)?;
    if read_peer(stream, payload, deadline)? != length {
        return Err(io::ErrorKind::UnexpectedEof.into());
    }
    let [1, high, middle, low, hello @ ..] = payload else {
        return Err(io::ErrorKind::InvalidData.into());
    };
    let body_length: usize =
        (usize::from(*high) << 16) | (usize::from(*middle) << 8) | usize::from(*low);
    if body_length != hello.len() || body_length < 34 {
        return Err(io::ErrorKind::InvalidData.into());
    }
    Ok(())
}

/// Replace the real installed definition after I/O admission, then release the read.
///
/// # Errors
/// Refuse incomplete peer I/O, a lost native read notification or a failed replacement.
/// # Panics
/// Fail if the replacement did not issue a new revision or the client sends more bytes.
fn replace_and_retire_peer(
    listener: TcpListener,
    deadline: Instant,
    read_started: &Receiver<()>,
    mut authority: SourcePolicyAuthority,
    expected: &SourceConfigurationSnapshot,
    target: KePreviewTarget,
) -> FixtureResult<()> {
    let mut stream: TcpStream = accept_peer(&listener, deadline)?;
    drop(listener);
    read_client_hello(&mut stream, deadline)?;
    read_started.recv_timeout(deadline.saturating_duration_since(Instant::now()))?;
    if Instant::now() >= deadline {
        return Err(io::Error::from(io::ErrorKind::TimedOut).into());
    }
    let replacement: SourceConfigurationSnapshot = authority.replace_ke_preview_configuration(
        expected,
        expected.addresses().clone(),
        target,
    )?;
    assert!(!replacement.same_configuration(expected));
    stream.shutdown(Shutdown::Write)?;
    let mut unexpected: [u8; 1] = [0];
    assert_eq!(read_peer(&mut stream, &mut unexpected, deadline)?, 0);
    Ok(())
}

/// Observe the real managed failure through its original worker, power and report owners.
/// Run this one ignored case in its own Windows process for permanent admission custody.
///
/// # Errors
/// Preserve test setup, the joined peer and an unexpected successful report.
/// # Panics
/// Fail for changed primary error, fabricated cleanup, unretired ownership or any UDP.
#[test]
#[ignore = "requires a dedicated Windows process for native managed KE retirement"]
fn managed_protected_native_read_keeps_stale_revision_as_its_only_tls_failure() -> FixtureResult<()>
{
    let listener: TcpListener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))?;
    listener.set_nonblocking(true)?;
    let ke_peer: SocketAddr = listener.local_addr()?;
    let udp: UdpSocket = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))?;
    udp.set_nonblocking(true)?;
    let target: KePreviewTarget = KePreviewTarget::new(ke_peer, "localhost")?;
    let limits: SourcePolicyLimits =
        SourcePolicyLimits::new(1, 1, Duration::from_millis(1), Duration::from_millis(4))?;
    let mut authority: SourcePolicyAuthority = SourcePolicyAuthority::install(limits)?;
    let source: SourceHandle = authority.source(0)?;
    let addresses: SourceAddressSet = SourceAddressSet::new(&[udp.local_addr()?])?;
    let expected: SourceConfigurationSnapshot =
        authority.install_ke_preview_configuration(&source, addresses, target.clone())?;
    let config: Config =
        Config::from_managed_protected(authority.authorize_protected_ke(&expected)?);
    let (signal, read_started): (SyncSender<()>, Receiver<()>) = sync_channel(1);
    READ_STARTED
        .set((ke_peer, signal))
        .map_err(|_| io::Error::other("managed fixture read witness already installed"))?;
    let deadline: Instant = Instant::now()
        .checked_add(PEER_WINDOW)
        .ok_or_else(|| io::Error::from(io::ErrorKind::TimedOut))?;
    let peer: thread::JoinHandle<FixtureResult<()>> = thread::Builder::new().spawn(move || {
        replace_and_retire_peer(
            listener,
            deadline,
            &read_started,
            authority,
            &expected,
            target,
        )
    })?;

    let result: Result<SourceProtectedProbeReport, SourceProtectedProbeFailure> =
        SourceProtectedProbeReport::from_observed(run_with_power_observer(config));
    peer.join()
        .map_err(|_| io::Error::other("managed fixture peer panicked"))??;
    let failure: SourceProtectedProbeFailure = result
        .err()
        .ok_or_else(|| io::Error::other("stale managed KE unexpectedly completed"))?;
    assert_eq!(failure.labels(), ("source-policy", "refused"));
    assert_eq!(failure.primary_labels(), Some(("source-policy", "refused")));
    assert_eq!(
        failure.source_policy_error(),
        Some(SourcePolicyError::ConfigurationChanged)
    );
    assert_eq!(failure.cleanup_labels(), None);
    assert_eq!(failure.admission_cleanup_labels(), None);
    assert!(!failure.power_stop_failed());
    assert_eq!(failure.power_unregistration_error(), None);
    assert!(failure.retained_configuration().is_none());
    assert_eq!(
        PROCESS_WORKER.observer().poll_until(deadline),
        Observation::Taken
    );
    assert_eq!(PROCESS_WORKER.sealed_count(), Ok(0));
    assert!(matches!(
        udp.recv_from(&mut [0; 1]),
        Err(error) if error.kind() == io::ErrorKind::WouldBlock
    ));
    Ok(())
}

/// Exact synthetic certificate identity reused from the admitted offline KE peer.
const MANAGED_PEER_IDENTITY: &str = "ke-a7e40ec4eade477dbaba6ca9e9f975a4.invalid";
/// This fixture's initial source and endpoint send intervals.
const INITIAL_SEND_INTERVAL: Duration = Duration::from_millis(1);

/// Read one numeric endpoint whose fixture and owning PID are pinned by the collector.
///
/// # Errors
/// Refuse missing, oversized, non-Unicode, non-IPv4-loopback or zero-port input.
fn managed_peer_endpoint(variable: &str) -> io::Result<SocketAddr> {
    let raw: std::ffi::OsString =
        env::var_os(variable).ok_or_else(|| io::Error::other("missing managed peer endpoint"))?;
    if raw.len() > "127.0.0.1:65535".len() {
        return Err(io::ErrorKind::InvalidInput.into());
    }
    let address: SocketAddrV4 = raw
        .to_str()
        .ok_or(io::ErrorKind::InvalidInput)?
        .parse()
        .map_err(|_| io::ErrorKind::InvalidInput)?;
    if *address.ip() != Ipv4Addr::LOCALHOST || address.port() == 0 {
        return Err(io::ErrorKind::InvalidInput.into());
    }
    Ok(SocketAddr::V4(address))
}

/// Recover the first send's guarded monotonic reservation from both actual policy records.
///
/// # Errors
/// Refuse a missing source floor or an unrepresentable reservation instant.
/// # Panics
/// Fail if either record took other than one send debit or changed restrictions.
fn verify_managed_send_debit(policy: &SourcePolicySnapshot) -> io::Result<Instant> {
    let floor: Instant = policy
        .source()
        .floor()
        .ok_or_else(|| io::Error::other("missing managed send floor"))?;
    for record in [policy.source(), policy.endpoint()] {
        assert_eq!(record.revision(), 2);
        assert_eq!(record.interval(), INITIAL_SEND_INTERVAL);
        assert_eq!(record.floor(), Some(floor));
        assert!(!record.refused());
        assert!(!record.unavailable());
    }
    floor
        .checked_sub(INITIAL_SEND_INTERVAL)
        .ok_or_else(|| io::Error::other("managed send floor precedes its interval"))
}

/// Exercise the public managed probe with one authenticated native exchange.
///
/// The collector owns the fixed one-leg peer and synthetic trust, verifies both
/// listener owners and joins both processes. This case uses the permanent Engine
/// slot and must run alone in a fresh Windows process.
///
/// # Errors
/// Preserve endpoint, source setup, public probe, history and output refusals.
/// # Panics
/// Fail if provenance, timing, one-send policy, or actual retirement differs.
#[test]
#[ignore = "requires the owned offline Windows one-leg TLS and UDP peer"]
fn managed_protected_public_probe_joins_one_authenticated_reply() -> Result<(), Box<dyn Error>> {
    let ke_peer: SocketAddr = managed_peer_endpoint("NV_NTS_MANAGED_KE_PEER")?;
    let udp_peer: SocketAddr = managed_peer_endpoint("NV_NTS_MANAGED_UDP_PEER")?;
    let target: KePreviewTarget = KePreviewTarget::new(ke_peer, MANAGED_PEER_IDENTITY)?;
    let limits: SourcePolicyLimits =
        SourcePolicyLimits::new(1, 1, INITIAL_SEND_INTERVAL, Duration::from_millis(4))?;
    let mut authority: SourcePolicyAuthority = SourcePolicyAuthority::install(limits)?;
    let source: SourceHandle = authority.source(0)?;
    let addresses: SourceAddressSet = SourceAddressSet::new(&[udp_peer])?;
    let expected: SourceConfigurationSnapshot =
        authority.install_ke_preview_configuration(&source, addresses, target)?;

    let started_at: Instant = Instant::now();
    let report: SourceProtectedProbeReport = authority.probe_nts_protected(&expected)?;
    let returned_at: Instant = Instant::now();

    assert!(report.configuration().same_configuration(&expected));
    assert_eq!(
        report.configuration().declared_scale(),
        SourceScale::Unknown
    );
    assert_eq!(report.packet_bytes(), 252);
    assert_eq!(report.leap_indicator(), 0);
    assert!(matches!(
        report.measurement(),
        Err(MeasurementRefusal::Unavailable)
    ));

    let committed_at: Instant = report.committed_at();
    let delay: Duration = report
        .receive_to_commit_delay()
        .ok_or_else(|| io::Error::other("missing managed receive-to-commit delay"))?;
    let receive_trailing_at: Instant = committed_at
        .checked_sub(delay)
        .ok_or_else(|| io::Error::other("invalid managed receive-to-commit delay"))?;
    let policy: SourcePolicySnapshot = source.policy_snapshot(udp_peer)?;
    let reserved_at: Instant = verify_managed_send_debit(&policy)?;
    assert!((started_at..=receive_trailing_at).contains(&reserved_at));
    assert!((started_at..=committed_at).contains(&receive_trailing_at));
    assert!(committed_at <= returned_at);

    let [minimum, maximum]: [Duration; 2] = report
        .application_wall_elapsed_bounds()
        .ok_or_else(|| io::Error::other("missing original application elapsed bounds"))?;
    let invocation_receive_span: Duration = receive_trailing_at
        .checked_duration_since(started_at)
        .ok_or_else(|| io::Error::other("receive precedes the invocation"))?;
    assert!(minimum <= maximum && maximum <= invocation_receive_span);

    assert_eq!(report.power_unregistration_error(), None);
    let inspect_until: Instant = Instant::now()
        .checked_add(Duration::from_secs(1))
        .ok_or_else(|| io::Error::from(io::ErrorKind::TimedOut))?;
    assert_eq!(
        PROCESS_WORKER.observer().poll_until(inspect_until),
        Observation::Taken
    );
    assert_eq!(PROCESS_WORKER.sealed_count(), Ok(0));

    writeln!(
        io::stdout().lock(),
        "scope=managed-protected-native eligible=0 fresh_sync=0 completed=1 packet_bytes={} li={} configuration=original receive_to_commit=present mapping=unavailable source_debits=1 endpoint_debits=1 worker_joined=1 observer_closed=1",
        report.packet_bytes(),
        report.leap_indicator()
    )?;
    Ok(())
}

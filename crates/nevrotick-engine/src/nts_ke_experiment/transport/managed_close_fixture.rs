// SPDX-License-Identifier: GPL-2.0-only
//! Native loopback control for the first managed transport failure at TLS retirement.

use std::error::Error;
use std::io::{self, Read};
use std::net::{Ipv4Addr, Shutdown, SocketAddr, SocketAddrV4, TcpListener, TcpStream, UdpSocket};
use std::sync::OnceLock;
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::thread;
use std::time::{Duration, Instant};

use super::super::{Config, PROCESS_WORKER, job::Observation, run_with_power_observer};
use crate::{
    KePreviewTarget, SourceAddressSet, SourceConfigurationSnapshot, SourcePolicyAuthority,
    SourcePolicyError, SourcePolicyLimits, SourceProtectedProbeFailure, SourceProtectedProbeReport,
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
    let source: crate::SourceHandle = authority.source(0)?;
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

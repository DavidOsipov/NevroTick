// SPDX-License-Identifier: GPL-2.0-only
//! The sole real Windows TLS/KE producer, confined to one synchronous job.
//!
//! No authenticated association escapes: strict same-connection keys/cookies
//! protect one untransmitted laboratory request before secret/native destruction.

use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use ntp_wire::{NtsKeAccumulatorError, NtsKeFrameError};
use rustls::time_provider::TimeProvider;
use rustls::{CipherSuite, ClientConfig, ClientConnection, StreamOwned, SupportedCipherSuite};
use rustls_pki_types::UnixTime;
use zeroize::Zeroizing;

use super::profile::StrictProfile;
use super::transport_fault::{self, SetupStage, TlsIoStage, TransportIo};
use super::{AssociationCandidate, AssociationOwner, ProviderScope, request::DirectionalKeys};
use super::{Config, Control, Failed, Failure, SCRATCH_LIMIT, encode_offer, read_response};

/// The actual numeric Windows socket and its original KE I/O currentness owner.
struct DeadlineStream<'a> {
    /// Sole owned connection; destruction closes it without another job.
    socket: TcpStream,
    /// Owns all pre/post-call observations and first-failure retention.
    admission: TransportIo<'a>,
}

impl Read for DeadlineStream<'_> {
    /// Apply the original handshake/exchange time and current definition for one read.
    ///
    /// # Errors
    /// Preserve timeout-setting, read or pre/post-call currentness failure.
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        self.admission.transfer(
            &mut self.socket,
            |socket: &mut TcpStream, timeout: Duration| socket.set_read_timeout(Some(timeout)),
            |socket: &mut TcpStream| socket.read(output),
        )
    }
}

impl Write for DeadlineStream<'_> {
    /// Apply the original handshake/exchange time and current definition for one write.
    ///
    /// # Errors
    /// Preserve timeout-setting, write or pre/post-call currentness failure.
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        self.admission.transfer(
            &mut self.socket,
            |socket: &mut TcpStream, timeout: Duration| socket.set_write_timeout(Some(timeout)),
            |socket: &mut TcpStream| socket.write(input),
        )
    }

    /// Flush under the same current definition and original deadline.
    ///
    /// # Errors
    /// Preserve timeout-setting, flush or pre/post-call currentness failure.
    fn flush(&mut self) -> io::Result<()> {
        self.admission.transfer(
            &mut self.socket,
            |socket: &mut TcpStream, timeout: Duration| socket.set_write_timeout(Some(timeout)),
            Write::flush,
        )
    }
}

/// Complete ephemeral result of this actual connection, never product authority.
struct InternalCandidate<'a> {
    /// Borrowed immutable original identity/name and configured numeric endpoint.
    config: &'a Config,
    /// The same invocation's cancellation/consume-once publication owner.
    control: &'a Control,
    /// Captured actual connected peer, checked against the original destination.
    peer: SocketAddr,
    /// Prospective same-IP UDP endpoint admitted from the actual reply.
    udp: SocketAddr,
    /// Immutable original-origin expiry, never refreshed by TLS/KE progress.
    expiry: Instant,
    /// Actual strict full-handshake snapshot; no authenticated boolean exists.
    profile: StrictProfile,
    /// Selected sole offered AEAD, bound to both actual exporter contexts.
    aead: u16,
    /// Regular client-to-server exporter result, wiped on ordinary destruction.
    c2s_key: Zeroizing<[u8; 32]>,
    /// Separate regular server-to-client exporter result, likewise wiped.
    s2c_key: Zeroizing<[u8; 32]>,
    /// Opaque owned cookies from this same actual negotiated reply.
    cookies: Vec<Zeroizing<Vec<u8>>>,
}

impl<'a> InternalCandidate<'a> {
    /// Check the binding, seal one private request, then commit and wipe once.
    ///
    /// # Errors
    /// Refuse an invalid profile, endpoint, invocation/expiry/AEAD binding,
    /// cookie budget, entropy/sealing or final currentness/commit guard. Failure
    /// destroys the candidate and private packet without returning a summary.
    fn publish(self, provider: &'a ProviderScope<'a>) -> Result<AssociationOwner<'a>, Failure> {
        let Self {
            config,
            control,
            peer,
            udp,
            expiry,
            profile,
            aead,
            c2s_key,
            s2c_key,
            cookies,
        } = self;
        if peer != config.peer || udp.ip() != peer.ip() || udp.port() == 0 {
            return Err(Failure::Endpoint);
        }
        if config.identity.is_empty() || expiry != control.expiry || aead != 15 {
            return Err(Failure::Control);
        }
        let candidate = AssociationCandidate::new(
            config,
            control,
            provider,
            udp,
            profile,
            DirectionalKeys::new(c2s_key, s2c_key),
            cookies,
        );
        AssociationOwner::receive(config, control, provider, candidate)
    }
}

/// Order exactly the three admitted TLS 1.3 suites without adding a provider.
fn suite_priority(suite: CipherSuite) -> u8 {
    match suite {
        CipherSuite::TLS13_AES_128_GCM_SHA256 => 0,
        CipherSuite::TLS13_AES_256_GCM_SHA384 => 1,
        CipherSuite::TLS13_CHACHA20_POLY1305_SHA256 => 2,
        _ => 3,
    }
}

/// Read the ordinary local wall clock for the TLS certificate check.
#[derive(Debug)]
struct LocalWallTime {
    /// Deterministic callback input, absent from the ordinary production provider.
    #[cfg(test)]
    fixture_time: Option<SystemTime>,
}

impl LocalWallTime {
    /// Select ordinary local wall time for each Rustls callback.
    const fn system() -> Self {
        Self {
            #[cfg(test)]
            fixture_time: None,
        }
    }
}

impl TimeProvider for LocalWallTime {
    /// Return Unix seconds for a representable wall reading, or `None` before 1970.
    fn current_time(&self) -> Option<UnixTime> {
        #[cfg(not(test))]
        let observed: SystemTime = SystemTime::now();
        #[cfg(test)]
        let observed: SystemTime = self.fixture_time.unwrap_or_else(SystemTime::now);
        observed
            .duration_since(SystemTime::UNIX_EPOCH)
            .ok()
            .map(UnixTime::since_unix_epoch)
    }
}

/// Configure explicit CNG/TLS13 and the ordinary OS verifier, with no extra roots.
///
/// # Errors
/// Return `Failure::TlsSetup` if the provider suite set, ordinary verifier or TLS
/// builder refuses. Return `Failure::Allocation` if a bounded application-owned
/// ALPN allocation fails.
fn client_config(wall_time: LocalWallTime) -> Result<ClientConfig, Failure> {
    let mut provider: rustls::crypto::CryptoProvider = rustls_cng_crypto::default_provider();
    provider
        .cipher_suites
        .sort_unstable_by_key(|suite| suite_priority(suite.suite()));
    if !provider
        .cipher_suites
        .iter()
        .map(SupportedCipherSuite::suite)
        .eq([
            CipherSuite::TLS13_AES_128_GCM_SHA256,
            CipherSuite::TLS13_AES_256_GCM_SHA384,
            CipherSuite::TLS13_CHACHA20_POLY1305_SHA256,
        ])
    {
        return Err(Failure::TlsSetup(SetupStage::ProviderSuites));
    }
    let provider: Arc<rustls::crypto::CryptoProvider> = Arc::new(provider);
    let verifier: rustls_platform_verifier::Verifier =
        rustls_platform_verifier::Verifier::new(Arc::clone(&provider))
            .map_err(|_| Failure::TlsSetup(SetupStage::Verifier))?;
    let mut tls: ClientConfig = ClientConfig::builder_with_details(provider, Arc::new(wall_time))
        .with_protocol_versions(&[&rustls::version::TLS13])
        .map_err(|_| Failure::TlsSetup(SetupStage::ProtocolVersions))?
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(verifier))
        .with_no_client_auth();
    let mut alpn: Vec<u8> = Vec::new();
    alpn.try_reserve_exact(7).map_err(|_| Failure::Allocation)?;
    alpn.extend_from_slice(b"ntske/1");
    tls.alpn_protocols
        .try_reserve_exact(1)
        .map_err(|_| Failure::Allocation)?;
    tls.alpn_protocols.push(alpn);
    tls.resumption = rustls::client::Resumption::disabled();
    tls.enable_early_data = false;
    Ok(tls)
}

/// Construct the actual ordinary TLS connection with fresh local wall reads.
///
/// # Errors
/// Preserve configuration or client-construction refusal.
fn connection(config: &Config) -> Result<ClientConnection, Failure> {
    let tls: ClientConfig = client_config(LocalWallTime::system())?;
    let mut connection: ClientConnection =
        ClientConnection::new(Arc::new(tls), config.server_name.clone())
            .map_err(|_| Failure::TlsSetup(SetupStage::ClientConnection))?;
    // This caps Rustls outgoing application buffering, not all TLS allocations.
    connection.set_buffer_limit(Some(SCRATCH_LIMIT));
    Ok(connection)
}

/// Reject only already-buffered post-EOM plaintext; never initiate a socket read.
///
/// # Errors
/// Return the framing owner's trailing-data failure for any buffered byte, or
/// retain a known plaintext-reader error. An empty clean close or currently
/// unavailable buffered plaintext succeeds without an additional socket read.
fn reject_buffered(connection: &mut ClientConnection) -> Result<(), Failure> {
    let mut byte = Zeroizing::new([0_u8; 1]);
    match connection.reader().read(byte.as_mut()) {
        Ok(0) => Ok(()),
        Ok(_) => Err(Failure::Frame(NtsKeAccumulatorError::Frame(
            NtsKeFrameError::TrailingData,
        ))),
        Err(error) if error.kind() == io::ErrorKind::WouldBlock => Ok(()),
        Err(error) => Err(transport_fault::project_io(TlsIoStage::Response, &error)),
    }
}

/// Complete full TLS, the sole real KE request/reply, and both regular exporters.
///
/// # Errors
/// Refuse handshake/profile, actual offer/write/response, framing/negotiation,
/// cookie/endpoint, exporter or fresh currentness failure. Preserve primary TLS
/// refusal before alert failure, or restore the first hidden/retried transport
/// cause. Partial owned keys/cookies drop on refusal.
fn prepare<'a>(
    stream: &mut StreamOwned<ClientConnection, DeadlineStream<'a>>,
    peer: SocketAddr,
) -> Result<InternalCandidate<'a>, Failure> {
    let config: &'a Config = stream.sock.admission.config();
    let handshake = stream
        .conn
        .complete_io(&mut stream.sock)
        .map_err(|error| transport_fault::project_io(TlsIoStage::Handshake, &error));
    stream
        .sock
        .admission
        .resolve(handshake, Failure::Handshake)?;
    stream.sock.admission.current(Failure::Handshake)?;
    let profile = StrictProfile::capture(&stream.conn)?;
    stream.sock.admission.begin_exchange();
    let mut storage = [0_u8; 16];
    let offer = encode_offer(&mut storage)?;
    let request = stream
        .write_all(offer.request_bytes())
        .and_then(|()| stream.flush())
        .map_err(|error| transport_fault::project_io(TlsIoStage::Request, &error));
    stream.sock.admission.resolve(request, Failure::Request)?;
    let response = read_response(stream, config, &offer);
    let reply = stream.sock.admission.resolve(response, Failure::Response)?;
    stream.sock.admission.current(Failure::Response)?;
    reject_buffered(&mut stream.conn)?;
    if reply.c2s_context.context_bytes() != &[0, 0, 0, 15, 0]
        || reply.s2c_context.context_bytes() != &[0, 0, 0, 15, 1]
    {
        return Err(Failure::Exporter);
    }
    let [_, _, aead_high, aead_low, _] = *reply.c2s_context.context_bytes();
    let aead = u16::from_be_bytes([aead_high, aead_low]);
    let c2s_key = stream
        .conn
        .export_keying_material(
            Zeroizing::new([0_u8; 32]),
            reply.c2s_context.label().as_bytes(),
            Some(reply.c2s_context.context_bytes()),
        )
        .map_err(|_| Failure::Exporter)?;
    let s2c_key = stream
        .conn
        .export_keying_material(
            Zeroizing::new([0_u8; 32]),
            reply.s2c_context.label().as_bytes(),
            Some(reply.s2c_context.context_bytes()),
        )
        .map_err(|_| Failure::Exporter)?;
    stream.sock.admission.current(Failure::Response)?;
    Ok(InternalCandidate {
        config,
        control: stream.sock.admission.control(),
        peer,
        udp: reply.udp,
        expiry: stream.sock.admission.control().expiry,
        profile,
        aead,
        c2s_key,
        s2c_key,
        cookies: reply.cookies,
    })
}

/// Queue and fully drain close-notify without restarting a failed handshake.
///
/// # Errors
/// Refuse a known transport/currentness fault, TLS-write zero/failure, socket
/// flush failure, or already-buffered post-EOM plaintext/error. A
/// successful return reports only the local close-notify drain and flush.
fn close(stream: &mut StreamOwned<ClientConnection, DeadlineStream<'_>>) -> Result<(), Failure> {
    stream.sock.admission.current(Failure::Shutdown)?;
    stream.conn.send_close_notify();
    while stream.conn.wants_write() {
        let written = stream
            .conn
            .write_tls(&mut stream.sock)
            .map_err(|error| transport_fault::project_io(TlsIoStage::Shutdown, &error));
        if stream.sock.admission.resolve(written, Failure::Shutdown)? == 0 {
            return Err(Failure::Shutdown(io::ErrorKind::WriteZero));
        }
    }
    // Rustls Writer::flush only replays a stored fatal TLS error. The close
    // result describes the record drain and the underlying writer's flush.
    let flushed = stream
        .sock
        .flush()
        .map_err(|error| transport_fault::project_io(TlsIoStage::Shutdown, &error));
    stream.sock.admission.resolve(flushed, Failure::Shutdown)?;
    stream.sock.admission.current(Failure::Shutdown)?;
    reject_buffered(&mut stream.conn)
}

/// Own every TLS/secret object inside the outermost CNG shutdown lifetime.
///
/// # Errors
/// Refuse numeric connect/peer, TLS/KE preparation, local close or final candidate
/// publication failure. Preserve an independently observed close refusal beside
/// a primary preparation failure. TLS is retired before the checked owner leaves.
pub(super) fn establish<'a>(
    config: &'a Config,
    control: &'a Control,
    provider: &'a ProviderScope<'a>,
) -> Result<AssociationOwner<'a>, Failed> {
    let mut admission: TransportIo<'_> = TransportIo::new(config, control);
    let socket: TcpStream = admission.connect(|peer: SocketAddr, timeout: Duration| {
        TcpStream::connect_timeout(&peer, timeout)
    })?;
    let peer = socket
        .peer_addr()
        .map_err(|error| Failure::Connect(error.kind()))?;
    if peer != config.peer {
        return Err(Failure::Peer.into());
    }
    admission.current(Failure::Handshake)?;
    let socket: DeadlineStream<'_> = DeadlineStream { socket, admission };
    let mut stream: StreamOwned<ClientConnection, DeadlineStream<'_>> =
        StreamOwned::new(connection(config)?, socket);
    let candidate: Result<InternalCandidate<'a>, Failure> = prepare(&mut stream, peer);
    let closed: Result<(), Failure> = close(&mut stream);
    drop(stream);
    // Retain an independently observed close refusal alongside its primary.
    match (candidate, closed) {
        (Err(primary), Err(cleanup)) => Err(Failed {
            primary,
            cleanup: Some(cleanup),
            admission_cleanup: None,
        }),
        (Err(primary), Ok(())) | (Ok(_), Err(primary)) => Err(primary.into()),
        (Ok(candidate), Ok(())) => candidate.publish(provider).map_err(Failed::from),
    }
}

#[cfg(test)]
mod tests {
    use super::{Failure, LocalWallTime, client_config};
    use rustls::ClientConfig;
    use rustls_pki_types::UnixTime;
    use std::time::{Duration, SystemTime};

    /// Reach the configured Rustls callback without changing the OS clock.
    ///
    /// # Errors
    /// Preserve any unexpected ordinary TLS configuration refusal.
    /// # Panics
    /// Fail if the callback accepts a pre-epoch date, refuses either healthy
    /// neighbor or changes the whole-second conversion.
    #[test]
    fn configured_local_wall_time_checks_the_unix_epoch() -> Result<(), Failure> {
        let epoch: SystemTime = SystemTime::UNIX_EPOCH;
        // Windows SystemTime represents multiples of 100 ns.
        let cases: [(SystemTime, Option<u64>); 3] = [
            (epoch - Duration::from_nanos(100), None),
            (epoch, Some(0)),
            (epoch + Duration::new(1, 100), Some(1)),
        ];
        for (observed, expected_seconds) in cases {
            let config: ClientConfig = client_config(LocalWallTime {
                fixture_time: Some(observed),
            })?;
            let actual_seconds: Option<u64> = config
                .time_provider
                .current_time()
                .as_ref()
                .map(UnixTime::as_secs);
            assert_eq!(actual_seconds, expected_seconds);
        }
        Ok(())
    }
}

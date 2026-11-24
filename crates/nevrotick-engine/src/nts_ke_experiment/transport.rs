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

use super::profile::{CertificateTimeProvenance, StrictProfile};
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
            |socket: &mut TcpStream| {
                #[cfg(test)]
                managed_close_fixture::signal_native_read(socket)?;
                socket.read(output)
            },
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

/// The selected verifier's wall input, issued only with its configured connection.
pub(super) struct ConfiguredCertificateTime {
    /// Code-owned classification of the installed provider, without its timestamp.
    provenance: CertificateTimeProvenance,
}

impl ConfiguredCertificateTime {
    /// Transfer the original configuration into the completed strict profile.
    pub(super) const fn into_provenance(self) -> CertificateTimeProvenance {
        self.provenance
    }
}

/// Construct the connection together with the exact wall provider classification.
/// The private preparation path consumes them together after the handshake.
///
/// # Errors
/// Preserve configuration or client-construction refusal.
fn connection(
    config: &Config,
    wall_time: LocalWallTime,
) -> Result<(ClientConnection, ConfiguredCertificateTime), Failure> {
    #[cfg(not(test))]
    let provenance: CertificateTimeProvenance =
        CertificateTimeProvenance::WindowsLaboratoryLocalWall;
    #[cfg(test)]
    let provenance: CertificateTimeProvenance = if wall_time.fixture_time.is_some() {
        CertificateTimeProvenance::WindowsLaboratoryFixtureWall
    } else {
        CertificateTimeProvenance::WindowsLaboratoryLocalWall
    };
    let tls: ClientConfig = client_config(wall_time)?;
    let mut connection: ClientConnection =
        ClientConnection::new(Arc::new(tls), config.server_name.clone())
            .map_err(|_| Failure::TlsSetup(SetupStage::ClientConnection))?;
    // This caps Rustls outgoing application buffering, not all TLS allocations.
    connection.set_buffer_limit(Some(SCRATCH_LIMIT));
    Ok((connection, ConfiguredCertificateTime { provenance }))
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
    certificate_time: ConfiguredCertificateTime,
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
    let profile: StrictProfile = StrictProfile::capture(&stream.conn, certificate_time)?;
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
    establish_with_time(config, control, provider, LocalWallTime::system())
}

/// Use one local time provider through the same socket, verifier and retirement.
///
/// # Errors
/// Preserve the original establishment and independently observed cleanup faults.
fn establish_with_time<'a>(
    config: &'a Config,
    control: &'a Control,
    provider: &'a ProviderScope<'a>,
    wall_time: LocalWallTime,
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
    let (connection, certificate_time): (ClientConnection, ConfiguredCertificateTime) =
        connection(config, wall_time)?;
    let mut stream: StreamOwned<ClientConnection, DeadlineStream<'_>> =
        StreamOwned::new(connection, socket);
    let candidate: Result<InternalCandidate<'a>, Failure> =
        prepare(&mut stream, peer, certificate_time);
    let retained_fault: Option<Failure> = stream
        .sock
        .admission
        .resolve(Ok(()), Failure::Shutdown)
        .err();
    let close_failure: Option<Failure> = match (&candidate, retained_fault) {
        (_, None) => close(&mut stream).err(),
        // A typed TLS error can take precedence over a separate alert I/O fault.
        (Err(Failure::Tls(_)) | Ok(_), Some(failure)) => Some(failure),
        // Preparation already reports the fault that prevents close from starting.
        (Err(_), Some(_)) => None,
    };
    drop(stream);
    match (candidate, close_failure) {
        (Err(primary), cleanup) => Err(Failed {
            primary,
            cleanup,
            admission_cleanup: None,
        }),
        (Ok(_), Some(primary)) => Err(primary.into()),
        (Ok(candidate), None) => candidate.publish(provider).map_err(Failed::from),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CertificateTimeProvenance, Config, ConfiguredCertificateTime, Failure, LocalWallTime,
        client_config, connection,
    };
    use rustls::{ClientConfig, ClientConnection};
    use rustls_pki_types::UnixTime;
    use std::ffi::OsString;
    use std::time::{Duration, SystemTime};

    /// Check the wall-input classification issued with each new TLS client.
    ///
    /// # Errors
    /// Preserve fixture construction or TLS setup refusal.
    /// # Panics
    /// Fail if either new client has completed a handshake or carries the wrong
    /// wall-input classification.
    #[test]
    fn connection_preserves_certificate_time_input_kind() -> Result<(), Failure> {
        let config: Config = Config::new(
            OsString::from("127.0.0.1:4460"),
            OsString::from("Time.Example"),
        )?;
        let fixed_wall: SystemTime = SystemTime::UNIX_EPOCH
            .checked_add(Duration::from_secs(1_700_000_000))
            .ok_or(Failure::Control)?;
        let cases: [(LocalWallTime, CertificateTimeProvenance); 2] = [
            (
                LocalWallTime::system(),
                CertificateTimeProvenance::WindowsLaboratoryLocalWall,
            ),
            (
                LocalWallTime {
                    fixture_time: Some(fixed_wall),
                },
                CertificateTimeProvenance::WindowsLaboratoryFixtureWall,
            ),
        ];
        for (wall_time, expected) in cases {
            let (client, configured_time): (ClientConnection, ConfiguredCertificateTime) =
                connection(&config, wall_time)?;
            assert!(client.is_handshaking());
            assert_eq!(configured_time.into_provenance(), expected);
        }
        Ok(())
    }

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

#[cfg(test)]
mod managed_close_fixture;

#[cfg(test)]
mod certificate_time_fixture {
    use std::ffi::OsString;
    use std::time::{Duration, Instant, SystemTime};

    use super::super::{Config, Control, Failed, Failure, PROCESS_ADMISSION, Phase, ProviderScope};
    use super::{AssociationOwner, CertificateTimeProvenance, LocalWallTime, establish_with_time};

    /// Server identity of the admitted offline fixture's synthetic certificate.
    const PEER_IDENTITY: &str = "ke-a7e40ec4eade477dbaba6ca9e9f975a4.invalid";

    /// 2026-11-23 00:00:00 UTC, inside the pinned laboratory chain and CRL window.
    const FIXTURE_WALL_SECONDS: u64 = 1_795_392_000;

    /// Observe the association returned by real TLS, KE and TLS retirement.
    /// The external controller owns the admitted peer and temporary guest trust.
    /// Run one case per client process, using the original one-shot provider.
    ///
    /// # Errors
    /// Preserve fixture input, provider, transport or observation refusal.
    /// # Panics
    /// Fail if the published phase, certificate-time class or cookie population differs.
    fn assert_accepted_handshake(
        wall_time: LocalWallTime,
        case: &str,
        expected: CertificateTimeProvenance,
    ) -> Result<(), Failed> {
        let peer: OsString = std::env::var_os("NV_TLS_CLOCK_PEER").ok_or(Failure::Arguments)?;
        let config: Config = Config::new(peer, OsString::from(PEER_IDENTITY))?;
        let control: Control = Control::new(Instant::now())?;
        let provider: ProviderScope<'_> = ProviderScope::new(PROCESS_ADMISSION.claim()?)?;
        let association: AssociationOwner<'_> =
            establish_with_time(&config, &control, &provider, wall_time)?;
        let actual: CertificateTimeProvenance = association.certificate_time_provenance();
        let (cookies, cookie_bytes): (usize, usize) = association.cookie_population()?;

        assert!(matches!(*control.guard()?.phase, Phase::Published));
        assert_eq!(actual, expected);
        assert_eq!((cookies, cookie_bytes), (1, 12));
        println!(
            "clock_case={case} negotiated=1 certificate_time={actual:?} cookies={cookies} cookie_bytes={cookie_bytes}"
        );
        Ok(())
    }

    /// Retain fixed test wall provenance after the admitted peer completes TLS and KE.
    /// # Errors
    /// Preserve fixture input, provider, transport or observation refusal.
    #[test]
    #[ignore = "requires the owned offline Windows NTS-KE peer and admitted laboratory trust"]
    fn accepted_fixture_wall_ke_retains_certificate_time_provenance() -> Result<(), Failed> {
        let fixed_wall: SystemTime = SystemTime::UNIX_EPOCH
            .checked_add(Duration::from_secs(FIXTURE_WALL_SECONDS))
            .ok_or(Failure::Control)?;
        let wall_time: LocalWallTime = LocalWallTime {
            fixture_time: Some(fixed_wall),
        };
        assert_accepted_handshake(
            wall_time,
            "fixture-wall",
            CertificateTimeProvenance::WindowsLaboratoryFixtureWall,
        )
    }

    /// Retain ordinary local wall provenance on the same accepted peer profile.
    /// # Errors
    /// Preserve fixture input, provider, transport or observation refusal.
    #[test]
    #[ignore = "requires the owned offline Windows NTS-KE peer and admitted laboratory trust"]
    fn accepted_ordinary_wall_ke_retains_certificate_time_provenance() -> Result<(), Failed> {
        assert_accepted_handshake(
            LocalWallTime::system(),
            "ordinary",
            CertificateTimeProvenance::WindowsLaboratoryLocalWall,
        )
    }

    /// Observe the real refusal after the TLS stream and provider scope retire.
    /// The external controller owns the peer and must run one case per client
    /// process. The ordinary verifier runs without the synthetic root installed.
    ///
    /// # Errors
    /// Preserve any fixture input, provider or control admission refusal.
    /// # Panics
    /// Fail for publication, a changed primary or a fabricated cleanup failure.
    fn assert_refused_handshake(
        wall_time: LocalWallTime,
        case: &str,
        expected: &str,
    ) -> Result<(), Failure> {
        let peer: OsString = std::env::var_os("NV_TLS_CLOCK_PEER").ok_or(Failure::Arguments)?;
        let config: Config = Config::new(peer, OsString::from(PEER_IDENTITY))?;
        let control: Control = Control::new(Instant::now())?;
        let failure: Failed = {
            let provider: ProviderScope<'_> = ProviderScope::new(PROCESS_ADMISSION.claim()?)?;
            establish_with_time(&config, &control, &provider, wall_time)
                .err()
                .ok_or(Failure::Control)?
        };

        assert!(
            matches!(*control.guard()?.phase, Phase::Open),
            "refused TLS must leave association publication open"
        );
        let primary: String = format!("{:?}", failure.primary);
        assert_eq!(primary, expected);
        assert_eq!(
            failure.cleanup, None,
            "close must not repeat the cached TLS error"
        );
        assert!(failure.admission_cleanup.is_none());
        println!(
            "clock_case={case} primary={primary} cleanup={:?}",
            failure.cleanup
        );
        Ok(())
    }

    /// The injected reading refuses a real certificate-bearing handshake.
    ///
    /// # Errors
    /// Preserve fixture or native provider admission failure.
    #[test]
    #[ignore = "requires the owned offline Windows certificate-bearing NTS-KE peer"]
    fn pre_epoch_certificate_handshake_preserves_local_clock_failure() -> Result<(), Failure> {
        let wall_time: LocalWallTime = LocalWallTime {
            fixture_time: Some(SystemTime::UNIX_EPOCH - Duration::from_nanos(100)),
        };
        assert_refused_handshake(
            wall_time,
            "pre-epoch",
            "Tls(TlsFailure { stage: Handshake, cause: LocalClock })",
        )
    }

    /// The first whole second outside the verifier's range remains a local-clock failure.
    /// # Errors
    /// Preserve fixture or native provider admission failure.
    #[test]
    #[ignore = "requires the owned offline Windows certificate-bearing NTS-KE peer"]
    fn above_range_certificate_handshake_preserves_local_clock_failure() -> Result<(), Failure> {
        let wall_time: LocalWallTime = LocalWallTime {
            fixture_time: Some(SystemTime::UNIX_EPOCH + Duration::from_secs(4_102_444_801)),
        };
        assert_refused_handshake(
            wall_time,
            "above-range",
            "Tls(TlsFailure { stage: Handshake, cause: LocalClock })",
        )
    }

    /// The formerly overflowing conversion date refuses with the same typed cause.
    /// # Errors
    /// Preserve fixture or native provider admission failure.
    #[test]
    #[ignore = "requires the owned offline Windows certificate-bearing NTS-KE peer"]
    fn far_future_certificate_handshake_preserves_local_clock_failure() -> Result<(), Failure> {
        let wall_time: LocalWallTime = LocalWallTime {
            fixture_time: Some(SystemTime::UNIX_EPOCH + Duration::from_secs(6_802_270_474)),
        };
        assert_refused_handshake(
            wall_time,
            "far-future",
            "Tls(TlsFailure { stage: Handshake, cause: LocalClock })",
        )
    }

    /// The ordinary local clock reaches the verifier and refuses the untrusted issuer.
    ///
    /// # Errors
    /// Preserve fixture or native provider admission failure.
    #[test]
    #[ignore = "requires the owned offline Windows certificate-bearing NTS-KE peer"]
    fn ordinary_certificate_handshake_preserves_certificate_failure() -> Result<(), Failure> {
        assert_refused_handshake(
            LocalWallTime::system(),
            "ordinary",
            "Tls(TlsFailure { stage: Handshake, cause: Certificate(Issuer) })",
        )
    }
}

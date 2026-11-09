//! The sole real Windows TLS/KE producer, confined to one synchronous job.
//!
//! No authenticated association escapes: strict same-connection keys/cookies
//! protect one untransmitted laboratory request before secret/native destruction.

use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;
use std::time::{Duration, Instant};

use ntp_wire::{NtsKeAccumulatorError, NtsKeFrameError};
use rustls::{
    CipherSuite, ClientConfig, ClientConnection, HandshakeKind, ProtocolVersion, StreamOwned,
    SupportedCipherSuite,
};
use rustls_cng_crypto::ShutdownHandle;
use zeroize::Zeroizing;

use super::transport_fault::{self, TransportFault};
use super::{
    Config, Control, Failed, Failure, IoPhase, SCRATCH_LIMIT, Summary, encode_offer, read_response,
};
use super::{handoff, request::DirectionalKeys};

/// One numeric socket whose every operation shares the original job deadlines.
struct DeadlineStream<'a> {
    /// Sole owned connection; destruction closes it without another job.
    socket: TcpStream,
    /// Original immutable deadlines and serialized publication authority.
    control: &'a Control,
    /// Handshake uses twenty seconds; subsequent work uses the same-origin sixty.
    phase: IoPhase,
    /// First fault is sticky, including interruption and suppressed write errors.
    fault: Option<TransportFault>,
}

impl DeadlineStream<'_> {
    /// Obtain fresh remaining time, refusing all I/O after the first fault.
    ///
    /// # Errors
    /// Return a nonretryable error after an earlier fault. Otherwise latch a
    /// deadline, cancellation or control refusal and return its I/O category.
    fn timeout(&mut self) -> io::Result<Duration> {
        if self.fault.is_some() {
            // Rustls retries Interrupted internally. A nonretryable subsequent
            // error stops that loop; resolve() still reports its original cause.
            return Err(io::ErrorKind::Other.into());
        }
        match self.control.remaining(self.phase) {
            Ok(duration) => Ok(duration),
            Err(failure) => {
                self.fault = Some(TransportFault::Control(failure));
                let kind = match failure {
                    Failure::Deadline => io::ErrorKind::TimedOut,
                    Failure::Cancelled => io::ErrorKind::Interrupted,
                    _ => io::ErrorKind::Other,
                };
                Err(kind.into())
            }
        }
    }

    /// Latch a native refusal or reject success observed after expiry/stop.
    ///
    /// # Errors
    /// Retain the supplied I/O error, or refuse a nominal success if the fresh
    /// deadline/cancellation guard fails. No successful value escapes refusal.
    fn completed<T>(&mut self, result: io::Result<T>) -> io::Result<T> {
        match result {
            Ok(value) => self.timeout().map(|_| value),
            Err(error) => {
                if self.fault.is_none() {
                    self.fault = Some(TransportFault::Io(error.kind()));
                }
                Err(error)
            }
        }
    }

    /// Preserve primary TLS refusal or restore a retried/hidden transport fault.
    ///
    /// # Errors
    /// Preserve a supplied TLS invalid-data failure before its best-effort alert
    /// fault. Otherwise restore the first latched control/stage-mapped I/O cause.
    /// With no latched fault, preserve the supplied result.
    fn resolve<T>(
        &self,
        result: Result<T, Failure>,
        stage: fn(io::ErrorKind) -> Failure,
    ) -> Result<T, Failure> {
        transport_fault::resolve(self.fault, result, stage)
    }

    /// Recheck current authority after CPU/native-verifier work without I/O.
    ///
    /// # Errors
    /// Refuse an existing transport fault, expired deadline, cancellation or
    /// unavailable control state; preserve the first exact failure category.
    fn current(&mut self, stage: fn(io::ErrorKind) -> Failure) -> Result<(), Failure> {
        let result = self
            .timeout()
            .map(|_| ())
            .map_err(|error| stage(error.kind()));
        self.resolve(result, stage)
    }
}

impl Read for DeadlineStream<'_> {
    /// Set a fresh finite timeout, perform one read, then refuse late completion.
    ///
    /// # Errors
    /// Propagate timeout-setting or read failure, including interruption. Refuse
    /// any latched fault or deadline/control failure before or after the read.
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let timeout = self.timeout()?;
        let setting = self.socket.set_read_timeout(Some(timeout));
        self.completed(setting)?;
        let result = self.socket.read(output);
        self.completed(result)
    }
}

impl Write for DeadlineStream<'_> {
    /// Set a fresh finite timeout, perform one write, then refuse late completion.
    ///
    /// # Errors
    /// Propagate timeout-setting or write failure, including interruption. Refuse
    /// any latched fault or deadline/control failure before or after the write.
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        let timeout = self.timeout()?;
        let setting = self.socket.set_write_timeout(Some(timeout));
        self.completed(setting)?;
        let result = self.socket.write(input);
        self.completed(result)
    }

    /// Flush this socket under the same guard, without resetting its deadline.
    ///
    /// # Errors
    /// Propagate timeout-setting or flush failure. Refuse any latched fault or
    /// deadline/control failure before or after the flush without retrying it.
    fn flush(&mut self) -> io::Result<()> {
        let timeout = self.timeout()?;
        let setting = self.socket.set_write_timeout(Some(timeout));
        self.completed(setting)?;
        let result = self.socket.flush();
        self.completed(result)
    }
}

/// Actual negotiated values captured only after the ordinary strict verifier.
struct StrictProfile {
    /// Actual protocol returned by this completed connection.
    protocol: ProtocolVersion,
    /// Actual selected suite from the sole explicit CNG provider.
    suite: CipherSuite,
    /// Full handshakes with or without `HelloRetryRequest` are admitted.
    handshake: HandshakeKind,
}

impl StrictProfile {
    /// Capture the completed connection's profile after exact ALPN admission.
    ///
    /// # Errors
    /// Return `Failure::TlsProfile` for an unfinished handshake, absent profile
    /// metadata, wrong ALPN, unsupported suite/protocol or resumed handshake.
    fn capture(connection: &ClientConnection) -> Result<Self, Failure> {
        if connection.is_handshaking() || connection.alpn_protocol() != Some(b"ntske/1".as_slice())
        {
            return Err(Failure::TlsProfile);
        }
        let profile = Self {
            protocol: connection.protocol_version().ok_or(Failure::TlsProfile)?,
            suite: connection
                .negotiated_cipher_suite()
                .ok_or(Failure::TlsProfile)?
                .suite(),
            handshake: connection.handshake_kind().ok_or(Failure::TlsProfile)?,
        };
        profile.check()?;
        Ok(profile)
    }

    /// Admit only the captured full TLS 1.3/CNG profile; no resumption qualifies.
    ///
    /// # Errors
    /// Return `Failure::TlsProfile` unless the captured protocol, suite and full
    /// handshake kind all meet the fixed laboratory profile.
    fn check(&self) -> Result<(), Failure> {
        if self.protocol != ProtocolVersion::TLSv1_3
            || !matches!(
                self.handshake,
                HandshakeKind::Full | HandshakeKind::FullWithHelloRetryRequest
            )
            || suite_priority(self.suite) == 3
        {
            return Err(Failure::TlsProfile);
        }
        Ok(())
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

impl InternalCandidate<'_> {
    /// Check the binding, seal one private request, then commit and wipe once.
    ///
    /// # Errors
    /// Refuse an invalid profile, endpoint, invocation/expiry/AEAD binding,
    /// cookie budget, entropy/sealing or final currentness/commit guard. Failure
    /// destroys the candidate and private packet without returning a summary.
    fn finish(self) -> Result<Summary, Failure> {
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
        profile.check()?;
        if peer != config.peer || udp.ip() != peer.ip() || udp.port() == 0 {
            return Err(Failure::Endpoint);
        }
        if config.identity.is_empty() || expiry != control.expiry || aead != 15 {
            return Err(Failure::Control);
        }
        let request = handoff::prepare(
            control,
            DirectionalKeys::new(c2s_key, s2c_key),
            &cookies,
            |output| {
                use rustls::crypto::SecureRandom;
                rustls_cng_crypto::SecureRandom
                    .fill(output)
                    .map_err(|_| Failure::Entropy)
            },
        )?;
        let summary = request.finish()?;
        drop(cookies);
        Ok(summary)
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

/// Construct explicit CNG/TLS13 and the ordinary OS verifier, with no extra roots.
///
/// # Errors
/// Return `Failure::TlsSetup` if the provider suite set, ordinary verifier, TLS
/// builder or client constructor refuses. Return `Failure::Allocation` if a
/// bounded application-owned ALPN allocation fails.
fn connection(config: &Config) -> Result<ClientConnection, Failure> {
    let mut provider = rustls_cng_crypto::default_provider();
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
        return Err(Failure::TlsSetup);
    }
    let provider = Arc::new(provider);
    let verifier = rustls_platform_verifier::Verifier::new(Arc::clone(&provider))
        .map_err(|_| Failure::TlsSetup)?;
    let mut tls = ClientConfig::builder_with_provider(provider)
        .with_protocol_versions(&[&rustls::version::TLS13])
        .map_err(|_| Failure::TlsSetup)?
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(verifier))
        .with_no_client_auth();
    let mut alpn = Vec::new();
    alpn.try_reserve_exact(7).map_err(|_| Failure::Allocation)?;
    alpn.extend_from_slice(b"ntske/1");
    tls.alpn_protocols
        .try_reserve_exact(1)
        .map_err(|_| Failure::Allocation)?;
    tls.alpn_protocols.push(alpn);
    tls.resumption = rustls::client::Resumption::disabled();
    tls.enable_early_data = false;
    let mut connection = ClientConnection::new(Arc::new(tls), config.server_name.clone())
        .map_err(|_| Failure::TlsSetup)?;
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
        Err(error) => Err(Failure::Response(error.kind())),
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
    config: &'a Config,
    peer: SocketAddr,
) -> Result<InternalCandidate<'a>, Failure> {
    let handshake = stream
        .conn
        .complete_io(&mut stream.sock)
        .map_err(|error| Failure::Handshake(error.kind()));
    stream.sock.resolve(handshake, Failure::Handshake)?;
    stream.sock.current(Failure::Handshake)?;
    let profile = StrictProfile::capture(&stream.conn)?;
    stream.sock.phase = IoPhase::Exchange;
    let mut storage = [0_u8; 16];
    let offer = encode_offer(&mut storage)?;
    let request = stream
        .write_all(offer.request_bytes())
        .and_then(|()| stream.flush())
        .map_err(|error| Failure::Request(error.kind()));
    stream.sock.resolve(request, Failure::Request)?;
    let response = read_response(stream, config, &offer);
    let reply = stream.sock.resolve(response, Failure::Response)?;
    stream.sock.current(Failure::Response)?;
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
    stream.sock.current(Failure::Response)?;
    Ok(InternalCandidate {
        config,
        control: stream.sock.control,
        peer,
        udp: reply.udp,
        expiry: stream.sock.control.expiry,
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
/// Refuse a known transport/currentness fault, TLS-write zero/failure, socket or
/// TLS-writer flush failure, or already-buffered post-EOM plaintext/error. A
/// successful return reports only the local close-notify drain and flush.
fn close(stream: &mut StreamOwned<ClientConnection, DeadlineStream<'_>>) -> Result<(), Failure> {
    stream.sock.current(Failure::Shutdown)?;
    stream.conn.send_close_notify();
    while stream.conn.wants_write() {
        let written = stream
            .conn
            .write_tls(&mut stream.sock)
            .map_err(|error| Failure::Shutdown(error.kind()));
        if stream.sock.resolve(written, Failure::Shutdown)? == 0 {
            return Err(Failure::Shutdown(io::ErrorKind::WriteZero));
        }
    }
    let flushed = stream
        .sock
        .flush()
        .and_then(|()| stream.conn.writer().flush())
        .map_err(|error| Failure::Shutdown(error.kind()));
    stream.sock.resolve(flushed, Failure::Shutdown)?;
    stream.sock.current(Failure::Shutdown)?;
    reject_buffered(&mut stream.conn)
}

/// Own every TLS/secret object inside the outermost CNG shutdown lifetime.
///
/// # Errors
/// Refuse numeric connect/peer, TLS/KE preparation, local close or final candidate
/// commit failure. Retain an independently observed close refusal alongside a
/// primary preparation failure; no TLS or secret owner escapes either outcome.
fn exchange_owned(config: &Config, control: &Control) -> Result<Summary, Failed> {
    let timeout = control.remaining(IoPhase::Handshake)?;
    let socket = TcpStream::connect_timeout(&config.peer, timeout)
        .map_err(|error| Failure::Connect(error.kind()))?;
    control.remaining(IoPhase::Handshake)?;
    let peer = socket
        .peer_addr()
        .map_err(|error| Failure::Connect(error.kind()))?;
    if peer != config.peer {
        return Err(Failure::Peer.into());
    }
    let socket = DeadlineStream {
        socket,
        control,
        phase: IoPhase::Handshake,
        fault: None,
    };
    let mut stream = StreamOwned::new(connection(config)?, socket);
    let candidate = prepare(&mut stream, config, peer);
    let closed = close(&mut stream);
    drop(stream);
    // Retain an independently observed close refusal alongside its primary.
    match (candidate, closed) {
        (Err(primary), Err(cleanup)) => Err(Failed {
            primary,
            cleanup: Some(cleanup),
            admission_cleanup: None,
        }),
        (Err(primary), Ok(())) | (Ok(_), Err(primary)) => Err(primary.into()),
        (Ok(candidate), Ok(())) => candidate.finish().map_err(Failed::from),
    }
}

/// Run exactly one real KE job and return only counts after secret/native drops.
///
/// # Errors
/// Refuse setup, strict trust/profile, bounded KE, exporter, cleanup, deadline or
/// cancellation failure. No error contains peer, identity, cookie or key bytes.
pub(super) fn exchange(config: &Config, control: &Control) -> Result<Summary, Failed> {
    control.remaining(IoPhase::Handshake)?;
    let shutdown = ShutdownHandle {};
    let result = exchange_owned(config, control);
    drop(shutdown);
    result
}

//! Sole Windows adapter for the fixed healthy peer; no trust-store operations.
//!
//! Inputs and readiness must be in an owned private guest directory. Safe
//! Windows file handles deny write/delete sharing and reject reparse files.
//! Parent-directory custody is supplied by the admitted single-owner guest job.

use std::fs::{self, Metadata, OpenOptions};
use std::io::{self, Read, Write};
use std::net::{IpAddr, Ipv4Addr, TcpListener, TcpStream};
use std::os::windows::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Component, Path, Prefix};
use std::sync::Arc;
use std::time::Instant;

use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use rustls::{HandshakeKind, ProtocolVersion, ServerConfig, ServerConnection, StreamOwned};
use rustls_cng_crypto::ShutdownHandle;
use zeroize::Zeroizing;

use super::{Budget, Config, Failed, Failure, Flow, OFFER, RESPONSE, admit_offer, ready_line, resolve};

/// Maximum admitted public certificate or private PKCS8 extent.
const DER_LIMIT: u64 = 16 * 1024;
/// Windows flag opens a reparse point itself instead of following its target.
const OPEN_REPARSE_POINT: u32 = 0x0020_0000;
/// Reject every reparse tag, not only symbolic links.
const REPARSE_ATTRIBUTE: u32 = 0x400;
/// Permit readers only; concurrent write/delete handles cannot coexist.
const SHARE_READ: u32 = 1;

/// Compare held-file observations without treating metadata as a content digest.
fn stamp(info: &Metadata) -> (u32, u64, u64, u64) {
    (
        info.file_attributes(),
        info.creation_time(),
        info.last_write_time(),
        info.file_size(),
    )
}

/// Require a local drive path with existing non-reparse directory ancestors.
fn check_parent(path: &Path) -> Result<(), Failure> {
    if !matches!(
        path.components().next(),
        Some(Component::Prefix(prefix))
            if matches!(prefix.kind(), Prefix::Disk(_) | Prefix::VerbatimDisk(_))
    ) {
        return Err(Failure::Input);
    }
    let parent = path.parent().ok_or(Failure::Input)?;
    for ancestor in parent.ancestors() {
        let info = fs::symlink_metadata(ancestor).map_err(|_| Failure::Input)?;
        if !info.is_dir() || info.file_attributes() & REPARSE_ATTRIBUTE != 0 {
            return Err(Failure::Input);
        }
    }
    Ok(())
}

/// Read one nonempty admitted extent into a wiping buffer under a held handle.
/// File sharing prevents ordinary concurrent writes/deletes; no directory-race or
/// elimination-of-all-provider-copy guarantee follows from these observations.
fn read_der(path: &Path) -> Result<Zeroizing<Vec<u8>>, Failure> {
    check_parent(path)?;
    let before = fs::symlink_metadata(path).map_err(|_| Failure::Input)?;
    if !before.is_file()
        || before.file_attributes() & REPARSE_ATTRIBUTE != 0
        || !(1..=DER_LIMIT).contains(&before.len())
    {
        return Err(Failure::Input);
    }
    let mut file = OpenOptions::new()
        .read(true)
        .share_mode(SHARE_READ)
        .custom_flags(OPEN_REPARSE_POINT)
        .open(path)
        .map_err(|_| Failure::Input)?;
    let held = file.metadata().map_err(|_| Failure::Input)?;
    if !held.is_file()
        || held.file_attributes() & REPARSE_ATTRIBUTE != 0
        || stamp(&held) != stamp(&before)
    {
        return Err(Failure::Input);
    }
    let length = usize::try_from(held.len()).map_err(|_| Failure::Input)?;
    let mut bytes = Zeroizing::new(Vec::new());
    bytes.try_reserve_exact(length).map_err(|_| Failure::Input)?;
    bytes.resize(length, 0);
    file.read_exact(bytes.as_mut()).map_err(|_| Failure::Input)?;
    let mut extra = Zeroizing::new([0_u8; 1]);
    if file.read(extra.as_mut()).map_err(|_| Failure::Input)? != 0
        || stamp(&file.metadata().map_err(|_| Failure::Input)?) != stamp(&held)
        || stamp(&fs::symlink_metadata(path).map_err(|_| Failure::Input)?) != stamp(&held)
    {
        return Err(Failure::Input);
    }
    Ok(bytes)
}

/// Publish only a complete new readiness record; never overwrite a destination.
/// Hard-link publication requires private NTFS, rather than a mapped output share.
fn publish_ready(path: &Path, port: u16) -> Result<(), Failure> {
    check_parent(path)?;
    let contents = ready_line(port)?;
    let mut name = path.file_name().ok_or(Failure::Input)?.to_os_string();
    name.push(".pending");
    let temporary = path.with_file_name(name);
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .share_mode(0)
        .custom_flags(OPEN_REPARSE_POINT)
        .open(&temporary)
        .map_err(|error| Failure::Ready {
            primary: error.kind(),
            cleanup: None,
        })?;
    let written = file
        .write_all(contents.as_bytes())
        .and_then(|()| file.flush())
        .and_then(|()| file.sync_all());
    drop(file);
    let published = written.and_then(|()| fs::hard_link(&temporary, path));
    let cleanup = fs::remove_file(&temporary);
    match (published, cleanup) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(primary), secondary) => Err(Failure::Ready {
            primary: primary.kind(),
            cleanup: secondary.err().map(|error| error.kind()),
        }),
        (Ok(()), Err(primary)) => Err(Failure::Ready {
            primary: primary.kind(),
            cleanup: None,
        }),
    }
}

/// Accepted socket plus the original deadline and separate actual-byte counters.
struct BudgetIo {
    /// Single numeric accepted socket; destruction closes only this connection.
    socket: TcpStream,
    /// Same original budget before and after every native operation.
    budget: Budget,
    /// First refusal remains sticky even if a library later suppresses an error.
    fault: Option<Failure>,
}

impl BudgetIo {
    /// Refuse old faults or expiration before an operation, returning a poll cap.
    fn guard(&mut self) -> io::Result<std::time::Duration> {
        if self.fault.is_some() {
            return Err(io::ErrorKind::Other.into());
        }
        self.budget.poll(Instant::now()).map_err(|failure| {
            self.fault = Some(failure);
            io::ErrorKind::TimedOut.into()
        })
    }

    /// Convert a budget refusal into a sticky nonretryable I/O failure.
    fn account(&mut self, flow: Flow, amount: usize) -> io::Result<()> {
        self.budget.charge(flow, amount).map_err(|failure| {
            self.fault = Some(failure);
            io::ErrorKind::Other.into()
        })
    }

    /// Preserve the original typed fault, including a suppressed nominal success.
    fn settle<T>(&self, result: io::Result<T>) -> Result<T, Failed> {
        resolve(self.fault, result.map_err(|error| Failure::Io(error.kind())))
    }

    /// Latch the first socket refusal without retaining its native message.
    fn refuse<T>(&mut self, error: io::Error) -> io::Result<T> {
        if self.fault.is_none() {
            self.fault = Some(Failure::Io(error.kind()));
        }
        Err(error)
    }

    /// Obtain a positive capped buffer size before a native socket operation.
    fn amount(&mut self, flow: Flow, length: usize) -> io::Result<usize> {
        self.budget.allowance(flow).map(|left| left.min(length)).map_err(|failure| {
            self.fault = Some(failure);
            io::ErrorKind::Other.into()
        })
    }
}

impl Read for BudgetIo {
    /// Retry only finite timeout/interruption polls under the original deadline.
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        loop {
            let timeout = self.guard()?;
            if output.is_empty() {
                return Ok(0);
            }
            let count = self.amount(Flow::Read, output.len())?;
            if let Err(error) = self.socket.set_read_timeout(Some(timeout)) {
                return self.refuse(error);
            }
            match self.socket.read(&mut output[..count]) {
                Ok(actual) => {
                    self.account(Flow::Read, actual)?;
                    self.guard()?;
                    return Ok(actual);
                }
                Err(error) if matches!(error.kind(), io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut | io::ErrorKind::Interrupted) => {}
                Err(error) => return self.refuse(error),
            }
        }
    }
}

impl Write for BudgetIo {
    /// Charge each actual write before returning; progress cannot renew time.
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        loop {
            let timeout = self.guard()?;
            if input.is_empty() {
                return Ok(0);
            }
            let count = self.amount(Flow::Write, input.len())?;
            if let Err(error) = self.socket.set_write_timeout(Some(timeout)) {
                return self.refuse(error);
            }
            match self.socket.write(&input[..count]) {
                Ok(actual) => {
                    self.account(Flow::Write, actual)?;
                    self.guard()?;
                    return Ok(actual);
                }
                Err(error) if matches!(error.kind(), io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut | io::ErrorKind::Interrupted) => {}
                Err(error) => return self.refuse(error),
            }
        }
    }

    /// Observe flush refusal and fresh currentness without another byte allowance.
    fn flush(&mut self) -> io::Result<()> {
        self.guard()?;
        if let Err(error) = self.socket.flush() {
            return self.refuse(error);
        }
        self.guard().map(|_| ())
    }
}

/// Create the strict server profile with the already selected CNG provider.
fn connection(certificate: &Path, key: &Path) -> Result<ServerConnection, Failure> {
    let mut certificate = read_der(certificate)?;
    let key = read_der(key)?;
    let certificate = CertificateDer::from(std::mem::take(&mut *certificate));
    let mut provider_key = Zeroizing::new(Vec::new());
    provider_key.try_reserve_exact(key.len()).map_err(|_| Failure::Input)?;
    provider_key.extend_from_slice(&key);
    // The public CNG API consumes an owned DER copy with no wiping Drop. Retire
    // the original read buffer explicitly; provider DER/native copies remain
    // unqualified. This fixture uses only a disposable synthetic signing key.
    let mut config = ServerConfig::builder_with_provider(Arc::new(rustls_cng_crypto::default_provider()))
        .with_protocol_versions(&[&rustls::version::TLS13])
        .map_err(|_| Failure::Tls)?
        .with_no_client_auth()
        .with_single_cert(
            vec![certificate],
            PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(std::mem::take(&mut *provider_key))),
        )
        .map_err(|_| Failure::Tls)?;
    drop(key);
    config.alpn_protocols = vec![b"ntske/1".to_vec()];
    config.send_tls13_tickets = 0;
    config.session_storage = Arc::new(rustls::server::NoServerSessionStorage {});
    config.max_early_data_size = 0;
    config.max_fragment_size = Some(4096);
    let mut connection = ServerConnection::new(Arc::new(config)).map_err(|_| Failure::Tls)?;
    connection.set_buffer_limit(Some(4096));
    Ok(connection)
}

/// Drain the local TLS close-notify before awaiting the client's actual alert.
fn close(stream: &mut StreamOwned<ServerConnection, BudgetIo>) -> Result<(), Failed> {
    stream.conn.send_close_notify();
    while stream.conn.wants_write() {
        let written = stream.conn.write_tls(&mut stream.sock);
        if stream.sock.settle(written)? == 0 {
            return Err(Failure::Io(io::ErrorKind::WriteZero).into());
        }
    }
    let flushed = stream.sock.flush();
    stream.sock.settle(flushed)?;
    loop {
        let mut extra = [0_u8; 1];
        match stream.conn.reader().read(&mut extra) {
            Ok(0) => return Ok(()),
            Ok(_) => return Err(Failure::Offer.into()),
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
            Err(_) => return Err(Failure::Closure.into()),
        }
        let read = stream.conn.read_tls(&mut stream.sock);
        if stream.sock.settle(read)? == 0 {
            return Err(Failure::Closure.into());
        }
        let state = stream.conn.process_new_packets().map_err(|_| Failure::Tls)?;
        if state.plaintext_bytes_to_read() != 0 {
            return Err(Failure::Offer.into());
        }
        if state.peer_has_closed() {
            return Ok(());
        }
    }
}

/// Own one listener/connection/config; return no TLS or key owner to the caller.
fn serve_owned(config: Config, origin: Instant) -> Result<(), Failed> {
    let budget = Budget::new(origin)?;
    let mut connection = connection(&config.certificate, &config.key)?;
    budget.poll(Instant::now())?;
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).map_err(|error| Failure::Io(error.kind()))?;
    listener.set_nonblocking(true).map_err(|error| Failure::Io(error.kind()))?;
    let port = listener.local_addr().map_err(|error| Failure::Io(error.kind()))?.port();
    publish_ready(&config.ready, port)?;
    let (socket, peer) = loop {
        let pause = budget.poll(Instant::now())?;
        match listener.accept() {
            Ok(accepted) => break accepted,
            Err(error) if matches!(error.kind(), io::ErrorKind::WouldBlock | io::ErrorKind::Interrupted) => std::thread::sleep(pause),
            Err(error) => return Err(Failure::Io(error.kind()).into()),
        }
    };
    drop(listener);
    if peer.ip() != IpAddr::V4(Ipv4Addr::LOCALHOST) {
        return Err(Failure::Arguments.into());
    }
    socket.set_nonblocking(false).map_err(|error| Failure::Io(error.kind()))?;
    let mut socket = BudgetIo { socket, budget, fault: None };
    let handshake = connection.complete_io(&mut socket);
    socket.settle(handshake)?;
    if connection.is_handshaking()
        || connection.protocol_version() != Some(ProtocolVersion::TLSv1_3)
        || connection.alpn_protocol() != Some(b"ntske/1".as_slice())
        || !matches!(connection.handshake_kind(), Some(HandshakeKind::Full | HandshakeKind::FullWithHelloRetryRequest))
    {
        return Err(Failure::Tls.into());
    }
    let mut stream = StreamOwned::new(connection, socket);
    let mut offer = [0_u8; OFFER.len()];
    let received = stream.read_exact(&mut offer);
    stream.sock.settle(received)?;
    admit_offer(&offer)?;
    let c2s = stream.conn.export_keying_material(Zeroizing::new([0_u8; 32]), b"EXPORTER-network-time-security", Some(&[0, 0, 0, 15, 0])).map_err(|_| Failure::Tls)?;
    let s2c = stream.conn.export_keying_material(Zeroizing::new([0_u8; 32]), b"EXPORTER-network-time-security", Some(&[0, 0, 0, 15, 1])).map_err(|_| Failure::Tls)?;
    drop((c2s, s2c));
    let sent = stream.write_all(&RESPONSE).and_then(|()| stream.flush());
    stream.sock.settle(sent)?;
    close(&mut stream)?;
    stream.sock.budget.poll(Instant::now())?;
    Ok(())
}

/// Keep every provider user inside the outer shutdown lifetime, including failure.
///
/// # Errors
/// Preserve setup/operation refusal; nominal success must still be within the
/// original lifetime after shutdown returns. Native close status is not exposed.
pub(super) fn serve(config: Config) -> Result<(), Failed> {
    let origin = Instant::now();
    let shutdown = ShutdownHandle {};
    let result = serve_owned(config, origin);
    drop(shutdown);
    result?;
    Budget::new(origin)?.poll(Instant::now())?;
    Ok(())
}

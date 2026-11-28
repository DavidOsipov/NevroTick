// SPDX-License-Identifier: GPL-2.0-only
//! Loopback controls for partial TCP request progress and withdrawal.

use super::{
    Capture, Connection, ExchangeError, ExchangeTransport, PendingTcpExchange, Phase, Receipt,
    RootKey, RoundAuthority, StagedReceipt,
};
use crate::{
    RECEIVE_CAP,
    roughtime::{exchange::Withdrawal, signed_reply},
};
use ed25519_dalek::SigningKey;
use std::{
    error::Error,
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    num::NonZeroUsize,
    ops::Range,
    thread,
    time::{Duration, Instant},
};

/// Original published request supplies the expected padding, tags and lengths.
const TEMPLATE: &[u8; 1036] =
    include_bytes!("../../../../tests/fixtures/roughtime/appendix-b-original-request.bin");
/// Published position of the server commitment.
const SERVER: Range<usize> = 56..88;
/// Published position of the request nonce.
const NONCE: Range<usize> = 88..120;
/// Splits the header/commitment, then the nonce/padding, with an uneven final write.
const OFFERED: NonZeroUsize = NonZeroUsize::MIN.saturating_add(62);
/// Bounds connection setup and every blocking peer observation.
const BUDGET: Duration = Duration::from_secs(2);

/// Bound fixture errors separately from exact protocol refusal assertions.
type TestResult<T> = Result<T, Box<dyn Error>>;

/// Fixture limit applied after taking the same remaining slice as production.
pub(super) struct RequestWriteLimit(NonZeroUsize);

impl RequestWriteLimit {
    /// Preserve ordinary full-slice writes for the other tests.
    pub(super) const UNLIMITED: Self = Self(NonZeroUsize::MAX);

    /// Offer no more than the fixture limit, without changing the offset.
    pub(super) fn apply<'a>(&self, remaining: &'a [u8]) -> &'a [u8] {
        &remaining[..remaining.len().min(self.0.get())]
    }
}

/// Start the real exchange and shorten only the bytes offered to each native write.
///
/// # Errors
/// Returns a fixture socket or unexpected exchange setup failure.
fn connect<'a>(
    authority: &'a RoundAuthority,
    root: &'a RootKey,
) -> TestResult<(PendingTcpExchange<'a>, TcpStream)> {
    let listener: TcpListener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
    let endpoint: SocketAddr = listener.local_addr()?;
    let stream: TcpStream = TcpStream::connect_timeout(&endpoint, BUDGET)?;
    // Avoid delayed sends between the fixture's deliberately small writes.
    stream.set_nodelay(true)?;
    listener.set_nonblocking(true)?;
    let (peer, _): (TcpStream, SocketAddr) = listener.accept()?;
    peer.set_read_timeout(Some(BUDGET))?;
    peer.set_write_timeout(Some(BUDGET))?;
    let mut pending: PendingTcpExchange<'a> =
        PendingTcpExchange::start(authority, stream, endpoint, root, BUDGET)
            .map_err(|error| format!("unexpected partial-write fixture start: {error:?}"))?;
    pending
        .connection
        .as_mut()
        .ok_or("missing stream")?
        .write_limit = RequestWriteLimit(OFFERED);
    Ok((pending, peer))
}

/// Observe a fresh request until at least `minimum` original bytes reach the peer.
/// Returns the observed prefix length and its cumulative send capture.
///
/// # Errors
/// Returns a fixture observation, exhausted poll budget or unexpected exchange failure.
/// # Panics
/// Panics if `minimum` is outside the request, progress or capture is invalid,
/// bytes differ, the phase changes incorrectly, or a receipt appears before a reply.
fn observe_writes(
    pending: &mut PendingTcpExchange<'_>,
    peer: &mut TcpStream,
    root: &[u8; 32],
    received: &mut [u8; TEMPLATE.len()],
    minimum: usize,
) -> TestResult<(usize, Capture)> {
    assert!((1..=received.len()).contains(&minimum));
    let mut expected: [u8; TEMPLATE.len()] = *TEMPLATE;
    expected[SERVER].copy_from_slice(&signed_reply::digest(&[&[0xff], root]));
    let nonce: &[u8] = &pending
        .connection
        .as_ref()
        .ok_or("missing stream")?
        .request
        .frame[NONCE];
    expected[NONCE].copy_from_slice(nonce);

    let cutoff: Instant = pending.cutoff;
    let mut observed: usize = 0;
    let mut prior: Option<Capture> = None;
    // Each poll either adds one or more bytes or spends the connection's stall allowance.
    for _ in 0..received.len() + usize::from(RECEIVE_CAP) {
        let before: Instant = Instant::now();
        assert!(
            pending
                .poll()
                .map_err(|error| format!("unexpected write refusal: {error:?}"))?
                .is_none()
        );
        let after: Instant = Instant::now();
        let connection: &Connection<'_> = pending.connection.as_ref().ok_or("missing stream")?;
        let written: usize = connection.written;
        assert!((observed..=received.len()).contains(&written));
        assert!(written - observed <= OFFERED.get());
        assert_eq!(pending.cutoff, cutoff);
        let phase: Phase = *pending
            .authority
            .phase()
            .map_err(|error| format!("fixture phase: {error:?}"))?;
        if written == received.len() {
            assert!(matches!(phase, Phase::Awaiting));
        } else {
            assert!(matches!(phase, Phase::Preparing));
        }
        if written == observed {
            assert_eq!(
                connection
                    .sent
                    .map(|capture| (capture.before, capture.after)),
                prior.map(|capture| (capture.before, capture.after)),
            );
            thread::sleep(Duration::from_millis(1));
            continue;
        }
        let capture: Capture = connection.sent.ok_or("missing write capture")?;
        assert!((before..=after).contains(&capture.after));
        if let Some(previous) = prior {
            assert_eq!(capture.before, previous.before);
        } else {
            assert!((before..=capture.after).contains(&capture.before));
        }
        let chunk: &mut [u8] = &mut received[observed..written];
        peer.read_exact(chunk)?;
        assert_eq!(chunk, &expected[observed..written]);
        if written >= minimum {
            return Ok((written, capture));
        }
        observed = written;
        prior = Some(capture);
    }
    Err("fixture exhausted its request write polls".into())
}

/// Authenticate only after the complete uneven request reaches the peer.
///
/// # Errors
/// Returns the first fixture, socket, response or publication failure.
/// # Panics
/// Panics on incorrect retained bytes, captures or stream retirement.
#[test]
fn partial_writes_keep_one_exact_authenticated_request() -> TestResult<()> {
    let anchor: SigningKey = SigningKey::from_bytes(&[31; 32]);
    let delegated: SigningKey = SigningKey::from_bytes(&[32; 32]);
    let anchor_bytes: [u8; 32] = anchor.verifying_key().to_bytes();
    let root: RootKey =
        RootKey::from_bytes(&anchor_bytes).map_err(|error| format!("fixture key: {error:?}"))?;
    let authority: RoundAuthority = RoundAuthority::new();
    let (mut pending, mut peer): (PendingTcpExchange<'_>, TcpStream) = connect(&authority, &root)?;
    let mut request: [u8; TEMPLATE.len()] = [0; TEMPLATE.len()];
    let (written, sent): (usize, Capture) = observe_writes(
        &mut pending,
        &mut peer,
        &anchor_bytes,
        &mut request,
        TEMPLATE.len(),
    )?;
    assert_eq!(written, request.len());
    let nonce: [u8; 32] = request[NONCE].try_into()?;
    let response: [u8; 476] =
        signed_reply::sign_reply(&anchor, &delegated, &request, nonce, 1_000, 3)?;
    peer.write_all(&response)?;
    let mut staged: Option<StagedReceipt<'_>> = None;
    for _ in 0..32 {
        staged = pending
            .poll()
            .map_err(|error| format!("unexpected receiving refusal: {error:?}"))?;
        if staged.is_some() {
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
    let staged: StagedReceipt<'_> = staged.ok_or("fixture response did not settle")?;
    let mut tail: [u8; 1] = [0];
    assert_eq!(peer.read(&mut tail)?, 0);
    let receipt: Receipt = staged
        .publish()
        .map_err(|error| format!("unexpected publication refusal: {error:?}"))?;
    assert_eq!(receipt.request_frame(), request);
    assert_eq!(receipt.response_frame(), response);
    assert_eq!(receipt.transport(), ExchangeTransport::Tcp);
    assert_eq!(receipt.bounds_seconds(), (997, 1003));
    assert_eq!(receipt.captures().0, (sent.before, sent.after));
    assert!(matches!(pending.poll(), Err(ExchangeError::Spent)));
    let phase: Phase = *authority
        .phase()
        .map_err(|error| format!("fixture phase: {error:?}"))?;
    assert!(matches!(phase, Phase::Spent));
    Ok(())
}

/// Withdraw while a verified prefix is the only client data on the stream.
///
/// # Errors
/// Returns an unexpected fixture or withdrawal failure.
/// # Panics
/// Panics if a later write, receipt, or retained stream survives withdrawal.
#[test]
fn withdrawal_between_partial_writes_closes_without_sending_the_tail() -> TestResult<()> {
    let anchor: SigningKey = SigningKey::from_bytes(&[31; 32]);
    let anchor_bytes: [u8; 32] = anchor.verifying_key().to_bytes();
    let root: RootKey =
        RootKey::from_bytes(&anchor_bytes).map_err(|error| format!("fixture key: {error:?}"))?;
    let authority: RoundAuthority = RoundAuthority::new();
    let (mut pending, mut peer): (PendingTcpExchange<'_>, TcpStream) = connect(&authority, &root)?;
    let mut request: [u8; TEMPLATE.len()] = [0; TEMPLATE.len()];
    let (written, _): (usize, Capture) =
        observe_writes(&mut pending, &mut peer, &anchor_bytes, &mut request, 1)?;
    assert!((1..request.len()).contains(&written));
    authority
        .withdraw(Withdrawal::Stop)
        .map_err(|error| format!("fixture withdrawal: {error:?}"))?;
    assert!(matches!(
        pending.poll(),
        Err(ExchangeError::Withdrawn(Withdrawal::Stop))
    ));
    let mut tail: [u8; 1] = [0];
    assert_eq!(peer.read(&mut tail)?, 0);
    assert!(matches!(pending.poll(), Err(ExchangeError::Spent)));
    let phase: Phase = *authority
        .phase()
        .map_err(|error| format!("fixture phase: {error:?}"))?;
    assert!(matches!(phase, Phase::Withdrawn(Withdrawal::Stop)));
    Ok(())
}

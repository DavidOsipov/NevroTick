// SPDX-License-Identifier: GPL-2.0-only
//! Real loopback controls for the ordered round's TCP worker handoff.

use super::{
    ExchangeError, OrderedRound, PEERS, ProbeProgress, RetryBinding, RetryRoundPeer, RetryState,
    RootKey, RoundControl, RoundError, Withdrawal,
};
use ed25519_dalek::SigningKey;
use std::{
    error::Error,
    fmt::Debug,
    io::{self, Read},
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    num::NonZeroU64,
    sync::mpsc::{self, Receiver, Sender},
    time::{Duration, Instant},
};
use tokio::{
    runtime::{Builder, Runtime},
    sync::oneshot,
    time::{sleep, timeout},
};

/// Independent bound on worker and observer waits in this fixture.
const WATCHDOG: Duration = Duration::from_secs(3);
/// The first charged retry floor begins after the actual worker is joined.
const FIRST_RETRY_WAIT: Duration = Duration::from_secs(1);

/// A fixture or unexpected product outcome.
type TestResult<T> = Result<T, Box<dyn Error>>;

/// Retain the observed typed cause when an expected fixture action refuses.
///
/// # Errors
/// Returns the unexpected refusal as a test failure.
fn checked<T, E: Debug>(result: Result<T, E>) -> TestResult<T> {
    result.map_err(|error| format!("unexpected TCP worker fixture outcome: {error:?}").into())
}

/// Produce distinct valid roots without depending on the admitted response parser.
///
/// # Errors
/// Returns an unexpected root constructor refusal.
fn fixture_root(seed: u8) -> TestResult<RootKey> {
    let key: SigningKey = SigningKey::from_bytes(&[seed; 32]);
    checked(RootKey::from_bytes(key.verifying_key().as_bytes()))
}

/// Select test aliases. The first endpoint is the only listener used by the round.
fn routes(first: SocketAddr) -> [SocketAddr; PEERS] {
    [
        first,
        SocketAddr::from(([127, 0, 0, 2], first.port())),
        SocketAddr::from(([127, 0, 0, 3], first.port())),
    ]
}

/// Admit independent retry states even when two rounds share the fixture roots.
///
/// # Errors
/// Returns an original binding refusal.
fn bindings(
    roots: &[RootKey; PEERS],
    endpoints: [SocketAddr; PEERS],
) -> TestResult<[RetryBinding<'_>; PEERS]> {
    let [first, second, third]: [SocketAddr; PEERS] = endpoints;
    let port: u16 = first.port();
    Ok([
        checked(RetryBinding::new(
            &roots[0],
            [first, SocketAddr::from(([127, 0, 0, 4], port))],
        ))?,
        checked(RetryBinding::new(
            &roots[1],
            [second, SocketAddr::from(([127, 0, 0, 5], port))],
        ))?,
        checked(RetryBinding::new(
            &roots[2],
            [third, SocketAddr::from(([127, 0, 0, 6], port))],
        ))?,
    ])
}

/// Start a bound fixture round whose first leg uses the Engine's TCP worker.
///
/// # Errors
/// Returns an unexpected selection or round refusal.
fn tcp_round<'a>(
    control: &'a RoundControl,
    bindings: &'a mut [RetryBinding<'_>; PEERS],
    endpoints: [SocketAddr; PEERS],
) -> TestResult<OrderedRound<'a>> {
    let [first, second, third]: &mut [RetryBinding<'_>; PEERS] = bindings;
    let groups: [NonZeroU64; PEERS] = [
        NonZeroU64::MIN,
        NonZeroU64::new(2).ok_or("zero second fixture group")?,
        NonZeroU64::new(3).ok_or("zero third fixture group")?,
    ];
    let peers: [RetryRoundPeer<'_>; PEERS] = [
        checked(RetryRoundPeer::new(
            first,
            endpoints[0],
            groups[0],
            groups[0],
        ))?,
        checked(RetryRoundPeer::new(
            second,
            endpoints[1],
            groups[1],
            groups[1],
        ))?,
        checked(RetryRoundPeer::new(
            third,
            endpoints[2],
            groups[2],
            groups[2],
        ))?,
    ];
    let mut round: OrderedRound<'_> = checked(OrderedRound::new_with_retry(
        control, peers, WATCHDOG, WATCHDOG,
    ))?;
    round.tcp_slot = Some(0);
    round.worker_connect = true;
    Ok(round)
}

/// Establish a real stream but withhold it from the round until the fixture releases it.
///
/// # Errors
/// Returns a native connect failure or a closed or timed out fixture gate.
fn gated_connection(
    endpoint: SocketAddr,
    remaining: Duration,
    entered: oneshot::Sender<()>,
    release: Receiver<()>,
) -> io::Result<TcpStream> {
    let stream: TcpStream = TcpStream::connect_timeout(&endpoint, remaining)?;
    entered
        .send(())
        .map_err(|()| io::Error::other("worker observer disappeared"))?;
    release.recv_timeout(WATCHDOG).map_err(io::Error::other)?;
    Ok(stream)
}

/// Observe worker progress, then withdraw the round while the connected worker is held.
///
/// # Errors
/// Returns a watchdog, fixture, or unexpected round failure.
///
/// # Panics
/// Panics if the worker inherits an authority lock or an inactive round can accept its stream.
async fn stop_waiting_round(
    round: &mut OrderedRound<'_>,
    control: &RoundControl,
    endpoint: SocketAddr,
    entered: oneshot::Sender<()>,
    observed: oneshot::Receiver<()>,
    released: Receiver<()>,
) -> TestResult<()> {
    let selected: SocketAddr = checked(round.start_next_with(move |peer, remaining| {
        gated_connection(peer, remaining, entered, released)
    }))?;
    assert_eq!(selected, endpoint);
    timeout(WATCHDOG, observed).await??;
    assert!(matches!(round.poll(), Ok(ProbeProgress::Waiting)));
    checked(control.withdraw(Withdrawal::Stop))?;
    assert!(matches!(
        round.poll(),
        Err(RoundError::Withdrawn(Withdrawal::Stop))
    ));
    assert!(round.receipt(0).is_none());
    Ok(())
}

/// After releasing the first worker, reap it through normal admission and drive a successor.
///
/// # Errors
/// Returns a distinct observer watchdog or unexpected start or withdrawal refusal.
///
/// # Panics
/// Panics if the successor uses another route or stays active after Stop.
async fn run_successor(
    bindings: &mut [RetryBinding<'_>; PEERS],
    endpoints: [SocketAddr; PEERS],
    listener: &TcpListener,
) -> TestResult<()> {
    let start: Instant = Instant::now();
    loop {
        let control: RoundControl = RoundControl::new();
        let mut round: OrderedRound<'_> = tcp_round(&control, bindings, endpoints)?;
        match round.start_next() {
            Ok(selected) => {
                assert_eq!(selected, endpoints[0]);
                observe_successor(&mut round, listener).await?;
                checked(control.withdraw(Withdrawal::Stop))?;
                assert!(matches!(
                    round.poll(),
                    Err(RoundError::Withdrawn(Withdrawal::Stop))
                ));
                return Ok(());
            }
            Err(RoundError::Exchange(ExchangeError::ConnectBusy)) if start.elapsed() < WATCHDOG => {
                drop(round);
                sleep(Duration::from_millis(1)).await;
            }
            Err(error) => return checked(Err(error)),
        }
    }
}

/// Require the actual successor to adopt its original stream and send a request.
///
/// # Errors
/// Returns a native, round or fixture watchdog failure.
///
/// # Panics
/// Panics if the successor publishes a response before the listener has supplied one.
async fn observe_successor(round: &mut OrderedRound<'_>, listener: &TcpListener) -> TestResult<()> {
    let mut peer: Option<TcpStream> = None;
    let start: Instant = Instant::now();
    while start.elapsed() < WATCHDOG {
        assert_eq!(checked(round.poll())?, ProbeProgress::Waiting);
        if peer.is_none() {
            match listener.accept() {
                Ok((accepted, _)) => {
                    accepted.set_nonblocking(true)?;
                    peer = Some(accepted);
                }
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => (),
                Err(error) => return Err(error.into()),
            }
        }
        if let Some(accepted) = peer.as_mut() {
            let mut first: [u8; 1] = [0];
            match accepted.read(&mut first) {
                Ok(1) => return Ok(()),
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => (),
                Ok(_) => return Err("successor stream closed before sending a request".into()),
                Err(error) => return Err(error.into()),
            }
        }
        sleep(Duration::from_millis(1)).await;
    }
    Err("successor did not send before the fixture watchdog".into())
}

/// A connected worker retains the process lane and original retry state through Stop.
///
/// # Errors
/// Returns a native, round or fixture watchdog failure.
///
/// # Panics
/// Panics on premature lane or retry release, missing retirement, or any retired request bytes.
#[test]
fn stopped_connect_keeps_worker_and_retry_until_retired() -> TestResult<()> {
    let _fixture: std::sync::MutexGuard<'_, ()> = crate::native_network_job::CUSTODY_FIXTURE
        .lock()
        .map_err(|_| "native worker fixture unavailable")?;
    let original: TcpListener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
    let successor: TcpListener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
    original.set_nonblocking(true)?;
    successor.set_nonblocking(true)?;
    let original_routes: [SocketAddr; PEERS] = routes(original.local_addr()?);
    let next_routes: [SocketAddr; PEERS] = routes(successor.local_addr()?);
    let roots: [RootKey; PEERS] = [fixture_root(19)?, fixture_root(23)?, fixture_root(29)?];
    let mut original_bindings: [RetryBinding<'_>; PEERS] = bindings(&roots, original_routes)?;
    let mut next_bindings: [RetryBinding<'_>; PEERS] = bindings(&roots, next_routes)?;
    let runtime: Runtime = Builder::new_current_thread().enable_time().build()?;

    let control: RoundControl = RoundControl::new();
    let mut round: OrderedRound<'_> = tcp_round(&control, &mut original_bindings, original_routes)?;
    let (entered, observed): (oneshot::Sender<()>, oneshot::Receiver<()>) = oneshot::channel();
    let (release, released): (Sender<()>, Receiver<()>) = mpsc::channel();
    runtime.block_on(stop_waiting_round(
        &mut round,
        &control,
        original_routes[0],
        entered,
        observed,
        released,
    ))?;
    drop(round);
    let (_, retry): (&RootKey, &mut RetryState) =
        checked(original_bindings[0].selection(original_routes[0]))?;
    assert!(matches!(
        retry.check(Instant::now()),
        Err(ExchangeError::RetryInFlight)
    ));

    let contender: RoundControl = RoundControl::new();
    let mut busy: OrderedRound<'_> = tcp_round(&contender, &mut next_bindings, next_routes)?;
    assert!(matches!(
        runtime.block_on(async { busy.start_next() }),
        Err(RoundError::Exchange(ExchangeError::ConnectBusy))
    ));
    assert!(matches!(successor.accept(), Err(error) if error.kind() == io::ErrorKind::WouldBlock));
    drop(busy);

    let (mut abandoned, _): (TcpStream, SocketAddr) = original.accept()?;
    abandoned.set_nonblocking(true)?;
    let mut byte: [u8; 1] = [0];
    assert!(
        matches!(abandoned.read(&mut byte), Err(error) if error.kind() == io::ErrorKind::WouldBlock)
    );
    let release_at: Instant = Instant::now();
    release.send(())?;

    runtime.block_on(run_successor(&mut next_bindings, next_routes, &successor))?;
    assert!(matches!(
        retry.check(release_at),
        Err(ExchangeError::RetryDeferred { until }) if until >= release_at + FIRST_RETRY_WAIT
    ));
    abandoned.set_nonblocking(false)?;
    abandoned.set_read_timeout(Some(WATCHDOG))?;
    assert_eq!(abandoned.read(&mut byte)?, 0);
    Ok(())
}

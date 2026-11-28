// SPDX-License-Identifier: GPL-2.0-only
//! Real loopback controls for the ordered round's TCP worker handoff.

use super::{
    ExchangeError, ExchangeTransport, OrderedRound, PEERS, PendingConnect, ProbeProgress,
    RetryBinding, RetryRoundPeer, RetryState, RootKey, RoundControl, RoundError, Withdrawal,
    round_cutoff,
};
use crate::{
    native_network_job::{CUSTODY_FIXTURE, NativeJob, NativeJobError},
    roughtime::{
        exchange::{PendingExchange, RoundAuthority},
        interval::IntervalPolicy,
        round::recovery::{ControlledRecovery, RecoveryControl, RecoveryDatePolicy},
    },
};
use ed25519_dalek::SigningKey;
use std::{
    error::Error,
    fmt::Debug,
    io::{self, Read},
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream, UdpSocket},
    num::{NonZeroU32, NonZeroU64},
    sync::{
        MutexGuard,
        mpsc::{self, Receiver, RecvTimeoutError, Sender, TryRecvError},
    },
    task::Poll,
    time::{Duration, Instant},
};
use time_domain::{EraWindow, LeapDateScope, LeapKnowledge, bundled_leap_knowledge};
use tokio::{
    runtime::{Builder, Runtime},
    sync::oneshot,
    time::{sleep, sleep_until, timeout},
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
    recovery: Option<&'a RecoveryControl>,
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
    let mut round: OrderedRound<'_> = if let Some(parent) = recovery {
        let cutoff: Instant = checked(round_cutoff(WATCHDOG, WATCHDOG))?;
        checked(OrderedRound::start_bound_until(
            control,
            Some(parent),
            peers,
            cutoff,
            WATCHDOG,
        ))?
    } else {
        checked(OrderedRound::new_with_retry(
            control, peers, WATCHDOG, WATCHDOG,
        ))?
    };
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
    release: &Receiver<()>,
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
        gated_connection(peer, remaining, entered, &released)
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
        let mut round: OrderedRound<'_> = tcp_round(&control, None, bindings, endpoints)?;
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
    let _fixture: MutexGuard<'_, ()> = CUSTODY_FIXTURE
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
    let mut round: OrderedRound<'_> =
        tcp_round(&control, None, &mut original_bindings, original_routes)?;
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
        checked(original_bindings[0].selection(original_routes[0], ExchangeTransport::Udp))?;
    assert!(matches!(
        retry.check(Instant::now()),
        Err(ExchangeError::RetryInFlight)
    ));

    let contender: RoundControl = RoundControl::new();
    let mut busy: OrderedRound<'_> = tcp_round(&contender, None, &mut next_bindings, next_routes)?;
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

/// Which original authority acknowledges Stop before the worker claims dispatch.
#[derive(Clone, Copy)]
enum StopAuthority {
    /// The child round control.
    Round,
    /// The exact parent recovery control.
    Recovery,
}

/// Preserve a real UDP failure floor on the binding that will next be used for TCP.
///
/// # Errors
/// Returns fixture socket failure or a missing scheduled retry.
///
/// # Panics
/// Panics if the independently observed UDP send is incomplete.
fn prior_retry_floor(binding: &mut RetryBinding<'_>, endpoint: SocketAddr) -> TestResult<Instant> {
    let socket: UdpSocket = UdpSocket::bind(endpoint)?;
    socket.set_read_timeout(Some(WATCHDOG))?;
    let authority: RoundAuthority = RoundAuthority::new();
    let pending: PendingExchange<'_> = checked(PendingExchange::send_with_retry(
        &authority, binding, endpoint, WATCHDOG,
    ))?;
    let mut packet: [u8; 1036] = [0; 1036];
    let (length, _): (usize, SocketAddr) = socket.recv_from(&mut packet)?;
    assert_eq!(length, packet.len());
    let before_retirement: Instant = Instant::now();
    drop(pending);
    let (_, retry): (&RootKey, &mut RetryState) =
        checked(binding.selection(endpoint, ExchangeTransport::Tcp))?;
    match retry.check(before_retirement) {
        Err(ExchangeError::RetryDeferred { until }) => Ok(until),
        other => Err(format!("missing charged UDP retry floor: {other:?}").into()),
    }
}

/// Supply a valid date policy to issue the original recovery authority.
///
/// # Errors
/// Returns an unexpected refusal of the literal fixture policy.
fn worker_date_policy<'a>(calendar: &'a LeapKnowledge<'a>) -> TestResult<RecoveryDatePolicy<'a>> {
    let rate: NonZeroU32 = NonZeroU32::new(1_000_000_000).ok_or("zero fixture rate")?;
    let interval: IntervalPolicy = checked(IntervalPolicy::new(
        [rate; 2],
        Duration::ZERO,
        WATCHDOG,
        WATCHDOG,
    ))?;
    let ticks_per_second: i128 = 1_i128 << 32;
    let earliest: i128 = 3_900_000_000 * ticks_per_second;
    let scope: LeapDateScope = checked(LeapDateScope::new(EraWindow::new(
        earliest,
        earliest + ticks_per_second,
    )?))?;
    Ok(RecoveryDatePolicy::new(interval, scope, calendar))
}

/// Observe cancellation in the original worker before the round can retire it.
///
/// # Errors
/// Returns unexpected settlement or a fixture watchdog failure.
async fn join_dispatch_refusal(round: &mut OrderedRound<'_>) -> TestResult<()> {
    let pending: &mut PendingConnect<'_> = round
        .pending_connect
        .as_mut()
        .ok_or("missing reserved worker")?;
    timeout(WATCHDOG, async {
        loop {
            match pending.try_join() {
                Poll::Ready(Err(ExchangeError::Spent)) => return Ok(()),
                Poll::Ready(other) => {
                    return Err(format!("unexpected pre-dispatch settlement: {other:?}").into());
                }
                Poll::Pending => sleep(Duration::from_millis(1)).await,
            }
        }
    })
    .await?
}

/// Let the same pre-claim gate proceed without withdrawal, then charge one failed successor.
///
/// # Errors
/// Returns a watchdog, fixture, or unexpected round failure.
///
/// # Panics
/// Panics if the released worker skips the connector, sends before release, or stays active after Stop.
async fn charged_successor(
    bindings: &mut [RetryBinding<'_>; PEERS],
    endpoints: [SocketAddr; PEERS],
    listener: &TcpListener,
) -> TestResult<(Instant, Instant)> {
    let control: RoundControl = RoundControl::new();
    let mut round: OrderedRound<'_> = tcp_round(&control, None, bindings, endpoints)?;
    let (entered, observed): (oneshot::Sender<()>, oneshot::Receiver<()>) = oneshot::channel();
    let (release, released): (Sender<()>, Receiver<()>) = mpsc::channel();
    round.before_connect_dispatch.0 = Some(Box::new(move || {
        entered
            .send(())
            .map_err(|()| io::Error::other("successor observer disappeared"))?;
        released.recv_timeout(WATCHDOG).map_err(io::Error::other)
    }));
    let (called, calls): (Sender<()>, Receiver<()>) = mpsc::channel();
    let selected: SocketAddr = checked(round.start_next_with(move |peer, remaining| {
        called.send(()).map_err(io::Error::other)?;
        TcpStream::connect_timeout(&peer, remaining)
    }))?;
    assert_eq!(selected, endpoints[0]);
    timeout(WATCHDOG, observed).await??;
    assert!(matches!(calls.try_recv(), Err(TryRecvError::Empty)));
    assert!(matches!(listener.accept(), Err(error) if error.kind() == io::ErrorKind::WouldBlock));
    release.send(())?;
    observe_successor(&mut round, listener).await?;
    assert_eq!(calls.try_recv(), Ok(()));
    assert_eq!(calls.try_recv(), Err(TryRecvError::Disconnected));
    checked(control.withdraw(Withdrawal::Stop))?;
    let before: Instant = Instant::now();
    assert!(matches!(
        round.poll(),
        Err(RoundError::Withdrawn(Withdrawal::Stop))
    ));
    Ok((before, Instant::now()))
}

/// Deny the original dispatch through exactly one authority, then drive a healthy neighbor.
///
/// # Errors
/// Returns a fixture, watchdog or unexpected round refusal.
///
/// # Panics
/// Panics on work before the claim, early lane release, a changed floor or a retired connection.
fn stop_before_dispatch(owner: StopAuthority) -> TestResult<()> {
    let _fixture: MutexGuard<'_, ()> = CUSTODY_FIXTURE
        .lock()
        .map_err(|_| "native worker fixture unavailable")?;
    let listener: TcpListener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
    listener.set_nonblocking(true)?;
    let endpoint: SocketAddr = listener.local_addr()?;
    let endpoints: [SocketAddr; PEERS] = routes(endpoint);
    let roots: [RootKey; PEERS] = [fixture_root(31)?, fixture_root(37)?, fixture_root(41)?];
    let mut original: [RetryBinding<'_>; PEERS] = bindings(&roots, endpoints)?;
    let floor: Instant = prior_retry_floor(&mut original[0], endpoint)?;
    let runtime: Runtime = Builder::new_current_thread().enable_time().build()?;
    runtime.block_on(async { sleep_until(tokio::time::Instant::from_std(floor)).await });

    let control: RoundControl = RoundControl::new();
    let recovery: RecoveryControl = RecoveryControl::new();
    let calendar: LeapKnowledge<'static> = bundled_leap_knowledge()?;
    let _invocation: ControlledRecovery<'_> = checked(ControlledRecovery::new(
        &recovery,
        WATCHDOG,
        worker_date_policy(&calendar)?,
    ))?;
    let mut round: OrderedRound<'_> =
        tcp_round(&control, Some(&recovery), &mut original, endpoints)?;
    let (entered, observed): (Sender<()>, Receiver<()>) = mpsc::channel();
    let (release, released): (Sender<()>, Receiver<()>) = mpsc::channel();
    round.before_connect_dispatch.0 = Some(Box::new(move || {
        entered.send(()).map_err(io::Error::other)?;
        released.recv_timeout(WATCHDOG).map_err(io::Error::other)
    }));
    let (called, calls): (Sender<()>, Receiver<()>) = mpsc::channel();
    let selected: SocketAddr = checked(round.start_next_with(move |peer, remaining| {
        called.send(()).map_err(io::Error::other)?;
        TcpStream::connect_timeout(&peer, remaining)
    }))?;
    assert_eq!(selected, endpoint);
    observed.recv_timeout(WATCHDOG)?;
    match owner {
        StopAuthority::Round => checked(control.withdraw(Withdrawal::Stop))?,
        StopAuthority::Recovery => checked(recovery.withdraw(Withdrawal::Stop))?,
    }
    assert!(matches!(
        NativeJob::start("nevrotick-connect-contender", |_| ()),
        Err(NativeJobError::Busy)
    ));
    assert!(matches!(calls.try_recv(), Err(TryRecvError::Empty)));
    assert!(matches!(listener.accept(), Err(error) if error.kind() == io::ErrorKind::WouldBlock));

    // Polling or dropping the round first would independently cancel the worker.
    release.send(())?;
    assert_eq!(
        calls.recv_timeout(WATCHDOG),
        Err(RecvTimeoutError::Disconnected)
    );
    assert!(matches!(
        NativeJob::start("nevrotick-connect-unjoined-contender", |_| ()),
        Err(NativeJobError::Busy)
    ));
    runtime.block_on(join_dispatch_refusal(&mut round))?;
    assert!(matches!(
        round.poll(),
        Err(RoundError::Withdrawn(Withdrawal::Stop))
    ));
    assert!(round.receipt(0).is_none());
    assert!(matches!(listener.accept(), Err(error) if error.kind() == io::ErrorKind::WouldBlock));
    drop(round);

    let (_, retry): (&RootKey, &mut RetryState) =
        checked(original[0].selection(endpoint, ExchangeTransport::Tcp))?;
    let before_floor: Instant = floor
        .checked_sub(Duration::from_nanos(1))
        .ok_or("unrepresentable prior floor predecessor")?;
    assert!(matches!(
        retry.check(before_floor),
        Err(ExchangeError::RetryDeferred { until }) if until == floor
    ));
    checked(retry.check(floor))?;
    let (before, after): (Instant, Instant) =
        runtime.block_on(charged_successor(&mut original, endpoints, &listener))?;
    if after.duration_since(before) >= Duration::from_millis(500) {
        return Err(
            "successor retirement bracket overlaps the first and second retry floors".into(),
        );
    }
    let (_, retry): (&RootKey, &mut RetryState) =
        checked(original[0].selection(endpoint, ExchangeTransport::Tcp))?;
    let second_wait: Duration = Duration::from_millis(1_500);
    let earliest: Instant = before
        .checked_add(second_wait)
        .ok_or("second retry floor overflow")?;
    let latest: Instant = after
        .checked_add(second_wait)
        .ok_or("second retry floor overflow")?;
    assert!(matches!(
        retry.check(before),
        Err(ExchangeError::RetryDeferred { until }) if (earliest..=latest).contains(&until)
    ));
    Ok(())
}

/// Round Stop cancels the still-reserved native connector.
///
/// # Errors
/// Returns the first fixture or dispatch sequence failure.
#[test]
fn round_stop_revokes_reserved_connect_before_dispatch() -> TestResult<()> {
    stop_before_dispatch(StopAuthority::Round)
}

/// Parent Stop cancels the still-reserved native connector.
///
/// # Errors
/// Returns the first fixture or dispatch sequence failure.
#[test]
fn recovery_stop_revokes_reserved_connect_before_dispatch() -> TestResult<()> {
    stop_before_dispatch(StopAuthority::Recovery)
}

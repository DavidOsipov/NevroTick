// SPDX-License-Identifier: GPL-2.0-only
//! Same-transition clock observations through bound date admission.

use super::{
    ControlledDate, ExchangeError, ExchangeTransport, IntervalError, IntervalPolicy, OrderedRound,
    PEERS, ProbeProgress, RetryBinding, RetryRoundPeer, RetryState, RootKey, RoundControl,
    RoundStatus,
};
use crate::roughtime::{
    exchange::{PendingExchange, RoundAuthority, retry::RetryRoute},
    signed_reply,
};
use ed25519_dalek::SigningKey;
use ntp_wire::roughtime::{Request, decode_request};
use std::{
    error::Error,
    fmt::Debug,
    iter::Copied,
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    num::{NonZeroU32, NonZeroU64},
    slice::Iter,
    thread,
    time::{Duration, Instant},
};
use time_domain::{EraWindow, LeapDateScope, LeapKnowledge, bundled_leap_knowledge};

/// Literal fixture date in Unix seconds, safely inside the bundled calendar.
const MIDPOINT: u64 = 1_483_228_900;
/// Corresponding literal seconds since the NTP epoch.
const NTP_SECONDS: i128 = 3_692_217_700;
/// Bound the real round and its ordered test observations.
const BUDGET: Duration = Duration::from_secs(5);

/// Failure in fixture plumbing or an unexpected product outcome.
type TestResult<T> = Result<T, Box<dyn Error>>;

/// Controlled peer with independent keys and its actual loopback endpoint.
struct Operator {
    /// Actual loopback receiver for the selected endpoint.
    socket: UdpSocket,
    /// Numeric route assigned by the loopback stack.
    endpoint: SocketAddr,
    /// Public test seed's delegation signer.
    anchor: SigningKey,
    /// Separate test key for the response signature.
    delegated: SigningKey,
    /// Checked root handed to the real round.
    root: RootKey,
}

impl Operator {
    /// Bind a bounded loopback receiver and check its exact root key.
    ///
    /// # Errors
    /// Returns a socket or fixture root refusal.
    fn bind(anchor_seed: u8, delegated_seed: u8) -> TestResult<Self> {
        let socket: UdpSocket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0))?;
        socket.set_read_timeout(Some(Duration::from_secs(1)))?;
        let endpoint: SocketAddr = socket.local_addr()?;
        let anchor: SigningKey = SigningKey::from_bytes(&[anchor_seed; 32]);
        let delegated: SigningKey = SigningKey::from_bytes(&[delegated_seed; 32]);
        let root: RootKey = checked(RootKey::from_bytes(anchor.verifying_key().as_bytes()))?;
        Ok(Self {
            socket,
            endpoint,
            anchor,
            delegated,
            root,
        })
    }

    /// Observe the actual request and sign an overlapping interval for its exact bytes.
    ///
    /// # Errors
    /// Returns a socket, input or incomplete fixture send failure.
    fn respond(&self) -> TestResult<()> {
        let mut buffer: [u8; 1_037] = [0; 1_037];
        let (length, sender): (usize, SocketAddr) = self.socket.recv_from(&mut buffer)?;
        let frame: &[u8] = &buffer[..length];
        let request: Request<'_> = checked(decode_request(frame))?;
        let response: [u8; 476] = signed_reply::sign_reply(
            &self.anchor,
            &self.delegated,
            frame,
            *request.nonce(),
            MIDPOINT,
            20,
        )?;
        if self.socket.send_to(&response, sender)? != response.len() {
            return Err("incomplete calendar fixture response".into());
        }
        Ok(())
    }

    /// Bind only this operator's actual UDP route and root.
    ///
    /// # Errors
    /// Returns a route or binding refusal.
    fn binding(&self) -> TestResult<RetryBinding<'_>> {
        let route: RetryRoute = checked(RetryRoute::new(self.endpoint, ExchangeTransport::Udp))?;
        checked(RetryBinding::new_for_routes(&self.root, route, None))
    }
}

/// Preserve typed failures as diagnostics for unexpected fixture outcomes.
///
/// # Errors
/// Returns the supplied refusal with its original category in the diagnostic.
fn checked<T, E: Debug>(result: Result<T, E>) -> TestResult<T> {
    result.map_err(|error| format!("unexpected date-admission fixture outcome: {error:?}").into())
}

/// Charge a binding with one original send whose response never arrives.
///
/// # Errors
/// Returns the actual send failure or an unexpected retry disposition.
///
/// # Panics
/// Panics if the original request has an unexpected length.
fn seed_debt(binding: &mut RetryBinding<'_>, peer: &Operator) -> TestResult<Instant> {
    let authority: RoundAuthority = RoundAuthority::new();
    let pending: PendingExchange<'_> = checked(PendingExchange::send_with_retry(
        &authority,
        binding,
        peer.endpoint,
        BUDGET,
    ))?;
    let mut unused_request: [u8; 1_037] = [0; 1_037];
    let (length, _): (usize, SocketAddr) = peer.socket.recv_from(&mut unused_request)?;
    assert_eq!(length, 1_036);
    let before: Instant = Instant::now();
    drop(pending);
    let (_, retry): (&RootKey, &mut RetryState) =
        checked(binding.selection(peer.endpoint, ExchangeTransport::Udp))?;
    match retry.check(before) {
        Err(ExchangeError::RetryDeferred { until }) => Ok(until),
        other => Err(format!("missing original retry debt: {other:?}").into()),
    }
}

/// Name the independently selected final sample or its absence.
#[derive(Clone, Copy)]
enum ClosingObservation {
    /// One nanosecond before the original deadline.
    BeforeCutoff,
    /// Exactly at the exclusive original deadline.
    AtCutoff,
    /// Earlier than the otherwise timely entry sample.
    Reversed,
    /// No observation remains after the entry sample.
    Exhausted,
}

/// Admit a genuinely completed round using each prescribed two-read history.
///
/// # Errors
/// Returns the first setup, transport, clock fixture or unexpected round failure.
///
/// # Panics
/// Panics if a refused date is published, debt changes on refusal, or a timely date fails to reset it.
fn check_closing_observation(
    closing: ClosingObservation,
    expected: Option<IntervalError>,
) -> TestResult<()> {
    let peers: [Operator; PEERS] = [
        Operator::bind(11, 61)?,
        Operator::bind(22, 62)?,
        Operator::bind(33, 63)?,
    ];
    let mut bindings: [RetryBinding<'_>; PEERS] = [
        peers[0].binding()?,
        peers[1].binding()?,
        peers[2].binding()?,
    ];
    let floors: [Instant; PEERS] = [
        seed_debt(&mut bindings[0], &peers[0])?,
        seed_debt(&mut bindings[1], &peers[1])?,
        seed_debt(&mut bindings[2], &peers[2])?,
    ];
    thread::sleep(
        floors
            .into_iter()
            .max()
            .ok_or("no original retry floors")?
            .saturating_duration_since(Instant::now()),
    );

    let control: RoundControl = RoundControl::new();
    let mut round: OrderedRound<'_> = complete_fixture_round(&control, &mut bindings, &peers)?;
    check_date(&mut round, closing, expected)?;
    drop(round);

    for ((binding, peer), floor) in bindings.iter_mut().zip(&peers).zip(floors) {
        let (_, retry): (&RootKey, &mut RetryState) =
            checked(binding.selection(peer.endpoint, ExchangeTransport::Udp))?;
        let predecessor: Instant = floor
            .checked_sub(Duration::from_nanos(1))
            .ok_or("no floor predecessor")?;
        if expected.is_some() {
            assert!(
                matches!(retry.check(predecessor), Err(ExchangeError::RetryDeferred { until }) if until == floor)
            );
            checked(retry.check(floor))?;
        } else {
            checked(retry.check(predecessor))?;
        }
    }
    Ok(())
}

/// Complete the three traversals through the bound round's real send and receive path.
///
/// # Errors
/// Returns a fixture socket or unexpected round refusal.
///
/// # Panics
/// Panics if the selected peers or receipt progression change.
fn complete_fixture_round<'a>(
    control: &'a RoundControl,
    bindings: &'a mut [RetryBinding<'_>; PEERS],
    peers: &[Operator; PEERS],
) -> TestResult<OrderedRound<'a>> {
    let [first, second, third]: &mut [RetryBinding<'_>; PEERS] = bindings;
    let groups: [NonZeroU64; PEERS] = [
        NonZeroU64::MIN,
        NonZeroU64::MIN.saturating_add(1),
        NonZeroU64::MIN.saturating_add(2),
    ];
    let selected: [RetryRoundPeer<'_>; PEERS] = [
        checked(RetryRoundPeer::new(
            first,
            peers[0].endpoint,
            groups[0],
            groups[0],
        ))?,
        checked(RetryRoundPeer::new(
            second,
            peers[1].endpoint,
            groups[1],
            groups[1],
        ))?,
        checked(RetryRoundPeer::new(
            third,
            peers[2].endpoint,
            groups[2],
            groups[2],
        ))?,
    ];
    let mut round: OrderedRound<'_> = checked(OrderedRound::new_with_retry(
        control, selected, BUDGET, BUDGET,
    ))?;
    for (position, peer) in peers.iter().cycle().take(super::EXCHANGES).enumerate() {
        assert_eq!(checked(round.start_next())?, peer.endpoint);
        peer.respond()?;
        let progress: ProbeProgress = loop {
            match checked(round.poll())? {
                ProbeProgress::Waiting => thread::yield_now(),
                settled => break settled,
            }
        };
        assert_eq!(
            progress,
            if position == super::EXCHANGES - 1 {
                ProbeProgress::Complete
            } else {
                ProbeProgress::Accepted
            }
        );
    }
    assert_eq!(round.status(), RoundStatus::Complete);
    Ok(round)
}

/// Drive one private bounded observation sequence through live date admission.
///
/// # Errors
/// Returns invalid fixture setup or unrepresentable sample times.
///
/// # Panics
/// Panics if the outcome, consumed observations, or evidence disagree with the selected history.
fn check_date(
    round: &mut OrderedRound<'_>,
    closing: ClosingObservation,
    expected: Option<IntervalError>,
) -> TestResult<()> {
    let rate: NonZeroU32 = NonZeroU32::new(1_000_000_000).ok_or("invalid fixture rate")?;
    let policy: IntervalPolicy = checked(IntervalPolicy::new(
        [rate; 2],
        Duration::ZERO,
        BUDGET,
        Duration::from_secs(60),
    ))?;
    let ticks: i128 = 1_i128 << 32;
    let scope: LeapDateScope = LeapDateScope::new(EraWindow::new(
        (NTP_SECONDS - 30) * ticks,
        (NTP_SECONDS + 30) * ticks,
    )?)?;
    let calendar: LeapKnowledge<'static> = bundled_leap_knowledge()?;
    let cutoff: Instant = round.cutoff;
    let before: Instant = cutoff
        .checked_sub(Duration::from_nanos(2))
        .ok_or("no entry observation")?;
    let second: Instant = match closing {
        ClosingObservation::BeforeCutoff => cutoff
            .checked_sub(Duration::from_nanos(1))
            .ok_or("no timely final observation")?,
        ClosingObservation::Reversed => cutoff
            .checked_sub(Duration::from_nanos(3))
            .ok_or("no reversed observation")?,
        ClosingObservation::AtCutoff | ClosingObservation::Exhausted => cutoff,
    };
    let pair: [Instant; 2] = [before, second];
    let history: &[Instant] = if matches!(closing, ClosingObservation::Exhausted) {
        &pair[..1]
    } else {
        &pair
    };
    let mut script: Copied<Iter<'_, Instant>> = history.iter().copied();
    let mut reads: usize = 0;
    let admitted: Result<ControlledDate<'_>, IntervalError> =
        round.admit_date_observed(&policy, scope, &calendar, || {
            reads += 1;
            script.next()
        });
    assert_eq!(admitted.as_ref().err().copied(), expected);
    assert_eq!(
        reads, 2,
        "admission changed the number of clock observations"
    );
    assert_eq!(
        script.next(),
        None,
        "admission omitted a required observation"
    );
    if let Ok(date) = admitted {
        assert_eq!(date.agreement().evaluated_at(), before);
        assert_eq!(date.agreement().original_cutoff(), cutoff);
        assert_eq!(date.scope(), scope);
        assert!(date.agreement().receipt(super::EXCHANGES - 1).is_some());
    }
    assert_eq!(
        round.admit_date(&policy, scope, &calendar).err(),
        Some(IntervalError::RetryAdmission)
    );
    Ok(())
}

/// Select entirely timely, exactly expired, reversed and exhausted observations.
///
/// # Errors
/// Returns the first unexpected fixture or date-admission failure.
#[test]
fn date_admission_rechecks_clock_before_reset() -> TestResult<()> {
    let cases: [(ClosingObservation, Option<IntervalError>); 4] = [
        (ClosingObservation::BeforeCutoff, None),
        (ClosingObservation::AtCutoff, Some(IntervalError::Expired)),
        (
            ClosingObservation::Reversed,
            Some(IntervalError::CaptureOrder),
        ),
        (
            ClosingObservation::Exhausted,
            Some(IntervalError::ClockUnavailable),
        ),
    ];
    for (closing, expected) in cases {
        check_closing_observation(closing, expected)?;
    }
    Ok(())
}

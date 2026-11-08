// SPDX-License-Identifier: GPL-2.0-only
//! Independent real-UDP controls for the finite same-source receiver.
//!
//! REQ-8/BND-1 and CHECK-6/7/8/11/14/25/27 are exercised only within
//! the admitted Linux loopback experiment. Literal peer facts and exact capture
//! reconstruction confer no authenticated, physical or statistical admission.

use std::error::Error;
use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

use nevrotick_engine::{
    ControlHandle, ControlSchedule, ExperimentalProfile, MeasurementRefusal, Rejection,
    RelativeMeasurementRefusal, SeriesConfig, SeriesEnd, SeriesNotStartedCause, SeriesReport,
    SlotReport, SoftwareClock, Status, TransportFacts, execute_series, prepare_series,
};
use time_domain::{RationalSign, RoundedRationalSeconds};

#[path = "private/series_process.rs"]
mod series_process;

/// Preserve actual fixture and application failures across the joined worker.
type Check<T> = Result<T, Box<dyn Error + Send + Sync>>;
/// Bound every blocking fixture receive/write, including a missing query.
const PEER_WAIT: Duration = Duration::from_secs(3);
/// Observe absence of one extra send after the application has completed.
const QUIET_WAIT: Duration = Duration::from_millis(100);
/// Independent fixed peer date, inside the declared experimental window.
const PEER_SECOND: u64 = 4_000_000_000;
/// Exact Q32 scaling, independent of the production converters.
const Q32: i128 = 4_294_967_296;

/// Check actual fixture captures independently; failures are raw setup/model errors.
/// No product measurement, coherence calculation or profile getter supplies this oracle.
/// # Errors
/// Reject inadmissible observed chronology, date or declared wall-progress gaps.
fn fixture_capture_model(captures: &[SoftwareClock], require_dates: bool) -> Check<()> {
    let Some(baseline) = captures.first() else {
        return Err(io::Error::other("actual fixture retained no capture baseline").into());
    };
    let baseline_wall = ntp_nanoseconds(baseline.utc())?;
    let mut previous_after = None;
    for (index, capture) in captures.iter().enumerate() {
        let before = capture.monotonic();
        let after = capture.monotonic_after();
        if before > after || previous_after.is_some_and(|previous| previous > before) {
            return Err(
                io::Error::other("actual fixture capture chronology is inadmissible").into(),
            );
        }
        previous_after = Some(after);
        let wall = ntp_nanoseconds(capture.utc())?;
        if require_dates && !(3_000_000_000_000_000_000..=4_500_000_000_999_999_999).contains(&wall)
        {
            return Err(
                io::Error::other("actual fixture wall date is outside its declaration").into(),
            );
        }
        if index == 0 {
            continue;
        }
        let lower = i128::try_from(
            before
                .checked_duration_since(baseline.monotonic_after())
                .ok_or_else(|| io::Error::other("actual fixture lower interval is unavailable"))?
                .as_nanos(),
        )?;
        let upper = i128::try_from(
            after
                .checked_duration_since(baseline.monotonic())
                .ok_or_else(|| io::Error::other("actual fixture upper interval is unavailable"))?
                .as_nanos(),
        )?;
        let delta = wall
            .checked_sub(baseline_wall)
            .ok_or_else(|| io::Error::other("actual fixture wall interval is unrepresentable"))?;
        let gap = if delta < lower {
            lower.checked_sub(delta)
        } else if delta > upper {
            delta.checked_sub(upper)
        } else {
            Some(0)
        }
        .ok_or_else(|| io::Error::other("actual fixture model gap is unrepresentable"))?;
        if gap > 30_000_000_000 {
            return Err(io::Error::other(
                "actual fixture exceeded its declared wall-progress allowance",
            )
            .into());
        }
    }
    Ok(())
}

/// Establish the fixture's actual local model before an expected product success.
/// Dates and tolerance are the fixed declarations used by this fixture's builder.
/// # Errors
/// Preserve actual capture/date/model prerequisites as raw fixture failures.
fn fixture_measurement_prerequisites(facts: &TransportFacts, require_dates: bool) -> Check<()> {
    let captures = [facts.send_before(), facts.send_after(), facts.read_return()];
    if facts.processing() < captures[2].monotonic_after() {
        return Err(
            io::Error::other("actual fixture processing precedes receive completion").into(),
        );
    }
    fixture_capture_model(&captures, require_dates)
}

/// Assert an independently valid pure product result without formatting its cause.
/// # Errors
/// Keep the original typed result for the existing propagation path.
/// # Panics
/// Reject an unexpected refusal using only the code-owned static fact.
fn product_ok<T, E>(result: Result<T, E>, fact: &'static str) -> Result<T, E> {
    assert!(result.is_ok(), "{fact}");
    result
}

/// Return a fixed failed fact without formatting request packets or tokens.
///
/// # Errors
/// Return the supplied fixed message when the independent condition is false.
fn check(condition: bool, fact: &'static str) -> io::Result<()> {
    if condition {
        Ok(())
    } else {
        series_process::semantic_check_failed(fact);
        Err(io::Error::other(fact))
    }
}

/// Finite peer scripts; every receive is bounded and every worker is joined.
#[derive(Clone, Copy)]
enum PeerCase {
    /// Reply to each of the three planned requests once.
    Healthy,
    /// Precede the first reply with bad length and the second with the old token.
    RejectedNeighbors,
    /// Withhold only the middle response and wait for the final planned query.
    MiddleTimeout,
    /// Publish stop only after the second actual query has arrived.
    Stop,
    /// Change the epoch only after the second actual query has arrived.
    Invalidate,
    /// Withhold the second response until the whole-series deadline ends it.
    Deadline,
}

/// Actual request observation; deliberately has no packet/token debug formatter.
struct Query {
    /// Exact bounded transmitted request, kept private to these assertions.
    packet: [u8; 48],
    /// Actual source endpoint observed by the independently owned peer socket.
    client: SocketAddr,
}

/// Bounded native observations retained after the fixture worker is reaped.
struct PeerObservations {
    /// The one actual destination configured for every planned request.
    endpoint: SocketAddr,
    /// At most the three planned actual requests, without a packet log.
    queries: [Option<Query>; 3],
}

/// Construct an ordinary server response from literal facts and one live nonce.
/// The peer processing interval is exactly one eighth second; it need not fit
/// the loopback elapsed time and grants no physically plausible sample claim.
fn reply(query: &[u8; 48], index: u8) -> [u8; 48] {
    let second = PEER_SECOND + u64::from(index);
    let receive = (second << 32) | 0x4000_0000;
    let transmit = (second << 32) | 0x6000_0000;
    let mut bytes = [0; 48];
    bytes[0] = 0x24; // LI=0, NTPv4, server mode.
    bytes[1] = 2;
    bytes[24..32].copy_from_slice(&query[40..48]);
    bytes[32..40].copy_from_slice(&receive.to_be_bytes());
    bytes[40..48].copy_from_slice(&transmit.to_be_bytes());
    bytes
}

/// Send one complete bounded datagram and reject anomalous short fixture writes.
///
/// # Errors
/// Preserve native send failure or incomplete packet delivery to the socket API.
fn send(peer: &UdpSocket, bytes: &[u8], client: SocketAddr) -> Check<()> {
    if peer.send_to(bytes, client)? != bytes.len() {
        return Err(io::Error::other("fixture native send was short").into());
    }
    Ok(())
}

/// Observe no extra query after the finite application path has completed.
///
/// # Errors
/// Reject any additional datagram or an unexpected native receive failure.
fn no_extra_query(peer: &UdpSocket) -> Check<()> {
    peer.set_read_timeout(Some(QUIET_WAIT))?;
    let mut bytes = [0; 49];
    match peer.recv_from(&mut bytes) {
        Err(error)
            if matches!(
                error.kind(),
                io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
            ) =>
        {
            Ok(())
        }
        Err(error) => Err(error.into()),
        Ok(_) => Err(io::Error::other("unexpected additional series query").into()),
    }
}

/// Serve one finite series with independent packet facts and event-driven control.
///
/// # Errors
/// Preserve setup/native/control errors or a violated literal query contract.
fn serve(
    peer: &UdpSocket,
    case: PeerCase,
    control: &ControlHandle,
    application_done: &Receiver<()>,
) -> Check<PeerObservations> {
    let endpoint = peer.local_addr()?;
    let mut queries: [Option<Query>; 3] = [None, None, None];
    let count = match case {
        PeerCase::Healthy | PeerCase::RejectedNeighbors | PeerCase::MiddleTimeout => 3,
        PeerCase::Stop | PeerCase::Invalidate | PeerCase::Deadline => 2,
    };
    let mut previous_reply: Option<[u8; 48]> = None;
    let mut seen_tokens = [[0; 8]; 3];
    for (index, observation) in queries.iter_mut().enumerate().take(count) {
        let mut packet = [0; 49];
        let (length, client) = peer.recv_from(&mut packet)?;
        check(length == 48, "query was not exactly one 48-byte packet")?;
        check(packet[0] == 0x23, "query was not an NTPv4 client request")?;
        check(
            packet[1..40].iter().all(|byte| *byte == 0),
            "query included an undeclared payload",
        )?;
        check(
            client.ip().is_loopback() && client.port() != 0,
            "query did not use an actual loopback client endpoint",
        )?;
        check(
            packet[40..48].iter().any(|byte| *byte != 0),
            "query correlation token was zero",
        )?;
        for earlier in &seen_tokens[..index] {
            check(
                packet[40..48] != *earlier,
                "series reused an earlier request token",
            )?;
        }
        seen_tokens[index].copy_from_slice(&packet[40..48]);
        let packet: [u8; 48] = packet[..48].try_into()?;
        let bytes = reply(&packet, u8::try_from(index)?);
        match (case, index) {
            (PeerCase::RejectedNeighbors, 0) => {
                send(peer, &bytes[..47], client)?;
                send(peer, &bytes, client)?;
            }
            (PeerCase::RejectedNeighbors, 1) => {
                let old = previous_reply
                    .as_ref()
                    .ok_or_else(|| io::Error::other("missing prior reply fixture"))?;
                // Replay into the new actual endpoint, rather than the old port.
                send(peer, old, client)?;
                send(peer, &bytes, client)?;
            }
            (PeerCase::Stop, 1) => control.stop(),
            (PeerCase::Invalidate, 1) => control.invalidate()?,
            (PeerCase::MiddleTimeout | PeerCase::Deadline, 1) => {}
            _ => send(peer, &bytes, client)?,
        }
        previous_reply = Some(bytes);
        *observation = Some(Query { packet, client });
    }
    application_done.recv_timeout(PEER_WAIT)?;
    no_extra_query(peer)?;
    Ok(PeerObservations { endpoint, queries })
}

/// Supply an explicit date/model declaration independently of captured host time.
/// This broad experimental interval is not a true-date or physical-error source.
///
/// # Errors
/// Forward the checked result after asserting the fixed input is admitted.
/// # Panics
/// Reject refusal of the independently valid laboratory declaration.
fn profile() -> Result<ExperimentalProfile, MeasurementRefusal> {
    let declared = ExperimentalProfile::new(3_000_000_000, 4_500_000_000, 30_000_000_000);
    assert!(
        declared.is_ok(),
        "independently valid experimental profile was refused"
    );
    declared
}

/// Run the actual public entrypoint and reap the sole fixture worker on all exits.
/// At most one fixture socket/thread is added; no sleep schedules application work.
///
/// # Errors
/// Preserve application and fixture failures; join even after application failure.
fn run_case(case: PeerCase, with_profile: bool) -> Check<(SeriesReport, PeerObservations)> {
    let peer = UdpSocket::bind("127.0.0.1:0")?;
    run_bound_case(peer, case, with_profile)
}

/// Run one owned pre-bound loopback peer through the same finite public fixture.
///
/// # Errors
/// Preserve fixture setup, application and joined-worker failures without fallback.
/// # Panics
/// Reject refusal of independently valid configuration and schedule values.
fn run_bound_case(
    peer: UdpSocket,
    case: PeerCase,
    with_profile: bool,
) -> Check<(SeriesReport, PeerObservations)> {
    peer.set_read_timeout(Some(PEER_WAIT))?;
    peer.set_write_timeout(Some(PEER_WAIT))?;
    let (request_timeout, series_timeout) = if matches!(case, PeerCase::Deadline) {
        (Duration::from_millis(600), Duration::from_millis(600))
    } else {
        (Duration::from_millis(600), Duration::from_secs(3))
    };
    let config = SeriesConfig::new(peer.local_addr()?, request_timeout, series_timeout);
    assert!(
        config.is_ok(),
        "independently valid series configuration was refused"
    );
    let config = config?;
    let config = if with_profile {
        config.with_experimental_profile(profile()?)
    } else {
        config
    };
    let (prepared, control) = prepare_series(&config)?;
    let schedule = ControlSchedule::new(None, None);
    assert!(
        schedule.is_ok(),
        "independently valid control schedule was refused"
    );
    let schedule = schedule?;
    let (application_done, application_returned) = mpsc::sync_channel(1);
    thread::scope(|scope| {
        let worker = scope.spawn(move || serve(&peer, case, &control, &application_returned));
        let application = execute_series(prepared, &schedule);
        // Notify even on an application error, then reap before propagating it.
        let notified = application_done.send(());
        let fixture = worker
            .join()
            .map_err(|_| io::Error::other("owned peer worker panicked"))?;
        let report = application?;
        let observations = fixture?;
        notified?;
        Ok((report, observations))
    })
}

/// Borrow the actual terminal record without inventing a missing observation.
///
/// # Errors
/// Reject an unexpected unstarted position.
fn slot(report: &SeriesReport, index: usize) -> Check<&SlotReport> {
    let record = report.attempts()[index].report();
    check(record.is_some(), "expected terminal series attempt")?;
    record.ok_or_else(|| io::Error::other("expected terminal series attempt").into())
}

/// Require actual completed transport facts; numerical acceptance is separate.
///
/// # Errors
/// Reject an unexpected terminal record without correlated header facts.
fn facts(report: &SeriesReport, index: usize) -> Check<&TransportFacts> {
    let retained = slot(report, index)?.facts();
    check(retained.is_some(), "expected retained transport facts")?;
    retained.ok_or_else(|| io::Error::other("expected retained transport facts").into())
}

/// Check report indexing, zero admission, cleanup and the actually observed sockets.
/// Successful host rebinding is limited socket-drop evidence, not Windows release.
///
/// # Errors
/// Preserve rebind/setup failure or a mismatched endpoint/terminal report contract.
fn common_report_checks(report: &SeriesReport, observed: &PeerObservations) -> Check<()> {
    if report.primary_failure().is_some()
        || report.stop_cleanup_failure().is_some()
        || report.invalidate_cleanup_failure().is_some()
        || report.attempts().iter().any(|attempt| {
            attempt
                .report()
                .is_some_and(|record| record.status() == Status::Io || record.io_cause().is_some())
        })
    {
        return Err(io::Error::other(
            "actual series fixture retained a driver or native I/O cause",
        )
        .into());
    }
    check(
        report.peer_endpoint() == observed.endpoint,
        "series changed its peer",
    )?;
    check(
        report.admitted_sample_count() == 0,
        "series admitted a sample",
    )?;
    check(
        report.eligible_sample_count() == 0,
        "series made a sample eligible",
    )?;
    check(
        report.primary_failure().is_none(),
        "unexpected primary series failure",
    )?;
    check(
        report.stop_cleanup_failure().is_none() && report.invalidate_cleanup_failure().is_none(),
        "series hid an owned timer cleanup failure",
    )?;
    for (index, attempt) in report.attempts().iter().enumerate() {
        check(
            usize::from(attempt.index()) == index,
            "wrong attempt position",
        )?;
        match (attempt.report(), observed.queries[index].as_ref()) {
            (Some(record), Some(query)) => {
                check(
                    usize::from(record.slot()) == index,
                    "wrong terminal slot index",
                )?;
                check(record.sent(), "observed query was not recorded as sent")?;
                check(
                    record.local_endpoint() == query.client,
                    "report lost the actual socket endpoint",
                )?;
                if let Some(retained) = record.facts() {
                    check(
                        retained.peer_endpoint() == observed.endpoint
                            && retained.local_endpoint() == query.client,
                        "retained facts changed actual native endpoints",
                    )?;
                    check(
                        retained
                            .header()
                            .origin_timestamp()
                            .bits()
                            .to_be_bytes()
                            .as_slice()
                            == &query.packet[40..48],
                        "retained header lost its actual request correlation",
                    )?;
                }
                check(
                    attempt.not_started_cause().is_none(),
                    "started request was unstarted",
                )?;
                let released = UdpSocket::bind(record.local_endpoint())?;
                drop(released);
            }
            (None, None) => check(
                attempt.not_started_cause().is_some(),
                "missing attempt had no terminal cause",
            )?,
            _ => return Err(io::Error::other("query/report presence mismatch").into()),
        }
    }
    Ok(())
}

/// Convert retained nominal wall readings independently using the known epoch.
///
/// # Errors
/// Reject an unsupported pre-Unix fixture host or an unrepresentable duration.
fn ntp_nanoseconds(value: SystemTime) -> Check<i128> {
    let unix = i128::try_from(value.duration_since(SystemTime::UNIX_EPOCH)?.as_nanos())?;
    Ok(unix + 2_208_988_800_000_000_000)
}

/// Derive exact phase and signed raw RTT directly from peer bits and captures.
/// No production measurement/conversion function supplies these expected facts.
/// Return the independently derived phase numerator for later relative checks.
///
/// # Errors
/// Preserve unsupported wall-time conversion or a violated exact result.
fn check_exact_measurement(fact: &TransportFacts, index: u8) -> Check<i128> {
    let second = PEER_SECOND + u64::from(index);
    let receive = (second << 32) | 0x4000_0000;
    let transmit = (second << 32) | 0x6000_0000;
    check(
        fact.peer_receive_bits() == receive,
        "retained peer receive bits changed",
    )?;
    check(
        fact.peer_transmit_bits() == transmit,
        "retained peer transmit bits changed",
    )?;
    let header = fact.header();
    let header_receive = u64::from(header.receive_timestamp().seconds());
    let header_transmit = u64::from(header.transmit_timestamp().seconds());
    check(
        header.version() == 4
            && header.mode() == 4
            && header.leap_indicator() == 0
            && header.stratum() == 2
            && header.receive_timestamp().bits() != 0
            && header.transmit_timestamp().bits() != 0
            && (3_000_000_000..=4_500_000_000).contains(&header_receive)
            && (3_000_000_000..=4_500_000_000).contains(&header_transmit),
        "series retained header changed fixed ordinary peer facts",
    )?;
    fixture_measurement_prerequisites(fact, true)?;
    let first = ntp_nanoseconds(fact.send_before().utc())?;
    let fourth = ntp_nanoseconds(fact.read_return().utc())?;
    let phase =
        (i128::from(receive) + i128::from(transmit)) * 1_000_000_000 - (first + fourth) * Q32;
    let round_trip =
        (fourth - first) * Q32 - (i128::from(transmit) - i128::from(receive)) * 1_000_000_000;
    let measurement = product_ok(
        fact.measurement(),
        "valid original series measurement was refused",
    )?;
    check(
        measurement.nominal_phase().numerator() == phase
            && measurement.nominal_phase().denominator() == 8_589_934_592,
        "nominal phase did not reconstruct from actual captures",
    )?;
    check(
        measurement.nominal_round_trip().numerator() == round_trip
            && measurement.nominal_round_trip().denominator() == 4_294_967_296,
        "signed raw RTT did not reconstruct from actual captures",
    )?;
    check(
        measurement.acquisition_age()
            == fact
                .processing()
                .duration_since(fact.read_return().monotonic_after()),
        "original acquisition age was refreshed or changed",
    )?;
    check(
        measurement.declared_limit_ns() == 30_000_000_000,
        "original experimental allowance was replaced",
    )?;
    Ok(phase)
}

/// Encode the exact expected midpoint as half-nanosecond ticks from real origin.
///
/// # Errors
/// Reject a capture that precedes the declared actual series origin.
fn expected_reference(origin: Instant, fact: &TransportFacts) -> Check<u128> {
    let before = fact
        .send_before()
        .monotonic()
        .checked_duration_since(origin)
        .ok_or_else(|| io::Error::other("send capture preceded actual series origin"))?;
    let read = fact
        .read_return()
        .monotonic()
        .checked_duration_since(origin)
        .ok_or_else(|| io::Error::other("receive capture preceded actual series origin"))?;
    Ok(before.as_nanos() + read.as_nanos())
}

/// Compare an exact signed rational tuple without using production arithmetic.
///
/// # Errors
/// Reject changed sign, numerator or units; zero independently requires positive sign.
fn check_ratio(result: RoundedRationalSeconds, numerator: i128, denominator: u64) -> Check<()> {
    let sign = if numerator < 0 {
        RationalSign::Negative
    } else {
        RationalSign::Positive
    };
    check(
        result.sign() == sign
            && result.numerator_magnitude() == numerator.unsigned_abs()
            && result.denominator().get() == denominator,
        "relative result lost exact signed rational units",
    )?;
    Ok(())
}

/// Three actual requests share one origin but retain distinct exchange ownership.
/// Reconstruct every actual midpoint, including its observed half-bit, and exact
/// nominal peer/capture arithmetic without imposing physical timing assumptions.
///
/// # Errors
/// Preserve fixture, transport, model and exact common-origin reconstruction failures.
fn healthy_report_checks(report: &SeriesReport, observed: &PeerObservations) -> Check<()> {
    common_report_checks(report, observed)?;
    check(
        report.end() == SeriesEnd::Completed,
        "three attempts did not complete",
    )?;
    for index in 0..3 {
        let record = slot(report, index)?;
        check(
            record.status() == Status::Header,
            "valid ordinary header was refused",
        )?;
        check(
            record.received() == 1 && record.rejected() == 0,
            "wrong receive count",
        )?;
        let row = facts(report, index)?;
        let row_phase = check_exact_measurement(row, u8::try_from(index)?)?;
        let row_ticks = expected_reference(report.origin(), row)?;
        for anchor_index in 0..3 {
            let anchor = facts(report, anchor_index)?;
            let anchor_phase = check_exact_measurement(anchor, u8::try_from(anchor_index)?)?;
            check(
                row.same_exchange(anchor) == (index == anchor_index),
                "distinct requests shared exchange identity",
            )?;
            let view = product_ok(
                row.relative_to(anchor),
                "valid same-run relative view was refused",
            )?;
            let anchor_ticks = expected_reference(report.origin(), anchor)?;
            let row_reference = view.row_reference();
            let anchor_reference = view.anchor_reference();
            check(
                u128::from(row_reference.whole_nanoseconds()) == row_ticks / 2
                    && row_reference.has_half_nanosecond() == (row_ticks % 2 != 0)
                    && u128::from(anchor_reference.whole_nanoseconds()) == anchor_ticks / 2
                    && anchor_reference.has_half_nanosecond() == (anchor_ticks % 2 != 0),
                "actual common-origin midpoint lost a half tick",
            )?;
            check_ratio(
                view.reference_delta_seconds(),
                i128::try_from(row_ticks)? - i128::try_from(anchor_ticks)?,
                2_000_000_000,
            )?;
            check_ratio(
                view.phase_delta_seconds(),
                row_phase - anchor_phase,
                8_589_934_592_000_000_000,
            )?;
            check(
                view.row().same_exchange(row) && view.anchor().same_exchange(anchor),
                "relative view replaced historical facts",
            )?;
        }
    }
    Ok(())
}

/// Three IPv4 requests retain actual distinct ownership and one exact origin.
///
/// # Errors
/// Preserve fixture, transport, model and independent report assertion failures.
#[test]
fn three_real_requests_retain_distinct_identity_and_one_exact_origin() -> Check<()> {
    if series_process::parent_or_child(
        "three_real_requests_retain_distinct_identity_and_one_exact_origin",
    )? {
        return Ok(());
    }
    let (report, observed) = run_case(PeerCase::Healthy, true)?;
    healthy_report_checks(&report, &observed)
}

/// The admitted IPv6 loopback entrypoint obeys the same three-request contract.
/// An unavailable IPv6 fixture is an error; this case never skips or uses IPv4.
///
/// # Errors
/// Preserve IPv6 bind/setup, transport, model and independent assertion failures.
#[test]
fn ipv6_real_requests_retain_distinct_identity_and_one_exact_origin() -> Check<()> {
    if series_process::parent_or_child(
        "ipv6_real_requests_retain_distinct_identity_and_one_exact_origin",
    )? {
        return Ok(());
    }
    let peer = UdpSocket::bind("[::1]:0")?;
    let (report, observed) = run_bound_case(peer, PeerCase::Healthy, true)?;
    check(
        observed.endpoint.is_ipv6(),
        "fixture did not retain its IPv6 peer",
    )?;
    healthy_report_checks(&report, &observed)
}

/// Malformed and prior-token packets do not consume a request; a correlated
/// ordinary header consumes it even when no experimental model is available.
///
/// # Errors
/// Preserve actual UDP/fixture failures or a changed rejection/refusal contract.
#[test]
fn unavailable_measurements_still_consume_once_after_rejected_neighbors() -> Check<()> {
    if series_process::parent_or_child(
        "unavailable_measurements_still_consume_once_after_rejected_neighbors",
    )? {
        return Ok(());
    }
    let (report, observed) = run_case(PeerCase::RejectedNeighbors, false)?;
    common_report_checks(&report, &observed)?;
    check(
        report.end() == SeriesEnd::Completed,
        "refusal stopped later attempts",
    )?;
    for (index, rejection) in [Some(Rejection::Length), Some(Rejection::Origin), None]
        .into_iter()
        .enumerate()
    {
        let record = slot(&report, index)?;
        check(
            record.status() == Status::Header,
            "valid neighbor was not consumed",
        )?;
        let rejected = u16::from(rejection.is_some());
        check(
            record.rejected() == rejected && record.received() == rejected + 1,
            "invalid datagram did not consume exactly one receive budget entry",
        )?;
        check(
            record.last_rejection() == rejection,
            "wrong literal rejection reason",
        )?;
        let retained = facts(&report, index)?;
        check(
            retained.experimental_profile().is_none(),
            "profile was inferred from host",
        )?;
        check(
            matches!(retained.measurement(), Err(MeasurementRefusal::Unavailable)),
            "missing declaration became a numerical measurement",
        )?;
        check(
            matches!(
                retained.relative_to(retained),
                Err(RelativeMeasurementRefusal::RowMeasurement(
                    MeasurementRefusal::Unavailable
                ))
            ),
            "relative view supplied a missing measurement",
        )?;
    }
    Ok(())
}

/// Ordinary loss consumes the middle position and allows exactly one final attempt.
/// The first acquisition keeps its original capture, payload and processing age.
///
/// # Errors
/// Preserve fixture/native failures or a changed timeout/history contract.
#[test]
fn middle_request_timeout_preserves_earlier_facts_and_starts_final_attempt() -> Check<()> {
    if series_process::parent_or_child(
        "middle_request_timeout_preserves_earlier_facts_and_starts_final_attempt",
    )? {
        return Ok(());
    }
    let (report, observed) = run_case(PeerCase::MiddleTimeout, true)?;
    common_report_checks(&report, &observed)?;
    check(
        report.end() == SeriesEnd::Completed,
        "request timeout ended whole series",
    )?;
    check(
        slot(&report, 0)?.status() == Status::Header,
        "first reply was lost",
    )?;
    check(
        slot(&report, 1)?.status() == Status::Timeout,
        "silence was not a timeout",
    )?;
    check(
        slot(&report, 1)?.facts().is_none(),
        "timeout invented header facts",
    )?;
    check(
        slot(&report, 2)?.status() == Status::Header,
        "final attempt was not completed",
    )?;
    let first = facts(&report, 0)?;
    let last = facts(&report, 2)?;
    check_exact_measurement(first, 0)?;
    check_exact_measurement(last, 2)?;
    check(
        first.processing() < last.send_before().monotonic(),
        "earlier acquisition was relabelled after a later timeout",
    )?;
    let age = product_ok(
        first.measurement(),
        "valid retained series measurement was refused",
    )?
    .acquisition_age();
    let view = product_ok(
        first.relative_to(last),
        "valid timeout-neighbor relative view was refused",
    )?;
    check(
        product_ok(
            view.row().measurement(),
            "valid relative-row measurement was refused",
        )?
        .acquisition_age()
            == age,
        "choosing a later anchor refreshed an old acquisition",
    )?;
    Ok(())
}

/// Check terminal control after an actual second send while preserving first facts.
///
/// # Errors
/// Preserve native/fixture failures or a missing retirement/not-started distinction.
fn terminal_case(
    case: PeerCase,
    end: SeriesEnd,
    status: Status,
    expected_not_started: SeriesNotStartedCause,
) -> Check<()> {
    let (report, observed) = run_case(case, true)?;
    common_report_checks(&report, &observed)?;
    check(report.end() == end, "wrong terminal series cause")?;
    check(
        slot(&report, 0)?.status() == Status::Header,
        "control rewrote prior completion",
    )?;
    check_exact_measurement(facts(&report, 0)?, 0)?;
    let active = slot(&report, 1)?;
    check(
        active.status() == status,
        "active request was not terminally retired",
    )?;
    check(
        active.facts().is_none(),
        "terminal retirement invented a reply",
    )?;
    let final_attempt = &report.attempts()[2];
    check(
        final_attempt.report().is_none()
            && final_attempt.not_started_cause() == Some(expected_not_started),
        "later attempt lost its explicit not-started cause",
    )?;
    let first = facts(&report, 0)?;
    check(
        report.done_elapsed_ns()
            > first
                .processing()
                .duration_since(report.origin())
                .as_nanos(),
        "historical processing time was replaced with series completion time",
    )?;
    Ok(())
}

/// A stop acknowledged after the second send cannot rewrite first facts or send third.
///
/// # Errors
/// Preserve actual fixture/control/native failures or changed terminal semantics.
#[test]
fn stop_preserves_first_completion_and_marks_third_not_started() -> Check<()> {
    if series_process::parent_or_child(
        "stop_preserves_first_completion_and_marks_third_not_started",
    )? {
        return Ok(());
    }
    terminal_case(
        PeerCase::Stop,
        SeriesEnd::Stopped,
        Status::Stopped,
        SeriesNotStartedCause::Stopped,
    )
}

/// Invalidation after the second send retires current authority and retains history.
///
/// # Errors
/// Preserve actual fixture/control/native failures or changed epoch semantics.
#[test]
fn invalidation_preserves_first_completion_and_marks_third_not_started() -> Check<()> {
    if series_process::parent_or_child(
        "invalidation_preserves_first_completion_and_marks_third_not_started",
    )? {
        return Ok(());
    }
    terminal_case(
        PeerCase::Invalidate,
        SeriesEnd::Stale,
        Status::Stale,
        SeriesNotStartedCause::Stale,
    )
}

/// The original whole-series deadline bounds a silent later request without reset.
///
/// # Errors
/// Preserve actual fixture/native failures or changed whole-series deadline semantics.
#[test]
fn total_deadline_preserves_first_completion_and_prevents_third_send() -> Check<()> {
    if series_process::parent_or_child(
        "total_deadline_preserves_first_completion_and_prevents_third_send",
    )? {
        return Ok(());
    }
    terminal_case(
        PeerCase::Deadline,
        SeriesEnd::SeriesDeadline,
        Status::Timeout,
        SeriesNotStartedCause::SeriesDeadline,
    )
}

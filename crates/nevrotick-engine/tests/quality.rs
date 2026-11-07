// SPDX-License-Identifier: GPL-2.0-only
//! Independent real-UDP controls for conditional exchange-quality evaluation.
//!
//! REQ-8/BND-1 and CHECK-6/7/8/11/14/25/27 retain their controlled Linux scope.
//! Explicit numerical declarations below supply neither calibrated capture
//! accuracy nor production date, source, scale, currentness or eligibility.

use std::error::Error;
use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, SystemTime};

use nevrotick_engine::{
    ConditionalQuality, ControlSchedule, DeclaredFrequencyInterval, DeclaredQualityAssumptions,
    DeclaredQualityLimits, ExperimentalProfile, MeasurementRefusal, QualityRefusal, SeriesConfig,
    SeriesEnd, SeriesReport, SlotReport, Status, TransportFacts, evaluate_conditional_quality,
    execute_series, prepare_series,
};
use time_domain::{RationalSign, RoundedRationalSeconds};

/// Retain setup, application and joined-peer errors without flattening them.
type Check<T> = Result<T, Box<dyn Error + Send + Sync>>;
/// Bound each fixture socket operation and its application-completion notice.
const PEER_WAIT: Duration = Duration::from_secs(3);
/// Bound the post-completion observation of an unexpected fourth query.
const QUIET_WAIT: Duration = Duration::from_millis(100);
/// Independent literal peer seconds, admitted by the explicit experimental scope.
const PEER_SECOND: u64 = 4_000_000_000;
/// Exact Q32 scale used only by the independent capture reconstruction.
const Q32: i128 = 4_294_967_296;
/// Exact seconds denominator of corrected nominal local/raw-RTT numerators.
const NOMINAL_DENOMINATOR: u64 = 4_294_967_296_000_000_000;
/// Literal peer receive fraction, one quarter second.
const RECEIVE_FRACTION: u64 = 0x4000_0000;
/// Literal peer transmit fraction, three eighths second.
const TRANSMIT_FRACTION: u64 = 0x6000_0000;

/// Record a bounded semantic failure without exposing captured packets or tokens.
/// # Errors
/// Reject precisely the independent condition named by the static fact.
fn check(condition: bool, fact: &'static str) -> io::Result<()> {
    if condition {
        Ok(())
    } else {
        Err(io::Error::other(fact))
    }
}

/// Observe the three actual client endpoints without retaining correlation bytes.
struct PeerObservations {
    /// Actual destination configured in the checked series.
    endpoint: SocketAddr,
    /// Actual source endpoint of each independently observed query.
    clients: [Option<SocketAddr>; 3],
}

/// Reply with one fixed peer interval and distinct literal header quality claims.
/// The peer interval need not be physically plausible for the captured loopback.
fn reply(query: &[u8; 48]) -> [u8; 48] {
    let receive = (PEER_SECOND << 32) | RECEIVE_FRACTION;
    let transmit = (PEER_SECOND << 32) | TRANSMIT_FRACTION;
    let mut bytes = [0; 48];
    bytes[0] = 0x24; // LI=0, NTPv4, server mode; this admits no time scale.
    bytes[1] = 2;
    bytes[3] = (-4_i8).to_be_bytes()[0]; // Peer precision is exactly 1/16 s.
    bytes[4..8].copy_from_slice(&0x8000_0000_u32.to_be_bytes());
    bytes[8..12].copy_from_slice(&0x0000_8000_u32.to_be_bytes());
    bytes[24..32].copy_from_slice(&query[40..48]);
    bytes[32..40].copy_from_slice(&receive.to_be_bytes());
    bytes[40..48].copy_from_slice(&transmit.to_be_bytes());
    bytes
}

/// Serve exactly three actual requests, then observe absence of a fourth send.
/// # Errors
/// Preserve bounded native/notice errors and reject malformed fixture queries.
fn serve(peer: &UdpSocket, application_done: &Receiver<()>) -> Check<PeerObservations> {
    let endpoint = peer.local_addr()?;
    let mut clients = [None, None, None];
    let mut tokens = [[0; 8]; 3];
    for (index, client_record) in clients.iter_mut().enumerate() {
        let mut bytes = [0; 49];
        let (length, client) = peer.recv_from(&mut bytes)?;
        check(length == 48, "quality peer observed a non-48-byte query")?;
        check(bytes[0] == 0x23, "quality peer observed a non-client query")?;
        check(
            bytes[1..40].iter().all(|byte| *byte == 0),
            "quality query carried an undeclared payload",
        )?;
        check(
            bytes[40..48].iter().any(|byte| *byte != 0),
            "quality query correlation token was zero",
        )?;
        for earlier in &tokens[..index] {
            check(
                bytes[40..48] != *earlier,
                "quality series reused an earlier request token",
            )?;
        }
        tokens[index].copy_from_slice(&bytes[40..48]);
        let query: [u8; 48] = bytes[..48].try_into()?;
        let response = reply(&query);
        check(
            peer.send_to(&response, client)? == response.len(),
            "quality fixture native send was short",
        )?;
        *client_record = Some(client);
    }
    application_done.recv_timeout(PEER_WAIT)?;
    peer.set_read_timeout(Some(QUIET_WAIT))?;
    let mut unexpected = [0; 49];
    match peer.recv_from(&mut unexpected) {
        Err(error)
            if matches!(
                error.kind(),
                io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
            ) => {}
        Err(error) => return Err(error.into()),
        Ok(_) => return Err(io::Error::other("quality series sent a fourth query").into()),
    }
    Ok(PeerObservations { endpoint, clients })
}

/// Execute the real public series, joining the sole peer before propagating errors.
/// # Errors
/// Preserve setup, checked declaration, application, notice and worker failures.
fn run_series(with_profile: bool) -> Check<(SeriesReport, PeerObservations)> {
    let peer = UdpSocket::bind("127.0.0.1:0")?;
    peer.set_read_timeout(Some(PEER_WAIT))?;
    peer.set_write_timeout(Some(PEER_WAIT))?;
    let config = SeriesConfig::new(
        peer.local_addr()?,
        Duration::from_secs(1),
        Duration::from_secs(3),
    )?;
    let config = if with_profile {
        config.with_experimental_profile(ExperimentalProfile::new(
            3_000_000_000,
            4_500_000_000,
            30_000_000_000,
        )?)
    } else {
        config
    };
    let (prepared, _control) = prepare_series(&config)?;
    let schedule = ControlSchedule::new(None, None)?;
    let (application_done, application_returned) = mpsc::sync_channel(1);
    thread::scope(|scope| {
        let worker = scope.spawn(move || serve(&peer, &application_returned));
        let application = execute_series(prepared, &schedule);
        let notified = application_done.send(());
        let fixture = worker
            .join()
            .map_err(|_| io::Error::other("quality peer worker panicked"))?;
        let report = application?;
        let observed = fixture?;
        notified?;
        Ok((report, observed))
    })
}

/// Require each independently observed query's matching immutable terminal report.
/// # Errors
/// Reject missing/failed attempts, changed endpoints, admission or cleanup failures.
fn check_report(report: &SeriesReport, observed: &PeerObservations) -> Check<()> {
    check(
        report.end() == SeriesEnd::Completed,
        "quality series was incomplete",
    )?;
    check(
        report.peer_endpoint() == observed.endpoint,
        "quality series changed its configured peer",
    )?;
    check(
        report.admitted_sample_count() == 0 && report.eligible_sample_count() == 0,
        "conditional quality created sample admission or eligibility",
    )?;
    check(
        report.primary_failure().is_none()
            && report.stop_cleanup_failure().is_none()
            && report.invalidate_cleanup_failure().is_none(),
        "quality series lost a primary or cleanup outcome",
    )?;
    for (index, attempt) in report.attempts().iter().enumerate() {
        let record = slot(report, index)?;
        check(
            usize::from(attempt.index()) == index
                && usize::from(record.slot()) == index
                && attempt.not_started_cause().is_none(),
            "quality result lost its original attempt position",
        )?;
        check(
            record.status() == Status::Header && record.sent() && record.received() == 1,
            "quality fixture did not deliver one ordinary completion",
        )?;
        check(
            Some(record.local_endpoint()) == observed.clients[index],
            "quality report lost its actual queried endpoint",
        )?;
    }
    Ok(())
}

/// Borrow an actual completed position rather than manufacturing absent facts.
/// # Errors
/// Reject a missing planned position or absent terminal completion.
fn slot(report: &SeriesReport, index: usize) -> Check<&SlotReport> {
    report
        .attempts()
        .get(index)
        .and_then(|attempt| attempt.report())
        .ok_or_else(|| io::Error::other("missing quality terminal report").into())
}

/// Borrow only the original actual completion's privately produced facts.
/// # Errors
/// Reject a terminal report without retained ordinary-header observations.
fn facts(report: &SeriesReport, index: usize) -> Check<&TransportFacts> {
    slot(report, index)?
        .facts()
        .ok_or_else(|| io::Error::other("missing quality transport facts").into())
}

/// Construct explicit dyadic assumptions with no implicit physical interpretation.
/// # Errors
/// Forward any refusal of the supplied finite experimental declarations.
fn assumptions(advertised: f64, delay: f64, processing: f64) -> Check<DeclaredQualityAssumptions> {
    Ok(DeclaredQualityAssumptions::new(
        1.0 / 16.0,
        1.0 / 4.0,
        1.0 / 2.0,
        DeclaredFrequencyInterval::new(-1.0 / 8.0, 3.0 / 8.0)?,
        DeclaredQualityLimits::new(advertised, delay, processing)?,
    )?)
}

/// Convert an actual software wall read independently using the fixed epoch offset.
/// # Errors
/// Reject unsupported pre-Unix captures and unrepresentable exact nanoseconds.
fn ntp_nanoseconds(value: SystemTime) -> Check<i128> {
    let unix = i128::try_from(value.duration_since(SystemTime::UNIX_EPOCH)?.as_nanos())?;
    Ok(unix + 2_208_988_800_000_000_000)
}

/// Check the actual exact retained signed tuple, separately from its floating view.
/// # Errors
/// Reject changed sign, magnitude or fixed seconds denominator.
fn check_ratio(value: RoundedRationalSeconds, numerator: i128, denominator: u64) -> Check<()> {
    let sign = if numerator < 0 {
        RationalSign::Negative
    } else {
        RationalSign::Positive
    };
    check(
        value.sign() == sign
            && value.numerator_magnitude() == numerator.unsigned_abs()
            && value.denominator().get() == denominator,
        "conditional quality lost exact signed interval representation",
    )?;
    Ok(())
}

/// Convert the small signed captured interval independently of the product converter.
/// # Errors
/// Reject a captured duration outside the finite fixture's signed-i32-second scope.
fn seconds_from_nanoseconds(nanoseconds: i128) -> Check<f64> {
    let seconds = i32::try_from(nanoseconds.div_euclid(1_000_000_000))?;
    let fraction = i32::try_from(nanoseconds.rem_euclid(1_000_000_000))?;
    Ok(f64::from(seconds) + f64::from(fraction) / 1_000_000_000.0)
}

/// Compare independently reconstructed binary64 quantities with an arithmetic bound.
/// At most sixteen primitive roundings cover the split decimal conversion and
/// each tested short formula; this bound is not a physical uncertainty allowance.
/// # Errors
/// Reject nonfinite output or disagreement beyond that binary64 rounding allowance.
fn check_near(actual: f64, expected: f64) -> Check<()> {
    let tolerance = 16.0 * f64::EPSILON * expected.abs().max(1.0);
    check(
        actual.is_finite() && (actual - expected).abs() <= tolerance,
        "conditional quality disagreed with independent captured arithmetic",
    )?;
    Ok(())
}

/// Preserve the original bounded diagnostic and its explicit unauthenticated state.
/// # Errors
/// Preserve writer/encoding failure or a changed authentication/eligibility label.
fn diagnostic(record: &SlotReport) -> Check<Vec<u8>> {
    let mut output = Vec::new();
    record.write_measurement(&mut output)?;
    let text = std::str::from_utf8(&output)?;
    check(
        output.len() < 1_024 && text.contains("authentication=none eligible=0"),
        "conditional quality changed measurement authentication or eligibility",
    )?;
    Ok(output)
}

/// Verify actual-source association, exact signed intervals and literal quality terms.
/// # Errors
/// Preserve missing measurement, interval reconstruction or semantic formula failures.
fn check_quality(
    result: &ConditionalQuality<'_>,
    fact: &TransportFacts,
    declared: &DeclaredQualityAssumptions,
) -> Check<()> {
    check(
        std::ptr::eq(result.original_facts(), fact)
            && std::ptr::eq(result.declared_assumptions(), declared),
        "conditional quality lost its actual facts or assumption borrow",
    )?;
    let header = fact.header();
    check(
        header.precision() == -4
            && header.root_delay_bits() == 0x8000_0000
            && header.root_dispersion_bits() == 0x0000_8000,
        "conditional quality changed literal peer claims",
    )?;
    check(
        fact.peer_receive_bits() == ((PEER_SECOND << 32) | RECEIVE_FRACTION)
            && fact.peer_transmit_bits() == ((PEER_SECOND << 32) | TRANSMIT_FRACTION),
        "quality completion lost the literal peer timestamps",
    )?;
    let local_ns =
        ntp_nanoseconds(fact.read_return().utc())? - ntp_nanoseconds(fact.send_before().utc())?;
    let peer_ticks = i128::from(TRANSMIT_FRACTION - RECEIVE_FRACTION);
    let local_numerator = local_ns * Q32;
    let raw_numerator = local_numerator - peer_ticks * 1_000_000_000;
    check_ratio(
        result.local_interval(),
        local_numerator,
        NOMINAL_DENOMINATOR,
    )?;
    check_ratio(result.server_interval(), peer_ticks, 4_294_967_296)?;
    check_ratio(result.raw_round_trip(), raw_numerator, NOMINAL_DENOMINATOR)?;
    check(
        fact.measurement()?.nominal_round_trip().numerator() == raw_numerator,
        "original nominal RTT did not reconstruct from actual wall captures",
    )?;
    let local_seconds = seconds_from_nanoseconds(local_ns)?;
    let raw_seconds = local_seconds - 1.0 / 8.0;
    let delay = raw_seconds.abs().max(1.0 / 8.0);
    let dispersion = 1.0 / 2.0 + (1.0 / 4.0) * local_seconds.abs();
    check_near(result.local_interval().seconds(), local_seconds)?;
    check_near(result.server_interval().seconds(), 1.0 / 8.0)?;
    check_near(result.raw_round_trip().seconds(), raw_seconds)?;
    check_near(result.peer_precision_seconds(), 1.0 / 16.0)?;
    check_near(result.combined_precision_seconds(), 1.0 / 8.0)?;
    check_near(result.frequency_half_width(), 1.0 / 4.0)?;
    check_near(result.adjusted_delay_seconds(), delay)?;
    check_near(result.peer_dispersion_seconds(), dispersion)?;
    check_near(result.advertised_root_delay_seconds(), 32_768.0)?;
    check_near(result.advertised_root_dispersion_seconds(), 1.0 / 2.0)?;
    check_near(result.advertised_distance_seconds(), 16_384.5)?;
    check_near(result.root_delay_seconds(), 32_768.0 + delay)?;
    check_near(result.root_dispersion_seconds(), 1.0 / 2.0 + dispersion)?;
    check_near(result.peer_distance_seconds(), delay / 2.0 + dispersion)?;
    check_near(
        result.root_distance_seconds(),
        16_384.0 + delay / 2.0 + 1.0 / 2.0 + dispersion,
    )?;
    check_near(result.delay_minus_dispersion_seconds(), delay - dispersion)?;
    let original_age = fact
        .processing()
        .checked_duration_since(fact.read_return().monotonic_after())
        .ok_or_else(|| io::Error::other("quality capture age order changed"))?;
    check(
        result.original_facts().measurement()?.acquisition_age() == original_age,
        "conditional quality refreshed original acquisition age",
    )?;
    Ok(())
}

/// Three real completions retain actual producer association and exact capture truth.
/// # Errors
/// Preserve fixture, application, quality and independently reconstructed failures.
#[test]
fn actual_series_quality_retains_signed_intervals_source_and_original_age() -> Check<()> {
    let (report, observed) = run_series(true)?;
    check_report(&report, &observed)?;
    let declared = assumptions(32_768.0, 128.0, 1.0 / 4.0)?;
    for index in 0..3 {
        let record = slot(&report, index)?;
        let before = diagnostic(record)?;
        let fact = facts(&report, index)?;
        let result = evaluate_conditional_quality(fact, &declared)?;
        check_quality(&result, fact, &declared)?;
        for other_index in 0..3 {
            check(
                std::ptr::eq(result.original_facts(), facts(&report, other_index)?)
                    == (index == other_index),
                "quality result substituted another actual completion",
            )?;
        }
        check(
            before == diagnostic(record)?,
            "quality evaluation changed original diagnostic",
        )?;
    }
    check_report(&report, &observed)
}

/// Raw peer quality claims cannot replace absent date/model measurement admission.
/// A separately executed valid neighbor proves this path does not reject all facts.
/// # Errors
/// Preserve fixture failures or a wrong semantic missing-measurement classification.
#[test]
fn absent_measurement_refuses_quality_with_actual_valid_neighbor() -> Check<()> {
    let declared = assumptions(32_768.0, 128.0, 1.0 / 4.0)?;
    let (unavailable, observed) = run_series(false)?;
    check_report(&unavailable, &observed)?;
    for index in 0..3 {
        let record = slot(&unavailable, index)?;
        let before = diagnostic(record)?;
        check(
            matches!(
                evaluate_conditional_quality(facts(&unavailable, index)?, &declared),
                Err(QualityRefusal::Measurement(MeasurementRefusal::Unavailable))
            ),
            "quality did not preserve the missing measurement refusal",
        )?;
        check(
            before == diagnostic(record)?,
            "quality refusal changed original diagnostic",
        )?;
    }
    let (neighbor, observed) = run_series(true)?;
    check_report(&neighbor, &observed)?;
    let fact = facts(&neighbor, 0)?;
    let result = evaluate_conditional_quality(fact, &declared)?;
    check_quality(&result, fact, &declared)
}

/// Literal independent boundaries refuse quality while leaving completion consumed.
/// # Errors
/// Preserve fixture or constructor errors and reject wrong closed policy variants.
#[test]
fn actual_quality_limits_refuse_without_rewriting_completion_or_age() -> Check<()> {
    let (report, observed) = run_series(true)?;
    check_report(&report, &observed)?;
    let record = slot(&report, 0)?;
    let before = diagnostic(record)?;
    let fact = facts(&report, 0)?;
    let root_equal = assumptions(16_384.5, 128.0, 1.0 / 4.0)?;
    check(
        matches!(
            evaluate_conditional_quality(fact, &root_equal),
            Err(QualityRefusal::AdvertisedDistance)
        ),
        "advertised distance equality bypassed the strict quality limit",
    )?;
    let insufficient_precision = assumptions(32_768.0, 1.0 / 16.0, 1.0 / 4.0)?;
    check(
        matches!(
            evaluate_conditional_quality(fact, &insufficient_precision),
            Err(QualityRefusal::PrecisionDelay)
        ),
        "combined precision bypassed its independent delay limit",
    )?;
    let excessive_processing = assumptions(32_768.0, 128.0, 1.0 / 16.0)?;
    check(
        matches!(
            evaluate_conditional_quality(fact, &excessive_processing),
            Err(QualityRefusal::ServerProcessing)
        ),
        "literal peer processing interval bypassed its quality limit",
    )?;
    let processing_equal = assumptions(32_768.0, 128.0, 1.0 / 8.0)?;
    let accepted = evaluate_conditional_quality(fact, &processing_equal)?;
    check_quality(&accepted, fact, &processing_equal)?;
    check(
        before == diagnostic(record)?,
        "quality rejection changed original completion or age",
    )?;
    check_report(&report, &observed)
}

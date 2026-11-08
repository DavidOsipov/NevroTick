// SPDX-License-Identifier: GPL-2.0-only
//! Genuine sequential session acquisitions; no production source or clock authority.
//!
//! REQ-8/BND-1; CHECK-6/7/8/11/14/15/23/25/26/27. Native peer operations have
//! finite deadlines, every peer is joined, and only fixed facts diagnose failures.

use std::{
    error::Error,
    io,
    net::{SocketAddr, UdpSocket},
    sync::mpsc::{self, Receiver, TryRecvError},
    thread,
    time::{Duration, Instant},
};

use nevrotick_engine::{
    AcquisitionError, AcquisitionReference, AcquisitionReferenceRefusal, AcquisitionReport,
    AcquisitionSession, CoherenceUnavailable, ControlHandle, ControlSchedule, ExperimentalProfile,
    MeasurementRefusal, PreparedAcquisition, RelativeMeasurementRefusal, SeriesCoherence,
    SeriesConfig, SeriesEnd, SoftwareClock, Status, TransportFacts,
};
use ntp_wire::RawHeader;

/// Keep native/driver fixture errors typed; generic errors never earn semantic credit.
type Check<T> = Result<T, Box<dyn Error + Send + Sync>>;
/// Bound the independent peer and its actual-return acknowledgement.
const PEER_WAIT: Duration = Duration::from_secs(3);
/// Sample application completion without relying on a timing sleep.
const PEER_POLL: Duration = Duration::from_millis(100);
/// Independently fixed peer date, inside the declared experimental fixture window.
const PEER_SECOND: u64 = 4_000_000_000;

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

/// Convert an actual software wall read with the fixture's literal epoch offset.
/// # Errors
/// Keep unsupported host dates and exact conversion failures raw.
fn ntp_nanoseconds(value: std::time::SystemTime) -> Check<i128> {
    let unix = i128::try_from(value.duration_since(std::time::UNIX_EPOCH)?.as_nanos())?;
    Ok(unix + 2_208_988_800_000_000_000)
}

/// Check the fixed-first baseline over all actual retained triplets in this run.
/// # Errors
/// Keep observed cross-request model discontinuity raw before any status assertion.
fn fixture_series_coherence_prerequisites(report: &AcquisitionReport) -> Check<()> {
    let mut captures = Vec::with_capacity(9);
    for index in 0..3 {
        let original = fact(report, index)?;
        fixture_measurement_prerequisites(original, false)?;
        captures.extend([
            original.send_before(),
            original.send_after(),
            original.read_return(),
        ]);
    }
    fixture_capture_model(&captures, false)
}

/// Check the deliberate 0..1 date refusal without requiring successful date admission.
/// # Errors
/// Keep actual clock/model failures and a lost negative setup raw.
fn fixture_outside_date_prerequisites(facts: &TransportFacts) -> Check<()> {
    fixture_measurement_prerequisites(facts, false)?;
    let mut outside = false;
    for capture in [facts.send_before(), facts.send_after(), facts.read_return()] {
        outside |= !(0..=1_999_999_999).contains(&ntp_nanoseconds(capture.utc())?);
    }
    if !outside {
        return Err(io::Error::other(
            "actual negative fixture no longer lies outside its date declaration",
        )
        .into());
    }
    Ok(())
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

/// An independent product fact with fixed diagnostic text, never a native error.
/// # Errors
/// Return the exact static fact only when its separately specified condition fails.
fn check(condition: bool, fact: &'static str) -> io::Result<()> {
    if condition {
        Ok(())
    } else {
        Err(io::Error::other(fact))
    }
}

/// Construct one explicitly declared, checked laboratory profile and configuration.
/// # Errors
/// Preserve original checked-constructor failures, without manufacturing authority.
fn config(peer: SocketAddr) -> Check<SeriesConfig> {
    let profile = product_ok(
        ExperimentalProfile::new(3_000_000_000, 4_500_000_000, 30_000_000_000),
        "valid session experimental profile was refused",
    )?;
    Ok(product_ok(
        SeriesConfig::new(peer, Duration::from_secs(1), Duration::from_secs(2)),
        "valid session series configuration was refused",
    )?
    .with_experimental_profile(profile))
}

/// Prepare an independently known Idle owner, distinguishing structural refusals.
/// # Errors
/// Preserve genuine Driver failures; assert only unexpected Busy/Retired/Invariant.
fn idle_plan(owner: &mut AcquisitionSession) -> Check<(PreparedAcquisition<'_>, ControlHandle)> {
    let result = owner.prepare();
    check(
        !matches!(
            &result,
            Err(AcquisitionError::Busy | AcquisitionError::Retired | AcquisitionError::Invariant)
        ),
        "known idle session refused an explicitly requested plan",
    )?;
    Ok(result?)
}

/// Extract an actual returned wrapper without converting native driver errors.
/// # Errors
/// Preserve Driver failures; diagnose only an impossible structural valid-plan refusal.
fn actual_report(result: Result<AcquisitionReport, AcquisitionError>) -> Check<AcquisitionReport> {
    check(
        !matches!(
            &result,
            Err(AcquisitionError::Busy | AcquisitionError::Retired | AcquisitionError::Invariant)
        ),
        "genuine prepared session refused consuming execution",
    )?;
    let report = result?;
    let series = report.series();
    if series.primary_failure().is_some()
        || series.stop_cleanup_failure().is_some()
        || series.invalidate_cleanup_failure().is_some()
        || series.attempts().iter().any(|attempt| {
            attempt
                .report()
                .is_some_and(|record| record.status() == Status::Io || record.io_cause().is_some())
        })
    {
        return Err(io::Error::other(
            "actual session fixture retained a driver or native I/O cause",
        )
        .into());
    }
    Ok(report)
}

/// Real query fields; no packet or opaque nonce has a Debug formatter.
struct Query {
    /// The exact opaque transmit token observed by the independent peer.
    token: [u8; 8],
    /// Actual client endpoint from the same native receive.
    client: SocketAddr,
}

/// At most three observed queries plus an independently observed excess-send bit.
struct Observed {
    /// Real configured destination, retained without formatting it on failure.
    peer: SocketAddr,
    /// Actual queries, in original arrival order; never a fabricated completion.
    queries: Vec<Query>,
    /// A fourth native query was observed after the product returned.
    extra: bool,
}

/// Read one genuine query or confirm that the application already returned early.
/// # Errors
/// Native/channel/deadline failures remain fixture errors rather than semantic facts.
fn next_query(
    peer: &UdpSocket,
    returned: &Receiver<()>,
    deadline: Instant,
) -> io::Result<Option<Query>> {
    loop {
        let mut packet = [0; 49];
        match peer.recv_from(&mut packet) {
            Ok((length, client)) => {
                check(length == 48, "session issued a non-header native query")?;
                check(
                    packet[0] == 0x23 && packet[1..40].iter().all(|byte| *byte == 0),
                    "session changed the declared native query layout",
                )?;
                let mut token = [0; 8];
                token.copy_from_slice(&packet[40..48]);
                check(
                    token.iter().any(|byte| *byte != 0),
                    "session query omitted its actual token",
                )?;
                return Ok(Some(Query { token, client }));
            }
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
                ) =>
            {
                match returned.try_recv() {
                    Ok(()) => return Ok(None),
                    Err(TryRecvError::Disconnected) => {
                        return Err(io::Error::other("session return observer disconnected"));
                    }
                    Err(TryRecvError::Empty) if Instant::now() < deadline => {}
                    Err(TryRecvError::Empty) => return Err(error),
                }
            }
            Err(error) => return Err(error),
        }
    }
}

/// Reply once per actual query, optionally delivering only an old handle's control.
/// # Errors
/// Bound all native work and preserve native/channel/control errors without credit.
fn serve(
    peer: &UdpSocket,
    returned: &Receiver<()>,
    old: Option<&ControlHandle>,
) -> io::Result<Observed> {
    peer.set_read_timeout(Some(PEER_POLL))?;
    peer.set_write_timeout(Some(PEER_WAIT))?;
    let deadline = Instant::now()
        .checked_add(PEER_WAIT)
        .ok_or_else(|| io::Error::other("session fixture deadline overflow"))?;
    let mut observed = Observed {
        peer: peer.local_addr()?,
        queries: Vec::with_capacity(3),
        extra: false,
    };
    for index in 0..3 {
        let Some(query) = next_query(peer, returned, deadline)? else {
            return Ok(observed);
        };
        check(
            observed
                .queries
                .iter()
                .all(|prior| prior.token != query.token),
            "one session series reused an actual request token",
        )?;
        if index == 0
            && let Some(handle) = old
        {
            handle.stop();
            handle.invalidate().map_err(io::Error::other)?;
        }
        let second = PEER_SECOND + u64::try_from(index).map_err(io::Error::other)?;
        let mut reply = [0; 48];
        reply[0] = 0x24;
        reply[1] = 2;
        reply[24..32].copy_from_slice(&query.token);
        reply[32..40].copy_from_slice(&((second << 32) | 0x4000_0000).to_be_bytes());
        reply[40..48].copy_from_slice(&((second << 32) | 0x6000_0000).to_be_bytes());
        if peer.send_to(&reply, query.client)? != 48 {
            return Err(io::Error::other(
                "session native fixture response was short",
            ));
        }
        observed.queries.push(query);
    }
    returned.recv_timeout(PEER_WAIT).map_err(io::Error::other)?;
    let mut extra = [0; 49];
    match peer.recv_from(&mut extra) {
        Ok(_) => observed.extra = true,
        Err(error)
            if matches!(
                error.kind(),
                io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
            ) => {}
        Err(error) => return Err(error),
    }
    Ok(observed)
}

/// Inspect actual reports against peer facts after all fixture resources are joined.
/// # Errors
/// Preserve original measurement refusal as a fixture prerequisite; reject changed facts.
#[expect(
    clippy::too_many_lines,
    reason = "Check one actual report's joined peer facts and original reference refusals together"
)]
fn inspect(
    report: &AcquisitionReport,
    observed: &Observed,
    expected_measurement: Result<(), MeasurementRefusal>,
) -> Check<()> {
    let series = report.series();
    if series.primary_failure().is_some()
        || series.stop_cleanup_failure().is_some()
        || series.invalidate_cleanup_failure().is_some()
    {
        return Err(io::Error::other(
            "session fixture driver retained a primary or cleanup failure",
        )
        .into());
    }
    check(
        observed.queries.len() == 3 && !observed.extra,
        "session did not perform exactly three requested queries",
    )?;
    check(
        series.end() == SeriesEnd::Completed,
        "healthy session series did not retain its actual normal end",
    )?;
    check(
        series.admitted_sample_count() == 0 && series.eligible_sample_count() == 0,
        "session provenance created sample authority",
    )?;
    let expected_coherence = if expected_measurement == Err(MeasurementRefusal::Unavailable) {
        SeriesCoherence::Unavailable(CoherenceUnavailable::NoProfile)
    } else {
        SeriesCoherence::ObservedCompatible
    };
    for (index, (attempt, query)) in series.attempts().iter().zip(&observed.queries).enumerate() {
        check(
            attempt.report().is_some() && attempt.not_started_cause().is_none(),
            "session lost a genuinely completed attempt",
        )?;
        let record = attempt
            .report()
            .ok_or_else(|| io::Error::other("missing checked session attempt"))?;
        check(
            record.status() == Status::Header
                && record.sent()
                && record.received() == 1
                && record.rejected() == 0,
            "session rewrote original terminal counters",
        )?;
        check(
            record.facts().is_some(),
            "session lost acquired transport facts",
        )?;
        let facts = record
            .facts()
            .ok_or_else(|| io::Error::other("missing checked session facts"))?;
        check(
            facts.peer_endpoint() == observed.peer
                && facts.local_endpoint() == query.client
                && facts.header().origin_timestamp().bits().to_be_bytes() == query.token,
            "session replaced actual peer request association",
        )?;
        let header = facts.header();
        let receive = u64::from(header.receive_timestamp().seconds());
        let transmit = u64::from(header.transmit_timestamp().seconds());
        check(
            header.version() == 4
                && header.mode() == 4
                && header.leap_indicator() == 0
                && header.stratum() == 2
                && header.receive_timestamp().bits() != 0
                && header.transmit_timestamp().bits() != 0
                && (3_000_000_000..=4_500_000_000).contains(&receive)
                && (3_000_000_000..=4_500_000_000).contains(&transmit),
            "session retained header changed fixed ordinary peer facts",
        )?;
        check(
            facts.send_before().monotonic() >= report.chart_origin(),
            "session chart was captured after acquisition",
        )?;
        for prior in &series.attempts()[..index] {
            let earlier = prior
                .report()
                .and_then(|record| record.facts())
                .ok_or_else(|| io::Error::other("missing checked earlier session facts"))?;
            check(
                !facts.same_exchange(earlier),
                "session merged two original attempt identities",
            )?;
        }
        match expected_measurement {
            Ok(()) => {
                inspect_reference(report, attempt.index())?;
            }
            Err(expected) => {
                if expected == MeasurementRefusal::DateScope {
                    fixture_outside_date_prerequisites(facts)?;
                }
                check(
                    matches!(facts.measurement(), Err(actual) if actual == expected),
                    "session changed the independently expected measurement refusal",
                )?;
            }
        }
    }
    if expected_coherence == SeriesCoherence::ObservedCompatible {
        fixture_series_coherence_prerequisites(report)?;
    }
    check(
        report.coherence() == expected_coherence,
        "session coherence replaced the declared within-run capture result",
    )?;
    Ok(())
}

/// Run one actual plan, preserving native errors and joining the finite peer first.
/// # Errors
/// Every spawn/channel/peer/driver failure remains visible; no detached worker survives.
fn run_three(
    owner: &mut AcquisitionSession,
    peer: &UdpSocket,
    old: Option<&ControlHandle>,
    expected_measurement: Result<(), MeasurementRefusal>,
) -> Check<(AcquisitionReport, ControlHandle)> {
    let (plan, control) = idle_plan(owner)?;
    let socket = peer.try_clone()?;
    let schedule = ControlSchedule::new(None, None)?;
    thread::scope(|scope| {
        let (returned, observed_return) = mpsc::sync_channel(1);
        let worker = thread::Builder::new()
            .spawn_scoped(scope, move || serve(&socket, &observed_return, old))?;
        let result = plan.execute(&schedule);
        let notification = returned.send(());
        let observed = worker
            .join()
            .map_err(|_| io::Error::other("session peer worker panicked"))??;
        notification.map_err(io::Error::other)?;
        let report = actual_report(result)?;
        inspect(&report, &observed, expected_measurement)?;
        Ok((report, control))
    })
}

/// Extract a reference only for independently checked genuine measured facts.
/// # Errors
/// Diagnose a product refusal before extracting the checked borrowed result.
fn checked_reference(report: &AcquisitionReport, position: u8) -> Check<AcquisitionReference<'_>> {
    let result = report.reference_at(position);
    check(
        result.is_ok(),
        "genuine measured attempt refused its own session reference",
    )?;
    Ok(result?)
}

/// Compare original associations and exact captured endpoints without shared math.
/// The wide endpoint sum is independent of subtraction-before-halving publication.
/// # Errors
/// Preserve original measurement/setup errors; diagnose only changed product facts.
fn inspect_reference(report: &AcquisitionReport, position: u8) -> Check<()> {
    let attempt = report
        .series()
        .attempts()
        .get(usize::from(position))
        .ok_or_else(|| io::Error::other("session reference fixture position outside domain"))?;
    let facts = fact(report, usize::from(position))?;
    fixture_measurement_prerequisites(facts, true)?;
    let measurement = product_ok(
        facts.measurement(),
        "valid original session measurement was refused",
    )?;
    let view = checked_reference(report, position)?;
    check(
        format!("{view:?}") == "AcquisitionReference { .. }",
        "session reference Debug exposed original captures or provenance",
    )?;
    check(
        std::ptr::eq(view.report(), report)
            && std::ptr::eq(view.attempt(), attempt)
            && std::ptr::eq(view.facts(), facts)
            && std::ptr::eq(view.measurement(), measurement)
            && view.attempt().index() == position,
        "session reference substituted an original borrowed association",
    )?;
    let origin = report.chart_origin();
    let lower = facts
        .send_before()
        .monotonic()
        .checked_duration_since(origin)
        .ok_or_else(|| io::Error::other("fixture send precedes its genuine session chart"))?
        .as_nanos();
    let upper = facts
        .read_return()
        .monotonic()
        .checked_duration_since(origin)
        .ok_or_else(|| io::Error::other("fixture read precedes its genuine session chart"))?
        .as_nanos();
    let midpoint = view.midpoint();
    check(
        u128::from(midpoint.whole_nanoseconds()) * 2 + u128::from(midpoint.has_half_nanosecond())
            == lower + upper,
        "session midpoint disagrees with original lower capture endpoint sum",
    )?;
    check(
        view.measurement().acquisition_age()
            == facts
                .processing()
                .duration_since(facts.read_return().monotonic_after()),
        "session reference renewed original receive-completion age",
    )?;
    let repeated = checked_reference(report, position)?.midpoint();
    check(
        repeated.whole_nanoseconds() == midpoint.whole_nanoseconds()
            && repeated.has_half_nanosecond() == midpoint.has_half_nanosecond(),
        "repeated historical reference derivation changed exact parts",
    )?;
    Ok(())
}

/// Immutable original coordinates/age; no snapshot prints opaque wire payloads.
struct Snapshot {
    /// Original complete header, including correlation and peer bits.
    header: RawHeader,
    /// Original pre-send monotonic observation.
    send_before: Instant,
    /// Original post-send monotonic observation.
    send_after: Instant,
    /// Original post-read monotonic observation.
    read_after: Instant,
    /// Actual processing coordinate retained by the original completion.
    processing: Instant,
    /// Immutable original acquisition age.
    age: Duration,
    /// Exact original chart midpoint integral part.
    midpoint_whole: u64,
    /// Exact original midpoint half-nanosecond part.
    midpoint_half: bool,
    /// Original report-owned conditional capture assessment, without current authority.
    coherence: SeriesCoherence,
}

/// Borrow one actual original attempt; index is checked by the owning report slice.
/// # Errors
/// Missing actual facts are independent product facts, not caller-created stand-ins.
fn fact(report: &AcquisitionReport, index: usize) -> Check<&TransportFacts> {
    let attempt = report
        .series()
        .attempts()
        .get(index)
        .ok_or_else(|| io::Error::other("session fixture index outside domain"))?;
    check(
        attempt.report().is_some(),
        "session historical attempt disappeared",
    )?;
    let record = attempt
        .report()
        .ok_or_else(|| io::Error::other("missing checked historical attempt"))?;
    check(
        record.facts().is_some(),
        "session historical facts disappeared",
    )?;
    Ok(record
        .facts()
        .ok_or_else(|| io::Error::other("missing checked historical facts"))?)
}

/// Snapshot exactly the actual three original completions, without refreshing them.
/// # Errors
/// Preserve original measurement refusal and independent missing-fact failures.
fn snapshots(report: &AcquisitionReport) -> Check<Vec<Snapshot>> {
    (0..3)
        .map(|index| {
            let original = fact(report, index)?;
            let midpoint =
                checked_reference(report, u8::try_from(index).map_err(io::Error::other)?)?
                    .midpoint();
            Ok(Snapshot {
                header: original.header(),
                send_before: original.send_before().monotonic(),
                send_after: original.send_after().monotonic(),
                read_after: original.read_return().monotonic_after(),
                processing: original.processing(),
                age: product_ok(
                    original.measurement(),
                    "valid session snapshot measurement was refused",
                )?
                .acquisition_age(),
                midpoint_whole: midpoint.whole_nanoseconds(),
                midpoint_half: midpoint.has_half_nanosecond(),
                coherence: report.coherence(),
            })
        })
        .collect()
}

/// Check historical preservation after later execution, invalidation and retirement.
/// # Errors
/// Preserve original measurement refusal and report only fixed independent fact text.
fn unchanged(report: &AcquisitionReport, saved: &[Snapshot]) -> Check<()> {
    check(
        saved.len() == 3,
        "session fixture lost its original snapshot domain",
    )?;
    for (index, snapshot) in saved.iter().enumerate() {
        let original = fact(report, index)?;
        let position = u8::try_from(index).map_err(io::Error::other)?;
        inspect_reference(report, position)?;
        let midpoint = checked_reference(report, position)?.midpoint();
        check(
            original.header() == snapshot.header
                && original.send_before().monotonic() == snapshot.send_before
                && original.send_after().monotonic() == snapshot.send_after
                && original.read_return().monotonic_after() == snapshot.read_after
                && original.processing() == snapshot.processing
                && product_ok(
                    original.measurement(),
                    "valid historical session measurement was refused",
                )?
                .acquisition_age()
                    == snapshot.age
                && midpoint.whole_nanoseconds() == snapshot.midpoint_whole
                && midpoint.has_half_nanosecond() == snapshot.midpoint_half
                && report.coherence() == snapshot.coherence,
            "session replacement or late control refreshed original facts or age",
        )?;
    }
    Ok(())
}

/// Separate explicit executions share provenance while retaining actual run identity.
/// # Errors
/// Preserve real fixture/driver errors after bounded peer join.
#[test]
fn two_requested_series_share_chart_keep_distinct_runs_and_old_handles_isolated() -> Check<()> {
    let peer = UdpSocket::bind(SocketAddr::from(([127, 0, 0, 1], 0)))?;
    let before = Instant::now();
    let mut owner = AcquisitionSession::new(config(peer.local_addr()?)?);
    let after = Instant::now();
    let (first, old) = run_three(&mut owner, &peer, None, Ok(()))?;
    let saved = snapshots(&first)?;
    let retained_reference = checked_reference(&first, 0)?;
    old.stop();
    old.invalidate()?;
    let (second, second_control) = run_three(&mut owner, &peer, Some(&old), Ok(()))?;
    check(
        first.same_session(&second) && second.same_session(&first),
        "separate session series lost their genuine chart identity",
    )?;
    check(
        first.chart_origin() == second.chart_origin()
            && first.chart_origin() >= before
            && first.chart_origin() <= after,
        "session origin was recreated after preparation",
    )?;
    check(
        first.series().origin() >= first.chart_origin()
            && second.series().origin() >= first.series().origin(),
        "session changed an original execution origin",
    )?;
    for row in 0..3 {
        let left = fact(&first, row)?;
        check(
            left.relative_to(left).is_ok(),
            "same-run healthy relative view was lost",
        )?;
        for anchor in 0..3 {
            let right = fact(&second, anchor)?;
            check(
                !left.same_exchange(right),
                "session merged distinct actual request lifetimes",
            )?;
            check(
                matches!(
                    left.relative_to(right),
                    Err(RelativeMeasurementRefusal::DifferentRun)
                ),
                "shared session provenance weakened different-run refusal",
            )?;
        }
    }
    owner.retire();
    owner.retire();
    check(
        matches!(owner.prepare(), Err(AcquisitionError::Retired)),
        "retired session reopened preparation",
    )?;
    drop(owner);
    check(
        std::ptr::eq(retained_reference.report(), &raw const first),
        "retiring and dropping the issuer invalidated a historical reference",
    )?;
    old.invalidate()?;
    second_control.stop();
    second_control.invalidate()?;
    let mut replacement = AcquisitionSession::new(config(peer.local_addr()?)?);
    let (third, _control) = run_three(&mut replacement, &peer, Some(&second_control), Ok(()))?;
    check(
        !first.same_session(&third) && !third.same_session(&second),
        "equal-endpoint replacement inherited foreign session identity",
    )?;
    check(
        format!("{first:?}") == "AcquisitionReport { .. }",
        "session report diagnostics exposed retained provenance",
    )?;
    unchanged(&first, &saved)?;
    Ok(())
}

/// A genuine stale report retires its issuer without altering historical completions.
/// # Errors
/// Preserve native/driver/control failures and require a healthy replacement neighbor.
#[test]
fn stale_execution_retires_owner_without_refreshing_retained_facts() -> Check<()> {
    let peer = UdpSocket::bind(SocketAddr::from(([127, 0, 0, 1], 0)))?;
    let mut owner = AcquisitionSession::new(config(peer.local_addr()?)?);
    let (first, _first_control) = run_three(&mut owner, &peer, None, Ok(()))?;
    let saved = snapshots(&first)?;
    let (plan, stale_control) = idle_plan(&mut owner)?;
    stale_control.invalidate()?;
    let stale = actual_report(plan.execute(&ControlSchedule::new(None, None)?))?;
    check(
        stale.series().end() == SeriesEnd::Stale && stale.same_session(&first),
        "stale execution lost its actual retirement or chart",
    )?;
    check(
        stale.coherence() == SeriesCoherence::Unavailable(CoherenceUnavailable::NoFacts),
        "stale unstarted execution fabricated capture compatibility",
    )?;
    check(
        stale.series().primary_failure().is_none()
            && stale.series().stop_cleanup_failure().is_none()
            && stale.series().invalidate_cleanup_failure().is_none(),
        "stale control fabricated an unrelated driver failure",
    )?;
    check(
        matches!(owner.prepare(), Err(AcquisitionError::Retired)),
        "stale session was reused after retirement",
    )?;
    let mut replacement = AcquisitionSession::new(config(peer.local_addr()?)?);
    let (fresh, _control) = run_three(&mut replacement, &peer, Some(&stale_control), Ok(()))?;
    check(
        !fresh.same_session(&first) && !fresh.same_session(&stale),
        "fresh neighbor inherited retired session provenance",
    )?;
    unchanged(&first, &saved)?;
    Ok(())
}

/// Actual normal stop/deadline returns permit a subsequent explicit preparation.
/// The silent peer stays bound; no closed-port ICMP or timing sleep is used.
/// # Errors
/// Preserve real preparation/runtime/driver errors; no query-count assumption is made.
#[test]
fn clean_stop_and_real_deadline_each_allow_a_fresh_preparation() -> Check<()> {
    let peer = UdpSocket::bind(SocketAddr::from(([127, 0, 0, 1], 0)))?;
    let config = product_ok(
        SeriesConfig::new(
            peer.local_addr()?,
            Duration::from_millis(50),
            Duration::from_millis(50),
        ),
        "valid session series configuration was refused",
    )?;
    let mut owner = AcquisitionSession::new(config);
    let (plan, control) = idle_plan(&mut owner)?;
    control.stop();
    let stopped = actual_report(plan.execute(&ControlSchedule::new(None, None)?))?;
    check(
        stopped.series().end() == SeriesEnd::Stopped,
        "pre-execute stop lost its normal terminal result",
    )?;
    check(
        stopped.series().primary_failure().is_none()
            && stopped.series().stop_cleanup_failure().is_none()
            && stopped.series().invalidate_cleanup_failure().is_none(),
        "clean stop acquired a primary or cleanup cause",
    )?;
    let (unused, _control) = idle_plan(&mut owner)?;
    drop(unused);
    let (plan, _control) = idle_plan(&mut owner)?;
    let expired = actual_report(plan.execute(&ControlSchedule::new(None, None)?))?;
    check(
        expired.series().end() == SeriesEnd::SeriesDeadline,
        "original whole-series deadline lost its normal result",
    )?;
    check(
        expired.series().primary_failure().is_none()
            && expired.series().stop_cleanup_failure().is_none()
            && expired.series().invalidate_cleanup_failure().is_none(),
        "clean deadline acquired a primary or cleanup cause",
    )?;
    check(
        stopped.same_session(&expired),
        "normal stop or deadline replaced the retained session chart",
    )?;
    check(
        stopped.coherence() == SeriesCoherence::Unavailable(CoherenceUnavailable::NoProfile)
            && expired.coherence() == SeriesCoherence::Unavailable(CoherenceUnavailable::NoProfile),
        "absent profile lost precedence over empty or timed-out capture observations",
    )?;
    let (unused, _control) = idle_plan(&mut owner)?;
    drop(unused);
    Ok(())
}

/// Original measurement refusals precede midpoint work, with no transport-only view.
/// One fixed ordinary response retains facts for unavailable and out-of-date profiles.
/// # Errors
/// Preserve real socket/configuration/driver/peer failures outside semantic credit.
#[test]
fn session_reference_refusals_preserve_original_causes_and_checked_positions() -> Check<()> {
    let peer = UdpSocket::bind(SocketAddr::from(([127, 0, 0, 1], 0)))?;
    let address = peer.local_addr()?;
    let outside_profile = product_ok(
        ExperimentalProfile::new(0, 1, 30_000_000_000),
        "valid session experimental profile was refused",
    )?;
    let outside_configuration = product_ok(
        SeriesConfig::new(address, Duration::from_secs(1), Duration::from_secs(2)),
        "valid session series configuration was refused",
    )?
    .with_experimental_profile(outside_profile);
    for (configuration, expected) in [
        (
            product_ok(
                SeriesConfig::new(address, Duration::from_secs(1), Duration::from_secs(2)),
                "valid session series configuration was refused",
            )?,
            MeasurementRefusal::Unavailable,
        ),
        (outside_configuration, MeasurementRefusal::DateScope),
    ] {
        let mut owner = AcquisitionSession::new(configuration);
        let (report, _control) = run_three(&mut owner, &peer, None, Err(expected))?;
        for position in 0..3 {
            check(
                matches!(
                    report.reference_at(position),
                    Err(AcquisitionReferenceRefusal::Measurement(actual)) if actual == expected
                ),
                "session reference replaced an original conditional measurement refusal",
            )?;
        }
        let (plan, control) = idle_plan(&mut owner)?;
        control.stop();
        let stopped = actual_report(plan.execute(&ControlSchedule::new(None, None)?))?;
        if stopped.series().primary_failure().is_some()
            || stopped.series().stop_cleanup_failure().is_some()
            || stopped.series().invalidate_cleanup_failure().is_some()
        {
            return Err(
                io::Error::other("pre-stopped reference fixture retained a driver cause").into(),
            );
        }
        check(
            stopped.series().end() == SeriesEnd::Stopped,
            "reference refusal fixture lost its original stopped end",
        )?;
        let expected_empty_coherence = if expected == MeasurementRefusal::Unavailable {
            SeriesCoherence::Unavailable(CoherenceUnavailable::NoProfile)
        } else {
            SeriesCoherence::Unavailable(CoherenceUnavailable::NoFacts)
        };
        check(
            stopped.coherence() == expected_empty_coherence,
            "pre-stopped coherence lost original profile or absent-facts precedence",
        )?;
        for position in 0..3 {
            check(
                matches!(
                    stopped.reference_at(position),
                    Err(AcquisitionReferenceRefusal::FactsUnavailable)
                ),
                "unstarted position fabricated a session reference or measurement refusal",
            )?;
        }
        for original in [&report, &stopped] {
            for position in [3, u8::MAX] {
                check(
                    matches!(
                        original.reference_at(position),
                        Err(AcquisitionReferenceRefusal::AttemptPosition)
                    ),
                    "session reference did not reject an absent position first",
                )?;
            }
        }
        let (unused, _control) = idle_plan(&mut owner)?;
        drop(unused);
    }
    Ok(())
}

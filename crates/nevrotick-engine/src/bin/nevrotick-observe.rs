//! Controlled loopback transport experiment; no service, estimator or clock effects.
//!
//! Exactly two explicit numeric loopback peers and a query lifetime are required. Initialization,
//! including OS entropy acquisition, precedes the measured active-control phase.
//! Output is a finite diagnostic projection, never an authenticated or eligible
//! synchronization sample. Native Windows teardown needs separate observations.

#![forbid(unsafe_code)]

use std::io::{self, Write};
use std::net::SocketAddr;
use std::process::ExitCode;
use std::time::Duration;

use nevrotick_engine::{Config, ControlSchedule, PilotError, PilotReport, execute, prepare};

/// Finite command syntax or runtime failures; no rejected input is retained.
#[derive(Debug)]
enum CliError {
    /// Missing, repeated, oversized or unsupported command argument.
    Arguments,
    /// A numeric endpoint failed parsing; its original bytes are discarded.
    Endpoint,
    /// The output sink rejected a bounded diagnostic record.
    Output,
    /// A checked engine operation failed with a safe typed code.
    Pilot(PilotError),
}

impl CliError {
    /// Project a fixed code without endpoint text, OS messages or raw packets.
    fn as_str(&self) -> &'static str {
        match self {
            Self::Arguments => "invalid_arguments",
            Self::Endpoint => "invalid_endpoint",
            Self::Output => "diagnostic_output_failed",
            Self::Pilot(error) => error.as_str(),
        }
    }
}

/// Checked syntax before the engine's mandatory endpoint/budget construction.
struct Arguments {
    /// Two caller-selected numeric addresses; Config owns semantic admission.
    peers: [SocketAddr; 2],
    /// Requested query lifetime; Config enforces its experiment bounds.
    timeout: Duration,
    /// Optional bounded stop injection after active-control startup.
    stop_after: Option<Duration>,
    /// Optional bounded generation invalidation after active-control startup.
    invalidate_after: Option<Duration>,
}

/// Parse at most twelve bounded arguments without retaining raw caller strings.
///
/// No network, file or clock mutation occurs. Endpoint meaning and timeout bounds
/// are admitted later through Config, which direct library callers also require.
///
/// # Errors
/// Reject missing/repeated options, oversized or non-text arguments, invalid
/// numeric endpoints and disallowed decimal duration syntax.
fn arguments() -> Result<Arguments, CliError> {
    let mut raw = std::env::args_os().skip(1);
    let mut peers = Vec::with_capacity(2);
    let mut timeout = None;
    let mut stop_after = None;
    let mut invalidate_after = None;
    let mut consumed = 0_u8;
    while let Some(option) = raw.next() {
        consumed = consumed.checked_add(2).ok_or(CliError::Arguments)?;
        if consumed > 12 || option.len() > 32 {
            return Err(CliError::Arguments);
        }
        let value = raw.next().ok_or(CliError::Arguments)?;
        if value.len() > 128 {
            return Err(CliError::Arguments);
        }
        let value = value.to_str().ok_or(CliError::Arguments)?;
        match option.to_str().ok_or(CliError::Arguments)? {
            "--peer" if peers.len() < 2 => {
                peers.push(value.parse().map_err(|_| CliError::Endpoint)?);
            }
            "--timeout-ms" if timeout.is_none() => {
                timeout = Some(milliseconds(value)?);
            }
            "--stop-after-ms" if stop_after.is_none() => {
                stop_after = Some(milliseconds(value)?);
            }
            "--invalidate-after-ms" if invalidate_after.is_none() => {
                invalidate_after = Some(milliseconds(value)?);
            }
            _ => return Err(CliError::Arguments),
        }
    }
    Ok(Arguments {
        peers: peers.try_into().map_err(|_| CliError::Arguments)?,
        timeout: timeout.ok_or(CliError::Arguments)?,
        stop_after,
        invalidate_after,
    })
}

/// Convert a bounded ASCII decimal duration, rejecting signs and other syntax.
///
/// # Errors
/// Reject empty, oversized, non-decimal or unrepresentable millisecond values.
fn milliseconds(value: &str) -> Result<Duration, CliError> {
    if value.is_empty() || value.len() > 8 || !value.bytes().all(|b| b.is_ascii_digit()) {
        return Err(CliError::Arguments);
    }
    let milliseconds = value.parse::<u64>().map_err(|_| CliError::Arguments)?;
    Ok(Duration::from_millis(milliseconds))
}

/// Write exactly two slot records and one completion record using safe fields.
///
/// # Errors
/// Return a diagnostic-output failure if the sink rejects any record; earlier
/// partial output is not relabelled as a complete report.
fn write_report(report: &PilotReport, output: &mut impl Write) -> Result<(), CliError> {
    for slot in report.slots() {
        write!(
            output,
            "event=result slot={} status={} authentication=none eligible=0 sent={} received={} rejected={}",
            slot.slot(),
            slot.status().as_str(),
            u8::from(slot.sent()),
            slot.received(),
            slot.rejected(),
        )
        .map_err(|_| CliError::Output)?;
        if let Some(stratum) = slot.stratum() {
            write!(output, " stratum={stratum}").map_err(|_| CliError::Output)?;
        }
        writeln!(output).map_err(|_| CliError::Output)?;
    }
    writeln!(output, "event=done elapsed_ns={}", report.done_elapsed_ns())
        .map_err(|_| CliError::Output)
}

/// Execute one explicit lab operation through the checked runtime owner.
///
/// # Errors
/// Return syntax, checked-plan, startup, executor or diagnostic sink failures.
/// Per-slot transport outcomes remain explicit bounded report values.
fn run() -> Result<(), CliError> {
    let args = arguments()?;
    let config = Config::new(args.peers, args.timeout).map_err(CliError::Pilot)?;
    let schedule =
        ControlSchedule::new(args.stop_after, args.invalidate_after).map_err(CliError::Pilot)?;
    // OS entropy and synchronous socket setup precede active-control timing.
    // Their source review does not establish bounded startup latency.
    let (pilot, control) = prepare(&config).map_err(CliError::Pilot)?;
    {
        let mut output = io::stdout().lock();
        for (slot, local) in pilot.local_endpoints().iter().enumerate() {
            writeln!(output, "event=bound slot={slot} local={local}")
                .map_err(|_| CliError::Output)?;
        }
        output.flush().map_err(|_| CliError::Output)?;
    }
    let report = execute(pilot, schedule).map_err(CliError::Pilot)?;
    // The external request handle outlives execution and its timer cleanup.
    drop(control);
    write_report(&report, &mut io::stdout().lock())
}

/// Return through Rust's normal process teardown; never change host clock state.
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => writeln!(io::stderr().lock(), "error={}", error.as_str())
            .map_or(ExitCode::FAILURE, |()| ExitCode::from(2)),
    }
}

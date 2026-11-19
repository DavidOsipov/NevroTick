//! Controlled loopback transport experiment; no service, estimator or clock effects.
//!
//! Exactly two explicit numeric loopback peers and a query lifetime are required. Initialization,
//! including OS entropy acquisition, precedes the measured active-control phase.
//! Output is a finite diagnostic projection, never an authenticated or eligible
//! synchronization sample. Native Windows teardown needs separate observations.

#![forbid(unsafe_code)]

use std::ffi::{OsStr, OsString};
use std::io::{self, Write};
use std::net::SocketAddr;
use std::process::ExitCode;
use std::time::Duration;

use nevrotick_engine::{
    Config, ControlHandle, ControlSchedule, ExperimentalProfile, PilotError, PilotReport,
    PreparedPilot, execute, prepare,
};

#[cfg(windows)]
use windows_platform_native::{ClockAdjustment, ResumeWatch, read_clock_adjustment};

#[cfg(any(windows, test))]
#[path = "nevrotick_observe/adjustment_observation.rs"]
mod adjustment_observation;

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
    /// The requested raw adjustment read needs the native Windows adapter.
    #[cfg(not(windows))]
    AdjustmentUnavailable,
    /// Windows could not establish resume notification before acquisition.
    #[cfg(windows)]
    ResumeRegistration,
    /// The callback was disarmed but native release failed. Keep an earlier failure.
    #[cfg(windows)]
    ResumeUnregistration {
        /// Preserve the first bounded diagnostic, if observation also failed.
        primary: Option<&'static str>,
    },
}

impl CliError {
    /// Project a fixed code without endpoint text, OS messages or raw packets.
    fn as_str(&self) -> &'static str {
        match self {
            Self::Arguments => "invalid_arguments",
            Self::Endpoint => "invalid_endpoint",
            Self::Output => "diagnostic_output_failed",
            Self::Pilot(error) => error.as_str(),
            #[cfg(not(windows))]
            Self::AdjustmentUnavailable => "clock_adjustment_unavailable",
            #[cfg(windows)]
            Self::ResumeRegistration => "resume_registration_failed",
            #[cfg(windows)]
            Self::ResumeUnregistration { primary } => {
                primary.unwrap_or("resume_unregistration_failed")
            }
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
    /// Explicit caller-declared experimental dates and software-clock model.
    experimental_profile: Option<ExperimentalProfile>,
    /// Request two read-only Windows adjustment observations.
    observe_adjustment: bool,
}

/// Private allowance for at most seven option/value pairs in this one CLI parse.
struct ArgumentBudget {
    /// Unconsumed complete-pair allowance; exhaustion can never wrap or reset.
    remaining_pairs: u8,
}

impl ArgumentBudget {
    /// Establish the CLI's fixed fourteen-atom allowance without reading input.
    fn for_cli() -> Self {
        Self { remaining_pairs: 7 }
    }

    /// Charge one raw option before decoding it or fetching its paired value.
    ///
    /// # Errors
    /// Reject exhaustion and preserve the previous state; repeated failure stays exhausted.
    fn charge_pair(&mut self) -> Result<(), CliError> {
        let remaining = self
            .remaining_pairs
            .checked_sub(1)
            .ok_or(CliError::Arguments)?;
        self.remaining_pairs = remaining;
        Ok(())
    }
}

/// Closed raw argument roles; callers cannot choose an arbitrary decoding budget.
#[derive(Clone, Copy)]
enum ArgumentTextKind {
    /// Option names admit at most thirty-two encoded bytes before grammar matching.
    Option,
    /// Option values admit at most one hundred twenty-eight encoded bytes.
    Value,
}

/// Admit bounded text without copying or retaining the original OS argument.
/// Check encoded byte length BEFORE UTF-8 conversion; grammar remains with the parser.
///
/// # Errors
/// Reject an over-budget or non-text argument with the same fixed syntax error.
fn argument_text(value: &OsStr, kind: ArgumentTextKind) -> Result<&str, CliError> {
    let maximum = match kind {
        ArgumentTextKind::Option => 32,
        ArgumentTextKind::Value => 128,
    };
    if value.len() > maximum {
        return Err(CliError::Arguments);
    }
    value.to_str().ok_or(CliError::Arguments)
}

/// Parse at most fourteen bounded arguments without retaining raw caller strings.
///
/// No network, file or clock mutation occurs. Endpoint meaning and timeout bounds
/// are admitted later through Config, which direct library callers also require.
///
/// # Errors
/// Reject missing/repeated options, oversized or non-text arguments, invalid
/// numeric endpoints and disallowed decimal duration syntax.
fn arguments(mut raw: impl Iterator<Item = OsString>) -> Result<Arguments, CliError> {
    let mut peers = Vec::with_capacity(2);
    let mut timeout = None;
    let mut stop_after = None;
    let mut invalidate_after = None;
    let mut experimental_profile = None;
    let mut observe_adjustment: bool = false;
    let mut budget = ArgumentBudget::for_cli();
    while let Some(option) = raw.next() {
        budget.charge_pair()?;
        let option = argument_text(&option, ArgumentTextKind::Option)?;
        let value = raw.next().ok_or(CliError::Arguments)?;
        let value = argument_text(&value, ArgumentTextKind::Value)?;
        match option {
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
            "--experimental-clock" if experimental_profile.is_none() => {
                experimental_profile = Some(clock_profile(value)?);
            }
            "--clock-adjustment" if value == "raw" && !observe_adjustment => {
                observe_adjustment = true;
            }
            _ => return Err(CliError::Arguments),
        }
    }
    Ok(Arguments {
        peers: peers.try_into().map_err(|_| CliError::Arguments)?,
        timeout: timeout.ok_or(CliError::Arguments)?,
        stop_after,
        invalidate_after,
        experimental_profile,
        observe_adjustment,
    })
}

/// Decode exactly three bounded unsigned decimal declarations, without inference.
///
/// The inclusive whole NTP seconds and comparison allowance are caller-supplied
/// experimental assumptions, never trusted date evidence or wire-time accuracy.
///
/// # Errors
/// Reject missing/extra fields, signs, whitespace, overflow or invalid profile bounds.
fn clock_profile(value: &str) -> Result<ExperimentalProfile, CliError> {
    let mut fields = value.split(':');
    let earliest = unsigned_decimal(fields.next().ok_or(CliError::Arguments)?)?;
    let latest = unsigned_decimal(fields.next().ok_or(CliError::Arguments)?)?;
    let allowance = unsigned_decimal(fields.next().ok_or(CliError::Arguments)?)?;
    if fields.next().is_some() {
        return Err(CliError::Arguments);
    }
    ExperimentalProfile::new(earliest, latest, allowance).map_err(|_| CliError::Arguments)
}

/// Decode one at-most-twenty-digit unsigned value; retain no raw caller text.
///
/// # Errors
/// Reject empty, signed, non-ASCII-decimal or unrepresentable values.
fn unsigned_decimal(value: &str) -> Result<u64, CliError> {
    if value.is_empty() || value.len() > 20 || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(CliError::Arguments);
    }
    value.parse().map_err(|_| CliError::Arguments)
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

/// Write two slot records, at most two measurement records and one completion record.
/// Every finite measurement line is below 1,024 bytes even at integer extremes.
/// A pre-send capture refusal adds its fixed reason to its result line.
/// Raw packets, tokens, native error text and endpoint strings are never included.
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
        if let Some(reason) = slot.capture_refusal() {
            write!(output, " reason={}", reason.as_str()).map_err(|_| CliError::Output)?;
        }
        writeln!(output).map_err(|_| CliError::Output)?;
        slot.write_measurement(output)
            .map_err(|_| CliError::Output)?;
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
    let args: Arguments = arguments(std::env::args_os().skip(1))?;
    #[cfg(not(windows))]
    if args.observe_adjustment {
        return Err(CliError::AdjustmentUnavailable);
    }
    let config = Config::new(args.peers, args.timeout).map_err(CliError::Pilot)?;
    let config = match args.experimental_profile {
        Some(profile) => config.with_experimental_profile(profile),
        None => config,
    };
    let schedule =
        ControlSchedule::new(args.stop_after, args.invalidate_after).map_err(CliError::Pilot)?;
    // OS entropy and synchronous socket setup precede active-control timing.
    // Their source review does not establish bounded startup latency.
    let (pilot, control): (PreparedPilot, ControlHandle) =
        prepare(&config).map_err(CliError::Pilot)?;
    #[cfg(windows)]
    let observer: ResumeWatch =
        ResumeWatch::subscribe(move || control.invalidate().map_err(|_| ()))
            .map_err(|_| CliError::ResumeRegistration)?;
    let result: Result<ObservedPilot, CliError> = observe(
        pilot,
        schedule,
        #[cfg(windows)]
        args.observe_adjustment,
    );
    #[cfg(windows)]
    if observer.close().is_err() {
        let primary: Option<&'static str> = result.err().map(|error| error.as_str());
        return Err(CliError::ResumeUnregistration { primary });
    }
    #[cfg(not(windows))]
    drop(control);
    let completed: ObservedPilot = result?;
    let mut output: io::StdoutLock<'static> = io::stdout().lock();
    #[cfg(windows)]
    if let Some(adjustment) = completed.adjustment {
        adjustment
            .write(&mut output)
            .map_err(|_| CliError::Output)?;
    }
    write_report(&completed.report, &mut output)
}

/// The actual pilot result with its optional native observation from the same invocation.
struct ObservedPilot {
    /// The original result after the actual engine runtime returned.
    report: PilotReport,
    /// Two raw reads around that specific Windows invocation, when requested.
    #[cfg(windows)]
    adjustment: Option<adjustment_observation::AdjustmentObservation>,
}

/// Forward one native read into the CLI's bounded historical projection.
///
/// # Errors
/// Preserve the original native error code, including an unreported code of zero.
#[cfg(windows)]
fn read_adjustment() -> Result<adjustment_observation::AdjustmentState, u32> {
    let state: ClockAdjustment = read_clock_adjustment()?;
    Ok(adjustment_observation::AdjustmentState {
        adjustment: state.adjustment(),
        increment: state.increment(),
        periodic_adjustment_disabled: state.periodic_adjustment_disabled(),
    })
}

/// Publish the prepared endpoints before executing, under the invocation's control.
///
/// # Errors
/// Preserve sink/setup and Engine errors, leaving teardown to the invocation owner.
fn observe(
    pilot: PreparedPilot,
    schedule: ControlSchedule,
    #[cfg(windows)] include_adjustment: bool,
) -> Result<ObservedPilot, CliError> {
    {
        let mut output: io::StdoutLock<'static> = io::stdout().lock();
        for (slot, local) in pilot.local_endpoints().iter().enumerate() {
            writeln!(output, "event=bound slot={slot} local={local}")
                .map_err(|_| CliError::Output)?;
        }
        output.flush().map_err(|_| CliError::Output)?;
    }
    #[cfg(windows)]
    if include_adjustment {
        let (report, reading): (
            Result<PilotReport, PilotError>,
            adjustment_observation::AdjustmentObservation,
        ) = adjustment_observation::observe(read_adjustment, || execute(pilot, schedule));
        return Ok(ObservedPilot {
            report: report.map_err(CliError::Pilot)?,
            adjustment: Some(reading),
        });
    }
    Ok(ObservedPilot {
        report: execute(pilot, schedule).map_err(CliError::Pilot)?,
        #[cfg(windows)]
        adjustment: None,
    })
}

/// Return through Rust's normal process teardown; never change host clock state.
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let mut stderr: io::StderrLock<'static> = io::stderr().lock();
            let result: io::Result<()> = writeln!(stderr, "error={}", error.as_str());
            #[cfg(windows)]
            if let CliError::ResumeUnregistration { primary: Some(_) } = error {
                let _cleanup_written: io::Result<()> =
                    writeln!(stderr, "cleanup=resume_unregistration_failed");
            }
            result.map_or(ExitCode::FAILURE, |()| ExitCode::from(2))
        }
    }
}

/// Direct resource-owner checks live outside the production coverage denominator.
#[cfg(test)]
#[path = "../../tests/private/argument_guards.rs"]
mod argument_guard_tests;

#[cfg(test)]
#[path = "../../tests/private/adjustment_observation.rs"]
mod adjustment_observation_tests;

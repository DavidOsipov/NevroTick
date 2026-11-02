//! Held diagnostic of the owned CNG provider's actual private initializer.
//!
//! All native work stays behind the derivative provider's safe probe API. This
//! binary forbids unsafe code, prints only fixed labels/ordinals/native statuses,
//! and distinguishes a baseline cleanup RED from an incomplete observation.

#![forbid(unsafe_code)]

use std::ffi::{OsStr, OsString};
#[cfg(windows)]
use std::io::Read;
use std::io::{self, Write};
use std::process::ExitCode;

#[cfg(windows)]
use rustls_cng_crypto::init_probe::{self, Conclusion, Operation, Phase, ProbeCase, ProbeReport};
#[cfg(windows)]
use rustls_cng_crypto::init_probe::{CacheCase, CacheOperation, CacheSession, CacheSnapshot};
#[cfg(windows)]
use rustls_cng_crypto::{ShutdownHandle, kx_group::X25519};

/// Exact case selected before any provider startup.
#[derive(Clone, Copy)]
enum Case {
    /// Normal private initializer and returned-owner cleanup.
    HealthyLoad,
    /// One actual invalid-algorithm call, then a fault-off neighbor.
    OpenInvalidOnce,
    /// One actual invalid-property call, then a fault-off neighbor.
    PropertyInvalidOnce,
    /// A real property failure through the public, cached X25519 initializer.
    CachedPropertyFailure,
    /// Drop the sole unused shutdown owner before any X25519 start.
    UnusedShutdown,
    /// Two public exchanges complete before the single shutdown owner drops.
    CachedHealthy,
}

impl Case {
    /// Admit only a fixed representation; raw arguments are never retained.
    fn parse(value: &OsStr) -> Option<Self> {
        if value == OsStr::new("healthy-load") {
            Some(Self::HealthyLoad)
        } else if value == OsStr::new("open-invalid-once") {
            Some(Self::OpenInvalidOnce)
        } else if value == OsStr::new("property-invalid-once") {
            Some(Self::PropertyInvalidOnce)
        } else if value == OsStr::new("cached-property-failure") {
            Some(Self::CachedPropertyFailure)
        } else if value == OsStr::new("unused-shutdown") {
            Some(Self::UnusedShutdown)
        } else if value == OsStr::new("cached-healthy") {
            Some(Self::CachedHealthy)
        } else {
            None
        }
    }

    /// Return the bounded admitted label.
    fn as_str(self) -> &'static str {
        match self {
            Self::HealthyLoad => "healthy-load",
            Self::OpenInvalidOnce => "open-invalid-once",
            Self::PropertyInvalidOnce => "property-invalid-once",
            Self::CachedPropertyFailure => "cached-property-failure",
            Self::UnusedShutdown => "unused-shutdown",
            Self::CachedHealthy => "cached-healthy",
        }
    }

    /// Map the closed CLI case into the provider's closed safe input.
    #[cfg(windows)]
    fn native(self) -> Option<ProbeCase> {
        match self {
            Self::HealthyLoad => Some(ProbeCase::HealthyLoad),
            Self::OpenInvalidOnce => Some(ProbeCase::OpenInvalidOnce),
            Self::PropertyInvalidOnce => Some(ProbeCase::PropertyInvalidOnce),
            Self::CachedPropertyFailure | Self::UnusedShutdown | Self::CachedHealthy => None,
        }
    }
}

/// Inspect exactly `--case LABEL`, consuming at most four iterator values.
fn parse_case(mut arguments: impl Iterator<Item = OsString>) -> Option<Case> {
    arguments.next()?;
    let flag = arguments.next()?;
    let label = arguments.next()?;
    if flag != OsStr::new("--case") || arguments.next().is_some() {
        return None;
    }
    Case::parse(&label)
}

/// Project a boolean without an allocation or an ambiguous absent value.
#[cfg(windows)]
fn bit(value: bool) -> &'static str {
    if value { "1" } else { "0" }
}

/// Project the actual native phase using only fixed labels.
#[cfg(windows)]
fn phase_label(phase: Phase) -> &'static str {
    match phase {
        Phase::Initial => "initial",
        Phase::Neighbor => "neighbor",
    }
}

/// Project the actual native operation, keeping both fixture cleanup roles distinct.
#[cfg(windows)]
fn operation_label(operation: Operation) -> &'static str {
    match operation {
        Operation::Open => "open",
        Operation::SetProperty => "set-property",
        Operation::ProviderCloseError => "provider-close-error",
        Operation::CloseReturned => "fixture-close-returned",
        Operation::CloseRecovery => "fixture-close-recovery",
    }
}

/// Write the saved pre-recovery snapshot; output itself occurs after native work.
///
/// # Errors
/// Propagates the actual sink failure without claiming a complete observation.
#[cfg(windows)]
fn write_snapshot(output: &mut impl Write, case: Case, report: &ProbeReport) -> io::Result<()> {
    writeln!(
        output,
        "event=probe-snapshot case={} saved_before_recovery=1 initial_returned={} orphans={} fault_consumed={}",
        case.as_str(),
        bit(report.initial_returned()),
        report.orphans_before_recovery(),
        bit(report.fault_consumed())
    )
}

/// Emit at most eight native events, one saved snapshot and one final result.
///
/// # Errors
/// Propagates write/flush failure; raw handles and provider strings are excluded.
#[cfg(windows)]
fn write_native(case: Case, report: &ProbeReport) -> io::Result<ExitCode> {
    let mut output = io::stdout().lock();
    let mut snapshot_written = false;
    for (index, event) in report.events().enumerate() {
        if index == report.before_recovery_events() {
            write_snapshot(&mut output, case, report)?;
            snapshot_written = true;
        }
        let ordinal = event
            .ordinal()
            .map_or_else(|| "none".to_owned(), |value| value.to_string());
        writeln!(
            output,
            "event=probe-native case={} phase={} operation={} ordinal={ordinal} status_i32={} status_hex={:08x} nt_success={}",
            case.as_str(),
            phase_label(event.phase()),
            operation_label(event.operation()),
            event.status(),
            event.status_bits(),
            bit(event.succeeded())
        )?;
    }
    if !snapshot_written {
        write_snapshot(&mut output, case, report)?;
    }
    let (status, code) = match report.conclusion() {
        Conclusion::Healthy => ("healthy", ExitCode::SUCCESS),
        Conclusion::OpenFailureObserved => ("open-failure-observed", ExitCode::SUCCESS),
        Conclusion::PropertyFailureCleaned => ("property-failure-cleaned", ExitCode::SUCCESS),
        Conclusion::CleanupGap => ("baseline-red", ExitCode::from(3)),
        Conclusion::Incomplete => ("incomplete", ExitCode::from(1)),
    };
    let neighbor = match report.neighbor_returned() {
        Some(true) => "ok",
        Some(false) => "error",
        None => "absent",
    };
    writeln!(
        output,
        "event=probe-result case={} status={status} neighbor_result={neighbor} cached_initializer=not-called shutdown_handle=not-created",
        case.as_str()
    )?;
    output.flush()?;
    Ok(code)
}

/// Write one closed boundary failure without invoking any native probe.
///
/// Callers supply only fixed case/cause labels, never raw CLI or provider text.
///
/// # Errors
/// Propagates the actual output write/flush error.
fn write_failure(scenario: &str, cause: &str) -> io::Result<()> {
    let mut output = io::stdout().lock();
    writeln!(
        output,
        "event=probe-result case={scenario} status=incomplete cause={cause}"
    )?;
    output.flush()
}

/// The public operation's control-flow outcome, without its error or panic payload.
#[cfg(windows)]
#[derive(Clone, Copy)]
enum PublicOutcome {
    /// Public operation returned the expected successful value.
    Success,
    /// Public operation returned a normal typed error.
    Error,
    /// Public operation unwound into the diagnostic boundary.
    Panic,
    /// The unused-owner case deliberately made no public call.
    NotCalled,
}

/// Project only fixed cache-operation labels.
#[cfg(windows)]
fn cache_operation_label(operation: CacheOperation) -> &'static str {
    match operation {
        CacheOperation::Open => "open",
        CacheOperation::SetProperty => "set-property",
        CacheOperation::ProviderCloseError => "provider-close-error",
        CacheOperation::ShutdownClose => "shutdown-close",
    }
}

/// Save the bounded observations without exposing native handles or input bytes.
///
/// # Errors
/// Returns the actual output write or flush failure.
#[cfg(windows)]
fn write_cache_snapshot(
    case: Case,
    phase: &str,
    outcome: PublicOutcome,
    snapshot: &CacheSnapshot,
) -> io::Result<()> {
    let mut output = io::stdout().lock();
    let public = match outcome {
        PublicOutcome::Success => "success",
        PublicOutcome::Error => "error",
        PublicOutcome::Panic => "panic",
        PublicOutcome::NotCalled => "not-called",
    };
    writeln!(
        output,
        "event=cache-snapshot case={} phase={phase} public={public} valid={} fault_consumed={} opens={} outstanding={}",
        case.as_str(),
        bit(snapshot.valid()),
        bit(snapshot.fault_consumed()),
        snapshot.open_count(),
        snapshot.outstanding()
    )?;
    for event in snapshot.events() {
        let ordinal = event
            .ordinal()
            .map_or_else(|| "none".to_owned(), |value| value.to_string());
        writeln!(
            output,
            "event=cache-native case={} phase={phase} operation={} ordinal={ordinal} status_i32={} status_hex={:08x} nt_success={}",
            case.as_str(),
            cache_operation_label(event.operation()),
            event.status(),
            event.status_bits(),
            bit(event.succeeded())
        )?;
    }
    output.flush()
}

/// Check a complete, independently specified sequence of actual native calls.
#[cfg(windows)]
fn events_match(snapshot: &CacheSnapshot, expected: &[(&str, Option<u8>, bool)]) -> bool {
    snapshot.events().count() == expected.len()
        && snapshot
            .events()
            .zip(expected)
            .all(|(event, &(name, ordinal, ok))| {
                cache_operation_label(event.operation()) == name
                    && event.ordinal() == ordinal
                    && event.succeeded() == ok
            })
}

/// Run two genuine public exchanges; no returned secret survives this function.
///
/// # Errors
/// Returns an error on exchange failure, invalid public keys or secret disagreement.
#[cfg(windows)]
fn public_healthy_pair() -> Result<(), ()> {
    let left = X25519.start().map_err(|_| ())?;
    let right = X25519.start().map_err(|_| ())?;
    let left_public: [u8; 32] = left.pub_key().try_into().map_err(|_| ())?;
    let right_public: [u8; 32] = right.pub_key().try_into().map_err(|_| ())?;
    if left_public == right_public {
        return Err(());
    }
    let left_secret = left.complete(&right_public).map_err(|_| ())?;
    let right_secret = right.complete(&left_public).map_err(|_| ())?;
    let left_bytes = left_secret.secret_bytes();
    let right_bytes = right_secret.secret_bytes();
    if left_bytes.len() != 32
        || left_bytes != right_bytes
        || left_bytes.iter().all(|byte| *byte == 0)
    {
        return Err(());
    }
    Ok(())
}

/// Interpret only complete actual observations; an unwind is never success.
#[cfg(windows)]
fn cache_verdict(
    case: Case,
    outcome: PublicOutcome,
    before: &CacheSnapshot,
    after: &CacheSnapshot,
) -> (&'static str, ExitCode) {
    let failed_property = [
        ("open", Some(0), true),
        ("set-property", Some(0), false),
        ("provider-close-error", Some(0), true),
    ];
    let failed_then_shutdown = [
        ("open", Some(0), true),
        ("set-property", Some(0), false),
        ("provider-close-error", Some(0), true),
        ("open", Some(1), true),
        ("set-property", Some(1), true),
        ("shutdown-close", Some(1), true),
    ];
    let acquired = [("open", Some(0), true), ("set-property", Some(0), true)];
    let acquired_and_closed = [
        ("open", Some(0), true),
        ("set-property", Some(0), true),
        ("shutdown-close", Some(0), true),
    ];
    if !before.valid() || !after.valid() || after.outstanding() != 0 {
        return ("incomplete", ExitCode::from(1));
    }
    match case {
        Case::CachedPropertyFailure
            if before.fault_consumed()
                && before.open_count() == 1
                && before.outstanding() == 0
                && events_match(before, &failed_property)
                && after.fault_consumed() =>
        {
            let no_new_work = after.open_count() == 1 && events_match(after, &failed_property);
            let drop_started =
                after.open_count() == 2 && events_match(after, &failed_then_shutdown);
            if !no_new_work && !drop_started {
                ("incomplete", ExitCode::from(1))
            } else if matches!(outcome, PublicOutcome::Panic)
                || (matches!(outcome, PublicOutcome::Error) && drop_started)
            {
                ("baseline-red", ExitCode::from(3))
            } else if matches!(outcome, PublicOutcome::Error) && no_new_work {
                ("error-propagated", ExitCode::SUCCESS)
            } else {
                ("incomplete", ExitCode::from(1))
            }
        }
        Case::UnusedShutdown
            if matches!(outcome, PublicOutcome::NotCalled)
                && before.open_count() == 0
                && before.outstanding() == 0
                && events_match(before, &[])
                && !before.fault_consumed()
                && !after.fault_consumed() =>
        {
            if after.open_count() == 0 && events_match(after, &[]) {
                ("unused-owner-no-start", ExitCode::SUCCESS)
            } else if after.open_count() == 1 && events_match(after, &acquired_and_closed) {
                ("baseline-red", ExitCode::from(3))
            } else {
                ("incomplete", ExitCode::from(1))
            }
        }
        Case::CachedHealthy
            if matches!(outcome, PublicOutcome::Success)
                && !before.fault_consumed()
                && !after.fault_consumed()
                && before.open_count() == 1
                && before.outstanding() == 1
                && events_match(before, &acquired)
                && after.open_count() == 1
                && events_match(after, &acquired_and_closed) =>
        {
            ("healthy", ExitCode::SUCCESS)
        }
        _ => ("incomplete", ExitCode::from(1)),
    }
}

/// Require the fixed owner arm and EOF before any cached provider work.
///
/// Input is bounded to the literal plus one byte. The owned parent must close
/// stdin and enforce the process deadline; this blocking read has no timer.
///
/// # Errors
/// Returns an error for a short, incorrect, overlong or unreadable arm.
#[cfg(windows)]
fn await_cache_arm() -> io::Result<()> {
    const ARM: &[u8] = b"NEVROTICK-CACHE-ARM-V1\n";
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut received = [0_u8; ARM.len()];
    input.read_exact(&mut received)?;
    if received.as_slice() != ARM {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid arm"));
    }
    let mut extra = [0_u8; 1];
    loop {
        match input.read(&mut extra) {
            Ok(0) => return Ok(()),
            Ok(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "overlong arm")),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(error),
        }
    }
}

/// Execute one bounded cache diagnostic with exactly one shutdown owner.
#[cfg(windows)]
fn execute_cache(case: Case) -> ExitCode {
    if await_cache_arm().is_err() {
        if write_failure(case.as_str(), "arm-rejected").is_err() {
            return ExitCode::from(1);
        }
        return ExitCode::from(1);
    }
    let selected = match case {
        Case::CachedPropertyFailure => CacheCase::CachedPropertyFailure,
        Case::UnusedShutdown => CacheCase::UnusedShutdown,
        Case::CachedHealthy => CacheCase::CachedHealthy,
        _ => return ExitCode::from(1),
    };
    let Ok(session) = CacheSession::begin(selected) else {
        if write_failure(case.as_str(), "session-unavailable").is_err() {
            return ExitCode::from(1);
        }
        return ExitCode::from(1);
    };
    // Keep the child-local redacting hook through the single shutdown drop.
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let shutdown = ShutdownHandle {};
    let outcome = if matches!(case, Case::UnusedShutdown) {
        PublicOutcome::NotCalled
    } else {
        let result = std::panic::catch_unwind(|| {
            if matches!(case, Case::CachedHealthy) {
                public_healthy_pair()
            } else {
                // An unexpectedly successful exchange is dropped inside this scope.
                X25519.start().map(|_| ()).map_err(|_| ())
            }
        });
        match result {
            Ok(Ok(())) => PublicOutcome::Success,
            Ok(Err(())) => PublicOutcome::Error,
            Err(_) => PublicOutcome::Panic,
        }
    };
    // The first snapshot is flushed before fault disarm and owner destruction.
    let before = session.snapshot();
    let saved_before = before
        .as_ref()
        .is_ok_and(|snapshot| write_cache_snapshot(case, "before-drop", outcome, snapshot).is_ok());
    let disarmed = session.disarm().is_ok();
    // Catch only to restore the hook and mark an incomplete shutdown. The
    // consumed owner is never dropped a second time after an unwind.
    let drop_result = std::panic::catch_unwind(|| drop(shutdown));
    let shutdown_ok = drop_result.is_ok();
    drop(drop_result);
    let after = session.finish();
    std::panic::set_hook(original_hook);
    let (Ok(before), Ok(after)) = (before, after) else {
        if write_failure(case.as_str(), "observation-unavailable").is_err() {
            return ExitCode::from(1);
        }
        return ExitCode::from(1);
    };
    let saved_after = write_cache_snapshot(case, "after-drop", outcome, &after).is_ok();
    let (status, code) = if saved_before && disarmed && shutdown_ok && saved_after {
        cache_verdict(case, outcome, &before, &after)
    } else {
        ("incomplete", ExitCode::from(1))
    };
    let mut output = io::stdout().lock();
    if writeln!(
        output,
        "event=cache-result case={} status={status}",
        case.as_str()
    )
    .is_err()
        || output.flush().is_err()
    {
        return ExitCode::from(1);
    }
    code
}

/// Invoke the one safe provider diagnostic; dependency panics are not success.
#[cfg(windows)]
fn execute(case: Case) -> ExitCode {
    let Some(native) = case.native() else {
        return execute_cache(case);
    };
    match init_probe::run(native) {
        Ok(report) => match write_native(case, &report) {
            Ok(code) => code,
            Err(_error) => ExitCode::from(1),
        },
        Err(error) => {
            let failure_cause = match error {
                init_probe::ProbeError::AlreadyUsed => "already-used",
                init_probe::ProbeError::StateUnavailable => "state-unavailable",
            };
            if write_failure(case.as_str(), failure_cause).is_err() {
                return ExitCode::from(1);
            }
            ExitCode::from(1)
        }
    }
}

/// Report unsupported native execution explicitly, with no provider load.
#[cfg(not(windows))]
fn execute(case: Case) -> ExitCode {
    if write_failure(case.as_str(), "unsupported").is_err() {
        return ExitCode::from(1);
    }
    ExitCode::from(1)
}

/// Admit the fixed CLI before native work; exit three means a real baseline RED.
///
/// An independently bounded parent must retain every native exit and raw record.
/// Zero reports only the selected observed control or error cleanup, never qualification.
fn main() -> ExitCode {
    if let Some(case) = parse_case(std::env::args_os()) {
        execute(case)
    } else {
        if write_failure("invalid", "usage").is_err() {
            return ExitCode::from(1);
        }
        ExitCode::from(2)
    }
}

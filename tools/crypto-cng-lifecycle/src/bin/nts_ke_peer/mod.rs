//! Private admission and finite budgets for the single healthy laboratory peer.
//!
//! Public protocol literals are independent of the client encoder. Private-key
//! paths, plaintext, certificates and exporter bytes never enter diagnostics.

use std::ffi::OsString;
use std::path::{Component, PathBuf};
use std::time::{Duration, Instant};

#[cfg(windows)]
mod transport;

/// Entire peer lifetime, measured from the original invocation before setup.
const LIFETIME: Duration = Duration::from_secs(60);
/// Maximum wait between original-deadline observations during network work.
const POLL: Duration = Duration::from_millis(200);
/// Independent cumulative encrypted-input and encrypted-output byte limits.
const IO_LIMIT: usize = 65_536;
/// Exact public NPN0 / AEAD15 / EOM offer, including critical record bits.
const OFFER: [u8; 16] = [
    0x80, 1, 0, 2, 0, 0, 0x80, 4, 0, 2, 0, 15, 0x80, 0, 0, 0,
];
/// Exact response with one public four-byte cookie; no endpoint alternative.
const RESPONSE: [u8; 24] = [
    0x80, 1, 0, 2, 0, 0, 0x80, 4, 0, 2, 0, 15, 0, 5, 0, 4, 0xaa, 0xbb, 0xcc, 0xdd,
    0x80, 0, 0, 0,
];

/// Closed failures contain no rejected path, TLS message or secret material.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Failure {
    /// The exact argument shape or bounded absolute path was not admitted.
    Arguments,
    /// The original finite deadline was reached, including equality.
    Deadline,
    /// A cumulative byte allowance or checked counter was exhausted.
    Budget,
    /// Received public application bytes differ from the one exact offer.
    Offer,
    /// A native operation refused; retain only its typed I/O category.
    Io(std::io::ErrorKind),
    /// DER/key admission or fallible bounded allocation refused.
    #[cfg(windows)]
    Input,
    /// Provider, TLS profile, signing or exporter operation refused.
    #[cfg(windows)]
    Tls,
    /// EOF without the peer's TLS close-notify cannot complete this fixture.
    #[cfg(windows)]
    Closure,
    /// Preserve readiness failure and any distinct owned-temp cleanup refusal.
    #[cfg(windows)]
    Ready {
        /// Original write/publication cause, or the sole cleanup cause.
        primary: std::io::ErrorKind,
        /// Secondary temp removal cause; never replaces the original failure.
        cleanup: Option<std::io::ErrorKind>,
    },
}

/// Preserve a primary operation refusal and a distinct later transport refusal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Failed {
    /// Original protocol/operation cause; cleanup cannot replace this value.
    primary: Failure,
    /// Independently observed secondary fault, containing only closed facts.
    cleanup: Option<Failure>,
}

impl From<Failure> for Failed {
    /// A single refusal has no independently observed cleanup cause.
    fn from(primary: Failure) -> Self {
        Self { primary, cleanup: None }
    }
}

/// Preserve a TLS invalid-data error before a later best-effort alert fault.
/// Otherwise a sticky fault refuses even nominal success; no fault keeps result.
fn resolve<T>(fault: Option<Failure>, result: Result<T, Failure>) -> Result<T, Failed> {
    match result {
        Err(primary @ Failure::Io(std::io::ErrorKind::InvalidData)) => Err(Failed {
            primary,
            cleanup: fault.filter(|secondary| *secondary != primary),
        }),
        other => match fault {
            Some(primary) => Err(primary.into()),
            None => other.map_err(Failed::from),
        },
    }
}

/// Three admitted local path representations; actual files require native checks.
struct Config {
    /// Public leaf DER, copied into the owned guest's private work directory.
    certificate: PathBuf,
    /// Private PKCS8 DER, opened without write/delete sharing or reparse following.
    key: PathBuf,
    /// New private-NTFS readiness record; never an existing record or mapped share.
    ready: PathBuf,
}

impl Config {
    /// Admit exactly three distinct bounded absolute ASCII paths before effects.
    fn parse(mut arguments: impl Iterator<Item = OsString>) -> Result<Self, Failure> {
        let (Some(_program), Some(certificate), Some(key), Some(ready), None) = (
            arguments.next(),
            arguments.next(),
            arguments.next(),
            arguments.next(),
            arguments.next(),
        ) else {
            return Err(Failure::Arguments);
        };
        let certificate = checked_path(certificate)?;
        let key = checked_path(key)?;
        let ready = checked_path(ready)?;
        if certificate == key || certificate == ready || key == ready {
            return Err(Failure::Arguments);
        }
        Ok(Self {
            certificate,
            key,
            ready,
        })
    }
}

/// Bound one path before conversion; reject relative/parent/root-only spellings.
fn checked_path(value: OsString) -> Result<PathBuf, Failure> {
    if value.is_empty()
        || value.len() > 1024
        || !value
            .to_str()
            .is_some_and(|text| text.is_ascii() && !text.as_bytes().contains(&0))
    {
        return Err(Failure::Arguments);
    }
    let path = PathBuf::from(value);
    if !path.is_absolute()
        || path.file_name().is_none()
        || path
            .components()
            .any(|part| matches!(part, Component::ParentDir))
    {
        return Err(Failure::Arguments);
    }
    Ok(path)
}

/// Only the fixed public offer authorizes sending the fixed public reply.
fn admit_offer(bytes: &[u8]) -> Result<(), Failure> {
    if bytes == OFFER {
        Ok(())
    } else {
        Err(Failure::Offer)
    }
}

/// Serialize only a nonzero numeric port into at most eleven ASCII bytes.
fn ready_line(port: u16) -> Result<String, Failure> {
    if port == 0 {
        return Err(Failure::Arguments);
    }
    Ok(format!("port={port}\n"))
}

/// Select one independent encrypted-I/O counter; no shared aggregate substitution.
#[derive(Clone, Copy)]
enum Flow {
    /// Bytes actually read from the accepted socket.
    Read,
    /// Bytes actually written to the accepted socket.
    Write,
}

/// Original deadline and checked counts; no TLS progress can renew either limit.
struct Budget {
    /// Immutable original-origin deadline; equality refuses.
    deadline: Instant,
    /// Total actual encrypted bytes read, never more than the fixed cap.
    read: usize,
    /// Separate actual encrypted output count under the same cap.
    written: usize,
}

impl Budget {
    /// Construct one finite lifetime without effects or a refreshed origin.
    fn new(origin: Instant) -> Result<Self, Failure> {
        Ok(Self {
            deadline: origin.checked_add(LIFETIME).ok_or(Failure::Deadline)?,
            read: 0,
            written: 0,
        })
    }

    /// Return a positive poll timeout, refusing equality and late observation.
    fn poll(&self, now: Instant) -> Result<Duration, Failure> {
        let left = self
            .deadline
            .checked_duration_since(now)
            .filter(|duration| !duration.is_zero())
            .ok_or(Failure::Deadline)?;
        Ok(left.min(POLL))
    }

    /// Limit the next syscall's actual buffer before any socket operation.
    fn allowance(&self, flow: Flow) -> Result<usize, Failure> {
        let count = match flow {
            Flow::Read => self.read,
            Flow::Write => self.written,
        };
        IO_LIMIT
            .checked_sub(count)
            .filter(|remaining| *remaining != 0)
            .ok_or(Failure::Budget)
    }

    /// Account actual bytes atomically; failure preserves the prior counter.
    fn charge(&mut self, flow: Flow, amount: usize) -> Result<(), Failure> {
        let count = match flow {
            Flow::Read => &mut self.read,
            Flow::Write => &mut self.written,
        };
        *count = count
            .checked_add(amount)
            .filter(|next| *next <= IO_LIMIT)
            .ok_or(Failure::Budget)?;
        Ok(())
    }
}

/// Admit CLI before touching CNG/files; report only closed bounded outcomes.
#[cfg(windows)]
pub(super) fn entry(arguments: impl Iterator<Item = OsString>) -> std::process::ExitCode {
    use std::io::Write;
    use std::process::ExitCode;
    let config = match Config::parse(arguments) {
        Ok(config) => config,
        Err(failure) => return report(failure.into(), ExitCode::from(2)),
    };
    match transport::serve(config) {
        Ok(()) => {
            let mut output = std::io::stdout().lock();
            if writeln!(output, "scope=laboratory-ke-peer completed=1 offer_bytes=16 response_bytes=24 exporters=2 peer_close=1").is_err()
                || output.flush().is_err()
            {
                return ExitCode::FAILURE;
            }
            ExitCode::SUCCESS
        }
        Err(failure) => report(failure, ExitCode::FAILURE),
    }
}

/// A failed diagnostic never converts its original operation exit to success.
#[cfg(windows)]
fn report(failure: Failed, code: std::process::ExitCode) -> std::process::ExitCode {
    use std::io::Write;
    let (stage, cause, readiness_cleanup) = labels(failure.primary);
    let (cleanup_stage, cleanup_cause, _) = failure
        .cleanup
        .map_or(("none", "none", "none"), labels);
    let _diagnostic_result = writeln!(
        std::io::stderr().lock(),
        "stage={stage} cause={cause} cleanup_stage={cleanup_stage} cleanup_cause={cleanup_cause} readiness_cleanup_cause={readiness_cleanup}"
    );
    code
}

/// Project only closed primary and owned-readiness-cleanup labels.
#[cfg(windows)]
fn labels(failure: Failure) -> (&'static str, &'static str, &'static str) {
    match failure {
        Failure::Arguments => ("admission", "arguments", "none"),
        Failure::Deadline => ("currentness", "deadline", "none"),
        Failure::Budget => ("resources", "byte-budget", "none"),
        Failure::Offer => ("request", "offer", "none"),
        Failure::Input => ("input", "file", "none"),
        Failure::Tls => ("tls", "refused", "none"),
        Failure::Closure => ("closure", "missing-notify", "none"),
        Failure::Io(kind) => ("io", io_label(kind), "none"),
        Failure::Ready { primary, cleanup } => (
            "readiness",
            io_label(primary),
            cleanup.map_or("none", io_label),
        ),
    }
}

/// Map typed native categories before formatting, retaining no arbitrary text.
#[cfg(windows)]
fn io_label(kind: std::io::ErrorKind) -> &'static str {
    match kind {
        std::io::ErrorKind::TimedOut => "timeout",
        std::io::ErrorKind::UnexpectedEof => "unexpected-eof",
        std::io::ErrorKind::AlreadyExists => "already-exists",
        std::io::ErrorKind::PermissionDenied => "permission",
        std::io::ErrorKind::ConnectionReset => "reset",
        std::io::ErrorKind::BrokenPipe => "broken-pipe",
        std::io::ErrorKind::InvalidData => "invalid-data",
        _ => "io",
    }
}

#[cfg(test)]
mod tests;

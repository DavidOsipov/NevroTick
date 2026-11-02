//! Held native CNG public-API fixture; one admitted case per fresh child process.
//!
//! The argument boundary precedes provider use. Windows cases keep one private
//! shutdown owner alive until every exchange and returned secret has dropped.
//! Output contains closed labels and actual call counters, never key bytes.
//! Source preparation grants no dependency-code, build or native admission.

#![forbid(unsafe_code)]

use std::ffi::{OsStr, OsString};
use std::io::{self, Write};
use std::process::ExitCode;

#[cfg(windows)]
use rustls::crypto::{ActiveKeyExchange, SharedSecret};
#[cfg(windows)]
use rustls_cng_crypto::{ShutdownHandle, kx_group::X25519};

/// Largest number of native exchange starts or completions in one fixed case.
#[cfg(windows)]
const CALL_LIMIT: u8 = 3;

/// A nonzero, deliberately short peer encoding; no valid-group claim is made.
#[cfg(windows)]
const SHORT_PEER: [u8; 31] = [1; 31];

/// A nonzero, deliberately long peer encoding; no valid-group claim is made.
#[cfg(windows)]
const LONG_PEER: [u8; 33] = [1; 33];

/// An admitted fixed case, constructed only from an exact CLI label.
#[derive(Clone, Copy)]
enum Case {
    /// Two native exchanges complete against each other's public share.
    Healthy,
    /// A 31-byte peer is rejected before a fresh healthy pair runs.
    PeerShort,
    /// A 33-byte peer is rejected before a fresh healthy pair runs.
    PeerLong,
}

impl Case {
    /// Admit an exact case label without formatting or retaining raw arguments.
    ///
    /// # Errors
    /// Returns the closed usage cause for every other representation.
    fn parse(label: &OsStr) -> Result<Self, Failure> {
        if label == OsStr::new("healthy") {
            Ok(Self::Healthy)
        } else if label == OsStr::new("peer-short") {
            Ok(Self::PeerShort)
        } else if label == OsStr::new("peer-long") {
            Ok(Self::PeerLong)
        } else {
            Err(Failure::Usage)
        }
    }

    /// Return the stable, bounded output label for an already admitted case.
    fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::PeerShort => "peer-short",
            Self::PeerLong => "peer-long",
        }
    }
}

/// Safe classification of an actual Rustls error, without its payload string.
#[cfg(windows)]
#[derive(Clone, Copy)]
enum Rejection {
    /// The provider returned the precise malformed-share error under test.
    InvalidKeyShare,
    /// Rustls carries a general provider error; native status is unavailable here.
    General,
    /// Another typed error was returned; its details are not projected.
    Other,
}

#[cfg(windows)]
impl Rejection {
    /// Classify a borrowed error without copying or exposing provider text.
    fn from_error(error: &rustls::Error) -> Self {
        match error {
            rustls::Error::PeerMisbehaved(rustls::PeerMisbehaved::InvalidKeyShare) => {
                Self::InvalidKeyShare
            }
            rustls::Error::General(_) => Self::General,
            _ => Self::Other,
        }
    }

    /// Return a closed cause label; no native status is inferred from text.
    fn as_str(self) -> &'static str {
        match self {
            Self::InvalidKeyShare => "invalid-key-share",
            Self::General => "general",
            Self::Other => "other",
        }
    }
}

/// A closed fixture failure; no variant owns raw input, key or error text.
#[derive(Clone, Copy)]
enum Failure {
    /// Arguments do not select exactly one supported case.
    Usage,
    /// This executable cannot perform the native Windows experiment here.
    #[cfg(not(windows))]
    Unsupported,
    /// Private call accounting would exceed the finite case budget.
    #[cfg(windows)]
    Budget,
    /// An actual provider start returned a classified error.
    #[cfg(windows)]
    Start(Rejection),
    /// A healthy completion returned a classified error.
    #[cfg(windows)]
    Complete(Rejection),
    /// A native public share has an unexpected length.
    #[cfg(windows)]
    PublicLength,
    /// The two healthy public shares are identical.
    #[cfg(windows)]
    DuplicatePublic,
    /// A completed shared secret has an unexpected length.
    #[cfg(windows)]
    SecretLength,
    /// The healthy exchanges disagree about their shared secret.
    #[cfg(windows)]
    SecretMismatch,
    /// The matching shared secret consists entirely of zero bytes.
    #[cfg(windows)]
    ZeroSecret,
    /// A private malformed-peer helper received an unintended fixture input.
    #[cfg(windows)]
    FixtureShape,
    /// The provider accepted the malformed peer instead of rejecting it.
    #[cfg(windows)]
    MalformedAccepted,
    /// The provider rejected malformed input with a different typed cause.
    #[cfg(windows)]
    WrongRejection(Rejection),
}

impl Failure {
    /// Identify the operation owning a failure using only fixed labels.
    fn stage(self) -> &'static str {
        match self {
            Self::Usage => "arguments",
            #[cfg(not(windows))]
            Self::Unsupported => "platform",
            #[cfg(windows)]
            Self::Budget => "budget",
            #[cfg(windows)]
            Self::Start(_) => "start",
            #[cfg(windows)]
            Self::Complete(_) => "complete",
            #[cfg(windows)]
            Self::PublicLength | Self::DuplicatePublic => "public-share",
            #[cfg(windows)]
            Self::SecretLength | Self::SecretMismatch | Self::ZeroSecret => "secret-check",
            #[cfg(windows)]
            Self::FixtureShape | Self::MalformedAccepted | Self::WrongRejection(_) => "peer-check",
        }
    }

    /// Return the bounded cause, preserving typed rejection distinctions.
    fn cause(self) -> &'static str {
        match self {
            Self::Usage => "usage",
            #[cfg(not(windows))]
            Self::Unsupported => "unsupported",
            #[cfg(windows)]
            Self::Budget => "call-limit",
            #[cfg(windows)]
            Self::Start(cause) | Self::Complete(cause) | Self::WrongRejection(cause) => {
                cause.as_str()
            }
            #[cfg(windows)]
            Self::PublicLength => "public-length",
            #[cfg(windows)]
            Self::DuplicatePublic => "duplicate-public",
            #[cfg(windows)]
            Self::SecretLength => "secret-length",
            #[cfg(windows)]
            Self::SecretMismatch => "secret-mismatch",
            #[cfg(windows)]
            Self::ZeroSecret => "zero-secret",
            #[cfg(windows)]
            Self::FixtureShape => "fixture-shape",
            #[cfg(windows)]
            Self::MalformedAccepted => "malformed-accepted",
        }
    }
}

/// Actual call observations, all initially zero and bounded by three on Windows.
#[derive(Default)]
struct Counts {
    /// Start calls admitted immediately before invoking the provider.
    start_attempts: u8,
    /// Start calls that actually returned an exchange owner.
    started: u8,
    /// Consuming completion calls admitted before invoking the provider.
    complete_attempts: u8,
    /// Completion calls that actually returned a shared-secret owner.
    completed: u8,
    /// Completion calls that actually returned any error.
    rejected: u8,
}

/// A final redacted observation, produced after the shutdown owner's drop returns.
struct Report {
    /// Admitted case, or no case when argument admission failed.
    case: Option<Case>,
    /// Counts of actual attempted and returned calls.
    counts: Counts,
    /// Case result; a panic or native fault cannot construct a successful report.
    outcome: Result<(), Failure>,
}

/// Parse exactly `--case LABEL`, inspecting at most three arguments after argv[0].
///
/// # Errors
/// Rejects missing, unknown or extra arguments before any CNG operation.
fn parse_case(mut arguments: impl Iterator<Item = OsString>) -> Result<Case, Failure> {
    if arguments.next().is_none() {
        return Err(Failure::Usage);
    }
    let flag = arguments.next().ok_or(Failure::Usage)?;
    let label = arguments.next().ok_or(Failure::Usage)?;
    if flag != OsStr::new("--case") || arguments.next().is_some() {
        return Err(Failure::Usage);
    }
    Case::parse(&label)
}

/// Advance one private counter without exceeding the fixed native-call limit.
///
/// # Errors
/// Leaves the counter unchanged if its value would exceed the case budget.
#[cfg(windows)]
fn increment(counter: &mut u8) -> Result<(), Failure> {
    let next = counter.checked_add(1).ok_or(Failure::Budget)?;
    if next > CALL_LIMIT {
        return Err(Failure::Budget);
    }
    *counter = next;
    Ok(())
}

/// Start one X25519 exchange after admitting its finite call budget.
///
/// # Errors
/// Returns a bounded budget or provider-start cause; no error payload is retained.
#[cfg(windows)]
fn start_exchange(counts: &mut Counts) -> Result<Box<dyn ActiveKeyExchange>, Failure> {
    increment(&mut counts.start_attempts)?;
    let exchange = X25519
        .start()
        .map_err(|error| Failure::Start(Rejection::from_error(&error)))?;
    increment(&mut counts.started)?;
    Ok(exchange)
}

/// The typed result of one consuming provider completion.
#[cfg(windows)]
enum Completion {
    /// Actual returned secret, retained only for an in-process comparison.
    Secret(SharedSecret),
    /// Actual rejection, projected without raw provider text.
    Rejected(Rejection),
}

/// Consume one actual exchange and account for its returned success or rejection.
///
/// # Errors
/// Rejects a completion without an available started exchange or finite budget.
#[cfg(windows)]
fn complete_exchange(
    exchange: Box<dyn ActiveKeyExchange>,
    peer: &[u8],
    counts: &mut Counts,
) -> Result<Completion, Failure> {
    if counts.complete_attempts >= counts.started {
        return Err(Failure::Budget);
    }
    increment(&mut counts.complete_attempts)?;
    match exchange.complete(peer) {
        Ok(secret) => {
            increment(&mut counts.completed)?;
            Ok(Completion::Secret(secret))
        }
        Err(error) => {
            increment(&mut counts.rejected)?;
            Ok(Completion::Rejected(Rejection::from_error(&error)))
        }
    }
}

/// Copy an exact public share into a fixed-size value; it is never logged.
///
/// # Errors
/// Rejects every public encoding whose length differs from 32 bytes.
#[cfg(windows)]
fn checked_public(share: &[u8]) -> Result<[u8; 32], Failure> {
    share.try_into().map_err(|_error| Failure::PublicLength)
}

/// Validate the pair's borrowed secrets without copying or formatting them.
///
/// # Errors
/// Rejects wrong lengths, disagreement or an all-zero shared secret.
#[cfg(windows)]
fn check_secrets(left: &[u8], right: &[u8]) -> Result<(), Failure> {
    if left.len() != 32 || right.len() != 32 {
        return Err(Failure::SecretLength);
    }
    if left != right {
        return Err(Failure::SecretMismatch);
    }
    if left.iter().all(|byte| *byte == 0) {
        return Err(Failure::ZeroSecret);
    }
    Ok(())
}

/// Require a healthy completion while preserving its owned secret until checked.
///
/// # Errors
/// Returns the actual typed rejection as a completion-stage fixture failure.
#[cfg(windows)]
fn require_secret(completion: Completion) -> Result<SharedSecret, Failure> {
    match completion {
        Completion::Secret(secret) => Ok(secret),
        Completion::Rejected(cause) => Err(Failure::Complete(cause)),
    }
}

/// Run two real native exchanges and check a shared-secret agreement property.
///
/// All local exchange/secret owners drop on both successful and error returns.
/// This agreement check is not an independent algorithm-vector qualification.
///
/// # Errors
/// Returns a bounded call, provider or pair-invariant failure.
#[cfg(windows)]
fn healthy_pair(counts: &mut Counts) -> Result<(), Failure> {
    let left = start_exchange(counts)?;
    let right = start_exchange(counts)?;
    let left_public = checked_public(left.pub_key())?;
    let right_public = checked_public(right.pub_key())?;
    if left_public == right_public {
        return Err(Failure::DuplicatePublic);
    }
    let left_secret = require_secret(complete_exchange(left, &right_public, counts)?)?;
    let right_secret = require_secret(complete_exchange(right, &left_public, counts)?)?;
    check_secrets(left_secret.secret_bytes(), right_secret.secret_bytes())
}

/// Require the exact malformed-length rejection, then run a fresh healthy pair.
///
/// Input admission selects the known length branch before starting native work.
/// The failed exchange is consumed/dropped before the neighbor starts.
///
/// # Errors
/// Rejects unintended fixture input, acceptance, a different error or a failed
/// healthy neighbor. No zero-peer behavior is used as the negative oracle.
#[cfg(windows)]
fn reject_then_healthy(peer: &[u8], counts: &mut Counts) -> Result<(), Failure> {
    if !matches!(peer.len(), 31 | 33) || peer.iter().all(|byte| *byte == 0) {
        return Err(Failure::FixtureShape);
    }
    let exchange = start_exchange(counts)?;
    match complete_exchange(exchange, peer, counts)? {
        Completion::Rejected(Rejection::InvalidKeyShare) => healthy_pair(counts),
        Completion::Rejected(cause) => Err(Failure::WrongRejection(cause)),
        Completion::Secret(secret) => {
            drop(secret);
            Err(Failure::MalformedAccepted)
        }
    }
}

/// Execute one Windows case under the documented single shutdown-owner lifetime.
///
/// Native work is synchronous and has no intrinsic timeout. A separately admitted
/// parent must bound and retain the child through actual exit/reaping. The final
/// report is constructed only after all exchange/secret owners and shutdown drop.
#[cfg(windows)]
fn execute_case(case: Case) -> Report {
    let shutdown = ShutdownHandle {};
    let mut counts = Counts::default();
    let outcome = match case {
        Case::Healthy => healthy_pair(&mut counts),
        Case::PeerShort => reject_then_healthy(&SHORT_PEER, &mut counts),
        Case::PeerLong => reject_then_healthy(&LONG_PEER, &mut counts),
    };
    drop(shutdown);
    Report {
        case: Some(case),
        counts,
        outcome,
    }
}

/// Report unsupported execution explicitly without loading a provider.
#[cfg(not(windows))]
fn execute_case(case: Case) -> Report {
    Report {
        case: Some(case),
        counts: Counts::default(),
        outcome: Err(Failure::Unsupported),
    }
}

/// Write one bounded ASCII result containing no key material or raw error text.
///
/// # Errors
/// Returns the actual stdout write failure; an incomplete output is not success.
fn write_report(report: &Report) -> io::Result<()> {
    let (status, stage, cause) = match report.outcome {
        Ok(()) => ("ok", "complete", "none"),
        Err(error) => ("error", error.stage(), error.cause()),
    };
    let scenario = report.case.map_or("invalid", Case::as_str);
    let mut output = io::stdout().lock();
    writeln!(
        output,
        "event=result case={scenario} status={status} stage={stage} cause={cause} start_attempts={} started={} complete_attempts={} completed={} rejected={}",
        report.counts.start_attempts,
        report.counts.started,
        report.counts.complete_attempts,
        report.counts.completed,
        report.counts.rejected
    )?;
    output.flush()
}

/// Admit arguments before provider startup and preserve failure through exit code.
///
/// Returns zero only for a completed case and complete output, two for invalid
/// CLI input, and one for unsupported execution or a case/output failure. A
/// dependency panic/native fault is not caught or converted into a passing case.
fn main() -> ExitCode {
    let (report, invalid_arguments) = match parse_case(std::env::args_os()) {
        Ok(case) => (execute_case(case), false),
        Err(error) => (
            Report {
                case: None,
                counts: Counts::default(),
                outcome: Err(error),
            },
            true,
        ),
    };
    if write_report(&report).is_err() {
        return ExitCode::from(1);
    }
    if invalid_arguments {
        ExitCode::from(2)
    } else if report.outcome.is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

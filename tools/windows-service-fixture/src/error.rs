// SPDX-License-Identifier: GPL-2.0-only
//! Bounded failures retaining original native codes without arbitrary error text.
//!
//! Fixture categories are printable single-line ASCII of at most 64 bytes.
//! Original numeric and typed causes remain separately inspectable.

use std::{error::Error, fmt};

use windows_platform::{
    privilege_observation::{EnginePrivilegeError, PrivilegeCaptureError},
    service::{BudgetError, LifecycleError},
};

/// The only native effect categories used by this inert fixture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeOperation {
    /// Connect this process to SCM exactly once.
    Dispatch,
    /// Register its fixed-role control handler.
    Register,
    /// Attempt one lifecycle-selected status report.
    Status,
    /// Query the current owner's thread token without changing impersonation.
    ThreadToken,
    /// Open the current process primary token with query rights only.
    ProcessToken,
    /// Resolve one of the three fixed privilege identities locally.
    PrivilegeLookup,
    /// Query the bounded primary-token privilege capture.
    PrivilegeQuery,
    /// Attempt explicit release of one owned token handle exactly once.
    TokenClose,
}

/// Original native failure captured before cleanup can overwrite thread error state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeError {
    /// Exact failing operation; no command or caller-selected service name.
    operation: NativeOperation,
    /// Raw native code, kept separately from safe diagnostic formatting.
    code: u32,
}

impl NativeError {
    /// Capture an already-observed failure without converting it into success.
    #[cfg(any(windows, test))]
    pub(crate) const fn new(operation: NativeOperation, code: u32) -> Self {
        Self { operation, code }
    }

    /// Read the failing API category without executing it.
    #[must_use]
    pub const fn operation(self) -> NativeOperation {
        self.operation
    }

    /// Read the original native code, including a surprising zero failure code.
    #[must_use]
    pub const fn code(self) -> u32 {
        self.code
    }
}

impl fmt::Display for NativeError {
    /// Format only the fixed API category; no raw input or untrusted system text.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.operation {
            NativeOperation::Dispatch => "laboratory SCM dispatcher failed",
            NativeOperation::Register => "laboratory SCM registration failed",
            NativeOperation::Status => "laboratory SCM status report failed",
            NativeOperation::ThreadToken => "laboratory thread token query failed",
            NativeOperation::ProcessToken => "laboratory process token query failed",
            NativeOperation::PrivilegeLookup => "laboratory privilege lookup failed",
            NativeOperation::PrivilegeQuery => "laboratory privilege capture failed",
            NativeOperation::TokenClose => "laboratory token handle close failed",
        })
    }
}

impl Error for NativeError {}

/// Laboratory failure, without product policy, privilege or recovery implications.
#[derive(Debug, Eq, PartialEq)]
pub enum FixtureError {
    /// No console/debug imitation of a Windows service is available.
    WindowsOnly,
    /// The process has already attempted dispatcher connection.
    AlreadyDispatched,
    /// Its process-lifetime owner slot has already been initialized.
    OwnerAlreadySet,
    /// More than the optional service-name metadata argument was supplied.
    UnsupportedArguments,
    /// The trusted monotonic observation seam supplied backwards time.
    ClockReversed,
    /// The host could not represent the fixed laboratory run deadline.
    ClockOverflow,
    /// The laboratory maximum run interval elapsed without an admitted Stop.
    RunLimitElapsed,
    /// Source-edited laboratory pending limits failed checked construction.
    Budget(BudgetError),
    /// The serialized portable owner refused an operation or status failed.
    Lifecycle(LifecycleError<NativeError>),
    /// The exact dispatcher or registration API failed.
    Native(NativeError),
    /// Complete token capture/context/cleanup could not establish known facts.
    TokenObservation(TokenObservationFailure),
    /// The fixed engine role observed a forbidden privilege, including disabled.
    PrivilegeAdmission(EnginePrivilegeError),
    /// SCM returned without a successful clean terminal report from this fixture.
    ServiceFailed,
    /// Dispatcher returned before an actual owner-result receipt was available.
    OutcomeUnavailable,
    /// A worker phase failed without acknowledging physical completion.
    Worker(WorkerError),
    /// The original cause accompanied by bounded independent cleanup/report facts.
    Cleanup(ServiceFailure),
}

impl fmt::Display for FixtureError {
    /// Project a bounded category without recursively formatting native payloads.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::WindowsOnly => "laboratory SCM fixture requires Windows",
            Self::AlreadyDispatched => "laboratory dispatcher was already attempted",
            Self::OwnerAlreadySet => "laboratory owner was already initialized",
            Self::UnsupportedArguments => "laboratory SCM start arguments are unsupported",
            Self::ClockReversed => "laboratory monotonic observation reversed",
            Self::ClockOverflow => "laboratory run deadline is unrepresentable",
            Self::RunLimitElapsed => "laboratory maximum run interval elapsed",
            Self::Budget(_) => "laboratory pending budget was rejected",
            Self::Lifecycle(_) => "laboratory service lifecycle failed",
            Self::Native(_) => "laboratory native service operation failed",
            Self::TokenObservation(_) => "laboratory token observation failed",
            Self::PrivilegeAdmission(_) => "laboratory engine privilege admission failed",
            Self::ServiceFailed => "laboratory service did not finish cleanly",
            Self::OutcomeUnavailable => "laboratory service outcome receipt is unavailable",
            Self::Worker(_) => "laboratory service worker failed",
            Self::Cleanup(_) => "laboratory service failed with cleanup facts",
        })
    }
}

impl Error for FixtureError {
    /// Retain typed native/lifecycle/budget causes for explicit inspection.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Budget(error) => Some(error),
            Self::Lifecycle(error) => Some(error),
            Self::Native(error) => Some(error),
            Self::TokenObservation(error) => Some(error),
            Self::PrivilegeAdmission(error) => Some(error),
            Self::Worker(error) => Some(error),
            Self::Cleanup(error) => Some(error),
            _ => None,
        }
    }
}

/// Fixed worker failure categories; these contain no arbitrary diagnostic text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkerError {
    /// Initialization failed; actual completion is still separately observed.
    Preparation,
    /// Operational activation was refused after Running acknowledgement.
    Activation,
    /// Operational health failed while its original work remained charged.
    Running,
    /// Actual completion resolved custody with a failed cleanup outcome.
    Cleanup,
}

impl fmt::Display for WorkerError {
    /// Render a fixed bounded category, independent of worker state or input.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Preparation => "laboratory worker preparation failed",
            Self::Activation => "laboratory worker activation failed",
            Self::Running => "laboratory worker operation failed",
            Self::Cleanup => "laboratory worker cleanup failed",
        })
    }
}

impl Error for WorkerError {}

/// One original cause and at most one result from each permitted cleanup effect.
/// Private construction prevents recursive accumulation of wrapped failures.
#[derive(Debug, Eq, PartialEq)]
pub struct ServiceFailure {
    /// One boxed cause, allocated only when independent secondary facts exist.
    primary: Box<FixtureError>,
    /// The sole completed worker cleanup failure, if later than the primary cause.
    completion: Option<WorkerError>,
    /// The sole `StopPending` failure, if later than the primary cause.
    stop_report: Option<LifecycleError<NativeError>>,
    /// The sole terminal report failure, if later than the primary cause.
    terminal_report: Option<LifecycleError<NativeError>>,
}

impl ServiceFailure {
    /// Wrap exactly once at the finished invocation's result boundary.
    #[cfg(any(windows, test))]
    pub(crate) fn new(
        primary: FixtureError,
        completion: Option<WorkerError>,
        stop_report: Option<LifecycleError<NativeError>>,
        terminal_report: Option<LifecycleError<NativeError>>,
    ) -> Self {
        Self {
            primary: Box::new(primary),
            completion,
            stop_report,
            terminal_report,
        }
    }

    /// Read the earliest cause without formatting its internal typed details.
    #[must_use]
    pub fn primary(&self) -> &FixtureError {
        &self.primary
    }

    /// Read the sole later failed completion observation, if any.
    #[must_use]
    pub const fn completion(&self) -> Option<WorkerError> {
        self.completion
    }

    /// Read the sole later failed `StopPending` operation, if any.
    #[must_use]
    pub const fn stop_report(&self) -> Option<&LifecycleError<NativeError>> {
        self.stop_report.as_ref()
    }

    /// Read the sole later failed terminal operation, if any.
    #[must_use]
    pub const fn terminal_report(&self) -> Option<&LifecycleError<NativeError>> {
        self.terminal_report.as_ref()
    }
}

impl fmt::Display for ServiceFailure {
    /// Project only a bounded category, leaving all causes explicitly inspectable.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("laboratory service retained cleanup failures")
    }
}

impl Error for ServiceFailure {
    /// Preserve the earliest typed failure; secondary facts have separate getters.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.primary.as_ref())
    }
}

/// Bounded first cause of an incomplete or refused token observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenObservationCause {
    /// Original failing native operation and code, captured immediately.
    Native(NativeError),
    /// A successful native capture has an unsupported raw representation.
    Capture(PrivilegeCaptureError),
    /// An unexpected current-thread token was opened; no impersonation is changed.
    UnexpectedThreadToken,
    /// An API reported success without returning an owned non-null handle.
    InvalidHandle,
}

impl fmt::Display for TokenObservationCause {
    /// Project a fixed bounded category without recursively formatting causes.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Native(_) => "laboratory token API failed",
            Self::Capture(_) => "laboratory token capture was rejected",
            Self::UnexpectedThreadToken => "laboratory owner has an unexpected thread token",
            Self::InvalidHandle => "laboratory token API returned an invalid handle",
        })
    }
}
impl Error for TokenObservationCause {
    /// Retain original native or checked-parser facts for typed inspection.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Native(error) => Some(error),
            Self::Capture(error) => Some(error),
            Self::UnexpectedThreadToken | Self::InvalidHandle => None,
        }
    }
}

/// Explicit single-attempt cleanup outcome; failure may leave a live native handle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenCleanupOutcome {
    /// No owned handle was obtained for this slot.
    NotOpened,
    /// The sole explicit close succeeded.
    Closed,
    /// The sole explicit close failed; no retry or resource-release claim follows.
    Failed(NativeError),
}

/// First failure plus both independent cleanup outcomes; no snapshot on failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TokenObservationFailure {
    /// First capture/context failure, or the first close failure if capture succeeded.
    cause: TokenObservationCause,
    /// Actual explicit primary-token cleanup result.
    primary_cleanup: TokenCleanupOutcome,
    /// Actual unexpected thread-token cleanup result.
    thread_cleanup: TokenCleanupOutcome,
}
impl TokenObservationFailure {
    /// Preserve first-cause ordering while retaining cleanup limitations separately.
    #[cfg(any(windows, test))]
    pub(crate) const fn new(
        cause: TokenObservationCause,
        primary_cleanup: TokenCleanupOutcome,
        thread_cleanup: TokenCleanupOutcome,
    ) -> Self {
        Self {
            cause,
            primary_cleanup,
            thread_cleanup,
        }
    }
    /// Read the original first cause without discarding subsequent cleanup facts.
    #[must_use]
    pub const fn cause(self) -> TokenObservationCause {
        self.cause
    }
    /// Read primary-handle cleanup, including possible retention until termination.
    #[must_use]
    pub const fn primary_cleanup(self) -> TokenCleanupOutcome {
        self.primary_cleanup
    }
    /// Read unexpected thread-handle cleanup independently of the first cause.
    #[must_use]
    pub const fn thread_cleanup(self) -> TokenCleanupOutcome {
        self.thread_cleanup
    }
}
impl fmt::Display for TokenObservationFailure {
    /// Keep detailed typed causes separate from safe diagnostic category output.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("laboratory token observation is indeterminate")
    }
}
impl Error for TokenObservationFailure {
    /// Preserve the actual first cause; cleanup outcomes remain explicit getters.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.cause)
    }
}

#[cfg(test)]
#[path = "error_tests.rs"]
mod tests;

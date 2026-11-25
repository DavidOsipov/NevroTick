// SPDX-License-Identifier: GPL-2.0-only
//! Bounded failures retaining original native codes without arbitrary error text.
//!
//! Fixture categories are printable single-line ASCII of at most 64 bytes.
//! Original numeric and typed causes remain separately inspectable.

use std::{error::Error, fmt};

use windows_platform::{
    privilege_observation::{PrivilegeCaptureError, ServicePrivilegeError},
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
    /// Resolve one of the four fixed privilege identities locally.
    PrivilegeLookup,
    /// Query the bounded primary-token privilege capture.
    PrivilegeQuery,
    /// Attempt explicit release of one owned token handle exactly once.
    TokenClose,
    /// Construct the local fixture endpoint's protected descriptor.
    BrokerEndpointDescriptor,
    /// Acquire the fixed first instance under the explicit descriptor.
    BrokerEndpointCreate,
    /// Create the private noninherited completion event before any submission.
    BrokerEndpointAcceptEvent,
    /// Submit the sole connection attempt on the held server instance.
    BrokerEndpointAcceptSubmit,
    /// Request cancellation of the exact outstanding connection attempt.
    BrokerEndpointAcceptCancel,
    /// Observe the exact connection attempt without waiting or releasing custody.
    BrokerEndpointAcceptCompletion,
    /// Prepare the private event for the armed first read.
    BrokerEndpointReadEvent,
    /// Submit the only read on the original accepted fixture pipe.
    BrokerEndpointReadSubmit,
    /// Target cancellation at the original read record.
    BrokerEndpointReadCancel,
    /// Collect native completion of the original read record.
    BrokerEndpointReadCompletion,
    /// Create the fixture's private reply event.
    BrokerEndpointReplyEvent,
    /// Submit one reply on the original accepted connection.
    BrokerEndpointReplySubmit,
    /// Cancel the exact outstanding reply write.
    BrokerEndpointReplyCancel,
    /// Collect the original reply's native result.
    BrokerEndpointReplyCompletion,
    /// Prepare the broker's process-lifetime control signal before registration.
    BrokerWakePrepare,
    /// Signal the broker owner after publishing a control notice.
    BrokerWakeSignal,
    /// Wait on the exact control and optional accept completion signals.
    BrokerWakeWait,
    /// Attempt explicit release of the original endpoint exactly once.
    BrokerEndpointClose,
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
            NativeOperation::BrokerEndpointDescriptor => "laboratory broker descriptor failed",
            NativeOperation::BrokerEndpointCreate => "laboratory broker endpoint creation failed",
            NativeOperation::BrokerEndpointAcceptEvent => "laboratory broker accept event failed",
            NativeOperation::BrokerEndpointAcceptSubmit => {
                "laboratory broker accept submission failed"
            }
            NativeOperation::BrokerEndpointAcceptCancel => {
                "laboratory broker accept cancellation failed"
            }
            NativeOperation::BrokerEndpointAcceptCompletion => {
                "laboratory broker accept completion unconfirmed"
            }
            NativeOperation::BrokerWakePrepare => "laboratory broker wake preparation failed",
            NativeOperation::BrokerWakeSignal => "laboratory broker wake signal failed",
            NativeOperation::BrokerWakeWait => "laboratory broker wake wait failed",
            NativeOperation::BrokerEndpointReadEvent => "laboratory broker read event failed",
            NativeOperation::BrokerEndpointReadSubmit => "laboratory broker read submission failed",
            NativeOperation::BrokerEndpointReadCancel => {
                "laboratory broker read cancellation failed"
            }
            NativeOperation::BrokerEndpointReadCompletion => {
                "laboratory broker read completion failed"
            }
            NativeOperation::BrokerEndpointReplyEvent => "laboratory broker reply event failed",
            NativeOperation::BrokerEndpointReplySubmit => {
                "laboratory broker reply submission failed"
            }
            NativeOperation::BrokerEndpointReplyCancel => {
                "laboratory broker reply cancellation failed"
            }
            NativeOperation::BrokerEndpointReplyCompletion => {
                "laboratory broker reply completion failed"
            }
            NativeOperation::BrokerEndpointClose => "laboratory broker endpoint close failed",
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
    /// The process could not establish the required image-load policy before SCM entry.
    #[cfg(windows)]
    ImageLoadPolicy(windows_platform_native::ImageLoadPolicyError),
    /// The process critical-error mode could not be confirmed before SCM entry.
    #[cfg(windows)]
    ServiceErrorMode(windows_platform_native::ServiceErrorModeError),
    /// The required legacy extension-point restriction was not confirmed.
    #[cfg(windows)]
    ExtensionPointPolicy(windows_platform_native::ExtensionPointPolicyError),
    /// Complete token capture/context/cleanup could not establish known facts.
    TokenObservation(TokenObservationFailure),
    /// A complete observation did not match the fixed service role's exact profile.
    PrivilegeAdmission(ServicePrivilegeError),
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
            #[cfg(windows)]
            Self::ImageLoadPolicy(_) => "laboratory service image-load policy refused",
            #[cfg(windows)]
            Self::ServiceErrorMode(_) => "laboratory service critical-error mode refused",
            #[cfg(windows)]
            Self::ExtensionPointPolicy(_) => "laboratory service extension-point policy refused",
            Self::TokenObservation(_) => "laboratory token observation failed",
            Self::PrivilegeAdmission(_) => "laboratory service privilege admission failed",
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
            #[cfg(windows)]
            Self::ImageLoadPolicy(error) => Some(error),
            #[cfg(windows)]
            Self::ServiceErrorMode(error) => Some(error),
            #[cfg(windows)]
            Self::ExtensionPointPolicy(error) => Some(error),
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
    /// Cleanup failed. Physical completion has its own observation.
    Cleanup,
    /// Descriptor or first-instance preparation failed before readiness.
    EndpointPreparation(NativeError),
    /// Native accept submission or a Running observation failed.
    EndpointAccept(NativeError),
    /// Native submission or Running observation of the armed first read failed.
    EndpointRead(NativeError),
    /// Cancelling or collecting the original read after fencing failed.
    EndpointReadCleanup(NativeError),
    /// Native submission or observation of the armed reply failed.
    EndpointReply(NativeError),
    /// Cancelling or collecting the original reply after fencing failed.
    EndpointReplyCleanup(NativeError),
    /// The empty-DACL reservation observed a terminal result before the Stop fence.
    EndpointUnexpectedCompletion,
    /// Cancelling or collecting the original accept after the Stop fence failed.
    EndpointAcceptCleanup(NativeError),
    /// An explicit close left endpoint release unconfirmed.
    EndpointClose(NativeError),
}

impl fmt::Display for WorkerError {
    /// Render a fixed bounded category, independent of worker state or input.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Preparation => "laboratory worker preparation failed",
            Self::Activation => "laboratory worker activation failed",
            Self::Running => "laboratory worker operation failed",
            Self::Cleanup => "laboratory worker cleanup failed",
            Self::EndpointPreparation(_) => "laboratory broker endpoint preparation failed",
            Self::EndpointAccept(_) => "laboratory broker accept failed",
            Self::EndpointRead(_) => "laboratory broker first read failed",
            Self::EndpointReadCleanup(_) => "laboratory broker first read retirement failed",
            Self::EndpointReply(_) => "laboratory broker reply failed",
            Self::EndpointReplyCleanup(_) => "laboratory broker reply retirement failed",
            Self::EndpointUnexpectedCompletion => "laboratory broker accept completed unexpectedly",
            Self::EndpointAcceptCleanup(_) => "laboratory broker accept retirement failed",
            Self::EndpointClose(_) => "laboratory broker endpoint release unconfirmed",
        })
    }
}

impl Error for WorkerError {
    /// Retain the original typed endpoint cause separately from diagnostics.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::EndpointPreparation(error)
            | Self::EndpointAccept(error)
            | Self::EndpointRead(error)
            | Self::EndpointReadCleanup(error)
            | Self::EndpointReply(error)
            | Self::EndpointReplyCleanup(error)
            | Self::EndpointAcceptCleanup(error)
            | Self::EndpointClose(error) => Some(error),
            Self::Preparation
            | Self::Activation
            | Self::Running
            | Self::Cleanup
            | Self::EndpointUnexpectedCompletion => None,
        }
    }
}

/// One original cause and one fixed slot per later operation/cleanup/report kind.
/// Private construction prevents recursive accumulation of wrapped failures.
#[derive(Debug, Eq, PartialEq)]
pub struct ServiceFailure {
    /// One boxed cause, allocated only when independent secondary facts exist.
    primary: Box<FixtureError>,
    /// Sole distinct later operation failure, independent of cleanup.
    operation: Option<WorkerError>,
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
        operation: Option<WorkerError>,
        completion: Option<WorkerError>,
        stop_report: Option<LifecycleError<NativeError>>,
        terminal_report: Option<LifecycleError<NativeError>>,
    ) -> Self {
        Self {
            primary: Box::new(primary),
            operation,
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

    /// Read the sole later operation failure without changing cleanup's meaning.
    #[must_use]
    pub const fn operation(&self) -> Option<WorkerError> {
        self.operation
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

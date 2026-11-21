// SPDX-License-Identifier: GPL-2.0-only
//! Bounded portable service diagnostics: closed decisions, checked v2 claims,
//! producer-owned sequence and independent required/optional memory routes.
//!
//! Required routing means one bounded attempt, never durable or native delivery.
//! Records preserve the original lifecycle observation; no API reads a clock,
//! performs I/O, accepts caller text or supplies service/clock authority. Live
//! instance, generation, correlation and UTC issuers are explicitly unavailable.
//! Passive decoding grants no producer provenance and cannot enqueue records.
//!
//! A legal external consumer reaches the real router and bounded memory sink:
//! ```
//! use windows_platform::diagnostics::{Component, DiagnosticObservation,
//!     DiagnosticSnapshot, ImportanceMask, MemoryRoutes, OptionalProfile, Router, ServiceEvent,
//!     WriterRecord};
//! let mut router: Router = Router::new(Component::Engine, OptionalProfile::Off,
//!     ImportanceMask::all(), MemoryRoutes::available());
//! router.emit(ServiceEvent::StartObserved, DiagnosticObservation::unavailable());
//! let writer_record: Option<WriterRecord> = router.emit_for_writer(
//!     ServiceEvent::RunningAcknowledged, DiagnosticObservation::unavailable());
//! assert!(writer_record.is_some());
//! let snapshot: DiagnosticSnapshot = router.finish();
//! assert_eq!(snapshot.required().history().len(), 2);
//! assert_eq!(snapshot.optional().history().len(), 0);
//! ```
//!
//! Passive bytes cannot enter the producer path:
//! ```compile_fail,E0308
//! use windows_platform::diagnostics::{DecodedRecord, DiagnosticObservation, Router};
//! fn inject(router: &mut Router, passive: DecodedRecord) {
//!     router.emit(passive, DiagnosticObservation::unavailable());
//! }
//! ```
//!
//! A decoded component claim cannot select the role of a new producer router:
//! ```compile_fail,E0308
//! use windows_platform::diagnostics::{DecodedRecord, ImportanceMask, MemoryRoutes,
//!     OptionalProfile, Router};
//! fn relabel(decoded: DecodedRecord) -> Router {
//!     Router::new(decoded.component(), OptionalProfile::Off,
//!         ImportanceMask::all(), MemoryRoutes::available())
//! }
//! ```
//!
//! The writer handoff has only live construction and cannot be cloned:
//! ```compile_fail,E0451
//! use windows_platform::diagnostics::{Component, WriterRecord};
//! fn relabel(original: WriterRecord) -> WriterRecord {
//!     WriterRecord { component: Component::Engine, ..original }
//! }
//! ```
//! ```compile_fail,E0277
//! use windows_platform::diagnostics::WriterRecord;
//! fn replay(original: &WriterRecord) -> WriterRecord {
//!     Clone::clone(original)
//! }
//! ```
//!
//! Passive component claims have no conversion into active composition roles:
//! ```compile_fail,E0277
//! use windows_platform::diagnostics::{Component, ComponentClaim};
//! fn convert(claim: ComponentClaim) -> Component { claim.into() }
//! ```
//!
//! Passive component claims have no borrowed active-role adapter:
//! ```compile_fail,E0277
//! use windows_platform::diagnostics::{Component, ComponentClaim};
//! fn borrow(claim: &ComponentClaim) -> &Component {
//!     <ComponentClaim as AsRef<Component>>::as_ref(claim)
//! }
//! ```
//!
//! Passive component claims cannot dereference-coerce to active composition roles:
//! ```compile_fail,E0308
//! use windows_platform::diagnostics::{Component, ComponentClaim};
//! fn dereference(claim: &ComponentClaim) -> &Component { claim }
//! ```
//!
//! Passive component claims have no cross-active equality:
//! ```compile_fail,E0277
//! use windows_platform::diagnostics::{Component, ComponentClaim};
//! fn compare(claim: ComponentClaim, active: Component) -> bool {
//!     <ComponentClaim as PartialEq<Component>>::eq(&claim, &active)
//! }
//! ```
//!
//! A decoded event claim cannot be forwarded as an active producer decision:
//! ```compile_fail,E0308
//! use windows_platform::diagnostics::{DecodedRecord, DiagnosticObservation, Router};
//! fn forward(router: &mut Router, decoded: DecodedRecord) {
//!     router.emit(decoded.event(), DiagnosticObservation::unavailable());
//! }
//! ```
//!
//! The same public profile permits reading a claim and independently constructing
//! an actual closed decision at its producer owner:
//! ```
//! use windows_platform::diagnostics::{Component, ComponentClaim, DecodeError, DecodedRecord,
//!     DiagnosticObservation, DiagnosticSnapshot, ImportanceMask, MemoryRoutes,
//!     OptionalProfile, Router, ServiceEvent, ServiceEventClaim};
//! # fn main() -> Result<(), DecodeError> {
//! let mut bytes: [u8; 128] = [0; 128];
//! bytes[..9].copy_from_slice(&[2, 1, 1, 1, 3, 5, 1, 0, 1]);
//! let decoded: DecodedRecord = DecodedRecord::decode(&bytes)?;
//! let component: ComponentClaim = decoded.component();
//! assert_eq!(component.code(), 1);
//! let claim: ServiceEventClaim = decoded.event();
//! assert_eq!(claim.id(), 1);
//! assert_eq!(claim.cause_code(), 0);
//! let mut router: Router = Router::new(Component::Engine, OptionalProfile::Off,
//!     ImportanceMask::all(), MemoryRoutes::available());
//! router.emit(ServiceEvent::StartObserved, DiagnosticObservation::unavailable());
//! let snapshot: DiagnosticSnapshot = router.finish();
//! assert_eq!(snapshot.required().history().len(), 1);
//! # Ok(()) }
//! ```
//!
//! Passive claims have no implicit conversion into producer events:
//! ```compile_fail,E0277
//! use windows_platform::diagnostics::{ServiceEvent, ServiceEventClaim};
//! fn convert(claim: ServiceEventClaim) -> ServiceEvent { claim.into() }
//! ```
//!
//! Passive claims do not provide a borrowed active-event adapter:
//! ```compile_fail,E0277
//! use windows_platform::diagnostics::{ServiceEvent, ServiceEventClaim};
//! fn borrow(claim: &ServiceEventClaim) -> &ServiceEvent {
//!     <ServiceEventClaim as AsRef<ServiceEvent>>::as_ref(claim)
//! }
//! ```
//!
//! Borrowing a passive claim cannot dereference-coerce into an active event:
//! ```compile_fail,E0308
//! use windows_platform::diagnostics::{ServiceEvent, ServiceEventClaim};
//! fn dereference(claim: &ServiceEventClaim) -> &ServiceEvent { claim }
//! ```
//!
//! Passive claims cannot compare directly with active producer events:
//! ```compile_fail,E0277
//! use windows_platform::diagnostics::{ServiceEvent, ServiceEventClaim};
//! fn compare(claim: ServiceEventClaim, active: ServiceEvent) -> bool {
//!     <ServiceEventClaim as PartialEq<ServiceEvent>>::eq(&claim, &active)
//! }
//! ```
//!
//! Raw bytes cannot construct a trusted lifecycle observation:
//! ```compile_fail,E0451
//! use windows_platform::diagnostics::DiagnosticObservation;
//! let forged = DiagnosticObservation { ..DiagnosticObservation::unavailable() };
//! ```
//!
//! Passive records have no public unchecked construction:
//! ```compile_fail,E0451
//! use windows_platform::diagnostics::DecodedRecord;
//! fn forge(record: DecodedRecord) {
//!     let forged = DecodedRecord { bytes: [0; 128],
//!         component: record.component(), event: record.event() };
//! }
//! ```
//!
//! Raw causes cannot bypass the mandatory fallible domain conversion:
//! ```compile_fail,E0277
//! use windows_platform::diagnostics::PrimaryCause;
//! let cause: PrimaryCause = 9u8.into();
//! ```
//!
//! Checked passive record bytes remain immutable:
//! ```compile_fail,E0594
//! use windows_platform::diagnostics::DecodedRecord;
//! fn change(record: DecodedRecord) { record.bytes()[0] = 1; }
//! ```
//!
//! No public enqueue method accepts a passive decoded record:
//! ```compile_fail,E0599
//! use windows_platform::diagnostics::{DecodedRecord, Router};
//! fn enqueue(router: &mut Router, record: DecodedRecord) { router.enqueue(record); }
//! ```
#![forbid(unsafe_code)]

use std::fmt;
use std::time::{Duration, Instant};

use garde::Validate;

/// Exact coordinated private v2 representation; historical v1 remains distinct.
pub const RECORD_BYTES: usize = 128;
/// Fixed pending and recent capacity for each independent portable route.
pub const ROUTE_CAPACITY: usize = 16;
/// Number of fixed catalogue keys per route, independent of causes or metadata.
const EVENT_COUNT: usize = 9;
/// Checked elapsed nanoseconds required for one finite-window refill.
const WINDOW_NS: u64 = 1_000_000_000;

/// Fixed role claim selected by executable composition, not authenticated identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Component {
    /// Network engine laboratory service role.
    Engine,
    /// Clock broker laboratory service role; grants no clock access.
    ClockBroker,
}

impl Component {
    /// Return the fixed wire role code without relying on Rust enum layout.
    #[must_use]
    pub const fn code(self) -> u8 {
        match self {
            Self::Engine => 1,
            Self::ClockBroker => 2,
        }
    }
}

/// Passive component claim decoded from bytes, never a producer role selection.
/// Actual executable composition independently selects its fixed [`Component`].
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct ComponentClaim {
    /// Private structurally checked claim; no active-role accessor is exposed.
    inner: Component,
}

impl ComponentClaim {
    /// Read only the claimed wire role code, without selecting a producer role.
    #[must_use]
    pub const fn code(self) -> u8 {
        self.inner.code()
    }
}

impl fmt::Debug for ComponentClaim {
    /// Format only bounded passive claim vocabulary, never an active role token.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ComponentClaim")
            .field("code", &self.code())
            .finish()
    }
}

/// Technical detail level, independent of operational importance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TechnicalLevel {
    /// Error technical detail.
    Error,
    /// Warning technical detail.
    Warn,
    /// Information technical detail.
    Info,
    /// Debug detail; no stage-one session admits it.
    Debug,
    /// Trace detail; no stage-one session admits it.
    Trace,
}

impl TechnicalLevel {
    /// Read the stable v2 code, without a layout cast.
    #[must_use]
    pub const fn code(self) -> u8 {
        match self {
            Self::Error => 1,
            Self::Warn => 2,
            Self::Info => 3,
            Self::Debug => 4,
            Self::Trace => 5,
        }
    }
}

/// Code-owned operational importance, not notification or correction authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Importance {
    /// Synthetic unsafe ownership/restoration threat; no live producer here.
    Critical,
    /// Actual whole required attempt/invocation failure.
    High,
    /// Synthetic sustained availability failure; no live producer here.
    Medium,
    /// Synthetic independently recoverable source problem; no live producer here.
    Low,
    /// Expected acknowledged lifecycle decision.
    Info,
}

impl Importance {
    /// Read the stable importance code.
    #[must_use]
    pub const fn code(self) -> u8 {
        match self {
            Self::Critical => 1,
            Self::High => 2,
            Self::Medium => 3,
            Self::Low => 4,
            Self::Info => 5,
        }
    }
}

/// Finite optional duplicate selection; required admission ignores this profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OptionalProfile {
    /// Collect no optional duplicates.
    Off,
    /// Collect error details only.
    Error,
    /// Collect error and warning details.
    Warn,
    /// Collect error, warning and information details.
    Info,
}

/// Checked five-bit optional importance selection; empty is legal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImportanceMask {
    /// Only bits zero through four can exist after construction.
    bits: u8,
}

impl ImportanceMask {
    /// Admit only the five importance bits, rejecting all unknown bits.
    ///
    /// # Errors
    /// Returns a closed mask error when bits five through seven are set.
    pub const fn new(bits: u8) -> Result<Self, DecodeError> {
        if bits & 0xe0 == 0 {
            Ok(Self { bits })
        } else {
            Err(DecodeError::InvalidImportanceMask)
        }
    }

    /// Select all five importance bands without admitting unknown bits.
    #[must_use]
    pub const fn all() -> Self {
        Self { bits: 0x1f }
    }

    /// Read the checked mask.
    #[must_use]
    pub const fn bits(self) -> u8 {
        self.bits
    }

    /// Test one typed band; this does not select required routing.
    #[must_use]
    pub const fn includes(self, importance: Importance) -> bool {
        self.bits & (1u8 << (importance.code() - 1)) != 0
    }
}

/// Pure optional policy, also usable for explicitly synthetic classification tests.
/// Live events can only use their sole catalogue descriptor.
#[must_use]
pub const fn optional_enabled(
    profile: OptionalProfile,
    mask: ImportanceMask,
    level: TechnicalLevel,
    importance: Importance,
) -> bool {
    let technical: bool = match profile {
        OptionalProfile::Off => false,
        OptionalProfile::Error => matches!(level, TechnicalLevel::Error),
        OptionalProfile::Warn => matches!(level, TechnicalLevel::Error | TechnicalLevel::Warn),
        OptionalProfile::Info => matches!(
            level,
            TechnicalLevel::Error | TechnicalLevel::Warn | TechnicalLevel::Info
        ),
    };
    technical && mask.includes(importance)
}

/// Closed permitted primary cause codes; raw conversion uses mandatory domain admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimaryCause {
    /// Stable permitted cause code 1.
    WindowsOnly,
    /// Stable permitted cause code 2.
    AlreadyDispatched,
    /// Stable permitted cause code 3.
    OwnerAlreadySet,
    /// Stable permitted cause code 4.
    UnsupportedArguments,
    /// Stable permitted cause code 5.
    ClockReversed,
    /// Stable permitted cause code 6.
    ClockOverflow,
    /// Stable permitted cause code 7.
    RunLimitElapsed,
    /// Stable permitted cause code 8.
    Budget,
    /// Stable permitted cause code 9.
    Native,
    /// Stable permitted cause code 10.
    LifecycleReport,
    /// Stable permitted cause code 11.
    LifecycleDeadlineElapsed,
    /// Stable permitted cause code 12.
    LifecycleInvalidTransition,
    /// Stable permitted cause code 13.
    LifecycleReportingFailed,
    /// Stable permitted cause code 14.
    LifecycleTimeReversed,
    /// Stable permitted cause code 15.
    LifecycleDeadlineOverflow,
    /// Stable permitted cause code 16.
    LifecycleCheckpointOverflow,
    /// Stable permitted cause code 17.
    LifecycleOutstandingWork,
    /// Stable permitted cause code 18.
    LifecycleWorkLimit,
    /// Stable permitted cause code 19.
    LifecycleFailureOutcomeRequired,
    /// Stable permitted cause code 20.
    WorkerPreparation,
    /// Stable permitted cause code 21.
    WorkerActivation,
    /// Stable permitted cause code 22.
    WorkerRunning,
    /// Stable permitted cause code 23.
    WorkerCleanup,
    /// Stable permitted cause code 24.
    TokenObservation,
    /// Stable permitted cause code 25.
    PrivilegeAdmission,
    /// Stable permitted cause code 26.
    ServiceFailed,
    /// Stable permitted cause code 27.
    OutcomeUnavailable,
    /// Stable permitted cause code 28.
    NestedCleanupNotTraversed,
}

impl PrimaryCause {
    /// Return the stable allowlisted cause code.
    #[must_use]
    pub const fn code(self) -> u8 {
        match self {
            Self::WindowsOnly => 1,
            Self::AlreadyDispatched => 2,
            Self::OwnerAlreadySet => 3,
            Self::UnsupportedArguments => 4,
            Self::ClockReversed => 5,
            Self::ClockOverflow => 6,
            Self::RunLimitElapsed => 7,
            Self::Budget => 8,
            Self::Native => 9,
            Self::LifecycleReport => 10,
            Self::LifecycleDeadlineElapsed => 11,
            Self::LifecycleInvalidTransition => 12,
            Self::LifecycleReportingFailed => 13,
            Self::LifecycleTimeReversed => 14,
            Self::LifecycleDeadlineOverflow => 15,
            Self::LifecycleCheckpointOverflow => 16,
            Self::LifecycleOutstandingWork => 17,
            Self::LifecycleWorkLimit => 18,
            Self::LifecycleFailureOutcomeRequired => 19,
            Self::WorkerPreparation => 20,
            Self::WorkerActivation => 21,
            Self::WorkerRunning => 22,
            Self::WorkerCleanup => 23,
            Self::TokenObservation => 24,
            Self::PrivilegeAdmission => 25,
            Self::ServiceFailed => 26,
            Self::OutcomeUnavailable => 27,
            Self::NestedCleanupNotTraversed => 28,
        }
    }

    /// Convert an already domain-checked code; absence remains explicit.
    fn from_checked_code(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::WindowsOnly),
            2 => Some(Self::AlreadyDispatched),
            3 => Some(Self::OwnerAlreadySet),
            4 => Some(Self::UnsupportedArguments),
            5 => Some(Self::ClockReversed),
            6 => Some(Self::ClockOverflow),
            7 => Some(Self::RunLimitElapsed),
            8 => Some(Self::Budget),
            9 => Some(Self::Native),
            10 => Some(Self::LifecycleReport),
            11 => Some(Self::LifecycleDeadlineElapsed),
            12 => Some(Self::LifecycleInvalidTransition),
            13 => Some(Self::LifecycleReportingFailed),
            14 => Some(Self::LifecycleTimeReversed),
            15 => Some(Self::LifecycleDeadlineOverflow),
            16 => Some(Self::LifecycleCheckpointOverflow),
            17 => Some(Self::LifecycleOutstandingWork),
            18 => Some(Self::LifecycleWorkLimit),
            19 => Some(Self::LifecycleFailureOutcomeRequired),
            20 => Some(Self::WorkerPreparation),
            21 => Some(Self::WorkerActivation),
            22 => Some(Self::WorkerRunning),
            23 => Some(Self::WorkerCleanup),
            24 => Some(Self::TokenObservation),
            25 => Some(Self::PrivilegeAdmission),
            26 => Some(Self::ServiceFailed),
            27 => Some(Self::OutcomeUnavailable),
            28 => Some(Self::NestedCleanupNotTraversed),
            _ => None,
        }
    }
}

impl TryFrom<u8> for PrimaryCause {
    type Error = DecodeError;

    /// Admit one raw cause through the same obligatory root rule as v2 decoding.
    fn try_from(code: u8) -> Result<Self, Self::Error> {
        let bytes: [u8; RECORD_BYTES] = raw_record(
            Component::Engine,
            4,
            code,
            1,
            DiagnosticObservation::unavailable(),
        );
        let _candidate: Candidate = Candidate::checked(bytes)?;
        Self::from_checked_code(code).ok_or(DecodeError::InvalidV2)
    }
}

/// Closed permitted lifecycle cause codes; raw conversion uses mandatory domain admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleCause {
    /// Stable permitted cause code 10.
    Report,
    /// Stable permitted cause code 11.
    DeadlineElapsed,
    /// Stable permitted cause code 12.
    InvalidTransition,
    /// Stable permitted cause code 13.
    ReportingFailed,
    /// Stable permitted cause code 14.
    TimeReversed,
    /// Stable permitted cause code 15.
    DeadlineOverflow,
    /// Stable permitted cause code 16.
    CheckpointOverflow,
    /// Stable permitted cause code 17.
    OutstandingWork,
    /// Stable permitted cause code 18.
    WorkLimit,
    /// Stable permitted cause code 19.
    FailureOutcomeRequired,
}

impl LifecycleCause {
    /// Return the stable allowlisted cause code.
    #[must_use]
    pub const fn code(self) -> u8 {
        match self {
            Self::Report => 10,
            Self::DeadlineElapsed => 11,
            Self::InvalidTransition => 12,
            Self::ReportingFailed => 13,
            Self::TimeReversed => 14,
            Self::DeadlineOverflow => 15,
            Self::CheckpointOverflow => 16,
            Self::OutstandingWork => 17,
            Self::WorkLimit => 18,
            Self::FailureOutcomeRequired => 19,
        }
    }

    /// Convert an already domain-checked code; absence remains explicit.
    fn from_checked_code(code: u8) -> Option<Self> {
        match code {
            10 => Some(Self::Report),
            11 => Some(Self::DeadlineElapsed),
            12 => Some(Self::InvalidTransition),
            13 => Some(Self::ReportingFailed),
            14 => Some(Self::TimeReversed),
            15 => Some(Self::DeadlineOverflow),
            16 => Some(Self::CheckpointOverflow),
            17 => Some(Self::OutstandingWork),
            18 => Some(Self::WorkLimit),
            19 => Some(Self::FailureOutcomeRequired),
            _ => None,
        }
    }
}

impl TryFrom<u8> for LifecycleCause {
    type Error = DecodeError;

    /// Admit one raw cause through the same obligatory root rule as v2 decoding.
    fn try_from(code: u8) -> Result<Self, Self::Error> {
        let bytes: [u8; RECORD_BYTES] = raw_record(
            Component::Engine,
            6,
            code,
            1,
            DiagnosticObservation::unavailable(),
        );
        let _candidate: Candidate = Candidate::checked(bytes)?;
        Self::from_checked_code(code).ok_or(DecodeError::InvalidV2)
    }
}

/// Closed permitted worker cause codes; raw conversion uses mandatory domain admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkerCause {
    /// Stable permitted cause code 20.
    Preparation,
    /// Stable permitted cause code 21.
    Activation,
    /// Stable permitted cause code 22.
    Running,
    /// Stable permitted cause code 23.
    Cleanup,
}

impl WorkerCause {
    /// Return the stable allowlisted cause code.
    #[must_use]
    pub const fn code(self) -> u8 {
        match self {
            Self::Preparation => 20,
            Self::Activation => 21,
            Self::Running => 22,
            Self::Cleanup => 23,
        }
    }

    /// Convert an already domain-checked code; absence remains explicit.
    fn from_checked_code(code: u8) -> Option<Self> {
        match code {
            20 => Some(Self::Preparation),
            21 => Some(Self::Activation),
            22 => Some(Self::Running),
            23 => Some(Self::Cleanup),
            _ => None,
        }
    }
}

impl TryFrom<u8> for WorkerCause {
    type Error = DecodeError;

    /// Admit one raw cause through the same obligatory root rule as v2 decoding.
    fn try_from(code: u8) -> Result<Self, Self::Error> {
        let bytes: [u8; RECORD_BYTES] = raw_record(
            Component::Engine,
            5,
            code,
            1,
            DiagnosticObservation::unavailable(),
        );
        let _candidate: Candidate = Candidate::checked(bytes)?;
        Self::from_checked_code(code).ok_or(DecodeError::InvalidV2)
    }
}

/// The nine actual service decision events; causes are event-specific closed types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceEvent {
    /// Invocation exists after actual registration and budget construction.
    StartObserved,
    /// Actual ready transition and Running acknowledgement succeeded.
    RunningAcknowledged,
    /// Serialized owner fence closed new work admission.
    AdmissionClosed,
    /// Earliest retained required attempt/invocation failure.
    PrimaryFailure(PrimaryCause),
    /// Separate actual completed-cleanup failure.
    CompletionFailure(WorkerCause),
    /// Actual `StopPending` lifecycle reporting failure.
    StopReportFailure(LifecycleCause),
    /// Actual terminal lifecycle reporting failure.
    TerminalReportFailure(LifecycleCause),
    /// Physical custody resolved and terminal reporting attempted, not clean success.
    Finished,
    /// Original whole invocation operation failure.
    OperationFailure(WorkerCause),
}

impl ServiceEvent {
    /// Return the catalogue ID; never infer it from Rust representation.
    #[must_use]
    pub const fn id(self) -> u8 {
        match self {
            Self::StartObserved => 1,
            Self::RunningAcknowledged => 2,
            Self::AdmissionClosed => 3,
            Self::PrimaryFailure(_) => 4,
            Self::CompletionFailure(_) => 5,
            Self::StopReportFailure(_) => 6,
            Self::TerminalReportFailure(_) => 7,
            Self::Finished => 8,
            Self::OperationFailure(_) => 9,
        }
    }

    /// Select the fixed catalogue/budget slot without conversions or external keys.
    const fn index(self) -> usize {
        match self {
            Self::StartObserved => 0,
            Self::RunningAcknowledged => 1,
            Self::AdmissionClosed => 2,
            Self::PrimaryFailure(_) => 3,
            Self::CompletionFailure(_) => 4,
            Self::StopReportFailure(_) => 5,
            Self::TerminalReportFailure(_) => 6,
            Self::Finished => 7,
            Self::OperationFailure(_) => 8,
        }
    }

    /// Return only the event-specific permitted cause, or canonical no-cause zero.
    #[must_use]
    pub const fn cause_code(self) -> u8 {
        match self {
            Self::StartObserved
            | Self::RunningAcknowledged
            | Self::AdmissionClosed
            | Self::Finished => 0,
            Self::PrimaryFailure(cause) => cause.code(),
            Self::CompletionFailure(cause) | Self::OperationFailure(cause) => cause.code(),
            Self::StopReportFailure(cause) | Self::TerminalReportFailure(cause) => cause.code(),
        }
    }

    /// Read this event's sole code-owned classification and permitted cause range.
    #[must_use]
    pub const fn descriptor(self) -> &'static EventDescriptor {
        &CATALOGUE[self.index()]
    }
}

/// Passive event claim from checked bytes, distinct from an actual producer decision.
/// Read-only catalogue observations grant no conversion into [`ServiceEvent`].
/// The executable decision owner must independently construct its actual event.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct ServiceEventClaim {
    /// Private closed representation; no active-event accessor or trait is exposed.
    inner: ServiceEvent,
}

impl ServiceEventClaim {
    /// Read only the claimed stable event ID, without creating a producer event.
    #[must_use]
    pub const fn id(self) -> u8 {
        self.inner.id()
    }

    /// Read the claimed allowlisted cause code, without reauthoring its decision.
    #[must_use]
    pub const fn cause_code(self) -> u8 {
        self.inner.cause_code()
    }

    /// Read the code-owned descriptor for this claim, never a producer token.
    #[must_use]
    pub const fn descriptor(self) -> &'static EventDescriptor {
        self.inner.descriptor()
    }
}

impl fmt::Debug for ServiceEventClaim {
    /// Format bounded passive vocabulary, without exposing its active representation.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ServiceEventClaim")
            .field("id", &self.id())
            .field("cause_code", &self.cause_code())
            .finish()
    }
}

/// Stable catalogue definition; callers cannot construct descriptors or change routing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EventDescriptor {
    /// Stable event ID, with one fixed array slot per event.
    id: u8,
    /// Code-owned technical level.
    level: TechnicalLevel,
    /// Independent operational importance.
    importance: Importance,
    /// First permitted cause code, inclusive.
    cause_min: u8,
    /// Last permitted cause code, inclusive.
    cause_max: u8,
}

impl EventDescriptor {
    /// Read this descriptor's stable ID.
    #[must_use]
    pub const fn id(self) -> u8 {
        self.id
    }
    /// Read the coordinated representation version.
    #[must_use]
    pub const fn version(self) -> u8 {
        2
    }
    /// Read the code-owned technical level.
    #[must_use]
    pub const fn level(self) -> TechnicalLevel {
        self.level
    }
    /// Read the code-owned operational importance.
    #[must_use]
    pub const fn importance(self) -> Importance {
        self.importance
    }
    /// Every live catalogue event attempts the required route.
    #[must_use]
    pub const fn required(self) -> bool {
        true
    }
    /// Every live catalogue event can also be selected as an optional duplicate.
    #[must_use]
    pub const fn optional_duplicate_eligible(self) -> bool {
        true
    }
    /// Test a cause against the only semantic catalogue owner.
    #[must_use]
    pub const fn permits_cause(self, cause: u8) -> bool {
        cause >= self.cause_min && cause <= self.cause_max
    }
}

/// Sole shared semantic catalogue, never caller-provided or translated data.
pub const CATALOGUE: [EventDescriptor; EVENT_COUNT] = [
    EventDescriptor {
        id: 1,
        level: TechnicalLevel::Info,
        importance: Importance::Info,
        cause_min: 0,
        cause_max: 0,
    },
    EventDescriptor {
        id: 2,
        level: TechnicalLevel::Info,
        importance: Importance::Info,
        cause_min: 0,
        cause_max: 0,
    },
    EventDescriptor {
        id: 3,
        level: TechnicalLevel::Info,
        importance: Importance::Info,
        cause_min: 0,
        cause_max: 0,
    },
    EventDescriptor {
        id: 4,
        level: TechnicalLevel::Error,
        importance: Importance::High,
        cause_min: 1,
        cause_max: 28,
    },
    EventDescriptor {
        id: 5,
        level: TechnicalLevel::Error,
        importance: Importance::High,
        cause_min: 20,
        cause_max: 23,
    },
    EventDescriptor {
        id: 6,
        level: TechnicalLevel::Error,
        importance: Importance::High,
        cause_min: 10,
        cause_max: 19,
    },
    EventDescriptor {
        id: 7,
        level: TechnicalLevel::Error,
        importance: Importance::High,
        cause_min: 10,
        cause_max: 19,
    },
    EventDescriptor {
        id: 8,
        level: TechnicalLevel::Info,
        importance: Importance::Info,
        cause_min: 0,
        cause_max: 0,
    },
    EventDescriptor {
        id: 9,
        level: TechnicalLevel::Error,
        importance: Importance::High,
        cause_min: 20,
        cause_max: 23,
    },
];

/// Private producer reason, intentionally stronger than the passive wire time tag.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ObservationReason {
    /// No lifecycle read occurred; finite initial budgets remain useful.
    NotObserved,
    /// Checked relative coordinate from the existing lifecycle owner.
    Accepted,
    /// Duration cannot fit the fixed nanosecond representation.
    CheckedElapsedOverflow,
    /// Last accepted lifecycle instant precedes the supplied original origin.
    OriginOrderFault,
    /// Existing lifecycle owner detected a clock fault after this accepted coordinate.
    ActualClockFault,
}

/// Producer-side observation from the existing lifecycle read path only.
/// No passive decoder or raw metadata constructor can create this token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiagnosticObservation {
    /// Preserves fault semantics even when wire time must be unavailable.
    reason: ObservationReason,
    /// Checked original relative coordinate, zero when unavailable.
    elapsed_ns: u64,
}

impl DiagnosticObservation {
    /// Express no actual lifecycle observation, without fabricating observed zero.
    #[must_use]
    pub const fn unavailable() -> Self {
        Self {
            reason: ObservationReason::NotObserved,
            elapsed_ns: 0,
        }
    }

    /// Project the existing owner's original observation without an ambient read.
    /// Ordering and conversion refusals precede the actual-fault historical claim.
    #[must_use]
    pub fn from_lifecycle(origin: Instant, last_accepted: Instant, faulted: bool) -> Self {
        let Some(elapsed): Option<Duration> = last_accepted.checked_duration_since(origin) else {
            return Self {
                reason: ObservationReason::OriginOrderFault,
                elapsed_ns: 0,
            };
        };
        let Ok(elapsed_ns): Result<u64, std::num::TryFromIntError> =
            u64::try_from(elapsed.as_nanos())
        else {
            return Self {
                reason: ObservationReason::CheckedElapsedOverflow,
                elapsed_ns: 0,
            };
        };
        Self {
            reason: if faulted {
                ObservationReason::ActualClockFault
            } else {
                ObservationReason::Accepted
            },
            elapsed_ns,
        }
    }

    /// Encode only the passive observation claim; routing uses the private reason.
    const fn wire_kind(self) -> u8 {
        match self.reason {
            ObservationReason::NotObserved
            | ObservationReason::CheckedElapsedOverflow
            | ObservationReason::OriginOrderFault => 0,
            ObservationReason::Accepted => 1,
            ObservationReason::ActualClockFault => 2,
        }
    }
}

/// Fixed redacted decode refusal; it retains neither input bytes nor garde reports.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecodeError {
    /// No version byte exists.
    Empty,
    /// Unknown version; checked before version-specific length.
    UnsupportedVersion(u8),
    /// V2 requires exactly 128 bytes; no partial or extra input is consumed.
    InvalidLength {
        /// Actual bounded-slice length, without retaining contents.
        actual: usize,
    },
    /// One root mandatory semantic rule rejected the candidate.
    InvalidV2,
    /// An optional mask contains unknown bits.
    InvalidImportanceMask,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message: &str = match self {
            Self::Empty => "empty diagnostic record",
            Self::UnsupportedVersion(_) => "unsupported diagnostic version",
            Self::InvalidLength { .. } => "invalid diagnostic record length",
            Self::InvalidV2 => "invalid-v2",
            Self::InvalidImportanceMask => "invalid importance mask",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for DecodeError {}

/// One fixed inert candidate with exactly one mandatory root garde rule.
struct Candidate {
    /// Full raw fixed representation; never exported before shared validation.
    bytes: [u8; RECORD_BYTES],
}

impl Candidate {
    /// Validate at most one root error and drop its bounded report before returning.
    ///
    /// # Errors
    /// Reject any representation that violates the complete v2 record contract.
    fn checked(bytes: [u8; RECORD_BYTES]) -> Result<Self, DecodeError> {
        let candidate: Self = Self { bytes };
        if let Err(report) = candidate.validate() {
            drop(report);
            return Err(DecodeError::InvalidV2);
        }
        Ok(candidate)
    }
}

impl Validate for Candidate {
    type Context = ();

    /// Exactly one whole-candidate rule; no field traversal or error accumulation.
    fn validate_into(
        &self,
        _context: &(),
        _parent: &mut dyn FnMut() -> garde::Path,
        report: &mut garde::Report,
    ) {
        if !semantic_valid(&self.bytes) {
            report.append(garde::Path::empty(), garde::Error::new("invalid-v2"));
        }
    }
}

/// Read an exact fixed-width little-endian integer from a known in-bounds offset.
fn read_u64(bytes: &[u8; RECORD_BYTES], offset: usize) -> u64 {
    let mut word: [u8; 8] = [0; 8];
    word.copy_from_slice(&bytes[offset..offset + 8]);
    u64::from_le_bytes(word)
}

/// Read an exact 128-bit metadata claim from a fixed in-bounds offset.
fn read_u128(bytes: &[u8; RECORD_BYTES], offset: usize) -> u128 {
    let mut word: [u8; 16] = [0; 16];
    word.copy_from_slice(&bytes[offset..offset + 16]);
    u128::from_le_bytes(word)
}

/// Read exact UTC nanoseconds from its fixed in-bounds offset.
fn read_nanos(bytes: &[u8; RECORD_BYTES]) -> u32 {
    let mut word: [u8; 4] = [0; 4];
    word.copy_from_slice(&bytes[104..108]);
    u32::from_le_bytes(word)
}

/// Sole scalar/cross-field semantic rule, invoked only by the mandatory root rule.
fn semantic_valid(bytes: &[u8; RECORD_BYTES]) -> bool {
    let Some(index): Option<u8> = bytes[3].checked_sub(1) else {
        return false;
    };
    let Some(descriptor): Option<&EventDescriptor> = CATALOGUE.get(usize::from(index)) else {
        return false;
    };
    if bytes[0] != 2
        || bytes[1] != 1
        || !matches!(bytes[2], 1 | 2)
        || bytes[4] != descriptor.level.code()
        || bytes[5] != descriptor.importance.code()
        || bytes[6] != 1
        || !descriptor.permits_cause(bytes[7])
        || read_u64(bytes, 8) == 0
        || !matches!(bytes[16], 0..=2)
        || (bytes[16] == 0 && read_u64(bytes, 17) != 0)
        || bytes[108..].iter().any(|byte: &u8| *byte != 0)
    {
        return false;
    }
    for (tag, offset) in [(25usize, 26usize), (78, 79)] {
        let value: u128 = read_u128(bytes, offset);
        if !matches!((bytes[tag], value), (0, 0) | (1, 1..)) {
            return false;
        }
    }
    for (tag, offset) in [(42usize, 43usize), (51, 52), (60, 61), (69, 70)] {
        let value: u64 = read_u64(bytes, offset);
        if !matches!((bytes[tag], value), (0, 0) | (1, 1..)) {
            return false;
        }
    }
    let nanos: u32 = read_nanos(bytes);
    match bytes[95] {
        0 => read_u64(bytes, 96) == 0 && nanos == 0,
        1 => nanos < 1_000_000_000,
        _ => false,
    }
}

/// Prepare a fixed v2 representation; the mandatory constructor checks its output.
/// The caller supplies only a typed role/observation, known ID/cause and sequence.
fn raw_record(
    component: Component,
    id: u8,
    cause: u8,
    sequence: u64,
    observation: DiagnosticObservation,
) -> [u8; RECORD_BYTES] {
    let mut bytes: [u8; RECORD_BYTES] = [0; RECORD_BYTES];
    bytes[0] = 2;
    bytes[1] = 1;
    bytes[2] = component.code();
    bytes[3] = id;
    let descriptor: Option<&EventDescriptor> = id
        .checked_sub(1)
        .and_then(|index: u8| CATALOGUE.get(usize::from(index)));
    if let Some(descriptor) = descriptor {
        bytes[4] = descriptor.level.code();
        bytes[5] = descriptor.importance.code();
    }
    bytes[6] = 1;
    bytes[7] = cause;
    bytes[8..16].copy_from_slice(&sequence.to_le_bytes());
    bytes[16] = observation.wire_kind();
    bytes[17..25].copy_from_slice(&observation.elapsed_ns.to_le_bytes());
    bytes
}

/// Passive lifecycle-coordinate claim, not a producer observation or freshness proof.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleObservationClaim {
    /// No representable original lifecycle coordinate.
    Unavailable,
    /// Claim about a last accepted coordinate relative to its original origin.
    LastAccepted(u64),
    /// Historical accepted coordinate retained after an actual fault claim.
    HistoricalAfterClockFault(u64),
}

impl LifecycleObservationClaim {
    /// Project the fixed observation fields from a checked record or its producer.
    const fn from_wire(kind: u8, elapsed_ns: u64) -> Self {
        match kind {
            1 => Self::LastAccepted(elapsed_ns),
            2 => Self::HistoricalAfterClockFault(elapsed_ns),
            _ => Self::Unavailable,
        }
    }
}

/// Original event fields available to the writer after required queue admission.
///
/// Only the live router constructs this handoff. It does not acknowledge
/// memory consumption, native delivery or durable retention.
#[must_use]
pub struct WriterRecord {
    /// Fixed role of the emitting router.
    component: Component,
    /// Original invocation-local sequence.
    sequence: u64,
    /// Original checked lifecycle observation.
    observation: LifecycleObservationClaim,
    /// Eligible catalogue ID.
    event_id: u8,
    /// Cause permitted by the eligible event's catalogue entry.
    cause_code: u8,
}

impl WriterRecord {
    /// Return the router's original component.
    #[must_use]
    pub const fn component(&self) -> Component {
        self.component
    }

    /// Return the original local sequence.
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Return the original lifecycle observation.
    #[must_use]
    pub const fn observation(&self) -> LifecycleObservationClaim {
        self.observation
    }

    /// Return the eligible event's fixed catalogue ID.
    #[must_use]
    pub const fn event_id(&self) -> u8 {
        self.event_id
    }

    /// Return the original permitted cause or canonical no-cause zero.
    #[must_use]
    pub const fn cause_code(&self) -> u8 {
        self.cause_code
    }
}

/// Passive nonzero metadata claim, structurally valid without issuer authenticity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MetadataClaim<T> {
    /// No issuer supplied this metadata; payload zero is canonical wire padding.
    Unavailable,
    /// Nonzero untrusted metadata claim from decoded bytes.
    ObservedClaim(T),
}

/// Passive UTC quality; wall-clock claims are never qualified time authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UtcClaim {
    /// No UTC issuer or clock read occurred.
    Unavailable,
    /// Floor Unix seconds and nonnegative nanoseconds, including pre-epoch time.
    SystemWallUnqualified {
        /// Exact signed Unix floor seconds.
        unix_seconds: i64,
        /// Nonnegative subsecond nanoseconds less than one billion.
        nanoseconds: u32,
    },
}

/// Immutable, checked passive v2 record; cannot construct live queue entries.
#[derive(Clone, Eq, PartialEq)]
pub struct DecodedRecord {
    /// Validated complete representation; no mutable access is exposed.
    bytes: [u8; RECORD_BYTES],
    /// Typed passive role claim, not an active composition role or identity.
    component: ComponentClaim,
    /// Typed passive event claim using the shared catalogue, not a producer event.
    event: ServiceEventClaim,
}

impl fmt::Debug for DecodedRecord {
    /// Project only closed passive claims; omit raw bytes, opaque identities and UTC.
    /// Exhaustive field handling requires review before any future field is exposed.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            // The complete representation includes opaque untrusted metadata, so
            // formatting deliberately excludes every raw or opaque byte field.
            bytes: _,
            component,
            event,
        }: &Self = self;
        formatter
            .debug_struct("DecodedRecord")
            .field("provenance", &"untrusted passive claims")
            .field("component", component)
            .field("event", event)
            .field("local_sequence", &self.sequence())
            .finish_non_exhaustive()
    }
}

impl DecodedRecord {
    /// Bound/version-check, then obligatorily validate one fixed inert candidate.
    ///
    /// # Errors
    /// Returns exact empty/version/length refusal or fixed semantic rejection.
    pub fn decode(input: &[u8]) -> Result<Self, DecodeError> {
        let Some(version): Option<&u8> = input.first() else {
            return Err(DecodeError::Empty);
        };
        if *version != 2 {
            return Err(DecodeError::UnsupportedVersion(*version));
        }
        let Ok(bytes): Result<[u8; RECORD_BYTES], std::array::TryFromSliceError> =
            <[u8; RECORD_BYTES]>::try_from(input)
        else {
            return Err(DecodeError::InvalidLength {
                actual: input.len(),
            });
        };
        let candidate: Candidate = Candidate::checked(bytes)?;
        let component: ComponentClaim = ComponentClaim {
            inner: match candidate.bytes[2] {
                1 => Component::Engine,
                2 => Component::ClockBroker,
                _ => return Err(DecodeError::InvalidV2),
            },
        };
        let event: ServiceEventClaim = ServiceEventClaim {
            inner: event_from_checked(&candidate.bytes).ok_or(DecodeError::InvalidV2)?,
        };
        Ok(Self {
            bytes: candidate.bytes,
            component,
            event,
        })
    }

    /// Borrow only the immutable passive representation.
    #[must_use]
    pub const fn bytes(&self) -> &[u8; RECORD_BYTES] {
        &self.bytes
    }
    /// Read a passive component claim that cannot select a producer router role.
    #[must_use]
    pub const fn component(&self) -> ComponentClaim {
        self.component
    }
    /// Read a passive event claim that cannot be forwarded as a producer decision.
    #[must_use]
    pub const fn event(&self) -> ServiceEventClaim {
        self.event
    }
    /// Read the invocation-local sequence, without cross-instance ordering.
    #[must_use]
    pub fn sequence(&self) -> u64 {
        read_u64(&self.bytes, 8)
    }
    /// Read the original lifecycle-coordinate claim, never consumption time.
    #[must_use]
    pub fn observation(&self) -> LifecycleObservationClaim {
        LifecycleObservationClaim::from_wire(self.bytes[16], read_u64(&self.bytes, 17))
    }
    /// Read the passive process instance claim, not authenticated process identity.
    #[must_use]
    pub fn process_instance(&self) -> MetadataClaim<u128> {
        self.claim128(25, 26)
    }
    /// Read the passive policy generation claim, not current policy permission.
    #[must_use]
    pub fn policy_generation(&self) -> MetadataClaim<u64> {
        self.claim64(42, 43)
    }
    /// Read the passive configuration generation claim.
    #[must_use]
    pub fn configuration_generation(&self) -> MetadataClaim<u64> {
        self.claim64(51, 52)
    }
    /// Read the passive association generation claim.
    #[must_use]
    pub fn association_generation(&self) -> MetadataClaim<u64> {
        self.claim64(60, 61)
    }
    /// Read the passive clock generation claim, not clock authority.
    #[must_use]
    pub fn clock_generation(&self) -> MetadataClaim<u64> {
        self.claim64(69, 70)
    }
    /// Read the passive correlation claim, without cross-record authenticity.
    #[must_use]
    pub fn correlation(&self) -> MetadataClaim<u128> {
        self.claim128(78, 79)
    }
    /// Read unqualified signed UTC claims; live stage-one emitters provide none.
    #[must_use]
    pub fn utc(&self) -> UtcClaim {
        if self.bytes[95] == 0 {
            UtcClaim::Unavailable
        } else {
            UtcClaim::SystemWallUnqualified {
                unix_seconds: i64::from_le_bytes(read_u64(&self.bytes, 96).to_le_bytes()),
                nanoseconds: read_nanos(&self.bytes),
            }
        }
    }
    /// Project an already checked 64-bit unavailable/nonzero metadata pair.
    fn claim64(&self, tag: usize, offset: usize) -> MetadataClaim<u64> {
        if self.bytes[tag] == 0 {
            MetadataClaim::Unavailable
        } else {
            MetadataClaim::ObservedClaim(read_u64(&self.bytes, offset))
        }
    }
    /// Project an already checked 128-bit unavailable/nonzero metadata pair.
    fn claim128(&self, tag: usize, offset: usize) -> MetadataClaim<u128> {
        if self.bytes[tag] == 0 {
            MetadataClaim::Unavailable
        } else {
            MetadataClaim::ObservedClaim(read_u128(&self.bytes, offset))
        }
    }
}

/// Convert scalar codes only after mandatory candidate acceptance.
fn event_from_checked(bytes: &[u8; RECORD_BYTES]) -> Option<ServiceEvent> {
    let cause: u8 = bytes[7];
    match bytes[3] {
        1 => Some(ServiceEvent::StartObserved),
        2 => Some(ServiceEvent::RunningAcknowledged),
        3 => Some(ServiceEvent::AdmissionClosed),
        4 => PrimaryCause::from_checked_code(cause).map(ServiceEvent::PrimaryFailure),
        5 => WorkerCause::from_checked_code(cause).map(ServiceEvent::CompletionFailure),
        6 => LifecycleCause::from_checked_code(cause).map(ServiceEvent::StopReportFailure),
        7 => LifecycleCause::from_checked_code(cause).map(ServiceEvent::TerminalReportFailure),
        8 => Some(ServiceEvent::Finished),
        9 => WorkerCause::from_checked_code(cause).map(ServiceEvent::OperationFailure),
        _ => None,
    }
}
/// Saturating diagnostic count; saturation never wraps or recursively emits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Counter {
    /// Exact accumulated count until overflow would occur.
    value: u64,
    /// At least one increment exceeded representable count.
    saturated: bool,
}

impl Counter {
    /// Initial fixed counter state, not an observation or issuer default.
    const ZERO: Self = Self {
        value: 0,
        saturated: false,
    };
    /// Read the retained bounded count.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.value
    }
    /// Read explicit information loss from arithmetic saturation.
    #[must_use]
    pub const fn saturated(self) -> bool {
        self.saturated
    }
    /// Charge once, preserving max and explicitly recording any overflow.
    fn increment(&mut self) {
        if let Some(next) = self.value.checked_add(1) {
            self.value = next;
        } else {
            self.saturated = true;
        }
    }
}

/// Per-ID coalescing facts; causes do not create additional rate keys.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EventHealth {
    /// First attempted bounded cause, including genuine no-cause zero.
    first_cause: Option<u8>,
    /// Some later attempt used a different closed cause.
    cause_variation: bool,
    /// Attempts refused for lack of per-ID or aggregate repeat credits.
    suppressed: Counter,
}

impl EventHealth {
    /// Empty fixed key state, independent of external labels.
    const EMPTY: Self = Self {
        first_cause: None,
        cause_variation: false,
        suppressed: Counter::ZERO,
    };
    /// Read the first bounded attempted cause without rewriting an accepted record.
    #[must_use]
    pub const fn first_cause(self) -> Option<u8> {
        self.first_cause
    }
    /// Read whether distinct permitted causes occurred for this same key.
    #[must_use]
    pub const fn cause_variation(self) -> bool {
        self.cause_variation
    }
    /// Read this key's coalesced attempt count.
    #[must_use]
    pub const fn suppressed(self) -> Counter {
        self.suppressed
    }
    /// Retain the first closed cause and variation before finite route admission.
    fn observe(&mut self, cause: u8) {
        match self.first_cause {
            None => self.first_cause = Some(cause),
            Some(first) => self.cause_variation |= first != cause,
        }
    }
}

/// Concrete finite consumer behavior selected by portable executable composition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConsumerState {
    /// Accept originals into the recent ring, explicitly overwriting oldest when full.
    Available,
    /// Refuse consumption as full without modifying retained records.
    Full,
    /// Refuse a reached original distinctly from fullness or absence.
    Reject,
    /// No consumer delivery is available.
    Unavailable,
}

/// Queue availability is independent of reached concrete consumer behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueState {
    /// Admit until fixed pending capacity is exhausted.
    Available,
    /// Refuse before consumer invocation, without refunding a spent rate credit.
    Unavailable,
}

/// Original accepted producer representation; no public conversion admits passive bytes.
#[derive(Clone, Copy)]
struct ProducedRecord {
    /// Fixed checked bytes frozen at emission, not consumer time.
    bytes: [u8; RECORD_BYTES],
}

/// Live queue custody can only originate in the producer's checked emit path.
#[derive(Clone, Copy)]
struct QueueEntry {
    /// Original produced record; no exposed enqueue/decoded conversion exists.
    record: ProducedRecord,
}

/// Immutable fixed recent history in chronological acceptance order.
#[derive(Clone)]
pub struct RecentHistory {
    /// Fixed owned ring slots; absent slots cannot masquerade as decoded records.
    records: [Option<ProducedRecord>; ROUTE_CAPACITY],
    /// Number of populated chronological records, never above capacity.
    len: usize,
}

impl RecentHistory {
    /// Allocate no heap and hold no record before actual consumption.
    const fn empty() -> Self {
        Self {
            records: [None; ROUTE_CAPACITY],
            len: 0,
        }
    }
    /// Read actual retained records, not queue admissions.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }
    /// Test whether any record was actually consumed and retained.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// Borrow fixed originals in chronological order; no mutable view is exposed.
    pub fn records(&self) -> impl ExactSizeIterator<Item = &[u8; RECORD_BYTES]> {
        RecentRecords {
            history: self,
            next: 0,
        }
    }
    /// Borrow a retained original by chronological ordinal.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&[u8; RECORD_BYTES]> {
        if index >= self.len {
            return None;
        }
        self.records
            .get(index)
            .and_then(Option::as_ref)
            .map(|record: &ProducedRecord| &record.bytes)
    }
    /// Retain the original, returning whether one oldest record was overwritten.
    fn retain(&mut self, record: ProducedRecord) -> bool {
        if self.len < ROUTE_CAPACITY {
            self.records[self.len] = Some(record);
            self.len += 1;
            false
        } else {
            self.records.copy_within(1.., 0);
            self.records[ROUTE_CAPACITY - 1] = Some(record);
            true
        }
    }
}

/// Read-only exact-size ordinal view over the private fixed history invariant.
struct RecentRecords<'a> {
    /// Original immutable ring, not a detached newly timed reconstruction.
    history: &'a RecentHistory,
    /// Next chronological ordinal, bounded by the immutable retained length.
    next: usize,
}

impl<'a> Iterator for RecentRecords<'a> {
    type Item = &'a [u8; RECORD_BYTES];
    fn next(&mut self) -> Option<Self::Item> {
        let record: Option<&'a [u8; RECORD_BYTES]> = self.history.get(self.next);
        if record.is_some() {
            self.next += 1;
        }
        record
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining: usize = self.history.len - self.next;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for RecentRecords<'_> {}

/// Concrete bounded consumer, with no callback, lock, I/O or retry surface.
pub struct MemoryConsumer {
    /// Fixed portable fault mode, not a user production knob.
    state: ConsumerState,
    /// Only actual accepted originals enter this fixed recent ring.
    recent: RecentHistory,
}

impl MemoryConsumer {
    /// Select one finite consumer state before connecting the actual invocation.
    #[must_use]
    pub const fn new(state: ConsumerState) -> Self {
        Self {
            state,
            recent: RecentHistory::empty(),
        }
    }
    /// Consume exactly one original queue entry and never invoke external work.
    fn consume_one(&mut self, entry: QueueEntry) -> ConsumerOutcome {
        match self.state {
            ConsumerState::Available => ConsumerOutcome::Accepted {
                overwritten: self.recent.retain(entry.record),
            },
            ConsumerState::Full => ConsumerOutcome::Full,
            ConsumerState::Reject => ConsumerOutcome::Reject,
            ConsumerState::Unavailable => ConsumerOutcome::Unavailable,
        }
    }
}

/// Closed actual consumer result, separate from queue admission.
enum ConsumerOutcome {
    /// Original retained; a full recent ring explicitly overwrote its oldest record.
    Accepted {
        /// Actual ring replacement observation.
        overwritten: bool,
    },
    /// Concrete consumer refused as full.
    Full,
    /// Concrete consumer reached and rejected the original.
    Reject,
    /// Concrete consumer was unavailable.
    Unavailable,
}

/// Two independent memory routes selected by executable composition.
pub struct MemoryRoutes {
    /// Required concrete consumer, never consumed by an optional flood.
    required: MemoryConsumer,
    /// Independent optional concrete consumer.
    optional: MemoryConsumer,
    /// Required queue admission state, not inferred from consumer state.
    required_queue: QueueState,
    /// Optional queue admission state.
    optional_queue: QueueState,
}

impl MemoryRoutes {
    /// Connect finite consumers without running them or granting authority.
    #[must_use]
    pub const fn new(required: MemoryConsumer, optional: MemoryConsumer) -> Self {
        Self {
            required,
            optional,
            required_queue: QueueState::Available,
            optional_queue: QueueState::Available,
        }
    }
    /// Select two available bounded consumers and their available queues.
    #[must_use]
    pub const fn available() -> Self {
        Self::new(
            MemoryConsumer::new(ConsumerState::Available),
            MemoryConsumer::new(ConsumerState::Available),
        )
    }
    /// Select independent queue absence controls before router composition.
    #[must_use]
    pub const fn with_queue_states(mut self, required: QueueState, optional: QueueState) -> Self {
        self.required_queue = required;
        self.optional_queue = optional;
        self
    }
}

/// Per-route accounting, preserving queue, consumption and history loss separately.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouteHealth {
    /// Actual successful queue admissions.
    queued: Counter,
    /// Spent-credit attempts refused by a full pending queue.
    queue_full: Counter,
    /// Spent-credit attempts refused by an unavailable queue.
    queue_unavailable: Counter,
    /// Optional attempts excluded before spending optional credits.
    filtered: Counter,
    /// Attempts coalesced without a route credit.
    suppressed: Counter,
    /// Original dequeued records presented to the concrete consumer.
    consumer_attempts: Counter,
    /// Actual concrete consumer acceptances.
    accepted: Counter,
    /// Reached consumer refusals because of its Full state.
    consumer_full: Counter,
    /// Reached distinct consumer rejections.
    consumer_rejected: Counter,
    /// Reached consumer-unavailable outcomes.
    consumer_unavailable: Counter,
    /// Actual oldest recent-record replacements.
    history_overwritten: Counter,
    /// One fixed coalescing entry per catalogue ID.
    events: [EventHealth; EVENT_COUNT],
}

impl RouteHealth {
    /// Fixed empty counters and keys before any route attempt.
    const EMPTY: Self = Self {
        queued: Counter::ZERO,
        queue_full: Counter::ZERO,
        queue_unavailable: Counter::ZERO,
        filtered: Counter::ZERO,
        suppressed: Counter::ZERO,
        consumer_attempts: Counter::ZERO,
        accepted: Counter::ZERO,
        consumer_full: Counter::ZERO,
        consumer_rejected: Counter::ZERO,
        consumer_unavailable: Counter::ZERO,
        history_overwritten: Counter::ZERO,
        events: [EventHealth::EMPTY; EVENT_COUNT],
    };
    /// Actual successful queue admissions.
    #[must_use]
    pub const fn queued(&self) -> Counter {
        self.queued
    }
    /// Spent-credit attempts refused by a full pending queue.
    #[must_use]
    pub const fn queue_full(&self) -> Counter {
        self.queue_full
    }
    /// Spent-credit attempts refused by an unavailable queue.
    #[must_use]
    pub const fn queue_unavailable(&self) -> Counter {
        self.queue_unavailable
    }
    /// Optional attempts excluded before spending optional credits.
    #[must_use]
    pub const fn filtered(&self) -> Counter {
        self.filtered
    }
    /// Attempts coalesced without a route credit.
    #[must_use]
    pub const fn suppressed(&self) -> Counter {
        self.suppressed
    }
    /// Original dequeued records presented to the concrete consumer.
    #[must_use]
    pub const fn consumer_attempts(&self) -> Counter {
        self.consumer_attempts
    }
    /// Actual concrete consumer acceptances.
    #[must_use]
    pub const fn accepted(&self) -> Counter {
        self.accepted
    }
    /// Reached consumer refusals because of its Full state.
    #[must_use]
    pub const fn consumer_full(&self) -> Counter {
        self.consumer_full
    }
    /// Reached distinct consumer rejections.
    #[must_use]
    pub const fn consumer_rejected(&self) -> Counter {
        self.consumer_rejected
    }
    /// Reached consumer-unavailable outcomes.
    #[must_use]
    pub const fn consumer_unavailable(&self) -> Counter {
        self.consumer_unavailable
    }
    /// Actual oldest recent-record replacements.
    #[must_use]
    pub const fn history_overwritten(&self) -> Counter {
        self.history_overwritten
    }
    /// Read coalescing facts for a known stable ID; unknown IDs create no key.
    #[must_use]
    pub fn per_event(&self, event_id: u8) -> Option<&EventHealth> {
        event_id
            .checked_sub(1)
            .and_then(|index: u8| self.events.get(usize::from(index)))
    }
    /// Detect any fixed-route arithmetic information loss.
    fn saturated(&self) -> bool {
        [
            self.queued,
            self.queue_full,
            self.queue_unavailable,
            self.filtered,
            self.suppressed,
            self.consumer_attempts,
            self.accepted,
            self.consumer_full,
            self.consumer_rejected,
            self.consumer_unavailable,
            self.history_overwritten,
        ]
        .iter()
        .any(|counter: &Counter| counter.saturated)
            || self
                .events
                .iter()
                .any(|event: &EventHealth| event.suppressed.saturated)
    }
}

/// Nine reserved first-ID credits and 23 shared repeat credits per finite window.
struct RouteBudget {
    /// Per-ID charges, bounded to four and indexed only by typed event.
    used: [u8; EVENT_COUNT],
    /// Repeats may not consume the nine first-ID reservations.
    repeat_remaining: u8,
}

impl RouteBudget {
    /// Start finite useful budgets even before any lifecycle observation.
    const fn initial() -> Self {
        Self {
            used: [0; EVENT_COUNT],
            repeat_remaining: 23,
        }
    }
    /// Charge once before queue attempt; failure/refusal cannot refund or retry.
    fn charge(&mut self, index: usize) -> bool {
        let used: &mut u8 = &mut self.used[index];
        if *used == 0 {
            *used = 1;
            true
        } else if *used < 4 && self.repeat_remaining != 0 {
            *used += 1;
            self.repeat_remaining -= 1;
            true
        } else {
            false
        }
    }
}

/// Actual route owner with independent queue, consumer, rate and loss state.
struct Route {
    /// Queue admission control, distinct from concrete consumer results.
    queue_state: QueueState,
    /// Fixed FIFO slots; a full queue never overwrites pending originals.
    queue: [Option<QueueEntry>; ROUTE_CAPACITY],
    /// Current pending originals, bounded to capacity.
    pending: usize,
    /// Real bounded consumer reached by pumping the same original entry.
    consumer: MemoryConsumer,
    /// Independent finite route rate budget.
    budget: RouteBudget,
    /// Saturating route health and fixed per-ID coalescing facts.
    health: RouteHealth,
}

impl Route {
    /// Connect an already selected consumer and finite empty queue.
    const fn new(consumer: MemoryConsumer, queue_state: QueueState) -> Self {
        Self {
            queue_state,
            queue: [None; ROUTE_CAPACITY],
            pending: 0,
            consumer,
            budget: RouteBudget::initial(),
            health: RouteHealth::EMPTY,
        }
    }
    /// Attempt one original using this route's independent pre-queue credit.
    /// Return whether the original entered the queue.
    fn attempt(&mut self, event: ServiceEvent, record: ProducedRecord) -> bool {
        let index: usize = event.index();
        self.health.events[index].observe(event.cause_code());
        if !self.budget.charge(index) {
            self.health.suppressed.increment();
            self.health.events[index].suppressed.increment();
            return false;
        }
        if self.queue_state == QueueState::Unavailable {
            self.health.queue_unavailable.increment();
        } else if self.pending == ROUTE_CAPACITY {
            self.health.queue_full.increment();
        } else {
            self.queue[self.pending] = Some(QueueEntry { record });
            self.pending += 1;
            self.health.queued.increment();
            return true;
        }
        false
    }
    /// At most one FIFO dequeue and one concrete consumption; no retry or callback.
    fn pump_one(&mut self) {
        if self.pending == 0 {
            return;
        }
        let entry: Option<QueueEntry> = self.queue[0].take();
        self.queue.copy_within(1.., 0);
        self.pending -= 1;
        self.queue[self.pending] = None;
        if let Some(entry) = entry {
            self.health.consumer_attempts.increment();
            match self.consumer.consume_one(entry) {
                ConsumerOutcome::Accepted { overwritten } => {
                    self.health.accepted.increment();
                    if overwritten {
                        self.health.history_overwritten.increment();
                    }
                }
                ConsumerOutcome::Full => self.health.consumer_full.increment(),
                ConsumerOutcome::Reject => self.health.consumer_rejected.increment(),
                ConsumerOutcome::Unavailable => self.health.consumer_unavailable.increment(),
            }
        }
    }
    /// Copy only fixed admitted recent/health state, preserving pending separately.
    fn snapshot(&self) -> RouteSnapshot {
        RouteSnapshot {
            history: self.consumer.recent.clone(),
            health: self.health,
            pending: self.pending,
        }
    }
}

/// Router-wide diagnostic losses and closed internal rate-clock fault reasons.
/// Fault counters count processed reason observations, including repeated cached
/// facts; they do not count independently established physical incidents.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouterHealth {
    /// Original semantically constructed events, regardless of delivery outcome.
    emitted: Counter,
    /// Mandatory post-transform domain construction refusals.
    construction_loss: Counter,
    /// Attempts after the last legal invocation-local sequence.
    sequence_loss: Counter,
    /// Producer attempts after final sealing, which cannot reopen this router.
    sealed_loss: Counter,
    /// Once true, no later coordinate can restore refill authority.
    rate_clock_fault: bool,
    /// Original elapsed duration failed checked u64-nanosecond conversion.
    elapsed_overflow: Counter,
    /// Original last accepted coordinate preceded its origin.
    origin_order_fault: Counter,
    /// Existing lifecycle owner reported an actual clock fault.
    actual_clock_fault: Counter,
}

impl RouterHealth {
    /// Initial health, without fabricated issuer or observed coordinate defaults.
    const EMPTY: Self = Self {
        emitted: Counter::ZERO,
        construction_loss: Counter::ZERO,
        sequence_loss: Counter::ZERO,
        sealed_loss: Counter::ZERO,
        rate_clock_fault: false,
        elapsed_overflow: Counter::ZERO,
        origin_order_fault: Counter::ZERO,
        actual_clock_fault: Counter::ZERO,
    };
    /// Read original checked emission count, not queue or delivery success.
    #[must_use]
    pub const fn emitted(&self) -> Counter {
        self.emitted
    }
    /// Read mandatory construction refusals without arbitrary report text.
    #[must_use]
    pub const fn construction_loss(&self) -> Counter {
        self.construction_loss
    }
    /// Read attempts refused after sequence exhaustion.
    #[must_use]
    pub const fn sequence_loss(&self) -> Counter {
        self.sequence_loss
    }
    /// Read attempts refused after production was sealed.
    #[must_use]
    pub const fn sealed_loss(&self) -> Counter {
        self.sealed_loss
    }
    /// Read permanent rate-clock fault; original unspent credits remain usable.
    #[must_use]
    pub const fn rate_clock_fault(&self) -> bool {
        self.rate_clock_fault
    }
    /// Read overflow reason observations even when wire time was unavailable.
    #[must_use]
    pub const fn elapsed_overflow(&self) -> Counter {
        self.elapsed_overflow
    }
    /// Read ordering fault reason observations even when wire time was unavailable.
    #[must_use]
    pub const fn origin_order_fault(&self) -> Counter {
        self.origin_order_fault
    }
    /// Read actual lifecycle clock-fault observations.
    #[must_use]
    pub const fn actual_clock_fault(&self) -> Counter {
        self.actual_clock_fault
    }
    /// Detect overall count uncertainty or outright diagnostic production loss.
    fn incomplete(&self) -> bool {
        self.emitted.saturated
            || self.construction_loss.value != 0
            || self.sequence_loss.value != 0
            || self.sealed_loss.value != 0
            || self.elapsed_overflow.saturated
            || self.origin_order_fault.saturated
            || self.actual_clock_fault.saturated
    }
}

/// Immutable route history and separate actual queue/consumer/loss observations.
#[derive(Clone)]
pub struct RouteSnapshot {
    /// Only originals actually accepted by the consumer, in chronological order.
    history: RecentHistory,
    /// Full route accounting; queue admission is not consumer acceptance.
    health: RouteHealth,
    /// Originals still queued when this view was taken.
    pending: usize,
}

impl RouteSnapshot {
    /// Borrow fixed actually consumed originals.
    #[must_use]
    pub const fn history(&self) -> &RecentHistory {
        &self.history
    }
    /// Borrow immutable route health.
    #[must_use]
    pub const fn health(&self) -> &RouteHealth {
        &self.health
    }
    /// Read pending originals that are not yet accepted recent history.
    #[must_use]
    pub const fn pending(&self) -> usize {
        self.pending
    }
    /// Whether this route's selected memory history has no known loss or pending work.
    /// Optional exclusion and native downstream remain separately qualified facts.
    #[must_use]
    pub fn history_complete(&self) -> bool {
        self.pending == 0
            && !self.health.saturated()
            && self.health.queue_full.value == 0
            && self.health.queue_unavailable.value == 0
            && self.health.suppressed.value == 0
            && self.health.consumer_full.value == 0
            && self.health.consumer_rejected.value == 0
            && self.health.consumer_unavailable.value == 0
            && self.health.history_overwritten.value == 0
    }
}

/// Immutable fixed snapshot; exposes no mutation, producer token or authority.
#[must_use = "diagnostic history and loss must remain associated with the actual host outcome"]
pub struct DiagnosticSnapshot {
    /// Required route's actual bounded history and independent health.
    required: RouteSnapshot,
    /// Optional duplicate route's actual bounded history and independent health.
    optional: RouteSnapshot,
    /// Producer-wide losses and rate-clock fault reasons.
    health: RouterHealth,
    /// Whether finalization has sealed production and bounded final pumping.
    sealed: bool,
}

impl DiagnosticSnapshot {
    /// Borrow required bounded history; native/durable delivery is not established.
    #[must_use]
    pub const fn required(&self) -> &RouteSnapshot {
        &self.required
    }
    /// Borrow independent optional duplicate history.
    #[must_use]
    pub const fn optional(&self) -> &RouteSnapshot {
        &self.optional
    }
    /// Borrow original producer health, separate from either route's consumer facts.
    #[must_use]
    pub const fn health(&self) -> &RouterHealth {
        &self.health
    }
    /// Observe whether production was sealed before this view.
    #[must_use]
    pub const fn sealed(&self) -> bool {
        self.sealed
    }
    /// Whether production and both selected memory routes are free of known gaps.
    /// Optional records deliberately excluded by policy are outside this history.
    /// This grants no native/durable delivery or cross-invocation history claim.
    #[must_use]
    pub fn history_complete(&self) -> bool {
        !self.health.incomplete()
            && self.required.history_complete()
            && self.optional.history_complete()
    }
    /// This portable consumer has no native delivery acknowledgement.
    #[must_use]
    pub const fn native_delivery_qualified(&self) -> bool {
        false
    }
}

/// Producer-bound role, sequence, finite routes and sticky rate-clock ownership.
/// No method accepts raw bytes, decoded records or positive metadata suppliers.
pub struct Router {
    /// Immutable role selected by composition, not a caller-per-event label.
    component: Component,
    /// Finite optional technical profile, separate from required routing.
    optional: OptionalProfile,
    /// Checked optional importance mask.
    importance: ImportanceMask,
    /// Next original local sequence; None permanently represents exhaustion.
    next_sequence: Option<u64>,
    /// First valid last-accepted coordinate or most recent actual refill anchor.
    window_anchor: Option<u64>,
    /// Last processed accepted coordinate, enforcing nondecreasing observations.
    last_coordinate: Option<u64>,
    /// Permanent production seal after finalization.
    sealed: bool,
    /// Independent required route storage, rate and consumer.
    required: Route,
    /// Independent optional duplicate route.
    duplicate: Route,
    /// Producer-wide fixed health and reason counters.
    health: RouterHealth,
}

impl Router {
    /// Bind the fixed role and finite route controls before any fallible host work.
    /// Both initial budgets exist even while lifecycle time is unavailable.
    #[must_use]
    pub const fn new(
        component: Component,
        optional: OptionalProfile,
        importance: ImportanceMask,
        sinks: MemoryRoutes,
    ) -> Self {
        Self {
            component,
            optional,
            importance,
            next_sequence: Some(1),
            window_anchor: None,
            last_coordinate: None,
            sealed: false,
            required: Route::new(sinks.required, sinks.required_queue),
            duplicate: Route::new(sinks.optional, sinks.optional_queue),
            health: RouterHealth::EMPTY,
        }
    }

    /// Freeze the original checked event/context and attempt required then optional routing.
    /// All failures affect only fixed diagnostics health, never the operation result.
    pub fn emit(&mut self, event: ServiceEvent, observation: DiagnosticObservation) {
        let _sequence: Option<u64> = self.emit_to_routes(event, observation);
    }

    /// Emit once and hand off an eligible original admitted by the required queue.
    ///
    /// Running acknowledgement and primary failure are eligible. Optional routing
    /// and subsequent memory consumption do not affect the writer handoff.
    #[must_use]
    pub fn emit_for_writer(
        &mut self,
        event: ServiceEvent,
        observation: DiagnosticObservation,
    ) -> Option<WriterRecord> {
        let sequence: u64 = self.emit_to_routes(event, observation)?;
        if !matches!(
            event,
            ServiceEvent::RunningAcknowledged | ServiceEvent::PrimaryFailure(_)
        ) {
            return None;
        }
        Some(WriterRecord {
            component: self.component,
            sequence,
            observation: LifecycleObservationClaim::from_wire(
                observation.wire_kind(),
                observation.elapsed_ns,
            ),
            event_id: event.id(),
            cause_code: event.cause_code(),
        })
    }

    /// Run the shared checked emission path and return the required admission sequence.
    fn emit_to_routes(
        &mut self,
        event: ServiceEvent,
        observation: DiagnosticObservation,
    ) -> Option<u64> {
        if self.sealed {
            self.health.sealed_loss.increment();
            return None;
        }
        let Some(sequence): Option<u64> = self.next_sequence else {
            self.health.sequence_loss.increment();
            return None;
        };
        let candidate: Result<Candidate, DecodeError> = Candidate::checked(raw_record(
            self.component,
            event.id(),
            event.cause_code(),
            sequence,
            observation,
        ));
        let Ok(candidate) = candidate else {
            self.health.construction_loss.increment();
            return None;
        };
        self.next_sequence = sequence.checked_add(1);
        self.health.emitted.increment();
        self.observe_rate_clock(observation);
        let record: ProducedRecord = ProducedRecord {
            bytes: candidate.bytes,
        };
        let required_queued: bool = self.required.attempt(event, record);
        let descriptor: &EventDescriptor = event.descriptor();
        if optional_enabled(
            self.optional,
            self.importance,
            descriptor.level,
            descriptor.importance,
        ) {
            self.duplicate.attempt(event, record);
        } else {
            self.duplicate.health.filtered.increment();
        }
        required_queued.then_some(sequence)
    }

    /// Perform at most one dequeue/concrete consumer attempt per independent route.
    /// This adds no clock read, lifecycle transition, callback or blocking work.
    pub fn pump_one(&mut self) {
        self.required.pump_one();
        self.duplicate.pump_one();
    }

    /// Copy fixed current history and health; queued originals remain explicitly pending.
    pub fn snapshot(&self) -> DiagnosticSnapshot {
        DiagnosticSnapshot {
            required: self.required.snapshot(),
            optional: self.duplicate.snapshot(),
            health: self.health,
            sealed: self.sealed,
        }
    }

    /// Seal production once and pump at most 16 originals per route before copying.
    /// Repeated finalization neither reopens production nor retries a refused record.
    pub fn finish(&mut self) -> DiagnosticSnapshot {
        if !self.sealed {
            self.sealed = true;
            for _ in 0..ROUTE_CAPACITY {
                self.pump_one();
            }
        }
        self.snapshot()
    }

    /// Consume the private reason before passive encoding can erase fault semantics.
    /// Initial unobserved credits are never reset by the first accepted coordinate.
    fn observe_rate_clock(&mut self, observation: DiagnosticObservation) {
        match observation.reason {
            ObservationReason::NotObserved => (),
            ObservationReason::CheckedElapsedOverflow => {
                self.health.rate_clock_fault = true;
                self.health.elapsed_overflow.increment();
            }
            ObservationReason::OriginOrderFault => {
                self.health.rate_clock_fault = true;
                self.health.origin_order_fault.increment();
            }
            ObservationReason::ActualClockFault => {
                self.health.rate_clock_fault = true;
                self.health.actual_clock_fault.increment();
            }
            ObservationReason::Accepted => {
                if self.health.rate_clock_fault {
                    return;
                }
                if self
                    .last_coordinate
                    .is_some_and(|last: u64| observation.elapsed_ns < last)
                {
                    self.health.rate_clock_fault = true;
                    self.health.origin_order_fault.increment();
                    return;
                }
                self.last_coordinate = Some(observation.elapsed_ns);
                match self.window_anchor {
                    None => self.window_anchor = Some(observation.elapsed_ns),
                    Some(anchor) => {
                        let Some(elapsed): Option<u64> = observation.elapsed_ns.checked_sub(anchor)
                        else {
                            self.health.rate_clock_fault = true;
                            self.health.origin_order_fault.increment();
                            return;
                        };
                        if elapsed >= WINDOW_NS {
                            self.required.budget = RouteBudget::initial();
                            self.duplicate.budget = RouteBudget::initial();
                            self.window_anchor = Some(observation.elapsed_ns);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "diagnostics_tests.rs"]
mod tests;

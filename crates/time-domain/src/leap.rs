//! Immutable, explicitly scoped leap knowledge without ambient dates or effects.
//!
//! Reviewed history, finite future coverage and a continuous rule have distinct
//! meanings. Declared data identities are labels, not authenticated UTC truth.
//! A caller supplies its complete possible-date interval and finite date scope;
//! this module selects no production calendar, bootstrap or correction policy.

mod bundled;
mod generated;
mod interpretation;
mod validation;

pub use bundled::bundled_leap_knowledge;

use core::fmt;

use garde::Validate;

use crate::{EraWindow, UtcCoordinateQ32};

/// Maximum retained baseline-plus-transition records in one checked ledger.
pub const MAX_LEAP_RECORDS: usize = 128;
/// Largest non-wrapping whole NTP-epoch second accepted by leap metadata.
pub const MAX_LEAP_NTP_SECONDS: u64 = 9_223_372_036_854_775_807;
/// NTP-epoch effective second of the distinct 1 January 1972 baseline.
pub const LEAP_BASELINE_NTP_SECONDS: u64 = 2_272_060_800;

/// Fixed redacted failures; rejected inputs and validation reports are not kept.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeapError {
    /// A metadata second exceeds the signed-i64 whole-second envelope.
    InvalidMetadata,
    /// Baseline, records, calendar meaning or coverage relationships are invalid.
    InvalidRecords,
    /// A scope starts before the supported 1972 ledger baseline.
    InvalidScope,
    /// A declared data revision is zero.
    InvalidIdentity,
    /// The complete candidate interval is not contained in the explicit scope.
    UnsupportedDates,
}

impl fmt::Display for LeapError {
    /// Write a fixed message without retaining or formatting rejected input.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidMetadata => "invalid leap metadata",
            Self::InvalidRecords => "invalid leap records",
            Self::InvalidScope => "invalid leap date scope",
            Self::InvalidIdentity => "invalid leap data identity",
            Self::UnsupportedDates => "unsupported leap dates",
        })
    }
}

impl std::error::Error for LeapError {}

/// Shared mandatory scalar admission, with no report retained after rejection.
#[derive(Validate)]
struct SecondCandidate {
    /// Nonnegative whole NTP-epoch seconds, without packet-era wrapping.
    #[garde(range(max = MAX_LEAP_NTP_SECONDS))]
    seconds: u64,
}

/// Immutable scalar after the shared mandatory garde rule has executed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct LeapSecond {
    /// Checked non-wrapping second; every bit pattern is bounded by construction.
    seconds: u64,
}

impl LeapSecond {
    /// Check the scalar once; successful construction allocates no heap memory.
    /// Rejection may temporarily allocate one single-field garde error report,
    /// discarded before returning. This operation has no external effects.
    ///
    /// # Errors
    /// Returns `InvalidMetadata` for a value above `i64::MAX`.
    fn new(seconds: u64) -> Result<Self, LeapError> {
        SecondCandidate { seconds }
            .validate()
            .map_err(|_| LeapError::InvalidMetadata)?;
        Ok(Self { seconds })
    }
}

/// Exclusive reviewed historical-completeness endpoint, not a current-date claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LeapHistoryBefore {
    /// Numeric validity belongs to the private shared scalar constructor.
    second: LeapSecond,
}

impl LeapHistoryBefore {
    /// Check a whole NTP-epoch endpoint; dataset relationships are checked later.
    ///
    /// # Errors
    /// Rejects seconds above `i64::MAX`, without publishing an endpoint.
    pub fn new(seconds: u64) -> Result<Self, LeapError> {
        Ok(Self {
            second: LeapSecond::new(seconds)?,
        })
    }

    /// Return the immutable exclusive endpoint without granting provenance.
    #[must_use]
    pub const fn ntp_seconds(self) -> u64 {
        self.second.seconds
    }
}

/// Explicit finite future-coverage endpoint; zero is a value, never absence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LeapFutureExpiry {
    /// Numeric validity belongs to the private shared scalar constructor.
    second: LeapSecond,
}

impl LeapFutureExpiry {
    /// Check a finite exclusive whole NTP-epoch endpoint, preserving zero.
    ///
    /// # Errors
    /// Rejects seconds above `i64::MAX`, without publishing an endpoint.
    pub fn new(seconds: u64) -> Result<Self, LeapError> {
        Ok(Self {
            second: LeapSecond::new(seconds)?,
        })
    }

    /// Return the finite value without interpreting it as present authority.
    #[must_use]
    pub const fn ntp_seconds(self) -> u64 {
        self.second.seconds
    }
}

/// Separately supplied effective no-event rule, not an offset correction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LeapContinuousFrom {
    /// Numeric validity belongs to the private shared scalar constructor.
    second: LeapSecond,
}

impl LeapContinuousFrom {
    /// Check the inclusive whole-second start of a declared continuous rule.
    ///
    /// # Errors
    /// Rejects seconds above `i64::MAX`; record conflicts are checked by history.
    pub fn new(seconds: u64) -> Result<Self, LeapError> {
        Ok(Self {
            second: LeapSecond::new(seconds)?,
        })
    }

    /// Return the immutable rule start; no earlier offset is inferred.
    #[must_use]
    pub const fn ntp_seconds(self) -> u64 {
        self.second.seconds
    }
}

/// Mandatory revision admission; fixed-width digest labels need no content rule.
#[derive(Validate)]
struct IdentityCandidate {
    /// A declared revision has an explicit nonzero identifier.
    #[garde(range(min = 1))]
    revision: u64,
}

/// Declared immutable dataset identity, without hashing or authentication claims.
///
/// Direct field construction cannot bypass revision admission:
/// ```compile_fail,E0451
/// use time_domain::LeapDataIdentity;
/// let _ = LeapDataIdentity {
///     revision: 0,
///     raw_source_sha256: [0; 32],
///     provenance_sha256: [0; 32],
/// };
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LeapDataIdentity {
    /// Nonzero identifier admitted by the shared garde rule.
    revision: u64,
    /// All 32-byte values are valid identity labels, enforced by the array type.
    raw_source_sha256: [u8; 32],
    /// All 32-byte provenance labels are valid; no digest is verified here.
    provenance_sha256: [u8; 32],
}

impl LeapDataIdentity {
    /// Admit a revision and retain exact fixed-width source/provenance labels.
    ///
    /// Digest bytes label the supplied data; this function performs no hashing,
    /// release verification, I/O or authority admission.
    ///
    /// # Errors
    /// Rejects revision zero, publishing no partial identity.
    pub fn new(
        revision: u64,
        raw_source_sha256: [u8; 32],
        provenance_sha256: [u8; 32],
    ) -> Result<Self, LeapError> {
        IdentityCandidate { revision }
            .validate()
            .map_err(|_| LeapError::InvalidIdentity)?;
        Ok(Self {
            revision,
            raw_source_sha256,
            provenance_sha256,
        })
    }

    /// Return the declared revision, without version-transition authority.
    #[must_use]
    pub const fn revision(self) -> u64 {
        self.revision
    }

    /// Return the exact source label without claiming that it has been verified.
    #[must_use]
    pub const fn raw_source_sha256(self) -> [u8; 32] {
        self.raw_source_sha256
    }

    /// Return the exact provenance label without establishing its authenticity.
    #[must_use]
    pub const fn provenance_sha256(self) -> [u8; 32] {
        self.provenance_sha256
    }
}

/// Mandatory date-scope rule applied to an already ordered numeric era window.
#[derive(Validate)]
struct ScopeCandidate {
    /// Whole scope must remain within the modeled 1972-and-later ledger domain.
    #[garde(custom(validation::scope_rule))]
    dates: EraWindow,
}

/// Finite explicit caller scope, carrying no production-date or anchor policy.
///
/// A numeric window cannot bypass the additional scope invariant:
/// ```compile_fail,E0451
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use time_domain::{EraWindow, LeapDateScope};
/// let _ = LeapDateScope { dates: EraWindow::new(0, 1)? };
/// # Ok(()) }
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LeapDateScope {
    /// Numeric range/order is owned by `EraWindow`; baseline by construction.
    dates: EraWindow,
}

impl LeapDateScope {
    /// Admit an explicitly supplied scope without selecting a default interval.
    ///
    /// # Errors
    /// Rejects a scope starting before the 1972 ledger baseline.
    pub fn new(dates: EraWindow) -> Result<Self, LeapError> {
        ScopeCandidate { dates }
            .validate()
            .map_err(|_| LeapError::InvalidScope)?;
        Ok(Self { dates })
    }

    /// Return the exact inclusive caller bounds, with no trusted-date guarantee.
    #[must_use]
    pub const fn dates(self) -> EraWindow {
        self.dates
    }
}

/// Offset knowledge over every candidate date in an interpreted interval.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeapOffsetKnowledge {
    /// One exact TAI-minus-UTC value applies throughout the whole interval.
    Known(i32),
    /// All offsets are known, with a change after earliest and through latest.
    Varying,
    /// Some offset knowledge is absent; a later no-event rule cannot fill it.
    Unknown,
}

/// Completeness of event knowledge, separate from exact offset knowledge.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeapEventKnowledge {
    /// The entire interval is covered and contains no effective event instant.
    NoEvents,
    /// The entire interval is covered and contains at least one effective event.
    KnownEvents,
    /// Some event coverage is absent; known events are still counted separately.
    Unknown,
}

/// Immutable whole-interval answer; public labels cannot construct this result.
///
/// A consumer cannot rewrite the checked answer's knowledge:
/// ```compile_fail,E0616
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use time_domain::{EraWindow, LeapDataIdentity, LeapDateScope, LeapHistoryBefore,
///     LeapKnowledge, LeapOffsetKnowledge};
/// let records = [(2_272_060_800, 10)];
/// let dates = EraWindow::new(2_272_060_800_i128 << 32, 2_272_060_800_i128 << 32)?;
/// let history = LeapKnowledge::new(&records, LeapHistoryBefore::new(3_992_284_800)?,
///     None, None, LeapDataIdentity::new(1, [1; 32], [2; 32])?)?;
/// let mut answer = history.interpret(dates, LeapDateScope::new(dates)?)?;
/// answer.offset = LeapOffsetKnowledge::Known(37);
/// # Ok(()) }
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LeapInterpretation {
    /// Exact complete inclusive input interval retained without narrowing.
    dates: EraWindow,
    /// Identity of the actual immutable dataset consumed by the query.
    identity: LeapDataIdentity,
    /// Offset completeness and uniformity, independent of event knowledge.
    offset: LeapOffsetKnowledge,
    /// Whether all events in the candidate interval are covered.
    events: LeapEventKnowledge,
    /// Event records whose effective instants belong to the inclusive interval.
    known_transition_count: usize,
}

impl LeapInterpretation {
    /// Return the whole original candidate interval without choosing an era.
    #[must_use]
    pub const fn dates(self) -> EraWindow {
        self.dates
    }

    /// Return the consumed dataset's declared structural identity.
    #[must_use]
    pub const fn identity(self) -> LeapDataIdentity {
        self.identity
    }

    /// Return exact, varying or missing offset knowledge over the whole interval.
    #[must_use]
    pub const fn offset(self) -> LeapOffsetKnowledge {
        self.offset
    }

    /// Return event-coverage knowledge without granting sample eligibility.
    #[must_use]
    pub const fn events(self) -> LeapEventKnowledge {
        self.events
    }

    /// Count effective event instants in the inclusive input, excluding baseline.
    ///
    /// An event at the earliest endpoint counts. Consequently `[T,T]` at an
    /// event can have one known event but a single known after-event offset.
    /// This calendar membership is not an exchange-crossing or guard decision.
    #[must_use]
    pub const fn known_transition_count(self) -> usize {
        self.known_transition_count
    }
}

/// Borrowed raw ledger and checked metadata submitted to one cross-field rule.
struct LedgerValue<'records> {
    /// Inert raw tuples; no consumer sees these as checked before admission.
    records: &'records [(u64, i32)],
    /// Explicit reviewed completeness, independent of source-file metadata.
    history_before: LeapHistoryBefore,
    /// Optional finite imported coverage, preserving `Some(0)`.
    expiry: Option<LeapFutureExpiry>,
    /// Optional separate continuous rule, checked against every event record.
    continuous: Option<LeapContinuousFrom>,
}

/// One mandatory garde carrier for count, record and cross-field relationships.
#[derive(Validate)]
struct LedgerCandidate<'records> {
    /// Every ledger component is checked by the shared bounded domain rule.
    #[garde(custom(validation::ledger_rule))]
    value: LedgerValue<'records>,
}

/// Checked immutable ledger borrowed from its caller, without ambient authority.
///
/// All raw construction passes through the mandatory ledger rule:
/// ```compile_fail,E0451
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use time_domain::{LeapDataIdentity, LeapHistoryBefore, LeapKnowledge};
/// let _ = LeapKnowledge {
///     records: &[(2_272_060_800, 37)],
///     history_before: LeapHistoryBefore::new(3_992_284_800)?,
///     expiry: None,
///     continuous: None,
///     identity: LeapDataIdentity::new(1, [1; 32], [2; 32])?,
/// };
/// # Ok(()) }
/// ```
/// The original tuples stay immutable while a later consumer uses their view:
/// ```compile_fail,E0506
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use time_domain::{LeapDataIdentity, LeapHistoryBefore, LeapKnowledge};
/// let mut records = [(2_272_060_800, 10)];
/// let history = LeapKnowledge::new(&records, LeapHistoryBefore::new(3_992_284_800)?,
///     None, None, LeapDataIdentity::new(1, [1; 32], [2; 32])?)?;
/// records[0].1 = 37;
/// assert_eq!(history.records()[0].1, 10);
/// # Ok(()) }
/// ```
/// A legal caller receives immutable checked knowledge with its original labels:
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use time_domain::{EraWindow, LeapDataIdentity, LeapDateScope, LeapEventKnowledge,
///     LeapHistoryBefore, LeapKnowledge, LeapOffsetKnowledge};
/// let records = [(2_272_060_800, 10)];
/// let dates = EraWindow::new(2_272_060_800_i128 << 32, 2_272_060_800_i128 << 32)?;
/// let identity = LeapDataIdentity::new(1, [1; 32], [2; 32])?;
/// let history = LeapKnowledge::new(&records, LeapHistoryBefore::new(3_992_284_800)?,
///     None, None, identity)?;
/// let answer = history.interpret(dates, LeapDateScope::new(dates)?)?;
/// assert_eq!(answer.offset(), LeapOffsetKnowledge::Known(10));
/// assert_eq!(answer.events(), LeapEventKnowledge::NoEvents);
/// assert_eq!(answer.identity(), identity);
/// # Ok(()) }
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LeapKnowledge<'records> {
    /// Validated baseline and strictly ordered monthly signed transitions.
    records: &'records [(u64, i32)],
    /// Reviewed historical completeness; no current-time comparison is implied.
    history_before: LeapHistoryBefore,
    /// Optional finite coverage; expiry cannot erase the historical interval.
    expiry: Option<LeapFutureExpiry>,
    /// Separate effective rule; it cannot repair an earlier missing offset.
    continuous: Option<LeapContinuousFrom>,
    /// Caller-supplied source/provenance identity, without digest verification.
    identity: LeapDataIdentity,
}

impl<'records> LeapKnowledge<'records> {
    /// Validate one bounded complete ledger before publishing an immutable view.
    ///
    /// At most 128 raw tuples are inspected, without sorting, I/O or mutation.
    /// Successful construction allocates no heap memory. Rejection may allocate
    /// one temporary garde report containing at most one fixed-message error at
    /// one field path; it is discarded before returning `InvalidRecords`.
    /// Rejection storage does not grow with the record count. The borrow prevents
    /// safe mutation of retained records.
    ///
    /// # Errors
    /// Rejects invalid baseline/count, timestamps, order, signed changes,
    /// calendar meaning, incomplete record coverage or rule contradictions.
    pub fn new(
        records: &'records [(u64, i32)],
        history_before: LeapHistoryBefore,
        expiry: Option<LeapFutureExpiry>,
        continuous: Option<LeapContinuousFrom>,
        identity: LeapDataIdentity,
    ) -> Result<Self, LeapError> {
        LedgerCandidate {
            value: LedgerValue {
                records,
                history_before,
                expiry,
                continuous,
            },
        }
        .validate()
        .map_err(|_| LeapError::InvalidRecords)?;
        Ok(Self {
            records,
            history_before,
            expiry,
            continuous,
            identity,
        })
    }

    /// Return the exact bounded immutable ledger already checked by construction.
    #[must_use]
    pub const fn records(&self) -> &'records [(u64, i32)] {
        self.records
    }

    /// Return the explicit reviewed endpoint, not a host-clock-derived date.
    #[must_use]
    pub const fn history_before(&self) -> LeapHistoryBefore {
        self.history_before
    }

    /// Return the optional finite coverage endpoint, retaining present zero.
    #[must_use]
    pub const fn expiry(&self) -> Option<LeapFutureExpiry> {
        self.expiry
    }

    /// Return the separately supplied no-event rule without synthesizing an offset.
    #[must_use]
    pub const fn continuous_from(&self) -> Option<LeapContinuousFrom> {
        self.continuous
    }

    /// Return exact declared identity labels, without authentication claims.
    #[must_use]
    pub const fn identity(&self) -> LeapDataIdentity {
        self.identity
    }

    /// Interpret the complete interval against the explicit finite caller scope.
    ///
    /// No ambient date, mutation, allocation or effects are permitted. Event
    /// membership includes both endpoints; offset variation counts changes
    /// strictly after the earliest endpoint through the inclusive latest one.
    /// At most 128 records are inspected, independently of interval width.
    /// Historical/finite coverage ends exclusively; the continuous rule starts
    /// inclusively. Touching boundaries bridge knowledge without an impulse,
    /// while a positive gap preserves unknown offset after the rule begins.
    ///
    /// # Errors
    /// Rejects any candidate interval not wholly contained in the caller scope.
    pub fn interpret(
        &self,
        possible_date: EraWindow,
        scope: LeapDateScope,
    ) -> Result<LeapInterpretation, LeapError> {
        interpretation::interpret(self, possible_date, scope)
    }
}

/// Exact whole-second-to-Q32 conversion inside the admitted scalar envelope.
///
/// The `u64` value has at most 63 bits after metadata/record admission, so
/// multiplication by `2^32` remains strictly inside the signed-i128 envelope.
fn second_ticks(seconds: u64) -> i128 {
    i128::from(seconds) * (1_i128 << 32)
}

/// Return the exact inclusive baseline coordinate without fallible conversion.
fn baseline_ticks() -> i128 {
    second_ticks(LEAP_BASELINE_NTP_SECONDS)
}

/// Read the exact coordinate bits of an immutable numeric endpoint.
fn coordinate_ticks(coordinate: UtcCoordinateQ32) -> i128 {
    coordinate.ticks()
}

// SPDX-License-Identifier: GPL-2.0-only
/*
 chronyd/chronyc - Programs for keeping computer clocks accurate.

 Copyright (C) Richard P. Curnow 1997-2003
 Copyright (C) Miroslav Lichvar 2011-2016, 2018, 2020-2025

 This program is free software; you can redistribute it and/or modify it under
 the terms of version 2 of the GNU General Public License as published by the
 Free Software Foundation.

 This program is distributed in the hope that it will be useful, but WITHOUT
 ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

 You should have received a copy of the GNU General Public License along with
 this program; if not, write to the Free Software Foundation, Inc.,
 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.
*/
//! Inert fixed-roster combination lifecycle, adapted from Chrony 4.9 sources.c.
//! Modified by David Osipov with AI assistance, 31 October 2026.
//! Pinned revision: 8df4f1263e206585e6c4e61352d042c2dd12916a, sources.c
//! lines 532-539, 882-916, 1402-1408, 1451-1465 and 1475-1502.
//!
//! The pinned upstream revision and notices belong to the parent module and
//! time-core provenance ledger. This owner adds checked fixture-only staging:
//! principal credit, compatibility penalties and emission ordinals commit once,
//! after all receiving, arithmetic and output checks. No clock, network,
//! authentication, acquisition or cross-instance replay authority is supplied.
//!
//! Storage is sixteen source slots. Each advance performs one real principal
//! advance on a private clone; at most sixteen compatibility visits and one
//! existing checked reduction follow. All per-source/report loops are bounded
//! by sixteen. Reports contain at most two parameter rules or one relationship
//! rule and are discarded. These are source bounds, not measured CPU/RSS work.
//!
//! Modified by David Osipov with AI assistance, 3 November 2026: S152 adds a
//! same-owner tracking mode with complete receiving before the shared scoring,
//! penalty, credit, event and sole-commit behavior.

use std::{fmt, num::NonZeroU64};

use garde::Validate;

use crate::{
    ArithmeticOperation, ArithmeticStage, FitError, FixtureMinimum, FixturePrincipalDecision,
    FixturePrincipalError, FixturePrincipalOutcomeKind, FixturePrincipalParameters,
    FixturePrincipalState, FixturePrincipalStratum, FixtureProjectedBatch, regression::arithmetic,
};

use super::{
    AdmittedFixtureSupporters, FixtureCombinationError, FixtureCombinationScalarIssue,
    FixtureCombinedEstimate, FixtureSupporterUncertainty, FixtureTrackingCombinationError,
    MAX_SUPPORTERS, RecordSource, TrackingSupporters, admit_lifecycle_source,
    admit_lifecycle_supporters, combine_lifecycle_supporters, uncertainty_issue,
};

/// Chrony reachability-history width used only for fixture maturity.
const MATURE_UPDATES: u8 = 8;
/// Chrony's mature and outside-witness distant-source penalty.
const DISTANT_PENALTY: u8 = 32;

/// Additive tracking lifecycle failure without raw entries or state payloads.
#[derive(PartialEq)]
pub enum FixtureTrackingLifecycleError {
    /// A complete original witness participant lacks its tracking attachment.
    MissingTracking {
        /// Canonical ascending witness ordinal, issued only in zero through fifteen.
        index: usize,
    },
    /// Preserve the real checked principal, compatibility or reduction failure.
    Lifecycle {
        /// Actual bounded cause from the existing transactional lifecycle owner.
        source: FixtureCombinationLifecycleError,
    },
}

impl fmt::Debug for FixtureTrackingLifecycleError {
    /// Reveal only the type without entries, labels, parameters or prior state.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureTrackingLifecycleError { .. }")
    }
}

impl fmt::Display for FixtureTrackingLifecycleError {
    /// Emit a fixed bounded code without formatting numerical or source payloads.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingTracking { .. } => {
                formatter.write_str("fixture tracking missing metadata")
            }
            Self::Lifecycle { .. } => formatter.write_str("fixture tracking lifecycle failure"),
        }
    }
}

impl std::error::Error for FixtureTrackingLifecycleError {
    /// Borrow the actual delegated lifecycle cause without allocation or effects.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Lifecycle { source } => Some(source),
            Self::MissingTracking { .. } => None,
        }
    }
}

impl From<FixtureCombinationLifecycleError> for FixtureTrackingLifecycleError {
    /// Preserve an actual legacy-owner cause without interpreting or changing state.
    fn from(source: FixtureCombinationLifecycleError) -> Self {
        Self::Lifecycle { source }
    }
}

impl From<FixtureTrackingCombinationError> for FixtureTrackingLifecycleError {
    /// Preserve missing canonical ordinal or the exact existing combination cause.
    fn from(error: FixtureTrackingCombinationError) -> Self {
        match error {
            FixtureTrackingCombinationError::MissingTracking { index } => {
                Self::MissingTracking { index }
            }
            FixtureTrackingCombinationError::Combination { source } => Self::Lifecycle {
                source: FixtureCombinationLifecycleError::Combination { source },
            },
        }
    }
}

/// Closed per-call metadata source; neither mode resets lifecycle state.
#[derive(Clone, Copy)]
enum SupporterMode<'input, 'entry> {
    /// Borrow existing explicit uncertainties and preserve anchor-phase semantics.
    Legacy(&'input [FixtureSupporterUncertainty<'entry>]),
    /// Derive complete fitted tracking only after the genuine staged decision.
    Tracking,
}

/// Explicit uncalibrated fixture parameter whose constructor rejected input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureCombinationParameterField {
    /// Strictly positive multiplier for distance and rate compatibility.
    CombineLimit,
    /// Nonnegative maximum-clock-error assertion, in seconds/second.
    MaxClockError,
}

/// Supported-scalar or fixture range rejection, without retaining raw values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureCombinationParameterIssue {
    /// NaN or either infinity is unsupported.
    NonFinite,
    /// Nonzero subnormal input is unsupported.
    Subnormal,
    /// A supported negative value is outside the fixture domain.
    Negative,
    /// The combine multiplier must exceed zero; either zero sign is rejected.
    NonPositive,
    /// The shared scalar classifier violated its documented issue projection.
    Unsupported,
}

/// Reached compatibility primitive whose actual arithmetic cause is retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureCombinationLifecycleStep {
    /// Original principal distance plus its checked reselection distance.
    SelectedDistance,
    /// Combine multiplier times the selected-distance quantity.
    DistanceLimit,
    /// Original principal rate minus original candidate rate.
    RateDifference,
    /// Original principal tracking skew plus candidate tracking skew.
    SkewSum,
    /// The skew sum plus the explicit maximum-clock-error assertion.
    MaxClockErrorSum,
    /// Combine multiplier times the complete skew/error sum.
    RateLimit,
}

/// Closed lifecycle errors; no raw parameters, records or validator reports.
#[derive(PartialEq)]
pub enum FixtureCombinationLifecycleError {
    /// The shared fixture parameter predicate rejected this declared field.
    InvalidParameter {
        /// First rejected field in constructor order.
        field: FixtureCombinationParameterField,
        /// Bounded supported-scalar/range classification.
        issue: FixtureCombinationParameterIssue,
    },
    /// Reachability event ordinal zero is not a new fixture event.
    InvalidReachabilityOrdinal,
    /// A reachability event does not name a member of this fixed roster.
    UnknownSource,
    /// A reachability event is not exactly the next event for that source.
    ReachabilityOrder,
    /// A source event or emission ordinal cannot advance without wrapping.
    CounterExhausted,
    /// A raw input slice exceeds the fixed sixteen-source capacity.
    CapacityExceeded,
    /// Preserve the actual real principal transition failure.
    Principal {
        /// Original bounded typed principal cause.
        source: FixturePrincipalError,
    },
    /// Preserve the parent admission, mask or checked-reduction failure.
    Combination {
        /// Original bounded typed combination cause.
        source: FixtureCombinationError,
    },
    /// Preserve the actual reached compatibility primitive failure.
    Arithmetic {
        /// Operation's declared compatibility role.
        step: FixtureCombinationLifecycleStep,
        /// Original shared checked-numerical cause.
        source: FitError,
    },
    /// Checked private state or transformed output relations disagree.
    Invariant,
    /// Mandatory validation disagreed with its maintained typed predicate.
    ValidationInvariant,
}

impl fmt::Debug for FixtureCombinationLifecycleError {
    /// Expose only the error type, never values, labels, state or reports.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureCombinationLifecycleError { .. }")
    }
}

impl fmt::Display for FixtureCombinationLifecycleError {
    /// Project closed codes and enum identifiers without formatting causes.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidParameter { field, issue } => {
                write!(
                    formatter,
                    "FIXTURE_COMBINATION_PARAMETER_{field:?}_{issue:?}"
                )
            }
            Self::Arithmetic { step, .. } => {
                write!(formatter, "FIXTURE_COMBINATION_ARITHMETIC_{step:?}")
            }
            Self::InvalidReachabilityOrdinal => {
                formatter.write_str("FIXTURE_COMBINATION_EVENT_ZERO")
            }
            Self::UnknownSource => formatter.write_str("FIXTURE_COMBINATION_UNKNOWN_SOURCE"),
            Self::ReachabilityOrder => formatter.write_str("FIXTURE_COMBINATION_EVENT_ORDER"),
            Self::CounterExhausted => formatter.write_str("FIXTURE_COMBINATION_COUNTER_EXHAUSTED"),
            Self::CapacityExceeded => formatter.write_str("FIXTURE_COMBINATION_CAPACITY"),
            Self::Principal { .. } => formatter.write_str("FIXTURE_COMBINATION_PRINCIPAL"),
            Self::Combination { .. } => formatter.write_str("FIXTURE_COMBINATION_KERNEL"),
            Self::Invariant => formatter.write_str("FIXTURE_COMBINATION_INVARIANT"),
            Self::ValidationInvariant => formatter.write_str("FIXTURE_COMBINATION_VALIDATION"),
        }
    }
}

impl std::error::Error for FixtureCombinationLifecycleError {
    /// Borrow delegated causes without flattening, cloning or authorizing logs.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Principal { source } => Some(source),
            Self::Combination { source } => Some(source),
            Self::Arithmetic { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Classify one raw parameter through the parent's supported-scalar predicate.
fn parameter_issue(
    value: f64,
    field: FixtureCombinationParameterField,
) -> Option<FixtureCombinationParameterIssue> {
    match uncertainty_issue(value) {
        Some(FixtureCombinationScalarIssue::NonFinite) => {
            Some(FixtureCombinationParameterIssue::NonFinite)
        }
        Some(FixtureCombinationScalarIssue::Subnormal) => {
            Some(FixtureCombinationParameterIssue::Subnormal)
        }
        Some(FixtureCombinationScalarIssue::Negative) => {
            Some(FixtureCombinationParameterIssue::Negative)
        }
        Some(FixtureCombinationScalarIssue::Unsupported) => {
            Some(FixtureCombinationParameterIssue::Unsupported)
        }
        None if field == FixtureCombinationParameterField::CombineLimit && value == 0.0 => {
            Some(FixtureCombinationParameterIssue::NonPositive)
        }
        None => None,
    }
}

/// Use the maintained scalar/range predicate in mandatory Garde callbacks.
/// # Errors
/// Return fixed rejection text only when the same typed predicate rejects.
fn parameter_rule(value: f64, field: FixtureCombinationParameterField) -> garde::Result {
    if parameter_issue(value, field).is_none() {
        Ok(())
    } else {
        Err(garde::Error::new("invalid fixture combination parameter"))
    }
}

/// Only the two raw narrowed scalars enter this parameter validation payload.
#[derive(Validate)]
struct RawParameters {
    /// Explicit strictly positive compatibility multiplier.
    #[garde(custom(|value: &f64, (): &()| parameter_rule(*value, FixtureCombinationParameterField::CombineLimit)))]
    combine_limit: f64,
    /// Explicit nonnegative maximum-error assertion in seconds/second.
    #[garde(custom(|value: &f64, (): &()| parameter_rule(*value, FixtureCombinationParameterField::MaxClockError)))]
    max_clock_error_seconds_per_second: f64,
}

/// Immutable explicit fixture parameters; they establish no physical calibration.
#[derive(Clone, Copy)]
pub struct FixtureCombinationParameters {
    /// Mandatorily admitted positive multiplier, with original bits.
    combine_limit: f64,
    /// Mandatorily admitted error assertion, preserving either zero sign.
    max_clock_error_seconds_per_second: f64,
}

impl fmt::Debug for FixtureCombinationParameters {
    /// Redact the actual supplied floating-point assertions.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureCombinationParameters { .. }")
    }
}

impl PartialEq for FixtureCombinationParameters {
    /// Preserve every admitted bit, including the maximum-error zero sign.
    fn eq(&self, other: &Self) -> bool {
        self.combine_limit.to_bits() == other.combine_limit.to_bits()
            && self.max_clock_error_seconds_per_second.to_bits()
                == other.max_clock_error_seconds_per_second.to_bits()
    }
}

impl FixtureCombinationParameters {
    /// Validate both explicit inputs without defaults, normalization or effects.
    /// # Errors
    /// Reject unsupported/negative scalars and a nonpositive combine multiplier.
    pub fn new(
        combine_limit: f64,
        max_clock_error_seconds_per_second: f64,
    ) -> Result<Self, FixtureCombinationLifecycleError> {
        let raw = RawParameters {
            combine_limit,
            max_clock_error_seconds_per_second,
        };
        let validation = raw.validate();
        for (field, value) in [
            (
                FixtureCombinationParameterField::CombineLimit,
                combine_limit,
            ),
            (
                FixtureCombinationParameterField::MaxClockError,
                max_clock_error_seconds_per_second,
            ),
        ] {
            if let Some(issue) = parameter_issue(value, field) {
                return Err(FixtureCombinationLifecycleError::InvalidParameter { field, issue });
            }
        }
        validation.map_err(|_| FixtureCombinationLifecycleError::ValidationInvariant)?;
        Ok(Self {
            combine_limit,
            max_clock_error_seconds_per_second,
        })
    }

    /// Copy the explicit positive compatibility multiplier without mutation.
    #[must_use]
    pub const fn combine_limit(&self) -> f64 {
        self.combine_limit
    }

    /// Copy the explicit uncalibrated error assertion, preserving its zero sign.
    #[must_use]
    pub const fn max_clock_error_seconds_per_second(&self) -> f64 {
        self.max_clock_error_seconds_per_second
    }
}

/// The sole new scalar restriction on a raw fixture reachability event.
#[derive(Validate)]
struct RawOrdinal {
    /// A raw ordinal must represent a first or later fixture event.
    #[garde(range(min = 1))]
    ordinal: u64,
}

/// Immutable inert event; registered membership and exact-next order are contextual.
#[derive(Clone, Copy)]
pub struct FixtureReachabilityUpdate {
    /// Checked nonzero label; registered membership belongs to the receiver.
    source_id: NonZeroU64,
    /// Mandatory-constructor-admitted nonzero event ordinal.
    ordinal: u64,
    /// Assertion copied without interpreting it as source eligibility.
    reachable: bool,
}

impl fmt::Debug for FixtureReachabilityUpdate {
    /// Do not expose labels or event payloads through automatic formatting.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureReachabilityUpdate { .. }")
    }
}

impl FixtureReachabilityUpdate {
    /// Admit one nonzero ordinal; a label and Boolean already have closed types.
    /// # Errors
    /// Reject ordinal zero through mandatory Garde validation.
    pub fn new(
        source_id: NonZeroU64,
        ordinal: u64,
        reachable: bool,
    ) -> Result<Self, FixtureCombinationLifecycleError> {
        let validation = RawOrdinal { ordinal }.validate();
        if ordinal == 0 {
            return Err(FixtureCombinationLifecycleError::InvalidReachabilityOrdinal);
        }
        validation.map_err(|_| FixtureCombinationLifecycleError::ValidationInvariant)?;
        Ok(Self {
            source_id,
            ordinal,
            reachable,
        })
    }

    /// Copy the asserted fixture source label without registering it.
    #[must_use]
    pub const fn source_id(&self) -> NonZeroU64 {
        self.source_id
    }
    /// Copy the admitted nonzero event ordinal without consuming it.
    #[must_use]
    pub const fn ordinal(&self) -> u64 {
        self.ordinal
    }
    /// Copy the outcome assertion; both Boolean values count as an update.
    #[must_use]
    pub const fn reachable(&self) -> bool {
        self.reachable
    }
}

/// State rules cover the relationship between maturity, watermark and penalty.
fn source_fields_valid(value: (u8, u8, u64)) -> bool {
    value.0 <= MATURE_UPDATES
        && value.1 <= DISTANT_PENALTY
        && u64::from(value.0) == value.2.min(u64::from(MATURE_UPDATES))
}

/// Delegate the complete bounded source-state relationship to one predicate.
/// # Errors
/// Reject a maturity, penalty or ordinal relationship outside the fixed domain.
fn source_fields_rule(value: (u8, u8, u64)) -> garde::Result {
    if source_fields_valid(value) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid fixture source counters"))
    }
}

/// One whole source-counter relationship; no raw fields bypass the shared rule.
#[derive(Validate)]
struct RawSourceFields {
    /// Saturating maturity, penalty and the complete event watermark.
    #[garde(custom(|value: &(u8, u8, u64), (): &()| source_fields_rule(*value)))]
    counters: (u8, u8, u64),
}

/// Read-only committed per-source fixture counters; no mutation factory is public.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct FixtureCombinationSourceState {
    /// Original registered nonzero fixture label.
    source_id: NonZeroU64,
    /// Number of admitted events, saturated at the fixed maturity width.
    reachability_updates: u8,
    /// Checked post-transition distant penalty.
    penalty: u8,
    /// Last accepted event ordinal, retained after maturity saturation.
    last_reachability_ordinal: u64,
}

impl fmt::Debug for FixtureCombinationSourceState {
    /// Redact source identity and counters from automatic diagnostics.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureCombinationSourceState { .. }")
    }
}

impl FixtureCombinationSourceState {
    /// Construct or transform one complete source counter tuple mandatorily.
    /// # Errors
    /// Reject an unsupported maturity, penalty or ordinal/maturity relationship.
    fn new(
        source_id: NonZeroU64,
        reachability_updates: u8,
        penalty: u8,
        last_reachability_ordinal: u64,
    ) -> Result<Self, FixtureCombinationLifecycleError> {
        let counters = (reachability_updates, penalty, last_reachability_ordinal);
        let validation = RawSourceFields { counters }.validate();
        if !source_fields_valid(counters) {
            return Err(FixtureCombinationLifecycleError::Invariant);
        }
        validation.map_err(|_| FixtureCombinationLifecycleError::ValidationInvariant)?;
        Ok(Self {
            source_id,
            reachability_updates,
            penalty,
            last_reachability_ordinal,
        })
    }

    /// Replace only a penalty through the same complete private constructor.
    /// # Errors
    /// Preserve mandatory validation failure without changing the prior source.
    fn with_penalty(self, penalty: u8) -> Result<Self, FixtureCombinationLifecycleError> {
        Self::new(
            self.source_id,
            self.reachability_updates,
            penalty,
            self.last_reachability_ordinal,
        )
    }

    /// Copy the registered label without creating observation credit.
    #[must_use]
    pub const fn source_id(&self) -> NonZeroU64 {
        self.source_id
    }
    /// Copy the saturating count of accepted reachability events, in 0..=8.
    #[must_use]
    pub const fn reachability_updates(&self) -> u8 {
        self.reachability_updates
    }
    /// Copy the post-transition distant penalty, in 0..=32.
    #[must_use]
    pub const fn penalty(&self) -> u8 {
        self.penalty
    }
    /// Copy the complete event watermark, which continues after maturity.
    #[must_use]
    pub const fn last_reachability_ordinal(&self) -> u64 {
        self.last_reachability_ordinal
    }
}

/// Prepare a nonwrapping next ordinal over the complete raw u64 domain.
/// # Errors
/// Return counter exhaustion at MAX; never wrap or silently reuse an identity.
fn checked_next(previous: u64) -> Result<NonZeroU64, FixtureCombinationLifecycleError> {
    previous
        .checked_add(1)
        .and_then(NonZeroU64::new)
        .ok_or(FixtureCombinationLifecycleError::CounterExhausted)
}

/// Bounded event/owner relationship, separate from raw-ordinal construction.
struct EventInput<'state> {
    /// Immutable current fixed roster and its complete event watermarks.
    inner: &'state Inner,
    /// Independently submitted checked ordinal, label and Boolean assertion.
    update: FixtureReachabilityUpdate,
}

/// Admit registered membership and exact-next ordering through one shared owner.
/// # Errors
/// Reject unknown labels, exhausted ordinals or any replay/skip before mutation.
fn admit_event(
    value: &EventInput<'_>,
) -> Result<(usize, FixtureCombinationSourceState), FixtureCombinationLifecycleError> {
    let index = source_index(value.inner, value.update.source_id)?;
    let source = source_at(value.inner, index)?;
    if checked_next(source.last_reachability_ordinal)?.get() != value.update.ordinal {
        return Err(FixtureCombinationLifecycleError::ReachabilityOrder);
    }
    Ok((index, source))
}

/// Use the same complete receiving predicate in mandatory contextual validation.
/// # Errors
/// Return fixed text for an unknown, exhausted, replayed or skipped event.
fn event_rule(value: &EventInput<'_>) -> garde::Result {
    admit_event(value)
        .map(|_| ())
        .map_err(|_| garde::Error::new("invalid fixture event relationship"))
}

/// One mandatory rule covers label membership and exact per-source event order.
#[derive(Validate)]
struct RawEventAdmission<'state> {
    /// Immutable owner and independently supplied event relationship.
    #[garde(custom(|value: &EventInput<'_>, (): &()| event_rule(value)))]
    value: EventInput<'state>,
}

/// No new compatibility assessment is implied by a non-ready status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureCombinationSourceStatus {
    /// Source is outside this round's actual witnessed agreement.
    OutsideWitness,
    /// Original principal actually contributes to a new estimate.
    Principal,
    /// Compatible original supporter actually contributes to a new estimate.
    Contributor,
    /// The strict root-distance comparison excluded this source.
    DistanceExcluded,
    /// The reached strict original-rate comparison excluded this source.
    RateExcluded,
    /// Compatible now, but its post-decrement penalty remains nonzero.
    PenaltyCountdown,
    /// No compatibility assessment was performed for this witnessed source.
    NotEvaluated,
}

/// One occupied output status, associated with its original roster label.
#[derive(Clone, Copy)]
struct SourceOutcome {
    /// Same registered label as the corresponding immutable state slot.
    source_id: NonZeroU64,
    /// Fresh-round assessment, or an explicit non-evaluation status.
    status: FixtureCombinationSourceStatus,
}

/// Privately clonable complete transaction; no reference into staged state escapes.
#[derive(Clone, PartialEq)]
struct Inner {
    /// Real principal owner, including its pending-credit and observation caches.
    principal: FixturePrincipalState,
    /// Fixed admitted compatibility inputs for this fixture instance.
    parameters: FixtureCombinationParameters,
    /// Occupied original roster prefix followed by inaccessible empty padding.
    sources: [Option<FixtureCombinationSourceState>; MAX_SUPPORTERS],
    /// Fixed occupied source count, delegated from admitted principal seeding.
    len: usize,
    /// Count of successfully committed ready estimates in this instance only.
    emitted_rounds: u64,
}

/// One fixed-roster fixture owner; cloning observation/emission credit is not public.
pub struct FixtureCombinationState {
    /// Sole mutable complete state, replaced only after successful preparation.
    inner: Inner,
}

impl fmt::Debug for FixtureCombinationState {
    /// Redact original principal metadata and all stored fixture counters.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureCombinationState { .. }")
    }
}

impl PartialEq for FixtureCombinationState {
    /// Compare complete committed state, including all parameter/score bits.
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

/// Complete immutable principal result and honest per-round contributor facts.
pub struct FixtureCombinationOutcome<'batch> {
    /// Actual same-batch principal result, borrowing original immutable entries.
    principal: FixturePrincipalDecision<'batch>,
    /// Newly computed original-record estimate, absent for non-ready outcomes.
    combined: Option<FixtureCombinedEstimate>,
    /// New checked emission identity for this owner lifetime only.
    emitted_round: Option<NonZeroU64>,
    /// Complete original roster statuses, with no accessible padding values.
    sources: [Option<SourceOutcome>; MAX_SUPPORTERS],
    /// Admitted occupied status prefix, matching the original state roster.
    len: usize,
}

impl fmt::Debug for FixtureCombinationOutcome<'_> {
    /// Expose only the outcome type, never raw estimates or original entries.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureCombinationOutcome { .. }")
    }
}

impl<'batch> FixtureCombinationOutcome<'batch> {
    /// Borrow the real same-batch principal result without repeating a transition.
    #[must_use]
    pub const fn principal(&self) -> &FixturePrincipalDecision<'batch> {
        &self.principal
    }
    /// Borrow a newly computed estimate only for an actually ready transition.
    #[must_use]
    pub const fn combined(&self) -> Option<&FixtureCombinedEstimate> {
        self.combined.as_ref()
    }
    /// Copy this owner's new round ordinal; non-ready outcomes return None.
    #[must_use]
    pub const fn emitted_round(&self) -> Option<NonZeroU64> {
        self.emitted_round
    }
    /// Count real witnessed voters independently of mathematical filtering.
    #[must_use]
    pub fn witness_count(&self) -> Option<usize> {
        self.principal
            .selection()
            .witnessed()
            .map(|witness| witness.supporters().len())
    }
    /// Count actual mathematical contributors only when a new estimate exists.
    #[must_use]
    pub fn contributor_count(&self) -> Option<usize> {
        self.combined
            .as_ref()
            .map(FixtureCombinedEstimate::supporter_count)
    }
    /// Observe one registered source's status without refreshing its assessment.
    #[must_use]
    pub fn source_status(&self, source_id: NonZeroU64) -> Option<FixtureCombinationSourceStatus> {
        self.sources
            .iter()
            .take(self.len)
            .flatten()
            .find(|source| source.source_id == source_id)
            .map(|source| source.status)
    }
}

/// Locate an occupied registered slot without exposing mutable state.
/// # Errors
/// Reject a label absent from the fixed occupied prefix.
fn source_index(
    inner: &Inner,
    label: NonZeroU64,
) -> Result<usize, FixtureCombinationLifecycleError> {
    inner
        .sources
        .iter()
        .take(inner.len)
        .position(|slot| {
            slot.as_ref()
                .is_some_and(|source| source.source_id == label)
        })
        .ok_or(FixtureCombinationLifecycleError::UnknownSource)
}

/// Read one occupied fixed slot; padding is never interpreted as a source.
/// # Errors
/// Reject an out-of-prefix or unexpectedly absent slot.
fn source_at(
    inner: &Inner,
    index: usize,
) -> Result<FixtureCombinationSourceState, FixtureCombinationLifecycleError> {
    if index >= inner.len {
        return Err(FixtureCombinationLifecycleError::Invariant);
    }
    inner
        .sources
        .get(index)
        .copied()
        .flatten()
        .ok_or(FixtureCombinationLifecycleError::Invariant)
}

/// Replace an existing same-label slot only after complete source construction.
/// # Errors
/// Reject a changed label or invalid slot before the only assignment.
fn replace_source(
    inner: &mut Inner,
    index: usize,
    source: FixtureCombinationSourceState,
) -> Result<(), FixtureCombinationLifecycleError> {
    if source_at(inner, index)?.source_id != source.source_id {
        return Err(FixtureCombinationLifecycleError::Invariant);
    }
    *inner
        .sources
        .get_mut(index)
        .ok_or(FixtureCombinationLifecycleError::Invariant)? = Some(source);
    Ok(())
}

/// Perform one separately checked compatibility primitive with a named cause.
/// # Errors
/// Retain the unchanged shared numerical failure under this compatibility step.
fn binary(
    left: f64,
    right: f64,
    operation: ArithmeticOperation,
    step: FixtureCombinationLifecycleStep,
) -> Result<f64, FixtureCombinationLifecycleError> {
    arithmetic(left, right, operation, ArithmeticStage::Combination)
        .map_err(|source| FixtureCombinationLifecycleError::Arithmetic { step, source })
}

/// Freeze the original principal-based distance threshold once for this round.
/// # Errors
/// Preserve either reached checked add/multiply failure without a partial limit.
fn distance_limit(
    admitted: &AdmittedFixtureSupporters<'_, '_>,
    inner: &Inner,
) -> Result<f64, FixtureCombinationLifecycleError> {
    let distance = binary(
        admitted.principal().projection().distance_seconds(),
        inner.principal.parameters().reselect_distance_seconds(),
        ArithmeticOperation::Add,
        FixtureCombinationLifecycleStep::SelectedDistance,
    )?;
    binary(
        inner.parameters.combine_limit,
        distance,
        ArithmeticOperation::Multiply,
        FixtureCombinationLifecycleStep::DistanceLimit,
    )
}

/// Classify only a nonprincipal original supporter, preserving distance short circuit.
/// # Errors
/// Preserve every reached checked primitive; do not evaluate rate after distance exclusion.
fn incompatibility(
    admitted: &AdmittedFixtureSupporters<'_, '_>,
    label: NonZeroU64,
    limit: f64,
    parameters: FixtureCombinationParameters,
) -> Result<Option<FixtureCombinationSourceStatus>, FixtureCombinationLifecycleError> {
    let record = admitted
        .record(label)
        .map_err(|source| FixtureCombinationLifecycleError::Combination { source })?;
    if record.entry().projection().distance_seconds() > limit {
        return Ok(Some(FixtureCombinationSourceStatus::DistanceExcluded));
    }
    let principal_label = admitted.principal().projection().context().source_id();
    let principal = admitted
        .record(principal_label)
        .map_err(|source| FixtureCombinationLifecycleError::Combination { source })?;
    let difference = binary(
        principal
            .entry()
            .projection()
            .phase_rate_seconds_per_second(),
        record.entry().projection().phase_rate_seconds_per_second(),
        ArithmeticOperation::Subtract,
        FixtureCombinationLifecycleStep::RateDifference,
    )?;
    let skew = binary(
        principal.tracking_skew_seconds_per_second(),
        record.tracking_skew_seconds_per_second(),
        ArithmeticOperation::Add,
        FixtureCombinationLifecycleStep::SkewSum,
    )?;
    let error = binary(
        skew,
        parameters.max_clock_error_seconds_per_second,
        ArithmeticOperation::Add,
        FixtureCombinationLifecycleStep::MaxClockErrorSum,
    )?;
    let rate_limit = binary(
        parameters.combine_limit,
        error,
        ArithmeticOperation::Multiply,
        FixtureCombinationLifecycleStep::RateLimit,
    )?;
    // Checked subtraction returns a supported finite normal or signed zero.
    // Absolute value changes only its sign bit, preserving that support domain.
    Ok((difference.abs() > rate_limit).then_some(FixtureCombinationSourceStatus::RateExcluded))
}

/// Populate statuses and reset nonwitness penalties only on the witnessed path.
/// # Errors
/// Reject a private slot or checked counter inconsistency on unpublished state.
fn prepare_scoring(
    inner: &mut Inner,
    decision: &FixturePrincipalDecision<'_>,
) -> Result<[Option<SourceOutcome>; MAX_SUPPORTERS], FixtureCombinationLifecycleError> {
    let mut statuses = [None; MAX_SUPPORTERS];
    for index in 0..inner.len {
        let source = source_at(inner, index)?;
        let outside = decision
            .selection()
            .witnessed()
            .is_some_and(|witness| !witness.supporters().any(|label| label == source.source_id));
        let status = if outside {
            replace_source(inner, index, source.with_penalty(DISTANT_PENALTY)?)?;
            FixtureCombinationSourceStatus::OutsideWitness
        } else {
            FixtureCombinationSourceStatus::NotEvaluated
        };
        *statuses
            .get_mut(index)
            .ok_or(FixtureCombinationLifecycleError::Invariant)? = Some(SourceOutcome {
            source_id: source.source_id,
            status,
        });
    }
    Ok(statuses)
}

/// Clear all source penalties after an actual ready principal replacement.
/// # Errors
/// Retain private source-construction/slot errors; the live owner is untouched.
fn clear_penalties(inner: &mut Inner) -> Result<(), FixtureCombinationLifecycleError> {
    for index in 0..inner.len {
        let source = source_at(inner, index)?;
        replace_source(inner, index, source.with_penalty(0)?)?;
    }
    Ok(())
}

/// Assess the original witness in sorted order and prepare its private mask.
/// # Errors
/// Preserve reached compatibility errors and reject slot/principal inconsistencies.
fn contributor_mask(
    inner: &mut Inner,
    admitted: &AdmittedFixtureSupporters<'_, '_>,
    statuses: &mut [Option<SourceOutcome>; MAX_SUPPORTERS],
) -> Result<[bool; MAX_SUPPORTERS], FixtureCombinationLifecycleError> {
    let mut included = [false; MAX_SUPPORTERS];
    let principal_label = admitted.principal().projection().context().source_id();
    let singleton = admitted.count() == 1;
    let limit = if singleton {
        None
    } else {
        Some(distance_limit(admitted, inner)?)
    };
    for (position, label) in admitted.labels().enumerate() {
        let index = source_index(inner, label)?;
        let source = source_at(inner, index)?;
        let (next, status, contributes) = if label == principal_label {
            if source.penalty != 0 {
                return Err(FixtureCombinationLifecycleError::Invariant);
            }
            (source, FixtureCombinationSourceStatus::Principal, true)
        } else {
            if singleton {
                return Err(FixtureCombinationLifecycleError::Invariant);
            }
            if let Some(reason) = incompatibility(
                admitted,
                label,
                limit.ok_or(FixtureCombinationLifecycleError::Invariant)?,
                inner.parameters,
            )? {
                let penalty = if source.reachability_updates >= MATURE_UPDATES {
                    DISTANT_PENALTY
                } else {
                    1
                };
                (source.with_penalty(penalty)?, reason, false)
            } else {
                let penalty = source.penalty.saturating_sub(1);
                let next = source.with_penalty(penalty)?;
                if penalty == 0 {
                    (next, FixtureCombinationSourceStatus::Contributor, true)
                } else {
                    (
                        next,
                        FixtureCombinationSourceStatus::PenaltyCountdown,
                        false,
                    )
                }
            }
        };
        replace_source(inner, index, next)?;
        *included
            .get_mut(position)
            .ok_or(FixtureCombinationLifecycleError::Invariant)? = contributes;
        *statuses
            .get_mut(index)
            .ok_or(FixtureCombinationLifecycleError::Invariant)? = Some(SourceOutcome {
            source_id: label,
            status,
        });
    }
    Ok(included)
}

/// Borrowed complete output relationship, validated before committing live state.
struct OutcomeInput<'value, 'batch> {
    /// Genuine result returned by the privately staged principal owner.
    principal: &'value FixturePrincipalDecision<'batch>,
    /// Optional numerical result already validated by the parent bridge.
    combined: Option<&'value FixtureCombinedEstimate>,
    /// Optional next per-instance ready-emission identity.
    round: Option<NonZeroU64>,
    /// Complete prepared output statuses, still unpublished.
    statuses: &'value [Option<SourceOutcome>; MAX_SUPPORTERS],
    /// Complete next state whose relations must agree with this output.
    inner: &'value Inner,
    /// Entry-state emission count, used to reject duplicate/partial publication.
    previous_rounds: u64,
}

/// Check all output relationships before any live state replacement.
fn outcome_valid(value: &OutcomeInput<'_, '_>) -> bool {
    let ready = value.principal.kind() == FixturePrincipalOutcomeKind::ReferenceReady;
    let inner = value.inner;
    if inner.len > MAX_SUPPORTERS
        || inner.len != inner.principal.roster_len()
        || value.principal.principal_label() != inner.principal.principal()
        || value.principal.selection().minimum() != inner.principal.minimum().get()
        || ready != value.combined.is_some()
        || ready != value.round.is_some()
    {
        return false;
    }
    if let Some(round) = value.round {
        if value.previous_rounds.checked_add(1) != Some(round.get())
            || round.get() != inner.emitted_rounds
        {
            return false;
        }
    } else if inner.emitted_rounds != value.previous_rounds {
        return false;
    }
    let mut contributors = 0_usize;
    let mut principals = 0_usize;
    for index in 0..MAX_SUPPORTERS {
        let (Some(source), Some(status)) = (
            inner.sources.get(index).copied().flatten(),
            value.statuses.get(index).copied().flatten(),
        ) else {
            if index < inner.len {
                return false;
            }
            if inner.sources.get(index).is_some_and(Option::is_some)
                || value.statuses.get(index).is_some_and(Option::is_some)
            {
                return false;
            }
            continue;
        };
        if index >= inner.len || source.source_id != status.source_id {
            return false;
        }
        let witnessed = value
            .principal
            .selection()
            .witnessed()
            .is_some_and(|witness| witness.supporters().any(|label| label == source.source_id));
        let contributes = matches!(
            status.status,
            FixtureCombinationSourceStatus::Principal | FixtureCombinationSourceStatus::Contributor
        );
        if !ready {
            let expected = if value.principal.selection().witnessed().is_some() && !witnessed {
                FixtureCombinationSourceStatus::OutsideWitness
            } else {
                FixtureCombinationSourceStatus::NotEvaluated
            };
            if status.status != expected {
                return false;
            }
        } else if !witnessed {
            if status.status != FixtureCombinationSourceStatus::OutsideWitness {
                return false;
            }
        } else if status.status == FixtureCombinationSourceStatus::OutsideWitness
            || status.status == FixtureCombinationSourceStatus::NotEvaluated
        {
            return false;
        }
        if status.status == FixtureCombinationSourceStatus::Principal {
            principals += 1;
            if Some(source.source_id) != value.principal.principal_label() || source.penalty != 0 {
                return false;
            }
        }
        if contributes {
            contributors += 1;
            if source.penalty != 0
                || !value.combined.is_some_and(|estimate| {
                    estimate.supporters().any(|label| label == source.source_id)
                })
            {
                return false;
            }
        } else if ready && witnessed && source.penalty == 0 {
            return false;
        }
    }
    estimate_relation_valid(value, contributors, principals)
}

/// Check only estimate cardinality and original-principal provenance.
///
/// The caller supplies the counts from its complete fixed-prefix status scan;
/// source/status and emission consistency remain in `outcome_valid`. Arbitrary
/// counts cannot bypass this helper's required cardinality/provenance relations.
/// This adds no scan, mutation, arithmetic or report allocation.
fn estimate_relation_valid(
    value: &OutcomeInput<'_, '_>,
    contributors: usize,
    principals: usize,
) -> bool {
    if let Some(estimate) = value.combined {
        let Some(principal) = value.principal.principal_entry() else {
            return false;
        };
        let projection = principal.projection();
        if principals != 1
            || contributors != estimate.supporter_count()
            || contributors == 0
            || estimate.principal_label() != projection.context().source_id()
            || estimate.frame() != projection.context().frame()
            || estimate.mapping_generation() != projection.context().generation()
            || estimate.anchor_observed_at() != projection.anchor_observed_at()
            || estimate.latest_observed_at() != projection.latest_observed_at()
            || estimate.fit_reference() != projection.fit_reference()
            || estimate.projected_at() != projection.projected_at()
        {
            return false;
        }
    } else if contributors != 0 || principals != 0 {
        return false;
    }
    true
}

/// Share the complete postcondition predicate with mandatory Garde validation.
/// # Errors
/// Return fixed text when the prepared state/output relationships disagree.
fn outcome_rule(value: &OutcomeInput<'_, '_>) -> garde::Result {
    if outcome_valid(value) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid fixture lifecycle output"))
    }
}

/// One fixed relationship rule covers every changed state/output association.
#[derive(Validate)]
struct RawOutcome<'value, 'batch> {
    /// Borrowed typed values and their still-unpublished shared relationships.
    #[garde(custom(|value: &OutcomeInput<'_, '_>, (): &()| outcome_rule(value)))]
    value: OutcomeInput<'value, 'batch>,
}

/// Validate the complete borrowed output, then move its genuine checked values.
/// # Errors
/// Reject any relationship or validation disagreement before moving or committing.
fn complete_outcome<'batch>(
    principal: FixturePrincipalDecision<'batch>,
    combined: Option<FixtureCombinedEstimate>,
    round: Option<NonZeroU64>,
    statuses: [Option<SourceOutcome>; MAX_SUPPORTERS],
    inner: &Inner,
    previous_rounds: u64,
) -> Result<FixtureCombinationOutcome<'batch>, FixtureCombinationLifecycleError> {
    let value = OutcomeInput {
        principal: &principal,
        combined: combined.as_ref(),
        round,
        statuses: &statuses,
        inner,
        previous_rounds,
    };
    let raw = RawOutcome { value };
    let validation = raw.validate();
    if !outcome_valid(&raw.value) {
        return Err(FixtureCombinationLifecycleError::Invariant);
    }
    validation.map_err(|_| FixtureCombinationLifecycleError::ValidationInvariant)?;
    Ok(FixtureCombinationOutcome {
        principal,
        combined,
        emitted_round: round,
        sources: statuses,
        len: inner.len,
    })
}

/// Direct primitive exhaustion evidence is kept outside production line coverage.
#[cfg(test)]
#[path = "../../tests/private/lifecycle_counter.rs"]
mod counter_tests;

impl FixtureCombinationState {
    /// Admit one fixed owner through the real principal seed and checked counters.
    ///
    /// This resets only a newly constructed fixture instance. Maturity and event
    /// ordinals start at zero; principal seed observations retain genuine pending
    /// credit. No production source-incarnation or restart policy is implied.
    /// # Errors
    /// Preserve seed admission or private counter validation; publish no owner on error.
    pub fn seed(
        roster: &[NonZeroU64],
        principal_parameters: FixturePrincipalParameters,
        combination_parameters: FixtureCombinationParameters,
        minimum: FixtureMinimum,
        batch: &FixtureProjectedBatch<'_>,
        strata: &[FixturePrincipalStratum],
    ) -> Result<Self, FixtureCombinationLifecycleError> {
        if roster.len() > MAX_SUPPORTERS || strata.len() > MAX_SUPPORTERS {
            return Err(FixtureCombinationLifecycleError::CapacityExceeded);
        }
        let principal =
            FixturePrincipalState::seed(roster, principal_parameters, minimum, batch, strata)
                .map_err(|source| FixtureCombinationLifecycleError::Principal { source })?;
        let mut sources = [None; MAX_SUPPORTERS];
        for (index, label) in roster.iter().copied().enumerate() {
            *sources
                .get_mut(index)
                .ok_or(FixtureCombinationLifecycleError::Invariant)? =
                Some(FixtureCombinationSourceState::new(label, 0, 0, 0)?);
        }
        Ok(Self {
            inner: Inner {
                principal,
                parameters: combination_parameters,
                sources,
                len: roster.len(),
                emitted_rounds: 0,
            },
        })
    }

    /// Consume one exact-next inert reachability tick for one registered source.
    ///
    /// Both successful and unsuccessful updates increment maturity, saturating at
    /// eight while the watermark continues. No principal observation/pending
    /// credit, penalty countdown or emission changes. Failure changes nothing.
    /// # Errors
    /// Reject unknown labels, replay/skipped events, exhaustion or validation disagreement.
    pub fn record_reachability(
        &mut self,
        update: FixtureReachabilityUpdate,
    ) -> Result<(), FixtureCombinationLifecycleError> {
        let (index, source) = {
            let raw = RawEventAdmission {
                value: EventInput {
                    inner: &self.inner,
                    update,
                },
            };
            let validation = raw.validate();
            let admitted = admit_event(&raw.value)?;
            validation.map_err(|_| FixtureCombinationLifecycleError::ValidationInvariant)?;
            admitted
        };
        let maturity = source
            .reachability_updates
            .saturating_add(1)
            .min(MATURE_UPDATES);
        let next = FixtureCombinationSourceState::new(
            source.source_id,
            maturity,
            source.penalty,
            update.ordinal,
        )?;
        replace_source(&mut self.inner, index, next)
    }

    /// Stage a real principal transition, compatibility filtering and one estimate.
    ///
    /// All metadata is admitted before penalty/filter effects, including records
    /// that will be excluded. Only a genuine ready decision runs compatibility,
    /// decrements penalties or emits a new ordinal. Every failure preserves the
    /// complete prior owner, including credit and event/observation watermarks.
    /// # Errors
    /// Preserve actual principal/admission/arithmetic/reduction causes; reject
    /// capacity, counter exhaustion and inconsistent output before the sole commit.
    pub fn advance<'batch>(
        &mut self,
        batch: &'batch FixtureProjectedBatch<'_>,
        strata: &[FixturePrincipalStratum],
        uncertainties: &[FixtureSupporterUncertainty<'_>],
    ) -> Result<FixtureCombinationOutcome<'batch>, FixtureCombinationLifecycleError> {
        self.advance_mode(batch, strata, SupporterMode::Legacy(uncertainties))
            .map_err(|error| match error {
                FixtureTrackingLifecycleError::Lifecycle { source } => source,
                // Legacy mode never derives attachments; keep contradictory state closed.
                FixtureTrackingLifecycleError::MissingTracking { .. } => {
                    FixtureCombinationLifecycleError::Invariant
                }
            })
    }

    /// Stage one original transaction under a single explicit metadata source.
    ///
    /// The derived holder lives only inside the receiving/reduction block and
    /// drops before the genuine decision moves into the output. Complete witness
    /// metadata precedes scoring/filtering. Existing outside-witness resets remain;
    /// only ready decisions evaluate compatibility, countdown and new emissions.
    /// Every failure leaves the entire original owner unchanged.
    /// # Errors
    /// Preserve the first capacity, principal, metadata, compatibility or commit guard.
    fn advance_mode<'batch>(
        &mut self,
        batch: &'batch FixtureProjectedBatch<'_>,
        strata: &[FixturePrincipalStratum],
        mode: SupporterMode<'_, '_>,
    ) -> Result<FixtureCombinationOutcome<'batch>, FixtureTrackingLifecycleError> {
        if strata.len() > MAX_SUPPORTERS
            || matches!(mode, SupporterMode::Legacy(records) if records.len() > MAX_SUPPORTERS)
        {
            return Err(FixtureCombinationLifecycleError::CapacityExceeded.into());
        }
        let previous_principal = self.inner.principal.principal();
        let previous_rounds = self.inner.emitted_rounds;
        let mut staged = self.inner.clone();
        let decision = staged
            .principal
            .advance(batch, strata)
            .map_err(|source| FixtureCombinationLifecycleError::Principal { source })?;
        let (combined, round, statuses) = {
            let tracking = match mode {
                SupporterMode::Legacy(_) => None,
                SupporterMode::Tracking => Some(TrackingSupporters::from_decision(&decision)?),
            };
            let admitted = match mode {
                SupporterMode::Legacy(records) => admit_lifecycle_supporters(&decision, records),
                SupporterMode::Tracking => admit_lifecycle_source(
                    &decision,
                    RecordSource::Tracking(
                        tracking
                            .as_ref()
                            .ok_or(FixtureCombinationLifecycleError::Invariant)?,
                    ),
                ),
            }
            .map_err(|source| FixtureCombinationLifecycleError::Combination { source })?;
            let mut statuses = prepare_scoring(&mut staged, &decision)?;
            if decision.kind() == FixturePrincipalOutcomeKind::ReferenceReady {
                if decision.principal_label() != previous_principal {
                    clear_penalties(&mut staged)?;
                }
                let admitted = admitted
                    .as_ref()
                    .ok_or(FixtureCombinationLifecycleError::Invariant)?;
                let mask = contributor_mask(&mut staged, admitted, &mut statuses)?;
                let estimate = combine_lifecycle_supporters(admitted, mask)
                    .map_err(|source| FixtureCombinationLifecycleError::Combination { source })?;
                let round = checked_next(staged.emitted_rounds)?;
                staged.emitted_rounds = round.get();
                (Some(estimate), Some(round), statuses)
            } else {
                (None, None, statuses)
            }
        };
        let outcome = complete_outcome(
            decision,
            combined,
            round,
            statuses,
            &staged,
            previous_rounds,
        )?;
        self.inner = staged;
        Ok(outcome)
    }

    /// Advance using tracking derived only from the genuine batch's actual entries.
    ///
    /// No metadata override or detached report is accepted. Every original
    /// witnessed attachment is admitted before filtering, including a later-excluded
    /// participant and retained decisions. Shared principal credit, outside-witness
    /// resets and ready-only compatibility/countdown behavior remain unchanged.
    /// Only the final checked outcome commits state; no clock or network effect occurs.
    /// # Errors
    /// Preserve the first missing attachment or actual staged lifecycle failure;
    /// rejection leaves the complete owner and supplied batch/strata unchanged.
    pub fn advance_tracking<'batch>(
        &mut self,
        batch: &'batch FixtureProjectedBatch<'_>,
        strata: &[FixturePrincipalStratum],
    ) -> Result<FixtureCombinationOutcome<'batch>, FixtureTrackingLifecycleError> {
        self.advance_mode(batch, strata, SupporterMode::Tracking)
    }

    /// Copy this fixture instance's committed emission count without consuming it.
    #[must_use]
    pub const fn emitted_rounds(&self) -> u64 {
        self.inner.emitted_rounds
    }
    /// Borrow one registered source's committed counters; unknown labels are absent.
    #[must_use]
    pub fn source_state(&self, source_id: NonZeroU64) -> Option<&FixtureCombinationSourceState> {
        self.inner
            .sources
            .iter()
            .take(self.inner.len)
            .flatten()
            .find(|source| source.source_id == source_id)
    }
    /// Borrow the real principal state without permitting mutation through this owner.
    #[must_use]
    pub const fn principal_state(&self) -> &FixturePrincipalState {
        &self.inner.principal
    }
    /// Copy the unchanged explicit combination parameters without supplying defaults.
    #[must_use]
    pub const fn parameters(&self) -> FixtureCombinationParameters {
        self.inner.parameters
    }
}

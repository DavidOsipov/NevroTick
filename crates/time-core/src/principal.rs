// SPDX-License-Identifier: GPL-2.0-only
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2011-2016, 2018, 2020-2025
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of version 2 of the GNU General Public License as
 * published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 *
 **********************************************************************
*/
//! Fixed-roster fixture principal contracts without clock or freshness authority.
//!
//! Modified by David Osipov with AI assistance, 31 October 2026. This adapts
//! reviewed principal scoring/update-credit behavior from Chrony 4.9 `sources.c`,
//! revision `8df4f1263e206585e6c4e61352d042c2dd12916a`, selected ranges recorded
//! in the existing source ledger and crate provenance. A fixed ordered roster,
//! checked fixture event schedule and the existing common-witness selector
//! replace upstream global source ownership and eligibility policy.
//!
//! Parameters, strata and complete receiver relations have mandatory checked
//! construction. State transitions retain at most sixteen registered labels, one
//! observation watermark and pending credit per label, and original projection
//! metadata. A cached absent-source distance preserves scoring-before-replacement
//! order only: unlike Chrony's registered-source refresh, it establishes no aging
//! parity. Labels, mapping tags and observation times remain fixture assertions.
//! No network, ambient clock, persistence, reference update or protected effect
//! is performed. A future emitted decision consumes only inert fixture credit.

use std::{fmt, num::NonZeroU64};

use garde::Validate;
use time_domain::MonotonicNs;

use crate::{
    ArithmeticOperation, ArithmeticStage, FitError, FixtureMinimum, FixtureOutcomeKind,
    FixtureProjectedBatch, FixtureProjectedInterval, FixtureSelection, SelectionError,
    regression::arithmetic,
};

/// Existing fixture population bound, also limiting private state and scratch.
const MAX_PRINCIPAL_SOURCES: usize = 16;

/// Pinned source mechanism threshold, not a product configuration default.
const SCORE_LIMIT: f64 = 10.0;

/// Explicit parameter whose shared scalar rule rejected a raw value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixturePrincipalParameterField {
    /// Positive NTP reselection distance in seconds; no default is supplied.
    ReselectDistanceSeconds,
    /// Nonnegative per-stratum distance penalty in seconds.
    StratumWeightSeconds,
}

/// First scalar failure, before any rejected value can be retained or formatted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixturePrincipalScalarIssue {
    /// The value is NaN or either infinity.
    NonFinite,
    /// The nonzero value is subnormal, checked before its sign.
    Subnormal,
    /// The finite normal value is negative.
    Negative,
    /// Either signed zero was supplied for the strictly positive parameter.
    NonPositive,
}

/// One checked arithmetic role in the intended principal transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixturePrincipalStep {
    /// Multiply a signed exact stratum difference by the supplied weight.
    StratumPenalty,
    /// Add a stratum penalty to the original or cached root distance.
    AdjustedDistance,
    /// Add the NTP reselection penalty to a survivor's denominator.
    CandidateDistance,
    /// Divide one by a candidate distance when no principal exists.
    InitialScore,
    /// Divide the principal numerator by a candidate denominator.
    ScoreRatio,
    /// Multiply an existing score by its separately checked ratio.
    UpdatedScore,
}

/// Closed context field whose asserted identity differs from the state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixturePrincipalContextField {
    /// The asserted coordinate frame differs.
    Frame,
    /// The asserted fixture-mapping generation differs.
    MappingGeneration,
}

/// Bounded typed failures, with no raw label, instant, float or garde report.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum FixturePrincipalError {
    /// A raw parameter failed its sole shared admission rule.
    InvalidParameter {
        /// Closed parameter role in constructor order.
        field: FixturePrincipalParameterField,
        /// First scalar failure for that role.
        issue: FixturePrincipalScalarIssue,
    },
    /// A raw asserted NTP stratum is outside 1..=15.
    InvalidStratum,
    /// A fixed roster cannot be empty.
    EmptyRoster,
    /// More than sixteen roster labels were supplied.
    RosterCapacityExceeded,
    /// The first later roster occurrence repeated an earlier label.
    DuplicateRoster {
        /// First later duplicate position, bounded by the receiving owner.
        index: usize,
    },
    /// More than sixteen stratum mapping items were supplied.
    MetadataCapacityExceeded,
    /// Stratum mapping count differs from the complete current batch.
    MetadataCountMismatch,
    /// A current entry or metadata item names a label outside the fixed roster.
    UnknownSource {
        /// First offending position, bounded by the receiving owner.
        index: usize,
    },
    /// The first later metadata item repeated an earlier label.
    DuplicateMetadata {
        /// First later duplicate position, bounded by the receiving owner.
        index: usize,
    },
    /// A current projected entry lacks its required stratum mapping.
    MissingMetadata {
        /// Current batch position with no mapping, bounded below sixteen.
        index: usize,
    },
    /// One expected asserted state context field differs.
    ContextMismatch {
        /// Closed mismatching field without either actual context value.
        field: FixturePrincipalContextField,
    },
    /// Evaluation moved before the last accepted evaluation instant.
    BackwardEvaluation,
    /// A present source moved before its saved latest observation watermark.
    BackwardObservation {
        /// Offending current batch position, bounded below sixteen.
        index: usize,
    },
    /// More than one source advanced its watermark in one serialized event.
    MultipleObservations,
    /// A source changed stratum while keeping the same observation watermark.
    StratumWithoutObservation {
        /// Offending current batch position, bounded below sixteen.
        index: usize,
    },
    /// The actual delegated interval selector rejected its internal invariant.
    Selection {
        /// Actual typed selection cause, preserved without raw data.
        source: SelectionError,
    },
    /// A separately checked numeric primitive could not produce supported output.
    Arithmetic {
        /// Closed principal arithmetic role.
        step: FixturePrincipalStep,
        /// Actual bounded numeric owner cause.
        source: FitError,
    },
    /// Scaffold behavior or a completed private representation is unavailable.
    Invariant,
}

impl fmt::Debug for FixturePrincipalError {
    /// Emit a fixed type name, including for externally forged public metadata.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixturePrincipalError { .. }")
    }
}

impl fmt::Display for FixturePrincipalError {
    /// Format closed useful failure families without raw data or cause formatting.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidParameter { .. } => "fixture principal: invalid parameter",
            Self::InvalidStratum => "fixture principal: invalid stratum",
            Self::EmptyRoster => "fixture principal: empty roster",
            Self::RosterCapacityExceeded => "fixture principal: roster capacity",
            Self::DuplicateRoster { .. } => "fixture principal: duplicate roster label",
            Self::MetadataCapacityExceeded => "fixture principal: metadata capacity",
            Self::MetadataCountMismatch => "fixture principal: metadata count mismatch",
            Self::UnknownSource { .. } => "fixture principal: unknown source",
            Self::DuplicateMetadata { .. } => "fixture principal: duplicate metadata",
            Self::MissingMetadata { .. } => "fixture principal: missing metadata",
            Self::ContextMismatch { .. } => "fixture principal: context mismatch",
            Self::BackwardEvaluation => "fixture principal: backward evaluation",
            Self::BackwardObservation { .. } => "fixture principal: backward observation",
            Self::MultipleObservations => "fixture principal: multiple observations",
            Self::StratumWithoutObservation { .. } => {
                "fixture principal: stratum changed without observation"
            }
            Self::Selection { .. } => "fixture principal: selection failed",
            Self::Arithmetic { .. } => "fixture principal: arithmetic failed",
            Self::Invariant => "fixture principal: invariant",
        })
    }
}

impl std::error::Error for FixturePrincipalError {
    /// Borrow only actual typed delegated causes, never a retained raw report.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Selection { source } => Some(source),
            Self::Arithmetic { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Apply the sole parameter predicate in finite, subnormal, sign, positivity order.
fn parameter_issue(
    value: f64,
    field: FixturePrincipalParameterField,
) -> Option<FixturePrincipalScalarIssue> {
    if !value.is_finite() {
        Some(FixturePrincipalScalarIssue::NonFinite)
    } else if value.is_subnormal() {
        Some(FixturePrincipalScalarIssue::Subnormal)
    } else if value < 0.0 {
        Some(FixturePrincipalScalarIssue::Negative)
    } else if field == FixturePrincipalParameterField::ReselectDistanceSeconds && value == 0.0 {
        Some(FixturePrincipalScalarIssue::NonPositive)
    } else {
        None
    }
}

/// Adapt the sole reselection-distance predicate to garde without retaining input.
/// # Errors
/// Return one fixed error for an unsupported value, with no state or effects.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rules borrow their raw scalar field and context"
)]
fn reselect_rule(value: &f64, (): &()) -> garde::Result {
    if parameter_issue(
        *value,
        FixturePrincipalParameterField::ReselectDistanceSeconds,
    )
    .is_none()
    {
        Ok(())
    } else {
        Err(garde::Error::new("invalid fixture reselection distance"))
    }
}

/// Adapt the sole stratum-weight predicate to garde without retaining input.
/// # Errors
/// Return one fixed error for an unsupported value, with no state or effects.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rules borrow their raw scalar field and context"
)]
fn stratum_weight_rule(value: &f64, (): &()) -> garde::Result {
    if parameter_issue(*value, FixturePrincipalParameterField::StratumWeightSeconds).is_none() {
        Ok(())
    } else {
        Err(garde::Error::new("invalid fixture stratum weight"))
    }
}

/// Exactly two raw parameters, each covered by its maintained shared garde rule.
#[derive(Validate)]
struct RawParameters {
    /// Raw distance that must be positive finite normal.
    #[garde(custom(reselect_rule))]
    reselect_distance_seconds: f64,
    /// Raw penalty that must be nonnegative finite normal or signed zero.
    #[garde(custom(stratum_weight_rule))]
    stratum_weight_seconds: f64,
}

/// Two immutable checked fixture parameters, retaining their exact supplied bits.
#[derive(Clone, Copy)]
pub struct FixturePrincipalParameters {
    /// Positive finite normal NTP reselection distance in seconds.
    reselect_distance_seconds: f64,
    /// Nonnegative finite normal or signed-zero per-stratum penalty in seconds.
    stratum_weight_seconds: f64,
}

impl fmt::Debug for FixturePrincipalParameters {
    /// Format the type only, without parameter values or signed-zero information.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixturePrincipalParameters { .. }")
    }
}

impl PartialEq for FixturePrincipalParameters {
    /// Compare all admitted parameter bits without conflating the two signed zeros.
    fn eq(&self, other: &Self) -> bool {
        self.reselect_distance_seconds.to_bits() == other.reselect_distance_seconds.to_bits()
            && self.stratum_weight_seconds.to_bits() == other.stratum_weight_seconds.to_bits()
    }
}

impl FixturePrincipalParameters {
    /// Mandatorily validate both raw fields before private immutable construction.
    ///
    /// Work and raw report fields are fixed at two. The temporary garde report
    /// is discarded; no state, external effect or implicit default is supplied.
    /// # Errors
    /// Report the first parameter failure in reselection-distance, stratum-weight
    /// order; otherwise an unexplained garde rejection returns `Invariant`.
    pub fn new(
        reselect_distance_seconds: f64,
        stratum_weight_seconds: f64,
    ) -> Result<Self, FixturePrincipalError> {
        let raw = RawParameters {
            reselect_distance_seconds,
            stratum_weight_seconds,
        };
        let validated = raw.validate();
        for (value, field) in [
            (
                raw.reselect_distance_seconds,
                FixturePrincipalParameterField::ReselectDistanceSeconds,
            ),
            (
                raw.stratum_weight_seconds,
                FixturePrincipalParameterField::StratumWeightSeconds,
            ),
        ] {
            if let Some(issue) = parameter_issue(value, field) {
                return Err(FixturePrincipalError::InvalidParameter { field, issue });
            }
        }
        validated.map_err(|_| FixturePrincipalError::Invariant)?;
        Ok(Self {
            reselect_distance_seconds: raw.reselect_distance_seconds,
            stratum_weight_seconds: raw.stratum_weight_seconds,
        })
    }

    /// Copy the unchanged checked positive reselection distance, without effects.
    #[must_use]
    pub const fn reselect_distance_seconds(&self) -> f64 {
        self.reselect_distance_seconds
    }

    /// Copy the unchanged checked stratum penalty, preserving signed-zero bits.
    #[must_use]
    pub const fn stratum_weight_seconds(&self) -> f64 {
        self.stratum_weight_seconds
    }
}

/// Sole raw stratum field, covered by the actual inclusive 1..=15 garde rule.
#[derive(Validate)]
struct RawStratum {
    /// Caller-asserted fixture stratum, not authenticated protocol evidence.
    #[garde(range(min = 1, max = 15))]
    value: u8,
}

/// Immutable asserted source-to-stratum mapping with no eligibility authority.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct FixturePrincipalStratum {
    /// Nonzero caller-asserted source label, checked by its existing integer type.
    source_id: NonZeroU64,
    /// Mandatory-garde-admitted fixture stratum in 1..=15.
    stratum: u8,
}

impl fmt::Debug for FixturePrincipalStratum {
    /// Format only the type, without label or asserted stratum.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixturePrincipalStratum { .. }")
    }
}

impl FixturePrincipalStratum {
    /// Validate the raw stratum before retaining its already-nonzero source label.
    ///
    /// Validation has one fixed raw field; its report is discarded. The source
    /// is an assertion, not proof of identity, independence, freshness or trust.
    /// # Errors
    /// Reject raw strata outside 1..=15 without producing a mapping or effects.
    pub fn new(source_id: NonZeroU64, stratum: u8) -> Result<Self, FixturePrincipalError> {
        let raw = RawStratum { value: stratum };
        raw.validate()
            .map_err(|_| FixturePrincipalError::InvalidStratum)?;
        Ok(Self {
            source_id,
            stratum: raw.value,
        })
    }

    /// Copy the unchanged source assertion without creating identity authority.
    #[must_use]
    pub const fn source_id(&self) -> NonZeroU64 {
        self.source_id
    }

    /// Copy the checked fixture stratum without protocol or freshness claims.
    #[must_use]
    pub const fn stratum(&self) -> u8 {
        self.stratum
    }
}

/// Read-only state of one registered source; only the principal owner constructs it.
#[derive(Clone)]
pub struct FixturePrincipalSourceState {
    /// Fixed roster label, without independent real-source provenance.
    source_id: NonZeroU64,
    /// Supported finite score; initial inverse-distance scores may be below one.
    score: f64,
    /// Exact latest admitted observation watermark, absent before first sighting.
    latest_observed_at: Option<MonotonicNs>,
    /// Unconsumed inert fixture observation credit, never a freshness assertion.
    pending: bool,
    /// Last original projected distance, retained during registered absence.
    distance_seconds: Option<f64>,
    /// Last checked asserted stratum, retained during registered absence.
    stratum: Option<u8>,
}

impl fmt::Debug for FixturePrincipalSourceState {
    /// Format the type only, without keys, observation times, scores or credit.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixturePrincipalSourceState { .. }")
    }
}

impl PartialEq for FixturePrincipalSourceState {
    /// Compare every retained field, using exact bits for floating-point state.
    fn eq(&self, other: &Self) -> bool {
        self.source_id == other.source_id
            && self.score.to_bits() == other.score.to_bits()
            && self.latest_observed_at == other.latest_observed_at
            && self.pending == other.pending
            && self.distance_seconds.map(f64::to_bits) == other.distance_seconds.map(f64::to_bits)
            && self.stratum == other.stratum
    }
}

impl FixturePrincipalSourceState {
    /// Copy the fixed source assertion without creating new authority.
    #[must_use]
    pub const fn source_id(&self) -> NonZeroU64 {
        self.source_id
    }

    /// Copy the last committed score without updating it or consuming credit.
    #[must_use]
    pub const fn score(&self) -> f64 {
        self.score
    }

    /// Copy the original observation watermark without refreshing its age.
    #[must_use]
    pub const fn latest_observed_at(&self) -> Option<MonotonicNs> {
        self.latest_observed_at
    }

    /// Observe unconsumed fixture credit without granting freshness or an effect.
    #[must_use]
    pub const fn pending(&self) -> bool {
        self.pending
    }

    /// Copy the last original projection distance; absence does not age this cache.
    #[must_use]
    pub const fn distance_seconds(&self) -> Option<f64> {
        self.distance_seconds
    }

    /// Copy the last checked fixture stratum, absent before first sighting.
    #[must_use]
    pub const fn stratum(&self) -> Option<u8> {
        self.stratum
    }
}

/// Fixed-roster principal state with one exclusive, failure-atomic transition owner.
///
/// Only the seed/advance owners may establish or replace these private fields.
/// Cloning preserves exact inert state and grants no additional source freshness,
/// generation authority, observation or external effect. Equality compares every
/// retained field, including exact float bits, to expose rejected-state changes.
#[derive(Clone)]
pub struct FixturePrincipalState {
    /// Same-frame assertion fixed for this fixture-state lifetime.
    frame: NonZeroU64,
    /// Same-mapping assertion fixed for this fixture-state lifetime.
    mapping_generation: NonZeroU64,
    /// Explicit fixed parameters; changing them requires a new state.
    parameters: FixturePrincipalParameters,
    /// Independently checked fixed minimum, never reduced after source loss.
    minimum: FixtureMinimum,
    /// Fixed ordered occupied roster followed only by empty slots.
    sources: [Option<FixturePrincipalSourceState>; MAX_PRINCIPAL_SOURCES],
    /// Occupied source prefix, bounded in 1..=16 by the intended seed owner.
    roster_len: usize,
    /// Current principal label, or none after unavailable/waiting outcomes.
    principal: Option<NonZeroU64>,
    /// Last accepted evaluation instant, with no ambient clock certification.
    evaluated_at: MonotonicNs,
}

impl fmt::Debug for FixturePrincipalState {
    /// Format only the type, without source state, labels, times or parameters.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixturePrincipalState { .. }")
    }
}

impl PartialEq for FixturePrincipalState {
    /// Compare complete retained state, preserving signed-zero differences.
    fn eq(&self, other: &Self) -> bool {
        self.frame == other.frame
            && self.mapping_generation == other.mapping_generation
            && self.parameters == other.parameters
            && self.minimum.get() == other.minimum.get()
            && self.sources == other.sources
            && self.roster_len == other.roster_len
            && self.principal == other.principal
            && self.evaluated_at == other.evaluated_at
    }
}

/// One bounded raw roster field, covered by its complete shared relationship rule.
#[derive(Validate)]
struct RosterCandidate<'a> {
    /// Complete source order; validation owns emptiness, capacity and duplication.
    #[garde(custom(roster_rule))]
    values: &'a [NonZeroU64],
}

/// Find the first roster failure without reading an over-capacity element.
fn roster_issue(values: &[NonZeroU64]) -> Option<FixturePrincipalError> {
    if values.is_empty() {
        return Some(FixturePrincipalError::EmptyRoster);
    }
    if values.len() > MAX_PRINCIPAL_SOURCES {
        return Some(FixturePrincipalError::RosterCapacityExceeded);
    }
    for (index, value) in values.iter().enumerate() {
        if values.iter().take(index).any(|earlier| earlier == value) {
            return Some(FixturePrincipalError::DuplicateRoster { index });
        }
    }
    None
}

/// Apply the sole roster relationship rule inside mandatory garde admission.
/// # Errors
/// Return a fixed redacted failure; no input, state or effect is changed.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rules borrow their raw field and context"
)]
fn roster_rule(values: &[NonZeroU64], (): &()) -> garde::Result {
    if roster_issue(values).is_none() {
        Ok(())
    } else {
        Err(garde::Error::new("invalid fixture principal roster"))
    }
}

/// Admit a whole raw roster before any indexed source construction.
/// # Errors
/// Preserve the first bounded roster issue or an unexplained garde invariant.
fn admit_roster(values: &[NonZeroU64]) -> Result<(), FixturePrincipalError> {
    if values.len() > MAX_PRINCIPAL_SOURCES {
        return Err(FixturePrincipalError::RosterCapacityExceeded);
    }
    let candidate = RosterCandidate { values };
    let validated = candidate.validate();
    if let Some(error) = roster_issue(values) {
        return Err(error);
    }
    validated.map_err(|_| FixturePrincipalError::Invariant)
}

/// Complete borrowed receiver relationships; every field belongs to one rule.
#[derive(Clone, Copy)]
struct ReceiverInput<'a> {
    /// Original immutable projected entries, never copied into state.
    entries: &'a [FixtureProjectedInterval],
    /// Complete checked stratum mappings for exactly these entries.
    strata: &'a [FixturePrincipalStratum],
    /// Original checked batch's frame assertion.
    frame: NonZeroU64,
    /// Original checked batch's fixture-mapping assertion.
    mapping_generation: NonZeroU64,
    /// Original exact common evaluation instant.
    at: MonotonicNs,
    /// Private seed path admits all initial observations as preexisting credit.
    seed: bool,
}

/// The single contextual garde field covers the entire receiving relationship.
#[derive(Validate)]
#[garde(context(FixturePrincipalState))]
struct ReceiverCandidate<'a> {
    /// No skipped relationship: the shared rule examines the whole input.
    #[garde(custom(receiver_rule))]
    input: ReceiverInput<'a>,
}

/// Locate one occupied roster position, bounded by sixteen asserted labels.
fn source_index(state: &FixturePrincipalState, label: NonZeroU64) -> Option<usize> {
    state
        .sources
        .iter()
        .take(state.roster_len)
        .position(|source| {
            source
                .as_ref()
                .is_some_and(|source| source.source_id == label)
        })
}

/// Borrow a source only from a completed occupied slot.
/// # Errors
/// Reject an impossible missing or out-of-prefix slot without changing state.
fn source_at(
    state: &FixturePrincipalState,
    index: usize,
) -> Result<&FixturePrincipalSourceState, FixturePrincipalError> {
    if index >= state.roster_len {
        return Err(FixturePrincipalError::Invariant);
    }
    state
        .sources
        .get(index)
        .and_then(Option::as_ref)
        .ok_or(FixturePrincipalError::Invariant)
}

/// Find one checked stratum by asserted label in an already bounded mapping.
fn mapped_stratum(input: &ReceiverInput<'_>, label: NonZeroU64) -> Option<u8> {
    input
        .strata
        .iter()
        .find(|stratum| stratum.source_id == label)
        .map(|stratum| stratum.stratum)
}

/// Inspect metadata coverage and uniqueness after both slice capacities pass.
fn metadata_issue(
    input: &ReceiverInput<'_>,
    state: &FixturePrincipalState,
) -> Option<FixturePrincipalError> {
    for (index, stratum) in input.strata.iter().enumerate() {
        if source_index(state, stratum.source_id).is_none() {
            return Some(FixturePrincipalError::UnknownSource { index });
        }
        if input
            .strata
            .iter()
            .take(index)
            .any(|earlier| earlier.source_id == stratum.source_id)
        {
            return Some(FixturePrincipalError::DuplicateMetadata { index });
        }
    }
    for (index, entry) in input.entries.iter().enumerate() {
        let label = entry.projection().context().source_id();
        if source_index(state, label).is_none() {
            return Some(FixturePrincipalError::UnknownSource { index });
        }
        if mapped_stratum(input, label).is_none() {
            return Some(FixturePrincipalError::MissingMetadata { index });
        }
    }
    None
}

/// Check exact observation/stratum evolution without trusting an update flag.
fn event_issue(
    input: &ReceiverInput<'_>,
    state: &FixturePrincipalState,
) -> Option<FixturePrincipalError> {
    if input.seed {
        return None;
    }
    let mut changed = false;
    for (index, entry) in input.entries.iter().enumerate() {
        let projection = entry.projection();
        let label = projection.context().source_id();
        let Some(source) = state.source_state(label) else {
            return Some(FixturePrincipalError::Invariant);
        };
        let latest = projection.latest_observed_at();
        if source
            .latest_observed_at
            .is_some_and(|previous| latest < previous)
        {
            return Some(FixturePrincipalError::BackwardObservation { index });
        }
        if source.latest_observed_at == Some(latest) {
            if mapped_stratum(input, label) != source.stratum {
                return Some(FixturePrincipalError::StratumWithoutObservation { index });
            }
        } else if changed {
            return Some(FixturePrincipalError::MultipleObservations);
        } else {
            changed = true;
        }
    }
    None
}

/// Find the first receiver failure in capacity, context, metadata, event order.
fn receiver_issue(
    input: &ReceiverInput<'_>,
    state: &FixturePrincipalState,
) -> Option<FixturePrincipalError> {
    if input.entries.len() > MAX_PRINCIPAL_SOURCES {
        return Some(FixturePrincipalError::Invariant);
    }
    if input.strata.len() > MAX_PRINCIPAL_SOURCES {
        return Some(FixturePrincipalError::MetadataCapacityExceeded);
    }
    if input.strata.len() != input.entries.len() {
        return Some(FixturePrincipalError::MetadataCountMismatch);
    }
    if input.frame != state.frame {
        return Some(FixturePrincipalError::ContextMismatch {
            field: FixturePrincipalContextField::Frame,
        });
    }
    if input.mapping_generation != state.mapping_generation {
        return Some(FixturePrincipalError::ContextMismatch {
            field: FixturePrincipalContextField::MappingGeneration,
        });
    }
    if input.at < state.evaluated_at {
        return Some(FixturePrincipalError::BackwardEvaluation);
    }
    metadata_issue(input, state).or_else(|| event_issue(input, state))
}

/// Run the complete shared receiver rule through mandatory contextual garde.
/// # Errors
/// Return one fixed failure without exposing its rejected relationships.
fn receiver_rule(input: &ReceiverInput<'_>, state: &FixturePrincipalState) -> garde::Result {
    if receiver_issue(input, state).is_none() {
        Ok(())
    } else {
        Err(garde::Error::new("invalid fixture principal receiver"))
    }
}

/// Admit all borrowed relationships without mutation or retaining a raw report.
/// # Errors
/// Capacity rejects before element reads; other failures preserve typed order.
fn admit_receiver(
    input: ReceiverInput<'_>,
    state: &FixturePrincipalState,
) -> Result<(), FixturePrincipalError> {
    if input.strata.len() > MAX_PRINCIPAL_SOURCES {
        return Err(FixturePrincipalError::MetadataCapacityExceeded);
    }
    if input.entries.len() > MAX_PRINCIPAL_SOURCES {
        return Err(FixturePrincipalError::Invariant);
    }
    let candidate = ReceiverCandidate { input };
    let validated = candidate.validate_with(state);
    if let Some(error) = receiver_issue(&input, state) {
        return Err(error);
    }
    validated.map_err(|_| FixturePrincipalError::Invariant)
}

/// Stage original metadata and at most one already-admitted changed observation.
///
/// The caller owns this unpublished state. Absence leaves cached source metadata
/// and pending credit intact; projection alone never creates another observation.
/// # Errors
/// Reject an impossible missing checked lookup without publishing partial state.
fn stage_entries(
    input: &ReceiverInput<'_>,
    state: &mut FixturePrincipalState,
) -> Result<Option<NonZeroU64>, FixturePrincipalError> {
    let mut updated = None;
    for entry in input.entries {
        let projection = entry.projection();
        let label = projection.context().source_id();
        let index = source_index(state, label).ok_or(FixturePrincipalError::Invariant)?;
        let stratum = mapped_stratum(input, label).ok_or(FixturePrincipalError::Invariant)?;
        let source = state
            .sources
            .get_mut(index)
            .and_then(Option::as_mut)
            .ok_or(FixturePrincipalError::Invariant)?;
        let latest = projection.latest_observed_at();
        if input.seed || source.latest_observed_at != Some(latest) {
            source.pending = true;
            if !input.seed {
                if updated.is_some() {
                    return Err(FixturePrincipalError::Invariant);
                }
                updated = Some(label);
            }
        }
        source.latest_observed_at = Some(latest);
        source.distance_seconds = Some(projection.distance_seconds());
        source.stratum = Some(stratum);
    }
    state.evaluated_at = input.at;
    Ok(updated)
}

/// Delegate one numeric primitive to the sole shared supported-arithmetic owner.
/// # Errors
/// Preserve the actual numeric cause under its principal step and stage.
fn principal_arithmetic(
    left: f64,
    right: f64,
    operation: ArithmeticOperation,
    step: FixturePrincipalStep,
) -> Result<f64, FixturePrincipalError> {
    arithmetic(left, right, operation, ArithmeticStage::Principal)
        .map_err(|source| FixturePrincipalError::Arithmetic { step, source })
}

/// Compute a signed stratum-adjusted original or cached distance in two primitives.
/// # Errors
/// Reject absent metadata or the first unsupported multiply/add intermediate.
fn adjusted_distance(
    source: &FixturePrincipalSourceState,
    minimum_stratum: u8,
    weight: f64,
) -> Result<f64, FixturePrincipalError> {
    let stratum = source.stratum.ok_or(FixturePrincipalError::Invariant)?;
    let distance = source
        .distance_seconds
        .ok_or(FixturePrincipalError::Invariant)?;
    let difference = i16::from(stratum) - i16::from(minimum_stratum);
    let penalty = principal_arithmetic(
        f64::from(difference),
        weight,
        ArithmeticOperation::Multiply,
        FixturePrincipalStep::StratumPenalty,
    )?;
    principal_arithmetic(
        distance,
        penalty,
        ArithmeticOperation::Add,
        FixturePrincipalStep::AdjustedDistance,
    )
}

/// Checked current survivor membership and minimum asserted stratum.
struct SurvivorSet {
    /// Roster-aligned membership flags; unused suffix remains false.
    members: [bool; MAX_PRINCIPAL_SOURCES],
    /// Actual minimum stratum among the real witness supporters.
    minimum_stratum: u8,
}

/// Bind actual selection supporters to this same fixed roster's staged metadata.
/// # Errors
/// Reject missing witness, unknown supporters or absent committed stratum data.
fn survivor_set(
    state: &FixturePrincipalState,
    selection: &FixtureSelection,
) -> Result<SurvivorSet, FixturePrincipalError> {
    let witness = selection
        .witnessed()
        .ok_or(FixturePrincipalError::Invariant)?;
    let mut members = [false; MAX_PRINCIPAL_SOURCES];
    let mut minimum_stratum = None;
    for label in witness.supporters() {
        let index = source_index(state, label).ok_or(FixturePrincipalError::Invariant)?;
        let stratum = source_at(state, index)?
            .stratum
            .ok_or(FixturePrincipalError::Invariant)?;
        let member = members
            .get_mut(index)
            .ok_or(FixturePrincipalError::Invariant)?;
        *member = true;
        minimum_stratum =
            Some(minimum_stratum.map_or(stratum, |previous: u8| previous.min(stratum)));
    }
    Ok(SurvivorSet {
        members,
        minimum_stratum: minimum_stratum.ok_or(FixturePrincipalError::Invariant)?,
    })
}

/// Update one survivor's score using the original single-event pair trigger.
/// # Errors
/// Reject the first unsupported distance/ratio/product before score flooring.
fn updated_score(
    source: &FixturePrincipalSourceState,
    minimum_stratum: u8,
    parameters: FixturePrincipalParameters,
    principal: Option<(NonZeroU64, f64)>,
    updated: Option<NonZeroU64>,
) -> Result<f64, FixturePrincipalError> {
    let adjusted = adjusted_distance(source, minimum_stratum, parameters.stratum_weight_seconds)?;
    let denominator = principal_arithmetic(
        adjusted,
        parameters.reselect_distance_seconds,
        ArithmeticOperation::Add,
        FixturePrincipalStep::CandidateDistance,
    )?;
    let Some((principal_label, numerator)) = principal else {
        return principal_arithmetic(
            1.0,
            denominator,
            ArithmeticOperation::Divide,
            FixturePrincipalStep::InitialScore,
        );
    };
    if updated.is_none_or(|label| label != source.source_id && label != principal_label) {
        return Ok(source.score);
    }
    let ratio = principal_arithmetic(
        numerator,
        denominator,
        ArithmeticOperation::Divide,
        FixturePrincipalStep::ScoreRatio,
    )?;
    let score = principal_arithmetic(
        source.score,
        ratio,
        ArithmeticOperation::Multiply,
        FixturePrincipalStep::UpdatedScore,
    )?;
    Ok(if score < 1.0 { 1.0 } else { score })
}

/// Update scores in stable roster order before choosing the first maximum.
/// # Errors
/// Reject missing occupied slots/metadata or the first checked numeric failure.
fn rank_sources(
    state: &mut FixturePrincipalState,
    survivors: &SurvivorSet,
    updated: Option<NonZeroU64>,
) -> Result<(usize, f64), FixturePrincipalError> {
    let principal = if let Some(label) = state.principal {
        let index = source_index(state, label).ok_or(FixturePrincipalError::Invariant)?;
        Some((
            label,
            adjusted_distance(
                source_at(state, index)?,
                survivors.minimum_stratum,
                state.parameters.stratum_weight_seconds,
            )?,
        ))
    } else {
        None
    };
    let mut maximum = None;
    let mut maximum_score = 0.0;
    for index in 0..state.roster_len {
        let source = state
            .sources
            .get_mut(index)
            .and_then(Option::as_mut)
            .ok_or(FixturePrincipalError::Invariant)?;
        if !survivors
            .members
            .get(index)
            .copied()
            .ok_or(FixturePrincipalError::Invariant)?
        {
            source.score = 1.0;
            continue;
        }
        source.score = updated_score(
            source,
            survivors.minimum_stratum,
            state.parameters,
            principal,
            updated,
        )?;
        if maximum_score < source.score {
            maximum_score = source.score;
            maximum = Some(index);
        }
    }
    Ok((
        maximum.ok_or(FixturePrincipalError::Invariant)?,
        maximum_score,
    ))
}

/// Apply pending-credit admission after source-order scoring, then consume credit.
/// # Errors
/// Reject impossible roster/member references without publishing staged changes.
fn choose_principal(
    state: &mut FixturePrincipalState,
    survivors: &SurvivorSet,
    best_index: usize,
    best_score: f64,
) -> Result<FixturePrincipalOutcomeKind, FixturePrincipalError> {
    let best = source_at(state, best_index)?;
    let best_label = best.source_id;
    let best_pending = best.pending;
    let current_is_member = if let Some(label) = state.principal {
        let index = source_index(state, label).ok_or(FixturePrincipalError::Invariant)?;
        survivors
            .members
            .get(index)
            .copied()
            .ok_or(FixturePrincipalError::Invariant)?
    } else {
        false
    };
    if !current_is_member || (state.principal != Some(best_label) && best_score > SCORE_LIMIT) {
        if !best_pending {
            state.principal = None;
            return Ok(FixturePrincipalOutcomeKind::WaitingForUpdate);
        }
        state.principal = Some(best_label);
        for source in state.sources.iter_mut().take(state.roster_len).flatten() {
            source.score = 1.0;
        }
    }
    let label = state.principal.ok_or(FixturePrincipalError::Invariant)?;
    let index = source_index(state, label).ok_or(FixturePrincipalError::Invariant)?;
    if !source_at(state, index)?.pending {
        return Ok(FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate);
    }
    for source in state.sources.iter_mut().take(state.roster_len).flatten() {
        source.pending = false;
    }
    Ok(FixturePrincipalOutcomeKind::ReferenceReady)
}

/// Produce a private same-entry decision only after its output relations hold.
/// # Errors
/// Reject any mismatched outcome, selected label/index or supporter association.
fn complete_decision(
    entries: &[FixtureProjectedInterval],
    selection: FixtureSelection,
    principal_label: Option<NonZeroU64>,
    kind: FixturePrincipalOutcomeKind,
    updated_source: Option<NonZeroU64>,
) -> Result<FixturePrincipalDecision<'_>, FixturePrincipalError> {
    let selected_kind = matches!(
        kind,
        FixturePrincipalOutcomeKind::ReferenceReady
            | FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    );
    if selected_kind != principal_label.is_some()
        || (kind == FixturePrincipalOutcomeKind::Unavailable
            && selection.outcome() == FixtureOutcomeKind::WitnessedFixture)
        || (kind != FixturePrincipalOutcomeKind::Unavailable
            && selection.outcome() != FixtureOutcomeKind::WitnessedFixture)
    {
        return Err(FixturePrincipalError::Invariant);
    }
    let principal_index = if let Some(label) = principal_label {
        let index = entries
            .iter()
            .position(|entry| entry.projection().context().source_id() == label)
            .ok_or(FixturePrincipalError::Invariant)?;
        if !selection
            .witnessed()
            .ok_or(FixturePrincipalError::Invariant)?
            .supporters()
            .any(|supporter| supporter == label)
        {
            return Err(FixturePrincipalError::Invariant);
        }
        Some(index)
    } else {
        None
    };
    Ok(FixturePrincipalDecision {
        entries,
        selection,
        principal_index,
        principal_label,
        kind,
        updated_source,
    })
}

impl FixturePrincipalState {
    /// Admit a fixed roster and complete checked seed metadata without selection.
    ///
    /// Capacity precedes indexed reads. Mandatory whole-roster and contextual
    /// garde checks establish exact coverage before original scalar metadata is
    /// staged. Existing observations become unconsumed fixture credit, scores are
    /// one and no principal is selected. All input wrappers remain borrowed only
    /// for this call; no selector, history operation or external effect occurs.
    /// # Errors
    /// Preserve first roster, metadata/context or internal representation cause;
    /// rejection produces no state and leaves all inputs unchanged.
    pub fn seed(
        roster: &[NonZeroU64],
        parameters: FixturePrincipalParameters,
        minimum: FixtureMinimum,
        batch: &FixtureProjectedBatch<'_>,
        strata: &[FixturePrincipalStratum],
    ) -> Result<Self, FixturePrincipalError> {
        admit_roster(roster)?;
        let mut state = Self {
            frame: batch.frame(),
            mapping_generation: batch.mapping_generation(),
            parameters,
            minimum,
            sources: [const { None }; MAX_PRINCIPAL_SOURCES],
            roster_len: roster.len(),
            principal: None,
            evaluated_at: batch.projected_at(),
        };
        for (index, &source_id) in roster.iter().enumerate() {
            let slot = state
                .sources
                .get_mut(index)
                .ok_or(FixturePrincipalError::Invariant)?;
            *slot = Some(FixturePrincipalSourceState {
                source_id,
                score: 1.0,
                latest_observed_at: None,
                pending: false,
                distance_seconds: None,
                stratum: None,
            });
        }
        let input = ReceiverInput {
            entries: batch.entries(),
            strata,
            frame: batch.frame(),
            mapping_generation: batch.mapping_generation(),
            at: batch.projected_at(),
            seed: true,
        };
        admit_receiver(input, &state)?;
        if stage_entries(&input, &mut state)?.is_some() {
            return Err(FixturePrincipalError::Invariant);
        }
        Ok(state)
    }

    /// Advance one actual fixture event through the real same-batch selector.
    ///
    /// Mandatory contextual garde admission derives zero or one newer original
    /// observation; reevaluation cannot create credit or multiply pair scores.
    /// One fixed state clone stages all metadata, one real selection, source-order
    /// checked arithmetic and decision construction before the sole commit. Valid
    /// unavailable/waiting results clear selection while retaining still-pending
    /// credit. No clock/reference update or other external effect is performed.
    /// # Errors
    /// Preserve the first context/metadata/event, actual selector or numeric cause.
    /// Every error leaves all prior state bits and the caller's batch unchanged.
    pub fn advance<'batch>(
        &mut self,
        batch: &'batch FixtureProjectedBatch<'_>,
        strata: &[FixturePrincipalStratum],
    ) -> Result<FixturePrincipalDecision<'batch>, FixturePrincipalError> {
        let input = ReceiverInput {
            entries: batch.entries(),
            strata,
            frame: batch.frame(),
            mapping_generation: batch.mapping_generation(),
            at: batch.projected_at(),
            seed: false,
        };
        admit_receiver(input, self)?;
        let mut staged = self.clone();
        let updated_source = stage_entries(&input, &mut staged)?;
        let selection = batch
            .select(staged.minimum)
            .map_err(|source| FixturePrincipalError::Selection { source })?;
        let kind = if selection.outcome() == FixtureOutcomeKind::WitnessedFixture {
            let survivors = survivor_set(&staged, &selection)?;
            let (best_index, best_score) = rank_sources(&mut staged, &survivors, updated_source)?;
            choose_principal(&mut staged, &survivors, best_index, best_score)?
        } else {
            if selection.witnessed().is_some() {
                return Err(FixturePrincipalError::Invariant);
            }
            staged.principal = None;
            FixturePrincipalOutcomeKind::Unavailable
        };
        let decision = complete_decision(
            batch.entries(),
            selection,
            staged.principal,
            kind,
            updated_source,
        )?;
        *self = staged;
        Ok(decision)
    }

    /// Borrow one registered source's committed state without mutation or credit.
    #[must_use]
    pub fn source_state(&self, source_id: NonZeroU64) -> Option<&FixturePrincipalSourceState> {
        self.sources
            .iter()
            .take(self.roster_len)
            .flatten()
            .find(|source| source.source_id == source_id)
    }

    /// Copy the current principal label without reporting new observation evidence.
    #[must_use]
    pub const fn principal(&self) -> Option<NonZeroU64> {
        self.principal
    }

    /// Copy the state's asserted frame without certifying a real clock origin.
    #[must_use]
    pub const fn frame(&self) -> NonZeroU64 {
        self.frame
    }

    /// Copy the state's asserted mapping generation without source-epoch claims.
    #[must_use]
    pub const fn mapping_generation(&self) -> NonZeroU64 {
        self.mapping_generation
    }

    /// Copy the last accepted evaluation instant without reading an ambient clock.
    #[must_use]
    pub const fn evaluated_at(&self) -> MonotonicNs {
        self.evaluated_at
    }

    /// Copy the fixed roster count, without examining or exposing empty padding.
    #[must_use]
    pub const fn roster_len(&self) -> usize {
        self.roster_len
    }

    /// Copy the unchanged checked explicit fixture parameters.
    #[must_use]
    pub const fn parameters(&self) -> FixturePrincipalParameters {
        self.parameters
    }

    /// Copy the unchanged checked configured minimum, without population fallback.
    #[must_use]
    pub const fn minimum(&self) -> FixtureMinimum {
        self.minimum
    }
}

/// Closed fixture decision states, none of which imply a reference/clock effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixturePrincipalOutcomeKind {
    /// The real selector supplied no usable fixture agreement.
    Unavailable,
    /// The winning replacement had no unconsumed fixture observation.
    WaitingForUpdate,
    /// The principal remained selected without a new principal estimate.
    RetainedWithoutNewEstimate,
    /// An inert principal estimate consumed pending fixture credit.
    ReferenceReady,
}

/// Immutable result bound to original batch entries and its actual selection.
///
/// Only the principal transition may construct a completed result. Its optional
/// principal index/label must identify the same original occupied entry, and
/// reference-ready credit is true exactly for `ReferenceReady`. No public raw
/// factory, mutable fields, unchecked conversion or deserialization is supplied.
pub struct FixturePrincipalDecision<'batch> {
    /// Original wrappers from the actual selected batch, without owning copies.
    entries: &'batch [FixtureProjectedInterval],
    /// Actual owned output of the same batch's real interval selector.
    selection: FixtureSelection,
    /// Checked occupied-entry index for the optional selected principal.
    principal_index: Option<usize>,
    /// Label corresponding exactly to the checked principal entry.
    principal_label: Option<NonZeroU64>,
    /// Closed outcome tied to the same completed state transition.
    kind: FixturePrincipalOutcomeKind,
    /// Sole derived newly observed label, absent for pure reevaluation.
    updated_source: Option<NonZeroU64>,
}

impl fmt::Debug for FixturePrincipalDecision<'_> {
    /// Format only the type, without entries, selection, keys, observations or credit.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixturePrincipalDecision { .. }")
    }
}

impl FixturePrincipalDecision<'_> {
    /// Copy the closed decision kind without creating another estimate.
    #[must_use]
    pub const fn kind(&self) -> FixturePrincipalOutcomeKind {
        self.kind
    }

    /// Borrow the actual same-batch interval selection without replacement.
    #[must_use]
    pub const fn selection(&self) -> &FixtureSelection {
        &self.selection
    }

    /// Borrow every original current batch entry in its supplied order.
    #[must_use]
    pub const fn entries(&self) -> &[FixtureProjectedInterval] {
        self.entries
    }

    /// Copy the optional selected label, without granting real-source authority.
    #[must_use]
    pub const fn principal_label(&self) -> Option<NonZeroU64> {
        self.principal_label
    }

    /// Borrow the checked same-batch principal entry, absent for an unselected result.
    #[must_use]
    pub fn principal_entry(&self) -> Option<&FixtureProjectedInterval> {
        self.principal_index
            .and_then(|index| self.entries.get(index))
    }

    /// Observe pure emitted-estimate credit without implying an applied effect.
    #[must_use]
    pub const fn reference_ready(&self) -> bool {
        matches!(self.kind, FixturePrincipalOutcomeKind::ReferenceReady)
    }

    /// Copy the sole derived observed source, without manufacturing an observation.
    #[must_use]
    pub const fn updated_source(&self) -> Option<NonZeroU64> {
        self.updated_source
    }
}

/// Test-only direct receiver checks live outside the production line denominator.
#[cfg(test)]
#[path = "../tests/private/principal_guards.rs"]
mod private_decision_guards;

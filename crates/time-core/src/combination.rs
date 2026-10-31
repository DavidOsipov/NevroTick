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

 =======================================================================

 The routines in this file manage the complete pool of sources that
 we might be synchronizing to.  This includes NTP sources and others
 (e.g. local reference clocks, eyeball + wristwatch etc).

 */
//! Checked, inert fixture inputs and output relations for supporter combination.
//!
//! Modified by David Osipov with AI assistance, 31 October 2026. The reviewed
//! Chrony 4.9 `sources.c` mapping pins revision
//! `8df4f1263e206585e6c4e61352d042c2dd12916a`, lines 882–886, 925–926,
//! 932–950 and 955. See `../PROVENANCE.md` and the additive Chrony ledger.
//! Mandatory scalar admission and the actual-entry receiving boundary precede
//! deterministic checked phase/rate reductions over the complete witness set.
//! A genuine singleton preserves all five source/uncertainty bit patterns.
//!
//! All inputs remain borrowed and unchanged. Capacity is sixteen before indexed
//! work; no caller-defined supporter subset, compatibility penalty, event credit,
//! reference publication, ambient clock, network or persistence effect is supplied.
//! Tracking skew is an explicit common-instant assertion, separate from projection
//! growth skew. Neither supplied uncertainty is a calibrated physical guarantee.

use std::{fmt, num::NonZeroU64, ptr};

use garde::Validate;
use time_domain::MonotonicNs;

use crate::{
    ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitError, FixtureContext,
    FixturePrincipalDecision, FixturePrincipalOutcomeKind, FixtureProjectedInterval,
    regression::{arithmetic, checked_square_root, scalar_issue},
};

/// Existing fixture participant capacity; checked before indexing caller records.
const MAX_SUPPORTERS: usize = 16;

/// Supplied uncertainty role, in its constructor's first-error order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureCombinationScalarField {
    /// Phase standard deviation at the projection instant, in seconds.
    PhaseStandardDeviation,
    /// Rate standard deviation at the projection instant, in seconds/second.
    RateStandardDeviation,
    /// Tracking skew at the projection instant, in seconds/second.
    TrackingSkew,
}

/// Shared scalar support or nonnegative-domain rejection without raw payloads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureCombinationScalarIssue {
    /// A scalar is NaN or either infinity.
    NonFinite,
    /// A nonzero scalar is subnormal.
    Subnormal,
    /// A supported scalar is negative; negative zero remains admitted.
    Negative,
    /// The shared classifier violated its documented two-issue projection.
    Unsupported,
}

/// First incompatible field of an uncertainty record's original entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureCombinationContextField {
    /// The asserted frame differs from the original principal frame.
    Frame,
    /// The asserted mapping generation differs.
    MappingGeneration,
    /// The exact common projection instant differs.
    ProjectedAt,
}

/// A multisource reciprocal's checked positive denominator role.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureCombinationDenominatorField {
    /// Original projected root distance, in seconds.
    RootDistance,
    /// Supplied rate standard deviation, in seconds/second.
    RateStandardDeviation,
    /// Supplied tracking skew, in seconds/second.
    TrackingSkew,
}

/// Actual separately evaluated primitive within the deterministic kernel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureCombinationStep {
    /// Reciprocal original root distance.
    PhaseWeight,
    /// Square of supplied rate standard deviation.
    RateVariance,
    /// Reciprocal rate variance.
    RateWeight,
    /// Square of supplied tracking skew.
    TrackingSkewVariance,
    /// Reciprocal squared tracking skew.
    TrackingSkewWeight,
    /// Ascending-label phase-weight reduction.
    PhaseWeightSum,
    /// Original phase multiplied by its phase weight.
    WeightedPhase,
    /// Ascending-label weighted-phase reduction.
    PhaseSum,
    /// Square of supplied phase standard deviation.
    PhaseVariance,
    /// Original source phase minus the original principal phase.
    PrincipalDifference,
    /// Square of the principal-centered phase difference.
    DisagreementVariance,
    /// Sum of phase variance and principal-centered disagreement.
    PhaseVarianceAndDisagreement,
    /// Phase variance/disagreement multiplied by phase weight.
    WeightedPhaseVariance,
    /// Ascending-label weighted phase-variance reduction.
    PhaseVarianceSum,
    /// Ascending-label rate-weight reduction.
    RateWeightSum,
    /// Original signed rate multiplied by its rate weight.
    WeightedRate,
    /// Ascending-label weighted-rate reduction.
    RateSum,
    /// Ascending-label reciprocal squared-skew reduction.
    TrackingSkewWeightSum,
    /// Weighted phase sum divided by phase-weight sum.
    PhaseMean,
    /// Weighted phase-variance sum divided by phase-weight sum.
    PhaseVarianceMean,
    /// Actual checked square root of mean phase variance.
    PhaseStandardDeviation,
    /// Weighted rate sum divided by rate-weight sum.
    RateMean,
    /// Actual checked square root of rate-weight sum.
    RateWeightRoot,
    /// Reciprocal square root of rate-weight sum.
    RateStandardDeviation,
    /// Actual checked square root of reciprocal squared-skew sum.
    TrackingSkewWeightRoot,
    /// Reciprocal square root of reciprocal squared-skew sum.
    TrackingSkew,
}

/// Bounded source, association or numerical rejection; no float is retained.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum FixtureCombinationError {
    /// More than sixteen uncertainty records were supplied, before indexed reads.
    CapacityExceeded,
    /// First unsupported uncertainty, in phase/rate/tracking-skew field order.
    InvalidUncertainty {
        /// Rejected supplied scalar role.
        field: FixtureCombinationScalarField,
        /// Shared support or sign failure.
        issue: FixtureCombinationScalarIssue,
    },
    /// Mandatory Garde validation disagreed with the maintained scalar predicate.
    ValidationInvariant,
    /// The record count differs from the actual supporter count or absent set.
    MetadataCountMismatch,
    /// First context mismatch in supplied-index then frame/generation/time order.
    ContextMismatch {
        /// Supplied record index, issued only below sixteen.
        index: usize,
        /// Exact incompatible context role.
        field: FixtureCombinationContextField,
    },
    /// An uncertainty record borrows a copy or other entry rather than the original.
    EntryMismatch {
        /// Supplied record index, issued only below sixteen.
        index: usize,
    },
    /// First repeated original source label in the supplied records.
    DuplicateUncertainty {
        /// Supplied record index, issued only below sixteen.
        index: usize,
    },
    /// An actual original entry is not one of the decision's actual supporters.
    UnexpectedUncertainty {
        /// Supplied record index, issued only below sixteen.
        index: usize,
    },
    /// A multisource reciprocal received either sign of zero.
    ZeroDenominator {
        /// Canonical ascending supporter index, issued only below sixteen.
        index: usize,
        /// Positive denominator role; no raw value is retained.
        field: FixtureCombinationDenominatorField,
    },
    /// Actual shared arithmetic or square-root cause under its producing step.
    Arithmetic {
        /// Exact separately evaluated producing primitive.
        step: FixtureCombinationStep,
        /// Actual typed cause from the shared checked owner.
        source: FitError,
    },
    /// A checked decision, private representation or output relation disagreed.
    Invariant,
}

impl fmt::Debug for FixtureCombinationError {
    /// Format the type without entries, labels, uncertainties or raw reports.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureCombinationError { .. }")
    }
}

impl fmt::Display for FixtureCombinationError {
    /// Retain only fixed codes, bounded indices and a bounded actual typed cause.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUncertainty { field, issue } => {
                write!(formatter, "combination uncertainty {field:?}: {issue:?}")
            }
            Self::ContextMismatch { index, field } => {
                write!(formatter, "combination context at {index}: {field:?}")
            }
            Self::EntryMismatch { index } => write!(formatter, "combination entry at {index}"),
            Self::DuplicateUncertainty { index } => {
                write!(formatter, "duplicate combination uncertainty at {index}")
            }
            Self::UnexpectedUncertainty { index } => {
                write!(formatter, "unexpected combination uncertainty at {index}")
            }
            Self::ZeroDenominator { index, field } => {
                write!(
                    formatter,
                    "zero combination denominator at {index}: {field:?}"
                )
            }
            Self::Arithmetic { step, source } => {
                write!(formatter, "combination arithmetic {step:?}: {source}")
            }
            Self::CapacityExceeded => formatter.write_str("combination capacity exceeded"),
            Self::ValidationInvariant => formatter.write_str("combination validation disagreement"),
            Self::MetadataCountMismatch => {
                formatter.write_str("combination metadata count mismatch")
            }
            Self::Invariant => formatter.write_str("combination invariant disagreement"),
        }
    }
}

impl std::error::Error for FixtureCombinationError {
    /// Borrow only the originating shared numerical cause, without reconstruction.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Arithmetic { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Classify one raw uncertainty without altering bits or consulting another field.
fn uncertainty_issue(value: f64) -> Option<FixtureCombinationScalarIssue> {
    match scalar_issue(value) {
        Some(ArithmeticIssue::NonFinite) => Some(FixtureCombinationScalarIssue::NonFinite),
        Some(ArithmeticIssue::Subnormal) => Some(FixtureCombinationScalarIssue::Subnormal),
        Some(_) => Some(FixtureCombinationScalarIssue::Unsupported),
        None if value < 0.0 => Some(FixtureCombinationScalarIssue::Negative),
        None => None,
    }
}

/// Reuse the scalar predicate for the mandatory borrowed Garde callbacks.
/// # Errors
/// Return fixed rejection text only; values and validator reports are not retained.
fn uncertainty_rule(value: f64) -> garde::Result {
    if uncertainty_issue(value).is_none() {
        Ok(())
    } else {
        Err(garde::Error::new("invalid fixture combination uncertainty"))
    }
}

/// Exactly three raw scalars; the checked borrowed entry is kept outside this DTO.
#[derive(Validate)]
struct RawUncertainty {
    /// Supplied same-instant phase standard deviation, finite normal or signed zero.
    #[garde(custom(|value: &f64, (): &()| uncertainty_rule(*value)))]
    phase_standard_deviation_seconds: f64,
    /// Supplied same-instant rate standard deviation, finite normal or signed zero.
    #[garde(custom(|value: &f64, (): &()| uncertainty_rule(*value)))]
    rate_standard_deviation_seconds_per_second: f64,
    /// Supplied same-instant tracking skew, finite normal or signed zero.
    #[garde(custom(|value: &f64, (): &()| uncertainty_rule(*value)))]
    tracking_skew_seconds_per_second: f64,
}

impl RawUncertainty {
    /// Project the same rules in declared field order, without copying raw values.
    fn first_error(&self) -> Option<FixtureCombinationError> {
        for (field, value) in [
            (
                FixtureCombinationScalarField::PhaseStandardDeviation,
                self.phase_standard_deviation_seconds,
            ),
            (
                FixtureCombinationScalarField::RateStandardDeviation,
                self.rate_standard_deviation_seconds_per_second,
            ),
            (
                FixtureCombinationScalarField::TrackingSkew,
                self.tracking_skew_seconds_per_second,
            ),
        ] {
            if let Some(issue) = uncertainty_issue(value) {
                return Some(FixtureCombinationError::InvalidUncertainty { field, issue });
            }
        }
        None
    }
}

/// Immutable common-instant uncertainties associated with an actual entry borrow.
///
/// The constructor enforces scalar support and sign through mandatory Garde.
/// Original-entry identity and decision membership belong to the receiving kernel.
/// No physical calibration, freshness, compatibility or independent-error claim is
/// attached to these explicit fixture assertions. Borrowing forbids mutation and
/// prevents the value from outliving its original storage.
#[derive(Clone, Copy)]
pub struct FixtureSupporterUncertainty<'entry> {
    /// Existing checked projection enclosure; no raw field is skipped by Garde.
    entry: &'entry FixtureProjectedInterval,
    /// Admitted phase standard deviation at the common projection instant.
    phase_standard_deviation_seconds: f64,
    /// Admitted rate standard deviation at the common projection instant.
    rate_standard_deviation_seconds_per_second: f64,
    /// Admitted tracking skew, distinct from projection growth skew.
    tracking_skew_seconds_per_second: f64,
}

impl fmt::Debug for FixtureSupporterUncertainty<'_> {
    /// Format the type without entries, labels or uncertainty bits.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureSupporterUncertainty { .. }")
    }
}

impl<'entry> FixtureSupporterUncertainty<'entry> {
    /// Check phase SD, rate SD and tracking skew in that order, preserving bits.
    ///
    /// Every scalar must be finite normal or signed zero and numerically
    /// nonnegative. The original entry remains unchanged and separately borrowed;
    /// this operation establishes no decision membership or event authority.
    /// Garde's bounded temporary rejection report may allocate and is discarded.
    /// # Errors
    /// Return the first typed field/issue or mandatory-validation disagreement.
    pub fn new(
        entry: &'entry FixtureProjectedInterval,
        phase_standard_deviation_seconds: f64,
        rate_standard_deviation_seconds_per_second: f64,
        tracking_skew_seconds_per_second: f64,
    ) -> Result<Self, FixtureCombinationError> {
        let raw = RawUncertainty {
            phase_standard_deviation_seconds,
            rate_standard_deviation_seconds_per_second,
            tracking_skew_seconds_per_second,
        };
        let validation = raw.validate();
        if let Some(error) = raw.first_error() {
            return Err(error);
        }
        validation.map_err(|_| FixtureCombinationError::ValidationInvariant)?;
        Ok(Self {
            entry,
            phase_standard_deviation_seconds,
            rate_standard_deviation_seconds_per_second,
            tracking_skew_seconds_per_second,
        })
    }

    /// Borrow the actual immutable original supplied at construction.
    #[must_use]
    pub const fn entry(&self) -> &'entry FixtureProjectedInterval {
        self.entry
    }

    /// Copy the supplied same-instant phase SD, retaining signed-zero bits.
    #[must_use]
    pub const fn phase_standard_deviation_seconds(&self) -> f64 {
        self.phase_standard_deviation_seconds
    }

    /// Copy the supplied same-instant rate SD, retaining signed-zero bits.
    #[must_use]
    pub const fn rate_standard_deviation_seconds_per_second(&self) -> f64 {
        self.rate_standard_deviation_seconds_per_second
    }

    /// Copy supplied tracking skew without substituting projection growth skew.
    #[must_use]
    pub const fn tracking_skew_seconds_per_second(&self) -> f64 {
        self.tracking_skew_seconds_per_second
    }
}

/// Closed mathematical-kernel outcome, without reference or event authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureCombinationKernelKind {
    /// The genuine principal decision supplied no principal for a numerical view.
    Unavailable,
    /// A complete checked inert numerical view exists for actual supporters.
    Combined,
}

/// Private checked output representation, retaining the original principal kind.
#[derive(Clone, Copy)]
pub struct FixtureCombinationKernelOutcome {
    /// Unchanged input outcome, including unavailable versus waiting provenance.
    principal_kind: FixturePrincipalOutcomeKind,
    /// Complete estimate only after all input and transformed-output checks.
    combined: Option<FixtureCombinedEstimate>,
}

impl fmt::Debug for FixtureCombinationKernelOutcome {
    /// Format the type without decisions, entries, labels or numerical quantities.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureCombinationKernelOutcome { .. }")
    }
}

impl FixtureCombinationKernelOutcome {
    /// Copy the mathematical outcome without manufacturing reference credit.
    #[must_use]
    pub const fn kind(&self) -> FixtureCombinationKernelKind {
        if self.combined.is_some() {
            FixtureCombinationKernelKind::Combined
        } else {
            FixtureCombinationKernelKind::Unavailable
        }
    }

    /// Copy the unchanged genuine principal kind; retained views gain no credit.
    #[must_use]
    pub const fn principal_kind(&self) -> FixturePrincipalOutcomeKind {
        self.principal_kind
    }

    /// Borrow the complete checked view, absent when no principal was supplied.
    #[must_use]
    pub const fn combined(&self) -> Option<&FixtureCombinedEstimate> {
        self.combined.as_ref()
    }
}

/// Complete inert numerical view with original principal and supporter provenance.
///
/// Fields are private; no raw factory or mutation is exported. The five floating
/// outputs carry the checked fixture arithmetic's support envelope, never physical
/// confidence, freshness, compatibility certification or clock authority.
#[derive(Clone, Copy)]
pub struct FixtureCombinedEstimate {
    /// Original principal context, including its asserted source label.
    context: FixtureContext,
    /// Exact original principal anchor instant, without refreshing observation age.
    anchor_observed_at: MonotonicNs,
    /// Exact original principal latest observation instant.
    latest_observed_at: MonotonicNs,
    /// Exact original principal fit-reference instant.
    fit_reference: MonotonicNs,
    /// Exact common projection instant supplied to the original batch.
    projected_at: MonotonicNs,
    /// Complete sorted supporter prefix; unused padding is inaccessible.
    labels: [NonZeroU64; MAX_SUPPORTERS],
    /// Admitted occupied label prefix, in 1..=16.
    supporter_count: usize,
    /// Positive-behind weighted original phase, in seconds.
    phase_seconds: f64,
    /// Weighted original signed fitted rate, in seconds/second.
    phase_rate_seconds_per_second: f64,
    /// Principal-centered phase standard deviation at the common instant.
    phase_standard_deviation_seconds: f64,
    /// Reciprocal-square-weight rate standard deviation, in seconds/second.
    rate_standard_deviation_seconds_per_second: f64,
    /// Reciprocal-square-weight tracking skew, in seconds/second.
    tracking_skew_seconds_per_second: f64,
}

impl fmt::Debug for FixtureCombinedEstimate {
    /// Format only the type, without original contexts, labels or numerical values.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureCombinedEstimate { .. }")
    }
}

impl FixtureCombinedEstimate {
    /// Copy the original principal source label, without real-source authority.
    #[must_use]
    pub const fn principal_label(&self) -> NonZeroU64 {
        self.context.source_id()
    }

    /// Copy the unchanged asserted common frame.
    #[must_use]
    pub const fn frame(&self) -> NonZeroU64 {
        self.context.frame()
    }

    /// Copy the unchanged asserted common mapping generation.
    #[must_use]
    pub const fn mapping_generation(&self) -> NonZeroU64 {
        self.context.generation()
    }

    /// Copy the original principal anchor instant, without fresh observation credit.
    #[must_use]
    pub const fn anchor_observed_at(&self) -> MonotonicNs {
        self.anchor_observed_at
    }

    /// Copy the original principal latest observation instant unchanged.
    #[must_use]
    pub const fn latest_observed_at(&self) -> MonotonicNs {
        self.latest_observed_at
    }

    /// Copy the original principal's actual fit-reference instant unchanged.
    #[must_use]
    pub const fn fit_reference(&self) -> MonotonicNs {
        self.fit_reference
    }

    /// Copy the actual common projection instant without an ambient-clock read.
    #[must_use]
    pub const fn projected_at(&self) -> MonotonicNs {
        self.projected_at
    }

    /// Copy the complete occupied supporter count, never an inferred vote count.
    #[must_use]
    pub const fn supporter_count(&self) -> usize {
        self.supporter_count
    }

    /// Iterate every actual supporter in ascending label order, excluding padding.
    pub fn supporters(&self) -> impl ExactSizeIterator<Item = NonZeroU64> + '_ {
        self.labels.iter().take(self.supporter_count).copied()
    }

    /// Copy the checked positive-behind phase in seconds.
    #[must_use]
    pub const fn phase_seconds(&self) -> f64 {
        self.phase_seconds
    }

    /// Copy the checked signed original fitted-rate mean in seconds/second.
    #[must_use]
    pub const fn phase_rate_seconds_per_second(&self) -> f64 {
        self.phase_rate_seconds_per_second
    }

    /// Copy the checked principal-centered phase standard deviation in seconds.
    #[must_use]
    pub const fn phase_standard_deviation_seconds(&self) -> f64 {
        self.phase_standard_deviation_seconds
    }

    /// Copy the checked rate standard deviation in seconds/second.
    #[must_use]
    pub const fn rate_standard_deviation_seconds_per_second(&self) -> f64 {
        self.rate_standard_deviation_seconds_per_second
    }

    /// Copy the checked tracking skew, without a physical confidence assertion.
    #[must_use]
    pub const fn tracking_skew_seconds_per_second(&self) -> f64 {
        self.tracking_skew_seconds_per_second
    }
}

/// Copy a checked record's original label; immutable borrowed entry owns validity.
fn record_label(record: &FixtureSupporterUncertainty<'_>) -> NonZeroU64 {
    record.entry.projection().context().source_id()
}

/// Require exact frame, generation and instant in their documented first-error order.
/// # Errors
/// Return only a bounded supplied index and the first mismatched field.
fn admit_context(
    entry: &FixtureProjectedInterval,
    principal: &FixtureProjectedInterval,
    index: usize,
) -> Result<(), FixtureCombinationError> {
    let actual = entry.projection();
    let expected = principal.projection();
    let field = if actual.context().frame() != expected.context().frame() {
        Some(FixtureCombinationContextField::Frame)
    } else if actual.context().generation() != expected.context().generation() {
        Some(FixtureCombinationContextField::MappingGeneration)
    } else if actual.projected_at() != expected.projected_at() {
        Some(FixtureCombinationContextField::ProjectedAt)
    } else {
        None
    };
    if let Some(field) = field {
        return Err(FixtureCombinationError::ContextMismatch { index, field });
    }
    Ok(())
}

/// Admit exactly one immutable record per actual witnessed original supporter.
///
/// Caller has checked capacity and supplied a genuine principal entry. Each
/// supplied-index check orders context, original address, duplicate then membership;
/// all loops are at most sixteen. No record or source state is modified.
/// # Errors
/// Reject missing/extra metadata or its first contextual/association violation.
fn admit_records(
    decision: &FixturePrincipalDecision<'_>,
    records: &[FixtureSupporterUncertainty<'_>],
    principal: &FixtureProjectedInterval,
) -> Result<(), FixtureCombinationError> {
    let witness = decision
        .selection()
        .witnessed()
        .ok_or(FixtureCombinationError::Invariant)?;
    let count = witness.supporters().len();
    if count == 0 || count > MAX_SUPPORTERS || decision.entries().len() > MAX_SUPPORTERS {
        return Err(FixtureCombinationError::Invariant);
    }
    if records.len() != count {
        return Err(FixtureCombinationError::MetadataCountMismatch);
    }
    let principal_label = principal.projection().context().source_id();
    if decision.principal_label() != Some(principal_label)
        || !witness.supporters().any(|label| label == principal_label)
    {
        return Err(FixtureCombinationError::Invariant);
    }
    for (index, record) in records.iter().enumerate() {
        admit_context(record.entry, principal, index)?;
        if !decision
            .entries()
            .iter()
            .any(|entry| ptr::eq(entry, record.entry))
        {
            return Err(FixtureCombinationError::EntryMismatch { index });
        }
        let label = record_label(record);
        if records
            .iter()
            .take(index)
            .any(|previous| record_label(previous) == label)
        {
            return Err(FixtureCombinationError::DuplicateUncertainty { index });
        }
        if !witness.supporters().any(|supporter| supporter == label) {
            return Err(FixtureCombinationError::UnexpectedUncertainty { index });
        }
    }
    Ok(())
}

/// Evaluate one unfused binary primitive under its actual combination step.
///
/// The shared numerical owner checks both operands and the computed result;
/// there is no clamp, normalization, state mutation or partial publication.
/// # Errors
/// Preserve the originating supported-domain or arithmetic cause and exact step.
fn binary(
    left: f64,
    right: f64,
    operation: ArithmeticOperation,
    step: FixtureCombinationStep,
) -> Result<f64, FixtureCombinationError> {
    arithmetic(left, right, operation, ArithmeticStage::Combination)
        .map_err(|source| FixtureCombinationError::Arithmetic { step, source })
}

/// Evaluate one actual checked square root, retaining its unary error identity.
/// # Errors
/// Preserve the shared root cause and exact producing combination step.
fn square_root(value: f64, step: FixtureCombinationStep) -> Result<f64, FixtureCombinationError> {
    checked_square_root(value, ArithmeticStage::Combination)
        .map_err(|source| FixtureCombinationError::Arithmetic { step, source })
}

/// Require a nonzero denominator already owned by a checked entry/uncertainty.
///
/// The supplying type guarantees a finite normal-or-signed-zero nonnegative
/// value. This additional multisource guard admits only its strictly positive
/// normal branch, without arithmetic or changes to the original bits.
/// # Errors
/// Reject either zero sign with its canonical supporter index and scalar role.
fn positive_denominator(
    value: f64,
    index: usize,
    field: FixtureCombinationDenominatorField,
) -> Result<(), FixtureCombinationError> {
    if value == 0.0 {
        Err(FixtureCombinationError::ZeroDenominator { index, field })
    } else {
        Ok(())
    }
}

/// Five supported computed quantities, without context or effect authority.
///
/// The singleton path borrows their validity from the immutable projection and
/// checked uncertainty; the multisource path uses separately checked primitives.
struct Quantities {
    /// Positive-behind original phase or weighted phase, in seconds.
    phase_seconds: f64,
    /// Original or weighted signed fitted rate, in seconds/second.
    phase_rate_seconds_per_second: f64,
    /// Original or principal-centered phase standard deviation, in seconds.
    phase_standard_deviation_seconds: f64,
    /// Original or reciprocal-square-weight rate standard deviation.
    rate_standard_deviation_seconds_per_second: f64,
    /// Original or reciprocal-square-weight tracking skew, in seconds/second.
    tracking_skew_seconds_per_second: f64,
}

/// Six supported local accumulators, never exposed before complete reduction.
struct Sums {
    /// Ascending-label sum of reciprocal root-distance phase weights.
    phase_weight: f64,
    /// Ascending-label sum of phase-weight times original common-time phase.
    phase: f64,
    /// Ascending-label weighted principal-centered phase variance.
    phase_variance: f64,
    /// Ascending-label sum of reciprocal squared rate standard deviations.
    rate_weight: f64,
    /// Ascending-label sum of rate-weight times original signed fitted rate.
    rate: f64,
    /// Ascending-label sum of reciprocal squared tracking skews.
    tracking_skew_weight: f64,
}

impl Sums {
    /// Initialize every reduction to positive zero without consulting input state.
    const fn empty() -> Self {
        Self {
            phase_weight: 0.0,
            phase: 0.0,
            phase_variance: 0.0,
            rate_weight: 0.0,
            rate: 0.0,
            tracking_skew_weight: 0.0,
        }
    }

    /// Stage one complete original supporter into these private bounded sums.
    ///
    /// The caller supplies an admitted record in ascending actual-witness label
    /// order and the original principal phase. Denominator guards precede all
    /// arithmetic for this supporter. Every binary primitive is separate; the
    /// unchanged previous sums remain available if any operation rejects.
    /// Exactly eighteen binary operations are attempted on the successful path.
    /// # Errors
    /// Preserve the first zero denominator or actual primitive failure; no sum
    /// update is returned until the entire supporter contribution succeeds.
    fn with_supporter(
        &self,
        record: &FixtureSupporterUncertainty<'_>,
        principal_phase: f64,
        index: usize,
    ) -> Result<Self, FixtureCombinationError> {
        use ArithmeticOperation::{Add, Divide, Multiply, Subtract};
        use FixtureCombinationStep as Step;

        let projection = record.entry.projection();
        let distance = projection.distance_seconds();
        let phase = projection.center_seconds();
        let rate = projection.phase_rate_seconds_per_second();
        let phase_sd = record.phase_standard_deviation_seconds;
        let rate_sd = record.rate_standard_deviation_seconds_per_second;
        let tracking_skew = record.tracking_skew_seconds_per_second;
        for (value, field) in [
            (distance, FixtureCombinationDenominatorField::RootDistance),
            (
                rate_sd,
                FixtureCombinationDenominatorField::RateStandardDeviation,
            ),
            (
                tracking_skew,
                FixtureCombinationDenominatorField::TrackingSkew,
            ),
        ] {
            positive_denominator(value, index, field)?;
        }
        let phase_weight = binary(1.0, distance, Divide, Step::PhaseWeight)?;
        let rate_variance = binary(rate_sd, rate_sd, Multiply, Step::RateVariance)?;
        let rate_weight = binary(1.0, rate_variance, Divide, Step::RateWeight)?;
        let tracking_skew_variance = binary(
            tracking_skew,
            tracking_skew,
            Multiply,
            Step::TrackingSkewVariance,
        )?;
        let tracking_skew_weight = binary(
            1.0,
            tracking_skew_variance,
            Divide,
            Step::TrackingSkewWeight,
        )?;

        let sum_phase_weight = binary(self.phase_weight, phase_weight, Add, Step::PhaseWeightSum)?;
        let weighted_phase = binary(phase_weight, phase, Multiply, Step::WeightedPhase)?;
        let sum_phase = binary(self.phase, weighted_phase, Add, Step::PhaseSum)?;
        let phase_variance = binary(phase_sd, phase_sd, Multiply, Step::PhaseVariance)?;
        let difference = binary(phase, principal_phase, Subtract, Step::PrincipalDifference)?;
        let disagreement = binary(difference, difference, Multiply, Step::DisagreementVariance)?;
        let combined_variance = binary(
            phase_variance,
            disagreement,
            Add,
            Step::PhaseVarianceAndDisagreement,
        )?;
        let weighted_variance = binary(
            phase_weight,
            combined_variance,
            Multiply,
            Step::WeightedPhaseVariance,
        )?;
        let sum_phase_variance = binary(
            self.phase_variance,
            weighted_variance,
            Add,
            Step::PhaseVarianceSum,
        )?;
        let sum_rate_weight = binary(self.rate_weight, rate_weight, Add, Step::RateWeightSum)?;
        let weighted_rate = binary(rate_weight, rate, Multiply, Step::WeightedRate)?;
        let sum_rate = binary(self.rate, weighted_rate, Add, Step::RateSum)?;
        let sum_tracking_skew_weight = binary(
            self.tracking_skew_weight,
            tracking_skew_weight,
            Add,
            Step::TrackingSkewWeightSum,
        )?;
        Ok(Self {
            phase_weight: sum_phase_weight,
            phase: sum_phase,
            phase_variance: sum_phase_variance,
            rate_weight: sum_rate_weight,
            rate: sum_rate,
            tracking_skew_weight: sum_tracking_skew_weight,
        })
    }

    /// Complete a nonempty multisource reduction with five binary calls and three roots.
    ///
    /// Supported sums came only from complete checked supporter contributions;
    /// the denominator and root owners independently admit the final operations.
    /// # Errors
    /// Preserve the first exact final step/cause without publishing any quantity.
    fn finish(&self) -> Result<Quantities, FixtureCombinationError> {
        use ArithmeticOperation::Divide;
        use FixtureCombinationStep as Step;

        let phase_seconds = binary(self.phase, self.phase_weight, Divide, Step::PhaseMean)?;
        let phase_variance = binary(
            self.phase_variance,
            self.phase_weight,
            Divide,
            Step::PhaseVarianceMean,
        )?;
        let phase_standard_deviation_seconds =
            square_root(phase_variance, Step::PhaseStandardDeviation)?;
        let phase_rate_seconds_per_second =
            binary(self.rate, self.rate_weight, Divide, Step::RateMean)?;
        let rate_weight_root = square_root(self.rate_weight, Step::RateWeightRoot)?;
        let rate_standard_deviation_seconds_per_second =
            binary(1.0, rate_weight_root, Divide, Step::RateStandardDeviation)?;
        let tracking_skew_weight_root =
            square_root(self.tracking_skew_weight, Step::TrackingSkewWeightRoot)?;
        let tracking_skew_seconds_per_second =
            binary(1.0, tracking_skew_weight_root, Divide, Step::TrackingSkew)?;
        Ok(Quantities {
            phase_seconds,
            phase_rate_seconds_per_second,
            phase_standard_deviation_seconds,
            rate_standard_deviation_seconds_per_second,
            tracking_skew_seconds_per_second,
        })
    }
}

/// Borrow one already admitted original record by its actual witness label.
///
/// The receiver established exact unique coverage; the search is at most sixteen
/// and does not create an alternate participant or mutable reference.
/// # Errors
/// Fail closed if private checked coverage contradicts the witness.
fn supporter_record<'records, 'entry>(
    records: &'records [FixtureSupporterUncertainty<'entry>],
    label: NonZeroU64,
) -> Result<&'records FixtureSupporterUncertainty<'entry>, FixtureCombinationError> {
    records
        .iter()
        .find(|record| record_label(record) == label)
        .ok_or(FixtureCombinationError::Invariant)
}

/// Copy a genuine singleton's five admitted scalar bit patterns without arithmetic.
///
/// The immutable projection and uncertainty constructor own all scalar validity.
/// There is no reciprocal, square root, extrapolation, normalization or fallback.
fn singleton_quantities(record: &FixtureSupporterUncertainty<'_>) -> Quantities {
    let projection = record.entry.projection();
    Quantities {
        phase_seconds: projection.center_seconds(),
        phase_rate_seconds_per_second: projection.phase_rate_seconds_per_second(),
        phase_standard_deviation_seconds: record.phase_standard_deviation_seconds,
        rate_standard_deviation_seconds_per_second: record
            .rate_standard_deviation_seconds_per_second,
        tracking_skew_seconds_per_second: record.tracking_skew_seconds_per_second,
    }
}

/// Complete the estimate after private scalar transformations and exact label custody.
///
/// The caller supplies an occupied ascending actual-witness prefix of one to
/// sixteen labels, including the original principal. Scalar outputs are checked
/// again at this private publication seam; context/ages are copied unchanged.
/// # Errors
/// Reject an impossible prefix or unsupported/transformed uncertainty before any
/// public output exists; no input, source state or reference authority changes.
fn complete_estimate(
    principal: &FixtureProjectedInterval,
    labels: [NonZeroU64; MAX_SUPPORTERS],
    count: usize,
    quantities: &Quantities,
) -> Result<FixtureCombinedEstimate, FixtureCombinationError> {
    if count == 0 || count > MAX_SUPPORTERS {
        return Err(FixtureCombinationError::Invariant);
    }
    for value in [
        quantities.phase_seconds,
        quantities.phase_rate_seconds_per_second,
    ] {
        if scalar_issue(value).is_some() {
            return Err(FixtureCombinationError::Invariant);
        }
    }
    for value in [
        quantities.phase_standard_deviation_seconds,
        quantities.rate_standard_deviation_seconds_per_second,
        quantities.tracking_skew_seconds_per_second,
    ] {
        if uncertainty_issue(value).is_some() {
            return Err(FixtureCombinationError::Invariant);
        }
    }
    let projection = principal.projection();
    Ok(FixtureCombinedEstimate {
        context: projection.context(),
        anchor_observed_at: projection.anchor_observed_at(),
        latest_observed_at: projection.latest_observed_at(),
        fit_reference: projection.fit_reference(),
        projected_at: projection.projected_at(),
        labels,
        supporter_count: count,
        phase_seconds: quantities.phase_seconds,
        phase_rate_seconds_per_second: quantities.phase_rate_seconds_per_second,
        phase_standard_deviation_seconds: quantities.phase_standard_deviation_seconds,
        rate_standard_deviation_seconds_per_second: quantities
            .rate_standard_deviation_seconds_per_second,
        tracking_skew_seconds_per_second: quantities.tracking_skew_seconds_per_second,
    })
}

/// Admit a genuine principal decision and the complete actual supporter metadata.
///
/// Capacity precedes entry work; absent principals require empty metadata and
/// preserve their original unavailable/waiting kind. Present principals require
/// exact original-entry identity and complete current witness coverage. Inputs
/// stay unchanged, no heap-backed scratch or state/effect is introduced, and no
/// fresh event credit is produced. Genuine singleton scalars pass through bitwise;
/// multisource reductions use every supporter in ascending label order and reject
/// unsupported arithmetic without dropping a participant or returning a fallback.
/// # Errors
/// Reject capacity, first association error, an inconsistent checked decision or
/// the exact checked primitive/denominator failure without a partial output.
pub fn combine_fixture_supporters(
    decision: &FixturePrincipalDecision<'_>,
    uncertainties: &[FixtureSupporterUncertainty<'_>],
) -> Result<FixtureCombinationKernelOutcome, FixtureCombinationError> {
    if uncertainties.len() > MAX_SUPPORTERS {
        return Err(FixtureCombinationError::CapacityExceeded);
    }
    let kind = decision.kind();
    let selected_kind = matches!(
        kind,
        FixturePrincipalOutcomeKind::ReferenceReady
            | FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    );
    let Some(principal) = decision.principal_entry() else {
        if !uncertainties.is_empty() {
            return Err(FixtureCombinationError::MetadataCountMismatch);
        }
        if selected_kind || decision.principal_label().is_some() {
            return Err(FixtureCombinationError::Invariant);
        }
        return Ok(FixtureCombinationKernelOutcome {
            principal_kind: kind,
            combined: None,
        });
    };
    if !selected_kind {
        return Err(FixtureCombinationError::Invariant);
    }
    admit_records(decision, uncertainties, principal)?;
    let witness = decision
        .selection()
        .witnessed()
        .ok_or(FixtureCombinationError::Invariant)?;
    let count = witness.supporters().len();
    let principal_label = principal.projection().context().source_id();
    let mut labels = [principal_label; MAX_SUPPORTERS];
    for (index, label) in witness.supporters().enumerate() {
        *labels
            .get_mut(index)
            .ok_or(FixtureCombinationError::Invariant)? = label;
    }
    let quantities = if count == 1 {
        singleton_quantities(supporter_record(uncertainties, principal_label)?)
    } else {
        let principal_phase = principal.projection().center_seconds();
        let mut sums = Sums::empty();
        for (index, label) in witness.supporters().enumerate() {
            let record = supporter_record(uncertainties, label)?;
            sums = sums.with_supporter(record, principal_phase, index)?;
        }
        sums.finish()?
    };
    let combined = complete_estimate(principal, labels, count, &quantities)?;
    Ok(FixtureCombinationKernelOutcome {
        principal_kind: kind,
        combined: Some(combined),
    })
}

/// Independent genuine-public-input cases share only crate exports with this owner.
#[cfg(test)]
#[path = "../tests/private/combination_cases.rs"]
mod combination_cases;

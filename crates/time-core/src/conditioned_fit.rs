// SPDX-License-Identifier: GPL-2.0-only
/*
 chronyd/chronyc - Programs for keeping computer clocks accurate.

**********************************************************************
* Copyright (C) Richard P. Curnow  1997-2003
* Copyright (C) Miroslav Lichvar  2011, 2016-2017
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
//! Actual-parent suffixes and enclosed, conditioned inverse-divisor affine fits.
//!
//! Modified by David Osipov with AI assistance, 7 November 2026. The centered
//! inverse-divisor equations adapt Chrony 4.9 `regress.c`, pinned revision
//! 8df4f1263e206585e6c4e61352d042c2dd12916a, selected lines 69–93 and 252–269.
//! See ../PROVENANCE.md and the additive Chrony ledger. This is source-derived.
//!
//! N2a borrows N1's actual population, exact newest anchors, binary scale exponents
//! and unchanged divisors. Five chronological passes stage saved reciprocals,
//! geometry, centered response, coefficients and weighted SSE. Separate interval
//! equations enclose the original exact coordinates, including N1 conversion and
//! all new arithmetic. Geometry is admitted before any response arithmetic.
//! Checked normalized and lifted outputs publish atomically. No absolute phase is
//! converted to binary64; the exact borrowed anchor remains separate. Successful
//! work uses fixed storage for at most 64 rows, with no allocation, ambient read,
//! I/O, history mutation, source authority or clock effect.

use std::fmt;

use garde::Validate;

use crate::conditioned_window::{
    ExactObservationRow, PreparedConditionedBatch, PreparedConditionedPoint,
};

/// Separate finite point and original-target interval arithmetic owner.
mod arithmetic;

use arithmetic::{
    ArithmeticError, Interval, ScalarEnclosure, point_add, point_div_positive, point_mul, point_sub,
};

/// N1's complete admitted population and the fixed scratch bound.
const MAX_ROWS: usize = 64;

/// Exact corrected-phase denominator, retained separately from local corrections.
const PHASE_DENOMINATOR: u128 = 8_589_934_592_000_000_000;

/// Bounded numerical stage, without original numeric payloads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConditionedFitStage {
    /// Saved reciprocal and chronological weighted-reference mean.
    WeightsAndReferenceMean,
    /// Centered reference geometry before response arithmetic.
    Geometry,
    /// Chronological weighted phase mean.
    PhaseMean,
    /// Centered covariance followed by rate and intercept.
    CovarianceAndCoefficients,
    /// Fresh chronological residual weighted sum of squares.
    ResidualSum,
    /// Atomic normalized scalar enclosure construction.
    NormalizedOutput,
    /// Checked relative phase-correction lifting.
    PhaseLifting,
    /// Checked dimensionless frequency-ratio lifting.
    RateLifting,
    /// Checked weighted seconds-squared lifting.
    SseLifting,
}

/// Fixed operation family, without operands or computed values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConditionedFitOperation {
    /// Separately rounded addition or its outward enclosure.
    Add,
    /// Separately rounded subtraction or its outward enclosure.
    Subtract,
    /// Separately rounded multiplication or its outward enclosure.
    Multiply,
    /// Division by a strictly positive point or interval.
    Divide,
    /// Dedicated outward interval square.
    Square,
    /// Seed an interval from an admitted point and its conversion bound.
    Seed,
    /// Publish a hull and outward absolute arithmetic-error bound.
    Publish,
    /// Admit and encode an exact binary lifting factor.
    PowerOfTwo,
}

/// Public bounded projection of the private arithmetic owner's refusal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConditionedFitIssue {
    /// A point operand or separately rounded result is nonfinite.
    NonFinite,
    /// Division lacks a strictly positive admitted denominator.
    NonPositiveDenominator,
    /// Nonzero stored multiplication/division rounded to zero.
    PointUnderflow,
    /// A raw interval or arithmetic-error representation is invalid.
    InvalidInterval,
    /// Outward endpoints or arithmetic-error bounds exhaust the finite range.
    ErrorRange,
}

/// Typed, redacted refusal preserving every borrowed input bit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConditionedFitError {
    /// The requested suffix start exceeds the admitted parent population.
    SuffixRange {
        /// Supplied start; no subtraction or access occurred before refusal.
        start: usize,
        /// Original population count, between three and 64.
        count: usize,
    },
    /// A checked primitive or output representation refused at its actual owner.
    Arithmetic {
        /// Fixed algorithm stage.
        stage: ConditionedFitStage,
        /// Original row index, or no index for an aggregate/output operation.
        index: Option<usize>,
        /// Fixed numerical operation.
        operation: ConditionedFitOperation,
        /// Redacted typed arithmetic cause.
        issue: ConditionedFitIssue,
    },
    /// Point or exact-target geometry lacks the required positive margin.
    UnresolvedGeometry,
    /// A private transformed value contradicted its established bounds.
    OutputInvariant,
    /// A mandatory validator disagreed with the maintained admission predicates.
    ValidationInvariant,
}

impl fmt::Display for ConditionedFitError {
    /// Format only fixed causes and permitted counts, indexes and stages.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SuffixRange { start, count } => {
                write!(formatter, "suffix start {start} exceeds population {count}")
            }
            Self::Arithmetic {
                stage,
                index,
                operation,
                issue,
            } => write!(
                formatter,
                "conditioned {stage:?} {operation:?} failed at {index:?}: {issue:?}"
            ),
            Self::UnresolvedGeometry => formatter.write_str("unresolved conditioned geometry"),
            Self::OutputInvariant => formatter.write_str("conditioned output invariant failed"),
            Self::ValidationInvariant => {
                formatter.write_str("conditioned validation invariant failed")
            }
        }
    }
}

impl std::error::Error for ConditionedFitError {}

/// Actual immutable population borrow with a checked contiguous index range.
struct SuffixView<'batch, 'rows> {
    /// N1 owns all original rows, anchors, scales and per-row divisor invariants.
    parent: &'batch PreparedConditionedBatch<'rows>,
    /// Checked start in the closed range zero through the parent count.
    start: usize,
    /// Complete retained suffix count, exactly parent count minus start.
    count: usize,
}

impl<'rows> SuffixView<'_, 'rows> {
    /// Resolve only a local retained index, checking before addition and access.
    fn original_index(&self, index: usize) -> Option<usize> {
        if index < self.count {
            self.start.checked_add(index)
        } else {
            None
        }
    }

    /// Borrow the actual retained prepared point; no point/weight pair is rebuilt.
    fn point(&self, index: usize) -> Option<&PreparedConditionedPoint<'rows>> {
        self.parent.point(self.original_index(index)?)
    }

    /// Borrow the actual original row, preserving its separate source lifetime.
    fn original_row(&self, index: usize) -> Option<&'rows ExactObservationRow> {
        self.parent.original_rows().get(self.original_index(index)?)
    }
}

/// Private checked 3..=64-row suffix retaining the actual parent population.
///
/// No constructor, mutable view, default or independent weight attachment exists.
/// The parent and its separately borrowed original rows must both remain alive.
///
/// ```compile_fail,E0451
/// use time_core::ConditionedSuffix;
/// fn construct(base: ConditionedSuffix<'_, '_>) {
///     let _ = ConditionedSuffix { ..base };
/// }
/// ```
///
/// ```compile_fail,E0616
/// use time_core::ConditionedSuffix;
/// fn mutate(mut base: ConditionedSuffix<'_, '_>) { base.view.count = 0; }
/// ```
///
/// ```compile_fail,E0277
/// use time_core::ConditionedSuffix;
/// let _: ConditionedSuffix<'_, '_> = Default::default();
/// ```
///
/// ```compile_fail,E0515
/// use time_core::{ConditionedFitError, ConditionedSuffixOutcome,
///     PreparedConditionedBatch, prepare_conditioned_suffix};
/// fn escape<'rows>(parent: PreparedConditionedBatch<'rows>)
///     -> Result<ConditionedSuffixOutcome<'rows, 'rows>, ConditionedFitError>
/// {
///     prepare_conditioned_suffix(&parent, 0)
/// }
/// ```
///
/// ```compile_fail
/// use time_core::{ConditionedSuffix, ExactObservationRow};
/// fn extend_rows<'batch, 'rows>(suffix: &ConditionedSuffix<'batch, 'rows>)
///     -> Option<&'static ExactObservationRow>
/// {
///     suffix.original_row(0)
/// }
/// ```
pub struct ConditionedSuffix<'batch, 'rows> {
    /// Sole checked range producer owns the complete ready-suffix invariant.
    view: SuffixView<'batch, 'rows>,
}

impl fmt::Debug for ConditionedSuffix<'_, '_> {
    /// Redact original rows, declarations, anchors, scales and numeric points.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ConditionedSuffix { .. }")
    }
}

impl<'batch, 'rows> ConditionedSuffix<'batch, 'rows> {
    /// Borrow the actual complete N1 parent without reweighting or reanchoring.
    #[must_use]
    pub const fn parent(&self) -> &'batch PreparedConditionedBatch<'rows> {
        self.view.parent
    }

    /// Return the original-population start of this complete suffix.
    #[must_use]
    pub const fn original_start(&self) -> usize {
        self.view.start
    }

    /// Return the admitted suffix count, between three and 64.
    #[must_use]
    pub const fn sample_count(&self) -> usize {
        self.view.count
    }

    /// Borrow a local suffix point; every out-of-range index returns `None`.
    #[must_use]
    pub fn point(&self, index: usize) -> Option<&PreparedConditionedPoint<'rows>> {
        self.view.point(index)
    }

    /// Borrow the actual local original row, including its unchanged divisor owner.
    #[must_use]
    pub fn original_row(&self, index: usize) -> Option<&'rows ExactObservationRow> {
        self.view.original_row(index)
    }
}

/// Checked 0..=2-row suffix, preserving the full parent's metadata even when empty.
///
/// This type cannot be passed to the fit producer and exposes no fit-ready cast.
///
/// ```compile_fail,E0451
/// use time_core::InsufficientConditionedSuffix;
/// fn construct(base: InsufficientConditionedSuffix<'_, '_>) {
///     let _ = InsufficientConditionedSuffix { ..base };
/// }
/// ```
///
/// ```compile_fail,E0616
/// use time_core::InsufficientConditionedSuffix;
/// fn mutate(mut base: InsufficientConditionedSuffix<'_, '_>) { base.view.start = 0; }
/// ```
///
/// ```compile_fail,E0277
/// use time_core::InsufficientConditionedSuffix;
/// let _: InsufficientConditionedSuffix<'_, '_> = Default::default();
/// ```
///
/// ```compile_fail,E0308
/// use time_core::{InsufficientConditionedSuffix, fit_conditioned_suffix};
/// fn fit_short(short: &InsufficientConditionedSuffix<'_, '_>) {
///     let _ = fit_conditioned_suffix(short);
/// }
/// ```
pub struct InsufficientConditionedSuffix<'batch, 'rows> {
    /// The shared range producer owns this exact short-suffix association.
    view: SuffixView<'batch, 'rows>,
}

impl fmt::Debug for InsufficientConditionedSuffix<'_, '_> {
    /// Redact rows and the complete parent chart's numerical payloads.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("InsufficientConditionedSuffix { .. }")
    }
}

impl<'batch, 'rows> InsufficientConditionedSuffix<'batch, 'rows> {
    /// Borrow the actual complete parent, including anchors for an empty suffix.
    #[must_use]
    pub const fn parent(&self) -> &'batch PreparedConditionedBatch<'rows> {
        self.view.parent
    }

    /// Return the unchanged original-population start.
    #[must_use]
    pub const fn original_start(&self) -> usize {
        self.view.start
    }

    /// Return exactly zero, one or two retained rows.
    #[must_use]
    pub const fn sample_count(&self) -> usize {
        self.view.count
    }

    /// Borrow only a retained prepared point; empty/out-of-range access is absent.
    #[must_use]
    pub fn point(&self, index: usize) -> Option<&PreparedConditionedPoint<'rows>> {
        self.view.point(index)
    }

    /// Borrow only a retained original row with its separate unchanged lifetime.
    #[must_use]
    pub fn original_row(&self, index: usize) -> Option<&'rows ExactObservationRow> {
        self.view.original_row(index)
    }
}

/// Exclusive complete suffix admission, with no partial fit-ready output.
#[derive(Debug)]
pub enum ConditionedSuffixOutcome<'batch, 'rows> {
    /// At least three retained rows may enter the separately checked fit.
    Ready(ConditionedSuffix<'batch, 'rows>),
    /// Zero, one or two retained rows preserve metadata without fitting.
    Insufficient(InsufficientConditionedSuffix<'batch, 'rows>),
}

/// Raw range admission; the tuple's shared predicate owns start and parent count.
#[derive(Validate)]
struct SuffixCandidate {
    /// Parent count is bounded before subtraction, traversal or original access.
    #[garde(custom(suffix_range_rule))]
    range: (usize, usize),
}

/// Decide the complete closed start range over a valid ready N1 population.
fn valid_suffix_range((start, count): (usize, usize)) -> bool {
    (3..=MAX_ROWS).contains(&count) && start <= count
}

/// Mandatory Garde adapter for the shared suffix-range predicate.
/// # Errors
/// Return fixed text without retaining a report or an input payload.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow raw tuples and their unit context"
)]
fn suffix_range_rule(range: &(usize, usize), (): &()) -> garde::Result {
    if valid_suffix_range(*range) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid conditioned suffix range"))
    }
}

/// Admit one complete contiguous suffix of the actual prepared population.
///
/// Start equal to the parent count produces an empty, metadata-retaining result.
/// Original scales, anchors and per-row divisor bits are never recomputed.
/// Successful work allocates nothing; a rejected temporary Garde report is bounded
/// and discarded. All input bits and parent/row lifetimes remain unchanged.
/// # Errors
/// Reject `start > parent.sample_count()` before subtraction or point access.
pub fn prepare_conditioned_suffix<'batch, 'rows>(
    parent: &'batch PreparedConditionedBatch<'rows>,
    start: usize,
) -> Result<ConditionedSuffixOutcome<'batch, 'rows>, ConditionedFitError> {
    let parent_count = parent.sample_count();
    let candidate = SuffixCandidate {
        range: (start, parent_count),
    };
    candidate.validate().map_err(|_| {
        if start > parent_count {
            ConditionedFitError::SuffixRange {
                start,
                count: parent_count,
            }
        } else if !valid_suffix_range(candidate.range) {
            ConditionedFitError::OutputInvariant
        } else {
            ConditionedFitError::ValidationInvariant
        }
    })?;
    let count = parent_count
        .checked_sub(start)
        .ok_or(ConditionedFitError::OutputInvariant)?;
    let view = SuffixView {
        parent,
        start,
        count,
    };
    if count < 3 {
        Ok(ConditionedSuffixOutcome::Insufficient(
            InsufficientConditionedSuffix { view },
        ))
    } else {
        Ok(ConditionedSuffixOutcome::Ready(ConditionedSuffix { view }))
    }
}

/// Enclosed dimensionless estimate in the actual retained normalized N1 chart.
///
/// Arithmetic bounds describe original-coordinate conversion and floating
/// arithmetic, without physical uncertainty or a relative-accuracy promise.
///
/// ```compile_fail,E0451
/// use time_core::ConditionedNormalizedEstimate;
/// fn construct(base: ConditionedNormalizedEstimate) {
///     let _ = ConditionedNormalizedEstimate { ..base };
/// }
/// ```
///
/// ```compile_fail,E0616
/// use time_core::ConditionedNormalizedEstimate;
/// fn mutate(mut base: ConditionedNormalizedEstimate) { base.enclosed = base.enclosed; }
/// ```
///
/// ```compile_fail,E0277
/// use time_core::ConditionedNormalizedEstimate;
/// let _: ConditionedNormalizedEstimate = Default::default();
/// ```
#[derive(Clone, Copy)]
pub struct ConditionedNormalizedEstimate {
    /// The private arithmetic producer owns finite hull/error representation.
    enclosed: ScalarEnclosure,
}

impl fmt::Debug for ConditionedNormalizedEstimate {
    /// Redact coefficient, interval endpoints and arithmetic-error magnitude.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ConditionedNormalizedEstimate { .. }")
    }
}

impl ConditionedNormalizedEstimate {
    /// Return the separately rounded computed point, with canonical positive zero.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.enclosed.value()
    }

    /// Return the finite lower endpoint containing point and exact target.
    #[must_use]
    pub const fn lower(self) -> f64 {
        self.enclosed.lower()
    }

    /// Return the finite upper endpoint containing point and exact target.
    #[must_use]
    pub const fn upper(self) -> f64 {
        self.enclosed.upper()
    }

    /// Bound absolute point-to-exact-target arithmetic error in normalized units.
    #[must_use]
    pub const fn absolute_error(self) -> f64 {
        self.enclosed.absolute_error()
    }
}

/// Enclosed phase correction in seconds relative to the separate exact anchor.
///
/// ```compile_fail,E0451
/// use time_core::ConditionedPhaseSeconds;
/// fn construct(base: ConditionedPhaseSeconds) {
///     let _ = ConditionedPhaseSeconds { ..base };
/// }
/// ```
///
/// ```compile_fail,E0616
/// use time_core::ConditionedPhaseSeconds;
/// fn mutate(mut base: ConditionedPhaseSeconds) { base.enclosed = base.enclosed; }
/// ```
///
/// ```compile_fail,E0277
/// use time_core::ConditionedPhaseSeconds;
/// let _: ConditionedPhaseSeconds = Default::default();
/// ```
#[derive(Clone, Copy)]
pub struct ConditionedPhaseSeconds {
    /// Checked finite lifted hull/error representation from the arithmetic owner.
    enclosed: ScalarEnclosure,
}

impl fmt::Debug for ConditionedPhaseSeconds {
    /// Redact the correction and arithmetic enclosure.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ConditionedPhaseSeconds { .. }")
    }
}

impl ConditionedPhaseSeconds {
    /// Return the separately rounded relative correction in seconds.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.enclosed.value()
    }

    /// Return the lower correction bound, excluding the separate exact anchor.
    #[must_use]
    pub const fn lower(self) -> f64 {
        self.enclosed.lower()
    }

    /// Return the upper correction bound, excluding the separate exact anchor.
    #[must_use]
    pub const fn upper(self) -> f64 {
        self.enclosed.upper()
    }

    /// Bound the correction's absolute arithmetic error in seconds.
    #[must_use]
    pub const fn absolute_error(self) -> f64 {
        self.enclosed.absolute_error()
    }
}

/// Enclosed phase-seconds per reference-second frequency ratio.
///
/// ```compile_fail,E0451
/// use time_core::ConditionedRateRatio;
/// fn construct(base: ConditionedRateRatio) {
///     let _ = ConditionedRateRatio { ..base };
/// }
/// ```
///
/// ```compile_fail,E0616
/// use time_core::ConditionedRateRatio;
/// fn mutate(mut base: ConditionedRateRatio) { base.enclosed = base.enclosed; }
/// ```
///
/// ```compile_fail,E0277
/// use time_core::ConditionedRateRatio;
/// let _: ConditionedRateRatio = Default::default();
/// ```
///
/// ```compile_fail,E0308
/// use time_core::{ConditionedPhaseSeconds, ConditionedRateRatio};
/// fn confuse_units(phase: ConditionedPhaseSeconds) {
///     let _: ConditionedRateRatio = phase;
/// }
/// ```
#[derive(Clone, Copy)]
pub struct ConditionedRateRatio {
    /// Checked finite lifted hull/error representation from the arithmetic owner.
    enclosed: ScalarEnclosure,
}

impl fmt::Debug for ConditionedRateRatio {
    /// Redact the computed frequency ratio and all arithmetic bounds.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ConditionedRateRatio { .. }")
    }
}

impl ConditionedRateRatio {
    /// Return the separately rounded frequency ratio.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.enclosed.value()
    }

    /// Return the finite lower frequency-ratio endpoint.
    #[must_use]
    pub const fn lower(self) -> f64 {
        self.enclosed.lower()
    }

    /// Return the finite upper frequency-ratio endpoint.
    #[must_use]
    pub const fn upper(self) -> f64 {
        self.enclosed.upper()
    }

    /// Bound absolute frequency-ratio arithmetic error.
    #[must_use]
    pub const fn absolute_error(self) -> f64 {
        self.enclosed.absolute_error()
    }
}

/// Enclosed inverse-divisor weighted residual sum in seconds squared.
///
/// This is an SSE, without degrees-of-freedom variance or confidence semantics.
///
/// ```compile_fail,E0451
/// use time_core::ConditionedWeightedSecondsSquared;
/// fn construct(base: ConditionedWeightedSecondsSquared) {
///     let _ = ConditionedWeightedSecondsSquared { ..base };
/// }
/// ```
///
/// ```compile_fail,E0616
/// use time_core::ConditionedWeightedSecondsSquared;
/// fn mutate(mut base: ConditionedWeightedSecondsSquared) { base.enclosed = base.enclosed; }
/// ```
///
/// ```compile_fail,E0277
/// use time_core::ConditionedWeightedSecondsSquared;
/// let _: ConditionedWeightedSecondsSquared = Default::default();
/// ```
#[derive(Clone, Copy)]
pub struct ConditionedWeightedSecondsSquared {
    /// Checked finite lifted hull/error representation from the arithmetic owner.
    enclosed: ScalarEnclosure,
}

impl fmt::Debug for ConditionedWeightedSecondsSquared {
    /// Redact the weighted SSE and arithmetic enclosure.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ConditionedWeightedSecondsSquared { .. }")
    }
}

impl ConditionedWeightedSecondsSquared {
    /// Return the separately rounded weighted SSE in seconds squared.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.enclosed.value()
    }

    /// Return the finite lower weighted-SSE endpoint in seconds squared.
    #[must_use]
    pub const fn lower(self) -> f64 {
        self.enclosed.lower()
    }

    /// Return the finite upper weighted-SSE endpoint in seconds squared.
    #[must_use]
    pub const fn upper(self) -> f64 {
        self.enclosed.upper()
    }

    /// Bound the weighted SSE's absolute arithmetic error in seconds squared.
    #[must_use]
    pub const fn absolute_error(self) -> f64 {
        self.enclosed.absolute_error()
    }
}

/// Atomic six-output fit borrowing the actual ready suffix and original rows.
///
/// Private construction binds all outputs to one completed computation. Immutable
/// checked scalar wrappers own representation; the fit producer owns the exact
/// mathematical relationship. No independent certificate/anchor attachment,
/// mutation, deserialization, default or legacy fixture conversion exists.
///
/// ```compile_fail,E0451
/// use time_core::ConditionedFit;
/// fn construct(base: ConditionedFit<'_, '_, '_>) {
///     let _ = ConditionedFit { ..base };
/// }
/// ```
///
/// ```compile_fail,E0616
/// use time_core::{ConditionedFit, ConditionedSuffix};
/// fn reattach<'fit, 'batch, 'rows>(mut fit: ConditionedFit<'fit, 'batch, 'rows>,
///     other: &'fit ConditionedSuffix<'batch, 'rows>)
/// {
///     fit.suffix = other;
/// }
/// ```
///
/// ```compile_fail,E0277
/// use time_core::ConditionedFit;
/// let _: ConditionedFit<'_, '_, '_> = Default::default();
/// ```
///
/// ```compile_fail,E0515
/// use time_core::{ConditionedFit, ConditionedFitError, ConditionedSuffix,
///     fit_conditioned_suffix};
/// fn escape<'batch, 'rows>(suffix: ConditionedSuffix<'batch, 'rows>)
///     -> Result<ConditionedFit<'batch, 'batch, 'rows>, ConditionedFitError>
/// {
///     fit_conditioned_suffix(&suffix)
/// }
/// ```
pub struct ConditionedFit<'fit, 'batch, 'rows> {
    /// Actual borrowed suffix; the fit cannot outlive it or its parent/rows.
    suffix: &'fit ConditionedSuffix<'batch, 'rows>,
    /// Enclosed normalized correction at the parent's newest reference.
    normalized_intercept: ConditionedNormalizedEstimate,
    /// Enclosed normalized phase per normalized reference.
    normalized_rate: ConditionedNormalizedEstimate,
    /// Enclosed normalized inverse-divisor residual sum of squares.
    normalized_weighted_sse: ConditionedNormalizedEstimate,
    /// Enclosed correction seconds, without an absolute phase conversion.
    phase_correction_seconds: ConditionedPhaseSeconds,
    /// Enclosed dimensionless frequency ratio.
    frequency_ratio: ConditionedRateRatio,
    /// Enclosed inverse-divisor weighted seconds-squared residual sum.
    weighted_seconds_squared: ConditionedWeightedSecondsSquared,
}

impl fmt::Debug for ConditionedFit<'_, '_, '_> {
    /// Redact rows, anchor, scales and all computed/output numeric payloads.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ConditionedFit { .. }")
    }
}

impl<'fit, 'batch, 'rows> ConditionedFit<'fit, 'batch, 'rows> {
    /// Borrow the actual ready suffix consumed by this completed fit.
    #[must_use]
    pub const fn suffix(&self) -> &'fit ConditionedSuffix<'batch, 'rows> {
        self.suffix
    }

    /// Borrow the actual full parent's newest row, retaining exact anchor bits.
    #[must_use]
    pub const fn newest_row(&self) -> &'rows ExactObservationRow {
        self.suffix.parent().newest_row()
    }

    /// Return the unchanged exact anchor numerator, never an absolute binary64.
    #[must_use]
    pub const fn exact_phase_anchor_numerator(&self) -> i128 {
        self.newest_row().corrected_phase_numerator()
    }

    /// Return the fixed denominator of the separate exact newest phase anchor.
    #[must_use]
    pub const fn exact_phase_anchor_denominator(&self) -> u128 {
        PHASE_DENOMINATOR
    }

    /// Return the enclosed normalized intercept relative to the exact anchor.
    #[must_use]
    pub const fn normalized_intercept(&self) -> ConditionedNormalizedEstimate {
        self.normalized_intercept
    }

    /// Return the enclosed normalized affine rate.
    #[must_use]
    pub const fn normalized_rate(&self) -> ConditionedNormalizedEstimate {
        self.normalized_rate
    }

    /// Return the enclosed normalized weighted SSE, without variance semantics.
    #[must_use]
    pub const fn normalized_weighted_sse(&self) -> ConditionedNormalizedEstimate {
        self.normalized_weighted_sse
    }

    /// Return correction seconds to be kept separate from the exact anchor.
    #[must_use]
    pub const fn phase_correction_seconds(&self) -> ConditionedPhaseSeconds {
        self.phase_correction_seconds
    }

    /// Return the enclosed lifted frequency ratio, with no clock authority.
    #[must_use]
    pub const fn frequency_ratio(&self) -> ConditionedRateRatio {
        self.frequency_ratio
    }

    /// Return the enclosed lifted weighted SSE in seconds squared.
    #[must_use]
    pub const fn weighted_seconds_squared(&self) -> ConditionedWeightedSecondsSquared {
        self.weighted_seconds_squared
    }
}

/// Fixed permitted diagnostic context for one actual numerical operation.
#[derive(Clone, Copy)]
struct ArithmeticContext {
    /// Stage owns sequencing and whether response arithmetic is admitted.
    stage: ConditionedFitStage,
    /// Original row index, absent for aggregate and output operations.
    index: Option<usize>,
}

impl ArithmeticContext {
    /// Aggregate-only fixed context, carrying no numeric payload.
    const fn aggregate(stage: ConditionedFitStage) -> Self {
        Self { stage, index: None }
    }

    /// Bind a retained local row to its checked original-population index.
    /// # Errors
    /// Reject an impossible private index contradiction without row access.
    fn row(
        stage: ConditionedFitStage,
        suffix: &ConditionedSuffix<'_, '_>,
        local_index: usize,
    ) -> Result<Self, ConditionedFitError> {
        let index = suffix
            .view
            .original_index(local_index)
            .ok_or(ConditionedFitError::OutputInvariant)?;
        Ok(Self {
            stage,
            index: Some(index),
        })
    }

    /// Project a checked private arithmetic result into bounded public context.
    /// # Errors
    /// Retain its typed cause without returning any operand or Garde report.
    fn apply<T>(
        self,
        operation: ConditionedFitOperation,
        result: Result<T, ArithmeticError>,
    ) -> Result<T, ConditionedFitError> {
        result.map_err(|cause| ConditionedFitError::Arithmetic {
            stage: self.stage,
            index: self.index,
            operation,
            issue: match cause {
                ArithmeticError::NonFinite => ConditionedFitIssue::NonFinite,
                ArithmeticError::NonPositiveDenominator => {
                    ConditionedFitIssue::NonPositiveDenominator
                }
                ArithmeticError::PointUnderflow => ConditionedFitIssue::PointUnderflow,
                ArithmeticError::InvalidInterval => ConditionedFitIssue::InvalidInterval,
                ArithmeticError::ErrorRange => ConditionedFitIssue::ErrorRange,
            },
        })
    }
}

/// One associated scratch row; only the checked active prefix is consumed.
#[derive(Clone, Copy)]
struct FitScratch {
    /// Once-rounded positive reciprocal; inactive tail entries are zero.
    reciprocal: f64,
    /// Exact retained binary64 divisor's reciprocal enclosure.
    reciprocal_target: Interval,
    /// Original exact reference enclosure seeded before geometry admission.
    reference_target: Interval,
    /// Original exact phase enclosure seeded only after geometry admission.
    phase_target: Interval,
}

/// A computed scalar paired privately with its original exact-target enclosure.
#[derive(Clone, Copy)]
struct StagedEstimate {
    /// Checked separately rounded computed point.
    point: f64,
    /// Enclosure propagated from original source coordinates and exact reciprocals.
    target: Interval,
}

/// Geometry-only pass result; its existence admits later response arithmetic.
struct PreparedGeometry {
    /// Associated fixed row scratch; only suffix-count entries are consumed.
    scratch: [FitScratch; MAX_ROWS],
    /// Chronological rounded and enclosed sum of reciprocal weights.
    total_weight: StagedEstimate,
    /// Chronological rounded and enclosed weighted reference mean.
    reference_mean: StagedEstimate,
    /// Strictly positive point and lower-enclosed centered geometry.
    geometry: StagedEstimate,
}

/// Retrieve only a checked active point, projecting impossible contradictions.
/// # Errors
/// Refuse absent internal points without falling back or dropping a row.
fn retained_point<'suffix, 'rows>(
    suffix: &'suffix ConditionedSuffix<'_, 'rows>,
    index: usize,
) -> Result<&'suffix PreparedConditionedPoint<'rows>, ConditionedFitError> {
    suffix
        .point(index)
        .ok_or(ConditionedFitError::OutputInvariant)
}

/// Complete pass one and pass two without reading or transforming any response.
/// # Errors
/// Refuse actual primitive failures or unavailable positive geometry before phase
/// arithmetic. The fixed scratch and every input remain private and unchanged.
fn prepare_geometry(
    suffix: &ConditionedSuffix<'_, '_>,
) -> Result<PreparedGeometry, ConditionedFitError> {
    use ConditionedFitOperation::{Add, Divide, Multiply, Seed, Square, Subtract};
    let mean_context = ArithmeticContext::aggregate(ConditionedFitStage::WeightsAndReferenceMean);
    let zero = mean_context.apply(Seed, Interval::exact(0.0))?;
    let one = mean_context.apply(Seed, Interval::exact(1.0))?;
    let blank = FitScratch {
        reciprocal: 0.0,
        reciprocal_target: zero,
        reference_target: zero,
        phase_target: zero,
    };
    let mut scratch = [blank; MAX_ROWS];
    let mut total_weight = StagedEstimate {
        point: 0.0,
        target: zero,
    };
    let mut weighted_reference = total_weight;
    for (local, saved) in scratch.iter_mut().take(suffix.sample_count()).enumerate() {
        let context =
            ArithmeticContext::row(ConditionedFitStage::WeightsAndReferenceMean, suffix, local)?;
        let point = retained_point(suffix, local)?;
        saved.reciprocal =
            context.apply(Divide, point_div_positive(1.0, point.variance_divisor()))?;
        let divisor = context.apply(Seed, Interval::exact(point.variance_divisor()))?;
        saved.reciprocal_target = context.apply(Divide, one.div_positive(divisor))?;
        saved.reference_target = context.apply(
            Seed,
            Interval::from_error(
                point.normalized_reference(),
                point.normalized_reference_roundoff(),
            ),
        )?;
        total_weight.point = context.apply(Add, point_add(total_weight.point, saved.reciprocal))?;
        total_weight.target =
            context.apply(Add, total_weight.target.add(saved.reciprocal_target))?;
        let product = context.apply(
            Multiply,
            point_mul(saved.reciprocal, point.normalized_reference()),
        )?;
        weighted_reference.point =
            context.apply(Add, point_add(weighted_reference.point, product))?;
        let target_product = context.apply(
            Multiply,
            saved.reciprocal_target.mul(saved.reference_target),
        )?;
        weighted_reference.target =
            context.apply(Add, weighted_reference.target.add(target_product))?;
    }
    let reference_mean = StagedEstimate {
        point: mean_context.apply(
            Divide,
            point_div_positive(weighted_reference.point, total_weight.point),
        )?,
        target: mean_context.apply(
            Divide,
            weighted_reference.target.div_positive(total_weight.target),
        )?,
    };
    let mut geometry = StagedEstimate {
        point: 0.0,
        target: zero,
    };
    for (local, saved) in scratch.iter().take(suffix.sample_count()).enumerate() {
        let context = ArithmeticContext::row(ConditionedFitStage::Geometry, suffix, local)?;
        let point = retained_point(suffix, local)?;
        let delta = context.apply(
            Subtract,
            point_sub(point.normalized_reference(), reference_mean.point),
        )?;
        let weighted_delta = context.apply(Multiply, point_mul(saved.reciprocal, delta))?;
        let contribution = context.apply(Multiply, point_mul(weighted_delta, delta))?;
        geometry.point = context.apply(Add, point_add(geometry.point, contribution))?;
        let delta_target =
            context.apply(Subtract, saved.reference_target.sub(reference_mean.target))?;
        let square_target = context.apply(Square, delta_target.square())?;
        let contribution_target =
            context.apply(Multiply, saved.reciprocal_target.mul(square_target))?;
        geometry.target = context.apply(Add, geometry.target.add(contribution_target))?;
    }
    if geometry.point <= 0.0 || geometry.target.lower() <= 0.0 {
        return Err(ConditionedFitError::UnresolvedGeometry);
    }
    Ok(PreparedGeometry {
        scratch,
        total_weight,
        reference_mean,
        geometry,
    })
}

/// Complete chronological pass three, using only saved same-row reciprocals.
/// # Errors
/// Preserve phase-seeding, multiplication, reduction and division refusals.
fn prepare_phase_mean(
    suffix: &ConditionedSuffix<'_, '_>,
    prepared: &mut PreparedGeometry,
) -> Result<StagedEstimate, ConditionedFitError> {
    use ConditionedFitOperation::{Add, Divide, Multiply, Seed};
    let aggregate = ArithmeticContext::aggregate(ConditionedFitStage::PhaseMean);
    let mut weighted_phase = StagedEstimate {
        point: 0.0,
        target: aggregate.apply(Seed, Interval::exact(0.0))?,
    };
    for (local, saved) in prepared
        .scratch
        .iter_mut()
        .take(suffix.sample_count())
        .enumerate()
    {
        let context = ArithmeticContext::row(ConditionedFitStage::PhaseMean, suffix, local)?;
        let point = retained_point(suffix, local)?;
        saved.phase_target = context.apply(
            Seed,
            Interval::from_error(point.normalized_phase(), point.normalized_phase_roundoff()),
        )?;
        let product = context.apply(
            Multiply,
            point_mul(saved.reciprocal, point.normalized_phase()),
        )?;
        weighted_phase.point = context.apply(Add, point_add(weighted_phase.point, product))?;
        let product_target =
            context.apply(Multiply, saved.reciprocal_target.mul(saved.phase_target))?;
        weighted_phase.target = context.apply(Add, weighted_phase.target.add(product_target))?;
    }
    Ok(StagedEstimate {
        point: aggregate.apply(
            Divide,
            point_div_positive(weighted_phase.point, prepared.total_weight.point),
        )?,
        target: aggregate.apply(
            Divide,
            weighted_phase
                .target
                .div_positive(prepared.total_weight.target),
        )?,
    })
}

/// Complete pass four with centered response and no constant-result shortcut.
/// # Errors
/// Preserve actual same-row covariance and aggregate coefficient failures.
fn prepare_coefficients(
    suffix: &ConditionedSuffix<'_, '_>,
    prepared: &PreparedGeometry,
    phase_mean: StagedEstimate,
) -> Result<(StagedEstimate, StagedEstimate), ConditionedFitError> {
    use ConditionedFitOperation::{Add, Divide, Multiply, Seed, Subtract};
    let aggregate = ArithmeticContext::aggregate(ConditionedFitStage::CovarianceAndCoefficients);
    let mut covariance = StagedEstimate {
        point: 0.0,
        target: aggregate.apply(Seed, Interval::exact(0.0))?,
    };
    for (local, saved) in prepared
        .scratch
        .iter()
        .take(suffix.sample_count())
        .enumerate()
    {
        let context = ArithmeticContext::row(
            ConditionedFitStage::CovarianceAndCoefficients,
            suffix,
            local,
        )?;
        let point = retained_point(suffix, local)?;
        let reference_delta = context.apply(
            Subtract,
            point_sub(point.normalized_reference(), prepared.reference_mean.point),
        )?;
        let phase_delta = context.apply(
            Subtract,
            point_sub(point.normalized_phase(), phase_mean.point),
        )?;
        let weighted_reference =
            context.apply(Multiply, point_mul(saved.reciprocal, reference_delta))?;
        let product = context.apply(Multiply, point_mul(weighted_reference, phase_delta))?;
        covariance.point = context.apply(Add, point_add(covariance.point, product))?;
        let reference_target = context.apply(
            Subtract,
            saved.reference_target.sub(prepared.reference_mean.target),
        )?;
        let phase_target = context.apply(Subtract, saved.phase_target.sub(phase_mean.target))?;
        let weighted_reference_target =
            context.apply(Multiply, saved.reciprocal_target.mul(reference_target))?;
        let product_target =
            context.apply(Multiply, weighted_reference_target.mul(phase_target))?;
        covariance.target = context.apply(Add, covariance.target.add(product_target))?;
    }
    let rate = StagedEstimate {
        point: aggregate.apply(
            Divide,
            point_div_positive(covariance.point, prepared.geometry.point),
        )?,
        target: aggregate.apply(
            Divide,
            covariance.target.div_positive(prepared.geometry.target),
        )?,
    };
    let reference_product = aggregate.apply(
        Multiply,
        point_mul(rate.point, prepared.reference_mean.point),
    )?;
    let reference_product_target =
        aggregate.apply(Multiply, rate.target.mul(prepared.reference_mean.target))?;
    let intercept = StagedEstimate {
        point: aggregate.apply(Subtract, point_sub(phase_mean.point, reference_product))?,
        target: aggregate.apply(Subtract, phase_mean.target.sub(reference_product_target))?,
    };
    Ok((intercept, rate))
}

/// Complete pass five with the frozen point order and original-target SSE chain.
/// # Errors
/// Refuse genuine residual/product/reduction underflow, overflow or enclosure loss.
fn prepare_residual_sum(
    suffix: &ConditionedSuffix<'_, '_>,
    prepared: &PreparedGeometry,
    intercept: StagedEstimate,
    rate: StagedEstimate,
) -> Result<StagedEstimate, ConditionedFitError> {
    use ConditionedFitOperation::{Add, Multiply, Seed, Square, Subtract};
    let aggregate = ArithmeticContext::aggregate(ConditionedFitStage::ResidualSum);
    let mut sum = StagedEstimate {
        point: 0.0,
        target: aggregate.apply(Seed, Interval::exact(0.0))?,
    };
    for (local, saved) in prepared
        .scratch
        .iter()
        .take(suffix.sample_count())
        .enumerate()
    {
        let context = ArithmeticContext::row(ConditionedFitStage::ResidualSum, suffix, local)?;
        let point = retained_point(suffix, local)?;
        let phase_difference = context.apply(
            Subtract,
            point_sub(point.normalized_phase(), intercept.point),
        )?;
        let prediction = context.apply(
            Multiply,
            point_mul(rate.point, point.normalized_reference()),
        )?;
        let residual = context.apply(Subtract, point_sub(phase_difference, prediction))?;
        let weighted_residual = context.apply(Multiply, point_mul(saved.reciprocal, residual))?;
        let contribution = context.apply(Multiply, point_mul(weighted_residual, residual))?;
        sum.point = context.apply(Add, point_add(sum.point, contribution))?;
        let prediction_target = context.apply(Multiply, rate.target.mul(saved.reference_target))?;
        let line_target = context.apply(Add, intercept.target.add(prediction_target))?;
        let residual_target = context.apply(Subtract, saved.phase_target.sub(line_target))?;
        let square_target = context.apply(Square, residual_target.square())?;
        let contribution_target =
            context.apply(Multiply, saved.reciprocal_target.mul(square_target))?;
        sum.target = context.apply(Add, sum.target.add(contribution_target))?;
    }
    Ok(sum)
}

/// Closed lift role owns its exponent envelope and fixed output stage.
#[derive(Clone, Copy)]
enum LiftRole {
    /// Relative phase seconds use the original phase scale.
    Phase,
    /// Frequency ratio uses original phase scale minus time scale.
    Rate,
    /// Weighted seconds squared use twice the original phase scale.
    Sse,
}

impl LiftRole {
    /// Return the fixed permitted exponent interval for this physical unit.
    const fn limits(self) -> (i32, i32) {
        match self {
            Self::Phase => (-62, 66),
            Self::Rate => (-97, 95),
            Self::Sse => (-124, 132),
        }
    }

    /// Return the corresponding aggregate-only public diagnostic stage.
    const fn stage(self) -> ConditionedFitStage {
        match self {
            Self::Phase => ConditionedFitStage::PhaseLifting,
            Self::Rate => ConditionedFitStage::RateLifting,
            Self::Sse => ConditionedFitStage::SseLifting,
        }
    }
}

/// Raw transformed exponent and its closed unit owner are validated together.
#[derive(Validate)]
struct LiftPowerCandidate {
    /// The shared role-specific predicate owns the complete admitted range.
    #[garde(custom(lift_power_rule))]
    power: (LiftRole, i32),
}

/// Decide the fixed range before checked binary exponent encoding.
fn valid_lift_power((role, exponent): (LiftRole, i32)) -> bool {
    let (minimum, maximum) = role.limits();
    (minimum..=maximum).contains(&exponent)
}

/// Mandatory Garde adapter for exact lifting-factor range admission.
/// # Errors
/// Return only fixed text without retaining an exponent or validator report.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow raw tuples and their unit context"
)]
fn lift_power_rule(power: &(LiftRole, i32), (): &()) -> garde::Result {
    if valid_lift_power(*power) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid conditioned lift exponent"))
    }
}

/// Encode an admitted normal exact power of two without floating exponentiation.
/// # Errors
/// Refuse raw exponent/range or checked-integer contradictions before lifting.
fn lifting_factor(role: LiftRole, exponent: i32) -> Result<f64, ConditionedFitError> {
    let candidate = LiftPowerCandidate {
        power: (role, exponent),
    };
    candidate.validate().map_err(|_| {
        if valid_lift_power(candidate.power) {
            ConditionedFitError::ValidationInvariant
        } else {
            ConditionedFitError::OutputInvariant
        }
    })?;
    let biased = exponent
        .checked_add(1023)
        .ok_or(ConditionedFitError::OutputInvariant)?;
    let encoding = u64::try_from(biased)
        .map_err(|_| ConditionedFitError::OutputInvariant)?
        .checked_shl(52)
        .ok_or(ConditionedFitError::OutputInvariant)?;
    Ok(f64::from_bits(encoding))
}

/// Publish one normalized estimate through the sole mandatory scalar validator.
/// # Errors
/// Refuse finite hull/error exhaustion; no caller-supplied certificate is accepted.
fn publish_normalized(
    estimate: StagedEstimate,
) -> Result<ConditionedNormalizedEstimate, ConditionedFitError> {
    let context = ArithmeticContext::aggregate(ConditionedFitStage::NormalizedOutput);
    let enclosed = context.apply(
        ConditionedFitOperation::Publish,
        ScalarEnclosure::new(estimate.point, estimate.target),
    )?;
    Ok(ConditionedNormalizedEstimate { enclosed })
}

/// Lift an original-target estimate and reconstruct a fresh checked hull/error.
/// # Errors
/// Refuse point underflow/overflow or enclosure/error-range exhaustion atomically.
fn lift_estimate(
    estimate: StagedEstimate,
    role: LiftRole,
    exponent: i32,
) -> Result<ScalarEnclosure, ConditionedFitError> {
    use ConditionedFitOperation::{Multiply, Publish, Seed};
    let context = ArithmeticContext::aggregate(role.stage());
    let factor = lifting_factor(role, exponent)?;
    let point = context.apply(Multiply, point_mul(estimate.point, factor))?;
    let factor_target = context.apply(Seed, Interval::exact(factor))?;
    let target = context.apply(Multiply, estimate.target.mul(factor_target))?;
    context.apply(Publish, ScalarEnclosure::new(point, target))
}

/// Fit the complete checked suffix and publish six enclosed estimates atomically.
///
/// Every reciprocal is computed once and retained by original row. The exact-real
/// target uses original half-tick/phase differences and retained binary64 divisors,
/// rather than idealized upstream weights or rounded-coordinate targets. Geometry
/// admission precedes phase arithmetic. All five passes use chronological left
/// reductions and separate binary64 primitives. The exact newest phase anchor
/// remains borrowed separately from the lifted correction. Successful work uses
/// fixed bounded scratch and allocates nothing; rejected Garde reports are bounded
/// temporary values. No input/history/authority/clock state changes.
/// # Errors
/// Return actual primitive failures, unavailable positive geometry or checked
/// output/lifting refusal. No partial normalized or lifted fit escapes.
/// # Examples
/// ```
/// use std::num::NonZeroU64;
/// use time_core::{ConditionedBatchOutcome, ConditionedSuffixOutcome,
///     DeclaredObservationContext, ExactObservationRow, fit_conditioned_suffix,
///     prepare_conditioned_batch, prepare_conditioned_suffix};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let tag = NonZeroU64::MIN;
/// let context = DeclaredObservationContext::new(tag, tag, tag, tag, tag);
/// let rows = [
///     ExactObservationRow::new(context, tag, 0, false, 7, 1.0)?,
///     ExactObservationRow::new(context, NonZeroU64::new(2).ok_or("key")?,
///         1_000_000_000, false, 7, 1.0)?,
///     ExactObservationRow::new(context, NonZeroU64::new(3).ok_or("key")?,
///         2_000_000_000, false, 7, 1.0)?,
/// ];
/// let ConditionedBatchOutcome::Prepared(parent) =
///     prepare_conditioned_batch(context, &rows, 1.0)?
/// else { return Err("expected prepared parent".into()); };
/// let ConditionedSuffixOutcome::Ready(suffix) = prepare_conditioned_suffix(&parent, 0)?
/// else { return Err("expected ready suffix".into()); };
/// let fit = fit_conditioned_suffix(&suffix)?;
/// assert!(std::ptr::eq(fit.suffix(), &suffix));
/// assert!(std::ptr::eq(fit.newest_row(), &rows[2]));
/// assert_eq!(fit.exact_phase_anchor_numerator(), 7);
/// assert_eq!(fit.phase_correction_seconds().value().to_bits(), 0);
/// assert_eq!(fit.frequency_ratio().value().to_bits(), 0);
/// let ConditionedSuffixOutcome::Insufficient(empty) =
///     prepare_conditioned_suffix(&parent, parent.sample_count())?
/// else { return Err("expected empty suffix".into()); };
/// assert_eq!(empty.sample_count(), 0);
/// assert!(std::ptr::eq(empty.parent(), &parent));
/// # Ok(()) }
/// ```
pub fn fit_conditioned_suffix<'fit, 'batch, 'rows>(
    suffix: &'fit ConditionedSuffix<'batch, 'rows>,
) -> Result<ConditionedFit<'fit, 'batch, 'rows>, ConditionedFitError> {
    let mut prepared = prepare_geometry(suffix)?;
    let phase_mean = prepare_phase_mean(suffix, &mut prepared)?;
    let (intercept, rate) = prepare_coefficients(suffix, &prepared, phase_mean)?;
    let weighted_sse = prepare_residual_sum(suffix, &prepared, intercept, rate)?;
    let normalized_intercept = publish_normalized(intercept)?;
    let normalized_rate = publish_normalized(rate)?;
    let normalized_weighted_sse = publish_normalized(weighted_sse)?;
    let phase_exponent = suffix.parent().phase_scale_exponent();
    let rate_exponent = phase_exponent
        .checked_sub(suffix.parent().time_scale_exponent())
        .ok_or(ConditionedFitError::OutputInvariant)?;
    let sse_exponent = phase_exponent
        .checked_mul(2)
        .ok_or(ConditionedFitError::OutputInvariant)?;
    let phase_correction_seconds = ConditionedPhaseSeconds {
        enclosed: lift_estimate(intercept, LiftRole::Phase, phase_exponent)?,
    };
    let frequency_ratio = ConditionedRateRatio {
        enclosed: lift_estimate(rate, LiftRole::Rate, rate_exponent)?,
    };
    let weighted_seconds_squared = ConditionedWeightedSecondsSquared {
        enclosed: lift_estimate(weighted_sse, LiftRole::Sse, sse_exponent)?,
    };
    Ok(ConditionedFit {
        suffix,
        normalized_intercept,
        normalized_rate,
        normalized_weighted_sse,
        phase_correction_seconds,
        frequency_ratio,
        weighted_seconds_squared,
    })
}

/// Direct raw-validator, diagnostic and late-lifting controls supplement oracles.
#[cfg(test)]
#[path = "../tests/private/conditioned_fit.rs"]
mod tests;

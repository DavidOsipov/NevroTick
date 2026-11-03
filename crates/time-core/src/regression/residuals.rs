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
//! Private owner for same-fit residual diagnostics and coefficient standard errors.
//!
//! Modified by David Osipov with AI assistance, 30 October 2026. The guarded
//! operation sequence adapts selected Chrony 4.9 `RGR_FindBestRegression`
//! statements at regress.c 277-280 and 303-312, pinned revision
//! 8df4f1263e206585e6c4e61352d042c2dd12916a. The actual source-statistics caller
//! chooses that function; the supplemental weighted-regression function has a
//! different normalization association. See ../../PROVENANCE.md and the additive
//! private source ledger for retained notices and deliberately omitted behavior.
//! Modified by David Osipov with AI assistance, 3 November 2026, to retain the
//! same-fit geometry and expose explicit coefficient standard errors from
//! regress.c 309-311. Physical-time geometry is reconstructed from the existing
//! normalized geometry; this adaptation claims no Chrony bit parity.
//!
//! The sole public producer fits one immutable input exactly once, then computes
//! chronological residuals using that returned line and prepared input geometry.
//! All three squared-second outputs pass mandatory private garde construction;
//! the complete owned report is published only after every operation succeeds.
//! One bounded arithmetic pass follows the fit, using fixed scalar scratch with
//! no input mutation, I/O or other external effects. Scalar validation owns only
//! a bounded one-field temporary report. Fixed report fields are below 512 logical
//! bytes; this is no allocation, RSS, native-resource or uncertainty qualification.
//! The separate standard-error method uses eight checked binary primitives and
//! two checked roots, then mandatory rate/phase admission. It retains no input
//! borrow, performs no fit or scan, and does not alter the residual producer.

use std::{fmt, num::NonZeroU64};

use garde::Validate;

use super::{
    ArithmeticOperation, ArithmeticStage, BatchPoints, FitError, FitInput, FitPoint, FixtureLine,
    Geometry, arithmetic, checked_square_root, fit, runs::TerminalCandidate,
};

/// Published algebraic diagnostic, with squared-second units for every variant.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResidualField {
    /// Chronological sum of squared residuals divided by their fixture divisors.
    WeightedSse,
    /// Weighted SSE divided by the checked sample count minus two.
    ResidualScale,
    /// Residual scale times sample count, then divided by inverse-divisor sum W.
    NormalizedDataVariance,
}

/// First violated statistic invariant, in the documented precedence order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResidualIssue {
    /// Raw output is NaN or either infinity.
    NonFinite,
    /// Raw nonzero output is subnormal, before checking its sign.
    Subnormal,
    /// Finite normal output is negative; either signed zero remains legal.
    Negative,
}

/// Exact producer step for one separate response-derived binary64 primitive.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResidualStep {
    /// Subtract returned phase A from point phase y.
    PhaseDifference,
    /// Multiply returned physical rate B by raw relative seconds x.
    Trend,
    /// Subtract trend from the already computed y-A difference.
    Residual,
    /// Multiply a residual by itself.
    Square,
    /// Divide the square by its point's dimensionless variance divisor.
    Weight,
    /// Add the weighted square to the chronological SSE accumulator.
    Accumulate,
    /// Divide SSE by the checked degrees of freedom n-2.
    Scale,
    /// Multiply residual scale by the checked sample count n.
    NormalizeMultiply,
    /// Divide the preceding product by prepared inverse-divisor sum W.
    NormalizeDivide,
}

impl ResidualStep {
    /// Map a closed step to its sole primitive, without arithmetic or effects.
    ///
    /// Primitive callers cannot independently relabel the operation. This is the
    /// one exhaustive step-to-operation owner for all residual arithmetic.
    const fn operation(self) -> ArithmeticOperation {
        match self {
            Self::PhaseDifference | Self::Residual => ArithmeticOperation::Subtract,
            Self::Trend | Self::Square | Self::NormalizeMultiply => ArithmeticOperation::Multiply,
            Self::Weight | Self::Scale | Self::NormalizeDivide => ArithmeticOperation::Divide,
            Self::Accumulate => ArithmeticOperation::Add,
        }
    }
}

/// Bounded typed residual rejection; no raw numeric input or library report.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResidualError {
    /// The fresh parent fit failed before any residual work or report publication.
    Fit(FitError),
    /// One residual primitive failed, preserving its step and actual typed cause.
    Arithmetic {
        /// Exact residual producer, distinct even for repeated Divide operations.
        step: ResidualStep,
        /// Actual shared Response-stage arithmetic rejection, without relabelling.
        source: FitError,
    },
    /// First unsupported output invariant for the named squared-second scalar.
    InvalidStatistic {
        /// Published scalar role; this is never an attacker-selected field name.
        field: ResidualField,
        /// Shared nonfinite/subnormal/negative predicate result.
        issue: ResidualIssue,
    },
    /// Mandatory garde disagreed with the shared scalar predicate for this field.
    ValidationInvariant {
        /// The sole statistic being privately admitted.
        field: ResidualField,
    },
    /// Checked count/degree conversion disagreed with its immutable input owner.
    CountInvariant,
}

impl fmt::Display for ResidualError {
    /// Format bounded type/step/field/issue context without raw values or reports.
    ///
    /// Every public enum construction is supported, including a manually supplied
    /// non-arithmetic source. The parent's bounded typed formatting exposes no
    /// raw floating values or library reports; the cause is also borrowable.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fit(source) => write!(formatter, "fresh fixture fit rejected: {source}"),
            Self::Arithmetic { step, source } => {
                write!(formatter, "residual arithmetic {step:?}: {source}")
            }
            Self::InvalidStatistic { field, issue } => {
                write!(formatter, "residual statistic {field:?}: {issue:?}")
            }
            Self::ValidationInvariant { field } => {
                write!(formatter, "residual validation invariant {field:?}")
            }
            Self::CountInvariant => formatter.write_str("residual count invariant disagreement"),
        }
    }
}

impl std::error::Error for ResidualError {
    /// Borrow the retained typed cause for every source-bearing public variant.
    ///
    /// This total projection performs no parsing, allocation, filtering or effects.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Fit(source) | Self::Arithmetic { source, .. } => Some(source),
            Self::InvalidStatistic { .. }
            | Self::ValidationInvariant { .. }
            | Self::CountInvariant => None,
        }
    }
}

/// Coefficient diagnostic role; rate admission precedes phase admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureStandardErrorField {
    /// Fitted rate standard error in seconds per second.
    Rate,
    /// Fitted phase standard error in seconds at the original zero reference.
    Phase,
}

/// Separate checked primitive in the frozen same-fit standard-error sequence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureStandardErrorStep {
    /// Multiply the retained span by itself in the Geometry stage.
    SpanSquare,
    /// Multiply normalized centered spread by squared span in Geometry.
    CenteredTimeSquare,
    /// Divide residual scale by centered physical-time spread in Response.
    RateVariance,
    /// Take the checked rate-variance square root in Response.
    RateSquareRoot,
    /// Multiply normalized mean time by span in Geometry.
    TimeMean,
    /// Multiply mean physical time by rate standard error in Response.
    PhaseMeanProduct,
    /// Square the preceding phase contribution in Response.
    PhaseMeanSquare,
    /// Divide residual scale by inverse-divisor weight sum in Response.
    PhaseBaseVariance,
    /// Add the base variance and squared mean-time contribution in Response.
    PhaseVariance,
    /// Take the checked phase-variance square root in Response.
    PhaseSquareRoot,
}

/// Bounded typed standard-error rejection without raw geometry or numeric values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureStandardError {
    /// One binary primitive or root failed, retaining its actual typed cause.
    Arithmetic {
        /// Exact primitive in the new diagnostic sequence.
        step: FixtureStandardErrorStep,
        /// Actual shared arithmetic or square-root error, without relabelling.
        source: FitError,
    },
    /// The shared output predicate rejected one final standard error.
    InvalidStatistic {
        /// Quantity role, carrying its distinct phase or rate units.
        field: FixtureStandardErrorField,
        /// Shared nonfinite/subnormal/negative predicate result.
        issue: ResidualIssue,
    },
    /// Mandatory Garde disagreed with the shared output predicate.
    ValidationInvariant {
        /// Quantity whose private scalar construction rejected publication.
        field: FixtureStandardErrorField,
    },
}

impl fmt::Display for FixtureStandardError {
    /// Format bounded role/step/cause metadata, never raw geometry or reports.
    ///
    /// Manually supplied public causes remain supported and use their existing
    /// bounded typed formatting; no error text is parsed or stored.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Arithmetic { step, source } => {
                write!(
                    formatter,
                    "fixture standard-error arithmetic {step:?}: {source}"
                )
            }
            Self::InvalidStatistic { field, issue } => {
                write!(
                    formatter,
                    "fixture standard-error statistic {field:?}: {issue:?}"
                )
            }
            Self::ValidationInvariant { field } => {
                write!(
                    formatter,
                    "fixture standard-error validation invariant {field:?}"
                )
            }
        }
    }
}

impl std::error::Error for FixtureStandardError {
    /// Borrow every retained public cause unchanged; other variants have no cause.
    ///
    /// This total projection performs no arithmetic, allocation or filtering.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Arithmetic { source, .. } => Some(source),
            Self::InvalidStatistic { .. } | Self::ValidationInvariant { .. } => None,
        }
    }
}

/// Private finite normal nonnegative or signed-zero coefficient-error scalar.
///
/// Its containing result field owns the phase-seconds or rate-seconds/second
/// meaning. Only mandatory shared Garde admission may publish a value.
#[derive(Clone, Copy)]
struct StandardErrorScalar {
    /// Original admitted scalar bits, without normalization or a positive floor.
    value: f64,
}

impl StandardErrorScalar {
    /// Admit any raw scalar through the existing shared Garde rule and predicate.
    ///
    /// Field selects only the error's units/role; all raw binary64 values are
    /// supported inputs to this check. Validation always runs before publishing
    /// unchanged bits. One bounded one-field temporary report is discarded.
    /// # Errors
    /// Return `NonFinite`, `Subnormal` or `Negative` in shared predicate order, then
    /// an otherwise unexplained Garde disagreement. No partial value escapes.
    fn new(value: f64, field: FixtureStandardErrorField) -> Result<Self, FixtureStandardError> {
        let validation = StatisticCandidate { value }.validate();
        if let Some(issue) = residual_issue(value) {
            return Err(FixtureStandardError::InvalidStatistic { field, issue });
        }
        validation.map_err(|_| FixtureStandardError::ValidationInvariant { field })?;
        Ok(Self { value })
    }
}

/// Binary-only step selection prevents callers from relabelling operations/stages.
#[derive(Clone, Copy)]
enum StandardErrorBinaryStep {
    /// Checked squared physical span.
    SpanSquare,
    /// Checked centered physical-time spread.
    CenteredTimeSquare,
    /// Residual scale divided by centered physical-time spread.
    RateVariance,
    /// Checked physical mean time.
    TimeMean,
    /// Mean time multiplied by fitted rate standard error.
    PhaseMeanProduct,
    /// Squared mean-time/rate-error contribution.
    PhaseMeanSquare,
    /// Residual scale divided by inverse-divisor weight sum.
    PhaseBaseVariance,
    /// Sum of base phase variance and squared mean-time contribution.
    PhaseVariance,
}

impl StandardErrorBinaryStep {
    /// Map every binary-only step to its sole public step, primitive and stage.
    ///
    /// Exhaustive mapping has constant work, no arithmetic or effects, and no
    /// unsupported square-root variant requiring a synthetic error fallback.
    const fn metadata(
        self,
    ) -> (
        FixtureStandardErrorStep,
        ArithmeticOperation,
        ArithmeticStage,
    ) {
        match self {
            Self::SpanSquare => (
                FixtureStandardErrorStep::SpanSquare,
                ArithmeticOperation::Multiply,
                ArithmeticStage::Geometry,
            ),
            Self::CenteredTimeSquare => (
                FixtureStandardErrorStep::CenteredTimeSquare,
                ArithmeticOperation::Multiply,
                ArithmeticStage::Geometry,
            ),
            Self::RateVariance => (
                FixtureStandardErrorStep::RateVariance,
                ArithmeticOperation::Divide,
                ArithmeticStage::Response,
            ),
            Self::TimeMean => (
                FixtureStandardErrorStep::TimeMean,
                ArithmeticOperation::Multiply,
                ArithmeticStage::Geometry,
            ),
            Self::PhaseMeanProduct => (
                FixtureStandardErrorStep::PhaseMeanProduct,
                ArithmeticOperation::Multiply,
                ArithmeticStage::Response,
            ),
            Self::PhaseMeanSquare => (
                FixtureStandardErrorStep::PhaseMeanSquare,
                ArithmeticOperation::Multiply,
                ArithmeticStage::Response,
            ),
            Self::PhaseBaseVariance => (
                FixtureStandardErrorStep::PhaseBaseVariance,
                ArithmeticOperation::Divide,
                ArithmeticStage::Response,
            ),
            Self::PhaseVariance => (
                FixtureStandardErrorStep::PhaseVariance,
                ArithmeticOperation::Add,
                ArithmeticStage::Response,
            ),
        }
    }
}

/// Evaluate one mapped binary primitive over independently checked raw operands.
///
/// Every raw binary64 pair is admitted to the shared arithmetic guard. The step
/// owns operation/stage; no report association or prior scalar check is assumed.
/// There is no allocation, mutation or partially published diagnostic.
/// # Errors
/// Retain the exact shared arithmetic cause under the mapped public step.
fn standard_error_arithmetic(
    left: f64,
    right: f64,
    step: StandardErrorBinaryStep,
) -> Result<f64, FixtureStandardError> {
    let (step, operation, stage) = step.metadata();
    arithmetic(left, right, operation, stage)
        .map_err(|source| FixtureStandardError::Arithmetic { step, source })
}

/// Evaluate one checked root over the complete raw binary64 domain.
///
/// Role chooses only `RateSquareRoot` or `PhaseSquareRoot`; both belong to Response.
/// The existing root owner checks its input/result and preserves signed zeros.
/// There is no allocation, mutation, clamp or independent result publication.
/// # Errors
/// Preserve the actual shared `SquareRoot` cause with its corresponding step.
fn standard_error_square_root(
    value: f64,
    field: FixtureStandardErrorField,
) -> Result<f64, FixtureStandardError> {
    let step = match field {
        FixtureStandardErrorField::Rate => FixtureStandardErrorStep::RateSquareRoot,
        FixtureStandardErrorField::Phase => FixtureStandardErrorStep::PhaseSquareRoot,
    };
    checked_square_root(value, ArithmeticStage::Response)
        .map_err(|source| FixtureStandardError::Arithmetic { step, source })
}

/// Immutable conditional standard errors for one original fitted population.
///
/// Phase refers to that fit's original zero-reference instant. These algebraic
/// diagnostics imply no calibrated confidence, tracking skew, aged uncertainty,
/// eligibility or clock authority. There is no public constructor, mutation,
/// Default or decoding route. Fixed fields remain below 512 logical bytes.
/// The existing producer and immutable getters are the legal public route;
/// metadata cannot be rewritten independently of its fitted population:
/// ```compile_fail
/// use time_core::FixtureStandardErrors;
/// fn replace_population(mut result: FixtureStandardErrors) {
///     result.sample_count = 0;
/// }
/// ```
/// A default cannot fabricate a report without a fitted population:
/// ```compile_fail
/// use time_core::FixtureStandardErrors;
/// let result = FixtureStandardErrors::default();
/// ```
#[derive(Clone, Copy)]
pub struct FixtureStandardErrors {
    /// Same checked fitted report's asserted frame, without new authority.
    frame: NonZeroU64,
    /// Same report's current fitted population in three through 64.
    sample_count: usize,
    /// Same report's exact sample count minus two.
    degrees_of_freedom: usize,
    /// Standard error in seconds at the fitted zero-reference instant.
    phase: StandardErrorScalar,
    /// Standard error in seconds per second of the fitted physical rate.
    rate: StandardErrorScalar,
}

impl fmt::Debug for FixtureStandardErrors {
    /// Format identity/count metadata only, excluding both numeric diagnostics.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FixtureStandardErrors")
            .field("frame", &self.frame)
            .field("sample_count", &self.sample_count)
            .field("degrees_of_freedom", &self.degrees_of_freedom)
            .finish_non_exhaustive()
    }
}

impl FixtureStandardErrors {
    /// Return the original fitted report's asserted frame without new authority.
    #[must_use]
    pub const fn frame(&self) -> NonZeroU64 {
        self.frame
    }

    /// Return the original fitted population without scanning or effects.
    #[must_use]
    pub const fn sample_count(&self) -> usize {
        self.sample_count
    }

    /// Return the original fitted count minus two without new arithmetic.
    #[must_use]
    pub const fn degrees_of_freedom(&self) -> usize {
        self.degrees_of_freedom
    }

    /// Return the admitted phase standard error in seconds, preserving its bits.
    #[must_use]
    pub const fn phase_standard_error_seconds(&self) -> f64 {
        self.phase.value
    }

    /// Return admitted rate standard error in seconds per second unchanged.
    #[must_use]
    pub const fn rate_standard_error_seconds_per_second(&self) -> f64 {
        self.rate.value
    }
}

/// Sole output predicate: finite normal nonnegative values or either signed zero.
///
/// All raw f64 values are admitted for this constant-work, effect-free check.
/// It preserves `NonFinite` -> `Subnormal` -> `Negative` precedence without changing
/// any bits, reusing point-input bounds, formatting data or clamping a result.
fn residual_issue(value: f64) -> Option<ResidualIssue> {
    if !value.is_finite() {
        return Some(ResidualIssue::NonFinite);
    }
    if value.is_subnormal() {
        return Some(ResidualIssue::Subnormal);
    }
    if value < 0.0 {
        return Some(ResidualIssue::Negative);
    }
    None
}

/// Adapt the same output predicate to garde's borrowed custom-rule signature.
/// # Errors
/// Rejects an unsupported raw scalar with a fixed, payload-free library error.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rule signature borrows its raw field and context"
)]
fn statistic_rule(value: &f64, (): &()) -> garde::Result {
    if residual_issue(*value).is_none() {
        Ok(())
    } else {
        Err(garde::Error::new("invalid fixture residual statistic"))
    }
}

/// One raw statistic, covered by the shared garde rule with no skipped fields.
#[derive(Validate)]
struct StatisticCandidate {
    /// Raw nonnegative diagnostic; the checked wrapper owns its physical units.
    /// The shared rule permits values above the point-input range.
    #[garde(custom(statistic_rule))]
    value: f64,
}

/// Immutable private squared-second scalar admitted through the sole constructor.
#[derive(Clone, Copy)]
struct ResidualScalar {
    /// Original finite normal nonnegative value or signed zero, with bits retained.
    value: f64,
}

impl ResidualScalar {
    /// Unavoidably run garde, then publish only an admitted scalar unchanged.
    ///
    /// Every field shares this constructor and predicate. Validation is constant
    /// work over one fixed field; the temporary report never leaves construction.
    /// # Errors
    /// Returns the known typed issue before an unexplained garde disagreement.
    fn new(value: f64, field: ResidualField) -> Result<Self, ResidualError> {
        let candidate = StatisticCandidate { value };
        let validation = candidate.validate();
        if let Some(issue) = residual_issue(value) {
            return Err(ResidualError::InvalidStatistic { field, issue });
        }
        validation.map_err(|_| ResidualError::ValidationInvariant { field })?;
        Ok(Self { value })
    }
}

/// Perform the primitive selected only by step, preserving the shared typed cause.
///
/// The complete raw f64 operand domain is checked by the shared arithmetic owner:
/// nonfinite/subnormal operands, denominator positivity, unsupported results and
/// nonzero multiplication/division lost to zero. Legal cancellation and either
/// signed zero are preserved. One separate ordinary primitive, constant work and
/// no I/O/state/report publication occur; no hidden same-input promise is needed.
/// # Errors
/// Wraps the actual Response-stage `FitError::Arithmetic` with the exact step.
fn residual_arithmetic(left: f64, right: f64, step: ResidualStep) -> Result<f64, ResidualError> {
    arithmetic(left, right, step.operation(), ArithmeticStage::Response)
        .map_err(|source| ResidualError::Arithmetic { step, source })
}

/// Evaluate one supplied checked point and line, without claiming their association.
///
/// The separately checked order is phase difference, trend and residual. This
/// constant-work helper has no mutation or publication; callers own association.
/// # Errors
/// Returns the first unsupported primitive with its exact residual step.
pub(super) fn checked_residual(point: &FitPoint, line: &FixtureLine) -> Result<f64, ResidualError> {
    let phase_difference = residual_arithmetic(
        point.phase_seconds,
        line.phase_seconds,
        ResidualStep::PhaseDifference,
    )?;
    let trend = residual_arithmetic(
        line.phase_rate_seconds_per_second,
        point.relative_seconds,
        ResidualStep::Trend,
    )?;
    residual_arithmetic(phase_difference, trend, ResidualStep::Residual)
}

/// Sum checked weighted residual squares for an explicit point view and line.
///
/// The immutable view owns its 3..64 checked points; evaluation preserves its
/// supplied order, which the public producer's `FitInput` owns as chronological.
/// Any checked `FixtureLine` is accepted, including one fitted to different data
/// or a different frame. This helper promises the sum for that supplied pair,
/// without an ordering, optimum or same-input precondition. Each point evaluates
/// `y-A`, physical `B*x`, their difference, its square, `square/d`, then the
/// sum in the supplied order from positive zero. Returned raw SSE has squared-
/// second units and is finite normal nonnegative or positive zero. Publication still
/// requires the shared private scalar constructor. Work is O(n), n<=64, with
/// fixed scalar scratch and no mutation or external effects. This is no measured
/// allocation or RSS guarantee for validation and diagnostics.
/// # Errors
/// Returns the first unsupported primitive with its exact step and typed cause;
/// no partial sum or report is published.
fn weighted_sse(batch: &BatchPoints<'_>, line: &FixtureLine) -> Result<f64, ResidualError> {
    let mut sum = 0.0;
    for point in batch.values {
        let residual = checked_residual(point, line)?;
        let square = residual_arithmetic(residual, residual, ResidualStep::Square)?;
        let weighted = residual_arithmetic(square, point.variance_divisor, ResidualStep::Weight)?;
        sum = residual_arithmetic(sum, weighted, ResidualStep::Accumulate)?;
    }
    Ok(sum)
}

/// Immutable fixture diagnostics for one freshly fitted, unchanged input.
///
/// The sole public producer owns its fresh returned line and diagnostics for the
/// same immutable borrowed input; matching frame numbers alone cannot establish
/// that association. Fields are private with no Default, raw factory, mutation or
/// deserialization. All three scalars use mandatory private garde admission.
/// Fixed owned fields are below 512 logical bytes; no allocation or RSS claim.
#[derive(Clone, Copy)]
pub struct FixtureResidualStatistics {
    /// Exact owned line returned by the producer's one same-input parent fit.
    line: FixtureLine,
    /// Immutable sample count, checked against the same input's 3..64 owner.
    sample_count: usize,
    /// Immutable degrees of freedom, exactly `sample_count-2` in 1..62.
    degrees_of_freedom: usize,
    /// Privately admitted chronological weighted residual SSE, in seconds squared.
    weighted_sse: ResidualScalar,
    /// Privately admitted SSE/(n-2), in seconds squared.
    residual_scale: ResidualScalar,
    /// Privately admitted (scale*n)/W, in seconds squared.
    normalized_data_variance: ResidualScalar,
    /// Exact producing input's checked time/divisor geometry; four scalar copies.
    geometry: Geometry,
}

impl fmt::Debug for FixtureResidualStatistics {
    /// Format type/frame/count/dof only; exclude line coefficients and statistics.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FixtureResidualStatistics")
            .field("frame", &self.frame())
            .field("sample_count", &self.sample_count)
            .field("degrees_of_freedom", &self.degrees_of_freedom)
            .finish_non_exhaustive()
    }
}

impl FixtureResidualStatistics {
    /// Borrow the exact owned fresh line without mutation, arithmetic or effects.
    #[must_use]
    pub const fn line(&self) -> &FixtureLine {
        &self.line
    }

    /// Return the contained line's asserted fixture frame without new authority.
    #[must_use]
    pub const fn frame(&self) -> NonZeroU64 {
        self.line.frame()
    }

    /// Return the immutable checked sample count without scanning or effects.
    #[must_use]
    pub const fn sample_count(&self) -> usize {
        self.sample_count
    }

    /// Return the immutable checked n-2 count without arithmetic or effects.
    #[must_use]
    pub const fn degrees_of_freedom(&self) -> usize {
        self.degrees_of_freedom
    }

    /// Return admitted weighted residual SSE in squared seconds, preserving bits.
    #[must_use]
    pub const fn weighted_sse_seconds_squared(&self) -> f64 {
        self.weighted_sse.value
    }

    /// Return admitted SSE/(n-2) in squared seconds without transformation.
    #[must_use]
    pub const fn residual_scale_seconds_squared(&self) -> f64 {
        self.residual_scale.value
    }

    /// Return admitted (scale*n)/W in squared seconds without transformation.
    #[must_use]
    pub const fn normalized_data_variance_seconds_squared(&self) -> f64 {
        self.normalized_data_variance.value
    }

    /// Calculate conditional phase/rate standard errors from this report alone.
    ///
    /// The retained geometry belongs to the exact input that produced the line
    /// and residual scale; no unrelated input or line can be supplied. Compute
    /// `H*H, Szz*H_squared, scale/V, rate root, mean_z*H, mean_time*rate_error`,
    /// that product squared, scale/W, their phase-variance sum and phase root in
    /// that order. Each primitive is separately rounded and guarded. The scale
    /// is SSE/(n-2), never normalized data variance; no FMA or clamp is used.
    ///
    /// Rate then phase pass mandatory shared Garde admission before the complete
    /// pair is published with unchanged frame/count/degrees of freedom. There
    /// are eight binary primitives, two roots and two scalar admissions, with
    /// constant scalar scratch and no refit, scan, retained allocation, report
    /// mutation or external effect. Garde's bounded temporary report is discarded.
    /// # Errors
    /// Return the first unsupported primitive/root in the stated order, then
    /// the first rate/phase scalar admission error. No partial pair escapes.
    pub fn standard_errors(&self) -> Result<FixtureStandardErrors, FixtureStandardError> {
        let span_squared = standard_error_arithmetic(
            self.geometry.span,
            self.geometry.span,
            StandardErrorBinaryStep::SpanSquare,
        )?;
        let centered_time_square = standard_error_arithmetic(
            self.geometry.centered_z_square,
            span_squared,
            StandardErrorBinaryStep::CenteredTimeSquare,
        )?;
        let rate_variance = standard_error_arithmetic(
            self.residual_scale.value,
            centered_time_square,
            StandardErrorBinaryStep::RateVariance,
        )?;
        let rate_error =
            standard_error_square_root(rate_variance, FixtureStandardErrorField::Rate)?;
        let mean_time = standard_error_arithmetic(
            self.geometry.mean_z,
            self.geometry.span,
            StandardErrorBinaryStep::TimeMean,
        )?;
        let mean_product = standard_error_arithmetic(
            mean_time,
            rate_error,
            StandardErrorBinaryStep::PhaseMeanProduct,
        )?;
        let mean_square = standard_error_arithmetic(
            mean_product,
            mean_product,
            StandardErrorBinaryStep::PhaseMeanSquare,
        )?;
        let base_variance = standard_error_arithmetic(
            self.residual_scale.value,
            self.geometry.weight,
            StandardErrorBinaryStep::PhaseBaseVariance,
        )?;
        let phase_variance = standard_error_arithmetic(
            base_variance,
            mean_square,
            StandardErrorBinaryStep::PhaseVariance,
        )?;
        let phase_error =
            standard_error_square_root(phase_variance, FixtureStandardErrorField::Phase)?;
        let rate = StandardErrorScalar::new(rate_error, FixtureStandardErrorField::Rate)?;
        let phase = StandardErrorScalar::new(phase_error, FixtureStandardErrorField::Phase)?;
        Ok(FixtureStandardErrors {
            frame: self.frame(),
            sample_count: self.sample_count,
            degrees_of_freedom: self.degrees_of_freedom,
            phase,
            rate,
        })
    }
}

/// Produce fresh-line residual diagnostics for the same immutable fixture input.
///
/// The sequence is exactly `y-A`, `B*x` using the returned physical rate,
/// difference, square, `square/d` and chronological SSE addition, then `E/(n-2)`,
/// `scale*n` and `product/W`. Each primitive is separately rounded and guarded; no
/// FMA, reassociation, clamp or positive floor is permitted. Reports are algebraic
/// diagnostics, not physical uncertainty, eligibility or correction authority.
/// The immutable input owns count 3..64, ordered checked points and prepared `W`.
/// Checked integer conversion makes count and degrees of freedom exact in `f64`;
/// all three outputs pass the sole mandatory garde/private scalar constructor.
/// Work is bounded by one fit plus O(n), n<=64, with fixed scalar scratch and no
/// external effects. Scalar validation owns only a one-field temporary report;
/// no allocation measurement is inferred. The result retains no input borrow.
/// # Errors
/// Calls `fit` exactly once and returns `Fit` immediately on failure. Otherwise
/// returns the first unsupported residual primitive, a checked count disagreement
/// or scalar-admission error. Failure preserves the input and publishes no report.
pub fn residual_statistics(input: &FitInput) -> Result<FixtureResidualStatistics, ResidualError> {
    let line = fit(input).map_err(ResidualError::Fit)?;
    let weighted_sse = weighted_sse(&input.batch(), &line)?;
    finish_statistics(input, line, weighted_sse)
}

/// Finish supplied same-owner numerical values without asserting their association.
///
/// Callers own association of input, line and SSE. The checked input owns count
/// and geometry. Arithmetic, count and private scalar admission preserve the
/// original producer's order. No partial result or effect is published.
/// # Errors
/// Returns the first count, arithmetic or private statistic-admission failure.
fn finish_statistics(
    input: &FitInput,
    line: FixtureLine,
    weighted_sse: f64,
) -> Result<FixtureResidualStatistics, ResidualError> {
    let sample_count = input.sample_count();
    let degrees_of_freedom = sample_count
        .checked_sub(2)
        .ok_or(ResidualError::CountInvariant)?;
    let count = f64::from(u8::try_from(sample_count).map_err(|_| ResidualError::CountInvariant)?);
    let degrees =
        f64::from(u8::try_from(degrees_of_freedom).map_err(|_| ResidualError::CountInvariant)?);
    let residual_scale = residual_arithmetic(weighted_sse, degrees, ResidualStep::Scale)?;
    let variance_product =
        residual_arithmetic(residual_scale, count, ResidualStep::NormalizeMultiply)?;
    let normalized_data_variance = residual_arithmetic(
        variance_product,
        input.geometry.weight,
        ResidualStep::NormalizeDivide,
    )?;
    Ok(FixtureResidualStatistics {
        line,
        sample_count,
        degrees_of_freedom,
        weighted_sse: ResidualScalar::new(weighted_sse, ResidualField::WeightedSse)?,
        residual_scale: ResidualScalar::new(residual_scale, ResidualField::ResidualScale)?,
        normalized_data_variance: ResidualScalar::new(
            normalized_data_variance,
            ResidualField::NormalizedDataVariance,
        )?,
        geometry: input.geometry,
    })
}

/// Internal terminal error retaining row attribution when one exists.
pub(super) enum TerminalStatisticsError {
    /// A checked row primitive failed at its original-input index.
    Row {
        /// Original-input point index established by the candidate owner.
        index: usize,
        /// Actual residual arithmetic rejection.
        source: ResidualError,
    },
    /// A count, aggregate arithmetic or output-admission failure.
    Summary(ResidualError),
    /// The candidate's checked row association could not be projected.
    Invariant,
}

/// Produce statistics using only the terminal owner's same-fit checked cache.
///
/// Rows and their divisors are traversed once in suffix order; each row keeps
/// its original-input index through all checked square/divide/add operations.
/// No fit, external effect, partial report or independently supplied line occurs.
/// # Errors
/// Returns the first checked row, summary or candidate-association failure.
pub(super) fn statistics_from_terminal(
    terminal: &TerminalCandidate,
) -> Result<FixtureResidualStatistics, TerminalStatisticsError> {
    let mut sum = 0.0;
    for (offset, point) in terminal.suffix().batch().values.iter().enumerate() {
        let (index, residual) = terminal
            .suffix_residual(offset)
            .ok_or(TerminalStatisticsError::Invariant)?;
        let square = residual_arithmetic(residual, residual, ResidualStep::Square)
            .map_err(|source| TerminalStatisticsError::Row { index, source })?;
        let weighted = residual_arithmetic(square, point.variance_divisor, ResidualStep::Weight)
            .map_err(|source| TerminalStatisticsError::Row { index, source })?;
        sum = residual_arithmetic(sum, weighted, ResidualStep::Accumulate)
            .map_err(|source| TerminalStatisticsError::Row { index, source })?;
    }
    finish_statistics(terminal.suffix(), terminal.line(), sum)
        .map_err(TerminalStatisticsError::Summary)
}

/// Source-derived raw helper controls; bodies remain outside the critical module.
#[cfg(test)]
#[path = "../../tests/private/residual_guards.rs"]
mod residual_guards;

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
//! Checked synthetic points, response-independent geometry and bounded affine fit.
//!
//! Modified by David Osipov with AI assistance, 30 October 2026. Geometry adapts
//! only inverse-divisor weighting and centered-time equations from Chrony 4.9
//! regress.c, pinned revision 8df4f1263e206585e6c4e61352d042c2dd12916a,
//! selected bodies 69-87 and 252-269. See ../PROVENANCE.md and the private
//! candidate's additive Chrony ledger. This is source-derived, not clean-room.
//!
//! Sole admission: three bounded garde rules inside `FitPoint::new`, then raw
//! batch count, strict chronological order, final zero, span and geometry inside
//! `FitInput::new`. Immutable checked points are reused without reparsing. Every
//! evaluated binary64 primitive is separately assigned and checked; geometry
//! reads x/divisor only. Fit tests exact constant phases before any response
//! arithmetic; only nonconstant data evaluates response mean/covariance. Arrays and
//! work are fixed at 64 points, without heap-backed owned input storage or I/O.
//! Garde's temporary rejected-point report is bounded to three fixed fields and
//! discarded; only typed codes leave admission. No inputs or report are logged.

use std::{fmt, num::NonZeroU64};

use garde::Validate;

/// Bounded fixture history owns exclusive staging and complete state publication.
mod history;

/// Exact fixture observation admission and separately gated newest-reference preparation.
mod normalization;

/// Pure checked projection from an explicitly identified same-history fixture anchor.
mod projection;

/// Residual diagnostics reuse this owner's immutable input and checked primitives.
mod residuals;

/// Checked bounded fixture residual-runs and suffix selection.
mod runs;

pub use history::{FixtureHistory, FixtureHistoryReport, HistoryError};

pub use normalization::{
    FixtureContext, FixtureNormalization, FixtureObservation, InsufficientFixture,
    NormalizationComponent, NormalizationError, NormalizationStep, ObservationError,
    PreparedFixtureInput, prepare_fixture_input,
};

pub use projection::{
    FixtureAnchorProjection, FixtureProjectionError, FixtureProjectionField,
    FixtureProjectionIssue, FixtureProjectionStep, FixtureProjectionTerms, project_fixture_anchor,
};

pub use residuals::{
    FixtureResidualStatistics, ResidualError, ResidualField, ResidualIssue, ResidualStep,
    residual_statistics,
};

pub use runs::{
    FixtureRunsError, FixtureRunsSelection, FixtureRunsStage, FixtureRunsStop, select_fixture_runs,
};

/// Maximum admitted points and sole fixed point-array capacity.
const MAX_POINTS: usize = 64;
/// Minimum regression population; checked before reading or copying points.
const MIN_POINTS: usize = 3;
/// Inclusive minimum span and divisor in this synthetic fixture contract.
const ONE_SIXTEENTH: f64 = 1.0 / 16.0;
/// Inclusive minimum calculated binary64 weighted normalized-time spread.
const MIN_SPREAD: f64 = 1.0 / 256.0;

/// Constructor argument role, in the declared first-error precedence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PointField {
    /// Relative seconds from the asserted newest reference, in [-4096,0].
    RelativeSeconds,
    /// Synthetic phase seconds, positive when local is behind, in [-1,1].
    PhaseSeconds,
    /// Positive dimensionless variance divisor, in [1/16,16].
    VarianceDivisor,
}

/// First violated scalar invariant, checked in this enum's documented order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PointIssue {
    /// Raw value is NaN or either infinity.
    NonFinite,
    /// Raw nonzero value is subnormal, before any range decision.
    Subnormal,
    /// Finite normal-or-zero raw value is outside the field's inclusive range.
    OutOfRange,
}

/// Numerical ownership stage, without raw floating operands or context strings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArithmeticStage {
    /// Response-independent normalized-time geometry and explicit time-component conversions.
    Geometry,
    /// Response mean/covariance, residual diagnostics and explicit phase-component conversions.
    Response,
    /// Affine phase/rate coefficient calculations.
    Coefficients,
    /// Private result admission preserving the originating operation code.
    Result,
    /// Fixed-roster principal distance, score and hysteresis calculations.
    Principal,
}

/// One separately evaluated binary64 operation; fused operations are excluded.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArithmeticOperation {
    /// Left-to-right sum or explicitly named addition.
    Add,
    /// Explicitly named subtraction; ordinary cancellation to zero is legal.
    Subtract,
    /// Explicitly named multiplication with nonzero-to-zero underflow rejection.
    Multiply,
    /// Division with a positive denominator and explicit underflow rejection.
    Divide,
}

/// Unsupported arithmetic has a typed cause and no retained raw operand/result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArithmeticIssue {
    /// An operand or computed result is NaN or infinite.
    NonFinite,
    /// An operand or computed nonzero result is subnormal.
    Subnormal,
    /// Multiplication/division lost a nonzero value to either signed zero.
    ZeroUnderflow,
    /// A division denominator was zero or negative.
    NonPositiveDenominator,
}

/// Bounded domain failures; returned indices are proven below 64.
///
/// No raw float, rejected report, source endpoint or allocation is retained.
/// Known point errors preserve their shared typed rule's field/issue. The
/// invariant variant is reserved for a validator/domain disagreement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FitError {
    /// The first invalid field in constructor argument order.
    Point {
        /// Fixed constructor argument role.
        field: PointField,
        /// First failed rule for that field.
        issue: PointIssue,
    },
    /// Raw batch count is outside the inclusive 3..64 envelope.
    CountOutOfRange,
    /// The first non-increasing pair, with returned second-point index <64.
    NonIncreasingTime {
        /// Module-issued index of the later point in the first invalid pair.
        index: usize,
    },
    /// The last admitted time is not numerically zero.
    ReferenceNotZero,
    /// Span from oldest point to newest zero is outside [1/16,4096].
    SpanOutOfRange,
    /// Calculated binary64 normalized-time spread is below 1/256.
    InsufficientSpread,
    /// Unsupported primitive arithmetic, without exposing its raw values.
    Arithmetic {
        /// Numerical owner stage.
        stage: ArithmeticStage,
        /// Separately evaluated primitive.
        operation: ArithmeticOperation,
        /// Bounded cause code.
        issue: ArithmeticIssue,
    },
    /// Mandatory garde validation disagreed with all shared scalar rules.
    ValidationInvariant,
}

impl fmt::Display for FitError {
    /// Format only fixed type codes and the permitted bounded index.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Point { field, issue } => write!(formatter, "point {field:?}: {issue:?}"),
            Self::CountOutOfRange => formatter.write_str("sample count outside 3..64"),
            Self::NonIncreasingTime { index } => {
                write!(formatter, "non-increasing time at index {index}")
            }
            Self::ReferenceNotZero => formatter.write_str("newest reference is not zero"),
            Self::SpanOutOfRange => formatter.write_str("relative-time span outside fixture range"),
            Self::InsufficientSpread => formatter.write_str("insufficient calculated time spread"),
            Self::Arithmetic {
                stage,
                operation,
                issue,
            } => {
                write!(formatter, "arithmetic {stage:?}/{operation:?}: {issue:?}")
            }
            Self::ValidationInvariant => formatter.write_str("validation invariant disagreement"),
        }
    }
}

impl std::error::Error for FitError {}

/// Sole scalar predicate owner for garde rules and typed error projection.
///
/// All raw f64 values are admitted to this constant-work, effect-free predicate.
/// It preserves `NonFinite` -> `Subnormal` -> `OutOfRange` precedence; zero is never
/// subnormal. It returns no formatted input or library error object.
fn point_issue(value: f64, field: PointField) -> Option<PointIssue> {
    if !value.is_finite() {
        return Some(PointIssue::NonFinite);
    }
    if value.is_subnormal() {
        return Some(PointIssue::Subnormal);
    }
    let in_range = match field {
        PointField::RelativeSeconds => (-4096.0..=0.0).contains(&value),
        PointField::PhaseSeconds => (-1.0..=1.0).contains(&value),
        PointField::VarianceDivisor => (ONE_SIXTEENTH..=16.0).contains(&value),
    };
    if in_range {
        None
    } else {
        Some(PointIssue::OutOfRange)
    }
}

/// Reuse the scalar owner from garde; rejection text is fixed and payload-free.
/// # Errors
/// Rejects the shared field predicate without formatting the raw f64 value.
fn field_rule(value: f64, field: PointField) -> garde::Result {
    if point_issue(value, field).is_none() {
        Ok(())
    } else {
        Err(garde::Error::new("invalid synthetic fit field"))
    }
}

/// Adapt the relative-time predicate to garde's required borrowed signature.
/// # Errors
/// Returns only the fixed redacted rule failure for an invalid relative time.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rule signature borrows its raw field and context"
)]
fn relative_rule(value: &f64, (): &()) -> garde::Result {
    field_rule(*value, PointField::RelativeSeconds)
}

/// Adapt the phase predicate to garde's required borrowed signature.
/// # Errors
/// Returns only the fixed redacted rule failure for an invalid phase.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rule signature borrows its raw field and context"
)]
fn phase_rule(value: &f64, (): &()) -> garde::Result {
    field_rule(*value, PointField::PhaseSeconds)
}

/// Adapt the divisor predicate to garde's required borrowed signature.
/// # Errors
/// Returns only the fixed redacted rule failure for an invalid divisor.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rule signature borrows its raw field and context"
)]
fn divisor_rule(value: &f64, (): &()) -> garde::Result {
    field_rule(*value, PointField::VarianceDivisor)
}

/// Three raw fields, each covered by its shared garde rule; no skipped field.
#[derive(Validate)]
struct PointCandidate {
    /// Raw relative seconds, validated before publication.
    #[garde(custom(relative_rule))]
    relative_seconds: f64,
    /// Raw positive-behind phase seconds, validated before publication.
    #[garde(custom(phase_rule))]
    phase_seconds: f64,
    /// Raw dimensionless variance divisor, validated before publication.
    #[garde(custom(divisor_rule))]
    variance_divisor: f64,
}

impl PointCandidate {
    /// Project the first shared scalar error in argument order without effects.
    fn first_error(&self) -> Option<FitError> {
        for (value, field) in [
            (self.relative_seconds, PointField::RelativeSeconds),
            (self.phase_seconds, PointField::PhaseSeconds),
            (self.variance_divisor, PointField::VarianceDivisor),
        ] {
            if let Some(issue) = point_issue(value, field) {
                return Some(FitError::Point { field, issue });
            }
        }
        None
    }
}

/// Immutable admitted synthetic point; clone/copy preserves its scalar invariants.
///
/// ```compile_fail,E0451
/// use time_core::FitPoint;
/// let _ = FitPoint { relative_seconds: -1.0, phase_seconds: 0.0, variance_divisor: 1.0 };
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct FitPoint {
    /// Checked finite normal-or-zero relative seconds in [-4096,0].
    relative_seconds: f64,
    /// Checked finite normal-or-zero positive-behind seconds in [-1,1].
    phase_seconds: f64,
    /// Checked finite positive dimensionless divisor in [1/16,16].
    variance_divisor: f64,
}

impl fmt::Debug for FitPoint {
    /// Reveal the type without automatically formatting raw numeric inputs.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FitPoint { .. }")
    }
}

impl FitPoint {
    /// Run all three mandatory garde rules before publishing an immutable point.
    ///
    /// The temporary report covers three fixed fields and is discarded. Typed
    /// errors reuse the exact same scalar predicates, never library error text.
    /// There are no external effects or caller-optional validation routes.
    /// # Errors
    /// Returns the first field's NonFinite/Subnormal/OutOfRange issue. Only an
    /// otherwise unexplained validator disagreement becomes `ValidationInvariant`.
    pub fn new(
        relative_seconds: f64,
        phase_seconds: f64,
        variance_divisor: f64,
    ) -> Result<Self, FitError> {
        let candidate = PointCandidate {
            relative_seconds,
            phase_seconds,
            variance_divisor,
        };
        let validation = candidate.validate();
        if let Some(error) = candidate.first_error() {
            return Err(error);
        }
        validation.map_err(|_| FitError::ValidationInvariant)?;
        Ok(Self {
            relative_seconds,
            phase_seconds,
            variance_divisor,
        })
    }

    /// Return the unchanged checked relative seconds, without mutation/effects.
    #[must_use]
    pub const fn relative_seconds(&self) -> f64 {
        self.relative_seconds
    }

    /// Return unchanged checked positive-behind seconds, without effects.
    #[must_use]
    pub const fn phase_seconds(&self) -> f64 {
        self.phase_seconds
    }

    /// Return the unchanged dimensionless divisor, without policy interpretation.
    #[must_use]
    pub const fn variance_divisor(&self) -> f64 {
        self.variance_divisor
    }
}

/// Return one bounded arithmetic cause with no raw operands or side effects.
fn arithmetic_error(
    stage: ArithmeticStage,
    operation: ArithmeticOperation,
    issue: ArithmeticIssue,
) -> FitError {
    FitError::Arithmetic {
        stage,
        operation,
        issue,
    }
}

/// Admit one supported normal-or-zero scalar without changing its bits.
///
/// The stage and operation identify the actual caller/producing primitive, never
/// a parsed error message. No arithmetic, normalization or range clamp occurs.
/// # Errors
/// Rejects nonfinite then nonzero subnormal values with the supplied typed cause.
fn supported_result(
    value: f64,
    operation: ArithmeticOperation,
    stage: ArithmeticStage,
) -> Result<f64, FitError> {
    if !value.is_finite() {
        return Err(arithmetic_error(
            stage,
            operation,
            ArithmeticIssue::NonFinite,
        ));
    }
    if value.is_subnormal() {
        return Err(arithmetic_error(
            stage,
            operation,
            ArithmeticIssue::Subnormal,
        ));
    }
    Ok(value)
}

/// Evaluate exactly one separately rounded primitive and check its support domain.
///
/// Raw scalar helper inputs are independently checked; no unchecked domain value
/// is required for hostile operand controls. Addition/subtraction cancellation
/// and zero operands are legal. Work is constant; errors publish no partial value.
/// # Errors
/// Rejects nonfinite then subnormal operands, a nonpositive divide denominator,
/// nonfinite/subnormal results, or nonzero multiplication/division lost to zero.
pub(super) fn arithmetic(
    left: f64,
    right: f64,
    operation: ArithmeticOperation,
    stage: ArithmeticStage,
) -> Result<f64, FitError> {
    if !left.is_finite() || !right.is_finite() {
        return Err(arithmetic_error(
            stage,
            operation,
            ArithmeticIssue::NonFinite,
        ));
    }
    if left.is_subnormal() || right.is_subnormal() {
        return Err(arithmetic_error(
            stage,
            operation,
            ArithmeticIssue::Subnormal,
        ));
    }
    if operation == ArithmeticOperation::Divide && right <= 0.0 {
        return Err(arithmetic_error(
            stage,
            operation,
            ArithmeticIssue::NonPositiveDenominator,
        ));
    }
    let result = match operation {
        ArithmeticOperation::Add => left + right,
        ArithmeticOperation::Subtract => left - right,
        ArithmeticOperation::Multiply => left * right,
        ArithmeticOperation::Divide => left / right,
    };
    let result = supported_result(result, operation, stage)?;
    let lost_nonzero = match operation {
        ArithmeticOperation::Multiply => left != 0.0 && right != 0.0 && result == 0.0,
        ArithmeticOperation::Divide => left != 0.0 && result == 0.0,
        ArithmeticOperation::Add | ArithmeticOperation::Subtract => false,
    };
    if lost_nonzero {
        return Err(arithmetic_error(
            stage,
            operation,
            ArithmeticIssue::ZeroUnderflow,
        ));
    }
    Ok(result)
}

/// Prepared time/divisor-only geometry, checked before batch publication.
#[derive(Clone, Copy)]
struct Geometry {
    /// Checked oldest-to-newest span H in [1/16,4096] seconds.
    span: f64,
    /// Checked positive finite inverse-divisor weight sum W.
    weight: f64,
    /// Checked normalized-time weighted mean `mu_z`.
    mean_z: f64,
    /// Checked positive centered normalized-time weighted square sum Szz.
    centered_z_square: f64,
}

/// Borrowed already checked points with count admitted before any element access.
///
/// Private construction freezes the immutable slice's 3..64 count invariant
/// across order/geometry helpers; no helper relies on optional raw-slice checks.
struct BatchPoints<'a> {
    /// Immutable admitted point prefix with inclusive count 3..64.
    values: &'a [FitPoint],
}

impl<'a> BatchPoints<'a> {
    /// Check count before reading or copying a point; retain only an immutable view.
    /// # Errors
    /// Rejects raw counts outside 3..64 without reading an element or effects.
    fn new(values: &'a [FitPoint]) -> Result<Self, FitError> {
        if !(MIN_POINTS..=MAX_POINTS).contains(&values.len()) {
            return Err(FitError::CountOutOfRange);
        }
        Ok(Self { values })
    }
}

/// Check batch order, newest reference and span, without reading phase values.
///
/// Caller has admitted 3..64 immutable points. No copying or mutation occurs;
/// returned error indices follow count admission and are strictly below 64.
/// # Errors
/// Rejects first non-increasing pair, then nonzero final reference, then bad span.
fn batch_span(batch: &BatchPoints<'_>) -> Result<f64, FitError> {
    let points = batch.values;
    for (index, pair) in points.windows(2).enumerate() {
        if pair[0].relative_seconds >= pair[1].relative_seconds {
            return Err(FitError::NonIncreasingTime { index: index + 1 });
        }
    }
    if points[points.len() - 1].relative_seconds != 0.0 {
        return Err(FitError::ReferenceNotZero);
    }
    let span = -points[0].relative_seconds;
    if !(ONE_SIXTEENTH..=4096.0).contains(&span) {
        return Err(FitError::SpanOutOfRange);
    }
    Ok(span)
}

/// First chronological geometry pass; evaluate q,z,W and sum(q*z) in order.
///
/// Inputs are admitted points and positive checked span. No response field is
/// read. All sums are left-to-right; no fused multiply-add or reduction exists.
/// # Errors
/// Propagates the exact unsupported primitive's typed geometry error.
fn geometry_mean(points: &BatchPoints<'_>, span: f64) -> Result<(f64, f64), FitError> {
    let mut weight = 0.0;
    let mut weighted_z = 0.0;
    for point in points.values {
        let q = arithmetic(
            1.0,
            point.variance_divisor,
            ArithmeticOperation::Divide,
            ArithmeticStage::Geometry,
        )?;
        let z = arithmetic(
            point.relative_seconds,
            span,
            ArithmeticOperation::Divide,
            ArithmeticStage::Geometry,
        )?;
        weight = arithmetic(
            weight,
            q,
            ArithmeticOperation::Add,
            ArithmeticStage::Geometry,
        )?;
        let term = arithmetic(
            q,
            z,
            ArithmeticOperation::Multiply,
            ArithmeticStage::Geometry,
        )?;
        weighted_z = arithmetic(
            weighted_z,
            term,
            ArithmeticOperation::Add,
            ArithmeticStage::Geometry,
        )?;
    }
    let mean_z = arithmetic(
        weighted_z,
        weight,
        ArithmeticOperation::Divide,
        ArithmeticStage::Geometry,
    )?;
    Ok((weight, mean_z))
}

/// Second chronological geometry pass; written left association is q*delta*delta.
///
/// Inputs are admitted immutable points and checked first-pass values. Only x/d
/// are read; no mean/weighted product/covariance of phase is evaluated.
/// # Errors
/// Propagates the exact unsupported separately rounded primitive operation.
fn geometry_square(points: &BatchPoints<'_>, span: f64, mean_z: f64) -> Result<f64, FitError> {
    let mut square = 0.0;
    for point in points.values {
        let q = arithmetic(
            1.0,
            point.variance_divisor,
            ArithmeticOperation::Divide,
            ArithmeticStage::Geometry,
        )?;
        let z = arithmetic(
            point.relative_seconds,
            span,
            ArithmeticOperation::Divide,
            ArithmeticStage::Geometry,
        )?;
        let delta = arithmetic(
            z,
            mean_z,
            ArithmeticOperation::Subtract,
            ArithmeticStage::Geometry,
        )?;
        let weighted_delta = arithmetic(
            q,
            delta,
            ArithmeticOperation::Multiply,
            ArithmeticStage::Geometry,
        )?;
        let term = arithmetic(
            weighted_delta,
            delta,
            ArithmeticOperation::Multiply,
            ArithmeticStage::Geometry,
        )?;
        square = arithmetic(
            square,
            term,
            ArithmeticOperation::Add,
            ArithmeticStage::Geometry,
        )?;
    }
    Ok(square)
}

/// Prepare checked geometry with exactly two bounded passes over admitted points.
///
/// Count/order/reference/span admission precedes this helper. Computed binary64
/// spread, rather than an exact-real comparison, is the adopted threshold rule.
/// A zero Szz yields insufficient spread; it never becomes a usable denominator.
/// # Errors
/// Rejects unsupported arithmetic or calculated spread below 1/256.
fn geometry(points: &BatchPoints<'_>, span: f64) -> Result<Geometry, FitError> {
    let (weight, mean_z) = geometry_mean(points, span)?;
    let centered_z_square = geometry_square(points, span, mean_z)?;
    let spread = arithmetic(
        centered_z_square,
        weight,
        ArithmeticOperation::Divide,
        ArithmeticStage::Geometry,
    )?;
    if spread < MIN_SPREAD {
        return Err(FitError::InsufficientSpread);
    }
    Ok(Geometry {
        span,
        weight,
        mean_z,
        centered_z_square,
    })
}

/// Immutable batch with at most 64 copied points and response-independent geometry.
///
/// The sole point array is 64 * 24 = 1536 logical bytes on the admitted f64
/// representation. Four geometry scalars, count and frame keep the owned value
/// below the reviewed 4 KiB logical bound. This is no heap/RSS/stack measurement.
/// Unused array slots reuse an admitted point, never an unchecked/default value.
#[derive(Clone)]
pub struct FitInput {
    /// Caller-asserted nonzero synthetic mapping/reference identity.
    frame: NonZeroU64,
    /// Fixed admitted-point storage; only the len prefix is used.
    points: [FitPoint; MAX_POINTS],
    /// Private count in the inclusive 3..64 range.
    len: usize,
    /// Checked x/divisor-only geometry on the immutable point prefix.
    geometry: Geometry,
}

impl fmt::Debug for FitInput {
    /// Format permitted identity/count only, without points or numeric geometry.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FitInput")
            .field("frame", &self.frame)
            .field("sample_count", &self.len)
            .finish_non_exhaustive()
    }
}

impl FitInput {
    /// Admit count before reading/copying points; prepare geometry before publish.
    ///
    /// Checked points are reused without a second raw admission. Failure leaves
    /// the caller's input unchanged and publishes no partially admitted batch.
    /// Work is O(n), n<=64; owned storage is one fixed array and four scalars.
    /// # Errors
    /// Count precedes increasing-time, final-zero, span and geometry/spread errors.
    pub fn new(frame: NonZeroU64, points: &[FitPoint]) -> Result<Self, FitError> {
        let batch = BatchPoints::new(points)?;
        let span = batch_span(&batch)?;
        let prepared = geometry(&batch, span)?;
        let mut stored = [batch.values[0]; MAX_POINTS];
        stored[..batch.values.len()].copy_from_slice(batch.values);
        Ok(Self {
            frame,
            points: stored,
            len: batch.values.len(),
            geometry: prepared,
        })
    }

    /// Borrow the immutable 3..64-point prefix whose constructor/Clone retain count.
    ///
    /// This is an invariant-preserving view, without reparsing points, arithmetic,
    /// allocation or a raw-slice caller admission route.
    fn batch(&self) -> BatchPoints<'_> {
        BatchPoints {
            values: &self.points[..self.len],
        }
    }

    /// Return the caller-asserted frame; this grants no native mapping trust.
    #[must_use]
    pub const fn frame(&self) -> NonZeroU64 {
        self.frame
    }

    /// Return the already admitted immutable count without scanning or effects.
    #[must_use]
    pub const fn sample_count(&self) -> usize {
        self.len
    }
}

/// Immutable synthetic affine coefficients, constructible only from checked values.
///
/// Private checked coefficients admit finite normal-or-zero outputs without
/// reusing point input range bounds. The frame remains caller-asserted.
/// No public constructor, mutable getter, default or deserialization route exists.
#[derive(Clone, Copy)]
pub struct FixtureLine {
    /// Same asserted fixture frame as the completed batch.
    frame: NonZeroU64,
    /// Checked positive-behind phase seconds at the newest reference.
    phase_seconds: f64,
    /// Checked phase seconds per relative-time second, with its own output bound.
    phase_rate_seconds_per_second: f64,
}

impl fmt::Debug for FixtureLine {
    /// Format only the asserted frame, without numeric output or input payloads.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FixtureLine")
            .field("frame", &self.frame)
            .finish_non_exhaustive()
    }
}

impl FixtureLine {
    /// Publish only private invariant-bearing coefficients, preserving exact bits.
    ///
    /// Each input is already admitted by transformed-output checks or immutable
    /// point/zero construction. No public raw constructor, arithmetic, further
    /// range restriction or side effect is introduced here.
    fn new(frame: NonZeroU64, phase: Coefficient, rate: Coefficient) -> Self {
        Self {
            frame,
            phase_seconds: phase.value,
            phase_rate_seconds_per_second: rate.value,
        }
    }

    /// Return the unchanged asserted frame without effects or native authority.
    #[must_use]
    pub const fn frame(&self) -> NonZeroU64 {
        self.frame
    }

    /// Return checked output positive-behind phase seconds without mutation.
    #[must_use]
    pub const fn phase_seconds(&self) -> f64 {
        self.phase_seconds
    }

    /// Return checked output phase seconds per relative-time second.
    #[must_use]
    pub const fn phase_rate_seconds_per_second(&self) -> f64 {
        self.phase_rate_seconds_per_second
    }
}

/// Private finite normal-or-zero coefficient, separate from the point envelope.
#[derive(Clone, Copy)]
struct Coefficient {
    /// Immutable checked output scalar; no public raw access or mutation route.
    value: f64,
}

impl Coefficient {
    /// Recheck a transformed output with its actual originating primitive code.
    ///
    /// This admits output invariants only; no point range or physical bound is
    /// imposed. Return the original bits, including any valid signed zero.
    /// # Errors
    /// Rejects nonfinite/subnormal results with Result stage and actual operation.
    fn new(value: f64, operation: ArithmeticOperation) -> Result<Self, FitError> {
        Ok(Self {
            value: supported_result(value, operation, ArithmeticStage::Result)?,
        })
    }

    /// Preserve an immutable admitted point's finite normal-or-zero phase bits.
    ///
    /// `FitPoint`'s mandatory constructor owns the reusable invariant; there is no
    /// raw floating factory here and no fabricated primitive operation to report.
    fn from_point(point: &FitPoint) -> Self {
        Self {
            value: point.phase_seconds,
        }
    }

    /// Construct the exact supported zero-rate constant without arithmetic/effects.
    fn zero() -> Self {
        Self { value: 0.0 }
    }
}

/// Return the original first point iff all admitted phases are numerically equal.
///
/// The counted immutable batch supplies at least three checked points. Numerical
/// equality deliberately treats mixed signed zeros alike; returned phase bits
/// still come from the original first point. Work is bounded at 64 comparisons.
#[expect(
    clippy::float_cmp,
    reason = "Reviewed exact constant-series semantics require numerical equality without epsilon, including mixed signed zeros"
)]
fn constant_point<'a>(points: &BatchPoints<'a>) -> Option<&'a FitPoint> {
    let first = &points.values[0];
    if points
        .values
        .iter()
        .all(|point| point.phase_seconds == first.phase_seconds)
    {
        Some(first)
    } else {
        None
    }
}

/// Compute the nonconstant response mean in the prescribed chronological order.
///
/// The immutable batch and geometry are already checked. Each q*y term and sum
/// is a separately rounded primitive; no response operation runs for constants.
/// # Errors
/// Propagates the actual unsupported Response-stage primitive without a fallback.
fn response_mean(points: &BatchPoints<'_>, geometry: &Geometry) -> Result<f64, FitError> {
    let mut weighted_y = 0.0;
    for point in points.values {
        let q = arithmetic(
            1.0,
            point.variance_divisor,
            ArithmeticOperation::Divide,
            ArithmeticStage::Response,
        )?;
        let term = arithmetic(
            q,
            point.phase_seconds,
            ArithmeticOperation::Multiply,
            ArithmeticStage::Response,
        )?;
        weighted_y = arithmetic(
            weighted_y,
            term,
            ArithmeticOperation::Add,
            ArithmeticStage::Response,
        )?;
    }
    arithmetic(
        weighted_y,
        geometry.weight,
        ArithmeticOperation::Divide,
        ArithmeticStage::Response,
    )
}

/// Compute nonconstant response covariance with written left association q*dz*dy.
///
/// Inputs are immutable checked points/geometry and a supported response mean.
/// No reordered reduction, FMA, response storage or external effect exists.
/// # Errors
/// Propagates the actual unsupported Response-stage separately rounded primitive.
fn response_covariance(
    points: &BatchPoints<'_>,
    geometry: &Geometry,
    mean_y: f64,
) -> Result<f64, FitError> {
    let mut covariance = 0.0;
    for point in points.values {
        let q = arithmetic(
            1.0,
            point.variance_divisor,
            ArithmeticOperation::Divide,
            ArithmeticStage::Response,
        )?;
        let z = arithmetic(
            point.relative_seconds,
            geometry.span,
            ArithmeticOperation::Divide,
            ArithmeticStage::Response,
        )?;
        let delta_z = arithmetic(
            z,
            geometry.mean_z,
            ArithmeticOperation::Subtract,
            ArithmeticStage::Response,
        )?;
        let delta_y = arithmetic(
            point.phase_seconds,
            mean_y,
            ArithmeticOperation::Subtract,
            ArithmeticStage::Response,
        )?;
        let weighted_delta = arithmetic(
            q,
            delta_z,
            ArithmeticOperation::Multiply,
            ArithmeticStage::Response,
        )?;
        let term = arithmetic(
            weighted_delta,
            delta_y,
            ArithmeticOperation::Multiply,
            ArithmeticStage::Response,
        )?;
        covariance = arithmetic(
            covariance,
            term,
            ArithmeticOperation::Add,
            ArithmeticStage::Response,
        )?;
    }
    Ok(covariance)
}

/// Produce physical phase/rate coefficients from checked nonconstant statistics.
///
/// Geometry supplies positive Szz/H; response mean/covariance are supported
/// scalars. Output admission is distinct from input ranges and preserves frame.
/// # Errors
/// Propagates Coefficients-stage arithmetic or Result-stage output rejection.
fn line_from_response(
    frame: NonZeroU64,
    geometry: &Geometry,
    mean_y: f64,
    covariance: f64,
) -> Result<FixtureLine, FitError> {
    let scaled_rate = arithmetic(
        covariance,
        geometry.centered_z_square,
        ArithmeticOperation::Divide,
        ArithmeticStage::Coefficients,
    )?;
    let shift = arithmetic(
        scaled_rate,
        geometry.mean_z,
        ArithmeticOperation::Multiply,
        ArithmeticStage::Coefficients,
    )?;
    let phase = arithmetic(
        mean_y,
        shift,
        ArithmeticOperation::Subtract,
        ArithmeticStage::Coefficients,
    )?;
    let rate = arithmetic(
        scaled_rate,
        geometry.span,
        ArithmeticOperation::Divide,
        ArithmeticStage::Coefficients,
    )?;
    let phase = Coefficient::new(phase, ArithmeticOperation::Subtract)?;
    let rate = Coefficient::new(rate, ArithmeticOperation::Divide)?;
    Ok(FixtureLine::new(frame, phase, rate))
}

/// Fit a bounded synthetic line while preserving the immutable admitted input.
///
/// Geometry has already been checked before this call. Exact constant data
/// returns the first admitted phase bits and exact positive zero rate without
/// response arithmetic. Otherwise the reviewed ordered weighted mean/covariance
/// and coefficient operations run in O(n), n<=64, without allocation or effects.
/// Outputs are fixture coefficients, never eligible samples or uncertainty.
/// # Errors
/// Rejects the first unsupported response/coefficient/result primitive; no clamp,
/// constant fallback after failure, input mutation or last-result reuse occurs.
pub fn fit(input: &FitInput) -> Result<FixtureLine, FitError> {
    let points = input.batch();
    if let Some(point) = constant_point(&points) {
        return Ok(FixtureLine::new(
            input.frame,
            Coefficient::from_point(point),
            Coefficient::zero(),
        ));
    }
    let mean_y = response_mean(&points, &input.geometry)?;
    let covariance = response_covariance(&points, &input.geometry, mean_y)?;
    line_from_response(input.frame, &input.geometry, mean_y, covariance)
}

/// Source-derived private scalar guard controls; bodies stay outside critical file.
#[cfg(test)]
#[path = "../tests/private/unit_guards.rs"]
mod unit_guards;

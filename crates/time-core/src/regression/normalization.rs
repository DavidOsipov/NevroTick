// SPDX-License-Identifier: GPL-2.0-only
/*
 chronyd/chronyc - Programs for keeping computer clocks accurate.

**********************************************************************
* Copyright (C) Richard P. Curnow  1997-2003
* Copyright (C) Miroslav Lichvar  2011-2014, 2016-2018, 2021
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
//! Private owner for exact fixture observation admission and checked preparation.
//!
//! Modified by David Osipov with AI assistance, 30 October 2026. The bounded
//! newest-reference geometry adapts only `convert_to_intervals`, `sourcestats.c`
//! lines 353-368, Chrony 4.9 revision
//! `8df4f1263e206585e6c4e61352d042c2dd12916a`. The original notices above and
//! the additive source ledger precede this authoring. No ring, runs, history or
//! source-quality policy is copied. See `../../PROVENANCE.md` for the exact scope.
//!
//! `FixtureContext` owns three nonzero asserted tags. `FixtureObservation::new`
//! unavoidably validates its narrow integer phase and shared divisor rules with
//! garde; immutable time and broad phase values retain their `time_domain`
//! owners outside that two-field scalar candidate. Rejected reports are bounded
//! temporaries and never leave admission. The values grant no provenance trust,
//! native clock continuity, source eligibility or clock authority.
//!
//! Preparation bounds count before reading a row, checks all asserted contexts
//! before order, then checks exact domain durations and the maximum window before
//! any floating conversion. Insufficient counts publish only admitted metadata;
//! sufficient counts use checked relative/phase conversion, exactly one real point
//! construction per row and one same-input batch construction before publication.
//! Fixed scratch and immutable borrows establish no allocation/RSS or native
//! qualification. No fit, statistic, ambient clock, history or external effect is
//! invoked; errors preserve input bytes and publish no partial prepared payload.

use std::{fmt, num::NonZeroU64};

use garde::Validate;
use time_domain::{DomainError, MonotonicNs, PhaseOffsetQ33};

use super::{
    ArithmeticOperation, ArithmeticStage, FitError, FitInput, FitPoint, MAX_POINTS, MIN_POINTS,
    PointField, PointIssue, arithmetic, divisor_rule, point_issue,
};

/// Inclusive magnitude of the admitted positive-behind phase numerator over `2^33`.
const MAX_PHASE_NUMERATOR: i128 = 1_i128 << 33;
/// Inclusive raw integer envelope for exact binary64 reconstruction.
const MAX_EXACT_INTEGER: u64 = 1_u64 << 53;
/// Inclusive exact oldest-to-newest window before any floating conversion.
const MAX_ELAPSED_NANOSECONDS: u64 = 4_096_000_000_000;
/// Inclusive exact minimum span required only for a sufficient batch.
const MIN_SPAN_NANOSECONDS: u64 = 62_500_000;

/// Immutable caller-asserted fixture identity, not authenticated provenance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixtureContext {
    /// Nonzero caller-asserted source identity, with no independence claim.
    source_id: NonZeroU64,
    /// Nonzero assertion of one monotonic origin and phase-coordinate mapping.
    frame: NonZeroU64,
    /// Nonzero caller-asserted generation for the supplied fixture mapping.
    generation: NonZeroU64,
}

impl FixtureContext {
    /// Retain three already nonzero tags unchanged, without effects or validation work.
    ///
    /// This infallible constructor asserts fixture labels only; it performs no
    /// authentication, clock read, mapping check, allocation or state transition.
    #[must_use]
    pub const fn new(source_id: NonZeroU64, frame: NonZeroU64, generation: NonZeroU64) -> Self {
        Self {
            source_id,
            frame,
            generation,
        }
    }

    /// Return the unchanged nonzero source assertion without effects or authority.
    #[must_use]
    pub const fn source_id(&self) -> NonZeroU64 {
        self.source_id
    }

    /// Return the unchanged asserted origin/mapping tag without native qualification.
    #[must_use]
    pub const fn frame(&self) -> NonZeroU64 {
        self.frame
    }

    /// Return the unchanged asserted generation without a current-state check.
    #[must_use]
    pub const fn generation(&self) -> NonZeroU64 {
        self.generation
    }
}

/// Bounded observation rejection; no raw scalar or garde report is retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationError {
    /// The exact phase numerator is outside inclusive `[-2^33,2^33]`.
    PhaseOutOfRange,
    /// The shared variance-divisor predicate rejected its first violated rule.
    Divisor {
        /// Shared nonfinite/subnormal/range issue, in that precedence order.
        issue: PointIssue,
    },
    /// Mandatory garde disagreed with both maintained scalar predicates.
    ValidationInvariant,
}

impl fmt::Display for ObservationError {
    /// Format closed bounded context only, including every manually forged variant.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PhaseOutOfRange => {
                formatter.write_str("fixture observation phase outside Q33 range")
            }
            Self::Divisor { issue } => write!(formatter, "fixture observation divisor: {issue:?}"),
            Self::ValidationInvariant => {
                formatter.write_str("fixture observation validation invariant")
            }
        }
    }
}

impl std::error::Error for ObservationError {}

/// Test every raw integer against the one narrow phase envelope, without effects.
///
/// All `i128` values are accepted as arguments. The total fixed-work predicate
/// returns only a range decision; it changes no value or diagnostic payload.
fn phase_in_range(value: i128) -> bool {
    (-MAX_PHASE_NUMERATOR..=MAX_PHASE_NUMERATOR).contains(&value)
}

/// Adapt the same integer predicate to garde's borrowed rule signature.
///
/// The raw integer and unit context are inspected without publication or effects.
/// # Errors
/// Rejects an out-of-range numerator with a fixed payload-free library message.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rule signature borrows its raw field and context"
)]
fn phase_rule(value: &i128, (): &()) -> garde::Result {
    if phase_in_range(*value) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid fixture phase numerator"))
    }
}

/// Exactly two raw scalars, each covered by its maintained garde rule.
#[derive(Validate)]
struct ObservationCandidate {
    /// Exact raw numerator; the integer predicate owns its narrow `Q33` bound.
    #[garde(custom(phase_rule))]
    phase_numerator: i128,
    /// Raw dimensionless divisor; the parent rule owns its unchanged envelope.
    #[garde(custom(divisor_rule))]
    variance_divisor: f64,
}

/// Immutable exact fixture observation with mandatory private scalar admission.
///
/// The typed time and broad phase retain their existing immutable domain owners.
/// The sole constructor adds the narrow phase/divisor guarantees; no default,
/// raw factory, deserialization, mutable field or getter bypass is provided.
#[derive(Clone, Copy)]
pub struct FixtureObservation {
    /// Checked nonzero fixture labels, with no native provenance promise.
    context: FixtureContext,
    /// Unchanged supplied monotonic reading, never formatted by this owner.
    observed_at: MonotonicNs,
    /// Unchanged exact phase with privately admitted narrow numerator.
    phase: PhaseOffsetQ33,
    /// Unchanged finite positive divisor admitted by the shared point rule.
    variance_divisor: f64,
}

impl fmt::Debug for FixtureObservation {
    /// Format type and permitted context tags only; exclude all numeric payloads.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FixtureObservation")
            .field("context", &self.context)
            .finish_non_exhaustive()
    }
}

impl FixtureObservation {
    /// Unavoidably run both scalar garde rules, then retain exact immutable inputs.
    ///
    /// The supplied domain values remain unchanged; no time conversion, ordering,
    /// clock read, allocation measurement or preparation occurs. Garde owns only
    /// one bounded two-field temporary report, which is discarded on rejection.
    /// # Errors
    /// Phase range precedes the shared divisor issue; only a disagreement between
    /// successful predicates and mandatory garde becomes `ValidationInvariant`.
    pub fn new(
        context: FixtureContext,
        observed_at: MonotonicNs,
        phase: PhaseOffsetQ33,
        divisor: f64,
    ) -> Result<Self, ObservationError> {
        let candidate = ObservationCandidate {
            phase_numerator: phase.numerator(),
            variance_divisor: divisor,
        };
        let validation = candidate.validate();
        if !phase_in_range(candidate.phase_numerator) {
            return Err(ObservationError::PhaseOutOfRange);
        }
        if let Some(issue) = point_issue(divisor, PointField::VarianceDivisor) {
            return Err(ObservationError::Divisor { issue });
        }
        validation.map_err(|_| ObservationError::ValidationInvariant)?;
        Ok(Self {
            context,
            observed_at,
            phase,
            variance_divisor: divisor,
        })
    }

    /// Return the complete immutable context unchanged, without checking authority.
    #[must_use]
    pub const fn context(&self) -> FixtureContext {
        self.context
    }

    /// Return the exact supplied monotonic reading without conversion or effects.
    #[must_use]
    pub const fn observed_at(&self) -> MonotonicNs {
        self.observed_at
    }

    /// Return the exact narrow admitted phase without nanosecond quantization.
    #[must_use]
    pub const fn phase(&self) -> PhaseOffsetQ33 {
        self.phase
    }

    /// Return the unchanged checked dimensionless divisor without transformation.
    #[must_use]
    pub const fn variance_divisor(&self) -> f64 {
        self.variance_divisor
    }
}

/// Closed numerical conversion owner, without raw values or ambient context.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NormalizationComponent {
    /// Exact elapsed monotonic nanoseconds converted to relative seconds.
    Time,
    /// Narrow exact positive-behind `Q33` phase converted to seconds.
    Phase,
}

impl NormalizationComponent {
    /// Select the existing arithmetic stage for a closed component, without effects.
    const fn stage(self) -> ArithmeticStage {
        match self {
            Self::Time => ArithmeticStage::Geometry,
            Self::Phase => ArithmeticStage::Response,
        }
    }
}

/// Exact step for one separate guarded binary64 conversion primitive.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NormalizationStep {
    /// Multiply the exact high word by exactly `2^32`.
    IntegerHighProduct,
    /// Add the exact low word to the preceding high product.
    IntegerLowSum,
    /// Divide an exact integer by the component's exact seconds denominator.
    SecondsDivide,
}

impl NormalizationStep {
    /// Map every step to its sole primitive; callers cannot relabel an operation.
    const fn operation(self) -> ArithmeticOperation {
        match self {
            Self::IntegerHighProduct => ArithmeticOperation::Multiply,
            Self::IntegerLowSum => ArithmeticOperation::Add,
            Self::SecondsDivide => ArithmeticOperation::Divide,
        }
    }
}

/// Bounded preparation failure with typed causes and no retained input payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NormalizationError {
    /// The raw slice exceeds the maximum of `64` rows, before any element access.
    CountOutOfRange,
    /// A row's asserted context differs from the caller's expected context.
    ContextMismatch {
        /// Module-issued row index below `64`; forged public values remain safe.
        index: usize,
    },
    /// The first non-increasing pair of exact supplied monotonic readings.
    NonIncreasingTime {
        /// Module-issued index of the later point, below `64`.
        index: usize,
    },
    /// An exact elapsed value exceeds the inclusive `4096`-second window.
    ElapsedRange {
        /// Module-issued row index below `64`.
        index: usize,
    },
    /// A sufficient batch has exact span below `62_500_000` nanoseconds.
    SpanTooShort,
    /// A private raw integer argument exceeds its independent `2^53` envelope.
    IntegerRange {
        /// Closed conversion owner, never a caller-selected label.
        component: NormalizationComponent,
    },
    /// A checked conversion disagreed with the immutable input owner.
    ConversionInvariant {
        /// Closed conversion owner, without the raw integer or converted value.
        component: NormalizationComponent,
    },
    /// A real checked time-domain operation rejected the supplied relationship.
    Domain {
        /// Module-issued row index below `64`.
        index: usize,
        /// Actual bounded time-domain cause, never reformatted input data.
        source: DomainError,
    },
    /// Mandatory transformed-point construction rejected its actual output.
    Point {
        /// Module-issued row index below `64`.
        index: usize,
        /// Actual shared point-owner rejection, preserving its typed context.
        source: FitError,
    },
    /// The one same-input checked batch construction failed before publication.
    Input(FitError),
    /// One separate primitive failed, preserving its exact component and step.
    Arithmetic {
        /// Closed owner selecting the existing arithmetic stage.
        component: NormalizationComponent,
        /// Closed step selecting its sole primitive.
        step: NormalizationStep,
        /// Actual shared arithmetic cause; manual public construction is total.
        source: FitError,
    },
}

impl fmt::Display for NormalizationError {
    /// Format closed component/step/index/cause context without raw payloads.
    ///
    /// Every public combination, including `usize::MAX` and a non-arithmetic
    /// retained `FitError`, is handled without assumptions or parsing. Source
    /// formatting is bounded by its existing typed owner's redacted contract.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CountOutOfRange => formatter.write_str("normalization count above 64"),
            Self::ContextMismatch { index } => {
                write!(formatter, "normalization context mismatch at {index}")
            }
            Self::NonIncreasingTime { index } => {
                write!(formatter, "normalization non-increasing time at {index}")
            }
            Self::ElapsedRange { index } => {
                write!(formatter, "normalization elapsed range at {index}")
            }
            Self::SpanTooShort => formatter.write_str("normalization span below 1/16 second"),
            Self::IntegerRange { component } => {
                write!(formatter, "normalization integer range {component:?}")
            }
            Self::ConversionInvariant { component } => write!(
                formatter,
                "normalization conversion invariant {component:?}"
            ),
            Self::Domain { index, source } => {
                write!(formatter, "normalization domain at {index}: {source}")
            }
            Self::Point { index, source } => {
                write!(formatter, "normalization point at {index}: {source}")
            }
            Self::Input(source) => write!(formatter, "normalization input: {source}"),
            Self::Arithmetic {
                component,
                step,
                source,
            } => write!(formatter, "normalization {component:?}/{step:?}: {source}"),
        }
    }
}

impl std::error::Error for NormalizationError {
    /// Borrow each actual typed cause; the closed chain has no input or report.
    ///
    /// This projection is total for manually constructed variants and performs
    /// no arithmetic, parsing, allocation or effects.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Domain { source, .. } => Some(source),
            Self::Point { source, .. } | Self::Input(source) | Self::Arithmetic { source, .. } => {
                Some(source)
            }
            Self::CountOutOfRange
            | Self::ContextMismatch { .. }
            | Self::NonIncreasingTime { .. }
            | Self::ElapsedRange { .. }
            | Self::SpanTooShort
            | Self::IntegerRange { .. }
            | Self::ConversionInvariant { .. } => None,
        }
    }
}

/// Run the primitive chosen only by step and component, retaining its real cause.
///
/// All raw `f64` operand pairs are checked by the existing arithmetic owner;
/// component and step alone choose stage/operation. Unsupported operands/results,
/// nonpositive division and nonzero-to-zero underflow retain that owner's first
/// rejection. Legal cancellation and signed zeros are unchanged. One separate
/// ordinary primitive uses fixed scalar scratch, with no publication or effects.
/// # Errors
/// Wraps the actual shared `FitError` with its exact closed conversion context.
fn normalization_arithmetic(
    left: f64,
    right: f64,
    component: NormalizationComponent,
    step: NormalizationStep,
) -> Result<f64, NormalizationError> {
    arithmetic(left, right, step.operation(), component.stage()).map_err(|source| {
        NormalizationError::Arithmetic {
            component,
            step,
            source,
        }
    })
}

/// Guard and exactly reconstruct a raw unsigned integer through two primitives.
///
/// All `u64` values and closed components are accepted arguments. After the
/// inclusive `2^53` guard, checked high/low `u32` words convert exactly. A guarded
/// high-word multiplication by `2^32` precedes the separate guarded low-word sum;
/// both are exact within the admitted envelope. The result is finite normal or
/// positive zero, with no lost integer precision, cast, reassociation or FMA.
/// Fixed scalar work preserves inputs, without mutation or external effects.
/// # Errors
/// Returns `IntegerRange` above `2^53`, a checked-conversion disagreement, or the
/// exact component/step's actual shared arithmetic error. No partial value escapes.
fn integer_to_f64(
    value: u64,
    component: NormalizationComponent,
) -> Result<f64, NormalizationError> {
    if value > MAX_EXACT_INTEGER {
        return Err(NormalizationError::IntegerRange { component });
    }
    let high = u32::try_from(value >> 32)
        .map_err(|_| NormalizationError::ConversionInvariant { component })?;
    let low = u32::try_from(value & u64::from(u32::MAX))
        .map_err(|_| NormalizationError::ConversionInvariant { component })?;
    let high_product = normalization_arithmetic(
        f64::from(high),
        4_294_967_296.0,
        component,
        NormalizationStep::IntegerHighProduct,
    )?;
    normalization_arithmetic(
        high_product,
        f64::from(low),
        component,
        NormalizationStep::IntegerLowSum,
    )
}

/// Convert a raw exact nonnegative duration into negative relative seconds or `+0`.
///
/// All `u64` arguments are accepted; the integer helper enforces its independent
/// `2^53` envelope. Division by exact `1_000_000_000.0` is the only inexact step;
/// unary sign application is exact, with explicit positive zero for elapsed zero.
/// Successful nonzero results are finite normal and negative. This helper has no
/// hidden window precondition: the public producer separately enforces its exact
/// `4096`-second window before calling it. Work is constant and effect-free.
/// # Errors
/// Preserves the raw integer-helper failure or the actual time-component division
/// cause. No absolute reading, caller state or partial result is published.
fn relative_seconds(elapsed_nanoseconds: u64) -> Result<f64, NormalizationError> {
    let exact = integer_to_f64(elapsed_nanoseconds, NormalizationComponent::Time)?;
    let seconds = normalization_arithmetic(
        exact,
        1_000_000_000.0,
        NormalizationComponent::Time,
        NormalizationStep::SecondsDivide,
    )?;
    if elapsed_nanoseconds == 0 {
        Ok(0.0)
    } else {
        Ok(-seconds)
    }
}

/// Exactly convert an observation's already narrow immutable phase to seconds.
///
/// `FixtureObservation::new` owns the inclusive numerator envelope `[-2^33,2^33]`;
/// unchanged typed admission is reused without a raw construction route. Checked
/// signed conversion and `unsigned_abs` precede exact integer reconstruction,
/// then separate division by exact `2^33` and exact sign application. Zero is
/// explicitly positive; other results are exact finite normal values in `[-1,1]`.
/// No nanosecond rounding, mutation, allocation growth or external effect occurs.
/// # Errors
/// Preserves the actual component/step arithmetic error or a checked-conversion
/// disagreement with the immutable observation owner; no partial value escapes.
fn phase_seconds(observation: &FixtureObservation) -> Result<f64, NormalizationError> {
    let signed = i64::try_from(observation.phase.numerator()).map_err(|_| {
        NormalizationError::ConversionInvariant {
            component: NormalizationComponent::Phase,
        }
    })?;
    let exact = integer_to_f64(signed.unsigned_abs(), NormalizationComponent::Phase)?;
    let seconds = normalization_arithmetic(
        exact,
        8_589_934_592.0,
        NormalizationComponent::Phase,
        NormalizationStep::SecondsDivide,
    )?;
    match signed.cmp(&0) {
        std::cmp::Ordering::Equal => Ok(0.0),
        std::cmp::Ordering::Less => Ok(-seconds),
        std::cmp::Ordering::Greater => Ok(seconds),
    }
}

/// Immutable insufficient outcome; only the checked producer can create it.
///
/// Its private count is `0..=2`, and the exact actual newest reference is absent
/// exactly when that count is zero. Context/order/window admission precedes
/// publication. No point, batch, coefficient or statistic accompanies this outcome.
#[derive(Clone, Copy)]
pub struct InsufficientFixture {
    /// Immutable asserted fixture context shared by every admitted row.
    context: FixtureContext,
    /// Exact admitted insufficient count, in inclusive `0..=2`.
    sample_count: usize,
    /// Actual exact last reading, absent only for zero rows; never formatted.
    reference: Option<MonotonicNs>,
}

impl fmt::Debug for InsufficientFixture {
    /// Format permitted type/context/count and reference presence, without clocks.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InsufficientFixture")
            .field("context", &self.context)
            .field("sample_count", &self.sample_count)
            .field("reference_present", &self.reference.is_some())
            .finish_non_exhaustive()
    }
}

impl InsufficientFixture {
    /// Return unchanged asserted context, with no new validation or authority.
    #[must_use]
    pub const fn context(&self) -> FixtureContext {
        self.context
    }

    /// Return the immutable admitted `0..=2` count without scanning or effects.
    #[must_use]
    pub const fn sample_count(&self) -> usize {
        self.sample_count
    }

    /// Return the exact newest supplied reading, absent exactly for zero rows.
    #[must_use]
    pub const fn reference(&self) -> Option<MonotonicNs> {
        self.reference
    }
}

/// Immutable association of a fresh checked input, exact reference and context.
///
/// The sole producer owns all fields together after successful admission;
/// no public or private mixed-input factory is supplied. Count and point storage
/// remain owned by the contained `FitInput`; cloning preserves that whole value.
/// Owned fields imply no RSS/resource qualification, and no input borrow is
/// retained by a published payload.
#[derive(Clone)]
pub struct PreparedFixtureInput {
    /// Exact same asserted fixture context as every admitted source observation.
    context: FixtureContext,
    /// Exact actual last monotonic reading, excluded from formatting.
    reference: MonotonicNs,
    /// Fresh checked input, with frame equal to the paired context's frame.
    input: FitInput,
}

impl fmt::Debug for PreparedFixtureInput {
    /// Format type/context/count only, excluding reference and transformed values.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PreparedFixtureInput")
            .field("context", &self.context)
            .field("sample_count", &self.sample_count())
            .finish_non_exhaustive()
    }
}

impl PreparedFixtureInput {
    /// Return the unchanged whole asserted context without effects or authority.
    #[must_use]
    pub const fn context(&self) -> FixtureContext {
        self.context
    }

    /// Return the held input's checked `3..=64` count, without a second count owner.
    #[must_use]
    pub const fn sample_count(&self) -> usize {
        self.input.sample_count()
    }

    /// Return the exact supplied newest monotonic reading without conversion.
    #[must_use]
    pub const fn reference(&self) -> MonotonicNs {
        self.reference
    }

    /// Borrow the exact paired immutable checked input; retain no mutable escape.
    #[must_use]
    pub const fn input(&self) -> &FitInput {
        &self.input
    }

    /// Borrow an actual admitted prefix point, or return `None` outside that prefix.
    ///
    /// All `usize` arguments, including the count and `usize::MAX`, are handled
    /// without panic. The returned borrow is tied to `self`; it neither exposes
    /// unused fixed-capacity slots nor reconstructs a point. Work is constant,
    /// with no constructor, preparation, arithmetic, mutation or external effect.
    #[must_use]
    pub fn point(&self, index: usize) -> Option<&FitPoint> {
        self.input.batch().values.get(index)
    }
}

/// Complete immutable normalization outcome, whose payloads have private fields.
///
/// Wrapping an existing payload changes no admitted metadata/data association.
/// No default, deserialization, raw factory, unchecked extraction or mutation is
/// supplied; only the same-input checked producer creates a new payload.
#[derive(Clone)]
#[expect(
    clippy::large_enum_variant,
    reason = "the frozen API deliberately owns one bounded checked input for Prepared and only metadata for Insufficient; no allocation or RSS claim follows"
)]
pub enum FixtureNormalization {
    /// Checked context/order/window with too few actual observations to fit.
    Insufficient(InsufficientFixture),
    /// Fresh checked input paired with its actual newest reference and context.
    Prepared(PreparedFixtureInput),
}

impl fmt::Debug for FixtureNormalization {
    /// Format only the variant and its payload's permitted bounded metadata.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Insufficient(value) => formatter
                .debug_tuple("FixtureNormalization::Insufficient")
                .field(value)
                .finish(),
            Self::Prepared(value) => formatter
                .debug_tuple("FixtureNormalization::Prepared")
                .field(value)
                .finish(),
        }
    }
}

/// Prepare exact fixture observations relative to their actual newest reference.
///
/// Bound count before element access, check every context before exact time order,
/// then check every exact domain duration and maximum window before any float.
/// Counts `0..=2` publish only admitted insufficient metadata. Counts `3..=64`
/// additionally enforce the exact minimum span, convert rows chronologically,
/// construct exactly one actual `FitPoint` per row and one fresh `FitInput`, then
/// publish its paired immutable context/reference. Parent spread admission remains
/// authoritative. Scratch is one fixed duration array and one fixed point array;
/// work is O(n), n<=64, without fit/statistics, ambient reads or external effects.
/// Rejection preserves observations and publishes no partial result; successful
/// owned payloads retain no input borrow or native/resource qualification.
/// # Errors
/// Count precedes all-context mismatch, first non-increasing pair, chronological
/// domain/elapsed-window failure, sufficient-batch minimum span, conversion/point
/// failures and the actual final input-owner error. Every issued index is below
/// `64`; invariant disagreements fail closed rather than panic or clamp.
pub fn prepare_fixture_input(
    expected_context: FixtureContext,
    observations: &[FixtureObservation],
) -> Result<FixtureNormalization, NormalizationError> {
    let count = observations.len();
    if count > MAX_POINTS {
        return Err(NormalizationError::CountOutOfRange);
    }
    for (index, observation) in observations.iter().enumerate() {
        if observation.context != expected_context {
            return Err(NormalizationError::ContextMismatch { index });
        }
    }
    for (index, pair) in observations.windows(2).enumerate() {
        if pair[0].observed_at.nanoseconds() >= pair[1].observed_at.nanoseconds() {
            return Err(NormalizationError::NonIncreasingTime { index: index + 1 });
        }
    }
    let reference = observations
        .last()
        .map(|observation| observation.observed_at);
    let mut elapsed_nanoseconds = [0_u64; MAX_POINTS];
    if let Some(reference) = reference {
        for (index, observation) in observations.iter().enumerate() {
            let elapsed = reference
                .duration_since(observation.observed_at)
                .map_err(|source| NormalizationError::Domain { index, source })?
                .nanoseconds();
            if elapsed > MAX_ELAPSED_NANOSECONDS {
                return Err(NormalizationError::ElapsedRange { index });
            }
            elapsed_nanoseconds[index] = elapsed;
        }
    }
    if count < MIN_POINTS {
        return Ok(FixtureNormalization::Insufficient(InsufficientFixture {
            context: expected_context,
            sample_count: count,
            reference,
        }));
    }
    if elapsed_nanoseconds[0] < MIN_SPAN_NANOSECONDS {
        return Err(NormalizationError::SpanTooShort);
    }
    let reference = reference.ok_or(NormalizationError::ConversionInvariant {
        component: NormalizationComponent::Time,
    })?;
    let first_observation =
        observations
            .first()
            .ok_or(NormalizationError::ConversionInvariant {
                component: NormalizationComponent::Time,
            })?;
    let first_relative = relative_seconds(elapsed_nanoseconds[0])?;
    let first_phase = phase_seconds(first_observation)?;
    let first = FitPoint::new(
        first_relative,
        first_phase,
        first_observation.variance_divisor,
    )
    .map_err(|source| NormalizationError::Point { index: 0, source })?;
    let mut points = [first; MAX_POINTS];
    for (index, observation) in observations.iter().enumerate().skip(1) {
        let relative = relative_seconds(elapsed_nanoseconds[index])?;
        let phase = phase_seconds(observation)?;
        points[index] = FitPoint::new(relative, phase, observation.variance_divisor)
            .map_err(|source| NormalizationError::Point { index, source })?;
    }
    let input = FitInput::new(expected_context.frame, &points[..count])
        .map_err(NormalizationError::Input)?;
    Ok(FixtureNormalization::Prepared(PreparedFixtureInput {
        context: expected_context,
        reference,
        input,
    }))
}

/// Raw predicate, mandatory garde, primitive and unsigned-envelope controls only.
#[cfg(test)]
#[path = "../../tests/private/normalization_guards.rs"]
mod normalization_guards;

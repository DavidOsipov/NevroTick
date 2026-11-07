// SPDX-License-Identifier: GPL-2.0-only
//! Pure, bounded preparation of one declared observation population.
//!
//! Exact half-nanosecond reference and corrected-phase differences are formed
//! before their single rational-to-binary64 conversion. Exact integer spans select
//! power-of-two conditioning scales. Every point retains its actual borrowed row,
//! conversion bounds and unchanged whole-population variance divisor. Declared
//! labels assert associations only: they establish no physical source, freshness,
//! authentication, fit, history admission or clock authority. Work uses fixed
//! storage without successful heap allocation, ambient reads, I/O or mutation.

use std::{cmp::Ordering, fmt, num::NonZeroU64};

use garde::Validate;
use time_domain::{RationalSecondsError, RationalSign, RoundedRationalSeconds};

use crate::{
    WindowVarianceDivisors, WindowVarianceError, window_variance::positive_finite_seconds,
};

/// Fixed maximum population, admitted before any original row is inspected.
const MAX_ROWS: usize = 64;
/// Exact number of half-nanosecond ticks in one second.
const REFERENCE_DENOMINATOR: u64 = 2_000_000_000;
/// Exact corrected-phase denominator, preserving the engine's rational chart.
const PHASE_DENOMINATOR: u64 = 8_589_934_592_000_000_000;
/// Binary64 exponent field, separated from immutable sign and significand bits.
const EXPONENT_MASK: u64 = 0x7ff0_0000_0000_0000;

/// Coordinate family for a bounded nested rational-conversion failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConditionedCoordinate {
    /// Exact self-minus-newest reference seconds.
    Reference,
    /// Exact self-minus-newest corrected-phase seconds.
    Phase,
}

/// Payload-free admission failures, with only permitted counts, indexes and causes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConditionedWindowError {
    /// The reference exceeds the selected u64-nanosecond capture chart.
    InvalidReference,
    /// A row distance is not positive finite seconds.
    InvalidPeerDistance,
    /// Count admission failed before any input row was traversed.
    CountOutOfRange {
        /// Original raw slice length, without copied rows or numeric payloads.
        count: usize,
    },
    /// Explicit current precision is not positive finite seconds.
    InvalidPrecision,
    /// The first row whose declared context differs from the expected context.
    ContextMismatch {
        /// Original index, smaller than the admitted maximum population.
        index: usize,
    },
    /// The first observation key repeated in original order.
    DuplicateObservationKey {
        /// Original index of the repeated key, without retaining the key.
        index: usize,
    },
    /// The first reference not strictly greater than its exact predecessor.
    NonIncreasingReference {
        /// Original index of the non-increasing reference.
        index: usize,
    },
    /// Whole-population variance derivation refused its admitted numeric inputs.
    Quality {
        /// Exact bounded cause; never a Garde report or raw scalar.
        cause: WindowVarianceError,
    },
    /// The shared exact rational converter refused a coordinate.
    Conversion {
        /// Original index of the refused coordinate.
        index: usize,
        /// Fixed coordinate family, without timestamps or phase numerators.
        coordinate: ConditionedCoordinate,
        /// Exact bounded converter cause.
        cause: RationalSecondsError,
    },
    /// Exact ordered references collided after once-rounded conditioning.
    RoundedReferenceCollision {
        /// Original index of the first rounded non-increasing reference.
        index: usize,
    },
    /// Checked integer arithmetic contradicted its documented bounded envelope.
    ArithmeticInvariant,
    /// A transformed value contradicted its mandatory output contract.
    OutputInvariant,
    /// Mandatory validation disagreed with the maintained shared predicates.
    ValidationInvariant,
}

impl fmt::Display for ConditionedWindowError {
    /// Format only fixed failure families and permitted counts or indexes.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidReference => formatter.write_str("invalid declared reference"),
            Self::InvalidPeerDistance => formatter.write_str("invalid declared peer distance"),
            Self::CountOutOfRange { count } => {
                write!(formatter, "conditioned count {count} outside 0..64")
            }
            Self::InvalidPrecision => formatter.write_str("invalid current precision"),
            Self::ContextMismatch { index } => {
                write!(formatter, "declared context mismatch at {index}")
            }
            Self::DuplicateObservationKey { index } => {
                write!(formatter, "duplicate observation key at {index}")
            }
            Self::NonIncreasingReference { index } => {
                write!(formatter, "non-increasing exact reference at {index}")
            }
            Self::Quality { .. } => formatter.write_str("conditioned quality derivation failed"),
            Self::Conversion {
                index, coordinate, ..
            } => write!(
                formatter,
                "conditioned {coordinate:?} conversion failed at {index}"
            ),
            Self::RoundedReferenceCollision { index } => {
                write!(formatter, "rounded reference collision at {index}")
            }
            Self::ArithmeticInvariant => formatter.write_str("conditioned arithmetic invariant"),
            Self::OutputInvariant => formatter.write_str("conditioned output invariant"),
            Self::ValidationInvariant => formatter.write_str("conditioned validation invariant"),
        }
    }
}

impl std::error::Error for ConditionedWindowError {
    /// Borrow only bounded typed causes, never original rows or validator reports.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Quality { cause } => Some(cause),
            Self::Conversion { cause, .. } => Some(cause),
            Self::InvalidReference
            | Self::InvalidPeerDistance
            | Self::CountOutOfRange { .. }
            | Self::InvalidPrecision
            | Self::ContextMismatch { .. }
            | Self::DuplicateObservationKey { .. }
            | Self::NonIncreasingReference { .. }
            | Self::RoundedReferenceCollision { .. }
            | Self::ArithmeticInvariant
            | Self::OutputInvariant
            | Self::ValidationInvariant => None,
        }
    }
}

/// Five immutable nonzero caller declarations, with no genuine-source authority.
///
/// ```compile_fail,E0451
/// use std::num::NonZeroU64;
/// use time_core::DeclaredObservationContext;
/// fn bypass(base: DeclaredObservationContext) {
///     let _ = DeclaredObservationContext { logical_source: NonZeroU64::MIN, ..base };
/// }
/// ```
///
/// ```compile_fail,E0277
/// use time_core::DeclaredObservationContext;
/// let _: DeclaredObservationContext = Default::default();
/// ```
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct DeclaredObservationContext {
    /// Nonzero logical-source declaration; its type owns nonzero admission.
    logical_source: NonZeroU64,
    /// Nonzero declared configuration generation.
    configuration_generation: NonZeroU64,
    /// Nonzero declared association generation.
    association_generation: NonZeroU64,
    /// Nonzero declared clock generation.
    clock_generation: NonZeroU64,
    /// Nonzero declared mapping generation.
    mapping_generation: NonZeroU64,
}

impl fmt::Debug for DeclaredObservationContext {
    /// Redact caller labels instead of treating numeric declarations as diagnostics.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("DeclaredObservationContext { .. }")
    }
}

impl DeclaredObservationContext {
    /// Retain five typed nonzero labels unchanged, without effects or validation debt.
    #[must_use]
    pub const fn new(
        logical_source: NonZeroU64,
        configuration_generation: NonZeroU64,
        association_generation: NonZeroU64,
        clock_generation: NonZeroU64,
        mapping_generation: NonZeroU64,
    ) -> Self {
        Self {
            logical_source,
            configuration_generation,
            association_generation,
            clock_generation,
            mapping_generation,
        }
    }

    /// Return the unchanged nonzero declared logical-source label.
    #[must_use]
    pub const fn logical_source(self) -> NonZeroU64 {
        self.logical_source
    }
    /// Return the unchanged nonzero declared configuration generation.
    #[must_use]
    pub const fn configuration_generation(self) -> NonZeroU64 {
        self.configuration_generation
    }
    /// Return the unchanged nonzero declared association generation.
    #[must_use]
    pub const fn association_generation(self) -> NonZeroU64 {
        self.association_generation
    }
    /// Return the unchanged nonzero declared clock generation.
    #[must_use]
    pub const fn clock_generation(self) -> NonZeroU64 {
        self.clock_generation
    }
    /// Return the unchanged nonzero declared mapping generation.
    #[must_use]
    pub const fn mapping_generation(self) -> NonZeroU64 {
        self.mapping_generation
    }
}

/// Checked exact numeric row, carrying declarations rather than physical provenance.
///
/// Its phase is the complete `i128` numerator over
/// `8_589_934_592_000_000_000` seconds. Reference half ticks and positive subnormal
/// distances are retained. There is no mutation, default or unchecked conversion.
///
/// ```
/// use std::num::NonZeroU64;
/// use time_core::{DeclaredObservationContext, ExactObservationRow};
/// let tag = NonZeroU64::MIN;
/// let context = DeclaredObservationContext::new(tag, tag, tag, tag, tag);
/// let row = ExactObservationRow::new(context, tag, 1, true, i128::MIN, 1.0)?;
/// assert_eq!(row.reference_whole_ns(), 1);
/// assert!(row.reference_half());
/// assert_eq!(row.corrected_phase_numerator(), i128::MIN);
/// # Ok::<(), time_core::ConditionedWindowError>(())
/// ```
///
/// ```compile_fail,E0451
/// use time_core::ExactObservationRow;
/// fn bypass(base: ExactObservationRow) {
///     let _ = ExactObservationRow { peer_distance_seconds: 1.0, ..base };
/// }
/// ```
///
/// ```compile_fail,E0616
/// use time_core::ExactObservationRow;
/// let _ = |mut row: ExactObservationRow| { row.corrected_phase_numerator = 0; };
/// ```
///
/// ```compile_fail,E0277
/// use time_core::ExactObservationRow;
/// let _: ExactObservationRow = Default::default();
/// ```
///
/// ```compile_fail,E0277
/// use time_core::ExactObservationRow;
/// let _: ExactObservationRow = (0_u64, 0_i128, 1.0_f64).into();
/// ```
#[derive(Clone, Copy)]
pub struct ExactObservationRow {
    /// Typed immutable caller context, checked against the expected context in batch admission.
    context: DeclaredObservationContext,
    /// Nonzero caller observation key; genuine replay prevention remains outside N1.
    observation_key: NonZeroU64,
    /// Exact whole nanoseconds in the selected unsigned capture chart.
    reference_whole_ns: u64,
    /// Exact additional half nanosecond, with the maximum-whole combination rejected.
    reference_half: bool,
    /// Exact full-domain corrected-phase numerator over the fixed phase denominator.
    corrected_phase_numerator: i128,
    /// Checked positive finite seconds, preserving all supplied bits.
    peer_distance_seconds: f64,
}

impl fmt::Debug for ExactObservationRow {
    /// Redact original timestamps, labels, keys, phase and distance bits.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ExactObservationRow { .. }")
    }
}

/// Raw row fields, with typed full-domain fields explicitly owned by their types.
#[derive(Validate)]
struct RowCandidate {
    /// Context's private typed constructor owns all five nonzero labels.
    #[garde(skip)]
    context: DeclaredObservationContext,
    /// `NonZeroU64` owns the complete admitted observation-key envelope.
    #[garde(skip)]
    observation_key: NonZeroU64,
    /// One shared rule owns the combined exact reference chart boundary.
    #[garde(custom(reference_rule))]
    reference: (u64, bool),
    /// The primitive `i128` type owns the full admitted phase-numerator domain.
    #[garde(skip)]
    corrected_phase_numerator: i128,
    /// Shared positive-finite seconds rule, including positive subnormals.
    #[garde(custom(seconds_rule))]
    peer_distance_seconds: f64,
}

/// Decide the sole excluded reference pair without arithmetic or effects.
fn valid_reference(reference: (u64, bool)) -> bool {
    reference != (u64::MAX, true)
}

/// Adapt exact reference admission to mandatory Garde validation.
/// # Errors
/// Reject the single reference beyond the selected capture chart with fixed text.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow their unit validation context"
)]
fn reference_rule(reference: &(u64, bool), (): &()) -> garde::Result {
    if valid_reference(*reference) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid exact reference"))
    }
}

/// Adapt the existing quality owner's positive-finite seconds rule.
/// # Errors
/// Reject nonpositive or nonfinite seconds with no retained scalar or report.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow raw fields"
)]
fn seconds_rule(value: &f64, (): &()) -> garde::Result {
    if positive_finite_seconds(*value) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid positive seconds"))
    }
}

impl ExactObservationRow {
    /// Admit raw numeric fields through the sole mandatory shared validation path.
    ///
    /// Typed labels/key and full-domain phase require no narrower policy. Positive
    /// subnormal distances pass unchanged. Successful work allocates nothing;
    /// rejected temporary Garde reports are bounded and discarded.
    /// # Errors
    /// Reject the reference boundary before an invalid distance, then validator
    /// disagreement. No caller input changes or partial row escapes.
    pub fn new(
        context: DeclaredObservationContext,
        observation_key: NonZeroU64,
        reference_whole_ns: u64,
        reference_half: bool,
        corrected_phase_numerator: i128,
        peer_distance_seconds: f64,
    ) -> Result<Self, ConditionedWindowError> {
        let candidate = RowCandidate {
            context,
            observation_key,
            reference: (reference_whole_ns, reference_half),
            corrected_phase_numerator,
            peer_distance_seconds,
        };
        candidate.validate().map_err(|_| {
            if !valid_reference(candidate.reference) {
                ConditionedWindowError::InvalidReference
            } else if !positive_finite_seconds(candidate.peer_distance_seconds) {
                ConditionedWindowError::InvalidPeerDistance
            } else {
                ConditionedWindowError::ValidationInvariant
            }
        })?;
        Ok(Self {
            context: candidate.context,
            observation_key: candidate.observation_key,
            reference_whole_ns: candidate.reference.0,
            reference_half: candidate.reference.1,
            corrected_phase_numerator: candidate.corrected_phase_numerator,
            peer_distance_seconds: candidate.peer_distance_seconds,
        })
    }

    /// Return the unchanged declared association without renewing authority.
    #[must_use]
    pub const fn context(self) -> DeclaredObservationContext {
        self.context
    }
    /// Return the immutable nonzero asserted observation key.
    #[must_use]
    pub const fn observation_key(self) -> NonZeroU64 {
        self.observation_key
    }
    /// Return exact whole nanoseconds without flooring the separate half tick.
    #[must_use]
    pub const fn reference_whole_ns(self) -> u64 {
        self.reference_whole_ns
    }
    /// Return the exact additional half-nanosecond bit.
    #[must_use]
    pub const fn reference_half(self) -> bool {
        self.reference_half
    }
    /// Return the unchanged full-domain phase numerator over the fixed denominator.
    #[must_use]
    pub const fn corrected_phase_numerator(self) -> i128 {
        self.corrected_phase_numerator
    }
    /// Return admitted positive finite peer-distance seconds with unchanged bits.
    #[must_use]
    pub const fn peer_distance_seconds(self) -> f64 {
        self.peer_distance_seconds
    }
}

/// One immutable prepared coordinate, borrowing its actual original population row.
///
/// Only whole-window preparation constructs this point. Its divisor cannot be
/// independently attached to another row, and exact tuples and arithmetic bounds
/// remain available without claiming physical uncertainty or fit readiness.
///
/// ```compile_fail,E0451
/// use time_core::PreparedConditionedPoint;
/// fn bypass(base: PreparedConditionedPoint<'_>) {
///     let _ = PreparedConditionedPoint { variance_divisor: 1.0, ..base };
/// }
/// ```
///
/// ```compile_fail,E0277
/// use time_core::PreparedConditionedPoint;
/// let _: PreparedConditionedPoint<'_> = Default::default();
/// ```
#[derive(Clone, Copy)]
pub struct PreparedConditionedPoint<'a> {
    /// Actual borrowed row, never a reconstructed row or independently labelled copy.
    original_row: &'a ExactObservationRow,
    /// Once-rounded exact signed half-tick self-minus-anchor seconds.
    reference_delta: RoundedRationalSeconds,
    /// Once-rounded exact full-domain phase self-minus-anchor seconds.
    phase_delta: RoundedRationalSeconds,
    /// Exactly scaled reference coordinate, normal or positive zero in [-1,0].
    normalized_reference: f64,
    /// Exactly scaled phase coordinate, normal or positive zero in [-1,1].
    normalized_phase: f64,
    /// Exactly scaled reference-conversion bound, positive normal or positive zero.
    normalized_reference_roundoff: f64,
    /// Exactly scaled phase-conversion bound, positive normal or positive zero.
    normalized_phase_roundoff: f64,
    /// Unchanged whole-window divisor at this original row's index.
    variance_divisor: f64,
}

impl fmt::Debug for PreparedConditionedPoint<'_> {
    /// Redact both original rows and transformed numeric payloads.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PreparedConditionedPoint { .. }")
    }
}

impl<'a> PreparedConditionedPoint<'a> {
    /// Borrow the actual input row without copying or changing its lifetime.
    #[must_use]
    pub const fn original_row(&self) -> &'a ExactObservationRow {
        self.original_row
    }
    /// Return the exact reference tuple, once-rounded seconds and arithmetic bound.
    #[must_use]
    pub const fn reference_delta(&self) -> RoundedRationalSeconds {
        self.reference_delta
    }
    /// Return the exact phase tuple, once-rounded seconds and arithmetic bound.
    #[must_use]
    pub const fn phase_delta(&self) -> RoundedRationalSeconds {
        self.phase_delta
    }
    /// Return the checked reference coordinate in [-1,0] with canonical zero.
    #[must_use]
    pub const fn normalized_reference(&self) -> f64 {
        self.normalized_reference
    }
    /// Return the checked phase coordinate in [-1,1] with canonical zero.
    #[must_use]
    pub const fn normalized_phase(&self) -> f64 {
        self.normalized_phase
    }
    /// Return only the scaled arithmetic reference-conversion bound.
    #[must_use]
    pub const fn normalized_reference_roundoff(&self) -> f64 {
        self.normalized_reference_roundoff
    }
    /// Return only the scaled arithmetic phase-conversion bound.
    #[must_use]
    pub const fn normalized_phase_roundoff(&self) -> f64 {
        self.normalized_phase_roundoff
    }
    /// Return this row's unchanged uncapped whole-population divisor.
    #[must_use]
    pub const fn variance_divisor(&self) -> f64 {
        self.variance_divisor
    }
}

/// Sealed complete preparation of one original 3..=64-row population.
///
/// Points and original rows are immutable. Indexes outside the retained count,
/// including `usize::MAX`, return `None`. No mutation API is exposed.
///
/// ```compile_fail,E0451
/// use time_core::PreparedConditionedBatch;
/// fn bypass(base: PreparedConditionedBatch<'_>) {
///     let _ = PreparedConditionedBatch { time_scale_exponent: 0, ..base };
/// }
/// ```
///
/// ```compile_fail,E0616
/// use time_core::PreparedConditionedBatch;
/// let _ = |mut batch: PreparedConditionedBatch<'_>| { batch.phase_scale_exponent = 0; };
/// ```
///
/// ```compile_fail,E0277
/// use time_core::PreparedConditionedBatch;
/// let _: PreparedConditionedBatch<'_> = Default::default();
/// ```
///
/// ```compile_fail,E0277
/// use time_core::{ExactObservationRow, PreparedConditionedBatch};
/// let _ = |rows: &[ExactObservationRow]| -> PreparedConditionedBatch<'_> { rows.into() };
/// ```
pub struct PreparedConditionedBatch<'a> {
    /// Immutable expected declared association admitted for every retained row.
    declared_context: DeclaredObservationContext,
    /// Original immutable complete population, retaining source order and bits.
    original_rows: &'a [ExactObservationRow],
    /// Explicit positive finite current precision, unchanged from admission.
    current_precision_seconds: f64,
    /// Actual newest original row, preserving both exact anchors.
    newest_row: &'a ExactObservationRow,
    /// Smallest exact-span covering binary exponent in [-29,35].
    time_scale_exponent: i32,
    /// Smallest exact maximum-phase covering exponent in [-62,66], or zero.
    phase_scale_exponent: i32,
    /// Only the admitted original-length prefix is observable; tail copies are hidden.
    points: [PreparedConditionedPoint<'a>; MAX_ROWS],
}

impl fmt::Debug for PreparedConditionedBatch<'_> {
    /// Redact anchors, contexts, original rows and all numerical point payloads.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PreparedConditionedBatch { .. }")
    }
}

impl<'a> PreparedConditionedBatch<'a> {
    /// Return the unchanged expected declared association, without current authority.
    #[must_use]
    pub const fn declared_context(&self) -> DeclaredObservationContext {
        self.declared_context
    }
    /// Borrow the unchanged complete original row slice.
    #[must_use]
    pub const fn original_rows(&self) -> &'a [ExactObservationRow] {
        self.original_rows
    }
    /// Return the original admitted population count, between three and 64.
    #[must_use]
    pub const fn sample_count(&self) -> usize {
        self.original_rows.len()
    }
    /// Return explicit current precision without fresh ambient lookup or rounding.
    #[must_use]
    pub const fn current_precision_seconds(&self) -> f64 {
        self.current_precision_seconds
    }
    /// Borrow the actual newest row, retaining both exact anchors.
    #[must_use]
    pub const fn newest_row(&self) -> &'a ExactObservationRow {
        self.newest_row
    }
    /// Return the exact-span time exponent; normalized x is x times 2^-exponent.
    #[must_use]
    pub const fn time_scale_exponent(&self) -> i32 {
        self.time_scale_exponent
    }
    /// Return the exact-maximum phase exponent, or zero for all-zero differences.
    #[must_use]
    pub const fn phase_scale_exponent(&self) -> i32 {
        self.phase_scale_exponent
    }
    /// Borrow only an admitted original-order point; every larger index is absent.
    #[must_use]
    pub fn point(&self, index: usize) -> Option<&PreparedConditionedPoint<'a>> {
        if index < self.original_rows.len() {
            self.points.get(index)
        } else {
            None
        }
    }
}

/// Admitted 0..=2-row population, retaining quality without normalized geometry.
///
/// Empty input has no anchor or divisor vector. One/two rows retain unchanged
/// derived divisors and actual row anchors. No fit-ready point or scale is exposed.
///
/// ```compile_fail,E0451
/// use time_core::InsufficientConditionedBatch;
/// fn bypass(base: InsufficientConditionedBatch<'_>) {
///     let _ = InsufficientConditionedBatch { current_precision_seconds: 0.0, ..base };
/// }
/// ```
///
/// ```compile_fail,E0616
/// use time_core::InsufficientConditionedBatch;
/// let _ = |mut batch: InsufficientConditionedBatch<'_>| { batch.current_precision_seconds = 0.0; };
/// ```
///
/// ```compile_fail,E0277
/// use time_core::InsufficientConditionedBatch;
/// let _: InsufficientConditionedBatch<'_> = Default::default();
/// ```
pub struct InsufficientConditionedBatch<'a> {
    /// Expected declared association checked before quality derivation.
    declared_context: DeclaredObservationContext,
    /// Unchanged complete original short population.
    original_rows: &'a [ExactObservationRow],
    /// Admitted explicit current precision, even when the population is empty.
    current_precision_seconds: f64,
    /// No vector for empty input; unchanged bounded prefix for one/two rows.
    divisors: Option<[f64; MAX_ROWS]>,
}

impl fmt::Debug for InsufficientConditionedBatch<'_> {
    /// Redact declared labels, row payloads, precision and quality values.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("InsufficientConditionedBatch { .. }")
    }
}

impl<'a> InsufficientConditionedBatch<'a> {
    /// Return the unchanged expected declared association.
    #[must_use]
    pub const fn declared_context(&self) -> DeclaredObservationContext {
        self.declared_context
    }
    /// Borrow the unchanged original short population, including an empty slice.
    #[must_use]
    pub const fn original_rows(&self) -> &'a [ExactObservationRow] {
        self.original_rows
    }
    /// Return the admitted count, between zero and two.
    #[must_use]
    pub const fn sample_count(&self) -> usize {
        self.original_rows.len()
    }
    /// Return admitted explicit precision with all input bits unchanged.
    #[must_use]
    pub const fn current_precision_seconds(&self) -> f64 {
        self.current_precision_seconds
    }
    /// Borrow the actual newest row when present, without manufacturing an anchor.
    #[must_use]
    pub fn newest_row(&self) -> Option<&'a ExactObservationRow> {
        self.original_rows.last()
    }
    /// Return a retained original-index divisor, or None at every absent index.
    #[must_use]
    pub fn variance_divisor(&self, index: usize) -> Option<f64> {
        if index >= self.original_rows.len() {
            return None;
        }
        self.divisors
            .as_ref()
            .and_then(|divisors| divisors.get(index))
            .copied()
    }
}

/// Closed numerical outcome, separating complete preparation from insufficiency.
#[expect(
    clippy::large_enum_variant,
    reason = "bounded inline storage preserves the no-successful-heap-allocation contract"
)]
pub enum ConditionedBatchOutcome<'a> {
    /// One complete admitted 3..=64-row conditioned population.
    Prepared(PreparedConditionedBatch<'a>),
    /// One admitted short population, with no normalized points or fit readiness.
    Insufficient(InsufficientConditionedBatch<'a>),
}

impl fmt::Debug for ConditionedBatchOutcome<'_> {
    /// Expose the outcome family while keeping all original/transformed data redacted.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Prepared(_) => "ConditionedBatchOutcome::Prepared { .. }",
            Self::Insufficient(_) => "ConditionedBatchOutcome::Insufficient { .. }",
        })
    }
}

/// Mandatory raw count admission, independent of all row elements.
#[derive(Validate)]
struct CountCandidate {
    /// One bounded count rule, admitting 0..=64 before any traversal.
    #[garde(custom(count_rule))]
    count: usize,
}

/// Decide raw count admission without inspecting a row or allocating storage.
fn valid_count(count: usize) -> bool {
    count <= MAX_ROWS
}

/// Adapt the fixed count predicate to mandatory Garde admission.
/// # Errors
/// Reject counts above 64 without traversing the supplied row slice.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow raw fields"
)]
fn count_rule(count: &usize, (): &()) -> garde::Result {
    if valid_count(*count) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid conditioned count"))
    }
}

/// Mandatory current-precision admission using the existing quality scalar owner.
#[derive(Validate)]
struct PrecisionCandidate {
    /// Raw explicit seconds, admitted even for an empty population.
    #[garde(custom(seconds_rule))]
    seconds: f64,
}

/// Convert every admitted reference to its exact 65-bit half-tick representation.
fn reference_ticks(row: &ExactObservationRow) -> u128 {
    u128::from(row.reference_whole_ns) * 2 + u128::from(row.reference_half)
}

/// Find the first association/order failure, with count bounded before traversal.
///
/// Contexts precede all keys; keys precede all exact references. This predicate
/// accepts any raw slice but never inspects an element if its count exceeds 64.
fn membership_error(
    rows: &[ExactObservationRow],
    expected: DeclaredObservationContext,
) -> Option<ConditionedWindowError> {
    if !valid_count(rows.len()) {
        return Some(ConditionedWindowError::CountOutOfRange { count: rows.len() });
    }
    for (index, row) in rows.iter().enumerate() {
        if row.context != expected {
            return Some(ConditionedWindowError::ContextMismatch { index });
        }
    }
    for (index, row) in rows.iter().enumerate() {
        if rows[..index]
            .iter()
            .any(|earlier| earlier.observation_key == row.observation_key)
        {
            return Some(ConditionedWindowError::DuplicateObservationKey { index });
        }
    }
    for (index, pair) in rows.windows(2).enumerate() {
        if reference_ticks(&pair[0]) >= reference_ticks(&pair[1]) {
            return Some(ConditionedWindowError::NonIncreasingReference { index: index + 1 });
        }
    }
    None
}

/// Mandatory whole-population association, key and exact-order validation.
#[derive(Validate)]
#[garde(context(DeclaredObservationContext))]
struct MembershipCandidate<'a> {
    /// One shared relation rule retains all original duplicates and row order.
    #[garde(custom(membership_rule))]
    rows: &'a [ExactObservationRow],
}

/// Adapt original-order membership rules without retaining or formatting row data.
/// # Errors
/// Reject the first fixed-order membership failure through one bounded report.
fn membership_rule(
    rows: &&[ExactObservationRow],
    expected: &DeclaredObservationContext,
) -> garde::Result {
    if membership_error(rows, *expected).is_none() {
        Ok(())
    } else {
        Err(garde::Error::new("invalid conditioned membership"))
    }
}

/// Derive exactly one complete quality vector and copy its unchanged admitted prefix.
///
/// All slices of checked rows and raw precisions are admitted at this boundary.
/// Oversized counts reject before scratch access; the existing quality owner
/// admits the nonempty count and precision. Scratch cannot escape or be self-borrowed.
/// # Errors
/// Preserve the quality owner's bounded typed cause; no partial vector escapes.
fn derive_divisors(
    rows: &[ExactObservationRow],
    precision: f64,
) -> Result<[f64; MAX_ROWS], ConditionedWindowError> {
    if !valid_count(rows.len()) {
        return Err(ConditionedWindowError::CountOutOfRange { count: rows.len() });
    }
    let mut distances = [0.0; MAX_ROWS];
    for (destination, row) in distances.iter_mut().zip(rows) {
        *destination = row.peer_distance_seconds;
    }
    let quality = WindowVarianceDivisors::derive(&distances[..rows.len()], precision)
        .map_err(|cause| ConditionedWindowError::Quality { cause })?;
    let mut divisors = [1.0; MAX_ROWS];
    divisors[..rows.len()].copy_from_slice(quality.divisors());
    Ok(divisors)
}

/// Compute ceil(log2(m/d)) with exact quotients and no shifted denominator.
///
/// Every magnitude and nonzero-u64 denominator is admitted; exact zero returns
/// zero. Nonzero results cover [-63,128], using only checked scalar arithmetic.
/// # Errors
/// Reject an impossible checked quotient/logarithm or integer-conversion failure.
fn ceiling_exponent(
    magnitude: u128,
    denominator: NonZeroU64,
) -> Result<i32, ConditionedWindowError> {
    if magnitude == 0 {
        return Ok(0);
    }
    let denominator = u128::from(denominator.get());
    let quotient = if magnitude >= denominator {
        magnitude / denominator
    } else {
        denominator / magnitude
    };
    let floor = quotient
        .checked_ilog2()
        .ok_or(ConditionedWindowError::ArithmeticInvariant)?;
    let floor = i32::try_from(floor).map_err(|_| ConditionedWindowError::ArithmeticInvariant)?;
    if magnitude < denominator {
        Ok(-floor)
    } else {
        floor
            .checked_add(i32::from(
                !magnitude.is_multiple_of(denominator) || !quotient.is_power_of_two(),
            ))
            .ok_or(ConditionedWindowError::ArithmeticInvariant)
    }
}

/// Construct the checked positive denominator without any substitute or fallback.
///
/// All raw u64 values are accepted as input; fixed nonzero chart constants pass.
/// # Errors
/// Refuse zero as an arithmetic-invariant contradiction without partial output.
fn denominator(value: u64) -> Result<NonZeroU64, ConditionedWindowError> {
    NonZeroU64::new(value).ok_or(ConditionedWindowError::ArithmeticInvariant)
}

/// Mandatory admission of exact-span selected exponents before coordinate work.
#[derive(Validate)]
struct ScaleCandidate {
    /// Three exact strictly ordered half-tick references imply the [-29,35] bound.
    #[garde(custom(time_exponent_rule))]
    time: i32,
    /// Full i128 phase differences over the selected denominator imply [-62,66].
    #[garde(custom(phase_exponent_rule))]
    phase: i32,
}

/// Admit exact-span selected exponents through the actual production output guard.
///
/// All raw i32 pairs are accepted as input. Only validated scale candidates are
/// returned for coordinate preparation; no normalization, substitution or effect occurs.
/// # Errors
/// Reject either exponent outside its frozen envelope, or a validator disagreement.
fn admit_scales(scales: ScaleCandidate) -> Result<ScaleCandidate, ConditionedWindowError> {
    scales.validate().map_err(|_| {
        if !valid_time_exponent(scales.time) || !valid_phase_exponent(scales.phase) {
            ConditionedWindowError::OutputInvariant
        } else {
            ConditionedWindowError::ValidationInvariant
        }
    })?;
    Ok(scales)
}

/// Decide the selected exact-reference exponent envelope without arithmetic effects.
fn valid_time_exponent(value: i32) -> bool {
    (-29..=35).contains(&value)
}
/// Decide the selected exact-phase exponent envelope, including all-zero exponent zero.
fn valid_phase_exponent(value: i32) -> bool {
    (-62..=66).contains(&value)
}

/// Adapt the exact-reference scale envelope to mandatory output validation.
/// # Errors
/// Reject a contradiction of the admitted three-row half-tick span bound.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow transformed fields"
)]
fn time_exponent_rule(value: &i32, (): &()) -> garde::Result {
    if valid_time_exponent(*value) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid time exponent"))
    }
}

/// Adapt the exact-phase scale envelope to mandatory output validation.
/// # Errors
/// Reject a contradiction of the admitted full-domain phase-difference bound.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow transformed fields"
)]
fn phase_exponent_rule(value: &i32, (): &()) -> garde::Result {
    if valid_phase_exponent(*value) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid phase exponent"))
    }
}

/// Admit a finite normal scalar or the unique positive-zero representation.
fn normal_or_positive_zero(value: f64) -> bool {
    value.is_normal() || value.to_bits() == 0
}

/// Scale a normal or positive zero by an exact binary power with bit-level control.
///
/// Every raw scalar/exponent is accepted as input. A supported normal product is
/// checked against the unchanged sign/significand and shifted exponent bits,
/// proving this multiplication introduces no additional rounding. Subnormal
/// input/output, nonfinite input/output, negative zero or exponent overflow reject.
/// # Errors
/// Return a fixed output invariant, never clamp or lose a nonzero value to zero.
fn exact_scale(value: f64, exponent: i32) -> Result<f64, ConditionedWindowError> {
    if !normal_or_positive_zero(value) {
        return Err(ConditionedWindowError::OutputInvariant);
    }
    let power_exponent = exponent
        .checked_add(1023)
        .ok_or(ConditionedWindowError::OutputInvariant)?;
    if !(1..=2046).contains(&power_exponent) {
        return Err(ConditionedWindowError::OutputInvariant);
    }
    let power_bits =
        u64::try_from(power_exponent).map_err(|_| ConditionedWindowError::OutputInvariant)? << 52;
    let scaled = value * f64::from_bits(power_bits);
    let expected_bits = if value.to_bits() == 0 {
        0
    } else {
        let original_exponent = i32::try_from((value.to_bits() & EXPONENT_MASK) >> 52)
            .map_err(|_| ConditionedWindowError::OutputInvariant)?;
        let output_exponent = original_exponent
            .checked_add(exponent)
            .ok_or(ConditionedWindowError::OutputInvariant)?;
        if !(1..=2046).contains(&output_exponent) {
            return Err(ConditionedWindowError::OutputInvariant);
        }
        let output_exponent =
            u64::try_from(output_exponent).map_err(|_| ConditionedWindowError::OutputInvariant)?;
        (value.to_bits() & !EXPONENT_MASK) | (output_exponent << 52)
    };
    if scaled.to_bits() != expected_bits {
        return Err(ConditionedWindowError::OutputInvariant);
    }
    Ok(scaled)
}

/// Checked raw transformed scalars, validated before their private point publishes.
#[derive(Clone, Copy, Validate)]
struct PointCandidate {
    /// Normal or positive-zero normalized reference within [-1,0].
    #[garde(custom(normalized_reference_rule))]
    reference: f64,
    /// Normal or positive-zero normalized phase within [-1,1].
    #[garde(custom(normalized_phase_rule))]
    phase: f64,
    /// Positive normal arithmetic-bound power of two, or canonical positive zero.
    #[garde(custom(bound_rule))]
    reference_roundoff: f64,
    /// Positive normal arithmetic-bound power of two, or canonical positive zero.
    #[garde(custom(bound_rule))]
    phase_roundoff: f64,
    /// Finite whole-window dimensionless divisor at least one, without a cap.
    #[garde(custom(divisor_rule))]
    divisor: f64,
}

/// Decide normalized reference postconditions independently of physical uncertainty.
fn valid_normalized_reference(value: f64) -> bool {
    normal_or_positive_zero(value) && (-1.0..=0.0).contains(&value)
}
/// Decide normalized phase postconditions without clamping a rounded coordinate.
fn valid_normalized_phase(value: f64) -> bool {
    normal_or_positive_zero(value) && (-1.0..=1.0).contains(&value)
}
/// Decide scaled bound postconditions, preserving exact positive binary powers.
fn valid_bound(value: f64) -> bool {
    value.to_bits() == 0
        || (value.is_normal() && value > 0.0 && value.to_bits() & ((1_u64 << 52) - 1) == 0)
}
/// Decide uncapped divisor postconditions without categorically rejecting tiny reciprocals.
fn valid_divisor(value: f64) -> bool {
    value.is_finite() && value >= 1.0
}

/// Adapt normalized-reference postconditions to mandatory Garde publication.
/// # Errors
/// Reject nonnormal/noncanonical-zero or out-of-range coordinates with fixed text.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow transformed fields"
)]
fn normalized_reference_rule(value: &f64, (): &()) -> garde::Result {
    if valid_normalized_reference(*value) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid normalized reference"))
    }
}
/// Adapt normalized-phase postconditions to mandatory Garde publication.
/// # Errors
/// Reject nonnormal/noncanonical-zero or out-of-range phases with fixed text.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow transformed fields"
)]
fn normalized_phase_rule(value: &f64, (): &()) -> garde::Result {
    if valid_normalized_phase(*value) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid normalized phase"))
    }
}
/// Adapt exact binary-power arithmetic-bound postconditions to mandatory Garde.
/// # Errors
/// Reject malformed, negative, nonnormal or noncanonical-zero bounds.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow transformed fields"
)]
fn bound_rule(value: &f64, (): &()) -> garde::Result {
    if valid_bound(*value) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid normalized bound"))
    }
}
/// Adapt unchanged uncapped divisor postconditions to mandatory Garde.
/// # Errors
/// Reject nonfinite divisors or divisors smaller than one.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow transformed fields"
)]
fn divisor_rule(value: &f64, (): &()) -> garde::Result {
    if valid_divisor(*value) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid associated divisor"))
    }
}

/// Publish transformed scalars only through their mandatory shared output rules.
///
/// Exact deltas are immutable checked converter values; the caller owns their
/// original-row/anchor relationship and exact scaling. No raw scalar is trusted.
/// # Errors
/// Reject malformed transformed outputs without publishing a point or changing input.
fn publish_point(
    row: &ExactObservationRow,
    reference_delta: RoundedRationalSeconds,
    phase_delta: RoundedRationalSeconds,
    candidate: PointCandidate,
) -> Result<PreparedConditionedPoint<'_>, ConditionedWindowError> {
    candidate.validate().map_err(|_| {
        if !valid_normalized_reference(candidate.reference)
            || !valid_normalized_phase(candidate.phase)
            || !valid_bound(candidate.reference_roundoff)
            || !valid_bound(candidate.phase_roundoff)
            || !valid_divisor(candidate.divisor)
        {
            ConditionedWindowError::OutputInvariant
        } else {
            ConditionedWindowError::ValidationInvariant
        }
    })?;
    Ok(PreparedConditionedPoint {
        original_row: row,
        reference_delta,
        phase_delta,
        normalized_reference: candidate.reference,
        normalized_phase: candidate.phase,
        normalized_reference_roundoff: candidate.reference_roundoff,
        normalized_phase_roundoff: candidate.phase_roundoff,
        variance_divisor: candidate.divisor,
    })
}

/// Convert exact signed differences once and prepare one original associated row.
///
/// Both rows and exponents have completed whole-population admission. References
/// use exact widened half ticks; phase subtraction supports both i128 extrema.
/// # Errors
/// Preserve indexed typed conversion failures or fixed scaling/output failures.
fn prepare_point<'a>(
    row: &'a ExactObservationRow,
    anchor: &ExactObservationRow,
    index: usize,
    scales: &ScaleCandidate,
    divisor: f64,
) -> Result<PreparedConditionedPoint<'a>, ConditionedWindowError> {
    let reference = reference_ticks(row);
    let anchor_reference = reference_ticks(anchor);
    let reference_sign = if reference < anchor_reference {
        RationalSign::Negative
    } else {
        RationalSign::Positive
    };
    let reference_delta = RoundedRationalSeconds::new(
        reference_sign,
        reference.abs_diff(anchor_reference),
        denominator(REFERENCE_DENOMINATOR)?,
    )
    .map_err(|cause| ConditionedWindowError::Conversion {
        index,
        coordinate: ConditionedCoordinate::Reference,
        cause,
    })?;
    let phase_sign = match row
        .corrected_phase_numerator
        .cmp(&anchor.corrected_phase_numerator)
    {
        Ordering::Less => RationalSign::Negative,
        Ordering::Equal | Ordering::Greater => RationalSign::Positive,
    };
    let phase_delta = RoundedRationalSeconds::new(
        phase_sign,
        row.corrected_phase_numerator
            .abs_diff(anchor.corrected_phase_numerator),
        denominator(PHASE_DENOMINATOR)?,
    )
    .map_err(|cause| ConditionedWindowError::Conversion {
        index,
        coordinate: ConditionedCoordinate::Phase,
        cause,
    })?;
    let reference_scale = scales
        .time
        .checked_neg()
        .ok_or(ConditionedWindowError::OutputInvariant)?;
    let phase_scale = scales
        .phase
        .checked_neg()
        .ok_or(ConditionedWindowError::OutputInvariant)?;
    let candidate = PointCandidate {
        reference: exact_scale(reference_delta.seconds(), reference_scale)?,
        phase: exact_scale(phase_delta.seconds(), phase_scale)?,
        reference_roundoff: exact_scale(
            reference_delta.absolute_roundoff_seconds(),
            reference_scale,
        )?,
        phase_roundoff: exact_scale(phase_delta.absolute_roundoff_seconds(), phase_scale)?,
        divisor,
    };
    publish_point(row, reference_delta, phase_delta, candidate)
}

/// Prepare one declared original population without fitting or altering its rows.
///
/// Count (0..=64) precedes positive-finite precision, all contexts, all duplicate
/// keys and exact strict reference order. One whole-population quality derivation
/// precedes coordinate conversion. Empty input skips derivation; counts below
/// three return only admitted insufficient metadata. Ready preparation retains
/// newest exact anchors, once-rounded deltas and bounds, integer-selected scales
/// and unchanged divisors. Rounded reference collisions refuse the whole batch.
/// Successful work uses fixed storage with no heap allocation, I/O, ambient reads,
/// logging, fitting, history transition or caller mutation.
/// # Errors
/// Return the first failure in the stated admission order, quality arithmetic,
/// original-order conversion, scale/output contradiction or rounded collision.
/// Rejection preserves all original bits and publishes no partial payload.
///
/// ```
/// use std::num::NonZeroU64;
/// use time_core::{ConditionedBatchOutcome, DeclaredObservationContext, ExactObservationRow, prepare_conditioned_batch};
/// let tag = NonZeroU64::MIN;
/// let context = DeclaredObservationContext::new(tag, tag, tag, tag, tag);
/// let rows = [
///     ExactObservationRow::new(context, NonZeroU64::new(1).ok_or("key")?, 0, false, 0, 1.0)?,
///     ExactObservationRow::new(context, NonZeroU64::new(2).ok_or("key")?, 1, false, 0, 1.0)?,
///     ExactObservationRow::new(context, NonZeroU64::new(3).ok_or("key")?, 2, false, 0, 1.0)?,
/// ];
/// let ConditionedBatchOutcome::Prepared(batch) = prepare_conditioned_batch(context, &rows, 0.25)? else { return Err("insufficient".into()); };
/// assert_eq!(batch.sample_count(), 3);
/// assert!(std::ptr::eq(batch.newest_row(), &rows[2]));
/// assert_eq!(batch.point(2).ok_or("point")?.normalized_reference().to_bits(), 0);
/// assert!(batch.point(usize::MAX).is_none());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// The original rows stay borrowed until the outcome's last use:
/// ```compile_fail,E0506
/// use time_core::{DeclaredObservationContext, ExactObservationRow, prepare_conditioned_batch};
/// fn change(rows: &mut [ExactObservationRow; 3], context: DeclaredObservationContext) {
///     let outcome = prepare_conditioned_batch(context, rows, 1.0);
///     rows[0] = rows[1];
///     let _ = outcome.is_ok();
/// }
/// ```
///
/// A borrowed population cannot escape its actual row lifetime:
/// ```compile_fail
/// use time_core::{ConditionedBatchOutcome, ConditionedWindowError, DeclaredObservationContext, ExactObservationRow, prepare_conditioned_batch};
/// fn escape(rows: &[ExactObservationRow], context: DeclaredObservationContext) -> Result<ConditionedBatchOutcome<'static>, ConditionedWindowError> {
///     prepare_conditioned_batch(context, rows, 1.0)
/// }
/// ```
pub fn prepare_conditioned_batch(
    expected_context: DeclaredObservationContext,
    rows: &[ExactObservationRow],
    current_precision_seconds: f64,
) -> Result<ConditionedBatchOutcome<'_>, ConditionedWindowError> {
    CountCandidate { count: rows.len() }
        .validate()
        .map_err(|_| {
            if valid_count(rows.len()) {
                ConditionedWindowError::ValidationInvariant
            } else {
                ConditionedWindowError::CountOutOfRange { count: rows.len() }
            }
        })?;
    PrecisionCandidate {
        seconds: current_precision_seconds,
    }
    .validate()
    .map_err(|_| {
        if positive_finite_seconds(current_precision_seconds) {
            ConditionedWindowError::ValidationInvariant
        } else {
            ConditionedWindowError::InvalidPrecision
        }
    })?;
    MembershipCandidate { rows }
        .validate_with(&expected_context)
        .map_err(|_| {
            membership_error(rows, expected_context)
                .unwrap_or(ConditionedWindowError::ValidationInvariant)
        })?;
    let Some(anchor) = rows.last() else {
        return Ok(ConditionedBatchOutcome::Insufficient(
            InsufficientConditionedBatch {
                declared_context: expected_context,
                original_rows: rows,
                current_precision_seconds,
                divisors: None,
            },
        ));
    };
    let divisors = derive_divisors(rows, current_precision_seconds)?;
    if rows.len() < 3 {
        return Ok(ConditionedBatchOutcome::Insufficient(
            InsufficientConditionedBatch {
                declared_context: expected_context,
                original_rows: rows,
                current_precision_seconds,
                divisors: Some(divisors),
            },
        ));
    }
    let first = rows
        .first()
        .ok_or(ConditionedWindowError::ArithmeticInvariant)?;
    let reference_span = reference_ticks(anchor)
        .checked_sub(reference_ticks(first))
        .ok_or(ConditionedWindowError::ArithmeticInvariant)?;
    let largest_phase = rows
        .iter()
        .map(|row| {
            row.corrected_phase_numerator
                .abs_diff(anchor.corrected_phase_numerator)
        })
        .max()
        .ok_or(ConditionedWindowError::ArithmeticInvariant)?;
    let scales = admit_scales(ScaleCandidate {
        time: ceiling_exponent(reference_span, denominator(REFERENCE_DENOMINATOR)?)?,
        phase: ceiling_exponent(largest_phase, denominator(PHASE_DENOMINATOR)?)?,
    })?;
    let first_point = prepare_point(first, anchor, 0, &scales, divisors[0])?;
    let mut points = [first_point; MAX_ROWS];
    let mut previous_reference = first_point.normalized_reference;
    for (index, row) in rows.iter().enumerate().skip(1) {
        let point = prepare_point(row, anchor, index, &scales, divisors[index])?;
        if point.normalized_reference <= previous_reference {
            return Err(ConditionedWindowError::RoundedReferenceCollision { index });
        }
        previous_reference = point.normalized_reference;
        points[index] = point;
    }
    Ok(ConditionedBatchOutcome::Prepared(
        PreparedConditionedBatch {
            declared_context: expected_context,
            original_rows: rows,
            current_precision_seconds,
            newest_row: anchor,
            time_scale_exponent: scales.time,
            phase_scale_exponent: scales.phase,
            points,
        },
    ))
}

/// Internal raw-output and exact-arithmetic controls remain outside the source owner.
#[cfg(test)]
#[path = "../tests/private/conditioned_window.rs"]
mod tests;

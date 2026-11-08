// SPDX-License-Identifier: GPL-2.0-only
//! Exact same-origin midpoint arithmetic and anchoring within one transport run.
//!
//! The transport owner retains the real origin and distinct request identities.
//! This module borrows terminal facts, checks their shared run and conditional
//! measurements, and converts exact differences once. It reads no clock, changes
//! no observation age or owner state, and supplies no history, physical accuracy,
//! authentication, source agreement or clock authority. Successful work uses
//! bounded scalar storage; rejected Garde admission may allocate a bounded report.
#![forbid(unsafe_code)]

use std::{
    cmp::Ordering,
    error::Error,
    fmt,
    num::NonZeroU64,
    sync::Arc,
    time::{Duration, Instant},
};

use garde::Validate;
use time_domain::{RationalSign, RoundedRationalSeconds};

use super::{MeasurementRefusal, RunClockContext, TransportFacts};

/// Exact seconds denominator for corrected phase numerators in half-Q32 nanoseconds.
const PHASE_SECONDS_DENOMINATOR: u64 = 8_589_934_592_000_000_000;
/// Exact seconds denominator for a reference represented in half nanoseconds.
const REFERENCE_SECONDS_DENOMINATOR: u64 = 2_000_000_000;

/// Fixed, payload-free reasons that an experimental relative view is unavailable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelativeMeasurementRefusal {
    /// The two terminal facts belong to different actual run-context owners.
    DifferentRun,
    /// Preserve the row's original bounded conditional-measurement refusal.
    RowMeasurement(MeasurementRefusal),
    /// Preserve the anchor's original bounded conditional-measurement refusal.
    AnchorMeasurement(MeasurementRefusal),
    /// At least one selected capture precedes the retained actual run origin.
    ReferenceBeforeOrigin,
    /// The selected read precedes its send, or raw reference endpoints are reversed.
    ReferenceOrder,
    /// A same-origin duration cannot be represented as unsigned 64-bit nanoseconds.
    ReferenceRange,
    /// A checked exact operation or the shared seconds conversion is unavailable.
    Arithmetic,
    /// Mandatory admission rejected a transformed reference representation.
    OutputInvariant,
}

impl fmt::Display for RelativeMeasurementRefusal {
    /// Write a fixed operation description without coordinates, endpoints or reports.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::DifferentRun => "relative measurement run mismatch",
            Self::RowMeasurement(_) => "relative row measurement unavailable",
            Self::AnchorMeasurement(_) => "relative anchor measurement unavailable",
            Self::ReferenceBeforeOrigin => "relative reference before origin",
            Self::ReferenceOrder => "relative reference order",
            Self::ReferenceRange => "relative reference range",
            Self::Arithmetic => "relative measurement arithmetic",
            Self::OutputInvariant => "relative reference output invariant",
        })
    }
}

impl Error for RelativeMeasurementRefusal {
    /// Borrow only an original fixed measurement cause; no validation report escapes.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RowMeasurement(reason) | Self::AnchorMeasurement(reason) => Some(reason),
            Self::DifferentRun
            | Self::ReferenceBeforeOrigin
            | Self::ReferenceOrder
            | Self::ReferenceRange
            | Self::Arithmetic
            | Self::OutputInvariant => None,
        }
    }
}

/// Exact nominal midpoint in nanoseconds from its retained experimental chart origin.
///
/// The value retains whole nanoseconds and, when its half bit is set, one further
/// half nanosecond.
/// Only the private checked midpoint owner constructs this representation. The
/// containing relative or acquisition view retains its actual capture and chart association;
/// this scalar alone supplies no context-free equality, ordering or provenance.
///
/// A legal consumer can read both exact parts from a checked view:
/// ```
/// use nevrotick_engine::ExperimentalRelativeMeasurement;
/// fn parts(view: &ExperimentalRelativeMeasurement<'_>) -> (u64, bool) {
///     let reference = view.row_reference();
///     (reference.whole_nanoseconds(), reference.has_half_nanosecond())
/// }
/// let _read = parts;
/// ```
/// Raw fields cannot bypass midpoint admission:
/// ```compile_fail,E0451
/// use nevrotick_engine::ExperimentalReferenceNs;
/// let _ = ExperimentalReferenceNs { whole_ns: 0, half: false };
/// ```
/// An existing reference cannot be mutated:
/// ```compile_fail,E0616
/// use nevrotick_engine::ExperimentalReferenceNs;
/// let _ = |mut value: ExperimentalReferenceNs| { value.half = false; };
/// ```
/// Missing references have no default substitute:
/// ```compile_fail,E0599
/// use nevrotick_engine::ExperimentalReferenceNs;
/// let _ = ExperimentalReferenceNs::default();
/// ```
/// A primitive cannot manufacture an unchecked reference:
/// ```compile_fail,E0277
/// use nevrotick_engine::ExperimentalReferenceNs;
/// let _: ExperimentalReferenceNs = 0_u64.into();
/// ```
#[derive(Clone, Copy)]
pub struct ExperimentalReferenceNs {
    /// Integral nanoseconds of the checked midpoint, bounded by its original endpoints.
    whole_ns: u64,
    /// Whether that exact midpoint has one additional half nanosecond.
    half: bool,
}

impl fmt::Debug for ExperimentalReferenceNs {
    /// Expose the type without formatting chart-derived coordinates.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ExperimentalReferenceNs { .. }")
    }
}

impl ExperimentalReferenceNs {
    /// Return the exact integral midpoint component, without rounding or effects.
    #[must_use]
    pub const fn whole_nanoseconds(self) -> u64 {
        self.whole_ns
    }

    /// Return the retained half-nanosecond component, without inventing finer precision.
    #[must_use]
    pub const fn has_half_nanosecond(self) -> bool {
        self.half
    }
}

/// Immutable self-minus-anchor differences retaining both actual terminal facts.
///
/// The view borrows a single actual run chart without retaining native resources.
/// Both original profiles, endpoints, request identities, captures and ages remain
/// available through those facts. Rounded values include arithmetic-only bounds;
/// this view establishes no usable source history, current freshness or authority.
///
/// A legal consumer borrows the checked result and its original facts:
/// ```
/// use nevrotick_engine::{ExperimentalRelativeMeasurement, TransportFacts};
/// fn originals<'a>(view: &ExperimentalRelativeMeasurement<'a>)
///     -> (&'a TransportFacts, &'a TransportFacts)
/// {
///     let _phase = view.phase_delta_seconds();
///     let _reference = view.reference_delta_seconds();
///     (view.row(), view.anchor())
/// }
/// let _read = originals;
/// ```
/// Getters do not permit unchecked construction from existing facts:
/// ```compile_fail,E0451
/// use nevrotick_engine::ExperimentalRelativeMeasurement;
/// fn bypass<'a>(value: ExperimentalRelativeMeasurement<'a>)
///     -> ExperimentalRelativeMeasurement<'a>
/// {
///     ExperimentalRelativeMeasurement {
///         row: value.row(),
///         anchor: value.anchor(),
///         row_reference: value.row_reference(),
///         anchor_reference: value.anchor_reference(),
///         phase_delta_seconds: value.phase_delta_seconds(),
///         reference_delta_seconds: value.reference_delta_seconds(),
///     }
/// }
/// ```
/// Published associations cannot be mutated:
/// ```compile_fail,E0616
/// use nevrotick_engine::ExperimentalRelativeMeasurement;
/// let _ = |mut value: ExperimentalRelativeMeasurement<'_>| {
///     value.row_reference = value.anchor_reference();
/// };
/// ```
/// No default substitutes for missing observations:
/// ```compile_fail,E0599
/// use nevrotick_engine::ExperimentalRelativeMeasurement;
/// let _ = ExperimentalRelativeMeasurement::default();
/// ```
/// A fact cannot supply an infallible conversion:
/// ```compile_fail,E0277
/// use nevrotick_engine::{ExperimentalRelativeMeasurement, TransportFacts};
/// fn bypass<'a>(row: &'a TransportFacts) -> ExperimentalRelativeMeasurement<'a> {
///     row.into()
/// }
/// ```
/// The result cannot outlive its borrowed original observations:
/// ```compile_fail
/// use nevrotick_engine::{ExperimentalRelativeMeasurement, RelativeMeasurementRefusal, TransportFacts};
/// fn escape(row: &TransportFacts)
///     -> Result<ExperimentalRelativeMeasurement<'static>, RelativeMeasurementRefusal>
/// {
///     row.relative_to(row)
/// }
/// ```
pub struct ExperimentalRelativeMeasurement<'a> {
    /// Original immutable row, including its acquisition time and request owner.
    row: &'a TransportFacts,
    /// Original immutable anchor, retained without merging its logical source.
    anchor: &'a TransportFacts,
    /// Exact midpoint of this row's selected captures from the common origin.
    row_reference: ExperimentalReferenceNs,
    /// Exact midpoint of the anchor's selected captures from that same origin.
    anchor_reference: ExperimentalReferenceNs,
    /// Exact corrected phase difference, converted once to seconds with its bound.
    phase_delta_seconds: RoundedRationalSeconds,
    /// Exact half-nanosecond reference difference, converted once to seconds.
    reference_delta_seconds: RoundedRationalSeconds,
}

impl fmt::Debug for ExperimentalRelativeMeasurement<'_> {
    /// Expose the type without formatting either original fact or any numeric value.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ExperimentalRelativeMeasurement { .. }")
    }
}

impl<'a> ExperimentalRelativeMeasurement<'a> {
    /// Stage one checked view from original facts in the same actual run.
    ///
    /// The caller borrows terminal facts; this owner checks actual context identity,
    /// both original measurement results and the chronology of the selected lower
    /// captures. All exact and floating transformations finish before publication.
    /// Success allocates nothing and reads no clock; refusal preserves every fact,
    /// age, original refusal and owning transport state.
    /// # Errors
    /// Preserve run, original measurement, chronology, range or conversion refusal.
    pub(super) fn new(
        row: &'a TransportFacts,
        anchor: &'a TransportFacts,
    ) -> Result<Self, RelativeMeasurementRefusal> {
        let origin = shared_origin(&row.run_context, &anchor.run_context)?;
        let row_measurement = row
            .measurement()
            .map_err(RelativeMeasurementRefusal::RowMeasurement)?;
        let anchor_measurement = anchor
            .measurement()
            .map_err(RelativeMeasurementRefusal::AnchorMeasurement)?;
        let row_reference = reference_from_instants(
            origin,
            row.send_before().monotonic(),
            row.read_return().monotonic(),
        )?;
        let anchor_reference = reference_from_instants(
            origin,
            anchor.send_before().monotonic(),
            anchor.read_return().monotonic(),
        )?;
        let phase_delta_seconds = phase_difference(
            row_measurement.nominal_phase().numerator(),
            anchor_measurement.nominal_phase().numerator(),
        )?;
        let reference_delta_seconds = reference_difference(row_reference, anchor_reference)?;
        Ok(Self {
            row,
            anchor,
            row_reference,
            anchor_reference,
            phase_delta_seconds,
            reference_delta_seconds,
        })
    }

    /// Borrow the unchanged original row with its original lifetime and identity.
    #[must_use]
    pub const fn row(&self) -> &'a TransportFacts {
        self.row
    }

    /// Borrow the unchanged anchor without renewing its acquisition age.
    #[must_use]
    pub const fn anchor(&self) -> &'a TransportFacts {
        self.anchor
    }

    /// Return this row's exact common-origin midpoint, without another conversion.
    #[must_use]
    pub const fn row_reference(&self) -> ExperimentalReferenceNs {
        self.row_reference
    }

    /// Return the anchor's exact common-origin midpoint, preserving its half bit.
    #[must_use]
    pub const fn anchor_reference(&self) -> ExperimentalReferenceNs {
        self.anchor_reference
    }

    /// Return self-minus-anchor nominal phase in seconds with arithmetic roundoff only.
    #[must_use]
    pub const fn phase_delta_seconds(&self) -> RoundedRationalSeconds {
        self.phase_delta_seconds
    }

    /// Return self-minus-anchor reference time in seconds with arithmetic roundoff only.
    #[must_use]
    pub const fn reference_delta_seconds(&self) -> RoundedRationalSeconds {
        self.reference_delta_seconds
    }
}

/// Return the retained origin only for the same actual context allocation.
///
/// Equal numeric origins do not establish a common run. Borrowing these private
/// owners allocates nothing and retains no additional native resource or task.
/// # Errors
/// Reject distinct context allocations, including equal-valued origins.
fn shared_origin(
    row: &Arc<RunClockContext>,
    anchor: &Arc<RunClockContext>,
) -> Result<Instant, RelativeMeasurementRefusal> {
    if !Arc::ptr_eq(row, anchor) {
        return Err(RelativeMeasurementRefusal::DifferentRun);
    }
    Ok(row.origin)
}

/// Raw transformed midpoint parts; all four fields retain their full `u64` domain.
struct MidpointParts {
    /// Inclusive first endpoint in nanoseconds from the already checked origin.
    lower: u64,
    /// Inclusive last endpoint; the shared rule rejects values below `lower`.
    upper: u64,
    /// Proposed integral midpoint component, checked against both original endpoints.
    whole: u64,
    /// Proposed half bit; the shared rule admits only zero or one.
    half: u64,
}

/// Mandatory shared admission covers every raw part and their exact association.
#[derive(Validate)]
struct MidpointCandidate {
    /// One maintained rule checks endpoints, half shape and the doubled-sum relation.
    #[garde(custom(midpoint_rule))]
    parts: MidpointParts,
}

/// Admit exactly an ordered endpoint pair and its integral/half midpoint.
///
/// All raw fields are `u64`: their endpoint sum is below `2^65`, and even the
/// unchecked proposed `2*whole+half` is below `2^66`. Widening first to `u128`
/// makes both calculations exact across the full raw domain, without overflow.
/// The rule is fixed work with one fixed rejected-message allocation at most.
/// # Errors
/// Reject reversed endpoints, a half value above one or any midpoint disagreement.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rules require borrowed values and context"
)]
fn midpoint_rule(parts: &MidpointParts, (): &()) -> garde::Result {
    let expected = u128::from(parts.lower) + u128::from(parts.upper);
    let proposed = u128::from(parts.whole) * 2 + u128::from(parts.half);
    if parts.lower <= parts.upper && parts.half <= 1 && proposed == expected {
        Ok(())
    } else {
        Err(garde::Error::new("invalid experimental midpoint"))
    }
}

/// Publish only raw midpoint parts admitted by the shared mandatory Garde rule.
///
/// This is the only reference constructor. A valid half becomes a bool only after
/// full association admission; no partial reference or report escapes. Success
/// allocates nothing; rejection may allocate and drops one bounded Garde report.
/// # Errors
/// Return a fixed output-invariant refusal for every rejected raw part combination.
fn publish_reference(
    parts: MidpointParts,
) -> Result<ExperimentalReferenceNs, RelativeMeasurementRefusal> {
    let candidate = MidpointCandidate { parts };
    candidate
        .validate()
        .map_err(|_| RelativeMeasurementRefusal::OutputInvariant)?;
    Ok(ExperimentalReferenceNs {
        whole_ns: candidate.parts.whole,
        half: candidate.parts.half == 1,
    })
}

/// Retain the exact midpoint of every ordered raw pair of `u64` nanosecond offsets.
///
/// Subtract before halving, then add to the lower endpoint: the calculation never
/// sums endpoints in `u64` or discards an odd half nanosecond. The actual shared
/// publisher checks the complete transformed association before returning it.
/// # Errors
/// Reject reversed offsets or a checked arithmetic/publication disagreement.
fn reference_from_bounds(
    lower: u64,
    upper: u64,
) -> Result<ExperimentalReferenceNs, RelativeMeasurementRefusal> {
    let width = upper
        .checked_sub(lower)
        .ok_or(RelativeMeasurementRefusal::ReferenceOrder)?;
    let whole = lower
        .checked_add(width / 2)
        .ok_or(RelativeMeasurementRefusal::Arithmetic)?;
    publish_reference(MidpointParts {
        lower,
        upper,
        whole,
        half: width % 2,
    })
}

/// Narrow an entire nonnegative duration to exact unsigned nanoseconds.
///
/// All `Duration` values are accepted at this raw boundary; the output requires
/// at most `u64::MAX` nanoseconds. No time lookup, allocation or state effect occurs.
/// # Errors
/// Reject the first and every later unrepresentable nanosecond value.
fn duration_nanoseconds(duration: Duration) -> Result<u64, RelativeMeasurementRefusal> {
    u64::try_from(duration.as_nanos()).map_err(|_| RelativeMeasurementRefusal::ReferenceRange)
}

/// Place actual selected lower captures in their already associated immutable chart.
///
/// The calling owner establishes the actual run or session association; arbitrary
/// matching labels or lifetimes cannot establish provenance. This helper checks
/// chronology and exact representation only, never current authority.
/// Every raw `Instant` triple is accepted for checking. Explicit ordering rejects
/// pre-origin and reversed values before duration subtraction, including Windows'
/// small-negative-to-zero interval tolerance. Both checked durations are narrowed
/// before midpoint construction. No ambient clock is read and no age is renewed.
/// # Errors
/// Preserve before-origin, order, range, arithmetic or midpoint-admission refusal.
pub(super) fn reference_from_instants(
    origin: Instant,
    send: Instant,
    read: Instant,
) -> Result<ExperimentalReferenceNs, RelativeMeasurementRefusal> {
    if send < origin || read < origin {
        return Err(RelativeMeasurementRefusal::ReferenceBeforeOrigin);
    }
    if read < send {
        return Err(RelativeMeasurementRefusal::ReferenceOrder);
    }
    let lower = send
        .checked_duration_since(origin)
        .ok_or(RelativeMeasurementRefusal::ReferenceBeforeOrigin)?;
    let upper = read
        .checked_duration_since(origin)
        .ok_or(RelativeMeasurementRefusal::ReferenceBeforeOrigin)?;
    reference_from_bounds(duration_nanoseconds(lower)?, duration_nanoseconds(upper)?)
}

/// Convert an entire raw signed-magnitude seconds ratio through its checked owner.
///
/// All sign/magnitude/denominator values are checked here. The shared converter
/// canonicalizes zero and validates the once-rounded seconds and arithmetic bound.
/// No numeric payload or report is retained by the fixed error mapping.
/// # Errors
/// Reject a zero denominator or preserve a shared checked-conversion failure.
fn convert_seconds(
    sign: RationalSign,
    magnitude: u128,
    denominator: u64,
) -> Result<RoundedRationalSeconds, RelativeMeasurementRefusal> {
    let denominator = NonZeroU64::new(denominator).ok_or(RelativeMeasurementRefusal::Arithmetic)?;
    RoundedRationalSeconds::new(sign, magnitude, denominator)
        .map_err(|_| RelativeMeasurementRefusal::Arithmetic)
}

/// Subtract exact corrected phase numerators before their single seconds conversion.
///
/// The raw domain is the complete pair of `i128` values, not only today's capture
/// envelope. Unsigned absolute difference preserves even `i128::MIN` to
/// `i128::MAX`; equality explicitly selects canonical positive zero. Positive
/// means the row's nominal phase exceeds its anchor's. Work has no external effect.
/// # Errors
/// Preserve an unavailable shared checked seconds conversion without partial output.
pub(super) fn phase_difference(
    row: i128,
    anchor: i128,
) -> Result<RoundedRationalSeconds, RelativeMeasurementRefusal> {
    let (sign, magnitude) = match row.cmp(&anchor) {
        Ordering::Less => (RationalSign::Negative, row.abs_diff(anchor)),
        Ordering::Equal => (RationalSign::Positive, 0),
        Ordering::Greater => (RationalSign::Positive, row.abs_diff(anchor)),
    };
    convert_seconds(sign, magnitude, PHASE_SECONDS_DENOMINATOR)
}

/// Encode every checked reference exactly as an unsigned half-nanosecond count.
///
/// A `u64` whole and boolean half fit below `2^65` after widening to `u128`.
/// The private checked reference owns the midpoint invariant. This numeric
/// encoding grants no identity or current clock-mapping claim and has no effects.
fn reference_ticks(value: ExperimentalReferenceNs) -> u128 {
    u128::from(value.whole_ns) * 2 + u128::from(value.half)
}

/// Subtract checked common-chart reference values exactly before converting seconds.
///
/// The enclosing constructor already matched actual run identity; this pure
/// numeric helper accepts the complete checked-reference domain without reading
/// clocks or inferring identity from numbers. Equality explicitly produces positive
/// zero; positive means this row's reference is later than its anchor's.
/// # Errors
/// Preserve an unavailable shared checked seconds conversion without partial output.
fn reference_difference(
    row: ExperimentalReferenceNs,
    anchor: ExperimentalReferenceNs,
) -> Result<RoundedRationalSeconds, RelativeMeasurementRefusal> {
    let row = reference_ticks(row);
    let anchor = reference_ticks(anchor);
    let (sign, magnitude) = match row.cmp(&anchor) {
        Ordering::Less => (RationalSign::Negative, row.abs_diff(anchor)),
        Ordering::Equal => (RationalSign::Positive, 0),
        Ordering::Greater => (RationalSign::Positive, row.abs_diff(anchor)),
    };
    convert_seconds(sign, magnitude, REFERENCE_SECONDS_DENOMINATOR)
}

/// Independent raw-boundary controls live outside the production coverage owner.
#[cfg(test)]
#[path = "../tests/private/relative.rs"]
mod tests;

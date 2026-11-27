// SPDX-License-Identifier: GPL-2.0-only
//! Conditional common-instant interval admission for controlled Roughtime.
//!
//! A positive result retains the original observations and declared
//! assumptions. Live catalogue, clock-frame and UTC-scale qualification
//! remain with their owners before any dependent NTS use.

use super::{
    exchange::{Receipt, Withdrawal},
    round::{Conflict, EXCHANGES, PEERS},
};
use crate::measurement::NTP_UNIX_OFFSET_SECONDS;
use std::{
    num::{NonZeroU32, NonZeroU64},
    time::{Duration, Instant},
};
use time_domain::{EraWindow, LeapDateScope, LeapInterpretation, LeapKnowledge};

/// Denominator of the declared UTC-nanoseconds-per-monotonic-nanosecond rate.
const RATE_DENOMINATOR: u128 = 1_000_000_000;
/// Exact interval units per whole Unix second.
const SCALED_SECOND: u128 = RATE_DENOMINATOR * 1_000_000_000;
/// First whole second outside the unsigned Roughtime wire date envelope.
const UNIX_NS_LIMIT: u128 = (1_u128 << 64) * 1_000_000_000;

/// Immutable caller-declared limits for the controlled receiving consumer.
#[derive(Clone, Copy)]
pub struct IntervalPolicy {
    /// Ordered positive UTC nanoseconds per billion monotonic nanoseconds.
    rates: [NonZeroU32; 2],
    /// Additional uncertainty applied outward at both projection endpoints.
    projection_margin: Duration,
    /// Exclusive age since the oldest possible monotonic processing instant.
    max_age: Duration,
    /// Inclusive maximum outward width of each individual projected interval.
    max_width: Duration,
}

impl IntervalPolicy {
    /// Admit explicit monotonic-age and Unix-interval limits with no defaults.
    ///
    /// Rates bound UTC elapsed time per measured monotonic elapsed time.
    /// The projection margin covers additional declared UTC endpoint error.
    ///
    /// # Errors
    /// Refuses reversed rates or zero age or width limits.
    pub fn new(
        rates: [NonZeroU32; 2],
        projection_margin: Duration,
        max_age: Duration,
        max_width: Duration,
    ) -> Result<Self, IntervalError> {
        if rates[0] > rates[1] || max_age.is_zero() || max_width.is_zero() {
            return Err(IntervalError::Configuration);
        }
        Ok(Self {
            rates,
            projection_margin,
            max_age,
            max_width,
        })
    }
}

/// The first reason controlled receipt evidence cannot supply an agreement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum IntervalError {
    /// A declared rate, age or width limit is invalid.
    Configuration,
    /// The original fixed sequence did not complete.
    NotComplete,
    /// The complete round has no remaining bound retry admission.
    RetryAdmission,
    /// The original round retained a signed causal contradiction.
    Conflict(Conflict),
    /// The original round authority was withdrawn.
    Withdrawn(Withdrawal),
    /// The original authority guard could not be obtained.
    AuthorityUnavailable,
    /// The original exclusive round deadline elapsed.
    Expired,
    /// The evaluation does not follow ordered original captures.
    CaptureOrder,
    /// At least one original receipt has reached its exclusive age limit.
    Stale,
    /// The Unix-coordinate or scaling calculation exceeds the numeric envelope.
    Arithmetic,
    /// An observation alone is wider than the declared usefulness bound.
    TooWide,
    /// The visits to one selected group have no positive-width common part.
    GroupDisagreement,
    /// The three declared group intervals have no positive-width common part.
    Disagreement,
    /// The complete outward date enclosure is outside the supplied date scope.
    DateScope,
}

/// A strictly positive open interval with exact endpoints in scaled Unix nanoseconds.
#[derive(Clone, Copy)]
struct ExactInterval {
    /// Lower numerator with the fixed rate denominator.
    lower: u128,
    /// Upper numerator with the fixed rate denominator.
    upper: u128,
}

impl ExactInterval {
    /// Intersect exact open intervals, refusing contact at just one endpoint.
    fn intersect(self, other: Self) -> Option<Self> {
        let lower: u128 = self.lower.max(other.lower);
        let upper: u128 = self.upper.min(other.upper);
        (lower < upper).then_some(Self { lower, upper })
    }

    /// Express the open enclosure in integer Unix nanoseconds, rounding outward.
    fn outward_ns(self) -> (u128, u128) {
        (
            self.lower / RATE_DENOMINATOR,
            self.upper.div_ceil(RATE_DENOMINATOR),
        )
    }

    /// Enclose the exact Unix interval in inclusive unfolded NTP coordinates.
    ///
    /// # Errors
    /// Refuses an endpoint outside the signed Q32 coordinate envelope.
    fn ntp_window(self) -> Result<EraWindow, IntervalError> {
        let (lower, _): (i128, i128) = ntp_endpoint(self.lower)?;
        let (_, upper): (i128, i128) = ntp_endpoint(self.upper)?;
        EraWindow::new(lower, upper).map_err(|_| IntervalError::Arithmetic)
    }
}

/// Return the two adjacent Q32 enclosures of one exact scaled Unix endpoint.
///
/// # Errors
/// Refuses a value that cannot be converted into the arithmetic representation.
fn ntp_endpoint(scaled: u128) -> Result<(i128, i128), IntervalError> {
    let unix_seconds: u64 =
        u64::try_from(scaled / SCALED_SECOND).map_err(|_| IntervalError::Arithmetic)?;
    // A wire second and the epoch offset fit within 65 bits before Q32 scaling.
    let whole_ticks: i128 = (i128::from(unix_seconds) + NTP_UNIX_OFFSET_SECONDS) << 32;
    // The fractional numerator is below 2^92, regardless of the date.
    let fraction: u128 = (scaled % SCALED_SECOND) << 32;
    let floor: i128 =
        i128::try_from(fraction / SCALED_SECOND).map_err(|_| IntervalError::Arithmetic)?;
    let ceil: i128 =
        i128::try_from(fraction.div_ceil(SCALED_SECOND)).map_err(|_| IntervalError::Arithmetic)?;
    Ok((whole_ticks + floor, whole_ticks + ceil))
}

/// One conditional agreement at the original round owner's evaluation instant.
///
/// The borrow retains the actual nine authenticated observations. Its three
/// groups describe the controlled roster and still require live independent
/// provenance before entering recovery.
#[must_use = "retain the evaluated bounds with their exact source receipts"]
pub struct ControlledAgreement<'a> {
    /// Exact underlying intersection, kept until an endpoint projection is requested.
    interval: ExactInterval,
    /// One monotonic sample under the original round's guard.
    evaluated_at: Instant,
    /// Original exclusive cutoff, never renewed by the evaluation.
    original_cutoff: Instant,
    /// Full original receipt sequence in measurement order.
    receipts: &'a [Receipt],
    /// Declared operator and upstream-clock group IDs in roster order.
    groups: [(NonZeroU64, NonZeroU64); PEERS],
    /// Exact declared projection rules used at this evaluation instant.
    policy: IntervalPolicy,
}

impl ControlledAgreement<'_> {
    /// Return outward-rounded open interval endpoints in Unix nanoseconds.
    #[must_use]
    pub fn bounds_unix_ns(&self) -> (u128, u128) {
        self.interval.outward_ns()
    }

    /// Return the original guarded monotonic instant used by all projections.
    #[must_use]
    pub const fn evaluated_at(&self) -> Instant {
        self.evaluated_at
    }

    /// Return the original round's exclusive deadline.
    #[must_use]
    pub const fn original_cutoff(&self) -> Instant {
        self.original_cutoff
    }

    /// Count each declared supporting group once across the three traversals.
    #[must_use]
    pub const fn supporting_groups(&self) -> usize {
        self.groups.len()
    }

    /// Borrow the declared operator and clock groups associated with the receipts.
    #[must_use]
    pub const fn declared_groups(&self) -> &[(NonZeroU64, NonZeroU64); PEERS] {
        &self.groups
    }

    /// Borrow the projection policy used by this evaluation.
    #[must_use]
    pub const fn policy(&self) -> &IntervalPolicy {
        &self.policy
    }

    /// Borrow the original authenticated receipt at this sequence position.
    #[must_use]
    pub fn receipt(&self, position: usize) -> Option<&Receipt> {
        self.receipts.get(position)
    }
}

/// A complete controlled agreement interpreted under one supplied date scope.
///
/// The calendar answer describes the whole interval under the supplied ledger.
/// Source-scale provenance and current authority for a dependent use still need
/// their own admission. This result supplies no certificate time or clock effect.
///
/// ```compile_fail,E0451
/// use nevrotick_engine::roughtime::interval::{ControlledAgreement, ControlledDate};
/// use time_domain::{LeapDateScope, LeapInterpretation};
/// fn fabricate(agreement: ControlledAgreement<'_>, scope: LeapDateScope,
///              interpretation: LeapInterpretation) -> ControlledDate<'_> {
///     ControlledDate { agreement, scope, interpretation }
/// }
/// ```
#[must_use = "retain the complete date interpretation with its original Roughtime evidence"]
pub struct ControlledDate<'a> {
    /// Original guarded evaluation with exact bounds, policy and source receipts.
    agreement: ControlledAgreement<'a>,
    /// Explicit caller scope that contained the entire outward interval.
    scope: LeapDateScope,
    /// Whole-interval answer with exact Q32 bounds and supplied dataset identity.
    interpretation: LeapInterpretation,
}

impl<'a> ControlledDate<'a> {
    /// Admit the original agreement under the round owner's current guard.
    ///
    /// # Errors
    /// Refuses an unrepresentable or out-of-scope interval.
    pub(super) fn admit(
        agreement: ControlledAgreement<'a>,
        scope: LeapDateScope,
        calendar: &LeapKnowledge<'_>,
    ) -> Result<Self, IntervalError> {
        let dates: EraWindow = agreement.interval.ntp_window()?;
        let interpretation: LeapInterpretation = calendar
            .interpret(dates, scope)
            .map_err(|_| IntervalError::DateScope)?;
        Ok(Self {
            agreement,
            scope,
            interpretation,
        })
    }

    /// Borrow the exact original conditional agreement and its source evidence.
    #[must_use]
    pub const fn agreement(&self) -> &ControlledAgreement<'a> {
        &self.agreement
    }

    /// Return the exact inclusive scope used at admission.
    #[must_use]
    pub const fn scope(&self) -> LeapDateScope {
        self.scope
    }

    /// Return calendar knowledge for the full outward interval and its dataset.
    #[must_use]
    pub const fn interpretation(&self) -> LeapInterpretation {
        self.interpretation
    }
}

/// Evaluate the exact ordered transcript under the caller's original round guard.
///
/// # Errors
/// Refuses incomplete, expired, stale, overwide, incompatible or invalid evidence.
pub(super) fn evaluate<'a>(
    receipts: &'a [Receipt],
    groups: [(NonZeroU64, NonZeroU64); PEERS],
    evaluated_at: Instant,
    original_cutoff: Instant,
    policy: &IntervalPolicy,
) -> Result<ControlledAgreement<'a>, IntervalError> {
    if receipts.len() != EXCHANGES {
        return Err(IntervalError::NotComplete);
    }
    if evaluated_at >= original_cutoff {
        return Err(IntervalError::Expired);
    }
    let mut by_group: [Option<ExactInterval>; PEERS] = [None; PEERS];
    let mut previous_read: Option<Instant> = None;
    for (index, receipt) in receipts.iter().enumerate() {
        let ((sent_before, _), (_, received_after)): ((Instant, Instant), (Instant, Instant)) =
            receipt.captures();
        if previous_read.is_some_and(|previous| sent_before < previous) {
            return Err(IntervalError::CaptureOrder);
        }
        let projected: ExactInterval = project(
            receipt.bounds_seconds(),
            sent_before,
            received_after,
            evaluated_at,
            policy,
        )?;
        previous_read = Some(received_after);
        let slot: &mut Option<ExactInterval> = &mut by_group[index % PEERS];
        *slot = Some(match *slot {
            Some(previous) => previous
                .intersect(projected)
                .ok_or(IntervalError::GroupDisagreement)?,
            None => projected,
        });
    }
    let [Some(first), Some(second), Some(third)]: [Option<ExactInterval>; PEERS] = by_group else {
        return Err(IntervalError::NotComplete);
    };
    let interval: ExactInterval = first
        .intersect(second)
        .and_then(|common| common.intersect(third))
        .ok_or(IntervalError::Disagreement)?;
    Ok(ControlledAgreement {
        interval,
        evaluated_at,
        original_cutoff,
        receipts,
        groups,
        policy: *policy,
    })
}

/// Bound all feasible processing instants by the outer native capture bracket.
///
/// # Errors
/// Refuses invalid or stale captures, arithmetic outside the Unix envelope,
/// unordered bounds, or an observation wider than the policy allows.
fn project(
    bounds: (u64, u64),
    send_before: Instant,
    receive_after: Instant,
    evaluated_at: Instant,
    policy: &IntervalPolicy,
) -> Result<ExactInterval, IntervalError> {
    // Windows checked_duration_since can treat a small reversal as zero.
    if send_before > receive_after || receive_after > evaluated_at {
        return Err(IntervalError::CaptureOrder);
    }
    let youngest: Duration = evaluated_at
        .checked_duration_since(receive_after)
        .ok_or(IntervalError::CaptureOrder)?;
    let oldest: Duration = evaluated_at
        .checked_duration_since(send_before)
        .ok_or(IntervalError::CaptureOrder)?;
    if oldest >= policy.max_age {
        return Err(IntervalError::Stale);
    }
    let margin: u128 = policy
        .projection_margin
        .as_nanos()
        .checked_mul(RATE_DENOMINATOR)
        .ok_or(IntervalError::Arithmetic)?;
    let lower: u128 = projected_endpoint(bounds.0, youngest, policy.rates[0])?
        .checked_sub(margin)
        .ok_or(IntervalError::Arithmetic)?;
    let upper: u128 = projected_endpoint(bounds.1, oldest, policy.rates[1])?
        .checked_add(margin)
        .ok_or(IntervalError::Arithmetic)?;
    if bounds.0 >= bounds.1 || lower >= upper {
        return Err(IntervalError::Arithmetic);
    }
    let interval: ExactInterval = ExactInterval { lower, upper };
    let (outer_lower, outer_upper): (u128, u128) = interval.outward_ns();
    if outer_upper >= UNIX_NS_LIMIT {
        return Err(IntervalError::Arithmetic);
    }
    if outer_upper - outer_lower > policy.max_width.as_nanos() {
        return Err(IntervalError::TooWide);
    }
    Ok(interval)
}

/// Project an endpoint exactly with the shared positive rate denominator.
///
/// # Errors
/// Refuses arithmetic outside the scaled-coordinate representation.
fn projected_endpoint(
    seconds: u64,
    elapsed: Duration,
    rate: NonZeroU32,
) -> Result<u128, IntervalError> {
    // Every wire second is less than 2^64 and SCALED_SECOND is less than 2^60.
    let original: u128 = u128::from(seconds) * SCALED_SECOND;
    let advance: u128 = elapsed
        .as_nanos()
        .checked_mul(u128::from(rate.get()))
        .ok_or(IntervalError::Arithmetic)?;
    original
        .checked_add(advance)
        .ok_or(IntervalError::Arithmetic)
}

#[cfg(test)]
mod tests;

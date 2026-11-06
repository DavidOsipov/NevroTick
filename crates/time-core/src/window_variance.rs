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
//! Checked whole-window derivation of source-quality variance divisors.
//!
//! Modified by David Osipov with AI assistance, 6 November 2026. The selected
//! derivation comes from Chrony 4.9 `sourcestats.c` lines 496 and 528-550,
//! `regress.c` median semantics at lines 415-442 and `util.h` lines 275-284,
//! pinned revision 8df4f1263e206585e6c4e61352d042c2dd12916a. See the crate's
//! provenance and the existing Chrony ledger for notices and adaptation scope.
//!
//! Mandatory bounded garde admission precedes scratch-only sorting and the
//! selected whole-window median/spread derivation. Each binary64 primitive is
//! separately rounded and checked for a finite result; finite subnormals remain
//! supported. Divisors preserve original input order and have no fixture cap.
//! This numerical operation supplies no regression readiness, physical
//! uncertainty, authenticated quality claim or source eligibility.
//! The result borrows the original immutable numeric window and retains explicit
//! precision; it establishes no source identity, freshness or current authority.
//! There is no I/O, logging, clock lookup, history mutation or persistent state.

use std::fmt;

use garde::Validate;

/// Sole fixed capacity for admitted input windows and inline divisor storage.
const MAX_WINDOW_SAMPLES: usize = 64;

/// Source-selected binary64 ratio between sample deviation and distance spread.
const SD_TO_DISTANCE_RATIO: f64 = 0.7;

/// Separately rounded arithmetic step in the selected window derivation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowVarianceStage {
    /// Addition of the two central values in an even-sized window.
    MedianSum,
    /// Multiplication of the even median sum by one half.
    MedianAverage,
    /// Subtraction of the minimum distance from the median.
    MedianDifference,
    /// Division of the median difference by the selected binary64 0.7 constant.
    Spread,
    /// Addition of the minimum distance and explicit local precision.
    Threshold,
    /// Subtraction of the threshold from a larger distance.
    Excess,
    /// Division of a positive excess by the clamped positive spread.
    Ratio,
    /// Addition of one to the distance-to-spread ratio.
    Increment,
    /// Squaring of the dimensionless increment.
    Square,
}

/// Bounded, payload-free admission or arithmetic failure for one whole window.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowVarianceError {
    /// The raw window length is outside the inclusive 1..=64 range.
    CountOutOfRange {
        /// Original supplied length; no input elements have been traversed.
        count: usize,
    },
    /// Local precision is nonfinite or is not strictly positive.
    InvalidPrecision,
    /// The first invalid distance in original input order.
    InvalidDistance {
        /// Zero-based input index, proven smaller than 64 by count admission.
        index: usize,
    },
    /// A separately evaluated arithmetic primitive produced a nonfinite result.
    Arithmetic {
        /// Actual producing arithmetic step, without raw operands or outputs.
        stage: WindowVarianceStage,
    },
    /// Mandatory garde validation disagreed with the shared admission predicates.
    ValidationInvariant,
}

impl fmt::Display for WindowVarianceError {
    /// Format fixed error descriptions and permitted count/index or step codes.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CountOutOfRange { count } => {
                write!(formatter, "window count {count} outside 1..64")
            }
            Self::InvalidPrecision => formatter.write_str("invalid local precision"),
            Self::InvalidDistance { index } => {
                write!(formatter, "invalid peer distance at index {index}")
            }
            Self::Arithmetic { stage } => write!(formatter, "window arithmetic {stage:?}"),
            Self::ValidationInvariant => formatter.write_str("validation invariant disagreement"),
        }
    }
}

impl std::error::Error for WindowVarianceError {}

/// Decide count admission in constant work without inspecting any input element.
///
/// Every raw `usize` is accepted by this effect-free predicate. Its result is the
/// sole count rule shared by mandatory garde admission and typed error projection.
fn valid_count(count: usize) -> bool {
    (1..=MAX_WINDOW_SAMPLES).contains(&count)
}

/// Adapt the shared count predicate to garde's borrowed custom-rule signature.
///
/// # Errors
/// Rejects counts outside 1..=64 with fixed text and no input traversal or effects.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rules require borrowing their raw field and context"
)]
fn count_rule(count: &usize, (): &()) -> garde::Result {
    if valid_count(*count) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid window count"))
    }
}

/// Raw count covered by the shared constant-work rule before any window traversal.
#[derive(Validate)]
struct CountCandidate {
    /// Untrusted input slice length, admitted before scanning its elements.
    #[garde(custom(count_rule))]
    count: usize,
}

/// Run mandatory bounded count admission before any element traversal or copying.
///
/// # Errors
/// Returns the supplied count on rejection or a fixed validator disagreement.
/// No input elements, caller state or partially admitted result are touched.
fn admit_count(count: usize) -> Result<(), WindowVarianceError> {
    let validation = CountCandidate { count }.validate();
    if !valid_count(count) {
        return Err(WindowVarianceError::CountOutOfRange { count });
    }
    validation.map_err(|_| WindowVarianceError::ValidationInvariant)
}

/// Decide the shared seconds rule without arithmetic, normalization or effects.
///
/// All raw binary64 values are accepted by this predicate. Finite values strictly
/// above zero pass, including subnormals; both zeros, negatives and nonfinite
/// values fail. The same rule owns precision and every peer-distance admission.
fn positive_finite_seconds(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

/// Adapt the common positive-finite predicate to garde without exposing raw data.
///
/// # Errors
/// Rejects the shared scalar rule with one fixed payload-free message.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rules require borrowing their raw field and context"
)]
fn seconds_rule(value: &f64, (): &()) -> garde::Result {
    if positive_finite_seconds(*value) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid positive seconds"))
    }
}

/// One raw scalar covered by mandatory garde validation before publication.
#[derive(Validate)]
struct SecondsCandidate {
    /// Raw seconds value, retaining its original bits throughout admission.
    #[garde(custom(seconds_rule))]
    value: f64,
}

/// Run mandatory single-field admission and project its caller-owned typed error.
///
/// The caller supplies the field/index error appropriate to this raw scalar.
/// Work and a possible temporary invalid-report allocation are bounded to one
/// field. No raw values or report escape; no caller state or input bits change.
/// # Errors
/// Returns `invalid` when the common scalar predicate rejects, or the fixed
/// validation-invariant error if garde unexpectedly disagrees with that predicate.
fn admit_seconds(value: f64, invalid: WindowVarianceError) -> Result<(), WindowVarianceError> {
    let validation = SecondsCandidate { value }.validate();
    if !positive_finite_seconds(value) {
        return Err(invalid);
    }
    validation.map_err(|_| WindowVarianceError::ValidationInvariant)
}

/// Admit one separately rounded result without changing finite binary64 bits.
///
/// Any raw result is accepted as input; finite values, including both zeros and
/// subnormals, pass unchanged. The caller identifies the actual producing step.
/// This constant-work helper performs no arithmetic, allocation or state effect.
/// # Errors
/// Returns the supplied arithmetic step when the result is NaN or infinite.
fn finite_result(result: f64, stage: WindowVarianceStage) -> Result<f64, WindowVarianceError> {
    if result.is_finite() {
        Ok(result)
    } else {
        Err(WindowVarianceError::Arithmetic { stage })
    }
}

/// Immutable bounded numeric window, explicit precision and aligned inline divisors.
///
/// The sole fallible constructor establishes positive finite input values, count
/// 1..=64 and a same-length divisor prefix. Its immutable borrow preserves the
/// original numeric snapshot, not source identity or current-context authority.
/// Divisors are finite, dimensionless and at least one. Larger divisors reduce
/// influence in an inverse-divisor consumer. A changed window needs a fresh
/// derivation; no divisor is a permanently attached observation property.
///
/// ```
/// use time_core::{WindowVarianceDivisors, WindowVarianceError};
/// let distances = [1.0, 1.0, 2.0];
/// let window = WindowVarianceDivisors::derive(&distances, 0.25)?;
/// assert_eq!(window.peer_distances_seconds(), &distances);
/// assert_eq!(window.local_precision_seconds(), 0.25);
/// assert_eq!(window.divisors(), &[1.0, 1.0, 16.0]);
/// # Ok::<(), WindowVarianceError>(())
/// ```
///
/// Raw construction cannot bypass admission:
///
/// ```compile_fail,E0451
/// use time_core::WindowVarianceDivisors;
/// let distances = [1.0];
/// let _ = WindowVarianceDivisors {
///     peer_distances_seconds: &distances,
///     local_precision_seconds: 0.25,
///     divisors: [1.0; 64],
/// };
/// ```
///
/// Private fields cannot be mutated after checked construction:
///
/// ```compile_fail,E0616
/// use time_core::WindowVarianceDivisors;
/// let distances = [1.0];
/// let mut window = WindowVarianceDivisors::derive(&distances, 0.25).expect("valid window");
/// window.local_precision_seconds = 0.0;
/// ```
///
/// There is no unchecked default:
///
/// ```compile_fail,E0277
/// use time_core::WindowVarianceDivisors;
/// let _: WindowVarianceDivisors<'_> = Default::default();
/// ```
///
/// Raw tuples have no infallible conversion:
///
/// ```compile_fail,E0277
/// use time_core::WindowVarianceDivisors;
/// let distances = [1.0];
/// let _: WindowVarianceDivisors<'_> = (&distances[..], 0.25).into();
/// ```
///
/// The borrowed numeric snapshot cannot change while the derived view is used:
///
/// ```compile_fail,E0506
/// use time_core::WindowVarianceDivisors;
/// let mut distances = [1.0, 1.0, 2.0];
/// let window = WindowVarianceDivisors::derive(&distances, 0.25).expect("valid window");
/// distances[0] = 2.0;
/// assert_eq!(window.divisors(), &[1.0, 1.0, 16.0]);
/// ```
pub struct WindowVarianceDivisors<'a> {
    /// Original checked immutable distances, preserving all input bits and order.
    peer_distances_seconds: &'a [f64],
    /// Explicit checked positive finite precision, retaining the supplied bits.
    local_precision_seconds: f64,
    /// Fixed inline storage; only the original input-length prefix is exposed.
    divisors: [f64; MAX_WINDOW_SAMPLES],
}

impl fmt::Debug for WindowVarianceDivisors<'_> {
    /// Reveal only the owner type, without automatically formatting numeric inputs.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("WindowVarianceDivisors { .. }")
    }
}

impl<'a> WindowVarianceDivisors<'a> {
    /// Admit one bounded window and retain its original numeric snapshot.
    ///
    /// Count admission precedes precision and original-order distance admission.
    /// Every check runs through mandatory shared garde rules; a temporary rejected
    /// report is bounded and discarded. Positive subnormals, unordered values,
    /// duplicates and precision above the minimum distance are legal inputs.
    /// Only bounded scratch is sorted. The even median preserves the source's
    /// sum-then-half order; spread is `max(precision, min(raw_spread, minimum))`,
    /// including inverted bounds. Each evaluated arithmetic primitive is separately
    /// rounded and checked for finiteness. Finite subnormals remain supported.
    /// Output retains original index order, has no cap or reciprocal restriction,
    /// and is published only after every divisor is finite and at least one.
    /// There is no I/O, clock, logging, history or current-authority effect.
    /// # Errors
    /// Returns count, precision or first invalid-distance errors in that order,
    /// followed by the first nonfinite arithmetic step in algorithm order, or a
    /// fixed validation-invariant disagreement. Source-order intermediate overflow
    /// rejects even if a later clamp could conceal it. Rejection publishes no
    /// result and preserves every caller input bit.
    /// # Panics
    /// No panic is reachable from the public input domain: count admission proves
    /// every scratch-prefix, middle-value and divisor index is within capacity.
    pub fn derive(
        peer_distances_seconds: &'a [f64],
        local_precision_seconds: f64,
    ) -> Result<Self, WindowVarianceError> {
        admit_count(peer_distances_seconds.len())?;
        admit_seconds(
            local_precision_seconds,
            WindowVarianceError::InvalidPrecision,
        )?;
        for (index, distance) in peer_distances_seconds.iter().enumerate() {
            admit_seconds(*distance, WindowVarianceError::InvalidDistance { index })?;
        }
        let count = peer_distances_seconds.len();
        let mut ordered = [0.0; MAX_WINDOW_SAMPLES];
        let ordered_window = &mut ordered[..count];
        ordered_window.copy_from_slice(peer_distances_seconds);
        ordered_window.sort_unstable_by(f64::total_cmp);
        let minimum = ordered_window[0];
        let middle = count / 2;
        let median = if count.is_multiple_of(2) {
            let sum = finite_result(
                ordered_window[middle - 1] + ordered_window[middle],
                WindowVarianceStage::MedianSum,
            )?;
            finite_result(sum * 0.5, WindowVarianceStage::MedianAverage)?
        } else {
            ordered_window[middle]
        };
        let difference = finite_result(median - minimum, WindowVarianceStage::MedianDifference)?;
        let raw_spread = finite_result(
            difference / SD_TO_DISTANCE_RATIO,
            WindowVarianceStage::Spread,
        )?;
        let spread = local_precision_seconds.max(raw_spread.min(minimum));
        if !positive_finite_seconds(spread) {
            return Err(WindowVarianceError::ValidationInvariant);
        }
        let threshold = finite_result(
            minimum + local_precision_seconds,
            WindowVarianceStage::Threshold,
        )?;
        let mut divisors = [1.0; MAX_WINDOW_SAMPLES];
        for (divisor, distance) in divisors.iter_mut().zip(peer_distances_seconds) {
            if *distance > threshold {
                let excess = finite_result(*distance - threshold, WindowVarianceStage::Excess)?;
                let ratio = finite_result(excess / spread, WindowVarianceStage::Ratio)?;
                let increment = finite_result(1.0 + ratio, WindowVarianceStage::Increment)?;
                *divisor = finite_result(increment * increment, WindowVarianceStage::Square)?;
            }
            if *divisor < 1.0 {
                return Err(WindowVarianceError::ValidationInvariant);
            }
        }
        Ok(Self {
            peer_distances_seconds,
            local_precision_seconds,
            divisors,
        })
    }

    /// Return the unchanged checked input slice with its original order and bits.
    #[must_use]
    pub const fn peer_distances_seconds(&self) -> &'a [f64] {
        self.peer_distances_seconds
    }

    /// Return explicit checked precision unchanged, without ambient lookup/effects.
    #[must_use]
    pub const fn local_precision_seconds(&self) -> f64 {
        self.local_precision_seconds
    }

    /// Return exactly one immutable divisor for each original input index.
    ///
    /// The stored count and inline capacity are established together by `derive`.
    /// # Panics
    /// No panic is reachable through the public construction API: the original
    /// slice length is checked against the inline capacity and cannot change.
    #[must_use]
    pub fn divisors(&self) -> &[f64] {
        &self.divisors[..self.peer_distances_seconds.len()]
    }
}

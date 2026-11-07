// SPDX-License-Identifier: GPL-2.0-only
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2009-2024 (ntp_core.c)
 * Copyright (C) Miroslav Lichvar  2009, 2012-2023 (util.c)
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
//! Producer-bound nominal source quality, conditional on declared assumptions.
//!
//! Modified by David Osipov with AI assistance, 7 November 2026. The basic-client
//! calculation adapts Chrony 4.9 revision
//! `8df4f1263e206585e6c4e61352d042c2dd12916a`, `ntp_core.c` lines 2061,
//! 2161–2199 and 2221–2225, and `util.c` lines 651–654 and 908–920. The existing
//! engine task and port ledger retain the exact source hashes and adaptation.
//! Explicit caller limits replace upstream constants; no startup zero-skew
//! assumption, interleaved mode, admission authority or source history is ported.
//!
//! One immutable result borrows the actual terminal facts and checked declared
//! assumptions. Exact intervals are formed before once-rounded conversion. All
//! subsequent binary64 operations are separate, checked computed values, with
//! gradual underflow and canonical positive zero. Conversion bounds remain
//! separate; they are not propagated into a total arithmetic or physical bound.
//! There are no clock reads, I/O, mutations, callbacks or freshness updates.
//! Successful work uses fixed scalar storage; rejected Garde reports are bounded
//! temporaries. This supplies no calibrated uncertainty, eligibility or vote.
#![forbid(unsafe_code)]

use std::{error::Error, fmt, num::NonZeroU64};

use garde::Validate;
use time_domain::{RationalSecondsError, RationalSign, RoundedRationalSeconds};

use super::{MeasurementRefusal, TransportFacts};

/// Decimal scale in the exact local-interval and corrected-RTT numerators.
const NANOS_PER_SECOND: i128 = 1_000_000_000;
/// Exact seconds denominator of a corrected local interval or raw RTT.
const CORRECTED_SECONDS_DENOMINATOR: u64 = 4_294_967_296_000_000_000;
/// Exact seconds denominator of the remote Q32 interval.
const Q32_SECONDS_DENOMINATOR: u64 = 4_294_967_296;

/// Fixed field identity for rejected numeric assumptions, without their values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QualityAssumption {
    /// Historical local quantum must be positive finite seconds.
    HistoricalQuantum,
    /// The TX allowance must be nonnegative finite seconds; either zero is legal.
    TransmitCaptureAllowance,
    /// The RX allowance must be nonnegative finite seconds; either zero is legal.
    ReceiveCaptureAllowance,
    /// Prior frequency endpoints must be finite and ordered.
    FrequencyInterval,
    /// The advertised-distance maximum must be positive finite seconds.
    AdvertisedDistanceLimit,
    /// The delay maximum must be positive finite seconds.
    DelayLimit,
    /// The absolute server-processing maximum must be positive finite seconds.
    ServerProcessingLimit,
}

/// The exact interval whose checked arithmetic or conversion was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QualityInterval {
    /// Corrected nominal local T4 minus T1, in seconds.
    Local,
    /// Signed nominal peer T3 minus T2, in seconds.
    Server,
    /// Independently retained corrected nominal raw round-trip time.
    RawRoundTrip,
}

/// Fixed stage of a nonfinite computed result; no operands are retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QualityStage {
    /// Local plus remote precision.
    CombinedPrecision,
    /// Prior upper frequency minus lower frequency.
    FrequencyWidth,
    /// Half of the checked frequency width.
    FrequencyHalfWidth,
    /// Frequency half-width times absolute local interval.
    FrequencyDispersion,
    /// Capture/precision floor plus frequency dispersion.
    PeerDispersion,
    /// Unsigned advertised root-delay conversion.
    AdvertisedRootDelay,
    /// Unsigned advertised root-dispersion conversion.
    AdvertisedRootDispersion,
    /// Half advertised delay plus advertised dispersion.
    AdvertisedDistance,
    /// Advertised delay plus adjusted peer delay.
    RootDelay,
    /// Advertised dispersion plus peer dispersion.
    RootDispersion,
    /// Half adjusted peer delay plus peer dispersion.
    PeerDistance,
    /// Half total root delay plus total root dispersion.
    RootDistance,
    /// Adjusted peer delay minus peer dispersion.
    AdjustedDelay,
}

/// Bounded refusals with actual typed measurement/conversion causes where present.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QualityRefusal {
    /// A mandatory numeric assumption was invalid.
    InvalidAssumption(QualityAssumption),
    /// Preserve the original terminal measurement refusal.
    Measurement(MeasurementRefusal),
    /// A checked exact interval operation failed.
    ExactArithmetic(QualityInterval),
    /// Preserve the actual once-rounded rational conversion failure.
    Rational {
        /// The interval being converted.
        interval: QualityInterval,
        /// The converter's bounded, payload-free cause.
        source: RationalSecondsError,
    },
    /// A separately evaluated binary64 operation produced a nonfinite value.
    NonFinite(QualityStage),
    /// Computed advertised distance was not strictly below its maximum.
    AdvertisedDistance,
    /// Combined precision exceeded the inclusive delay maximum.
    PrecisionDelay,
    /// Adjusted delay minus peer dispersion exceeded the inclusive delay maximum.
    AdjustedDelay,
    /// Absolute signed server interval exceeded its inclusive maximum.
    ServerProcessing,
    /// A transformed computed result contradicted its checked output contract.
    OutputInvariant,
    /// Mandatory validation rejected otherwise valid assumptions.
    ValidationInvariant,
}

impl fmt::Display for QualityRefusal {
    /// Emit a fixed description without assumptions, packets or numeric payloads.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidAssumption(_) => "conditional quality invalid assumption",
            Self::Measurement(_) => "conditional quality measurement unavailable",
            Self::ExactArithmetic(_) => "conditional quality exact arithmetic",
            Self::Rational { .. } => "conditional quality rational conversion",
            Self::NonFinite(_) => "conditional quality nonfinite arithmetic",
            Self::AdvertisedDistance => "conditional quality advertised distance",
            Self::PrecisionDelay => "conditional quality precision delay",
            Self::AdjustedDelay => "conditional quality adjusted delay",
            Self::ServerProcessing => "conditional quality server processing",
            Self::OutputInvariant => "conditional quality output invariant",
            Self::ValidationInvariant => "conditional quality validation invariant",
        })
    }
}

impl Error for QualityRefusal {
    /// Borrow only the original payload-free measurement or conversion cause.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Measurement(source) => Some(source),
            Self::Rational { source, .. } => Some(source),
            Self::InvalidAssumption(_)
            | Self::ExactArithmetic(_)
            | Self::NonFinite(_)
            | Self::AdvertisedDistance
            | Self::PrecisionDelay
            | Self::AdjustedDelay
            | Self::ServerProcessing
            | Self::OutputInvariant
            | Self::ValidationInvariant => None,
        }
    }
}

/// Sole positive-seconds predicate; positive finite subnormals remain legal.
fn positive_finite(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

/// Sole nonnegative-seconds predicate; both original zero bit patterns are legal.
fn nonnegative_finite(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

/// Sole frequency-interval predicate; width overflow is checked during evaluation.
fn ordered_frequency([lower, upper]: [f64; 2]) -> bool {
    lower.is_finite() && upper.is_finite() && lower <= upper
}

/// Adapt the positive finite rule to mandatory Garde admission.
/// # Errors
/// Reject a nonpositive or nonfinite value with fixed, payload-free text.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde borrows fields and unit context"
)]
fn positive_rule(value: &f64, (): &()) -> garde::Result {
    if positive_finite(*value) {
        Ok(())
    } else {
        Err(garde::Error::new("expected positive finite seconds"))
    }
}

/// Adapt the nonnegative finite rule to mandatory Garde admission.
/// # Errors
/// Reject negative or nonfinite values without formatting the rejected value.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde borrows fields and unit context"
)]
fn nonnegative_rule(value: &f64, (): &()) -> garde::Result {
    if nonnegative_finite(*value) {
        Ok(())
    } else {
        Err(garde::Error::new("expected nonnegative finite seconds"))
    }
}

/// Adapt the complete frequency relationship to mandatory Garde admission.
/// # Errors
/// Reject nonfinite or reversed endpoints; the report contains no endpoints.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde borrows its unit validation context"
)]
fn frequency_rule(value: &[f64; 2], (): &()) -> garde::Result {
    if ordered_frequency(*value) {
        Ok(())
    } else {
        Err(garde::Error::new(
            "expected ordered finite frequency endpoints",
        ))
    }
}

/// Immutable declared prior-frequency endpoints, in seconds per second.
///
/// Numeric admission establishes neither same-source provenance nor physical truth.
/// ```compile_fail,E0616
/// use nevrotick_engine::DeclaredFrequencyInterval;
/// fn mutate(mut prior: DeclaredFrequencyInterval) { prior.endpoints = [0.0, 0.0]; }
/// ```
#[derive(Clone, Copy, Validate)]
pub struct DeclaredFrequencyInterval {
    /// The complete endpoint rule owns finiteness and lower-to-upper ordering.
    #[garde(custom(frequency_rule))]
    endpoints: [f64; 2],
}

impl DeclaredFrequencyInterval {
    /// Admit explicit immutable endpoints, preserving both original bit patterns.
    /// # Errors
    /// Reject nonfinite or reversed endpoints without creating a usable prior.
    pub fn new(lower: f64, upper: f64) -> Result<Self, QualityRefusal> {
        let candidate = Self {
            endpoints: [lower, upper],
        };
        candidate.validate().map_err(|_| {
            if ordered_frequency(candidate.endpoints) {
                QualityRefusal::ValidationInvariant
            } else {
                QualityRefusal::InvalidAssumption(QualityAssumption::FrequencyInterval)
            }
        })?;
        Ok(candidate)
    }

    /// Return the original declared lower endpoint, without a current-truth claim.
    #[must_use]
    pub const fn lower(&self) -> f64 {
        self.endpoints[0]
    }

    /// Return the original declared upper endpoint, without a current-truth claim.
    #[must_use]
    pub const fn upper(&self) -> f64 {
        self.endpoints[1]
    }
}

impl fmt::Debug for DeclaredFrequencyInterval {
    /// Redact all declared frequency values.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("DeclaredFrequencyInterval { .. }")
    }
}

/// Explicit positive finite conditional limits, in seconds; no production defaults.
/// ```compile_fail,E0451
/// use nevrotick_engine::DeclaredQualityLimits;
/// let _ = DeclaredQualityLimits { advertised_distance: 1.0, delay: 1.0, server_processing: 1.0 };
/// ```
#[derive(Clone, Copy, Validate)]
pub struct DeclaredQualityLimits {
    /// Strict maximum of computed advertised half-delay plus dispersion.
    #[garde(custom(positive_rule))]
    advertised_distance: f64,
    /// Inclusive maximum of precision and adjusted delay minus dispersion.
    #[garde(custom(positive_rule))]
    delay: f64,
    /// Inclusive maximum of the absolute computed peer interval.
    #[garde(custom(positive_rule))]
    server_processing: f64,
}

impl DeclaredQualityLimits {
    /// Admit all explicit conditional maxima without changing their bit patterns.
    /// # Errors
    /// Reject the first invalid maximum in advertised/delay/server order.
    pub fn new(
        advertised_distance: f64,
        delay: f64,
        server_processing: f64,
    ) -> Result<Self, QualityRefusal> {
        let candidate = Self {
            advertised_distance,
            delay,
            server_processing,
        };
        candidate.validate().map_err(|_| {
            candidate.issue().map_or(
                QualityRefusal::ValidationInvariant,
                QualityRefusal::InvalidAssumption,
            )
        })?;
        Ok(candidate)
    }

    /// Return the first invalid field using the same admitted scalar predicates.
    fn issue(&self) -> Option<QualityAssumption> {
        if !positive_finite(self.advertised_distance) {
            Some(QualityAssumption::AdvertisedDistanceLimit)
        } else if !positive_finite(self.delay) {
            Some(QualityAssumption::DelayLimit)
        } else if !positive_finite(self.server_processing) {
            Some(QualityAssumption::ServerProcessingLimit)
        } else {
            None
        }
    }

    /// Return the original strict advertised-distance maximum in seconds.
    #[must_use]
    pub const fn maximum_advertised_distance_seconds(&self) -> f64 {
        self.advertised_distance
    }

    /// Return the original inclusive delay maximum in seconds.
    #[must_use]
    pub const fn maximum_delay_seconds(&self) -> f64 {
        self.delay
    }

    /// Return the original inclusive absolute server-processing maximum in seconds.
    #[must_use]
    pub const fn maximum_server_processing_seconds(&self) -> f64 {
        self.server_processing
    }
}

impl fmt::Debug for DeclaredQualityLimits {
    /// Redact the caller's limit values.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("DeclaredQualityLimits { .. }")
    }
}

/// Checked immutable numeric assumptions; neither calibration nor current authority.
///
/// Historical quantum does not supply the current precision for N1 weighting.
/// Capture allowances may retain either zero bit pattern; no missing value has a
/// default. All nested constructors and this constructor invoke mandatory Garde.
/// ```
/// use nevrotick_engine::{DeclaredFrequencyInterval, DeclaredQualityAssumptions,
///     DeclaredQualityLimits, QualityRefusal};
/// # fn main() -> Result<(), QualityRefusal> {
/// let frequency = DeclaredFrequencyInterval::new(-1.0 / 64.0, 1.0 / 64.0)?;
/// let limits = DeclaredQualityLimits::new(1.0, 1.0, 1.0)?;
/// let assumptions = DeclaredQualityAssumptions::new(
///     1.0 / 1024.0, 1.0 / 256.0, 1.0 / 512.0, frequency, limits)?;
/// assert_eq!(assumptions.historical_local_quantum_seconds(), 1.0 / 1024.0);
/// # Ok(())
/// # }
/// ```
/// ```compile_fail,E0616
/// use nevrotick_engine::DeclaredQualityAssumptions;
/// fn mutate(mut assumptions: DeclaredQualityAssumptions) { assumptions.local_quantum = 0.0; }
/// ```
/// ```compile_fail,E0599
/// use nevrotick_engine::DeclaredQualityAssumptions;
/// let _ = DeclaredQualityAssumptions::default();
/// ```
#[derive(Clone, Copy, Validate)]
pub struct DeclaredQualityAssumptions {
    /// Explicit positive finite historical local quantum, in seconds.
    #[garde(custom(positive_rule))]
    local_quantum: f64,
    /// Explicit nonnegative finite TX capture allowance, in seconds.
    #[garde(custom(nonnegative_rule))]
    transmit_allowance: f64,
    /// Explicit nonnegative finite RX capture allowance, in seconds.
    #[garde(custom(nonnegative_rule))]
    receive_allowance: f64,
    /// Mandatory nested validation owns both endpoints and their relationship.
    #[garde(dive)]
    frequency: DeclaredFrequencyInterval,
    /// Mandatory nested validation owns every positive finite conditional limit.
    #[garde(dive)]
    limits: DeclaredQualityLimits,
}

impl DeclaredQualityAssumptions {
    /// Admit every required numeric assumption, preserving original input bits.
    /// # Errors
    /// Reject quantum, TX, RX, prior or limits in that fixed order; no effects occur.
    pub fn new(
        local_quantum: f64,
        transmit_allowance: f64,
        receive_allowance: f64,
        frequency: DeclaredFrequencyInterval,
        limits: DeclaredQualityLimits,
    ) -> Result<Self, QualityRefusal> {
        let candidate = Self {
            local_quantum,
            transmit_allowance,
            receive_allowance,
            frequency,
            limits,
        };
        candidate.validate().map_err(|_| {
            let issue = if !positive_finite(candidate.local_quantum) {
                Some(QualityAssumption::HistoricalQuantum)
            } else if !nonnegative_finite(candidate.transmit_allowance) {
                Some(QualityAssumption::TransmitCaptureAllowance)
            } else if !nonnegative_finite(candidate.receive_allowance) {
                Some(QualityAssumption::ReceiveCaptureAllowance)
            } else if !ordered_frequency(candidate.frequency.endpoints) {
                Some(QualityAssumption::FrequencyInterval)
            } else {
                candidate.limits.issue()
            };
            issue.map_or(
                QualityRefusal::ValidationInvariant,
                QualityRefusal::InvalidAssumption,
            )
        })?;
        Ok(candidate)
    }

    /// Return the original declared historical local quantum in seconds.
    #[must_use]
    pub const fn historical_local_quantum_seconds(&self) -> f64 {
        self.local_quantum
    }

    /// Return the original declared TX capture allowance, including its zero sign.
    #[must_use]
    pub const fn transmit_capture_allowance_seconds(&self) -> f64 {
        self.transmit_allowance
    }

    /// Return the original declared RX capture allowance, including its zero sign.
    #[must_use]
    pub const fn receive_capture_allowance_seconds(&self) -> f64 {
        self.receive_allowance
    }

    /// Borrow the original checked declared prior without a physical-truth claim.
    #[must_use]
    pub const fn frequency_interval(&self) -> &DeclaredFrequencyInterval {
        &self.frequency
    }

    /// Borrow the original explicit conditional limits.
    #[must_use]
    pub const fn limits(&self) -> &DeclaredQualityLimits {
        &self.limits
    }
}

impl fmt::Debug for DeclaredQualityAssumptions {
    /// Redact declared physical quantities, prior endpoints and policy limits.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("DeclaredQualityAssumptions { .. }")
    }
}

/// Three independently once-rounded intervals retaining their original exact tuples.
struct ExactQualityIntervals {
    /// Decimal-corrected nominal local interval L.
    local: RoundedRationalSeconds,
    /// Signed nominal server interval R.
    server: RoundedRationalSeconds,
    /// Independent original corrected nominal raw RTT, never rounded L minus R.
    round_trip: RoundedRationalSeconds,
}

/// Convert any checked exact signed numerator with the named positive denominator.
/// # Errors
/// Preserve the actual rational converter cause; denominator contradiction is exact arithmetic.
fn interval_seconds(
    numerator: i128,
    denominator: u64,
    interval: QualityInterval,
) -> Result<RoundedRationalSeconds, QualityRefusal> {
    let denominator =
        NonZeroU64::new(denominator).ok_or(QualityRefusal::ExactArithmetic(interval))?;
    let sign = if numerator < 0 {
        RationalSign::Negative
    } else {
        RationalSign::Positive
    };
    RoundedRationalSeconds::new(sign, numerator.unsigned_abs(), denominator)
        .map_err(|source| QualityRefusal::Rational { interval, source })
}

/// Derive exact differences exclusively from the same successful terminal measurement.
/// # Errors
/// Preserve original measurement refusal, checked exact failure or actual conversion cause.
fn exact_intervals(facts: &TransportFacts) -> Result<ExactQualityIntervals, QualityRefusal> {
    let measurement = facts.measurement().map_err(QualityRefusal::Measurement)?;
    let [t1, t2, t3, t4] = measurement.coordinates();
    let [r1, r4] = measurement.local_fractional_remainders().map(i128::from);
    let local = t4
        .ticks()
        .checked_sub(t1.ticks())
        .and_then(|ticks| ticks.checked_mul(NANOS_PER_SECOND))
        .and_then(|value| value.checked_add(r4))
        .and_then(|value| value.checked_sub(r1))
        .ok_or(QualityRefusal::ExactArithmetic(QualityInterval::Local))?;
    let server = t3
        .ticks()
        .checked_sub(t2.ticks())
        .ok_or(QualityRefusal::ExactArithmetic(QualityInterval::Server))?;
    Ok(ExactQualityIntervals {
        local: interval_seconds(local, CORRECTED_SECONDS_DENOMINATOR, QualityInterval::Local)?,
        server: interval_seconds(server, Q32_SECONDS_DENOMINATOR, QualityInterval::Server)?,
        round_trip: interval_seconds(
            measurement.nominal_round_trip().numerator(),
            CORRECTED_SECONDS_DENOMINATOR,
            QualityInterval::RawRoundTrip,
        )?,
    })
}

/// Refuse a nonfinite computed intermediate and canonicalize every legal zero.
///
/// Finite signed/subnormal results are preserved. Nonzero-to-zero rounding is
/// deliberately accepted under the computed-value contract, without exactness claims.
/// # Errors
/// Return only the fixed arithmetic stage for a nonfinite result.
fn finite(stage: QualityStage, value: f64) -> Result<f64, QualityRefusal> {
    if !value.is_finite() {
        return Err(QualityRefusal::NonFinite(stage));
    }
    Ok(if value == 0.0 { 0.0 } else { value })
}

/// Check the final finite signed output predicate, including canonical zero.
fn finite_output(value: f64) -> bool {
    value.is_finite() && (value != 0.0 || value.to_bits() == 0)
}

/// Mandatory finite signed-result admission after the checked operation chain.
/// # Errors
/// Reject nonfinite or negative-zero computed outputs with fixed text.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde borrows fields and unit context"
)]
fn finite_output_rule(value: &f64, (): &()) -> garde::Result {
    if finite_output(*value) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid finite output"))
    }
}

/// Admit computed nonnegative finite values with the unique positive-zero encoding.
/// # Errors
/// Reject nonfinite, negative or negative-zero outputs; raw assumptions use their own rule.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde borrows fields and unit context"
)]
fn nonnegative_output_rule(value: &f64, (): &()) -> garde::Result {
    if nonnegative_finite(*value) && finite_output(*value) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid nonnegative output"))
    }
}

/// Atomic computed values; all raw-to-output rules execute before publication.
#[derive(Validate)]
struct ComputedQuality {
    /// Exact binary power from the clamped peer precision exponent.
    #[garde(custom(positive_rule))]
    peer_precision: f64,
    /// Checked local quantum plus peer precision P.
    #[garde(custom(positive_rule))]
    combined_precision: f64,
    /// Checked adjusted delay D, floored by P.
    #[garde(custom(positive_rule))]
    adjusted_delay: f64,
    /// Computed nonnegative prior frequency half-width K.
    #[garde(custom(nonnegative_output_rule))]
    frequency_half_width: f64,
    /// Checked positive peer dispersion E.
    #[garde(custom(positive_rule))]
    peer_dispersion: f64,
    /// Unsigned 16.16 advertised root delay.
    #[garde(custom(nonnegative_output_rule))]
    advertised_root_delay: f64,
    /// Unsigned 16.16 advertised root dispersion.
    #[garde(custom(nonnegative_output_rule))]
    advertised_root_dispersion: f64,
    /// Checked advertised half-delay plus dispersion.
    #[garde(custom(nonnegative_output_rule))]
    advertised_distance: f64,
    /// Checked advertised delay plus D.
    #[garde(custom(positive_rule))]
    root_delay: f64,
    /// Checked advertised dispersion plus E.
    #[garde(custom(positive_rule))]
    root_dispersion: f64,
    /// Checked D/2 plus E.
    #[garde(custom(positive_rule))]
    peer_distance: f64,
    /// Checked total root delay/2 plus total root dispersion.
    #[garde(custom(positive_rule))]
    root_distance: f64,
    /// Checked signed D minus E; may be negative without refusal.
    #[garde(custom(finite_output_rule))]
    delay_minus_dispersion: f64,
}

/// Evaluate separate binary64 primitives only after exact conversions/admission.
/// # Errors
/// Reject the first nonfinite intermediate or final transformed-output invariant.
fn compute_quality(
    facts: &TransportFacts,
    assumptions: &DeclaredQualityAssumptions,
    intervals: &ExactQualityIntervals,
) -> Result<ComputedQuality, QualityRefusal> {
    let header = facts.header();
    let exponent = i32::from(header.precision()).clamp(-31, 31);
    let biased = u64::try_from(exponent + 1023).map_err(|_| QualityRefusal::OutputInvariant)?;
    let peer_precision = f64::from_bits(biased << 52);
    let combined_precision = finite(
        QualityStage::CombinedPrecision,
        assumptions.local_quantum + peer_precision,
    )?;
    // All max operands are admitted finite values; no NaN-suppressing max occurs.
    let adjusted_delay = intervals.round_trip.seconds().abs().max(combined_precision);
    let width = finite(
        QualityStage::FrequencyWidth,
        assumptions.frequency.upper() - assumptions.frequency.lower(),
    )?;
    let frequency_half_width = finite(QualityStage::FrequencyHalfWidth, width / 2.0)?;
    let frequency_dispersion = finite(
        QualityStage::FrequencyDispersion,
        frequency_half_width * intervals.local.seconds().abs(),
    )?;
    let capture_floor = combined_precision.max(
        assumptions
            .transmit_allowance
            .max(assumptions.receive_allowance),
    );
    let peer_dispersion = finite(
        QualityStage::PeerDispersion,
        capture_floor + frequency_dispersion,
    )?;
    let advertised_root_delay = finite(
        QualityStage::AdvertisedRootDelay,
        f64::from(header.root_delay_bits()) / 65_536.0,
    )?;
    let advertised_root_dispersion = finite(
        QualityStage::AdvertisedRootDispersion,
        f64::from(header.root_dispersion_bits()) / 65_536.0,
    )?;
    let advertised_half_delay = finite(
        QualityStage::AdvertisedDistance,
        advertised_root_delay / 2.0,
    )?;
    let advertised_distance = finite(
        QualityStage::AdvertisedDistance,
        advertised_half_delay + advertised_root_dispersion,
    )?;
    let root_delay = finite(
        QualityStage::RootDelay,
        advertised_root_delay + adjusted_delay,
    )?;
    let root_dispersion = finite(
        QualityStage::RootDispersion,
        advertised_root_dispersion + peer_dispersion,
    )?;
    let half_delay = finite(QualityStage::PeerDistance, adjusted_delay / 2.0)?;
    let peer_distance = finite(QualityStage::PeerDistance, half_delay + peer_dispersion)?;
    let half_root_delay = finite(QualityStage::RootDistance, root_delay / 2.0)?;
    let root_distance = finite(
        QualityStage::RootDistance,
        half_root_delay + root_dispersion,
    )?;
    let delay_minus_dispersion = finite(
        QualityStage::AdjustedDelay,
        adjusted_delay - peer_dispersion,
    )?;
    let candidate = ComputedQuality {
        peer_precision,
        combined_precision,
        adjusted_delay,
        frequency_half_width,
        peer_dispersion,
        advertised_root_delay,
        advertised_root_dispersion,
        advertised_distance,
        root_delay,
        root_dispersion,
        peer_distance,
        root_distance,
        delay_minus_dispersion,
    };
    candidate
        .validate()
        .map_err(|_| QualityRefusal::OutputInvariant)?;
    Ok(candidate)
}

/// A computed nominal result inseparable from its actual facts and declared inputs.
///
/// The private evaluator is the sole constructor; borrowed owners cannot be replaced.
/// ```compile_fail,E0616
/// use nevrotick_engine::{ConditionalQuality, TransportFacts};
/// fn substitute<'a>(mut quality: ConditionalQuality<'a>, other: &'a TransportFacts) {
///     quality.facts = other;
/// }
/// ```
/// ```compile_fail,E0599
/// use nevrotick_engine::ConditionalQuality;
/// let _ = ConditionalQuality::default();
/// ```
/// The result cannot outlive either actual borrowed owner:
/// ```compile_fail
/// use nevrotick_engine::{ConditionalQuality, DeclaredQualityAssumptions,
///     QualityRefusal, TransportFacts, evaluate_conditional_quality};
/// fn escape(facts: &TransportFacts, assumptions: &DeclaredQualityAssumptions)
///     -> Result<ConditionalQuality<'static>, QualityRefusal> {
///     evaluate_conditional_quality(facts, assumptions)
/// }
/// ```
pub struct ConditionalQuality<'a> {
    /// Actual terminal producer, retained with all captures, identities and original age.
    facts: &'a TransportFacts,
    /// Original checked declarations, retained without physical-truth assertion.
    assumptions: &'a DeclaredQualityAssumptions,
    /// Checked exact tuples, once-rounded values and separate arithmetic bounds.
    intervals: ExactQualityIntervals,
    /// Fully checked computed values; no statistical or physical enclosure is claimed.
    computed: ComputedQuality,
}

impl fmt::Debug for ConditionalQuality<'_> {
    /// Redact facts, assumptions, intervals, quality values and original age.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ConditionalQuality { .. }")
    }
}

impl<'a> ConditionalQuality<'a> {
    /// Borrow the exact original terminal facts; evaluation never renews their age.
    #[must_use]
    pub const fn original_facts(&self) -> &'a TransportFacts {
        self.facts
    }
    /// Borrow the exact original declared assumptions without a current-truth claim.
    #[must_use]
    pub const fn declared_assumptions(&self) -> &'a DeclaredQualityAssumptions {
        self.assumptions
    }
    /// Return exact corrected L and its once-rounded seconds/arithmetic bound.
    #[must_use]
    pub const fn local_interval(&self) -> RoundedRationalSeconds {
        self.intervals.local
    }
    /// Return exact signed R and its once-rounded seconds/arithmetic bound.
    #[must_use]
    pub const fn server_interval(&self) -> RoundedRationalSeconds {
        self.intervals.server
    }
    /// Return the independent exact corrected signed RTT, never rounded L minus R.
    #[must_use]
    pub const fn raw_round_trip(&self) -> RoundedRationalSeconds {
        self.intervals.round_trip
    }
    /// Return clamped remote precision p, in seconds.
    #[must_use]
    pub const fn peer_precision_seconds(&self) -> f64 {
        self.computed.peer_precision
    }
    /// Return computed historical local plus remote precision P, in seconds.
    #[must_use]
    pub const fn combined_precision_seconds(&self) -> f64 {
        self.computed.combined_precision
    }
    /// Return computed precision-floored absolute raw RTT D, in seconds.
    #[must_use]
    pub const fn adjusted_delay_seconds(&self) -> f64 {
        self.computed.adjusted_delay
    }
    /// Return computed prior-frequency half-width K, in seconds per second.
    #[must_use]
    pub const fn frequency_half_width(&self) -> f64 {
        self.computed.frequency_half_width
    }
    /// Return computed peer dispersion E, conditional on declared allowances/prior.
    #[must_use]
    pub const fn peer_dispersion_seconds(&self) -> f64 {
        self.computed.peer_dispersion
    }
    /// Return the same header's unsigned 16.16 advertised root delay in seconds.
    #[must_use]
    pub const fn advertised_root_delay_seconds(&self) -> f64 {
        self.computed.advertised_root_delay
    }
    /// Return the same header's unsigned 16.16 advertised root dispersion in seconds.
    #[must_use]
    pub const fn advertised_root_dispersion_seconds(&self) -> f64 {
        self.computed.advertised_root_dispersion
    }
    /// Return computed advertised half-delay plus dispersion, in seconds.
    #[must_use]
    pub const fn advertised_distance_seconds(&self) -> f64 {
        self.computed.advertised_distance
    }
    /// Return computed total root delay RD, in seconds.
    #[must_use]
    pub const fn root_delay_seconds(&self) -> f64 {
        self.computed.root_delay
    }
    /// Return computed total root dispersion RE, in seconds.
    #[must_use]
    pub const fn root_dispersion_seconds(&self) -> f64 {
        self.computed.root_dispersion
    }
    /// Return computed within-source distance D/2+E, with no physical-bound claim.
    #[must_use]
    pub const fn peer_distance_seconds(&self) -> f64 {
        self.computed.peer_distance
    }
    /// Return computed root distance RD/2+RE, with no physical-bound claim.
    #[must_use]
    pub const fn root_distance_seconds(&self) -> f64 {
        self.computed.root_distance
    }
    /// Return the computed signed D-E used by the inclusive delay comparison.
    #[must_use]
    pub const fn delay_minus_dispersion_seconds(&self) -> f64 {
        self.computed.delay_minus_dispersion
    }
}

/// Evaluate one actual terminal observation under explicit checked numeric assumptions.
///
/// Only the original facts supply the header and conditional measurement. Comparisons
/// use the computed binary64 values: advertised distance is strict, then precision,
/// D-E and absolute server interval use inclusive limits. No clock/state change,
/// source eligibility, history append or physical calibration follows success.
/// # Errors
/// Preserve measurement/conversion/arithmetic refusal before policy checks, then
/// advertised distance, precision delay, adjusted delay and server-processing order.
pub fn evaluate_conditional_quality<'a>(
    facts: &'a TransportFacts,
    assumptions: &'a DeclaredQualityAssumptions,
) -> Result<ConditionalQuality<'a>, QualityRefusal> {
    let intervals = exact_intervals(facts)?;
    let computed = compute_quality(facts, assumptions, &intervals)?;
    if computed.advertised_distance >= assumptions.limits.advertised_distance {
        return Err(QualityRefusal::AdvertisedDistance);
    }
    if computed.combined_precision > assumptions.limits.delay {
        return Err(QualityRefusal::PrecisionDelay);
    }
    if computed.delay_minus_dispersion > assumptions.limits.delay {
        return Err(QualityRefusal::AdjustedDelay);
    }
    if intervals.server.seconds().abs() > assumptions.limits.server_processing {
        return Err(QualityRefusal::ServerProcessing);
    }
    Ok(ConditionalQuality {
        facts,
        assumptions,
        intervals,
        computed,
    })
}

/// Focused internal numerical and mandatory-admission controls; no native claim.
#[cfg(test)]
mod tests;

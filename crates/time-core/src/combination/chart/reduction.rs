// SPDX-License-Identifier: GPL-2.0-only
//! Private chart reduction over already admitted actual supporter records.
//!
//! The unchanged combination owner computes the five supported point values.
//! Companions follow the same ordered point operations for the two means and
//! separately bound their new arithmetic. This module has no state or effects.

use crate::combination::admission::chart_receiving::{
    AdmittedChartSupporters, ChartTrackingRecord,
};
use crate::combination::{
    FixtureCombinationError, FixtureCombinationStep, Quantities, Sums, SupporterValues,
};

use super::{Companion, CompanionError};
use std::fmt;

/// A checked point-reduction, propagated-bound or private consistency failure.
#[derive(Debug)]
pub(in crate::combination) enum ChartReductionError {
    /// Preserve the existing point owner's actual checked error.
    Combination(FixtureCombinationError),
    /// Preserve the companion owner's actual checked error.
    Companion(CompanionError),
    /// The separately evaluated means did not preserve identical point bits.
    Invariant,
}

impl fmt::Display for ChartReductionError {
    /// Display only a bounded failure family, never numerical inputs.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Combination(_) => "chart combination: point arithmetic failed",
            Self::Companion(_) => "chart combination: error bound failed",
            Self::Invariant => "chart combination: point mismatch",
        })
    }
}

impl std::error::Error for ChartReductionError {
    /// Borrow the actual checked cause without reconstructing an error.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Combination(source) => Some(source),
            Self::Companion(source) => Some(source),
            Self::Invariant => None,
        }
    }
}

/// Five completed points and separate deterministic mean-error bounds.
pub(in crate::combination) struct ChartQuantities {
    /// Actual checked combination points, including statistical SD and skew.
    pub(in crate::combination) points: Quantities,
    /// Absolute new-arithmetic bound for the fitted phase mean.
    pub(in crate::combination) phase_error: f64,
    /// Absolute new-arithmetic bound for the fitted rate mean.
    pub(in crate::combination) rate_error: f64,
}

impl ChartQuantities {
    /// Copy one genuine original-witness singleton without denominator arithmetic.
    ///
    /// The admitted complete original witness must contain exactly one entry.
    /// The same checked attachment owns the transformed values, separate bounds
    /// and unchanged statistical metadata.
    /// # Errors
    /// Reject a non-singleton witness or inconsistent admitted principal.
    pub(in crate::combination) fn singleton(
        admitted: &AdmittedChartSupporters<'_, '_, '_>,
    ) -> Result<Self, ChartReductionError> {
        if admitted.count() != 1 {
            return Err(ChartReductionError::Invariant);
        }
        let record = admitted
            .record(admitted.principal_original().context().source_id())
            .map_err(ChartReductionError::Combination)?;
        let projection = record.entry().projection();
        let tracking = record.tracking();
        Ok(Self {
            points: Quantities {
                phase_seconds: tracking.phase_seconds(),
                phase_rate_seconds_per_second: projection.phase_rate_seconds_per_second(),
                phase_standard_deviation_seconds: tracking.phase_standard_deviation_seconds(),
                rate_standard_deviation_seconds_per_second: tracking
                    .rate_standard_deviation_seconds_per_second(),
                tracking_skew_seconds_per_second: tracking.tracking_skew_seconds_per_second(),
            },
            phase_error: tracking.phase_roundoff_seconds(),
            rate_error: projection.rate_roundoff_seconds_per_second(),
        })
    }
}

/// Transaction-local ascending-label sums for a non-singleton witness.
pub(in crate::combination) struct ChartSums {
    /// Existing point and statistical accumulators.
    points: Sums,
    /// Sum of phase weights and its absolute error.
    phase_weight: Companion,
    /// Weighted fitted-phase sum and its absolute error.
    phase: Companion,
    /// Sum of rate weights and its absolute error.
    rate_weight: Companion,
    /// Weighted fitted-rate sum and its absolute error.
    rate: Companion,
}

impl ChartSums {
    /// Start an unpublished reduction with positive-zero point/error pairs.
    /// # Errors
    /// Preserve an unexpected checked zero-construction failure.
    pub(in crate::combination) fn empty() -> Result<Self, ChartReductionError> {
        let zero = Companion::exact(0.0).map_err(ChartReductionError::Companion)?;
        Ok(Self {
            points: Sums::empty(),
            phase_weight: zero,
            phase: zero,
            rate_weight: zero,
            rate: zero,
        })
    }

    /// Stage one actual record in canonical order, without changing prior sums.
    ///
    /// Source distance and SD come from the same admitted actual entry. The
    /// transformed fitted phase/rate and their bounds come from that entry's
    /// chart projection. The caller owns complete admission and witness order.
    /// # Errors
    /// Preserve the first reached point, bound or private construction failure.
    pub(in crate::combination) fn with_record(
        &self,
        record: ChartTrackingRecord<'_, '_>,
        principal_phase: f64,
        index: usize,
    ) -> Result<Self, ChartReductionError> {
        use FixtureCombinationStep as Step;
        let projection = record.entry().projection();
        let original = record.original();
        let tracking = record.tracking();
        let distance = original.distance_seconds();
        let rate_sd = tracking.rate_standard_deviation_seconds_per_second();
        let points = self
            .points
            .with_values(
                SupporterValues {
                    distance,
                    phase: tracking.phase_seconds(),
                    rate: projection.phase_rate_seconds_per_second(),
                    phase_sd: tracking.phase_standard_deviation_seconds(),
                    rate_sd,
                    tracking_skew: tracking.tracking_skew_seconds_per_second(),
                },
                principal_phase,
                index,
            )
            .map_err(ChartReductionError::Combination)?;

        let exact = |value| Companion::exact(value).map_err(ChartReductionError::Companion);
        let map = ChartReductionError::Companion;
        let one = exact(1.0)?;
        let phase = Companion::new(tracking.phase_seconds(), tracking.phase_roundoff_seconds())
            .map_err(map)?;
        let rate = Companion::new(
            projection.phase_rate_seconds_per_second(),
            projection.rate_roundoff_seconds_per_second(),
        )
        .map_err(map)?;
        let phase_weight = one
            .divide_positive(exact(distance)?, Step::PhaseWeight)
            .map_err(map)?;
        let deviation = exact(rate_sd)?;
        let variance = deviation
            .multiply(deviation, Step::RateVariance)
            .map_err(map)?;
        let rate_weight = one
            .divide_positive(variance, Step::RateWeight)
            .map_err(map)?;
        let phase_weight_sum = self
            .phase_weight
            .add(phase_weight, Step::PhaseWeightSum)
            .map_err(map)?;
        let weighted_phase = phase_weight
            .multiply(phase, Step::WeightedPhase)
            .map_err(map)?;
        let phase_sum = self
            .phase
            .add(weighted_phase, Step::PhaseSum)
            .map_err(map)?;
        let rate_weight_sum = self
            .rate_weight
            .add(rate_weight, Step::RateWeightSum)
            .map_err(map)?;
        let weighted_rate = rate_weight
            .multiply(rate, Step::WeightedRate)
            .map_err(map)?;
        let rate_sum = self.rate.add(weighted_rate, Step::RateSum).map_err(map)?;
        Ok(Self {
            points,
            phase_weight: phase_weight_sum,
            phase: phase_sum,
            rate_weight: rate_weight_sum,
            rate: rate_sum,
        })
    }

    /// Finish both means after complete nonempty reduction and compare point bits.
    /// # Errors
    /// Preserve actual point/bound failures or reject any private order mismatch.
    pub(in crate::combination) fn finish(&self) -> Result<ChartQuantities, ChartReductionError> {
        use FixtureCombinationStep as Step;
        let points = self
            .points
            .finish()
            .map_err(ChartReductionError::Combination)?;
        let phase = self
            .phase
            .divide_positive(self.phase_weight, Step::PhaseMean)
            .map_err(ChartReductionError::Companion)?;
        let rate = self
            .rate
            .divide_positive(self.rate_weight, Step::RateMean)
            .map_err(ChartReductionError::Companion)?;
        if phase.value().to_bits() != points.phase_seconds.to_bits()
            || rate.value().to_bits() != points.phase_rate_seconds_per_second.to_bits()
        {
            return Err(ChartReductionError::Invariant);
        }
        Ok(ChartQuantities {
            points,
            phase_error: phase.error(),
            rate_error: rate.error(),
        })
    }
}

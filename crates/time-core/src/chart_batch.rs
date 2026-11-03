// SPDX-License-Identifier: GPL-2.0-only
//! Bounded common-chart admission and provenance-retaining numerical selection.
//!
//! Compatibility means identical checked frame/domain values and displacement/
//! rate bits, plus one exact evaluation instant. Independently constructed equal
//! descriptors are compatible; every original descriptor borrow is retained.
//! Selection delegates to the existing interval owner without refitting histories,
//! changing observations or granting freshness, eligibility or clock authority.

use std::fmt;

use garde::Validate;
use time_domain::MonotonicNs;

use crate::{
    BatchError, FixtureAffineChart, FixtureBatch, FixtureChartDomain, FixtureChartFrames,
    FixtureChartInterval, FixtureMinimum, FixtureSelection, SelectionError, select_fixture,
};

/// The existing interval owner's fixed capacity; admission checks it before reads.
const MAX_CHART_ENTRIES: usize = 16;

/// First incompatible relationship within one original chart interval.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureChartMismatchKind {
    /// At least one source/target frame or mapping-generation tag differs.
    Frames,
    /// Inclusive domain start, common reference or inclusive end differs.
    Domain,
    /// Displacement bits differ, including distinct signed-zero encodings.
    Displacement,
    /// Rate bits differ, including distinct signed-zero encodings.
    Rate,
    /// The original projection was evaluated at another exact monotonic instant.
    EvaluationTime,
}

/// Bounded chart-admission failure without raw labels, instants or scalar values.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum FixtureChartBatchError {
    /// The complete supplied slice exceeds sixteen entries.
    CapacityExceeded,
    /// The declared query is outside the expected chart's inclusive domain.
    EvaluationOutsideDomain,
    /// First incompatible entry, after header admission and before duplicate checks.
    Mismatch {
        /// Bounded zero-based input index, in ascending input order.
        index: usize,
        /// First relationship at that index in the documented field order.
        kind: FixtureChartMismatchKind,
    },
    /// The actual interval-batch owner rejected otherwise compatible entries.
    Batch {
        /// Original typed cause, including the earliest later duplicate position.
        source: BatchError,
    },
    /// Mandatory validation or checked staging disagreed with admitted inputs.
    Invariant,
}

impl fmt::Debug for FixtureChartBatchError {
    /// Retain a fixed bounded description without formatting rejected metadata.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureChartBatchError { .. }")
    }
}

impl fmt::Display for FixtureChartBatchError {
    /// Display a fixed code without chart values or source identities.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("chart fixture batch rejected")
    }
}

impl std::error::Error for FixtureChartBatchError {
    /// Preserve the actual downstream batch cause without inventing one.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Batch { source } => Some(source),
            _ => None,
        }
    }
}

/// Immutable already-checked descriptor facts and the exact caller-supplied query.
struct ChartExpectation {
    /// All four checked source/target mapping assertions.
    frames: FixtureChartFrames,
    /// Full checked domain, including the common displacement reference.
    domain: FixtureChartDomain,
    /// Exact admitted displacement representation, without normalization.
    phase_bits: u64,
    /// Exact admitted rate representation, without normalization.
    rate_bits: u64,
    /// Declared query; its relationship to the domain is checked at admission.
    at: MonotonicNs,
}

/// Sole relationship carrier; the constructor always consumes its validation.
#[derive(garde::Validate)]
#[garde(context(ChartExpectation))]
struct ChartBatchCandidate<'entries, 'chart> {
    /// Complete original immutable slice; checked by the shared bounded predicate.
    #[garde(custom(chart_entries_rule))]
    values: &'entries [FixtureChartInterval<'chart>],
}

/// Compare one actual descriptor with immutable expected fields in contract order.
///
/// Checked field types retain their own invariants; scalar bits preserve signed
/// zero. This pure predicate compares assertions without granting map authority.
fn descriptor_mismatch(
    frames: FixtureChartFrames,
    domain: FixtureChartDomain,
    phase_bits: u64,
    rate_bits: u64,
    actual: &FixtureAffineChart,
) -> Option<FixtureChartMismatchKind> {
    if actual.frames() != frames {
        Some(FixtureChartMismatchKind::Frames)
    } else if actual.domain() != domain {
        Some(FixtureChartMismatchKind::Domain)
    } else if actual.phase_at_reference_seconds().to_bits() != phase_bits {
        Some(FixtureChartMismatchKind::Displacement)
    } else if actual.rate_seconds_per_second().to_bits() != rate_bits {
        Some(FixtureChartMismatchKind::Rate)
    } else {
        None
    }
}

/// Reuse complete value/bit descriptor comparison at another checked receiver.
///
/// Equal values at different addresses agree. No observation, generation or
/// achieved clock effect follows from matching mathematical descriptors.
pub(crate) fn chart_descriptor_mismatch(
    expected: &FixtureAffineChart,
    actual: &FixtureAffineChart,
) -> Option<FixtureChartMismatchKind> {
    descriptor_mismatch(
        expected.frames(),
        expected.domain(),
        expected.phase_at_reference_seconds().to_bits(),
        expected.rate_seconds_per_second().to_bits(),
        actual,
    )
}

/// Return the first bounded admission failure, without modifying any input.
///
/// Capacity precedes header-domain admission, then indices ascend with frames,
/// domain, displacement bits, rate bits and evaluation time in that order.
/// Checked interval geometry remains owned by each original interval.
fn first_issue(
    values: &[FixtureChartInterval<'_>],
    expected: &ChartExpectation,
) -> Option<FixtureChartBatchError> {
    if values.len() > MAX_CHART_ENTRIES {
        return Some(FixtureChartBatchError::CapacityExceeded);
    }
    if expected.at < expected.domain.start() || expected.at > expected.domain.end() {
        return Some(FixtureChartBatchError::EvaluationOutsideDomain);
    }
    for (index, value) in values.iter().enumerate() {
        let projection = value.projection();
        let kind = descriptor_mismatch(
            expected.frames,
            expected.domain,
            expected.phase_bits,
            expected.rate_bits,
            projection.chart(),
        )
        .or_else(|| {
            (projection.source_projection().projected_at() != expected.at)
                .then_some(FixtureChartMismatchKind::EvaluationTime)
        });
        if let Some(kind) = kind {
            return Some(FixtureChartBatchError::Mismatch { index, kind });
        }
    }
    None
}

/// Apply the same capacity/header/entry predicate inside mandatory Garde validation.
/// # Errors
/// Return a fixed bounded validation error for the first incompatible relationship.
fn chart_entries_rule(
    values: &[FixtureChartInterval<'_>],
    expected: &ChartExpectation,
) -> garde::Result {
    if first_issue(values, expected).is_some() {
        Err(garde::Error::new("incompatible chart fixture entries"))
    } else {
        Ok(())
    }
}

/// Borrow original chart wrappers while privately owning their checked numeric batch.
///
/// No constructor accepts naked intervals or legacy projected wrappers. Equal
/// descriptor values at distinct addresses are accepted, while each wrapper keeps
/// its actual original descriptor. No map, observation or history is rewritten.
///
/// ```
/// use time_core::{FixtureAffineChart, FixtureChartBatch, FixtureChartInterval,
///     FixtureMinimum};
/// use time_domain::MonotonicNs;
/// fn inspect<'entries, 'chart>(
///     chart: &'chart FixtureAffineChart,
///     at: MonotonicNs,
///     entries: &'entries [FixtureChartInterval<'chart>],
///     minimum: FixtureMinimum,
/// ) -> Result<(), Box<dyn std::error::Error>> {
///     let batch = FixtureChartBatch::new(chart, at, entries)?;
///     let selected = batch.select(minimum)?;
///     assert!(std::ptr::eq(selected.batch(), std::ptr::from_ref(&batch)));
///     assert_eq!(selected.selection().population(), entries.len());
///     Ok(())
/// }
/// ```
///
/// External callers cannot forge the private admitted representation:
///
/// ```compile_fail,E0451
/// use time_core::{FixtureAffineChart, FixtureBatch, FixtureChartBatch,
///     FixtureChartInterval};
/// use time_domain::MonotonicNs;
/// fn forge<'entries, 'chart>(
///     chart: &'chart FixtureAffineChart,
///     at: MonotonicNs,
///     values: &'entries [FixtureChartInterval<'chart>],
///     intervals: FixtureBatch,
/// ) -> FixtureChartBatch<'entries, 'chart> {
///     FixtureChartBatch { chart, at, values, intervals }
/// }
/// ```
///
/// No automatic chart-to-legacy conversion drops the chart or arithmetic bound:
///
/// ```compile_fail,E0277
/// use time_core::{FixtureChartInterval, FixtureProjectedInterval};
/// fn drop_chart(value: FixtureChartInterval<'_>) -> FixtureProjectedInterval {
///     value.into()
/// }
/// ```
pub struct FixtureChartBatch<'entries, 'chart> {
    /// Declared checked descriptor; every admitted original agrees by value/bits.
    chart: &'chart FixtureAffineChart,
    /// Exact common evaluation instant, within the declared descriptor domain.
    at: MonotonicNs,
    /// Entire original input in its original order, without filtering or copying.
    values: &'entries [FixtureChartInterval<'chart>],
    /// Actual downstream owner of interval capacity and unique source labels.
    intervals: FixtureBatch,
}

impl fmt::Debug for FixtureChartBatch<'_, '_> {
    /// Redact all original chart, source and observation values.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureChartBatch { .. }")
    }
}

impl<'entries, 'chart> FixtureChartBatch<'entries, 'chart> {
    /// Admit one complete slice without reprojection, refitting or partial output.
    ///
    /// Capacity rejects before entry reads. All other potentially successful paths
    /// consume mandatory shared Garde validation, retain ordered relationship
    /// causes, then use the actual interval-batch constructor for duplicate labels.
    /// Scratch is fixed at sixteen checked intervals; no input is mutated.
    /// # Errors
    /// Reject capacity, out-of-domain header, first context/time mismatch, then
    /// preserve actual `BatchError`. Unexpected validation/staging disagreement
    /// returns `Invariant`. Empty input still requires an in-domain query.
    pub fn new(
        expected_chart: &'chart FixtureAffineChart,
        expected_at: MonotonicNs,
        values: &'entries [FixtureChartInterval<'chart>],
    ) -> Result<Self, FixtureChartBatchError> {
        if values.len() > MAX_CHART_ENTRIES {
            return Err(FixtureChartBatchError::CapacityExceeded);
        }
        let expected = ChartExpectation {
            frames: expected_chart.frames(),
            domain: expected_chart.domain(),
            phase_bits: expected_chart.phase_at_reference_seconds().to_bits(),
            rate_bits: expected_chart.rate_seconds_per_second().to_bits(),
            at: expected_at,
        };
        let raw = ChartBatchCandidate { values };
        let validated = raw.validate_with(&expected);
        if let Some(error) = first_issue(values, &expected) {
            return Err(error);
        }
        validated.map_err(|_| FixtureChartBatchError::Invariant)?;
        let intervals = if let Some(first) = values.first() {
            let mut scratch = [*first.interval(); MAX_CHART_ENTRIES];
            for (slot, value) in scratch.iter_mut().zip(values.iter()) {
                *slot = *value.interval();
            }
            let occupied = scratch
                .get(..values.len())
                .ok_or(FixtureChartBatchError::Invariant)?;
            FixtureBatch::new(occupied)
        } else {
            FixtureBatch::new(&[])
        }
        .map_err(|source| FixtureChartBatchError::Batch { source })?;
        Ok(Self {
            chart: expected_chart,
            at: expected_at,
            values,
            intervals,
        })
    }

    /// Borrow the declared descriptor without replacing any entry's own descriptor.
    #[must_use]
    pub const fn chart(&self) -> &'chart FixtureAffineChart {
        self.chart
    }

    /// Return the exact admitted common query without reading an ambient clock.
    #[must_use]
    pub const fn projected_at(&self) -> MonotonicNs {
        self.at
    }

    /// Borrow the complete original slice in its original order.
    #[must_use]
    pub const fn entries(&self) -> &'entries [FixtureChartInterval<'chart>] {
        self.values
    }

    /// Return the admitted population without filtering sources or changing state.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.values.len()
    }

    /// Report empty admitted input, without asserting any observation occurred.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Select through the existing majority owner and retain this exact batch.
    ///
    /// The supplied minimum is unchanged through source loss or disagreement.
    /// The result borrows this batch and cannot outlive it. No entry is dropped,
    /// no new observation is credited and no legacy projected wrapper is created.
    /// # Errors
    /// Preserve the selector's actual typed invariant failure without fallback.
    pub fn select(
        &self,
        minimum: FixtureMinimum,
    ) -> Result<FixtureChartSelection<'_, 'entries, 'chart>, SelectionError> {
        let selection = select_fixture(&self.intervals, minimum)?;
        Ok(FixtureChartSelection {
            batch: self,
            selection,
        })
    }
}

/// Numerical selection bound to its exact admitted chart batch and original input.
///
/// Read-only numeric facts remain inert. This wrapper has no public constructor,
/// mutable-inner access, owned legacy conversion or independent freshness claim.
///
/// The result cannot escape a batch destroyed at the end of a function:
///
/// ```compile_fail,E0515
/// use time_core::{FixtureAffineChart, FixtureChartBatch, FixtureChartInterval,
///     FixtureChartSelection, FixtureMinimum};
/// use time_domain::MonotonicNs;
/// fn detach<'entries, 'chart>(
///     chart: &'chart FixtureAffineChart,
///     at: MonotonicNs,
///     values: &'entries [FixtureChartInterval<'chart>],
///     minimum: FixtureMinimum,
/// ) -> Result<FixtureChartSelection<'static, 'entries, 'chart>, Box<dyn std::error::Error>> {
///     let batch = FixtureChartBatch::new(chart, at, values)?;
///     Ok(batch.select(minimum)?)
/// }
/// ```
///
/// Supplying a genuine batch and detached numeric result grants no constructor:
///
/// ```compile_fail,E0451
/// use time_core::{FixtureChartBatch, FixtureChartSelection, FixtureSelection};
/// fn forge<'batch, 'entries, 'chart>(
///     batch: &'batch FixtureChartBatch<'entries, 'chart>,
///     selection: FixtureSelection,
/// ) -> FixtureChartSelection<'batch, 'entries, 'chart> {
///     FixtureChartSelection { batch, selection }
/// }
/// ```
pub struct FixtureChartSelection<'batch, 'entries, 'chart> {
    /// Exact immutable batch that supplied the complete voting population.
    batch: &'batch FixtureChartBatch<'entries, 'chart>,
    /// Actual unmodified result from the existing interval-majority owner.
    selection: FixtureSelection,
}

impl fmt::Debug for FixtureChartSelection<'_, '_, '_> {
    /// Keep result formatting independent of all source and chart values.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureChartSelection { .. }")
    }
}

impl<'batch, 'entries, 'chart> FixtureChartSelection<'batch, 'entries, 'chart> {
    /// Borrow the actual admitted batch, preserving its original slice and chart.
    #[must_use]
    pub const fn batch(&self) -> &'batch FixtureChartBatch<'entries, 'chart> {
        self.batch
    }

    /// Inspect actual numerical output without constructing a new chart result.
    #[must_use]
    pub const fn selection(&self) -> &FixtureSelection {
        &self.selection
    }
}

#[cfg(test)]
#[path = "../tests/private/chart_batch_guards.rs"]
mod tests;

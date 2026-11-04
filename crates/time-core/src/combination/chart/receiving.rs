// SPDX-License-Identifier: GPL-2.0-only
//! Complete original-witness tracking admission from genuine chart decisions.
//!
//! Actual chart wrappers and their own tracking attachments remain borrowed.
//! Raw source context, rate and distance come only from `source_projection()`;
//! transformed fitted phase and its error stay paired with the same entry's SD.
//! Admission performs no filtering, arithmetic, state transition or publication.
//! The chart lifecycle consumer admits this complete relationship before effects.

use std::{num::NonZeroU64, ptr};

use garde::Validate;

use crate::{
    FixtureAnchorProjection, FixtureChartInterval, FixtureChartPrincipalDecision,
    FixtureChartTracking, FixturePrincipalOutcomeKind, FixtureSelection,
};

use crate::combination::{
    FixtureCombinationContextField, FixtureCombinationError, FixtureTrackingCombinationError,
    MAX_SUPPORTERS,
};

use super::{AdmittedOriginals, OriginalRecord};

/// One actual chart slot and the same slot's immutable fitted attachment.
#[derive(Clone, Copy)]
pub(in crate::combination) struct ChartTrackingRecord<'entries, 'chart> {
    /// Actual final batch slot; a copied equal wrapper is a different identity.
    entry: &'entries FixtureChartInterval<'chart>,
    /// Exact attachment borrowed from this entry's own opaque projection.
    tracking: &'entries FixtureChartTracking,
}

impl<'entries, 'chart> ChartTrackingRecord<'entries, 'chart> {
    /// Borrow the actual wrapper, retaining its original descriptor identity.
    pub(in crate::combination) const fn entry(self) -> &'entries FixtureChartInterval<'chart> {
        self.entry
    }

    /// Borrow the fitted phase/error and same-producer SD/skew together.
    pub(in crate::combination) const fn tracking(self) -> &'entries FixtureChartTracking {
        self.tracking
    }

    /// Borrow unchanged original metadata for scoring and compatibility only.
    pub(in crate::combination) const fn original(self) -> &'entries FixtureAnchorProjection {
        self.entry.projection().source_projection()
    }
}

/// Complete canonical witness records, derived before any compatibility filter.
pub(in crate::combination) struct ChartTrackingSupporters<'entries, 'chart> {
    /// Occupied canonical prefix; no fabricated entry or attachment is stored.
    records: [Option<ChartTrackingRecord<'entries, 'chart>>; MAX_SUPPORTERS],
    /// Original witness size, or zero when the actual decision has no principal.
    count: usize,
}

impl<'entries, 'chart> ChartTrackingSupporters<'entries, 'chart> {
    /// Derive every actual witnessed attachment in ascending original-label order.
    ///
    /// An absent principal yields an empty holder. Otherwise all witnessed
    /// entries are visited, including later-excluded and retained participants.
    /// The checked decision owns selection/descriptor provenance; no wrapper,
    /// uncertainty, source history or observation credit is reconstructed.
    /// # Errors
    /// Reject inconsistent bounded occupancy or the first missing witness attachment.
    pub(in crate::combination) fn from_decision(
        decision: &FixtureChartPrincipalDecision<'_, 'entries, 'chart>,
    ) -> Result<Self, FixtureTrackingCombinationError> {
        let mut records = [None; MAX_SUPPORTERS];
        if decision.entries().len() > MAX_SUPPORTERS {
            return Err(FixtureTrackingCombinationError::Combination {
                source: FixtureCombinationError::CapacityExceeded,
            });
        }
        if decision.principal_entry().is_none() {
            return Ok(Self { records, count: 0 });
        }
        let witness = decision.selection().witnessed().ok_or(
            FixtureTrackingCombinationError::Combination {
                source: FixtureCombinationError::Invariant,
            },
        )?;
        let count = witness.supporters().len();
        if count == 0 || count > MAX_SUPPORTERS {
            return Err(FixtureTrackingCombinationError::Combination {
                source: FixtureCombinationError::Invariant,
            });
        }
        for (index, label) in witness.supporters().enumerate() {
            let entry = decision
                .entries()
                .iter()
                .find(|entry| entry.projection().source_projection().context().source_id() == label)
                .ok_or(FixtureTrackingCombinationError::Combination {
                    source: FixtureCombinationError::Invariant,
                })?;
            let tracking = entry
                .projection()
                .tracking()
                .ok_or(FixtureTrackingCombinationError::MissingTracking { index })?;
            *records
                .get_mut(index)
                .ok_or(FixtureTrackingCombinationError::Combination {
                    source: FixtureCombinationError::Invariant,
                })? = Some(ChartTrackingRecord { entry, tracking });
        }
        Ok(Self { records, count })
    }

    /// Borrow one occupied canonical record without exposing inaccessible padding.
    fn get(&self, index: usize) -> Option<ChartTrackingRecord<'entries, 'chart>> {
        if index >= self.count {
            return None;
        }
        self.records.get(index).copied().flatten()
    }
}

/// Complete immutable relationship retained after mandatory receiving admission.
pub(in crate::combination) struct AdmittedChartSupporters<'input, 'entries, 'chart> {
    /// Borrow the actual decision's owned selection, never a detached replacement.
    selection: &'input FixtureSelection,
    /// Actual witnessed principal slot from that exact decision's batch.
    principal: &'entries FixtureChartInterval<'chart>,
    /// Complete same-entry attachment holder, including excluded participants.
    records: &'input ChartTrackingSupporters<'entries, 'chart>,
    /// Production compatibility view retains only exact original metadata borrows.
    originals: AdmittedOriginals<'entries, 'entries>,
}

impl<'entries, 'chart> AdmittedChartSupporters<'_, 'entries, 'chart> {
    /// Return the complete original agreement population without filtering it.
    pub(in crate::combination) const fn count(&self) -> usize {
        self.originals.count()
    }

    /// Iterate every original witness label in canonical ascending order.
    pub(in crate::combination) fn labels(&self) -> impl ExactSizeIterator<Item = NonZeroU64> + '_ {
        self.originals.labels()
    }

    /// Borrow the shared production compatibility view without reducing its witness.
    pub(in crate::combination) const fn originals(&self) -> &AdmittedOriginals<'entries, 'entries> {
        &self.originals
    }

    /// Borrow the actual decision's selection without recomputing its witness.
    pub(in crate::combination) const fn selection(&self) -> &FixtureSelection {
        self.selection
    }

    /// Borrow the genuine chart principal slot with its own descriptor lifetime.
    pub(in crate::combination) const fn principal(&self) -> &'entries FixtureChartInterval<'chart> {
        self.principal
    }

    /// Borrow only original principal metadata for compatibility and credit.
    pub(in crate::combination) const fn principal_original(
        &self,
    ) -> &'entries FixtureAnchorProjection {
        self.originals.principal_original()
    }

    /// Copy a borrowed same-entry record for one fully admitted original label.
    /// # Errors
    /// Reject a label outside the complete admitted witness without fallback.
    pub(in crate::combination) fn record(
        &self,
        label: NonZeroU64,
    ) -> Result<ChartTrackingRecord<'entries, 'chart>, FixtureCombinationError> {
        (0..self.count())
            .filter_map(|index| self.records.get(index))
            .find(|record| record.original().context().source_id() == label)
            .ok_or(FixtureCombinationError::Invariant)
    }
}

/// Full relationship inputs; raw controls cannot evade the maintained predicate.
type ChartSourceInputs<'input, 'batch, 'entries, 'chart> = (
    &'input FixtureChartPrincipalDecision<'batch, 'entries, 'chart>,
    &'input ChartTrackingSupporters<'entries, 'chart>,
    &'entries FixtureChartInterval<'chart>,
);

/// Mandatory Garde admission of all actual-wrapper and attachment relationships.
#[derive(Validate)]
struct RawChartSourceAdmission<'input, 'batch, 'entries, 'chart> {
    /// Context, actual identity, uniqueness, witness and attachment use one rule.
    #[garde(custom(|value: &ChartSourceInputs<'_, '_, '_, '_>, (): &()| {
        admit_chart_records(value)
            .map_err(|_| garde::Error::new("invalid fixture chart tracking relationship"))
    }))]
    inputs: ChartSourceInputs<'input, 'batch, 'entries, 'chart>,
}

/// Check complete chart membership while retaining each actual wrapper/attachment.
///
/// Count and genuine principal precede per-record checks. Each canonical record
/// checks raw frame/generation/query, actual wrapper address, duplicate label,
/// witness membership, then attachment address. Checked chart construction owns
/// scalar/descriptor invariants, and the real chart batch owns compatibility.
/// # Errors
/// Preserve the first bounded receiving cause; no state or input is changed.
fn admit_chart_records(
    inputs: &ChartSourceInputs<'_, '_, '_, '_>,
) -> Result<(), FixtureCombinationError> {
    let (decision, records, principal) = inputs;
    let witness = decision
        .selection()
        .witnessed()
        .ok_or(FixtureCombinationError::Invariant)?;
    let count = witness.supporters().len();
    if count == 0 || count > MAX_SUPPORTERS || decision.entries().len() > MAX_SUPPORTERS {
        return Err(FixtureCombinationError::Invariant);
    }
    if records.count != count {
        return Err(FixtureCombinationError::MetadataCountMismatch);
    }
    let expected = principal.projection().source_projection();
    let principal_label = expected.context().source_id();
    if !decision
        .principal_entry()
        .is_some_and(|actual| ptr::eq(actual, *principal))
        || decision.principal_label() != Some(principal_label)
        || !witness.supporters().any(|label| label == principal_label)
    {
        return Err(FixtureCombinationError::Invariant);
    }
    for index in 0..count {
        let record = records
            .get(index)
            .ok_or(FixtureCombinationError::Invariant)?;
        let original = record.original();
        let field = if original.context().frame() != expected.context().frame() {
            Some(FixtureCombinationContextField::Frame)
        } else if original.context().generation() != expected.context().generation() {
            Some(FixtureCombinationContextField::MappingGeneration)
        } else if original.projected_at() != expected.projected_at() {
            Some(FixtureCombinationContextField::ProjectedAt)
        } else {
            None
        };
        if let Some(field) = field {
            return Err(FixtureCombinationError::ContextMismatch { index, field });
        }
        if !decision
            .entries()
            .iter()
            .any(|entry| ptr::eq(entry, record.entry))
        {
            return Err(FixtureCombinationError::EntryMismatch { index });
        }
        let label = original.context().source_id();
        if (0..index)
            .filter_map(|previous| records.get(previous))
            .any(|previous| previous.original().context().source_id() == label)
        {
            return Err(FixtureCombinationError::DuplicateUncertainty { index });
        }
        if !witness.supporters().any(|supporter| supporter == label) {
            return Err(FixtureCombinationError::UnexpectedUncertainty { index });
        }
        if !record
            .entry
            .projection()
            .tracking()
            .is_some_and(|actual| ptr::eq(actual, record.tracking))
        {
            return Err(FixtureCombinationError::Invariant);
        }
    }
    if records.records.iter().skip(count).any(Option::is_some) {
        return Err(FixtureCombinationError::Invariant);
    }
    Ok(())
}

/// Admit complete actual chart tracking before scoring, filtering or arithmetic.
///
/// Capacity precedes Garde/report work. Absent principals require an empty holder
/// and yield no token. Selected and retained principals require every original
/// witness record. Mandatory Garde and typed projection use the same immutable
/// predicate in at most two complete passes; reports are discarded.
/// # Errors
/// Preserve capacity, absence consistency, first receiving cause or validation
/// disagreement. No selection, estimate, state, credit or effect is manufactured.
pub(in crate::combination) fn admit_chart_source<'input, 'entries, 'chart>(
    decision: &'input FixtureChartPrincipalDecision<'_, 'entries, 'chart>,
    records: &'input ChartTrackingSupporters<'entries, 'chart>,
) -> Result<
    Option<AdmittedChartSupporters<'input, 'entries, 'chart>>,
    FixtureTrackingCombinationError,
> {
    let complete = || -> Result<_, FixtureCombinationError> {
        if records.count > MAX_SUPPORTERS || decision.entries().len() > MAX_SUPPORTERS {
            return Err(FixtureCombinationError::CapacityExceeded);
        }
        let selected = matches!(
            decision.kind(),
            FixturePrincipalOutcomeKind::ReferenceReady
                | FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
        );
        let Some(principal) = decision.principal_entry() else {
            if records.count != 0 {
                return Err(FixtureCombinationError::MetadataCountMismatch);
            }
            if selected || decision.principal_label().is_some() {
                return Err(FixtureCombinationError::Invariant);
            }
            if records.records.iter().any(Option::is_some) {
                return Err(FixtureCombinationError::Invariant);
            }
            return Ok(None);
        };
        if !selected {
            return Err(FixtureCombinationError::Invariant);
        }
        let raw = RawChartSourceAdmission {
            inputs: (decision, records, principal),
        };
        let validated = raw.validate();
        admit_chart_records(&raw.inputs)?;
        validated.map_err(|_| FixtureCombinationError::Invariant)?;
        let witness = decision
            .selection()
            .witnessed()
            .ok_or(FixtureCombinationError::Invariant)?;
        let count = witness.supporters().len();
        let mut labels = [principal
            .projection()
            .source_projection()
            .context()
            .source_id(); MAX_SUPPORTERS];
        let mut originals = [None; MAX_SUPPORTERS];
        for (index, label) in witness.supporters().enumerate() {
            *labels
                .get_mut(index)
                .ok_or(FixtureCombinationError::Invariant)? = label;
            let record = records
                .get(index)
                .ok_or(FixtureCombinationError::Invariant)?;
            *originals
                .get_mut(index)
                .ok_or(FixtureCombinationError::Invariant)? = Some(OriginalRecord {
                original: record.original(),
                tracking_skew_seconds_per_second: record
                    .tracking()
                    .tracking_skew_seconds_per_second(),
            });
        }
        Ok(Some(AdmittedChartSupporters {
            selection: decision.selection(),
            principal,
            records,
            originals: AdmittedOriginals {
                principal: principal.projection().source_projection(),
                records: originals,
                labels,
                count,
            },
        }))
    };
    complete().map_err(|source| FixtureTrackingCombinationError::Combination { source })
}

/// Independent controls construct real histories, projections, batches and decisions.
#[cfg(test)]
#[path = "receiving_tests.rs"]
mod tests;

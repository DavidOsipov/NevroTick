// SPDX-License-Identifier: GPL-2.0-only

//! Private complete-witness admission and filtered reduction bridge.
//!
//! Every original metadata record is admitted before a caller can choose the
//! smaller mathematical contributor set. Neither a decision nor a witness is
//! manufactured. Fixed capacities bound every scan; no external effects occur.
//! Modified by David Osipov with AI assistance, 3 November 2026: complete
//! tracking records share receiving and reduction with legacy anchor records.

use std::num::NonZeroU64;

use garde::Validate;

use crate::{
    FixtureAnchorProjection, FixturePrincipalDecision, FixturePrincipalOutcomeKind,
    FixtureProjectedInterval,
};

use super::{
    FixtureCombinationError, FixtureCombinedEstimate, FixtureSupporterUncertainty, MAX_SUPPORTERS,
    RecordSource, RecordView, Sums, admit_record_source, admit_records, complete_estimate,
};

/// Immutable compatibility metadata from one admitted actual original entry.
///
/// Construction borrows an admitted entry's exact raw subobject and copies its
/// source-owned checked skew unchanged. The receiving owner establishes actual
/// wrapper identity; its numerical token retains that wrapper and any descriptor.
/// No wrapper, projection or source identity is rebuilt here.
#[derive(Clone, Copy)]
pub(super) struct OriginalRecord<'entry> {
    /// Exact raw subobject of the actual slot established by receiving admission.
    original: &'entry FixtureAnchorProjection,
    /// Checked same-instant skew, preserving the original signed-zero bits.
    tracking_skew_seconds_per_second: f64,
}

impl<'entry> OriginalRecord<'entry> {
    /// Preserve an immutable checked record's actual entry and admitted skew.
    const fn from_uncertainty(record: &FixtureSupporterUncertainty<'entry>) -> Self {
        Self {
            original: record.entry().projection(),
            tracking_skew_seconds_per_second: record.tracking_skew_seconds_per_second(),
        }
    }

    /// Borrow original source metadata without changing raw rate or distance.
    pub(super) const fn original(self) -> &'entry FixtureAnchorProjection {
        self.original
    }

    /// Copy the checked source-owned skew without changing its bit pattern.
    pub(super) const fn tracking_skew_seconds_per_second(self) -> f64 {
        self.tracking_skew_seconds_per_second
    }
}

/// Complete original metadata after actual-witness receiving admission.
///
/// Complete receiving owners construct this immutable token only after mandatory
/// admission. Exact raw borrows retain actual entry lifetimes; their surrounding
/// numerical token retains wrapper/descriptor identity. Fixed sixteen-slot scratch
/// is materialized in the existing occupied-prefix loop, without refitting or
/// heap allocation. This proves no calibration, freshness or external authority.
pub(super) struct AdmittedOriginals<'input, 'entry> {
    /// Exact raw principal subobject borrowed through the genuine decision.
    principal: &'input FixtureAnchorProjection,
    /// Complete checked original metadata, including later-excluded participants.
    records: [Option<OriginalRecord<'entry>>; MAX_SUPPORTERS],
    /// Complete original witness labels in ascending order.
    labels: [NonZeroU64; MAX_SUPPORTERS],
    /// Occupied original-witness prefix in one through sixteen.
    count: usize,
}

impl<'input, 'entry> AdmittedOriginals<'input, 'entry> {
    /// Return the genuine agreement cardinality without filtering it.
    pub(super) const fn count(&self) -> usize {
        self.count
    }

    /// Iterate original witness labels without allocation or reordering.
    pub(super) fn labels(&self) -> impl ExactSizeIterator<Item = NonZeroU64> + '_ {
        self.labels.iter().take(self.count).copied()
    }

    /// Borrow the actual principal's raw metadata without changing provenance.
    pub(super) const fn principal_original(&self) -> &'input FixtureAnchorProjection {
        self.principal
    }

    /// Borrow a complete admitted record by its actual witness label.
    /// # Errors
    /// Reject a label absent from the admitted original records.
    pub(super) fn record(
        &self,
        label: NonZeroU64,
    ) -> Result<OriginalRecord<'entry>, FixtureCombinationError> {
        self.records
            .iter()
            .take(self.count)
            .flatten()
            .copied()
            .find(|record| record.original().context().source_id() == label)
            .ok_or(FixtureCombinationError::Invariant)
    }
}

/// Complete legacy or tracking records with their admitted original metadata.
///
/// The receiving owner constructs both views together after mandatory validation.
/// Compatibility borrows only originals; reduction retains its established mode.
pub(super) struct AdmittedFixtureSupporters<'input, 'entry> {
    /// Actual legacy wrapper remains available for its established reduction output.
    principal: &'input FixtureProjectedInterval,
    /// Unchanged admitted legacy/tracking numerical record source.
    records: RecordSource<'input, 'entry>,
    /// Same complete relationship projected only to immutable raw compatibility data.
    originals: AdmittedOriginals<'input, 'entry>,
}

impl<'input, 'entry> AdmittedFixtureSupporters<'input, 'entry> {
    /// Borrow all original metadata before compatibility filtering or arithmetic.
    pub(super) const fn originals(&self) -> &AdmittedOriginals<'input, 'entry> {
        &self.originals
    }

    /// Return original agreement cardinality independently of filtering.
    pub(super) const fn count(&self) -> usize {
        self.originals.count()
    }

    /// Iterate the complete original witness in its established ascending order.
    pub(super) fn labels(&self) -> impl ExactSizeIterator<Item = NonZeroU64> + '_ {
        self.originals.labels()
    }

    /// Borrow the actual principal for independent raw receiving controls.
    #[cfg(test)]
    pub(super) const fn principal(&self) -> &FixtureProjectedInterval {
        self.principal
    }

    /// Borrow an original checked uncertainty for independent receiving controls.
    /// # Errors
    /// Reject a label outside the complete admitted witness.
    #[cfg(test)]
    pub(super) fn record(
        &self,
        label: NonZeroU64,
    ) -> Result<&FixtureSupporterUncertainty<'entry>, FixtureCombinationError> {
        self.records.find(label).map(|record| record.uncertainty)
    }

    /// Borrow one admitted record under the same original-witness phase mode.
    /// # Errors
    /// Reject contradictory private label coverage without fallback or effects.
    fn values(&self, label: NonZeroU64) -> Result<RecordView<'_, 'entry>, FixtureCombinationError> {
        self.records.find(label)
    }
}

/// One relationship whose checked parts may still be associated incorrectly.
type SupporterAdmissionInputs<'input, 'entry> = (
    &'input FixturePrincipalDecision<'entry>,
    &'input [FixtureSupporterUncertainty<'entry>],
    &'input FixtureProjectedInterval,
);

/// Mandatory Garde validation through the existing complete receiving owner.
#[derive(Validate)]
struct RawSupporterAdmission<'input, 'entry> {
    /// Count, identity, context and every membership share one maintained rule.
    #[garde(custom(|value: &SupporterAdmissionInputs<'_, '_>, (): &()| {
        admit_records(value.0, value.1, value.2)
            .map_err(|_| garde::Error::new("invalid fixture supporter relationship"))
    }))]
    inputs: SupporterAdmissionInputs<'input, 'entry>,
}

/// Same complete original relationship expressed through the private source view.
type SourceAdmissionInputs<'input, 'entry> = (
    &'input FixturePrincipalDecision<'entry>,
    RecordSource<'input, 'entry>,
    &'input FixtureProjectedInterval,
);

/// Mandatory Garde receiving for the closed tracking source representation.
#[derive(Validate)]
struct RawSourceAdmission<'input, 'entry> {
    /// All transformed associations share the same complete receiving predicate.
    #[garde(custom(|value: &SourceAdmissionInputs<'_, '_>, (): &()| {
        admit_record_source(value.0, value.1, value.2)
            .map_err(|_| garde::Error::new("invalid fixture tracking relationship"))
    }))]
    inputs: SourceAdmissionInputs<'input, 'entry>,
}

/// Admit every genuine witnessed record before any compatibility filtering.
///
/// Bounds precede validation/report work. Garde and typed-cause projection use
/// the same immutable predicate, making at most two complete bounded passes.
/// An absent principal requires empty metadata and yields no reduction token.
/// # Errors
/// Preserve the existing first receiving cause; retain no raw validator report.
pub(super) fn admit_lifecycle_supporters<'input, 'entry>(
    decision: &'input FixturePrincipalDecision<'entry>,
    records: &'input [FixtureSupporterUncertainty<'entry>],
) -> Result<Option<AdmittedFixtureSupporters<'input, 'entry>>, FixtureCombinationError> {
    admit_lifecycle_source(decision, RecordSource::Legacy(records))
}

/// Admit every original witnessed record under one immutable phase interpretation.
///
/// Legacy raw-control shape and predicates remain intact. Tracking uses the same
/// predicate through a mandatory Garde source-view candidate. No compatibility
/// filtering, penalty effects or mutable result escape precedes complete admission.
/// # Errors
/// Preserve capacity, absent-principal, actual-entry and validation failures.
pub(super) fn admit_lifecycle_source<'input, 'entry>(
    decision: &'input FixturePrincipalDecision<'entry>,
    records: RecordSource<'input, 'entry>,
) -> Result<Option<AdmittedFixtureSupporters<'input, 'entry>>, FixtureCombinationError> {
    if records.len() > MAX_SUPPORTERS {
        return Err(FixtureCombinationError::CapacityExceeded);
    }
    let selected = matches!(
        decision.kind(),
        FixturePrincipalOutcomeKind::ReferenceReady
            | FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    );
    let Some(principal) = decision.principal_entry() else {
        if records.len() != 0 {
            return Err(FixtureCombinationError::MetadataCountMismatch);
        }
        if selected || decision.principal_label().is_some() {
            return Err(FixtureCombinationError::Invariant);
        }
        return Ok(None);
    };
    if !selected {
        return Err(FixtureCombinationError::Invariant);
    }
    let validated = match records {
        RecordSource::Legacy(values) => RawSupporterAdmission {
            inputs: (decision, values, principal),
        }
        .validate(),
        RecordSource::Tracking(_) => RawSourceAdmission {
            inputs: (decision, records, principal),
        }
        .validate(),
    };
    admit_record_source(decision, records, principal)?;
    validated.map_err(|_| FixtureCombinationError::Invariant)?;
    let witness = decision
        .selection()
        .witnessed()
        .ok_or(FixtureCombinationError::Invariant)?;
    let count = witness.supporters().len();
    let mut labels = [principal.projection().context().source_id(); MAX_SUPPORTERS];
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
            .ok_or(FixtureCombinationError::Invariant)? =
            Some(OriginalRecord::from_uncertainty(record.uncertainty));
    }
    Ok(Some(AdmittedFixtureSupporters {
        principal,
        records,
        originals: AdmittedOriginals {
            principal: principal.projection(),
            records: originals,
            labels,
            count,
        },
    }))
}

/// Original positions and a fixed candidate subset, without naked label inputs.
type ContributorInputs<'mask, 'input, 'entry> = (
    &'mask AdmittedFixtureSupporters<'input, 'entry>,
    [bool; MAX_SUPPORTERS],
);

/// Check all mask positions and derive the contributing occupied count.
/// # Errors
/// Reject empty masks, missing principal or any flag outside the actual witness.
fn contributor_count(
    inputs: &ContributorInputs<'_, '_, '_>,
) -> Result<usize, FixtureCombinationError> {
    let (admitted, included) = inputs;
    if admitted.count() == 0 || admitted.count() > MAX_SUPPORTERS {
        return Err(FixtureCombinationError::Invariant);
    }
    let principal = admitted
        .originals
        .principal_original()
        .context()
        .source_id();
    let mut principal_present = false;
    let mut count = 0_usize;
    for (index, &is_included) in included.iter().enumerate() {
        if !is_included {
            continue;
        }
        if index >= admitted.count() {
            return Err(FixtureCombinationError::Invariant);
        }
        let label = admitted
            .originals
            .labels
            .get(index)
            .ok_or(FixtureCombinationError::Invariant)?;
        principal_present |= *label == principal;
        count = count
            .checked_add(1)
            .ok_or(FixtureCombinationError::Invariant)?;
    }
    if count == 0 || !principal_present {
        return Err(FixtureCombinationError::Invariant);
    }
    Ok(count)
}

/// One mandatory rule covers nonempty subset and principal inclusion together.
#[derive(Validate)]
struct RawContributors<'mask, 'input, 'entry> {
    /// Boolean positions retain the original admitted witness association.
    #[garde(custom(|value: &ContributorInputs<'_, '_, '_>, (): &()| {
        contributor_count(value).map(|_| ())
            .map_err(|_| garde::Error::new("invalid fixture contributor mask"))
    }))]
    inputs: ContributorInputs<'mask, 'input, 'entry>,
}

/// Reduce original admitted contributors without manufacturing a new witness.
///
/// Validate the complete fixed mask before arithmetic. Only a genuine original
/// singleton uses exact copy. A larger witness filtered to one still reaches
/// ordinary denominator guards and all three roots. Preserve original principal
/// centering, metadata and ascending labels. Successful reduction attempts at
/// most 18n+5 binary operations/three roots plus two sixteen-position mask scans.
/// # Errors
/// Preserve mask, original-record, denominator, arithmetic and output failures.
pub(super) fn combine_lifecycle_supporters(
    admitted: &AdmittedFixtureSupporters<'_, '_>,
    included: [bool; MAX_SUPPORTERS],
) -> Result<FixtureCombinedEstimate, FixtureCombinationError> {
    let raw = RawContributors {
        inputs: (admitted, included),
    };
    let validated = raw.validate();
    let count = contributor_count(&raw.inputs)?;
    validated.map_err(|_| FixtureCombinationError::Invariant)?;
    let principal_label = admitted
        .originals
        .principal_original()
        .context()
        .source_id();
    let mut labels = [principal_label; MAX_SUPPORTERS];
    let mut occupied = 0_usize;
    let mut sums = Sums::empty();
    let principal_phase = admitted.values(principal_label)?.phase;
    for (index, label) in admitted.labels().enumerate() {
        if !included
            .get(index)
            .copied()
            .ok_or(FixtureCombinationError::Invariant)?
        {
            continue;
        }
        *labels
            .get_mut(occupied)
            .ok_or(FixtureCombinationError::Invariant)? = label;
        occupied = occupied
            .checked_add(1)
            .ok_or(FixtureCombinationError::Invariant)?;
        if admitted.count() > 1 {
            sums = sums.with_supporter(admitted.values(label)?, principal_phase, index)?;
        }
    }
    if occupied != count {
        return Err(FixtureCombinationError::Invariant);
    }
    let quantities = if admitted.count() == 1 {
        admitted.values(principal_label)?.singleton()
    } else {
        sums.finish()?
    };
    complete_estimate(admitted.principal, labels, count, &quantities)
}

/// Private raw-receiver controls use genuine public producer values.
#[cfg(test)]
#[path = "../../tests/private/lifecycle_admission.rs"]
mod receiving_tests;

/// Complete chart receiving for the genuine chart lifecycle consumer.
#[path = "chart/receiving.rs"]
pub(super) mod chart_receiving;

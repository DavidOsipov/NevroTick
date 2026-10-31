// SPDX-License-Identifier: GPL-2.0-only

//! Private complete-witness admission and filtered reduction bridge.
//!
//! Every original metadata record is admitted before a caller can choose the
//! smaller mathematical contributor set. Neither a decision nor a witness is
//! manufactured. Fixed capacities bound every scan; no external effects occur.

use std::num::NonZeroU64;

use garde::Validate;

use crate::{FixturePrincipalDecision, FixturePrincipalOutcomeKind, FixtureProjectedInterval};

use super::{
    FixtureCombinationError, FixtureCombinedEstimate, FixtureSupporterUncertainty, MAX_SUPPORTERS,
    Sums, admit_records, complete_estimate, singleton_quantities, supporter_record,
};

/// Complete original metadata after actual-witness receiving admission.
///
/// Only this module constructs the immutable token. Its record and principal
/// borrows retain original identities; this proves no physical calibration,
/// authentication, freshness or external authority.
pub(super) struct AdmittedFixtureSupporters<'input, 'entry> {
    /// Actual original principal borrowed through the genuine decision.
    principal: &'input FixtureProjectedInterval,
    /// Every checked record for the original witness, including excluded records.
    records: &'input [FixtureSupporterUncertainty<'entry>],
    /// Complete original witness labels in ascending order.
    labels: [NonZeroU64; MAX_SUPPORTERS],
    /// Occupied original-witness prefix in one through sixteen.
    count: usize,
}

impl<'entry> AdmittedFixtureSupporters<'_, 'entry> {
    /// Return the genuine agreement cardinality without filtering it.
    pub(super) const fn count(&self) -> usize {
        self.count
    }

    /// Iterate original witness labels without allocation or reordering.
    pub(super) fn labels(&self) -> impl ExactSizeIterator<Item = NonZeroU64> + '_ {
        self.labels.iter().take(self.count).copied()
    }

    /// Borrow the original principal without changing provenance.
    pub(super) const fn principal(&self) -> &FixtureProjectedInterval {
        self.principal
    }

    /// Borrow a complete admitted record by its actual witness label.
    /// # Errors
    /// Reject a label absent from the admitted original records.
    pub(super) fn record(
        &self,
        label: NonZeroU64,
    ) -> Result<&FixtureSupporterUncertainty<'entry>, FixtureCombinationError> {
        supporter_record(self.records, label)
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
    if records.len() > MAX_SUPPORTERS {
        return Err(FixtureCombinationError::CapacityExceeded);
    }
    let selected = matches!(
        decision.kind(),
        FixturePrincipalOutcomeKind::ReferenceReady
            | FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    );
    let Some(principal) = decision.principal_entry() else {
        if !records.is_empty() {
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
    let raw = RawSupporterAdmission {
        inputs: (decision, records, principal),
    };
    let validated = raw.validate();
    admit_records(decision, records, principal)?;
    validated.map_err(|_| FixtureCombinationError::Invariant)?;
    let witness = decision
        .selection()
        .witnessed()
        .ok_or(FixtureCombinationError::Invariant)?;
    let count = witness.supporters().len();
    let mut labels = [principal.projection().context().source_id(); MAX_SUPPORTERS];
    for (index, label) in witness.supporters().enumerate() {
        *labels
            .get_mut(index)
            .ok_or(FixtureCombinationError::Invariant)? = label;
    }
    Ok(Some(AdmittedFixtureSupporters {
        principal,
        records,
        labels,
        count,
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
    if admitted.count == 0 || admitted.count > MAX_SUPPORTERS {
        return Err(FixtureCombinationError::Invariant);
    }
    let principal = admitted.principal.projection().context().source_id();
    let mut principal_present = false;
    let mut count = 0_usize;
    for (index, &is_included) in included.iter().enumerate() {
        if !is_included {
            continue;
        }
        if index >= admitted.count {
            return Err(FixtureCombinationError::Invariant);
        }
        let label = admitted
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
    let principal_label = admitted.principal.projection().context().source_id();
    let mut labels = [principal_label; MAX_SUPPORTERS];
    let mut occupied = 0_usize;
    let mut sums = Sums::empty();
    let principal_phase = admitted.principal.projection().center_seconds();
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
        if admitted.count > 1 {
            sums = sums.with_supporter(admitted.record(label)?, principal_phase, index)?;
        }
    }
    if occupied != count {
        return Err(FixtureCombinationError::Invariant);
    }
    let quantities = if admitted.count == 1 {
        singleton_quantities(admitted.record(principal_label)?)
    } else {
        sums.finish()?
    };
    complete_estimate(admitted.principal, labels, count, &quantities)
}

/// Private raw-receiver controls use genuine public producer values.
#[cfg(test)]
#[path = "../../tests/private/lifecycle_admission.rs"]
mod receiving_tests;

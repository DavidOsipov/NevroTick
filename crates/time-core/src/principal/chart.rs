// SPDX-License-Identifier: GPL-2.0-only
//! Retained raw-source principal receiving through explicit mathematical charts.
//!
//! One checked chart batch supplies both geometry and the original source
//! metadata. The parent owns observation admission, score/credit transitions and
//! the sole state commit; this adapter retains the actual chart selection in its
//! output. An explicitly expected target view may change without installing a
//! source-generation transition, reseeding credit or acknowledging a clock effect.

use std::{fmt, num::NonZeroU64};

use garde::Validate;

use super::{
    FixturePrincipalError, FixturePrincipalOutcomeKind, FixturePrincipalState,
    FixturePrincipalStratum, MAX_PRINCIPAL_SOURCES, ReceiverEntries, ReceiverInput, apply_selection,
    decision_index, prepare_transition,
};
use crate::{
    FixtureAffineChart, FixtureChartBatch, FixtureChartInterval, FixtureChartMismatchKind,
    FixtureChartSelection, FixtureSelection, chart_batch::chart_descriptor_mismatch,
};

/// Bounded chart expectation failure or the actual retained principal cause.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum FixtureChartPrincipalError {
    /// First descriptor mismatch in frames, domain, displacement and rate order.
    ChartMismatch {
        /// Closed role; no descriptor, source label or scalar is retained.
        kind: FixtureChartMismatchKind,
    },
    /// Actual original-source admission, selection or transition failure.
    Principal {
        /// Typed bounded cause, including mandatory-validation disagreement.
        source: FixturePrincipalError,
    },
}

impl fmt::Debug for FixtureChartPrincipalError {
    /// Redact all input metadata with one fixed type description.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureChartPrincipalError { .. }")
    }
}

impl fmt::Display for FixtureChartPrincipalError {
    /// Display a fixed family code without formatting supplied values or causes.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::ChartMismatch { .. } => "fixture chart principal: chart mismatch",
            Self::Principal { .. } => "fixture chart principal: principal rejected",
        })
    }
}

impl std::error::Error for FixtureChartPrincipalError {
    /// Preserve only an actual principal cause; a descriptor mismatch has none.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ChartMismatch { .. } => None,
            Self::Principal { source } => Some(source),
        }
    }
}

impl From<FixturePrincipalError> for FixtureChartPrincipalError {
    /// Wrap the unchanged bounded cause without state, formatting or effects.
    fn from(source: FixturePrincipalError) -> Self {
        Self::Principal { source }
    }
}

/// One checked actual descriptor with a mandatory expected-value relationship.
#[derive(Validate)]
#[garde(context(FixtureAffineChart))]
struct ChartReceiverCandidate<'chart> {
    /// The shared predicate covers all complete descriptor values and scalar bits.
    #[garde(custom(chart_receiver_rule))]
    actual: &'chart FixtureAffineChart,
}

/// Apply the common descriptor predicate inside mandatory Garde admission.
/// # Errors
/// Return a fixed validation failure without retaining either checked descriptor.
fn chart_receiver_rule(
    actual: &FixtureAffineChart,
    expected: &FixtureAffineChart,
) -> garde::Result {
    if chart_descriptor_mismatch(expected, actual).is_some() {
        Err(garde::Error::new("incompatible principal chart descriptor"))
    } else {
        Ok(())
    }
}

/// Admit an explicitly expected descriptor by complete checked values and bits.
///
/// Immutable chart construction already owns scalar/domain validity. This owner
/// consumes mandatory relationship validation and grants no current authority.
/// # Errors
/// Preserve the first descriptor mismatch or validation disagreement; change no state.
fn admit_chart(
    expected: &FixtureAffineChart,
    actual: &FixtureAffineChart,
) -> Result<(), FixtureChartPrincipalError> {
    let candidate = ChartReceiverCandidate { actual };
    let validated = candidate.validate_with(expected);
    if let Some(kind) = chart_descriptor_mismatch(expected, actual) {
        return Err(FixtureChartPrincipalError::ChartMismatch { kind });
    }
    validated.map_err(|_| FixturePrincipalError::Invariant.into())
}

/// Immutable principal result owning the actual chart-bound selection.
///
/// The selected principal is an original entry of that exact batch and one of
/// its witnessed supporters. Every wrapper retains its own genuine descriptor,
/// raw-source provenance, transformed values and separate arithmetic bounds.
/// No public constructor, mutable access or conversion to legacy entries exists.
/// This result supplies inert estimate credit, not freshness or a clock effect.
pub struct FixtureChartPrincipalDecision<'batch, 'entries, 'chart> {
    /// Actual owned output from the retained owner's fixed-minimum batch selection.
    selected: FixtureChartSelection<'batch, 'entries, 'chart>,
    /// Checked index of the optional original principal entry in the actual batch.
    principal_index: Option<usize>,
    /// Label identifying that same optional original witnessed entry.
    principal_label: Option<NonZeroU64>,
    /// Completed parent score/credit transition, with no external effect.
    kind: FixturePrincipalOutcomeKind,
    /// Sole genuinely newer original observation, absent for pure reevaluation.
    updated_source: Option<NonZeroU64>,
}

impl fmt::Debug for FixtureChartPrincipalDecision<'_, '_, '_> {
    /// Redact labels, descriptors, values, observation times and fixture credit.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureChartPrincipalDecision { .. }")
    }
}

impl<'batch, 'entries, 'chart> FixtureChartPrincipalDecision<'batch, 'entries, 'chart> {
    /// Copy the completed inert outcome without creating another estimate.
    #[must_use]
    pub const fn kind(&self) -> FixturePrincipalOutcomeKind {
        self.kind
    }

    /// Borrow the real fixed-minimum interval selection without replacement.
    #[must_use]
    pub const fn selection(&self) -> &FixtureSelection {
        self.selected.selection()
    }

    /// Borrow the actual chart selection together with its exact batch provenance.
    #[must_use]
    pub const fn chart_selection(&self) -> &FixtureChartSelection<'batch, 'entries, 'chart> {
        &self.selected
    }

    /// Borrow the exact selected chart batch, never a reconstructed substitute.
    #[must_use]
    pub const fn batch(&self) -> &'batch FixtureChartBatch<'entries, 'chart> {
        self.selected.batch()
    }

    /// Borrow every original chart entry in its complete supplied order.
    #[must_use]
    pub const fn entries(&self) -> &'entries [FixtureChartInterval<'chart>] {
        self.selected.batch().entries()
    }

    /// Copy the selected original source assertion without real-source authority.
    #[must_use]
    pub const fn principal_label(&self) -> Option<NonZeroU64> {
        self.principal_label
    }

    /// Borrow the actual original principal entry, absent for an unselected result.
    #[must_use]
    pub fn principal_entry(&self) -> Option<&'entries FixtureChartInterval<'chart>> {
        self.principal_index
            .and_then(|index| self.entries().get(index))
    }

    /// Observe consumed fixture estimate credit without implying an applied effect.
    #[must_use]
    pub const fn reference_ready(&self) -> bool {
        matches!(self.kind, FixturePrincipalOutcomeKind::ReferenceReady)
    }

    /// Copy the derived original-source event, without treating chart time as one.
    #[must_use]
    pub const fn updated_source(&self) -> Option<NonZeroU64> {
        self.updated_source
    }
}

/// Complete one chart-specific result using the shared parent output relation.
/// # Errors
/// Reject an inconsistent outcome or original principal association before commit.
fn complete_chart_decision<'batch, 'entries, 'chart>(
    selected: FixtureChartSelection<'batch, 'entries, 'chart>,
    principal_label: Option<NonZeroU64>,
    kind: FixturePrincipalOutcomeKind,
    updated_source: Option<NonZeroU64>,
) -> Result<FixtureChartPrincipalDecision<'batch, 'entries, 'chart>, FixturePrincipalError> {
    let principal_index = decision_index(
        ReceiverEntries::Chart(selected.batch().entries()),
        selected.selection(),
        principal_label,
        kind,
    )?;
    Ok(FixtureChartPrincipalDecision {
        selected,
        principal_index,
        principal_label,
        kind,
        updated_source,
    })
}

impl FixturePrincipalState {
    /// Advance this retained raw-source owner through one explicit chart view.
    ///
    /// Capacity precedes the mandatory complete expected-descriptor relationship.
    /// The shared receiver then checks original source frame/generation, metadata,
    /// nondecreasing query and genuine observation evolution. Scoring uses original
    /// root distance, while selection uses the chart's bound-consuming geometry and
    /// this owner's unchanged minimum. A private clone stages the existing state
    /// transition; the actual chart decision is completed before the sole commit.
    ///
    /// Equal descriptors at different addresses are accepted. Another explicitly
    /// expected mathematical target view may be used later over the same retained
    /// raw origin; target tags never replace this owner's source identity. Chart
    /// values do not grant new observation credit, migrate an installed generation,
    /// certify freshness or acknowledge an achieved clock correction.
    /// # Errors
    /// Preserve metadata-capacity, expected-descriptor, then original receiver/
    /// selector/arithmetic causes. Every error preserves all prior state bits and
    /// the immutable batch. No network, history, persistence or clock effect occurs.
    pub fn advance_chart<'batch, 'entries, 'chart>(
        &mut self,
        expected_chart: &FixtureAffineChart,
        batch: &'batch FixtureChartBatch<'entries, 'chart>,
        strata: &[FixturePrincipalStratum],
    ) -> Result<FixtureChartPrincipalDecision<'batch, 'entries, 'chart>, FixtureChartPrincipalError> {
        if strata.len() > MAX_PRINCIPAL_SOURCES {
            return Err(FixturePrincipalError::MetadataCapacityExceeded.into());
        }
        admit_chart(expected_chart, batch.chart())?;
        let frames = batch.chart().frames();
        let input = ReceiverInput {
            entries: ReceiverEntries::Chart(batch.entries()),
            strata,
            frame: frames.source_frame(),
            mapping_generation: frames.source_generation(),
            at: batch.projected_at(),
            seed: false,
        };
        let (mut staged, updated_source) = prepare_transition(input, self)?;
        let selected = batch
            .select(staged.minimum)
            .map_err(|source| FixturePrincipalError::Selection { source })?;
        let kind = apply_selection(&mut staged, selected.selection(), updated_source)?;
        let decision = complete_chart_decision(selected, staged.principal, kind, updated_source)?;
        *self = staged;
        Ok(decision)
    }
}

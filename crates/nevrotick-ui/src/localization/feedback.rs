//! Retained typed GUI feedback, independent of any selected catalog revision.
//!
//! A closed code-owned slot and a privately checked `UiText` request are the only
//! inputs. No translated string is retained as presenter state or interpreted to
//! recover an action. Preparing one immutable snapshot renders at most 29 bounded
//! values into an unpublished candidate; rejection changes neither the retained
//! requests nor any window, catalog, service, preference or editor draft.
#![forbid(unsafe_code)]

use crate::localization::presentation::UiText;
use crate::localization::{Error, PlainText, Snapshot};

/// Exact code-owned feedback capacity, independent of catalog input or counts.
const SLOT_COUNT: usize = 29;

/// Closed presentation destinations; catalogs cannot supply slot identities.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FeedbackSlot {
    /// Current source editor rejection.
    EditorError,
    /// Current source edit or review outcome.
    EditorFeedback,
    /// Current raw interval rejection.
    IntervalError,
    /// Current accepted interval or cancel feedback.
    IntervalFeedback,
    /// Current process-local preference rejection.
    PreferenceError,
    /// Current process-local preference outcome.
    PreferenceFeedback,
    /// Current date-recovery input rejection.
    RecoveryError,
    /// Current date-recovery draft outcome.
    RecoveryFeedback,
    /// Checked return-destination wording for date recovery.
    RecoveryBackLabel,
    /// Runtime-observation-specific quit headline.
    QuitHeadline,
    /// Runtime-observation-specific quit consequence.
    QuitConsequence,
    /// Current shell request outcome.
    ShellFeedback,
    /// Current service-management request outcome.
    ServiceFeedback,
    /// Current notification request or availability outcome.
    NotificationFeedback,
    /// Current diagnostic-preview outcome.
    DiagnosticFeedback,
    /// Current policy-preview rejection.
    PolicyError,
    /// Current source query admission rejection.
    SourceQueryError,
    /// Typed accepted check interval including positive whole-minute facts.
    AcceptedIntervalLabel,
    /// Current status request outcome.
    StatusFeedback,
    /// Current tray request or availability outcome.
    TrayFeedback,
    /// Current typed status headline.
    StatusHeadline,
    /// Current typed status consequence.
    StatusConsequence,
    /// Current typed status profile heading.
    StatusProfileLabel,
    /// Current typed status profile description.
    StatusProfile,
    /// Current typed status authentication fact.
    StatusAuthentication,
    /// Current typed status agreement fact.
    StatusAgreement,
    /// Current typed status freshness fact.
    StatusFreshness,
    /// Current typed status check-action wording.
    StatusCheckLabel,
    /// Current typed status remedy-action wording.
    StatusRemedyLabel,
}

impl FeedbackSlot {
    /// Map each closed destination to one distinct in-bounds private array slot.
    ///
    /// This exhaustive mapping admits no integer conversion or external index.
    const fn index(self) -> usize {
        match self {
            Self::EditorError => 0,
            Self::EditorFeedback => 1,
            Self::IntervalError => 2,
            Self::IntervalFeedback => 3,
            Self::PreferenceError => 4,
            Self::PreferenceFeedback => 5,
            Self::RecoveryError => 6,
            Self::RecoveryFeedback => 7,
            Self::RecoveryBackLabel => 8,
            Self::QuitHeadline => 9,
            Self::QuitConsequence => 10,
            Self::ShellFeedback => 11,
            Self::ServiceFeedback => 12,
            Self::NotificationFeedback => 13,
            Self::DiagnosticFeedback => 14,
            Self::PolicyError => 15,
            Self::SourceQueryError => 16,
            Self::AcceptedIntervalLabel => 17,
            Self::StatusFeedback => 18,
            Self::TrayFeedback => 19,
            Self::StatusHeadline => 20,
            Self::StatusConsequence => 21,
            Self::StatusProfileLabel => 22,
            Self::StatusProfile => 23,
            Self::StatusAuthentication => 24,
            Self::StatusAgreement => 25,
            Self::StatusFreshness => 26,
            Self::StatusCheckLabel => 27,
            Self::StatusRemedyLabel => 28,
        }
    }
}

/// One bounded owner of typed requests, with absent feedback distinct from text.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiFeedbackSet {
    /// Only checked requests; a `None` slot has no message to translate.
    requests: [Option<UiText>; SLOT_COUNT],
}

impl UiFeedbackSet {
    /// Copy a checked retained request without rendering or changing its meaning.
    #[must_use]
    pub(crate) fn get(&self, slot: FeedbackSlot) -> Option<UiText> {
        self.requests[slot.index()]
    }

    /// Replace or clear exactly one typed destination, preserving all other slots.
    ///
    /// The closed mapping establishes array bounds. No raw words, integer slot,
    /// catalog authority, external effect or fallible transition enters here.
    pub(crate) fn replace(&mut self, slot: FeedbackSlot, request: Option<UiText>) {
        self.requests[slot.index()] = request;
    }

    /// Render every retained request from the same immutable catalog snapshot.
    ///
    /// The fixed candidate remains private until every present request renders.
    /// An absent request stays `None`, rather than looking up an empty message.
    /// The operation is read-only with respect to retained presenter state and
    /// publishes no partial values to a window or other consumer.
    ///
    /// # Errors
    /// Returns the first protected-contract, argument or bounded-render error,
    /// dropping the entire prepared candidate and preserving all prior requests.
    pub(crate) fn prepare(&self, snapshot: &Snapshot<'_>) -> Result<PreparedFeedback, Error> {
        let mut values = std::array::from_fn(|_| None);
        for (value, request) in values.iter_mut().zip(&self.requests) {
            *value = request
                .map(|request| request.render(snapshot))
                .transpose()?;
        }
        Ok(PreparedFeedback { values })
    }
}

/// Fully prepared immutable feedback projection; it grants no publication right.
#[derive(Debug)]
pub(crate) struct PreparedFeedback {
    /// At most 29 independently bounded literal values from one borrowed snapshot.
    values: [Option<PlainText>; SLOT_COUNT],
}

impl PreparedFeedback {
    /// Borrow one prepared literal value; an absent slot remains absent.
    #[must_use]
    pub(crate) fn get(&self, slot: FeedbackSlot) -> Option<&PlainText> {
        self.values[slot.index()].as_ref()
    }
}

#[cfg(test)]
/// Source controls for typed slot isolation and empty-state preservation.
mod tests {
    use super::{FeedbackSlot, SLOT_COUNT, UiFeedbackSet};
    use crate::localization::presentation::UiText;
    use crate::localization::{CatalogStore, Error, MessageId};
    use crate::presentation::UiError;

    /// Independent complete inventory used to detect mapping omissions or aliasing.
    const SLOTS: [FeedbackSlot; SLOT_COUNT] = [
        FeedbackSlot::EditorError,
        FeedbackSlot::EditorFeedback,
        FeedbackSlot::IntervalError,
        FeedbackSlot::IntervalFeedback,
        FeedbackSlot::PreferenceError,
        FeedbackSlot::PreferenceFeedback,
        FeedbackSlot::RecoveryError,
        FeedbackSlot::RecoveryFeedback,
        FeedbackSlot::RecoveryBackLabel,
        FeedbackSlot::QuitHeadline,
        FeedbackSlot::QuitConsequence,
        FeedbackSlot::ShellFeedback,
        FeedbackSlot::ServiceFeedback,
        FeedbackSlot::NotificationFeedback,
        FeedbackSlot::DiagnosticFeedback,
        FeedbackSlot::PolicyError,
        FeedbackSlot::SourceQueryError,
        FeedbackSlot::AcceptedIntervalLabel,
        FeedbackSlot::StatusFeedback,
        FeedbackSlot::TrayFeedback,
        FeedbackSlot::StatusHeadline,
        FeedbackSlot::StatusConsequence,
        FeedbackSlot::StatusProfileLabel,
        FeedbackSlot::StatusProfile,
        FeedbackSlot::StatusAuthentication,
        FeedbackSlot::StatusAgreement,
        FeedbackSlot::StatusFreshness,
        FeedbackSlot::StatusCheckLabel,
        FeedbackSlot::StatusRemedyLabel,
    ];

    /// Admit the existing exact fixed fixture through unit-only mechanism authority.
    ///
    /// No release grant is created; ordinary-library APIs remain closed.
    ///
    /// # Errors
    /// Propagates unexpected rejection of the exact protected unit fixture.
    fn fixture_store() -> Result<CatalogStore, Error> {
        let bytes = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/test-only-english.cbor"
        ));
        CatalogStore::bootstrap_with_authority(
            bytes,
            crate::localization::approvals::Authority::SYNTHETIC_FIXTURE,
        )
    }

    /// Each typed destination maps to exactly one finite slot and starts absent.
    ///
    /// # Panics
    /// Fails if the mapping aliases, exceeds its bound, omits a slot or starts set.
    #[test]
    fn closed_slots_are_distinct_bounded_and_initially_absent() {
        let feedback = UiFeedbackSet::default();
        let mut seen = [false; SLOT_COUNT];
        for slot in SLOTS {
            assert!(!seen[slot.index()]);
            seen[slot.index()] = true;
            assert!(feedback.get(slot).is_none());
        }
        assert!(seen.into_iter().all(core::convert::identity));
    }

    /// Replacing and clearing any slot preserves all neighboring typed requests.
    ///
    /// # Panics
    /// Fails if a single destination update changes another destination or meaning.
    #[test]
    fn replace_and_clear_touch_only_the_named_slot() {
        let first = UiText::ui_error(UiError::PageUnavailable);
        let second = UiText::ui_error(UiError::Model(crate::model::ModelError::InvalidHost));
        for changed in SLOTS {
            let mut feedback = UiFeedbackSet::default();
            for slot in SLOTS {
                feedback.replace(slot, Some(first));
            }
            feedback.replace(changed, Some(second));
            for slot in SLOTS {
                assert_eq!(
                    feedback.get(slot),
                    Some(if slot == changed { second } else { first })
                );
            }
            feedback.replace(changed, None);
            for slot in SLOTS {
                assert_eq!(
                    feedback.get(slot),
                    if slot == changed { None } else { Some(first) }
                );
            }
        }
    }
    /// Empty feedback does not become a lookup or an empty translated message.
    ///
    /// # Errors
    /// Propagates unexpected fixture admission or preparation rejection.
    ///
    /// # Panics
    /// Fails if preparation manufactures any message or mutates retained state.
    #[test]
    fn preparing_absent_feedback_preserves_absence_and_owner() -> Result<(), Error> {
        let store = fixture_store()?;
        let feedback = UiFeedbackSet::default();
        let before = feedback.clone();
        let prepared = feedback.prepare(&store.snapshot())?;
        for slot in SLOTS {
            assert!(prepared.get(slot).is_none());
        }
        assert_eq!(feedback, before);
        Ok(())
    }

    /// Present requests use the same checked snapshot and clearing stays local.
    ///
    /// # Errors
    /// Propagates unexpected fixture admission, typed interval or rendering failure.
    ///
    /// # Panics
    /// Fails if projection changes text, loses typed minute facts or clears a neighbor.
    #[test]
    fn preparing_typed_neighbors_uses_one_snapshot_and_preserves_requests() -> Result<(), Error> {
        let store = fixture_store()?;
        let snapshot = store.snapshot();
        let first = UiText::fixed(MessageId::FIXTURE_SAMPLE_LABEL);
        let interval = UiText::interval(crate::preferences::CheckInterval::TwoMinutes)?;
        let mut feedback = UiFeedbackSet::default();
        feedback.replace(FeedbackSlot::EditorFeedback, Some(first));
        feedback.replace(FeedbackSlot::AcceptedIntervalLabel, Some(interval));
        let before = feedback.clone();
        let prepared = feedback.prepare(&snapshot)?;
        assert_eq!(
            prepared
                .get(FeedbackSlot::EditorFeedback)
                .map(crate::localization::PlainText::as_str),
            Some(snapshot.text(MessageId::FIXTURE_SAMPLE_LABEL)?.as_str()),
        );
        assert_eq!(
            prepared
                .get(FeedbackSlot::AcceptedIntervalLabel)
                .map(crate::localization::PlainText::as_str),
            Some(interval.render(&snapshot)?.as_str()),
        );
        assert!(prepared.get(FeedbackSlot::EditorError).is_none());
        assert_eq!(feedback, before);
        feedback.replace(FeedbackSlot::EditorFeedback, None);
        let cleared = feedback.prepare(&snapshot)?;
        assert!(cleared.get(FeedbackSlot::EditorFeedback).is_none());
        assert_eq!(
            feedback.get(FeedbackSlot::AcceptedIntervalLabel),
            Some(interval)
        );
        assert_eq!(
            cleared
                .get(FeedbackSlot::AcceptedIntervalLabel)
                .map(crate::localization::PlainText::as_str),
            prepared
                .get(FeedbackSlot::AcceptedIntervalLabel)
                .map(crate::localization::PlainText::as_str),
        );
        Ok(())
    }

    /// A late wrong-kind request rejects the entire unpublished candidate.
    ///
    /// # Errors
    /// Propagates unexpected fixture admission or healthy preparation failure.
    ///
    /// # Panics
    /// Fails if mismatch renders, retained requests mutate, or prior projection changes.
    #[test]
    fn wrong_argument_contract_preserves_owner_and_prior_prepared_values() -> Result<(), Error> {
        let store = fixture_store()?;
        let snapshot = store.snapshot();
        let good = UiText::fixed(MessageId::FIXTURE_SAMPLE_LABEL);
        let mut feedback = UiFeedbackSet::default();
        feedback.replace(FeedbackSlot::EditorFeedback, Some(good));
        let prior = feedback.prepare(&snapshot)?;
        // This code-owned ID requires typed minutes. Omitting them is a
        // presentation contract mismatch, never a request to infer raw text.
        feedback.replace(
            FeedbackSlot::StatusRemedyLabel,
            Some(UiText::fixed(MessageId::INTERVAL_CUSTOM)),
        );
        let rejected = feedback.clone();
        assert!(matches!(feedback.prepare(&snapshot), Err(Error::Arguments)));
        assert_eq!(feedback, rejected);
        assert_eq!(
            prior
                .get(FeedbackSlot::EditorFeedback)
                .map(crate::localization::PlainText::as_str),
            Some(snapshot.text(MessageId::FIXTURE_SAMPLE_LABEL)?.as_str()),
        );
        assert!(prior.get(FeedbackSlot::StatusRemedyLabel).is_none());
        feedback.replace(FeedbackSlot::StatusRemedyLabel, Some(good));
        let healthy = feedback.prepare(&snapshot)?;
        assert!(healthy.get(FeedbackSlot::StatusRemedyLabel).is_some());
        Ok(())
    }
}

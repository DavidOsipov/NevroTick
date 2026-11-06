//! F-06/v0.4 local profile-preview expectations; no machine-policy activation.
#![forbid(unsafe_code)]

use nevrotick_ui::policy::{PolicyDrafts, PolicyError, PolicyPhase, PolicyProfile};
use nevrotick_ui::source_drafts::{Protocol, RawSourceInput, SourceDraftError, SourceDrafts};

/// Independent raw-choice, profile, transport and no-write-intent expectations.
const PROFILES: [(i32, PolicyProfile, Option<Protocol>, bool); 3] = [
    (0, PolicyProfile::NtsOnly, Some(Protocol::Nts), false),
    (1, PolicyProfile::PlainNtp, Some(Protocol::PlainNtp), false),
    (2, PolicyProfile::ObserveOnly, None, true),
];

/// Unknown signed choices, including both adjacent boundaries and extremes.
const INVALID_CHOICES: [i32; 8] = [i32::MIN, -3, -2, -1, 3, 4, 255, i32::MAX];

/// The three explicit choices carry their own transport/authentication meaning.
///
/// # Panics
/// Fails if admission invents a fallback or observe-only acquires a transport.
#[test]
fn closed_choices_preserve_explicit_transport_and_observation_semantics() {
    for (choice, profile, transport, observe_only) in PROFILES {
        assert_eq!(PolicyProfile::from_view(choice), Ok(profile));
        assert_eq!(profile.view_choice(), choice);
        assert_eq!(profile.transport_requirement(), transport);
        assert_eq!(profile.is_observe_only(), observe_only);
    }
    for choice in INVALID_CHOICES {
        assert_eq!(
            PolicyProfile::from_view(choice),
            Err(PolicyError::InvalidChoice)
        );
    }
}

/// Constructors retain an explicit baseline without silently creating a draft.
///
/// # Panics
/// Fails if first edit/cancel changes the baseline, phase or preview presence.
#[test]
fn each_baseline_starts_in_summary_and_edit_cancel_uses_its_own_choice() {
    for (choice, baseline, _, _) in PROFILES {
        let mut owner = PolicyDrafts::new(baseline);
        let initial = owner.clone();
        assert_eq!(owner.baseline(), baseline);
        assert_eq!(owner.preview(), None);
        assert_eq!(owner.phase(), PolicyPhase::Summary);
        assert_eq!(owner.edit(), Ok(choice));
        assert_eq!(owner.phase(), PolicyPhase::Editing);
        assert_eq!(owner.preview(), None);
        assert_eq!(owner.cancel_edit(), Ok(choice));
        assert_eq!(owner, initial);
    }
}

/// NTS is an inert fixture baseline, with no accepted profile preview yet.
///
/// # Panics
/// Fails if the fixture impersonates an active preview or selects plain NTP.
#[test]
fn fixture_is_nts_baseline_with_no_preview() {
    let owner = PolicyDrafts::fixture();
    assert_eq!(owner, PolicyDrafts::new(PolicyProfile::NtsOnly));
    assert_eq!(owner.baseline(), PolicyProfile::NtsOnly);
    assert_eq!(owner.preview(), None);
    assert_eq!(owner.phase(), PolicyPhase::Summary);
}

/// Every baseline/preview combination follows review, retain and discard rules.
///
/// # Panics
/// Fails if a local review/keep/discard operation replaces the baseline.
#[test]
fn all_choices_can_be_reviewed_retained_reopened_and_discarded() {
    for (baseline_choice, baseline, _, _) in PROFILES {
        for (preview_choice, preview, _, _) in PROFILES {
            let mut owner = PolicyDrafts::new(baseline);
            assert_eq!(owner.edit(), Ok(baseline_choice));
            assert_eq!(owner.review(preview_choice), Ok(()));
            assert_eq!(owner.phase(), PolicyPhase::Review);
            assert_eq!(owner.preview(), Some(preview));
            assert_eq!(owner.baseline(), baseline);

            assert_eq!(owner.keep_preview(), Ok(()));
            assert_eq!(owner.phase(), PolicyPhase::Summary);
            assert_eq!(owner.preview(), Some(preview));
            assert_eq!(owner.baseline(), baseline);

            assert_eq!(owner.open_review(), Ok(()));
            assert_eq!(owner.phase(), PolicyPhase::Review);
            assert_eq!(owner.preview(), Some(preview));
            assert_eq!(owner.discard_preview(), Ok(()));
            assert_eq!(owner, PolicyDrafts::new(baseline));
            assert_eq!(owner.edit(), Ok(baseline_choice));
        }
    }
}

/// Raw invalid choices cannot partially publish or exit an in-progress editor.
///
/// # Panics
/// Fails if rejection changes any state or a subsequent legal choice is denied.
#[test]
fn invalid_first_review_preserves_the_entire_editor_state() {
    for (baseline_choice, baseline, _, _) in PROFILES {
        let mut owner = PolicyDrafts::new(baseline);
        assert_eq!(owner.edit(), Ok(baseline_choice));
        let editing = owner.clone();
        for choice in INVALID_CHOICES {
            assert_eq!(owner.review(choice), Err(PolicyError::InvalidChoice));
            assert_eq!(owner, editing);
        }
        assert_eq!(owner.review(1), Ok(()));
        assert_eq!(owner.preview(), Some(PolicyProfile::PlainNtp));
        assert_eq!(owner.phase(), PolicyPhase::Review);
        assert_eq!(owner.baseline(), baseline);
    }
}

/// Cancelling an invalid re-edit restores the exact retained summary preview.
///
/// # Panics
/// Fails if invalid raw text replaces a prior preview or cancel discards it.
#[test]
fn cancelled_invalid_edit_retains_the_previously_reviewed_preview() {
    for (choice, preview, _, _) in PROFILES {
        let mut owner = PolicyDrafts::new(PolicyProfile::NtsOnly);
        assert_eq!(owner.edit(), Ok(0));
        assert_eq!(owner.review(choice), Ok(()));
        assert_eq!(owner.keep_preview(), Ok(()));
        let saved_summary = owner.clone();

        assert_eq!(owner.edit(), Ok(choice));
        let saved_editor = owner.clone();
        for invalid in INVALID_CHOICES {
            assert_eq!(owner.review(invalid), Err(PolicyError::InvalidChoice));
            assert_eq!(owner, saved_editor);
        }
        assert_eq!(owner.cancel_edit(), Ok(choice));
        assert_eq!(owner, saved_summary);
        assert_eq!(owner.preview(), Some(preview));
        assert_eq!(owner.baseline(), PolicyProfile::NtsOnly);
        assert_eq!(owner.open_review(), Ok(()));
        assert_eq!(owner.preview(), Some(preview));
    }
}

/// Direct receiving calls must enforce Summary guards without widget help.
///
/// # Panics
/// Fails if a bypass creates, discards or publishes a preview outside Editing.
#[test]
fn summary_handlers_reject_direct_bypasses_atomically() {
    let mut owner = PolicyDrafts::new(PolicyProfile::NtsOnly);
    let initial = owner.clone();
    for choice in [i32::MIN, -1, 0, 1, 2, 3, i32::MAX] {
        assert_eq!(owner.review(choice), Err(PolicyError::WrongPhase));
        assert_eq!(owner, initial);
    }
    assert_eq!(owner.open_review(), Err(PolicyError::NoPreview));
    assert_eq!(owner, initial);
    assert_eq!(owner.keep_preview(), Err(PolicyError::WrongPhase));
    assert_eq!(owner, initial);
    assert_eq!(owner.cancel_edit(), Err(PolicyError::WrongPhase));
    assert_eq!(owner, initial);
    assert_eq!(owner.discard_preview(), Err(PolicyError::WrongPhase));
    assert_eq!(owner, initial);
    assert_eq!(owner.edit(), Ok(0));
}

/// Editing cannot be restarted, kept, reopened or discarded by direct calls.
///
/// # Panics
/// Fails if a wrong-phase operation alters the editor or its retained preview.
#[test]
fn editing_handlers_reject_wrong_phase_atomically_with_and_without_preview() {
    for retained in [None, Some(PolicyProfile::PlainNtp)] {
        let mut owner = PolicyDrafts::new(PolicyProfile::NtsOnly);
        if retained.is_some() {
            assert_eq!(owner.edit(), Ok(0));
            assert_eq!(owner.review(1), Ok(()));
            assert_eq!(owner.keep_preview(), Ok(()));
        }
        assert_eq!(owner.edit(), Ok(i32::from(retained.is_some())));
        let editing = owner.clone();
        assert_eq!(owner.edit(), Err(PolicyError::WrongPhase));
        assert_eq!(owner, editing);
        assert_eq!(owner.open_review(), Err(PolicyError::WrongPhase));
        assert_eq!(owner, editing);
        assert_eq!(owner.keep_preview(), Err(PolicyError::WrongPhase));
        assert_eq!(owner, editing);
        assert_eq!(owner.discard_preview(), Err(PolicyError::WrongPhase));
        assert_eq!(owner, editing);
        assert_eq!(owner.preview(), retained);
        assert_eq!(owner.review(2), Ok(()));
        assert_eq!(owner.preview(), Some(PolicyProfile::ObserveOnly));
    }
}

/// Review rejects callback replay and cancel/open bypasses; explicit edit works.
///
/// # Panics
/// Fails if a repeated review changes a previously accepted local preview.
#[test]
fn review_handlers_reject_replay_and_wrong_phase_atomically() {
    for (choice, preview, _, _) in PROFILES {
        let mut owner = PolicyDrafts::new(PolicyProfile::NtsOnly);
        assert_eq!(owner.edit(), Ok(0));
        assert_eq!(owner.review(choice), Ok(()));
        let reviewing = owner.clone();
        for raw in [i32::MIN, -1, 0, 1, 2, 3, i32::MAX] {
            assert_eq!(owner.review(raw), Err(PolicyError::WrongPhase));
            assert_eq!(owner, reviewing);
        }
        assert_eq!(owner.open_review(), Err(PolicyError::WrongPhase));
        assert_eq!(owner, reviewing);
        assert_eq!(owner.cancel_edit(), Err(PolicyError::WrongPhase));
        assert_eq!(owner, reviewing);
        assert_eq!(owner.edit(), Ok(choice));
        assert_eq!(owner.phase(), PolicyPhase::Editing);
        assert_eq!(owner.preview(), Some(preview));
        assert_eq!(owner.cancel_edit(), Ok(choice));
        assert_eq!(owner.phase(), PolicyPhase::Summary);
        assert_eq!(owner.preview(), Some(preview));
    }
}

/// An editor can replace a retained preview, including choosing the baseline.
///
/// # Panics
/// Fails if opening from Review uses the baseline or retains a superseded choice.
#[test]
fn editing_from_review_replaces_only_the_local_preview() {
    for (first_choice, _, _, _) in PROFILES {
        for (next_choice, next_profile, _, _) in PROFILES {
            let mut owner = PolicyDrafts::new(PolicyProfile::NtsOnly);
            assert_eq!(owner.edit(), Ok(0));
            assert_eq!(owner.review(first_choice), Ok(()));
            assert_eq!(owner.edit(), Ok(first_choice));
            assert_eq!(owner.review(next_choice), Ok(()));
            assert_eq!(owner.phase(), PolicyPhase::Review);
            assert_eq!(owner.preview(), Some(next_profile));
            assert_eq!(owner.baseline(), PolicyProfile::NtsOnly);
        }
    }
}

/// A retained Summary preview is protected by the same direct-handler guards.
///
/// # Panics
/// Fails if Summary accepts review/discard replay or keep behaves as activation.
#[test]
fn keeping_preview_changes_only_phase_and_does_not_authorize_summary_handlers() {
    let mut owner = PolicyDrafts::new(PolicyProfile::NtsOnly);
    assert_eq!(owner.edit(), Ok(0));
    assert_eq!(owner.review(2), Ok(()));
    assert_eq!(owner.keep_preview(), Ok(()));
    let summary = owner.clone();
    assert_eq!(owner.baseline(), PolicyProfile::NtsOnly);
    assert_eq!(owner.preview(), Some(PolicyProfile::ObserveOnly));
    assert_eq!(owner.review(1), Err(PolicyError::WrongPhase));
    assert_eq!(owner, summary);
    assert_eq!(owner.discard_preview(), Err(PolicyError::WrongPhase));
    assert_eq!(owner, summary);
    assert_eq!(owner.keep_preview(), Err(PolicyError::WrongPhase));
    assert_eq!(owner, summary);
    assert_eq!(owner.cancel_edit(), Err(PolicyError::WrongPhase));
    assert_eq!(owner, summary);
    assert_eq!(owner.open_review(), Ok(()));
    assert_eq!(owner.discard_preview(), Ok(()));
    assert_eq!(owner.open_review(), Err(PolicyError::NoPreview));
    assert_eq!(owner, PolicyDrafts::new(PolicyProfile::NtsOnly));
}

/// Choosing plain NTP or observation never rewrites separately admitted sources.
///
/// # Errors
/// Propagates unexpected failure admitting the two explicit source fixtures.
///
/// # Panics
/// Fails if a profile preview changes source protocols/ports or its baseline.
#[test]
fn policy_previews_preserve_source_protocols_and_ports() -> Result<(), SourceDraftError> {
    let sources = SourceDrafts::from_baseline(&[
        RawSourceInput {
            label: "Authenticated fixture",
            host: "nts.example.invalid",
            protocol_choice: 0,
            port: "4460",
        },
        RawSourceInput {
            label: "Unauthenticated fixture",
            host: "ntp.example.invalid",
            protocol_choice: 1,
            port: "123",
        },
    ])?;
    let original_sources = sources.clone();
    for (choice, profile, required_protocol, _) in PROFILES {
        let mut policy = PolicyDrafts::new(PolicyProfile::NtsOnly);
        assert_eq!(policy.edit(), Ok(0));
        assert_eq!(policy.review(choice), Ok(()));
        assert_eq!(policy.keep_preview(), Ok(()));
        assert_eq!(policy.preview(), Some(profile));
        assert_eq!(profile.transport_requirement(), required_protocol);
        assert_eq!(policy.baseline(), PolicyProfile::NtsOnly);
        assert_eq!(sources, original_sources);
        assert_eq!(sources.baseline()[0].protocol(), Protocol::Nts);
        assert_eq!(sources.baseline()[0].fields().port(), 4460);
        assert_eq!(sources.baseline()[1].protocol(), Protocol::PlainNtp);
        assert_eq!(sources.baseline()[1].fields().port(), 123);
    }
    Ok(())
}

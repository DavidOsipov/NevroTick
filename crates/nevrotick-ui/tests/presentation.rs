//! Independent controller expectations through the portable presentation API.
#![forbid(unsafe_code)]

use nevrotick_ui::model::ModelError;
use nevrotick_ui::presentation::{Mode, Page, UiError, UiSession};

/// Simple starts with useful fixtures and rejects every hidden page atomically.
///
/// # Errors
/// Propagates unexpected fixture or allowed-navigation rejection.
///
/// # Panics
/// Fails when the independently specified mode/navigation contract is violated.
#[test]
fn simple_mode_has_checked_fixtures_and_guards_direct_navigation() -> Result<(), UiError> {
    let mut session = UiSession::fixture()?;
    assert_eq!(session.mode(), Mode::Simple);
    assert_eq!(session.page(), Page::Status);
    assert_eq!(session.sources().len(), 3);
    assert_eq!(session.selected_index(), 0);
    assert_eq!(session.selected().label(), "Sample North");
    session.navigate(Page::Settings)?;
    let before = session.clone();
    for page in [Page::Sources, Page::Statistics, Page::Diagnostics] {
        assert!(page.is_power_only());
        assert_eq!(session.navigate(page), Err(UiError::PageUnavailable));
        assert_eq!(session, before);
    }
    for page in [Page::About, Page::Status, Page::Settings] {
        assert!(!page.is_power_only());
        session.navigate(page)?;
        assert_eq!(session.page(), page);
    }
    Ok(())
}

/// Each Power-only page returns after Simple's Status fallback, preserving data.
///
/// # Errors
/// Propagates unexpected fixture, edit or allowed-navigation rejection.
///
/// # Panics
/// Fails when a mode transition loses remembered navigation or accepted values.
#[test]
fn power_page_is_remembered_without_resetting_accepted_preview() -> Result<(), UiError> {
    for page in [Page::Sources, Page::Statistics, Page::Diagnostics] {
        let mut session = UiSession::fixture()?;
        session.set_mode(Mode::Power);
        session.navigate(page)?;
        let reset = session.update_preview("Edited sample", "clock.example.invalid", "1234")?;
        assert_eq!(reset.source().port(), 1234);
        let accepted = session.sources().to_vec();
        session.set_mode(Mode::Simple);
        assert_eq!(session.page(), Page::Status);
        assert_eq!(session.mode(), Mode::Simple);
        session.set_mode(Mode::Simple);
        session.navigate(Page::Status)?;
        session.set_mode(Mode::Power);
        assert_eq!(session.page(), page);
        assert_eq!(session.sources(), accepted.as_slice());
        let before = session.clone();
        session.set_mode(Mode::Power);
        assert_eq!(session, before);
    }
    Ok(())
}

/// Shared pages survive mode changes and supersede automatic Power restoration.
///
/// # Errors
/// Propagates unexpected fixture or allowed-navigation rejection.
///
/// # Panics
/// Fails when switching mode unexpectedly redirects a shared page.
#[test]
fn shared_pages_remain_reachable_across_mode_changes() -> Result<(), UiError> {
    for shared in [Page::Settings, Page::About] {
        let mut session = UiSession::fixture()?;
        session.set_mode(Mode::Power);
        session.navigate(Page::Diagnostics)?;
        session.set_mode(Mode::Simple);
        session.navigate(shared)?;
        session.set_mode(Mode::Power);
        assert_eq!(session.page(), shared);
        session.set_mode(Mode::Simple);
        assert_eq!(session.page(), shared);
        session.set_mode(Mode::Power);
        assert_eq!(session.page(), shared);
    }
    let mut session = UiSession::fixture()?;
    session.set_mode(Mode::Power);
    session.navigate(Page::Sources)?;
    session.navigate(Page::Status)?;
    session.set_mode(Mode::Simple);
    session.set_mode(Mode::Power);
    assert_eq!(session.page(), Page::Status);
    Ok(())
}

/// Signed callback selection cannot escape the fixed inventory or reset state.
///
/// # Errors
/// Propagates unexpected fixture or legal-selection rejection.
///
/// # Panics
/// Fails when an invalid callback index mutates any accepted state.
#[test]
fn signed_selection_is_checked_and_returns_accepted_reset_projection() -> Result<(), UiError> {
    let mut session = UiSession::fixture()?;
    let reset = session.select_source(2)?;
    assert_eq!(reset.selected_index(), 2);
    assert_eq!(reset.source().label(), "Sample West");
    assert_eq!(reset.message(), "");
    let before = session.clone();
    for index in [-1, i32::MIN, 3, i32::MAX] {
        assert_eq!(
            session.select_source(index),
            Err(UiError::Model(ModelError::InvalidSelection))
        );
        assert_eq!(session, before);
    }
    let reset = session.select_source(1)?;
    assert_eq!(reset.selected_index(), 1);
    assert_eq!(reset.source().host(), "south.example.invalid");
    assert_eq!(session.page(), Page::Status);
    assert_eq!(session.mode(), Mode::Simple);
    Ok(())
}

/// Direct edits publish checked projections; malformed updates retain all state.
///
/// # Errors
/// Propagates unexpected fixture, legal-selection or legal-edit rejection.
///
/// # Panics
/// Fails when update projection, failure atomicity or reset-on-restart is broken.
#[test]
fn direct_updates_are_atomic_and_publish_only_accepted_values() -> Result<(), UiError> {
    let mut session = UiSession::fixture()?;
    let initial = session.clone();
    let reset = session.select_source(1)?;
    assert_eq!(reset.selected_index(), 1);
    let reset = session.update_preview("Edited sample", "192.0.2.25", "1234")?;
    assert_eq!(reset.selected_index(), 1);
    assert_eq!(reset.source().label(), "Edited sample");
    assert_eq!(reset.source().host(), "192.0.2.25");
    assert_eq!(reset.source().port(), 1234);
    assert_eq!(
        reset.message(),
        "Preview updated in memory only. No connection or policy change was made."
    );
    assert_eq!(session.sources()[0], initial.sources()[0]);
    assert_eq!(session.sources()[2], initial.sources()[2]);
    let before = session.clone();
    assert_eq!(
        session.update_preview("secret\nlabel", "192.0.2.25", "1234"),
        Err(UiError::Model(ModelError::InvalidLabel))
    );
    assert_eq!(session, before);
    assert_eq!(
        session.update_preview("Edited sample", "https://example.invalid", "1234"),
        Err(UiError::Model(ModelError::InvalidHost))
    );
    assert_eq!(session, before);
    assert_eq!(
        session.update_preview("Edited sample", "192.0.2.25", "65536"),
        Err(UiError::Model(ModelError::InvalidPort))
    );
    assert_eq!(session, before);
    let oversize = "secret".repeat(1000);
    assert_eq!(
        session.update_preview(&oversize, "192.0.2.25", "1234"),
        Err(UiError::Model(ModelError::InvalidLabel))
    );
    assert_eq!(session, before);
    let reset = session.update_preview("Healthy neighbor", "clock.example.invalid", "4460")?;
    assert_eq!(reset.source().label(), "Healthy neighbor");
    assert_eq!(session.sources().len(), 3);
    assert_eq!(UiSession::fixture()?, initial);
    Ok(())
}

/// Cancel returns the accepted source without modifying data or navigation.
///
/// # Errors
/// Propagates unexpected fixture, navigation, legal-selection or edit rejection.
///
/// # Panics
/// Fails when Cancel loses an accepted edit or changes session state.
#[test]
fn cancel_instructs_the_view_to_reset_to_current_accepted_fields() -> Result<(), UiError> {
    let mut session = UiSession::fixture()?;
    session.set_mode(Mode::Power);
    session.navigate(Page::Sources)?;
    let reset = session.select_source(2)?;
    assert_eq!(reset.selected_index(), 2);
    let reset = session.update_preview("Accepted", "clock.example.invalid", "4460")?;
    assert_eq!(reset.source().label(), "Accepted");
    assert!(session.update_preview("", "rejected.example", "0").is_err());
    session.navigate(Page::Settings)?;
    let before = session.clone();
    let reset = session.cancel();
    assert_eq!(reset.selected_index(), 2);
    assert_eq!(reset.source().label(), "Accepted");
    assert_eq!(reset.source().host(), "clock.example.invalid");
    assert_eq!(reset.source().port(), 4460);
    assert_eq!(reset.message(), "Unaccepted editor changes discarded.");
    assert_eq!(session, before);
    Ok(())
}

/// Rejected callback data produces fixed feedback without retaining its payload.
///
/// # Errors
/// Propagates unexpected fixture rejection.
///
/// # Panics
/// Fails when malformed input is accepted or appears in binding feedback.
#[test]
fn binding_error_text_is_fixed_and_contains_no_rejected_payload() -> Result<(), UiError> {
    let mut session = UiSession::fixture()?;
    let before = session.clone();
    let error = session
        .update_preview("secret\nvalue", "clock.example.invalid", "4460")
        .err();
    assert_eq!(error, Some(UiError::Model(ModelError::InvalidLabel)));
    assert_eq!(
        error.map(UiError::text),
        Some("The sample display name is invalid.")
    );
    assert_eq!(
        error.map(|cause| cause.to_string()).as_deref(),
        Some("The sample display name is invalid.")
    );
    assert_eq!(session, before);
    Ok(())
}

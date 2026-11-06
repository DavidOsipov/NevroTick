//! Source-independent navigation contract, retaining legacy `UiSession` behavior.
#![forbid(unsafe_code)]

use nevrotick_ui::presentation::{Mode, NavigationState, Page, UiError, UiSession};

/// Navigation construction needs no source fixture and starts Simple/Status.
///
/// # Panics
/// Fails if initialization or same-mode transitions change the reachable page.
#[test]
fn source_independent_navigation_starts_simple_and_repeated_mode_is_noop() {
    let mut navigation = NavigationState::new();
    assert_eq!(navigation, NavigationState::default());
    assert_eq!(navigation.mode(), Mode::Simple);
    assert_eq!(navigation.page(), Page::Status);
    let before = navigation.clone();
    navigation.set_mode(Mode::Simple);
    assert_eq!(navigation, before);
}

/// Hidden Power pages reject independently of the view's visible tab set.
///
/// # Errors
/// Propagates unexpected navigation rejection for a shared page.
///
/// # Panics
/// Fails when a direct hidden-page request changes checked state.
#[test]
fn simple_mode_rejects_hidden_pages_atomically_and_keeps_shared_routes() -> Result<(), UiError> {
    let mut navigation = NavigationState::new();
    for shared in [Page::Status, Page::Settings, Page::About] {
        navigation.navigate(shared)?;
        for hidden in [Page::Sources, Page::Statistics, Page::Diagnostics] {
            let before = navigation.clone();
            assert_eq!(navigation.navigate(hidden), Err(UiError::PageUnavailable));
            assert_eq!(navigation, before);
        }
    }
    Ok(())
}

/// Power fallback/return and shared-page retention have the original semantics.
///
/// # Errors
/// Propagates unexpected allowed-page rejection.
///
/// # Panics
/// Fails when switching modes loses remembered Power or current shared context.
#[test]
fn remembered_power_page_and_shared_page_context_survive_round_trips() -> Result<(), UiError> {
    for power in [Page::Sources, Page::Statistics, Page::Diagnostics] {
        let mut navigation = NavigationState::new();
        navigation.set_mode(Mode::Power);
        navigation.navigate(power)?;
        let before = navigation.clone();
        navigation.set_mode(Mode::Power);
        assert_eq!(navigation, before);
        navigation.set_mode(Mode::Simple);
        assert_eq!(navigation.page(), Page::Status);
        navigation.set_mode(Mode::Power);
        assert_eq!(navigation.page(), power);
        for shared in [Page::Settings, Page::About] {
            navigation.set_mode(Mode::Simple);
            navigation.navigate(shared)?;
            navigation.set_mode(Mode::Power);
            assert_eq!(navigation.page(), shared);
            navigation.set_mode(Mode::Simple);
            assert_eq!(navigation.page(), shared);
        }
    }
    Ok(())
}

/// The legacy fixture session delegates navigation without touching sources.
///
/// # Errors
/// Propagates unexpected fixture/edit or allowed-navigation failure.
///
/// # Panics
/// Fails when extraction changes legacy source APIs or navigation semantics.
#[test]
fn legacy_session_delegates_navigation_without_changing_source_contract() -> Result<(), UiError> {
    let mut session = UiSession::fixture()?;
    let mut navigation = NavigationState::new();
    assert_eq!(session.select_source(2)?.selected_index(), 2);
    assert_eq!(
        session
            .update_preview("Retained", "retained.example.invalid", "1234")?
            .source()
            .label(),
        "Retained"
    );
    let sources = session.sources().to_vec();
    for mode in [Mode::Power, Mode::Simple, Mode::Power] {
        session.set_mode(mode);
        navigation.set_mode(mode);
        for page in [
            Page::Sources,
            Page::Settings,
            Page::About,
            Page::Statistics,
            Page::Status,
        ] {
            assert_eq!(session.navigate(page), navigation.navigate(page));
            assert_eq!(session.mode(), navigation.mode());
            assert_eq!(session.page(), navigation.page());
            assert_eq!(session.sources(), sources.as_slice());
            assert_eq!(session.selected_index(), 2);
            assert_eq!(session.selected().label(), "Retained");
        }
    }
    Ok(())
}

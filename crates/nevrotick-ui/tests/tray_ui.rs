//! Pure menu receiver checks; no native tray, loop, service or network is used.
#![forbid(unsafe_code)]

use nevrotick_ui::shell::{ShellRequest, TrayMenuAction};
use nevrotick_ui::tray_ui::{
    FixtureTrayMenu, TrayCheckBlock, TrayCheckContext, TrayInteraction, TrayItemId, TrayLabel,
    TrayRejection,
};

/// The six exact identities are stable even while one row changes its wording.
///
/// # Panics
/// Fails if unknown signed values are admitted, IDs alias or valid rows disappear.
#[test]
fn exact_signed_item_identity_has_both_rejected_and_valid_neighbors() {
    for raw in [i32::MIN, -2, -1, 6, 7, i32::MAX] {
        assert_eq!(TrayItemId::parse(raw), Err(TrayRejection::InvalidItem));
    }
    for (raw, id) in (0..6).zip([
        TrayItemId::Open,
        TrayItemId::ContextualCheck,
        TrayItemId::Diagnostics,
        TrayItemId::Settings,
        TrayItemId::About,
        TrayItemId::QuitUi,
    ]) {
        assert_eq!(TrayItemId::parse(raw), Ok(id));
        assert_eq!(id.view_id(), raw);
    }
}

/// Inspecting a menu gives six stable rows and cannot request measurements.
///
/// # Panics
/// Fails if the fixture claims a successful check or loses a contextual label.
#[test]
fn contextual_check_is_unavailable_under_every_fixture_measurement_context() {
    for (context, expected_label) in [
        (TrayCheckContext::Synchronization, TrayLabel::SyncNow),
        (TrayCheckContext::CheckSourcesOnly, TrayLabel::CheckSources),
    ] {
        for block in [
            TrayCheckBlock::NativeAdapterUnavailable,
            TrayCheckBlock::StateUnconfirmed,
            TrayCheckBlock::OperationPending,
            TrayCheckBlock::RateLimited,
        ] {
            let menu = FixtureTrayMenu::new(context, block);
            let before = menu;
            let rows = menu.entries(TrayInteraction::Free);
            assert_eq!(rows.len(), 6);
            let contextual = rows[1];
            assert_eq!(contextual.id(), TrayItemId::ContextualCheck);
            assert_eq!(contextual.label(), expected_label);
            assert!(!contextual.enabled());
            assert_eq!(
                contextual.rejection(),
                Some(TrayRejection::CheckUnavailable(block)),
            );
            // A direct callback cannot bypass the disabled projected row.
            assert_eq!(
                menu.request(1, TrayInteraction::Free),
                Err(TrayRejection::CheckUnavailable(block)),
            );
            // Reopening/reading a menu is only a bounded immutable projection.
            for _ in 0..32 {
                assert_eq!(menu.entries(TrayInteraction::Free), rows);
            }
            assert_eq!(menu, before);
        }
    }
}

/// A stopped-service context replaces the check with ordinary navigation only.
///
/// # Panics
/// Fails if a missing check adapter blocks management or a click starts a check.
#[test]
fn stopped_service_context_opens_management_without_a_measurement_command() {
    for block in [
        TrayCheckBlock::NativeAdapterUnavailable,
        TrayCheckBlock::StateUnconfirmed,
        TrayCheckBlock::OperationPending,
        TrayCheckBlock::RateLimited,
    ] {
        let menu = FixtureTrayMenu::new(TrayCheckContext::ServicesStopped, block);
        let row = menu.entries(TrayInteraction::Free)[1];
        assert_eq!(row.id(), TrayItemId::ContextualCheck);
        assert_eq!(row.label(), TrayLabel::ManageBackgroundService);
        assert!(row.enabled());
        assert_eq!(row.rejection(), None);
        assert_eq!(
            menu.request(1, TrayInteraction::Free),
            Ok(ShellRequest::Menu(TrayMenuAction::TimeManagement)),
        );
    }
}

/// Ordinary navigation and warning entry work independently of missing adapters.
///
/// # Panics
/// Fails on an invented measurement/exit effect, missing shared page or route drift.
#[test]
fn positive_navigation_and_quit_warning_remain_available_without_native_tray() {
    let menu = FixtureTrayMenu::new(
        TrayCheckContext::CheckSourcesOnly,
        TrayCheckBlock::NativeAdapterUnavailable,
    );
    for (id, expected, label) in [
        (
            TrayItemId::Open,
            ShellRequest::Menu(TrayMenuAction::OpenNevroTick),
            TrayLabel::Open,
        ),
        (
            TrayItemId::Diagnostics,
            ShellRequest::Menu(TrayMenuAction::Diagnostics),
            TrayLabel::Diagnostics,
        ),
        (
            TrayItemId::Settings,
            ShellRequest::Menu(TrayMenuAction::Settings),
            TrayLabel::Settings,
        ),
        (
            TrayItemId::About,
            ShellRequest::Menu(TrayMenuAction::About),
            TrayLabel::About,
        ),
        (
            TrayItemId::QuitUi,
            ShellRequest::Menu(TrayMenuAction::QuitUi),
            TrayLabel::QuitUi,
        ),
    ] {
        let row = menu
            .entries(TrayInteraction::Free)
            .into_iter()
            .find(|entry| entry.id() == id);
        assert_eq!(
            row.map(nevrotick_ui::tray_ui::TrayEntry::label),
            Some(label)
        );
        assert!(row.is_some_and(nevrotick_ui::tray_ui::TrayEntry::enabled));
        assert_eq!(
            menu.request(id.view_id(), TrayInteraction::Free),
            Ok(expected)
        );
    }
}

/// A prior enabled row never authorizes a later modal or already-exiting request.
///
/// # Panics
/// Fails if a direct callback uses stale availability instead of its current guard.
#[test]
fn receiving_guard_rejects_every_row_during_modal_and_pending_exit() {
    let menu = FixtureTrayMenu::new(
        TrayCheckContext::ServicesStopped,
        TrayCheckBlock::NativeAdapterUnavailable,
    );
    assert!(
        menu.entries(TrayInteraction::Free)
            .iter()
            .all(|row| row.enabled())
    );
    for (guard, expected) in [
        (TrayInteraction::DialogOwned, TrayRejection::DialogOwned),
        (TrayInteraction::ExitRequested, TrayRejection::ExitRequested),
    ] {
        for row in menu.entries(guard) {
            assert!(!row.enabled());
            assert_eq!(row.rejection(), Some(expected));
            assert_eq!(menu.request(row.id().view_id(), guard), Err(expected));
        }
    }
    // Once the private owner supplies a fresh free guard, a legal neighbor works.
    assert_eq!(
        menu.request(TrayItemId::Open.view_id(), TrayInteraction::Free),
        Ok(ShellRequest::Menu(TrayMenuAction::OpenNevroTick)),
    );
}

/// Raw identity rejection is bounded and cannot dispatch another row as fallback.
///
/// # Panics
/// Fails if malformed identities select Open, Quit or a contextual action.
#[test]
fn direct_invalid_identity_never_selects_an_effect_or_route() {
    let menu = FixtureTrayMenu::new(
        TrayCheckContext::ServicesStopped,
        TrayCheckBlock::NativeAdapterUnavailable,
    );
    for guard in [
        TrayInteraction::Free,
        TrayInteraction::DialogOwned,
        TrayInteraction::ExitRequested,
    ] {
        for raw in [i32::MIN, -1, 6, i32::MAX] {
            assert_eq!(menu.request(raw, guard), Err(TrayRejection::InvalidItem));
        }
    }
}

/// Frozen English text is inert ASCII with distinct code-owned message identity.
///
/// # Panics
/// Fails if localization preparation changes IDs, implies retry duration or hides a reason.
#[test]
fn english_label_and_reason_inventory_has_stable_distinct_ids() {
    let labels = [
        TrayLabel::Open,
        TrayLabel::SyncNow,
        TrayLabel::CheckSources,
        TrayLabel::ManageBackgroundService,
        TrayLabel::Diagnostics,
        TrayLabel::Settings,
        TrayLabel::About,
        TrayLabel::QuitUi,
    ];
    for (index, label) in labels.iter().enumerate() {
        assert!(label.english().is_ascii() && !label.english().is_empty());
        assert!(
            !labels[..index]
                .iter()
                .any(|prior| prior.catalog_id() == label.catalog_id())
        );
    }
    let reasons = [
        TrayRejection::InvalidItem,
        TrayRejection::DialogOwned,
        TrayRejection::ExitRequested,
        TrayRejection::CheckUnavailable(TrayCheckBlock::NativeAdapterUnavailable),
        TrayRejection::CheckUnavailable(TrayCheckBlock::StateUnconfirmed),
        TrayRejection::CheckUnavailable(TrayCheckBlock::OperationPending),
        TrayRejection::CheckUnavailable(TrayCheckBlock::RateLimited),
    ];
    for (index, reason) in reasons.iter().enumerate() {
        assert!(reason.english().is_ascii() && !reason.english().is_empty());
        assert!(reason.catalog_id().starts_with("ui.tray.block."));
        assert!(
            !reasons[..index]
                .iter()
                .any(|prior| prior.catalog_id() == reason.catalog_id())
        );
    }
    assert_eq!(TrayLabel::Diagnostics.english(), "Diagnostics (Power user)");
    assert_eq!(
        TrayLabel::ManageBackgroundService.english(),
        "Manage background service"
    );
}

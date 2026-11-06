//! Presentation choices cannot claim collection, export or correction success.
use nevrotick_ui::diagnostics_ui::{CollectionScope, DiagnosticPreview, DiagnosticView};

/// Invalid and offscreen choices cannot replace the accepted local preview.
///
/// # Panics
/// Fails if closed-choice validation is bypassed or rejection changes state.
#[test]
fn closed_choices_preserve_the_previous_local_preview_on_rejection() {
    let mut owner = DiagnosticPreview::default();
    assert_eq!(owner.view(), DiagnosticView::Overview);
    let original = owner.clone();
    assert!(owner.select_collection(0, 0).is_err());
    assert_eq!(owner, original);
    for invalid in [i32::MIN, -1, 4, i32::MAX] {
        assert!(owner.select_view(invalid).is_err());
        assert_eq!(owner, original);
    }
    assert!(owner.select_view(2).is_ok());
    assert!(owner.select_collection(2, 1).is_ok());
    assert_eq!(owner.scope(), CollectionScope::UiAndService);
    assert_eq!(owner.duration_choice(), 1);
    let accepted = owner.clone();
    for (scope, duration) in [(-1, 0), (3, 0), (0, -1), (0, 3), (i32::MAX, i32::MIN)] {
        assert!(owner.select_collection(scope, duration).is_err());
        assert_eq!(owner, accepted);
    }
    assert!(owner.select_view(1).is_ok());
    assert_eq!(owner.view(), DiagnosticView::Report);
    assert!(owner.select_collection(0, 0).is_err());
    assert_eq!(owner.scope(), CollectionScope::UiAndService);
    assert!(owner.select_view(3).is_ok());
    assert_eq!(owner.view(), DiagnosticView::CorrectionReview);
}

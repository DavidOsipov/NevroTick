//! Independent UI-M0 expectations through the public production preview API.
#![forbid(unsafe_code)]

use nevrotick_ui::model::{ModelError, PreviewModel, SourcePreview};

/// A legal edit changes only the selected source and never survives restart.
///
/// # Errors
/// Propagates an unexpected rejection of a legal fixture or healthy edit.
///
/// # Panics
/// Fails an assertion when the independently specified contract is violated.
#[test]
fn accepted_edit_is_local_and_restart_discards_it() -> Result<(), ModelError> {
    let mut model = PreviewModel::fixture()?;
    let initial = model.clone();
    model.select(1)?;
    model.update_selected("New sample", "clock.example.invalid", "1234")?;
    assert_eq!(model.selected().label(), "New sample");
    assert_eq!(model.selected().host(), "clock.example.invalid");
    assert_eq!(model.selected().port(), 1234);
    assert_eq!(model.sources()[0], initial.sources()[0]);
    assert_eq!(model.sources()[2], initial.sources()[2]);
    assert_eq!(PreviewModel::fixture()?, initial);
    Ok(())
}

/// Direct handlers reject malicious and boundary inputs without changing state.
///
/// # Errors
/// Propagates an unexpected rejection of a legal fixture or healthy edit.
///
/// # Panics
/// Fails an assertion when the independently specified contract is violated.
#[test]
fn invalid_direct_edits_preserve_all_previously_accepted_state() -> Result<(), ModelError> {
    let mut model = PreviewModel::fixture()?;
    model.update_selected("Valid neighbor", "192.0.2.10", "65535")?;
    let before = model.clone();
    for (label, host, port, error) in [
        (
            "",
            "clock.example.invalid",
            "4460",
            ModelError::InvalidLabel,
        ),
        (
            " name",
            "clock.example.invalid",
            "4460",
            ModelError::InvalidLabel,
        ),
        (
            "bad\nlabel",
            "clock.example.invalid",
            "4460",
            ModelError::InvalidLabel,
        ),
        (
            "Sample",
            "https://example.invalid",
            "4460",
            ModelError::InvalidHost,
        ),
        ("Sample", "-bad.example", "4460", ModelError::InvalidHost),
        ("Sample", "bad..example", "4460", ModelError::InvalidHost),
        (
            "Sample",
            "clock.example:443",
            "4460",
            ModelError::InvalidHost,
        ),
        ("Sample", "clock.example", "0", ModelError::InvalidPort),
        ("Sample", "clock.example", "65536", ModelError::InvalidPort),
        ("Sample", "clock.example", "+123", ModelError::InvalidPort),
        ("Sample", "clock.example", " 123", ModelError::InvalidPort),
        ("Sample", "clock.example", "1e3", ModelError::InvalidPort),
    ] {
        assert_eq!(model.update_selected(label, host, port), Err(error));
        assert_eq!(model, before);
    }
    Ok(())
}

/// Maximal accepted neighbors and just-over-limit inputs exercise real admission.
///
/// # Errors
/// Propagates an unexpected rejection of a legal fixture or healthy edit.
///
/// # Panics
/// Fails an assertion when the independently specified contract is violated.
#[test]
fn field_boundaries_and_ip_literals_are_checked() -> Result<(), ModelError> {
    let label = "x".repeat(64);
    let host = format!(
        "{}.{}.{}.{}",
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(61)
    );
    assert_eq!(host.len(), 253);
    assert!(SourcePreview::parse(&label, &host, "65535").is_ok());
    assert_eq!(
        SourcePreview::parse(&"x".repeat(65), &host, "1"),
        Err(ModelError::InvalidLabel)
    );
    assert_eq!(
        SourcePreview::parse("Sample", &format!("{host}x"), "1"),
        Err(ModelError::InvalidHost)
    );
    assert_eq!(
        SourcePreview::parse("Sample", &format!("{}.invalid", "a".repeat(64)), "1"),
        Err(ModelError::InvalidHost)
    );
    assert_eq!(
        SourcePreview::parse("Sample", "clock.example", "000001"),
        Err(ModelError::InvalidPort)
    );
    assert_eq!(SourcePreview::parse("IPv6", "2001:db8::1", "1")?.port(), 1);
    assert!(SourcePreview::parse("IPv4", "192.0.2.1", "123").is_ok());
    Ok(())
}

/// Invalid selection preserves state, including after the maximum usize input.
///
/// # Errors
/// Propagates an unexpected rejection of a legal fixture or healthy edit.
///
/// # Panics
/// Fails an assertion when the independently specified contract is violated.
#[test]
fn selection_rejects_out_of_range_indices() -> Result<(), ModelError> {
    let mut model = PreviewModel::fixture()?;
    model.select(2)?;
    let before = model.clone();
    for index in [3, usize::MAX] {
        assert_eq!(model.select(index), Err(ModelError::InvalidSelection));
        assert_eq!(model, before);
    }
    Ok(())
}

/// Repeated valid updates replace a bounded row without growing the inventory.
///
/// # Errors
/// Propagates an unexpected rejection of a legal fixture or healthy edit.
///
/// # Panics
/// Fails an assertion when the independently specified contract is violated.
#[test]
fn repeated_edits_keep_fixed_inventory_and_redacted_errors() -> Result<(), ModelError> {
    let mut model = PreviewModel::fixture()?;
    for _ in 0..256 {
        model.update_selected("Sample", "clock.example.invalid", "4460")?;
        assert_eq!(model.sources().len(), 3);
    }
    let result = model.update_selected("secret\nvalue", "clock.example", "4460");
    assert_eq!(result, Err(ModelError::InvalidLabel));
    assert!(!ModelError::InvalidLabel.to_string().contains("secret"));
    Ok(())
}

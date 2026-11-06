//! Date-helper receiving tests: no network or clock work is possible here.
#![forbid(unsafe_code)]

use nevrotick_ui::recovery::{RecoveryDrafts, RecoveryError, RecoveryPreview};

/// Off is the default; one/two explicit helpers remain local, unauthenticated.
///
/// # Errors
/// Propagates an unexpected valid-neighbor admission failure.
///
/// # Panics
/// Fails when Off/default, fixed port, used-field or discard behavior regresses.
#[test]
fn helpers_are_off_by_default_and_only_used_fields_are_admitted() -> Result<(), RecoveryError> {
    let mut owner = RecoveryDrafts::default();
    assert_eq!(owner.preview().helpers(), []);
    owner.update(1, "first.example.invalid", "https://unused.invalid/")?;
    assert_eq!(owner.preview().helpers().len(), 1);
    assert_eq!(owner.preview().helpers()[0].host(), "first.example.invalid");
    assert_eq!(owner.preview().helpers()[0].port(), 123);
    owner.update(2, "first.example.invalid", "2001:db8::2")?;
    assert_eq!(owner.preview().helpers().len(), 2);
    owner.update(0, "unused bad raw text", "::invalid")?;
    assert_eq!(owner.preview().helpers(), []);
    owner.update(1, "192.0.2.1", "")?;
    owner.discard();
    assert_eq!(owner.preview().helpers(), []);
    Ok(())
}

/// Closed counts and shared identity rules guard actual direct receiving calls.
///
/// # Errors
/// Propagates an unexpected healthy setup or recovery failure.
///
/// # Panics
/// Fails if malformed fields/duplicates partly replace a valid preview.
#[test]
fn invalid_and_duplicate_helpers_preserve_the_previous_preview() -> Result<(), RecoveryError> {
    let mut owner = RecoveryDrafts::default();
    owner.update(1, "saved.example.invalid", "")?;
    let saved = owner.clone();
    for count in [i32::MIN, -1, 3, i32::MAX] {
        assert_eq!(
            owner.update(count, "valid.example.invalid", ""),
            Err(RecoveryError::InvalidCount)
        );
        assert_eq!(owner, saved);
    }
    let oversized = "x".repeat(254);
    for host in [
        "",
        " ",
        "https://example.invalid",
        "999.999.999.999",
        "host.invalid..",
        "юникод.invalid",
        &oversized,
    ] {
        assert!(owner.update(1, host, "").is_err());
        assert_eq!(owner, saved);
        assert!(owner.update(2, "neighbor.example.invalid", host).is_err());
        assert_eq!(owner, saved);
    }
    for (first, second) in [
        ("EXAMPLE.invalid", "example.invalid."),
        ("192.0.2.1", "192.0.2.1"),
        ("2001:db8::1", "2001:0DB8:0:0:0:0:0:1"),
    ] {
        assert!(owner.update(2, first, second).is_err());
        assert_eq!(owner, saved);
    }
    let accepted = RecoveryPreview::parse(2, "example.invalid", "neighbor.example.invalid")?;
    assert_eq!(accepted.helpers().len(), 2);
    Ok(())
}

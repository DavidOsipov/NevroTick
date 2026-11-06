//! Public release entrypoints cannot inherit the unit-test fixture authority.
#![forbid(unsafe_code)]

use nevrotick_ui::localization::{self, CatalogStore, Error};

/// Technically valid fixture data remains unapproved in the normal library.
///
/// # Errors
/// Propagates unexpected failure to prepare a technically valid review candidate.
///
/// # Panics
/// Fails if a data file or unit-only grant can activate the release API.
#[test]
fn production_api_rejects_the_synthetic_unit_fixture() -> Result<(), Error> {
    let english = include_bytes!("../locales/en.json");
    let expected = include_bytes!("../fixtures/test-only-english.cbor");
    let candidate = localization::prepare_unapproved_json(english)?;
    assert_eq!(candidate.bytes_for_review(), expected);
    assert!(matches!(
        localization::pack_approved_json(english),
        Err(Error::Approval)
    ));
    assert!(matches!(
        CatalogStore::bootstrap_english(expected),
        Err(Error::Approval)
    ));
    Ok(())
}

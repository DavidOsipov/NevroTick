//! Full raw-i128 public admission through actual Garde and coordinate construction.
//!
//! No input is assumed valid and no private product state is assembled. This
//! theorem supplies no anchor provenance, date-scope, resolver or clock authority.
//! The real dependency graph, allocation model and unwinding remain prerequisites.

use crate::{DomainError, EraWindow};

/// Independent decimal bounds of the signed 96-bit Q32 coordinate format.
const LOWEST_COORDINATE: i128 = -39_614_081_257_132_168_796_771_975_168;
/// Inclusive upper bound, including every fractional bit of the last second.
const HIGHEST_COORDINATE: i128 = 39_614_081_257_132_168_796_771_975_167;

/// Observe the actual constructor against the complete independent raw contract.
fn constructor_contract(earliest: i128, latest: i128) -> bool {
    let expected = earliest >= LOWEST_COORDINATE
        && earliest <= HIGHEST_COORDINATE
        && latest >= LOWEST_COORDINATE
        && latest <= HIGHEST_COORDINATE
        && earliest <= latest;
    let result = EraWindow::new(earliest, latest);
    let accepted = result.is_ok();
    assert_eq!(accepted, expected, "public window admission iff full raw contract");
    match result {
        Ok(window) => {
            assert_eq!(window.earliest().ticks(), earliest, "exact earliest coordinate retained");
            assert_eq!(window.latest().ticks(), latest, "exact latest coordinate retained");
            assert!(
                window.earliest().ticks() <= window.latest().ticks(),
                "constructed endpoints remain ordered"
            );
        }
        Err(error) => assert_eq!(
            error,
            DomainError::InvalidEraWindow,
            "every raw constructor rejection is InvalidEraWindow"
        ),
    }
    accepted
}

#[kani::proof]
fn era_constructor_full_domain() {
    let earliest = kani::any::<i128>();
    let latest = kani::any::<i128>();
    let accepted = constructor_contract(earliest, latest);
    kani::cover!(earliest == 0 && latest == 0 && accepted, "zero singleton is admitted");
    kani::cover!(
        earliest == LOWEST_COORDINATE && latest == LOWEST_COORDINATE && accepted,
        "minimum singleton is admitted"
    );
    kani::cover!(
        earliest == HIGHEST_COORDINATE && latest == HIGHEST_COORDINATE && accepted,
        "maximum singleton is admitted"
    );
    kani::cover!(
        earliest == LOWEST_COORDINATE && latest == HIGHEST_COORDINATE && accepted,
        "complete numeric window is admitted"
    );
    kani::cover!(earliest == 1 && latest == 0 && !accepted, "inverted valid neighbors rejected");
    kani::cover!(
        earliest == LOWEST_COORDINATE - 1 && latest == 0 && !accepted,
        "only lower endpoint fails Garde"
    );
    kani::cover!(
        earliest == 0 && latest == HIGHEST_COORDINATE + 1 && !accepted,
        "only upper endpoint fails Garde"
    );
    kani::cover!(
        earliest == LOWEST_COORDINATE - 1 && latest == HIGHEST_COORDINATE + 1 && !accepted,
        "both endpoint report entries are reachable"
    );
    kani::cover!(
        earliest == i128::MIN && latest == i128::MAX && !accepted,
        "raw i128 extremes are rejected"
    );
}

/// Tightening an inclusive ordering predicate must reject the positive control.
#[kani::proof]
fn era_constructor_control_singleton() {
    assert!(EraWindow::new(0, 0).is_ok(), "singleton public window admitted");
}

/// Swapping actual retained coordinates must lose an ordinary ordered endpoint.
#[kani::proof]
fn era_constructor_control_retention() {
    match EraWindow::new(0, 1) {
        Ok(window) => assert_eq!(window.earliest().ticks(), 0, "ordered lower endpoint retained"),
        Err(_) => assert!(false, "ordinary public constructor positive control"),
    }
}

/// Garde's real invalid-input path must retain the constructor's public error.
#[kani::proof]
fn era_constructor_control_error() {
    assert_eq!(
        EraWindow::new(i128::MAX, i128::MAX).err(),
        Some(DomainError::InvalidEraWindow),
        "invalid endpoint mapped to public window error"
    );
}

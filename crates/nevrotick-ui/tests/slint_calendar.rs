//! Check the pinned Slint civil-date adapter through its generated-code exports.
//!
//! The tests exercise deterministic calendar behavior without starting a UI or
//! reading the system clock. They run with the existing UI integration suites.
#![forbid(unsafe_code)]

use slint::private_unstable_api::re_exports::{
    format_date, month_day_count, month_offset, parse_date,
};

/// Gregorian month lengths, weekday offsets and default widget formats.
///
/// # Panics
/// Fails if the patched core disagrees with the fixed civil-date examples.
#[test]
fn widget_calendar_and_default_formats() {
    for (month, days) in (1_u32..=12).zip([31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]) {
        assert_eq!(month_day_count(month, 2024), Some(days));
    }
    assert_eq!(month_day_count(2, 2000), Some(29));
    assert_eq!(month_day_count(2, 1900), Some(28));
    assert_eq!(month_offset(9, 2024), 0);
    assert_eq!(month_offset(7, 2024), 1);
    assert_eq!(format_date("%B %Y", 29, 2, 2024).as_str(), "February 2024");
    assert_eq!(
        format_date("%a, %b %d", 29, 2, 2024).as_str(),
        "Thu, Feb 29"
    );
    assert_eq!(parse_date("02/29/2024", "%m/%d/%Y"), Some([29, 2, 2024]));
    assert_eq!(parse_date("2/9/2024", "%m/%d/%Y"), Some([9, 2, 2024]));
    assert_eq!(parse_date(" 2/ 9/2024", "%m/%d/%Y"), Some([9, 2, 2024]));
    assert_eq!(parse_date("02/09/24", "%m/%d/%Y"), Some([9, 2, 24]));
    assert_eq!(
        parse_date("02/09/+00010000", "%m/%d/%Y"),
        Some([9, 2, 10_000])
    );
    assert_eq!(parse_date("2024-02-29", "%F"), Some([29, 2, 2024]));
}

/// Preserve the accepted year interval and reject invalid or conflicting inputs.
///
/// # Panics
/// Fails if a boundary date or rejection sentinel differs from the contract.
#[test]
fn calendar_rejects_invalid_and_conflicting_fields() {
    assert_eq!(month_day_count(1, -262_143), Some(31));
    assert_eq!(month_day_count(1, -262_144), None);
    assert_eq!(month_day_count(11, 262_142), Some(30));
    assert_eq!(month_day_count(12, 262_142), None);
    assert_eq!(month_day_count(1, 262_143), None);
    assert_eq!(month_day_count(12, i32::MAX), None);
    assert_eq!(month_day_count(u32::MAX, 2024), None);
    assert_eq!(format_date("%", 29, 2, 2024).as_str(), "");
    assert_eq!(format_date("%Y", 29, 2, 1900).as_str(), "");
    assert_eq!(parse_date("2024", "%"), None);
    assert_eq!(parse_date("2024-02-29 Friday", "%F %A"), None);
    assert_eq!(parse_date("2024-02-29 03", "%F %m"), None);
    assert_eq!(parse_date("02/29/1900", "%m/%d/%Y"), None);
    assert_eq!(parse_date("02/09/+ 10000", "%m/%d/%Y"), None);
    assert_eq!(parse_date("02/09/2024\n", "%m/%d/%Y"), None);
    assert_eq!(parse_date("02/29/2024x", "%m/%d/%Y"), None);
}

/// Otherwise valid dates and formats at the byte limit are still accepted.
///
/// # Panics
/// Fails if the limit rejects an allowed input or accepts a larger one.
#[test]
fn calendar_enforces_text_and_format_byte_limits() {
    let padded: String = format!("{}2/9/2024", " ".repeat(1016));
    assert_eq!(parse_date(&padded, "%m/%d/%Y"), Some([9, 2, 2024]));
    let oversized: String = format!(" {padded}");
    assert_eq!(parse_date(&oversized, "%m/%d/%Y"), None);

    // Escaped percent signs keep the matching input shorter than the format.
    let format: String = format!("%F{}", "%%".repeat(511));
    let date: String = format!("2024-02-29{}", "%".repeat(511));
    assert_eq!(parse_date(&date, &format), Some([29, 2, 2024]));
    assert_eq!(format_date(&format, 29, 2, 2024).as_str(), date);

    let oversized_format: String = format!("{format}x");
    let matching_date: String = format!("{date}x");
    assert_eq!(parse_date(&matching_date, &oversized_format), None);
    assert_eq!(format_date(&oversized_format, 29, 2, 2024).as_str(), "");
}

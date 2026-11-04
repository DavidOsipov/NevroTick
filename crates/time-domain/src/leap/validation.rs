//! Shared bounded ledger and scope rules, always reached through construction.

use super::{
    EraWindow, LEAP_BASELINE_NTP_SECONDS, LedgerValue, MAX_LEAP_NTP_SECONDS, MAX_LEAP_RECORDS,
    baseline_ticks, coordinate_ticks,
};

/// Check the full caller scope against the model baseline, without ambient time.
///
/// # Errors
/// Returns one fixed garde error when any scope date precedes the baseline.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rules require borrowed values and context"
)]
pub(super) fn scope_rule(dates: &EraWindow, (): &()) -> garde::Result {
    if coordinate_ticks(dates.earliest()) < baseline_ticks() {
        return Err(garde::Error::new("invalid leap date scope"));
    }
    Ok(())
}

/// Check every raw tuple and metadata relationship before a ledger can escape.
///
/// The count is bounded before iteration. There is no allocation, sorting or
/// mutation; failures expose no rejected value or partial checked ledger.
///
/// # Errors
/// Returns one fixed garde error for an invalid ledger or cross-field relation.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rules require borrowed values and context"
)]
pub(super) fn ledger_rule(value: &LedgerValue<'_>, (): &()) -> garde::Result {
    if !valid_ledger(value) {
        return Err(garde::Error::new("invalid leap ledger"));
    }
    Ok(())
}

/// Enforce baseline, finite coverage and all monthly signed-change invariants.
fn valid_ledger(value: &LedgerValue<'_>) -> bool {
    if value.records.is_empty() || value.records.len() > MAX_LEAP_RECORDS {
        return false;
    }
    if value.records.first() != Some(&(LEAP_BASELINE_NTP_SECONDS, 10)) {
        return false;
    }
    let reviewed_before = value.history_before.ntp_seconds();
    if reviewed_before <= LEAP_BASELINE_NTP_SECONDS {
        return false;
    }
    let known_before = value.expiry.map_or(reviewed_before, |expiry| {
        reviewed_before.max(expiry.ntp_seconds())
    });
    let mut previous = (LEAP_BASELINE_NTP_SECONDS, 10_i32);
    for &(seconds, offset) in value.records.iter().skip(1) {
        if seconds > MAX_LEAP_NTP_SECONDS || seconds <= previous.0 || seconds >= known_before {
            return false;
        }
        if !matches!(offset.checked_sub(previous.1), Some(-1 | 1)) || !is_month_start(seconds) {
            return false;
        }
        if value
            .continuous
            .is_some_and(|rule| seconds >= rule.ntp_seconds())
        {
            return false;
        }
        previous = (seconds, offset);
    }
    true
}

/// Recognize Gregorian month starts in constant work over the admitted envelope.
///
/// Input seconds are nonnegative and at most `i64::MAX`. Whole-day conversion
/// and the 400-year cycle use i128, leaving ample headroom for every intermediate.
/// A March-based cycle has fixed month-start positions, including leap years.
fn is_month_start(seconds: u64) -> bool {
    if !seconds.is_multiple_of(86_400) {
        return false;
    }
    let march_days = i128::from(seconds / 86_400) + 693_901;
    let day_in_cycle = march_days.rem_euclid(146_097);
    let year_in_cycle = (day_in_cycle - day_in_cycle / 1_460 + day_in_cycle / 36_524
        - day_in_cycle / 146_096)
        / 365;
    let day_in_year =
        day_in_cycle - (365 * year_in_cycle + year_in_cycle / 4 - year_in_cycle / 100);
    matches!(
        day_in_year,
        0 | 31 | 61 | 92 | 122 | 153 | 184 | 214 | 245 | 275 | 306 | 337
    )
}

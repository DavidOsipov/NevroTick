// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

use crate::SharedString;
use time::{
    Date, Month,
    format_description::{BorrowedFormatItem, Component, parse_strftime_borrowed},
};

#[cfg(not(feature = "std"))]
compile_error!("The NevroTick desktop Slint derivative requires the std feature.");

const MIN_YEAR: i32 = -262_143;
const MAX_YEAR: i32 = 262_142;
const MAX_DATE_TEXT_BYTES: usize = 1024;

pub fn use_24_hour_format() -> bool {
    true
}

fn calendar_date(day: u32, month: u32, year: i32) -> Option<Date> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return None;
    }
    let month: Month = Month::try_from(u8::try_from(month).ok()?).ok()?;
    Date::from_calendar_date(year, month, u8::try_from(day).ok()?).ok()
}

fn date_parts(date: Date) -> [i32; 3] {
    [
        i32::from(date.day()),
        i32::from(u8::from(date.month())),
        date.year(),
    ]
}

/// Returns the number of days in a given month.
pub fn month_day_count(month: u32, year: i32) -> Option<i32> {
    let date: Date = calendar_date(1, month, year)?;
    // The original implementation requires the next month to be representable.
    if date.year() == MAX_YEAR && date.month() == Month::December {
        return None;
    }
    Some(i32::from(time::util::days_in_month(
        date.month(),
        date.year(),
    )))
}

pub fn month_offset(month: u32, year: i32) -> i32 {
    calendar_date(1, month, year).map_or(0, |date| {
        i32::from(date.weekday().number_days_from_sunday())
    })
}

pub fn format_date(format: &str, day: u32, month: u32, year: i32) -> SharedString {
    if format.len() > MAX_DATE_TEXT_BYTES {
        return SharedString::default();
    }
    let Some(date) = calendar_date(day, month, year) else {
        return SharedString::default();
    };
    parse_strftime_borrowed(format)
        .ok()
        .and_then(|description| date.format(&description).ok())
        .map_or_else(SharedString::default, SharedString::from)
}

/// Keep the default widget's variable-width numeric input behavior.
fn parse_picker_date(text: &str) -> Option<Date> {
    fn component(text: &str, digit_limit: usize) -> Option<u32> {
        let digits: &str = text.trim_start();
        if digits.len() > digit_limit || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
            return None;
        }
        digits.parse().ok()
    }

    let mut parts: core::str::Split<'_, char> = text.split('/');
    let month: u32 = component(parts.next()?, 2)?;
    let day: u32 = component(parts.next()?, 2)?;
    let raw_year: &str = parts.next()?.trim_start();
    if parts.next().is_some() {
        return None;
    }
    let year: i32 = if let Some(digits) = raw_year.strip_prefix(['-', '+']) {
        if !digits.bytes().all(|byte| byte.is_ascii_digit()) {
            return None;
        }
        raw_year.parse().ok()?
    } else {
        i32::try_from(component(raw_year, 4)?).ok()?
    };
    calendar_date(day, month, year)
}

/// Only admit one Gregorian representation. Extra fields can otherwise be
/// ignored by time's conversion to Date, even if they contradict the result.
fn gregorian_fields(items: &[BorrowedFormatItem<'_>], seen: &mut u8) -> bool {
    for item in items {
        let field: u8 = match item {
            BorrowedFormatItem::StringLiteral(_) => continue,
            BorrowedFormatItem::Compound(parts) => {
                if !gregorian_fields(parts, seen) {
                    return false;
                }
                continue;
            }
            BorrowedFormatItem::Component(
                Component::CalendarYearFullExtendedRange(_)
                | Component::CalendarYearFullStandardRange(_),
            ) => 1,
            BorrowedFormatItem::Component(
                Component::MonthNumerical(_) | Component::MonthShort(_) | Component::MonthLong(_),
            ) => 2,
            BorrowedFormatItem::Component(Component::Day(_)) => 4,
            _ => return false,
        };
        if *seen & field != 0 {
            return false;
        }
        *seen |= field;
    }
    true
}

pub fn parse_date(date: &str, format: &str) -> Option<[i32; 3]> {
    if date.len() > MAX_DATE_TEXT_BYTES || format.len() > MAX_DATE_TEXT_BYTES {
        return None;
    }
    if format == "%m/%d/%Y" {
        return parse_picker_date(date).map(date_parts);
    }
    let description: alloc::vec::Vec<BorrowedFormatItem<'_>> =
        parse_strftime_borrowed(format).ok()?;
    let mut seen: u8 = 0;
    if !gregorian_fields(&description, &mut seen) || seen != 7 {
        return None;
    }
    let parsed: Date = Date::parse(date, &description).ok()?;
    (MIN_YEAR..=MAX_YEAR)
        .contains(&parsed.year())
        .then(|| date_parts(parsed))
}

#[cfg(feature = "std")]
pub fn date_now() -> [i32; 3] {
    time::OffsetDateTime::now_local().map_or([-1, -1, -1], |now| date_parts(now.date()))
}

#[cfg(feature = "ffi")]
mod ffi {
    #![allow(unsafe_code)]

    use super::*;

    #[unsafe(no_mangle)]
    pub extern "C" fn slint_date_time_use_24_hour_format() -> bool {
        use_24_hour_format()
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn slint_date_time_month_day_count(month: u32, year: i32) -> i32 {
        month_day_count(month, year).unwrap_or(0)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn slint_date_time_month_offset(month: u32, year: i32) -> i32 {
        month_offset(month, year)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn slint_date_time_format_date(
        format: &SharedString,
        day: u32,
        month: u32,
        year: i32,
        out: &mut SharedString,
    ) {
        *out = format_date(format, day, month, year)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn slint_date_time_date_now(d: &mut i32, m: &mut i32, y: &mut i32) {
        [*d, *m, *y] = date_now();
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn slint_date_time_parse_date(
        date: &SharedString,
        format: &SharedString,
        d: &mut i32,
        m: &mut i32,
        y: &mut i32,
    ) -> bool {
        if let Some(x) = parse_date(date, format) {
            [*d, *m, *y] = x;
            true
        } else {
            false
        }
    }
}

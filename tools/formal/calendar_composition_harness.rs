//! Four jointly required calendar theorems; no theorem is imported as an assumption.
//! March segment y contains March of year y through February of Gregorian y+1.

const CYCLE_DAYS: u32 = 146_097;

/// Independent forward Gregorian count, including the complete cycle endpoint.
fn forward_start(years: u128) -> u128 {
    assert!(
        years <= 400 * u128::from(u64::MAX) + 399,
        "forward year representation bound"
    );
    365 * years + years / 4 - years / 100 + years / 400
}

fn cycle_start(year: u32) -> u32 {
    assert!(year <= 400, "cycle year representation bound");
    365 * year + year / 4 - year / 100 + year / 400
}

fn gregorian_leap(year: u32) -> bool {
    assert!((1..=400).contains(&year), "Gregorian year domain");
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn year_length(year: u32) -> u32 {
    assert!(year < 400, "March segment year domain");
    365 + u32::from(gregorian_leap(year + 1))
}

/// Prefix sum of ordinary Gregorian month lengths rotated March through February.
/// The final February is included only for endpoint 12. No production match list
/// or inverse-year expression supplies this independently checked oracle.
fn month_prefix(month: u8, leap: bool) -> u32 {
    assert!(month <= 12, "March month endpoint domain");
    let mut days = 0;
    if month > 0 {
        days += 31;
    }
    if month > 1 {
        days += 30;
    }
    if month > 2 {
        days += 31;
    }
    if month > 3 {
        days += 30;
    }
    if month > 4 {
        days += 31;
    }
    if month > 5 {
        days += 31;
    }
    if month > 6 {
        days += 30;
    }
    if month > 7 {
        days += 31;
    }
    if month > 8 {
        days += 30;
    }
    if month > 9 {
        days += 31;
    }
    if month > 10 {
        days += 31;
    }
    if month > 11 {
        days += 28 + u32::from(leap);
    }
    days
}

/// An independent civil-month length table checks the prefix recurrence.
fn month_length(month: u8, leap: bool) -> u32 {
    assert!(month < 12, "March month domain");
    match month {
        1 | 3 | 6 | 8 => 30,
        11 => 28 + u32::from(leap),
        _ => 31,
    }
}

fn is_forward_month_start(ordinal: u32, leap: bool) -> bool {
    ordinal == month_prefix(0, leap)
        || ordinal == month_prefix(1, leap)
        || ordinal == month_prefix(2, leap)
        || ordinal == month_prefix(3, leap)
        || ordinal == month_prefix(4, leap)
        || ordinal == month_prefix(5, leap)
        || ordinal == month_prefix(6, leap)
        || ordinal == month_prefix(7, leap)
        || ordinal == month_prefix(8, leap)
        || ordinal == month_prefix(9, leap)
        || ordinal == month_prefix(10, leap)
        || ordinal == month_prefix(11, leap)
}

fn split_year(day: u32, lower: u32, upper: u32) -> (u32, u32) {
    assert!(
        lower < upper && upper <= 400,
        "forward search index bracket"
    );
    assert!(
        cycle_start(lower) <= day && day < cycle_start(upper),
        "forward search day bracket"
    );
    let middle = lower + (upper - lower) / 2;
    if cycle_start(middle) <= day {
        (middle, upper)
    } else {
        (lower, middle)
    }
}

/// Nine explicit forward-only bisections cover all 400 candidate segments.
fn represent(day: u32) -> (u32, u32) {
    assert!(day < CYCLE_DAYS, "forward cycle day domain");
    let (lower, upper) = split_year(day, 0, 400);
    let (lower, upper) = split_year(day, lower, upper);
    let (lower, upper) = split_year(day, lower, upper);
    let (lower, upper) = split_year(day, lower, upper);
    let (lower, upper) = split_year(day, lower, upper);
    let (lower, upper) = split_year(day, lower, upper);
    let (lower, upper) = split_year(day, lower, upper);
    let (lower, upper) = split_year(day, lower, upper);
    let (lower, upper) = split_year(day, lower, upper);
    assert!(
        upper == lower + 1 && lower < 400 && cycle_start(lower) <= day && day < cycle_start(upper),
        "forward search year bracket"
    );
    (lower, day - cycle_start(lower))
}

#[kani::proof]
fn calendar_source_full_domain() {
    let seconds = kani::any::<u64>();
    let mut represented = None;
    let accepted = calendar::proof_prefix(seconds, &mut represented);
    let aligned = seconds % 86_400 == 0;
    assert!(
        accepted == aligned && represented.is_some() == aligned,
        "prefix alignment result"
    );
    let whole_days = i128::from(seconds / 86_400);
    assert!(
        (0..=213_503_982_334_601).contains(&whole_days),
        "prefix whole-day envelope"
    );
    let derived_epoch = forward_start(1899) + u128::from(month_prefix(10, false));
    assert!(
        derived_epoch <= i128::MAX as u128,
        "prefix epoch conversion fits"
    );
    let absolute = whole_days + derived_epoch as i128;
    assert!(
        (693_901..=213_503_983_028_502).contains(&absolute),
        "prefix positive normalization envelope"
    );
    if let Some(day) = represented {
        // The proved positive envelope makes signed remainder the same
        // mathematical cycle normalization as unsigned or Euclidean remainder.
        let expected = absolute % i128::from(CYCLE_DAYS);
        assert_eq!(day, expected, "prefix exact epoch mapping");
        assert!(
            day >= 0 && day < i128::from(CYCLE_DAYS),
            "prefix cycle domain"
        );
    }
    // Full-u64 prefix arithmetic is machine checked here.
    // Exact H-to-prefix/suffix binding is a separately enforced, reviewed
    // fixed-source structural rule; no machine H == factor theorem is claimed.
    kani::cover!(calendar::proof_original(0), "source NTP epoch succeeds");
    kani::cover!(!calendar::proof_original(1), "source subday rejects");
    kani::cover!(
        !calendar::proof_original(u64::MAX),
        "source upper word rejects"
    );
    kani::cover!(
        calendar::proof_original(12_622_780_802_272_060_800),
        "source high month succeeds"
    );
}

#[kani::proof]
fn calendar_representation_full_domain() {
    let day = kani::any::<u32>();
    if day < CYCLE_DAYS {
        let (year, ordinal) = represent(day);
        assert_eq!(
            cycle_start(year) + ordinal,
            day,
            "cycle day has forward representation"
        );
        assert!(
            year < 400
                && ordinal < year_length(year)
                && year <= u32::from(u16::MAX)
                && ordinal <= u32::from(u16::MAX),
            "forward ordinal domain"
        );
    }
    kani::cover!(
        represent(0) == (0, 0) && calendar::proof_suffix(0),
        "forward first day represented"
    );
    kani::cover!(
        represent(146_096) == (399, 365) && !calendar::proof_suffix(146_096),
        "forward final day represented"
    );
    kani::cover!(
        represent(36_524) == (100, 0) && calendar::proof_suffix(36_524),
        "forward century day represented"
    );
    kani::cover!(
        represent(146_068) == (399, 337) && calendar::proof_suffix(146_068),
        "forward leap February start represented"
    );
}

#[kani::proof]
fn calendar_suffix_full_domain() {
    let year = u32::from(kani::any::<u16>());
    let ordinal = u32::from(kani::any::<u16>());
    if year < 400 && ordinal < year_length(year) {
        let day = cycle_start(year) + ordinal;
        assert!(day < CYCLE_DAYS, "generated cycle domain");
        assert_eq!(
            calendar::proof_suffix(i128::from(day)),
            is_forward_month_start(ordinal, gregorian_leap(year + 1)),
            "generated suffix month equivalence"
        );
    }
    kani::cover!(calendar::proof_suffix(0), "suffix cycle origin succeeds");
    kani::cover!(
        calendar::proof_suffix(36_524),
        "suffix century March succeeds"
    );
    kani::cover!(
        !calendar::proof_suffix(146_096),
        "suffix final leap day rejects"
    );
    kani::cover!(
        calendar::proof_suffix(146_068),
        "suffix February start succeeds"
    );
}

#[kani::proof]
fn calendar_gregorian_laws() {
    let year = u32::from(kani::any::<u16>()) % 400;
    let month = kani::any::<u8>() % 12;
    let cycles = kani::any::<u64>();
    let leap = gregorian_leap(year + 1);
    assert_eq!(
        cycle_start(year + 1) - cycle_start(year),
        year_length(year),
        "Gregorian segment year recurrence"
    );
    assert_eq!(
        month_prefix(month + 1, leap) - month_prefix(month, leap),
        month_length(month, leap),
        "Gregorian March month recurrence"
    );
    assert_eq!(month_prefix(0, leap), 0, "March prefix origin");
    assert_eq!(
        month_prefix(12, leap),
        year_length(year),
        "March segment total length"
    );
    assert_eq!(cycle_start(400), CYCLE_DAYS, "Gregorian cycle period");
    assert_eq!(
        forward_start(400 * u128::from(cycles) + u128::from(year)),
        u128::from(CYCLE_DAYS) * u128::from(cycles) + u128::from(cycle_start(year)),
        "arbitrary cycle forward law"
    );
    let epoch = forward_start(1899) + u128::from(month_prefix(10, false));
    assert_eq!(epoch, 693_901, "NTP March epoch anchor");
    assert_eq!(
        epoch % u128::from(CYCLE_DAYS),
        u128::from(cycle_start(299) + month_prefix(10, false)),
        "NTP cycle epoch position"
    );
    kani::cover!(
        calendar::proof_original(2_274_739_200),
        "laws ordinary leap month"
    );
    kani::cover!(
        calendar::proof_original(6_316_531_200),
        "laws non-400 century March"
    );
    kani::cover!(
        calendar::proof_original(3_160_857_600),
        "laws 400-year century March"
    );
    kani::cover!(
        forward_start(400 * u128::from(u64::MAX)) == u128::from(CYCLE_DAYS) * u128::from(u64::MAX),
        "laws maximum cycle model"
    );
}

#[kani::proof]
fn calendar_composition_control_success() {
    assert!(
        calendar::proof_original(2_274_739_200),
        "required month-start success"
    );
}

#[kani::proof]
fn calendar_composition_control_alignment() {
    assert!(!calendar::proof_original(1), "sub-day rejection");
}

#[kani::proof]
fn calendar_composition_control_century() {
    assert!(
        calendar::proof_original(6_316_531_200),
        "century month-start boundary"
    );
}

//! Full-u64 month-start equivalence with an independent January-forward oracle.
//!
//! No input assumptions are used. Gregorian calendar laws below specify the
//! oracle separately; neither private ledger admission nor Garde is projected.

const CALENDAR_CYCLE_DAYS: u32 = 146_097;
const NTP_EPOCH_ORDINAL_ZERO_BASED: u64 = 693_595;

/// Count days in complete January-based Gregorian years, beginning at year 1.
/// The bound covers every u64 cycle count and every year within its cycle.
fn forward_year_start(years: u128) -> u128 {
    assert!(
        years <= 400 * u128::from(u64::MAX) + 399,
        "forward-calendar representation bound"
    );
    365 * years + years / 4 - years / 100 + years / 400
}

/// The forward count for the first 400 years, with the final endpoint included.
fn cycle_year_start(years: u32) -> u32 {
    assert!(years <= 400, "cycle-year index bound");
    365 * years + years / 4 - years / 100 + years / 400
}

/// The ordinary Gregorian year rule, independent of the subject's inversion.
fn gregorian_leap(year: u32) -> bool {
    assert!((1..=400).contains(&year), "Gregorian cycle-year bound");
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// One explicit search step; assertions preserve, rather than assume, bracketing.
fn split_year(day: u32, lower: u32, upper: u32) -> (u32, u32) {
    assert!(lower < upper && upper <= 400, "year-search index bracket");
    assert!(
        cycle_year_start(lower) <= day && day < cycle_year_start(upper),
        "year-search day bracket"
    );
    let middle = lower + (upper - lower) / 2;
    if cycle_year_start(middle) <= day {
        (middle, upper)
    } else {
        (lower, middle)
    }
}

/// Nine steps suffice for 400 candidate years; there is no loop to unwind.
fn january_year(day: u32) -> u32 {
    assert!(day < CALENDAR_CYCLE_DAYS, "cycle-day bound");
    let (lower, upper) = split_year(day, 0, 400);
    let (lower, upper) = split_year(day, lower, upper);
    let (lower, upper) = split_year(day, lower, upper);
    let (lower, upper) = split_year(day, lower, upper);
    let (lower, upper) = split_year(day, lower, upper);
    let (lower, upper) = split_year(day, lower, upper);
    let (lower, upper) = split_year(day, lower, upper);
    let (lower, upper) = split_year(day, lower, upper);
    let (lower, upper) = split_year(day, lower, upper);
    assert!(upper == lower + 1, "year-search completed");
    assert!(
        cycle_year_start(lower) <= day && day < cycle_year_start(upper),
        "resolved January-year bracket"
    );
    lower
}

/// Days before each January-based month; February's leap day affects March on.
fn january_month_start(month: u8, leap: bool) -> u32 {
    let shift = u32::from(leap);
    match month {
        0 => 0,
        1 => 31,
        2 => 59 + shift,
        3 => 90 + shift,
        4 => 120 + shift,
        5 => 151 + shift,
        6 => 181 + shift,
        7 => 212 + shift,
        8 => 243 + shift,
        9 => 273 + shift,
        10 => 304 + shift,
        11 => 334 + shift,
        _ => {
            assert!(false, "month index must be admitted");
            0
        }
    }
}

/// Independent ordinary month lengths, used to check the prefix specification.
fn gregorian_month_length(month: u8, leap: bool) -> u32 {
    assert!(month < 12, "month-length index bound");
    match month {
        1 => 28 + u32::from(leap),
        3 | 5 | 8 | 10 => 30,
        _ => 31,
    }
}

/// Exact civil-month membership for every u64 second, including sub-day values.
fn january_oracle(seconds: u64) -> bool {
    if seconds % 86_400 != 0 {
        return false;
    }
    // u64::MAX / 86400 + 693595 = 213503983028196, so this cannot overflow.
    let day =
        ((seconds / 86_400 + NTP_EPOCH_ORDINAL_ZERO_BASED) % u64::from(CALENDAR_CYCLE_DAYS)) as u32;
    let year = january_year(day);
    let ordinal = day - cycle_year_start(year);
    let leap = gregorian_leap(year + 1);
    ordinal == january_month_start(0, leap)
        || ordinal == january_month_start(1, leap)
        || ordinal == january_month_start(2, leap)
        || ordinal == january_month_start(3, leap)
        || ordinal == january_month_start(4, leap)
        || ordinal == january_month_start(5, leap)
        || ordinal == january_month_start(6, leap)
        || ordinal == january_month_start(7, leap)
        || ordinal == january_month_start(8, leap)
        || ordinal == january_month_start(9, leap)
        || ordinal == january_month_start(10, leap)
        || ordinal == january_month_start(11, leap)
}

#[kani::proof]
fn leap_calendar_full_domain() {
    let seconds = kani::any::<u64>();
    let observed = calendar::proof_is_month_start(seconds);
    assert_eq!(
        observed,
        january_oracle(seconds),
        "Gregorian month-start equivalence"
    );
    kani::cover!(seconds == 0 && observed, "NTP epoch is a month start");
    kani::cover!(seconds == 1 && !observed, "sub-day value is rejected");
    kani::cover!(
        seconds == 2_277_158_400 && !observed,
        "1972 leap day is rejected"
    );
    kani::cover!(
        seconds == 6_316_531_200 && observed,
        "2100 March starts after an ordinary February"
    );
    kani::cover!(
        seconds == 12_622_780_802_272_060_800 && observed,
        "month start above signed-i64 is covered"
    );
    kani::cover!(
        seconds == u64::MAX && !observed,
        "raw u64 maximum is covered"
    );
}

/// Check the oracle's forward Gregorian laws, including arbitrary cycle counts.
#[kani::proof]
fn leap_calendar_oracle_laws() {
    let year = u32::from(kani::any::<u16>()) % 400;
    let month = kani::any::<u8>() % 12;
    let cycles = kani::any::<u64>();
    let leap = gregorian_leap(year + 1);
    let year_days = cycle_year_start(year + 1) - cycle_year_start(year);
    assert_eq!(
        year_days,
        365 + u32::from(leap),
        "Gregorian year recurrence"
    );
    let next_month = if month == 11 {
        year_days
    } else {
        january_month_start(month + 1, leap)
    };
    assert_eq!(
        next_month - january_month_start(month, leap),
        gregorian_month_length(month, leap),
        "Gregorian month recurrence"
    );
    assert_eq!(january_month_start(0, leap), 0, "January begins the year");
    assert_eq!(
        cycle_year_start(400),
        CALENDAR_CYCLE_DAYS,
        "400-year period length"
    );
    assert_eq!(
        forward_year_start(400 * u128::from(cycles) + u128::from(year)),
        u128::from(CALENDAR_CYCLE_DAYS) * u128::from(cycles) + u128::from(cycle_year_start(year)),
        "arbitrary-cycle forward correspondence"
    );
    assert_eq!(
        forward_year_start(1899),
        u128::from(NTP_EPOCH_ORDINAL_ZERO_BASED),
        "1900 January NTP epoch anchor"
    );
    assert_eq!(
        NTP_EPOCH_ORDINAL_ZERO_BASED % u64::from(CALENDAR_CYCLE_DAYS),
        u64::from(cycle_year_start(299)),
        "NTP epoch cycle position"
    );
    kani::cover!(year == 3 && leap, "ordinary leap year is covered");
    kani::cover!(year == 99 && !leap, "non-400 century is covered");
    kani::cover!(year == 399 && leap, "400-year leap century is covered");
    kani::cover!(
        cycles == u64::MAX,
        "arbitrary cycle-count maximum is covered"
    );
}

#[kani::proof]
fn leap_calendar_control_success() {
    assert!(
        calendar::proof_is_month_start(2_274_739_200),
        "required month-start success"
    );
}

#[kani::proof]
fn leap_calendar_control_alignment() {
    assert!(!calendar::proof_is_month_start(1), "sub-day rejection");
}

#[kani::proof]
fn leap_calendar_control_century() {
    assert!(
        calendar::proof_is_month_start(6_316_531_200),
        "century month-start boundary"
    );
}

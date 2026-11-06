// Finite Gregorian model laws only. The exact existing model helper prefix is
// supplied by the projection, and the symbolic wide-period theorem remains a
// separate required leaf. These controls alter the auxiliary model, not product H.

#[kani::proof]
fn calendar_finite_laws_full_domain() {
    let year = u32::from(kani::any::<u16>()) % 400;
    let month = kani::any::<u8>() % 12;
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
fn calendar_finite_laws_control_year() {
    assert_eq!(
        cycle_start(1) - cycle_start(0),
        year_length(0),
        "Gregorian segment year recurrence"
    );
}

#[kani::proof]
fn calendar_finite_laws_control_month() {
    assert_eq!(
        month_prefix(1, false) - month_prefix(0, false),
        month_length(0, false),
        "Gregorian March month recurrence"
    );
}

#[kani::proof]
fn calendar_finite_laws_control_epoch() {
    assert_eq!(
        forward_start(1899) + u128::from(month_prefix(10, false)),
        693_901,
        "NTP March epoch anchor"
    );
}

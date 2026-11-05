//! Full typed-domain coverage flags, with point-set and concrete-gap oracles.
//!
//! Product Garde/ledger/window constructors are omitted, not stubbed or proved.
//! The projection documents the narrow trusted prefix/view correspondence.

use leap_coverage::{LeapError, observe};

const BASE_SECONDS: u64 = 2_272_060_800;
const BASE: i128 = 2_272_060_800_i128 << 32;
const MIN_COORDINATE: i128 = -(1_i128 << 95);
const MAX_COORDINATE: i128 = (1_i128 << 95) - 1;

#[derive(Clone, Copy)]
struct Input {
    first: i128,
    last: i128,
    scope_first: i128,
    scope_last: i128,
    history: u64,
    expiry: Option<u64>,
    continuous: Option<u64>,
}

/// Generate every signed-96-bit coordinate, preserving all fractional bits.
fn coordinate() -> i128 {
    (i128::from(kani::any::<i32>()) << 64) + i128::from(kani::any::<u64>())
}

/// All absent/present values, including Some(0), inside the scalar type domain.
fn metadata() -> Option<u64> {
    let seconds = kani::any::<u64>() >> 1;
    if kani::any::<bool>() { Some(seconds) } else { None }
}

/// Independent unsigned bit concatenation, not the subject's signed multiply.
fn encoded(seconds: u64) -> i128 {
    (u128::from(seconds) << 32) as i128
}

/// Membership in the complement of the three independently supplied coverages.
/// Callers only query signed-96-bit coordinates. Negative/pre-baseline points
/// cannot lie in an admitted knowledge interval and never form a gap witness.
fn unknown_at(instant: i128, input: Input) -> bool {
    if instant < BASE {
        return false;
    }
    let whole_seconds = ((instant as u128) >> 32) as u64;
    let reviewed = whole_seconds < input.history;
    let imported = input.expiry.is_some_and(|expiry| whole_seconds < expiry);
    let continuous = input.continuous.is_some_and(|start| whole_seconds >= start);
    !reviewed && !imported && !continuous
}

/// Intersect the unknown point set with an interval by advancing a candidate
/// past each independent historical/imported endpoint. Preserve an already
/// fractional left endpoint. Every returned value is checked again by membership.
fn gap_witness(left: i128, right: i128, input: Input) -> Option<i128> {
    let mut candidate = left;
    if ((candidate as u128) >> 32) < u128::from(input.history) {
        candidate = encoded(input.history);
    }
    if let Some(expiry) = input.expiry {
        if ((candidate as u128) >> 32) < u128::from(expiry) {
            candidate = encoded(expiry);
        }
    }
    if candidate <= right && unknown_at(candidate, input) { Some(candidate) } else { None }
}

fn valid_gap(witness: Option<i128>, left: i128, right: i128, input: Input) -> bool {
    witness.is_some_and(|point| left <= point && point <= right && unknown_at(point, input))
}

/// Both directions: a completeness claim excludes EVERY symbolic unknown point;
/// an incompleteness claim must furnish an actual in-range unknown witness.
fn contract(input: Input, probe: i128) -> Option<(bool, bool)> {
    let observed = observe(input.first, input.last, input.scope_first, input.scope_last,
        input.history, input.expiry, input.continuous);
    let contained = input.scope_first <= input.first && input.last <= input.scope_last;
    assert_eq!(observed.is_ok(), contained, "scope success iff inclusive containment");
    let (offsets, events) = match observed {
        Ok(flags) => flags,
        Err(error) => {
            assert_eq!(error, LeapError::UnsupportedDates, "outside scope error classification");
            return None;
        }
    };
    let unknown = unknown_at(probe, input);
    let offset_point = BASE <= probe && probe <= input.last;
    let event_point = input.first <= probe && probe <= input.last;
    assert!(!offsets || !offset_point || !unknown,
        "offset completeness excludes every unknown point");
    assert!(!events || !event_point || !unknown,
        "event completeness excludes every unknown point");
    let offset_gap = gap_witness(BASE, input.last, input);
    let event_gap = gap_witness(input.first, input.last, input);
    assert!(offsets || valid_gap(offset_gap, BASE, input.last, input),
        "incomplete offset has a concrete gap");
    assert!(events || valid_gap(event_gap, input.first, input.last, input),
        "incomplete events have a concrete gap");
    assert!(!offsets || offset_gap.is_none(), "complete offset has no gap witness");
    assert!(!events || event_gap.is_none(), "complete events have no gap witness");
    assert!(!offsets || events, "complete offset implies complete events");
    Some((offsets, events))
}

#[kani::proof]
fn leap_coverage_full_domain() {
    let input = Input { first: coordinate(), last: coordinate(),
        scope_first: coordinate(), scope_last: coordinate(),
        history: kani::any::<u64>() >> 1, expiry: metadata(), continuous: metadata() };
    // Only existing typed invariants: no coverage result or date is assumed.
    kani::assume(input.first <= input.last);
    kani::assume(input.scope_first <= input.scope_last);
    kani::assume(input.scope_first >= BASE);
    kani::assume(input.history > BASE_SECONDS);
    let probe = coordinate();
    let result = contract(input, probe);
    let history = encoded(input.history);
    kani::cover!(input.first == MIN_COORDINATE && input.last == MIN_COORDINATE
        && result.is_none(), "signed minimum outside scope is rejected");
    kani::cover!(input.first == MAX_COORDINATE && input.last == MAX_COORDINATE
        && input.scope_last == BASE && result.is_none(), "signed maximum outside narrow scope is rejected");
    kani::cover!(input.first == BASE && input.last == BASE && input.expiry.is_none()
        && input.continuous.is_none() && result == Some((true, true)), "singleton at baseline remains covered");
    kani::cover!(input.first == history && input.last == history && input.expiry.is_none()
        && input.continuous.is_none() && result == Some((false, false)), "exclusive history cutoff is unknown");
    kani::cover!(input.first == history - 1 && input.last == history - 1 && input.expiry.is_none()
        && input.continuous.is_none() && result == Some((true, true)), "last fractional tick before cutoff is covered");
    kani::cover!(input.expiry.is_some_and(|end| end > input.history) && input.continuous.is_none()
        && input.first == history && input.last == history && result == Some((true, true)), "finite expiry extends history");
    kani::cover!(input.continuous.is_some_and(|start| start > input.history && input.first == encoded(start))
        && input.first == input.last && input.expiry.is_none() && result == Some((false, true)),
        "continuous start after gap has event-only coverage");
    kani::cover!(input.continuous == Some(input.history) && input.expiry.is_none()
        && input.first == BASE && input.last == history && result == Some((true, true)),
        "continuous rule exactly bridges history");
    kani::cover!(input.continuous == Some(0) && result == Some((true, true)), "continuous zero rule is retained");
    kani::cover!(input.expiry == Some(0) && input.continuous.is_none()
        && input.first == BASE && input.last == BASE && result == Some((true, true)), "finite zero expiry preserves history");
    kani::cover!(input.first == history + 1 && input.last == input.first && input.expiry.is_none()
        && input.continuous.is_some_and(|start| start > input.history) && result == Some((false, false)),
        "fractional candidate starts inside an unknown gap");
    kani::cover!(input.first == MAX_COORDINATE && input.last == MAX_COORDINATE
        && input.continuous == Some(i64::MAX as u64) && input.expiry.is_none()
        && input.history < i64::MAX as u64 && result == Some((false, true)),
        "upper coordinate with continuous coverage is admitted");
    kani::cover!(input.history == i64::MAX as u64 && input.expiry == Some(i64::MAX as u64)
        && input.continuous == Some(0) && input.first == BASE && input.last == BASE
        && result == Some((true, true)), "full scalar metadata endpoints are included");
}

/// Constants satisfy all view invariants and avoid calling the open constructors.
fn fixed(first: i128, last: i128, continuous: Option<u64>) -> Result<(bool, bool), LeapError> {
    observe(first, last, BASE, MAX_COORDINATE, BASE_SECONDS + 1, None, continuous)
}

#[kani::proof]
fn leap_coverage_control_exclusive_end() {
    let cutoff = encoded(BASE_SECONDS + 1);
    assert_eq!((fixed(cutoff - 1, cutoff - 1, None), fixed(cutoff + 1, cutoff + 1, None)),
        (Ok((true, true)), Ok((false, false))), "exclusive-end neighbors remain correct");
    assert_eq!(fixed(cutoff, cutoff, None), Ok((false, false)),
        "exclusive completeness endpoint remains unknown");
}

#[kani::proof]
fn leap_coverage_control_continuous_start() {
    let start = BASE_SECONDS + 3;
    let instant = encoded(start);
    assert_eq!((fixed(instant - 1, instant - 1, Some(start)), fixed(instant + 1, instant + 1, Some(start))),
        (Ok((false, false)), Ok((false, true))), "continuous-start neighbors remain correct");
    assert_eq!(fixed(instant, instant, Some(start)), Ok((false, true)),
        "continuous start is included for events");
}

#[kani::proof]
fn leap_coverage_control_gap() {
    let history = BASE_SECONDS + 1;
    let first = encoded(history);
    let last = encoded(history + 1);
    assert_eq!((fixed(first, last, Some(history)), fixed(first, last, Some(history + 2))),
        (Ok((true, true)), Ok((false, false))), "bridge-gap neighbors remain correct");
    assert_eq!(fixed(first, last, Some(history + 1)), Ok((false, false)),
        "unbridged gap remains unknown");
}

// SPDX-License-Identifier: GPL-2.0-only
// Source-only, unverified ghost specification candidate; not product code.
// Insert inside verus! after the exact Fraction, Ordering and error definitions.
// No executable/Post correspondence or callee import is established by this file.
use vstd::arithmetic::power::pow;

/// Exact positive dyadic denominator for every raw machine exponent.
spec fn bridge_denominator(shift: u32) -> int {
    pow(2, shift as nat)
}

/// Complete raw constructor admission, including every admitted zero encoding.
spec fn bridge_constructor_admitted(numerator: u64, shift: u32) -> bool {
    shift <= 1041
        && (numerator as int) < pow(2, 53)
        && (numerator == 0 || (numerator as int) < bridge_denominator(shift))
}

/// Canonical zero or a checked proper nonnegative dyadic fraction.
spec fn bridge_checked(value: Fraction) -> bool {
    value.shift <= 1041
        && (value.numerator as int) < pow(2, 53)
        && if value.numerator == 0 {
            value.shift == 0
        } else {
            1 <= value.shift
                && (value.numerator as int) < bridge_denominator(value.shift)
        }
}

/// Exact successful fields, required success, and the sole permitted rejection.
spec fn bridge_constructor_post(
    numerator: u64,
    shift: u32,
    result: &Result<Fraction, FixtureEnclosureError>,
) -> bool {
    let admitted = bridge_constructor_admitted(numerator, shift);
    match result {
        Ok(value) => {
            admitted
                && bridge_checked(*value)
                && value.numerator == numerator
                && value.shift == if numerator == 0 { 0 } else { shift }
        }
        Err(FixtureEnclosureError::Invariant) => !admitted,
        Err(_) => false,
    }
}

/// Total raw helper: invalid representations map to Equal; valid ones compare
/// exact mathematical cross products, without fixed-width alignment or rounding.
spec fn bridge_rational_order(left: Fraction, right: Fraction) -> Ordering {
    if !bridge_checked(left) || !bridge_checked(right) {
        Ordering::Equal
    } else {
        let left_scaled = (left.numerator as int) * bridge_denominator(right.shift);
        let right_scaled = (right.numerator as int) * bridge_denominator(left.shift);
        if left_scaled < right_scaled {
            Ordering::Less
        } else if left_scaled > right_scaled {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

/// All checked pairs require the exact native ordering and reject every error.
spec fn bridge_comparison_post(
    left: Fraction,
    right: Fraction,
    result: &Result<Ordering, FixtureEnclosureError>,
) -> bool {
    bridge_checked(left)
        && bridge_checked(right)
        && match result {
            Ok(order) => *order == bridge_rational_order(left, right),
            Err(_) => false,
        }
}

/// On checked input, complement succeeds exactly at or above one half and
/// returns its exact positive complement; every other admitted input is Invariant.
/// The relation is false on every result for an unchecked raw representation.
spec fn bridge_complement_post(
    input: Fraction,
    result: &Result<Fraction, FixtureEnclosureError>,
) -> bool {
    if !bridge_checked(input) {
        false
    } else {
        let denominator = bridge_denominator(input.shift);
        let required_success = 2 * (input.numerator as int) >= denominator;
        match result {
            Ok(value) => {
                required_success
                    && bridge_checked(*value)
                    && value.shift == input.shift
                    && (value.numerator as int) + (input.numerator as int) == denominator
                    && 1 <= value.numerator
                    && (value.numerator as int) <= pow(2, 52)
            }
            Err(FixtureEnclosureError::Invariant) => !required_success,
            Err(_) => false,
        }
    }
}

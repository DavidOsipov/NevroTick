// SPDX-License-Identifier: GPL-2.0-only
// Tool/API readiness control only; this proves no product Fraction contract.
use std::cmp::Ordering;
use vstd::prelude::*;

verus! {

#[derive(Clone, Copy, PartialEq, Eq)]
enum Failure {
    Invariant,
    Unrepresentable,
}

fn propagated_failure() -> (r: Result<u64, Failure>)
    ensures matches!(r, Err(Failure::Invariant)),
{
    let value = 0_u64.checked_sub(1).ok_or(Failure::Invariant)?;
    Ok(value)
}

fn compatibility_healthy() -> (r: Result<(), Failure>)
    ensures matches!(r, Ok(())),
{
    let n = 3_u64;
    let zeros = n.leading_zeros();
    assert(zeros == 62);
    assert(0_u64.leading_zeros() == 64);

    let leading = i64::from(64 - zeros) - i64::from(2_u32);
    assert(leading == 0);
    let negative = i64::from(1_u32) - i64::from(2_u32);
    assert(negative == -1);
    assert(matches!(negative.cmp(&leading), Ordering::Less));

    let shifted = n.checked_shl(1).ok_or(Failure::Invariant)?;
    assert(shifted == 6);
    let remainder = shifted.checked_sub(1).ok_or(Failure::Invariant)?;
    assert(remainder == 5);
    assert(u128::from(remainder) == 5);

    let ordering = remainder.cmp(&n);
    assert(matches!(ordering, Ordering::Greater));
    assert(ordering != Ordering::Less);
    assert(leading.cmp(&0_i64) == Ordering::Equal);

    // checked_shl rejects excessive counts; it still permits truncation.
    assert(matches!(1_u64.checked_shl(64), None));
    assert(matches!((1_u64 << 63).checked_shl(1), Some(0)));
    let mask = (1_u64 << 2) - 1;
    let masked = 7_u64 & mask;
    assert(masked == 3);

    let Some(value) = Some(remainder) else {
        return Err(Failure::Invariant);
    };
    assert(value == 5);
    assert(Failure::Invariant != Failure::Unrepresentable);

    let failure = propagated_failure();
    assert(matches!(failure, Err(Failure::Invariant)));
    Ok(())
}

// Independent detector control, with no calls or contracts used by either
// healthy function. It intentionally fails; it is not a product mutation.
fn compatibility_fault() {
    assert(false);
}

}

// SPDX-License-Identifier: GPL-2.0-only
// UNPROVED ghost-source candidate; no compiler or verifier has checked this file.
// Intended insertion inside verus!, after a separate reviewed erasure change.
// Reuses the existing bridge specification's `pow` import in that same scope.
// This file imports no product contract and changes no executable predicate.
// APIs inspected at verus-lang/verus 8bf68ee741df833ea5d0fe6a2255126fdea87e47:
// vstd/std_specs/bits.rs, arithmetic/power.rs, power2.rs and div_mod.rs.
// Still unexecuted: quotient/cast inference, opaque unfolding, recursive decrease,
// and discharge of every assertion below. API inspection is not proof evidence.

use vstd::arithmetic::div_mod::{
    lemma_div_is_strictly_smaller, lemma_fundamental_div_mod, lemma_mod_bound,
};
use vstd::arithmetic::power::{lemma_pow0, lemma_pow_positive};
use vstd::arithmetic::power2::{lemma_pow2, lemma_pow2_unfold};
use vstd::std_specs::bits::{axiom_u64_leading_zeros, u64_leading_zeros};

/// Full raw-domain bit-length/properness relation, including zero and s >= 64.
/// Pure ghost reasoning; no effects or additional admission preconditions.
/// Recursion follows n/2, so at most 64 nonzero induction steps are needed.
proof fn lemma_bridge_bit_length_power(n: u64, s: u32)
    ensures
        (64 - u64_leading_zeros(n) <= s as int)
            <==> ((n as int) < pow(2, s as nat)),
    decreases n,
{
    axiom_u64_leading_zeros(n);
    lemma_pow_positive(2, s as nat);
    lemma_pow0(2);
    reveal(u64_leading_zeros);

    if n == 0 {
        assert(u64_leading_zeros(n) == 64);
        assert(0 <= s as int);
        assert((n as int) < pow(2, s as nat));
    } else if s == 0 {
        assert(u64_leading_zeros(n) < 64);
        assert(64 - u64_leading_zeros(n) > 0);
        assert(pow(2, s as nat) == 1);
        assert((n as int) >= 1);
    } else {
        // Both casts stay within the original machine domains, without wrapping.
        assert(0 <= s as int - 1 < u32::MAX as int);
        let previous: u32 = (s - 1) as u32;
        assert(previous as int == s as int - 1);
        assert(previous as nat == (s as nat - 1) as nat);
        let half: u64 = n / 2;
        assert(half as int == (n as int) / 2);
        lemma_div_is_strictly_smaller(n as int, 2);
        assert(half < n);
        lemma_bridge_bit_length_power(half, previous);

        assert(u64_leading_zeros(n) == u64_leading_zeros(half) - 1);
        assert((64 - u64_leading_zeros(n) <= s as int)
            <==> (64 - u64_leading_zeros(half) <= previous as int));

        lemma_pow_positive(2, previous as nat);
        lemma_pow2(s as nat);
        lemma_pow2(previous as nat);
        lemma_pow2_unfold(s as nat);
        let cutoff: int = pow(2, previous as nat);
        assert(cutoff > 0);
        assert(pow(2, s as nat) == 2 * cutoff);

        // Integer remainder bounds retain the strict equality boundary.
        lemma_fundamental_div_mod(n as int, 2);
        lemma_mod_bound(n as int, 2);
        assert(n as int == 2 * (half as int) + (n as int) % 2);
        assert(0 <= (n as int) % 2 <= 1);
        assert(((half as int) < cutoff) <==> ((n as int) < 2 * cutoff)) by {
            if (half as int) < cutoff {
                assert(half as int <= cutoff - 1);
                assert(n as int <= 2 * (cutoff - 1) + 1);
                assert((n as int) < 2 * cutoff);
            } else {
                assert(half as int >= cutoff);
                assert(n as int >= 2 * cutoff);
            }
        }
        assert((64 - u64_leading_zeros(half) <= previous as int)
            <==> ((half as int) < cutoff));
        assert((64 - u64_leading_zeros(n) <= s as int)
            <==> ((n as int) < pow(2, s as nat)));
    }
}

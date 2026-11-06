// SPDX-License-Identifier: GPL-2.0-only
// UNPROVED source candidate: no compiler or verifier has checked these lemmas.
// Total ghost helpers for the original rational-order predicate; no effects,
// product imports, project assumptions, executable rewrites or preconditions.
// Pinned upstream: verus-lang/verus 8bf68ee741df833ea5d0fe6a2255126fdea87e47.

/// Every raw u64 numerator shifts losslessly in u128 for every gap below 64.
proof fn lemma_bridge_u128_shifted_u64(n: u64, g: u32)
    ensures
        g < 64 ==> ((n as u128) << g) == ((n as u128) << (g as u128)),
        g < 64 ==> (((n as u128) << g) as int == (n as int) * pow(2, g as nat)),
        g < 64 ==> (0 <= (n as int) * pow(2, g as nat) < pow(2, 127)),
{
    if g < 64 {
        vstd::arithmetic::power2::lemma2_to64();
        vstd::arithmetic::power2::lemma_pow2(64);
        vstd::arithmetic::power2::lemma_pow2(g as nat);
        vstd::arithmetic::power2::lemma_pow2(127);
        vstd::arithmetic::power::lemma_pow_positive(2, g as nat);
        assert((n as int) < pow(2, 64));
        vstd::arithmetic::mul::lemma_mul_strict_inequality(
            n as int, pow(2, 64), pow(2, g as nat));
        vstd::arithmetic::mul::lemma_mul_inequality(0, n as int, pow(2, g as nat));
        vstd::arithmetic::power::lemma_pow_adds(2, 64, g as nat);
        vstd::arithmetic::power::lemma_pow_increases(2, 64 + g as nat, 127);
        assert(0 <= (n as int) * pow(2, g as nat) < pow(2, 127));
        vstd::bits::lemma_u128_pow2_no_overflow(127);
        assert(pow(2, 127) < u128::MAX as int);
        assert((n as u128) as int == n as int);
        assert((g as u128) as nat == g as nat);
        vstd::bits::lemma_u128_shl_is_mul(n as u128, g as u128);
        assert((g < 64)
            ==> (((n as u128) << g) == ((n as u128) << (g as u128)))) by (bit_vector);
        assert(((n as u128) << g) as int == (n as int) * pow(2, g as nat));
    }
}

/// Positivity and high/low factoring hold for arbitrary raw u32 exponents.
proof fn lemma_bridge_denominator_factor(lo: u32, hi: u32)
    ensures
        0 < bridge_denominator(lo),
        0 < bridge_denominator(hi),
        lo <= hi ==> bridge_denominator(hi)
            == bridge_denominator(lo) * pow(2, (hi as int - lo as int) as nat),
{
    reveal(bridge_denominator);
    vstd::arithmetic::power::lemma_pow_positive(2, lo as nat);
    vstd::arithmetic::power::lemma_pow_positive(2, hi as nat);
    if lo <= hi {
        assert((hi as int - lo as int) as nat == (hi as nat - lo as nat) as nat);
        vstd::arithmetic::power::lemma_pow_sub_add_cancel(2, hi as nat, lo as nat);
        vstd::arithmetic::mul::lemma_mul_is_commutative(
            pow(2, (hi as int - lo as int) as nat), pow(2, lo as nat));
    }
}

/// A positive common factor preserves all three order relations over raw ints.
proof fn lemma_bridge_positive_factor_order(a: int, b: int, k: int)
    ensures
        k > 0 ==> ((a * k < b * k) <==> (a < b)),
        k > 0 ==> ((a * k > b * k) <==> (a > b)),
        k > 0 ==> ((a * k == b * k) <==> (a == b)),
{
    if k > 0 {
        if a < b {
            vstd::arithmetic::mul::lemma_mul_strict_inequality(a, b, k);
        } else if a > b {
            vstd::arithmetic::mul::lemma_mul_strict_inequality(b, a, k);
        } else {
            assert(a == b);
            assert(a * k == b * k);
        }
    }
}

/// A nonzero numerator shifted by at least 64 exceeds a checked numerator.
proof fn lemma_bridge_rational_gap_dominates(n: u64, m: u64, g: u32)
    ensures
        (1 <= n && (m as int) < pow(2, 53) && 64 <= g)
            ==> ((m as int) < (n as int) * pow(2, g as nat)),
{
    if 1 <= n && (m as int) < pow(2, 53) && 64 <= g {
        vstd::arithmetic::power::lemma_pow_positive(2, g as nat);
        vstd::arithmetic::power::lemma_pow_increases(2, 53, g as nat);
        vstd::arithmetic::mul::lemma_mul_inequality(1, n as int, pow(2, g as nat));
        assert((m as int) < pow(2, 53) <= pow(2, g as nat));
        assert(pow(2, g as nat) <= (n as int) * pow(2, g as nat));
    }
}

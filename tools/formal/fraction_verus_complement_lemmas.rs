// SPDX-License-Identifier: GPL-2.0-only
// UNPROVED ghost-source candidate; no compiler or verifier has checked this file.
// Insert inside the reviewed verus! scope, after the unchanged bit-length source.
// All raw u32/u64 values remain inputs; both conclusions have explicit guards.
// No executable code, caller contract, project assumption or precondition is added.
// APIs inspected at verus-lang/verus 8bf68ee741df833ea5d0fe6a2255126fdea87e47:
// vstd/bits.rs, arithmetic/power.rs, power2.rs and std_specs/convert.rs.
// API inspection and these annotations are not completed proof evidence.

/// Exact machine denominator only in the executable Some branch's shift window.
/// Total raw-domain ghost function; the out-of-window conclusion is vacuous.
proof fn lemma_bridge_u128_shift_power(s: u32)
    ensures
        (1 <= s && s <= 53)
            ==> ((1_u128 << s) as int == pow(2, s as nat)),
{
    if 1 <= s && s <= 53 {
        assert((s as u128) as nat == s as nat);
        vstd::bits::lemma_u128_pow2_no_overflow(s as nat);
        vstd::bits::lemma_u128_shl_is_mul(1_u128, s as u128);
        vstd::arithmetic::power2::lemma_pow2(s as nat);

        // The upstream shift theorem takes u128; the actual expression uses u32.
        // Preserve the actual expression and prove their identity explicitly.
        assert((1 <= s && s <= 53)
            ==> ((1_u128 << s) == (1_u128 << (s as u128)))) by (bit_vector);
        assert((1_u128 << s) as int == pow(2, s as nat));
    }
}

/// Checked zero and every checked shift above 53 are strictly below one half.
/// Total raw-domain ghost function; checkedness is a guard, not a precondition.
proof fn lemma_bridge_complement_reject_outside(n: u64, s: u32)
    ensures
        (bridge_checked(Fraction { numerator: n, shift: s }) && (s == 0 || s > 53))
            ==> (2 * (n as int) < bridge_denominator(s)),
{
    reveal(bridge_checked);
    reveal(bridge_denominator);
    if bridge_checked(Fraction { numerator: n, shift: s }) && (s == 0 || s > 53) {
        if s == 0 {
            assert(n == 0);
            vstd::arithmetic::power::lemma_pow0(2);
            assert(2 * (n as int) < pow(2, s as nat));
        } else {
            assert(54 <= s);
            assert((n as int) < pow(2, 53));
            vstd::arithmetic::power2::lemma_pow2(53);
            vstd::arithmetic::power2::lemma_pow2(54);
            vstd::arithmetic::power2::lemma_pow2_unfold(54);
            assert(pow(2, 54) == 2 * pow(2, 53));
            vstd::arithmetic::power::lemma_pow_increases(2, 54, s as nat);
            assert(2 * (n as int) < pow(2, 54));
            assert(2 * (n as int) < pow(2, s as nat));
        }
    }
}

/* SPDX-License-Identifier: GPL-2.0-only */
/* Compile separately for x86-64-v3 without LTO across the baseline entry. */

/* An admitted launch executes AVX2 locally and exits without starting services. */
unsigned long inert_v3_payload(void) {
    unsigned int mask;
    __asm__ volatile("vpxor %%ymm0, %%ymm0, %%ymm0\n\t"
                     "vpcmpeqd %%ymm0, %%ymm0, %%ymm0\n\t"
                     "vpmovmskb %%ymm0, %0\n\t"
                     "vzeroupper"
                     : "=r"(mask) : : "ymm0");
    return mask == 0xffffffffU ? 0UL : 2UL;
}

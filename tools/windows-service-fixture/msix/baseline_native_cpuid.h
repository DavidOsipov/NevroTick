/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef NEVROTICK_BASELINE_NATIVE_CPUID_H
#define NEVROTICK_BASELINE_NATIVE_CPUID_H

#include "inert_admission.h"

_Static_assert(sizeof(unsigned int) == 4, "CPUID registers are 32 bits");
_Static_assert(sizeof(unsigned long long) == 8, "XCR0 is 64 bits");

/* x64 supplies CPUID. The admission owner bounds feature leaves before use. */
static InertCpuid query_cpuid(unsigned int leaf) {
    InertCpuid observed;
    unsigned int unused_edx;
    __asm__ volatile("cpuid"
                     : "=a"(observed.eax), "=b"(observed.ebx), "=c"(observed.ecx), "=d"(unused_edx)
                     : "a"(leaf), "c"(0U));
    return observed;
}

/* Called only after the baseline gate observes both XSAVE and OSXSAVE. */
static unsigned long long query_xcr0(void) {
    unsigned int low;
    unsigned int high;
    __asm__ volatile("xgetbv" : "=a"(low), "=d"(high) : "c"(0U));
    return ((unsigned long long)high << 32U) | low;
}

#endif

/* SPDX-License-Identifier: GPL-2.0-only */
/* Baseline-only PE entry for the inert application in the Disabled fixture. */

#include "inert_admission.h"

_Static_assert(sizeof(unsigned long) == 4, "Windows DWORD is 32 bits");
_Static_assert(sizeof(unsigned int) == 4, "CPUID registers are 32 bits");
_Static_assert(sizeof(unsigned long long) == 8, "XCR0 is 64 bits");

__declspec(dllimport) __declspec(noreturn) void __stdcall ExitProcess(unsigned long);

/* Defined in a separate v3 translation unit. The baseline gate owns its only call. */
unsigned long inert_v3_payload(void);

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

/* The loader enters here without a CRT startup or application TLS callback. */
void __stdcall inert_entry(void) {
    const unsigned long status = inert_launch(query_cpuid, query_xcr0, inert_v3_payload);
    ExitProcess(status);
}

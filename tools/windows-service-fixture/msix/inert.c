/* SPDX-License-Identifier: GPL-2.0-only */
/* Baseline-only PE entry for the inert application in the Disabled fixture. */

#include "baseline_native_cpuid.h"

_Static_assert(sizeof(unsigned long) == 4, "Windows DWORD is 32 bits");

__declspec(dllimport) __declspec(noreturn) void __stdcall ExitProcess(unsigned long);

/* Defined in a separate v3 translation unit. The baseline gate owns its only call. */
unsigned long inert_v3_payload(void);

/* The loader enters here without a CRT startup or application TLS callback. */
void __stdcall inert_entry(void) {
    const unsigned long status = inert_launch(query_cpuid, query_xcr0, inert_v3_payload);
    ExitProcess(status);
}

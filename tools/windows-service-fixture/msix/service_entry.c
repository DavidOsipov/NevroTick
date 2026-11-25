/* SPDX-License-Identifier: GPL-2.0-only */
/* Baseline-only PE entry for either fixed laboratory service executable. */

#include "baseline_native_cpuid.h"

_Static_assert(sizeof(unsigned long) == 4, "Windows DWORD is 32 bits");
_Static_assert(sizeof(int) == 4, "the MSVC CRT startup result is 32 bits");

__declspec(dllimport) __declspec(noreturn) void __stdcall ExitProcess(unsigned long);

/* The existing CUI entry. It initializes the CRT before the Rust main. */
int __cdecl mainCRTStartup(void);

/* Called only on CPU/OS admission. Preserve the 32-bit status if CRT returns. */
__declspec(noreturn) static unsigned long start_service(void) {
    ExitProcess((unsigned long)mainCRTStartup());
}

/* The linker selects this separately compiled baseline entry for the two roles. */
void __stdcall nevrotick_service_entry(void) {
    (void)inert_launch(query_cpuid, query_xcr0, start_service);
    /* The admitted callback never returns. ERROR_NOT_SUPPORTED is a refusal. */
    ExitProcess(50UL);
}

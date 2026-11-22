/* SPDX-License-Identifier: GPL-2.0-only */
/* An accidental application launch exits without starting either service. */

_Static_assert(sizeof(unsigned long) == 4, "Windows DWORD is 32 bits");
__declspec(dllimport) __declspec(noreturn) void __stdcall ExitProcess(unsigned long);

void __stdcall inert_entry(void) {
    ExitProcess(0UL);
}

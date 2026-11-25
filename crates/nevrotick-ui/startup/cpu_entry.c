/* SPDX-License-Identifier: GPL-3.0-only */
/* GUI entry compiled separately for x86-64. No CRT state is available here. */

typedef struct {
    unsigned int eax;
    unsigned int ebx;
    unsigned int ecx;
} CpuId;

#if defined(NEVROTICK_UI_ENTRY_CONTROL)
static CpuId read_cpuid(unsigned int leaf);
static unsigned int read_xcr0(void);
#else
#if !defined(__x86_64__) || !defined(_WIN64)
#error "The NevroTick GUI entry requires the Windows x64 ABI"
#endif

static CpuId read_cpuid(unsigned int leaf) {
    CpuId registers;
    unsigned int edx;
    __asm__ volatile("cpuid"
                     : "=a"(registers.eax), "=b"(registers.ebx), "=c"(registers.ecx), "=d"(edx)
                     : "a"(leaf), "c"(0U));
    return registers;
}

/* Call only with both XSAVE and OSXSAVE set. Return low bits for the x87 and AVX checks. */
static unsigned int read_xcr0(void) {
    unsigned int low;
    unsigned int high;
    __asm__ volatile("xgetbv" : "=a"(low), "=d"(high) : "c"(0U));
    return low;
}
#endif

/* x86-64 psABI v2/v3 features, as reported by these CPUID leaves.
 * The Windows x64 execution environment already requires baseline x86-64.
 */
static int admits_v3(void) {
    if (read_cpuid(0U).eax < 7U || read_cpuid(0x80000000U).eax < 0x80000001U) {
        return 0;
    }

    const unsigned int leaf1_required =
        (1U << 0) | (1U << 9) | (1U << 12) | (1U << 13) | (1U << 19) |
        (1U << 20) | (1U << 22) | (1U << 23) | (1U << 26) |
        (1U << 27) | (1U << 28) | (1U << 29);
    const unsigned int leaf1 = read_cpuid(1U).ecx;
    if ((leaf1 & leaf1_required) != leaf1_required) {
        return 0;
    }

    const unsigned int leaf7_required = (1U << 3) | (1U << 5) | (1U << 8);
    const unsigned int leaf7 = read_cpuid(7U).ebx;
    const unsigned int ext1_required = (1U << 0) | (1U << 5);
    const unsigned int ext1 = read_cpuid(0x80000001U).ecx;
    if ((leaf7 & leaf7_required) != leaf7_required ||
        (ext1 & ext1_required) != ext1_required) {
        return 0;
    }

    return (read_xcr0() & 0x07U) == 0x07U;
}

/* This is the retained CRT entry selected for the Rust main, even in a GUI PE. */
int mainCRTStartup(void);

#if !defined(NEVROTICK_UI_ENTRY_CONTROL)
__declspec(dllimport)
#endif
_Noreturn void ExitProcess(unsigned int status);

/* Process entry. Refusal ends this process before entering the original CRT. */
void nevrotick_ui_entry(void) {
    if (admits_v3()) {
        ExitProcess((unsigned int)mainCRTStartup());
    }
    ExitProcess(50U);
}

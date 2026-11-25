/* SPDX-License-Identifier: GPL-3.0-only */
/* Hosted control for the real entry, substituting register observations and exits. */
#include <assert.h>
#include <setjmp.h>
#include <stddef.h>

#define NEVROTICK_UI_ENTRY_CONTROL
#include "cpu_entry.c"

typedef struct {
    unsigned int basic_max;
    unsigned int extended_max;
    unsigned int leaf1_ecx;
    unsigned int leaf7_ebx;
    unsigned int ext1_ecx;
    unsigned int xcr0;
} Fixture;

static Fixture fixture;
static jmp_buf entry_exit;
static unsigned int crt_calls;
static unsigned int xgetbv_calls;
static unsigned int exit_status;

static CpuId read_cpuid(unsigned int leaf) {
    switch (leaf) {
    case 0U:
        return (CpuId){fixture.basic_max, 0U, 0U};
    case 0x80000000U:
        return (CpuId){fixture.extended_max, 0U, 0U};
    case 1U:
        assert(fixture.basic_max >= leaf);
        return (CpuId){0U, 0U, fixture.leaf1_ecx};
    case 7U:
        assert(fixture.basic_max >= leaf);
        return (CpuId){0U, fixture.leaf7_ebx, 0U};
    case 0x80000001U:
        assert(fixture.extended_max >= leaf);
        return (CpuId){0U, 0U, fixture.ext1_ecx};
    default:
        assert(0 && "Unexpected CPUID leaf");
        return (CpuId){0U, 0U, 0U};
    }
}

static unsigned int read_xcr0(void) {
    assert((fixture.leaf1_ecx & 0x0C000000U) == 0x0C000000U);
    ++xgetbv_calls;
    return fixture.xcr0;
}

int mainCRTStartup(void) {
    ++crt_calls;
    return -17;
}

_Noreturn void ExitProcess(unsigned int status) {
    exit_status = status;
    longjmp(entry_exit, 1);
}

static void observe_entry(Fixture registers, unsigned int expected_crt,
                          unsigned int expected_xgetbv, unsigned int expected_exit) {
    fixture = registers;
    crt_calls = 0U;
    xgetbv_calls = 0U;
    exit_status = 0U;
    if (setjmp(entry_exit) == 0) {
        nevrotick_ui_entry();
        assert(0 && "The PE entry returned");
    }
    assert(crt_calls == expected_crt);
    assert(xgetbv_calls == expected_xgetbv);
    assert(exit_status == expected_exit);
}

int main(void) {
    const Fixture v3 = {7U, 0x80000001U, 0x3CD83201U, 0x00000128U, 0x21U, 7U};
    observe_entry(v3, 1U, 1U, 0xFFFFFFEFU);

    Fixture fault = v3;
    fault.basic_max = 6U;
    observe_entry(fault, 0U, 0U, 50U);
    fault = v3;
    fault.extended_max = 0x80000000U;
    observe_entry(fault, 0U, 0U, 50U);

    /* Independent feature-bit inventory from CPUID leaf definitions. */
    const unsigned int leaf1_bits[] = {0U, 9U, 12U, 13U, 19U, 20U, 22U, 23U,
                                       26U, 27U, 28U, 29U};
    const unsigned int leaf7_bits[] = {3U, 5U, 8U};
    const unsigned int ext1_bits[] = {0U, 5U};
    for (size_t i = 0; i < sizeof(leaf1_bits) / sizeof(leaf1_bits[0]); ++i) {
        fault = v3;
        fault.leaf1_ecx &= ~(1U << leaf1_bits[i]);
        observe_entry(fault, 0U, 0U, 50U);
    }
    for (size_t i = 0; i < sizeof(leaf7_bits) / sizeof(leaf7_bits[0]); ++i) {
        fault = v3;
        fault.leaf7_ebx &= ~(1U << leaf7_bits[i]);
        observe_entry(fault, 0U, 0U, 50U);
    }
    for (size_t i = 0; i < sizeof(ext1_bits) / sizeof(ext1_bits[0]); ++i) {
        fault = v3;
        fault.ext1_ecx &= ~(1U << ext1_bits[i]);
        observe_entry(fault, 0U, 0U, 50U);
    }

    fault = v3;
    fault.xcr0 = 1U;
    observe_entry(fault, 0U, 1U, 50U);
    fault.xcr0 = 3U;
    observe_entry(fault, 0U, 1U, 50U);
    fault.xcr0 = 5U;
    observe_entry(fault, 0U, 1U, 50U);
    fault.xcr0 = 6U;
    observe_entry(fault, 0U, 1U, 50U);
    fault.xcr0 = 0xE7U;
    observe_entry(fault, 1U, 1U, 0xFFFFFFEFU);
    return 0;
}

/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef NEVROTICK_INERT_ADMISSION_H
#define NEVROTICK_INERT_ADMISSION_H

/* Only the CPUID registers used by the fixed, subleaf-zero queries. */
typedef struct {
    unsigned int eax;
    unsigned int ebx;
    unsigned int ecx;
} InertCpuid;

typedef InertCpuid (*InertQueryCpuid)(unsigned int leaf);
typedef unsigned long long (*InertQueryXcr0)(void);
typedef unsigned long (*InertPayload)(void);

/*
 * The x64 process environment supplies the baseline ISA. Check all cumulative
 * v2/v3 features and the enabled vector state before calling the v3 body once.
 * Return 1 on refusal, without invoking the payload. The fixed providers are
 * synchronous, bounded and carry no authority beyond observing this CPU.
 */
static inline unsigned long inert_launch(
    InertQueryCpuid cpuid, InertQueryXcr0 xcr0, InertPayload payload
) {
    const unsigned int leaf1_required = 0x3cd83201U;
    const unsigned int leaf7_required = 0x00000128U;
    const unsigned int extended1_required = 0x00000021U;
    if (cpuid(0U).eax < 7U || cpuid(0x80000000U).eax < 0x80000001U) {
        return 1UL;
    }
    /* Leaf 1 includes XSAVE and OSXSAVE. A refusal here never reads XCR0. */
    if ((cpuid(1U).ecx & leaf1_required) != leaf1_required ||
        (cpuid(7U).ebx & leaf7_required) != leaf7_required ||
        (cpuid(0x80000001U).ecx & extended1_required) != extended1_required) {
        return 1UL;
    }
    if ((xcr0() & 7ULL) != 7ULL) {
        return 1UL;
    }
    return payload();
}

#endif

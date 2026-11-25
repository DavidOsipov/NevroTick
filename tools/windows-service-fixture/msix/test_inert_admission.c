/* SPDX-License-Identifier: GPL-2.0-only */
/* Portable ordered-provider controls for the exact baseline launch decision. */

#include <assert.h>
#include <stdio.h>

#include "inert_admission.h"

typedef struct {
    unsigned int basic_max;
    unsigned int extended_max;
    unsigned int leaf1;
    unsigned int leaf7;
    unsigned int extended1;
    unsigned long long vector_state;
    unsigned long payload_status;
} Fixture;

static Fixture fixture;
static unsigned int next_observation;
static unsigned int cases;

/* The observed leaf and register must both match the admitted instruction. */
static InertCpuid scripted_cpuid(unsigned int leaf) {
    static const unsigned int leaves[] = {0U, 0x80000000U, 1U, 7U, 0x80000001U};
    assert(next_observation < sizeof(leaves) / sizeof(leaves[0]));
    assert(leaf == leaves[next_observation++]);
    switch (leaf) {
        case 0U: return (InertCpuid){fixture.basic_max, ~0U, ~0U};
        case 0x80000000U: return (InertCpuid){fixture.extended_max, ~0U, ~0U};
        case 1U: return (InertCpuid){~0U, ~0U, fixture.leaf1};
        case 7U: return (InertCpuid){~0U, fixture.leaf7, ~0U};
        default: return (InertCpuid){~0U, ~0U, fixture.extended1};
    }
}

/* A refused CPU must never reach the guarded vector-state observation. */
static unsigned long long scripted_xcr0(void) {
    assert(next_observation++ == 5U);
    assert((fixture.leaf1 & (3U << 26U)) == (3U << 26U));
    return fixture.vector_state;
}

/* Record the one effect and return its independently chosen completion. */
static unsigned long scripted_payload(void) {
    assert(next_observation++ == 6U);
    return fixture.payload_status;
}

/* Reconcile both the result and the entire prefix of observed effects. */
static void check(Fixture input, unsigned int observations, unsigned long expected) {
    fixture = input;
    next_observation = 0U;
    assert(inert_launch(scripted_cpuid, scripted_xcr0, scripted_payload) == expected);
    assert(next_observation == observations);
    ++cases;
}

int main(void) {
    static const unsigned int leaf1_bits[] = {0U, 9U, 12U, 13U, 19U, 20U, 22U, 23U, 26U, 27U, 28U, 29U};
    static const unsigned int leaf7_bits[] = {3U, 5U, 8U};
    static const unsigned int extended1_bits[] = {0U, 5U};
    const Fixture healthy = {7U, 0x80000001U, 0x3cd83201U, 0x128U, 0x21U, 7ULL, 0UL};
    check(healthy, 7U, 0UL);

    Fixture input = healthy;
    input.basic_max = 6U;
    check(input, 1U, 1UL);
    input = healthy;
    input.extended_max = 0x80000000U;
    check(input, 2U, 1UL);

    for (unsigned int i = 0U; i < sizeof(leaf1_bits) / sizeof(leaf1_bits[0]); ++i) {
        input = healthy;
        input.leaf1 &= ~(1U << leaf1_bits[i]);
        check(input, 3U, 1UL);
    }
    for (unsigned int i = 0U; i < sizeof(leaf7_bits) / sizeof(leaf7_bits[0]); ++i) {
        input = healthy;
        input.leaf7 &= ~(1U << leaf7_bits[i]);
        check(input, 4U, 1UL);
    }
    for (unsigned int i = 0U; i < sizeof(extended1_bits) / sizeof(extended1_bits[0]); ++i) {
        input = healthy;
        input.extended1 &= ~(1U << extended1_bits[i]);
        check(input, 5U, 1UL);
    }

    for (unsigned long long value = 0ULL; value < 7ULL; ++value) {
        input = healthy;
        input.vector_state = value | 0x200ULL;
        check(input, 6U, 1UL);
    }
    input = (Fixture){0x20U, 0x80000008U, ~0U, ~0U, ~0U, 0x207ULL, 2UL};
    check(input, 7U, 2UL);
    printf("inert admission: %u cases passed\n", cases);
    return 0;
}

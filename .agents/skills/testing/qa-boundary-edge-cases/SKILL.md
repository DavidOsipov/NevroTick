---
name: "qa-boundary-edge-cases"
description: "Build explicit just-below, exact and just-above NevroTick boundary cases for time, resources, representation and lifecycle rules."
---

# Test exact boundary neighbors

Read [the testing contract](../CONTRACT.md).

Find the authoritative limit and its units/inclusivity before choosing cases. Use exact typed values and independent expected results; do not derive a test threshold from the production constant alone when the requirement supplies its own value.

Partition signed arithmetic/rounding, timestamp eras and ranges, 2^53 neighbors and fixed-width endpoints, NaN/infinity after computation, policy/cumulative budgets, deadline equality and generation transitions. Include raw integer spellings and trailing-message boundaries for strict JSON; no float intermediate.

For resource limits test permitted maximum and one-over across frames, clients, in-flight requests, collections, depth, strings and error payloads. Measure pre-allocation/work behavior when claimed; a post-parse reject is insufficient. For lifecycle, test cancellation immediately before/after the owned effect and lost acknowledgement.

Representation boundaries also include omitted/default/null versus explicit false, zero or empty values where their meaning differs. Use the shared [case-design recipe](../references/case-design.md) for feasible interactions between these partitions and current policy/state; an earlier unrelated rejection does not exercise the intended boundary.

Retain valid controls, exact case data and effect assertions. Use [numerical/protocol truth](../references/numerical-protocol.md); native representability/actuation limits additionally require Windows execution.

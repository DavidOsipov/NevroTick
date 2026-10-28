---
name: "qa-boundary-edge-cases"
description: "Build explicit just-below, exact and just-above NevroTick boundary cases for time, resources, representation and lifecycle rules."
---

# Test exact boundary neighbors

Read [the testing contract](../CONTRACT.md).

Find authoritative limit/units/inclusivity before cases. Use exact typed values/independent expectations; requirement-supplied threshold must not derive from production constant alone.

Partition signed arithmetic/rounding, timestamp eras and ranges, 2^53 neighbors and fixed-width endpoints, NaN/infinity after computation, policy/cumulative budgets, deadline equality and generation transitions. Include raw integer spellings and trailing-message boundaries for strict JSON; no float intermediate.

Resource limits: permitted maximum/one-over for frames, clients, in-flight requests, collections, depth, strings, error payloads. Measure claimed pre-allocation/work behavior; post-parse reject insufficient. Lifecycle: cancellation immediately before/after owned effect, lost acknowledgement.

Representation boundaries: omitted/default/null versus explicit false/zero/empty where meaning differs. Use shared [case-design recipe](../references/case-design.md) for feasible partition/current-policy/state interactions; earlier unrelated rejection does not exercise intended boundary.

Retain valid controls, exact case data and effect assertions. Use [numerical/protocol truth](../references/numerical-protocol.md); native representability/actuation limits additionally require Windows execution.

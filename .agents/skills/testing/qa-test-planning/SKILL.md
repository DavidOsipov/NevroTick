---
name: "qa-test-planning"
description: "Design a bounded, requirement-grounded test plan for a NevroTick behavior or change before choosing fixtures, environments and tools."
---

# Plan Rust and Windows tests

Read [the testing contract](../CONTRACT.md).

Start from observable requirements/actual producer → state → consumer flow; no existing-test-directory default. For nontrivial unresolved architecture use current planning owner; no second plan.

Each selected obligation: preconditions, admitted input/action, independent expected result, forbidden effects, owner, cheapest adequate evidence layer, valid/invalid controls. Include affected recovery paths/indirect consumers/exact candidate identity. Use [numerical/protocol truth](../references/numerical-protocol.md) and [Windows evidence](../references/windows-evidence.md) only where relevant.

Use [case design](../references/case-design.md) for potentially hidden optional/default/state/interacting-rule cases. Record feasible combinations/independent decision-table outcomes; directly cover known higher-order interactions. Pairwise/count-based coverage proves no completeness.

Separate test discovery/compiler/static proof/simulated behavior/real Windows protected effects. Plan reproducible fixtures/fault seams early; identify assertions needing application-action observation before harness cleanup. Bound cases/generated sequences/time/concurrency/retries/writable resources.

Map change to existing tests; add only missing evidence. Preserve release coverage/mutation obligations; no every-technique-per-file mandate. Finish plan with executable selections/explicit gaps; proposed tests have not passed.

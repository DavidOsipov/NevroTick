---
name: "qa-test-planning"
description: "Design a bounded, requirement-grounded test plan for a NevroTick behavior or change before choosing fixtures, environments and tools."
---

# Plan Rust and Windows tests

Read [the testing contract](../CONTRACT.md).

Start from observable requirements and the actual producer → state → consumer flow, not the existing test directory. For nontrivial unresolved architecture use the current planning owner rather than opening a second plan.

For each selected obligation record preconditions, admitted input/action, independent expected result, forbidden effects, owner, cheapest adequate evidence layer and valid/invalid controls. Include affected recovery paths, indirect consumers and exact candidate identity. Use [numerical/protocol truth](../references/numerical-protocol.md) and [Windows evidence](../references/windows-evidence.md) only where relevant.

Use [case design](../references/case-design.md) when optional/default values, state or interacting rules could hide cases. Record feasible combinations and independent decision-table outcomes; cover known higher-order interactions directly rather than claiming pairwise or count-based completeness.

Separate test discovery, compiler/static proof, simulated behavior and real Windows protected effects. Plan reproducible fixtures and fault seams early; identify assertions that must observe application action before harness cleanup. Set bounds for cases, generated sequences, time, concurrency, retries and writable resources.

Map the change to existing tests, then add only missing evidence. Preserve release coverage/mutation obligations without imposing every technique on each file. The plan ends with executable selections and explicit gaps, not a claim that proposed tests passed.

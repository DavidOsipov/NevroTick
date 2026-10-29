---
name: "qa-test-planning"
description: "Design a bounded, requirement-grounded test plan for a NevroTick behavior or change before choosing fixtures, environments and tools."
---

# Plan Rust and Windows tests

Read [the testing contract](../CONTRACT.md).

Start from observable requirements/actual producer → state → consumer flow; no existing-test-directory default. For nontrivial unresolved architecture use current planning owner; no second plan.

Each selected obligation: preconditions, admitted input/action, independent expected result, forbidden effects, owner, cheapest adequate evidence layer, valid/invalid controls. Include affected recovery paths/indirect consumers/exact candidate identity. Use [numerical/protocol truth](../references/numerical-protocol.md) and [Windows evidence](../references/windows-evidence.md) only where relevant.

Inventory the changed function/module contracts and their enforcing types,
mandatory constructors and live guards. Plan direct-call/bypass and transformed
output checks through [contract QA](../qa-contract-tests/SKILL.md), including a
compiling external control for claimed privacy/type restrictions. Garde field
annotations and wrapper names alone cannot discharge the obligation.

Use [case design](../references/case-design.md) for potentially hidden optional/default/state/interacting-rule cases. Record feasible combinations/independent decision-table outcomes; directly cover known higher-order interactions. Pairwise/count-based coverage proves no completeness.

Separate test discovery/compiler/static proof/simulated behavior/real Windows protected effects. Plan reproducible fixtures/fault seams early; identify assertions needing application-action observation before harness cleanup. Bound cases/generated sequences/time/concurrency/retries/writable resources.

For multiple profiles, candidates, fault controls or replay runs, use [campaign accounting](../references/campaign-budget.md) to distinguish per-invocation ceilings from complete planned and consumed work. Include product-reaching supervisor controls; keep validation-only work separate.

Use [configured analysis](../references/configured-tooling.md#additional-rust-analysis)
to select Miri for reached portable memory behavior, cargo-fuzz for hostile
parser inputs, Loom for instrumented schedules, Kani for finite properties,
Lockbud for compiler-compatible static lock screening, or cargo-vet for actual
dependency-review obligations. Name a concrete missing observation before adding
a lane; tool controls and product harness execution are distinct plan items.
For uncertain source-to-sink paths, use
[CodeQL data-flow analysis](../references/codeql-data-flow.md) and retain database,
query-model and native/process coverage gaps in the same plan.

For changed native allocation, handle, callback or cancellation ownership, use
[Windows memory diagnostics](../references/windows-memory.md). Plan the reached
API/allocator paths, discriminating controls, intended test token and one owner
for profile setup/restoration. Separately define retained-resource bounds and
release observations across repeats/failure/recovery before process teardown;
instrumented fault detection does not replace uninstrumented CHECK-21 budgets.
Record unsupported paths and unavailable Windows execution as remaining gaps.

Map change to existing tests; add only missing evidence. Preserve release coverage/mutation obligations; no every-technique-per-file mandate. Finish plan with executable selections/explicit gaps; proposed tests have not passed.

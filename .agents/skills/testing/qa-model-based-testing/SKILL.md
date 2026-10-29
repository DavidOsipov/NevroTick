---
name: "qa-model-based-testing"
description: "Test NevroTick controller, authority and recovery sequences against an independently defined transition model."
---

# Compare state transitions with an independent model

Read [the testing contract](../CONTRACT.md).

Define small requirements-derived model; no production reducer helpers. Specify states/actions/preconditions/expected outputs/effects/forbidden transitions. Useful dimensions: policy/clock generations, source eligibility, pending proposals/acknowledgements, authority budgets, cancellation, recovery ownership.

Run bounded explicit/generated command sequences through production entry point. Compare model/application after every action; invalid commands must exercise documented rejection/no-state-change, never silently skip. Track eligible/executed actions; empty path cannot pass.

When creating/changing the driver or trace comparator, use [harness qualification](../references/harness-qualification.md) for independently specified input identity, clock, ordinal and comparison controls. Agreement with the model is insufficient if the driver delivered a different action.

Model acknowledgement loss, stale late completion, reconnect/restart/suspend/successor ownership. Separate UTC/monotonic/active-awake clocks. Minimize/replay failing concrete sequences from fresh state; preserve failure seam.

A sequential state model does not prove thread schedules or Windows OS behavior. Combine necessary [concurrency](../qa-state-concurrency-lifecycle/SKILL.md) and [native evidence](../references/windows-evidence.md) explicitly. Read [Rust techniques](../references/rust-techniques.md) for optional instrumented models and limits.

For a synchronization/property gap, select admitted Loom or Kani through
[configured analysis](../references/configured-tooling.md#additional-rust-analysis):
`python3 -B scripts/check-analysis.py --tool loom --fixtures` or the corresponding
`--tool kani --fixtures` controls. Loom requires instrumented primitives and a
bounded state space; Kani requires a reachable property harness, justified
assumptions and unwind bounds. Bootstrap controls prove no application model.
Keep the requirements-derived oracle independent and record all reachable,
excluded, incomplete and failed obligations before claiming model coverage.

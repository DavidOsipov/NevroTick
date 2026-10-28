---
name: "qa-model-based-testing"
description: "Test NevroTick controller, authority and recovery sequences against an independently defined transition model."
---

# Compare state transitions with an independent model

Read [the testing contract](../CONTRACT.md).

Define small requirements-derived model; no production reducer helpers. Specify states/actions/preconditions/expected outputs/effects/forbidden transitions. Useful dimensions: policy/clock generations, source eligibility, pending proposals/acknowledgements, authority budgets, cancellation, recovery ownership.

Run bounded explicit/generated command sequences through production entry point. Compare model/application after every action; invalid commands must exercise documented rejection/no-state-change, never silently skip. Track eligible/executed actions; empty path cannot pass.

Model acknowledgement loss, stale late completion, reconnect/restart/suspend/successor ownership. Separate UTC/monotonic/active-awake clocks. Minimize/replay failing concrete sequences from fresh state; preserve failure seam.

A sequential state model does not prove thread schedules or Windows OS behavior. Combine necessary [concurrency](../qa-state-concurrency-lifecycle/SKILL.md) and [native evidence](../references/windows-evidence.md) explicitly. Read [Rust techniques](../references/rust-techniques.md) for optional instrumented models and limits.

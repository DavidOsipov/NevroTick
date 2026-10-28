---
name: "qa-model-based-testing"
description: "Test NevroTick controller, authority and recovery sequences against an independently defined transition model."
---

# Compare state transitions with an independent model

Read [the testing contract](../CONTRACT.md).

Define a small model from requirements, not production reducer helpers. Specify states, actions, preconditions, expected outputs/effects and forbidden transitions. Useful dimensions include policy/clock generations, source eligibility, pending proposals/acknowledgements, authority budgets, cancellation and recovery ownership.

Run bounded explicit or generated command sequences through the production entry point. Compare model and application after every action; invalid commands should exercise documented rejection/no-state-change behavior rather than being silently skipped. Track eligible/executed actions so an empty path cannot pass.

Model acknowledgement loss, stale late completion, reconnect, restart, suspend and successor ownership. Keep UTC/monotonic/active-awake clocks separate. Minimize and replay failing concrete sequences from fresh state without repairing away the failure seam.

A sequential state model does not prove thread schedules or Windows OS behavior. Combine necessary [concurrency](../qa-state-concurrency-lifecycle/SKILL.md) and [native evidence](../references/windows-evidence.md) explicitly. Read [Rust techniques](../references/rust-techniques.md) for optional instrumented models and limits.

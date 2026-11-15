---
name: "qa-model-based-testing"
description: "Test NevroTick controller, authority and recovery sequences against an independent transition model, with conditional proptest-state-machine generation and shrinking."
---

# Compare state transitions with an independent model

Formal/Kani proof work and pending proof admission/metadata repairs are deferred
until after V1 under [OD-10 §14.1](../../../../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026).
The tool capabilities below create no pre-V1 proof execution or acceptance gate.

Read [the testing contract](../CONTRACT.md).

Define small requirements-derived model; no production reducer helpers. Specify states/actions/preconditions/expected outputs/effects/forbidden transitions. Useful dimensions: policy/clock generations, source eligibility, pending proposals/acknowledgements, authority budgets, cancellation, recovery ownership.

Run bounded explicit/generated command sequences through production entry point. Compare model/application after every action; invalid commands must exercise documented rejection/no-state-change, never silently skip. Track eligible/executed actions; empty path cannot pass.

Generated Rust histories: read [Proptest state machines](../references/proptest-state-machines.md).
Until framework and graph admission, use existing deterministic sequences.
Implement explicit invariants; fresh-state cleanup for every case, shrink and
replay. Retain concrete original/minimized histories; persisted seeds cannot
identify the same input after strategy changes.

When creating/changing the driver or trace comparator, use [harness qualification](../references/harness-qualification.md) for independently specified input identity, clock, ordinal and comparison controls. Agreement with the model is insufficient if the driver delivered a different action.

Model acknowledgement loss, stale late completion, reconnect/restart/suspend/successor ownership. Separate UTC/monotonic/active-awake clocks. Minimize/replay failing concrete sequences from fresh state; preserve failure seam.

A sequential state model does not prove thread schedules or Windows OS behavior. Combine necessary [concurrency](../qa-state-concurrency-lifecycle/SKILL.md) and [native evidence](../references/windows-evidence.md) explicitly. Read [Rust techniques](../references/rust-techniques.md) for optional instrumented models and limits.

For a concrete search gap between meaningful states sharing familiar branches,
use [state-guided fuzzing](../qa-state-guided-fuzzing/SKILL.md). Keep the model as
the oracle and derive exploration feedback from actual target observations.

Synchronization gap: select admitted Loom through
[configured analysis](../references/configured-tooling.md#additional-rust-analysis):
`python3 -B scripts/check-analysis.py --tool loom --fixtures` supplies tool controls,
then an actual model with instrumented primitives and bounded state space.
Bootstrap controls prove no application model. Kani remains deferred; use the
[formal workflow](../references/formal-verification.md) only on authorized post-V1 resumption.
Keep the requirements-derived oracle independent and record all reachable,
excluded, incomplete and failed obligations before claiming model coverage.

When asynchronous effect/acknowledgement/recovery complexity justifies a reusable
executable specification, consider [Quint trace conformance](../references/quint-trace-conformance.md).
Keep this conditional: model checking, Rust trace replay and actual Windows
effects supply distinct evidence; an existing small model needs no second stack.

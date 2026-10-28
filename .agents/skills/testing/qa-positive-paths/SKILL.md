---
name: "qa-positive-paths"
description: "Choose positive controls and expected successful NevroTick paths that prevent rejection-only or disabled-effect implementations from appearing correct."
---

# Verify legitimate successful behavior

Read [the testing contract](../CONTRACT.md).

Identify the legitimate caller, required current state and exact expected successful transition. Use normal constructors and real entry points, with independent expected values and acknowledged effects.

Cover representative valid operating states and important edge-valid inputs: ordinary and maximum allowed frames/policies, current authorized correction, supported persisted version and valid authenticated packet. Keep source identity, units, freshness and policy budgets visible.

Pair rejection tests with the closest valid neighbor; ensure the handler actually admits it and performs the promised effect exactly once. A stub returning success without publication or acknowledgement is insufficient. For optional recovery, show ordinary reacquisition still works.

Select the cheapest evidence layer reaching the claim. Pure valid controls cannot prove real Windows privilege/IPC behavior; use [Windows evidence](../references/windows-evidence.md) when needed.

---
name: "qa-positive-paths"
description: "Choose positive controls and expected successful NevroTick paths that prevent rejection-only or disabled-effect implementations from appearing correct."
---

# Verify legitimate successful behavior

Read [the testing contract](../CONTRACT.md).

Identify legitimate caller, required current state, exact expected successful transition. Use normal constructors/real entry points, independent expectations, acknowledged effects.

Cover representative valid states/important edge-valid inputs: ordinary/maximum allowed frames/policies, current authorized correction, supported persisted version, valid authenticated packet. Keep source identity/units/freshness/policy budgets visible.

Pair rejection with closest valid neighbor; verify handler admits it/performs promised effect exactly once. Success stub without publication/acknowledgement insufficient. Optional recovery: show ordinary reacquisition still works.

Select cheapest claim-reaching evidence layer. Pure valid controls cannot prove real Windows privilege/IPC behavior; use [Windows evidence](../references/windows-evidence.md) when needed.

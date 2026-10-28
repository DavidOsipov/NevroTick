---
name: "qa-context-checkpoint"
description: "Checkpoint or resume substantial NevroTick verification with current scope, evidence, owners, live effects and consumed budgets intact."
---

# Preserve verification continuity

Read [the testing contract](../CONTRACT.md).

For multi-worker, multi-phase or interrupted work maintain one discoverable coordinator index and one current owned state note per active worker. Use the existing task record/location where adequate; small local tasks need no extra report tree.

Record requirements/acceptance set, source/dirty/configuration/fixture/candidate identities, admitted effects, resource owners, exact completed commands/results, raw evidence paths, first failures/retries, unresolved findings, live processes/tasks and consumed time/attempt/case budgets. State the next concrete action.

For interrupted diagnosis, retain the last discriminating observation, still-open hypotheses and approaches ruled out by evidence. Carry observed method or harness failures forward with their avoidance or recovery action; distinguish those observations from untested concerns. Recheck the next action against these constraints before repeating an earlier approach.

On resume reconcile actual files/index, running jobs, machine/service/clock state, ownership and artifacts before continuing. Revalidate only changed prerequisites; old reports do not prove unchanged candidate bytes. Preserve aggregate budget consumption and pending cleanup.

Keep unfinished recovery state and evidence available. The coordinator may remove only registered disposable coordination notes after accepted handoff/closeout and retention of required results. Do not delete user receipts, force compaction or restart completed work merely because a phase ended.

The observed-failure handoff detail adapts GSD's pinned
[pause-work method](https://github.com/gsd-build/get-shit-done/blob/bdcaab2c752d9a33a1a1ca9acf3a3c81fb991815/get-shit-done/workflows/pause-work.md); retain its
[MIT notice](../../provenance/GSD-LICENSE.txt).

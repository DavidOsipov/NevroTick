---
name: "qa-context-checkpoint"
description: "Checkpoint or resume substantial NevroTick verification with current scope, evidence, owners, live effects and consumed budgets intact."
---

# Preserve verification continuity

Read [the testing contract](../CONTRACT.md).

For multi-worker/multi-phase/interrupted work keep one discoverable coordinator index/one current owned state note per active worker. Reuse adequate task record/location; small local tasks need no extra report tree.

Record requirements/acceptance set, source/dirty/configuration/fixture/candidate identities, admitted effects, resource owners, exact completed commands/results, raw evidence paths, first failures/retries, unresolved findings, live processes/tasks and consumed time/attempt/case budgets. State the next concrete action.

For repeated review findings, use the shared
[finding lifecycle](../../planning/references/review-and-handoff.md#preserve-finding-identity-and-resolution-evidence)
in this record: stable issue IDs, raw-result links, disposition, verification and
invalidation conditions. Retain reviewer completion and unresolved work; preserve
the distinction between an applied edit, a verified fix and an owner decision.

Interrupted diagnosis: retain last discriminating observation/open hypotheses/evidence-ruled-out approaches. Carry observed method/harness failures with avoidance/recovery action; distinguish untested concerns. Before repeating approach, recheck next action against constraints.

On resume reconcile actual files/index/running jobs/machine/service/clock state/ownership/artifacts before continuing. Revalidate only changed prerequisites; old reports prove no unchanged candidate bytes. Preserve aggregate budget consumption/pending cleanup.

Keep unfinished recovery state/evidence available. Coordinator may remove only registered disposable coordination notes after accepted handoff/closeout and required-result retention. No deleting user receipts/forced compaction/restarting completed work merely at phase end.

The observed-failure handoff detail adapts GSD's pinned
[pause-work method](https://github.com/gsd-build/get-shit-done/blob/bdcaab2c752d9a33a1a1ca9acf3a3c81fb991815/get-shit-done/workflows/pause-work.md); retain its
[MIT notice](../../provenance/GSD-LICENSE.txt).

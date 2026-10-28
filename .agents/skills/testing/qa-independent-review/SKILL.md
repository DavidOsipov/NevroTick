---
name: "qa-independent-review"
description: "Independently review NevroTick verification claims, tests and candidate changes against requirements and actual evidence."
---

# Review evidence independently

Read [the testing contract](../CONTRACT.md).

Receive the bounded requirement/scope, frozen candidate inputs and raw evidence; avoid treating the implementer's conclusion as an oracle. Inspect the actual changed path, test selection, expectations, valid controls, effect reachability and remaining platform gaps.

Distinguish source reasoning, proposed tests, executed host tests and native Windows observations. Challenge zero-selection success, setup failures called RED, shared-oracle round trips, mocks standing in for OS authority, stale candidate identity and cleanup/retry omissions.

For a plausible finding, report location, violated guarantee, reachable scenario, consequence, evidence and uncertainty. Use a bounded discriminating check when permitted; do not speculate into unrelated repairs or claim statistical independence from reviewer agreement.

Return a scoped verdict with unresolved required evidence and material findings. Independent review supports [final qualification](../qa-final-qualification/SKILL.md), but does not multiply executions or replace missing runtime checks.

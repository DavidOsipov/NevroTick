---
name: "qa-independent-review"
description: "Independently review NevroTick verification claims, tests and candidate changes against requirements and actual evidence."
---

# Review evidence independently

Read [the testing contract](../CONTRACT.md).

Receive bounded requirement/scope, frozen candidate inputs, raw evidence; implementer conclusion is no oracle. Inspect actual changed path/test selection/expectations/valid controls/effect reachability/remaining platform gaps.

Distinguish source reasoning/proposed tests/executed host tests/native Windows observations. Challenge zero-selection success, setup-as-RED, shared-oracle round trips, mocks-as-OS-authority, stale candidate identity, cleanup/retry omissions.

For changed function/module contracts, follow
[contract enforcement](../../implementation/implement-rust-contracts/references/contract-enforcement.md)
from requirement to enforcing owner and real consumer. Independently challenge
omitted garde invocation, original-DTO use, constructor/mutation bypasses,
unchecked transformed outputs, release-disabled rejection and stale context.
Review each `skip` justification; reject `allow_unvalidated`. Confirm valid
neighboring controls and intended external compile diagnostics, not only attributes
or tests that manually validate before calling an unchecked API.

For analyzer-derived claims, separate tool controls from actual product
harness/database execution and inspect bounds, assumptions and excluded paths.
Apply [CodeQL data-flow evidence checks](../references/codeql-data-flow.md) when
paths or empty results support a conclusion: verify source/database/query/model
identity, real source/sink reachability and guard semantics yourself. A diagram,
query compilation or alert count is no independent reproduction of a defect.

Plausible finding: location/violated guarantee/reachable scenario/consequence/evidence/uncertainty. Use permitted bounded discriminating check; no speculative unrelated repairs or reviewer-agreement-as-statistical-independence claim.

Return a scoped verdict with unresolved required evidence and material findings. Independent review supports [final qualification](../qa-final-qualification/SKILL.md), but does not multiply executions or replace missing runtime checks.

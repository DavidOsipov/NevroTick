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
For mathematical proof claims, use the [proof workflow](../references/formal-verification.md)
to challenge semantic strength, actual-operation reachability, constructor/callee
composition and expected-versus-completed scope independently of harness names.
Identify the named useful product-operation guarantee and independent specification.
Check that each helper closes a real requirement or a necessary declared dependency,
with a credible route through the remaining composition. Preserve useful local
results while keeping the product guarantee open; do not require a discovered bug
as proof of value. Reconcile total preparation/tooling/solver effort and stopped
attempts; reject an unrelated easy theorem or green infrastructure as completion.
Check report consumers against genuine emitted fixtures or the pinned exporter
schema, not mutually confirming mocks. Separate attempted/completed/qualified jobs,
unique guarantees, semantic controls and tool-only checks. A historical fixed
campaign is not a repeatable current-candidate regression gate. Retain gaps in the
[post-V1 debt record](../../../../tasks/development-backlog.md#product-guarantees-still-open);
review grants no execution or consumer-repair exception to the V1 deferral.

Apply [CodeQL data-flow evidence checks](../references/codeql-data-flow.md) when
paths or empty results support a conclusion: verify source/database/query/model
identity, real source/sink reachability and guard semantics yourself. A diagram,
query compilation or alert count is no independent reproduction of a defect.

Plausible finding: location/violated guarantee/reachable scenario/consequence/evidence/uncertainty. Use permitted bounded discriminating check; no speculative unrelated repairs or reviewer-agreement-as-statistical-independence claim.

Return a scoped verdict with unresolved required evidence and material findings. Independent review supports [final qualification](../qa-final-qualification/SKILL.md), but does not multiply executions or replace missing runtime checks.

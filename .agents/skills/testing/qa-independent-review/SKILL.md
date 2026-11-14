---
name: "qa-independent-review"
description: "Independently review NevroTick verification claims, tests and candidate changes against requirements and actual evidence."
---

# Review evidence independently

Read [the testing contract](../CONTRACT.md).

Receive bounded requirement/scope, frozen candidate inputs, raw evidence; implementer conclusion is no oracle. Inspect actual changed path/test selection/expectations/valid controls/effect reachability/remaining platform gaps.

Distinguish source reasoning/proposed tests/executed host tests/native Windows observations. Challenge zero-selection success, setup-as-RED, shared-oracle round trips, mocks-as-OS-authority, stale candidate identity, cleanup/retry omissions.

## Compare verification criteria with the accepted baseline

Compare candidate verification changes with the last accepted baseline and
authoritative requirements; passing candidate output is insufficient.
Cover semantic expected values and tolerances; test, feature, target and
job selection; critical-file lists, thresholds and denominators; mutation
exclusions and equivalent dispositions; incomplete-to-pass reclassification;
and checker or workflow acceptance logic. Record affected surfaces in a small
table in the existing review/task record:

| Surface | Accepted baseline | Candidate | Authority and independent evidence |
| --- | --- | --- | --- |
| Source/hash binding | Exact admitted inputs | Exact new inputs | Rebind through existing admission; retain unchanged acceptance criteria. |
| Acceptance criterion | Exact old value, selection or rule | Exact proposed value, selection or rule | Name authoritative reason; retain counterexample and healthy control. |

Separate legitimate source/hash rebinding from acceptance changes. Compare old
and new criteria; never blindly run an old source-bound verifier on new bytes or
weaken custody rejection. Execute with authorized current binding. A candidate-
supplied or changed checker cannot independently authorize its own acceptance;
inspect semantic delta and authority. Inherited failure, timeout or missing
required result stays incomplete unless the requirement and evidence support
another disposition. Existing owner/decision rules govern material changes; no
universal human approval or server-policy change.

## Review lint-suppression changes

Inspect new or widened suppressions: inner `#![allow(...)]`,
outer `#[allow(...)]`, `#[expect(...)]`, lint groups, module scope and active
`cfg_attr` branches. [Clippy's `allow_attributes`](https://clippy.rust-lang.org/rust-1.100.0/index.html#allow_attributes)
does not cover inner allow attributes. Rust permits [reason-bearing lint attributes](https://doc.rust-lang.org/reference/attributes/diagnostics.html#lint-reasons),
including allows; a reason proves neither necessity nor narrow scope.
Check the named lint, exact item/target/features, justification and stale-
expectation behavior. Group/module expectations may stay fulfilled while hiding
additional findings; warning-free output proves no narrow exception. Preserve
legitimate admitted `cfg_attr` and generated Slint's named, scoped expectations.
Do not replace them with blanket group exceptions or reject them merely for
requiring a generated-code exception.

If recurring suppression or acceptance drift reveals a concrete automation gap,
prefer bounded source-aware checks in existing tooling with paired bad/healthy
controls. Account for comments/strings, nested attributes, conditional activation
and generated-source ownership; substring matches prove no effective policy.
Name unsupported syntax and coverage limits. No check claimed implemented;
no new parser framework or dependency required.

## Challenge the changed guarantee

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
Check each helper closes a real requirement or necessary declared dependency,
with a credible remaining composition route. Preserve useful local results and
keep the product guarantee open; no discovered bug required to prove value.
Reconcile total preparation/tooling/solver effort and stopped
attempts; reject an unrelated easy theorem or green infrastructure as completion.
Check report consumers against genuine emitted fixtures or pinned exporter
schema, not mutually confirming mocks. Separate attempted/completed/qualified jobs,
unique guarantees, semantic controls and tool-only checks. A historical fixed
campaign is no repeatable current-candidate regression gate. Retain gaps in the
[post-V1 debt record](../../../../tasks/development-backlog.md#product-guarantees-still-open);
review grants no execution or consumer-repair exception to the V1 deferral.

Apply [CodeQL data-flow evidence checks](../references/codeql-data-flow.md) when
paths or empty results support conclusions: verify source/database/query/model
identity, real source/sink reachability and guard semantics yourself. A diagram,
query compilation or alert count is no independent reproduction of a defect.

Plausible finding: location/violated guarantee/reachable scenario/consequence/evidence/uncertainty. Use permitted bounded discriminating check; no speculative unrelated repairs or reviewer-agreement-as-statistical-independence claim.

Return a scoped verdict with unresolved required evidence and material findings. Independent review supports [final qualification](../qa-final-qualification/SKILL.md), but does not multiply executions or replace missing runtime checks.

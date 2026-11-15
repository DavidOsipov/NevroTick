---
name: "qa-independent-review"
description: "Independently review NevroTick verification claims, tests and candidate changes against requirements and actual evidence."
---

# Review evidence independently

Read [testing contract](../CONTRACT.md).

Receive bounded requirements/scope, frozen inputs, raw evidence. Implementer
conclusion is no oracle. Inspect changed paths, selection, expectations, valid
controls, reached effects and platform gaps.

Logical/long chains: challenge [flow/invariant audit](../references/flow-audit.md)
against actual callers, complete relevant bodies, boundary contracts. Independently
seek omitted roots, alternate constructors, bypasses, errors, changing context.
Trace checked versus consumed object, exact units, ownership, acknowledgements,
original failure through cleanup. Require source-backed continuation obligations;
review slice integration, not just local verdicts. Check reuse against callers,
callee contracts, configuration/evidence identity. Ledger proves no correctness
and adds no approval gate.

Separate source reasoning, proposed tests, host execution, native observations.
Challenge zero selection, setup-as-RED, shared-oracle round trips, mocks-as-authority,
stale identity and omitted cleanup/retries.

## Compare verification criteria with the accepted baseline

Compare candidate with accepted baseline/requirements, not merely passing output:
expected values/tolerances; tests/features/targets/jobs; critical files, thresholds,
denominators; mutation exclusions/equivalences; incomplete-to-pass changes;
checker/workflow acceptance. Record in existing review/task:

| Surface | Accepted baseline | Candidate | Authority and independent evidence |
| --- | --- | --- | --- |
| Source/hash binding | Exact admitted inputs | Exact new inputs | Existing re-admission; unchanged acceptance criteria. |
| Acceptance criterion | Exact old value, selection or rule | Exact proposed value, selection or rule | Authoritative reason, counterexample, healthy control. |

Separate hash/source rebinding from acceptance changes. Compare old/new criteria;
never apply stale source-bound verifier blindly or weaken custody rejection.
Execute authorized current binding. Changed/candidate checker cannot authorize
itself: inspect semantic delta/authority. Inherited failure, timeout or missing
required evidence stays incomplete absent supporting requirement/evidence.
Existing owner/decision rules govern material changes; no universal human approval
or server-policy change.

## Review lint-suppression changes

Inspect new/widened inner `#![allow(...)]`, outer `#[allow(...)]`,
`#[expect(...)]`, groups/module scope and active `cfg_attr`.
[Clippy's `allow_attributes`](https://clippy.rust-lang.org/rust-1.100.0/index.html#allow_attributes)
misses inner allows. Rust [reason-bearing lint attributes](https://doc.rust-lang.org/reference/attributes/diagnostics.html#lint-reasons)
include allows; reason proves neither necessity nor narrowness.
Check lint, item/target/features, justification, stale-expectation behavior.
Group/module expectations can remain fulfilled while hiding other findings;
warning-free proves no narrow exception. Preserve legitimate `cfg_attr` and
generated Slint's named scoped expectations; neither blanket group replacements
nor rejection solely for generated-code exception.

Recurring suppression/acceptance drift: bounded source-aware existing-tool checks,
paired bad/healthy controls. Account for comments/strings, nesting, conditional
activation, generated ownership. Substrings prove no effective policy. Name syntax/
coverage gaps; claim no implementation, require no new parser/dependency framework.

## Challenge the changed guarantee

Generated histories/schedules: inspect delivered actions, instrumented primitives,
observed state/effects, independent oracle. Require healthy/requirement-linked
fault controls, fresh concrete replay, exact source/configuration, honest selected/
completed counts. Seeds/sampled schedules/minimized model traces prove neither
untouched production nor missing real-runtime/native behavior. Select relevant
[state-machine](../references/proptest-state-machines.md),
[Shuttle](../references/shuttle-concurrency.md) or
[Python admission](../references/hypothesis-admission.md) reference.

Changed contracts: follow [contract enforcement](../../implementation/implement-rust-contracts/references/contract-enforcement.md)
from requirement to enforcing owner/actual consumer. Challenge missing garde call,
original DTO, constructor/mutation bypass, unchecked transformed output,
release-disabled rejection, stale context. Review each `skip`; reject
`allow_unvalidated`. Require legitimate neighbors and intended external compile
diagnostics; attributes/manual prevalidation of unchecked APIs are insufficient.

Analyzer claims: separate tool controls from actual product/database execution;
inspect bounds, assumptions, excluded paths. Proof claims: [proof workflow](../references/formal-verification.md)
for semantic strength, actual-operation reachability, constructor/callee composition,
expected/completed scope independent of harness names. Name useful product guarantee
and independent specification. Helpers must close requirement/necessary declared
dependency with credible composition route. Preserve local results; parent remains
open, no bug required for value. Account total prep/tooling/solver effort/stops;
reject unrelated easy theorem/green infrastructure as completion.
Check consumers against genuine emitted fixtures/pinned exporter schema, not mutually
confirming mocks. Separate attempts/completions/qualified jobs, unique guarantees,
semantic controls, tooling. Historical fixed campaign is no current repeatable gate.
Retain [post-V1 debt](../../../../tasks/development-backlog.md#product-guarantees-still-open);
review lifts no V1 execution/consumer-repair deferral.

For paths/empty results, apply [CodeQL evidence checks](../references/codeql-data-flow.md):
verify source/database/query/models, real endpoints and guards yourself. Diagram,
compilation or alert count is no independent defect reproduction.

Plausible finding: location, violated guarantee, reachable scenario, consequence,
evidence, uncertainty. Use authorized bounded discriminating check. No speculative
unrelated repair or reviewer-agreement-as-statistical-independence claim.

Return scoped verdict, unresolved required evidence, material findings. Support
[final qualification](../qa-final-qualification/SKILL.md); do not multiply executions
or replace missing runtime checks.

---
name: "plan-independent-review"
description: "Independently review a NevroTick plan or revised stage for missing consumers, unsafe assumptions, weak evidence and unavailable prerequisites; return ready, revise or blocked for that stage."
---

# Review a plan's next stage

Review requested stage; do not implement or amend policy. Read applicable `AGENTS.md`, `docs/README.md`, canonical architecture, [testing contract](../../testing/CONTRACT.md). Obtain request, plan/revision, source/dirty identity, next-stage scope, existing results. Read cited artifacts yourself; author verdict is not evidence. Use fresh reviewer where available; disclose self-review.

| Inspect | Reject an unsupported claim when |
| --- | --- |
| Request and scope | The plan adds product behavior, dependencies, privileged operations or publication beyond authorization. |
| Ownership and flow | A producer, derived state, IPC consumer, effect, service lifecycle or package component is lost; proposed crates are described as existing. |
| Function/module contracts | Preconditions or postconditions lack an enforcing owner; garde is derived but callers can skip validation; mutation/FFI/migration bypasses private construction; internal typed guarantees are discarded or duplicated as conflicting rules. |
| Security and trust | Serde success grants authority; NTS authentication is called UTC accuracy; plain hints authorize clock writes; token/account names stand in for effective isolation. |
| Checks | Expectations come only from the same algorithm/serializer; controls reject all inputs; mocked clocks/tokens establish native enforcement; WSL compilation is called Windows qualification. |
| Execution and recovery | The next stage lacks its Windows environment, bounded clock experiment, independent observation, cleanup, ownership proof, compatible rollback or integrated exit. |
| Evidence status | Architecture, retained research, planned checks or a green historical run are promoted to current implementation/release proof. |

Verify uncertain affected dependency/API facts against current official/upstream evidence. Missing evidence is neither product defect nor N/A. Resolve disputed source facts through bounded inspection, not votes. Review only changed sections while relevant frozen state unchanged; invalidate affected conclusions on source drift.

For changed contracts, require a real invalid/bypass case and valid neighbor at
the affected caller/consumer. Apply
[contract enforcement](../../implementation/implement-rust-contracts/references/contract-enforcement.md)
to field-coverage exemptions, release-active rejection and transformation results;
a proposed attribute, comment or test list is not executed enforcement.

When a plan relies on CodeQL paths or absence of findings, apply
[the data-flow evidence checks](../../testing/references/codeql-data-flow.md):
require source/database/target/feature identity, explicit sources/sinks and
guard models, query controls and uncovered seams. Tool installation, query
compilation and an empty result cannot stand in for whole-path qualification.

Return plan/revision/source identity, stage, reviewer independence, `ready`, `revise` or `blocked`. Each material finding: severity, requirement/boundary, exact location/evidence, consequence, smallest correction/owner. `ready`: named stage can start; later stages may remain open. `revise`: correctable plan error. `blocked`: unavailable necessary decision/capability/required review and resolution. Relabelled self-review cannot fulfil required independence.

Return one permitted next action. Wording preferences do not block readiness; reviewer agreement accepts no residual risk, proves no test execution/ASVS conformance, grants no release permission.

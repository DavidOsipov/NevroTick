---
name: "qa-subagent-dispatch"
description: "Dispatch independent NevroTick test/review tasks with clear boundary ownership, evidence identity and bounded shared resource use."
---

# Delegate complementary verification work

Read [the testing contract](../CONTRACT.md).

For implementation workers, follow [development delegation](../../planning/references/testable-slices.md#development-delegation); use this skill for their assigned verification and independent QA/review workers.

Split independent causal boundaries/risk: numerical truth, protocol admission, IPC authority, recovery/persistence, native UI, package lifecycle. Avoid repeated generic reviews. Explicitly assign cross-worker seams/final integration.

Each worker: request/requirements/acceptance IDs, source including dirty state/candidate/raw inputs, interface dependencies, mode, exclusive writable paths/resources, permitted effects, expected observations, finite budget/stop bounds and cleanup. Independent reviewer receives requirements/evidence, no persuasive verdict/desired finding.

Return compact evidence: input identity, resulting patch/source/artifact identity, exact commands/environment/results and raw-evidence paths, gaps/blocked dependencies, consumed budget, live effects/cleanup and next action. Reference retained logs instead of pasting them into every worker response.

For review workers, apply
[fresh context and completion accounting](../../planning/references/review-and-handoff.md#freeze-review-inputs-and-account-for-completion).
Record actual context mode, inspected inputs and completed/incomplete/not-run
status in the coordinator record before synthesis; a launched worker is no result.

Apply [workstation isolation](../../../../docs/windows-time-development-hardening.md#33-agent-workstation-and-sandbox-rules). One owner per shared writable file/environment/Cargo output/port/simulator clock/Windows machine. Separate independent artifacts/resources; bound aggregate CPU/memory/time. Parallel read-only research may accompany one controlled build/native job. Capacity unavailable: record sequential fallback. While workers run, advance unassigned independent checks without duplicating assigned campaigns.

Inspect returned patches/artifacts and reconcile facts/disagreements/missing evidence/first failures/cleanup/consumed budgets in one coordinator record. Verify affected cross-worker seams on the integrated candidate before acceptance; worker completion alone closes no integrated gate. Reviewer agreement neither multiplies executions nor establishes statistical independence. Use [context continuity](../qa-context-checkpoint/SKILL.md) for durable handoff.

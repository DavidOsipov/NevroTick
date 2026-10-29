---
name: "plan-code-change"
description: "Plan or resume NevroTick Rust features, fixes, migrations or refactors with uncertain impact; give understood small changes a concise in-chat plan."
---

# Plan a NevroTick change

Produce concrete next action within existing request. Planning-only requests end with plan; implementation requests continue when next stage ready. No added approval round.

Read applicable `AGENTS.md`, `docs/README.md`, canonical architecture and affected source/checks. Inspect dirty state before writes; untracked docs/tooling may be sole implementation context. Read [the testing contract](../../testing/CONTRACT.md) before selecting checks.

For known local owner, unchanged trust/data/lifecycle contract and known focused check, record only: requested behavior, owner/files/callers, expected observation/check, unknowns, next action. Label correction needs no plan file or specialist review.

For multiple stages, uncertain impact, security-sensitive work or migration:

1. Reuse existing plan; otherwise use [the template](../templates/change-plan.md). Record request, non-goals, authorization, source/dirty identity, next question. Preserve unknowns.
2. Trace real producer → transformations/state → final consumer/effect. Include service registrations, IPC dispatch, generated package content, native UI consumers. Graphs/architecture proposals do not establish live callers. Reproduce defects or state evidence gap.
   For each changed function/module, map input and output guarantees, error/effect
   behavior and invalidation conditions to an enforcing type, mandatory shared
   constructor or current guard. Follow
   [contract enforcement](../../implementation/implement-rust-contracts/references/contract-enforcement.md);
   include direct-call/bypass and transformed-result checks in the next stage.
3. Use [plan-architecture](../plan-architecture/SKILL.md) only for changed/uncertain ownership, interfaces, dependency direction, data meaning or lifecycle. Use [plan-threat-model](../plan-threat-model/SKILL.md) for material/unknown protection change; otherwise record examined unchanged surfaces.
4. Give [qa-test-planning](../../testing/qa-test-planning/SKILL.md) requirements/independent expectations. Expand next stage: inputs, action, valid/invalid controls, environment, cleanup, exit. Read [testable slices](../references/testable-slices.md) for cross-process/delegated work.
5. Obtain [independent plan review](../plan-independent-review/SKILL.md) for substantial plans; resolve material findings, recheck affected sections. Later stages may remain explicitly deferred; do not delay ready P0 experiment to invent full product.

Separate pure Rust checks, Windows-native behavior, package lifecycle, final release evidence. Before Cargo source exists, report Cargo checks unavailable; no dummy project or documentation-as-passing-build claim. Real clock experiments: architecture section 14.9, disposable Windows VMs, bounded operations/recovery.

When step 2 cannot establish a cross-function source-to-sink path, use
[CodeQL data-flow analysis](../../testing/references/codeql-data-flow.md) for
current source, recording database coverage and unmodeled native/process seams.
An architecture-only plan needs no database; unavailable extraction is an
evidence gap, not grounds to create application scaffolding.

Return plan/location, named next stage, readiness, remaining blockers. Stop planning once stage has verified owners/prerequisites, observable acceptance, no unresolved dependent decision. On resume recheck relevant source/environment; reuse evidence while its state unchanged.

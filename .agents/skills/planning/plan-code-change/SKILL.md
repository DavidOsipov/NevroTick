---
name: "plan-code-change"
description: "Plan or resume NevroTick Rust features, fixes, migrations or refactors with uncertain impact; give understood small changes a concise in-chat plan."
---

# Plan a NevroTick change

Produce a concrete next action within the existing request. Planning-only requests end with a plan; an implementation request can continue when its next stage is ready. This skill adds no approval round.

Read applicable `AGENTS.md`, `docs/README.md`, relevant canonical architecture and affected source/checks. Inspect current dirty state before writes; untracked documentation and tooling may be the only implementation context. Read [the testing contract](../../testing/CONTRACT.md) before selecting checks.

For a known local owner, unchanged trust/data/lifecycle contract and a known focused check, record only: requested behavior, owner/files and callers, expected observation/check, unknowns and next action. Do not require a plan file or specialist review for a label correction.

For multiple stages, uncertain impact, security-sensitive work or a migration:

1. Reuse the existing plan; otherwise use [the template](../templates/change-plan.md). Record request, non-goals, authorization, source/dirty identity and next question. Preserve unknowns as unknowns.
2. Trace the real producer through transformations, state and final consumer/effect. Include service registrations, IPC dispatch, generated package content and native UI consumers. A graph or architecture proposal does not establish live callers. For a defect, reproduce it or state the evidence gap.
3. Use [plan-architecture](../plan-architecture/SKILL.md) only for changed or uncertain ownership, interfaces, dependency direction, data meaning or lifecycle. Use [plan-threat-model](../plan-threat-model/SKILL.md) for a material or unknown protection change; otherwise record examined unchanged surfaces.
4. Give [qa-test-planning](../../testing/qa-test-planning/SKILL.md) requirements and independent expectations. Expand the next stage into inputs, action, valid/invalid controls, environment, cleanup and exit. Read [testable slices](../references/testable-slices.md) for cross-process or delegated work.
5. Obtain [independent plan review](../plan-independent-review/SKILL.md) for substantial plans, resolve material findings and recheck affected sections. Later stages can remain explicitly deferred; do not delay a ready P0 experiment to invent the full product.

Separate pure Rust checks, Windows-native behavior, package lifecycle and final release evidence. Before Cargo source exists, report Cargo checks as unavailable rather than creating a dummy project or calling documentation a passing build. Real clock experiments follow architecture section 14.9 in disposable Windows VMs with bounded operations and recovery.

Return plan/location, named next stage, readiness and remaining blockers. Stop planning when that stage has verified owners/prerequisites, observable acceptance and no unresolved dependent decision. Recheck relevant source and environment on resume; reuse evidence while its state remains unchanged.

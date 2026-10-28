---
name: "qa-subagent-dispatch"
description: "Dispatch independent NevroTick test/review tasks with clear boundary ownership, evidence identity and bounded shared resource use."
---

# Delegate complementary verification work

Read [the testing contract](../CONTRACT.md).

Split work by independent causal boundaries or risk: numerical truth, protocol admission, IPC authority, recovery/persistence, native UI, package lifecycle. Avoid several workers repeating a generic repository review. Assign cross-worker seams and final integration explicitly.

Provide each worker the request, requirements, source/candidate identity, raw inputs, mode, allowed paths/effects, expected observations and finite budget. Independent review should receive requirements/evidence rather than the author's persuasive verdict or desired finding.

One owner controls a shared writable file/environment, Cargo output, port, simulator clock or Windows machine. Separate independent artifacts/resources; bound aggregate CPU/memory/time. Parallel read-only research may proceed while one controlled build/native job runs. If capacity is unavailable, use a recorded sequential fallback.

Reconcile actual facts, disagreements, missing evidence, first failures, cleanup and consumed budgets in one coordinator record. Reviewer agreement does not multiply test execution or establish statistical independence. Use [context continuity](../qa-context-checkpoint/SKILL.md) for durable handoff.

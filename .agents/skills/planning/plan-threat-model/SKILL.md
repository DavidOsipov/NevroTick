---
name: "plan-threat-model"
description: "Plan security controls and independent checks for changed or uncertain NevroTick inputs, Windows permissions, NTP/NTS trust, IPC, persistence, dependencies, packaging or protected effects."
---

# Threat model the changed path

Use same plan/existing authorization. Read applicable instructions, `docs/README.md`, architecture sections 3–8, 10–15, relevant [security owner](../../security/README.md). Load only affected sections. Reuse canonical boundary/threat/check IDs; label assumptions.

1. Inspect changed callers, bytes, state, parsers, authorization, storage, dependencies, destinations, failure paths. If understood/unchanged, record examined paths/unchanged controls; stop. Unknown security effect needs concrete missing fact/evidence owner; neither safe nor N/A.
2. Write attacker-controlled input/identity, preconditions, crossed boundary, reachable protected effect, consequence. Use [the threat method](../references/threat-model-method.md) for competing explanations, attacker capabilities, residual-risk separation.
3. Put controls at actual owners: bounded input admission; strict decoding; private domain construction; current OS identity/policy/generation/freshness/budget guards; serialized effect/actual acknowledgement. Valid data, authenticated packet, signed UI establish different facts; authorize no arbitrary clock/machine-policy changes.
4. Read `docs/asvs-5.0.0-applicability.md` and exact affected pinned-source clauses. Separate applicability, planned implementation, observed verification. Native adaptations are not literal equivalence; L3 includes applicable L1/L2 clauses and full-text stronger conditions. Keep unresolved native revocation, logging, trust-policy decisions visible.
5. Define hostile/legitimate neighboring cases, independent expected result, forbidden effect, environment, resource bounds, cleanup. Route execution through [qa-security-boundaries](../../testing/qa-security-boundaries/SKILL.md) and needed QA owners. Mocks cannot establish Windows token, pipe, clock or installer enforcement.
6. Return material changes to [plan-independent-review](../plan-independent-review/SKILL.md). Name actual unresolved risk/decision owner. Continue unaffected work; workflow invocation implies no new approval.

Keep NTS-only tracking/opt-in bootstrap provenance exact; failures must not silently lower source count, trust checks or authentication. Retained-research TUF remains unadopted, no current updater requirement. Stop once each material threat has owner/control/check or named unresolved decision. Planned controls/review agreement do not demonstrate security.

---
name: "plan-threat-model"
description: "Plan security controls and independent checks for changed or uncertain NevroTick inputs, Windows permissions, NTP/NTS trust, IPC, persistence, dependencies, packaging or protected effects."
---

# Threat model the changed path

Use the same plan and existing authorization. Read applicable instructions, `docs/README.md`, architecture sections 3–8, 10–15 and the relevant [security owner](../../security/README.md). Load only affected sections. Reuse canonical boundary/threat/check IDs and label assumptions.

1. Inspect changed callers, bytes, state, parsers, authorization, storage, dependencies, destinations and failure paths. If understood and unchanged, record examined paths and unchanged controls, then stop. An unknown security effect needs a concrete missing fact and evidence owner; it cannot be called safe or N/A.
2. Write attacker-controlled input/identity, preconditions, crossed boundary, reachable protected effect and consequence. Use [the threat method](../references/threat-model-method.md) for competing explanations, attacker capabilities and residual-risk separation.
3. Put controls at their actual owners: bounded input admission; strict decoding; private domain construction; current OS identity/policy/generation/freshness/budget guards; serialized effect and actual acknowledgement. Valid data, an authenticated packet and a signed UI each establish different facts. They do not authorize arbitrary clock or machine-policy changes.
4. Read `docs/asvs-5.0.0-applicability.md` and exact affected clauses from its pinned source. Preserve applicability, planned implementation and observed verification separately. Native adaptations are not literal equivalence; L3 includes applicable L1/L2 clauses and stronger conditions in their full text. Keep unresolved native revocation, logging and trust-policy decisions visible.
5. Define hostile and legitimate neighboring cases, independent expected result, forbidden effect, environment, resource bounds and cleanup. Route execution through [qa-security-boundaries](../../testing/qa-security-boundaries/SKILL.md) and other needed QA owners. Mocks cannot establish Windows token, pipe, clock or installer enforcement.
6. Return material changes to [plan-independent-review](../plan-independent-review/SKILL.md). Name the actual owner of any unresolved risk/decision. Continue unaffected work; no new approval is implied merely by invoking this workflow.

Keep NTS-only tracking and opt-in bootstrap provenance exact; failures must not silently lower source count, trust checks or authentication. TUF in retained research is an unadopted candidate, not a current updater requirement. Stop when each material threat has an owner/control/check or a named unresolved decision. Planned controls and review agreement remain distinct from demonstrated security.

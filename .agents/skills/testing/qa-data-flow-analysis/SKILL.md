---
name: "qa-data-flow-analysis"
description: "Audit NevroTick Rust data, control and lifecycle chains against invariants using source-backed slices, qualified CodeQL paths and independent counterexamples."
---

# Analyze Rust source-to-sink paths

Formal/Kani proofs and pending proof admission/metadata repairs: post-V1 under
[OD-10 §14.1](../../../../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026).
Tools below grant no pre-V1 proof execution or acceptance gate.

Read [testing contract](../CONTRACT.md); choose the method with
[source-backed flow audits](../references/flow-audit.md). Use for uncertain
propagation, guard bypass or secret disclosure. Preserve requested review/tests/
implementation scope and current candidate identity.

When CodeQL is selected, read [CodeQL data-flow analysis](../references/codeql-data-flow.md);
queries use [pinned Rust recipe](../references/codeql-rust-query.md).

Logical/long chains: reuse existing task's flow/invariant ledger. Inventory real
inputs, protected effects, alternate callers independently of CodeQL models. Prioritize high-impact,
poorly modeled boundaries; expose unaudited frontiers. Read complete relevant
functions/types in bounded slices; retain boundary contracts and integration review.
Diagrams locate evidence.

Before repeat work, compare source, caller wiring, callee contracts, configuration,
query/model/evidence identities. Reuse unchanged qualified segments; invalidate
changed dependencies and affected transitive consumers. Check new callers even
with unchanged callees. Record reuse reasons/gaps; claim no measured speed or
exhaustive paths.

1. Name path question, allowed/forbidden result: attacker/secret source, derived
   representations, stored state, guards, protected/output sink. Trace actual
   producers/consumers, consumed fields/values and enforcing owners. Include
   errors, callbacks, async and alternate constructors. Separate value flow,
   taint, call reachability and current authorization.
2. When using CodeQL, inspect exposed MCP tools/resources and admitted CLI/packs.
   Freeze source/database, extractor, target/features/dependencies, query/models.
   Check extractor defaults and actual launcher controls before Windows/production
   claims. Verify extracted source/functions/call targets; database existence or
   compilation proves no path. One owner; bounded extraction/query outputs.
3. Use admitted Rust pack's primary API/source for appropriate local/global
   value/taint models; online examples may be newer. Inventory sources/sinks
   separately; independently validate extra steps/barriers. Select by resolved
   identity/location, not shared name. AST/CFG/call graphs prove no taint flow.
4. Barrier only for exact protected property/value. Serde, authentication or earlier
   authorization grants no universal safety. Trace checked result versus original
   DTO, secret copies/formatting, stale state after awaits and guard/effect order.
   For garde, require mandatory constructor invocation and success path to consumed
   object. `Validate`, `validate` or `Valid<T>` alone proves no complete invariant;
   inspect custom rules, exemptions, alternate construction, later mutation/context.
5. Challenge intended seam with reachable faulty/clean controls and exact expected
   endpoint pairs/absences. New query/model: broken selector or overbroad barrier
   must fail those endpoint expectations. Diagnose missing paths via selectors, local
   edges, bounded partial flow before adding models. Retain exits, result/path
   counts, exclusions, pagination/path limits; reconcile full decode with BQRS.
   Setup errors, zero selection, unmodeled sinks prove no absence; retain failure.
6. Inspect paths in current callers; challenge competing explanation. State
   reachability, violated guarantee, consequence, uncertainty; independently
   reproduce credible defects with bounded behavior. Map actual IPC seams; retain
   macro/generated/dependency/native FFI gaps.

No product Cargo source: authorized isolated query/runner fixtures only; no app
scaffolding or fixture-as-product scan. Local MCP leaves hosted CodeQL deferred.
Rust source proves no native Windows token/pipe/clock/package enforcement.
Unavailable CodeQL/configuration: deliver possible bounded source trace, name
missing stage; unavailability proves no absent flow.

Needed execution: bounded cargo-fuzz for raw inputs, Miri for portable memory,
Loom for instrumented synchronization, post-V1 Kani for finite invariants through
[configured analysis](../references/configured-tooling.md#additional-rust-analysis).
Protected effects: [security boundaries](../qa-security-boundaries/SKILL.md).
Use compact reference record in existing task; one-off receipts outside skill pack.
Report candidate/database/query identity, controls, paths, observations, gaps.
Empty alerts establish no whole-system safety.

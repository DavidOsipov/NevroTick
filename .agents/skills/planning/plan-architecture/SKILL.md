---
name: "plan-architecture"
description: "Plan NevroTick Rust ownership, module boundaries, state authority, IPC and persistence contracts, Windows effects, dependency direction or lifecycle changes."
---

# Plan architecture at the real owner

Start with request, current source, existing plan; read `docs/README.md` and applicable `docs/windows-time-architecture.md` sections. Architecture canonical; proposed workspace boundaries do not prove crates exist. Return bounded conclusion if inspected owners, meaning, interfaces, effects, recovery unchanged.

1. Find canonical writer, value-changing transformations, every affected consumer. Inspect dispatch/registrations, emitted package inputs, static references. Record current paths, dirty/source identity. Distinguish observed calls, tool candidates, inference; account for relevant `cfg`/features, generated code, macros, trait dispatch before claiming absence. Use repository symbol tools once Rust exists; tool availability does not prove source.
2. Reuse current owner. Pure core independent of sockets, filesystem, Windows handles, ambient wall time. Engine owns network observations; broker owns current authorization/clock effects; UI presents state; installer/helper owns narrow elevated lifecycle operations. Read [Rust ownership decisions](../references/architecture-decisions.md) for abstraction, async or FFI questions.
3. For changed representation, trust, persistence or independently writable state, read [data and state contracts](../references/data-contracts.md). Define producer/consumer, exact units/meaning, authority, invariants, permitted transformations, failure behavior. Input parsing does not prove subsequent transformation or serialized output.
4. Name event/state owner, generation, cancellation path, resource bounds, cleanup on success/disconnection/timeout/restart/suspend. Tie protected effects to current state. Reconcile ambiguous effects; do not retry step. Journal records prior intent; grants no current clock ownership.
5. Compare smallest direct design with credible alternative only where current requirement needs choice. Record decision, reason, tradeoff, invalidation condition. Do not port class-pattern catalogue, global registry or event-bus framework into Rust by analogy.
6. Give [qa-test-planning](../../testing/qa-test-planning/SKILL.md) independent valid/failure examples, consumer observations, forbidden effects, required environments. Share boundary IDs with [plan-threat-model](../plan-threat-model/SKILL.md); security/QA retain owners.

Update same plan: current flow, affected paths, reused/new `DEC/BND/REQ/THR/CHECK` links, stage order, integrated phase exit, unresolved facts. Stop once implementer can identify each rule's owner, allowed transformation, observable failure. Proposal/source review does not verify Windows behavior.

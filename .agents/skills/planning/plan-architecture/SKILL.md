---
name: "plan-architecture"
description: "Plan NevroTick Rust ownership, module boundaries, state authority, IPC and persistence contracts, Windows effects, dependency direction or lifecycle changes."
---

# Plan architecture at the real owner

Start with the request, current source and existing plan; read `docs/README.md` and applicable sections of `docs/windows-time-architecture.md`. The architecture is canonical. Its proposed workspace boundaries do not prove that crates exist. Return a bounded conclusion if inspected owners, meaning, interfaces, effects and recovery are unchanged.

1. Find the canonical writer, value-changing transformations and every affected consumer. Inspect dispatch/registrations and emitted package inputs as well as static references. Record current paths and dirty/source identity. Distinguish observed calls, tool-reported candidates and inference; account for relevant `cfg`/features, generated code, macros and trait dispatch before claiming absence. Use repository code tools for symbols once Rust exists; tool availability is not evidence of source.
2. Reuse the current owner. Keep the pure core independent of sockets, filesystem, Windows handles and ambient wall time. The engine owns network observations; the broker owns current authorization and clock effects; the UI presents state; the installer/helper owns narrow elevated lifecycle operations. Read [Rust ownership decisions](../references/architecture-decisions.md) for abstraction, async or FFI questions.
3. For changed representation, trust, persistence or independently writable state, read [data and state contracts](../references/data-contracts.md). Define producer, consumer, exact units/meaning, authority, invariants, permitted transformations and failure behavior. Parsing an input does not prove a subsequent transformation or serialized output.
4. Name the event/state owner, generation, cancellation path, resource bounds and cleanup on success, disconnection, timeout, restart and suspend. Tie protected effects to current state. Reconcile ambiguous effects rather than retrying a step. A journal records prior intent; it does not grant current clock ownership.
5. Compare the smallest direct design with a credible alternative only where a current requirement needs a choice. Record decision, reason, tradeoff and invalidation condition. Do not port a class-pattern catalogue, global registry or event-bus framework into Rust by analogy.
6. Give [qa-test-planning](../../testing/qa-test-planning/SKILL.md) independent valid/failure examples, consumer observations, forbidden effects and required environments. Share boundary IDs with [plan-threat-model](../plan-threat-model/SKILL.md); security and QA retain their owners.

Update the same plan with current flow, affected paths, reused/new `DEC/BND/REQ/THR/CHECK` links, stage order, integrated phase exit and unresolved facts. Stop when an implementer can identify each rule's owner, allowed transformation and observable failure. A proposal or source review is not verified Windows behavior.

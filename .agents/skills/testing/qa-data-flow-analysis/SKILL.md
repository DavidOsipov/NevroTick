---
name: "qa-data-flow-analysis"
description: "Trace NevroTick Rust external, untrusted or secret inputs through functions, transformations, state and guards to privileged effects or output sinks using scoped CodeQL MCP analysis and independent path checks."
---

# Analyze Rust source-to-sink paths

Read [the testing contract](../CONTRACT.md) and
[CodeQL data-flow analysis](../references/codeql-data-flow.md). For query authoring,
use the [pinned Rust query recipe](../references/codeql-rust-query.md). Use this skill
when propagation, a guard bypass or secret disclosure is uncertain; preserve
the requested review/tests/implementation scope and current candidate identity.

1. State one path question and its expected allowed/forbidden result: attacker
   or secret source, derived representations, stored state, required guards and
   protected/output sink. Map each hop to the actual producer and consumer,
   including the field/value consumed and its enforcing owner. Include error,
   callback, asynchronous and alternate-constructor paths. Distinguish value
   flow, taint propagation, call reachability and current authorization.
2. Inspect the installed CodeQL MCP server's currently exposed tools/resources
   and admitted CLI/query packs. Freeze source/database identity, extractor,
   target, features, dependencies and query/model versions. Check extractor
   defaults and the launcher's actual configuration controls before claiming a
   Windows/production configuration. Verify relevant source, functions and call
   targets are actually extracted; database existence and query compilation prove
   no path result. Keep bounded extraction and query outputs under one owner.
3. Use the admitted Rust pack's primary API documentation/source to choose
   local/global data flow or taint models appropriate to the question. Current
   online examples may use APIs newer than the admitted pack. Enumerate source
   and sink membership separately before tracing; validate additional flow steps
   and barriers independently. Scope selectors by resolved identity and location,
   not merely a function name shared by unrelated implementations.
   AST, CFG and call graphs locate code; they do not establish taint propagation.
4. Treat a guard as a barrier only for the exact property and value it protects.
   Serde success, authentication or earlier authorization grants no universal
   safety. Follow checked results versus original DTOs, secret copies/formatting,
   stale state after awaits and each caller's actual guard-to-effect ordering.
   For garde-backed contracts, trace the mandatory constructor invocation and
   success branch to the consumed object. A `Validate` derive, a function named
   `validate` or `Valid<T>` does not establish every semantic invariant; inspect
   custom rules, exemptions, alternate construction and later mutation/context.
5. Challenge the query with reachable faulty and clean controls at the intended
   seam. Assert the expected source/sink pairs, including expected absences;
   prove that a deliberately broken selector or overbroad barrier fails those
   expectations when qualifying a new query/model. Debug a missing path through
   selector inventories, local edges and bounded partial flow before adding a
   model. Record process exit, query result/path counts and exclusions. Reconcile
   decoded rows with BQRS totals and retain pagination/path limits. Setup errors,
   zero selection or an unmodeled sink
   cannot establish absence of a defect; preserve the failing control.
6. Inspect returned paths in current code and test a competing explanation.
   State reachability, violated guarantee, consequence and remaining uncertainty;
   confirm a credible defect through an independent bounded behavioral case.
   Map cross-process paths at actual IPC seams and retain unmodeled macro,
   generated-code, dependency and native FFI coverage gaps.

Without product Cargo source, use only authorized isolated query/runner fixtures;
do not scaffold an application or call fixture results a product scan. Local MCP
analysis leaves hosted CodeQL deferral in place. Rust source analysis proves no
native Windows token/pipe/clock or complete-package enforcement.
When CodeQL or a required database configuration is unavailable, deliver the
bounded source trace that is possible and name the missing analysis stage;
do not turn tool unavailability into an absence-of-flow conclusion.

For a path needing execution evidence, select bounded cargo-fuzz for raw inputs,
Miri for portable memory behavior, Loom for instrumented synchronization or Kani
for a finite invariant through [configured analysis](../references/configured-tooling.md#additional-rust-analysis).
Use [security boundaries](../qa-security-boundaries/SKILL.md) for protected effects.
Use the reference's compact trace/evidence record in the existing task; keep
one-off receipts outside the skills pack. Report candidate/database/query identity,
controls, reviewed paths, observations
and unresolved coverage; an empty alert set is not a whole-system safety claim.

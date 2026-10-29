---
name: "implement-rust"
description: "Implement, refactor or review NevroTick Rust logic, ownership, errors, state, concurrency and Windows adapter boundaries."
---

# Implement Rust

Start with repository instructions/current architectural owner. Preserve requested
implementation/review/planning scope, dirty work and resolved decisions. Ordinary
code edits must not create Cargo workspaces, dependencies or service architecture
as side effects.

Read the shared [implementation contract](../CONTRACT.md). For every changed
function/module, identify preconditions, postconditions, failure and effect/state
guarantees and their enforcement. Use private checked types in signatures;
validate new raw values and transformed results before exposure. Mandatory garde
validation belongs in the shared fallible construction path, so direct callers
cannot forget a separate validation step. See
[contract enforcement](../implement-rust-contracts/references/contract-enforcement.md).

Locate producer, callers, mutation paths, effects and target/feature lane.
Distinguish pure core, wire admission, engine, broker, Windows adapter and UI.
With source present, initialize Serena before symbol/reference queries; use Probe
for Rust search/extraction and configured documentation tools for design context.
Empty graph before source exists is expected.

Choose smallest model that makes meaningful mistakes hard to express:

- Private newtypes for units/identity; enums for explicit states/outcomes;
  `Result` for expected failures; checked boundary conversion/arithmetic.
- Preserve exact timestamps/counters until deliberate conversion of small deltas.
  Reject non-finite estimator/FFI results and validate post-conversion bounds.
- Match ownership, borrowing and lifetimes to actual handle/state owners.
  No `Arc<Mutex<_>>`, cloning or dynamic dispatch merely to silence compiler errors.
  Locks must cover complete invariant-bearing transactions.
- Never erase operational failure into `None`, empty results or stale success.
  Redact secrets; retain structured context sufficient to diagnose rejection.

For FFI, handles, panic policy, async work, cancellation or lifecycle changes, use
[Rust and Windows boundaries](references/rust-windows.md). Pure core forbids unsafe;
Windows adapters need small reviewed unsafe contracts. UI/engine have no
clock-setting privilege.

For Windows service registration, SCM callbacks/status, account isolation or
stop/restart changes, read [the service adapter guide](references/windows-service-adapters.md)
directly. Record separate engine/broker recovery settings and an operation/access
matrix before qualifying native behavior; wrapper examples establish neither.

For compiler/runner/feature changes or review-feedback intake, read
[execution evidence](references/execution-evidence.md) before selecting commands.

For translated or behavior-adapted chrony algorithms, or a deterministic simulator
used to qualify their behavior, read the
[chrony port and simulator recipe](references/chrony-port-simulator.md). Work one
algorithm-sized slice from exact upstream provenance through an independent
numerical contract and focused evidence. Preserve architecture section 18.3's
GPL-2.0-only, notices and corresponding-source/build obligations; translation does
not establish clean-room provenance or clear an unwritten dependency graph.

Route data admission or internal invariant contracts to
[implement-rust-contracts](../implement-rust-contracts/SKILL.md), compatibility
changes to [evolution](../evolve-data-contracts/SKILL.md), and controls/focus/rendered
status to [native UI](../implement-native-ui/SKILL.md). Reuse immutable typed
guarantees across internal calls; recheck facts invalidated by state/context changes.

For an explicit performance task or measured cost/regression, use
[performance optimization](../optimize-rust-performance/SKILL.md): attribute the
bottleneck, compare bounded baseline/candidate runs and preserve correctness,
security and resource limits. Ordinary Rust edits need no benchmark campaign.

Select executable checks from the
[configured-tool workflows](../../testing/references/configured-tooling.md).
For real Rust source, run applicable format/compiler/Clippy lanes and focused
regressions for the changed owner and callers; API/documentation changes also
need their rustdoc/doctest lane. Admit actual target/features before execution;
host checks cannot replace Windows adapter linking or native observations.
Policy/runner changes use existing healthy/rejection controls, not a fabricated
application workspace. Preserve target mitigation flags and narrow expectations.

For a concrete memory/ownership gap, select admitted Miri on reached portable
tests. For synchronization changes, select an instrumented bounded Loom model;
use experimental Lockbud only after the real source/features compile on its
separate admitted compiler. Small arithmetic/state invariants may suit Kani;
hostile parser/conversion inputs may suit cargo-fuzz. Use the guide's guarded
analysis lane and then qualify the real harness; bootstrap controls alone test
no product path. Keep shipping Rust and native Windows observations separate.

For reached Windows heap/handle/FFI or resource-lifetime changes, use
[native memory diagnostics](../../testing/references/windows-memory.md).
Select Application Verifier/PageHeap for native misuse and separate lifecycle
measurements for retention; retain the target's intended ordinary-user token
and restore only owned image settings after the run.

When locating uncertain cross-function input-to-effect or secret-to-output paths,
read [CodeQL data-flow analysis](../../testing/references/codeql-data-flow.md).
Confirm query paths against current callers and current guards; a missing result
cannot establish an unreachable effect.

Verify changed guarantee/actual callers with smallest appropriate behavioral
check and the selected configured checks.
For bugs, show causal failure and healthy control before repair where practical.
Use [QA contract](../../testing/CONTRACT.md) for test integrity. Compiler acceptance
proves type obligations, not freshness, authorization, numerical truth or Win32
runtime behavior.

Report result, changed owner/callers, failure/effect decisions, executed checks
and target/runtime uncertainty. Reuse passing evidence while relevant state stays
unchanged; small edits need no blanket release-gate campaign.

Adapted David Osipov skill material: [licence and notices](THIRD_PARTY_NOTICES.md).

---
name: "implement-rust"
description: "Implement, refactor or review NevroTick Rust logic, ownership, errors, state, concurrency and Windows adapter boundaries."
---

# Implement Rust

Start with repository instructions and the current architectural owner. Preserve
the request's implementation/review/planning scope, existing dirty work and resolved
decisions. Do not create a Cargo workspace, new dependency or service architecture
as a side effect of an ordinary code edit.

Locate the producer, callers, mutation paths, effects and target/feature lane.
Distinguish pure core, wire admission, engine, broker, Windows adapter and UI.
Use Serena symbols/references after its required initialization when source exists;
use Probe for Rust search/extraction and the configured documentation tools for
design context. An empty graph before source exists is expected.

Choose the smallest model that makes meaningful mistakes hard to express:

- Private newtypes for units/identity; enums for explicit states and outcomes;
  `Result` for expected failures; checked conversion/arithmetic at boundaries.
- Preserve exact timestamps/counters until deliberate conversion of small deltas.
  Reject non-finite estimator/FFI results and validate post-conversion bounds.
- Make ownership, borrowing and lifetime rules follow actual handle/state owners.
  Avoid introducing `Arc<Mutex<_>>`, cloning or dynamic dispatch merely to silence a
  compiler error. A lock must cover the complete invariant-bearing transaction.
- Do not erase operational failure into `None`, an empty result or stale success.
  Redact secrets and preserve enough structured context to diagnose rejection.

Use [Rust and Windows boundaries](references/rust-windows.md) when changing FFI,
handles, panic policy, async work, cancellation or lifecycle. Pure core forbids
unsafe; Windows adapters have small reviewed unsafe contracts. Keep the UI and
engine free of clock-setting privilege.

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
status to [native UI](../implement-native-ui/SKILL.md). Use the shared
[contract](../CONTRACT.md) only when its data-contract trigger applies.

Verify the changed guarantee and actual callers with the smallest appropriate
behavioral check, plus applicable compiler/Clippy/format checks in the existing
target matrix. For a bug, demonstrate the causal failure and healthy control before
the repair when practical. Use the [QA contract](../../testing/CONTRACT.md) for
test integrity. Compiler acceptance proves type obligations, not freshness,
authorization, numerical truth or Win32 runtime behavior.

Report the result, changed owner/callers, failure/effect decisions, checks actually
executed and remaining target/runtime uncertainty. Reuse passing evidence while
the relevant state is unchanged; do not impose every release gate on a small edit.

Adapted David Osipov skill material: [licence and notices](THIRD_PARTY_NOTICES.md).

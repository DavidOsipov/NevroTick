---
name: "qa-state-concurrency-lifecycle"
description: "Verify NevroTick asynchronous state, concurrency, cancellation and resource ownership with deterministic scenarios and conditional Shuttle or Loom schedule exploration."
---

# Test timing, cancellation and ownership

Read [testing contract](../CONTRACT.md).

For deterministic runtime cases, Shuttle or Loom, use the
[tool-selection guide](../references/generated-testing-selection.md). Read the
selected manual's integration, execution and replay sections before building a harness.

Source review: shared [flow/invariant audit](../references/flow-audit.md).
Separate data from state/happens-before obligations: each read's value/version,
lock/lease, invalidation, effect, acknowledgement. Compare terminal-decision
snapshot with transferred snapshot. Challenge changes between reads and after
completion; later state must not rewrite history. Close ownership/cleanup seams;
reuse evidence only while actual dependencies remain valid.

Trace owned tasks/permits/timers/handles/pending effects/publications; generation/
current-authority checks after async work. Cooperative cancellation proves no
native stop.

Deterministic barriers/notifications/separate injected clocks: cancel before/during
native work, late replies, policy changes, suspend/resume, reconnect, shutdown.
Assert stale work cannot publish/act; cancelled call retains permit until actual
release; cumulative budgets survive restart/reconnect.

Test ambiguous acknowledgements, successor clock ownership, crash during transient
slew, SCM transitions, UI exit while service continues. Observe actual resources/
cleanup before teardown. Abort requires external recovery owner.

Heap/handle misuse or repeated retention: [Windows memory diagnostics](../references/windows-memory.md).
Challenge partial construction, cancellation, late completion at real owner;
retain pre-teardown counts. Clean verifier/PageHeap proves neither bounded
retention nor release of every task/permit.

SCM wrapper/status/callbacks: [Windows service adapters](../../implementation/implement-rust/references/windows-service-adapters.md).
Separate permitted native lifecycle from synthetic sequences; released callback
outside contract is no native product RED.

Small models cover declared primitives/state space only; imported Loom explores
no ordinary OS synchronization. Read [Rust techniques](../references/rust-techniques.md),
[Windows evidence](../references/windows-evidence.md); scheduling tests prove no SCM/FFI effects.

Reproducible async ownership/publication schedule gap: [Shuttle concurrency](../references/shuttle-concurrency.md).
Qualify small production path with healthy/requirement-linked faults. Compare
outcome, published checkpoint and effects against same current authority; old
snapshot is no fresh guard. Map primitives/transitive dependencies; retain concrete
replay; run the same semantic scenarios on actual selected Tokio runtime. Sampled
schedules, sequential models, native observations differ. Reference admits no
dependency or analysis-command flag.

Admitted Loom: specific invariant, bounded preemptions/permutations. Miri: reached
portable ownership/aliasing. Experimental Lockbud: static lock screening after
separate-compiler compatibility with actual source. Reached race risk: separately
admitted TSan via [sanitizer selection](../references/sanitizers.md); synchronization
visibility/targets limit scope, lifecycle assertions still required.
Use [configured analysis](../references/configured-tooling.md#additional-rust-analysis)
for tool controls, then actual product models/tests. Retain crate selection, fresh
Lockbud outputs, `--exit-code-on-bug`; skipped compile/ignored finding is no clean
analysis. Never lower shipping Rust for tool compatibility.

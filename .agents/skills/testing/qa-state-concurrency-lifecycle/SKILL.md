---
name: "qa-state-concurrency-lifecycle"
description: "Verify NevroTick asynchronous state, concurrency, cancellation, service and resource lifecycle behavior at its actual ownership boundaries."
---

# Test timing, cancellation and ownership

Read [the testing contract](../CONTRACT.md).

Trace each owned task/permit/timer/handle/pending effect/publication. Identify generation/current-authority checks after async work. Cancellation cooperative; does not prove native operation stopped.

Use deterministic barriers/notifications/separate injected clocks: cancellation before/during native work, late replies, policy changes, suspend/resume, reconnect, shutdown. Assert stale work cannot publish/act; no extra permit while cancelled call still owns one; cumulative budgets survive restart/reconnect.

Test ambiguous acknowledgements, successor clock ownership, crash during transient slew, SCM transitions, UI exit without service termination. Observe real resource counts/cleanup before fixture teardown. Abort tests need external recovery owner.

For native heap/handle misuse and repeated lifecycle retention, select
[Windows memory diagnostics](../references/windows-memory.md). Challenge partial
construction, cancellation and late completion at the real owner; retain actual
resource counts before teardown. A clean verifier/PageHeap run does not prove
bounded retention or release of every task/permit.

SCM wrapper/status/callback qualification: read [Windows service adapters](../../implementation/implement-rust/references/windows-service-adapters.md). Distinguish native-lifecycle permitted controls/synthetic facade sequences; invoking released callback outside contract is not native product RED.

Use instrumented small models only for their declared primitives/state space; an imported Loom crate does not explore ordinary OS synchronization. Read [Rust techniques](../references/rust-techniques.md) and [Windows evidence](../references/windows-evidence.md); pure scheduling tests do not qualify actual SCM/FFI effects.

Select admitted Loom for a specific synchronization invariant and bounded
preemption/permutation model; use Miri for reached portable ownership/aliasing
paths. Experimental Lockbud can complement these with static lock screening
only after real-source compatibility with its separate compiler is established.
For reached data-race risks, consider separately admitted TSan through
[sanitizer selection](../references/sanitizers.md); synchronization visibility
and supported targets constrain it, and it does not replace lifecycle assertions.
Use [configured analysis](../references/configured-tooling.md#additional-rust-analysis)
for tool controls, then the actual product model/tests. Retain exact crate
selection, fresh Lockbud build outputs and `--exit-code-on-bug`; a skipped compile
or ignored finding is no clean analysis. Never lower shipping Rust to fit it.

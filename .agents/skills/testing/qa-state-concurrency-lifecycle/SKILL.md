---
name: "qa-state-concurrency-lifecycle"
description: "Verify NevroTick asynchronous state, concurrency, cancellation, service and resource lifecycle behavior at its actual ownership boundaries."
---

# Test timing, cancellation and ownership

Read [the testing contract](../CONTRACT.md).

Trace each owned task, permit, timer, handle, pending effect and publication. Identify where generations and current authority are checked after asynchronous work. Cancellation is cooperative; it does not prove the native operation stopped.

Use deterministic barriers/notifications and separate injected clocks to exercise cancellation before/during native work, late replies, policy changes, suspend/resume, reconnect and shutdown. Assert stale work cannot publish or act, no extra permit is minted while a cancelled call still owns one, and cumulative budgets survive restart/reconnect.

Test ambiguous acknowledgements, successor clock ownership, crash during transient slew, SCM transitions and UI exit without service termination. Observe real resource counts/cleanup before fixture teardown. Abort tests require an external recovery owner.

For SCM wrapper/status/callback qualification, read [Windows service adapters](../../implementation/implement-rust/references/windows-service-adapters.md). Distinguish controls permitted by the native lifecycle from synthetic facade sequences; invoking a released callback outside its contract is not a native product RED.

Use instrumented small models only for their declared primitives/state space; an imported Loom crate does not explore ordinary OS synchronization. Read [Rust techniques](../references/rust-techniques.md) and [Windows evidence](../references/windows-evidence.md); pure scheduling tests do not qualify actual SCM/FFI effects.

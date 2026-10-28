# Rust ownership and abstraction decisions

Read for a real abstraction, concurrency, platform or dependency decision. Start from the canonical architecture, not a preferred pattern name.

| Pressure | Small starting point | Evidence needed before adding machinery |
| --- | --- | --- |
| Multiple observation/effect implementations | Narrow trait or concrete adapter at the effect boundary | Actual second implementation or independently testable effect seam; preserve error/ownership semantics. |
| Closed protocol/state alternatives | Enums with exhaustive transitions and checked payloads | Required extension/version strategy; avoid stringly typed dispatch or an open plugin registry. |
| Invalid partial construction | Private fields plus `TryFrom`/fallible constructor | Cross-field invariants and checked integer/unit conversion; a builder is justified only by real staged construction. |
| Shared mutable model | One event owner and bounded message queue | Ordering, backpressure, deadlines, cancellation, shutdown and stale-generation behavior. `Arc<Mutex<_>>` does not establish those contracts. |
| Cached/projected state | One canonical writer; revision-tagged immutable snapshots | Invalidation/rebuild and mixed-generation consumer rules. Cache hits are not fresh authorization. |
| Windows FFI | Reviewed narrow safe adapter with owned RAII handles | ABI, pointer lifetime/alignment, thread affinity, cancellation, invalid-handle values, error capture and cleanup. Safe wrapper names do not prove unsafe preconditions. |

Keep `unsafe` out of the pure core, protocol parser, domain and IPC contracts as required by the architecture. Keep Windows effects behind the platform adapter. A Rust destructor supports ordinary resource cleanup; it does not run after arbitrary process termination and cannot prove global clock restoration. Async cancellation can drop work at await points; specify who owns partially completed effects and how completion is reconciled.

For each chosen approach record invariant, affected owner/interface, alternatives, tradeoff, current evidence and invalidation condition. Do not add a crate, service, generic framework or elaborate pattern merely to satisfy the skill. Suggested crate boundaries can start as modules if dependency direction remains enforceable.

# Rust ownership and abstraction decisions

Read for actual abstraction/concurrency/platform/dependency decision. Start from canonical architecture; no preferred-pattern default.

| Pressure | Small starting point | Evidence needed before adding machinery |
| --- | --- | --- |
| Multiple observation/effect implementations | Narrow trait or concrete adapter at the effect boundary | Actual second implementation or independently testable effect seam; preserve error/ownership semantics. |
| Closed protocol/state alternatives | Enums with exhaustive transitions and checked payloads | Required extension/version strategy; avoid stringly typed dispatch or an open plugin registry. |
| Invalid partial construction | Private fields plus `TryFrom`/fallible constructor | Cross-field invariants and checked integer/unit conversion; a builder is justified only by real staged construction. |
| Shared mutable model | One event owner and bounded message queue | Ordering, backpressure, deadlines, cancellation, shutdown and stale-generation behavior. `Arc<Mutex<_>>` does not establish those contracts. |
| Cached/projected state | One canonical writer; revision-tagged immutable snapshots | Invalidation/rebuild and mixed-generation consumer rules. Cache hits are not fresh authorization. |
| Windows FFI | Reviewed narrow safe adapter with owned RAII handles | ABI, pointer lifetime/alignment, thread affinity, cancellation, invalid-handle values, error capture and cleanup. Safe wrapper names do not prove unsafe preconditions. |

Keep `unsafe` outside pure core, protocol parser, domain, IPC contracts per architecture. Windows effects behind platform adapter. Rust destructor supports ordinary cleanup; does not run after arbitrary process termination or prove global clock restoration. Async cancellation can drop work at await points; specify partially completed effect owner/completion reconciliation.

For each approach record invariant, affected owner/interface, alternatives, tradeoff, current evidence, invalidation condition. No crate/service/generic framework/elaborate pattern merely to satisfy skill. Suggested crates may start as modules if dependency direction enforceable.

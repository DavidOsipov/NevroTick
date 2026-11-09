# Tokio at the engine boundary

Use when changing the engine's Tokio runtime, task ownership, blocking bridge,
I/O, channels or scheduling. Follow architecture §§17.1, 18.6.1 and 18.7.1 and
the actual task owner. This reference supplies API-specific implementation
guidance; it selects no new runtime, TLS adapter, feature or execution campaign.

- [Check the actual feature boundary](#check-the-actual-feature-boundary)
- [Keep the driver alive and active](#keep-the-driver-alive-and-active)
- [Review blocking-pool dependencies](#review-blocking-pool-dependencies)
- [Settle owned tasks explicitly](#settle-owned-tasks-explicitly)
- [Choose channels by delivery semantics](#choose-channels-by-delivery-semantics)
- [Preserve I/O progress and final eligibility](#preserve-io-progress-and-final-eligibility)
- [Choose periodic scheduling only at its consumer](#choose-periodic-scheduling-only-at-its-consumer)
- [Use the existing evidence owners](#use-the-existing-evidence-owners)

## Check the actual feature boundary

Re-read the engine manifest, lockfile and owner before applying examples. The
9 November 2026 snapshot selects Tokio **1.54.1**, defaults disabled, with
`rt`, `net`, `time`, `sync`, `macros`; Rust is 1.99.1 / edition 2024. `execute`
and `execute_series` privately own current-thread runtimes, enable I/O/time,
and set `max_blocking_threads(2)`. Their existing spawned work is at most two
owned control timers; this is no implemented production KE worker pool.

The selected `sync` feature supplies the channels below; `rt` supplies ordinary
task handles and JoinSet. Async buffered/framing helpers may require `io-util`
and its `bytes` dependency; paused time needs `test-util`. Neither is selected
by this reference. `block_in_place` requires `rt-multi-thread` and cannot run
inside a current-thread runtime even if that feature is enabled. Admit a real
feature/target change through the existing graph owner; do not copy `full`,
unstable configuration or companion crates merely to follow an example.

## Keep the driver alive and active

If an admitted synchronous worker polls an async KE future through
`Handle::block_on`, name the owner concurrently running `Runtime::block_on` on
that same current-thread runtime. Only the latter drives its I/O and timers.
`enter` supplies runtime context; a cloned Handle neither drives resources nor
prevents runtime shutdown. Keep the required driver active through dependent
work's terminal disposition, including the chosen stop path. Do not return from
the sole driver and then synchronously wait for work that still needs it.

Place blocking certificate verification away from the responsive event-loop/
control owner. An admitted worker may own the whole synchronous KE exchange;
that route has no Tokio I/O-driver dependency for its synchronous transport.
An async TLS adapter does not make its synchronous verifier nonblocking, and a
timeout cannot interrupt that callback. Preserve the existing complete-worker
choice and native stop/recovery contract; no nested runtime per request follows.

## Review blocking-pool dependencies

Trace reachable submissions as well as visible `spawn_blocking` calls. Tokio's
blocking pool is shared with DNS resolution and, when enabled, filesystem/std-I/O
work. Its thread cap does not bound the submission queue. If every pool thread
is occupied by an outer worker waiting on another job queued to the same pool,
those nested jobs cannot start until outer capacity returns. With the current
cap of two, two such outer workers are enough to create this progress dependency.
An independently driven finite deadline may release their waits; it does not
prove queued or started nested work has been retired. This is a conditional
design case, not an observed deadlock in the current numeric-address lab.

At the real KE freeze, compare resolution before worker submission, a complete
synchronous exchange with an explicit resolver owner, or already-authorized
bounded persistent workers. Pre-resolution still needs destination/path
re-admission at actual connection and effect boundaries. Keep hidden work in the
same aggregate admission/custody accounting. Increasing a cap or adding a pool
does not by itself establish progress or preserve those bounds.

Application admission, completion-slot reservation, result-byte custody and
actual-work permits remain in
[the native-boundary reference](rust-windows.md#async-bounded-work-and-state).
Use its transactional refusal, close/drop, late-result and stuck-job cases at
the actual owner; do not duplicate that contract in a new task framework.

## Settle owned tasks explicitly

Keep cancellation request, observed task termination, domain `Result` and native
completion distinct. Dropping a JoinHandle detaches the task. The current
`finish_timer` requests abort and awaits the handle, preserving domain errors
and a fixed join-failure category; two explicit handles are sufficient here.

If a later real task population warrants JoinSet, bound admission before spawn
and explicitly drain `join_next` when outcomes matter. `abort_all` alone does
not observe settlement; `shutdown` discards task outcomes, including panics;
`join_all` panics on a JoinError. None terminates an already-started blocking
native call. Runtime Drop can wait indefinitely for blocking work;
`shutdown_timeout`/`shutdown_background` stop waiting without proving its cleanup.
Retain [error and panic policy](error-handling.md), including safe outputs and
payload disposal, at its current owner.

## Choose channels by delivery semantics

| Primitive | Required interpretation at the real consumer |
| --- | --- |
| `watch` | Latest snapshot; intermediate updates may coalesce. Fits the existing latched stop/current generation. Use `borrow_and_update` with change waiting and keep borrow guards short, away from callbacks, arbitrary blocking and awaits. It is no per-event acknowledgement ledger. |
| Bounded `mpsc` | Individual queued messages with bounded slot capacity. Cancellation of `send(value)` drops the unsent value; reserve before transfer where custody requires it. Successful send is no consumption acknowledgement. Follow the existing native/result reservation contract above. |
| `broadcast` | A lagging receiver can lose older values and receive `Lagged`. Define resnapshot/recovery before using it; it cannot alone guarantee mandatory completion custody. |
| `Notify` | Data-free wakeup; `notify_one` retains at most one permit, so notifications can coalesce. Recheck authoritative state; never count jobs or acknowledgements from wakeups. Use the documented registration/recheck pattern if multiple consumers are actually required. |

The current control channel already follows the snapshot model. Preserve its
serialized summary transaction; do not rewrite it or add notification-elision
logic without an actual changed contract or measured cost.

## Preserve I/O progress and final eligibility

Set nonblocking before importing a standard socket into Tokio. Readiness can be
spurious; handle `WouldBlock` through readiness waiting with bounded attempts,
not a busy retry loop. When using biased `select!`, place required control and
deadline branches ahead of perpetually ready data and retain a bounded work
quantum. Branch priority alone does not enforce a final deadline or generation.

Keep original absolute stage/total deadlines across partial progress and retries.
`timeout_at` may return `Ok` for an immediately ready future even at/after its
deadline; a non-yielding poll can also finish after expiry. Treat wrapper success
only as a completed value. Immediately before the serialized publication/effect,
sample the owner's authoritative current time and recheck strict expiry,
generation, policy and destination eligibility. Do not reset the deadline on
each fragment or remove the current UDP/series final guard.

For future stream/async KE work, apply the existing
[framing rule](../../../../../docs/windows-time-architecture.md#544-framing-cancellation-and-native-completion):
retain partial header/body/write offsets when resuming, or retire the connection.
Restarting cancelled `read_exact`/`write_all` on the same stream can lose or
duplicate progress. UDP receive cancellation safety does not transfer to stream
framing. Preserve complete-request flush, bounded parsing and same-connection
exporter ownership; this guidance admits no async TLS adapter.

## Choose periodic scheduling only at its consumer

When a real production polling scheduler selects Interval, decide first-fire,
missed-tick and resume behavior explicitly. `interval` fires immediately first
and defaults to Burst; missed-tick strategies apply only beyond 5 ms of lateness
in the inspected version. Skip or Delay alone is no protocol packet-rate limit.
Keep burst/backoff budgets, current generations, suspend invalidation and final
send admission with the existing scheduler owner. Adaptive one-shot
`sleep_until` may remain simpler. There is no current engine Interval consumer;
do not replace the one-shot timers or make this a prerequisite for S2.2.

## Use the existing evidence owners

Use the [S2.2 Tokio follow-up](../../../../../tasks/engine-transport-pilot.md#tokio-refinements-at-the-ke-boundary--9-november-2026)
for real driver, saturated-worker and final-publication controls, including
healthy neighbors and independent finite watchdogs where a stalled driver is
deliberately exercised. These are pending integration cases, not current passes.

For clock injection/paused-time limits, read
[Rust time and asynchronous execution](../../../testing/references/rust-techniques.md#time-and-asynchronous-execution).
For a concrete measurement question, read
[stable Tokio observations](../../optimize-rust-performance/references/measurement.md#stable-tokio-observations).
Preserve their distinctions between Tokio time and std/QPC, completion and
auto-advance, spawned-task counts and total work, scheduler activity and CPU.
Neither tool supplies native Windows suspend, callback or shutdown qualification.

Primary sources, inspected versions and retrieval limits:
[Tokio source register](../../SOURCES.md#tokio-engine-guidance--9-november-2026).

# Shuttle: instrumented concurrency evidence

Use for named scheduling, ownership, cancellation or publication invariants.
Start with [testing contract](../CONTRACT.md), [lifecycle skill](../qa-state-concurrency-lifecycle/SKILL.md),
[tool selector](generated-testing-selection.md) and [configured tooling](configured-tooling.md).
This manual admits no dependency, installation, product harness or new
`check-analysis.py --tool` selection. Current task permissions, Windows/fuzz/storage/UI
holds and OD-10's post-V1 formal deferral remain. Schedule tests establish bounded
execution evidence; they do not discharge formal or native product obligations.

If the selected library/runtime is missing, use [setup and availability](generated-testing-setup.md#rust-libraries-and-the-actual-test-build)
for dependency declarations and existing guarded preparation. Current
task authority controls adoption; a missing package is a preparation task.

## Contents

- [Reviewed source and adoption status](#reviewed-source-and-adoption-status)
- [When Shuttle fits](#when-shuttle-fits)
- [Prepare the actual production boundary](#prepare-the-actual-production-boundary)
- [Model limitations](#model-limitations)
- [Choose a scheduler and explicit bounds](#choose-a-scheduler-and-explicit-bounds)
- [Minimal source-checked example](#minimal-source-checked-example)
- [Run and replay procedure](#run-and-replay-procedure)
- [Requirement-linked controls](#requirement-linked-controls)
- [Real Tokio counterpart](#real-tokio-counterpart)
- [Troubleshooting](#troubleshooting)
- [Report and stopping checklist](#report-and-stopping-checklist)

## Reviewed source and adoption status

Observed **2026-11-15 UTC** through official GitHub source: latest release returned
`awslabs/shuttle` **v0.9.10**, tag commit **855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b**.
[Changelog][release-source] dates it November 4; [GitHub release][release] published
November 5, 2026. Source observations, not selected NevroTick dependencies.
One precise Context7 lookup returned the unrelated Shuttle cloud platform; its
documentation was not used. Future lookups must distinguish `awslabs/shuttle` from `shuttle-hq/shuttle`.

Reviewed cohort: `shuttle` 0.9.10, `shuttle-engine` 0.1.8,
`shuttle-std` 0.1.5, `shuttle-schedulers` 0.1.1, `shuttle-tokio` 1.0.0,
`shuttle-tokio-impl` 0.1.2 and `shuttle-tokio-impl-inner` 0.1.7.
The wrapper was not republished for inner fixes; its version alone cannot
establish the resolved implementation. Recent fixes change cancellation/drop,
Notify, channel, watch and initialization scheduling; old replays can change validity.

Before adoption, use [dependency review](../../environment/dependency-updates/SKILL.md)
and [OD-19](../../../../docs/nevrotick-owner-decisions.md#23-od-19--14-day-registry-dependency-cooldown).
Apply **14-day whole-delta registry cooldown**, including changed/new transitives,
host/build and proc-macro dependencies. Verify every relevant registry publication
timestamp; release/changelog dates prove no registry age. No automatic selection
follows when a calendar threshold passes. Missing age evidence needs the existing
concrete whole-graph exception procedure, not a persistent override or an older
wrapper version disguising a newer inner crate. Review licences per package:
wrapper and inner implementation licence declarations differ.

## When Shuttle fits

| Question | Appropriate evidence and Shuttle's role |
| --- | --- |
| Can completion race with stop, generation replacement or publication? | Shuttle fits a bounded production path whose relevant tasks, synchronization and effects can be controlled. Search many interleavings and preserve an exact failing schedule. |
| Is one already-understood ordering wrong? | First write a deterministic test with barriers/notifications around the real owner. Use Shuttle when unexamined interleavings remain; it should not replace a simple discriminating regression. |
| Are timeout boundaries, timer ordering or current-thread Tokio behavior correct? | Use actual selected Tokio runtime and explicit/injected clocks. Shuttle time adapters cannot establish elapsed-time semantics. |
| Is a small atomic/synchronization algorithm correct under weaker memory orderings? | Consider separately admitted Loom with explicit model bounds and documented omissions. Shuttle treats all atomic orderings as sequentially consistent. |
| Did Windows cancel native work, release a handle or complete an I/O operation? | Require native owner and actual completion/resource evidence. Shuttle can explore the surrounding decision protocol through controlled ports only. |

Start with few actors and a short operation sequence. Larger scenarios may benefit
from randomized schedules, but passing samples prove only reached assertions.
Even completed DFS covers only the declared instrumented model and inputs.
See [Shuttle's design][overview], [atomic limits][atomics] and [Loom's limitations][loom].

## Prepare the actual production boundary

1. In the existing task, name the requirement/CHECK, production owner/callers,
   independent expected result, admitted inputs, forbidden effects and environment.
   Define the terminal condition, for example, stop acknowledged only after actual
   joins, while an old generation cannot publish or acquire replacement custody.
2. Inspect current source/dirty bytes, manifests, lockfile, compiler, target,
   default/optional features, build scripts and runner. Record any missing
   admitted harness and prepare it through the shared setup procedure when authorized.
3. Use a narrow import/adapter seam for synchronization in the **same
   production implementation**. Do not copy its state machine into a second test
   implementation and treat agreement as qualification. After feature and dependency
   graph admission, a feature-gated seam can select these imports:

   ```rust
   #[cfg(feature = "shuttle")]
   pub(crate) use shuttle::{sync, thread};
   #[cfg(not(feature = "shuttle"))]
   pub(crate) use std::{sync, thread};
   ```

   `shuttle` illustrates an application feature, not an existing repository
   feature declaration. Audit every caller for direct imports bypassing this seam.
   A library consumed by an integration test normally builds without `cfg(test)`;
   gating instrumentation on `all(test, feature = "shuttle")` can silently
   leave the production dependency uninstrumented. Select feature/cfg boundaries
   for the actual build topology; a dev-dependency alone may not serve that library build.
4. For Tokio code, review the [wrapper mechanism][wrappers] and actual manifests.
   The Cargo dependency key `tokio` may alias package `shuttle-tokio`; activating
   the application's feature must propagate to `tokio/shuttle`. This replaces
   that dependency edge, not every transitive use of package `tokio`.

| Edge or feature | What the reviewed source actually does |
| --- | --- |
| `shuttle-tokio` with `shuttle` disabled | Re-exports original Tokio; the compatible Tokio version remains independently resolved. |
| `tokio/shuttle` on the wrapper alias | Selects `shuttle-tokio-impl`, which exposes inner models plus some original Tokio modules. |
| `rt`, `sync`, `time`, `macros`, `test-util` | Feature forwarding follows wrapper → impl → inner and original Tokio. Verify exact required row; default features are empty. |
| `full` | Enables the wrapper's defined feature bundle; it does not include `test-util` or unstable features and does not mean all behavior is modeled. |
| `io`, `net`, `fs` and `signal` exports | Original Tokio re-exports, not controlled simulations. Replace reached external operations with explicit test ports. |
| Other crates, direct `std`, parking_lot, Tokio-util and task-local use | Inspect their own imports/features. A wrapper at one application edge or `shuttle_enabler` is no whole-graph instrumentation proof. |

5. Capture normal and instrumented resolved graphs, including duplicate
   package versions, feature unification, normal/build/dev edges and host/target
   differences. Check selected inner versions against the repaired cohort. Older
   mismatched engine/task-local implementations have caused false passes or panics.
   Avoid enabling entire wrapper collections merely to save feature-list maintenance.
6. Map each production task/thread, lock, atomic, channel, wake, cancellation,
   join, clock and protected effect to its actual model adapter or explicit exclusion.
   Construct/reset all per-execution application state inside the runner closure.
   Keep the independent oracle outside production decisions; observations must not
   perform the transition or add synchronization that makes the race impossible.

Sources: [wrapper manifest][wrapper-manifest], [impl exports][impl-exports],
[inner manifest][inner-manifest], [release changes][release-source].

## Model limitations

- **Memory:** [all Shuttle atomic orderings become SeqCst][atomics]. Do not use
  a green run to validate Acquire/Release/Relaxed publication. `shuttle::sync::Arc`
  is currently a standard-library re-export, not an instrumented refcount model.
- **Scheduling:** control uses instrumented yield points. A pure CPU loop,
  OS-blocking lock, native thread or hidden library synchronization may escape
  the scheduler; one apparent step can block forever. Instrument all relevant
  paths, rather than adding arbitrary yields until a test happens to pass.
- **Tokio runtime:** the [runtime adapter][runtime] has no real blocking pool;
  `spawn_blocking` wraps its closure in a modeled future. Builder worker counts,
  runtime metrics and runtime-flavor behavior do not qualify actual pool/resource
  configuration. Native work must remain behind a controlled completion adapter.
- **Time:** [timeout durations/deadlines are ignored][time].
  `trigger_timeouts(predicate)` expires current and future matching task-label
  timeouts; `clear_triggers()` clears triggers, not outstanding timeout futures.
  Cancel/drop/join pending operations and reset the fixture between executions.
  Select labels so unrelated operations do not all expire together.
- **Sleep/intervals:** `pause`/`resume` do nothing; `advance` yields. Most `Sleep`
  polls yield and complete without waiting; very distant deadlines remain pending.
  Intervals yield a bounded number of ticks only when configured with
  `SHUTTLE_INTERVAL_TICKS`; otherwise the default is effectively unbounded.
  Missed-tick settings/reset supply no real timer semantics; the adapter
  still reads Tokio `Instant`. Inject distinct UTC, monotonic and active-awake
  clocks for application decisions; no Windows suspend/resume or clock-write claim follows.
- **Failure cleanup:** unfinished-task stacks may be deliberately retained after
  failure; [shutdown settings][config] also govern closure/future retention.
  Observe product joins/permits/resources before teardown. Use bounded isolated
  processes for repeated negative controls; do not classify tool teardown retention
  as a product leak or let fixture cleanup hide an actual product leak.

## Choose a scheduler and explicit bounds

The [function implementation][api] is authoritative when overview wording differs.
Each tested closure must implement `Fn() + Send + Sync + 'static` and is reused
across executions. Own inputs and initialize fresh case state inside it.

| Scheduler | Exact entry points | Selection and limits |
| --- | --- | --- |
| Random | `check_random(f, iterations)`; `check_random_with_seed(f, seed, iterations)`; `RandomScheduler::new_from_seed(seed, iterations)` | General first search; chooses a runnable task randomly at each scheduling decision. Iterations can revisit equivalent schedules; they are not unique-coverage counts. |
| PCT | `check_pct(f, iterations, depth)`; `PctScheduler::new(depth, iterations)`; `new_from_seed(seed, depth, iterations)` | Prioritizes bugs needing few priority changes. Depth must exceed zero; first iteration estimates scheduling length. Implementation samples up to `depth - 1` change points and also responds to explicit yields. Depth is not a global context-switch cap or a guarantee of defect detection. |
| DFS | `check_dfs(f, Some(max_iterations))`; `DfsScheduler::new(Some(max_iterations), false)` | Tiny finite spaces only. The argument bounds **executions**, not schedule depth. `None` removes that iteration bound. Random-data requests fail by default; allowing them fixes a data stream and does not enumerate all values. |
| Replay | `replay(f, encoded_schedule)`; `replay_from_file(f, path)`; `ReplayScheduler::new_from_file(path)` | Reproduce a recorded execution with identical inputs/source/graph. No new search or automatic minimization. Convenience functions use default config; custom bounds require `Runner`. |

For all policies set a finite execution budget and `Config::max_steps =
MaxSteps::FailAfter(n)`. A step is an instrumented atomic region, not one source
line or one millisecond. `max_time` is checked **between executions only**;
use the admitted external deadline/memory supervisor for stalled iterations.
Bound actors, operations, retries, ticks, input bytes, output and total CPU.
`ContinueAfter` abandons an iteration and proceeds; never report a truncated
execution as clean. `Runner::run` returns an invocation count, not a
coverage/completeness certificate. Compare with the selected budget; report
the stop reason. A small count alone cannot establish DFS exhaustion.

Sources: [random][random], [PCT][pct], [DFS][dfs], [Config][config], [Runner][runner].

## Minimal source-checked example

**Educational only: source-checked against the pinned APIs; not compiled or
executed in this documentation task and not NevroTick qualification.** The
example exposes a lost-update fault: two workers must each contribute exactly
one increment and one distinct effect record. Independent expectation:
`2` and IDs `[0, 1]`, not recomputed through the update implementation.

```rust
use shuttle::scheduler::{RandomScheduler, ReplayScheduler};
use shuttle::sync::{Arc, Mutex};
use shuttle::{thread, Config, FailurePersistence, MaxSteps, Runner};
use std::path::{Path, PathBuf};
use std::time::Duration;

fn counter_case(inject_fault: bool) {
    let state = Arc::new(Mutex::new((0_u32, Vec::<u32>::new())));
    let mut workers = Vec::new();
    for id in 0_u32..2 {
        let state = Arc::clone(&state);
        workers.push(thread::spawn(move || {
            if inject_fault {
                let old = state.lock().unwrap().0;
                thread::yield_now();
                let mut state = state.lock().unwrap();
                state.0 = old + 1; // Fault: update from a stale snapshot.
                state.1.push(id);
            } else {
                let mut state = state.lock().unwrap();
                state.0 += 1;
                state.1.push(id);
            }
        }));
    }
    for worker in workers {
        worker.join().unwrap();
    }
    let state = state.lock().unwrap();
    assert_eq!(state.0, 2, "each worker must contribute one increment");
    let mut observed_effects = state.1.clone();
    observed_effects.sort_unstable();
    assert_eq!(observed_effects, vec![0, 1], "each worker acts once");
}

fn bounded_config(directory: PathBuf) -> Config {
    let mut config = Config::default(); // Config is non-exhaustive.
    config.max_steps = MaxSteps::FailAfter(10_000);
    config.max_time = Some(Duration::from_secs(10));
    config.failure_persistence = FailurePersistence::File(Some(directory));
    config
}

fn explore(inject_fault: bool, directory: PathBuf) {
    let scheduler = RandomScheduler::new_from_seed(0x5eed, 1_000);
    let completed = Runner::new(scheduler, bounded_config(directory))
        .run(move || counter_case(inject_fault));
    assert_eq!(completed, 1_000, "campaign stopped before its budget");
}

fn replay_fault(path: &Path, directory: PathBuf) -> std::io::Result<()> {
    let scheduler = ReplayScheduler::new_from_file(path)?;
    Runner::new(scheduler, bounded_config(directory)).run(|| counter_case(true));
    Ok(())
}
```

Numbers are finite example budgets, not repository defaults or measured costs.
`explore(false, directory)` is the healthy control. A separately selected negative
control calls `explore(true, directory)`; sensitivity requires the
intended assertion to fail and replay. The seed is illustrative,
not an observed counterexample claim. Do not accept a step-limit/setup panic as
that failure. Adapt the pattern to actual production owners after admission;
the counter never substitutes for a product-linked control.
APIs checked: [sync exports][sync], [threads][thread], [scheduler functions][api],
[configuration][config], [runner][runner] and [replay][replay].

## Run and replay procedure

1. After graph/environment and task admission, expose separate, exactly named
   tests for healthy control, intended-fault control and production scenario.
   Use the selected frozen runner from [configured tooling](configured-tooling.md),
   with actual package/target/features and discovered nonzero selection.
   Do not invent `--tool shuttle` or use this example to justify a new Cargo lane.
2. Allocate an owned, existing, writable evidence directory per candidate/case.
   `FailurePersistence::File(Some(directory))` writes the next free
   `scheduleNNN.txt`; `None` inside `File` means current directory. The writer
   creates no missing parent directories; write failure falls back to printing.
   Capture stderr and confirm the emitted path/content exists.
3. Establish healthy control, then a requirement-linked deliberate fault.
   Preserve assertion, concrete inputs, state/effect trace, scheduler/configuration,
   source/dirty bytes, harness/oracle, lockfile and environment with the schedule.
   A historical defect execution does not qualify an unseen candidate.
4. Replay the original fault using `ReplayScheduler::new_from_file(path)` with
   the same `Runner` config and test body. `replay_fault` illustrates
   this. Malformed encoding can panic although file loading returns `Result`.
   Retain original artifact; never overwrite it with a new run or repair result.
5. Check uncontrolled nondeterminism before enlarging search. Wrap the chosen
   scheduler in `UncontrolledNondeterminismCheckScheduler::new(scheduler)` and
   use the same bounded runner, or use `check_uncontrolled_nondeterminism(f, n)`
   with its default config. It replays each generated schedule once and compares
   scheduling/random-data events and runnable sets; budget roughly twice the
   executions. Passing does not prove every uncontrolled source was removed.
6. Record `SHUTTLE_RANDOM_SEED`: it overrides explicit random/PCT constructor
   seeds. Check ambient random generators, hash iteration, wall clocks, globals,
   external I/O and logging that touches synchronization. The Tokio helper's
   `SHUTTLE_TRACE_DIR`, `SHUTTLE_TRACE_FILE`, `SHUTTLE_ITERATIONS` and related
   settings are helper-specific, not universal `Runner` options; do not assume
   an environment variable changed your custom runner.
7. Add bounded random/PCT batches or tiny DFS only for the remaining evidence gap.
   Record seeds and completed invocations. Minimize a failure in a separate
   bounded procedure preserving the intended oracle failure and healthy neighbor.
   Retain original/reduced inputs and schedules, then a named concrete regression.

Sources: [file persistence][persistence], [replay behavior][replay],
[nondeterminism checker][nondeterminism], [Tokio helper settings][helper].

## Requirement-linked controls

| Boundary | Cases and independent observation |
| --- | --- |
| Current authority/generation | Hold completion, replace policy/generation, release completion. Assert returned outcome, published checkpoint and protected effects all use current authority; a copied older snapshot is not a fresh guard. Include the still-current accepted neighbor. |
| Cancellation/permits | Stop before work, while queued, after permit acquisition and during native work. Assert actual outstanding-work custody, no replacement permit while the cancelled operation still owns one, and release only at real completion/recovery. |
| Watch stop/publication | Stop already set; read-before-subscribe; stop between check/register/wait; sender/receiver drop; cancellation and post-stop completion. Observe the durable predicate and publication ordering, not notification receipt alone. |
| Channel reservations/wakes | Cancellation of pending reserve/acquire, notification before/after registration, selected waiter dropped, last sender dropped and receiver closed. Count delivered/forbidden effects and retained permits independently. |
| Restart/reconnect | Old reply after successor ownership, ambiguous acknowledgement and repeated reconnect. Preserve cumulative budgets; report uncertainty instead of silently treating missing acknowledgement as no effect. |
| Shutdown/resource ownership | Observe joins, timers, permits, handles and pending effects before fixture teardown. Crash/abort needs an external recovery owner; destructors cannot be assumed to run. |

Watch retains only its last value; a newly subscribed receiver
considers the current value seen. `borrow` and `borrow_and_update` differ in
seen-state; `changed` and `wait_for` must be checked against the actual adapter.
Keep borrows short, including across waits. Map all reached send/subscribe/borrow/
close/drop paths; query methods such as `has_changed` need not yield. The reviewed
release added scheduling points around several registration/read/drop operations;
more explored schedules can increase runtime without changing your program.
See [watch source][watch-adapter] and [release fixes][release-source].

## Real Tokio counterpart

Run same semantic scenarios through the selected **real current-thread Tokio
runtime**, with shared production decisions and an independently derived oracle.
Use the admitted normal feature row and verify it resolves original Tokio.
Coordinate start, registration, policy replacement and actual completion through
barriers/notifications or controlled ports. A sleep or repeated lucky run does
not demonstrate the intended race ordering. Compare allowed outcomes/effect order,
not exact scheduler traces across different runtimes.

When timer semantics matter, real Tokio paused time needs `time`/`test-util` and
current-thread support. It affects Tokio's `Instant`, not std/Windows clocks;
idle execution may auto-advance. `advance` jumps time but does not await every
expired operation: wait for the application's completion condition. Keep UTC,
monotonic and active-awake decisions independently injectable. See
[real Tokio clock semantics][tokio-clock] and [Rust techniques](rust-techniques.md).
Actual Windows/FFI completion and resource counts remain separately owned native
evidence; the real Tokio counterpart alone does not qualify them.

## Troubleshooting

| Observation | First discriminating check |
| --- | --- |
| Missing API, type mismatch or runtime-context panic | Inspect exact wrapper/inner/engine versions, enabled features, direct-import bypasses and library `cfg(test)` assumptions. Do not add dependencies or change production runtime merely to compile a harness. |
| Deadlock only under Shuttle | Inspect blocked tasks and production wait conditions; compare adapter semantics and known upstream fixes. Pending timer without a trigger, a watch borrow held across wait or unsupported I/O can cause a harness deadlock. Confirm actual owner/control before labelling product RED. |
| Process hangs without hitting step bound | Check OS blocking or pure computation inside one step. `max_time` cannot interrupt it; retain supervisor termination as incomplete evidence. |
| Step bound reached | Distinguish starvation, infinite ticks, busy polling and genuine bounded-progress failure. A sampled scheduler supplies no blanket fairness guarantee. Do not convert the bound to `ContinueAfter` to obtain GREEN. |
| Replay says task not runnable, schedule ended early or wrong event kind | Recheck candidate, inputs, feature graph, adapter version, ambient state and tracing verbosity. Preserve the mismatch and original evidence; never use `set_allow_incomplete` to disguise it. |
| No schedule file | Check emitted path, directory ownership/existence, captured stderr fallback and process-global panic-hook configuration. Separate selected tests into owned processes where necessary. |
| Negative-control memory grows | Distinguish documented failed-execution retention from product state measured before teardown. Bound failures/process lifetime; observe actual task count and optional vector-clock cost. |
| Fast green or fewer iterations than requested | Verify nonzero test discovery, actual production entry-point reachability, scheduler count, time cap and selected controls. Returning without panic does not prove complete search. |

## Report and stopping checklist

- Name requirement/CHECK, expected state/effects and production caller chain.
- Bind candidate including dirty bytes, harness/oracle, concrete input, lockfile,
  compiler/target/features, resolved wrapper cohort and instrumented/excluded paths.
- Record selected/completed executions, step/time/actor/tick/output bounds, seeds,
  ambient overrides, stop reason, runner exits and external supervisor outcome.
- Identify executed healthy/fault controls and intended diagnostic. Keep setup,
  deadlock, truncation, cancellation and ordinary assertion failures distinct.
- Retain original schedule/input/trace, exact replay result, reduction limits and
  concrete regression; schedule replay alone is not automatic minimization.
- Record product cleanup before teardown, real Tokio counterpart and remaining
  native/FFI/weak-memory/timing evidence under their existing owners.
- Stop at the bounded task exit. Missing admission, production
  linkage, discriminating controls, replay or required counterpart stays incomplete;
  a green sampled model supports only its reached instrumented assertions.

[release]: https://github.com/awslabs/shuttle/releases/tag/v0.9.10
[release-source]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/CHANGELOG.md
[overview]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/shuttle/src/lib.rs
[wrappers]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/wrappers/README.md
[wrapper-manifest]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/wrappers/tokio/wrappers/shuttle-tokio/Cargo.toml
[impl-exports]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/wrappers/tokio/impls/tokio/src/lib.rs
[inner-manifest]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/wrappers/tokio/impls/tokio/inner/Cargo.toml
[atomics]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/shuttle-std/src/sync/atomic/mod.rs
[sync]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/shuttle-std/src/sync/mod.rs
[thread]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/shuttle-std/src/thread.rs
[time]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/wrappers/tokio/impls/tokio/inner/src/time.rs
[runtime]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/wrappers/tokio/impls/tokio/inner/src/runtime.rs
[api]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/shuttle-schedulers/src/check.rs
[random]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/shuttle-schedulers/src/random.rs
[pct]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/shuttle-schedulers/src/pct.rs
[dfs]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/shuttle-schedulers/src/dfs.rs
[config]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/shuttle-engine/src/config.rs
[runner]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/shuttle-engine/src/runtime/runner.rs
[persistence]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/shuttle-engine/src/runtime/failure.rs
[replay]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/shuttle-schedulers/src/replay.rs
[nondeterminism]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/shuttle-schedulers/src/uncontrolled_nondeterminism.rs
[helper]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/wrappers/tokio/impls/tokio/inner/src/lib.rs
[watch-adapter]: https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/wrappers/tokio/impls/tokio/inner/src/sync/watch.rs
[tokio-clock]: https://github.com/tokio-rs/tokio/blob/tokio-1.54.1/tokio/src/time/clock.rs
[loom]: https://github.com/tokio-rs/loom/blob/v0.7.2/src/lib.rs

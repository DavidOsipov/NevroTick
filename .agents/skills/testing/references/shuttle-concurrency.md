# Shuttle: instrumented concurrency evidence

For named ownership/cancellation/publication invariant, read [testing contract](../CONTRACT.md),
[configured tooling](configured-tooling.md) and [Rust techniques](rust-techniques.md).
Admits no dependency, installation, product harness or new `check-analysis.py --tool` selection.
Follow current task permissions; preserve remaining Windows/fuzz/storage/UI holds and post-V1 formal deferral.

## Reviewed upstream facts — 2026-11-15

Reviewed `awslabs/shuttle` `v0.9.10` / `855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b`; observation, not admission.
[Changelog][release-source] dates 0.9.10 November 4; [GitHub release][release]
published November 5, 2026. Cohort: `shuttle-engine` 0.1.8 and
`shuttle-tokio-impl-inner` 0.1.7; wrapper `shuttle-tokio` remains 1.0.0.
Repairs: watch/Notify/channel scheduling points; wrapper version alone cannot establish fixed inner selection.
Future adoption requires [dependency review](../../environment/dependency-updates/SKILL.md)
and OD-19's 14-day whole-delta cooldown using every changed registry publication
timestamp. Changelog/release dates prove no registry age or calendar-triggered graph/exception approval.

## Admit the actual boundary

- Record requirement/CHECK owner, independent expected state/effects, current
  source plus dirty bytes, harness/oracle identity, lockfile, compiler, target,
  cfg/features, scheduler, dependency graph and adapter implementations.
- Run actual production decision/ownership logic through admitted adapters; rewrites test themselves.
  Derive oracle independently from contract; compare effects and state.
- Map each task/thread, lock, atomic, channel, wake, cancellation, join and clock
  from production owner to modeled primitive. Inspect transitive dependencies.
  Ordinary Tokio/std/OS synchronization invisible without replacement;
  one wrapper/feature proves no whole-graph instrumentation.
- [Wrappers][wrappers] switch selected libraries by feature. Verify enabled and
  disabled graphs and API semantics; no blanket alias/feature substitution.
  Unsupported or inaccurately modeled Tokio constructs remain gaps.
- [Shuttle atomics][atomics] model all orderings as sequentially consistent;
  no weak-memory correctness claim. Use separately admitted evidence.
- Exclude real I/O, FFI, native threads and OS clock changes from model via
  controlled ports. Modeled completion proves no actual native completion.
- Inject distinct UTC, monotonic and active-awake sources. [Tokio adapter time][time]
  models no elapsed time: timeout durations/deadlines ignored; explicit triggers
  instead. pause/resume do nothing; advance yields. State modeled
  timer rules, bound interval ticks and reset triggers/state between executions.
  Stubs are neither real Tokio paused time nor Windows suspend/resume proof.

## Choose bounded schedules and preserve replay

- [Public APIs][api]: `check_random(f, iterations)`,
  `check_random_with_seed(f, seed, iterations)` and `check_pct(f, iterations, depth)`.
  For tiny spaces, `check_dfs(f, Some(max_iterations))` bounds executions, not
  schedule depth. Sampled or truncated runs prove no exhaustive correctness.
- Use `Runner::new(scheduler, config).run(f)` for explicit policy. Bound executions,
  steps, tasks, input/sequence sizes, retries, output, total CPU/memory/time.
  [Config][config] supports `MaxSteps::FailAfter(n)` and `max_time`; time checked
  only between executions. Retain external deadline/memory supervisor for
  blocked steps or uncontrolled operations. Never turn budget stop into success
  with `ContinueAfter`; report incomplete executions separately.
- Select persistence directory with `FailurePersistence::File(Some(directory))`;
  retain emitted original schedule file, concrete inputs, seed, assertion,
  observed effects and exact identities.
  `replay(f, encoded_schedule)` / `replay_from_file(f, path)` use default config;
  use [ReplayScheduler][replay] with `Runner` to preserve custom bounds/configuration.
- Check uncontrolled randomness, wall-clock reads, hash iteration, ambient
  globals, I/O and logging-dependent synchronization. Use bounded
  `check_uncontrolled_nondeterminism` or its scheduler wrapper: replays schedules,
  compares runnable tasks; passing cannot prove all nondeterminism removed.
- Reproduce intended failure on identical inputs/configuration; never silence mismatches
  with `set_allow_incomplete`. Source/adapter/logging changes can invalidate replay; retain originals.
- Replay is not automatic minimization. Reduce inputs/actors/schedules only through
  separate bounded procedure preserving independent failure and healthy neighbor;
  retain original/reduced artifacts and concrete regression.

## Controls and real-runtime counterpart

- First challenge healthy and deliberately faulty controls through actual owner.
  Executed historical defect control can establish sensitivity, never status of unseen tree/repair.
- For watch-based stop/publication, exercise read-before-subscribe, stop between
  check/register/wait, stop already set, sender/receiver drop, cancellation, late
  completion and post-stop publication. [Tokio watch][watch] retains only last
  value; new subscription considers current value seen. Assert actual state predicate
  and lifecycle contract; notification receipt insufficient.
- Verify mapped `send`, `subscribe`, borrow/seen-state, `changed`/`wait_for`, close
  and drop paths against [adapter source][watch-adapter]. Recent scheduling fixes
  enlarge explored schedules; query methods need not create scheduling points.
- Repeat same semantic scenarios through actual selected Tokio runtime
  with controlled barriers/notifications and bounded deadlines. Keep production
  logic shared, oracle independent; sleeps/repeated lucky runs are not race controls.
  Separate real runtime/native Windows/external-operation evidence.
- Observe application joins/permits/resources before harness teardown. Cancellation
  of awaiting future does not release custody of still-running native work.
  Separate documented Shuttle failure-teardown retention from product leaks;
  use bounded isolated processes for repeated failing executions.

Report selected/completed schedules, steps/bounds, controls, replay, instrumentation/exclusions,
exact identities and cleanup. Missing admission, production linkage, controls, replay or real-runtime
prerequisites remain incomplete; green supports only reached instrumented assertions.

[release-source]: https://github.com/awslabs/shuttle/blob/v0.9.10/CHANGELOG.md
[release]: https://github.com/awslabs/shuttle/releases/tag/v0.9.10
[wrappers]: https://github.com/awslabs/shuttle/blob/v0.9.10/wrappers/README.md
[atomics]: https://github.com/awslabs/shuttle/blob/v0.9.10/shuttle-std/src/sync/atomic/mod.rs
[time]: https://github.com/awslabs/shuttle/blob/v0.9.10/wrappers/tokio/impls/tokio/inner/src/time.rs
[api]: https://github.com/awslabs/shuttle/blob/v0.9.10/shuttle-schedulers/src/check.rs
[config]: https://github.com/awslabs/shuttle/blob/v0.9.10/shuttle-engine/src/config.rs
[replay]: https://github.com/awslabs/shuttle/blob/v0.9.10/shuttle-schedulers/src/replay.rs
[watch]: https://docs.rs/tokio/1.54.1/tokio/sync/watch/index.html
[watch-adapter]: https://github.com/awslabs/shuttle/blob/v0.9.10/wrappers/tokio/impls/tokio/inner/src/sync/watch.rs

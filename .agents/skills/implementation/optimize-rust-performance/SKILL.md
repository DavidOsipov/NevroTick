---
name: "optimize-rust-performance"
description: "Profile and optimize NevroTick Rust/Windows CPU, allocation, latency, wakeups, startup or binary-size bottlenecks with comparable before/after measurements and preserved correctness/security contracts. Use for a performance regression or explicit optimization task; resource assessment alone belongs to qa-performance-resources."
---

# Optimize measured Rust and Windows costs

Read [the implementation contract](../CONTRACT.md). Preserve requested
implementation, planning or review scope. Use
[architecture section 17](../../../../docs/windows-time-architecture.md#17-resource-and-performance-targets)
for budgets and runtime constraints: proposed targets are not measured results.
Change a budget only through the existing explicit decision process.

For benchmark/profiler selection and comparison design, read
[measurement guidance](references/measurement.md). Use
[resource QA](../../testing/qa-performance-resources/SKILL.md) for independent
measurement and acceptance; optimization and resource qualification answer
different questions.

## Establish the cause before changing it

1. Name the observed cost, useful-work outcome and affected owner/callers.
   Distinguish CPU execution from waiting, allocation rate from retained growth,
   throughput from tail latency, and cold startup from steady state. Trace the
   real producer → state → consumer path, including rejection and recovery.
2. Freeze baseline source/dirty state, artifact, target/features, toolchain,
   effective profile and environment. Define representative input, measurement
   start/end events, primary metric, guard metrics, repeat/time/storage limits
   and decision rule before tuning. Reproduce the cost with useful behavior
   intact; an empty workload or disabled functionality is no baseline.
3. Attribute the cost with the smallest supported observation: sampled stacks,
   wait/queue timing, allocation/retention evidence, call counts or emitted
   artifact inventory. Use matching symbols and inspect real callers. A hot
   stack or expensive-looking source suggests a hypothesis, not a proven cause.
   Record unsupported native paths and profiler overhead explicitly.

## Make one causal change

Prefer removing measured duplicate work, reducing bounded algorithmic work or
allocations, and correcting ownership/scheduling before adding architectural
complexity. Useful choices depend on the observed cause:

| Observed cost | Candidate change and obligation |
| --- | --- |
| Repeated decode/copy/allocation | Reuse immutable checked values or bounded buffers; preserve admission and output invariants, ownership and secret handling. |
| Excessive repeated computation | Improve the algorithm or use a bounded cache with complete identity, generation, invalidation and eviction rules; measure cold/miss/hostile cases as well as hits. |
| Busy polling or redundant UI updates | Coalesce equivalent work or use event-driven notification within canonical freshness and responsiveness contracts. |
| Lock/queue/blocking delay | Shorten demonstrated contention while preserving complete invariant-bearing transactions, fairness, byte/work bounds and permits held by actual native jobs. |
| Startup or artifact bloat | Inspect actual initialization, selected features and emitted payloads; measure exact profile changes and route dependency changes through admission. |

Change one causal factor at a time where practical. Preserve the original
candidate for comparison and existing user edits. Do not add a runtime, cache,
allocator, worker pool, benchmark dependency or new unsafe code merely because
it might be faster; each needs its existing admission/review and evidence.

Mandatory garde-backed construction, strict decoding, authorization/freshness
checks, redaction, checked arithmetic and Windows mitigations remain enforced.
Reuse proven immutable domain invariants without repeating unchanged validation;
re-establish transformed outputs and current contextual guards where required.
Never skip validation, narrow a lock past guard/effect atomicity, weaken hashing
against hostile input, use fast-math, or change estimator precision to win a
benchmark. Preserve numerical/time-domain contracts and useful failure paths.
Do not disable Defender/protections, raise global timer resolution or expand
resource budgets as an incidental optimization.

## Verify and decide

Run independent correctness checks through the changed production entry points,
including a useful accepted neighbor and relevant hostile/boundary/recovery
cases. Select the affected compiler/lint, contract, numerical, ownership and
concurrency checks through [configured tooling](../../testing/references/configured-tooling.md).
Changed native lifetimes also use [memory diagnostics](../../testing/references/windows-memory.md);
keep those instrumented runs separate from representative performance samples.

Compare bounded repeated baseline/candidate runs under comparable conditions,
alternating order where practical. Retain raw samples, failures and outliers;
report absolute change, relative change when meaningful, spread and guard-metric
trade-offs. A microbenchmark improvement must reach the affected real workload
before claiming a product improvement. Noise, missing observations or changed
conditions can leave the result inconclusive.

Keep an optimization only when the declared decision rule and correctness checks
support its benefit. Otherwise revert only task-owned experimental edits or
report the unresolved comparison; do not label an unmeasured refactor faster.
Stop the bounded experiment when its question is resolved or its budget is
consumed. Record hypothesis, exact compared identities, change, commands,
measurements, regressions/trade-offs and remaining gaps in the existing task
record; no new report tree is required.

Without real Cargo source, produce only scoped guidance/analysis; do not create
an application to manufacture a benchmark. UI-M0 stays fixture-only with D1–D3
deferred. A targeted experiment does not complete CHECK-21's one-hour baseline
or 72-hour reconnect/fault soak, and does not authorize clock/service experiments.

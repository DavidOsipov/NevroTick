# Performance measurement and profiling

Use with [the optimization workflow](../SKILL.md) and
[resource QA](../../../testing/qa-performance-resources/SKILL.md). The
[architecture](../../../../../docs/windows-time-architecture.md#17-resource-and-performance-targets)
owns budgets, clock domains, runtime limits and milestone applicability. This
reference selects evidence; it admits no new tools or dependencies.

## Define a comparison that can answer the question

Record baseline/candidate source including dirty content, binary and matching
symbols, lockfile/features, target, compiler/linker and effective build flags.
Match workload/corpus, seeds, relevant input sizes, process identity, OS/hardware,
power/thermal conditions, Windows security settings and background activity.
Do not compare debug with optimized builds and attribute the whole difference
to a code edit. Profile/feature changes are themselves the experimental factor;
keep other relevant inputs fixed and record both configurations.

Define useful work and start/end events before sampling. Separate initialization,
cold cache, warm cache, steady tracking, invalid-input rejection and recovery.
For latency/queue tests, specify offered arrival rate, concurrency and completed,
rejected, timed-out and dropped work; a faster reject-all or silently reduced load
is a failed comparison. Include relevant cache misses, source churn and bounded
hostile traffic so a common-case gain cannot hide unbounded worst-case work.

Choose the primary metric and regression constraints before seeing results.
Use bounded repeated runs, warm-up where appropriate and alternating baseline/
candidate order when practical. Retain all raw observations and reasons for
failed/missing samples. Report sample count, units, distribution/spread and the
preselected statistic; never select only the fastest trial or discard a bad
tail without explanation. A p95 needs enough observations to be informative;
do not invent confidence from a handful of trials or correlated loop iterations.

For a lower-is-better metric, report both the absolute difference and, when the
baseline is positive, `(baseline - candidate) / baseline * 100%` as reduction.
Distinguish reduction from speedup/throughput. Use a justified uncertainty method
if giving intervals; a difference near measurement noise remains inconclusive.
No universal percentage improvement or fixed trial count replaces the workload's
declared decision rule. Report whether the canonical budget is actually met
separately from whether a local change improved a metric.

Use an interval clock appropriate to the requirement, never wall-clock UTC for
elapsed benchmarking in a clock-changing system. Windows QPC is independent of
system-time adjustments but includes suspended time; it is not the active-awake
clock. Record suspend/resume, clock adapter, resolution and timer overhead, and
keep very small durations above measurement noise. Do not substitute direct TSC
reads or change global timer resolution. See Microsoft's
[high-resolution timestamps](https://learn.microsoft.com/en-us/windows/win32/sysinfo/acquiring-high-resolution-time-stamps).

## Select an observation, then verify the real workload

| Question | Smallest useful evidence and limits |
| --- | --- |
| Where is CPU spent? | Sample the owning process with matching symbols; separate running stacks from waiting. A sample share is attribution within the capture, not the promised saving from removing a function. |
| Why is a request/UI action slow? | Measure its full admission-to-completion interval and queue/blocking stages. Include tail latency, cancellation and busy/rejection outcomes. Pure-function speed alone does not explain end-to-end delay. |
| Why does memory grow? | Distinguish allocation rate, peak/retained private allocations and allocator caching; count owning handles/tasks/queues and release events before teardown. Use the shared native-memory workflow for corruption evidence. |
| Why are idle cost or wakeups high? | Observe timer/event/job sources and service ownership over the declared window. Record idle and active work separately; throughput tests cannot establish idle efficiency. |
| Why are startup or size high? | Observe cold usable-state events and the emitted binary/MSI plus required payloads. Inspect initialization and actual feature/dependency contributions; neither stripped symbols nor omitted required runtime payloads prove a valid product saving. |

Tool selection is conditional:

- **Existing native counters or scoped in-process timing** may already answer
  the question. Define what they count and corroborate the owning process/work.
- **Windows Performance Recorder/Analyzer (WPR/WPA)** are optional native
  profiling candidates. Inspect installation, version, available profiles,
  privilege/session requirements and symbol support before use. Installing SDK
  Debugging Tools/AppVerifier does not establish that this profiling lane is
  installed or admitted. Select only needed providers/stacks, duration, trace
  storage and an owned session; do not cancel unrelated recordings or run an
  ordinary candidate elevated merely to collect a trace.
- **A focused Rust benchmark harness** can isolate a measured pure hot path.
  Reuse admitted infrastructure before proposing a crate. Inspect the actual
  Cargo benchmark target, effective profile and selected features. Cargo's
  `bench` profile inherits `release`, but configuration/environment overrides
  affect effective settings; keep required overflow checks, panic policy and
  mitigations. Higher optimization levels need measurement rather than an
  assumption of speed. See [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html).
- Use runtime-varying representative inputs and observe meaningful output so
  the compiler cannot simply remove useful work. Rust's
  [`black_box`](https://doc.rust-lang.org/std/hint/fn.black_box.html) is a
  best-effort benchmark aid, not a guarantee against optimization and not a
  constant-time or security primitive. Validate output independently and inspect
  the timed region; include allocation/setup when it belongs to the claimed cost.

WPR memory recording is circular and can overwrite earlier events; file recording
can grow until storage is exhausted. Bound capture duration/storage externally,
choose buffer/detail settings deliberately, and inspect dropped events and
scenario coverage before interpreting absence. Trace files may expose paths or
sensitive payloads; use synthetic inputs, restricted local storage and the
existing redaction policy. Stop/export only owned sessions and verify cleanup.
See [WPR logging modes](https://learn.microsoft.com/en-us/windows-hardware/test/wpt/logging-mode).

WPA's comparison view can compare baseline and candidate traces, but selected
intervals must represent the same scenario. It does not establish causality or
statistical significance. See
[WPA comparative analysis](https://learn.microsoft.com/en-us/windows-hardware/test/wpt/comparative-analysis-views).

Use profiled/instrumented runs to locate a cause, then confirm the proposed
benefit with comparable representative runs without heavy instrumentation.
AppVerifier/PageHeap, Miri, sanitizer, coverage and debugger timings are not
shipping performance measurements. Preserve symbols and raw evidence, plus
differences between instrumented and measurement artifacts. Host/Linux results
support only their measured scope; Windows runtime/UI/service claims need
the corresponding native observations. Local improvements leave applicable
CHECK-21 and package qualification outstanding.

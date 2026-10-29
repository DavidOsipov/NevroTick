---
name: "qa-performance-resources"
description: "Measure NevroTick CPU, memory, wakeups, handles, queues and responsiveness under declared steady, hostile and recovery workloads."
---

# Measure resource and responsiveness budgets

Read [the testing contract](../CONTRACT.md).

For authorized optimization work, pair this measurement workflow with
[performance optimization](../../implementation/optimize-rust-performance/SKILL.md).
Use its [measurement reference](../../implementation/optimize-rust-performance/references/measurement.md)
when choosing profiling/benchmark methods or comparing candidates. QA-only scope
does not authorize product edits; identify the bottleneck and evidence gap.

Use architecture section 17/CHECK-21 budgets; identify named hardware, Windows security profile, candidate hash, workload. No invented thresholds or VM substitution for hardware-specific requirement.

Choose bounded steady tracking/idle UI/reconnect/outage/malformed requests/NAK storms/source changes/logging/long-running recovery workloads. Measure actual owning process/service, CPU/working set/handles/threads/wakeups, queue/work bounds, UI/control-loop responsiveness.

Before sampling define metric start/stop events, bounded repeats/warmup, preselected statistic/spread. Retain all raw samples/outliers/failed/missing observations/reasons; unavailable is not zero. Compare only comparable candidate/environment identities; near-noise/unspecified-decision-rule verdict remains uncertain.

Observe before fixture teardown; forced child cleanup must not hide leaks. Separate cold/warm caches/startup/steady state; preserve time series/workload inputs/first spikes/failures. Keep UTC/monotonic/active-awake distinctions.

For suspected native heap/handle misuse or retained resources, read
[Windows memory diagnostics](../references/windows-memory.md). Separate
Application Verifier/PageHeap fault detection from measured allocation, handle,
thread and queue retention across repeated lifecycle/recovery cases. Heavy
instrumentation cannot supply the uninstrumented performance budget; nextest's
leak timeout is no heap-leak check.

A short smoke does not establish the planned one-hour baseline or 72-hour soak. Run only the authorized scope/budget and report remaining qualification. Read [Windows evidence](../references/windows-evidence.md); instrumentation overhead and unavailable physical power states remain explicit.

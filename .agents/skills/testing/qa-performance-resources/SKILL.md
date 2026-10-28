---
name: "qa-performance-resources"
description: "Measure NevroTick CPU, memory, wakeups, handles, queues and responsiveness under declared steady, hostile and recovery workloads."
---

# Measure resource and responsiveness budgets

Read [the testing contract](../CONTRACT.md).

Use the architecture's section 17 and CHECK-21 budgets; identify named hardware, Windows security profile, candidate hash and workload. Do not substitute invented thresholds or a VM measurement for a hardware-specific requirement.

Choose bounded workloads for steady tracking/idle UI, reconnect/outage, malformed requests, NAK storms, source changes, logging and long-running recovery. Measure the actual owning process/service, CPU/working set/handles/threads/wakeups, queue/work bounds and UI/control-loop responsiveness.

Define the metric's start/stop events, bounded repeats/warmup and preselected statistic plus spread before sampling. Retain all raw samples, outliers and failed/missing observations with reasons; an unavailable measurement is not zero. Compare before/after only with comparable candidate/environment identities, and keep a near-noise or unspecified-decision-rule verdict uncertain.

Observe before fixture teardown so forced child cleanup cannot hide leaks. Separate cold/warm caches and startup from steady state; preserve time series, workload inputs and first spikes/failures. Keep UTC/monotonic/active-awake timing distinctions.

A short smoke does not establish the planned one-hour baseline or 72-hour soak. Run only the authorized scope/budget and report remaining qualification. Read [Windows evidence](../references/windows-evidence.md); instrumentation overhead and unavailable physical power states remain explicit.

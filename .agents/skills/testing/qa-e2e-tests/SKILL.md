---
name: "qa-e2e-tests"
description: "Test a NevroTick user or service journey through built executable entry points and actual observable Windows results."
---

# Exercise complete Windows journeys

Read [the testing contract](../CONTRACT.md).

Select finite journey/independent acceptance facts; identify built candidate, feature/target configuration, isolated Windows environment. Use actual installed/built UI/CLI/services for selected scope; internal helpers cannot replace journey.

Exercise relevant starting states: boot before login, unelevated UI, connect/acquire/track, outage/holdover/recovery, settings/elevation, package maintenance. Check trust/freshness/uncertainty semantics/actual authorized effects; Sync now requests measurements, never unconditional stepping.

Observe relevant real UIA/keyboard interactions, IPC, persisted state, broker acknowledgements. Disposable environment for OS effects; prove cleanup/restoration. Name omitted hardware/security-profile/account combinations.

Read [Windows evidence](../references/windows-evidence.md). E2E success on one environment does not replace numerical vectors, all package lifecycle faults or independent final acceptance.

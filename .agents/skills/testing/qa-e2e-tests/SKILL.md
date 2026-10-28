---
name: "qa-e2e-tests"
description: "Test a NevroTick user or service journey through built executable entry points and actual observable Windows results."
---

# Exercise complete Windows journeys

Read [the testing contract](../CONTRACT.md).

Select a finite journey and independent acceptance facts, then identify the built candidate, feature/target configuration and isolated Windows environment. Use actual installed/built UI, CLI and services for the selected scope rather than calling internal helpers in place of the journey.

Exercise relevant starting states: boot before login, unelevated UI, connect/acquire/track, outage/holdover/recovery, settings/elevation or package maintenance. Check trust/freshness/uncertainty semantics and actual authorized effects; pressing Sync now requests measurements rather than unconditional stepping.

Observe real UIA/keyboard interactions, IPC, persisted state and broker acknowledgements as relevant. Keep OS effects in a disposable environment and prove cleanup/restoration. Name omitted hardware, security-profile or account combinations.

Read [Windows evidence](../references/windows-evidence.md). E2E success on one environment does not replace numerical vectors, all package lifecycle faults or independent final acceptance.

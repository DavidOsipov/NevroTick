---
name: "qa-fault-recovery"
description: "Test NevroTick controlled failures for bounded recovery, preserved authority, durable state and honest status."
---

# Inject faults and verify recovery

Read [the testing contract](../CONTRACT.md).

Before injection select actual failure seam/expected phase-specific state. Trace relevant policy/journal writes, network associations, broker effects, SCM transitions, installer compensation.

Inject disk-full/interrupted replacement, corrupt/old state, process death, lost acknowledgements, delayed native calls, source outage, managed/competing clock ownership. Verify prior valid policy survives failed updates; invalid mandatory authority blocks effects; optional cache corruption triggers documented reacquisition; budgets do not reset.

Clock/installer faults: isolated Windows machine/external cleanup/restoration owner. Test mid-system-change interruption; assume no universal MSI custom-action reversal. Recovery must respect successor ownership; no blind rate reset.

Use [architecture §20.2.1](../../../../docs/windows-time-architecture.md#2021-contract-review-acceptance-refinements)
for reservation write/flush, clock effect, outcome write/flush and reply faults;
unknown effects remain charged, without duplicate steps. Inspect adjustment mode
as well as rate; include intact-but-wrong floor recovery, Fast Startup epochs,
queued SCM restart during maintenance and wrong-clock/offline helper restoration
when those owners are in scope. Retain valid neighboring recovery controls.

Include normal-operation/recovery success controls. Observe bounded retries, failure state/status, actual resources before teardown. Use [Windows evidence](../references/windows-evidence.md)/selected CHECK cases; classify harness/reset failures separately.

---
name: "qa-fault-recovery"
description: "Test NevroTick controlled failures for bounded recovery, preserved authority, durable state and honest status."
---

# Inject faults and verify recovery

Read [the testing contract](../CONTRACT.md).

Before injection select actual failure seam/expected phase-specific state. Trace relevant policy/journal writes, network associations, broker effects, SCM transitions, installer compensation.

Inject disk-full/interrupted replacement, corrupt/old state, process death, lost acknowledgements, delayed native calls, source outage, managed/competing clock ownership. Verify prior valid policy survives failed updates; invalid mandatory authority blocks effects; optional cache corruption triggers documented reacquisition; budgets do not reset.

Clock/installer faults: isolated Windows machine/external cleanup/restoration owner. Test mid-system-change interruption; assume no universal MSI custom-action reversal. Recovery must respect successor ownership; no blind rate reset.

Include normal-operation/recovery success controls. Observe bounded retries, failure state/status, actual resources before teardown. Use [Windows evidence](../references/windows-evidence.md)/selected CHECK cases; classify harness/reset failures separately.

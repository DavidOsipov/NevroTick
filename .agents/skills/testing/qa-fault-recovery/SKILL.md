---
name: "qa-fault-recovery"
description: "Test NevroTick controlled failures for bounded recovery, preserved authority, durable state and honest status."
---

# Inject faults and verify recovery

Read [the testing contract](../CONTRACT.md).

Select the actual failure seam and expected phase-specific state before injection. Trace policy/journal writes, network associations, broker effects, SCM transitions and installer compensation where relevant.

Inject disk-full/interrupted replacement, corrupt/old state, process death, lost acknowledgements, delayed native calls, source outage and managed/competing clock ownership. Verify prior valid policy survives failed updates, invalid mandatory authority blocks effects, optional cache corruption triggers documented reacquisition, and budgets are not reset.

For clock/installer faults use an isolated Windows machine and external cleanup/restoration owner. Test interruption in the middle of system changes; MSI rollback cannot be assumed to reverse every custom action. Recovery must respect successor ownership rather than blindly resetting a rate.

Include normal successful operation and recovery controls. Observe bounded retries, state/status after failure and actual resources before teardown. Use [Windows evidence](../references/windows-evidence.md) and the selected CHECK cases; classify harness/reset failures separately.

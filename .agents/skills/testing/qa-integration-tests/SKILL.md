---
name: "qa-integration-tests"
description: "Verify NevroTick behavior across real Rust modules, processes, protocol peers or persistence adapters at an explicitly chosen integration boundary."
---

# Test real collaborator boundaries

Read [the testing contract](../CONTRACT.md).

Map the production path across the selected collaborators and identify the first actual effect. Use real decoder/domain/guard consumers and existing adapters; substitute only external dependencies necessary for deterministic isolation.

Run controlled loopback NTP/NTS peers, real file replacement or real Windows named pipes/service adapters according to the claim. Test not just serialization but caller admission, data provenance, stale generations, failure atomicity and acknowledgement reconciliation. Include an independently valid control beside the failure path.

Use [numerical/protocol truth](../references/numerical-protocol.md) for vectors and [Windows evidence](../references/windows-evidence.md) for tokens, ACLs and actuation. Pure transport mocks cannot establish protected OS behavior.

Assign fixture/port/path/process ownership, cap startup/operation/shutdown time and inspect cleanup before forced teardown. Retain actual inputs/outputs and native outcomes; broader application journeys remain unproved unless exercised.

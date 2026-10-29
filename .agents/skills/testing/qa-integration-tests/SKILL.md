---
name: "qa-integration-tests"
description: "Verify NevroTick behavior across real Rust modules, processes, protocol peers or persistence adapters at an explicitly chosen integration boundary."
---

# Test real collaborator boundaries

Read [the testing contract](../CONTRACT.md).

Map production path across selected collaborators; identify first actual effect. Use real decoder/domain/guard consumers/existing adapters; substitute only external dependencies needed for deterministic isolation.

At each module handoff, verify the consumer receives the checked value and the
producer's postconditions satisfy its preconditions. Exercise a transformation
or current-state change that invalidates a formerly valid input; no original DTO,
borrowed validation result or stale wrapper may bypass the enforcing constructor
or effect guard. Derive expectations independently of garde's rule implementation.

Per claim, run controlled loopback NTP/NTS peers, real file replacement or real Windows named pipes/service adapters. Beyond serialization, test caller admission/data provenance/stale generations/failure atomicity/acknowledgement reconciliation. Include independently valid control beside failure path.

Use [numerical/protocol truth](../references/numerical-protocol.md) for vectors and [Windows evidence](../references/windows-evidence.md) for tokens, ACLs and actuation. Pure transport mocks cannot establish protected OS behavior.

Assign fixture/port/path/process ownership; cap startup/operation/shutdown time; inspect cleanup before forced teardown. Retain actual inputs/outputs/native outcomes; unexercised broader journeys remain unproved.

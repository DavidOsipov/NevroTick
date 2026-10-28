---
name: "qa-component-tests"
description: "Verify a bounded NevroTick component such as a controller, IPC client or native view with explicit collaborator and platform limits."
---

# Test native application components

Read [the testing contract](../CONTRACT.md).

Choose component boundary/observable contract. Exercise production component with controlled collaborators; identify real/doubled transport, clock, persistence, OS, UI.

Drive inputs through public component interface; assert resulting state, emitted commands/events, exposed accessibility semantics, cleanup. Cover success/rejection/collaborator failure/late completion after generation/cancellation changes. Observer must not set desired state.

UI: test status projection/settings validation/command availability/stale/reconnecting states without normal-user process clock authority. In-process view tests cannot prove native keyboard/AT delivery/UAC behavior. Use [Windows evidence](../references/windows-evidence.md) for required real HWND/UIA path.

Use bounded completion conditions; no fixed sleeps. Retain/report real integration/native follow-up for boundary omitted by doubles.

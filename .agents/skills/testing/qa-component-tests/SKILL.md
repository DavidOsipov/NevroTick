---
name: "qa-component-tests"
description: "Verify a bounded NevroTick component such as a controller, IPC client or native view with explicit collaborator and platform limits."
---

# Test native application components

Read [the testing contract](../CONTRACT.md).

Choose a component boundary and its observable contract. Exercise the production component with controlled collaborators; name which transport, clock, persistence, OS and UI parts are real versus doubled.

Drive inputs through the public component interface and assert resulting state, emitted commands/events, accessibility semantics where exposed and resource cleanup. Cover success, rejection, collaborator failure and late completion after generation/cancellation changes. The observer must not set the desired state itself.

For the UI, test status projection, settings validation, command availability and stale/reconnecting states without granting the normal-user process clock authority. In-process view tests cannot prove native keyboard/AT delivery or UAC behavior. Use [Windows evidence](../references/windows-evidence.md) when a real HWND/UIA path is required.

Use bounded completion conditions rather than fixed sleeps. Keep a real integration/native follow-up for the boundary omitted by doubles and report it explicitly.

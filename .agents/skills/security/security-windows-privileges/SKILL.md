---
name: "security-windows-privileges"
description: "Implement or review NevroTick Windows service identities, effective tokens, privilege limits, process access, DLL loading and clock ownership enforcement."
---

# Enforce Windows authority at the effect

Read [the security contract](../CONTRACT.md), architecture sections 4, 10–11 and 15, and [native proof requirements](../references/native-proof.md) for OS qualification.

For SCM/service work, use [implement-windows-service](../../implementation/implement-windows-service/SKILL.md)
for lifecycle routing and the service guide's [object/operation access matrix](../../implementation/implement-rust/references/windows-service-adapters.md#freeze-the-object-and-operation-access-matrix)
and [per-service recovery matrix](../../implementation/implement-rust/references/windows-service-adapters.md#freeze-service-configuration-and-recovery-behavior).
Keep installer/helper configuration authority separate from runtime status
reporting; verify effective rights and recovery behavior under actual accounts.

Apply [management admission and recovery](../../../../docs/windows-time-architecture.md#421-service-management-admission-and-administrative-recovery).
SCM rights authorize principals, not EXE names; record exact normal/admin routes
and fixed verbs. Normal management requires qualified component admission, current actor
authority and effect guards. Preserve explicit elevated recovery
without a healthy UI/service. Test prior-handle authority across DACL repair and
final-object security after move/replacement; local-only IPC proves no remote-SCM
restriction. Package identity supplies OS evidence; proves neither uncompromised
live code nor permission for every operation.

For service-scoped network restrictions or event-provider authority, read relevant [Windows host integration](../references/windows-host-integration.md) sections. Distinguish actual token/service and direct/delegated network owners from configured policy; require bounded product-owned restoration.

1. Identify caller, target securable object, requested rights and protected effect. Preserve own-process engine/broker separation: network-facing engine lacks clock-setting privilege; only narrow broker writes global clock; normal UI is `asInvoker`; installer/helper owns bounded elevated lifecycle work.
2. Apply [the canonical service profiles](../../../../docs/windows-time-architecture.md#411-service-privilege-profiles). Inspect actual SCM account, own-process/service SID, explicit nonempty required-privilege configuration and complete primary-token privilege set. Reject every extra privilege, including disabled/unknown entries, missing required privilege and incomplete capture; three forbidden-name checks are insufficient. Configuration is a startup filter, not a grant or current-token proof. Account names prove no isolation. Keep captured client tokens separate from the service profile.
3. Use service-SID-scoped ACLs and minimum specific rights for process/thread/token/pipe/files/registry/SCM objects. No broad LocalService write access, engine-to-broker injection/token duplication/writable memory or inheritable privileged handles. Keep broker external network restriction explicit; virtual service identity alone does not remove networking.
4. Keep raw Windows calls and unsafe validity/ownership/thread contracts in reviewed platform adapter. Use owned non-inheritable RAII handles, stable handles for identity-sensitive decisions and captured native errors. Explicit cleanup covers shutdown/cancellation; destructors cannot run after arbitrary process death.
5. Before every clock write, including reset/startup/cancellation/recovery, establish current product ownership generation and absence of known successor/controller conflict. Saved journals or matching numeric rates are insufficient. Ambiguous/lost ownership stops internal actuation and reports Blocked; never overwrite potentially foreign global adjustment.
6. Use protected installation paths and reviewed DLL search policy. Inspect final binaries/imports and actual loads. Source mitigation flags or assumed system DLL search order prove no emitted/effective protection.

For broker actuation, qualify the canonical serialized privilege lifetime:
current guards, correct non-impersonating context, checked API-specific
enable/use/disable and observed disabled baseline before ordinary work. Check
`ERROR_NOT_ALL_ASSIGNED` as well as the Boolean result; removal is not temporary
disabling. A primary-token change is process-wide, and a compromised broker can
re-enable a present privilege. Provider, locked-memory, peer-query or child-launch
failure never authorizes adding privileges; preserve the existing operation owner
and report concrete incompatibility. Object access masks/default-owner eligibility
are separate from `Se*` privileges. Use
[the paired profile cases](../../../../docs/windows-time-development-hardening.md#service-privilege-profile-qualification)
and existing W11-W05/W11-W10 handoff; do not create a second policy/verifier.

Select real Windows checks with [qa-security-boundaries](../../testing/qa-security-boundaries/SKILL.md): actual service tokens, prohibited rights/effects denied, legitimate broker operation succeeds, startup misconfiguration blocks, known ownership handover/restoration and successor ownership preserved. Restrict real clock probes to approved disposable VM contract. Report authority/ACL/token evidence, actual rights and remaining host/administrator race limits.

For uncertain routes into privileged adapters, use
[CodeQL data-flow analysis](../../testing/references/codeql-data-flow.md) to
follow external command/path/value inputs to clock, SCM, token, process and
file sinks. Confirm current guards at actual callers and record FFI/model gaps.
A Rust wrapper's checked type or a static path result does not establish native
rights, ownership, DLL resolution or protection against replacement races.

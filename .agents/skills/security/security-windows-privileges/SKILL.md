---
name: "security-windows-privileges"
description: "Implement or review NevroTick Windows service identities, effective tokens, privilege limits, process access, DLL loading and clock ownership enforcement."
---

# Enforce Windows authority at the effect

Read [the security contract](../CONTRACT.md), architecture sections 4, 10–11 and 15, and [native proof requirements](../references/native-proof.md) for OS qualification.

For SCM/service hardening, use the [service adapter guide](../../implementation/implement-rust/references/windows-service-adapters.md), especially its [object/operation access matrix](../../implementation/implement-rust/references/windows-service-adapters.md#freeze-the-object-and-operation-access-matrix) and [per-service recovery matrix](../../implementation/implement-rust/references/windows-service-adapters.md#freeze-service-configuration-and-recovery-behavior). Keep installer/helper configuration authority separate from runtime status reporting; verify effective rights and recovery behavior under actual accounts.

For service-scoped network restrictions or event-provider authority, read relevant [Windows host integration](../references/windows-host-integration.md) sections. Distinguish actual token/service and direct/delegated network owners from configured policy; require bounded product-owned restoration.

1. Identify caller, target securable object, requested rights and protected effect. Preserve own-process engine/broker separation: network-facing engine lacks clock-setting privilege; only narrow broker writes global clock; normal UI is `asInvoker`; installer/helper owns bounded elevated lifecycle work.
2. Inspect actual SCM account, service SID, required-privilege configuration and effective runtime tokens. Disabled token privileges may remain activatable. Prove engine lacks `SeSystemtimePrivilege`, `SeDebugPrivilege`, `SeImpersonatePrivilege` and unnecessary token/process rights; preserve architecture's documented OS exception. Account names prove no isolation.
3. Use service-SID-scoped ACLs and minimum specific rights for process/thread/token/pipe/files/registry/SCM objects. No broad LocalService write access, engine-to-broker injection/token duplication/writable memory or inheritable privileged handles. Keep broker external network restriction explicit; virtual service identity alone does not remove networking.
4. Keep raw Windows calls and unsafe validity/ownership/thread contracts in reviewed platform adapter. Use owned non-inheritable RAII handles, stable handles for identity-sensitive decisions and captured native errors. Explicit cleanup covers shutdown/cancellation; destructors cannot run after arbitrary process death.
5. Before every clock write, including reset/startup/cancellation/recovery, establish current product ownership generation and absence of known successor/controller conflict. Saved journals or matching numeric rates are insufficient. Ambiguous/lost ownership stops internal actuation and reports Blocked; never overwrite potentially foreign global adjustment.
6. Use protected installation paths and reviewed DLL search policy. Inspect final binaries/imports and actual loads. Source mitigation flags or assumed system DLL search order prove no emitted/effective protection.

Select real Windows checks with [qa-security-boundaries](../../testing/qa-security-boundaries/SKILL.md): actual service tokens, prohibited rights/effects denied, legitimate broker operation succeeds, startup misconfiguration blocks, known ownership handover/restoration and successor ownership preserved. Restrict real clock probes to approved disposable VM contract. Report authority/ACL/token evidence, actual rights and remaining host/administrator race limits.

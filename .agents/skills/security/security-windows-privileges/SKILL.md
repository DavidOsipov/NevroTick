---
name: "security-windows-privileges"
description: "Implement or review NevroTick Windows service identities, effective tokens, privilege limits, process access, DLL loading and clock ownership enforcement."
---

# Enforce Windows authority at the effect

Read [the security contract](../CONTRACT.md), architecture sections 4, 10–11 and 15, and [native proof requirements](../references/native-proof.md) when qualifying OS behavior.

For service-scoped network restrictions or event-provider authority, read the relevant [Windows host integration](../references/windows-host-integration.md) sections. They distinguish actual token/service and direct/delegated network owners from configured policy, and require bounded product-owned restoration.

1. Identify caller, target securable object, requested rights and protected effect. Preserve own-process engine/broker separation: the network-facing engine lacks clock-setting privilege; only the narrow broker writes the global clock; normal UI is `asInvoker`; installer/helper owns bounded elevated lifecycle operations.
2. Inspect actual SCM account, service SID, required-privilege configuration and effective runtime tokens. A privilege merely disabled in a token can remain activatable. Prove the engine lacks `SeSystemtimePrivilege`, `SeDebugPrivilege`, `SeImpersonatePrivilege` and unnecessary token/process rights; preserve the architecture's documented OS exception. Account names alone prove no isolation.
3. Use service-SID-scoped ACLs and minimum specific rights for process/thread/token/pipe/files/registry/SCM objects. No broad LocalService write access, engine-to-broker injection/token duplication/writable memory or inheritable privileged handles. Keep broker external network restriction explicit; a virtual service identity alone does not remove networking capability.
4. Keep raw Windows calls and unsafe validity/ownership/thread requirements in the reviewed platform adapter. Use owned non-inheritable RAII handles, stable handles for identity-sensitive decisions and captured native errors. Cleanup is explicit for shutdown and cancellation; destructors cannot run after arbitrary process death.
5. Before every clock write, including reset/startup/cancellation/recovery, establish current product ownership generation and absence of a known successor/controller conflict. A saved journal or matching numeric rate is insufficient. On ambiguous/lost ownership stop internal actuation and report Blocked; do not overwrite a potentially foreign global adjustment.
6. For code loading, use protected installation paths and reviewed DLL search policy. Inspect final binaries/imports and actual loads. A source mitigation flag or assumed system DLL search order does not prove emitted/effective protection.

Select real Windows checks with [qa-security-boundaries](../../testing/qa-security-boundaries/SKILL.md): actual service tokens, prohibited rights/effects denied, legitimate broker operation succeeds, startup misconfiguration blocks, known ownership handover/restoration and successor ownership preserved. Restrict real clock probes to the approved disposable VM contract. Return authority/ACL/token evidence, actual rights and remaining host/administrator race limits.

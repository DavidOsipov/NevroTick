# NevroTick: useful Windows 11 integration mechanisms

**Document:** NV-WIN11-RESEARCH-001, revision 1.3<br>
**Date:** 7 November 2026<br>
**Owner:** David Osipov<br>
**Status:** Owner-requested documentation integration; source research and proposed qualification, no native execution.

Start with [the documentation guide](../README.md). The [architecture](../windows-time-architecture.md) owns technical contracts; the [development workstream](development-plan.md#85-windows-11-integration-research-and-qualification) owns phase placement, and [the agent task](../../tasks/windows11-integration.md) owns bounded assignments and execution evidence. The source review was completed on 2–3 November 2026.

**3 November follow-up:** the [second Microsoft Learn MCP pass](windows11-integration-follow-up.md)
adds exact native lifetime/error cases, narrower API alternatives and documented
source discrepancies. Read it alongside this first review before dependent
adapter work. The original inspected baseline and review claims below retain
their original scope; the follow-up was reconciled with later `45ff247` task
adoption and execution holds. Its findings extend the existing W11/ST-W owners,
without selecting optional APIs or qualifying Windows behavior.

**4 November leap-policy amendment:** [NV-LP-001](leap-second-policy.md) now
selects the v1 leap contract, including silent-LI sources, native ownership and
the default process presentation. Its Windows work stays with W11-W02/S0.4/P4;
its immutable dataset follows the existing complete-release channel. The source
review below retains its original baseline and supplies no native qualification.

**7 November service-hardening amendment:** the
[management review](#service-management-hardening--7-november-2026) adds
application admission, explicit administrative exceptions and native lifetime
limits to existing SCM/IPC/recovery requirements. It changes no execution hold,
account topology or current Core priority.

## Decision and scope

The strongest incremental candidates are packet timestamps, concrete service-scoped network restrictions, DNS configuration and policy adapters, power-aware background work, and lifecycle checks that cover Modern Standby and MSI Restart Manager. Several mechanisms improve an existing design rather than add a new product feature. They should enter the relevant planned work when it is reached; they do not displace the current time-client work.

This amendment records the owner-approved research and future work. Its original
survey selected no conditional API, dependency or different service topology.
The later [OD-05 delivery decision](../nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership)
selects complete hardened MSIX preparation under the
[focused plan](windows-msix-store.md), superseding this survey's earlier MSI-first
direction. No application code, system configuration, clock, service, package or
dependency is changed by documentation integration. Existing execution holds
remain. Proposed tests below are future bounded qualifications; W11-W labels
locate work while existing CHECK, DNS-C and ST-T owners retain acceptance authority.

### Baseline inspected

The coordinator refreshed GitHub main and its complete tree at `a6c2401b9a8166515293653f14f637e5543c689e` (tree `7d0558ab295a7859e0d7d9e5b1180bea0fe30443`, 1,249 entries). Local material was a source/documentation snapshot, not a Git checkout. Relevant sources included the [architecture][R1], [hardening plan][R2], [development plan][R3], [whole-computer DNS plan][R4], [DNS upstream lessons][R5], and [protected-storage plan][R6]. “New” below means a useful addition or concretization relative to those inspected documents; it is not an exhaustive absence claim about every repository file.

The accepted direction remains Rust on Windows 11 x64; an own-process network engine with least privilege; a separate privileged clock broker; an unelevated Slint UI selected under [OD-13](../nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit), replacing the inspected Win32 baseline; an administrative lifecycle owner; and one complete hardened MSIX for the owner's Store attempt. Service provisioning and recovery remain unqualified. MSI-specific research below is retained for a later owner-selected alternative, not a prerequisite or assumed MSIX hook. The DNS design remains a future whole-computer answering service with loopback ingress, pinned DoQ-only upstream, DNSSEC, bounded work and privacy-preserving caching. Existing VBS/TPM optional-storage decisions and mandatory clock-continuity requirements remain separate. Most of these subsystems are planned, not shipping capabilities.

Already covered well: clock-domain separation, suspend/resume invalidation, W32Time ownership, restricted service identities, object ACLs, per-role mitigations, protected maintenance state, finite SCM recovery, DNS ownership/restoration, private namespace handling, ETW, and the distinction between isolated CNG roots and full VBS enclaves. This report concentrates on concrete native mechanisms and remaining composition questions.

## Priority map

| Area | Useful increment | Placement in existing work | Recommendation |
| --- | --- | --- | --- |
| Time measurement | Winsock RX/TX timestamps; optional NIC/QPC correlation | Native time/network adapter | Strong bounded candidate; software path first |
| Sleep and lifecycle | Modern Standby suspension, timer semantics, actual service-setting activation | Existing platform and service checks | Clarify contracts before claiming native correctness |
| DNS ownership | Versioned native configuration, exclusive endpoints, reconciled notifications | Future DNS platform slice | Strong implementation candidates |
| Network confinement | Windows Service Hardening service restriction rules | Existing installer/security boundary | Qualify before custom WFP management |
| MSI maintenance | Restart Manager as an additional restart source | Packaging and recovery | Extend the existing startup interlock |
| Adaptive cache | Native power, connection-cost and memory signals; scoped EcoQoS | Later DNS refresh/statistics work | Useful, with fixed bounds retained |
| Cryptographic throughput | CNG SIMD hash batches and reused provider handles | Measured DNSSEC work | Conditional optimization, not a speed claim |
| Diagnosis | Correlated WPR/WPA traces; optional PerfLib V2 counters | Existing diagnostic design | High value for debugging and support |
| Fault discovery | Explicit AppVerifier low-resource profile | Existing native analysis tooling | Candidate extension requiring qualification |
| UI integration | Native UIA notifications; local app notifications | Deferred live UI/accessibility stages | Useful once live state exists |
| Large architectural changes | LPAC, Win32 app isolation, full enclaves, package identity | Separate existing experiments | Hold until a specific benefit justifies them |

## 1. Measure packets closer to the network stack

### Software and hardware packet timestamps

Microsoft documents UDP receive timestamps through `WSARecvMsg` ancillary data and transmit timestamps correlated by an application-supplied ID. Software timestamps use QPC; hardware timestamps use the NIC's counter. TX retrieval can report “not yet available,” and its bounded timestamp buffer can drop new entries when full. Socket configuration alone does not establish that system/interface timestamping is active. [T1] [T2]

**Proposed use.** Add an optional measurement adapter that records the actual timestamp source. First compare software packet timestamps with the existing application-boundary timestamps. This can reveal local scheduling and queueing contributions to measurement error. Preserve a qualified application-level path when the optional facility is unavailable, with an honest source/uncertainty label. A missing TX timestamp must not hang the exchange or be replaced by an invented exact send time.

For hardware timing, check both supported and currently enabled capabilities. Support for PTP packets alone is insufficient for arbitrary NTP UDP traffic. Microsoft exposes a cross-timestamp operation that brackets a NIC clock observation with QPC readings; its documented Windows 11 client floor is build 22000.194. Clock frequency information may be unavailable or inadequate as an exact conversion. [T2] [T3] [T8]

**Proposed qualification.** Bind every timestamp to the exchange, socket generation, actual interface identity, and a current conversion model. The Windows packet-info structures provide interface information; the simple Microsoft timestamp example assumes a single NIC. Therefore a per-NIC model without actual packet attribution is insufficient. Check two adapters with different capabilities, a route switch, stale TX metadata, counter reset, buffer saturation, and ordinary unsupported hardware. RX interface metadata and qualified TX interface selection belong to this adapter. [T1] [T4]

Maintain a separate local measurement record: receiving a more precise TX time later cannot retroactively rewrite bytes already transmitted or authenticated. Neither software nor hardware timestamping removes network asymmetry or proves the remote server's UTC accuracy. System/driver configuration changes, if eventually needed, belong to an explicitly authorized lifecycle operation rather than the unprivileged engine.

### Sleep, timers, and leap presentation

Microsoft's Modern Standby guidance describes substantial session-0 throttling, and says additional services **may** be suspended starting with Windows 11 24H2. This is not a guarantee that every service always suspends, nor a promise that a service gets CPU by a deadline. [T5]

**Design implication.** A programmed clock-rate change cannot be treated as a finite lease merely because a timer intends to cancel it later. Even a system clock that excludes sleep does not measure the broker's execution time. Retain existing residual-rate, cleanup-uncertainty, holdover and generation contracts; test quiescence and recovery under the selected native backend before making guarantees. Optional cache persistence must not block broker cleanup.

`SetWaitableTimerEx` offers tolerable delay for coalescing. On Windows 8 and later, relative timers exclude low-power time; absolute timers respond to system-time changes. [T6] Use timers as wake-up requests and recheck the actual deadline and generation when execution resumes. Background cache work can tolerate coalescing; a single timer policy should not be applied indiscriminately to broker work. High-resolution timers require a measured benefit.

The selected [v1 Windows leap contract](leap-second-policy.md#6-windows-contract-and-limits)
keeps the kernel as the sole explicit-event executor. NevroTick does not schedule
another leap, change global settings/history or parse `w32tm` output as a runtime
protocol. Stopping W32Time does not prove native leap knowledge absent. Preserve
the transactional ordinary-clock takeover, foreign-owner checks and restoration.

Qualify the **default compatibility presentation** in engine and broker: no
`:60` opt-in, with each process's mode recorded and verified through the documented
process APIs. A positive leap stretches the displayed 59th second over two real
seconds in this mode; this is not global leap disablement. [T7] Cover formatting,
logging and imported libraries, and invalidate the mapping if presentation is
unexpected or unverified. Keep FILETIME ticks, civil UTC, NTP coordinates and
monotonic intervals distinct; qualify documented conversions and preserve
sub-millisecond precision outside the guard.

The [event guard](leap-second-policy.md#5-event-guard-holdover-and-reacquisition)
settles transient corrections, excludes ambiguous exchanges and reacquires fresh
ordinary agreement before correcting the measured residual. No synthetic extra
second follows an OS-handled event. Unverified native event knowledge limits
exact event-time UTC; it does not justify an undocumented setter or a second
executor. Unknown future-calendar coverage alone still permits ordinary tracking
outside possible-event guards, with the persistent qualifier defined by NV-LP-001.

## 2. Integrate the DNS service without losing Windows policy

### Native configuration with an ownership record

`GetInterfaceDnsSettings` and `SetInterfaceDnsSettings` are candidates for typed per-interface work. They are versioned; writes select fields through flags, including a separate IPv6 flag. Their function baseline is build 19041, while individual structure versions have their own requirements. The public V3 contract and SDK bindings must not be conflated with an unrestricted future-version contract. [N1] [N2] [N3]

**Proposed use.** The administrative lifecycle owner records interface identity, the exact selected fields, observed origin/presence where supported, original values, product-written values, and the active generation. It only routes clients to loopback after required listeners and upstream readiness pass the existing checks.

**Unresolved proof.** An effective DNS server list is not sufficient to restore DHCP inheritance. Microsoft's reset semantics explicitly distinguish DHCP defaults from static server values. The native Get/Set pages do not establish a complete origin snapshot or a cross-interface transaction. [N4] Test one DHCP and one static interface, both address families, partial failure and an intervening VPN/administrator edit. Restore only changes still owned by the product. Unsupported or ambiguous origin needs an explicit result; do not turn dynamic configuration into static values by guesswork.

### Exclusive loopback endpoints and reconciled network changes

`SO_EXCLUSIVEADDRUSE`, set before binding, is the concrete Windows candidate for exclusive listener ownership. `SO_REUSEADDR` can lead to indeterminate delivery with competing bindings. [N5] Start from deliberately scoped IPv4/IPv6 loopback UDP/TCP listeners. Test port conflicts before enrollment, competing binds while active, restart gaps, and negative LAN probes. An exclusive acquired socket neither authenticates a local caller nor protects the interval while the service is stopped.

`NotifyIpInterfaceChange` and related address/route APIs can trigger reconciliation. Their callback data and cancellation lifetime need care; an initial registration notification is not a network change. [N6] Enqueue bounded/coalesced work, refresh actual state outside the callback, advance the relevant generation, and reject stale publications. These APIs do not promise notification of every DNS-only or NRPT policy edit. Include a separately qualified policy-refresh mechanism and a DNS-only change test with stable addresses.

### Private namespaces, encrypted DNS, WSL and enterprise controls

Windows NRPT, suffix search and interface selection affect DNS routing. [N7] The existing private-name contract should include exact/suffix rules, short names, VPN disconnects and private-server timeouts. A private-name failure must not silently become a public DoQ request. Calling the ordinary Windows resolver from the local DNS service after loopback enrollment can also create a dependency cycle; preserve the planned nonrecursive bootstrap graph.

Inventory the host's encrypted-DNS and discovery policy before enrollment. Native DoH/DoT settings do not implement the project's pinned DoQ path. `DnsQueryRaw` is an interesting policy-aware diagnostic candidate, but the currently inspected page is marked prerelease and lacks a clear supported-client minimum. Likewise, a settings-V4 Rust symbol or an Insider DNR announcement is not sufficient evidence of supported behavior on the selected release. [N8] [N9] [N10] [N15]

WSL DNS tunneling and Docker Desktop deserve distinct compatibility rows: Microsoft documents Windows policy integration for WSL tunneling, while Docker Desktop container queries use a different path. [N11] [N12] Verify synthetic queries through each supported environment. Do not widen a loopback listener to a LAN listener merely to make a VM reach it. “Whole-computer DNS service” remains a product goal with explicit coverage; changing Windows DNS settings cannot establish that every application's independent resolver uses it.

Zero Trust DNS is especially relevant to enterprise coexistence. Current documentation lists Enterprise/Education and an approved DoH/DoT resolver model using Windows DNS Client and WFP. A custom DoQ resolver is not automatically equivalent to that authorization path. [N13] [N14] Detect managed conflicts and report them. Enabling a machine-wide ZTDNS policy would be a separate product/administrator decision; it is not part of this research recommendation.

## 3. Constrain service networking with a Windows-owned mechanism

`INetFwServiceRestriction::RestrictService` and its separate Windows Service Hardening rules collection are a concrete candidate for the existing broker-denial/engine-restriction design. Microsoft's sample applies default service restrictions and adds required exceptions within the WSH collection. Rule application may be delayed. [S1] [S2]

**Proposed use.** The administrative lifecycle owner installs distinct service profiles: the broker has no external network need; the engine receives only its approved network capabilities. Qualify WSH before introducing custom WFP management. Preserve the selected service SID/token profile: do not copy the sample's `serviceSidRestricted = FALSE` into a restricted-engine design.

**Composition boundary.** This is a specific WSH mechanism. Ordinary explicit firewall blocks still take precedence over ordinary allows. [S3] Also observe certificate or DNS operations delegated to other services; restricting one process does not automatically constrain every delegate. Test actual installed identities, IPv4/IPv6, permitted and forbidden destinations, rule activation delay, update/recovery, and managed-policy conflicts before reporting confinement as effective.

Custom WFP remains conditional on a demonstrated WSH gap. Its object ACLs, transaction scope, and filter/session lifetime require their own design. Dynamic-session objects are removed when their session ends. [S11] [S12] [S13] A denial that disappears when its owner crashes can defeat the intended protection. No new kernel driver or machine-wide policy change follows from this recommendation.

## 4. Make installation and service state reflect actual Windows behavior

### Service management hardening — 7 November 2026

**Scope/evidence.** Owner requested maximum supported service hardening; normal
management through NevroTick, administrative-utility exception retained. Three
bounded lanes covered SCM/assets, IPC/application identity and repository
reconciliation; coordinator covered reliability/existing mitigations. Sources:
Microsoft API documentation, MSRC policy and Microsoft engineering explanations.
Product rules/proposed tests are engineering conclusions, not measured protection.
Canonical sections/skills compared at `c8df999abb2309d11cd4484159671a7110c21fa4`;
Scoped source review; no exhaustive audit or native qualification.

Main increment:
[management admission and recovery contract](../windows-time-architecture.md#421-service-management-admission-and-administrative-recovery).
Existing least privilege, mutual IPC, maximum-compatible mitigations and finite
recovery remain required. Add no duplicate security framework, skill or acceptance catalogue. Owner map:

| Layer | Required hardening/reliability outcome | Existing implementation/qualification owner |
| --- | --- | --- |
| Service roles and tokens | Separate own-process engine/broker; engine lacks clock/debug/impersonation authority; broker retains only required privileges and no external-network role. Inspect actual token, owner and restricted/service SIDs before normal work | Architecture §4.1; S0.2; W11-W05; CHECK-1/27 |
| SCM management | Explicit principal/operation/route matrix, exact manager/service rights, fixed targets and deliberate elevated recovery. Normal UI has no direct lifecycle/configuration authority | Architecture §4.2.1; service-adapter access matrix; W11-W05; CHECK-1/2/13/23/27 |
| Application admission | Current OS-authorized actor, admitted installed NevroTick component and current effect guards all pass. Unadmitted same-user processes, copied helpers and unrelated signed packages gain no protected admission merely from user identity or signature validity; admitted-process abuse retains §14.3's residual scope | Architecture §4.2.1/§19.4–19.5; S0.3/W11-W10; CHECK-1/2/15/18 |
| Local IPC | Explicit protected DACL, exact pipe rights, remote rejection on every instance, continuous server ownership, bidirectional identity and explicit identification-level client QoS | Architecture §5.1/§5.4; S0.3/S3.2; CHECK-2/3/25/27 |
| Process and loaded code | Review process/thread/token access and handle inheritance/duplication. Qualify required DEP/ASLR/CFG/CET, dynamic-code, CIG, image-load, extension-point, strict-handle and role-compatible Win32k/child restrictions on final artifacts and activation paths | Architecture §15.1–15.4; hardening §6.3–6.4; CHECK-16/18/20/23/26 |
| Files and configuration | Protect immutable code, DLL search, service registry state, policy/journal and parent replacement authority. Inspect actual final owner/DACL after creation, replacement, repair and update | Architecture §12/§15.2; service-adapter object matrix; CHECK-1/13/15/23 |
| Network confinement | Broker denied external traffic; engine gets only approved direct/delegated capabilities. Record rule activation and enterprise-policy conflicts | This plan §3; W11-W04; CHECK-1/18/23 |
| Work and native lifetimes | Finite caller/global bytes, clients, queues, retries and deadlines; legitimate neighboring work progresses. Cancellation retains native buffers/handles/permits until actual completion | Architecture §5.4/§17.1; CHECK-3/25/27 |
| Startup, stop and recovery | Readiness means usable service, not trusted UTC. One lifecycle owner; short callbacks; real progress; cleanup before final Stopped. Per-service finite restart policy, maintenance interlock and independently usable crash recovery | Architecture §11.2; service-adapter lifecycle/recovery matrices; CHECK-12/13/27 |
| Mandatory clock authority | Current ownership, durable reservations/outcomes and correction limits survive crashes, reconnects and updates. Unknown outcome blocks further effects; restart does not refund authority | Architecture §10–12; P4; CHECK-10/11/12/15/25 |
| Installation and servicing | Complete hardened MSIX, supported pre-start isolation, protected payload and fresh admission; repair/removal preserve an independently invocable recovery owner until obligations settle | MSIX plan §7.5–7.6; W11-W05/W11-W10/P6; CHECK-1/13/18/23/27 |
| Evidence and drift | Record configured, effective and actually challenged state separately. Bounded redacted security decisions and lifecycle outcomes must not invent caller identity or successful cleanup | Architecture §12.3; hardening §9.8–9.11; CHECK-14/17/27 |

**SCM authorization is not application authentication.** Access checks compare
effective token/requested rights against a descriptor. Service
descriptors cannot select EXE names. Default local-authenticated-user grants
include custom controls; service `GENERIC_EXECUTE` also contains start/stop,
pause/continue and custom-control rights. The HandlerEx interface provides
control/event data and service-owned context, not an authenticated original
calling application. Therefore protected product commands belong behind the
qualified IPC method boundary. Retain only separately justified query rights
for observation/server verification; hiding enumeration is not a security goal.
Use actual installed descriptors, not historical default-permission prose, as
qualification evidence. [H1] [H2] [H3]

**Package identity is useful evidence with a bounded claim.** Windows writes packaged-process
identity through controlled token/deployment paths; IPC package strings supply
no equivalent evidence. NevroTick must bind the actual
peer to its approved installed package/publisher/component/release and protected
content. Full-trust MSIX does not automatically create an AppContainer; inspect
the actual activation and token, including elevated helper cases. Neither package
identity nor a signature proves untampered live memory, safe loaded code or an
authorized verb. CIG and Trusted Launch retain their separate existing contracts.
[H4] [H5]

AppContainer/profile capability SIDs do not alone prove publisher provenance or
exclusive use by the intended application; launch authority, injection and handle
delegation still need a threat model. The documented HSA custom-capability/SCCD
route requires Microsoft reservation/approval and binds authorized packages and
certificate-signature hashes. It is not a generic private-capability shortcut
for this utility. Neither that route nor a new AppContainer/driver/topology is
selected. PPL remains excluded: Windows protection levels are reserved and the
third-party antimalware route has special eligibility/signing requirements.
[X2] [X9] [H6]

**Existing authority survives superficial repair.** The Windows handle model
checks access on acquisition; later DACL tightening does not revoke existing
handles. Require a pre-existing-handle case across SCM/IPC repair; no universal
revocation recipe follows. Establish
protection before exposure and recheck current protected-operation policy.
Likewise, obtaining a stable process handle after PID lookup does not prove the
earlier acquisition interval was race-free. Review actual peer/token/process
association, process-object rights, inherited/duplicated handles and authorized
proxying; reject inconsistent identity. [H7] [H11] [H21]

**Final objects and supported configuration paths matter.** Default file ACLs
are assigned on creation, not merely on rename/move. Moving an updater artifact into a protected
directory proves no secure owner/DACL. Inspect
final objects and parent replacement authority through actual supported package
servicing. SCM configuration functions, not direct Services-registry writes,
own service database mutation. Use fully qualified/correctly quoted executable paths and reviewed protected
DLL dependency search paths; safe-search
ordering alone leaves writable locations possible. [H8] [H9] [H20] [H22]

**Local IPC and administrative recovery are separate policies.** Reject remote clients on every pipe
instance; exact data rights avoid the
`FILE_GENERIC_WRITE`/instance-creation alias. Limit client SQOS explicitly so a
fake endpoint cannot gain unnecessary impersonation authority. Serialize
read/caller capture because impersonation uses the last message read; reject
failed impersonation and stop process execution after failed reversion. Existing IPC requirements; no new transport selected. [H12] [H13] [H14] [H16] [H23]

SCM can use remote RPC/TCP and named-pipe transports independently of product
IPC. Record supported/denied administrative remote use and observed host
policy; do not change host-wide firewall/RPC/UAC settings merely to make
NevroTick management succeed. Retain deliberate elevated administrative repair
when UI or service is unavailable. Ordinary-service ACLs cannot promise defense
against malicious local administrator/kernel control; this scope does not erase
the distinct Windows VBS/protected-feature boundaries. [H10] [H24]

**Reliability is part of the protection.** `CancelIoEx` requests cancellation;
native completion still owns `OVERLAPPED` and its buffers. `SetServiceStatus`
requires meaningful progress and cleanup before the single final Stopped report.
SCM repeats the last configured failure action; a bounded restart policy needs
an explicit terminal `SC_ACTION_NONE` and justified reset/backoff semantics. Queued restart
cannot be canceled merely by clearing the failure-action array. The existing
lifecycle owner must explicitly disable the affected service and retain the
protected maintenance-startup interlock; the interlock supplements disabling
and does not cancel the queued action. The non-crash
flag has separate documented activation timing. Keep process-specific failure,
network-unavailable/degraded operation, orderly stop and crash recovery distinct;
no restart loop or false Running may conceal failed admission. Reuse existing
deadlines and decision owners rather than copy sample numbers. [H15] [H17] [H18] [H19] [S8]

**Next bounded work.** Freeze the architecture §4.2.1 route/actor/component/verb
matrix and its allowed/denied neighbors under S0.2/S0.3 and W11-W05/W11-W10.
Specify the package-aware peer verifier and independent recovery entry point;
both remain unqualified. Existing hardening §9.9/§9.11 owns same-principal/wrong-
application, pre-existing-handle, legitimate elevated-admin and unavailable-UI
controls. Native work needs existing admission/disposable fixture; source tests
cannot qualify Windows enforcement. Current Core priority, broad
Windows stop, fuzz/storage holds, UI-M0 deferrals and post-V1 formal deferral stay.
No production limits, account mapping, dependency or new execution permission
is selected by this research.

### Restart Manager is another restart source

Windows Installer integrates with Restart Manager, which can stop and restart services during updates. `MSIDISABLERMRESTART` controls this behavior only when Restart Manager is in use. [S4] [S5]

**Proposed refinement.** Route RM-triggered starts through the same protected maintenance/generation admission as boot, manual start and SCM recovery. A public MSI property must not authorize clearing the maintenance state. In a future disposable-VM fixture, hold an update before commit, attempt each restart route, then test commit and rollback with RM enabled and unavailable. Preserve honest reboot-required results. This extends the existing complete-MSI lifecycle design rather than creating an updater.

### Configuration readback is not effective-state proof

Microsoft documents different activation points for different service settings. Required-privilege changes apply at the next service start; the service-SID and non-crash failure-actions flag pages state the next system start. [S6] [S7] [S8] Qualify first installation separately instead of inferring that every fresh installation universally requires a reboot.

The existing service record should distinguish configured value, documented activation point, observed token/policy and pending restart/reboot. A matching registry/configuration value is not enough to hand over clock authority. A restricted service SID also does not replace explicit read-confidentiality ACLs.

### Service privilege source review — 9 November 2026

David requested a concrete privilege choice for each existing service. The
selected sets and admission/lifetime rules now live in
[architecture §4.1.1](../windows-time-architecture.md#411-service-privilege-profiles);
this section records their API basis and compatibility limits. It is a design
decision supported by source review, not an observed Windows/MSIX token.
[W11-W05's continuation](../../tasks/windows11-integration.md#438-service-privilege-profile-handoff--9-november-2026)
retains S0.2/S0.3/P3/P4/P6 ownership and all current execution holds.

| Actual or conditionally selected operation | Primary source and privilege conclusion |
| --- | --- |
| Configure the runtime privilege set | [`SERVICE_REQUIRED_PRIVILEGES_INFOW`][S6] filters at the next service start, retains `SeChangeNotifyPrivilege` and unions requirements in shared-process hosting. The explicit nonempty role lists avoid depending on unspecified empty-list interpretations. This is not an account-right grant or current-token proof. |
| Read UTC, monotonic counters and current adjustment | [`GetSystemTimePreciseAsFileTime`](https://learn.microsoft.com/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimepreciseasfiletime), [`QueryPerformanceCounter`](https://learn.microsoft.com/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter) and [`GetSystemTimeAdjustmentPrecise`](https://learn.microsoft.com/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimeadjustmentprecise) document no extra `Se*` requirement for the reads. A combined getter/setter sample does not make clock-setting authority necessary for measurement. |
| Step system UTC | [`SetSystemTime`](https://learn.microsoft.com/windows/win32/api/sysinfoapi/nf-sysinfoapi-setsystemtime) requires a present `SeSystemtimePrivilege` and enables/disables it internally. The engine must lack it, including its disabled form. Only the broker's guarded step path may invoke the setter. |
| Set/restore the selected precise adjustment | [`SetSystemTimeAdjustmentPrecise`](https://learn.microsoft.com/windows/win32/api/sysinfoapi/nf-sysinfoapi-setsystemtimeadjustmentprecise) uses the same clock privilege; its caller enables it before and disables it after the call. Sample code is not the product's guard, cleanup or error oracle. |
| Adjust the broker's privilege state | [`AdjustTokenPrivileges`](https://learn.microsoft.com/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges) cannot add absent privileges. Nonzero return with `ERROR_NOT_ALL_ASSIGNED` is not complete success; removal is irreversible. `TOKEN_ADJUST_PRIVILEGES`/`TOKEN_QUERY` are token-object access rights, not extra `Se*` privileges. |
| Identify a named-pipe client | [`ImpersonateNamedPipeClient`][H13] permits identification-level capture without `SeImpersonatePrivilege`. [`OpenThreadToken`](https://learn.microsoft.com/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthreadtoken) with `OpenAsSelf=TRUE` uses the process context for its access check; it does not bypass the token DACL. Require the explicit identification QoS and actual token level in architecture §5.4.2. |
| Authorize the captured client | [`AccessCheck`](https://learn.microsoft.com/windows/win32/api/securitybaseapi/nf-securitybaseapi-accesscheck) and [`CheckTokenMembership`](https://learn.microsoft.com/windows/win32/api/securitybaseapi/nf-securitybaseapi-checktokenmembership) accept an explicit impersonation token with `TOKEN_QUERY`. Revert before application authorization/dispatch; check API outcomes and authorization outcome separately. No ambient/null-token fallback or effects under the client token. |
| High-resolution waitable timer | [`CreateWaitableTimerExW`](https://learn.microsoft.com/windows/win32/api/synchapi/nf-synchapi-createwaitabletimerexw) documents object rights and the high-resolution flag, not a requirement for clock-setting privilege. Qualify the selected scheduling/deadline behavior; do not add realtime process priority or `SeIncreaseBasePriorityPrivilege` as an accuracy shortcut. |
| Service status/control and ordinary owned files/IPC | [SCM rights][H1] and [file/object rights][H8] are separate from token privileges. `SERVICE_CHANGE_CONFIG`, `WRITE_DAC`, `WRITE_OWNER` and `TOKEN_QUERY` are not entries in `RequiredPrivileges`. Runtime status ownership grants no installation or sibling-service reconfiguration authority. |
| Change a default owner or provision an audit descriptor | [Token-object rights](https://learn.microsoft.com/windows/win32/secauthz/access-rights-for-access-token-objects) and [`TOKEN_OWNER` eligibility](https://learn.microsoft.com/windows-hardware/drivers/ddi/ntifs/ns-ntifs-_token_owner) govern default-owner changes. They differ from changing an existing object's descriptor. [`SetSecurityInfo`](https://learn.microsoft.com/windows/win32/api/aclapi/nf-aclapi-setsecurityinfo) requires `SeSecurityPrivilege` for SACL access; any required provisioning belongs to the authorized lifecycle owner, not the runtime service allowlist. |
| Lock a bounded secret buffer | [`VirtualLock`](https://learn.microsoft.com/windows/win32/api/memoryapi/nf-memoryapi-virtuallock) does not establish a need for `SeLockMemoryPrivilege`. Its working-set bound matters: [`SetProcessWorkingSetSize`](https://learn.microsoft.com/windows/win32/api/memoryapi/nf-memoryapi-setprocessworkingsetsize) requires `SeIncreaseWorkingSetPrivilege` when increasing either limit. Qualify the required finite lock budget and checked failure under the selected token; do not silently expand privileges or weaken required secret handling. |
| NTS/TLS/CNG and protected storage | No reached requirement establishes broad impersonation, TCB, debug or clock authority for the engine. For the conditional [Schannel credential API](https://learn.microsoft.com/windows/win32/secauthn/acquirecredentialshandle--schannel), ordinary own-context credentials differ from acquiring another logon session's credentials; Schannel requires `pvLogonID=NULL`. [`BCryptGenRandom`](https://learn.microsoft.com/windows/win32/api/bcrypt/nf-bcrypt-bcryptgenrandom), selected key/provider ACLs and conditional [DPAPI](https://learn.microsoft.com/windows/win32/api/dpapi/nf-dpapi-cryptprotectdata) still need exact-token/native qualification. This selects neither Schannel nor a DPAPI fallback and proves no complete provider/DNS/network closure. |

**SID and privilege restrictions are independent.** Service `Restricted` uses
a write-restricted token, as explained by [Microsoft's service-hardening
guidance](https://techcommunity.microsoft.com/blog/askperf/ws2008-windows-service-hardening/372702)
and the current [SID structure][S7]. Its restricting list also contains logon,
Everyone and Write Restricted SIDs; the flag does not mean only an object with
the service's own SID can be written. It does not replace confidentiality ACLs,
object/default-owner proof, privilege filtering or
[service network restriction](https://learn.microsoft.com/windows/win32/api/netfw/nf-netfw-inetfwservicerestriction-restrictservice).
`SeChangeNotifyPrivilege`'s documented [traverse behavior](https://learn.microsoft.com/windows/win32/secauthz/privilege-constants)
does not grant general target-object read/write access.

**Conditional process-creation conflict.** The retained restricted-child
alternative in [the MSIX research](windows-msix-store.md#restricted-child-concrete-benefit-and-restart-tradeoff)
uses `CreateProcessAsUserW`, which typically requires `SeIncreaseQuotaPrivilege`.
Its restricted-copy exception concerns `SeAssignPrimaryTokenPrivilege`, not
every prerequisite. Neither extra privilege is in the selected broker profile.
That unselected topology is not an admitted workaround for SCM provisioning.
An additional required privilege or incompatible provider is a named gap for the
existing owner, never an automatic privilege fallback.

**Observed source scope at `13ce18d`:** `crates/windows-platform` supplies portable
contracts; `tools/windows-service-fixture` supplies inert native laboratory
adapters. The native privilege capture resolves three fixed forbidden-engine
LUIDs, and the portable snapshot keeps those facts. Broker observation is
descriptive; a full per-role allowlist and SID/ACL/owner qualification remain
open. Do not relabel those narrower historical checks as implementation of
architecture §4.1.1. Extend their existing owner rather than add a second verifier.

**9 November implementation follow-up:** the existing owner now retains the
complete bounded privilege set and enforces both exact role profiles before
laboratory worker preparation. The
[current task](../../tasks/windows11-integration.md#438-service-privilege-profile-handoff--9-november-2026)
records platform measurements, caller qualification and remaining native evidence.
This extends the source snapshot above; it does not retroactively broaden its
receipts or qualify native token, SID/ACL, provisioning or installed behavior.

### Child-process policy and shutdown need composition checks

Microsoft warns that child-process restriction can be bypassed if the process can acquire another process handle with rights such as `PROCESS_CREATE_PROCESS` or `PROCESS_VM_WRITE`. [S9] Extend the existing process/object access matrix with harmless requests for those rights under the actual installed identities. A denied ordinary child launch and a queried mitigation bit are insufficient by themselves. The existing unresolved SCM creation-policy route remains unresolved; this report does not authorize a custom launcher workaround.

Current Windows preshutdown defaults are much shorter than old examples: 10 seconds from build 15063. [S10] Select a justified explicit budget if using preshutdown. Broker cancellation and required journal work must remain independent of optional cache writes. Orderly-shutdown support does not guarantee cleanup after process death or power loss.

## 5. Make adaptive caching cooperate with the operating system

The DNS plan already includes battery/metered-network policy. The following APIs provide concrete inputs rather than a new prediction model:

| Native input | Documented meaning | Proposed application policy |
| --- | --- | --- |
| `PowerRegisterForEffectivePowerModeNotifications` | Effective system power mode, with versioned notification support [P1] | Reduce optional heatmap analysis and speculative refresh in constrained modes; process callbacks through a bounded owner |
| `INetworkCostManager::GetCost` | Cost information for an indicated destination/connection; query can be pending [P2] | Defer discretionary traffic on metered/roaming/unknown-cost paths according to explicit policy; do not equate “unknown” with free |
| `CreateMemoryResourceNotification` | System-wide low/high available physical-memory conditions [P3] | Trim cold optional cache entries and pause prefetch under pressure; preserve required state and hard caps |
| Per-thread EcoQoS | OS energy-efficiency treatment for eligible work [P4] | Apply only to suitable background workers after measurement; keep latency-critical time/foreground work separately classified |

**Design inference.** The cache controller can combine user demand, heat, TTL/validation state and a small OS-resource state. For example, on an unconstrained connection it refreshes an eligible hot entry before demand; on metered battery operation it reduces speculation; under memory pressure it discards cold optional entries. These are proposed product decisions, not behavior supplied automatically by the APIs.

Resource hints are neither security authority nor proof of reachability. Memory notifications are level-signaled and can remain signaled; coalesce handling and use hysteresis rather than a wake-up loop that spins continuously. They do not enforce a process budget or guarantee a warning before allocation failure. Maintain fixed entry/byte/work bounds and ordinary failure handling.

EcoQoS should not be applied process-wide to the clock engine merely because it also performs optional work. The current single-thread runtime prototype makes work placement especially consequential. A background task label is useful only if the actual execution thread receives the intended policy. Recheck foreground latency and energy use together.

## 6. Use native cryptographic acceleration for a measured workload

`BCryptCreateMultiHash` supports SIMD batching for SHA-256, SHA-384 and SHA-512. Microsoft explicitly describes a throughput/individual-latency tradeoff. `BCryptProcessMultiOperations` currently defines hash operations; it does not create a multicore DNSSEC validator or accelerate all signature/TLS/NTS/AEAD work. [C1] [C2]

**Proposed candidate.** If profiling finds a ready queue of independent eligible hashes, compare the approved existing backend with a bounded CNG batch using identical inputs. Do not delay an isolated foreground query merely to fill a batch. Measure tail latency, throughput, CPU time and energy on supported CPUs; test small loads and adversarial work as well as warm batches.

Parallel verification is a separate scheduler decision: retain bounded workers, fairness, cancellation and dependency-aware admission. SIMD within a worker and work across cores can complement one another, but neither is a result until measured. This mechanism does not close the DNS plan's outstanding crypto/backend decision.

Opening CNG algorithm providers can be expensive; Microsoft recommends cached handles or eligible pseudo-handles. [C3] Preserve the project's provider/algorithm policy and ownership requirements when applying that advice. A provider change, cached handle or faster hash must not alter validation semantics, secretly add a fallback, or bypass resource accounting.

## 7. Diagnose actual Windows behavior and inject native failures

### ETW, WPR/WPA and PerfLib V2

ETW is already in the project design. WPR/WPA supplies a useful concrete investigation workflow for correlating application events with OS activity. [D1] Proposed event boundaries are packet arrival, queue admission, parsing/verification, sample selection, broker admission and native acknowledgement. Correlate with monotonic IDs/times because the wall clock may be wrong or deliberately changed. Use bounded, typed, redacted fields; no DNS labels, cookies or key material by default.

A focused trace can distinguish packet/network delay from time waiting for a worker, signature work or a native operation. Preserve the existing finite diagnostic-session owner. ETW can lose events, and its session statistics expose loss counts. [D2] An incomplete trace cannot prove that an absent event never occurred; record loss when traces support an experiment. ETW is not the durable journal for protected state.

PerfLib V2 is an optional addition for Windows operator tooling. It supplies counters from the provider's own process, unlike the deprecated V1 DLL model. Microsoft positions counters for coarse administrative monitoring, not profiling or collection more than once per second. [D3] [D4] Suitable proposed counters include eligible sources, age of authenticated synchronization, uncertainty, queue occupancy, cache hit rate and resource-denial counts. Keep them aggregate and semantically defined; a last-sync timestamp without trust/quality context is misleading. No extra resident telemetry service is necessary.

### Application Verifier low-resource simulation

The inspected `policy/windows-memory.json` admits Application Verifier Basics plus full PageHeap. It does not admit an arbitrary Low Resource Simulation profile. Microsoft's tooling supports resource-failure injection; its `/faults` shortcut defaults to a limited set of allocation faults rather than every possible native resource failure. [D5]

**Proposed extension.** Select one named failure question, define the exact profile, then qualify healthy and deliberate-failure cases in the existing isolated native tooling workflow. Valuable subjects include allocation/handle failures during startup, callback registration, CNG use and cleanup. Verify graceful refusal, truthful service status, no half-applied protected effect and cleanup after disabling the profile. This supplements existing model/fuzz/fault testing; it is not evidence that current tests are absent or that this profile has run.

## 8. Present live state through native UI facilities

`UiaRaiseNotificationEvent` supports accessible Win32 state announcements. A newer processing choice, `NotificationProcessing_ImportantCurrentThenMostRecent`, is documented from build 26100; the base notification API is older. [U1] [U2] This can help avoid repeated interruptions while retaining a meaningful latest status, but normal ticking statistics should not be announced as important. Use a compatible older processing mode where required and test actual Narrator behavior in the deferred accessibility phase.

Local app notifications through Windows App SDK also support unpackaged desktop apps. Elevated app notification scenarios are not supported; registration can involve the SDK Singleton package. [U3] [U4] [U5] An unelevated UI is therefore the natural presentation owner. Evaluate deployment/dependency cost before adding the SDK solely for notifications. Local notifications do not by themselves justify package identity.

Candidate messages include unavailable protected storage, optional-cache mode, prolonged time-source loss and a managed-DNS conflict. A notification reflects a current observed state and reason. Its action must revalidate the current service generation and authorization; clicking it is not permission for a privileged policy change. These are future live-UI integrations, not additions to the current fixture-only UI milestone.

## 9. Conditional mechanisms worth understanding

| Mechanism | Potential value | Reason to hold or limit it |
| --- | --- | --- |
| Job Objects | OS-level process resource ceilings [X1] | CPU limits can affect timing; memory limits can cause allocation failures. Killing engine and broker together could prevent required rate cleanup. Adopt only for a concrete role with safe ownership/lifetime semantics. |
| LPAC/AppContainer worker | Additional isolation for a selected untrusted-work component [X2] | A separate worker needs explicit capability, IPC, lifecycle and any required resource-brokering design. The standalone creation API is not proof of a compatible SCM service launch. |
| Win32 app isolation | Packaged Win32 sandbox path [X3] | Current dedicated documentation marks it preview and lists Windows 11 24H2/build 26100 plus packaging prerequisites. Do not treat it as an already supported production service route. |
| Stricter CET/EHCONT settings | More precise control-flow and module-admission enforcement [X4] | Existing ACG/CFG/CET work remains the owner. Qualify the actual Rust/native dependency closure, selected strict mode and metadata; headers/flags alone do not prove effective enforcement. |
| Full VBS enclaves | Execute selected code within a stronger isolation boundary [X5] | Distinct from VBS-isolated CNG roots. Current runtime requirements include build 26100.2314+ and VBS/HVCI; development and publication separately require supported tooling and special signing. Cost/compatibility must justify a new boundary; encrypted bulk cache need not imply enclave execution. |
| Package identity with external location | Access selected identity-dependent facilities while the existing installer owns files [X6] | Does not automatically sandbox or confer package integrity on external files. Preserve the complete MSI. Adopt only for a named API/UX benefit. |
| WinGet distribution | Publish the same MSI through an additional user/admin channel [X7] | No need for a second updater. Release signatures, hash provenance and ordinary installer lifecycle still apply. Useful after packaging/release gates exist. |
| W32Time provider DLL | Alternative integration with Windows' existing time-service control architecture [X8] | Changes the controller/ownership design and is not a small optimization of the selected broker. Consider only for a distinct future product goal. |
| Protected Process Light | Strong process protection in eligible scenarios [X9] | Windows levels are reserved and the antimalware route has special eligibility/signing requirements. Ordinary utility Authenticode signing is not sufficient. Exclude as a general NevroTick option. |
| SCM service triggers | On-demand activation for a genuinely optional component [X10] | An address-arrival trigger is not Internet readiness; named-pipe/RPC endpoint triggers are not first-UDP/53 activation. They do not justify stopping required time/DNS work. |

The later [MSIX/Store plan](windows-msix-store.md) now owns the selected complete
hardened MSIX target within W11-W10. It preserves the catalog-trust/signing
benefits alongside restricted service admission, supported least-privilege
provisioning and recovery-before-removal gaps. Use
[architecture §19.4](../windows-time-architecture.md#194-future-complete-msix-gate)
and the [current task amendment](../../tasks/windows11-integration.md#48-owner-selected-hardened-msix--3-november-2026).
Source/design preparation does not require prior Store approval; actual Store
outcome and final-artifact qualification remain distinct later stages. No split
product or broader service account follows from that choice. The external-location
identity row above applies only to a later retained-MSI alternative; the selected
MSIX profile excludes mutable/external executable content.

## 10. How an implementation agent should use this research

Use the existing phase, CHECK, DNS-C and storage-task owners. Select one relevant mechanism when its product slice is reached, then record:

1. The useful property being added and the existing behavior it must preserve.
2. Official API contract, supported build/edition, runtime capability and actual service/process owner.
3. Configuration required, its activation point, and how effective state will be observed.
4. A bounded healthy case and a failure/counterexample case; use synthetic data and dummy clock effects where sufficient.
5. Unsupported, conflict and cleanup outcomes; distinguish required behavior from optional enhancement.
6. Evidence from the final installed path and dependency closure before claiming native support.

For the next relevant platform work, the most consequential questions are whether service restrictions and startup admission are effective, whether sleep can outlive a planned clock cleanup, and whether packet timestamp provenance is preserved. For the later DNS work, concentrate on reversible ownership, private namespaces, actual endpoint coverage and bounded background resource use. Add the diagnostics needed to answer those questions, then evaluate optimization candidates using those measurements.

No speedup, tighter UTC error, exploit resistance percentage or reduced bug probability is claimed by this research. An independent source/fit review checked the three research lanes and the consolidated report. Its requested provenance, lifecycle, performance and source-mapping clarifications are incorporated; no central recommendation required withdrawal. The [agent task](../../tasks/windows11-integration.md) retains the repository-integration review and scoped documentation checks. The Windows qualification proposals remain unexecuted.

## Sources

Links below are primary Microsoft documentation, except the pinned project baseline. Recommendations and test designs are this report's engineering inferences. Build floors are API-specific; an SDK symbol or broad “Windows 11” label does not establish complete product compatibility.

[R1]: https://github.com/DavidOsipov/NevroTick/blob/a6c2401b9a8166515293653f14f637e5543c689e/docs/windows-time-architecture.md
[R2]: https://github.com/DavidOsipov/NevroTick/blob/a6c2401b9a8166515293653f14f637e5543c689e/docs/windows-time-development-hardening.md
[R3]: https://github.com/DavidOsipov/NevroTick/blob/a6c2401b9a8166515293653f14f637e5543c689e/docs/plans/development-plan.md
[R4]: https://github.com/DavidOsipov/NevroTick/blob/a6c2401b9a8166515293653f14f637e5543c689e/docs/plans/system-dns-cache-design.md
[R5]: https://github.com/DavidOsipov/NevroTick/blob/a6c2401b9a8166515293653f14f637e5543c689e/docs/plans/dns-upstream-lessons.md
[R6]: https://github.com/DavidOsipov/NevroTick/blob/a6c2401b9a8166515293653f14f637e5543c689e/docs/plans/windows-protected-storage.md
[T1]: https://learn.microsoft.com/en-us/windows/win32/winsock/winsock-timestamping
[T2]: https://learn.microsoft.com/en-us/windows/win32/iphlp/packet-timestamping
[T3]: https://learn.microsoft.com/en-us/windows/win32/api/iphlpapi/nf-iphlpapi-captureinterfacehardwarecrosstimestamp
[T4]: https://learn.microsoft.com/en-us/windows/win32/api/ws2ipdef/ns-ws2ipdef-in_pktinfo
[T5]: https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/prepare-software-for-modern-standby
[T6]: https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-setwaitabletimerex
[T7]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_leap_second_info
[T8]: https://learn.microsoft.com/en-us/windows/win32/api/iphlpapi/ns-iphlpapi-interface_timestamp_capabilities
[N1]: https://learn.microsoft.com/en-us/windows/win32/api/netioapi/nf-netioapi-getinterfacednssettings
[N2]: https://learn.microsoft.com/en-us/windows/win32/api/netioapi/nf-netioapi-setinterfacednssettings
[N3]: https://learn.microsoft.com/en-us/windows/win32/api/netioapi/ns-netioapi-dns_interface_settings3
[N4]: https://learn.microsoft.com/en-us/powershell/module/dnsclient/set-dnsclientserveraddress?view=windowsserver2025-ps
[N5]: https://learn.microsoft.com/en-us/windows/win32/winsock/using-so-reuseaddr-and-so-exclusiveaddruse
[N6]: https://learn.microsoft.com/en-us/windows/win32/api/netioapi/nf-netioapi-notifyipinterfacechange
[N7]: https://learn.microsoft.com/en-us/windows/security/operating-system-security/network-security/vpn/vpn-name-resolution
[N8]: https://learn.microsoft.com/en-us/windows/win32/api/windns/nf-windns-dnsqueryraw
[N9]: https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/netsh-dnsclient
[N10]: https://blogs.windows.com/windows-insider/2023/10/25/announcing-windows-11-insider-preview-build-25982-canary-channel/
[N11]: https://learn.microsoft.com/en-us/windows/wsl/networking
[N12]: https://learn.microsoft.com/en-us/windows/wsl/troubleshooting#networking-considerations-with-dns-tunneling
[N13]: https://learn.microsoft.com/en-us/windows/security/operating-system-security/network-security/zero-trust-dns/
[N14]: https://learn.microsoft.com/en-us/windows/security/operating-system-security/network-security/zero-trust-dns/deploying-zero-trust-dns
[N15]: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/NetworkManagement/IpHelper/struct.DNS_INTERFACE_SETTINGS4.html
[S1]: https://learn.microsoft.com/en-us/windows/win32/api/netfw/nf-netfw-inetfwservicerestriction-restrictservice
[S2]: https://learn.microsoft.com/en-us/previous-versions/windows/desktop/ics/c-restricting-service
[S3]: https://learn.microsoft.com/en-us/windows/security/operating-system-security/network-security/windows-firewall/rules
[S4]: https://learn.microsoft.com/en-us/windows/win32/rstmgr/restart-manager-portal
[S5]: https://learn.microsoft.com/en-us/windows/win32/msi/msidisablermrestart
[S6]: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_required_privileges_infow
[S7]: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_sid_info
[S8]: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_failure_actions_flag
[S9]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute
[S10]: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_preshutdown_info
[S11]: https://learn.microsoft.com/en-us/windows/win32/fwp/access-control
[S12]: https://learn.microsoft.com/en-us/windows/win32/api/fwpmtypes/ns-fwpmtypes-fwpm_session0
[S13]: https://learn.microsoft.com/en-us/windows/win32/api/fwpmu/nf-fwpmu-fwpmtransactionbegin0
[P1]: https://learn.microsoft.com/en-us/windows/win32/api/powersetting/nf-powersetting-powerregisterforeffectivepowermodenotifications
[P2]: https://learn.microsoft.com/en-us/windows/win32/api/netlistmgr/nf-netlistmgr-inetworkcostmanager-getcost
[P3]: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-creatememoryresourcenotification
[P4]: https://learn.microsoft.com/en-us/windows/win32/procthread/quality-of-service
[C1]: https://learn.microsoft.com/en-us/windows/win32/api/bcrypt/nf-bcrypt-bcryptcreatemultihash
[C2]: https://learn.microsoft.com/en-us/windows/win32/api/bcrypt/nf-bcrypt-bcryptprocessmultioperations
[C3]: https://learn.microsoft.com/en-us/windows/win32/api/bcrypt/nf-bcrypt-bcryptopenalgorithmprovider
[D1]: https://learn.microsoft.com/en-us/windows/win32/tracelogging/tracelogging-record-and-display-tracelogging-events
[D2]: https://learn.microsoft.com/en-us/windows/win32/api/evntrace/ns-evntrace-event_trace_properties
[D3]: https://learn.microsoft.com/en-us/windows/win32/perfctrs/about-performance-counters
[D4]: https://learn.microsoft.com/en-us/windows/win32/perfctrs/providing-counter-data-using-version-2-0
[D5]: https://learn.microsoft.com/en-us/windows-hardware/drivers/devtest/application-verifier-testing-applications
[U1]: https://learn.microsoft.com/en-us/windows/win32/api/uiautomationcoreapi/nf-uiautomationcoreapi-uiaraisenotificationevent
[U2]: https://learn.microsoft.com/en-us/windows/win32/api/uiautomationcore/ne-uiautomationcore-notificationprocessing
[U3]: https://learn.microsoft.com/en-us/windows/apps/develop/notifications/
[U4]: https://learn.microsoft.com/en-us/windows/apps/develop/notifications/app-notifications/
[U5]: https://learn.microsoft.com/windows/windows-app-sdk/api/winrt/microsoft.windows.appnotifications.appnotificationmanager.register
[X1]: https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects
[X2]: https://learn.microsoft.com/en-us/windows/win32/secauthz/implementing-an-appcontainer
[X3]: https://learn.microsoft.com/en-us/windows/win32/secauthz/app-isolation-overview
[X4]: https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_user_shadow_stack_policy
[X5]: https://learn.microsoft.com/en-us/windows/win32/trusted-execution/vbs-enclaves
[X6]: https://learn.microsoft.com/en-us/windows/apps/desktop/modernize/grant-identity-to-nonpackaged-apps-overview
[X7]: https://learn.microsoft.com/en-us/windows/package-manager/package/manifest
[X8]: https://learn.microsoft.com/en-us/windows/win32/sysinfo/creating-a-time-provider
[X9]: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_launch_protected_info
[X10]: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_trigger
[H1]: https://learn.microsoft.com/en-us/windows/win32/services/service-security-and-access-rights
[H2]: https://learn.microsoft.com/en-us/windows/win32/secauthz/how-dacls-control-access-to-an-object
[H3]: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nc-winsvc-lphandler_function_ex
[H4]: https://devblogs.microsoft.com/insidemsix/msix-chain-of-trust/
[H5]: https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/app-capability-declarations
[H6]: https://learn.microsoft.com/en-us/windows-hardware/drivers/devapps/hardware-support-app--hsa--steps-for-driver-developers
[H7]: https://devblogs.microsoft.com/oldnewthing/20200320-00/?p=103579
[H8]: https://learn.microsoft.com/en-us/windows/win32/fileio/file-security-and-access-rights
[H9]: https://learn.microsoft.com/en-us/windows/win32/services/service-configuration-programs
[H10]: https://learn.microsoft.com/en-us/windows/win32/services/services-and-rpc-tcp
[H11]: https://learn.microsoft.com/en-us/windows/win32/procthread/process-security-and-access-rights
[H12]: https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew
[H13]: https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-impersonatenamedpipeclient
[H14]: https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-reverttoself
[H15]: https://learn.microsoft.com/en-us/windows/win32/api/ioapiset/nf-ioapiset-cancelioex
[H16]: https://learn.microsoft.com/en-us/windows/win32/ipc/named-pipe-security-and-access-rights
[H17]: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-setservicestatus
[H18]: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_failure_actionsw
[H19]: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-changeserviceconfig2w
[H20]: https://learn.microsoft.com/en-us/windows/win32/dlls/dynamic-link-library-security
[H21]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_information
[H22]: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-createservicew
[H23]: https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew
[H24]: https://www.microsoft.com/en-us/msrc/windows-security-servicing-criteria


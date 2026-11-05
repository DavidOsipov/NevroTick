# NevroTick: useful Windows 11 integration mechanisms

**Document:** NV-WIN11-RESEARCH-001, revision 1.2<br>
**Date:** 4 November 2026<br>
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

The accepted direction remains Rust on Windows 11 x64; an own-process network engine with least privilege; a separate privileged clock broker; an unelevated Slint UI selected under [OD-11](../nevrotick-owner-decisions.md#15-od-11--replace-the-win32-ui-with-slint), replacing the inspected Win32 baseline; an administrative lifecycle owner; and one complete hardened MSIX for the owner's Store attempt. Service provisioning and recovery remain unqualified. MSI-specific research below is retained for a later owner-selected alternative, not a prerequisite or assumed MSIX hook. The DNS design remains a future whole-computer answering service with loopback ingress, pinned DoQ-only upstream, DNSSEC, bounded work and privacy-preserving caching. Existing VBS/TPM optional-storage decisions and mandatory clock-continuity requirements remain separate. Most of these subsystems are planned, not shipping capabilities.

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

### Restart Manager is another restart source

Windows Installer integrates with Restart Manager, which can stop and restart services during updates. `MSIDISABLERMRESTART` controls this behavior only when Restart Manager is in use. [S4] [S5]

**Proposed refinement.** Route RM-triggered starts through the same protected maintenance/generation admission as boot, manual start and SCM recovery. A public MSI property must not authorize clearing the maintenance state. In a future disposable-VM fixture, hold an update before commit, attempt each restart route, then test commit and rollback with RM enabled and unavailable. Preserve honest reboot-required results. This extends the existing complete-MSI lifecycle design rather than creating an updater.

### Configuration readback is not effective-state proof

Microsoft documents different activation points for different service settings. Required-privilege changes apply at the next service start; the service-SID and non-crash failure-actions flag pages state the next system start. [S6] [S7] [S8] Qualify first installation separately instead of inferring that every fresh installation universally requires a reboot.

The existing service record should distinguish configured value, documented activation point, observed token/policy and pending restart/reboot. A matching registry/configuration value is not enough to hand over clock authority. A restricted service SID also does not replace explicit read-confidentiality ACLs.

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


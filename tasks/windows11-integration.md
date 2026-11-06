# Windows 11 integration: agent task and evidence record

**Current owner amendment — 6 November 2026:** [OD-10 §14.1](../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026)
defers **all formal-proof work until after the first released software version
(V1)**. Retain the proof receipts and incomplete obligations below; their older
“next”, “mandatory before acceptance” and proof-first directions do not authorize
pre-V1 work or block first-version implementation, phase acceptance or release.
No new solver/compiler proof campaign, verifier extension, formal source
preparation or proof-metadata consumer repair is scheduled before V1. Resume
through normal bounded planning after that release. Runtime validation, checked
arithmetic, independent tests, property/boundary cases, 90% security-primitive
coverage, 95% critical-file mutation, security/native/package/release gates and
all existing Windows/fuzz/storage/UI holds remain in force. Current product work
follows [development §7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent).


**Date:** 5 November 2026, Asia/Tbilisi; earlier dated receipts retain their recorded time zones.<br>
**Status:** Research recorded; earlier integration/adoption review receipts retain their original candidates. The later owner decision in section 4.8 selects complete hardened MSIX preparation and David's Store attempt with services; it supersedes the MSI-first disposition in sections 4.6–4.7. Section 4.14 records the concrete source profile; section 4.15 records the deeper provisioning/removal research and unselected alternatives. Section 4.16 records the repository-local Windows Sandbox management skill and its documentation evidence. Section 4.19 records the later owner-ready scope and executed Sandbox management smoke. Section 4.20 routes the selected leap policy into W11-W02 and package provenance. Section 4.21 adds release-operations documentation, proposed support terms and pending rehearsals under the existing release/lifecycle owners. Section 4.24 refines pre-start provisioning; section 4.25 refines independent removal with recovery still pending. Section 4.26 records partial W11-W01 preparation through the S2.1 engine handoff. Package implementation and native product qualification remain unexecuted.<br>
**Initial inspected project baseline:** [`a6c2401b9a8166515293653f14f637e5543c689e`](https://github.com/DavidOsipov/NevroTick/tree/a6c2401b9a8166515293653f14f637e5543c689e), tree `7d0558ab295a7859e0d7d9e5b1180bea0fe30443`. This task was initially prepared from an API-derived partial snapshot, not a local Git checkout; no dirty/index/staged-state claim applies to that preparation. Later adoption and research candidates are identified in section 4.  
**Owner request:** integrate the completed Microsoft Windows 11 investigation into maintained documentation, architecture, the development plan and tasks for implementation agents.

**Platform status, 5 November 2026:** [OD-14](../docs/nevrotick-owner-decisions.md#18-od-14--compact-windows-platform-scope)
limits initial intended scope to Windows 11 26H2 and the current Server counterpart,
Server 2025 Desktop Experience, both x64. [Section 4.32](#432-two-family-windows-qualification-matrix--5-november-2026)
owns the compact profile/evidence matrix; availability and complete-product
results remain unverified/not run. Earlier W11 records retain their original scope.

## 1. Authority, scope and reconciliation

**UI amendment — 5 November 2026:** [OD-13](../docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit)
selects Slint as the default UI toolkit. The current source inventory and older
Win32 observations below retain their exact candidates. UI-M0's existing
[migration handoff](ui-m0.md#slint-migration-handoff--5-november-2026) owns pending
graph/licence, generated-code and native evidence; W11-W09 and package owners
must qualify the actual Slint graph when their slices are reached. The selection
lifts no Windows/fuzz/storage hold or D1–D3 deferral.

Read [AGENTS.md](../AGENTS.md), the [documentation guide](../docs/README.md),
the [Windows 11 research](../docs/plans/windows11-integration.md), its
[Microsoft Learn follow-up](../docs/plans/windows11-integration-follow-up.md), the
[MSIX/Store assessment](../docs/plans/windows-msix-store.md) for packaging work,
the [release policy](../docs/release-policy.md) for support/promotion/incident work, and
[development §8.5](../docs/plans/development-plan.md#85-windows-11-integration-research-and-qualification).
The research retains primary-source contracts, proposed adaptations and support
limits; this record owns bounded assignments and subsequent execution evidence.
It does not replace the [CHECK catalog](../docs/windows-time-architecture.md#202-acceptance-catalog),
[DNS-C owners](../docs/plans/system-dns-cache-design.md#11-acceptance-cases-for-the-future-implementation)
or [ST-T owners](../docs/plans/windows-protected-storage.md#7-focused-threat-scenarios-and-acceptance-mapping).
**W11-W01–W11-W10 are task-local work labels, not new acceptance gates.**

| Reconciliation with the inspected plans | Disposition |
| --- | --- |
| Clock domains, sleep invalidation, restricted identities, maintenance inhibition, DNS ownership and fixed resource limits | Retained; native mechanisms below make existing contracts and observations more concrete. |
| Packet timestamps, native DNS adapters, WSH, resource hints and CNG batches | Candidates to assess within their existing slice. Optional enhancement failure must have a truthful declared outcome; selecting none of these does not waive an existing required property. |
| Modern Standby, service-setting activation, Restart Manager and powerful process handles | Added composition cases for existing timing, service, mitigation and package owners; source documentation establishes no effective state. |
| Current development and user interface | Development §7's current portable Core checkpoint and P0–P7 order remain. UI-M0 stays fixture-only; D1, D2 and D3 remain Deferred until their existing re-entry gates. |
| DNS and storage policy | DoQ-only upstream with leaf/intermediate pins remains; no implicit DoH/DoT/DNSCrypt/plaintext fallback or ZTDNS enablement. Optional VBS/TPM/no-cache state remains separate from mandatory clock authority. |
| Release operations | The release policy prepares support records, containment and corrective recovery; its first-stable support baseline remains proposed. Existing W11-W10/W11-W05 and CHECK/OPEN owners retain actual decisions, rehearsals and native evidence. |

This request authorizes documentation integration. It does not lift the current
Windows execution stop, separate fuzz no-retry hold or storage-experiment pause,
or authorize dependency acquisition, tool installation,
system configuration, keys, clock effects or service/package experiments. Future
work uses its existing applicable authorization, contract and environment gates;
this task adds no blanket approval round. Material changes to product policy or
trust boundaries use the existing owner-decision procedure, blocking only their
dependent work. Do not scaffold product code to make a planning check executable.

Current Windows scope follows the later [section 4.19 disposition](#419-owner-ready-sandbox-scope--3-november-2026):
owned-Sandbox management and separately admitted guest work are permitted within
that boundary. The blocked stager/artifact, separate fuzz hold, storage pause
and restrictions on general host Windows execution remain.

Use [plan-code-change](../.agents/skills/planning/plan-code-change/SKILL.md) and
the [testing contract](../.agents/skills/testing/CONTRACT.md). Select only relevant
leaves from the [skills index](../.agents/skills/README.md) and
[phase skill map](../docs/plans/development-plan.md#5-required-skill-reading-and-use-by-phase):
Rust contracts and numerical/protocol truth for timestamps; Windows service
adapters, privileges and IPC for lifecycle/identity; crypto and dependency
admission for CNG; resource QA and native-memory diagnostics for measurement;
native UI/accessibility skills only at the restored UI stage. Repository tools
and admitted profiles remain the execution authority.

## 2. Next source-only assignment

Continue **one W11-W01 timestamp-provenance card** from the
[partial preparation in §4.26](#426-s21-timestamp-provenance-preparation--5-november-2026)
when the relevant acquisition owner is assigned. Its engine handoff preserves
real acquisition facts and separates experimental measurement from later Core
admission; the finite mapping profile and selected SDK/backend questions remain.
This source/documentation action does not replace development §7's proof-first
queue or the engine's separately held native qualification.
Reuse the completed research; do not restart a general Windows API survey.

The card identifies the proposed producer → metadata → conversion → sample
consumer path, actual source owners where they exist, private invariants,
independent expected results and the healthy/refused traces listed below.
Separate supported build/API, runtime capability, configuration activation and
observed behavior. Leave hardware mapping and installed-token facts unresolved
where no evidence exists. Record one selected scope, finite cases/resources,
required environment and cleanup before later execution.

Carry the [second-pass packet/FFI findings](../docs/plans/windows11-integration-follow-up.md#2-packet-timestamps-socket-metadata-and-completion)
into that same card: unchanged-route timestamp-mode transitions, separate data
and control-buffer admission, and actual completion ownership if a direct
overlapped adapter is proposed. Record the documented signature/layout conflicts
as unresolved selected-SDK/binding questions; an MCP result is not an ABI test.

**Exit:** a reviewer can trace the card's timestamp identities, uncertainty,
invalidation and unsupported outcomes to the existing CHECK owners. It neither
selects hardware timing nor claims an implemented adapter. Stop at that card;
source-only uncertainty does not justify a native probe. The remaining work
items enter only when their corresponding product slice is ready.

## 3. Bounded work assignments

All implementation/qualification actions below are planned and unexecuted.
Expand only the next ready authorized item into exact writable files, candidate
identity, admitted graph/runner, case/time/resource limits and cleanup. Retain
healthy neighbors and inspect the actual enforcing consumer; a mock or Windows
cross-build does not establish native behavior. Optional experiments do not all
become prerequisites for the next time-client slice.

### W11-W01 Preserve packet timestamp provenance

**Owner/stage:** engine/platform; source card first, optional adapter in S2.1/P2.
**Prerequisites:** existing exchange, units, generation and admission contracts;
[timestamp research](../docs/plans/windows11-integration.md#1-measure-packets-closer-to-the-network-stack).

**Current source disposition:** [§4.26](#426-s21-timestamp-provenance-preparation--5-november-2026)
records partial preparation through the existing engine card. The complete card,
selected mapping/adapter contract and all native qualification remain open.

**Work:** distinguish application, software-QPC and NIC timestamps. Bind metadata
to the exchange, socket generation, actual interface and current conversion
model; qualify RX packet attribution and TX interface selection. Bound TX IDs,
retention/retrieval and missing/saturated metadata. Supported NIC capabilities,
currently enabled mode and an adequate conversion are separate conditions.
Start with software timing; hardware timing requires its own evidenced benefit.
Late TX metadata is a local measurement and cannot rewrite sent/authenticated
wire bytes. Device configuration belongs to the lifecycle owner.

**Second-pass refinement:** evaluate `RegisterInterfaceTimestampConfigChange`
as an invalidation signal even when the route/interface identity is unchanged.
Its callback supplies caller context, not the affected interface; coalesce and
re-query outside the callback. Preserve context through draining unregister,
without self-unregister or a lock needed by the callback. Notifications do not
atomically identify the mode that produced queued RX/TX metadata. Keep the
documentation's support-table differences and `VOID`/`DWORD` unregister conflict
explicit until the selected SDK/generated binding is checked.

For `WSARecvMsg`, validate payload and ancillary data independently, including
`MSG_TRUNC`/`MSG_CTRUNC`, returned lengths, control-header framing/alignment and
reinitialized capacities/flags. Dual-stack IPv4 packet information also needs
the IPv4 option. Received `IN_PKTINFO.ipi_addr` is the local destination, not
the peer. Resolve the `WSACMSGHDR.cmsg_len` overview's `UINT` versus dedicated
page's `SIZE_T` discrepancy against the actual user-mode SDK/binding; do not
hand-copy the struct. An application-timestamp fallback still needs its own
complete admission/provenance contract. For cross-thread QPC readings, retain
the documented adjacent-tick ordering ambiguity and checked conversion bounds.

If a direct overlapped socket adapter is selected, specify close/issue
serialization, socket generation, immediate/pending/terminal states and actual
completion mode, including skip-on-success. Close/cancellation does not establish
that completions were delivered; buffers and charges remain owned until their
terminal observation. This conditional contract neither establishes how dropping
a Tokio future affects native I/O nor selects a runtime rewrite. See the
[packet and completion supplement](../docs/plans/windows11-integration-follow-up.md#2-packet-timestamps-socket-metadata-and-completion).

**Controls/exit:** independent vectors and healthy application/software cases
preserve sample arithmetic. Missing metadata, stale TX IDs, socket recreation,
two interfaces with different modes, route change, counter reset and saturated
buffers cannot admit a mislabeled sample or unbounded wait. Unsupported optional
timing retains a qualified, honestly labelled application path. Later native
comparisons use controlled peers and actual NIC/build/driver evidence, without
clock writes; no UTC-accuracy improvement follows from placement alone.
Add healthy unchanged-mode and stable-route mode-change traces, queued old TX
metadata, burst/stop races, exact/one-short data and control buffers, extra
control messages and reused output fields. Where direct overlapped I/O is
selected, cover immediate success with/without skip mode, pending completion,
cancellation losing to success, close with pending work and numeric handle reuse.
**Mapping:** CHECK-6/11/21/27 for packet admission and adapter behavior;
CHECK-7/25 only for affected downstream numerical and construction invariants
at their existing phase scopes.

### W11-W02 Reconcile sleep, timers and clock presentation

**Owner/stage:** platform/clock; S0.4/P4, with DNS lifetime consumers at DNS-1/3/5.
**Prerequisites:** [clock backend](../docs/windows-time-architecture.md#10-windows-clock-backend),
owned restoration, the [sleep/timer research](../docs/plans/windows11-integration.md#sleep-timers-and-leap-presentation),
and the selected [leap policy](../docs/plans/leap-second-policy.md#6-windows-contract-and-limits).
The [S1.4 task](s1.4-leap-policy.md) owns the pure calendar/guard prerequisites;
The current Core checkpoint and applicable selection prerequisites remain ahead
of dependent leap-policy implementation.

**Clock-limit preparation:** [§4.31](#431-clock-correction-limits-preparation--5-november-2026)
assigns native conversion, actual combined delivery, uncertainty and power/epoch
evidence to this S0.4/P4 owner. Values and native qualification remain open.

**Work:** preserve UTC, QPC and active-awake meanings. Modern Standby may suspend
services; a sleep-excluding system clock does not measure broker execution.
Recheck authoritative deadlines and generations after delayed/coalesced wakes.
No timer promises a finite lease on a programmed rate. Keep residual-rate and
unknown-cleanup outcomes, with mandatory broker work independent of optional
saves. High-resolution timers remain conditional on measured need.

For v1, retain the kernel as the **sole explicit leap-event executor** and qualify
the default compatibility presentation in engine and broker, recording/verifying
their own mode through supported process APIs without opting into `:60`.
Unexpected mode cannot reuse a trusted mapping. Keep FILETIME, civil UTC and NTP
coordinates distinct and preserve sub-millisecond conversion precision outside
the guard. No global leap setting/history changes, event injection, guessed NT
setter, temporary W32Time start or `w32tm` parser belongs to the production path.
Operator/lab status output is diagnostic evidence, not a stable API contract.
Separately admitted synthetic-event fixtures follow the
[S0.4 lab boundary](s1.4-leap-policy.md#3-native-mapping-and-broker-effects), with
documented setup, independent observation and restoration; they are not shipped.

The [follow-up](../docs/plans/windows11-integration-follow-up.md#2-packet-timestamps-socket-metadata-and-completion)
adds the actual I/O wait and unattended-resume path: an IOCP timeout can exclude
low-power time, and a dequeued batch can contain failed operations. Recheck the
authoritative deadline on execution rather than treating that timeout as elapsed
freshness. Cover unattended/automatic resume without UI activity and the
sleep-to-hibernation transition; no power callback promises execution by a deadline.

**Controls/exit:** pure traces distinguish elapsed freshness from active work;
healthy recovery accepts fresh results while stale ones fail. Later permitted
native cases cover actual S3/S4 and Modern Standby hardware, AC/DC, delayed
execution, UTC change and suspend near expiry. A generic VM does not prove
Modern Standby behavior. Observe actual adjustment mode/trajectory and cleanup
under the existing clock fixture; retain uncertain outcomes. Timer policies do
not resurrect DNS TTLs or enqueue every missed refresh. **Mapping:**
CHECK-10/11/12/21/25/27; DNS-C05/11 for affected lifetime/context consumers.

Carry the [NV-LP-001 guard and native limits](../docs/plans/leap-second-policy.md#5-event-guard-holdover-and-reacquisition)
into those same S0.4/P4 observations:

- Compare OS-handled, missing and unexpected native events, both signs, with
  healthy all-LI=0 sources. Exact event-time UTC requires separate OS knowledge
  evidence; a missing proof retains visible uncertainty and residual recovery.
- Challenge default presentation, changed/unsupported mode, conversion ranges
  and precision, plus delayed crossing exchanges. No hidden `:60` opt-in, fixed
  FILETIME-to-NTP formula or universal 37-second correction may pass.
- Observe transient cancellation outcome, owned base rate and actual trajectory
  through guard, sleep/restart and reacquisition. Kernel action is not a broker
  acknowledgement; an OS-handled event must not cause a duplicate ±1-second act.
- After the guard require fresh ordinary independent agreement and correct only
  the measured residual under existing budgets. Future coverage unknown alone
  permits qualified Tracking outside guards; timeout alone restores no authority.

### W11-W03 Qualify native DNS ownership and coverage

**Owner/stage:** DNS/platform/lifecycle; DNS-0 contract, DNS-1 listeners,
DNS-3 native integration. **Prerequisites:**
[DNS activation/restoration](../docs/plans/system-dns-cache-design.md#23-windows-activation-ownership-and-restoration)
and [native DNS research](../docs/plans/windows11-integration.md#2-integrate-the-dns-service-without-losing-windows-policy).

**Work:** evaluate versioned Get/SetInterfaceDnsSettings with exact field masks,
allocation/lifetime owners, both address families and an ownership record.
Effective server lists do not prove DHCP inheritance; the native origin/reset
path remains unresolved until demonstrated. Acquire deliberately scoped
exclusive loopback UDP/TCP listeners before enrollment. Coalesce network
notifications into reconciliation; separately cover DNS-only/NRPT policy edits
and callback cancellation. Inventory namespace/suffix routes, host encrypted-DNS
policy, WSL tunneling, Docker's distinct path and managed ZTDNS conflicts.
`DnsQueryRaw`, settings V4 and DNR remain support-qualified candidates.

The [DNS follow-up](../docs/plans/windows11-integration-follow-up.md#5-dns-integration-and-resource-hints)
adds a selected native resolver's two completion paths: only
`DNS_REQUEST_PENDING` establishes asynchronous callback ownership. Immediate
success/failure and pending/late completion must settle once; retain pending
result/cancel/context storage until actual completion. A matching NRPT rule can
replace `DNS_QUERY_REQUEST3.pCustomServers`; an explicit server list is not
universal egress authority. Native custom-server examples do not implement the
adopted DoQ/pin transport. Keep this adapter's actual purpose and egress separate.

For interface enrollment, changing `NameServer` must preserve suffix,
registration, discovery and other unowned fields in both address families.
`EnableLLMNR` also affects mDNS; a reserved field such as
`DisableUnconstrainedQueries` is not an available security control. Privacy
descriptions distinguish NevroTick caches/statistics from the Windows DNS Client
and application caches: Forget does not erase every computer-wide DNS trace.
Do not force TTL zero or flush global caches to improve heatmap visibility.

**Controls/exit:** later disposable fixtures pair DHCP/static and IPv4/IPv6
healthy enrollment with partial writes, intervening administrator/VPN edits,
port conflicts, restart gaps, event bursts and callback-in-progress stop.
Synthetic private-name timeout/disconnect must not send that name to public
DoQ; a stable-address DNS-only edit must still invalidate affected work. Observe
declared WSL/container routing and negative LAN probes. Unsupported/ambiguous
origins and managed conflicts are explicit outcomes; never widen listeners,
disable policy or enable ZTDNS to make coverage appear complete. **Mapping:**
DNS-C01/02/03/11/17; CHECK-13/25/27 where lifecycle/adapters are affected.
The added privacy/Forget boundary uses existing DNS-C14/15. Add paired NRPT
matching/nonmatching names, immediate/pending resolver paths and an enrollment
case that preserves local discovery after an intervening administrator edit.

### W11-W04 Qualify service-scoped network restrictions

**Owner/stage:** security/lifecycle/platform; P0/P3 and installed P6 path.
**Prerequisites:** selected service identities and
[WSH research](../docs/plans/windows11-integration.md#3-constrain-service-networking-with-a-windows-owned-mechanism).

**Clock-limit preparation:** [§4.31](#431-clock-correction-limits-preparation--5-november-2026)
requires effective-token/protection evidence beside useful permitted operations
and engine denial; API samples do not establish clock authority.

**Work:** assess `INetFwServiceRestriction` and its separate WSH rule collection
before custom WFP. Retain the restricted engine SID profile; sample parameters
are not product policy. Give broker denial and engine allowances distinct
owners, activation observations and managed-conflict outcomes. Include delegated
DNS/certificate work. Ordinary explicit firewall blocks still override ordinary
allows; they are not interchangeable with WSH defaults/exceptions.

Before selecting WSH or another COM adapter, apply the
[COM ownership contract](../docs/plans/windows11-integration-follow-up.md#3-service-lifecycle-installation-and-com):
per-thread apartment/init balance (including `S_FALSE`), process-wide security
initialization timing, interface agility/marshaling, callback direction and
release owner. An incompatible apartment or early implicit security setup is
a typed failure, not permission to relax the profile. Final `CoUninitialize`
can dispatch messages; retain shutdown guards and callback state through this
reentrant boundary. Test ordinary permitted activation under final mitigations
beside mismatched apartment, late security initialization and shutdown callback
cases. Do not add a generic COM runtime, process or broad security flags.

**Controls/exit:** actual installed identities perform approved IPv4/IPv6 work
while forbidden destinations and broker external traffic fail. Observe activation
delay, restart/update and policy conflicts, with legitimate provider operations
beside denials. A matching rule query alone proves no confinement. Custom WFP
needs a demonstrated gap and separate object ACL/transaction/session-lifetime
design; a disappearing denial after owner death is not acceptable evidence.
**Mapping:** CHECK-1/9/16/23/27, with CHECK-9 limited to delegated certificate
retrieval; DNS-C14/17 when the DNS service participates.

### W11-W05 Observe effective SCM, maintenance and process policy

**Owner/stage:** platform/security/lifecycle; P0/P3, P4 cleanup and P6 complete MSIX.
**Prerequisites:** existing token/object, mitigation, mandatory-state and
[installation research](../docs/plans/windows11-integration.md#4-make-installation-and-service-state-reflect-actual-windows-behavior).

**Clock-limit preparation:** [§4.31](#431-clock-correction-limits-preparation--5-november-2026)
keeps actual native-job completion, broker cleanup, durable late outcomes and
conditional recovery exposure with this owner; configured delays prove no expiry.

**Work:** record configured settings, documented activation point, actual token/
policy and pending restart/reboot separately. Qualify first installation rather
than assuming a universal reboot. Route every supported package/service restart
through the same protected maintenance/generation admission as boot, manual and
SCM recovery. For a later selected MSI alternative, include Restart Manager and
prove public MSI properties cannot clear that guard. Extend child-policy observations with
harmless requests for `PROCESS_CREATE_PROCESS`/`PROCESS_VM_WRITE` on dummy peers
and inherited/duplicated-handle review. The SCM creation-policy route remains
unresolved; no custom launcher is selected. If preshutdown is used, give it an
explicit budget independent of optional cache work.

**Retained MSI-specific candidates, only if that alternative is later selected:**
evaluate declarative `MsiLockPermissionsEx` before
custom ACL actions where it expresses the required descriptor. Qualify account/
service-SID resolution and ordering, installed/reinstalled resource coverage,
parent inheritance, conflicting conditions and final owner/DACL behavior after
runtime replacement. Its formatted descriptor is not ordinary MSI-property
substitution. Inheritable parent permissions can protect later children; the
table does not reapply every individually authored descriptor to arbitrary new
objects. Inspect the actual MSI and valid/failed repair/upgrade paths.

Compare `GetServiceDirectory`/`GetServiceRegistryStateKey` only for suitably
optional/disposable state initially. Service deletion removes that state;
reboot persistence does not prove delete/recreate-upgrade survival. Do not move
mandatory correction/ownership/restoration records into it without their
existing lifecycle contract. Actual account, child-object and replacement
access still need evidence; the shared variant has different write permissions.

For a selected SCM observer, distinguish nonblocking subscription callbacks
with draining unsubscribe from one-shot APC notification/alertable-wait rules.
Coalesce and re-query outside callbacks; handle zero-mask and delete-pending
cases without deadlock or retaining handles that prevent uninstall. Neither a
notification nor SCM Running establishes authenticated product readiness.
Future UI consumers retain D2. These [lifecycle candidates](../docs/plans/windows11-integration-follow-up.md#3-service-lifecycle-installation-and-com)
reuse this owner and the existing storage task; `ReplaceFileW` failure/recovery
cases remain with [ST-W02/03](windows-protected-storage.md#st-w02-enforce-operation-outcomes-and-model-durability).

For the selected full-MSIX target, use the [section 4.8 assignment](#48-owner-selected-hardened-msix--3-november-2026)
with W11-W10. Supported pre-start privilege configuration, update/repair
persistence, multiuser ownership and recovery-before-removal must preserve the
same lifecycle boundary. MSIX manifest/service support, package replacement and
first-UI setup do not substitute for those observations. Prepare the design now;
native execution retains its hold. Do not use MSI actions/properties as assumed
MSIX hooks, or require an MSI prerequisite to obtain the intended isolation.

The [5 November continuation](#424-pre-start-service-provisioning-refinement--5-november-2026)
and [MSIX plan §7.5](../docs/plans/windows-msix-store.md#75-pre-start-provisioning-authority-and-admission--5-november-2026)
put the first provisioning discriminator before setters: inspect actual
deployment-created service objects and request/close each necessary handle right
under the proposed actor. Authority, supported no-UI invocation and servicing
reentry are separate unresolved conditions. Later controlled SCM management start
precedes fresh-process admission and normal work; creation-time protection must
already hold. This source refinement neither selects a candidate nor admits the
probe. The independent-removal pair and all current execution dispositions remain.

The [independent-removal continuation](#425-independent-removal-with-outstanding-recovery--5-november-2026)
and [MSIX plan §7.6](../docs/plans/windows-msix-store.md#76-independent-removal-with-outstanding-recovery--5-november-2026)
require the same durable dummy obligation to remain pending at removal in both
running and already-stopped/crashed cases. Prove fresh independent activation,
usable code/data/current rights and the actual effect/outcome; keep clean STOP,
successor denial and observer/instrumentation controls separate. Native package
retention plus an independent task remains unselected and incomplete.

For bad-release recovery, apply [release-policy §6](../docs/release-policy.md#6-mandatory-state-compatibility-and-owned-recovery)
to the affected version's actual mandatory state, partial migrations and uncertain
effects. A compatible code fallback cannot restore an obsolete authority ledger;
new-version refusal must retain a usable recovery route while prior takeover
remains. The procedure adds no new native execution grant or provisioning owner.

**Controls/exit:** allowed service work and private-object access succeed;
disallowed token/handle access fails under installed identities. Hold maintenance
before commit; RM/manual/queued recovery starts must not admit normal protected
work. Compare successful update, supported rollback/recovery and honest
reboot-required outcomes; RM disabled/unavailable cases apply to the retained
MSI route. Delayed cleanup must not fabricate Stopped. Pure
lifecycle traces precede disposable native/service/package observations; no rights
grant is dismissed because a harmless child-launch test passed. **Mapping:**
CHECK-1/12/13/15/16/23/27; ST-T03/10/12/17 for affected storage owners.

### W11-W06 Make optional cache work resource-aware

**Owner/stage:** DNS scheduler/resource owner; DNS-4 shadow inputs, DNS-5 refresh.
**Prerequisites:** ordinary-cache baseline, fixed caps, reserved time/foreground
capacity and [resource-input research](../docs/plans/windows11-integration.md#5-make-adaptive-caching-cooperate-with-the-operating-system).

**Work:** define a small state for effective power mode, destination cost and
system memory pressure; unknown cost is not free. Coalesce level-signaled memory
notifications with hysteresis. Trim only eligible optional state; maintain
retained-reader charges, bounded scanning and allocation-error handling.
Per-thread EcoQoS is optional for suitable actual background workers, not a
process-wide setting on the time engine or a label on a shared runtime task.

Compare the [native connectivity-hint candidate](../docs/plans/windows11-integration-follow-up.md#5-dns-integration-and-resource-hints)
with COM cost tracking for the actual required input. Global/per-interface
`GetNetworkConnectivityHint*` results can avoid COM plumbing if sufficient;
they are not destination-aware `GetCost` equivalents or proof of an upstream
route. Bind observations to network generation and actual egress; coalesce
notifications and retain callback lifetime through cancellation. Unknown cost
is not free, and an Internet-status hint alone must not forbid bounded attempts
to a reachable controlled peer. Add two-interface/different-cost, route-change,
unknown-hint and stop-during-callback cases while required work still progresses.

**Controls/exit:** normal demand and required work progress under healthy states;
persistent low-memory signal, pending cost query, battery/metered transitions,
resume and allocation failure neither spin nor exceed caps. Pressure cannot
erase mandatory authority or extend answer freshness. Compare foreground/tail
latency, wakeups, CPU and energy with the existing baselines on named hardware;
no autonomous wake merely to maintain heatmaps. **Mapping:** DNS-C05/08/11/12/18;
CHECK-21/27 where shared work is affected. Native hints are not security authority.
The new hint cases primarily use DNS-C11/12/18; DNS-C08 applies only when the
selected change also affects temperature or seasonal-model behavior.

### W11-W07 Evaluate bounded CNG hash batches

**Owner/stage:** DNSSEC/crypto/performance; DNS-2 after a measured eligible queue.
**Prerequisites:** admitted validator/provider graph, independent vectors and
[CNG throughput research](../docs/plans/windows11-integration.md#6-use-native-cryptographic-acceleration-for-a-measured-workload).

**Work:** compare the admitted backend with `BCryptCreateMultiHash` for eligible
independent SHA-256/384/512 inputs; cache appropriate provider handles only under
the existing algorithm/lifetime policy. SIMD batching is separate from bounded
multicore scheduling and does not accelerate all signature, TLS, NTS or AEAD
operations. Do not delay an isolated foreground request to fill a batch or use
this candidate to decide the still-open crypto graph implicitly.

**Controls/exit:** independent known-answer and accepted/rejected validation
neighbors agree across selected paths. Small/empty/partial workloads, hostile
work, cancellation and provider failures retain fair admission and real custody.
Measure tail latency, throughput, CPU and energy on supported CPU/OS rows before
adoption. No benefit, capability absence or unacceptable latency can leave this
candidate deferred while required DNSSEC acceleration/multicore work retains its
owner. **Mapping:** DNS-C04/12/16; CHECK-20/21/25/26/27 as applicable. Hash evidence
does not qualify optional-storage roots or a different cryptographic operation.

### W11-W08 Trace native behavior and qualify one fault profile

**Owner/stage:** diagnostics/QA and the affected component's existing phase.
**Prerequisites:** typed redaction/session policy and
[diagnostic research](../docs/plans/windows11-integration.md#7-diagnose-actual-windows-behavior-and-inject-native-failures).

**Work:** use monotonic IDs/times to correlate packet, queue, verification,
selection, broker admission and actual acknowledgement with WPR/WPA. Record ETW
loss; missing trace events cannot prove absence and traces cannot replace the
durable journal. PerfLib V2 aggregate counters are optional coarse monitoring.
For one named low-resource question, first admit its exact Application Verifier
profile and healthy/deliberate-failure controls under the existing tool policy.
Current Basics/full PageHeap admission does not admit arbitrary extra profiles;
the `/faults` shortcut is not coverage of every native resource failure.

The [diagnostics follow-up](../docs/plans/windows11-integration-follow-up.md#4-diagnostics-replacement-and-protected-storage)
adds multiple-controller qualification. ETW enable/keyword interest is aggregated
and cannot authorize or extend product verbose diagnostics. Retain the local
authorization/expiry guard independently of controller-supplied `SourceId`;
`GUID_NULL` is a GUID value, not a nullable-pointer contract. Capture-state follows
the same field/resource policy. Provider registration/enable rights, session
control/consumer rights and ETL-file access/retention are separate observations.
The local guard bounds emission; it does not make events exclusive to one
matching session or recall already collected ETL data.

Exercise two controllers with different filters, expiry while one remains,
capture-state and provider restart; required-event routing remains useful and
synthetic secrets remain absent. For crash privacy, identify each collector:
process WER/NOHEAP choices do not establish control of independent LocalDumps or
every debugger path. Reuse [ST-W07](windows-protected-storage.md#st-w07-verify-sensitive-memory-lifetime-and-actual-output-privacy)
for admitted dummy-secret checks; do not alter administrator dump policy here.

**Controls/exit:** actual allowed diagnostics remain useful and synthetic secret/
domain markers remain absent. Trace loss is reported. A selected allocation,
handle, registration or CNG failure reaches the intended refusal/cleanup owner
without half-applied effects or false Running. Restore the owned per-image
profile on every exit and inspect cleanup before harness teardown. Instrumented
fault discovery and uninstrumented resource measurements remain separate.
**Mapping:** CHECK-1/17/21/27, with CHECK-1 limited to diagnostic access;
ST-T14/16 and DNS-C12/14 for affected storage/DNS paths.

### W11-W09 Add truthful native notifications at the deferred UI stage

**Owner/stage:** UI/accessibility; P5 after the relevant D2/D3 re-entry gate.
**Prerequisites:** the admitted Slint/backend/renderer and selected accessibility
exposure from the [UI migration chain](ui-m0.md#slint-migration-handoff--5-november-2026),
actual authenticated current-generation service snapshots, restored D2/D3
contracts and [native UI research](../docs/plans/windows11-integration.md#8-present-live-state-through-native-ui-facilities).

**Work:** qualify the actual Slint accessibility/Windows bridge without assuming
inherited Win32-control semantics. Evaluate `UiaRaiseNotificationEvent` and supported processing modes for
meaningful state changes; ordinary ticking status must not create important
announcement spam. Local Windows App SDK notifications are a separate candidate
with deployment/dependency cost; unpackaged support does not remove that review
or justify package identity automatically. Keep presentation unelevated and
revalidate current generation/authority at every notification action.
Any selected UI COM class also uses W11-W04's class-specific apartment,
security-initialization and teardown contract; this does not restore D2/D3 early.

**Controls/exit:** meaningful current loss/recovery or optional-cache status is
announced accurately; stale events, rapid repetition and expired actions cannot
mislead or authorize protected effects. Observe actual Narrator/keyboard behavior
on the selected build, including a compatible older processing mode where
required. The build-26100 enum choice is not the base API floor or accessibility
proof. **Mapping:** CHECK-14/17/22; ST-T01/14 when storage status/privacy is reached.
UI-M0 D1/D2/D3 remain Deferred; this task activates none of them.

### W11-W10 Re-enter conditional mechanisms only for a concrete need

The [conditional research](../docs/plans/windows11-integration.md#9-conditional-mechanisms-worth-understanding)
and [second-pass reserves](../docs/plans/windows11-integration-follow-up.md#6-conditional-mechanisms)
own source contracts and limits. These reserves are not a prerequisite list.
The later OD-05 decision selects the MSIX row for preparation; the other rows
retain their own conditional triggers. The heading is preserved for existing links.

| Reserve / existing owner | Re-entry condition and bounded exit |
| --- | --- |
| Job Objects or SCM triggers / platform | A measured optional-role resource/activation need; compare healthy useful work with limit exhaustion, stopping/maintenance races and owner death. Preserve independent broker cleanup; an address or endpoint trigger is not Internet readiness or first-UDP/53 activation. |
| LPAC/AppContainer or preview Win32 app isolation / architecture/security | A named untrusted-work boundary justifies capabilities, IPC, supported launch/deployment and lifetime costs. Observe actual denied access beside admitted work; standalone creation APIs do not prove SCM compatibility. No new privileged broker follows automatically. |
| Stricter CET/EHCONT / build/security | A selected mitigation property for the exact Rust/native closure; inspect metadata and actual enforcement beside legitimate provider/callback behavior. Existing CHECK-16/20/26 owners remain. |
| Full VBS enclave / storage/release | A secret operation needs enclave execution beyond isolated CNG roots. Separate runtime build/VBS/HVCI from development/signing eligibility, and use the existing storage task and ST-T11/12/13; the native storage pause remains. |
| External-location identity or WinGet / UI/packaging | External executable content is excluded by the selected MSIX profile; reconsider only through a later explicit architecture decision, not to bypass packaging gates. An additional WinGet acquisition path needs the same complete-package identity/lifecycle and CHECK-18/23 evidence. Local notifications alone do not select another updater. |
| Complete hardened MSIX with Store distribution / packaging/release/security | **Selected preparation target under amended OD-05.** Execute the source/design work in [section 4.8](#48-owner-selected-hardened-msix--3-november-2026) using the [focused plan](../docs/plans/windows-msix-store.md). Explicit content integrity, immutable executable paths, Store-compatible CIG and per-role protections join supported service provisioning and complete recovery. Store preapproval is not a preparation gate; actual signed-artifact qualification follows David's controlled Store attempt. All existing §19.4/CHECK owners and execution holds remain; MSI is a retained alternative for a later owner decision. |
| W32Time provider DLL / architecture/clock | An explicitly different Windows-managed-discipline goal. First reconcile controller ownership, trust and restoration in a source-only decision card; never add it as a competing controller. Existing CHECK-5/10/13 remain. |
| VBS key claims / storage/security | Actual-key origin evidence is the named blocking question. The new `NCryptCreateClaim`/`NCryptVerifyClaim` extensions are prerelease; the base API floor does not qualify them. Define trusted anchor, subject/freshness binding, replay/version/debug/update policy and attestation-key/hash/padding prerequisites. Do not add roles to wrapping/signing roots or adopt a new root, cloud service or hardware rollback claim. Reuse the storage reserve and ST-T01/05/13. |
| Side-channel policy fields / per-process security | A named threat, process role and supported hardware/build justify one field. Account for irreversible enablement, scheduler dependencies and common-page exceptions; readback is not an enforcement measurement. Compare healthy timing/crypto/IPC and CPU/energy costs under CHECK-16/21, without raising privileges or enabling every bit. |
| UDP coalescing/segmentation / selected transport | A measured bottleneck justifies a backend/option comparison, such as future DoQ. Prove original datagram boundaries, short-final-segment and missing/truncated-control outcomes, per-datagram accounting and timestamp attribution. This is not DNS request coalescing or a reason to change NTP defaults/global offloads; existing CHECK-6/21/25/27 and affected DNS-C12 apply. |
| VM Generation ID / clock/security | A restore-witness need has a supported privilege design. The documented admin/SYSTEM device access is not permission to broaden engine/broker accounts. Cover missing/access-denied device, stale read/change, restore and cancellation; a witness cannot reset consumed authority or prove a trustworthy boot counter. Reuse CHECK-11/12/15/27. |

Selected MSIX preparation also consumes [section 4.21](#421-release-operations-integration--4-november-2026)
and the [release policy](../docs/release-policy.md). W11-W10 retains support and
publication/incident readiness; W11-W05 retains mandatory-state and lifecycle
recovery. Source preparation is recorded, while owner choices and actual
rehearsals remain open at their existing phase gates.

**Metadata requirement, 5 November 2026:** W11-W10/P6 consumes the
[product/package metadata contract](../docs/plans/windows-package-metadata.md)
under OD-05 §7.6. Before the package slice, inventory applicable fields and their
authoritative values, unresolved Store/support/version inputs and per-component
resources. Then generate from one maintained record and independently inspect
emitted PE resources, selected MSIX or conditionally selected MSI metadata,
localization and actual installed surfaces. Use the verified Windows API map;
source binding presence is not dependency or native qualification. Existing
CHECK-18/20/23/26/27 and W11-W05's service-description/lifecycle ownership apply.
This source handoff selects no new crate, package tool, identity or release.

**Metadata workflow skill, 5 November 2026 (UTC):**
[maintain-windows-metadata](../.agents/skills/implementation/maintain-windows-metadata/SKILL.md)
now supplies the reusable collection, authority/conflict, storage, generation and
independent-readback procedure. Its four references and illustrative TOML/JSON
records extend the [contract's workflow recommendation](../docs/plans/windows-package-metadata.md#11-harvesting-storage-and-use-workflow--5-november-2026).
Use repository-owned values and assigned Store identity, retain raw observations
separately, and reuse the frozen CycloneDX/package evidence owners. Do not use
Win32_Product for harvesting, equate Cargo resolution with shipped payloads, or
treat MSIX reader success as signature verification. This is source preparation;
the production record/schema, generator, package and native evidence remain open.
Next W11-W10/P6 action is to populate the current field register and resolve its
required-stage inputs before implementing the admitted build/readback path.
The current Core priority and native/fuzz/storage holds remain unchanged.

Skill delivery validation: the package checker passed for 65 registered skills;
the skill-creator validator and TOML/JSON example parsing passed, and added
local Markdown paths/heading anchors resolved. No product/native gate was run.
A bounded fresh-agent use exercise could not start because the agent service
reported a context-window failure; no independent behavioral evaluation or
measured instruction-quality improvement is claimed.

The [provisioning refinement](#424-pre-start-service-provisioning-refinement--5-november-2026)
and separate [independent-removal refinement](#425-independent-removal-with-outstanding-recovery--5-november-2026)
supply W11-W05/W11-W10's next bounded source/evidence routing. Retention does not
establish post-deregistration activation or recovery authority; the selected
complete-package direction and phase gates remain.

Protected Process Light is excluded as a normal utility-service option; ordinary
publisher signing supplies no protected-service eligibility. No reserve selects
a dependency, weaker fallback, service topology or machine-wide policy here.

## 4. Evidence and continuation

| Evidence layer | Current state |
| --- | --- |
| Microsoft source investigation and critical source/fit review | Completed research retained in the linked document; documentation facts and engineering inferences are distinguished. No native experiments were executed. |
| Documentation, architecture, plan and task integration | Earlier integration and local-adoption review/validation retain the candidates in sections 4.1–4.3. The second-pass integration is recorded from section 4.4; the MSIX/Store amendment is recorded from section 4.6. Earlier passes do not qualify changed text. |
| W11-W01 source contract | Partially prepared in [§4.26](#426-s21-timestamp-provenance-preparation--5-november-2026); the engine handoff identifies current producers/consumers and evidence gaps. Finite mapping/profile, selected SDK/backend questions and complete card exit remain open. |
| Portable implementation/model | No W11 work-item, CHECK, DNS-C or ST-T implementation pass established by this integration. |
| Native Windows/service identity | No product qualification established by this integration; section 4.19's scoped Sandbox permission and all remaining artifact/fuzz/storage and environment gates apply. |
| Installed MSIX and release | Unexecuted; local test-signed mechanics, Store acceptance and actual Store-delivered trust/mitigation qualification remain separate evidence layers. |
| Stable-release operations and incident recovery | Written source-based specification and proposed support baseline in section 4.21; actual support approval, account readiness, rehearsals and native recovery remain pending. |

For later work, record the selected item and existing acceptance IDs, exact
source/configuration/graph/candidate, writable owner, admitted runner/environment,
independent expectations, healthy/failure controls, finite limits and actual
results. Preserve failures, skips, unsupported/conflict outcomes and cleanup.
Source review, model execution, cross-build, native observation and installed
package evidence answer different questions; never promote one to another.
Recheck only affected evidence when the relevant bytes, provider, account,
Windows build, device/driver, policy or package change. Continue in this record
and the existing owning task rather than starting a parallel plan.

### 4.1 Integration review and documentation validation receipt

**Stage and outcome:** independent review completed; **READY for the
owner-authorized documentation integration/publication stage**. Reviewer
`windows_integration_publish_review` had fresh context (`fork_turns: none`),
read-only candidate access and no authorship of these files. The reviewer used
the current repository `plan-independent-review` skill, read both new documents
in full and all meaningful changes in the eight existing files, and checked
phase/acceptance mappings against their existing owners. No unresolved finding
remains within that scope.

**Reviewed identity:** baseline commit
`a6c2401b9a8166515293653f14f637e5543c689e`, tree
`7d0558ab295a7859e0d7d9e5b1180bea0fe30443`; ten-file review manifest SHA-256
`eae0d60fba76668a8d8a61cf59d04d8c769b52c2283d7c969e7810ef4c0fbc10`. The reviewer
independently recomputed all ten byte counts, SHA-256 and Git blob hashes:
ten matches, zero mismatches. Exact reviewed file identities follow. The task
identity is **before this administrative receipt and status amendment**; the
other nine files retain their reviewed bytes.

| Reviewed file | SHA-256 |
| --- | --- |
| `AGENTS.md` | `3e1518d0c64e68a9eda505108e06d195ee17244909aaa2cc674e0675f26aa11e` |
| `ROADMAP.md` | `3edb84b35a91b95171417046d85aae86f2ff6a5fb2a1e7666b58e62d8c9fe802` |
| `docs/README.md` | `a595e50843b3083fe5502984092fd44e2e4af96e68e424da26f1a5a33a90c79a` |
| `docs/plans/development-plan.md` | `909934ad256cc165df8e35875863b7d1e2671124efda98524095b2cbb05f1818` |
| `docs/plans/system-dns-cache-design.md` | `2cdd254a216411d700ce2cc40dd007f0e511d18bb0c3a97df2af9c4e52777699` |
| `docs/plans/windows-protected-storage.md` | `475ef85e0fdf38885ccbf13113f21881437fe8f570dc948c60454068bc378f6b` |
| `docs/plans/windows11-integration.md` | `708530380464c8274a6963a3e7d90dd6e1f00702e2b29761521529ae8740d101` |
| `docs/windows-time-architecture.md` | `5fd8d856dd4b38a977b0237ead0e526e676af664f47c249a468134562ce22abb` |
| `docs/windows-time-development-hardening.md` | `722beea82c69caf156f25259044c83b30b7db83c20b230dbf964319a7c69a9a2` |
| `tasks/windows11-integration.md` | `599cf9226c458c93d28f769ad3ff9536618974e222ef417e50904e427a643946` |

**Reviewer comprehension, preserved verbatim:**

> The change retains Windows 11 research as a source-attributed proposal and distributes its concrete future checks into existing platform, DNS, storage, diagnostic and UI owners. W11-W labels locate bounded assignments; they must not create new canonical acceptance IDs or adopt an API, runtime dependency, security profile, release channel or service topology.
>
> The inspected baseline has pure/fixture time components plus a bounded observation engine; production services, whole-computer DNS and protected optional storage largely remain plans. S1.3 remains next, UI-M0 remains fixture-only with D1/D2/D3 deferred, and native/storage experiments remain paused. One complete MSI, least-privilege engine/broker separation, DoQ-only pins, and mandatory clock state separate from optional cache remain controlling constraints.

**Findings and disposition:**

- **W11-IR-01 — low, fixed and independently verified.** The WSH hardening row
  referred to CHECK-8, whose canonical subject is duplicate sources/minimum
  agreement. Replaced that unrelated mapping with CHECK-9 explicitly limited to
  delegated certificate retrieval; W11-W04 and development section 8.5 carry the
  same qualification.
- **W11-IR-02 — low, fixed and independently verified.** W11-W01 and development
  section 8.5 omitted CHECK-6's packet-admission owner. Both now include it;
  CHECK-7/25 apply only to affected downstream numerical/construction invariants.

**Coordinator validation:** scoped documentation checks passed for exactly ten
Markdown files. The checked scope is changed lines in existing files and new
documents in full: 64 distinct introduced local links/anchors resolve against
the complete pinned tree and actual candidate/base bytes; 69 new source
reference definitions are complete and distinct; six GitHub source links retain
pinned blob revisions. No missing anchor-source bytes, reference errors,
UTF-8/replacement-character, final-newline or changed-line whitespace defects
remain. The development plan's entire section 7, all 27 CHECK catalog rows and
all 18 DNS-C acceptance rows are byte-identical to baseline; the architecture's
REQ/BND/DEC/THR/CHECK/ST-T identifier set is unchanged. W11-W01 through W11-W10
each have exactly one work-item heading. The scoped validator is rerun on the
final receipt-bearing candidate before publication. No root documentation
checksum manifest exists in the inspected tree; separate design manifests were
outside this change.

**Limits and continuation:** the completed source/fit research was reused; this
integration review was not another exhaustive Microsoft API or external-link
availability audit. Review and documentation checks establish no Rust, Windows,
service, installer, storage, performance or accessibility result. The coordinator
reconciles only this factual receipt/status delta with the completed review, then
checks expected-head concurrency and exact published paths/blob identities.
Substantive changes reopen the affected review conclusion. W11-W01's source card
and all implementation/native assignments above remain unexecuted.

### 4.2 Local checkout adoption — 3 November 2026

David requested the same pull and task-adoption procedure as the preceding
storage update. Local `main` fast-forwarded from `a6c2401` to `f163c19`.
All 120 staged paths were preserved. Three overlapping routing documents
(`AGENTS.md`, the documentation guide and development plan) were merged
additively; current Windows/fuzz stops, the storage queue, Core checkpoints,
detailed phase gates and historical review receipts remain. The source/API
review in section 4.1 applies to its original candidate, not this merged checkout.

At its next checkpoint, the primary development coordinator reads
[development §7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
and [§8.5](../docs/plans/development-plan.md#85-windows-11-integration-research-and-qualification),
records each applicable W11 assignment's owner, disposition and next action here,
and carries its contracts into the owning task before dependent implementation.
The existing ST-W01–ST-W07 queue stays separate. This is a discoverable handoff;
acknowledgement by a separately running agent is pending its next checkpoint.

Initial dispositions: W11-W01 is ready for one source-only timestamp-provenance
card when assigned to the acquisition owner, alongside the existing product
priority. W11-W02–W11-W08 remain planned at their named phases and prerequisites,
with Windows observations held; W11-W07 also requires an evidenced eligible
workload. W11-W09 remains deferred until its existing live-UI/accessibility
triggers. W11-W10 remains a conditional reserve. Optional candidates are not
silently selected by adopting an assignment; existing required properties and
CHECK/DNS-C/ST-T ownership remain binding.

Current Core work continues from the S150 task addendum, which records the
fitted-anchor bridge's development pass and remaining projection qualification;
do not repeat its preceding source-only assignment. No Core source or evidence
was changed by this documentation handoff. The broad Windows execution stop,
separate fuzz no-retry hold, storage-experiment pause, DoQ-only policy, complete
MSI, process/mandatory-state boundaries and UI-M0 deferrals remain in force.
No tool/dependency/provider, hardware timestamp path, new analyzer profile or
distribution channel is admitted. New handoff edits remain unstaged.

Local documentation checks and the new combined-candidate review follow below;
neither this handoff nor its review establishes implemented/native behavior.

### 4.3 Combined local-candidate review and validation

**3 November 2026:** `windows11_adoption_final_review`, delegated read-only with
`fork_turns: "none"`, completed the combined routing review. Verdict: **ready for
this source-only adoption stage**, with no material findings. Its comprehension
is retained verbatim:

> The change makes the imported Windows 11 research discoverable through startup instructions → development §7/§8.5 → the existing W11 task. W11-W01 is a bounded source-only assignment for the acquisition owner; W11-W02–08 retain their phase prerequisites and Windows hold, W11-W09 remains deferred, and W11-W10 remains conditional. Publishing this routing does not claim acknowledgement by the separately running AI.

> Current execution remains governed by the existing Core task and holds. The §7 S150 pointer matches the actual task: the fitted-anchor bridge has a portable development pass, with projection qualification still outstanding. The ST-W queue, Windows stop, separate fuzz hold, storage pause, DoQ-only policy, complete MSI and UI-M0 boundaries remain intact. Optional APIs, providers and dependency graphs remain unselected.

The four-file pre-receipt manifest SHA-256 is
`910226768a20117df4f996e677941f4b3ba3faf9db62f1df217da80caab722d5`.
It binds `AGENTS.md`, `docs/README.md`, `docs/plans/development-plan.md` and this
task before adding this subsection. All four file hashes matched during review;
this factual receipt is a subsequent addition, not a reviewed implementation.

Local checks resolved 796 link/anchor occurrences across the ten imported/current
documents, found no conflict markers and passed routing whitespace checks.
All 117 nonoverlapping staged entries retain their original blob/mode/stage;
three overlapping documents were merged additively. The staged path set remains
120 and the preserved post-pull index is unchanged. Concurrent Core work remains
owned by its existing agent. No product build, analyzer campaign or Windows
operation was run for this pull/adoption task. No root documentation checksum
manifest is present. New routing edits remain unstaged.

Continuation remains the coordinator acknowledgement and owner/disposition/next
action record in section 4.2, alongside the active S150 qualification. No W11
implementation or native result is credited by this documentation review.

### 4.4 Microsoft Learn second-pass integration — 3 November 2026

David requested the completed second investigation be entered into documentation.
The source investigation compared the published `f163c193b0bb4167ec5c39e16b1db1b581509750`
baseline. This integration instead starts from current GitHub main
`45ff247d84e5e43bd13f54de1c84aa13ce2e6efd`, tree
`6fccf094d8eae9d8dbccfb406afed0e08e2b11f9`, after the intervening code/test and
local-adoption commit. It uses an API-derived partial snapshot, not a local Git
checkout; earlier index/staged-state observations in sections 4.2–4.3 are
historical and do not describe this workspace.

The [follow-up source record](../docs/plans/windows11-integration-follow-up.md)
retains the Microsoft Learn MCP evidence, engineering adaptations and research
review corrections. The assignments above now include timestamp configuration
notifications, ancillary-data/ABI boundaries, conditional socket completion,
COM ownership, service-state lifetime, declarative MSI ACLs, SCM subscriptions,
native connectivity hints, DNS completion/policy/field preservation, ETW
controller/access separation and collector-specific privacy. `ReplaceFileW`
post-failure reconciliation stays with ST-W02/03. VBS key claims, side-channel
fields, UDP batching and VM generation remain conditional reserves.

This extends existing work items and acceptance ownership rather than adopting
those APIs or opening another task chain. Documentation contradictions remain
explicit selected-SDK/binding questions. Reuse the completed research when the
acquisition owner prepares the one W11-W01 source card; do not count this import
as that card's execution or restart a general survey.

The current portable Core checkpoint in development §7, imported ST-W queue,
Windows execution stop, separate fuzz no-retry hold and storage pause remain.
W11-W02–08 retain their named phase/prerequisite gates; W11-W09 retains D2/D3 and
W11-W10 its specific re-entry triggers. DoQ-only pins, complete MSI, selected
service roles and separate mandatory clock state remain. The source/fit review
from the investigation does not qualify these edited files; integrated review
and documentation checks are recorded separately after the candidate is frozen.
No product, native, SDK/ABI, MSI, security-tool or performance result is claimed.

### 4.5 Second-pass integrated review and documentation validation

**3 November 2026:** `/root/wlearn_docs_independent_review`, delegated read-only
with `fork_turns: "none"`, completed review of the frozen twelve-document
candidate. Verdict: **ready for this documentation integration stage; no material
findings**. It checked all introduced clauses and the full new supplement against
surrounding canonical owners, raw Microsoft evidence and current task boundaries.
A separate fresh-context helper checked the DNS sources; three targeted complete
Microsoft pages resolved missing raw-page coverage. No broad survey or native
execution was repeated. Its two comprehension paragraphs are retained verbatim:

> The proposed change incorporates the second Microsoft Learn investigation into NevroTick's maintained documentation and its existing W11 and storage assignments. It adds specific API transitions, source discrepancies and future healthy/refused cases to the existing technical and qualification owners. It does not select the optional mechanisms, execute the source cards or turn research into implementation evidence.

> The relevant current system has portable Rust Core work in progress while Windows execution, the separately held fuzz runner and storage experiments remain stopped. The proposed production design separates the network-facing engine, narrow clock broker, ordinary-user UI and lifecycle helper within one complete MSI. Optional NTS/DNS persistence remains separate from mandatory clock continuity; future DNS retains DoQ-only upstream with leaf/intermediate pins, and UI-M0's D1/D2/D3 work remains deferred.

The reviewed manifest SHA-256 is
`2654e3b5c2b5afea59a18ba0fc1c5566c6e9cd87882dbe2ccd2992d84983768e`.
The completed raw review SHA-256 is
`c3c99b0aa9af4ae1adf9b58fa4bcb076f00bd50d157345914dc1c9e412eeb3f1`.
The reviewer independently matched each candidate's byte count, SHA-256 and Git
blob hash. The following hashes identify the pre-receipt files; this factual
subsection is a later addition to the task, not a reviewed implementation.

| Reviewed path | Pre-receipt SHA-256 |
| --- | --- |
| `AGENTS.md` | `cbb15ca20107b4dc586fb872fce79c2d1c3a6ce619cee99fa5bad766572fb854` |
| `ROADMAP.md` | `57260b38b9fa18cb9f74fd91290b3d347e9733d1cfa325e5055b3e9dba42c006` |
| `docs/README.md` | `57a8cdfb47c2c83343849e836e83065093d38a7dcd9476fa5438aacd7b5992ff` |
| `docs/plans/development-plan.md` | `cab10179b904226fc2ada8ec438441b0308eb218ab32ace1f2bbfcae65f9d6b9` |
| `docs/plans/system-dns-cache-design.md` | `58ae3422fff3bd7a6dfc00ddb9029dddbc35ff2335b7f535320cb756a0a9fce3` |
| `docs/plans/windows-protected-storage.md` | `25d54e4853543165ddb17ba428b99b00d3dea7ca40f5212b4e421705cdc9adb8` |
| `docs/plans/windows11-integration-follow-up.md` | `bf23ebd5c0ca126a6dd2d45f494aa509dc89a1c2440dd27c737925f5a0c1f3f1` |
| `docs/plans/windows11-integration.md` | `42c51e3d28de10e6ae1872260bd0803a381a1425c2c05ba4e1d10ce195e9a478` |
| `docs/windows-time-architecture.md` | `6e05ae398ecacc7d3ad2337f3174b1af1f99f3877b2ad08a22eefc3c9c566c56` |
| `docs/windows-time-development-hardening.md` | `063db4884a4ff9a158d83739bdd102a9e0fc75d2ad6d48d4e2832fdd1dbb722a` |
| `tasks/windows-protected-storage.md` | `9141911910a0d45bdd4476b9ebf2b7eea1b8bce622edac529167e5cc39944e3d` |
| `tasks/windows11-integration.md` | `3f24a8c4f093ee891a2c959cc9190c48a8be8112305fab7e8bbc9aeb4ab6e24c` |

The coordinator's scoped documentation check passed all 54 distinct introduced
local source/target/anchor combinations, checked the new supplement's 53 used
Microsoft source references and found no unresolved reference, conflict marker
or invalid added whitespace. The reviewer independently resolved 71 introduced
local-link occurrences covering those 54 combinations. These counts describe
the frozen candidate before this receipt; they are not full-repository or remote
availability checks. All 53 source URLs are distinct Microsoft Learn pages.

Both checks preserve all 27 CHECK rows, 18 DNS-C rows and 18 ST-T rows, the whole
current development §7, and the W11 task §§4.1–4.3/storage task §§6.1–6.4 historical
receipts. Current `main` was rechecked at `45ff247` before review. No root
`docs/SHA256SUMS.txt` exists; no bundle-integrity result is claimed. The candidate
contains only the twelve Markdown paths above, including one new source supplement.

Review scope excludes unrelated old claims, actual SDK headers, application
implementation conformance and all native service/filesystem/COM/ETW/DNS/MSI
behavior. The source discrepancies and selected-build/provider questions remain
future evidence under their existing owners. No Rust build, analyzer/fuzz campaign,
Windows operation, package, clock, key or dependency change was executed here.

The coordinator rechecks the appended receipt and uses an expected-head,
non-force publication with exact path/blob verification. A substantive later edit
reopens only its affected review conclusions. W11-W01's source card, ST-W01 and
later implementation/native assignments remain unexecuted by this integration;
current Core work and every recorded execution hold remain controlling.

### 4.6 MSIX and Microsoft Store refinement — 3 November 2026

**Historical checkpoint:** the following request/disposition was current before
David's later decision in [section 4.8](#48-owner-selected-hardened-msix--3-november-2026).
Retain its research and evidence identity; its MSI-first/conditional-MSIX selection
is superseded, not the least-privilege, recovery or execution-hold requirements.

**Request and candidate:** record the completed MSIX/Store research in the
maintained docs, architecture, plan and existing agent task. Source integration
starts from `070927855ed99e1a0683d9f291d3462d33343118`, tree
`b3375bf865414e3c4958a33dd71de43c2e501ea5`. The partial API snapshot is matched by
Git blob identities; it is not a local checkout or a report of another agent's
dirty/staged state. The [source assessment](../docs/plans/windows-msix-store.md)
retains Microsoft Learn and actual SignPath evidence checked 2 November UTC.

**Disposition:** the named cost/distribution question makes one W11-W10
source-only card useful at the packaging checkpoint. It does not replace W11-W01
or development §7's portable Core work. One complete MSI, OD-05, current service
topology, acceptance catalogs and historical task receipts remain. All full-MSIX
gates in [architecture §19.4](../docs/windows-time-architecture.md#194-future-complete-msix-gate)
must pass before adoption. Research or documentation review proves no Store
approval, supported provisioner, effective token or native recovery behavior.

**Next bounded action:** the packaging/release owner prepares that single card
from the existing study, with W11-W05's lifecycle owner and security review.
Read `plan-architecture`, `security-build-deployment`, the testing contract and
the applicable service/privilege guidance. Record the exact proposed package,
known supporting method, unresolved dependency, owner and next evidence for
each row; do not restart a broad API or signing-provider survey.

| Card section / existing owner | Required source/design result | Later distinguishing evidence |
| --- | --- | --- |
| Publisher and Store admission / release | Actual account/country assumptions, manifest identity and requested service/elevation capabilities; draft precise §10.2.4 and capability questions for the complete utility. Free onboarding and Windows schema support are separate from eligibility. | Record a real response when contact is authorized; certification and account acceptance remain distinct from CHECK-18/23 application evidence. |
| Supported provisioning / platform/security | Map the current virtual-account mismatch. Name a supported way to establish required privileges, SIDs, owners/ACLs and network restrictions before service exposure and after repair/update. Compare a separate-process LocalService alternative only if it can meet the same boundary. | CHECK-1/15/16: legitimate work succeeds while engine clock authority, token/memory injection and neighboring-service data access fail under actual installed identities. Missing provisioning support leaves MSIX deferred. |
| Recovery and machine lifetime / W11-W05, clock, storage | Identify who quiesces work and settles rate/journal/W32Time recovery before the last required code or record disappears. Distinguish STOP, update, Settings uninstall, repair, Reset, last-user removal and delayed payload removal; no mandatory first UI launch or external installer. | CHECK-5/12/13/15/23/27: healthy maintenance and interrupted/reboot paths preserve current ownership, consumed authority, mandatory continuity and later administrator/GPO changes. Optional cache removal must not erase the only recovery path. |
| Installed trust / release/security | Define package/publisher identity, approved release, actual payload and helper/IPC binding. Keep catalog trust, embedded signatures and CIG policy distinct; carry the signing guide's wrong-clock/offline matrix. | CHECK-16/18/23: valid intended Store-delivered package works; tampered, unrelated valid Store-signed and unauthorized copied-out/old-release artifacts cannot gain helper authority. Actual dependency loading and recovery refusal must be observed. |
| Delivery and corrective updates / packaging/release | Keep a complete product. Compare Store Web Installer's online entry point, any specifically qualified offline path, initial private audience/flights and higher-version corrective release. `UpdateWhileInUse=defer` is conditional, not a cleanup guarantee. | CHECK-13/18/23: actual delivery/update preserves one controller and compatible state; listing rollback does not masquerade as downgrade of already updated machines or permit indefinite security-update delay. |

**Source-card exit:** a reviewer can distinguish established source contracts,
project-specific inferences, missing vendor/provisioning answers and each later
observation. A named unsupported requirement justifies deferring the channel,
not broadening service privileges or splitting the product. The card can finish
with an explicit unresolved external question; no invented approval is needed.

**Later exits:** first identify a concrete supported full-package design, then
qualify the actual intended Store-delivered artifact and supported Windows
builds under the existing phase/environment gates. Store certification is not
proof of application correctness. Normal administrative installation consent
remains; behavior before first application UI launch and before login must be
observed separately. Reuse the existing CHECK owners and this evidence record.

This documentation request authorizes no account creation, Microsoft/provider
contact, application upload, signing request or Store release. Source-only
preparation is permitted; current Windows execution STOP, separate fuzz no-retry
hold, storage pause and UI-M0/D1–D3 boundaries remain. Final documentation
review/validation is recorded separately below after completion; no code,
installation or product-qualification result is claimed.

### 4.7 MSIX documentation review and validation

**Completed independent review:** a fresh-context reviewer
(`fork_turns: none`) inspected the frozen ten-file candidate for the
documentation-integration stage and returned **ready, with no material
findings**. The review did not inherit the author discussion or retrieve private
memory. Its attributed comprehension paragraphs are retained verbatim:

> The amendment records what complete MSIX distribution through Microsoft Store could offer NevroTick: free package signing, catalog-backed content trust, hosted delivery and updates. It integrates that research into the existing architecture, signing guidance and W11 work queue, while making service admission, supported provisioning and recovery-before-removal explicit unresolved conditions. Its immediate continuation is one bounded source/design card at the packaging checkpoint; it does not select MSIX or authorize an experiment or submission.

> The current documented product has implemented portable protocol/Core owners and a restricted native UI prototype, with further Core qualification in progress. Its selected deployment and service design remains a complete per-machine MSI, a LocalService network engine, a separate virtual-account clock broker and an elevated lifecycle helper; these planned product boundaries are not established native deployment results. OD-05 retains conditional SignPath signing and an accepted initial OpenPGP alternative subject to its outstanding qualification gates. Windows execution, the separate fuzz retry, storage experiments and the specified UI extensions remain held or deferred.

The reviewed manifest SHA-256 is
`9041cb822f8b799238d691062d7bd5bb097edd68b60d49e90a633801c53365d8`.
The completed raw review SHA-256 is
`5688e866716a14b337b6580206d5ee1ff001b849415948a2c3a8efb380a6ba73`.
These identify the candidate before this factual receipt. The reviewer corrected
its own phrase about OpenPGP qualification to the wording above; candidate bytes
and the documentation-stage verdict did not change.

The reviewer independently matched all ten candidate SHA-256/Git blob identities
and the nine existing source blobs against `0709278`; the tenth document is new.
All 32 distinct changed/new local-link and anchor combinations resolved against
the complete pinned tree and retrieved target bytes. Consequential fees, account
policy, services, custom actions, catalog trust and lifecycle assertions were
checked against retained primary pages; this was not a fresh exhaustive retrieval
of every source or a native experiment.

The coordinator's scoped documentation check also passed those 32 combinations,
all 30 source-register identities and added-whitespace/conflict/heading checks.
All 27 CHECK, 18 DNS-C and 18 ST-T catalog rows, development section 7, accepted
owner decisions and prior W11/storage task receipts remain unchanged. Counts
describe the frozen pre-receipt candidate; the coordinator rechecks this factual
appendix before expected-parent, non-force publication and verifies the resulting
commit/tree/path identities. No root documentation checksum bundle is present.

Review covered the ten changed Markdown files and the relevant existing owners;
it excluded product-code auditing, an implemented package-aware verifier, actual
SDK/native deployment behavior and applicant/vendor approval. No source card,
MSIX build, Windows run, signing or release was completed by this amendment.
The next permitted packaging action remains the source-only card in section 4.6
when that owner is assigned; current Core work and execution holds remain.

### 4.8 Owner-selected hardened MSIX — 3 November 2026

**Request and authority:** David asked to maximize MSIX protection, then try
Microsoft Store publication with the services himself; after a refusal he will
reconsider delivery. [OD-05](../docs/nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership)
records this direct amendment. The integrated source/documentation baseline is
`661281ad0e6504338d352b943154522b800974fa`. Earlier receipts retain their original
candidates. Complete hardened MSIX is selected for preparation; no finished
package, active mitigation, supported account mapping or Store acceptance is
asserted.

**Changed and retained obligations:** MSIX preparation no longer waits for Store
preapproval or completion of an MSI release. David owns the later submission
and release action. MSI/SignPath/OpenPGP remain retained alternatives for a later
decision, with no automatic fallback. Preserve the complete product, separate
engine/broker authority, independent mandatory-state owner, W32Time restoration,
existing CHECK/AC/ST-T catalogs, Core priority, Windows execution STOP, separate
fuzz no-retry hold, storage pause and UI-M0/D1–D3 boundaries. No new acceptance
catalog, packaging tool, dependency or runtime policy is installed by this work.

**Next bounded action:** W11-W10's packaging/security owner produces the concrete
source/design profile below with W11-W05 and the release/lifecycle owners. Reuse
the [focused plan §§3–7](../docs/plans/windows-msix-store.md#3-package-trust-and-the-privileged-helper)
and existing per-process mitigation manifest; do not repeat the general source
survey. Read `security-build-deployment`, `security-windows-privileges`,
`qa-package-artifacts` and the testing contract before dependent implementation.

The subsequent source-profile result and next permitted action are recorded in
[section 4.14](#414-concrete-msix-source-profile--3-november-2026); retain this
assignment and its original evidence boundary.

| Existing owner / phase | Required output and independent expectation | Disposition and next evidence |
| --- | --- | --- |
| W11-W10, package/build; S0.6/P6 | Exact component/entry-point/dependency/capability inventory, future manifest requirements and immutable code/data layout. Explicit `PackageIntegrity` content enforcement; no mutable/external executable content or runtime-downloaded native code. | Source/design ready. CHECK-16/18/20/23/26 later inspect the actual final manifest/images/inventory and reject tamper while clean installation works. |
| W11-W10, platform/security; P0/P3/P6 | Per-role CIG with Store allowance, ACG/CFG/CET/image/extension/child/Win32k targets, activation owner/phase, required versus optional status and bounded failure/recovery. Distinguish SCM activation from child-process attributes; record pre-opt-in loads. | Source/design ready; implementation depends on actual roles/providers. CHECK-16/26/27 later require readback and harmless deny controls beside real permitted workflows; audit or unsupported is not enforced. |
| W11-W05, platform/IPC/security; S0.2/S0.3/P3 | Supported pre-exposure service provisioning, actual account/privilege/SID/owner/ACL matrix, process/token isolation, broker network denial and package-aware helper/IPC authorization. Neither account name nor a valid Store signer grants authority. | Mapping/provisioning remains unresolved; no LocalSystem shortcut or unqualified two-LocalService selection. CHECK-1/2/15 observe actual denied engine effects and accepted narrow operations after install/repair/update. |
| W11-W05, clock/storage/lifecycle; S0.6/P4/P6 | Recovery-before-removal protocol with explicit owner/generation, rate cleanup, journal continuity and W32Time restoration. Distinguish service STOP, update, crash, repair, Reset and last-user removal, including no first UI launch. | Supported recovery bridge remains unresolved. CHECK-5/12/13/23/27 later challenge interruption, forced shutdown and successor cleanup; no removal of the last recovery path. |
| W11-W10, release/build; P6 | Local candidate and certificate/trust evidence labelled separately from actual Store-delivered bytes. Prepare component purpose, capabilities, least privilege, consent, startup, update/removal and reviewer steps for David's submission. | Source notes ready; no prior Store permission needed to prepare them. CHECK-18/23 and affected CHECK-9 carry exact trust/time/offline/certificate-retrieval conditions and approved installed-helper identity. |
| W11-W10, release/independent QA; P6/P7 | Local complete mechanics → David's controlled/private initial Store attempt → exact delivered package/catalog/dependency/CIG and lifecycle qualification before broad release. Retain actual refusal reason if rejected. | Build/native/submission unexecuted. Existing CHECK-24 review reconciles evidence; certification is no security pass and actual Store signature is not required before the initial submission. |

The source/design exit is an implementable supported mechanism for each required
boundary, or a precise unresolved blocker with an owner and next action. A source
card may finish while external certification is unknown. It must not invent
provisioning support, lower required protection or mark a missing native result
as passed. Later stages follow
[plan §7](../docs/plans/windows-msix-store.md#7-bounded-continuation-and-existing-acceptance-owners)
and architecture §19.4 with their actual phase/environment prerequisites.

**This amendment's verification:** documentation consistency, preserved canonical
identifiers/history, current Microsoft primary-source semantics, relative links
and skill-pack routing. Product builds/tests, service/clock experiments,
machine-policy changes, account creation, signing and Store upload are outside
this executed amendment. Review and validation results are recorded below only
after checking the integrated candidate.

### 4.9 Hardened-MSIX documentation review and validation

The integrated amendment changes 25 maintained documentation/skill-routing files
against `661281ad0e6504338d352b943154522b800974fa`. The partial review checkout's
276 original files were matched to the pinned remote Git blob identities; the
complete remote tree was not truncated. It is not a claim about another agent's
local staged or dirty state.

Independent source review checked the selected plan and OD-05 against Microsoft
Learn, including explicit integrity enforcement, Store-compatible CIG, SCM versus
child-process activation and the noncircular Store-signing sequence. A separate
integration review checked architecture/features, task/phase routing and skills.
It found one remaining Authenticode-only P6 publisher-trust sentence; the
coordinator corrected it to the selected Store package/catalog profile and the
reviewer verified the fix. No material findings remained in the reviewed scope.
The coordinator separately inspected the signing/hardening changes.

Completed document checks:

- `python3 -B .agents/skills/scripts/check_skills.py`: **PASS**, all 62 skills'
  metadata, UI prompts, links and inventories. Imported upstream bytes/notices
  remain unchanged. The first snapshot preflight lacked linked files; actual
  pinned targets were fetched and verified before this final pass.
- Scoped documentation validation: **63 new local-link/anchor combinations**
  resolve; changed-file types, headings, fences and conflict markers pass.
  All 27 CHECK and 20 AC rows retain their identifiers; all 345 ASVS clause
  records are unchanged. The current Core next-task section and earlier W11
  review/history receipts remain unchanged.
- `git diff --check`: **PASS** after correcting added-line whitespace.
  No root documentation `SHA256SUMS.txt` exists; no bundle-integrity claim is
  substituted for it. The separate design-bundle manifests are untouched.

These are source/documentation results. No product build, native denial test,
service/clock action, real package/signature, account or Store submission was
executed. Section 4.8 owns continuation. Recheck this factual receipt, final
links and changed bytes before expected-parent publication; existing Windows,
fuzz and storage holds remain.

The following three local records preserve their original checkpoint facts.
Statements about S151/S152 work, staging and pending acknowledgement are historical;
use [development §7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
and its current task addenda for present Core status. Section 4.12 records the
Learn follow-up's subsequent coordinator acknowledgement. Its MSI-first and
conditional-MSIX dispositions predate the amended OD-05; section 4.8 now owns
selected MSIX preparation and supersedes those earlier packaging dispositions.

### 4.10 Local second-pass adoption — 3 November 2026

David requested the remote update be pulled and routed to the primary development
agent. Local `main` fast-forwarded from `45ff247` to `0709278`. At the pull,
eleven local Core/ledger files were preserved byte-for-byte; there were no staged
changes or overlapping paths. Existing Core work may continue under its owner.
The upstream integration review in section 4.5 retains its original candidate;
it does not qualify this local handoff.

The [startup guide](../AGENTS.md#start-each-task) and
[development §7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
route the Learn follow-up into this task and
[storage §6.5](windows-protected-storage.md#65-microsoft-learn-follow-up-routing--3-november-2026).
At its next checkpoint, the primary coordinator records each applicable W11/ST-W
owner, ready/held/deferred disposition and next action in those existing records
before dependent work. This is a discoverable handoff; acknowledgement by a
separately running agent remains pending.

W11-W01 and ST-W01 remain unexecuted source-only cards, ready when their owners
reach the relevant slice. The second pass refines the existing W11-W01–W11-W10 and
ST-W01–ST-W07 cases; it creates no parallel work-item set or new acceptance IDs.
W11-W02–W11-W08 and ST-W02–ST-W07 retain their phase/contract prerequisites;
actual Windows work remains held. W11-W09 remains deferred to D2/D3 re-entry;
W11-W10 and the storage reserves remain conditional. SDK/binding discrepancies,
per-call behavior and selected provider/build facts remain open evidence questions.

The existing Core task's S151 addendum records the current portable development
checkpoint and residuals-owner qualification still to be admitted. The
coordinator reconciles later Core addenda and current source before proceeding;
this handoff executes no Core test or Windows operation. The broad Windows stop,
separate fuzz no-retry hold, storage pause, mandatory-state separation, DoQ-only
policy, complete MSI and UI-M0 deferrals remain. No provider, dependency, API,
additional native profile or default is adopted by this routing. New handoff
edits remain unstaged.

Local documentation checks and a scoped review of this combined handoff are
recorded below; neither establishes product or native qualification.

### 4.11 Local second-pass handoff review and checks

**3 November 2026:** `learn_followup_final_review`, delegated read-only with
`fork_turns: "none"`, reviewed the combined four-file routing candidate. Its
verdict was **revise** for one pending factual receipt: then-numbered section 4.6 (now section 4.10) referred to
checks and review below before this subsection existed. It found no other
material routing discrepancy. This subsection supplies the requested receipt;
the reviewer did not inspect these later receipt bytes. Reviewer comprehension,
retained verbatim:

> The proposed handoff routes the second Microsoft Learn pass from startup guidance through development-plan §7 into the existing W11 and storage tasks. It asks the coordinator to record owners, dispositions and next actions at the next checkpoint; it does not claim another agent has acknowledged or executed the assignments.

> The current work remains portable Core S151, whose task records a development pass and pending residuals-owner qualification. W11-W01 and ST-W01 are unexecuted source-only cards; Windows execution, the separate fuzz retry and storage experiments remain held. The follow-up adds cases to existing owners without adopting APIs, providers, defaults or native profiles.

The four-file pre-receipt manifest SHA-256 is
`761f78ba67190dbcaaab68f8b43d9031f6a3811a9a51efc27f22883ef1bc6f22`.
It binds `AGENTS.md`, `docs/README.md`, `docs/plans/development-plan.md` and this
task before adding this subsection. The earlier section 4.5 source-integration
review retains its original, different candidate.

Local checks resolved 903 link/anchor occurrences across twelve imported/current
documents, found no conflict markers and passed whitespace checks for the routing
edits. At the pull, eleven local Core/ledger files were byte-identical after the
fast-forward, with no overlapping paths and no stash. The index was unchanged
when checked after routing. Concurrent Core work remains under its existing owner
and may advance independently. The root documentation checksum manifest is absent.
No product build, analyzer campaign or Windows operation was run for this handoff.

The coordinator's next action remains to record applicable W11/ST-W ownership,
dispositions and next actions here and in the storage task. W11-W01 and ST-W01
remain unexecuted source-only cards; separately running-agent acknowledgement is
not claimed. No API, provider, dependency or native profile is adopted here.

### 4.12 Primary development coordinator disposition — 3 November 2026

The primary P0–P7 coordinator has read development §7/§8.5, the imported work
queue and this task's §4.4/§4.10 follow-up. S151's later addenda now close its
selected Linux residuals gates through retained-data receiving; full P1/native
qualification remains open. The next pure fitted-tracking S152 journey consumes
no native packet timestamps, storage or Windows adapter, so these assignments
introduce no new prerequisite for that independent Core work. This is an actual
coordinator disposition, not execution of a source card or native API.

| Assignment | Owner and disposition | Next dependent action |
| --- | --- | --- |
| W11-W01 | Engine/platform/acquisition; source-only card ready, unexecuted | At S2.1/P2 bind exchange, socket generation, interface and conversion provenance; include second-pass ancillary, notification/callback and binding cases before a selected adapter. |
| W11-W02 | Platform/clock with DNS lifetime consumers; phase-gated, native held | Carry sleep/callback and awake/elapsed timer distinctions into S0.4/P4 and DNS-1/3/5. |
| W11-W03 | DNS/platform/lifecycle; phase-gated, native held | Freeze DNS-0 ownership, then DNS-1/3 listener/completion cases before native integration. |
| W11-W04 | Security/lifecycle/platform; phase-gated, native held | Carry effective network/service identity, delegated COM and refusal cases into the selected P0/P3/P6 row. |
| W11-W05 | Platform/security/lifecycle; phase-gated, native held | Carry effective SCM/process policy, MSI account/SID/ACL order and maintenance outcomes into their existing P0/P3/P4/P6 owners. |
| W11-W06 | DNS scheduler/resource; conditional on DNS-4/5 baseline | Compare bounded useful work and foreground reservations before adopting native hints; hints remain nonauthoritative. |
| W11-W07 | DNSSEC/crypto/performance; conditional on measured eligible DNS-2 queue | Preserve independent vectors, graph/provider admission and per-operation behavior before any batch API. |
| W11-W08 | Diagnostics/QA and reached component; phase-gated, native held | Admit one actual ETW/fault/session/retention row and coordinate shared privacy ownership with ST-W07. |
| W11-W09 | UI/accessibility; Deferred to D2/D3 re-entry | Restore the relevant real-snapshot/semantic/accessibility owners before P5 notification claims. |
| W11-W10 | Existing named reserve owners; Conditional | Re-enter only for the documented concrete need and existing decision/qualification gate; no new provider/profile selected. |

W11-W01 and ST-W01 remain unexecuted source-only cards. The broad Windows
execution stop, separate fuzz no-retry hold and storage-experiment pause remain.
The follow-up's SDK/generated-binding discrepancies require future actual
selected-row evidence; this acknowledgement grants no probing, live PowerShell,
service/clock/configuration effect, dependency or additional native profile.

**Combined local routing, 3 November 2026:** the earlier pending-acknowledgement
statements in section 4.10 are historical; section 4.12 records the later primary
coordinator disposition. The subsequently imported MSIX/Store assessment in
section 4.6 originally identified a conditional W11-W10 source card. That
MSI-first/conditional-MSIX disposition is superseded by section 4.8 and amended
OD-05. Historical review hashes retain their original candidate scope; no prior
review or coordinator acknowledgement proves adoption of the later amendment.

### 4.13 Local integration of the selected MSIX direction — 3 November 2026

This pull combines the owner-selected MSIX amendment `01bcb67` with the local
Core checkpoint `79d614f`. The local Learn records moved from sections 4.8–4.10
to 4.10–4.12 to preserve the published MSIX anchors. The retained Chrony ledger's
historical Windows-task section 4.10 therefore refers to the acknowledgement now
in section 4.12; its recorded bytes and evidence identities are unchanged.
W11-W10's packaging/security source/design profile in section 4.8 is ready at its
named phase; W11-W05's supported service provisioning
and recovery mechanisms remain unresolved. At the next coordinator checkpoint,
record adoption, owners and next actions for that amendment here before related
implementation. Section 4.12 acknowledges the earlier Learn follow-up only;
acknowledgement of the later MSIX amendment is not claimed. Preserve the current
Core priority, Windows execution stop, separate fuzz hold, storage pause and
UI-M0 deferrals. This integration executes no package, native or Store action.

### 4.14 Concrete MSIX source profile — 3 November 2026

**Request/candidate:** David asked to continue the selected MSIX work. This
source/design continuation uses `01bcb679cdf09720a1ae54dfdcae25b0864f4d77` and
the exact four changed plan/architecture/hardening/task files. The isolated
partial review checkout's 276 starting files were matched to that commit's
complete untruncated remote tree before edits. Source-profile date is
3 November 2026, Asia/Tbilisi.

Before publication, main advanced to `1becd9561f9de89d2a61bcabb0d2afc174bee49c`.
Its Core changes, updated startup/development guidance and task sections
4.10–4.13 were retained; the new source card moved to section 4.14. The three
other edited documents and inspected Cargo manifests/flags were unchanged
upstream. This is the packaging owner's source result; it does not claim the
primary Core coordinator has adopted or executed the later native assignments.

**Completed source work:** [plan §7.1](../docs/plans/windows-msix-store.md#71-concrete-source-inventory-and-manifest-profile)
records all six workspace members, actual UI/observer entry points and direct
dependencies, absent production broker/helper/service entry points, manifest
fields, capability dispositions and immutable-code/state boundaries.
[Section 7.2](../docs/plans/windows-msix-store.md#72-concrete-per-role-activation-record)
maps SCM, package UI and elevated-helper creation to supported initialization
and remaining creation-time gaps. Full primary-source reads and pinned Microsoft
reference XSDs supply the facts; no public documentation search was treated as
Windows behavior.

**Additional protection:** [Trusted Launch](../docs/plans/windows-msix-store.md#311-trusted-launch-for-the-supported-package-profile)
joins the strongest supported-profile target. Its documented build floor is
26100, it also requires explicit package integrity, and it limits executables
running under package identity to signed-catalog entries. It does not replace
CIG or project authorization. SDK/schema validity, the chosen OS matrix and
actual UI/SCM/helper identity/launch observations remain pending. The public
example's lowercase `uap10:content` and incomplete value declaration are retained
as source defects to avoid copying, not silently repaired platform evidence.

| Assignment / existing checks | Current disposition | Exact remaining work |
| --- | --- | --- |
| W11-W10 package/build; CHECK-16/18/20/23/26 | Source inventory and field-level profile prepared | Real production payloads, publisher identity, selected SDK/tool inputs and supported OS/CPU rows are still needed for a complete package. No dummy manifest or tool installation was added. |
| W11-W10 mitigation; CHECK-16/26/27 | Source activation map prepared; native enforcement unexecuted | Qualify each actual creation path and module closure. The runtime `MitigationOptIn` documentation includes Microsoft/Store/WHQL; no equivalence to the other CIG configuration layers or untested bit combination is adopted. |
| W11-W05 provisioning; CHECK-1/2/15/16 | Concrete complete-package bootstrap/order gap remains | The service schemas lack the required account/security configuration fields; SCM setters need an authorized owner. Establish pre-exposure token/SID/owner/ACL state and servicing continuity. Two LocalService services remain an unselected alternative. |
| W11-W05 recovery; CHECK-5/12/13/23/27 | Concrete independent-removal gap remains | Package events, STOP, a surviving journal and helper-initiated removal do not prove recovery for Settings/Reset when the service is stopped. Identify a supported barrier or surviving invocable recovery path. |
| Release; CHECK-18/23/24 | Capability/reviewer input table prepared; submission unexecuted | Fill with actual candidate identities and observed outcomes, then David owns the controlled Store attempt. No advance certification prerequisite or automatic MSI fallback is introduced. |

**Next permitted action:** carry these precise supported-mechanism questions
into W11-W05/S0.6 and the existing platform/lifecycle owners; implement only after
the dependent contract and actual role are ready. The
[bounded later experiment](../docs/plans/windows-msix-store.md#73-bounded-blockers-and-the-first-later-package-experiment)
first distinguishes running-service removal from removal after stop/crash,
with simulated recovery and no clock effects. A separate administrative-harness
probe can test SCM building blocks, but cannot qualify the production bootstrap.
Both native experiments remain held by the owner-recorded Windows execution
stop. Portable Core priority, separate fuzz no-retry hold, storage pause and
UI-M0/D1–D3 deferrals are unchanged.

This completes the assigned source card with precise open mechanisms and owners.
It establishes no implemented service, enforced mitigation, package build,
signing, Store submission or release readiness. Independent review and the final
candidate's documentation checks are recorded below after execution.

#### Source-profile independent review and checks

`msix_profile_review`, delegated read-only with `fork_turns: "none"`, completed
the independent review of the four-file source/design candidate. Verdict:
**READY for the source/design card only; no material findings.** Its
pre-receipt manifest SHA-256 is
`935c8510548a9859f1880ac28e237bdb402d41b3e8890100f995a0fe9801119a`.
The reviewer checked current project contracts and independently retrieved the
source inventory, pinned service XSDs and consequential Microsoft Learn pages.
Its comprehension paragraphs are retained verbatim:

> This continuation turns the selected complete-MSIX direction into a concrete source/design card. It inventories the existing workspace and missing production roles, gives the future manifest and activation owners explicit requirements, adds Trusted Launch as a supported-profile qualification target, and narrows the two principal lifecycle questions: how the package establishes service isolation before exposure, and how recovery remains executable through independent uninstall or Reset. It does not assert that those mechanisms are implemented or that the package is ready to build, submit, or release.

> NevroTick currently has six workspace members: a fixture-only Win32 UI, portable protocol and numerical owners, and a bounded observation engine with a CLI. Production broker, lifecycle helper, SCM service entry points, and a qualified complete package remain absent. The selected architecture still requires separate engine and broker authority, an unelevated UI, narrow elevated lifecycle work, and recovery before the last necessary code or journal disappears. David owns the Store attempt. Core work remains the priority, and the Windows, fuzz, storage, and UI deferrals remain in force.

The independent consistency pass and coordinator's direct checks established:

- Four intended Markdown files changed, with no unrelated source/configuration
  edits. Eight new relative-link occurrences resolve to their actual headings;
  fifteen new external source links and all source-register references are
  accounted for. All fourteen Markdown code blocks remain balanced.
- The 27 CHECK and 20 AC catalogue rows retain their exact content; the 345 ASVS
  records and worksheet are unchanged. Earlier W11 receipts, including the
  concurrent sections 4.10–4.13, retain their original content. Removing only
  this continuation, the status update and section 4.8 pointer reconstructs the
  exact upstream task. Current Core section 7 is unchanged by this amendment.
- Both XML fragments parse with their declared namespaces and the documented
  element casing/values. This is XML well-formedness and source agreement, not
  Windows SDK schema validation, package construction or enforcement evidence.
- `git diff --check` passed. The local snapshot's 276 baseline blob identities
  and modes were compared with the full untruncated `1becd956` remote tree.
  A transient local-only development-plan truncation was detected and repaired
  before the final comparison; all 276 then matched. The remote file was intact
  and is not changed by this amendment.

The review above covers the frozen pre-receipt source card. This factual receipt
was added afterward and receives final link/whitespace/content checks before
publication. No product or native gate is closed by these documentation results.
The next action and existing execution holds remain those recorded in section
4.14 above.

### 4.15 Provisioning and independent-removal research — 3 November 2026

**Request and baseline:** David requested deeper research into the unresolved
service provisioning and independent-removal questions, then explicitly asked
to retain that research in English in the repository documentation and tasks.
The publication basis is `474698543a07a69830d606388c5c2539e9a4e2ec`; the isolated
276-file review snapshot was matched to that commit's full remote tree.
This is a documentation handoff, not permission to adopt an alternative topology
or resume native work.

**Canonical continuation:** [MSIX plan §7.4](../docs/plans/windows-msix-store.md#74-service-provisioning-and-independent-removal-research)
owns the facts, candidate comparison, residual boundaries and bounded next
observations. Sections 5 and 7.1 of that plan now classify custom actions as
blocked for the ordinary utility scenario under the published grant policy,
rather than merely an unspecified partner possibility. An explicit applicable
Microsoft exception could change that disposition; the technical custom-
capability name alone does not.

**Completed research:** full Microsoft Learn reads establish the ineffective
SCM database lock, manual-service trigger possibility, StartService/readiness
distinction, configured-versus-effective privilege/SID state, restricted-child
creation and permanent privilege-removal tradeoff. Lifecycle findings refine
final stopped acknowledgement, delayed restart, same-account failure commands,
package/SCM notification limits, state deletion and W32Time trigger/task startup.
The source register retains S01–S64 and adds S65–S87. No native outcome is inferred.

| Existing assignment / checks | Disposition after this research | Next dependent action |
| --- | --- | --- |
| W11-W05 provisioning; CHECK-1/2/15/16 | Supported API building blocks; production owner/order still unresolved | Compare initially disabled services plus explicit elevated activation with a short-lived automatic LocalSystem bootstrap. Record activation UX, authority, Store capability and servicing tradeoffs through the existing owner-decision process before implementation. Neither is selected. |
| W11-W05/W11-W10 engine isolation; CHECK-1/2/15/16/26/27 | Restricted engine child is an unselected topology alternative | Freeze creation token/attributes, effective isolation, broker rights/networking, package identity and restart/lifetime contract before any native prototype. Do not add launcher privileges or adopt AppContainer by implication. |
| W11-W05 lifecycle/storage; CHECK-5/12/13/23/27 | Independent Settings/Reset/final-removal recovery remains unresolved | First identify a supported pre-removal barrier or retained authorized invocation. Challenge running-service versus stopped/crashed-service removal with an inert journal; a surviving record alone is not a recovery mechanism. |
| W11-W05 maintenance; CHECK-5/12/13/23/27 | Source-grounded delayed-restart and refused-update cases prepared | Include already queued restart during maintenance, cleanup before stopped acknowledgement, and version B refusing admission while version A's W32Time takeover remains. Safe refusal must not be confused with restored availability. |
| Release/W11-W10; CHECK-18/23/24 | Custom-action route blocked under ordinary published eligibility | Keep it outside the normal implementation route unless applicable Microsoft exception evidence appears. Preserve David's selected Store attempt with services and the absence of an automatic MSI fallback. |

An OS-owned scheduled recovery task remains a visible, unselected alternative.
Its external privileged state, secure executable/data binding, successor checks,
large-clock-jump behavior and eventual self-removal require a separate complete
contract. A fixed service-control command does not implement the existing clock
ownership/journal protocol. This research does not accept a hidden watchdog or
relax recovery before the last usable mechanism disappears.

**Next permitted work:** retain these findings with W11-W05/S0.6 and the existing
platform, security, lifecycle and release owners. Resolve the applicable owner
choice before dependent implementation; independent Core work continues. The
existing section 7.3 removal pair remains the first native discriminator after
its prerequisites and execution hold are resolved. Provisioning interruption,
update/repair, restricted-child restart and queued-restart cases are bounded
follow-ups, not an instruction to launch a full matrix now.

David also requested research into an isolated Windows experiment environment
for agents developing through WSL on a Windows 11 computer. That request is
environment-design work: it does not authorize host clock/service/Defender
changes, retry the blocked fixture or lift the current Windows execution stop.
Keep the separate fuzz no-retry hold, storage pause and UI-M0/D1–D3 deferrals.

**Reconciliation:** amended OD-05 and the complete-MSIX objective remain; all
prior source/native receipts retain their recorded candidates. The new material
narrows public capability availability and adds supported facts, candidate costs
and counterexamples. No account topology, new persistent privileged role,
dependency, implementation, package build, signing or Store action is adopted.
Independent review and final documentation-check results follow after execution.

#### Lifecycle-research independent review and checks

`msix_research_writeback_review`, delegated read-only with `fork_turns: "none"`,
completed the independent review. Verdict: **READY for the English MSIX research
documentation handoff only; no material findings.** The reviewer also used a
fresh-context, read-only verifier for the bounded custom-action, token and
service-activation claims. Its comprehension paragraphs are retained verbatim:

> This change retains the completed provisioning and independent-removal investigation in the existing MSIX plan and Windows task. It tightens the ordinary utility custom-action disposition, compares explicitly unselected provisioning and engine arrangements, and records concrete service-state, recovery-lifetime and servicing counterexamples. The deliverable is a source-research handoff with existing owners and bounded later observations; it does not select a topology, implement a package, or establish native behavior.

> The current authoritative design is one complete hardened MSIX prepared for David’s Store attempt, preserving separate engine and broker authority, an unelevated UI, narrow elevated lifecycle operations, and recovery before the last necessary mechanism disappears. The supplied repository record has portable protocol/Core work and fixture or observation components; production service/helper and complete-package qualification remain outstanding. Current Core work, the Windows execution stop, separate fuzz no-retry hold, storage pause and UI-M0 deferrals remain in force.

The frozen pre-receipt manifest SHA-256 is
`dc5453c78678d015805ea7e61840b3d927bbd38fc41b887ea699ad79346426cd`.
It identifies the reviewed plan blob
`58489c8fae3fb5155cf8723df89aa0e4a051f794` and task blob
`25a8eabf7339523a40568e0be7494037ce4838c5`, before this receipt was added.
The reviewer matched those bytes and their baseline blobs to the supplied local
review snapshot; the coordinator separately matched all 276 baseline files and
modes to the full untruncated remote `4746985` tree. The partial local snapshot
is not publishable remote history.

The independent review and coordinator's documentation checks established:

- Exactly the two intended Markdown files changed, with no mode or executable
  changes. `git diff --check` passed; the plan's two fenced blocks are balanced.
- S01–S87 are unique and contiguous, with no undefined source references.
  The S01–S64 register entries are byte-preserved. Both new cross-document
  links resolve to the actual section 7.4 and section 4.15 headings.
- Removing section 4.15 and restoring only the declared status-line update
  reconstructs the previous task body and all its earlier receipts exactly.
  The 27 CHECK rows, 20 AC rows and 345 ASVS records remain unchanged, as do
  the account, lifecycle, owner-decision, Core-priority and execution-hold
  contracts.
- Targeted primary-source reads support the custom-action eligibility boundary,
  configured-versus-effective service state, restricted-child qualifications,
  failure-action timing and removal-notification limits at the stated scope.
  Proposed topologies, activation UX and a scheduled recovery task remain
  explicitly unselected; no source statement substitutes for native evidence.

The review verdict covers the frozen pre-receipt candidate. This factual receipt
was appended afterward and checked separately for exact quotation, scope,
relative links, whitespace and preservation before publication. No build,
package operation, Windows execution, host change or Store action was performed.
Section 4.15's next permitted work and existing execution holds remain in force.

### 4.16 Windows Sandbox management skill — 3 November 2026

**Request and candidate:** David requested a concise English skill for agents
managing Windows Sandbox, with extensive researched references stored in this
repository. The source baseline is
`3c408bac7e9c8fb3970628748ffda7a9923eb0e7`, root tree
`95dfba40ccb5b9e90843aa7a881eb93a4f69d180`. The isolated review snapshot's
277 existing files and modes were matched to that full, untruncated remote tree;
its partial local Git history is not the publication history.

**Deliverable:** [manage-windows-sandbox](../.agents/skills/environment/manage-windows-sandbox/SKILL.md)
provides a short operating procedure and selective access to six detailed
references: native operations, configuration/boundaries, resources/WSL, optional
winapp automation, NevroTick evidence, and troubleshooting/closeout. UI metadata,
the local-skill inventory, agent routing, skills index, environment source index
and documentation guide make it discoverable. It is an original instructional
workflow, not an imported skill, installed host tool or executable controller.

**Source work:** Microsoft Learn full-page reads and code-sample retrieval were
checked against released Microsoft winapp v0.7.1 documentation and relevant
implementation/tests at `e3cd0b94f3e7b776ac51ba0564f4908094e979b5`.
The references distinguish installed client support from OS version, native
control status from the raw JSON guest exit, and guest commands from host
operations. They retain documentation discrepancies rather than copying invalid
examples. A separately identified development revision is used only to explain
version drift; it does not supply v0.7.1 options.

**Operating decisions encoded as guidance:**

- Preserve active WSL work and admit a short guest only after real resource
  preflight. The owner's possible 22 GB usage during a four-day run is reported
  context, not a current measurement or memory-leak diagnosis. The 3072 MB XML
  profile is a proposal, not an adopted or tested laptop configuration; the
  documented interface supplies no CPU quota.
- Default the proposed native profile to offline execution with explicit device
  redirection settings, narrow read-only input and separate writable evidence.
  Guest disposal cannot undo writes to mapped host folders. Policy and observed
  effective behavior remain separate from requested XML.
- Own and serialize one guest, name host/guest command context and preserve
  results before explicit termination. The agent's existing WSL/host access is
  not technically confined by skill text. Neither a viewer window nor a target
  snapshot is a restorable system checkpoint.
- Treat optional winapp as a separate networking/bootstrap profile. Its released
  backend connects to the guest through TCP, while project evaluation/build can
  still run on the host. Explicit guest targeting, installed-version checks and
  actual principal/output evidence prevent unsupported success claims.
- Map observations back to existing W11-W05/W11-W10 and CHECK owners. An inert
  service check, local MSIX deployment, Store-delivered qualification and exclusive
  clock ownership require different evidence. Sandbox teardown is not successful
  product recovery or uninstall.

**Reconciliation:** all prior task text/receipts remain, apart from the status
line's pointer to this section. No product contract, CHECK/AC/ASVS definition,
account topology, package design, dependency or host policy changes. Amended
OD-05, Core priority, the Windows execution stop, separate fuzz no-retry hold,
storage pause and UI-M0/D1–D3 deferrals remain. The skill adds no blanket approval
round and does not interpret this documentation request as permission to execute
Windows experiments or retry the blocked fixture.

**Documentation exit and next action:** check skill inventory/UI metadata,
relative links/anchors, Markdown, example XML and preservation of predecessor
task history. Use a fresh-context reviewer and bounded read-only usage exercises
to inspect the actual instruction candidate. Those checks assess documentation
and agent decisions; they do not establish Windows runtime compatibility,
performance or isolation. Record actual results below when completed. After
publication, agents can invoke `$manage-windows-sandbox` for preparation; a
native smoke run remains dependent on current authorization, resolved applicable
holds, available resources and the exact admitted environment. Independent Core
work continues without waiting for that run.

#### Sandbox-skill independent review and documentation checks

`sandbox_skill_independent_review`, delegated read-only with
`fork_turns: "none"`, completed the review. Its verdict was:
**READY for documentation publication only. Review completed. No material
findings and no required correction.** Its two comprehension paragraphs are
retained verbatim:

> This candidate adds an original English `manage-windows-sandbox` skill with a short operating sequence and six detailed references. It teaches agents how to prepare an owned disposable Windows session, distinguish host control from guest execution, protect active WSL work, collect bounded evidence and end the correct instance. The surrounding changes register and route the skill through the existing repository documentation and append its delivery record to Windows task §4.16. The examples are prospective guidance; the candidate does not implement a controller, configure a host or report an executed Sandbox experiment.

> The repository contains portable protocol and time/Core work, a fixture UI and bounded observation/transport components. Its selected delivery direction is one complete hardened MSIX for David’s Store attempt, while production service provisioning, independent-removal recovery and final package qualification remain unresolved or unexecuted. The engine, broker, ordinary-user UI and lifecycle helper retain distinct authority. The Defender-related Windows execution stop, separate fuzz no-retry hold, storage pause, Core priority and UI-M0 deferrals remain controlling; adding a laboratory skill changes none of them.

The frozen 14-file pre-receipt manifest SHA-256 was
`fd08d4f1b0a9d516692670702614eb1ffaf449f229c9185c04715a48837f2b09`.
It included skill blob `ebb32160e6dc3b5d5c1ba1bd45444c53fe318b45` and
pre-receipt task blob `7cb0ade4c30ced548688f71ab4ed03394db06317`.
The reviewer matched every candidate hash/size/blob and all 277 local baseline
paths/modes/blobs to the supplied mapping. The coordinator separately matched
that mapping to the full remote tree; the reviewer did not independently
reconstruct the remote tree. The local partial baseline was
`81ee6b31b93676345ee484347418af36ae34b91c`, not a publishable remote parent.

The reviewer read the actual diff and every new skill file, applicable agent,
documentation, testing/review instructions, task §§4.15–4.16, MSIX §7.4,
incident controls and relevant architecture boundaries. It retrieved primary
Microsoft configuration/FAQ/WSL/policy sources and obtained a completed fresh-
context source verification of the pinned native CLI/winapp implementation,
tests and version distinctions. That source verifier found no material false
or unsupported statements in the two control references. The reviewer disclosed
that a live-agent listing exposed an author completion summary; it did not use
that summary as evidence or an expected verdict. No private memory or parent
conversation was supplied to the fresh review.

**Completed checks:**

- The required pack checker passed: **63 skills**, including seven original
  local workflows, valid metadata/UI prompts, inventories and local links.
  No checker implementation or imported skill/inventory was changed.
- `git diff --check` and all **84 new relative file/anchor links** passed.
  New Markdown files had balanced fences, UTF-8/LF text and final newlines.
  Coordinator-only assertions initially overclassified existing Markdown
  hard-break spaces and added an extra newline when reconstructing a reviewed
  prefix. Correcting those comparison scopes resolved the checking errors;
  the predecessor's formatting and reviewed prefix bytes were unchanged.
- The proposed XML parsed and its intended offline/3072 MB/Protected Client,
  read-only input, writable output and no-autorun settings matched static
  assertions. This is syntax/content evidence, not Windows configuration
  acceptance or measured effective policy.
- The previous task body reconstructed exactly after removing this section
  and its sole status-line pointer. Architecture, features/acceptance, ASVS,
  owner decisions, MSIX plan and incident control documents remained unchanged.
  The delta was eight new and six modified ordinary files, with no executable
  or mode changes.

**Bounded usage exercise:** `sandbox_skill_usage_exercise` used
`fork_turns: "none"` and the frozen skill/reference candidate. It read the skill,
selected all six references and relevant authority sections, with no coordinator
receipt, review manifest, private memory or other agent artifact supplied.
Actions were limited to reading and returning operator decisions/command text;
no scenario command was executed. The coordinator checked the actual responses
against the following supplied facts and independent scope/evidence expectations:

| Synthetic case supplied | Observed response and disposition |
| --- | --- |
| An admitted inert experiment needs a free lab; available RAM is 2.1 GB with sustained CPU pressure; an existing Sandbox has no owner record; a multi-day WSL job belongs to the user | Deferred launch, preserved the unknown instance and WSL job, required changed capacity/ownership conditions without a generic reapproval. Preflight remained incomplete. |
| Native host status 0, empty stderr, raw guest `ExitCode` 23, and a success file naming a previous run | Refused the pass, separated successful dispatch from the nonzero guest result, rejected stale evidence and preserved bounded collection/owned teardown. It correctly retained the possibility that an explicitly expected negative exit needs its own matching evidence. |
| Healthy neighbor: host/guest status 0, current run/artifact identity, three independently satisfied observations, complete bounded output and observed cleanup | Accepted only those three observations for that candidate; kept guest termination and wider product/release claims separate. |
| Only a prebuilt offline guest artifact is admitted; another project's host build is held; proposed v0.7.1 winapp project build uses an offline profile and a target snapshot for rollback | Rejected the proposed host-build/network/checkpoint assumptions; proposed the admitted prebuilt artifact through native offline control, conditional on real readiness and ownership. |
| A small pure Rust arithmetic change belongs to the current portable task | Did not create/probe Sandbox or add a Windows readiness gate. |

All four task scenarios, including the healthy neighbor within the result case,
returned the expected bounded decisions. This single read-only exercise is not
a runtime test, an old/new comparison, statistical reliability evidence or a
claim that the skill prevents all agent mistakes. Shared filesystem access was
not represented as an isolation boundary; no prohibited context access was
reported in the exercise's supplied read inventory.

This factual receipt was appended after the reviewed candidate. Final checks
cover its exact quotations, identities, links, whitespace and preserved scope.
No Windows/WSL operation, Sandbox launch, host change, product build, package
installation, signing or Store action was performed for this delivery. The next
permitted action remains publication of this documentation and subsequent
preparation under the current task's actual authorization and holds.

### 4.17 Primary coordinator adoption of the selected MSIX profile — 3 November 2026

This acknowledgement covers the earlier selected MSIX profile; it predates the
provisioning/removal research in section 4.15 and Sandbox guidance in section
4.16. Their next-checkpoint routing is recorded in section 4.18.

At the S155 pure Core checkpoint, the primary development coordinator adopts
sections 4.13–4.14 and the selected complete-MSIX amendment in
[OD-05](../docs/nevrotick-owner-decisions.md#71-accepted-choices). The existing
source card inventories the six-member workspace, missing production roles,
future manifest fields and role-specific activation owners. Its prepared source
profile does not qualify a package, service bootstrap, mitigation or Store result.
Earlier MSI-first and conditional W11-W10 entries in sections 4.10–4.12 retain
their historical scope; this later disposition governs current delivery work.
MSI remains an alternative requiring a later owner decision.

| Assignment and owner | Current disposition | Next dependent action |
| --- | --- | --- |
| W11-W10, package/build; S0.6/P6 | Selected source/design profile prepared; actual package build and native checks unexecuted | Bind real production payloads and entry points, publisher, selected SDK/tool inputs and supported OS/CPU rows before the final manifest and complete-package checks. Carry explicit content integrity, immutable executable content and the Trusted Launch candidate gates from section 4.14. |
| W11-W10, platform/security with each process owner; P0/P3/P6 | Source activation map prepared; effective enforcement unqualified and native held | Apply the existing UI/SCM/helper activation owners and module-closure contracts before related adapters; qualify required protections, Store-compatible CIG and healthy/refused workflows on the actual selected creation paths. |
| W11-W05, platform/security/IPC/lifecycle; S0.2/S0.3/P3/P6 | Supported pre-exposure service provisioning remains unresolved; native held | Identify the authorized supported bootstrap owner and freeze effective account, privilege, SID, owner and ACL order through servicing. No broader service privilege or unselected two-LocalService substitution follows from the source profile. |
| W11-W05, clock/storage/lifecycle; S0.6/P4/P6 | Recovery through independent removal remains unresolved; native held | Establish a supported barrier or surviving invocable recovery path for Settings/Reset and removal after service stop/crash, including owner/generation, journal continuity and W32Time restoration. Preserve recovery before the last required code or state disappears. |
| W11-W10, release/build/independent QA; P6/P7 | Source justification ready at its named phase; build, submission and delivered-artifact qualification unexecuted | Prepare the complete candidate without speculative Store preapproval; David owns the controlled Store attempt. Independently qualify actual delivered bytes and lifecycle before broad release; an actual refusal returns to the owner decision. |

The storage task's ST-W01–ST-W07 and this task's W11-W01–W11-W09 keep their
existing owners, prerequisites and acceptance identities. ST-W01/W11-W01 remain
unexecuted source-only cards, and D2/D3 work remains deferred. S155 consumes no
Windows package, persistence, timestamp or service adapter, so this adoption
adds no prerequisite to its current portable Core work. The Windows execution
stop, separate fuzz no-retry hold, storage pause and Core priority remain in
force. This acknowledgement executes no new source card, native probe, package
or Store action and admits no dependency, tool or additional native profile.

### 4.18 Local routing of provisioning research and Sandbox guidance — 3 November 2026

This pull imports `3c408ba` and `4d8539b` into the active Core checkout. Sections
4.15–4.16 and MSIX plan section 7.4 retain the new source findings and Sandbox
skill documentation. The existing primary-coordinator acknowledgement moved
from section 4.15 to 4.17; its original scope and recorded facts are preserved.
At the next coordinator checkpoint, reconcile these refinements under existing
W11-W05/W11-W10 owners before related work; primary adoption of this later
research and skill handoff is not claimed by the pull.

Supported provisioning and independent-removal recovery remain unresolved.
Ordinary utility custom actions are blocked under the published grant policy
absent an applicable Microsoft exception. Disabled-service activation, temporary
bootstrap, restricted-child and scheduled-recovery alternatives remain unselected.
Retain the complete-MSIX direction, David's Store attempt and no automatic MSI
fallback. The later native discriminator remains the bounded inert-journal
running-versus-stopped/crashed removal pair after its prerequisites are met.

The Sandbox skill supplies source guidance; this integration runs no Windows
preflight, installation, guest, package or blocked-artifact retry. Preserve Core
priority, Windows execution STOP, the separate fuzz no-retry hold, storage pause
and UI-M0 deferrals. Guest disposal establishes no product recovery result.

**Primary coordinator reconciliation, 3 November 2026:** read the later
sections4.15–4.18 at the completed S155 checkpoint. W11-W05/W11-W10 retain the
provisioning, grant-policy and independent-removal findings for their next
source/design action; no alternative service activation or recovery mechanism is
selected. The Sandbox guidance remains documentation, with native management and
the running/stopped/crashed removal discriminator held behind their existing
prerequisites and Windows STOP. No new dependency or execution permission follows.
The requested compiler-lint cohort and existing Core work remain the active work.

### 4.19 Owner-ready Sandbox scope — 3 November 2026

David now reports Windows Sandbox ready for use and points to the prepared
management skill. This is scoped authorization for Sandbox management/readiness
and later admitted guest work; retain the Defender block and do not retry the
held S134 stager/artifact, the separate AES fuzz runner or paused storage work.
Host management is distinct from guest payload execution. Begin with bounded,
read-only installed-CLI/version/inventory and resource observations; identify one
owned session before connect/share/exec/stop. No host elevation, installation,
security/service/clock changes, broad writable shares or encoded PowerShell.
The first readiness observation is an environment check, not a product CHECK
pass. Preserve active WSL jobs and Core priority. A guest run requires recorded
identity/configuration, finite time/output/resource bounds and explicit targeting;
an unrelated existing guest remains untouched.

**Executed readiness result:** passed the bounded built-in smoke on Windows 11
Pro x64 build 26300 using installed native Sandbox CLI `0.8.107.0`. Owner `/root`
created fresh instance `074e6da9-7182-411e-8208-ef9bb5116e8e` after observing an
empty inventory and reconciled its returned ID. Recorded start/exec/stop
arguments target that ID.
Direct WSL invocation of the registered `wsb.exe` alias avoided a host shell for
start/exec/stop. The requested XML disabled networking, vGPU, clipboard, printer,
audio and video input, enabled ProtectedClient and capped memory at 2048 MB;
there were no shared folders or LogonCommand. Effective device/network protection
was not measured. Windows available physical memory was 3967 MB before launch
and 2827 MB after launch; active WSL work was preserved.

Three `ExistingLogin` guest CMD payloads returned expected guest exits **0, 42,
0**: success, intentional failure and the default `WDAGUtilityAccount` username
check. Each host dispatch exited 0, including the guest exit-42 control; host
success was not treated as guest success. The username check establishes no
standard unelevated token or complete identity qualification. The 18.06-second
run used 12 native management commands plus connect, stopped only its owned guest
and verified empty inventory. A final independent version/inventory bookend also
found no guest; the connect controller had exited 0.
The connect controller's PID and exit were retained, but its argv was omitted;
its exact ID binding cannot be independently reconstructed from these JSON
receipts. Retain connect arguments in the next run's existing receipt.

Evidence directory:
`/home/david_osipov/.local/share/nevrotick/windows-sandbox-readiness-20261103-gb2ewnm6`.
`owned-readiness-result.json` SHA-256:
`59cab05ce4d07d5bda6568f4e0226fa432a874e2777bef217a5be0ff733b2719`;
`final-preflight-bookend.json` binds that result and final inventory. Related
intent, progress, CLI-help and resource receipts remain in the same directory.
No NevroTick executable or held artifact was shared or run; no PowerShell,
installation, host configuration/security/service/clock/trust change, package
operation or product CHECK qualification occurred. Next: retain Core priority;
use this skill and fresh preflight for a separately admitted bounded guest
experiment when its product owner and prerequisites are ready. The blocked
stager/artifact, AES fuzz hold and storage pause remain unchanged.

Independent read-only receipt review found no substantive control defect and
confirmed the result hash, explicit start/exec/stop target, guest exit controls
and final absence. The connect-argv omission and protection/token evidence gaps
above remain explicit. Scoped staged/unstaged whitespace checks passed; all 370
local links/anchors across this task, the documentation guide and development
plan resolved. `docs/SHA256SUMS.txt` is absent; no bundle-integrity claim follows.

### 4.20 Selected leap-policy integration — 4 November 2026

[NV-LP-001](../docs/plans/leap-second-policy.md) selects silent-source handling,
independent leap knowledge, kernel-only explicit event execution and the bounded
guard/residual-recovery contract. This amendment replaces W11-W02's open process-
presentation choice with default compatibility mode, without claiming an adapter,
native event-table interface or completed Windows qualification.

| Existing owner / stage | Disposition and next action |
| --- | --- |
| W11-W02, platform/clock; S0.4/P4, CHECK-10/11/12/27 | Carry the selected mode, precision, actual native-event/trajectory and guard cases above into the owned clock fixture after its prerequisites. Native product execution remains unqualified; the S1.4 task owns pure policy work after S1.3. |
| W11-W10, package/build/release; P6, CHECK-18/23/26 | Bind the reviewed immutable compiled calendar and effective-dated policy to the same authenticated complete release. Record source/generator/revision/digest identity in submitted and delivered provenance; no downloader, hot file path or TUF adoption. |

The [package plan](../docs/plans/windows-msix-store.md#31-package-integrity-and-sealed-executable-content),
[signing guide](../docs/windows-time-signing-guide.md#61-store-msix-trust-and-publication-boundaries)
and [hardening matrix](../docs/windows-time-development-hardening.md#911-windows-integration-qualification-refinements)
carry these obligations into their current owners. Valid unknown future coverage
is distinct from corrupt package content and does not create a synchronization
shutdown outside guards. Release changes preserve consumed clock authority and
mandatory recovery state.

This is documentation integration, not a new implementation or execution grant.
Retain the current Core priority, the scoped Sandbox boundary in section 4.19, the blocked
stager/artifact, separate fuzz hold, storage pause and UI-M0 deferrals. Historical
receipts keep their original candidates; Store submission and broad release
remain David's separate decisions. The S1.4 task records the coordinated
documentation review and final-candidate checks.

### 4.21 Release operations integration — 4 November 2026

**Scope and candidate:** the owner's requested audit §2.1 addition covers stable
support/backports, bad-release containment and recovery, and compromised
publication authority. It does not reopen the separate MSIX provisioning survey.
The source baseline is upstream `dcda54448241d54c0554d141f636fab55364d1f4`.
The coordinator verified an untruncated 1,082-blob tree and 403 included
source-documentation/skill/task/manifest files by Git blob identity. The isolated
local documentation snapshot uses synthetic commits `822cfd8` and `058b43d`;
those are workspace identities, not upstream history or product-build receipts.
Product source and archived design/assets are not fully materialized locally.

**Research and integration completed:** three distinct research roles supplied
GitHub upstream evidence, Microsoft Learn MCP platform evidence and other primary
security/incident sources. The fourth agent read all three finalized handoffs,
reconciled contradictions and produced the 631-line release-policy draft,
SHA-256 `7327b551bbcc96fde348441c738919a22aa3c0b64b9468444a9425414331f57d`.
Its own reference check resolved all 18 local links/headings at the intended
`docs/` location. The fifth agent integrated that draft and maintained-document
routing. Earlier launch failures supplied no review evidence; only completed
research/synthesis work is counted here. A supporting read-only secondary-surface
scan completed after its initial launch failure and supplied impact routing,
not a product or exact-candidate qualification.

The coordinator read the complete draft and source/semantic integration, then
requested one explicit addition: Microsoft Repair retains app data, while Reset
deletes its app-data scope and reinstalls. The canonical policy adds that
qualification and MS-06, without inferring the fate of all separate ProgramData
or mandatory machine state. The [primary source register](../docs/release-policy.md#102-primary-source-register)
retains the durable references and distinguishes vendor facts, transferable
upstream practices and proposed NevroTick procedure. No source survey or
historical test result is promoted to native recovery evidence.

| Existing owner / stage | Current disposition | Next dependent action |
| --- | --- | --- |
| W11-W10; David/release, OPEN-11/14, P6/P7 | Operational specification written; no stable release or approved maintenance commitment | Use [release-policy §2](../docs/release-policy.md#2-support-status-and-proposed-baseline) for the concrete first-stable decision: qualified latest stable, bounded transition overlap, backport exceptions, supported profiles, communication/response ownership and rollout criteria. No numerical SLA/LTS/notice period is invented. |
| W11-W10; release/security, CHECK-18/20/24 | Publication-authority and account recovery procedures prepared; actual private inventory and rehearsals pending | Bind actual MSA/Entra/users/apps and credential/access/renewal owners, recovery contacts and trusted artifact custody. Rehearse only through the applicable authorized arrangements; David retains account/submission/visibility/release actions. |
| W11-W10; release/package, CHECK-18/23/24/26 | Store containment and corrective-delivery cases prepared; no Store operation performed | Qualify private audience, flights, gradual update/halt, unsafe fallback/cloned-submission and actual corrective reach/version headroom. Keep submitted/delivered identity and available versus installed recovery separate. |
| W11-W05 with W11-W10; clock/state/lifecycle, P0/P4/P6, CHECK-5/12/13/15/23/27 | Compatible recovery obligations clarified; supported provisioning/independent-removal mechanisms and native results still open | Freeze the required old/bad/new schema and generation matrix. Preserve consumed/reserved authority, uncertain effects, original W32Time state, maintenance inhibition and successor/admin/GPO ownership. Demonstrate a useful authorized recovery beside refusal; a new version's safe refusal does not repair prior takeover. |
| Existing diagnostics and independent release owners, CHECK-17/24/26 | Incident record, bounded guidance and closure criteria written | Qualify actual redacted export/evidence paths and exact corrective artifact. Record distribution containment, recovered publishing authority, available correction, recovered installed case and unresolved populations as separate outcomes. |

**Canonical reconciliation:** the single [release policy](../docs/release-policy.md)
owns operations. Architecture §11.2 now distinguishes compatible transaction/code
recovery from restoring all pre-update data; mandatory state and narrowly
authorized floor supersession keep their original owners. Signing §8 distinguishes
written procedures from pending rehearsals. Hardening §12.1 identifies this
existing policy separately from the still-future `toolchain-and-ci.md`. Root
README/SECURITY now describe the existing bounded, unauthenticated ineligible
loopback observer without claiming production NTP/NTS, services or clock effects.
The semantic-document registry routes the policy and corrects its stale MSI-first
signing description; no index setting or indexing operation changed.

Current feature, package, development, diagram, ASVS, storage and release-bound
calendar/catalogue surfaces link the same operational owner. All CHECK/OPEN/AC/OD
identities remain; ASVS's 345 clause records and unmet remediation-deadline
requirements retain their classifications and evidence status. Historical
reviews, frozen candidate receipts and archived copies keep their original
scope. No imported skill, product source, dependency, build workflow or Windows
configuration is changed by this documentation integration.

**Unresolved limits:** the first-stable support commitment; actual account and
recovery readiness; the supported Windows/CPU/package transition matrix;
provisioning and recovery before independent Reset/removal; wrong-clock/offline
behavior; and actual delivered-artifact/lifecycle qualification remain open.
An authentic malicious same-product Store package has no established independent
on-device rejection guarantee; local file protection also does not prevent
whole-machine snapshot rollback. These remain explicit THR-15/17 and existing
OPEN-11/12/14 evidence/risk decisions, not accepted exceptions.

Retain section 4.19's narrow Sandbox scope, the blocked stager/artifact, separate
fuzz no-retry hold, storage pause, UI-M0 deferrals and current Core priority.
No product test, native drill, account/credential action, Store submission,
signing, publication or contact with a third party was performed by the five
research/integration roles. Documentation validation and independent review below
have only their stated candidate and source/document scope.

#### Release-policy independent draft review

Reviewer `/root/research_github` completed the bounded read-only review of the
frozen draft SHA-256 `7327b551bbcc96fde348441c738919a22aa3c0b64b9468444a9425414331f57d`.
The reviewer did not author that draft, but retained its earlier GitHub research
context and authored one of the analyst's inputs. **This was not a fresh-context
review.** Its result does not cover the fifth agent's concurrent integrated edits.
The raw review SHA-256 is
`0233fc7e2a3d8ff7de3c51521872d02072084b6c58be22196ebb0c0cd8d1fbcc`.

The reviewer's comprehension is preserved verbatim:

> The draft consolidates NevroTick's release operations into one policy: explicit support status and a proposed first-stable maintenance model, exact-candidate promotion, distribution containment, compatible corrective releases, and recovery after publication authority is compromised. It separates changes to source, distribution, installed code and protected state, and assigns the decisions and evidence needed for each. It does not propose a new updater or pretend that a written runbook supplies an implemented recovery command.

> The current system is an early-stage project with a selected complete hardened Store MSIX direction and planned privileged timekeeping boundaries. David controls the Store attempt and exact release approval; native package, lifecycle, state-migration and delivered-artifact qualification remain incomplete. Existing contracts require current clock ownership, durable correction accounting, preserved W32Time restoration evidence and an invocable recovery path. The missing provisioning/removal bridge, publication-compromise residual and whole-machine rollback limits remain open in the draft rather than being treated as solved.

> **Verdict: READY for documentation integration.**
>
> **Material findings requiring correction:** none in the frozen draft and stated review scope.

The reviewer directly rechecked the consequential Store rollout/flight/version/
visibility distinctions and primary publication/incident guidance, while reusing
its pinned upstream reads. Its explicit limits retain support approval,
OPEN-11/12/14, real account exercises, native recovery and actual Store-produced
artifacts as unfinished. No new finding IDs or product passes were issued.
The coordinator separately reviewed the integrated semantic changes, including
the Repair/Reset/MS-06 addition, and found no material blocker in that scope.
Final publication still requires the exact frozen candidate and remote-head
checks; this receipt itself performs no release or repository publication.

#### Documentation validation and preserved evidence

The fifth agent ran these documentation-only checks on the integrated worktree
against the verified synthetic baseline `058b43dbd8cfeb0b2dcfe312a433fb82dde8ef27`.
The canonical policy SHA-256 is
`6db4adbcf4be6378db09805c04d929e558b46e5311984effc3bcf0e4f21827b5`.

| Actual check | Observed result and limit |
| --- | --- |
| `git diff --check` | Passed; no changed whitespace errors. No application build was run. |
| Local Markdown references against the full upstream tree plus current additions | Scanned 1,447 references across 25 changed Markdown files; zero new missing targets/headings. Forty-five pre-existing occurrences of 40 temporary workspace links remain in the verbatim catalogue-task review at lines 942–1038; they are unchanged provenance, not newly valid repository links. Headings were checked where their target text is materialized locally. |
| Semantic-document registry | JSON parsed; 17 unique names and paths, all 17 local targets present, exactly one release-policy entry. No index operation or setting change. |
| Canonical identities and ASVS records | Preserved all 27 CHECK, 14 OPEN, 26 THR, 20 AC, 9 OD and 345 ASVS clause identifiers in their declared order. All ASVS clause records are byte-identical to the baseline; no status/classification or remediation obligation was changed. |
| Baseline and unchanged-file hashes | All 403 baseline files match the pinned upstream Git blob identities. All 378 unchanged baseline files remain byte-identical, including 241 skill files and 12 supporting-review files. This is not a requalification of absent product binaries or unmaterialized assets. |
| Historical receipts and diagrams | Preserved W11 §§4.1–4.20, storage-task §6 onward, catalogue-draft §9 onward, architecture §24 onward and every original byte of the catalogue task. All five diagram-companion Mermaid fences are unchanged and match their canonical architecture counterparts. |
| Changed-file scope and authority reconciliation | 26 files: 25 Markdown documents, including the new policy, and one context-routing JSON file. Existing task/phase owners, current Core priority and scoped execution dispositions remain. Source review covers both conservative refusal and useful authorized recovery; it supplies no runtime outcome. |

The original documentation snapshot HEAD is unchanged and no local commit,
remote write or staged publication was performed by the integrator. The
coordinator owns final exact-candidate validation and any authorized repository
publication. No product/account/Store/native case in release-policy §9 is marked
executed by these checks. The next product actions remain the bounded owner
assignments above; documentation acceptance does not lift their prerequisites.

### 4.22 P6 stage and acceptance routing — 4 November 2026

W11-W10 remains the production P6 parent, with W11-W05 supplying service
provisioning and recovery; no second package task is created. The
[task index](README.md) maps the stages to the selected complete-MSIX contract.
Preparation, admitted laboratory execution, David's Store action and actual
Store-delivered qualification remain distinct. Current execution holds remain.

| Canonical stage | Owner and bounded continuation | Required observation / exit |
| --- | --- | --- |
| S6.1 | W11-W10 with platform/security; freeze per-binary mitigation/DLL policy including the actual Slint/backend/generated/native module closure | Effective installed-role protections for engine, broker, helper and UI, including permitted Slint rendering/editing and applicable assistive/IME workflows beside tamper/load refusals; compiler flags or old Win32/pilot evidence alone are insufficient. |
| S6.2 | W11-W05/W11-W10 lifecycle; resolve supported provisioning/removal and recovery before the admitted lab card | Install/update/repair/Reset/interruption/removal preserve ownership and restoration; failed recovery is visible and safe. |
| S6.3 | W11-W10 with service/UI owners; integrate the actual Slint executable/runtime/assets into complete one-product behavior | Unelevated launch, close-versus-service behavior, old-component refusal, downgrade policy and unattended/last-user removal preserve all required components and mandatory authority; the standalone pilot EXE is no package pass. |
| S6.4 | W11-W10 with release/license owner and independent QA; prepare exact submission then qualify delivered bytes | Reconcile exact Slint/framework/backend/features, generated code, executable/runtime/assets, Cargo/native/build/proc-macro grants/notices and complete SBOM/payload/size inventory; bind submitted/delivered identities, trust and installed contents. Store signing is not helper authorization. |

The full [P6 exit](../docs/plans/development-plan.md#p6--complete-installation-hardening-and-release-artifacts)
remains CHECK-13/16/18/20/23, applicable CHECK-5/15/26/27 and AC-15/20 on
supported clean Windows fixtures, with the applicable operational evidence in
[release-policy §9](../docs/release-policy.md#9-qualification-matrix-and-remaining-decisions).
Selected storage adds only its applicable ST-T12/13/17 gates. Store submission
and exact-candidate public release remain David's actions/approvals. Before broad
release, release/security consumes the prepared procedures in section 4.21 and
rehearses the selected profile's renewal, revocation, access-removal and recovery
under [signing guide §8](../docs/windows-time-signing-guide.md#8-completion-checklist-and-ownership).
David retains account, Store submission and publication authority; retained MSI
profiles add no prerequisite. OPEN-11 and CHECK-18/23/24 own this operational exit.
P7's whole-product reconciliation is the separate
[final qualification parent](development-backlog.md#p7-final-qualification).

### 4.23 Optional build-provenance research — 4 November 2026

David requested research on evidence connecting a Store release to his repository,
then explicitly directed that it be documented **without a mandatory release
blocker**. [OD-05 §7.5](../docs/nevrotick-owner-decisions.md#75-optional-enhanced-build-provenance--4-november-2026)
records the accepted decision. The [focused research](../docs/plans/release-provenance.md)
and [release-policy §3.3](../docs/release-policy.md#33-optional-enhanced-provenance)
separate the optional assurance from existing release obligations.

| Item | Disposition and owner |
| --- | --- |
| GitHub/Sigstore provenance attestations | Optional build/release enhancement; no selected workflow or executed attestation. |
| Independent reproducible Windows build | Optional later experiment; no bit-for-bit application or package result established. |
| Public source-to-Store evidence bundle | Optional release communication and verification; actual Store transformations and complete delivered-payload comparison remain unmeasured. |
| Exact source/build/submitted/delivered identities, required SBOM/licences, package trust and installed-product/lifecycle checks | Existing mandatory S6.4/CHECK owners retain their scope. These obligations do not implicitly require the three optional mechanisms above. |

The research separates signed provenance from independent compilation evidence
and neither from a guarantee of benign source. Store re-signing prevents assuming
outer-package hash equality; any future enhanced claim must state its actual
artifact/comparison scope. Missing optional evidence or unresolved reproducibility
alone cannot block P6/P7 or release. A discovered violation of an existing
mandatory integrity/security requirement still follows its existing gate.

This is documentation only. No workflow, build, signing, account, Store submission,
installed check or native experiment was executed. Current Core priority and the
scoped execution dispositions remain. The first-stable support proposal in §4.21
has not been approved by this separate provenance decision.


The integrated optional-provenance and leap-entry documentation received a
completed fresh-context review, **READY for documentary integration**. Its
[candidate identity, verbatim comprehension, limitations and documentation checks](s1.4-leap-policy.md#entry-reconciliation-and-independent-review--4-november-2026)
are retained in the shared entry-reconciliation record. This evidence adds no
release qualification or obligation to execute the optional provenance experiment.

### 4.24 Pre-start service provisioning refinement — 5 November 2026

**Request and baseline:** continue the audit gap for service provisioning before
first functional start inside one complete hardened MSIX. The integration basis
is upstream `64944208a8174c57f38f05abec72d7145dd37dca`; the coordinator verified
the clean 585-file snapshot against that tree. The three GitHub, web and Microsoft
Learn research lanes and their synthesis supply source findings, not native
results or independent remote-tree verification by every researcher.

**Completed source integration:** [MSIX plan §7.5](../docs/plans/windows-msix-store.md#75-pre-start-provisioning-authority-and-admission--5-november-2026)
owns the evidence classes, conditional sequence and first provisioning probe.
It retains S01–S87 and appends S88–S102. The current architecture, development
workstream, hardening guidance and service-adapter reference route the same
obligations. Sections 4.15/4.18/4.19/4.23 and their candidate-specific receipts
retain their historical scope; final integrated-candidate review and checks are
recorded separately after execution.

The material result is two unresolved prerequisites: **actual per-operation
authority on deployment-created service objects**, and **supported no-UI
invocation plus servicing reentry**. Generic CreateService defaults, elevation
or a LocalSystem account do not prove the first; automatic startup, bundling or
an external laboratory launcher do not prove the second. The public DACL report
is a reproduction lead with no owner field, not a universal MSIX/AppXSvc rule.
The pinned WSL example invokes MSI and supplies no complete-MSIX-only recipe.

| Existing owner / checks | Current disposition | Next dependent action |
| --- | --- | --- |
| W11-W05 provisioning; CHECK-1/2/15/16; OPEN-3/12/14 | Source contract refined; actual rights, supported setters and production invocation unresolved | Prepare the existing S0.6/disposable-environment card for a handle-only probe on untouched deployment-created objects. Query/status/read-security and necessary configuration rights are separate; WRITE_DAC/WRITE_OWNER apply only if the operation needs them. No setter/start/stop call in this first probe. |
| W11-W05/W11-W10 activation and identity; CHECK-1/2/16/23/26/27 | Configured state and runtime admission remain distinct | Once prerequisites and execution are admitted, preserve disabled/quiescent configuration, activation requirements, controlled SCM start into bounded authenticated management, then fresh lifetime-bound admission before normal work. Creation-time process/thread/token protection must already hold; START_PENDING PID or prior success is insufficient. |
| W11-W05 object security; CHECK-1/2/15/16 | Same-account isolation remains unqualified | Separate TokenOwner/defaults for later objects from initial process/thread/token descriptors and the token object's own security. Qualify earliest access, actual primary/impersonation paths and later replacements with intended allowed controls. Preserve raw dwServiceType and unknown bits beside wrapper interpretation. |
| W11-W05 servicing/lifecycle; CHECK-5/12/13/23/27 | Reentry, persistence and safe refusal/recovery remain unresolved | After the earlier boundaries, distinguish interrupted configuration, version update, same-version repair/re-registration, Reset and profile migration. A new version refusing admission must retain the already-owned recovery path; no stale marker or configuration replay over a successor. |
| W11-W10 packaging/release; CHECK-18/23/24 | Complete hardened MSIX remains selected; all three provisioning candidates remain unselected | Retain the fixed elevated helper, temporary automatic bootstrap and restricted-child alternatives with their unresolved costs. Use NV-DEC-001 before a dependent topology choice; no external/embedded MSI workaround, mandatory first UI or broader LocalSystem broker follows. |

**First future provisioning exit:** identify whether the exact proposed actor
can acquire each required right on the untouched targets, with a permitted query
and an available standard-user/UI mutation-denial control. Stop at a required
authority failure; do not take ownership or bypass deployment security. A
successful handle-only probe proves neither setter behavior nor production
invocation. Actual engine/broker actor controls wait for their later management
start; a synthetic same-account actor must be labelled. Installation setup and
the probe need their own bounded admission; the source card grants neither.

**Separate unresolved lifecycle item:** independent removal/Reset still needs a
supported recovery owner surviving until its work is durably complete. Section
7.3's inert-journal running-versus-already-stopped/crashed removal pair remains
the first removal/recovery discriminator. This provisioning refinement does not
replace it or close OPEN-3/12/14.

**Unchanged authority and holds:** current Core priority remains. Section 4.19's
scoped Sandbox readiness/management permission does not authorize this product
probe; the blocked S134 stager/artifact and general native restrictions, separate
fuzz no-retry hold, storage pause and UI-M0 D1–D3 deferrals remain. Enhanced
attestations, independent reproducible builds and a public proof bundle remain
optional under OD-05 §7.5; required artifact custody, SBOM/licence, package trust,
installed identity and native/lifecycle evidence retain their existing owners.
No code, manifest, dependency, package build, Windows execution, clock/service
mutation, signing, Store submission or release qualification is performed by
this amendment. The next permitted action is review of the integrated documents
and continued bounded source/design work under the existing owners.

#### Independent review and documentation verification

**Reviewed candidate:** upstream
`64944208a8174c57f38f05abec72d7145dd37dca` plus the seven-file documentary
diff with SHA-256
`65540c0dc425e24f9a8027fa4da1be706fed33879c1a6527433c8ba46f1ea9d8`.
The review checked every changed file against its frozen digest; the local
synthetic baseline was not presented as an upstream commit. This receipt is the
coordinator's subsequent append; it changes no reviewed requirement or source
interpretation.

**Review accounting:** `msix_provisioning_review` was launched with
`fork_turns: "none"`, read-only scope and the actual request, candidate,
canonical requirements and selected source evidence. Its initial launch failed
with a context-window error before producing usable review and is **not run**.
One bounded retry **completed** the requested review. It did not consult the
private synthesis, parent discussion, memory or authors' expected verdict.
The earlier three research lanes and fourth-agent synthesis supplied the fifth
agent's integration; they were not counted as this fresh review.

The reviewer inspected the full diff and actual integrated sections, applicable
AGENTS/planning/security/implementation/testing contracts, OD-05, architecture
and lifecycle/IPC context, current Core and Sandbox dispositions, and W11
assignment. It independently checked the decisive S88–S102 sources and existing
SCM/activation contracts. No necessary review input was missing. Full remote-tree
verification and the skill checker remained coordinator evidence, not work
attributed to this reviewer.

**Reviewer comprehension, preserved verbatim:**

> The change turns the open MSIX service-provisioning question into an ordered research and qualification path. It first asks whether the proposed actor can obtain the necessary rights on the services Windows actually deployed, separately asks what supported event invokes that actor without a first UI launch, and then orders configuration, activation, a controlled management start, fresh-process checks and normal work. It adds source limitations and future positive/negative cases without choosing a provisioner or claiming that the production mechanism exists.

> The current repository describes a six-member workspace with a fixture-only native UI, inert protocol and numerical components, modeled Core behavior and a bounded unauthenticated loopback observer. Those parts do not yet form the production synchronization pipeline. The two-service authority design, privileged lifecycle work and complete hardened MSIX remain planned and unqualified. The current Core continuation still takes priority; scoped Sandbox management does not admit this product experiment, and the blocked artifact, fuzz/storage holds and UI-M0 D1–D3 deferrals remain in force.

**Raw verdict:** “READY for documentary integration. Review completed. No
material documentation-stage findings.” No MSIX-R finding was raised.
The raw report's SHA-256 is
`1bb9d623c6cc0af9993309e5b3df6c0ecde5ffec693a5e1b46971c5a071a8f9b`.
This disposition applies to the documentary candidate and bounded source/design
continuation, not to a selected topology, native experiment or release.

**Documentation verification:**

- `git diff --check`: passed. The focused changed-document check covered
  **927 local links**, with no new or preexisting target/heading failures.
- All seven changed paths are the intended Markdown surfaces. Five architecture
  Mermaid blocks, the canonical ID set, historical task §4 content/receipts and
  the original running-versus-already-stopped removal experiment are preserved.
  Prior S01–S87 entries are byte-identical; all **102** source IDs are unique,
  contiguous and defined where cited. Fences remain balanced.
- The unchanged package checker initially returned 1 for 15 missing-link
  occurrences pointing to 13 files omitted from the partial local snapshot.
  Each target exists upstream. The coordinator populated a separate validation
  mirror with those exact upstream blobs, verified their Git identities and all
  seven frozen candidate file hashes, then reran
  `python3 -B .agents/skills/scripts/check_skills.py --root <verified-mirror>/.agents/skills`:
  **PASS, 63 skills**. No policy, inventory or product file was changed to obtain
  that result.
- The README-required
  `python3 -B .agents/skills/scripts/test_check_skills.py -v` completed with
  **25 tests passed**. These are skill-checker controls, not application,
  service, package or Windows qualification.

**Current exit and next action:** source synthesis and documentary integration
are complete. W11-W05/W11-W10 retain the supported production-owner, invocation,
creation-time isolation and native servicing gaps described above. Continue
bounded source/design preparation under those owners; any actual probe still
needs its exact fixture, actor/context, Windows/channel row, finite operations
and existing environment admission. Current Core priority, all stated holds,
the separate removal gap and optional enhanced-provenance policy remain.

### 4.25 Independent removal with outstanding recovery — 5 November 2026

**Request and baseline:** continue the next audit gap: independent Settings
uninstall/Reset when the service has already stopped/crashed and machine recovery
is still pending. The upstream basis is
`4e23c781ca0b2e03e80289bca891dae6df09b6d6`; the clean local materialization was
`ac2b51b1ba88cc378fe372dac212f7e0b4df66c9`, a workspace identity rather than
upstream publication history. The three completed GitHub, web and Microsoft Learn
research lanes were reconciled in the fourth-agent synthesis before documentary
integration. These source reports supply no native result or claim that every
researcher independently verified the remote tree.
Initial launches of those four roles failed with context-window errors; their
bounded retries completed. Failed launches supplied no usable finding or review
result and are not counted as completed research.

**Inspected scope and integration:** applicable repository instructions and
planning/architecture/threat, security/privilege/deployment, native-proof,
testing/lifecycle and service-adapter guidance; MSIX §§5/7.3–7.5, architecture
§11/THR-12/13/16/§19.4, current W11-W05/W10, scoped Sandbox §4.19 and provisioning
§4.24. This is a bounded recovery investigation, not a full implementation audit.
[MSIX plan §7.6](../docs/plans/windows-msix-store.md#76-independent-removal-with-outstanding-recovery--5-november-2026)
owns the new findings, primary evidence and future experiment. The source register
retains S01–S102 and appends S103–S121. Current architecture, hardening, development
routing and service-adapter guidance consume that same contract. Earlier §4
receipts and the separate §7.5 provisioning record retain their original content
and candidate-specific scope.

**Material result:** the supported composition of trusted compatible code,
protected mandatory data, current authority and independent invocation remains
unproved after every product service stops. Native dynamic dependencies can retain
opted-in main-package content; combining retention with an independent task is a
plausible **unselected inference**, with provisioning, post-deregistration launch,
identity, permissions, version handoff, timing and final retirement unresolved.
The guide's unpackaged-caller pattern is an applicability gap, not a blanket
prohibition or a supported self-pinning recipe. Its build-22621 feature floor
does not lower the full hardening profile, including Trusted Launch's separate
floor. Genuine strong references differ from incidental open-file survival.

`EnvironmentManager` supplies contrary evidence to any universal claim that
uninstall can never restore external state, but its resource-specific facility
is no generic W32Time or clock-recovery hook. The pinned WSL/MSIX Core installation
owners and TranslucentTB's user-process recovery have different authority and
lifetime contracts. No copied helper, embedded/external MSI, task/provider,
privileged resolver or wider engine/broker authority is adopted by analogy.

| Existing owner / checks | Current disposition | Next dependent action |
| --- | --- | --- |
| W11-W05 lifecycle; CHECK-5/12/13/23/27 | Source contract refined; supported independent owner remains unresolved | Name the candidate's production provisioning/activation assumptions, then prepare the same held S0.6 running/already-stopped removal pair with a durable obligation pending at removal. A clean earlier stop that already restored state does not exercise this gap. |
| W11-W05 authority/state; CHECK-1/2/15/16 | Required rights and cleanup authority remain separate proofs | Bind actual task/clock/SCM/cross-user-observation and code/anchor/journal access to fixed operations and current ownership. Added ACL grants and retained references need supported retirement; no privilege/DACL bypass or machine-wide conclusion from a current-user empty result. |
| W11-W10 package/activation; CHECK-16/18/23/26/27 | Native retained-content/task composition unselected and unqualified | Establish the exact OS/channel, opt-in, actual retained content and fresh post-deregistration activation with package identity, module closure, Trusted Launch/CIG and initial process protection intact. Retained bytes or a manual laboratory launch is narrower evidence. |
| W11-W05/W11-W10 servicing; CHECK-5/12/13/23/27 | Successor/version and final-removal cases remain open | Later challenge A's pending recovery while B refuses normal admission, stale A cleanup under admitted B, and dependency criteria retaining B while a task names A. Preserve mandatory schemas, consumed/uncertain authority and administrator/GPO changes; retire remaining task/anchor/rights only in qualified order. |
| W11-W10 release; CHECK-18/23/24 | Complete hardened MSIX remains the selected preparation target | Keep missing lifecycle support explicit in candidate/certification notes. Use NV-DEC-001 before a dependent choice of surviving privileged state or topology; an unresolved mechanism supplies no automatic MSI fallback or Store/native pass. |

**First future removal discriminator, held and unexecuted:** the inert fixture's
own transaction establishes owner A, original dummy X, product-owned Y and a
durable pending recovery obligation. Immediately before independent Settings
removal, an observer must see Y and no matching completion receipt. Compare the
running service with the already-stopped/crashed service under that same initial
condition. Observe the actual actor, dummy effect and durable disposition relative
to service/registration/code/data loss. A retained-path claim needs fresh activation
after the relevant deregistration, without an old product process supplying
continuity. A self-reported marker alone establishes no Y→X effect.

Retain clean STOP as a separate successful-recovery control and an admitted
successor/later-policy case as a no-restoration control, beside useful current-owner
completion. If a task or SCM restart settles recovery before removal, keep the
useful result but record the missing pending-at-removal condition. An abrupt exit
or interrupted inert seam needs its own admitted bounds; required work cannot
simply continue in the dying process after `SERVICE_STOPPED`.

The observer does not restore X, write completion, invoke the actor or pin code
solely to produce survival. Record mappings, genuine references, service/debugger
and inherited handles, unrelated registrations/provisioning and their actual
effects. Retain required CHECK-27/native-memory instrumentation with an appropriate
control; not every query handle pins payload. Bind real identity/token, entry
point/user, Windows/channel, task/anchor/generation, raw errors and independent
elapsed order. External teardown follows the result boundary and is not product
recovery. Finite attempts/time/work/output and cleanup ownership remain required.

Keep completed restoration, a verified retained invocable path, authorized
successor/divergence with durable no-restoration disposition, and pending recovery
without a usable actor distinct. Read-only Blocked protects authority without
proving restored availability. Only a credible first result justifies expanding
to pending-obligation Reset, nonfinal/last/all-user removal, deprovisioning,
update/repair/re-registration and reboot/interruption. The inert pair measures no
clock-rate retention or residual phase envelope; those existing native gates
remain separate.

**Next permitted action and holds:** continue bounded source/design work on the
supported owner/invocation question and review the integrated documents. Actual
execution still needs the exact candidate, mechanism, actor/context, environment,
finite operations and existing admission. Section 4.24's handle-only provisioning
probe remains a separate prerequisite; this research admits neither experiment.
OPEN-3/12/14 remain open and unavailable recovery blocks takeover before mutation.
Current Core priority, section 4.19's scoped Sandbox permission, blocked S134
artifact/general native restrictions, separate fuzz hold, storage pause and
UI-M0 D1–D3 deferrals remain. Enhanced attestations, independent reproducible
builds and a public proof bundle stay optional; required custody, SBOM/licence,
package integrity, installed identity and lifecycle evidence retain their scope.
No code/manifest/dependency, build, native/service/clock/trust, Store or release
action is performed by this amendment.

**At integration handoff, independent review and verification are pending
coordinator.** This integration record claims no completed fresh review, final
link/preservation/skill check or native qualification. The coordinator owns the
exact-candidate review and subsequent documentation verification receipt.

#### Independent review and documentation verification — 5 November 2026

**Reviewed candidate:** upstream
`4e23c781ca0b2e03e80289bca891dae6df09b6d6` plus the seven-file documentary
diff with SHA-256
`f3dd2d35d8d708bb8dde9bfab04d589dc05892169be1288c6e6babafb1fb1c19`.
The reviewer verified the working diff and all seven file digests against the
frozen inputs. The synthetic local baseline was not represented as upstream
history. This coordinator receipt is a subsequent append; the six other files
are unchanged and the reviewed task remains its byte-identical prefix.

**Review accounting:** `recovery_review` was launched with
`fork_turns: "none"`, the actual request, frozen candidate, canonical requirements,
primary evidence and read-only scope. Its initial context-window launch failure
produced no usable review and is **not run**. One bounded retry **completed** the
requested review. It did not consult the private research/synthesis reports,
parent discussion or memory. The integration worker likewise had an initial
context-window failure followed by completed read-only preparation and the
subsequent authorized integration. Those failures are not zero-finding results;
the earlier research/synthesis outcomes remain recorded above.

The reviewer inspected the full frozen diff and changed integrated sections,
applicable planning/security/privilege/deployment/native-proof/testing guidance,
OD-05, current Core and Sandbox dispositions, W11 routing, and representative
workspace manifests/source for existing versus planned behavior. It independently
checked the decisive S103–S121 sources and affected existing Microsoft contracts.
Some initial web opens failed; alternate primary retrieval supplied the needed
content. No unresolved missing technical input controls the verdict. The full
remote-tree materialization, earlier workers' histories, historical native receipts
and repository link/skill checks remain coordinator evidence, not work attributed
to this reviewer.

**Reviewer comprehension, preserved verbatim:**

> The amendment sharpens the unresolved recovery question for an MSIX removed while every product service is stopped and a machine obligation remains outstanding. It separates the survival of trusted code, protected data, current authority and independent invocation. It also strengthens the planned inert removal comparison so that earlier successful cleanup, surviving bytes or observer assistance cannot be mistaken for recovery through removal. Native dependency retention combined with an independent task remains an option to investigate, not an adopted mechanism.

> The inspected workspace contains six Rust members: a fixture UI, inert protocol and numerical owners, modeled Core behavior and a bounded loopback transport pilot. The production service pair, clock broker, lifecycle helper and complete MSIX recovery arrangement remain planned and unqualified. The current Core continuation retains priority; Sandbox readiness does not admit this experiment, the blocked artifact and fuzz/storage restrictions remain, and UI-M0 D1–D3 are deferred. This change supplies documentation and source interpretation rather than new implementation or Windows results.

**Raw verdict:** “READY for documentary integration. Review completed.”
No material documentation-stage finding was identified; no REC-R issue was
raised. The raw report SHA-256 is
`663d5d1d13eb8569c1a664828fa9f2b10412da2ad4dd35ef663cccae41f18a29`.
This verdict covers the documentary candidate and bounded source/design
continuation, not topology selection, experiment admission, native qualification
or release.

**Documentation verification:**

- `git diff --check`: passed. The focused check covered **945 local links** in
  the seven changed documents, with no new or preexisting target/heading failures.
- The intended seven Markdown paths are the entire delta. Five architecture
  Mermaid blocks, the canonical ID set, historical task §4 content, prior
  provisioning §7.5 and all **102** earlier source entries are byte-preserved.
  The full hardening lifecycle-qualification row is also byte-preserved; the
  preliminary inert experiment does not replace its restoration requirements.
- All **121** source IDs are unique and contiguous; every source ID used in
  bracketed citations is defined. Markdown fences remain balanced.
- `python3 -B .agents/skills/scripts/check_skills.py`: **PASS, 63 skills**, covering
  metadata, UI prompts, local links and inventories in the verified local
  materialization. No missing snapshot target or policy bypass was involved.
- `python3 -B .agents/skills/scripts/test_check_skills.py -v`: **25 tests passed**.
  These are skill-checker controls, not product, package, service or Windows tests.
- The coordinator rechecks the appended receipt's links, whitespace, preservation
  and frozen-prefix identity before publication. The unchanged six-file and skill
  inputs retain their earlier scoped verification; no unrelated campaign is rerun.

**Current exit and next action:** source synthesis, documentary integration and
independent review are complete. The supported independent recovery owner,
post-deregistration activation, actual rights, version/successor handoff and final
retirement remain open with W11-W05/W11-W10. Continue the bounded source/design
question and prepare any material owner decision before its dependent action.
The pending-obligation inert pair still requires its exact admitted mechanism,
fixture, actor/context, Windows/channel row, finite work and cleanup. Current
Core priority, the separate provisioning gap, execution holds and optional
enhanced-provenance policy remain. No native recovery or release pass is claimed.

### 4.26 S2.1 timestamp-provenance preparation — 5 November 2026

**Disposition:** W11-W01 is **partially prepared source work**. The
[existing engine integration card](engine-transport-pilot.md#s21-preparation--real-transport-to-an-admitted-time-measurement--5-november-2026)
records the actual runtime producers, the separate `Composition::deliver`/`record`
fixture chain and the absent engine-runtime bridge. It incorporates completed
GitHub, standards/web and Microsoft Learn MCP research through a fourth analysis
and fifth documentation integration. The card retains the inspected candidate,
launch/retry disposition and primary sources; this is not a completed native card or
an independent-review receipt.

**Prepared portion:** preserve genuine socket/request/endpoint identity, raw
header claims, software send-call and receive-return observations, processing
expiry, mapping/policy/calendar context and consume-once publication. Adjacent
`Instant`/`SystemTime` readings do not establish simultaneous capture, packet
placement or a stable UTC mapping. Increasing QPC does not prove that mapping
survived wall-clock, sleep or presentation changes. The Windows oversize error
can charge socket consumption without assigning peer identity, blame or freshness.
Native operation buffers, permits and identity remain owned through terminal
completion; these documentary contracts do not identify Tokio's backend behavior.
The engine card's first experimental measurement remains `eligible=0`; later
uncertainty/reference-time admission and actual Core acceptance are distinct exits.

**Remaining source and execution gaps:** the acquisition/platform owner still
needs the selected capture convention, finite mapping validity/error terms and
invalidation/unsupported outcomes, exact SDK/generated-binding and backend
questions, independent cases with finite resources, environment and cleanup.
Retain the existing ancillary/header and unregister discrepancies. Optional
software/NIC timestamps and direct-overlapped I/O remain conditional; their
supported, configured and observed states, interface attribution, queued metadata
and completion controls require their own evidence if selected. No native case,
API adoption or full W11-W01 completion is claimed.

Continue that bounded source contract when the owner reaches S2.1/P2, using this
card and W11-W01's existing cases rather than repeating a general API survey.
[Development §7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
keeps S1.1c's rounding/era proof continuation first. The selected Linux Core
checkpoint, leap implementation/proof scope, complete MSIX, optional enhanced
release provenance, S134/native and fuzz restrictions, storage pause and UI-M0
D1–D3 deferrals retain their recorded status. This preparation lifts no hold and
supplies no source-to-native qualification.

### 4.27 Address-family admission routing — 5 November 2026

**Owner/stage:** engine/protocol with the platform adapter owner, before dependent
S2.1/S2.2 acquisition integration. The [canonical contract](../docs/windows-time-architecture.md#address-family-admission-and-alternative-progress)
and [existing engine task card](engine-transport-pilot.md#address-family-admission-and-bounded-acquisition-progress--5-november-2026)
own local IPv4/IPv6 usability and bounded alternative-address progress. This note
routes the Windows portion; it adds no W11-W11 label or acceptance gate and does
not extend W11-W01's timestamp-only next card or depend on the future DNS service.

Reuse the existing interface/address/route reconciliation and callback-custody
rules. Qualify the selected adapter's initial observation, address suitability,
destination-specific route/source selection, changed-during-read and unknown
outcomes, bounded rechecks, and actual completion/stop behavior. Demonstrate
disabled-family skipping beside a usable alternate interface/LAN/VPN path;
global Internet/cost hints are not destination admission. `GetAdaptersAddresses`
and `GetBestRoute2` remain source-supported candidates, with exact SDK/binding,
intended-token behavior and native controls pending. Do not run synchronous
enumeration on packet-timing paths or infer usable addresses from list presence.
Distinguish genuinely disabled paths from dormant/on-demand paths that ordinary
permitted traffic can activate; qualify the bounded activation/disabled pair
without granting application authority to reconfigure adapters or VPNs.

**Disposition:** documentation prepared; implementation and native evidence
remain unexecuted. The engine card supplies finite case/profile preparation and
existing CHECK-6/8/9/21/27 ownership, with CHECK-25 only where construction guards
are reached. S1.1c remains first under development §7. Existing native/S134, fuzz,
storage and UI holds/deferrals retain their current scope; this entry permits no
new host configuration or public-server probe.

### 4.28 Explicit network-profile qualification — 5 November 2026

Extend §4.27 through the same engine/platform owners and
[network-profile card](engine-transport-pilot.md#network-path-history-and-user-routing--5-november-2026).
Apply the [canonical contract](../docs/windows-time-architecture.md#observed-path-history-and-user-network-policy)
before dependent S2.1/S2.2 direct binding. Under
[OD-11](../docs/nevrotick-owner-decisions.md#15-od-11--socks5-scope-and-deferral),
all proxy-specific qualification below belongs to deferred
[SOCKS-0](../ROADMAP.md#socks-0--optional-socks5-time-acquisition), after first
Windows delivery; it is not a current P2/P5 prerequisite. This adds no W11-W11 label
and does not make the future system DNS service a prerequisite for direct NTP.

Qualify selected interface identity to current IPv4/IPv6 index mapping, source
selection, actual outbound/receive provenance and removal/reuse races. Microsoft's
`IP_UNICAST_IF` takes an index in network byte order, while `IPV6_UNICAST_IF` uses
host byte order; wrapper/default behavior is not enforcement. Capture required
binding options before use and verify native effects, including supported
dual-stack/IPv4-mapped cases. No route-table or adapter reconfiguration is granted.

Keep configured local family, destination/transport success and proxy-side family
distinct. Reconcile cached history after attachment/route/VPN/resume changes.
For a proxy qualify both control and relay sockets under the actual engine token,
separate UDP-control lifetime from KE, and verify wrapper-aware timestamps and
uncertainty. A loopback proxy may be per-user and vanish at logout; selecting a
physical adapter cannot control that process's onward route. Browser proxy
settings, credentials and DNS behavior are not implicitly inherited by a service.

Reuse the [existing state allocation](../docs/windows-time-architecture.md#1213-network-state-reuses-existing-storage-owners),
not a native-adapter-owned database. Current interface/source observations and
generations feed the engine owner; persisted policy stores stable identity, not
a reusable live interface index. Qualify PATH-19's policy cutover with actual
outstanding sends, resolution/verifier jobs and proxy-control lifetime. A saved
revision, cancellation request or completed waiter is not effective-profile
evidence; late native completion retains its resource charge and cannot publish
as current. Already submitted remote/OS traffic is outside any recall claim.
The existing ST-W01/02/03/04/06/07 owners qualify a conditional proxy credential's
separate purpose, service readers and lifecycle only for a later authenticated
SOCKS5 extension. The first future subset uses loopback/no-auth/local-DNS and
needs no proxy secret purpose; no new secret backend or path.

**Disposition:** source-backed planning only. CHECK-2/6/8/9/12/14/17/21/25/27 and
AC-21/22 retain their existing evidence owners; the engine card supplies paired
controls. Native binding, controlled proxy compatibility, resolver/verifier egress,
service credentials and independent plan review remain unexecuted. Preserve the
current development §7 proof continuation, first fixed-peer measurement, provider,
storage and Windows execution gates and UI-M0 deferrals.

### 4.29 Windows component reuse routing — 5 November 2026

**Scope/status:** completed source comparison recorded against NevroTick
`e82f3f6b352fde88beceece4d3084862d8778338`; integration into the plan supplies
no build/native/provider/package pass. Use the
[canonical reuse register and source pins](../docs/windows-time-architecture.md#1862-windows-component-reuse-and-binding-coverage)
and [W11 adapter map](../docs/plans/windows11-integration-follow-up.md#windows-component-reuse-at-the-adapter-boundary).
The existing UI already uses `windows-sys`; the role-specific production adapters
remain planned. No W11-W11, second dispatcher or general Windows manager is added.

**Next action at the existing consumer's entry:** its platform/dependency owner
records operation and caller -> selected published crate/API/version/checksum/
source/features/SDK symbols -> resource/error/authority/completion obligations
still enforced by NevroTick -> independent legal/refused observation. Reuse that
card while its relevant candidate is unchanged. Check archive/source parity and
type-generation compatibility before executing dependency or generator inputs.
Publication, target compilation and runtime capability stay separate evidence.

| Existing assignment / consumer | Required reuse disposition before dependent implementation |
| --- | --- |
| W11-W01, S2.1 | Reuse IP Helper/Winsock for local family/route/interface observations and timestamps; retain destination-specific success, network generation, ancillary bounds and actual I/O completion. No engine-runtime rewrite. |
| W11-W02, S0.4/P4 | Reuse clock APIs; retain unit/domain mapping, current broker permission and acknowledged/uncertain correction outcomes. |
| W11-W03 | Reuse selected DNS query/settings APIs; preserve NRPT/private-name policy, cancellation ownership and exact restoration, with future DoQ/DNSSEC remaining separate. |
| W11-W04/05, S0.2/S0.3/P3 | Retain SCM/token/pipe/mitigation bindings and Mullvad service candidate. Microsoft plural `windows-services` stays held for status-error/progress/automatic-transition mismatches; extended controls exist. Qualify the actual wrapper lifetime and one lifecycle owner. |
| W11-W08, S3.4/S5.3 | Reuse Event Log/ETW or the admitted emitter; retain event/redaction/budget/authorisation policy and required sinks. |
| W11-W05 with ST-W01/03/04 | Preserve filesystem recovery and exact native rights. The [storage coverage card](windows-protected-storage.md#67-windows-binding-coverage-and-reuse--5-november-2026) owns the VBS-symbol gap and conditional registry-wrapper constraints. |
| W11-W09, S0.5/P5 | Follow [UI reuse](ui-m0.md#windows-component-reuse-handoff--5-november-2026) and [conditional toolkit evaluation](s0.5-ui-comparison.md#conditional-windows-toolkit-candidates--5-november-2026); select no new runtime or picker from main-only examples. |

**Exit:** the next selected adapter has complete source/API coverage, a named
product invariant owner and an observable valid/failure pair. CHECK-26/27 and
the assignment's existing CHECK/ST-T cases remain the actual admission gates.
Native controls must preserve genuine errors, denied rights, reentrancy/late
completion and legitimate progress; source signatures or library branding alone
cannot close them. Keep Core priority, Windows stop, separate fuzz no-retry hold,
storage pause, UI-M0 deferrals and selected complete-MSIX provisioning authority.

### 4.30 Ordinary S2.2 Windows qualification — 5 November 2026

**Disposition:** prepared source handoff, no native result or execution permission.
The [engine composition card](engine-transport-pilot.md#ordinary-full-nts-composition--s22-preparation-5-november-2026)
and [crypto profile](s0.1-crypto-feasibility.md#ordinary-s22-trust-profile-and-native-work--5-november-2026)
route existing W11-W01/04/05/08 and CHECK-9/21/26/27 work; add no new W11 gate.
Preserve development §7 and every native/S134, fuzz and storage hold.

Before the dependent experiment, bind the exact artifact/features, Windows build,
principal/profile loading, effective stores, normal-date profile, resource limits
and owned fixture/restoration. A controlled peer/root using ordinary validation
can establish that declared lab path only. Final installed service identity,
stores and enforcement remain P3/P6 checks; complete MSIX, Slint migration and
all P3 work are not prerequisites for the bounded S2.2 prototype.

| Existing owner | Paired native observation required at its admitted stage |
| --- | --- |
| Trust/platform, OPEN-8 | A valid selected-profile chain beside wrong identity/usage/date, revoked or explicitly distrusted leaf, removed root, unavailable/stale revocation and trust-policy cutover. Expected acceptance/refusal follows the frozen profile. Report opaque revocation status honestly; no automatic custom verifier or new strict-online policy. |
| Network/service, W11-W01/04/05 | Identify actual intermediate/root/revocation fetch processes, protocols, redirects and cache behavior. Pair permitted retrieval and approved LAN/Direct paths with forbidden egress and policy cutover; unsupported enforcement refuses visibly. An engine socket/firewall observation cannot establish delegated-service enforcement. |
| Lifecycle/diagnostics, W11-W04/05/08 | Slow/stalled retrieval, cancellation and late completion beside healthy acquisition/control responsiveness. Observe real submitted-job/byte bounds, retained permits/native inputs, stale-result disposal and bounded stop/restart behavior under §17.1/THR-26; waiter timeout is not completion. |

Primary API contracts: [chain construction](https://learn.microsoft.com/windows/win32/api/wincrypt/nf-wincrypt-certgetcertificatechain),
[SSL policy execution](https://learn.microsoft.com/windows/win32/api/wincrypt/nf-wincrypt-certverifycertificatechainpolicy)
and [URL retrieval](https://learn.microsoft.com/windows/win32/api/wincrypt/nf-wincrypt-cryptretrieveobjectbyurlw).
Record configured versus effective state and actual fetch/completion owners in
this task. Source/API availability, portable fixtures and an interactive user’s
successful chain do not qualify the installed service. Keep ordinary and
wrong-date/reused-session evidence separate.

### 4.31 Clock-correction limits preparation — 5 November 2026

**Disposition:** source preparation for OPEN-5/7/13 under existing S0.4/S0.7,
P4 and W11-W02/04/05. No production value, owner decision, experiment, native
result or execution permission is adopted. Inspected NevroTick source:
[`09caf0d3303abafab1edba0064a1c9c117514b70`](https://github.com/DavidOsipov/NevroTick/tree/09caf0d3303abafab1edba0064a1c9c117514b70).
The bounded upstream/API comparison below supplies definitions and contrary
evidence; it is not an exhaustive implementation audit or Windows qualification.

Keep four inputs distinct: product tolerances for disruption/error/availability;
native representation and observed capability; model/environment assumptions;
and broker-enforced authority. Measurements inform policy but cannot choose it.
Reuse [architecture §10](../docs/windows-time-architecture.md#10-windows-clock-backend),
THR-19 and §17.1, [R-02/R-03](../docs/nevrotick-research-handoff.md), and
[OD-02/04](../docs/nevrotick-owner-decisions.md). All numerical production inputs
below remain unresolved; the matrix is a worksheet, not a new IPC schema.

#### Field, provenance, measurement and admission matrix

Each future record names units/domain, source revision, supported population,
evidence basis (documented, assumed, observed or statistical), uncertainty,
separate approval status, decision owner and exact producer/consumer. Approval
does not establish physical evidence. Proposed observations are unexecuted.

| Fields | Provenance and existing owner | Proposed measurement | Admission and useful/refused controls |
| --- | --- | --- | --- |
| Base/transient/combined rate, base age/change, effective quantum | Raw adjustment/increment/mode/QPC frequency; S0.4/W11-W02 → S4.1/2 broker | Qualify one reference coordinate/sign/conversion; distinguish requested, representable, read-back and independently observed rates. Both signs, zero, adjacent quanta and saturation on declared builds/protections. | Healthy base-only and bounded delivery beside excessive/invalid combined requests with no effect. Numeric read-back proves neither ownership nor slope; retain component and combined ceilings. CHECK-10/25. |
| Step threshold, magnitude/target-date interval, direction/count, acquisition window, approval expiry | Approved profile and true-date/era evidence; R-02/R-03/S0.7 → S4.2 | Separate local date error, supported calendar/conversion domain and achieved-effect brackets; bind fresh agreement, snapshot/generation and one-use authorization. | Qualified ordinary and large authenticated recovery beside ambiguous era, stale/conflicting evidence, unsupported target or exhausted approval. OD-02's near-century goal remains; twelve hours for plain NTP is unapproved. Lost reply permits no repeat. CHECK-3/7/8/9/10/12. |
| Absolute movement, signed displacement, step count, windows/renewal, reserved/unknown authority | Actual combined delivery and original durable operation identity; broker/mandatory-state → S4.1/2 | Independent integration over qualified delivered intervals; partial-window allocation, rounding, boot/policy epochs and outcomes. Include legitimate persistent drift and slow base changes. | Useful oscillator discipline beside alternating signs, stable relabelling, cancellation/replacement, reload, reconnect, SCM restart and lost acknowledgement. No delivered-authority refund or silent renewal; unknown remains charged. CHECK-3/10/12/15. |
| Initial uncertainty, growth model, acceptable error/age, base validity, holdover expiry | Reference/timestamp/path/estimator/oscillator terms and model version; Core/engine/S0.7 → S4.3/5 | Separate shared bias, dependence/coverage assumptions, temperature/load/power strata, accepted-observation age and unknown actuator effects. | Useful bounded historical prediction during source loss beside expired model, correlated delay and resume discontinuity. Quality expires honestly; prediction grants no fresh vote or exceptional-step authority. Fresh reacquisition preserves quorum. CHECK-7/11/14. |
| Active delivery, elapsed freshness, mapping/boot epochs, finite leap guards | Paired UTC/QPC/unbiased-time brackets, power events and external reference; W11-W02/S0.4 → S4.3 | Cold boot, restart, S3/S4, Modern Standby, Fast Startup and VM restore; qualify actual native delivery and mapping rather than clock labels alone. | Healthy same-segment tracking/reacquisition beside crossing/late observations. Sleeping QPC time is not inferred delivery; ambiguous boot cannot renew budgets. Windows alone executes explicit leaps; fresh post-guard evidence admits only measured residual recovery. CHECK-10/11/25/27. |
| Stop/wait/native-job lifetime, restart/backoff, recovery exposure, permitted final mode | Actual job custody, current-owner cleanup and mode/trajectory; OPEN-13, W11-W04/05 → S4.4/5 | Separate native completion/resources, engine stop and broker cleanup; record attempts, censored/unrecovered cases and fault/load/power coverage. | Healthy orderly cancellation/owned recovery beside stalled work, broker death, crash loops, queued maintenance restart and successor/ambiguous ownership. Timeout frees no actual-work capacity; no blind reset. Observe product cleanup before harness restoration. CHECK-5/10/12/13/15/21/27. |

#### Actual combined movement and conditional uncertainty

For a precise interpretation to prepare under THR-19, let `c(a)` be the **actual
combined applied rate contribution**, in seconds/second against one qualified
actuator reference, and `a` qualified delivered active seconds. For window `W`:

`A_W = Σ |actual_step_i| + ∫_W |c(a)| da`

`D_W = Σ actual_step_i + ∫_W c(a) da`.

Scale ppm by `10^-6`. Use `c_b + c_t` only after establishing both terms are additive
in this same coordinate; unrelated native values or different ppm denominators
cannot be added. Chrony's inspected composition is multiplicative in its own
coordinates. Retain component records and base-change limits, but charge actual
combined movement, including stable compensation. Signed net movement alone,
transient-only charging and requested values cannot satisfy THR-19. Window
membership, partial intervals, rounding and conservative unknown allocation still
need explicit rules. Finite-window authority proves no permanent UTC-error bound.

A **conditional derived model** is `U(h) ≤ U0 + ∫_0^h b_f(t) dt + U_events(h)`
when initial error is bounded by `U0` and residual fractional-frequency error by
`b_f(t)` over the declared environment. `U_events` carries separately bounded
discontinuity, actuation and mapping error. Statistical estimates need explicit
coverage/dependence assumptions. Do not double-count drift/temperature or hide
shared bias through unjustified root-sum-square combination. Candidate holdover
expiry is the earliest approved age limit, model-validity expiry or uncertainty
crossing; none is selected here. Independent expected movement/error comes from
raw reference evidence and separate calculations, not the controller/formatter.

RFC 5905 distinguishes dispersion, jitter and root distance. RFC 8915 §8.6's
MAXDIST wording must not be mechanically mapped to a field already containing
dispersion plus half delay: define RTT, dispersion and half-delay first.
Verified RFC errata coverage was not established for this pass.
Chrony's linear dispersion and ntpd-rs's variance propagation use different
assumptions; neither supplies a universal holdover bound. See primary sources below.

#### Late physical settlement and current model publication

The inspected pure chain is retained history → genuine combination →
`FixtureReferenceInputState` → modeled settlement → existing affine composer.
Reuse [the rate receiver at the inspected pin](https://github.com/DavidOsipov/NevroTick/blob/09caf0d3303abafab1edba0064a1c9c117514b70/crates/time-core/src/reference_input/rate_delivery.rs)
and the [existing Core task](s1.3-source-selection-readiness.md), keeping their
fixture scope. Active and monotonic endpoints are independently ordered; local
ordinals/mapping/high-water marks survive chart replacement. Completed and
Cancelled account their delivered portion identically; ProvenNoEffect consumes
the ordinal, Unknown blocks the cursor, and post-admission arithmetic failure
cannot refund consumption. Caller tags are not native provenance or durable IDs;
the `2^53`-nanosecond exact-arithmetic guard is not a production holdover limit.
Outward arithmetic error is distinct from physical/model uncertainty.

The future **broker/mandatory-state owner** retains native-operation custody and
settles its actual late outcome against the original durable reservation even
after cancellation, source/clock-generation invalidation, model rejection or
loss of Ready. Persist the outcome before successful applied acknowledgement.
Accounting an old effect grants no new actuation authority; reconciliation that
needs another clock write still requires current ownership. Unknown effects stay
conservatively charged and unsafe follow-on effects remain blocked.

The **Core publication owner** separately admits usable feedback against current
model identity/generation and qualified physical mapping. Stale results restore
neither Ready nor fresh confidence and authorize no correction; resulting mapping
uncertainty follows invalidation/reacquisition policy. The Ready-only fixture
receiver cannot provide late durable custody. Preserve its guard; do not create
a second ledger or claim an implemented producer. Mandatory full-domain proofs
remain due at the dependent mathematical gate, separate from physical evidence.

#### Native API and sample limits

Microsoft explicitly gives the legacy adjustment API 100 ns units; the precise
getter/setter pages do not specify that interpretation. Preserve raw adjustment,
increment, mode and QPC frequency. Qualify the sample's QPC-scaled conversion,
rounding and actual slope; integer width, one input unit and the sample's
1,000 ppm guard establish neither an effective quantum nor a supported maximum.
The sample restores with `FALSE` irrespective of saved mode, so it is not a
general restoration recipe. Returning control with `TRUE` is a Windows handover,
not transient cancellation while retaining product ownership.

`AdjustTokenPrivileges` Boolean success can accompany `ERROR_NOT_ALL_ASSIGNED`;
it cannot add a missing privilege. Qualify effective tokens, not account names
or a disabled bit. `SYSTEMTIME` field limits establish no approved target-date
interval or achieved accuracy; `SetSystemTime` supplies no journal transaction
or durable effect identity. Repeating an absolute target later can move time
backward. Reuse selected `windows`/`windows-sys` bindings behind narrow adapters;
exact SDK/declarations/features remain qualification inputs, not new dependencies.

QPC includes sleep, while unbiased interrupt time excludes it; neither proves
broker execution. High-precision UTC reads alone establish no external UTC error.
Keep full read brackets and independent reference uncertainty. Service STOP,
timer signalling and configured SCM recovery delay establish no rate expiry.

#### Ordered handoff through existing owners

1. **S0.7 with P4/W11 owners:** complete definitions/provenance and explicit
   unknowns, identifying the next dependent choice through OD-04. Before a later
   authorized write, freeze candidate/artifact, SDK/bindings, supported build,
   hardware/power/protection configuration, exact operation/rate/phase/date/
   duration/count/attempt bounds, observer uncertainty, stop trigger and independent
   restoration owner. Lab safety limits and production policy are separate;
   unset lab bounds permit no write.
2. **S0.4/W11-W02 produces backend evidence:** begin read-only capability and
   legitimate denial observations. Only an admitted bounded native child measures
   signed conversion, mode/base, quantum, effect brackets and power delivery.
   Return qualified/unsupported cells to broker/Core; W11-W04/05 qualify effective
   tokens, protections and lifecycle authority under the actual configuration.
3. **S0.7 resolves dependent policy; S4.1/2 implements enforcement:** trace the
   admitted proposal through current guards, durable reservation, guarded effect,
   durable known/unknown outcome and acknowledgement. Fault persistence/effect/
   reply seams; retain original operation identity on duplicates. Observe both
   the useful accepted neighbor and the refused neighbor at the real effect.
4. **Core consumes qualified actual feedback:** connect the real producer to
   current receiving guards and existing numerical owners without refreshing
   observation age or changing uncertainty meaning. Independently qualify complete
   and partial-cancel journeys beside stale/unknown/arithmetic-failure controls.
   Broker settlement survives every model rejection.
5. **S4.3/4 with W11-W02/05 qualifies transitions:** cancellation/replacement,
   guard, sleep/resume, engine/broker death, ownership transfer and maintenance.
   Preserve mandatory records and usable recovery code/rights; retained package
   files or SCM configuration alone prove no independent recovery. Existing
   W11-W05/W11-W10 removal questions remain unresolved.
6. **S4.5/reference QA returns trajectories/distributions to S0.7:** include
   unsuccessful attempts and observation uncertainty. Compare candidate defaults,
   hard ceilings, administrative narrowing, legitimate drift/recovery and
   compromised-engine damage under the same population. OD-04 owns the concrete
   decision and subsequent validation; this card selects no production profile.

For residual error, compare the retained **actual rate with the intended recovery
state in the same coordinate**; their difference need not equal a named transient
term. `|residual_rate| × exposure_time` is conditional on both bounds and delivery
semantics, with uncertainty retained. Measured maxima, stop targets and configured
recovery delays supply no all-failures exposure bound. If recovery can remain
unavailable indefinitely, a nonzero retained rate has no demonstrated unconditional
finite residual ceiling. Statistical coverage requires representative sampling
and dependence assumptions; retain censored/unrecovered cases. No watchdog or
lease mechanism is selected. Observe product cleanup before independent lab
restoration; the latter is not evidence of product recovery.

#### Remaining inputs and current priority

Open inputs are native conversion/quantum/build/power mapping; ordinary and
exceptional rate/step/date policy; oscillator/environment bounds; uncertainty
meaning/coverage/age; windows/renewal/unknown allocation; trustworthy boot epochs;
durable reservation/outcome representation and filesystem evidence; effective
current-owner recovery; native-work/stop/restart budgets; and accepted residual
exposure/availability tradeoffs. Production quorum, fresh-vote/floor-supersession
and era decisions remain with R-02. Source review closes none of these gates.

[Development §7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
remains first, including the current proof continuation and unresolved public
era-constructor/held verifier work. Native/S134 stop, separate fuzz no-retry hold,
optional-storage pause, limited Sandbox scope and UI-M0 D1–D3 deferrals remain.
Direct-first acquisition, selected Slint and complete MSIX are unchanged. These
later measurements add no prerequisite to unrelated current Core work.

#### Primary evidence and transfer limits

- **Chrony:** selected [generic driver](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/sys_generic.c) and [reference controller](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/reference.c); comparison [current driver pin](https://github.com/mlichvar/chrony/blob/c97d8083cf9a9edc64d53236565916d326d8a109/sys_generic.c); [4.9 manual](https://chrony-project.org/doc/4.9/chrony.conf.html). Base/transient separation and driver-return feedback transfer as concepts; startup allowances, callback cleanup and Linux constants supply no durable Windows authority or recovery guarantee.
- **ntpd-rs v1.9.0:** [controller](https://github.com/pendulum-project/ntpd-rs/blob/46ec9bb4d5b6cb24f814f5543d85b9138afb4cba/ntp-proto/src/algorithm/kalman/mod.rs), [variance model](https://github.com/pendulum-project/ntpd-rs/blob/46ec9bb4d5b6cb24f814f5543d85b9138afb4cba/ntp-proto/src/algorithm/kalman/source.rs), [clock trait](https://github.com/pendulum-project/ntpd-rs/blob/46ec9bb4d5b6cb24f814f5543d85b9138afb4cba/ntp-proto/src/clock.rs). The accumulated counter excludes slews/startup and resets with controller construction; it cannot satisfy THR-19. Source defaults are not product limits.
- **Microsoft rate/privilege contracts:** [legacy setter](https://learn.microsoft.com/windows/win32/api/sysinfoapi/nf-sysinfoapi-setsystemtimeadjustment), [precise setter/sample](https://learn.microsoft.com/windows/win32/api/sysinfoapi/nf-sysinfoapi-setsystemtimeadjustmentprecise), [precise getter](https://learn.microsoft.com/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimeadjustmentprecise), [privilege results](https://learn.microsoft.com/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges), [step](https://learn.microsoft.com/windows/win32/api/sysinfoapi/nf-sysinfoapi-setsystemtime) and [SYSTEMTIME](https://learn.microsoft.com/windows/win32/api/minwinbase/ns-minwinbase-systemtime). Sample policy and representation are not tested product bounds.
- **Windows timing/lifecycle:** [QPC](https://learn.microsoft.com/windows/win32/sysinfo/acquiring-high-resolution-time-stamps), [unbiased time](https://learn.microsoft.com/windows/win32/api/realtimeapiset/nf-realtimeapiset-queryunbiasedinterrupttimeprecise), [service status](https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-setservicestatus), [failure actions](https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_failure_actionsw), [queued restart behavior](https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-changeserviceconfig2w). A finite action array can repeat its final action; configuration and wait completion do not prove bounded cleanup.
- **Protocol/model:** [RFC 5905 §§7.2/11.2.3](https://www.rfc-editor.org/rfc/rfc5905.html), [RFC 8915 §8.6](https://www.rfc-editor.org/rfc/rfc8915.html#section-8.6), [RFC 8633](https://www.rfc-editor.org/rfc/rfc8633.html), [NIST SP 1065](https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication1065.pdf). Delay, source dependence, assumed frequency tolerance and stability statistics need explicit interpretation.
- **Statistical recovery claims:** [NIST tolerance intervals](https://www.itl.nist.gov/div898/handbook/prc/section2/prc264.htm), [PNNL nonparametric upper tolerance limits](https://vsp.pnnl.gov/help/vsample/Design_UTL_Non_Parametric.htm). Population coverage/confidence under sampling assumptions is not an actuator lease or an all-failures deadline. Primary sources inspected 5 November 2026; no native measurements were executed.

#### Reconciliation and independent review — 5 November 2026

**Completed workflow:** separate GitHub, primary-web and Microsoft Learn research
lanes supplied a fourth analyst's Markdown synthesis; a fifth agent integrated
seven existing documents. Initial fresh launches failed with context-window
errors and supplied no usable role result; one bounded retry per role completed.
The integrated card retains its primary pins and transfer limits above.

| Obligation or relationship | Reconciliation with the inspected predecessor |
| --- | --- |
| Broker authority and actual movement | Existing THR-19, component/combined ceilings, durable reservation and conservative unknown charging remain. The worksheet defines a common qualified rate coordinate without adopting values or an IPC schema. |
| Late physical outcome and model publication | Broker custody must survive model rejection; current Core admission remains separate. The actual Ready-only fixture contract is retained, with no second ledger or claim of native implementation. |
| Residual recovery claim | The former illustrative transient-rate expression is narrowed to actual rate relative to the intended recovery state and explicitly conditional rate/exposure bounds. No measured maximum or configured delay is promoted to hard expiry. |
| Policy and qualification | OPEN-5/7/13, OD-02/04 and existing S0.4/S0.7/P4/W11 owners remain. Physical evidence, model assumptions, tolerances and approval are distinct; later values, effects and decisions remain open. |
| Current work and settled decisions | Development §7, the native/S134 stop, separate fuzz hold, storage pause, Sandbox/UI scope, Direct-first, selected Slint, complete MSIX and optional enhanced release provenance remain unchanged. |

**Frozen review identity:** upstream base
`09caf0d3303abafab1edba0064a1c9c117514b70`, full tree
`c866a9e6bf9ef518142cc0d1a38318749800cac3`; seven changed Markdown files,
47,732-byte patch, SHA-256
`b176c6711bbd270fd42e6b6632cad016082fc9558bd9acdb2fc5c546016cd070`.
The local `231d3e01a3447354a15ce0a7dba0138f0f1a6813` is a synthetic
partial-snapshot identity, not upstream history or a publishable tree. The
coordinator matched all seven baseline blobs against the complete remote tree;
the reviewer independently checked the local patch and candidate file identities.

**Review completion:** `clock_review` received fresh context with
`fork_turns: none` and read-only bounded scope. The first launch failed before
producing a review (`not run`); one bounded retry completed the requested review
(`completed`, `READY` for this documentation handoff). Its separate upstream
fact-check launch also failed without usable evidence; the reviewer performed
the primary checks directly. The raw report records actual inputs and limits.
No material `CLK-R` finding was established, so no finding-resolution cycle was
needed. Raw-report SHA-256:
`e3cf4e9fc1ac657190cf3662c8ece4d3175b2a5cd4a2f4a2a58fbdbfb94cc601`.

**Coordinator verification:** all seven files remained frozen throughout review.
Whitespace checks, 1,101 internal Markdown link occurrences and all 23 scoped
preservation checks passed. The five architecture Mermaid blocks, canonical
REQ/BND/DEC/THR/CHECK/OPEN identifiers, 136 earlier source entries, existing task
history, research obligations and current proof-priority prefix are retained.
No product, proof, native or fuzz experiment was executed, and no dependency,
production policy value, clock, service or trust store was changed.

The review verdict covers the frozen preparation patch, not this subsequently
appended receipt. The complete raw report below, including the reviewer's own
comprehension paragraphs, is retained verbatim. Its scoped readiness supplies no
production measurement, mathematical proof, risk acceptance or release approval.

#### Verbatim independent review

> The proposed change prepares the unresolved production clock limits for later decisions and qualification. It puts one field, evidence and control matrix in the existing Windows integration task, connects it to the existing phase owners, and clarifies three relationships: cumulative authority follows actual combined actuation, durable settlement of a physical operation survives rejection by the current model, and holdover or recovery error claims depend on explicit physical and exposure assumptions. It changes documentation and research routing without selecting production numbers or admitting an experiment.
>
> The relevant existing implementation is a pure fixture receiver and affine numerical machinery. The receiver accepts modeled terminal events only against its current Ready chart and local cursor; it provides neither Windows effect provenance nor persistent broker custody. A production broker, durable reservation/outcome integration and qualified physical feedback remain planned. Current work remains the Core mathematical-proof continuation; the blocked S134 artifact, separate fuzz hold, optional-storage pause, limited Sandbox authorization and UI-M0 deferrals remain applicable. The selected Windows-only explicit leap executor and the wider authenticated-recovery goal remain requirements awaiting their respective evidence.
>
> ## Candidate identity and completion
>
> - **Reviewer:** independent `clock_review` reviewer; completed one bounded, read-only documentation-stage review on 5 November 2026.
> - **Context:** fresh review context containing the bounded request and project instructions. I did not retrieve or use parent discussion, private KB/memory, the analyst report or the candidate author's verdict. A narrowly delegated upstream fact-check attempt failed without a usable result; it contributes no evidence or additional completed review. The primary-source checks below were performed directly by this reviewer.
> - **Reported real upstream base:** `09caf0d3303abafab1edba0064a1c9c117514b70`; reported full upstream tree `c866a9e6bf9ef518142cc0d1a38318749800cac3`. These are the review packet's upstream identities; I did not independently reconstruct the full remote tree.
> - **Observed synthetic local HEAD:** `231d3e01a3447354a15ce0a7dba0138f0f1a6813`. This partial materialization is not upstream history, and absent unrelated files are not treated as upstream deletions.
> - **Frozen patch:** `clock-limits-review-candidate.diff`, SHA-256 `b176c6711bbd270fd42e6b6632cad016082fc9558bd9acdb2fc5c546016cd070`. Both the supplied patch and the current seven-file Git diff produced this hash.
> - **Scope:** seven Markdown files, 304 insertions and one deletion. Git status showed exactly these seven modified files. All seven SHA-256 values and Git blob identities matched `clock-limits-review-candidate.json`; the final hash check still matched. `git diff --check` returned success with no diagnostics. No product code, dependency or policy value changed.
>
> | Reviewed file | Verified SHA-256 |
> | --- | --- |
> | `docs/README.md` | `1641fc7e436bcb77cb11b215403b0764c88e7a753e674f7e26cb95daa7b0866c` |
> | `docs/nevrotick-research-handoff.md` | `279d82b7c068e89e5b126e6e1f7092af18cfe8dd070f390d7fbb22d7bf552474` |
> | `docs/plans/development-plan.md` | `0d23db72d736243864cdef26931141bbf4905b8d55254bf2f174d5f3f95181f2` |
> | `docs/windows-time-architecture.md` | `1b7dc029078316ee34c0d423e4028f5c0fc98dee579df580251727b5fea9e3b7` |
> | `tasks/README.md` | `440ea9aa1c93a95d77b890d17912319cdd951b8aa6dccc03448957b728a85132` |
> | `tasks/development-backlog.md` | `5bc2e8154898d48ad3b58d28c95bce0093c6ae0e92cb944927f417ae0f380cc1` |
> | `tasks/windows11-integration.md` | `29428a0a8b696efd3496d44093b2ff2766362c2e3a43422eb630e91cfc7960c9` |
>
> ## Inputs actually inspected
>
> I read the request and manifest, the complete frozen patch, and the changed text against the following bounded current authorities and source paths:
>
> - `AGENTS.md`, `docs/README.md`, `.agents/skills/planning/plan-independent-review/SKILL.md`, `.agents/skills/planning/references/review-and-handoff.md`, `.agents/skills/testing/qa-independent-review/SKILL.md` and `.agents/skills/testing/CONTRACT.md`. The local inventory exposed no nested `AGENTS.md`.
> - `docs/windows-time-architecture.md` §§5.3, 10.1–10.5, THR-19, §17.1 and the relevant OPEN-5/7/13 records; `docs/nevrotick-owner-decisions.md` OD-02 and OD-04; `docs/nevrotick-research-handoff.md` R-02/R-03, R-07/R-08 and completion criteria.
> - `docs/plans/development-plan.md` P4, §7's current proof continuation and execution holds, and the changed §8.5 routing; `tasks/development-backlog.md` S0.4/S0.7 and P4; the changed task-index routing; `tasks/windows11-integration.md` authority/scope, W11-W02/04/05, §4.19's Sandbox boundary and the complete new §4.31.
> - `tasks/s1.3-source-selection-readiness.md` modeled settled active-time delivery contract, the historical four-owner qualification receipt and the current Fraction comparison proof contract. The receipt was read for its declared scope, not re-executed or independently requalified.
> - `Cargo.toml`, `crates/time-core/Cargo.toml` and `crates/time-domain/Cargo.toml`; the complete `crates/time-core/src/reference_input/rate_delivery.rs`; the current identity/readiness, mapping and feedback owners in `crates/time-core/src/reference_input.rs`; settled-rate effect/composition in `crates/time-core/src/regression/chart.rs`, corresponding conversion/integration/error machinery in `crates/time-core/src/regression/chart/roundoff.rs`, and the guarded integer conversion in `crates/time-core/src/regression/normalization.rs`.
>
> Direct primary-source inspection covered the consequential changed claims:
>
> | Primary input | Inspected scope |
> | --- | --- |
> | Microsoft [precise setter and sample](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-setsystemtimeadjustmentprecise), [precise getter](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimeadjustmentprecise), [legacy setter](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-setsystemtimeadjustment) and [privilege results](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges) | Parameter/mode documentation, sample QPC conversion and 1,000 ppm guard, fixed-`FALSE` restore calls, Boolean-only sample privilege check, and documented `ERROR_NOT_ALL_ASSIGNED` behavior. |
> | Microsoft [QPC guidance](https://learn.microsoft.com/en-us/windows/win32/sysinfo/acquiring-high-resolution-time-stamps), [unbiased time](https://learn.microsoft.com/en-us/windows/win32/api/realtimeapiset/nf-realtimeapiset-queryunbiasedinterrupttimeprecise), [service failure actions](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_failure_actionsw) and [queued restart behavior](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-changeserviceconfig2w) | Sleep inclusion/exclusion and precision limits; repetition of the final failure action; queued restart after a subsequent manual start/stop. |
> | Chrony [generic driver](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/sys_generic.c) and [reference controller](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/reference.c) at the selected immutable pin | `update_slew` composition, driver-return feedback, achieved-slew accounting and callback scheduling; `get_root_dispersion`'s elapsed-time growth terms. |
> | ntpd-rs [controller](https://github.com/pendulum-project/ntpd-rs/blob/46ec9bb4d5b6cb24f814f5543d85b9138afb4cba/ntp-proto/src/algorithm/kalman/mod.rs) and [source model](https://github.com/pendulum-project/ntpd-rs/blob/46ec9bb4d5b6cb24f814f5543d85b9138afb4cba/ntp-proto/src/algorithm/kalman/source.rs) at the cited pin | Startup versus accumulated-step checking, step versus slew branches, construction with default time data, and actual covariance/process-noise propagation. |
> | [RFC 5905](https://www.rfc-editor.org/rfc/rfc5905.html), [RFC 8915 §8.6](https://www.rfc-editor.org/rfc/rfc8915.html#section-8.6), and [NIST tolerance intervals](https://www.itl.nist.gov/div898/handbook/prc/section2/prc264.htm) | MAXDIST/root-distance definitions and half-delay interpretation; the difference between observed sample coverage and population coverage/confidence. |
>
> The three researcher reports were not needed as review oracles and were not consumed. The analyst report was not opened. Initial direct retrieval failures for some URLs were resolved through English Microsoft pages or the GitHub connector at the exact pins; no failed retrieval was counted as inspected evidence.
>
> ## Scoped verdict: READY
>
> **Ready as the specified documentation/research handoff. No material CLK-R finding was established in this candidate.** This verdict does not admit implementation, select numerical policy, qualify Windows behavior, establish a product proof, accept residual risk or approve release.
>
> The reasons for readiness are tied to the actual changed boundaries:
>
> 1. **Aggregate correction authority remains with the broker.** The proposed absolute-movement and signed-displacement definitions in §4.31 preserve architecture §5.3 and THR-19: stable compensation is included; cancellation, sign changes, reloads and restarts cannot refund delivered work. Separate component ceilings survive. The handoff requires a common qualified coordinate before adding rate components. This is necessary given the inspected Chrony composition, which scales the temporary term by the base rate and uses the driver-returned result. Window allocation and unknown outcomes remain explicit unresolved inputs rather than implicit policy defaults.
>
> 2. **The physical-outcome and current-model owners are coherently separated.** In the actual source, `settlement_parts` calls `reference_chart_identity` before accepting any event; that guard requires Ready and the exact chart instance. Known events then consume the cursor and invalidate the old model before fallible integration/composition. Unknown blocks the cursor. Architecture §10.2.1 and §4.31 accurately preserve these fixture limits while assigning future late durable settlement to broker/mandatory state. They do not instruct an implementer to weaken the Ready guard, erase a reservation after cancellation, or publish stale confidence. Any reconciliation requiring another write still needs current ownership.
>
> 3. **Physical uncertainty is not promoted from arithmetic or upstream statistics.** The `2^53` duration guard is a numerical conversion boundary, and the existing outward bounds concern exact stored operands. The text correctly keeps both separate from production holdover validity, actuator quantization and reference error. Its holdover inequality is expressly conditional on initial error, residual frequency and event bounds. Fresh source votes and exceptional recovery authority are not renewed by prediction, matching R-02 and OD-02. The RFC MAXDIST caution avoids assuming that a root-distance field is the same quantity as round-trip delay.
>
> 4. **Native and recovery claims have appropriate limits and paired controls.** The Microsoft sample supports the identified mode-restoration and privilege-check cautions; it supplies no tested product ceiling. The changed residual-rate calculation uses the difference from the intended recovery state, not an assumed transient label. Its finite exposure condition is explicit. SCM configuration and statistical observations are not represented as an all-failures expiry guarantee. Successful cleanup must be observed before fixture restoration, and unsuccessful/censored recovery attempts remain in the proposed evidence. Missing native qualification is an acknowledged later-stage obligation, not a demonstrated product failure.
>
> 5. **The handoff has existing producers, consumers and exits without displacing current work.** S0.4/W11-W02 produces qualified backend evidence; W11-W04/05 owns effective security and lifecycle observations; S0.7/OD-04 owns the dependent policy choice; S4.1/2 enforces and settles; Core admits current usable feedback; S4.3/4 and S4.5 provide transitions and independent trajectory evidence. The matrix pairs useful drift/recovery/holdover with refused or failed neighbors. It retains OPEN-5/7/13, R-02's era/quorum/floor choices, package-removal recovery questions and mandatory mathematical proof obligations. The new routing explicitly preserves development §7 and all applicable holds and deferrals.
>
> ## Evidence limits and invalidation conditions
>
> This was a source and document review, with read-only identity/whitespace checks. I ran no Cargo, native, proof, fuzz, clock, service, package or release campaign and changed no candidate file. I did not audit the full repository, all upstream implementations, every linked reference, all RFC errata, the exact Windows SDK/binding graph or the full remote tree. Historical qualification receipts were not transferred to the current candidate as new execution evidence. No unavailable later-stage measurement is classified as a failed product test.
>
> Production rate/step/date values, authority-window renewal/allocation, physical clock mapping, holdover coverage, durable representation and filesystem semantics, supported configurations, effective recovery and native-work budgets remain open. Those gaps do not prevent this handoff because its consumers, required observations and disabling conditions are stated. They do prevent the corresponding production claims or effects until their existing gates are met. A change to the frozen patch, governing owner decisions, receiver semantics or cited primary facts invalidates the affected conclusions and requires scoped reconciliation.
>
> ## One permitted next action
>
> The coordinator may retain this completed raw review verbatim in the existing `tasks/windows11-integration.md` §4.31 handoff and carry its scoped READY disposition forward, preserving the current development §7 continuation and all later decision/execution gates.

### 4.32 Two-family Windows qualification matrix — 5 November 2026

**Direction accepted; qualification open.** [OD-14](../docs/nevrotick-owner-decisions.md#18-od-14--compact-windows-platform-scope)
records David's Windows 11 26H2 plus corresponding Server instruction. The
researched counterpart is **Server 2025 LTSC**, not a Server “26H2” or an identical
client/server build. Pro/Standard and Desktop Experience below are engineering
fixture choices, not all-edition support promises. OPEN-2 stays open. This card
records no environment probe, product execution or first-stable approval.

#### Two intended rows

Both use `x86_64-pc-windows-msvc`, OD-07's intended **x86-64-v3**, the complete
hardened MSIX and selected Slint UI. The [current generic configuration](../.cargo/config.toml)
still uses `target-cpu=x86-64`; it does not qualify the intended migration.
Exact serviced UBR/KB selection remains an input: the latest listed optional
preview is not automatically the required baseline. [WM1]

| Intended family and first fixture | Suitable evidence environment | Current disposition, availability/result and missing inputs |
| --- | --- | --- |
| **Windows 11 26H2 x64**; Pro, GAC, build family **26300**; workgroup baseline | Owned persistent client VM for tokens, SCM, installation, sessions and reboot/recovery. Named client hardware for physical timing/power claims and CHECK-21. | **Fixture planned; execution held at existing gates. Availability unverified; complete-product result not run.** Need exact image/UBR/KB, owned capacity/restoration, complete admitted candidate, effective policy and independent observers. |
| **Windows Server 2025 x64**; Standard, LTSC, **Desktop Experience**, family **26100**; standalone non-DC baseline | Owned persistent Server installation with interactive desktop and unattended services. Matching **Server** hardware/evidence for physical timing/power and CHECK-21 claims. | **Fixture planned; execution held at existing gates. Availability unverified; complete-product result not run.** Need Server fixture, exact servicing, feature/manifest/dependency map and supported acquisition/licensing/install/update route, plus the same candidate/observer inputs. |

Server Core and further editions are outside these first fixtures, without a
universal incompatibility finding. Image publication is not owned availability;
availability is not fitness for a claim. A suitable VM can establish many native
enforcement/lifecycle properties, including supported HVCI configurations.
Physical oscillator/power claims need matching hardware: VM pause/restore and
one sleep class cannot qualify every S3/S4/Modern Standby case. Record actual
supported states and leave omitted claims unqualified. [WM2]

#### Shared case groups, applied to each row

These reuse W11-W02/04/05/10, S0.4/S0.7 and P4/P6/P7; UI retains S0.5/P5.
The [acceptance catalog](../docs/windows-time-architecture.md#202-acceptance-catalog)
owns requirements; groups below are neither new CHECK IDs nor executed passes.

| Group and existing checks | Independent useful/refused observations and owner |
| --- | --- |
| **Admission and enforced protection** — CHECK-1/16/20/26/27 | W11-W05/10 and build/security owners bind actual activation, token, early/lazy modules and queried per-role enforcement. Useful permitted work/load succeeds; wrong images/actions are denied; missing or query-failed **required** protection prevents readiness. Keep VBS configured/running, HVCI, App Control audit/enforced and process CET/CIG distinct. Preserve hardening §§6.4/6.7's required/optional classification and explicit optional dispositions. |
| **Authority and time ownership** — CHECK-1/2/5/12/15 | W11-W04/05 observe ordinary, filtered/elevated and delegated/revoked principals at the actual protected effect. UI/engine cannot gain broker clock authority; current authorized work succeeds. Managed domain members default to observation until an **administrator explicitly authorizes takeover**, with existing policy/ownership/restoration checks. Controlling GPO or renewed competing ownership stops new actuation without fighting policy. Workgroup is the first positive fixture, not a new product restriction; DC/PDC time service remains a non-goal. |
| **Lifecycle, timing and resources** — CHECK-5/9–13/21/23/27 | W11-W02/05/10, clock and performance owners observe boot before login, UI exit, real restart/restore, update/Reset/removal and pending recovery. Stale generations produce no new effects; fresh admitted recovery works. Distinguish UTC, QPC and active-awake time; record hypervisor/provider interventions and independent effect/accuracy observations. Observe product recovery and retained resources **before** laboratory teardown. Wrong-clock/offline trust retains its separate verifier controls. |

Attach [§17's resource evidence](../docs/windows-time-architecture.md#17-resource-and-performance-targets)
to **each claimed OS profile**, retaining the applicable one-hour baseline and
72-hour soak at their existing phase gates, hardware/load/power identity and
instrumented versus uninstrumented measurements. Client results cannot fill
Server cells. Short Slint idle samples do not replace CHECK-21; full production
UI/accessibility checks retain their existing re-entry gates.

Directly cover known interactions: final Store signer × actual SCM activation ×
complete early/lazy module graph × enforced role policy; service/update startup ×
CPU admission; and interrupted removal × pending recovery. Pairwise coverage
percentages cannot waive these interactions. Keep clock-limit policy and
measurement ownership in [§4.31](#431-clock-correction-limits-preparation--5-november-2026).

#### Three admission obligations and Server delivery

W11-W05/W11-W10 with the build owner resolve one compact card before dependent
implementation through [MSIX §§4/7.1](../docs/plans/windows-msix-store.md#4-services-capabilities-and-the-unresolved-provisioning-bridge):

1. **Package applicability:** qualify actual device-family/schema, feature and
   dependency support. A `MinVersion` at 26300 excludes intended Server 26100;
   a lower common minimum permits unintended clients. `MaxVersionTested` is not
   an upper support bound. Do not invent a `Windows.Server` family. [WM3]
2. **Product OS-family admission:** distinguish the selected product/release
   from registration permission. The supported manifest/activation/enforcement
   mechanism remains unresolved; a numeric minimum alone cannot select both
   intended families and exclude all others.
3. **Safe CPU admission:** a below-v3-compatible prerequisite must execute before
   **every** v3 payload and startup dependency, including direct, SCM, helper and
   update paths. Check the complete feature set and usable OS vector state;
   challenge masked/unsupported cases before effects. A globally v3-compiled
   guard or an illegal-instruction crash does not qualify refusal. Usable v3
   without optional AVX-512 retains the baseline path under
   [hardening §6.7](../docs/windows-time-development-hardening.md#67-v3-baseline-and-runtime-cpu-dispatch).

Current App Installer/WinGet support for Server 2025 does not establish this
complete service-bearing package's supported acquisition, licensing or update
route. Conversely, older Server 2019 Store restrictions do not prove universal
Server 2025 impossibility. Keep this precise delivery input unresolved. [WM4]
Local test-signed mechanics precede David's controlled Store attempt; actual
Store-delivered bytes and effective protections require their own checks before
broad release. No ordinary-utility custom action, alternate installer or split
package is selected to bypass the gap.

#### Receipt identity and ordered handoff

Preserve [UI-M0](ui-m0.md), [Slint pilot](s0.5-ui-comparison.md#slint-software-renderer-live-host-result--5-november-2026)
and [Sandbox §4.19](#419-owner-ready-sandbox-scope--3-november-2026) at their actual
pins/scopes. The Sandbox smoke executed built-in exit controls, **no NevroTick
payload**, and established neither effective protections nor an unelevated
token. Do not combine missing environment fields from different runs. Portable
CI, Windows cross-builds and hosted Server image descriptions qualify no current
client or complete-product row.

Bind each eventual receipt to source plus relevant dirty bytes; toolchain/SDK,
features and native/generated closure; local/submitted/delivered package hashes;
OS edition/channel/install mode/build/UBR/KB and pending reboot; CPU/vector state,
firmware/VM exposure; actual tokens, protection, domain/GPO, time providers,
power/network/trust; CHECK owner, independent oracle, useful/denied observations,
raw evidence and product cleanup. Keep proposed/held **disposition**,
available/unavailable/unverified **availability**, claim-specific **fitness** and
not-run/partial/failed/passed **result** separate. Missing infrastructure is an
input, not a failed product test; an isolated pass does not pass the whole row.

Revalidate affected claims after changes to bound bytes, servicing, firmware,
CPU masking, policy/providers, activation or delivery. Preserve scoped historical
passes and reuse unchanged evidence only with explicit identity correspondence.

The next owners proceed at existing prerequisites: platform/package/build resolve
the three admissions and delivery question, then freeze candidate/profile;
fixture owner establishes actual availability, fitness and restoration; admitted
mechanism/authority checks precede persistent lifecycle and row-specific timing,
power/resources; David performs the controlled Store attempt; W11-W10/P7 qualify
delivered bytes and obtain the support/release decision. No hardware or arbitrary
UBR choice is required to complete this documentation card.

[Development §7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
stays first: qualified comparison/complement, open timed-out addition and the
conditional bounded Verus pilot retain their stated scopes and fallback.
Native/S134 stop, separate fuzz no-retry hold, optional-storage pause, limited
Sandbox scope, UI-M0 D1–D3, Direct-first, Slint, complete MSIX and optional
enhanced release provenance remain. This card creates no immediate campaign.

#### Compact primary-source basis

Sources checked 5 November 2026; mutable platform pages require revalidation for
an actual candidate. They establish platform facts, not NevroTick qualification.

- **WM1:** [Windows 11 release information](https://learn.microsoft.com/en-us/windows/release-health/windows11-release-information), [Server release information](https://learn.microsoft.com/en-us/windows/release-health/windows-server-release-info), [update release cycle](https://learn.microsoft.com/en-us/windows/deployment/update/release-cycle): product/build families and servicing types.
- **WM2:** [Server installation options](https://learn.microsoft.com/en-us/windows-server/get-started/install-options-server-core-desktop-experience), [power states](https://learn.microsoft.com/en-us/windows/win32/power/system-power-states), [effective Device Guard state](https://learn.microsoft.com/en-us/windows/security/hardware-security/enable-virtualization-based-protection-of-code-integrity): environment and enforcement distinctions.
- **WM3:** [TargetDeviceFamily schema](https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-f-targetdevicefamily): applicability and tested-version semantics; actual activation remains a product obligation.
- **WM4:** [MSIX supported platforms](https://learn.microsoft.com/en-us/windows/msix/supported-platforms), [WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/), [WinGet on Server Desktop Experience](https://learn.microsoft.com/en-us/powershell/scripting/install/install-powershell-on-windows?view=powershell-7.6): current component availability does not prove complete-package delivery.

#### Reconciliation and completed review — 5 November 2026 UTC

The GitHub, primary-web and Microsoft Learn MCP research lanes completed, then
an analyst synthesized their results and a fifth agent integrated the compact
card and existing routes. Research began at `aea66c10734c5dedc3f010840bc3436b5dad6898`;
current Core/proof updates at `efd246fc8447b8f181879f9e6ae7c6a73bb8dc80` and the
Slint/iced reaffirmation at `21726b2db44f225da0aaed82f03255c2f2c81023` were
reconciled before the initial review. The later metadata contract at
`42dadffe1adb9fd0d41a41f8cedf314516b06bd5` was preserved before final handoff.
Historical source reports and CI observations retain their original identities.

**Decision reconciliation:** OD-14 supersedes generic client-only platform
preparation with the two intended families. Fixture editions are engineering
choices; qualification and stable support remain open. Existing managed-domain
administrator authority, DC/PDC non-goal, CPU/protection requirements, clock/UI
contracts, resource budgets, complete-MSIX direction and optional enhanced
release provenance remain. The current development §7 is byte-identical to the
latest upstream predecessor; no new matrix campaign precedes it.

**Initial frozen candidate:** 16 existing Markdown files over upstream
`21726b2db44f225da0aaed82f03255c2f2c81023`, tree
`58be1cc188a5818d17070874390925ca7f591c70`; diff SHA-256
`ab4026cd72a9c9c8c04152e6d85a15e2273bcac458c9f8c5ad999edc506fc33d`.
The reviewer received fresh context (`fork_turns: none`) and read-only scope.
Its first context-window launch failed without a report (`not run`); one bounded
retry **completed**, with **READY for documentation/research handoff** and no
material WM-R finding. Raw-report SHA-256:
`1d80babf4e74fb72c444435fe363c7280dbbbb1b31b2efe01b6b8f72275639f1`.

**Concurrent metadata reconciliation:** the same reviewer completed a bounded
source-drift continuation, reusing unaffected evidence. All original 16 authored
replacement sequences remained identical after the incoming additions. One
additional row in `docs/plans/windows-package-metadata.md` §6 aligns its device
family with the same unresolved two-profile admission contract. The resulting
17-file candidate is based on `42dadffe1adb9fd0d41a41f8cedf314516b06bd5`, full
tree `691270cd8a6f5f55b27ed77b6ba0854f4bc20423` (1,575 entries, not truncated),
with diff SHA-256
`f938f8fafd148066a83a025ddd5eb68f44c0b58259fa654a5faa52c5ea70e550`.
The addendum **completed**, also **READY** with no material finding; raw SHA-256:
`ffe2147fcf0d9d3ebe94a2cfa7d831b67193a7a5b3b17dac0ba41868de2d24b4`.

**Coordinator checks on that frozen candidate:** whitespace, 1,602 local Markdown
links and all 46 scoped preservation checks passed. All five architecture Mermaid
blocks, 138 prior source entries, canonical architecture IDs, clock/UI/resource
contracts, historical W11 receipts, prior decisions and research obligations are
retained. The coordinator matched all 739 materialized baseline blobs to the
latest full upstream tree. Synthetic local HEAD
`3dc95821813d001de87e8bf25d792379223cd2bb` is not upstream history or a publishable
tree; the review's two untouched script-mode differences must remain local.
Publication uses the complete real upstream tree and must preserve every path,
type and mode outside these intended Markdown updates.

Both raw reports below, including the reviewer's comprehension paragraphs, are
retained verbatim. Their verdicts cover the identified candidates, not this
subsequently appended factual receipt. Final receipt/link/preservation and full
tree publication checks remain coordinator evidence. No product/native/proof/fuzz
execution, environment allocation, dependency admission, service/clock/trust
change, stable-support approval or release qualification follows from this work.

#### Verbatim independent review — platform scope

> The proposed change gives OPEN-2 a narrow starting scope: Windows 11 26H2 x64 and Windows Server 2025 x64 with Desktop Experience. Pro and Standard are engineering fixtures within that direction. The patch adds a two-row qualification card to the existing Windows task and connects architecture, packaging, release policy and development routing to it. Selecting those targets does not assert that either complete product configuration works.
>
> The existing six-crate workspace provides a fixture UI, bounded protocol/numerical components and an unauthenticated loopback observer, rather than an integrated production time service. The inspected configuration still targets generic x86-64, and the UI entry point uses its Win32 adapter. V3 migration, Slint migration, production services/clock control and complete MSIX qualification remain pending. Current Core/proof work, native and fuzz restrictions, limited Sandbox authority, storage pause, Direct network scope and UI-M0 D1–D3 deferrals retain their separate owners. Mandatory mathematical proofs remain distinct from optional enhanced release provenance.
>
> ## Verdict and review identity
>
> **READY for the OPEN-2 documentation/research handoff only.** This bounded independent review completed on 5 November 2026. No material finding warrants a WM-R ID. Implementation readiness, native compatibility, stable support and release approval are outside this verdict.
>
> Context: fresh, separately delegated reviewer with request-only context and shared repository access. No inherited author discussion, analyst/integrator verdict, private KB or memory was used. This completed retry supplies actual review evidence; the preceding reported launch failure supplies none.
>
> | Identity | Independently observed result |
> | --- | --- |
> | Real upstream base | `21726b2db44f225da0aaed82f03255c2f2c81023`, confirmed through GitHub commit metadata |
> | Upstream tree | `58be1cc188a5818d17070874390925ca7f591c70`; 1,574 entries, not truncated |
> | Synthetic local HEAD | `2de5fed0c4ee45dcbfc3f9d2bfd16a7559c07f42`; all 738 materialized baseline blob hashes match the upstream tree |
> | Actual working patch | 16 existing Markdown files; 268 insertions, 22 deletions; every supplied file SHA-256 and size matched |
> | Exact diff SHA-256 | `ab4026cd72a9c9c8c04152e6d85a15e2273bcac458c9f8c5ad999edc506fc33d`; actual `git diff --binary HEAD` is byte-identical to the frozen patch |
>
> ## Inputs actually inspected
>
> I read the entire 16-file patch, candidate manifest, applicable `AGENTS.md`, `docs/README.md`, both independent-review skills, their shared review/handoff reference and testing contract.
>
> Bounded owner reads covered architecture §§1.2/17/20.2/OPEN-2; OD-05/07/10/13/14; hardening §§6.4/6.7/10; release-policy §§1–3/9; MSIX §§1/4/7/7.1; development-plan §§7/8/8.1/8.5; W11-W02/04/05/10 and §§4.19/4.32; and the S0.5 Slint pilot receipt. I checked workspace/UI/engine manifests, target configuration, toolchain, UI entry point and workflow runner excerpts. The GitHub research report's URL inventory located upstream pins; its conclusions were not an oracle. Primary pages and pinned sources were retrieved independently.
>
> ## Assessment
>
> - **Direction and evidence are separated correctly.** Microsoft's [client release record](https://learn.microsoft.com/en-us/windows/release-health/windows11-release-information) identifies 26H2/GAC/build 26300; its [Server record](https://learn.microsoft.com/en-us/windows/release-health/windows-server-release-info) identifies Server 2025 as current LTSC/build 26100. OD-14 calls the counterpart an engineering mapping and leaves exact servicing, availability and support open. It does not invent a Server “26H2” or all-edition coverage.
> - **Package registration is not mistaken for complete admission.** The [TargetDeviceFamily schema](https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-f-targetdevicefamily) supports the minimum-version and tested-version distinctions. The patch correctly exposes the common-minimum problem and retains OS-family enforcement as unfinished work. [MSIX platform support](https://learn.microsoft.com/en-us/windows/msix/supported-platforms) and [WinGet availability](https://learn.microsoft.com/en-us/windows/package-manager/winget/) do not establish this product's Server acquisition/licensing/update route. No unsupported family name, installer workaround or automatic fallback is selected.
> - **CPU and protection boundaries remain substantive.** Safe prerequisite execution before every v3 path preserves OD-07, including startup dependencies and SCM/update activation. The pinned Rust [detector](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/library/std_detect/src/detect/os/x86.rs) checks OS vector state; its [macro contract](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/library/std_detect/src/detect/arch/x86.rs) explains why compile-enabled features cannot independently prove runtime admission. Required protection failures remain blocking, while optional dispositions stay explicit.
> - **Existing product obligations survive the narrower scope.** Architecture §1.2's managed-domain observation default and administrator-authorized takeover remain, with GPO/competing-owner refusal and the DC/PDC non-goal. W11 §4.32 preserves per-profile physical timing/power evidence, named-hardware CHECK-21, one-hour/72-hour measurements and recovery observation before teardown. Short Slint, Sandbox, portable or cross-build receipts cannot fill those cells. The inspected Sandbox receipt supports the stated no-product-payload and incomplete token/protection limits. GitHub's [hosted-runner privilege policy](https://docs.github.com/en/actions/reference/runners/github-hosted-runners) and the [pinned Server image](https://github.com/actions/runner-images/blob/42d27e4a2c1232e5640a88ed42a974c8918ecd28/images/windows/Windows2025-VS2026-Readme.md) support separating build infrastructure from the intended product profiles.
>
> ## Evidence limits
>
> This is source/document review, with a clean `git diff --check`; no product, native, proof or fuzz execution occurred. Historical raw logs/binaries were not independently replayed. Fixture availability, exact profiles, enforcement mechanisms and Server delivery remain unverified inputs, not observed product defects. Mutable primary pages require revalidation for implementation.
>
> The synthetic snapshot is not upstream history: two untouched scripts, `scripts/setup-xwin.py` and `scripts/xwin-linker.py`, have local mode 100644 versus upstream 100755, despite matching blobs. Missing unrelated paths and those mode differences must not become publication changes. Neither affects the reviewed Markdown patch.
>
> ## One permitted next action
>
> The coordinator may retain this raw review verbatim in existing `tasks/windows11-integration.md` §4.32 and carry the documentation handoff forward, preserving upstream metadata and all current implementation, execution, support and release gates.

#### Verbatim independent review — concurrent metadata

> The concurrent upstream change adds a shared contract for product, executable and package metadata, with OD-05/P6/W11-W10 routing. The additional platform edit changes one `TargetDeviceFamily` table row so the metadata contract follows both intended OS profiles and the existing unresolved MSIX applicability questions.
>
> The current product remains the same bounded workspace reviewed previously. Metadata generation, emitted-package readback and installed observations are planned work; neither the upstream contract nor this row supplies a working package, native compatibility or support approval. Existing Core priority, execution restrictions and optional enhanced-provenance policy remain intact.
>
> ## Scope, inputs and identity
>
> **READY for the documentation/research handoff.** This completed addendum is the same reviewer's bounded continuation, reusing unaffected evidence from the initial independent review. No material WM-R finding is identified.
>
> Inspected inputs: both frozen candidate diffs/manifests; the incoming metadata patch and preservation record; current metadata-contract §§1/4/6/8/10, OD-05 §7.6, MSIX §7.1 and the incoming documentation/task routes.
>
> - Supplied upstream base: `42dadffe1adb9fd0d41a41f8cedf314516b06bd5`.
> - Observed synthetic HEAD: `3dc95821813d001de87e8bf25d792379223cd2bb`.
> - Actual diff SHA-256: `f938f8fafd148066a83a025ddd5eb68f44c0b58259fa654a5faa52c5ea70e550`; byte-identical to the frozen delta candidate. All 17 supplied file hashes/sizes match.
>
> I independently recomputed the replacement comparison: all original 16 edit sequences are identical. The actual old-to-new local baseline delta exactly matches the supplied incoming patch. Incoming additions remain verbatim except the deliberately replaced metadata row. The initial raw report remains unchanged at SHA-256 `1d80babf4e74fb72c444435fe363c7280dbbbb1b31b2efe01b6b8f72275639f1`. `git diff --check` passed.
>
> ## Assessment and limits
>
> The row in `docs/plans/windows-package-metadata.md` §6 agrees with `docs/plans/windows-msix-store.md` §7.1: `Windows.Desktop` remains a candidate requiring qualification for both OD-14 rows; numeric applicability and tested-version metadata establish neither an exclusive support boundary nor qualification. The inherited [TargetDeviceFamily schema evidence](https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-f-targetdevicefamily) remains applicable.
>
> The metadata additions retain authoritative value sources, explicit pending fields, independent artifact/readback checks, conditional MSI status and separate submitted/delivered identities. They introduce no contradictory platform claim or mandatory enhanced-provenance requirement.
>
> This is a drift/row review, not a broad metadata API audit or native/product/proof/fuzz execution. Upstream attribution uses the supplied incoming patch; no new whole-tree audit was performed. Initial evidence limits, including synthetic script-mode differences, continue to apply.
>
> ## One permitted next action
>
> Retain both raw reviews separately and verbatim in existing W11 §4.32, then carry the reconciled documentation handoff forward under its existing gates.

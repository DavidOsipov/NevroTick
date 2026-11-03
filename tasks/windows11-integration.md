# Windows 11 integration: agent task and evidence record

**Date:** 3 November 2026 (Asia/Tbilisi).  
**Status:** Research recorded; earlier integration/adoption review receipts retain their original candidates. The later owner decision in section 4.8 selects complete hardened MSIX preparation and David's Store attempt with services; it supersedes the MSI-first disposition in sections 4.6–4.7. Section 4.14 records the concrete source profile; section 4.15 records the deeper provisioning/removal research and unselected alternatives. Package implementation and native qualification remain unexecuted.<br>
**Initial inspected project baseline:** [`a6c2401b9a8166515293653f14f637e5543c689e`](https://github.com/DavidOsipov/NevroTick/tree/a6c2401b9a8166515293653f14f637e5543c689e), tree `7d0558ab295a7859e0d7d9e5b1180bea0fe30443`. This task was initially prepared from an API-derived partial snapshot, not a local Git checkout; no dirty/index/staged-state claim applies to that preparation. Later adoption and research candidates are identified in section 4.  
**Owner request:** integrate the completed Microsoft Windows 11 investigation into maintained documentation, architecture, the development plan and tasks for implementation agents.

## 1. Authority, scope and reconciliation

Read [AGENTS.md](../AGENTS.md), the [documentation guide](../docs/README.md),
the [Windows 11 research](../docs/plans/windows11-integration.md), its
[Microsoft Learn follow-up](../docs/plans/windows11-integration-follow-up.md), the
[MSIX/Store assessment](../docs/plans/windows-msix-store.md) for packaging work, and
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

This request authorizes documentation integration. It does not lift the current
Windows execution stop, separate fuzz no-retry hold or storage-experiment pause,
or authorize dependency acquisition, tool installation,
system configuration, keys, clock effects or service/package experiments. Future
work uses its existing applicable authorization, contract and environment gates;
this task adds no blanket approval round. Material changes to product policy or
trust boundaries use the existing owner-decision procedure, blocking only their
dependent work. Do not scaffold product code to make a planning check executable.

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

Prepare **one W11-W01 timestamp-provenance card** when the relevant acquisition
owner is assigned. This is a ready source/documentation action alongside the
existing plan, not a replacement for development §7's current portable Core
checkpoint or the engine's separately held native qualification.
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
owned restoration and the [sleep/timer research](../docs/plans/windows11-integration.md#sleep-timers-and-leap-presentation).

**Work:** preserve UTC, QPC and active-awake meanings. Modern Standby may suspend
services; a sleep-excluding system clock does not measure broker execution.
Recheck authoritative deadlines and generations after delayed/coalesced wakes.
No timer promises a finite lease on a programmed rate. Keep residual-rate and
unknown-cleanup outcomes, with mandatory broker work independent of optional
saves. Evaluate process leap presentation separately from global leap ownership;
high-resolution timers remain conditional on measured need.

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
**Prerequisites:** actual authenticated service snapshots, restored deferred
contracts and [native UI research](../docs/plans/windows11-integration.md#8-present-live-state-through-native-ui-facilities).

**Work:** evaluate `UiaRaiseNotificationEvent` and supported processing modes for
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

Protected Process Light is excluded as a normal utility-service option; ordinary
publisher signing supplies no protected-service eligibility. No reserve selects
a dependency, weaker fallback, service topology or machine-wide policy here.

## 4. Evidence and continuation

| Evidence layer | Current state |
| --- | --- |
| Microsoft source investigation and critical source/fit review | Completed research retained in the linked document; documentation facts and engineering inferences are distinguished. No native experiments were executed. |
| Documentation, architecture, plan and task integration | Earlier integration and local-adoption review/validation retain the candidates in sections 4.1–4.3. The second-pass integration is recorded from section 4.4; the MSIX/Store amendment is recorded from section 4.6. Earlier passes do not qualify changed text. |
| W11-W01 source contract | Ready bounded next action when assigned to the relevant acquisition owner; not executed by this integration. |
| Portable implementation/model | No W11 work-item, CHECK, DNS-C or ST-T implementation pass established by this integration. |
| Native Windows/service identity | Unexecuted; the current Windows execution stop, storage-experiment pause and applicable environment/authorization gates remain. The separate fuzz no-retry hold is also unchanged. |
| Installed MSIX and release | Unexecuted; local test-signed mechanics, Store acceptance and actual Store-delivered trust/mitigation qualification remain separate evidence layers. |

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

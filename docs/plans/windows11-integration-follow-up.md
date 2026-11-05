# NevroTick: Microsoft Learn follow-up for Windows 11 integration

**Document:** NV-WIN11-RESEARCH-002, revision 1.0<br>
**Date:** 3 November 2026 (Asia/Tbilisi)<br>
**Owner:** David Osipov<br>
**Status:** Source research and future qualification guidance; no implementation or native result.

Read [the documentation guide](../README.md), the
[first Windows integration review](windows11-integration.md) and the existing
[agent task](../../tasks/windows11-integration.md). The
[architecture](../windows-time-architecture.md) owns product contracts;
[development §8.5](development-plan.md#85-windows-11-integration-research-and-qualification)
owns phase placement. This supplement supplies source details and discriminating
cases for those owners, rather than another implementation plan.

## 1. Scope, evidence and adoption limits

The owner requested a second investigation through Windows Learn MCP, followed
by documentation integration. The research compared the relevant canonical
documents, tasks and service-adapter guidance at commit
`f163c193b0bb4167ec5c39e16b1db1b581509750`, tree
`1e6221691c18f30015daf854d1cbd7cc4c44b6c5`. Integration starts from the later
`45ff247d84e5e43bd13f54de1c84aa13ce2e6efd`, tree
`6fccf094d8eae9d8dbccfb406afed0e08e2b11f9`. The coordinator verified an
API-derived partial snapshot against the full tree; it is not a local checkout
or evidence about another agent's index or working files.

The investigation used targeted Microsoft Learn searches and complete-page
retrieval, with parallel time/network, lifecycle, storage/security and DNS/resource
questions. A fresh-context independent source review corrected the MSI inheritance
and ETW GUID wording and strengthened COM teardown, ETW routing, attestation and
ABI caveats. It did not repeat every lower-priority retrieval. A research review
does not qualify this integrated documentation candidate; its separate review and
checks belong in the existing task record.

The useful additions are specific API transitions, alternatives and failure
states. Clock-domain separation, native completion custody, privacy barriers,
indeterminate persistence, service recovery and fixed budgets were already
requirements. Absence of an API name in the inspected material establishes no
whole-repository omission or production vulnerability. Microsoft search results
can repeat chunks of one page, and official prose can be inconsistent. MCP is a
retrieval mechanism, not evidence of complete coverage, correct FFI or effective
Windows enforcement.

The [current next-task plan](development-plan.md#7-next-executable-task-for-a-development-agent)
continues to control portable Core work. Its broad Windows execution stop,
separate fuzz-runner no-retry hold and the storage-experiment pause remain in force.
The one complete MSI, least-privilege service separation, DoQ-only upstream with
leaf/intermediate pins, separate mandatory clock continuity and UI-M0/D1–D3
deferrals remain unchanged. No dependency, provider, optional API, privilege,
process, release channel or new acceptance gate is selected here. “High priority”
below means important when its existing adapter stage is reached.

All proposed checks are future work in the authorized environment. No SDK binding,
native ABI, Windows account, service, key, package, crash or performance experiment
was executed for this investigation. Source-backed behavior is identified below;
the proposed adaptations are engineering inferences requiring their own evidence.

## 2. Packet timestamps, socket metadata and completion

### Timestamp configuration can change without a route change

**Source fact.** `RegisterInterfaceTimestampConfigChange` supplies a dedicated
notification. Its callback receives only `CallerContext`, without identifying
the changed interface or new mode. Unregistration waits for outstanding callbacks
and promises no later callback after return; self-unregistration or holding a lock
needed by a callback can deadlock. [T1] [T2] [T3]

**Adaptation.** Extend W11-W01's provenance card with same-interface configuration
changes. Coalesce a dirty signal, re-query tracked interfaces outside the callback
and reconcile affected source/conversion generations. Keep callback context alive
through draining unregistration. This signal does not atomically identify the
configuration that produced queued RX/TX metadata; the selected adapter must
declare how it rejects or reconciles transition-era measurements.

**Future controls.** An unchanged route with changed timestamp mode, old queued TX
metadata, notification bursts, registration failure and stop during a callback
must preserve bounded work and prevent mislabeled samples. Retain an unchanged-mode
healthy neighbor. Map to W11-W01 and CHECK-6/11/21/27 at their existing scopes.

**Source discrepancy.** Registration/unregistration list build 20348, while the
callback page lists Windows 11 build 22000.194. More seriously, the unregistration
page has a `VOID` signature but `DWORD` return-value prose. Verify the selected
SDK/generated binding and supported runtime before implementing; do not infer a
fallible return value or universal support floor from the conflicting text.

### Packet payload and ancillary metadata need separate admission

**Source fact.** `WSAMSG` distinguishes payload truncation (`MSG_TRUNC`) from
control-data truncation (`MSG_CTRUNC`). `WSARecvMsg` uses a runtime extension-function
lookup. Its dual-stack packet-info setup needs `IP_PKTINFO` for IPv4 as well as the
IPv6 option. Received `IN_PKTINFO.ipi_addr` is the local destination address;
`ipi_ifindex` identifies the receiving interface. Neither field is the remote peer
address. [T4] [T5] [T6]

**Adaptation.** Admit payload, peer endpoint, local destination/interface and
optional timestamps independently. Validate input capacities, output lengths,
flags, control-message framing and alignment; reinitialize reused message outputs.
A complete NTP packet does not make truncated ancillary data valid. A declared
application-timestamp fallback is usable only if its own admission and provenance
contract remains satisfied. Otherwise the measurement is unusable.

**Future controls.** Exercise exact-size and one-short data/control buffers,
additional enabled control messages, stale outputs, dual-stack IPv4/IPv6 and two
interfaces. Valid neighbors must work without parsing a truncated packet prefix
or inventing metadata. These refine W11-W01, CHECK-6/25/27.

**ABI discrepancy.** The `WSAMSG` overview embeds `UINT cmsg_len`, whereas the
dedicated `WSACMSGHDR` page declares `SIZE_T cmsg_len`. The latter also lists a
WSK header context. This is material to x64 layout and does not qualify the
selected user-mode declaration. Use pinned SDK/generated bindings and verify the
actual layout; hand-copying either prose example into Rust is insufficient. [T4] [T7]

### Direct overlapped sockets need an explicit terminal-completion owner

**Source fact.** Microsoft's overlapped-socket guidance warns against continued
use after cancellation and recommends closing the socket. `closesocket` can make
its numeric descriptor immediately reusable, must not race another Winsock call
on that socket, and does not mean outstanding completions have arrived. [T8] [T9]

`FILE_SKIP_COMPLETION_PORT_ON_SUCCESS` changes the immediate-success path: under
its documented conditions, no IOCP entry is queued. A notification mode cannot
be removed from the same handle, and socket-provider/IFS compatibility matters.
Immediate and pending results therefore need different terminal-accounting rules.
[T10]

**Adaptation and controls.** If a direct overlapped adapter is selected, distinguish
logical cancellation, socket retirement and actual terminal completion. Serialize
issue/close, retain buffers and capacity charges until completion, bind reused
handles to generations and settle once according to the configured mode. Test
immediate success with/without skip mode, pending success/failure, cancellation
losing to success, multiple pending requests, out-of-order completion and handle
reuse, beside normal shutdown. W11-W01 and CHECK-27 own the native lifetime;
CHECK-6/21/25 apply to reached admission/resource/representation consumers.

Dropping a Tokio receive future is not evidence that the runtime calls native
cancellation. Inspect the selected implementation before applying these rules;
this finding does not recommend replacing the current engine runtime.

### Check the actual I/O wait across sleep and unattended resume

`GetQueuedCompletionStatusEx` excludes low-power time from its timeout on Windows
8 and later. A successful dequeue does not mean every returned I/O succeeded.
`PBT_APMRESUMESUSPEND` does not cover unattended/remote wake; the automatic-resume
path matters, with a documented Windows 10+ exception for waking only to enter
hibernation. `PowerRegisterSuspendResumeNotification` offers a callback route.
[T11] [T12] [T13] [T14]

Extend W11-W02's existing sleep/deadline model to the actual runtime wait. After
execution resumes, recheck the authoritative elapsed deadline and relevant
generations. No API here promises callback scheduling by a deadline. Future
cases include suspend during idle I/O waiting, unattended wake, sleep-to-hibernate
and a batch containing failed completions. Stale exchanges must remain unusable
and optional refreshes must not replay a missed-work backlog. Existing CHECK-11/21/27
and affected DNS-C05/11 retain ownership.

### Make fine QPC ordering assumptions explicit

Microsoft treats QPC readings on different threads that differ by ±1 tick as
ambiguously ordered. It discourages direct `RDTSC`/`RDTSCP`, unnecessary single-core
pinning and confusing read cost/resolution with precision; conversion arithmetic
can also overflow or lose precision. [T15]

Retain operation/generation identity for causality, bounded read uncertainty and
checked delta/rational conversion. Test equal/adjacent cross-thread values, large
counters, conversion neighbors and widened read brackets. Do not turn ambiguous
order into exact simultaneity or improve a bound by guessing where a slow read
occurred. This is a small W11-W01/02 numerical refinement under CHECK-25 and the
affected downstream CHECK-7 scope, not a new clock algorithm or CPU-affinity policy.

## 3. Service lifecycle, installation and COM

### Selected COM adapters need thread and process owners

The first review proposed COM-based WSH and cost mechanisms. `CoInitializeEx`
initializes each thread; every successful call, including `S_FALSE`, needs a
matching uninitialize. An incompatible apartment returns `RPC_E_CHANGED_MODE`.
Process-wide `CoInitializeSecurity` runs once, explicitly or implicitly; earlier
marshaling can cause a later explicit call to fail with `RPC_E_TOO_LATE`. [L1] [L2]

For each selected class, record its actual role, apartment/thread, activation,
proxy/callback directions, security-initialization timing, authentication and
impersonation needs, release order and final teardown. Do not move non-agile
interfaces between threads without supported marshaling or place process-global
security initialization inside an arbitrary reusable constructor. A null server
access descriptor has documented security implications; it does not establish
that a COM client automatically exposes a remotely callable server.

Final `CoUninitialize` can enter a modal loop and dispatch pending COM messages.
Treat teardown as potentially reentrant: preserve required state and shutdown
guards, and avoid locks that dispatched work needs. `EOAC_NO_CUSTOM_MARSHAL` is a
compatibility-sensitive candidate with documented exceptions, not a universal
hardening switch. [L3] [L4]

Future checks pair normal permitted operations with mismatched apartments,
implicit initialization before explicit security, callbacks during teardown and
the final service identity/mitigations. Do not weaken ACG/CIG/DLL/Win32k restrictions
or add privileges merely to make activation work. W11-W04/06 and future W11-W09
retain CHECK-16/27 ownership. The native cost alternative in §5 may avoid COM for
one engine need; it does not remove other selected classes' obligations or justify
a generic COM runtime/new process.

### Per-service state locations have a service-installation lifetime

`GetServiceDirectory` and `GetServiceRegistryStateKey` use the service's
`SERVICE_STATUS_HANDLE`; both list build 19041. The directory API requires an
enabled service SID and describes write access for LocalSystem and that SID.
The shared-directory variant lists Windows 11 build 22000 and additionally grants
local administrators write access. [L5] [L6] [L7]

Persistent service state survives reboot/OS update, but belongs to the service
installation: deletion of the service deletes its managed state. The directory
and registry-state type contracts make that lifecycle material. A major upgrade
that actually deletes/recreates a service can therefore destroy this state;
not every major upgrade necessarily does so. [L8] [L9]

Compare these locations with existing protected directories for optional or
disposable state. They can reduce provisioning code, but establish neither all
effective read/owner rights nor encryption, anti-rollback, secure erasure or
correct child descriptors. They are not a reason to move the correction journal,
consumed authority, trusted floor or restoration evidence without the separately
proved mandatory-state lifecycle. Shared access does not create an approved UI
write path.

Future storage/package fixtures should use two owned dummy services and intended
account profiles; inspect owner/DACL and allowed/denied read/write/replace, reboot,
repair, delete/recreate and interrupted upgrade. Observe key/privacy retirement
separately. W11-W05 and ST-W01/03 feed existing ST-T03/10/12/17 and CHECK-13/15/23;
no location migration is selected by this comparison.

### Assess declarative MSI permissions before privileged custom actions

`MsiLockPermissionsEx` can secure package-owned services, files, directories and
registry entries with Windows Installer 5.0+. It acts when the resource is
installed/reinstalled. Parent inheritance must be set before child creation;
the package cannot contain both this and the legacy `LockPermissions` table.
Multiple true permission conditions for one object fail. [L10] [L11]

`FormattedSDDLText` can resolve account names, but ordinary public/private MSI
properties are not substituted in `SDDLText`. Do not invent property interpolation
or copy ambient-user examples into elevated installation policy. Account/service
identity resolution and creation order remain actual-package questions. [L12]

Prefer a reviewed declarative descriptor where this mechanism expresses the
required contract. Inheritable parent permissions can protect later runtime
children; the table does not reapply a separately authored per-file descriptor to
arbitrary runtime creations/replacements. Verify final identity, owner, DACL and
inheritance rather than assuming either universal protection or none.

W11-W05's future S0.6/P6 cases should inspect the authored MSI, valid installation,
reinstall/repair, conflicting or invalid descriptors, parent-before-child order,
rollback and later replacement. Existing CHECK-1/13/15/23 apply. This candidate
supports the complete-MSI/minimal-custom-action direction without choosing a
new installer or changing public-property authority.

### SCM observer APIs have different callback lifetimes

`SubscribeServiceChangeNotifications` supports status, property and database
changes; its callback mask may be zero, requiring reconciliation. Callbacks must
not block. `UnsubscribeServiceChangeNotifications` waits for active callbacks and
deadlocks when called by the callback itself. Documentation lists Windows 8,
`Winsvcp.h` and `SecHost.dll`; actual selected bindings/exports remain unqualified.
[L13] [L14]

The alternative `NotifyServiceStatusChangeW` uses APC delivery and an alertable
wait, one-shot/rearm semantics and retained notification buffers. It requires
the relevant self-thread access, warns against RPC/SCM calls in the callback and
defines lagging-client/reconnect and delete-pending handle cleanup. Do not assume
the engine's Tokio loop supplies that execution model. [L15]

Select one narrow observer; coalesce notification into current-state queries
outside the callback. An event or SCM Running is not authenticated application
readiness. Preserve instance/generation and endpoint identity. Future rapid
start/stop/config/delete-recreate, zero-mask, callback-held-during-unsubscribe and
UI/helper-close cases should show bounded reconciliation, no deadlock and no
handle preventing uninstall, with healthy reconnection. W11-W05 owns lifecycle;
live UI consumption remains D2. Existing CHECK-13/14/23/27 retain their scopes.

## 4. Diagnostics, replacement and protected storage

### ETW interest is separate from diagnostic authorization

`PENABLECALLBACK` reports combined enable information from trace sessions.
Another controller can keep a provider enabled after one session stops. The
controller supplies `SourceId`; it can differ from the session GUID and its value
can be `GUID_NULL`, which is not a nullable-pointer ABI statement. Capture-state
is a separate request, and filter data is valid only during the callback. Blocking,
loader work and trace-control calls can cause callback deadlocks. [D1]

Keep NevroTick's local authorization/expiry guard in addition to ETW interest.
A second WPR session cannot extend that deadline or obtain authority from a GUID.
The guard controls emission, not delivery exclusively to the approved session:
`EventWrite` sends enabled events to matching sessions. Expiry cannot recall
already collected ETL data. [D2]

Provider registration/enable rights, session control/real-time consumer rights
and ETL file ACL/retention are separate surfaces. `EventAccessControl` describes
these distinctions and broad default groups; secure-session write permissions
also depend on the secure-mode setting. Do not add ordinary users to Performance
Log Users merely for product diagnostics. Required-event routing remains separate
from verbose collection. [D3]

W11-W08 future controls: two controllers with different filters, product expiry
while one controller remains enabled, capture-state, provider restart and actual
allowed/denied users. Synthetic secrets must remain absent, authorized output
useful and required sinks intact. Callback custody and finite resources remain
CHECK-27/21; authorization/privacy remain CHECK-1/17. No current leak is claimed.

### ReplaceFileW errors can describe partially changed state

The existing storage model already separates operation outcomes from owner
usability. `ReplaceFileW` supplies concrete cases for that model: [S1]

| Result | Documented namespace consequence |
| --- | --- |
| 1175, `ERROR_UNABLE_TO_REMOVE_REPLACED` | Both files retain their original names. |
| 1176, `ERROR_UNABLE_TO_MOVE_REPLACEMENT` | With a backup name, original names remain; without one, the replaced file can be absent while the replacement retains its staging name. |
| 1177, `ERROR_UNABLE_TO_MOVE_REPLACEMENT_2` | The replacement retains its original name after inheriting streams/attributes; the old file has another name, using the specified backup name when supplied. |
| Other error | Original names remain and no backup exists; complete metadata/stream inheritance is not guaranteed. |

The files must share a volume. `REPLACEFILE_WRITE_THROUGH` is unsupported;
ignore-merge/ACL flags can allow success without preserved ACLs. Successful
replacement uses the replacement file's file ID. Neither success nor an unchanged
path proves all identity, descriptor or durability properties. [S1]

The adapter must not translate `FALSE` into “no effect” or blindly delete a staging
file needed for recovery. Reconcile names, object identities, bytes, attributes
and access before cleanup, under the existing finite inventory and privacy rules.
Future ST-W01/02/03 cases vary the admitted backup slot and each documented error,
then interrupt recovery again. Native observation must establish the actual
filesystem branch; a portable model or source paragraph does not prove a flush
sequence. Existing ST-T06/07/10/16/17 and CHECK-15/27 remain the owners.

### Crash privacy must name the collector

`WerSetFlags` provides process-scoped `WER_FAULT_REPORTING_FLAG_NOHEAP`.
Windows LocalDumps is independently controlled, disabled by default and enabled
administratively, with global/per-image settings and account-specific paths.
It can operate when other WER reporting is disabled/cancelled and collects before
an application-recovery callback. [S2] [S3]

If a process crash policy is selected, qualify its actual scope separately from
LocalDumps/debugger paths; NOHEAP or one excluded region is not proof that every
collector omits secrets. Inventory actual policy without silently changing the
administrator's global settings. Future dummy-secret cases cover selected
allocations, reuse, registration failures and crash phases under final identities,
while retaining useful permitted diagnostics. W11-W08/ST-W07 refine ST-T14 and
CHECK-17; no production crash, real key or new dump collector is required here.

## 5. DNS integration and resource hints

### Native IP Helper hints may satisfy a narrower cost requirement

`GetNetworkConnectivityHint`, `GetNetworkConnectivityHintForInterface` and
`NotifyNetworkConnectivityHintChange` list build 19041. Their hint includes
connectivity, cost, roaming and approaching/exceeded data-limit state; notification
cancellation uses the IP Helper cancellation owner. The global API estimates the
aggregate experience, while the interface API addresses one interface. [N1] [N2] [N3] [N4]

Compare a small native adapter with COM `INetworkCostManager` if these inputs are
sufficient for optional refresh policy. These APIs take no upstream destination
and are not equivalent to destination-aware `GetCost`; neither prove a usable
route. Bind observations to current network generations and actual egress.
Unknown cost is not free, and a disconnected Internet hint is not an absolute
ban on bounded attempts to a reachable NTP/DoQ endpoint. Fixed budgets remain.

W11-W06 controls should include two interfaces with different costs, route change,
unknown hints, a reachable controlled endpoint despite negative Internet status,
and stop racing a callback. Preserve foreground/time work and cancellation
custody. Map discretionary traffic to DNS-C11/12/18 and reached resource/lifetime
contracts to CHECK-21/27; DNS-C08 applies only if the temperature/seasonal model
changes. No COM or native adapter is selected merely by comparison.

### Native DNS queries have separate completion and policy decisions

`DnsQueryEx` can complete synchronously or fail without a callback, including
documented local-machine/literal-IP cases. Only `DNS_REQUEST_PENDING` establishes
asynchronous completion. Retain pending result/cancel/context storage through
actual completion, and settle accounting exactly once on the selected path. Do
not generalize this to an unsupported rule that every cache hit is synchronous.
[N5]

`DNS_QUERY_REQUEST3`, listed from build 22000, says an NRPT match replaces
`pCustomServers` with the rule's servers. An explicit list therefore does not
establish universal destination authority. `DNS_CUSTOM_SERVER` describes UDP/DoH
and an optional insecure fallback, not the adopted DoQ-with-pins transport.
`DnsQueryRaw` remains prerelease with no clear client floor; the first review
already recorded this limit. [N6] [N7] [N8]

Where another selected native resolver path needs these APIs, W11-W03/CHECK-27
cases pair immediate/pending success and failure, cancellation/late completion,
and NRPT match/nonmatch with actual endpoint observations. Keep private-name
routing and bounded egress. This does not introduce DoH/UDP fallback for the
system-wide DoQ service or bypass existing NTS-resolution contracts.

### Preserve DNS settings outside the owned write set

`DNS_INTERFACE_SETTINGS.EnableLLMNR` controls both LLMNR and mDNS. IPv4 is the
default family; IPv6 requires its flag. Structure and Get/Set function support
tables have different floors. `DNS_INTERFACE_SETTINGS_EX.DisableUnconstrainedQueries`
is explicitly reserved, so its name is not a supported security control. [N9] [N10]

Add a W11-W03/DNS-C17 case where changing only NameServer preserves suffix,
registration, local discovery and other profile fields in both families.
Restoration still applies only to owned writes after intervening edits. These
structures do not prove DHCP-origin restoration or authorize disabling LLMNR/mDNS
as an enrollment side effect.

### NevroTick's forget action does not own every DNS cache

Microsoft documents `Get-DnsClientCache` as retrieving the Windows DNS Client
cache. NevroTick's existing DNS design already acknowledges OS/application cache
hits that never reach its listeners. This pass adds an explicit privacy boundary:
encrypted NevroTick snapshots or forgetting its learned history cannot promise
deletion of Windows/browser/other caches and DNS traces. [N11]

Distinguish own retained statistics, own response cache and external caches in
the eventual D2 settings/status. Do not claim which users can enumerate Windows
cache without actual account/policy evidence. No global flushing or TTL=0 change
follows from wanting more heatmap observations. Existing DNS-C14/15 and ST-T14/15
own the relevant privacy behavior; this is not a new cache-observation algorithm.

## 6. Conditional mechanisms

These remain W11-W10 or existing storage reserves. None is a prerequisite for
the current Core work or a reason to lift native holds.

### VBS key claims as a possible actual-key evidence route

`NCryptCreateClaim` documents `NCRYPT_CLAIM_VBS_ROOT` and
`NCRYPT_CLAIM_VBS_IDENTITY`; `NCryptVerifyClaim` can return key flags and root
trustlet identity, security version and debug status. The VBS extensions are
marked prerelease. The old base-API support table does not qualify them. [S4] [S5]

This is a concrete lead for the existing actual-root/reopen evidence gap.
Verification success is only one input to relying-party policy: accepted anchor,
subject/root binding, freshness/replay, flags, debug/version and supported-update
rules remain necessary. The creation sample's literal nonce and verification
sample's null parameter lists/printed details are not that policy.

An identity claim needs an appropriately marked attestation key, a hash parameter
and, for RSA, the documented padding parameters. Investigation does not approve
another key/root or add attestation use to existing wrapping/signing keys. If
reopened, compare real final-account roots with wrong/imported/software controls,
replay, malformed claims, unsupported providers/builds and cross-boot updates.
Unproved properties remain Unknown/RecoveryRequired. ST-W04 and ST-T01/05/13
retain ownership. No cloud service or hardware rollback guarantee is inferred.

### Side-channel policy requires a named threat and measured cost

`PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY` exposes controls for SMT branch
targets, security domains, page combining, speculative store bypass and core
sharing. Some settings cannot be disabled after activation; unsupported hardware
or system policy can make requests ineffective, and core sharing requires suitable
scheduler/hypervisor support. Page-combining restrictions have common-page exceptions.
The page does not establish one precise build floor for all fields. [S6]

Consider only a named role/field with a supported threat, actual enforcement
evidence and timing/CPU/energy measurements. Requested-bit readback is not complete
protection. Preserve healthy crypto/IPC and supported-VM cases; do not enable all
bits or broaden privileges by default. W11-W10/ST-W05 retain CHECK-16/21 ownership.
This does not protect against arbitrary same-process execution, an administrator
or every side channel.

### UDP coalescing and segmentation change framing assumptions

`UDP_RECV_MAX_COALESCED_SIZE` and `UDP_SEND_MSG_SIZE` default to zero. Opt-in
receive coalescing can place several same-source datagrams into one buffer,
while the send option can split a buffer into multiple datagrams;
`UDP_COALESCED_INFO` supplies the ordinary segment size and the last can be shorter.
The receive maximum is not a universal size cap for ordinary datagrams.
`WSASetUdpRecvMaxCoalescedSize` lists build 19041. [T16] [T17]

Inspect a selected later DoQ backend if a measured throughput bottleneck warrants
batching. Require original boundaries, per-datagram accounting, ancillary integrity
and a qualified timestamp attribution rule. Test no-coalescing, short final
segments, missing/truncated metadata, oversize uncoalesced input and failed sends.
W11-W10, affected W11-W01 and DNS-C12 own the question. DNS request deduplication
is a different mechanism. No NTP batching or machine-wide offload change is chosen.

### VM Generation ID has a privilege conflict

Microsoft's VM Generation ID device can signal supported guest restore/clone
transitions. Its documented access requires administrator/SYSTEM, it can be absent
and a read can become stale; the event-driven request uses the previous identifier
to avoid missing a later change. The article's legacy guest list does not establish
current support for every selected VM. [T18]

It may supply an additional restore witness, not a trusted boot counter or
protection against a malicious hypervisor. Do not broaden the restricted engine
or virtual-account broker to LocalSystem for it; one installation-time read also
cannot monitor later restores. If a compatible design is justified, qualify
restore/clone, absent/denied device, change races and cancelled IOCTL lifetime.
Consumed clock authority must not reset. W11-W10 and the existing VM/boot-epoch
owner retain CHECK-11/12/15/27; access and applicability remain unresolved.

## 7. Existing owners and next work

The assignments below reuse [W11-W01–W11-W10](../../tasks/windows11-integration.md#3-bounded-work-assignments)
and [ST-W01–ST-W07](../../tasks/windows-protected-storage.md#3-later-implementation-and-qualification-work).
They add source details and cases, not acceptance IDs or a parallel backlog.

| Existing assignment | Follow-up to carry into its next dependent stage |
| --- | --- |
| W11-W01 | Same-interface timestamp changes; separate ancillary admission; selected socket completion; exact SDK ABI and QPC assumptions. |
| W11-W02 | Actual I/O-wait timeout and unattended/hibernate resume cases alongside the existing sleep model. |
| W11-W03 | DNS immediate/pending completion, NRPT precedence and exact field preservation. Keep private-name routing and DoQ pins. |
| W11-W04 | COM ownership if WSH is selected; existing identity and network-confinement evidence remains. |
| W11-W05 | Compare declarative MSI ACLs and managed service state; select an SCM observer with its actual callback lifetime. |
| W11-W06 | Compare native global/interface hints with destination-aware COM information for the selected optional workload. |
| W11-W07 | No new hashing/provider requirement; the prior eligible-workload and measured batching gates remain. |
| W11-W08 | ETW emission authorization versus session/file access, and collector-specific crash privacy. |
| W11-W09 | Carry observer/privacy semantics into deferred D2/D3 consumers only at their existing triggers. |
| W11-W10 | Keep VBS claims, side-channel fields, UDP batching and VM-generation access conditional. |
| ST-W01/02/03 | Refine the existing construction/outcome/directory card with the ReplaceFile matrix, managed-state lifetime and MSI inheritance where selected. |
| ST-W04/05/07 | Conditional VBS claim prerequisites, measured role-specific side-channel policy and explicit collector/OS-cache privacy limits. |

The next source-only W11-W01 card remains bounded. The ready ST-W01 contract
card may consume these persistence details when assigned. Neither card is
recorded as executed here, and neither displaces the active Core checkpoint.
Later implementation uses its actual source path, finite cases, relevant native
environment and existing cleanup/authorization constraints. Reuse the research
rather than restarting a broad survey.

Do not promote already documented service recovery, generic cancellation,
memory budgets, DNSSEC validation or WER exclusions as discoveries. Native DNS
APIs/DO flags do not replace the selected local DNSSEC validator. No value was
established for direct TSC access, generic COM infrastructure, an automatic service
restart controller, broad SCM/ETW group access or global timer/offload changes.

### Windows component reuse at the adapter boundary

**5 November 2026 source-only integration.** Apply the dated
[architecture register and pinned sources S132–S136](../windows-time-architecture.md#1862-windows-component-reuse-and-binding-coverage)
inside existing assignments. `windows`/`windows-sys` already supply the planned
ABI surface; suitable focused wrappers can remove routine code. The narrow
product adapter remains responsible for current authority, bounds, effects and
real completion. No dependency, wrapper, toolkit or native result is selected
by the map below.

| Existing consumer / owner | Reusable Windows surface | Retained product contract |
| --- | --- | --- |
| W11-W01, S2.1; acquisition/platform | IP Helper adapter/route queries and notifications; Winsock interface options and packet timestamp APIs | Local configuration versus destination reachability, network generations, callback/buffer lifetime, timestamp provenance and actual completion. Keep the chosen Tokio orchestration. |
| W11-W02, S0.4/P4; clock/platform | Precise UTC, QPC, active-time and adjustment APIs | Clock-domain mapping, finite authorised correction, delivered-effect accounting and recovery; no general time-type replacement. |
| W11-W03; future DNS/platform | Native DNS queries and interface DNS settings | NRPT/VPN policy, query completion, exact owned field restoration and independent DoQ/DNSSEC/cache contracts. |
| W11-W04/05, S0.2/S0.3/P3; platform/IPC/security | SCM, token, named-pipe, overlapped-I/O and process-mitigation APIs | Actual token/ACL and peer proof, state transitions, callback/control errors, cancellation custody and effective protections. Retain Mullvad prototype and Microsoft plural service hold. |
| W11-W08, S3.4/S5.3; diagnostics/UI | ETW/Event Log and qualified emitter | Bounded typed/redacted events, authorised DEBUG expiry and required-event routing; tracing is not authoritative state. |
| ST-W01/03/04 and W11-W05; storage/lifecycle | NCrypt/CNG, protected file operations, conditional registry wrapper | VBS symbol/version coverage, actual key protection, directory/ACL/size limits and uncertain publication/recovery. Registry transactions do not cover service or file effects. |
| W11-W09, S0.5/P5; UI | Existing common controls and conditional window/Reactor/picker helpers | Preserve lifecycle/error observation and UI-M0 deferrals; compare runtime/package/accessibility/resource cost only at the separately scoped trigger. |

Before its next dependent adapter, record the exact operation -> selected
published crate/API/features/SDK symbols -> remaining invariant owner -> valid
and failing observation. Review the selected artifact rather than borrowing
current main's namespaces; wrapper support is not target-build support.
[W11 task §4.29](../../tasks/windows11-integration.md#429-windows-component-reuse-routing--5-november-2026)
owns that handoff, and [storage §6.7](../../tasks/windows-protected-storage.md#67-windows-binding-coverage-and-reuse--5-november-2026)
owns the specific VBS coverage gap. Existing CHECK-26/27 and per-surface checks,
Core priority and all native/fuzz/storage holds remain in force.

## 8. Primary sources

Retrieved through Windows Learn MCP during the 3 November 2026 investigation.
These are Microsoft documentation links, not pinned SDK headers or executed
platform evidence. Exact supported flags, bindings, build/provider/account and
effective behavior must be qualified when an adapter is selected. Citations in
the relevant sections preserve each source's purpose and known conflicts.

[T1]: https://learn.microsoft.com/windows/win32/api/iphlpapi/nf-iphlpapi-registerinterfacetimestampconfigchange
[T2]: https://learn.microsoft.com/windows/win32/api/iphlpapi/nc-iphlpapi-interface_timestamp_config_change_callback
[T3]: https://learn.microsoft.com/windows/win32/api/iphlpapi/nf-iphlpapi-unregisterinterfacetimestampconfigchange
[T4]: https://learn.microsoft.com/windows/win32/api/ws2def/ns-ws2def-wsamsg
[T5]: https://learn.microsoft.com/windows/win32/api/mswsock/nc-mswsock-lpfn_wsarecvmsg
[T6]: https://learn.microsoft.com/windows/win32/api/ws2ipdef/ns-ws2ipdef-in_pktinfo
[T7]: https://learn.microsoft.com/windows/win32/api/ws2def/ns-ws2def-wsacmsghdr
[T8]: https://learn.microsoft.com/windows/win32/winsock/overlapped-i-o-and-event-objects-2
[T9]: https://learn.microsoft.com/windows/win32/api/winsock/nf-winsock-closesocket
[T10]: https://learn.microsoft.com/windows/win32/api/winbase/nf-winbase-setfilecompletionnotificationmodes
[T11]: https://learn.microsoft.com/windows/win32/api/ioapiset/nf-ioapiset-getqueuedcompletionstatusex
[T12]: https://learn.microsoft.com/windows/win32/api/powerbase/nf-powerbase-powerregistersuspendresumenotification
[T13]: https://learn.microsoft.com/windows/win32/power/pbt-apmresumesuspend
[T14]: https://learn.microsoft.com/windows/win32/power/pbt-apmresumeautomatic
[T15]: https://learn.microsoft.com/windows/win32/sysinfo/acquiring-high-resolution-time-stamps
[T16]: https://learn.microsoft.com/windows/win32/winsock/ipproto-udp-socket-options
[T17]: https://learn.microsoft.com/windows/win32/api/ws2tcpip/nf-ws2tcpip-wsasetudprecvmaxcoalescedsize
[T18]: https://learn.microsoft.com/windows/win32/hyperv_v2/virtual-machine-generation-identifier
[L1]: https://learn.microsoft.com/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex
[L2]: https://learn.microsoft.com/windows/win32/api/combaseapi/nf-combaseapi-coinitializesecurity
[L3]: https://learn.microsoft.com/windows/win32/api/combaseapi/nf-combaseapi-couninitialize
[L4]: https://learn.microsoft.com/windows/win32/com/categorizing-dcom-proxies-and-stubs
[L5]: https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-getservicedirectory
[L6]: https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-getserviceregistrystatekey
[L7]: https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-getsharedservicedirectory
[L8]: https://learn.microsoft.com/windows/win32/api/winsvc/ne-winsvc-service_directory_type
[L9]: https://learn.microsoft.com/windows/win32/api/winsvc/ne-winsvc-service_registry_state_type
[L10]: https://learn.microsoft.com/windows/win32/msi/msilockpermissionsex-table
[L11]: https://learn.microsoft.com/windows/win32/msi/securing-resources-
[L12]: https://learn.microsoft.com/windows/win32/msi/formattedsddltext
[L13]: https://learn.microsoft.com/windows/win32/services/subscribeservicechangenotifications
[L14]: https://learn.microsoft.com/windows/win32/services/unsubscribeservicechangenotifications
[L15]: https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-notifyservicestatuschangew
[D1]: https://learn.microsoft.com/windows/win32/api/evntprov/nc-evntprov-penablecallback
[D2]: https://learn.microsoft.com/windows/win32/api/evntprov/nf-evntprov-eventwrite
[D3]: https://learn.microsoft.com/windows/win32/api/evntcons/nf-evntcons-eventaccesscontrol
[S1]: https://learn.microsoft.com/windows/win32/api/winbase/nf-winbase-replacefilew
[S2]: https://learn.microsoft.com/windows/win32/api/werapi/nf-werapi-wersetflags
[S3]: https://learn.microsoft.com/windows/win32/wer/collecting-user-mode-dumps
[S4]: https://learn.microsoft.com/windows/win32/api/ncrypt/nf-ncrypt-ncryptcreateclaim
[S5]: https://learn.microsoft.com/windows/win32/api/ncrypt/nf-ncrypt-ncryptverifyclaim
[S6]: https://learn.microsoft.com/windows/win32/api/winnt/ns-winnt-process_mitigation_side_channel_isolation_policy
[N1]: https://learn.microsoft.com/windows/win32/api/netioapi/nf-netioapi-getnetworkconnectivityhint
[N2]: https://learn.microsoft.com/windows/win32/api/netioapi/nf-netioapi-getnetworkconnectivityhintforinterface
[N3]: https://learn.microsoft.com/windows/win32/api/netioapi/nf-netioapi-notifynetworkconnectivityhintchange
[N4]: https://learn.microsoft.com/windows/win32/api/nldef/ns-nldef-nl_network_connectivity_hint
[N5]: https://learn.microsoft.com/windows/win32/api/windns/nf-windns-dnsqueryex
[N6]: https://learn.microsoft.com/windows/win32/api/windns/ns-windns-dns_query_request3
[N7]: https://learn.microsoft.com/windows/win32/api/windnsdef/ns-windnsdef-dns_custom_server
[N8]: https://learn.microsoft.com/windows/win32/api/windns/nf-windns-dnsqueryraw
[N9]: https://learn.microsoft.com/windows/win32/api/netioapi/ns-netioapi-dns_interface_settings
[N10]: https://learn.microsoft.com/windows/win32/api/netioapi/ns-netioapi-dns_interface_settings_ex
[N11]: https://learn.microsoft.com/powershell/module/dnsclient/get-dnsclientcache?view=windowsserver2025-ps

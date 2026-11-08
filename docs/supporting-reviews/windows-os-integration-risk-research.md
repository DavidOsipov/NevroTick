# Windows integration hazards: source map and vulnerability research

**Research date:** 8 November 2026 UTC.  
**Inspected NevroTick source:** `a78a8852bbd05dfd5f837a571fccb32b7b08fb81`, initially clean.  
**Purpose:** map actual and planned OS boundaries, use vulnerability databases to
challenge their assumptions, and hand concrete cases to existing implementation
owners. This is a bounded research review, not a complete code audit or a native
qualification result.

## 1. Result and interpretation

This pass did **not substantiate a new exploitable NevroTick vulnerability**.
It found useful platform semantics and recent third-party failures that should
shape the existing native qualification. Most relevant protections are already
specified in the architecture; their implementation and Windows evidence remain
separate obligations.

Two small refinements deserve explicit handoff:

1. In R-05, distinguish chain-building cache-only retrieval from revocation
   cache-only retrieval, and per-URL from cumulative revocation retrieval timeouts. A single
   similarly named flag cannot establish an offline, bounded verifier.
2. In ST-W03 and W11-W05/W11-W08/W11-W10, include **elevated error logs, crash
   records and other failure-side writes** in the protected-directory tests.
   Failure of the main operation must not leave a less-visible privileged write.

The CVEs below are either **external design analogues**, **fixed/nonmatching
dependency cases**, or **OS servicing leads requiring the actual target build**.
None receives a NevroTick severity. Database matches, a valid signature, a
configured protection and an executed application security boundary establish
different facts.

The [architecture](../windows-time-architecture.md),
[owner decisions](../nevrotick-owner-decisions.md) and existing CHECK/THR/OPEN
identities remain authoritative. The selected complete Store-first MSIX, all-file
signing and admitted interim self-signed MSI profile retain their exact scopes.
This review selects no updater, alternate service topology, customer trust-store
change or weaker verifier. Core ordering, post-V1 proof deferral and current
Windows, fuzz, storage and UI holds remain in force.

## 2. Inventory before vulnerability matching

**Implemented source** below means source is present; it does not mean production
adoption or native behavior was observed in this research. Indirect OS calls
through Rust/Tokio must be qualified against the selected target and dependency
graph, rather than inferred from a Rust method name.

| Boundary | Actual source or intended API | Current status and trust question |
| --- | --- | --- |
| Packet I/O and endpoint selection | [Engine manifest](../../crates/nevrotick-engine/Cargo.toml) and [transport](../../crates/nevrotick-engine/src/lib.rs): Tokio UDP, numeric socket addresses, OS randomness, monotonic and wall-time observations | Bounded observe-only experiment; its admission requires distinct loopback peers. No production DNS/TCP/TLS/NTS acquisition in this crate. Later remote endpoints, redirects, resolution and socket generations need their own admission. |
| Clock reads, deadlines and correction | Current `Instant`/`SystemTime` observations; planned `GetSystemTimePreciseAsFileTime`, `QueryPerformanceCounter`, `GetSystemTimeAdjustmentPrecise`, `SetSystemTimeAdjustmentPrecise`, `SetSystemTime` | [Architecture §10](../windows-time-architecture.md#10-windows-clock-backend) separates UTC, elapsed/active time, permission and delivered effect. The production clock broker/native mapping is not qualified. Sleep, VM restore and another time controller can invalidate otherwise valid observations. |
| SCM lifecycle | [Laboratory native adapter](../../tools/windows-service-fixture/src/native.rs): `StartServiceCtrlDispatcherW`, `RegisterServiceCtrlHandlerExW`, `SetServiceStatus` | Native source in standalone laboratory hosts. [Portable service contracts](../../crates/windows-platform/src/service.rs) do not prove installed identities, startup provisioning, actual stop or recovery. |
| Effective token and object authority | [Token adapter](../../tools/windows-service-fixture/src/native_token.rs): `OpenThreadToken`, `OpenProcessToken`, `GetTokenInformation`, privilege lookup and handle cleanup | Bounded self-observation exists; unexpected thread tokens and uncertain capture fail. The engine rejects presence of the three named forbidden privileges even when disabled. Full production allowlists, process/token access and final accounts remain open. |
| Local IPC | Planned named pipes, explicit descriptors, peer/token capture, impersonation/reversion and overlapped I/O | [§5.4](../windows-time-architecture.md#54-native-ipc-implementation-and-qualification) is a contract, not a qualified native implementation. Input SID/PID/path fields cannot supply caller authority; endpoint ownership and actual completion must survive churn. |
| File/state/registry authority | Planned protected mandatory-state owner, bounded optional storage, handle-based access, ACLs and lifecycle replacement | [§12.1](../windows-time-architecture.md#121-state-separation) and [storage task](../../tasks/windows-protected-storage.md) own the policy. A pathname, protected parent or encrypted blob does not alone prove final-object identity, secrecy or anti-rollback. |
| CNG primitives and provider lifetime | [CNG fixture](../../tools/crypto-cng-lifecycle/README.md), local `rustls-cng-crypto` derivative, `BCrypt*` handle and operation calls | Standalone source experiment. Rustls 0.23.46 and provider 0.1.2 are not a qualified TLS/NTS product. Known init/shutdown/concurrency seams remain in the existing crypto task. |
| Certificate/time/revocation policy | Planned Windows `CertGetCertificateChain` route through the preferred verifier candidate; separate Rustls and Schannel alternatives | [R-05](../nevrotick-research-handoff.md#7-r-05--windows-trust-with-a-wrong-system-date) and §18.7 own the selection. Candidate-time verification, service trust stores, revocation and root policy are not interchangeable. |
| Delegated networking and firewall | Planned DNS, PKI URL retrieval and service-scoped restrictions, including Windows Service Hardening evaluation | An engine rule does not automatically constrain traffic delegated to OS services. [W11-W03/W11-W04](../../tasks/windows11-integration.md#w11-w03-qualify-native-dns-ownership-and-coverage) retain resolution, actual destination and IPv4/IPv6 observations. |
| DLL loading and image admission | Actual UI/OS dependency graph; planned early load restrictions, final PE imports, package graph and CIG | [§15](../windows-time-architecture.md#15-exploit-protection-and-dll-loading) already distinguishes static imports, later loads and package identity. A signed executable or allowed signer class does not authorize every loaded component. |
| UI, UIA and COM/WinRT | [Active Slint entry](../../crates/nevrotick-ui/src/main.rs), local AccessKit derivatives, [tray](../../crates/nevrotick-ui/src/native_tray/windows.rs), [toast adapter](../../crates/nevrotick-ui/src/native_notifications_windows.rs) | Fixture UI; service/clock actions remain unavailable. The older `native.rs` Win32 adapter is not the selected exported entry. Toast source builds literal DOM nodes rather than parsing contributor XML; callbacks queue bounded observations rather than execute service commands. Native lifetime, identity and accessibility still need qualification. |
| Installation, maintenance and diagnostics | Planned complete MSIX, narrow elevated helper, W32Time handover, Event Log/ETW and declared crash collectors; MSI only if later selected | [§11](../windows-time-architecture.md#11-w32time-ownership-and-installation-lifecycle), §12.3 and §19 cover boot, repair, removal and recovery. Package replacement does not itself settle a clock effect, retire a service or protect every diagnostic sink. |

The [service handoff](../../tasks/windows11-integration.md#4358-service-source-handoff-and-integration-prerequisites--8-november-2026)
is the current scope authority for the laboratory adapters. No present production
pipe, protected-filesystem or SCM-provisioning implementation was inferred from
their portable types.

## 3. CVE evidence with applicability dispositions

Affected/fixed ranges below belong to the named external product. Recheck the
primary record before relying on a servicing threshold; these are dated research
observations, not an automatically maintained vulnerability feed.

| Record and primary evidence | Verified mechanism/range | NevroTick disposition and lesson |
| --- | --- | --- |
| **CVE-2026-10739**, [Cato CNA record][C1] and [vendor advisory][C2] | Cato SDP Client for Windows before 6.12.6 improperly validated a client-supplied SID over a local pipe, allowing SYSTEM file deletion. | **External analogue.** NevroTick §5.4.2 already requires OS-bound caller identity. Challenge a syntactically valid request carrying a forged SID/role; the field must not replace the captured token or current effect authorization. |
| **CVE-2026-107578**, [CNA][C3] and [hMailServer issue 71][C4] | CNA names 6.3.4/6.3.5, fixed in 6.3.6. A compromised low-privilege service could redirect paths later used by elevated administrative commands, including failure logs and crash records. | **External analogue; concrete test enrichment.** Include incidental elevated writes. The primary issue also describes analogous older custom-account configurations; do not reduce its prerequisites to any local user or claim every configuration is exploitable. Never import its temporary LocalSystem workaround into NevroTick. |
| **CVE-2026-106581**, [Docker CNA record][C5] | Before Docker Desktop 4.92.0, a signed Windows installer could execute actions from an unverified external `-package` input after UAC approval, as LocalSystem. | **External analogue.** Signature of the carrier/helper is not authorization of its inputs. Existing THR-20 and §19.6 require admitted component/release identity and protected custody through use. The CNA is the mechanism/range evidence; the linked current 4.92.0 release-notes section did not expose that CVE text during this pass. |
| **CVE-2026-13133**, [LINE vendor advisory][C6] | LINE Windows installer before 26.4.0 loaded `Msftedit.dll` through a relative, insecure search path. | **External analogue.** Exercise installer/helper and all actual entry paths with inert DLL canaries. Existing §15.2 already forbids ambient CWD/PATH reliance; an MSIX package graph has different search behavior from an unpackaged installer. |
| **CVE-2026-108468**, [Microsoft CNA record][C7] | Windows Installer local use-after-free/EoP. CNA `affected[]` includes x64 Windows 11 26H2 below 10.0.26300.9711, and Server 2025 below 10.0.26100.33650, within their listed product ranges. | **OS-baseline lead, not a demonstrated project path.** Actual host/guest build, UBR, servicing and reached component were not inspected. Use the two-family qualification matrix; do not treat a product-family label or a valid installer signature as evidence that Windows is patched. A later MSIX choice does not prove the whole host lacks Windows Installer exposure. |
| **CVE-2024-38014**, [Microsoft record][C8] and CISA KEV via Ansvar | Historical Windows Installer privilege-management vulnerability; KEV entry added 10 September 2024, with vendor remediation required. | **Historical exploited class.** KEV justifies attention to installer servicing and maintenance boundaries. It does not establish that supported 2026 targets or NevroTick are vulnerable, nor transfer a CVSS score to this project. |
| **CVE-2026-115567 / RUSTSEC-2026-0384**, [RustSec][C9] | Tokio 1.54.0 `Interval` race on its multi-thread runtime; fixed in 1.54.1. `Sleep` and `Timeout` are explicitly unaffected by that advisory. | **Nonmatching/fixed source case.** Current engine pins 1.54.1 and constructs a current-thread runtime. Preserve version/feature-aware checks when the separate post-V1 concurrency work changes the graph. This exclusion is not a general Tokio safety proof. |
| **CVE-2026-123609–123613**, [Rustls 0.23.46 release][C10], its four linked advisories and [the earlier TLS record-boundary advisory][C12] | NVD published these records on 8 November. The four issues 123609–12 are fixed in 0.23.46; 123613 was fixed in 0.23.45. The 0.23.46 release identifies decompression allocation, secret-bearing Debug output, unauthenticated hints and ECH rejection issues. | **Fixed/nonintegrated fixture cases.** Fixture pin is 0.23.46 and no product TLS transport is implemented. Still require exact provider/features, bounded handshake work and redacted secrets before adoption; a local provider patch needs source review beyond registry matching. |
| **CVE-2026-93599**, [WebPKI advisory][C11] | Malformed CRL panic, fixed in 0.103.13; depends on the CRL-processing path, which is opt-in in the stated upstream configuration. | **Fixed/nonintegrated fixture case.** Lockfile has 0.103.16; no current product verifier/CRL path was found. Do not report every TLS client as affected merely because WebPKI appears in a graph. |

For that Rustls group, the NVD MCP records for
[123609](https://nvd.nist.gov/vuln/detail/CVE-2026-123609),
[123610](https://nvd.nist.gov/vuln/detail/CVE-2026-123610),
[123611](https://nvd.nist.gov/vuln/detail/CVE-2026-123611),
[123612](https://nvd.nist.gov/vuln/detail/CVE-2026-123612) and
[123613](https://nvd.nist.gov/vuln/detail/CVE-2026-123613) supply the CVE-to-GHSA
mapping, respectively to `c8m5-c952-6cm4`, `h2v8-394m-p7vv`,
`w44r-93j7-qv28`, `vp88-798v-h67f` and `2mjx-qc3c-rqvc`. The upstream advisories
supply mechanism and version evidence. The fifth upstream page still says
"No known CVE"; its authenticated transcript prevents this defect from letting
a network-position attacker alter or complete a handshake. Its linked `5046bdf`
example concerns an analogous Go fix and is not used as a Rustls patch. [C12]

Other tempting matches were deliberately not promoted: the old Rustls server
loop CVE-2024-32650, Tokio named-pipe CVE-2023-22466 and Mio named-pipe
CVE-2024-27308 do not match the current versions and loopback UDP path.
Schannel CVE-2026-85818 is not automatically a Rustls+CNG issue. Cryptographic
Services CVE-2026-70014 and User Profile Service CVE-2026-83659 remain OS leads
without verified target-build/path applicability here. The Okta RADIUS installer
ACL case CVE-2026-78632 duplicates the existing same-account/owner-rights lesson;
it adds no independent requirement. OpenVPN pipe cases already cited in §5.4.5
were not counted as newly discovered gaps.

## 4. Platform behavior that deserves explicit attention

### 4.1 Pipe identity and rights are separate from safe message parsing

Microsoft documents that `FILE_GENERIC_WRITE` includes the bit used for
`FILE_CREATE_PIPE_INSTANCE`. A default descriptor is also unsuitable as proof
of the intended private endpoint. Existing §5.4 therefore requires individual
rights, protected descriptors and continuous endpoint custody. Test creation of
an impostor instance as well as connection permission. [M1]

`ImpersonateNamedPipeClient` binds to the calling thread and last message read;
identity must be captured before read-ahead can change the relevant context.
Failed impersonation cannot fall through as the server, and failed reversion
cannot leave the process doing later privileged work. These are already explicit
contracts, not newly alleged defects. [M2], [M3]

### 4.2 Account names, disabled privileges and parent directories are weak evidence

Required service privileges take effect at service startup, and shared-process
services receive their union. The engine's present-but-disabled clock privilege
is unacceptable; its current source observer correctly rejects it. Configured
service settings, activation point and the observed effective token remain
separate. [M4]

Objects created under a shared base account require final owner/DACL analysis;
encryption or a service SID alone does not establish secrecy from other services.
Creation, rename/replacement and already-open handles must be evaluated separately.
Existing §4.1, ST-T03 and ST-W03 already require these controls. [M5]

### 4.3 Certificate time, retrieval and timeout settings are not one switch

`CertGetCertificateChain` documents that supplied `pTime` does not recreate
historical root/trust-list state or replace every revocation time check. In
particular, the CRL itself must be valid at current time. A provisional NTS clock
therefore needs its own qualified trust policy under R-05. [M6]

Record separately `CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL` and
`CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY`: the first does not govern revocation
retrieval. Also record whether the revocation URL retrieval timeout is per URL or
cumulative under `CERT_CHAIN_REVOCATION_ACCUMULATIVE_TIMEOUT`. That cumulative
limit concerns revocation retrieval, not all chain-building traffic. These settings do not prove
an overall TLS deadline or absence of delegated OS traffic. Selecting a blanket
offline or soft-fail policy is outside this refinement. [M6], [M7]

### 4.4 A requested cancellation can finish successfully later

`CancelIoEx` does not wait for completion, and a cancellation can lose to normal
success. The operation's buffers and `OVERLAPPED` cannot be reused merely because
cancel returned. [M8] Existing THR-26/§17.1 also retains queue/worker charges for
nonpreemptible certificate/DNS work until actual settlement. A wrapper timeout,
dropped future or portable `WorkPermit::complete` assertion is not independent
proof of native completion. Keep stale-generation publication rejection separate
from resource lifetime.

### 4.5 A signature does not validate privileged parameters or loaded dependencies

The Docker and LINE cases support the existing distinction between authenticated
code and authorized input/load paths. Windows DLL search also has loaded-module,
API-set, package-graph and dependency rules; late hardening cannot retroactively
protect earlier imports. An absolute top-level DLL path alone is not a complete
dependency search policy. [C5], [C6], [M9]

For a later MSI route, `SecureCustomProperties` names properties passed to the
elevated server side; it is not semantic validation or confidentiality. Treat
public properties, transforms, repair and deferred custom-action inputs under
the existing fixed-verb/current-authorization contract. [M10], [M11] Apply the
declared signing profile without inventing customer certificate imports or a
public-CA-only prohibition of the admitted interim profile.

### 4.6 Recovery, UI callbacks and diagnostic collectors remain distinct owners

The service source consumes its status-token custody before terminal reporting;
Microsoft warns that another `SERVICE_STOPPED` call can use a closed RPC context.
That is useful static evidence, not a native service lifecycle pass. [M12]

Keep boot-before-UI, delayed restart during maintenance, Reset/last-user removal,
one clock owner and surviving recovery access in CHECK-5/12/13/23/27. Include
UTC jumps, suspend and VM restoration in the existing timestamp/freshness work.

UIA, native messages and toast activation are untrusted input channels, even
when normal-user UI code can safely parse their values. Existing source guards
and literal toast DOM construction are contrary evidence to an assumed arbitrary
XML-command path. Preserve callback apartment/reentrancy, bounded ingress,
unregistration and lifetime tests at S0.5/P5. WER, application crash output and
administrator-controlled collectors retain separate privacy claims under
CHECK-17; no universal dump-secrecy result follows from one exclusion setting.

## 5. Exact-version dependency lookups

The parent parsed lockfiles as data without executing Cargo, build scripts or
repository Python modules. The OSV MCP query covered **629 unique registry
name/version pairs** from these five lockfiles:

| Lockfile | Registry entries before cross-file deduplication |
| --- | ---: |
| Root `Cargo.lock` | 533 |
| `tools/windows-service-fixture/Cargo.lock` | 16 |
| `tools/crypto-cng-lifecycle/Cargo.lock` | 51 |
| `tools/crypto-aes-siv-witness/Cargo.lock` | 20 |
| `tools/state-guidance/engine/Cargo.lock` | 125 |

The database summary returned five advisory matches, 624 entries with no match,
zero errors and zero upstream truncations. The large MCP response abbreviated
its row presentation; all five flagged rows were subsequently queried in full.
An abbreviated unshown row was not independently inspected and is not a security
pass. Separate source-grounded network queries also checked candidate versions;
they did not replace this lockfile scan or qualify a product graph.

| Exact package | Advisory | Source-policy disposition |
| --- | --- | --- |
| `bincode` 2.0.1 | [RUSTSEC-2025-0141][D1] | Informational unmaintained; root lock only; recorded as metadata-only. |
| `ttf-parser` 0.25.1 | [RUSTSEC-2026-0192][D2] | Informational unmaintained; root lock only; recorded as metadata-only. |
| `zbus_names` 4.3.5 | [RUSTSEC-2026-0371][D3] | Informational unmaintained after upstream consolidation; metadata-only. |
| `zvariant` 5.15.1 | [RUSTSEC-2026-0370][D4] | Same maintenance category; metadata-only. |
| `zvariant_derive` 5.15.1 | [RUSTSEC-2026-0372][D5] | Same maintenance category; metadata-only. |

All five appear in `metadata_only_packages` and not in either target row's
selected `features` map in
[the maintained graph policy](../../policy/ui-m0-dependencies.json).
That is source-policy evidence, **not a fresh Cargo graph or emitted-binary
inspection**. These warnings justify watching feature/graph changes, not an
urgent runtime migration or a claim of five exploitable bugs. Existing
`deny.toml` keeps unmaintained/unsound review and an empty ignore list.

This is not a complete SBOM audit: local derivatives (`spin_on`, AccessKit and
the CNG provider), native/OS modules, compiler/SDK, build-tool execution,
JavaScript prototype tooling, formal/fuzz/other lock variants and actual package
contents need their applicable separate evidence. A no-match OSV result cannot
establish safety of source changes or APIs used incorrectly.

## 6. Handoff to existing owners

The order below expresses usefulness at the relevant stage, **not vulnerability
severity or a replacement for development §7**. Native rows remain held until
their existing environment/authorization gates admit execution. Tests use owned
disposable Windows fixtures, dummy identities/data and unrelated sentinel files;
no deployed services, real secrets or user clock changes are required.

| Existing owner and boundary | Bounded next action | Independent hostile and legitimate neighbors |
| --- | --- | --- |
| **R-05 / CRYPTO-04**, OPEN-8; THR-4/21/26; CHECK-8/9/26/27 | **Explicit refinement:** record both cache-only controls, actual verifier exposure/configuration, time sources and per-URL versus cumulative revocation retrieval timeout. If the selected wrapper cannot express policy, retain that admission gap. | Cold/missing or stale revocation material, blocked/redirected URL and wrong system date must produce the declared refusal/unknown result without prohibited traffic or false good status; the permitted chain and admitted cached/network path still succeed. Observe delegated traffic separately. |
| **ST-W03 + W11-W05/08/10**, THR-9/20; ST-T03/10/17 and CHECK-13/15/17/23 | **Explicit refinement:** enumerate elevated failure-side writes before the first effect, including startup errors, crash records, rollback and cleanup diagnostics. Reuse protected directory/object authority rather than a special logger bypass. | A harmless main-operation failure with a substituted service-writable parent/leaf must leave an unrelated sentinel unchanged; admitted useful diagnostics and a legitimate operation still work. Include reparse, hardlink, type, owner and retained-handle neighbors. |
| **S0.3/P3 platform/IPC**, THR-6/7/8/22; CHECK-2/3/12/25/27 | Execute the already-specified mutual identity, exact rights and thread-bound capture/reversion cases on the real adapter. | Forged SID/role, second server instance, read-ahead/context change and failed reversion cause no protected effect; ordinary concurrent authorized clients and a genuine restarted server work. |
| **W11-W05/S0.2/S3.1**, THR-5/6/16/23; CHECK-1/5/12/13/27 | Finish final account/privilege/object policy and activation evidence; retain truthful status/worker settlement during maintenance. | Forbidden privilege merely disabled, second same-account service and delayed SCM restart are refused or contained; intended startup, bounded work, stop and approved recovery succeed. |
| **Engine/platform/crypto**, THR-26; CHECK-8/9/11/12/21/27 | Qualify bounded execution of synchronous verification/native work before production TLS adoption. Retain the existing CNG initialization/shutdown source leads in S0.1. | Cancel/retry pressure and late native success do not free capacity/storage early or publish into a new generation; healthy requests reclaim capacity and shutdown observes actual settlement. No new worker-pool topology is selected. |
| **W11-W10/P6 and load-policy owner**, THR-14/15/20/23; CHECK-16/18/23/26 | Carry signed-carrier/untrusted-input and DLL-search analogues into the existing whole-artifact/install/repair controls. | Wrong package/component/release, untrusted maintenance input and inert candidate DLL are refused at their real consumers; valid package, dependency loads and authorized repair work. Keep actual Store-delivered identity and interim-profile trust results distinct. |
| **W11-W02, W11-W09 and W11-W08**, THR-10/11/12/18; existing CHECK-10/11/12/17/21/27 and UI gates | Retain clock discontinuity, UI callback/identity and collector-specific cases already owned by those stages. | Stale observations, late callbacks and prohibited diagnostic content cannot regain authority; useful fresh measurements, accessible UI and approved redacted diagnosis remain available. |
| **Two-family matrix / P6–P7**, CHECK-18/20/23/26 | Bind relevant OS advisories to exact component/build/UBR/KB and installed state for each declared target. Refresh on qualification, not by assuming the research date is a permanent clean bill. | A known affected/unresolved baseline cannot inherit a patched-baseline receipt; an actually patched supported fixture can complete normal installation/runtime controls. Retain vendor-confirmed applicability or exact uncertainty. |

No new release gate, runtime checker or universal CVE blocklist is introduced.
The [existing W11 record](../../tasks/windows11-integration.md#437-os-integration-vulnerability-research--8-november-2026)
owns integration and review; R-05 and ST-W03 own the two targeted refinements.

## 7. Research coverage and limits

Three separate agents contributed: network/TLS/CNG research; service/IPC/file
research; and an applicability judge. The parent mapped loading, packaging,
current UI surfaces and dependency queries, and corroborated the consequential
primary facts. These were separate read-only agents **with inherited context**,
not statistically independent audits or fresh-context reviews. The judge's
completed scope/result is retained in the W11 task.

MCP tools actually used were **NIST NVD search/get**, **OSV ecosystem/batch/detail**,
**Microsoft Learn search/fetch**, and **Ansvar freshness/KEV**. Ansvar reported
same-day NVD recent and KEV synchronization; its KEV response for CVE-2024-38014
reported a 19:15 UTC sync on 8 November. Database freshness does not prove complete
coverage, correct product mapping or a patched endpoint.

Discovery was bounded: service researcher retrieved 21/21 recent named-pipe
matches and 33/33 recent Windows-link matches, but only 100/314 Windows-service
matches. Network research included Rustls/Tokio and recent Windows crypto/DNS;
the DNS screen was 8/48. Parent installer discovery retrieved 30/87 matches over
120 days, then all 54 matches in the narrower last-45-day window. These screens
are not a deterministic exhaustive Windows CVE inventory. Selected records were
read in detail and checked against primary vendor/CNA/upstream sources.

MSRC's dynamic pages were sometimes unreadable; where exact version evidence
was needed, the corresponding Microsoft-authored CVE record was read instead.
For CVE-2026-108468, use its `affected[]` product/platform rows rather than rely
only on the less-complete CPE representation. Latest-version Tokio documentation
was not treated as exact-version execution evidence. External page instructions
and temporary vendor workarounds did not alter project policy.

No target code, build, test, PoC, endpoint probe, Windows API, installer, signer,
credential or service operation was executed. This session lacks the complete
OS-enforced isolation required by the security-audit skill for target-controlled
execution; previously failed namespace isolation was not retried unchanged.
Current project execution holds independently remain. Static source/manifest
reads, advisory retrieval and document validation cannot prove runtime reachability,
absence of memory errors, exact OS patch state or release readiness.

Revisit only affected conclusions when source/features/provider/packaging,
service account, OS servicing, policy or advisory data changes. The next agent
should take the relevant bounded owner row, not restart the whole CVE survey.

## 8. Skill and document integration — 9 November 2026

**Integration date:** 9 November 2026, Asia/Tbilisi. The research snapshot and
lookup receipts above remain dated 8 November UTC. This follow-up carries the
useful rules into existing implementation and QA entry points; it changes no
product source, dependency graph or native qualification result.

| Research lesson | Canonical or task owner | Reusable entry points and disposition |
| --- | --- | --- |
| Chain/revocation retrieval, time and native-job scope | [Architecture §18.7.2](../windows-time-architecture.md#1872-certificate-policy-is-a-separate-windows-integration-decision), [R-05](../nevrotick-research-handoff.md#7-r-05--windows-trust-with-a-wrong-system-date), existing S2.2 crypto card | Updated [security-crypto-secrets](../../.agents/skills/security/security-crypto-secrets/SKILL.md), [security-protocol-inputs](../../.agents/skills/security/security-protocol-inputs/SKILL.md) and [Windows host integration](../../.agents/skills/security/references/windows-host-integration.md#firewall-and-wfp) to reach those owners. Exact flag details remain in R-05. |
| Actual verifier/service-context evidence | [Hardening §9.11](../windows-time-development-hardening.md#911-windows-integration-qualification-refinements), OPEN-8/CRYPTO-04 and CHECK-8/9/26/27 | Added a shared [certificate/provider QA route](../../.agents/skills/testing/references/windows-evidence.md#certificate-and-crypto-provider-evidence), already reached by security/lifecycle QA. Provider primitives, TLS implementation, certificate policy and actual completion retain distinct evidence. |
| Privileged failure-side file effects | [Architecture §12.1](../windows-time-architecture.md#121-state-separation), §12.3.4 and [ST-W03](../../tasks/windows-protected-storage.md#st-w03-constrain-directory-authority-before-destructive-effects) | Explicit rule in [persistence/resources](../../.agents/skills/security/references/persistence-and-resources.md), reached from [security-privacy-logging](../../.agents/skills/security/security-privacy-logging/SKILL.md) and the [service adapter guide](../../.agents/skills/implementation/implement-rust/references/windows-service-adapters.md#establish-effective-unattended-error-behavior). Product-owned files and external OS collectors keep separate scopes. |
| Failure without a secondary forbidden write, with useful diagnosis | [Hardening §9.8](../windows-time-development-hardening.md#98-diagnostics-and-shared-adapter-verification), ST-W03/W11-W05/08/10; CHECK-13/15/17/23 | Updated [qa-fault-recovery](../../.agents/skills/testing/qa-fault-recovery/SKILL.md) to the unchanged-sentinel and admitted-diagnostics/legitimate-operation controls. A redaction check alone cannot establish destination authority. |
| Advisory applicability and maintenance warnings | Architecture §18.10/CHECK-26; dated records in this review and existing task | [Security contract](../../.agents/skills/security/CONTRACT.md) now requires primary mechanism/range, actual target/path and prerequisites. [dependency-updates](../../.agents/skills/environment/dependency-updates/SKILL.md) separates maintenance notices, selected features, host execution, local patches and incomplete lookups. No dated package allowlist or warning suppression is adopted. |
| OS servicing evidence | Existing two-family qualification matrix and applicable vendor record | [Windows evidence](../../.agents/skills/testing/references/windows-evidence.md#native-execution-and-isolation) binds relevant advisories to actual component/build/UBR/KB state. A family label or package signature does not establish patch state. |
| Signed carrier and privileged maintenance inputs | Architecture §19.6 and W11-W10, conditional MSI route only | [security-build-deployment](../../.agents/skills/security/security-build-deployment/SKILL.md) clarifies that `SecureCustomProperties` forwards values without validating them or granting confidentiality. Existing component/release, fixed-operation, custody and load rules remain controlling. |

Existing `implement-windows-service` already selects privacy/logging and the
service guide; its entry point needs no copied paragraph. The inspected
`security-windows-privileges`, `security-local-ipc`, native-proof and
`qa-state-concurrency-lifecycle` instructions already cover effective tokens,
pipe instance/identity/reversion, custody and real completion. Existing package
QA, metadata/signing and DLL-load rules already cover authenticated content,
actual dependencies and final delivered bytes. Their current contracts remain;
this follow-up adds no duplicate CVE catalogue or new skill leaf.

The documentation map and [W11 integration record](../../tasks/windows11-integration.md#4371-skill-and-canonical-guidance-integration--9-november-2026)
retain candidate-specific review/validation and remaining gaps. The existing
development queue already routes this research through W11 §4.37; its order,
Core priority, Store-first/interim-signing decisions and execution holds remain.

## Primary references

The references below are linked at the claims and dispositions they support.
Project paths identify the inspected source snapshot; external records were
retrieved on the research date and retain their own publication/update history.

[C1]: https://raw.githubusercontent.com/CVEProject/cvelistV5/main/cves/2026/10xxx/CVE-2026-10739.json
[C2]: https://knowledge.catonetworks.com/docs/cve-2026-10726-cve-2026-10739-impacts-windows-client-versions-lower-than-6126
[C3]: https://raw.githubusercontent.com/CVEProject/cvelistV5/main/cves/2026/107xxx/CVE-2026-107578.json
[C4]: https://gitlab.com/hmailserver/hmailserver/-/work_items/71
[C5]: https://raw.githubusercontent.com/CVEProject/cvelistV5/main/cves/2026/106xxx/CVE-2026-106581.json
[C6]: https://line.github.io/security-advisory-blog/CVE-2026-13133/
[C7]: https://raw.githubusercontent.com/CVEProject/cvelistV5/main/cves/2026/108xxx/CVE-2026-108468.json
[C8]: https://msrc.microsoft.com/update-guide/vulnerability/CVE-2024-38014
[C9]: https://rustsec.org/advisories/RUSTSEC-2026-0384.html
[C10]: https://github.com/rustls/rustls/releases/tag/v/0.23.46
[C11]: https://github.com/rustls/webpki/security/advisories/GHSA-82j2-j2ch-gfr8
[C12]: https://github.com/rustls/rustls/security/advisories/GHSA-2mjx-qc3c-rqvc
[M1]: https://learn.microsoft.com/en-us/windows/win32/ipc/named-pipe-security-and-access-rights
[M2]: https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-impersonatenamedpipeclient
[M3]: https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-reverttoself
[M4]: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_required_privileges_infow
[M5]: https://learn.microsoft.com/en-us/windows/win32/fileio/file-security-and-access-rights
[M6]: https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/nf-wincrypt-certgetcertificatechain
[M7]: https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/ns-wincrypt-cert_chain_para
[M8]: https://learn.microsoft.com/en-us/windows/win32/api/ioapiset/nf-ioapiset-cancelioex
[M9]: https://learn.microsoft.com/en-us/windows/win32/dlls/dynamic-link-library-search-order
[M10]: https://learn.microsoft.com/en-us/windows/win32/msi/securecustomproperties
[M11]: https://learn.microsoft.com/en-us/windows/win32/msi/guidelines-for-securing-custom-actions
[M12]: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-setservicestatus
[D1]: https://rustsec.org/advisories/RUSTSEC-2025-0141.html
[D2]: https://rustsec.org/advisories/RUSTSEC-2026-0192.html
[D3]: https://rustsec.org/advisories/RUSTSEC-2026-0371.html
[D4]: https://rustsec.org/advisories/RUSTSEC-2026-0370.html
[D5]: https://rustsec.org/advisories/RUSTSEC-2026-0372.html

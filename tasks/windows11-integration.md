# Windows 11 integration: agent task and evidence record

**Current delivery amendment — 21 November 2026:** [OD-05 §7.9](../docs/nevrotick-owner-decisions.md#79-direct-msix-distribution-microsoft-store-rejected--21-november-2026)
selects one complete MSIX for direct distribution and rejects Microsoft Store
publication. Signing and certificate trust are deferred; permitted source,
resource and package preparation continue. [§4.39](#439-direct-msix-distribution-decision--21-november-2026)
is the current W11-W10/MD-P4–MD-P5/WGT-0 handoff. Older Store-attempt,
refusal-first, Partner Center and `msstore`-first directions below are superseded;
their dated research and execution receipts retain only their historical scope.

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

**CPU/hardening correction, 6 November 2026:** the [existing build task](build-optimization.md#required-v3-and-maximum-compatible-hardening--6-november-2026)
records the now-configured v3 minimum, expanded required build protections and
shared admission controls. W11-W05/W11-W10 and existing CHECK-16/18/20/23/26
owners must implement and qualify every applicable final per-role protection at
the strongest supported compatible setting. Unknown or unexecuted is open work;
document concrete exclusions instead of silently retaining a weaker profile.
Final PE/DLL/runtime/CPU-startup observations remain pending under the existing
native hold; this amendment grants no Windows execution or release approval.

**Primary coordinator checkpoint, 7 November 2026:** the latest pull and
[bounded reference-bridge exit](engine-transport-pilot.md#common-run-reference-bridge--final-qualification-and-next-action)
preserve the W11-W01–W11-W10 ownership map in §4.12, as amended by §4.17 and
later task-specific records. W11-W01 remains partially prepared under §4.26;
the bridge adds no packet timestamp or Windows mapping evidence. W11-W02–W11-W05
and W11-W08 retain their phase owners and native hold; W11-W06/W11-W07 remain
conditional on their named DNS baselines; W11-W09 retains D2/D3 re-entry. Selected
W11-W10 MSIX preparation and David's Store attempt retain §4.17's owners and
next actions. The next pure Core conditioning contract changes no Windows adapter,
package or service. The §4.4/§4.31/§4.32 refinements and all execution holds remain
with those existing owners; no new source card or native result is claimed.

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

Continue **one W11-W01 timestamp supplier implementation card** from the
[partial preparation in §4.26](#426-s21-timestamp-provenance-preparation--5-november-2026)
when the relevant acquisition owner is assigned. Its engine handoff preserves
real acquisition facts and separates experimental measurement from later Core
admission. The [finite supplier/receiver contract](engine-transport-pilot.md#w11-w01-finite-capturemapping-supplier-contract--8-november-2026)
was settled on 8 November; the actual mapping issuer, justified finite physical
profile, selected SDK/backend and native evidence remain open. Apply the
[18 November qualification matrix](#timestamp-qualification-matrix--18-november-2026)
to the reached supplier or optional adapter; do not reopen the settled contract.
This source/documentation action does not replace development §7's ordinary
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
records partial implementation preparation through the existing engine card.
The finite supplier/receiver contract is settled; actual mapping/capability
issuers, their qualified physical profile and selected optional adapters/native
evidence remain open. The matrix below refines this existing assignment.

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

**Literature refinement — 13 November 2026:** [OD-18](../docs/nevrotick-owner-decisions.md#22-od-18--literature-derived-qualification-before-v1)
places inexpensive actual-consumer regressions before V1. The
[non-core research handoff](../docs/supporting-reviews/chrony-ntpsec-noncore-research.md)
maps Langer's latency experiments to this owner. Identical captured T1/T2/T3/T4
with later processing must preserve phase/RTT and increase only observation age;
coherent send-completion capture changes must not replace nominal T1. The
[engine task](engine-transport-pilot.md#literature-regression-source-preparation--13-november-2026)
owns the exact new tests and their execution status.

The inspected protected path seals before local send capture and captures receive
before authentication. A full crypto-duration subtraction would therefore
introduce a correction for time outside those captures. Preserve the separate
wire-correlation token and authenticated bytes. The two numerical tests exercise
the existing measurement consumer, not the complete protected-acquisition path.
Its producer-order and native-adapter evidence retain their own qualification.

Later selected adapter trials must pair application/OS captures under the same
estimator and independently observed timing truth, including matched and mixed
endpoint hardware. Include valid, delayed and missing-metadata controls. Report
capture fidelity, uncertainty and total synchronization error separately:
Langer's mixed-hardware result worsened after compensation. The paper selects no
universal correction, Windows busy-poll/power setting or measured product gain.
Required S2.1/P2 provenance stays on the V1 path; optional adapter adoption still
requires its existing contract and native evidence.

#### Timestamp qualification matrix — 18 November 2026

**Scope/status:** documentation and acceptance refinement against main
`238b9aed6c832d5d5be4a306357c52510395f144`. The
[fresh Microsoft source detail](../docs/plans/windows11-integration-follow-up.md#additional-packet-timestamp-checks--18-november-2026)
supplies API facts and discrepancies. Every added case below is **planned, not
executed by this update**. TQ labels are local case references under existing
CHECK owners, not new acceptance gates. Earlier executed cases keep their exact
candidate and scope; missing native/NIC evidence does not invalidate those results.

**Placement:** finish the required application capture/mapping supplier and its
actual consumer at S2.1/P2. Prepare inexpensive cases at the reached owner before
V1 under OD-18. Optional Winsock software evaluation comes first; hardware
adoption is conditional on its own measured benefit and qualified mapping. A V1
using a qualified application path need not implement packet timestamping or
support NIC hardware timestamps. If an optional path is shipped, its applicable cases
must pass before use. Development §7, post-V1 order and execution holds remain.

| Case / existing checks | Independent controls and required observation |
| --- | --- |
| **TQ-01 — clocks and conversions**; CHECK-7/11/25 | Preserve UTC, QPC, active-awake and NIC domains and original capture placement. Pair normal conversion with equal/adjacent cross-thread QPC ticks, wide capture brackets, range/rounding/overflow boundaries and an external wall step. Ambiguous order stays uncertain; QPC continuity grants neither UTC mapping nor delivered correction. Required for the baseline supplier; NIC inputs apply only when selected. |
| **TQ-02 — capability and activation**; CHECK-6/11/25 | Pair genuinely active general-packet timestamping with unsupported API/driver, query failure, supported-but-disabled, PTP-only, RX-only/TX-only and contradictory mode observations. Check `AllReceive` for generic RX and `AllTransmit` or `TaggedTransmit` for generic TX in the actual source set. Record each direction independently; a full-path label requires both. Directions omitted from `TIMESTAMPING_CONFIG.Flags` are disabled: challenge a later RX-only/TX-only call beside correct combined configuration. Software/hardware comparisons use separate interface modes, with application captures retained in each. Socket configuration success or both supported sets is no proof of an active mode. |
| **TQ-03 — actual packet path**; CHECK-6/11/27 | Controlled IPv4/IPv6 and two-adapter cases bind exchange, socket/association generation, RX interface/local destination and qualified TX selection. Switch routes/interfaces and challenge stale interface-index reuse and virtual/VPN attribution beside stable paths. Outer tunnel or later SOCKS-relay timing cannot stand in for the inner exchange without demonstrated association. Preserve current interface/family/trust policy; SOCKS-specific execution remains deferred. |
| **TQ-04 — receive metadata**; CHECK-6/21/25/27 | Retain the existing payload/control truncation and ABI cases; add absent, duplicate/conflicting and wrong-length timestamp messages plus a valid neighboring message. Treat `UINT64` as source-domain ticks, not nanoseconds/UTC. A sound payload does not repair bad metadata. Use only an independently admitted original application capture as fallback, or refuse the measurement. |
| **TQ-05 — transmit lifetime**; CHECK-6/21/27 | Pair immediate and delayed success with repeated `WSAEWOULDBLOCK`, a full buffer, absent records, retrieval after deadline/Stop, socket recreation and `UINT32` ID exhaustion/reuse. Freeze finite outstanding/queue/poll/retention budgets; no busy wait or reuse while an old record can match. Prevent or explicitly refuse `UDP_SEND_MSG_SIZE` coalesced TX on this path. Retrieval consumes the matching record once; late records cannot renew sample age, alter authenticated bytes or revise an already committed estimate. Observe bounded cleanup and a subsequent valid exchange. |
| **TQ-06 — NIC correlation**; CHECK-7/11/25 | Check `SupportsCrossTimestamp` and real capture success for this candidate. Independent rate/offset/drift fixtures and later native brackets exercise NIC→QPC→UTC with conservative composed uncertainty and finite model validity. Pair valid calibration, including zero/rounded nominal frequency, with failed cross capture, wide/inverted brackets, expired calibration, overflow and counter reset/discontinuity. Nominal frequency, midpoint or regression residual alone is no physical error guarantee. Unknown mapping yields fallback/refusal, never zero uncertainty. |
| **TQ-07 — transitions and retirement**; CHECK-11/21/27 | Existing configuration/callback/stop cases include same-mode NIC restart, driver/device change, sleep/hibernate/Modern Standby where supported, and lost UTC-mapping continuity. Retire affected issuers, queued metadata and crossing exchanges before final admission; fresh re-query/recalibration permits a healthy later exchange. Unknown transition coverage remains unavailable. Observe real native completion and callback draining before releasing context/resources; mock traces do not qualify native behavior. |
| **TQ-08 — fallback and truthful state**; CHECK-6/8/14/25 | Pair unavailable optional timing with a qualified application fallback, and with absent/stale application mapping. The first preserves useful acquisition with actual per-leg source and justified uncertainty; the second refuses production admission. Any RX/TX mixed-source path needs explicitly qualified conversions/error composition. No automatic NTS→NTP, route escape, renewed age, zero error or synthetic hardware-success label follows timing failure. |
| **TQ-09 — actual final consumer**; CHECK-6/7/11/25/27 | Trace original capture/packet identity → calibrated mapping → measurement → serialized consume-once admission. Challenge model/configuration replacement immediately before commit and TX arrival after publication, beside a valid same-generation exchange. Late facts may remain historical diagnostics but cannot update current history/readiness twice. Keep request wire-correlation bytes separate from local measured T1; no post-authentication rewrite or unconditional crypto-duration subtraction. |
| **TQ-10 — measured benefit and cost**; CHECK-7/8/21/24 | Compare application plus software, then application plus hardware under matched estimator, controlled peer/path, independent timing truth and declared idle/load conditions. Include mixed endpoint capabilities, missing/delayed metadata, both claimed families and stop/recovery. Report local capture latency, calibration/total uncertainty, end-to-end error, loss/fallback counts and CPU/wakeups/memory separately. Better placement alone proves no UTC improvement; unsupported hardware is an explicit optional outcome. |

**Native evidence and exit.** Bind each claimed result to the exact candidate,
OS family/build/servicing, SDK/bindings, effective service token/protections,
NIC/device/driver/firmware, interface/family/path and requested/active/observed
mode. Retain raw brackets/units, mapping revisions/validity/error terms, TX
capacity and polling limits, case counts, failures/missing records, independent
reference uncertainty and restoration outcome. Unknown, unsupported, not run,
failed and passed are distinct. Apply the [two-family matrix](#432-two-family-windows-qualification-matrix--5-november-2026);
client or virtual-NIC results do not establish Server or physical-NIC accuracy.
Identify whether the independent reference establishes relative timing or
traceable UTC; relative improvement alone supports no absolute UTC claim.
Freeze numeric budgets/tolerances before the bounded trial; no internet-accuracy
SLA or driver-setting change is selected here. Device configuration stays with
the authorized lifecycle owner, without broader engine privileges or host-policy
changes. Existing matching CHECK gates own release acceptance.

**Actual application capture comparison, 19 November 2026.** The
[Engine witness](engine-transport-pilot.md#actual-observe-cli-capture-bracket-witness--w11-w01-19-november-2026)
covers two matched source/receiver intervals through the real observe CLI on
the selected Windows Sandbox. The independently recorded peer intervals
satisfy the reported application lower bounds on the shared QPC clock.
Both reports remain mapping-unavailable and ineligible. This reaches the
selected virtual application path under TQ-01/08/09. Finite physical
errors, independent UTC, qualified invalidation and Client/Server release
evidence remain open.

**Current Windows clock source check.** The admitted installed Rust 1.100.0
`rust-src` matches the reviewed 1.99.1 Windows clock files byte for byte:

| Source under `library/std/src` | SHA-256 |
| --- | --- |
| `sys/time/windows.rs` | `3e344b796c406f5446dc0d31bb7737302723f21b4c606ed2b30794731e9b53de` |
| `sys/pal/windows/time.rs` | `59ba46af23ba2c12507bbb96d11bf76abe53df11397bd1baac8f864b378b6b1e` |
| `sys/helpers/mod.rs`, `mul_div_u64` function body | `fe740b3000d7e86bea56ab4631bed2786e2a2d8af581df1d7bf07a3f23112668` |

For nonnegative admitted counter values, positive common frequency and
the source arithmetic fit conditions (including the fixed epoch offset),
Windows `Instant` stores the floored QPC-to-nanosecond conversion plus
that offset. The conversion truncates each endpoint by less than 1 ns.
Subtracting two coordinates cancels the offset and leaves an arithmetic
representation error strictly between −1 ns and +1 ns. The current
`check_capture_order` compares the original `Instant` coordinates before
subtraction, so the standard library's small-backward-subtraction
tolerance cannot hide a reversed coordinate from admission.

This source result supports TQ-01 representation and ordering review.
Finite capture-placement error, mapping validity, physical error and
UTC traceability still require the qualified independent reference,
uncertainty envelope and invalidation owner in the
[existing supplier contract](engine-transport-pilot.md#w11-w01-finite-capturemapping-supplier-contract--8-november-2026).
The protected numerical caller continues to provide no mapping.

##### Documentation review and validation — 18 November 2026

**Candidate and decision:** documentation-only change across 11 Markdown files,
based on `238b9aed6c832d5d5be4a306357c52510395f144`. A fresh-context,
read-only independent reviewer inspected the integrated v2 patch (SHA-256
`82d6463139a111fc3e95ce3feda382f60e48751050d6e4a1a6ce2cf752ce9d77`),
the canonical owners and the Microsoft API sources. Verdict: **READY for
documentation integration**. Its two comprehension paragraphs are retained
verbatim:

> The patch turns the existing Windows timestamp work into ten concrete qualification cases, covering clock domains, directional capabilities, receive metadata, delayed transmit records, NIC calibration, transitions, fallback, the actual receiving consumer, and measured benefit. It connects those cases to the existing architecture, hardening guidance, development sequence, backlog, and two-family Windows matrix. The additions remain planned qualification work under existing CHECK owners; they introduce no hardware requirement or new acceptance-gate family.

> The documented system already has application-boundary captures, conditional numerical consumers, managed logical-source provenance, and bounded historical qualification results. Its finite capture/mapping supplier-and-receiver contract is settled, but genuine calibration and invalidation issuers, justified physical error bounds, production-positive admission, and the corresponding native qualification remain open. Winsock software timing and later hardware timing remain conditional enhancements to the required V1 provenance path. Existing routing, NTS trust, execution holds, and post-V1 ordering remain in force.

**Finding resolution:** TQ-REVIEW-01 (low) corrected two overbroad CHECK summaries:
CHECK-8 owns source identity/quorum, CHECK-14 truthful status, and CHECK-24
independent evidence review. A read-only v2→v3 delta review independently
verified patch SHA-256
`c49edc896beb8f9c4228fb9b366db10e99520dacf62085fc1b975646af808aad`,
confirmed the correction and three wording clarifications, and retained READY
with no remaining issue in that delta. This receipt is appended to that reviewed
substantive candidate; no implementation or acceptance scope changes follow.

**Documentation checks:** `git diff --check` passed; all 18 new/changed local
Markdown link occurrences resolve. The link scan also encountered one unchanged
reference-like prose line in the development plan, outside this patch; it is not
counted as a new link failure. No Cargo, SDK compilation, Windows/native/NIC
probe, benchmark or product-qualification test ran. Review and link integrity
establish documentation readiness only. The next dependent owner uses TQ-01–10
within the existing S2.1/P2 order and execution holds.

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

**Tokio selected-version refinement — 9 November 2026:** the
[Windows follow-up](../docs/supporting-reviews/windows-os-integration-risk-research.md#91-suspend-can-delay-delivery-after-an-elapsed-deadline)
traces Tokio 1.54.1/Mio 1.2.4 to the existing IOCP/QPC distinction. Mio #2006
remains open; Tokio #8344 was closed unmerged. At that reviewed checkpoint,
the engine's final elapsed-time guards rejected overdue results once polled,
and its stop/invalidation Sleeps used that same driver. An external control
update could wake it. Native resume integration was still open; the 19 November
source child below records the later observer. The lab's 30-second cap supplied
no qualified service-stop bound. These source observations retain that checkpoint.

Carry the actual resume-to-generation/control consumer into S2.1/S2.2 and
W11-W05/S3.1 as reached. When native execution is permitted, pair a healthy
exchange with a quiet pending exchange suspended across expiry and resumed
without UI/network activity. Observe deadline, notification, first owner execution,
final rejection/retirement and a subsequent valid exchange; include short-suspend
generation invalidation. A separately timed owned wake can distinguish parked
driver delay from a faulty final guard. CHECK-27 retains progress/stop, CHECK-21
reached resource observations, and existing CHECK-10/11/12 retain clock/mapping
effects. No timer-resolution workaround or native execution is admitted here.

**Observe CLI source child — 19 November 2026.** The
[resume integration checkpoint](engine-transport-pilot.md#automatic-resume-invalidation-in-the-windows-observe-cli)
connects automatic resume to the actual invocation's existing control watch.
The native platform leaf owns PowrProf registration and retirement. Both Engine
crate roots and the portable platform crate retain `forbid(unsafe_code)`.
The leaf closes the one-shot callback slot before native unregister. Delayed
callbacks can reach only permanent storage and cannot reach another invocation.
This is the TQ-07/09 delivery and control path. The
[Windows smoke](engine-transport-pilot.md#windows-resume-registration-and-close-smoke)
observed successful registration and explicit close with an inert callback.
Native transition coverage, delivered-event scheduling, calibrated mapping
and production admission remain open under their existing owners.

**Native leaf qualification owner and exit.** W11-W02's platform adapter owner
owns the `windows-resume-observation` security module and both critical files,
`resume_slot.rs` and `windows_resume.rs`. CHECK-19/24/25/27 require a current
target-specific inventory, at least 90% executable-line coverage and at least
95% mutation per critical file, with explicit ungenerated FFI scope. The Engine
metrics do not supply these leaf results. This acceptance remains open.

The same owner retains delivered-event, duplicate/delayed notification, disarm,
registration/retirement failure and actual resource observations under
CHECK-11/21/25/27. Controlled callbacks and an inert registration smoke retain
their narrower scopes. Physical S3/S4/Modern Standby cases require the owned
Windows laboratory and its recovery arrangement. The slot holds its mutex while
delivering the action; no callback or cancellation latency bound is qualified.
W11-W05/S3.1 must explicitly account for one registration attempt per process,
including after registration failure or close, before a service consumes it.

#### W11-W02 portable resume slot metrics — 19 November 2026

CHECK-19/24/25/27. On `8fc623f`, the isolated Rust 1.100.0 GNU/default
baseline ran two slot tests and reported 58/64 lines. The raw regions showed
that the pre-bind return and lock-poison refusal were unexercised.
The new controls check pre-bind inactivity, exactly-once consumption of a
refused action, lock ownership inside delivery, and capture custody on poison.

The frozen 613-input candidate `184a9daac268dbd3790062b3fd1227bff0d0f58bf02ebba11643a3b9fc11c971`
differs from `8fc623f` only in the inline slot tests. Root lock, native
implementation and feature graph are unchanged. Five GNU ordinary and five
optimized tests, the GNU visibility doctest, and GNU/MSVC Clippy passed.
The GNU per-file result is 115/126 (91.27%), including inline tests. The
report covers `resume_slot.rs`; it provides no LLVM branch counts or Windows
execution. The full GNU slot inventory produced six descriptors. One reached
execution and was caught, four failed strict lint compilation and one also
failed the `Action: Default` requirement. The viable score is 1/1, with zero
survivors or timeouts. Compiler refusals add zero behavioral catches.

Two separate controls compiled and exited 101 at their intended assertions.
Releasing the lock before delivery failed the lock-ownership observation.
Delivering after poison returned `Ok(())` when the test expected
`Err(())`. The unchanged ordinary run passed both tests. These two
controls add no credit to the generated inventory. Independent review
matched all 613 candidate hashes, the raw locations and the compiler
versus behavioral dispositions. Earlier setup refusals for tool PATH,
the reserved workspace mount and the read-only fault build, plus the
first candidate's similar-name lint failure, retain their original logs.

The Windows-source listing has eight descriptors. Its execution, target
coverage, mutation gate, registration/retirement error observations and
physical transition delivery remain open. The one viable generated slot
descriptor covers the event filter. It does not generate lock lifetime,
poison recovery or native ABI faults. Keep the earlier Engine KE-wide
source binding at its original candidate and reconcile the test-only delta
before using that current-candidate gate.

The next native child should keep the real registration and close in the
existing Engine smoke process. Use a separate fresh leaf process for the
direct callback controls and the matching MSVC LLVM profile. Observe native
error and one-shot retirement through their actual owners before claiming
the Windows file's gates.

| Evidence under `~/.local/share/nevrotick/native-leaf-metrics-20261119.S0htVns7/` | SHA-256 |
| --- | --- |
| `current/inputs.json` | `cebfef55c3d31932728f6358851bfa4f86d41b3c5d7dc9c575d1564629f72b85` |
| `current/coverage.raw.json` | `f32cd407dcd1d52af1f6aa6c1f88f1ebb357b7b8f772a794c008c092e3138907` |
| `current-mutation/mutants.out/outcomes.json` | `200e32c6c82b6c444607bf83e743f1219383a46deba2faf01efe04a5454a57c3` |
| `current-focus/unlock-before-delivery.json` | `07163707d0dd2b9dd03c46a0da894c0c5160854152d9fe04cbb9c84cc68eb4cd` |
| `current-focus/deliver-after-poison.json` | `b0c17532e5e6623f36a1a1610c38c08ff258dc697096f289f964371d18049e40` |

#### W11-W02 Windows callback and subscription observations — 19 November 2026

CHECK-19/21/24/25/27. The 613-input product
`420e87491ce60e43df06e92bcfa728d64f48640407a91903ef597e592532aca7`
adds a test in the Windows leaf and extends the existing Engine smoke.
The production code and Cargo inputs retain their original bytes. The
Engine test uses real PowrProf registration and explicit close. A second
observer receives `AlreadyUsed` before and after close. The separate leaf
process calls the production callback through the static descriptor with
events 7, 4, 18 and 4. It expects statuses 0, 31, 0 and 0, one invocation
of its inert failing action, and an empty slot on disarm.

The existing Engine runner passed all 17 MSVC build lanes. Its first native
attempt stopped before process launch at 1,472,240 KiB available RAM. A
fresh observation of the same counter showed 6,562,416 KiB. The unchanged
build then passed the expected gate controls, exact 255-name discovery,
and all 13 selected Windows cases. The two leaf builds retain the v3
flags. The LLVM build adds only `-C instrument-coverage`. GNU slot evidence
reuses the byte-identical slot tests and excludes the Windows-only additions.
MSVC leaf Clippy, both exact six-name discoveries and both six-test runs
passed. Independent review verified the image and symbol hashes, ordinary
user token, and Job drainage for each observed run.

Pinned LLVM read the profile from the instrumented six-test run. The
profile's PID and image match that child's observation. `resume_slot.rs`
reached 115/126 lines (91.27%). `windows_resume.rs` reached 22/63 (34.92%).
These include inline tests. LLVM emitted no branch counts. Registration
and close ran in the uninstrumented Engine process. Thus the Windows
adapter line gate remains open, alongside its 95% mutation gate,
native failure/retirement evidence and physical transition observations.
Use an instrumented actual subscriber and safe, typed failure controls
for the uncovered paths. Continue to keep direct callback and physical
event observations distinct.

The first leaf producer linked the ordinary test image but its reader
rejected Cargo's short `rustc` spelling. The next producer accepts both
spellings only under the admitted sandbox PATH and verifies the complete
target flag vector. Both attempts retain their original outcomes.

All eight exact generated Windows descriptors were built under the
unchanged MSVC target and strict lints. Six produced source-attributed
compiler refusals. Two linked and remain unexecuted: inversion of the
subscription guard and an empty `ResumeWatch::drop`. The actual healthy
registration case supplies an oracle for the former. For the latter,
exercise implicit Drop in a fresh process and observe disarm before
process teardown. Charge no behavioral kills to this compiler-only run.

| Evidence under `~/.local/share/nevrotick/engine-resume-native-current-20261119.VVojgKic/` | SHA-256 |
| --- | --- |
| `build-01/verification.json` | `bf7777f0197086e67dd70445e7c7eb9830e36f553a30a8733797c4fc633802c1` |
| `native-01/verification.json` | `a7c74e241b3b689cb53cc0ebd6952e1a33755cb803cb02671f295222be8dad5b` |
| `native-02/verification.json` | `c413acc8ef87d27f415d20e51f93244f8970265c2f662287023bd4d037353f8f` |
| `leaf-build-02/verification.json` | `bb2621dc7fc40ac939656c9eacf81ec54ca3eb7637051d828fd32c66929401e2` |
| `leaf-launch-build-01/verification.json` | `01f7ea15351a23ab91e05ca35b1a9faf8a17a7f3efd1273d43c566bf57949ac0` |
| `leaf-native-01/verification.json` | `5cef33a8285d44f06784a5d96316ff9b9f06bb70020f24b7bf3ef02db36a0b31` |
| `leaf-coverage-01/verification.json` | `14d049dcb02e57758cee7bc130e67b36169004c14356ebf444a211e058642756` |
| `leaf-active-target/verification.json` | `b8196396012575758e99ca02696597a823163c7982e04b0ef27213ed7f4779a9` |

#### W11-W02 implicit Drop and viable mutation results — 19 November 2026

CHECK-21/24/25/27. The 614-input candidate
`0997b2b458ea8c73c02d9dbd962297abd8d6a3295baf33289a75944111101c43`
adds the Windows-only `subscription_lifecycle` integration test. Its own
process owns the sole registration. The native callback records delivery
before destroying its capture. When the watch leaves scope, the test
requires retirement of an undelivered capture and one remaining reference
before harness teardown. An external notification observed during that
interval reports interruption and supplies no mutation credit.

The healthy ordinary and instrumented MSVC images each ran the one exact
test successfully. GNU and MSVC Clippy passed for the integration target.
The original generated guard inversion failed with
`AlreadyUsed` at initial registration. The original empty Drop failed at
the post-drop armed-capture assertion. Both had native exit 101 and a
drained Job. Their complete source maps differ from the healthy fixture
only in the selected mutation. The earlier library build refusals retain
their six source-attributed dispositions since the new integration target
does not enter that library test build. The selected Windows descriptor
inventory has **2 caught of 2 viable**, six compiler refusals, and zero
survivors, timeouts or exclusions. The generated per-file gate passes for
that inventory. A dropped capture is observable even though Drop does
not return the status of native unregistration.

Pinned LLVM merged the callback and healthy lifecycle profiles with
their exact images. The earlier callback unit profile covers unchanged
library source and target inputs. Only the separate integration file
entered the product map. `windows_resume.rs` reached **49/63 lines
(77.78%)**, up from 22/63 in the callback run. `resume_slot.rs` stays
115/126. LLVM emitted no branch counts. The Windows adapter line gate is
still open. Explicit close, duplicate admission and the handle-already-taken
return need instrumented execution. The registration failure, missing-handle
and retirement-status paths need safe typed fault controls. Preserve the
separate physical transition and teardown work.

| Evidence under `~/.local/share/nevrotick/engine-resume-native-current-20261119.VVojgKic/` | SHA-256 |
| --- | --- |
| `leaf-lifecycle-build-01/verification.json` | `ab71bb745fdb0777fe58b7f112585aff74d673a6bb77616855c4df5612a39785` |
| `drop-native-01/verification.json` | `79e321e3d030ce38db9be01a3742e14d02f8dcab33a911e8a7e64715e20c463d` |
| `leaf-lifecycle-coverage-01/verification.json` | `1521c258c434ace0201b28eb3f585e38017a787b5373a96e896650c68d0e3a19` |
| `leaf-lifecycle-lint-01/verification.json` | `c8d83c11591c7ad02da06b82b9b8d9ccb80179ae7f0a356a71bbcf4f110d1d74` |

#### W11-W02 instrumented close and typed failure handling — 19 November 2026

CHECK-19/21/24/25/27. On the 614-input `0997b2b4…` predecessor, the exact
Engine native close test passed with 254 cases filtered. Merging its
instrumented data with the callback and implicit Drop data raised
`windows_resume.rs` to 53/63 lines. The first close cross-link refused
the stable tool identity comparison before compilation. Its replacement
compares the complete serialized identity. The first three-profile
readout exceeded the output limit when exporting Engine instantiations.
The scoped export keeps both owner files and omits per-function output.

The current 615-input candidate
`59fca1490b7b999352275a8d9e72a15a5d76c18a27d44831b8fd0dbf14a4cfc2`
keeps the exact PowrProf ABI and the public subscription API. Two private
helpers own completion and retirement. A nonzero status takes precedence
over the handle output. A missing handle after a success status also
disarms the already consumed slot. Retirement takes the handle, disarms
delivery, holds the capture through the unregister attempt, and preserves
the exact status without retry. Local fixture tokens exercise those paths
without crossing the FFI boundary.

GNU and MSVC leaf Clippy and all three instrumented MSVC links passed.
The Windows runs passed all nine leaf tests, the one real Engine close
test, and the one implicit Drop test in separate, drained Jobs. The
fresh merge uses only these three profiles. The unchanged file-summary
line gate passes at **115/126 (91.27%)** for `resume_slot.rs` and
**78/79 (98.73%)** for `windows_resume.rs`. LLVM reported no branch
counts. The first readout incorrectly required its source annotation and
file-summary denominators to match. The fixed readout preserves both
observations and gates on the file summary. The annotations show 28/28 and
60/60 production lines respectively. They have a different denominator.
Forced statuses qualify Rust-side handling. Actual PowrProf fault delivery
and physical suspend/resume remain open.

The fresh ten-descriptor inventory covers the two new helpers and the
previous adapter responsibilities. All ten were compiled under ordinary
MSVC settings. Eight produced source-attributed compiler refusals. Both
ordinary healthy images ran their exact selected tests successfully. The
viable guard inversion returned `AlreadyUsed` at initial subscription,
and the empty Drop left the undelivered capture armed. Each failed at the
intended assertion with native exit 101 in a drained Job. The selected
current adapter mutation gate passes at **2/2 viable**, with no survivors,
timeouts or exclusions. Compiler refusals add zero behavioral catches.
The slot's production code and GNU test inputs are byte-identical to the
earlier scoped slot mutation run.

| Evidence under `~/.local/share/nevrotick/engine-resume-native-current-20261119.VVojgKic/` | SHA-256 |
| --- | --- |
| `close-native-01/verification.json` | `0eba966cf4684ba2a798f5b2c5c7eb0bba37c0fcd669ea415bbd6b5b448141ba` |
| `leaf-explicit-close-coverage-02/verification.json` | `4b53d9672c44439eadd4d30a87ab2c5a6c99df8559e22c72f5b584c6692b905b` |
| `resume-reducer-build-01/verification.json` | `1aa9ceb7b323fc23ff4ae191ba01cad0911436db0755422daabf748deb3fe2ce` |
| `resume-reducer-native-01/verification.json` | `5fbc3c29c5f5f93598123f9fc17a536be47ae538975bd1a7cd2c23101efe4de2` |
| `resume-reducer-coverage-02/verification.json` | `2c103940b9afdac5e6864a95d18d08def2187be8f8b446fc601414ed9937b4cd` |
| `resume-reducer-catalogue-01/verification.json` | `465f7cfb4554276306bf75bcf27f26ff4cb1427cbbe83317371283319ad93123` |
| `resume-reducer-active-build-01/verification.json` | `28623096c42f42623f1204b8082684fd3325a0891cd6bc440463591952d748c1` |
| `resume-reducer-active-native-01/verification.json` | `ce3d06db68c5f98dd84e8cb2ad42271ef7b8b0625e4c624b00c091dfd100d697` |

#### W11-W02 scoped metrics on the integrated source

CHECK-19/21/24/25/27. Between `59fca149…` and the integrated
615-input `dde78e2c…` candidate, the only changed inputs are the UI
dependency policy and two dependency-checker scripts. The current graph
replay confirms the same dependency-tree outputs in all four rows. Both
registered native sources, their test consumers and all 15 supplemental
build inputs match the retained receipts. The lock, compiler inventory
and vendor identity also match.

The selected per-file evidence therefore carries forward with its
recorded scope. The MSVC executable-line gates pass at 115/126 for the
slot and 78/79 for the adapter. The GNU slot inventory caught its sole
viable variant. The MSVC adapter inventory caught both viable variants.
The receipts retain the five GNU and eight MSVC compiler refusals and
the original source identities. LLVM emitted no branch counts. The
separate slot lock and poison controls add zero generated kills.

The readback is `native-resume-leaf-source-reconciliation-01.json`
under `~/.local/share/nevrotick/crypto-cng-protected-runtime-20261119`,
SHA-256 `02c3f01fa00accc19fe50a1a0ef2a4f5168a227ba11bd2cdb6d75ec246f37221`.
No new native workload ran. Physical delivery, callback latency, actual
PowrProf fault delivery and long-lived service reuse remain open.

The [suspend continuation](engine-transport-pilot.md#suspend-and-automatic-resume-invalidation)
uses the same invocation guard on a delivered pre-suspend notification. The
first qualifying event consumes the action. The focused controls and Windows
CLI cross-link pass. The actual transition and scheduling checks below remain
open.

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

**Service privileges, 9 November 2026:** consume
[architecture §4.1.1](../docs/windows-time-architecture.md#411-service-privilege-profiles)
and [the bounded continuation in §4.38](#438-service-privilege-profile-handoff--9-november-2026)
before dependent token/admission or package integration. Exact role sets are now
selected; full-profile observation/enforcement, supported provisioning and native
compatibility remain open. This refines the current service task, not §7's
executable Core queue or the Windows/fuzz/storage/UI/post-V1-proof holds.

**Service metadata refinement, 7 November 2026:** consume [metadata §12.2](../docs/plans/windows-package-metadata.md#122-scm-labels-localization-and-installed-propagation)
with W11-W10 before dependent service authoring/provisioning. Source preparation
is ready; native work remains held. Freeze stable technical identity separately
from display/purpose text and intended/next-run/runtime observations. Qualify
actual selected MSIX propagation rather than assuming either direct XML fields
or a post-install setter. Pair correct labels and legitimate update/rollback
with missing/wrong MUI resource, denied read, null/empty update distinctions and
stale package paths. Keep stored references and resolved locale/text distinct.
Query rights confer no change authority; no privilege/account/startup/recovery
change follows from a request for richer metadata. Reuse the current lifecycle,
CHECK-18/23/27 and W11-W10 readback owners; the final provisioning bridge remains open.

**Owner/stage:** platform/security/lifecycle; P0/P3, P4 cleanup and P6 complete MSIX.
**Prerequisites:** existing token/object, mitigation, mandatory-state and
[installation research](../docs/plans/windows11-integration.md#4-make-installation-and-service-state-reflect-actual-windows-behavior).

**7 November service-management refinement:** apply the
[source review](../docs/plans/windows11-integration.md#service-management-hardening--7-november-2026)
and [canonical route policy](../docs/windows-time-architecture.md#421-service-management-admission-and-administrative-recovery)
with W11-W10/S0.2/S0.3. Before choosing descriptors/peer verifier, record each operation's
normal route, effective principal, required installed component, object/right
and deliberate administrative/recovery exception. Public redacted
status and argument-free refresh retain their existing policy. Product commands
requiring application identity cannot use SCM custom-control delivery as proof
of that identity. Remote administrative SCM and local-only pipes need separate
dispositions; no global host-policy weakening follows.

Next source-only child: freeze matrix and valid/hostile controls; specify
still-unqualified package-aware peer binding and independent recovery entry point. Later native evidence must cover same-principal/wrong-app,
copied or unapproved component, identity acquisition/restart races, injected or
delegated authority, prior handles across DACL repair, final moved/replaced-object
security and intended elevated recovery without a healthy UI/service. Existing
hardening §9.9/§9.11 and CHECK-1/2/13/15/18/23/27 retain acceptance. Descriptor, signature or package-family match proves no application exclusivity. Source
research grants no native execution, new service identity, topology or dependency;
current priority and holds remain unchanged.

**7 November additional service practices:** the
[reviewed follow-up](../docs/plans/windows11-integration-follow-up.md#additional-service-practices--7-november-2026)
and dedicated [Windows service skill](../.agents/skills/implementation/implement-windows-service/SKILL.md)
refine this existing assignment; they establish no runtime defect or native pass.
**Disposition:** source preparation ready when the existing S0.2/S0.3/P3/P4/P6
owner reaches it; native execution held. Before dependent implementation, carry
the applicable rows into that owner's bounded card:

| Existing owner / checks | Paired observation and next action |
| --- | --- |
| Startup/configuration; CHECK-1/2/13/15/27 | Freeze process arguments, transient SCM arguments, native metadata/decoding and protected durable policy separately. Pair ordinary installed start/recovery with conflicting, malformed/excessive options; no role/root/endpoint/maintenance/clock-authority override or dependence on prior transient input. |
| Broker/lifecycle/mandatory state; CHECK-1/5/12/13/15/27 | Freeze the existing mutex's namespace, creation/open rights, owner/DACL, lifetime, native-thread owner and task serialization. Pair trusted creation/open with precreation, wrong type, denial and cross-session cases; test both abandonment with a surviving handle and fresh recreation after last-handle loss. Reconcile current controller, journal, generation and consumed allowance before effects; retain any reservation-availability limit. |
| Per-role startup with W11-W08/ST-W07; CHECK-16/17/21/27 | Record effective process/thread error policy, owner and phase. Pair ordinary start/useful diagnostics with selected critical-error failures through startup/work/stop; no modal wait. Keep WER invocation, UI, queue/upload and collector/privacy policy distinct, preserving administrator policy. |
| Startup/configuration with SEC-A02; CHECK-15/17/21/27 | Inventory actual environment/profile/dependency inputs. Pair correct account/no-login startup and forced spill with missing, inaccessible, overridden or redirected paths; verify protected authority and actual live-file custody rather than a temp-path string. |
| Actual dependency owner; CHECK-13/21/27 | Only if reached, freeze complete policy/installer/SCM/native-provider waits. Pair healthy ordering with delayed/held dependency; bounded progress/failure and later recovery preserve truthful readiness and no clock effects before required policy admission. |

No namespace API, account/wrapper, temp API, watchdog, restart constants or new
acceptance IDs are adopted. Current Core/N3 work remains next; this source-only
integration changes no execution hold or formal post-V1 deferral.

**Documentation/skill checks, 7 November 2026:** the 66-skill pack check and new
skill validator pass; 1,530 local-link/heading checks across 15 affected Markdown
files and scoped whitespace checks pass. Three independent source-only scenario
exercises covered startup/coordination, errors/native-job cancellation and
maintenance/recovery; the exercised skill SHA-256 is
`029ca3f356255beba19d4c2aa219e90868a6a0b53897d705452b79af9b0b4d34`.
A separate fresh-context source review accepted the integrated documentation
with no material findings, including the process/thread API documentation
discrepancy. These are instruction/document checks, not native service tests or
product qualification. Concurrent Core/N3 changes retain their own evidence.

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
If the selected observer enumerates services,
[EnumServicesStatusExW](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-enumservicesstatusexw)
can silently omit an installed service when query-status access is denied. An
absent row alone establishes neither service absence nor the cause of omission;
preserve unknown/unavailable observation separately from independently
established absence. Under CHECK-13/14/23/27, compare the same known service with
query access allowed and denied, plus a genuinely absent fixed-target control
and useful reconnection. No false removal/readiness, automatic elevation or
widened rights follows.
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

The [production event logger continuation](#43511-production-event-logger-plan--17-november-2026)
owns the required operational event pipeline and its staged integration. Optional
ETW tracing/fault experiments below do not replace that pipeline or lift its
native/package prerequisites.

**Diagnostic metadata refinement, 7 November 2026:** [metadata §12.3](../docs/plans/windows-package-metadata.md#123-artifact-and-diagnostic-projections)
and [catalogue C](../docs/supporting-reviews/windows-service-metadata-catalogue.md#appendix-c-diagnostics-and-performance-counters)
refine this assignment with W11-W10/P6. Before dependent resource authoring,
freeze stable decoding identity, event ID/version/typed payload and actual resource
paths, message/help/locale mapping and operation correlation. Preserve XML/native
domain discrepancies for admitted compiler/readback qualification. Pair a known
owned rendered event with missing/wrong resource, schema/version and locale cases;
inspect installed publisher metadata separately from current channel policy.
Privacy, finite retention/sink failure and ST-W07 sensitive-output cases remain
required. TraceLogging alone proves no durable Event Log catalogue; PerfLib stays
optional. Source/schema inventory is complete within its declared scopes; product
values, compilation, registration/rendering and native cases remain open/held
under existing CHECK-17/21/27 and package readback gates.

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

With W11-W05, apply the [7 November service-error refinement](../docs/plans/windows11-integration-follow-up.md#effective-noninteractive-error-behavior):
useful diagnostics must survive the selected unattended failure paths. Effective
error handling, WER invocation, UI and queue/upload choices are separate from
collector confidentiality; no-UI alone closes no privacy case. Native cases
retain this assignment's phase and hold.

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
**Status — 17 November 2026:** production incident notifications remain planned
and deferred. UIA announcements and local app/toast delivery are separate
qualification surfaces. Technical D2/D3 readiness does not supersede the
[rich-toast first-release deferral](../docs/windows-time-features.md#10-defer-from-the-basic-release);
revisit delivery at its existing reopening trigger. This documentation handoff
changes no release scope, Core/Engine priority or Windows/fuzz/storage/UI hold.

**Prerequisites:** the admitted Slint/backend/renderer and selected accessibility
exposure from the [UI migration chain](ui-m0.md#slint-migration-handoff--5-november-2026),
actual authenticated current-generation service snapshots, restored D2/D3
contracts and [native UI research](../docs/plans/windows11-integration.md#8-present-live-state-through-native-ui-facilities).

**Current source boundary.** The Slint native workflow installs an explicit
sample-test callback and `WindowsToastPort` using `Windows.UI.Notifications`.
`PreparedTestNotice` rejects non-test contexts with `LiveIncidentOwnerRequired`;
the port currently requires observed package identity and rejects its unqualified
desktop-installation mode. This supplies useful source, not a real incident
producer, general registration path or native display/activation/cleanup pass.
Retain the [sample-attempt qualification](slint-gui-handoff.md#bound-notification-attempt-hardening--14-november-2026)
within its exact scope; neither that receipt nor HTML preview results close this task.

**Work:** qualify the actual Slint accessibility/Windows bridge without assuming
inherited Win32-control semantics. Evaluate `UiaRaiseNotificationEvent` and supported processing modes for
meaningful state changes; ordinary ticking status must not create important
announcement spam. The existing WinRT transport is the inspected source starting
point; Windows App SDK remains an alternative candidate with deployment/graph
costs, not an adopted replacement. Windows unpackaged support does not qualify
this adapter's unimplemented registration path. Keep presentation unelevated and
revalidate current generation/authority at every notification action.
Any selected UI COM class also uses W11-W04's class-specific apartment,
security-initialization and teardown contract; this does not restore D2/D3 early.

**Next bounded stages, when their prerequisites are admitted:**

1. **Freeze the real incident contract.** UI/diagnostics and the actual
   service/clock/security decision owners bind the
   [W11-W08 event catalogue](#43511-production-event-logger-plan--17-november-2026)
   to an eligibility table: event ID/version and outcome, operational importance,
   incident identity/scope/generation, authoritative current observation,
   persistence/actionability rule, permitted summary and fixed navigation target.
   Include actual Critical and High cases with healthy/transient/expected-operation
   neighbors; retain absent producers as open rather than inventing live incidents.
   Technical ERROR/WARN, translated text and historical replay do not classify an
   alert. Name the startup/refusal observation owner: a service that failed to
   start cannot be presumed to supply its normal snapshot. UI disconnection or a
   missing SCM row establishes unavailable observation until stronger evidence
   establishes failure. Preserve W11-W05's observation and authority boundaries.
2. **Adopt the existing design into a reached portable path.** Use the
   [v0.4 trigger/lifecycle design](../docs/nevrotick-design-v0.4/review/status-notifications/WINDOWS-NOTIFICATIONS-RESEARCH.md#proposed-trigger-policy-for-the-existing-statuses)
   and its [completed-design refinements](../docs/nevrotick-design-v0.4/review/status-notifications/IMPLEMENTATION.md#notification-behavior).
   Freeze finite incident/queue/rate/cardinality bounds, elapsed-time persistence
   and action lifetimes, user/session ownership, reconnect/restart reconciliation
   and cleanup before the dependent implementation. The HTML ledger's eight
   entries are a sample allocation, not a product limit. Bind the admitted real
   producer/secured observation through the incident owner to an inspectable
   notification request and independent availability/outcome projection in the
   actual UI composition; a test-only constructor or isolated helper is insufficient.
   Stop this source slice at its declared portable exit; native display remains open.
3. **Qualify the selected Windows path separately.** With actual package/app
   identity, intended user token/session, current settings and admitted target
   environment, connect the prepared incident to the native adapter and observe
   presentation, safe activation, owned retirement and callback/worker cleanup.
   Record dependency/activation/registration needs at their existing UI/package
   owners; production text retains localization/privacy approval. Observe actual
   accessibility and resource behavior; cross-builds, source or API acceptance
   alone establish no native or release pass.

**Incident and delivery rules.** Preserve problem notifications On and tray/sign-in
Off as their existing distinct preferences; this handoff selects no new setting
or recovery-success notification. First eligible occurrence and material
escalation are reviewed incident transitions. Repetition, navigation, dismissal,
cancelled commands and preference changes neither resolve nor renumber an episode.
Expected acquisition/resume, intentional Stop/Disable, old history and isolated
trouble with healthy alternatives stay quiet under the adopted policy; soft
failures require a current authoritative persistent/actionable observation.
The explicit test keeps separate identity and cannot consume or resolve a problem.

Distinguish refusal before submission, actual attempt, OS acceptance/uncertainty
and independently observed banner/Notification Center behavior. A still-current
incident never submitted because presentation was unavailable may become eligible
when readiness returns, within its frozen bounds. An attempted/accepted/uncertain
submission cannot be retried merely because visibility is unknown or a preference
changes; do not port the HTML's simulated "unshown" state as a native receipt.
Respect user/organization suppression even for Critical events; keep underlying
status and required recording independent. No current presenter means no promise
of a newly detected immediate alert. Reconnect reconciles current incidents,
not an unbounded replay of old severe records.

Use bounded public-safe text suitable for lock-screen exposure and approved
user/session scope. Activation opens current Status/Diagnostics/review context;
validate the fixed route, incident generation and current state before navigation.
It performs no Apply/Retry/Start/Stop or clock effect and grants no administrator
authority. Fresh resolution retires only owned obsolete notices; native cleanup
failure remains explicit, and retained stale activation is still safely checked.

**Windows-specific admission.** [Toast priority](https://learn.microsoft.com/en-us/uwp/api/windows.ui.notifications.toastnotificationpriority)
is a presentation/power hint, not NevroTick importance or a suppression bypass.
The [urgent scenario](https://learn.microsoft.com/en-us/windows/apps/develop/notifications/app-notifications/app-notifications-content#scenarios)
is separately user-controlled; if selected, qualify availability through the
actual bindings and target build, with ordinary presentation/status fallback.
Overview/API build wording supplies no native qualification; do not use
Alarm/IncomingCall to evade suppression. Native
[expiration metadata](https://learn.microsoft.com/en-us/uwp/api/windows.ui.notifications.toastnotification.expirationtime)
uses a date/time; it cannot replace current-generation/elapsed validity checks
through clock correction, suspend or restart. Selected tag/group/history APIs
must operate only on owned notices; Windows history is not the incident authority.

| Acceptance pair at the reached stage | Required independent observation |
| --- | --- |
| Real Critical and High cases beside healthy/transient/expected-operation cases | Actual producer/observation reaches the incident owner and bounded request consumer; quiet neighbors stay quiet. Missing real producers remain incomplete, even when labelled synthetic controls pass. |
| New episode, repeat, escalation, dismissal, cancelled command and fresh resolution | Stable identity prevents duplicate attempts and false recovery; escalation/new current incident remains eligible under policy. Resolution retires only the relevant owned notice. |
| Preference Off, Windows blocked/unknown/suppressed, no presenter, then useful readiness | Status/required records remain; a never-attempted current incident can be reconsidered once, while prior uncertain/accepted attempts are not blindly replayed. Test identity stays separate. |
| Reconnect/restart, stale generation, historical severe event and unknown service observation | Current evidence determines eligibility; no invented service failure, historical problem resurgence or false healthy state. State and replay work stay bounded. |
| Current activation beside forged/expired/resolved/cross-user input, including clock changes | Only fixed current navigation succeeds; no protected operation or leaked detail. Native expiration or a successful Show cannot grant action authority. |
| Permitted Windows presentation beside suppression, unsupported optional urgency and native failure | Submission, acceptance/uncertainty and observed display remain distinct. Useful fallback/status survives; native API success is not evidence of user reading. |
| Finite flood, shutdown/callback race, privacy and actual keyboard/Narrator controls | Independent resource/lifetime observations show bounded work and completed owned cleanup; safe text and accessible navigation remain usable. No fixture/API-return substitution for actual Windows behavior. |

**Controls/exit:** meaningful current loss/recovery or optional-cache status is
announced accurately; stale events, rapid repetition and expired actions cannot
mislead or authorize protected effects. Observe actual Narrator/keyboard behavior
on the selected build, including a compatible older processing mode where
required. The build-26100 enum choice is not the base API floor or accessibility
proof. **Mapping:** CHECK-14/17/21/22/25/27 at their reached scopes; CHECK-23/26
when deployment/graph boundaries are reached; ST-T01/14 for storage status/privacy.
UI-M0 D1/D2/D3 remain Deferred; this task activates none of them.

**Documentation audit and independent review — completed 17 November 2026.**
Against baseline `c3997241c8cc90ed6cb9b0c91640a2cdfc90e070`,
`notification_plan_gaps` completed NP-01–NP-03 from current source/design/task
evidence; `notification_windows_contract` completed NW-01–NW-08 using the same
boundary and Windows Learn MCP sources. Both reports were complete before the
integrated review, including NW-05's correction from the earlier proposed
recovery-notification option to the completed design's explicit exclusion.
The two researchers and `notification_plan_judge` used fresh delegated contexts
(`fork_turns: none`). These identifiers track planning findings, not new gates.

| Findings | Disposition in this handoff |
| --- | --- |
| NP-01 / NW-01 | Verified documentary correction: bind the catalogue and actual observation/incident owners; retain unavailable startup evidence and absent producers honestly. |
| NP-02 / NW-03/05/07/08 | Carry existing completed-design rules into real-input and native acceptance; preserve submission uncertainty, session/privacy/action boundaries and no recovery-success feature. |
| NP-03 / NW-02 | Verified documentary correction: name the actual sample-only WinRT path and route the future continuation consistently through architecture, feature/hardening plans, P5/backlog and GUI handoff, with release/stage deferrals intact. |
| NW-04/06 | Conditional native details retained: importance versus presentation priority/Urgent, and absolute native expiry versus current-generation/elapsed validity. No API, SDK, duration or new build floor selected. |

`notification_plan_judge` inspected all seven Markdown changes, applicable
canonical/phase/design contracts, both completed reports, the actual sample
producer/consumer source and bounded Microsoft evidence at staged semantic tree
`fb757ff9978c3b0677c05a495ffa6299d8d3a56e`; the index matched that tree with no
unstaged changes. Verdict: **ready for the documentation/planning handoff only;
no material unresolved finding**. This attributed receipt was added afterward.
The coordinator checked all 13 added local links/anchors and three Microsoft
source URL syntaxes, preserved existing heading order and the byte-identical
first portable logger steps/exit, and passed `git diff --cached --check`.
No product test, build, scanner or native operation ran for this amendment.

Reviewer comprehension, retained verbatim:

> This revision makes W11-W09 the explicit continuation for turning real, current Critical/High problems into Windows notifications. It connects the event catalogue's decision owners to a separate incident eligibility contract, carries the completed design's suppression and lifecycle rules into production acceptance, and separates a reached portable request from actual Windows presentation. It updates the existing architecture, hardening and UI/P5 routing without selecting a new dependency, bringing rich toasts into the basic release or starting implementation.
>
> The current system has an HTML notification design, fixture UI state and a native `Windows.UI.Notifications` adapter for explicit sample tests. The inspected constructor rejects non-test contexts, and the Slint callback reaches that sample-only path. Actual submission consumes an attempt even when acceptance is uncertain; a successful API result is not a display/read receipt. There is no inspected real incident producer connected to this adapter, while secured production observations, native qualification, the relevant D2/D3 work and the rich-toast release continuation remain open or deferred.

### W11-W10 Re-enter conditional mechanisms only for a concrete need

**Pre-V1 WinGet assignment, 7 November 2026:** [OD-17](../docs/nevrotick-owner-decisions.md#21-od-17--winget-capability-before-v1)
adds [WGT-0](../ROADMAP.md#wgt-0--winget-distribution-capability-before-v1) to the
existing P6/P7 delivery work. Use [the plan](../docs/plans/winget-distribution.md)
and [§4.36](#436-winget-distribution-before-v1--7-november-2026) for the bounded
source/metadata, native lifecycle and release-operation sequence. Capability is
due before V1; external public-source availability is a separate observed result.
Direct complete MSIX under §4.39, current Core priority and execution holds remain.

**Adopted metadata delivery controls, 7 November 2026:** David authorized the
reviewed website-pipeline techniques under [metadata §13.5](../docs/plans/windows-package-metadata.md#135-adopted-metadata-delivery-controls--7-november-2026).
This extends the existing source-tool assignment, without importing website
code/data, creating another authority or admitting a cross-project dependency.
The metadata owner adds strict ORCID/ISNI check-character validation and
source-derived per-component/field/artifact obligations with independent parsed
source-output checks. Keep pending values empty, public/private destinations
explicit, known-valid controls and stale-tree rejection. Field-disposition counts
remain distinct from actual delivery and native qualification.

| Disposition and existing owner | Next action and independent exit |
| --- | --- |
| Completed bounded source-tool controls, MD-P1–MD-P3 metadata owner | Checksum and semantic delivery implementation passed the source receipt below. Preserve direct-model/real-CLI healthy controls and rejection of wrong check digits, missing/extra/duplicate bindings, same-count wrong values/owners and unintended public fields. This source result does not complete the dependent build/native rows. |
| Pending real consumer, MD-P4 build/component owner | Wire the frozen generation into actual resource consumers; independently read linked resources. Run cold → consumed-input change → warm → clean control, check rerun rules and actual selected compiler/SDK/target/profile identity. Preserve intentional per-component/version/licence differences. |
| Pending dependent package/native work, MD-P4–MD-P5 W11-W05/W11-W08/W11-W10 | Reconcile legacy Store field applicability without inventing identities or weakening schema validation. Check namespace references/collisions when real identities exist; exact package/export allowlists exclude internal authoring and private values. Retain accidental-copy/private-sentinel controls, actual built/signed/distributed/installed identities and independent resource readback. Signing remains later work; existing native prerequisites and execution holds remain. |

**Source-tool finalization, 7 November 2026:** David authorized closing the
subsequent audit's admission/observer defects and regression gaps under
[metadata §13.6](../docs/plans/windows-package-metadata.md#136-source-tool-finalization-and-failure-outcomes--7-november-2026).
The current portable checkpoint includes strict email and byte/encoding XML
admission, decoded/Store/service/resource-reference boundaries, independent
delivery bindings and literal properties, in-flight source/code/path identity,
and precommit versus publication-uncertain recovery. Preserve the audit's useful
healthy controls and require the formerly surviving identity-comparison fault
to fail through actual publication behavior. The historical adoption receipt
below does not establish completion of these later checks.

**Finalization receipt, 7 November 2026:** the dirty working-tree source candidate
at `df6e28a4641f2f046373a9c5640aa60736716a22` passed **227 tests in 95.385 seconds,
with no failures, errors or skips**, using Python 3.12.3 on Linux/WSL. This adds
44 permanent cases: 21 admission-boundary, 12 lifecycle, six bounded property
methods and five additional delivery checks. Independent expected bindings,
33 joint single-line values, six multiline product values and real-CLI examples
retain accepted neighboring behavior and per-role licence/privacy distinctions.

The XML reader now checks the actual UTF-8 byte/declaration form. Email syntax,
decoded/field limits and in-flight source/code/output identity have permanent
controls. Ordered file/containing-directory/staging flushes precede rename;
failed final parent flush preserves the committed tree and returns the distinct
typed/API and redacted CLI exit-3 outcome. Readback confirms visible contents,
not the lost durability acknowledgement. Release-mode source validation still
refuses the unfinished release candidate.

Four named isolated guard variants now fail through meaningful operation/result
assertions: final generation-code comparison, final check-code comparison,
parent-path refusal and replaced-check-root binding. The comparison controls
retain their observation call, so failures do not merely indicate a missing
test hook. This closes the audit's formerly surviving guard counterexample;
the bounded experiment establishes no general mutation percentage.

Two independent reviews accepted the source implementation, test oracles and
instructions. Ten independent API/CLI healthy/fault controls passed, including
postcommit uncertainty and preserved bytes. The coordinator also validated,
generated and checked the actual source and independently read the 15-file tree:
14 payload contracts, 160 selected source bindings and eight code-module hashes.
The 731 field dispositions and 261 unresolved values retain their prior scope;
the authored product record was not changed. First RED and corrected fixture
attempts remain in the evidence alongside final GREEN results.

Exact source/test/input identities, raw logs and output hashes are retained under
`/tmp/nevrotick-metadata-finalization-20261107-ln3poplx/`; the independent behavior
controls are under `/tmp/nevrotick-metadata-independent-20261107-o4wfvxgx/`.
No Cargo/resource compilation, native Windows, service/clock, package, signing,
Store, fuzz or formal operation occurred. Existing MD-P4–MD-P5 consumers,
native/package owners and all execution holds remain.

**Full portable metadata qualification, 7 November 2026 — in progress:** David
requested the remaining quantitative and static checks after the source-tool
audit. This extends the same MD-P1–MD-P3/W11-W10 evidence chain, with
CHECK-18/20/23/26/27; it does not enter MD-P4–MD-P5 or lift an execution hold.
The reviewed scope is declared in
[the metadata qualification manifest](../metadata/qualification/manifest.json):
all eight execution-module identities, five security-module line denominators
and six critical-file mutation denominators, including the field registry.
Package markers have no invented nonempty denominator. Registry and pinned
Unicode data require their actual structural controls and reconciliation with
the generated fault inventory.

The coordinator owns the integrated candidate, manifest, commands and final
readback. Separate workers own strict source/interface typing, subprocess-aware
coverage and isolated per-file mutation. Required exits are enforced static
controls and a healthy consumer, at least 90% executable-line coverage in each
selected security module with separate branch evidence, at least 95% adjusted
viable mutation score in each critical file with complete outcome accounting
and survivor review, and actual input-to-output acceptance. Coverage must reach
the isolated CLI interpreters. Import, setup or observer-hook failures are not
behavioral mutation kills; missing, timed-out or empty required rows remain
incomplete. Tool versions/configuration/source identities, first failures,
exclusions, candidate changes and consumed time are retained under
`/tmp/nevrotick-metadata-qualification-20261107-zu2t_iqg/`. This paragraph records
the authorized work and its exits, not a completed qualification result.

**Resource stop, 7 November 2026:** David asked to stop resource-constrained
qualification and resume when he starts it again. The filesystem was 96% full
with concurrent load; this is not a memory-exhaustion claim. Strict checking
passed the eight production modules, legal consumer and five negative controls.
Frozen corpus v4 passed 231 tests and corrected source-bound coverage readback
passed all five 90% security-module line thresholds, observing all 275 CLI
children. The default mutation inventory has 8,890 faults; its full execution
and per-file 95% qualification remain incomplete.

The expanded live corpus has a genuine healthy-source RED: seven consecutive
quotes in valid multiline TOML can conceal the seventeenth lexical container
from preflight, reaching the parser before the decoded-depth guard refuses it.
Fix this early-budget defect before claiming completion; preserve its independent
regression rather than removing or skipping it. Current added tests and raw
mutation results have not received a complete integrated final qualification.
Use [the restart checkpoint](../metadata/qualification/CHECKPOINT.md) for exact
evidence paths, completed metrics, remaining fault reviews and source-bound
invalidation after the repair. Source/input snapshots and first failures remain
under the existing evidence root above. Other agents' staged and working-tree
changes are preserved. This stop supplies no MD-P4–MD-P5, native, formal or
release pass and lifts no execution hold.

**Hosted checkpoint reconciliation, 8 November 2026:** signed coordinator
checkpoint `e76863fc2b5acc393d317fd93857213f410fe609` retains its selected
Core/engine/wire Linux results, but has no whole-product hosted pass.
[Repository security run 42474671456](https://github.com/DavidOsipov/NevroTick/actions/runs/42474671456)
passed its secret scan and failed metadata admission: the 244-test corpus reached
the two seven-quote preflight subtests above. Both source and regression-test Git
blobs are unchanged from parent `1237c072573be36805efe8569ff4e5a652886bce`.
This is the existing stopped metadata defect, not a new result or a reason to
weaken the guard. Its repair and full qualification remain paused under the
owner's resource-stop instruction; this reconciliation restarts neither.
[Portable run 42474671434](https://github.com/DavidOsipov/NevroTick/actions/runs/42474671434)
stopped before compilation on two stale six-member fixture expectations after
`windows-platform` became the seventh member. A separate source-only follow-up
pins all seven member identities and nine ordered tree scopes per target,
preserving the AccessKit exclusion and admission controls; 168 focused Python
checks pass. It does not alter dependencies, compiler policy or metadata code.
The initial main push reported a required-check bypass; later source fixes must
pass the actual required checks before promotion. The cloud child below may
prepare its bounded contract from the published coordinator baseline, while
reporting its own unavailable/failed checks and preserving all current holds.

**Owner-directed preflight repair, 8 November 2026 — source repair complete:** David
explicitly requested fixing the metadata defect after the published fixture
correction. This reopens the finite source repair and its affected verification
under MD-P1–MD-P3/W11-W10, CHECK-18/20/23/26/27. The original resource stop and
unfinished 8,890-variant campaign remain historical; this instruction does not
relabel their outcomes or automatically resume that full campaign. The prior
`/tmp/nevrotick-metadata-qualification-20261107-zu2t_iqg/` evidence directory is
now absent, so its raw tool, coverage and mutation receipts cannot be freshly
admitted or counted as retained current evidence. Preserve the written history;
recreate current admission rather than fabricating the missing records.

The receiving guarantee is unchanged: valid multiline basic/literal strings may
contain one or two delimiter quotes, but a following seventeenth lexical
container must be refused before `tomllib.loads`. Fifteen/sixteen lexical
containers and genuine quoted bracket payloads remain valid neighbors; the
later decoded-depth rule stays separate. Source owner: metadata contract;
implementation and semantic review are separate agents, with coordinator-owned
integrated verification on Linux/CPython 3.12. Keep the original seven-quote
regression unchanged. Freeze the repaired source, run strict ordinary/CLI and
available admitted static/coverage checks, then reconcile bounded distinguishing
faults and remaining per-file mutation debt. A bug-fix GREEN or hosted gate pass
will not establish full MD-P1–MD-P3, native/package or release qualification.
Fresh evidence uses the existing coordinator evidence chain under
`phase-plan-20261108-v1/metadata-preflight-repair-v1`; all unrelated execution
holds and post-V1 formal deferral remain.

The repaired contract source SHA256 is
`de6b4df99bc70e2c992c639adda8170f69074a9a4047980139aa529cebf0dc21`.
Only its multiline closing-state advance changes; APIs, error families and
resource ceilings remain. The exact closing-triple plus at-most-two-quote rule
matches [TOML 1.0](https://toml.io/en/v1.0.0#string) and the
[pinned CPython 3.12.3 parser](https://github.com/python/cpython/blob/v3.12.3/Lib/tomllib/_parser.py).

Fresh verification passes 247 ordinary tests, all eight preflight methods,
strict typing of eight modules plus one legal/five negative consumers, and four
compile-valid causal faults with healthy/restored eight-test controls. The
original five preflight methods remain byte-identical. Actual source CLI
validate/generate/check reconciles all 15 artifacts; release mode still refuses
unfinished data. The complete configured portable admission/accounting/source
cohort now passes 362 Python tests plus the UI-text checker. Its inherited
prepare-product fixture omission is corrected by two setup lines only: all 26
original test bodies/assertions remain unchanged, and real admission rules,
policies and dependencies are untouched.

The current full coverage run passes all 247 tests, observes all 283 launched
CLI subprocesses with none missing, and excludes 14 altered source-file identity
observations before healthy readback. Nonexecuting package markers retain zero
denominators. Branches are separate evidence:

| Module | Covered/executable lines | Covered/total branches |
| --- | ---: | ---: |
| `__main__.py` | 43/46 | 10/10 |
| `contract.py` | 580/636 | 307/360 |
| `delivery.py` | 275/296 | 153/172 |
| `fields.py` | 17/18 | 1/2 |
| `generate.py` | 357/376 | 119/136 |
| `text_policy.py` | 39/39 | 20/20 |

Every declared security module exceeds 90% executable lines. `coverage-v2/coverage-result.json`
has SHA256 `dd6d336d6442472abb6b876a7924581bc852b0deb5bdcbf0d1615419eb3c8efa`;
`typing-v1/result.json`, `verification.json`, `actual-cli-v1.json`,
`fault-controls-v1/verification.json` and `portable-control-cohort-v1.json`
retain the exact sources, tools, commands and outcomes. The first collector
attempt stopped at its 600.216-second wall bound and remains incomplete; the
unchanged-source retry passed in 670.237 seconds under a 1,800-second bound.
Independent semantic and source/documentation reviews found no blocker.

This fixes the demonstrated pre-parser defect. Four manually selected faults
supply no whole-file mutation denominator or 95% pass; full current critical-file
mutation qualification remains open. The absent historical 8,890-variant
artifacts are not re-admitted or counted. No native/package/release or full
metadata qualification follows, and all unrelated holds remain.

**Published repair and hosted closeout, 8 November 2026:** signed commit
`c4e21fd29906c9b48fed4e22fdd2386af2ca52b3` reached `origin/main`.
[Portable product run 42478884094](https://github.com/DavidOsipov/NevroTick/actions/runs/42478884094)
passed its actual isolated product step. The
[repository security run 42478884249](https://github.com/DavidOsipov/NevroTick/actions/runs/42478884249)
passed workflow policy, full-history/tree secret scanning and the security gate
on attempt 2; the four prepared CodeQL jobs were explicitly skipped under the
existing deferral. Attempt 1 passed the metadata suite but hit a scanner archive
read timeout before scanning; one bounded failed-job rerun completed. The push
reported bypass of pending required checks, retained in the publication receipt;
later green checks do not erase that event. `hosted-closeout-v1.json` in the
repair evidence directory records these exact handles. Full per-file mutation,
native/package and release qualification remain open.

**Adoption source receipt, 7 November 2026:** the dirty working-tree candidate
at `df6e28a4641f2f046373a9c5640aa60736716a22` passed the configured metadata suite:
**183 tests, 59.779 seconds, no failures, errors or skips**, on Linux/WSL with
Python 3.12.3. The new checksum and delivery controls add 28 tests. The actual
source CLI validated, generated and checked 15 files: 14 independently interpreted
payloads plus the external receipt. `field-coverage.json.delivery` records 160
selected source bindings, all 14 artifact contracts and explicit unexecuted
compiled/native/package observations. Its locators are semantic identity
selectors, not generic JSON Pointers. The receipt now hashes eight pipeline
modules; source facts, empty unknowns and the UTF-8 policy remain unchanged.
Release-mode validation still refuses this source candidate as required.

The first integrated run exposed one copied-runner fixture that omitted the new
`delivery.py` dependency and failed before healthy generation. Adding that module
to the fixture preserved its healthy and stale-code assertions; the focused case
then passed before the complete green rerun. Preserve both run logs. Independent
review accepted a healthy payload and rejected seven faulty inputs, including
same-count ownership, duplicate/private fields and unclaimed source text. The
66-skill inventory, both changed skill validators and 596 local documentation
links passed; source/test hashes and the initial staged index remained unchanged
during final verification. Evidence and candidate hashes are retained under
`/tmp/nevrotick-metadata-adoption-20261107-a3iaks61/`, with the independent controls
under `/tmp/nevrotick-delivery-independent-20261107-w1g5cfoi/`.

No Cargo/resource compilation, native Windows, service, clock, package, signing,
Store or formal execution occurred. The preceding source receipts retain only
their exact historical candidates.
This assignment leaves the current Core queue, selected MSIX route, Windows stop,
separate fuzz hold, storage pause and post-V1 formal deferral unchanged.

**Complete metadata register extension, 7 November 2026:** David requested the
full reviewed field scope after the initial prototype subset, research for unknown
facts, empty unanswered values and strict UTF-8 without emoji/hidden characters.
Apply [metadata §13.4](../docs/plans/windows-package-metadata.md#134-complete-field-register-and-source-text-policy--7-november-2026)
to the existing `metadata/` owner. The code-owned field registry defines the full
expected scoped set and applicability; the single TOML record supplies values and
dispositions. Preserve aliases as derivations, not competing edited facts.
Public author IDs, current rights/asset/source/locale references and intended
platform facts use repository evidence; unknown Store/service/provider/signing
identities remain empty. Generic Windows documentation cannot establish those
project assignments. Full coverage means every scoped field is dispositioned;
it does not authorize generic SDK configuration or enable conditional facilities.
The Unicode property data is pinned and offline. Check raw text/comments and
decoded TOML escapes while retaining ordinary international letters. The existing
CI job and source-tool acceptance remain; MD-P4–MD-P5/native/release holds persist.

**Complete-register source receipt, 7 November 2026:** the single 152,296-byte
TOML record now carries 731 exact field dispositions: 551 catalogue rows
(SCM 88, artifact/distribution 267, diagnostics 196) plus 180 shared/component
entries. Authored statuses are 21 populated, 123 derived, 180 pending and
407 not applicable. Resolution yields 63 nonempty values; 261 pending or
not-yet-resolvable derived entries remain empty. These mixed categories are
coverage accounting, not a product-readiness percentage or 731 static values.

Repository/owner evidence supplies author IDs, actual rights/asset/locale paths,
the published documentation locator and referenced lockfile path. The lockfile
reference proves neither an actual build nor a complete shipped graph. Unknown
Store/service/provider/signer identities and unsupported destinations stay empty.
Code-owned derivations prevent duplicate edited aliases. Generic native/schema,
behavioral, runtime and conditional catalogue rows are covered dispositions, not
new SDK setters; concrete native grammar remains with its existing owner.

Final verification: 155 tests passed with no skips/errors in 47.643 s using
Python 3.12.3/Linux; real `validate`, 15-file `generate` and read-only `check`
passed, while release mode correctly refused. Independent review reconciled all
551 catalogue identities and compared the pinned Unicode 18 repertoire across
all 1,114,112 code points without discrepancy. Raw/comment/escaped emoji, hidden
characters, invalid encoding and BOM refuse beside valid international controls.
Three bounded repairs have discriminating controls: shared Store additional-term
limits, hardware-entry limits and current English-only locale facts. The initial
152-case run had one stale CSV fixture expectation; the corrected exact-field
oracle preserves cardinality and multiline-literal checks. No implementation
rejection was weakened to satisfy that fixture.

The existing 66-skill package validation and source/link/whitespace checks passed.
Evidence is under `/tmp/nevrotick-metadata-complete-20261107-jszqzdva`, including
`tests-final-155.log` and the final generated tree. Current source artifacts
require fresh generation with the expanded code-owned contract; no old source
or output compatibility is claimed for this unreleased authoring schema.
Next remains the existing MD-P4 consumer/emitted-resource adoption and MD-P5
native/signing/delivery qualification. No Cargo graph, product runtime, Windows
configuration, Store account or signing/publication state changed in this work.


**Owner-authorized metadata pipeline, 7 November 2026:** David requested planning
and parallel implementation, with an easily found high-level directory. Use
[metadata/](../metadata/README.md) and the [canonical five-stage plan](../docs/plans/windows-package-metadata.md#13-metadata-pipeline-implementation--7-november-2026),
superseding the earlier illustrative `packaging/product.toml` location. The
coordinator owns integration; separate workers own strict source/checked model,
deterministic generators/CLI, and independent admission/output-custody tests.
MD-P1–MD-P3 implement the portable source-tool slice now; MD-P4–MD-P5 retain actual
compile/link/native/signing/delivery owners and holds. These are task-local stage
labels, not new CHECK IDs or a prerequisite to the current Core/N3 checkpoint.

Expected source-tool exit: one editable TOML record, exact existing Cargo fact
derivation without Cargo execution, code-owned release prerequisites, checked
immutable public projection inputs, literal-safe fixed outputs, atomic complete
publication, and read-only stale/tampered/extra-file rejection. Unknown future
identities remain pending with owner/next action; removing their rows cannot
authorize a release. CI uses the already-required workflow-policy job. Preserve
staged/concurrent work and record integrated candidate/results here at handoff.

**MD-P1–MD-P3 source-tool receipt, 7 November 2026:** the validator, immutable
model, deterministic generator, isolated-import CLI and source CI steps are
implemented in `metadata/`. The current source resolves four exact product/Cargo
snapshots, five roles (two emittable prototypes) and six explicit pending owners.
Real `validate`, fresh `generate` and read-only `check` passed for the 12-file
tree; release validation and stale pre-repair output correctly refused. The final
106-test suite passed with no skips in 10.729 s on Python 3.12.3/Linux. Existing
security helper controls passed 24 tests; configured actionlint/ShellCheck/pedantic
offline zizmor passed two workflows with two healthy/three rejection controls.

Independent review found three additional boundaries, repaired with behavioral
RED/healthy/GREEN controls: XML-forbidden characters, equality-spoofed scalar
models and staging cleanup when the child-directory open fails. Focused fresh
re-review accepted the repairs. The diagnostic pending-owner omission also has
RED/GREEN evidence. Earlier missing-module/RC-observer/fixture-selector failures
were harness issues; nonzero prototype revision support was an explicit source
contract refinement, not a claimed pre-existing product defect. Parent-directory
custody fixes passed their controlled cases without a historical RED claim.

Candidate evidence and generated source are retained under
`/tmp/nevrotick-metadata-pipeline-20261107-uy9iekfa`; final test log is
`/tmp/nevrotick-metadata-tests-final-106.log`. This is Linux build-host source
qualification, not an admitted Cargo/product graph, compiled Rust/RC, complete
MSIX/event catalogue, installed/Store-delivered metadata or release approval.
The release mode intentionally remains closed for this prototype schema.
Next: MD-P4 owners wire reached real consumers to a frozen generated tree and
independent emitted-resource readers before their existing component/package
slice; MD-P5 retains all native, account/provisioning, signing and delivery gates.
The current Core queue and all other execution holds remain unchanged.


The [conditional research](../docs/plans/windows11-integration.md#9-conditional-mechanisms-worth-understanding)
and [second-pass reserves](../docs/plans/windows11-integration-follow-up.md#6-conditional-mechanisms)
own source contracts and limits. These reserves are not a prerequisite list.
The later OD-05 decision selects the MSIX row for preparation; the 6 November
clarification also selects strict-CET build declarations and required applicable
runtime hardening. Other rows retain their own conditional triggers. The heading
is preserved for existing links.

| Reserve / existing owner | Re-entry condition and bounded exit |
| --- | --- |
| Job Objects or SCM triggers / platform | A measured optional-role resource/activation need; compare healthy useful work with limit exhaustion, stopping/maintenance races and owner death. Preserve independent broker cleanup; an address or endpoint trigger is not Internet readiness or first-UDP/53 activation. |
| LPAC/AppContainer or preview Win32 app isolation / architecture/security | A named untrusted-work boundary justifies capabilities, IPC, supported launch/deployment and lifetime costs. Observe actual denied access beside admitted work; standalone creation APIs do not prove SCM compatibility. No new privileged broker follows automatically. |
| Strict CET and EHCONT support / build/security | CET plus strict-CET image markers are now selected; require effective strict enforcement on the supported hardware/OS class. EHCONT generation remains limited by the pinned stable compiler and cannot be supplied by a linker-only switch. Inspect the exact metadata/module closure and legitimate provider/callback behavior under CHECK-16/20/26 and the current build-task amendment. |
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

**Service/artifact/diagnostics catalogue, 7 November 2026:** [metadata §12](../docs/plans/windows-package-metadata.md#12-service-and-diagnostics-metadata-refinement--7-november-2026)
now retains the reviewed bounded field catalogue once. The metadata/service and
package-QA skills route field kind, applicability, stable identity, resource/locale
projection and independent readers through this existing owner. SCM update wording
and package-ledger declaration wording were corrected during integration; unknown
service names, provider GUIDs, release/Store identity and public endpoints remain
pending rather than plausible placeholders. Proposed engine/broker wording is not
approved shipping behavior. Resolve each selected field as populated, derived,
pending with owner or not applicable with reason. This is documentation/source
preparation only, not a completed product register or resource generator.
Next action remains the field-register resolution above, now including SCM/MUI
servicing and event-resource rendering with W11-W05/W11-W08. Preserve authored,
generated, embedded, installed, signed/submitted and Store-delivered observations
and existing CHECK-18/20/23/26/27. No current Core prerequisite, new service,
provider/counter set, dependency, native probe, signing or publication is selected.

Metadata integration checks, 7 November 2026: 66-skill metadata/UI/inventory/link
check passed; 1,222 local path/heading checks and the illustrative TOML parse passed.
Three separate SCM/artifact/diagnostics source reviews informed integration;
diagnostics compared 187 XML members across 37 published schema pages with no
member mismatch. A fresh-context three-scenario skill exercise found no
decision-changing gap. These are source/instruction checks, not measured workflow
improvement, a resolved product register or artifact/native qualification.

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
and release action. MSI delivery remains a later owner decision with no automatic
fallback. The 8 November OD-05 §7.7 amendment below supersedes the then-retained
unsigned-MSI/OpenPGP exception and requires all supported individual signatures.
Preserve the complete product, separate
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

**9 November coordinator correction and renewed owner instruction:** David
explicitly says, “You may use Windows Sandbox.” The earlier scoped readiness
permission was carried forward too narrowly as a blanket native stop. Reviewed
native jobs may proceed in an explicitly owned disposable Sandbox after current
CLI/session/resource and candidate/job admission. Necessary management and exact
owned input/output staging remain distinct from payload execution in the guest.
This permits no live-host configuration, elevation, installation, security,
service, clock or trust-store changes. Keep the detected S134 stager/artifact
blocked; do not restore, re-encode or reuse it. Separate fuzz/storage/UI holds and
post-V1 formal deferral remain. Historical stop/readiness records below retain
their original dates and scopes; an experiment still needs its named owner,
expected observations, finite limits, explicit guest ID and cleanup custody.

**9 November readiness refresh:** after David opened Sandbox for this work,
the installed CLI `0.8.107.0` identified exactly
`dd47331c-14ec-425c-ae79-b0e4c618540d`. Coordinator `/root` recorded ownership
before guest execution and used explicit ID plus `ExistingLogin` for two built-in
CMD commands. Parsed guest exits were **0 and 42**, while both host dispatches
exited 0. This establishes the selected command/result path, not product
qualification or an unelevated token. No new guest, shared folder, PowerShell,
product image or host configuration change was used. The owner-opened guest's
requested XML and effective networking/device/protection settings are unknown;
the older offline configuration below does not describe it. Each command had a
20-second outer deadline. The guest remains open for the next admitted payload;
controller cancellation would not prove guest-job completion.
Receipt: external `phase-plan-20261108-v1/nts-ke-graph-v1/`
`sandbox-native-preflight-v1/session-adoption-readiness-v1.json`, SHA-256
`19c12f1b575e1424802ddbc026250c7e21ad74f0291274da3b54ffd141c0975c`.

**Next lab source card — 9 November:** coordinator/build CHECK-19/20/26 and
crypto S0.1 own the next ordinary AES witness. The selected std-only
[`cpu-os-v3-admission.rs`](../tools/windows-sandbox/cpu-os-v3-admission.rs)
must first pass pinned format/Clippy/rustdoc, baseline-only direct linking and
PE/import admission. Exact accepted output plus guest exit 0 admits the later
three v3 test images (6 library, 6 staged and 7 sealing cases); no fuzz, CNG,
services, clock or trust-store effects enter this child. Baseline compilation is
limited to this probe, retaining other hardening and workspace lint/profile
settings. Source review accepted the guarded XGETBV path and finite three-positive/
30-negative controls. The baseline cross-build below is complete; the later
guest launch was refused by application-control policy, as recorded below.
Require exact image/PDB/import identities, sequential jobs, 60 seconds per image,
180 seconds for the test cohort and bounded output/completion custody. A timeout
or dropped CLI observation cannot establish guest process completion.

The current session was re-inventoried as the same ID. A fresh narrow writable
output mapping and built-in guest CMD `ver` produced a 47-byte host-read-back
file, reporting **10.0.26100.9818**, with host and parsed guest exits 0. This
qualifies that output/version path, not the release platform matrix or an
unelevated token. No product image or PowerShell ran. Receipt:
`sandbox-native-preflight-v1/output-readiness-result-v1.json` under the existing
external checkpoint. Effective networking/device/protection settings remain
unknown. Freeze the exact source, flags, tools/SDK and guest job before payloads.
The exact outer `cmd /d /c CHILD >stdout 2>stderr` route also passed nested
built-in children exiting 0 and 42, with those parsed guest exits, both host
dispatches 0 and all four redirected streams empty. Receipt:
`sandbox-native-preflight-v1/mediated-exit-controls-v1.json`. Keep the image as
the sole/final command; do not append a marker that can replace its exit status.
This supplies console-route calibration, not detached-work completion. On a
timeout or output-limit failure, retain an incomplete result and dispose only
the exact owned guest; ID presence without state cannot prove it stopped.

**Baseline probe cross-build accepted:** all seven isolated boundary/compiler/
format/Clippy/rustdoc/link/LLVM commands passed, using only the one declared CPU
flag substitution and all 40 workspace lint controls. Source SHA256:
`7eb47dcb3b533011a873360eb2d11087614c8a90d40ce49f98af43018ea5c399`.
Independent readback accepted AMD64/PE32+/console, 191 CFG functions, relocations,
ASLR/DEP, both CET declarations and dependent-load flags `0xA00`; these remain
static image declarations. EXE SHA256:
`5aa099bc6522fc34bc98f738e160ae06ab3777f23a6f286a51096dbff2354188`.
The emitted PDB hash/header and CodeView basename match; internal PDB GUID/age
matching is unverified. Build record `cpu-os-v3-probe-build-v1/build.json` SHA256
`e00a9eb504ea58d0540218b38f5bb4a12fa8134bb77ce427a1d541c4c2b91d16`;
47.45 seconds, peak sampled owned RSS 218,202,112 bytes. All 43 named inputs,
stable/SDK inventories and source bookends passed. The later signed service
commit changes only Git publication custody; the build retains its actual
`6b947c5`/`54b77d26…` execution identities. No Cargo or native probe ran.

**Separate guest runtime prerequisite:** the probe's ten declared imports include
`VCRUNTIME140.dll`. A fixed-path guest CMD query returned 42 for its absence
(host dispatch 0), retained in `vcruntime-presence-v1.json`. Prepare the
[Microsoft x64 v14 runtime](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist/)
as a separate guest-only setup job: freeze its actual HTTPS source, bytes,
SHA256, version and embedded Microsoft Authenticode signature before execution.
Use explicit owned ID and guest `System` only for installation, direct WSB
execution with `/install /quiet /norestart`, a separate 180-second limit and
bounded setup logs. No host installer, host PowerShell, security/trust change or
restart is authorized. A readable guest-only signature query makes no execution-
policy change. Require a completed guest exit, installed runtime/version readback
and successful CPU probe before any v3 image; reboot-required/timeout/overflow
remains incomplete and invokes owned-guest disposal. Restore `ExistingLogin`
for ordinary witnesses. This selects neither product runtime redistribution nor
MSIX deployment mechanics, and changes no Cargo graph or product flags.

**Owner installation disposition:** David subsequently chose to install C++
himself. The assistant's setup path is therefore unexecuted: only the official
18,731,856-byte installer was downloaded (SHA256
`843068991daaa1f73ad9f6239bce4d0f6a07a51f18c37ea2a867e9beca71295c`).
No prepared installer-signature query, installer or guest elevation ran. David
then reported installation complete. Read-only, explicitly guest-targeted
PowerShell metadata queries found AMD64 `VCRUNTIME140.dll` 14.51.36247.0,
178,616 bytes, SHA256
`d1f4225df2cd877dbf130d5668a021dce3f94118455ff5ec952061c30afc9ce7`,
and the installed x64 package in both registry views. The first check incorrectly
compared that redistributable version with the Desktop import-library package
14.51.36254 and returned 17. Preserve that failed expectation: the pinned
Microsoft compiler-component manifest explicitly pairs Desktop 14.51.36254
with Redist.X64 14.51.36247. Corrected lab metadata admission is retained in
`sandbox-native-preflight-v1/runtime-admission-v2.json`, SHA256
`d24218fffa3739dc7eda54ee86077b6cc3adca5fa45b67e3ab1b17a483847b62`.
This establishes the guest metadata prerequisite, not successful loader
resolution, redistribution policy or product runtime qualification. No host
PowerShell, installation or trust/security mutation occurred.

**AES cross-build accepted; native tests not run:** the unchanged AES source
projection produced three fresh release/v3 test images and their PDBs. All five
isolated boundary/build/LLVM commands passed with one Cargo job; independent
review accepted all three PE/import readbacks and source/tool/SDK bookends.
Build record `aes-witness-windows-build-v1/build.json`, SHA256
`983656bae8fc198b93636141ab27085feddd10f8692d04e13743d72ee832fdb1`,
binds the exact artifacts and 6/6/7 expected test names. Elapsed 251.93 seconds;
peak sampled owned RSS 374,853,632 bytes. No native listing or test execution
occurred, and no AES image entered the guest input mapping.

**Native launch refusal and closeout:** the V1/V2 controllers stopped before
image dispatch because they expected a JSON share acknowledgement and a denied-
write diagnostic file respectively. Their incomplete receipts remain. V3
accepted the CLI's empty share acknowledgement only for that operation, verified
mapped input presence and denied writing, then attempted the exact baseline
probe under `ExistingLogin`. Host dispatch returned 0; parsed guest exit was
**4551**. Stdout was empty and the 166-byte stderr reported Device Guard policy
blocking the file. The selected SDK calls this error
`ERROR_SYSTEM_INTEGRITY_POLICY_VIOLATION`. The responsible policy, path rule,
trust decision or dependency has not been established. The controller's
`native_probe_started` flag means dispatch attempted, not execution of Rust
`main`. CPU/OS admission and embedded controls remain unexecuted/unverified;
the dependent AES native cohort remains incomplete.

Failure receipt `sandbox-cpu-native-v3/result.json`, SHA256
`e75853c969376a1a8b13ef33af8ebbd9d641e2c99349c544eb90669adf56e00b`,
and independent review `independent-review-v1.json`, SHA256
`ad45e23742d3b1a2e97e3809b56f624c601e2dd1c57f40211fe8042a32d94ad8`,
preserve this boundary. Exported logs and unchanged two-file input hashes were
verified before stopping only the owned guest; a fresh inventory was empty.
Closeout `sandbox-cpu-native-v3/closeout-v1.json`, SHA256
`b1b9df46d3103f670e6456b771ecbcc3693420b427ac3e8108e363864d86e930`,
records guest disposal separately from product cleanup. Its installed runtime
was disposable guest state. Do not retry this blocked image, relocate it to
evade the block, relax protection or substitute host execution. Retain the
native gap and continue the existing S2.2 source contract; further diagnostics
need a bounded reviewed job and a newly admitted guest.

**9 November owner execution amendment:** David additionally authorizes GitHub
Actions machines for this work, and permits main-host tests only if they will
not crash his Windows instance. Prefer a disposable GitHub-hosted Windows job
for native laboratory qualification; no such main-host safety guarantee has
been established, so no main-host payload is selected. Freeze exact candidate,
CPU/runtime/tool prerequisites, bounded test inventory, privacy/output limits
and independent expected outcomes before dispatch. Record runner image/build,
effective protections and actual exits; hosted Windows results do not qualify
David's machine, the full support matrix, installed services or packages.
This authorizes preparation and execution of the separately frozen, admitted
hosted route, including reviewed probe bytes identical to the local guest's.
Source/artifact reconciliation is technical admission and needs no additional
owner permission. It authorizes no protection relaxation, retry in the blocked
local Sandbox, S134/fuzz/storage work or production TLS/trust/provider waiver.
Existing source/dependency admission and critical-file gates remain mandatory.

**Hosted preparation follow-up, 9 November:** read-only source review selected
an explicit hosted placement profile in the existing `setup-xwin.py`, preserving
its default WSL storage rule and shared download/catalog/terms owners. Hosted
tool, locator and receipts must stay in a fresh external runner-owned area.
Placement alone is insufficient: current version commands lack wall/output
bounds, splat lacks a whole-job/output quota, and the post-write recursive
catalog scan is not bounded admission. Before invocation, the build owner must
reuse owned-process supervision and a byte/inode quota, then strictly admit
finite regular output files before hashing/consumption. No setup code or workflow
has been changed or dispatched for this preparation.

The concrete draft and researched artifact-Action candidates are retained under
`phase-plan-20261108-v1/nts-ke-graph-v1/hosted-windows-preparation-v1`:
`hosted-profile-contract.md`, SHA256
`c2a8893428b4f107b5bae6dad53f32b82b541438d4fa5d0625debf1db902a13b`, and
`actions-artifact-pins.json`, SHA256
`5784afbeed52510808659aaae0f04d5580bfd06439b2ee2a5ff0952d9c9c100b`.
These are unadopted preparation inputs. Continue with the portable Linux builder,
fixed bounded same-run artifact transfer and owned Windows process controller;
require the baseline CPU acceptance before listing the three AES test images.
The existing 6/6/7 oracle, runtime/PE identities and effective-policy observations
remain required. A fresh authorized hosted VM is a separate environment; preserve
the blocked guest's result and never weaken its policy to obtain execution.
Artifact metadata limits and strict-digest Action inputs alone do not bound
actual download writes; the transfer owner must implement and exercise that
bound before claiming a finite download quota.

**Hosted acquisition implementation contract — 9 November:** baseline `ffd47bb`
completed the selected handoff source/host child. This continuation implements
the preparation above under CHECK-19/20/21/26/27 and the existing REQ-8/BND-2
S2.2 owner. The coordinator owns integration and execution; delegated source and
test authors have separate writable scopes. Independent contract review accepted
this bounded acquisition stage; no native Windows or product pass follows.

The existing `setup-xwin.py` gains explicit `--hosted-runner`; its default WSL
`/mnt/[a-z]` rule, exact download/catalog pins and Microsoft development terms
remain. The hosted cache argument names a private canonical `RUNNER_TEMP` child,
with a fresh cache and explicit new receipt below its dedicated payload mount.
Bootstrap execution stays outside the payload's `noexec` mount. Tool, locator,
provenance, temporary files and logs remain profile-owned; ambient home paths
and Actions credentials are unavailable to acquired processes.

The payload uses the repository's existing **tmpfs quota pattern**, rather than
introducing a loop-device controller: total capacity at most 2 GiB and 65,536
inodes, ordinary-UID ownership/mode 0700, effective `noexec,nodev,nosuid`, distinct
device and exact mount identity. Admission rejects aliases, nested mounts,
links and pre-existing payloads. These are experimental ceilings until an actual
healthy hosted acquisition succeeds. Tmpfs storage consumes RAM/swap; the quota
is not an aggregate process-memory limit. See the
[kernel quota semantics](https://www.kernel.org/doc/html/latest/filesystems/tmpfs.html).

One 600-second acquisition deadline, bounded output, existing owned-process
supervision, PID/user isolation and process resource limits cover network stages
as well as version and splat commands. There is no unbounded/host fallback.
Finite archive expansion and descriptor-based no-follow inventory precede
catalog hashes, library consumption, accounting and cleanup. Original pins,
declared-versus-raw catalog checks and required library readback remain mandatory.

The manual Linux workflow is restricted to this repository's `main`, checks out
the exact triggering SHA, has read-only contents permission and no arbitrary
inputs. Its first observable exit is actual quota-backed acquisition plus
independent bounded receipt/library readback and owned-mount cleanup. A separate
small quota control must observe byte/inode exhaustion. No upload, compiler,
Windows execution, signing or package effect belongs to this stage. Without
retained SDK outputs, this experiment supplies no reusable downstream SDK.

Focused checks cover default compatibility, hosted admission before effects,
archive/inventory limits, timeout/output refusal and cleanup preserving the
original failure. A two-case RED already reproduces catalog symlink acceptance
beside a healthy regular payload; raw results are in
`phase-plan-20261108-v1/nts-ke-graph-v1/hosted-xwin-v1/catalog-red.{json,log}`.
Implementation and hosted execution remain unfinished at this contract freeze.
Preserve local Sandbox/S134 blocks and all unrelated execution holds.

**Hosted acquisition source checkpoint — 9 November:** implementation now adds
the explicit hosted profile, bounded bootstrap/inventory admission, whole-worker
supervision and independent receipt/library readback. The public CLI always
dispatches; its former internal worker selector is removed. The private entry
checks actual UID, inherited limits, read-only root and private bwrap PID owner
before acquisition. Its worker record remains unqualified until parent completion.
The manual workflow checks out the exact main SHA and keeps its privileged mount
operations confined to new owned runner directories, with independent mount-state
admission before deletion and primary-failure preservation.

Final local execution passes **72 selected tests**: 27 existing/setup cases,
26 hosted controls, 12 readback/ownership cases and seven inert workflow-shell
cases. Pinned actionlint, strict ShellCheck and offline pedantic zizmor pass all
three workflows with their healthy/rejection controls. Actual benign unprivileged
Linux namespace probes observe allowed payload/worker-bootstrap writes, refused
root/device/native-bootstrap writes, ordinary UID, cleared synthetic credential
and successful private-worker containment checks. Actual FD replacement and
producer/consumer lock controls pass. Quota/mount facts and shell mount operations
remain simulated in these local cases. The actual pinned cached xwin release
archive also parses into the four admitted regular members; no installation ran.

The source controls retained behavioral RED before repairing catalog symlink
admission, final-open hardlink replacement, self-invalidating cache receipts,
pre-existing payload admission and the new namespace's writable root. Initial
archive fixtures and a shell-probe routing mismatch were repaired without
relaxing production assertions. The first namespace-hardening attempt lacked
the bwrap-required explicit user-namespace option; its setup refusal remains
retained. The final route requires it and does not fall back to host execution.

Receipt `hosted-xwin-v1/verification-v2.json` under the same external
`phase-plan-20261108-v1/nts-ke-graph-v1` chain hashes to
`e7415c571ee8d878d47ce28ae9b44de7aa4151675f217f116a591d55c81fc372`.
Its 18 execution-input bookends match. Independent source/API review accepted
the supported route; it supplies no separate execution. The next action is
exact-candidate pre-commit qualification, signed publication and dispatch of
the manual Linux acquisition experiment. Actual dedicated quota exhaustion,
healthy acquisition, effective hosted observations and cleanup remain pending.
This prepares the Windows builder/consumer; it is no Windows execution, reusable
SDK export, provider adoption or completed S2.2/Core/engine result.

**First hosted result and diagnostic continuation — 10 November:** signed source
`6af28b6` was published and
[manual run 42686681057](https://github.com/DavidOsipov/NevroTick/actions/runs/42686681057)
checked out that exact commit. Its preparation and 27/26/24 offline cases passed.
The real tiny filesystem emitted both byte and inode `ENOSPC` control success.
The source-bound trace then reaches parent layout/ownership/quota admission and
the supervised worker, which exits 1. Neither timeout nor output overflow is
reported. Its child output was discarded, so namespace setup, worker admission,
bootstrap and acquisition remain competing causes; no acquisition or ceiling
sufficiency pass follows. The first raw log is retained as
`hosted-xwin-v1/hosted-run-v1.log`, SHA256
`844b8e2494632708978a2e57ff9c7e6a7976b13ea286c861f58024d86d2a4e28`.
Owned-quota cleanup is unobserved in that failed run; generic runner cleanup
establishes no such result.

The bounded follow-up preserves actual exit/failure and an ASCII JSON-escaped
256-character child-output tail, within a 4,096-character diagnostic. This is
bounded diagnostic text, not general secret redaction; environments and full
commands are not dumped. New real-child controls reproduce the missing diagnostic
beside healthy output before repair. Cleanup now emits its own completion fact
and primary/final exits after unmount, independent mount-state admission and
removal; it preserves the primary failure. Inert cases first reproduce the
missing observation, then pass healthy, failed-work and failed-cleanup neighbors.

A real nested namespace control now launches the native namespace from inside
the worker, using an owned non-`/tmp` fixture and the actual worker environment.
It observes private PID/user namespaces, ordinary UID, fresh home/temp, intended
payload/temp writes and refused root/device/native-bootstrap writes. It supplies
no dedicated quota, SDK, network or Windows observation. These changes qualify
one fresh diagnostic run; they do not establish or repair the unknown SDK cause.
All original limits, pins, thresholds and protection holds remain.

David also permits Windows Sandbox when useful and temporary accessibility
detachment if needed. The current standalone crypto graph already excludes
Slint/AccessKit and C++ REST SDK, so no accessibility detachment is selected.
The earlier guest runtime prerequisite concerned Visual C++ Redistributable;
the baseline probe's error 4551 still has no established policy/dependency cause.
Use a freshly admitted owned Sandbox profile when appropriate; this grants no
retry of the blocked image or protection relaxation. Keep this route separate
from the current Linux acquisition failure and from product UI qualification.

**Owner-reported guest SAC remediation — 10 November:** David now reports the
attached guest-only workaround succeeded and Sandbox is ready for use. The
handoff describes SAC blocking `wixstdba.dll` in the Visual C++ 2012 x64
11.0.61030 installer; the coordinator received its text, not the underlying
screenshot. Microsoft's [Sandbox collaborator](https://github.com/microsoft/Windows-Sandbox/issues/85#issuecomment-3025447682)
provides the same `VerifiedAndReputablePolicyState` DWORD 0 guest workaround;
[a counterexample](https://github.com/microsoft/Windows-Sandbox/issues/85#issuecomment-3025799619)
limits its general applicability. Preserve David's prepared guest, reconcile its
exact ID and authorized use before dispatch, and do not recreate or reapply the
setting merely for agent ownership. This supersedes the earlier fresh-guest
preference for this prepared instance. The next admitted native step may use
this changed SAC-off lab profile after artifact, session, resource and execution
preflight. Record configured setting, effective observations and actual result
separately. The installer result does not establish the original probe's 4551
cause or a successful probe, nor qualify SAC-enabled product behavior. No host
security change or Defender-held artifact is authorized by this report.

The [Sandbox skill](../.agents/skills/environment/manage-windows-sandbox/SKILL.md)
and its troubleshooting/configuration/operations references now retain the exact
guest command and readback, host/guest policy distinction, failed-`CiTool` meaning,
in-guest restart versus disposal, bounded Code Integrity 3077 collection and the
stop-on-unchanged-failure rule. Independent primary-source review confirmed the
workaround's scope. This documentation checkpoint runs no Windows commands;
David's successful result remains owner-reported until the next observed guest
run. Accessibility detachment remains unnecessary for the standalone crypto graph.

**Hosted Linux continuation parked:** exact diagnostic source `a4c5c3a` ran as
[42692416447](https://github.com/DavidOsipov/NevroTick/actions/runs/42692416447).
Setup's 27 cases passed; hosted controls ran 29 with one failure: the real nested
native bwrap could not create a user namespace inside the worker. SDK acquisition
was skipped. Raw log `hosted-xwin-v1/hosted-run-v2.log` is 37,694 bytes, SHA256
`d2cbbd8aa869587fa8736644626efdc2d8760ab3abcedd64d8a3ff86b24fe89e`.
The installed distro profile's inherited capability denial is the source-backed
leading explanation; this run did not capture its actual worker label/audit denial.
The nested control is now an explicit mandatory dedicated-workflow invocation,
separate from generic controls in the main Rust lane, whose stricter profile is
unchanged. The moved control additionally observes dropped capabilities and
`NoNewPrivs`, records worker/native policy labels and challenges further native
user-namespace creation. No assertion is skipped or budget increased.

A dedicated per-executable AppArmor compatibility profile and same-run
restrictive/compatible comparison were independently reviewed as a possible
hosted repair, but remain unimplemented and unexecuted. No profile/sysctl change
or new dispatch follows. David's newly available Sandbox takes priority over
further SDK tooling; retain the hosted failure and reopen that route only for a
named remaining native-environment gap. This closes neither SDK acquisition nor
Windows/provider/Core/engine acceptance. Existing S134, AES-fuzz, storage and
unrelated UI holds remain; formal work stays post-V1.

**Observed prepared-guest CPU/AES result — 10 November:** David explicitly
instructed use of his running clean guest with VC++ 14, 2013 and 2012 installed.
CLI `0.8.107.0` identified exactly `387dbcc7-4cb4-4979-8d60-0d71523494d4`;
the coordinator established its command channel, preserved the guest and used
only ID-targeted `ExistingLogin` payloads; this is no standard-user token proof.
Registry readback observes SAC's
configured DWORD 0 and installed x64 VC++ `v14.51.36247.00`; no installer,
registry mutation, restart or host PowerShell ran. Requested XML and effective
network/device policy remain unknown. The guest reports **10.0.26100.9818**;
this is a laboratory result outside the selected 26H2/Server 2025 release matrix.

The exact previously admitted baseline probe, EXE `5aa099bc…`, now returns
guest exit 0 and the exact 19-byte `cpu-os-v3=accepted\n`, with empty stderr.
All six original source/configuration hashes and both artifact hashes match
the retained build. Narrow read-only input and writable output mappings pass
positive-read and denied-write controls; both mediated exits 0/42 retain their
actual guest status. The first 975-MB host observation stopped preflight before
staging. Independent workload-specific review found no universal 1-GiB rule;
the refreshed observation before execution was 3,651 MB available physical and
34,214 MB available virtual memory. An earlier controller field-name error is
retained; neither setup stop counts as native execution. Build floors are unchanged.

The subsequent three exact release/v3 AES images passed all **19 ordinary native
tests**, with independently matched listing and completed names: library 6,
staged 6, sealing 7; zero failed/ignored/filtered, all guest exits 0 and empty
stderr. The sequential cohort took 6.28 seconds and produced 2,465 bytes of
bounded collected output. All 37 current original-source and 36 projection
hashes still match map `cd563b59…`; the six image/PDB hashes match the previously
reviewed Windows build. No compilation, fuzz, CNG, service, trust-store or clock
effects enter this cohort. Output limits are controller/readback limits, not a
claimed automatic writable-share quota.

Raw receipts under the existing external chain:
`sandbox-owner-ready-20261110-v1/admission.json`, SHA256
`13790ac87d377c77bbf50b3448293c72cb61e07e5187703d1f702f0c349a012d`,
and `sandbox-owner-ready-20261110-v1/aes-ordinary/result.json`, SHA256
`1779c0ac68df2ee8f6fb0a2935e4d195040646d1cfa8f3026b1c954d2a3f76c9`.
Independent retained-evidence review inspected the actual raw outputs and
reconciled every name, input/artifact/output hash and receipt link. This was a
same-context follow-up after a fresh-context skill review, not another execution.
The prior 4551 refusal remains historical evidence, without causal attribution
to SAC alone. Successful execution in this changed guest does not prove that
SAC is effectively disabled for every path or qualify SAC-enabled compatibility.
The prepared guest remains open with its runtimes; no disposal masked product
cleanup. Next: admit the real Windows laboratory KE/exporter/closure/handoff
path in this guest. Provider/ordinary-store trust, native sensitive-copy cleanup,
runtime protections, final OS matrix and production acquisition remain open.

**Native KE refusal checkpoint — 10 November:** the
[crypto task](s0.1-crypto-feasibility.md#native-ke-refusal-checkpoint--10-november-2026)
now records the retained combined release/v3 link, separate static readback and
five real guest cases. Missing/nonloopback arguments return exit 2; closed
loopback returns exit 1 with `connect/refused`; malformed TLS and EOF each drain
a 230-byte ClientHello and return client exit 1 with distinct primary
`handshake/invalid-data` and `handshake/unexpected-eof` labels. Secondary
`shutdown/invalid-data` and `response/unexpected-eof` remain recorded. Peer
controllers complete without forced kill or capture/harness-cleanup gaps;
this is no successful TLS-closure or native-cleanup qualification.

Native receipt `ke-native-refusal-v1/result.json`, SHA256
`f1589a77c1fae0cb14326c18d07377bcc7af6fe221f6037c906f8618781f95d5`,
binds 18 output files / 3,975 bytes to the admitted EXE/PDB and guest source.
Independent raw reconciliation and strict host readback pass their declared
scope. The first build's five-GiB stop remains incomplete; later readback
`d87444af94eeab3f1bd8f4bf79ccf5a4396d9bbdb8efb9f2318ee1c8a4e8e925`
uses the preserved link without compiling again. The crypto task also retains
the two omitted transitive tooling pins in older CPU/AES freezes; both were
fixed before this KE build, without rewriting old receipts.

The initial PowerShell `Restricted` setup stop is retained. All five baseline
policy scopes were `Undefined`; only the explicitly targeted guest PowerShell
process used `RemoteSigned`, with effective `RemoteSigned` and version
`5.1.26100.9818` observed. No persistent/host policy change, `Bypass`, PKI, service
or clock mutation occurred. The prepared guest remains open; Windows build
26100 is outside the selected 26H2/Server matrix, and SAC configured Off proves
no effective security profile. CNG/TLS first-flight rejection now ran; healthy
TLS, certificate trust/revocation, KE/exporters, erasure and complete mitigation
gates remain open. The [healthy fixture checkpoint](s0.1-crypto-feasibility.md#source-preparation-and-resource-stop--10-november-2026)
now has a reviewed contract and uncompiled peer/store-helper source. Read-only
preparation stopped before guest dispatch: observed host available memory was
928 MB against the 2048 MB minimum. Keep compiler/native work held until David
resumes after establishing headroom; preserve the guest and WSL settings.
The controller is now authored and source-reviewed under the
[source-only continuation](s0.1-crypto-feasibility.md#source-only-continuation-under-the-resource-hold);
its parsing/execution and helper/graph/artifact admission remain pending.
The HOST memory parser passes 18 pure tests without querying memory or changing
the resource floor. Actual healthy execution is still pending. Existing artifact/fuzz/storage/
UI holds and post-V1 formal deferral remain.

**Resource checkpoint:** the published `89f035e` verification copy and compiled
LLVM targets from two superseded incomplete privilege campaigns were deleted:
2,284 files / 302,154,214 logical bytes. Their source, reports and logs remain;
those old raw archives are intentionally partial. Current final platform and
caller archives remain complete. Retirement map:
`superseded-verification-payload-retirement-v1.json`, SHA256
`8fc06026a0881f0f4c8a822f9a0aadce49ec5137e07ae343c6d4c60e8a7cd02e`.
No compression was used.

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
| S6.4 | W11-W10 with release/license owner and independent QA; prepare exact submission then qualify delivered bytes | Reconcile exact Slint/framework/backend/features, generated code, executable/runtime/assets, grants/notices and complete SBOM/payload/size inventory; bind submitted/delivered identities, every required individual signature under guide §4.1, package trust and installed contents. Qualify post-Store inner-signature preservation; Store signing is not helper authorization. |

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

#### PE content trust refinement — 8 November 2026

**Scope and status:** the review of `981b01d186091ef5285128ccc9316d8e7e3e03e8`
found concrete padding guidance in the signing guide/package QA but incomplete
architecture and execution routing. This amendment specializes existing
W11-W10/S6.4 and applicable consumer owners; it creates no second package task.
Source/documentation integration is prepared. Behavioral, strict-verifier and
final-artifact qualification of this refinement remain **not run** under the
current prerequisites and native holds; no product code is changed here.

Apply [architecture section 19.6](../docs/windows-time-architecture.md#196-authenticated-artifact-content-and-consumer-trust)
and [signing guide section 5.2](../docs/windows-time-signing-guide.md#52-pe-certificate-padding-and-exact-final-bytes)
before dependent consumer integration and the S6.4 release checkpoint.

| Existing owner / checkpoint | Required continuation and observable exit |
| --- | --- |
| Reached metadata/helper/loader/maintenance consumer owner; CHECK-18/25 | Trace artifact bytes through parsing/admission to a command, destination, configuration or load decision. Excluded certificate content supplies no application authority; harmless hostile markers have no protected effect while admitted input works. Preserve actual verified-object identity through use; record a source-based applicability decision where no such consumer exists. |
| Every shipped PE under the later 8 November owner amendment; release/security verifier, CHECK-18 | Freeze actual Windows/tool/architecture, embedded signature source, trust action/flags, effective padding policy and restart evidence. Healthy embedded-signed PE succeeds; the otherwise-valid nonzero-padding case fails for that condition. A catalog result or unrelated chain failure cannot qualify this row. |
| W11-W10/S6.4 with independent QA; CHECK-18/23/26 | Bind full-file SHA-256 and lengths to the protected approved stage record. Reject substituted final bytes or components; reconcile local, signed, submitted, Store-delivered and installed identities at their actual stages. Provider documentation alone supplies no candidate-bound verification result. |

The existing candidate receipt must retain each applicable input/source identity,
expected and observed result, healthy control, signature mechanism and reasoned
non-applicability or not-run outcome. Keep embedded PE, package/catalog,
consumer-effect and project/release authority observations separate. Native-work
lifetime/retrieval limits remain CHECK-27 when reached; no unrelated campaign is
added. These observations join existing S6.4/P7 evidence before broad release.

**Next action:** at the next affected source integration, identify actual
consumers and their enforcement owners in this task. At the admitted artifact
checkpoint, qualify the existing guide's finite controls against the exact
candidate. Keep complete MSIX/Store selected and individual/package results distinct,
all native holds, and David's submission/publication authority. No new downloader,
updater, custom PE parser, customer-registry change or production signing is
authorized by this documentation amendment.

**Documentation validation:** three read-only technical/routing reviews approved
the integration; the final diff whitespace check and 25 added local links/anchors
passed, and all three changed skill frontmatters remain unchanged. The full
skill-pack checker remains **not run** because the execution sandbox's required
namespace isolation was unavailable in this session. Scoped static checks supply
no skill-script, consumer, Windows or release qualification result.

#### Sign every supported shipped artifact — 8 November 2026

**Owner instruction and source basis:** against `f408ebed1fa522f756a80f16d0b2155c36a3d3e7`,
David required signatures on the installer and all included binaries, then
expanded that requirement to every signable shipped file, including nonbinary
files. [OD-05 §7.7](../docs/nevrotick-owner-decisions.md#77-sign-every-supported-shipped-artifact--8-november-2026)
records that superseding decision. [Architecture §19.7](../docs/windows-time-architecture.md#197-signatures-for-the-complete-shipped-inventory)
owns acceptance; [signing guide §4.1](../docs/windows-time-signing-guide.md#41-required-signatures-for-the-shipped-inventory)
owns the single format/inventory/signing-order procedure. The preceding padding
receipt retains its original candidate scope; its catalog-only exception is
superseded by this later instruction, while its content-trust controls remain.

**Scope and current state:** source/documentation integration only. Every
supported individual file signature is required in addition to the signed
package, including every PE by real format and admitted signable scripts/data.
Preserve approved upstream signatures and authenticate remaining data through
the complete signed package. No new script host, product format, custom verifier,
provider account, paid certificate, workflow, signing operation or runtime code
is introduced. Production signing capability, emitted/delivered artifacts and
native qualification remain **open / not run**, not waived by documentation.

| Existing owner / checkpoint | Bounded continuation and observable exit |
| --- | --- |
| W11-W10/S6.4, package/build owner; CHECK-18/23/26 | Derive the complete shipping signature inventory from real format/container/stream contents, including nonstandard-extension PE and existing nonbinary signables. Bind expected project/upstream signer, supported signing/verifier profile, source, component/release identity and stage-specific exact bytes. Remaining data has explicit package coverage; lack of a key/provider is open work, not N/A. |
| Release/security owner with R-06; CHECK-18 | Resolve production signer capability for required inner artifacts on the selected MSIX, provider eligibility and protected approval/rotation/revocation custody. SignPath is only a candidate. Preserve upstream signatures; resolve unsigned third-party inputs through provenance/licence/provider policy before shipment. No unsigned-MSI/OpenPGP release exception remains. |
| Package owner and independent artifact QA; CHECK-18/23/26 | Finish metadata, sign/verify inner files and then containers inside-out. On the actual returned and Store-delivered candidate verify both individual and package signatures, inventory and required inner-signature survival. Qualify missing/removed/invalid/wrong-signer signatures, omitted nested PE, modified signed script and mismatched inventory beside a fully covered valid candidate. |
| Existing installed-code/consumer owners; CHECK-16/18/23, CHECK-25/27 only when reached | Identify real launch/load/verifier boundaries. Signature presence does not prove automatic checking; keep PackageIntegrity, required CIG, protected paths and helper/IPC admission. Apply the existing PE-padding/content-trust and finite native-work controls without a new scanner or runtime download path. |

**Release disposition:** missing required signatures, unresolved production
capability or incompatible Store transformation blocks release pending resolution
or an explicit owner decision. No catalog-only substitution, manual detached
consumer verification or automatic MSI fallback can close the gap. Independent
source preparation continues; complete MSIX/Store, Core priority, native/fuzz/
storage/UI holds, David's submission/release authority and optional enhanced
provenance retain their existing scope.

**Next action:** at the next assigned S6.4 source checkpoint, populate the
inventory and signer-capability card for actual intended artifacts, with unknown
rows visibly unresolved. Run the finite native/provider/delivered-artifact cases
only at their admitted stages. Documentation review and static validation are
recorded below when completed; no Windows or signing pass is implied.

**Documentation review and validation — 8 November 2026**

The frozen 19-file candidate used base
`f408ebed1fa522f756a80f16d0b2155c36a3d3e7`, manifest SHA-256
`c63867f4d08ad3f4bd4f72ffcc955c8ca082cbc34ae88e6fd239a82d95e9c2b2`
and diff SHA-256
`8172bc9d7ccdcccb1151a34a8c555d200a018bc6bc9f7c06ef2199b5e2e274ff`.
These identities precede this factual receipt.

- `padding_primary_sources` completed the read-only primary-source/core-rule
  review. SIGN-R1 (the cross-route evidence list still demanded MSI) and SIGN-R2
  (the provider table limited SignPath to MSI) were corrected in signing guide
  §§5–6, then verified against the frozen guide. The reviewer also verified the
  narrow internal/external CAB and format-limited append clarifications.
- `signing_coverage_judge` completed the separate read-only 19-file diff review,
  applicable instructions and targeted conflict-language check; it matched the
  frozen diff at start and finish. Its verdict was **READY for documentary
  integration**, with no blocking findings. It retained prior conversation
  context; it was not a fresh-context reviewer.
- Two `fork_turns: none` attempts, `all_signables_judge` and
  `all_signables_fresh_review`, returned the failure “Codex ran out of room in
  the model's context window.” They produced no usable review or confirmed
  inspection trace. Record them as **not run**, not zero findings or added coverage.

The judge's comprehension paragraphs are preserved verbatim:

> The candidate implements David’s 8 November decision as a production shipping requirement: the installer/package and every shipped artifact with a supported, format-appropriate signing mechanism must have verified signatures, including nonbinary formats. It supersedes the unsigned-MSI/OpenPGP exception, preserves acceptable upstream signatures, and makes missing production signing capability or incompatible Store transformation a release blocker. The technical rule lives in architecture §19.7; signing guide §4.1 owns the single format, inventory, verification and signing-order procedure.

> The current workflow remains preparation of one complete Store-first MSIX, followed by admitted local qualification, David’s controlled submission, and qualification of the actual Store-delivered result before broad release. W11-W10/S6.4 and the existing CHECK owners carry the work. These documents do not implement signing, select a provider, approve a paid certificate, authorize account/publication actions or establish native results; they preserve the existing execution holds and keep source preparation available while production signing remains unresolved.

Coordinator static validation passed the diff whitespace check and all **47**
added/changed local link-and-anchor pairs. All three modified skill frontmatters
are unchanged; no root documentation checksum bundle exists. The full
`check_skills.py` check remains **not run**: the `security-audit` skill requires
isolated execution of repository code, and the session's required OS namespace
isolation was unavailable. Trusted static reads/checks did not execute repository
code. These results establish neither a full skill-checker pass nor signing,
consumer, Windows, provider or release qualification. The final receipt receives
a whitespace/link-scope check before publication; its addition does not extend
the reviewers' inspected candidate.

#### Interim self-signed MSI clarification — 8 November 2026

**Owner correction and basis:** after `e9539c0b20b2e2d79a913420e932259fa8e4f14b`,
David reported no personal publicly trusted code-signing certificate and accepted
SignPath Foundation if eligible or an own generated certificate for an interim
MSI. [OD-05 §7.8](../docs/nevrotick-owner-decisions.md#78-signpath-and-interim-self-signed-msi--8-november-2026)
records the current decision; [signing guide §7.1](../docs/windows-time-signing-guide.md#71-interim-self-signed-msi-profile)
owns its evidence. This qualifies the earlier signing-capability requirement by
declared profile; it does not undo full-file coverage or revive unsigned/OpenPGP
substitution. Earlier review quotations/receipts describe their original candidates.

This permitted interim profile is not confined to laboratory builds, but its
availability does not select MSI instead of the current complete Store MSIX.
No public publisher-trust result or installation compatibility is established.
Use the same W11-W10/S6.4/R-06 card and existing checks:

| Existing owner / checks | Required continuation |
| --- | --- |
| Release/security; CHECK-18/26 | Bind the permitted MSI profile, full supported signature inventory and independently approved signing key to protected release authority. Qualify key custody/rotation/compromise handling, expected-key identity and tamper/removed/wrong-key controls in the declared verifier context; preserve approved upstream signatures and final-byte/padding checks. |
| Windows/package QA; CHECK-18/23 | Retain the unchanged clean-target untrusted-chain result separately from controlled signature verification. Qualify actual installation, warnings/refusal, runtime and maintenance on each claimed policy configuration without customer certificate import or protection changes. A blocked configuration is not a supported pass. |
| Existing helper/loader/lifecycle owners; CHECK-16/18/23, CHECK-27 when reached | Resolve each existing public-trust assumption for the profile without unconditional verifier success or weaker required CIG/loader admission. Qualify later migration to the admitted public signer and reject silent downgrade. Store MSI/EXE and direct-MSIX trust constraints remain separate. |

**Disposition:** absence of a publicly trusted certificate alone is accepted for
this named interim MSI profile. Missing/invalid required signatures or another
required-control incompatibility remains blocking for the affected configuration.
The code-signing provider's acceptance, MSI delivery selection and exact release
approval remain distinct. No certificate/key was generated, exported or installed,
and no signing, provider submission, workflow, native run or release was performed.
Current execution holds and optional-provenance scope remain unchanged.

**Next action:** populate the existing signer-capability card with the declared
profile and expected key/verifier/trust observations when the MSI alternative is
assigned; continue current Store-MSIX/Core preparation independently. Documentation
validation is recorded below after review, without a signing/native pass.

**Documentation review and validation — interim MSI clarification**

The frozen ten-file candidate uses base
`e9539c0b20b2e2d79a913420e932259fa8e4f14b`, manifest SHA-256
`3039bb099185adfcc56336e180f6abd01abbcc1f1ee95b63cd5edce0e18b722d`
and diff SHA-256
`c0ecbb9155d07d5a5d97fb328ad78b3d7ee02ade8da8057df4da0dd3e1d9a68c`.
These identities precede this factual receipt.

`padding_primary_sources` completed the scoped owner/signing-guide technical
review, matched both document identities and reported no technical findings.
`signing_coverage_judge` inspected the actual ten-file diff and relevant contracts,
matched the diff at start/end, and returned **READY for documentary integration**
with no blocking findings. Both were separate read-only reviewers with earlier
context, not fresh-context reviews. Failed fresh launches from the earlier
candidate supply no coverage for this clarification. No reviewer edited files
or executed repository code, Windows tools, signing or account actions.

The judge's comprehension is preserved verbatim:

> The candidate records David’s clarification as an accepted interim MSI signing option: an owner-controlled generated certificate may sign the installer and required owned payloads while publicly trusted signing is unavailable. This permission is expressly broader than laboratory use. It preserves the requirement to sign every supported shipped artifact, keeps approved upstream signatures, and separates verification against the approved project key from public Windows publisher trust.

> The project still prepares one complete Store-first MSIX; this clarification does not select MSI delivery or authorize signing or publication. If MSI is selected later, W11-W10/S6.4 and R-06 must qualify the declared self-signed profile, retain clean-machine trust failures accurately, resolve actual helper/loader dependencies and verify installation and maintenance on each claimed configuration. Lack of a public CA certificate alone no longer blocks that named profile, while missing signatures and incompatibility with another required control remain unresolved work.

Coordinator static checks passed all **20** added/changed local link/anchor pairs,
unchanged frontmatter for both modified skills and diff whitespace checks. No
root documentation checksum bundle exists. The full skill checker remains
**not run** because the `security-audit` skill's required OS namespace isolation
was unavailable; no repository execution bypass was used. The final receipt
receives a scope/whitespace check before publication. Documentation readiness
establishes no signing key, working MSI profile or native/release qualification.

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

**17 November source continuation:** the
[managed logical-source child](engine-transport-pilot.md#managed-logical-source-provenance--implementation-contract--17-november-2026)
retains actual catalogue/source identity through transport publication and
independent series/address-round reception. Managed and unmanaged provenance
cannot substitute for each other; session/endpoint replacement does not redefine
the logical source. Identity retains no mutable policy cell or permission.
Development, intended compiler controls and current line gates pass within the
[recorded scope](engine-transport-pilot.md#provenance-compiler-and-coverage-results--17-november-2026);
all four changed-owner95% gates pass independent adjudication with zero unresolved
survivors/timeouts, within the recorded GNU source scope. This supplies one genuine
provenance prerequisite, not a configuration/association revision, uninterrupted
clock mapping, calibrated error, authentication or Core eligibility. The settled
supplier contract and its other missing production/native exits remain in force.
Use development §7 for current implementation and execution routing.

**Installed configuration continuation — 17 November:** the
[numeric-source configuration child](engine-transport-pilot.md#installed-numeric-source-configuration--implementation-contract--17-november-2026)
now issues a checked revision at the real managed source. The final send and
reply guards enforce it. Series and address-round receivers retain the original
revision with completed facts. Replacement invalidates dependent pending work
and preserves admitted KoD restrictions and committed history. The current
`0910c474…` candidate passes the
[selected GNU checkpoint](engine-transport-pilot.md#installed-configuration--final-selected-gnu-checkpoint-18-november-2026).
The earlier external compiler controls retain their exact scope. Clock mapping, association and
key-lifetime binding, and production and native admission remain separate.

**8 November receiving checkpoint:** the
[report-owned session reference](engine-transport-pilot.md#report-owned-reference-final-qualification--8-november-2026)
now has bounded Linux qualification on 424-input `e931838b...`. Genuine reports
retain their private pre-acquisition chart and original successful measurement,
with checked exact midpoint derivation and unchanged ages. This does not provide
mapping continuity, calibrated finite errors or production admission; its three
unobserved factory paths and generation limits remain recorded.

**Finite source contract settled — 8 November:** acquisition/platform and the
engine receiving owner now have the
[producer/receiver contract](engine-transport-pilot.md#w11-w01-finite-capturemapping-supplier-contract--8-november-2026)
and exact pinned Rust/Tokio/Mio plus Microsoft source trace. No affirmative
mapping issuer is implemented. The contract requires real identity/evidence,
finite validity/errors and current invalidation, or explicit unavailable outcomes;
its accepted/stale/unavailable/late-completion controls preserve original captures
and ages. The pilot chart/watch value is not that authority. Actual provider and
native qualification remain open; no Windows probe is authorized.

**Completed portable child — 8 November:** the
[within-series observed-coherence veto](engine-transport-pilot.md#within-series-observed-coherence-veto--implementation-contract)
has [final bounded Linux qualification](engine-transport-pilot.md#within-series-coherence--final-joined-qualification).
It can reject contradictions among genuine retained
captures under one bounded run's existing declaration and retire future use.
Compatibility grants no mapping/physical-error guarantee. There is no cross-series
wall baseline or new tolerance. The raw-order repair, source-linked surrogate,
real fixtures and per-file gates retain their exact scopes; the surrogate is
not native Windows evidence. The supplier source contract above is settled;
actual provider/native implementation remains with acquisition/platform and
its admission. The next executable routing follows
[development §7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent),
which selects the independent inert NTS request formatter. All production
supplier and execution holds remain.

**6 November source continuation:** the [first controlled measurement checkpoint](engine-transport-pilot.md#first-controlled-measurement-source-checkpoint--6-november-2026)
implements generic software wall-read brackets, complete header/identity/context
retention and conditional nominal arithmetic. GNU tests and Windows cross-builds
pass; consistency observations prove no uninterrupted UTC mapping or finite
wire-capture error. W11-W01 remains partially prepared: actual Windows packet
placement, mapping/error and native completion/lifecycle qualification are open.
This does not select kernel/NIC timing APIs or lift the Windows execution hold.

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
selects the ordinary observation-to-Core chain: Q1's conditional-quality,
persistent session and report-owned reference checkpoints are qualified within
their recorded Linux scopes. Now prepare the actual capability/capture/mapping
suppliers in the existing receiving card. Core already computes exact
self-minus-anchor phase/reference differences, so another engine pairwise API
is not a prerequisite. Under OD-10 §14.1, all formal work remains deferred
until after the first released V1; it is not a prerequisite for this source
contract. The selected Linux Core
checkpoint, leap implementation/proof scope, complete MSIX, optional enhanced
release provenance, S134/native and fuzz restrictions, storage pause and UI-M0
D1–D3 deferrals retain their recorded status. This preparation lifts no hold and
supplies no source-to-native qualification.

#### W11-W01 application timing reference trial — 20 November 2026

S2.1, TQ-01/07/08/09 and CHECK-6/7/11/14/21/25/27. The acquisition and
Windows platform owners supply one reference for the original application
send and receive brackets. The Engine owner carries the accepted mapping
revision through the existing guarded consumer. Domain/bootstrap and S1.4
supply date, era and scale context. Core owns history admission after all
those inputs have been qualified.

**Readiness: blocked at reference admission.** The independently reviewed
[actual CLI witness](engine-transport-pilot.md#actual-observe-cli-capture-bracket-witness--w11-w01-19-november-2026)
binds the two peer observations to the 10 MHz guest QPC shared with the
Engine. Its 202 ns allowance covers the observed ordering and conversion
comparison. The evidence supports relative interval containment. A traceable
UTC reference, finite physical event-pairing errors and a valid mapping
envelope are still missing. The protected receiver continues to publish
timing unavailability.

Before execution, freeze the actual reference identity and calibration
record, traceable UTC scale and validity, the event-pairing method for T1
and T4, and independent error and drift bounds. State which physical
event each pair represents. Set numerical leg and exchange budgets before
looking at validation outputs. Admit compatible independent date and
peer-scale evidence. Reserve separate reference observations for verification.
Qualified bounds must also cover the interval between observations or
retire the mapping when continuity is uncertain.

For each held-out leg, compare mapped UTC with the independent reference
and include the reference and pairing error bounds. Require the resulting
worst-case error to fit inside the issued uncertainty, and the issued
uncertainty to fit inside the predeclared leg budget. The validity interval
must cover both complete original capture brackets. Propagate the bounded
leg, peer, scale and rounding uncertainties to the declared exchange
budget under the existing measurement contract.

At the actual serialized consume-once boundary, observe one admission for
the healthy same-revision exchange and one for a fresh exchange after
reissue. Require zero current admissions with absent or expired evidence,
excess uncertainty, a crossing capture or revision replacement before
commit. A late result retains the original observations, age and debit
without an additional history row or freshness publication. Controlled
invalidation establishes the software guard. Physical power and wall-clock
transitions require their own native evidence and restoration.

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

Both use `x86_64-pc-windows-msvc`, OD-07's required **x86-64-v3**, the complete
hardened MSIX and selected Slint UI. The [current target configuration](../.cargo/config.toml)
now selects v3 with the expanded required hardening vector; this is not native
CPU/startup or process-mitigation qualification.
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

These reuse W11-W01/02/04/05/10, S0.4/S0.7, S2.1/P2 and P4/P6/P7; UI retains S0.5/P5.
The [acceptance catalog](../docs/windows-time-architecture.md#202-acceptance-catalog)
owns requirements; groups below are neither new CHECK IDs nor executed passes.

| Group and existing checks | Independent useful/refused observations and owner |
| --- | --- |
| **Acquisition timestamps** — CHECK-6/7/11/21/25/27; CHECK-8 for source identity/quorum and CHECK-24 for independent evidence review | W11-W01 applies [TQ-01–10](#timestamp-qualification-matrix--18-november-2026) to the claimed application path and any selected software/NIC adapter. Record actual OS-family/NIC/driver/firmware/interface/family/mode and mapping evidence. Unsupported optional hardware retains a qualified baseline; unknown calibration or stale metadata cannot become precise current time. Virtual/client evidence does not fill physical/Server cells. |
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


### 4.33 Portable service lifecycle source start — 7 November 2026

**Request/disposition:** David requested reading the service docs/skills and
starting implementation. W11-W05/platform owns this bounded portable source
child of S0.2/S3.1 and CHECK-27. It prepares logic for the two separate own-process
roles already required by DEC-2 and architecture §§3–4: engine without clock
privilege, narrow clock broker with independent recovery. The on-demand helper
is not a third resident service. This request authorizes source work; the current
Windows execution stop, separate fuzz hold, storage pause, UI deferrals and
post-V1 formal deferral remain. The Core/engine continuation in development §7
is not completed or displaced by this source result.

**Authority/skills:** architecture §§3.4, 4.1, 4.2.1, 11, 12.3 and 17.1; the
[service skill](../.agents/skills/implementation/implement-windows-service/SKILL.md)
and its maintained adapter guide; Rust implementation/contract, planning and
contract-testing instructions. CHECK-1/2/13/15 and W11-W08/W11-W10 dependencies
remain open, including supported MSIX accounts/pre-start provisioning. No service
name/account, wrapper, dispatcher, native API feature or production timeout is
selected by this child.

**Implementation:** [windows-platform](../crates/windows-platform/Cargo.toml)
is a real seventh workspace member owning only the shared portable
[service lifecycle](../crates/windows-platform/src/service.rs), without engine,
Tokio, GUI or new registry dependencies. Its sole external edge is the existing
exact Garde 0.23.0/defaults-off/derive. The existing admission/check owners now
bind its local package, unchanged external feature closures, both target rows
and `service.rs` security/critical-file obligations. Existing external lockfile
records remain unchanged; the new record is only `windows-platform -> garde`.

**Independent expected results and enforcement:**

- One serialized, non-cloneable invocation owner holds the status reporter;
  attempted state and last successful acknowledgement remain distinct. Only
  healthy readiness reporting admits Running work. A report error permanently
  fences new work while cleanup remains possible.
- StartPending/StopPending accept no Stop control and use nonzero checkpoints/
  hints; stable states clear those fields. Only Running enables Stop. Progress
  never renews a fixed monotonic deadline; checking a stuck phase emits no fake
  progress. No pause/shutdown/preshutdown or arbitrary management surface exists.
- Duplicate internal stops neither report again nor restart the stop deadline.
  Outstanding bounded job permits prohibit readiness/terminal completion;
  cancellation, timeout and Drop release no capacity. Explicit actual completion
  releases only the originating invocation's charge, including after owner drop.
- Terminal Stopped is attempted once, even on status failure. An earlier report
  error requires an explicit nonzero failed terminal outcome. Cleanup/recovery
  must finish first; a terminal report failure cannot be retried using its context.
- Garde runs in private checked budget construction. Raw status construction and
  permit duplication are unavailable to external consumers. Raw/invalid inputs
  produce bounded typed refusals without OS effects.

**Candidate/custody:** based on dirty HEAD
`e5d9a0112e198ab41a50ee0b0057e06eab68ab84`; pre-existing engine/quality, metadata
and documentation work was retained. Other agents changed unrelated inputs during
this session, so these are scoped checks, not a frozen whole-workspace pass.
The exact 15-input source/configuration digest is
`a8771e9de83236c0e2f31c5fa47f8d14e590fcaca2dbec5643302c4ef50ec9f0`.
To reconstruct it, SHA-256 each path below in order, form
`PATH + " " + HEX_DIGEST + "\n"` per file, concatenate and SHA-256 that UTF-8 text:
`crates/windows-platform/Cargo.toml`, `crates/windows-platform/src/lib.rs`,
`crates/windows-platform/src/service.rs`, `crates/windows-platform/tests/service.rs`,
`Cargo.toml`, `Cargo.lock`, `policy/ui-m0-dependencies.json`,
`policy/verification-manifest.toml`, `scripts/verify.py`,
`scripts/slint_dependency_profile.py`, `scripts/tests/test_verify.py`,
`scripts/tests/test_slint_dependency_profile.py`, `rust-toolchain.toml`,
`.cargo/config.toml`, `policy/rust-workspace.toml`.
Main service source SHA-256:
`45daf9fae8416e16b50a344560e9393fe7c0479d299f3c45c1763d247e68eb4e`.

**Executed ordinary checks:**

- `cargo test --frozen -p windows-platform`: 12 public-consumer tests and three
  doctests passed; two doctests reject private construction/permit duplication.
  Configured nextest with `--lib --tests --user-config-file none --no-tests=fail`
  separately executed all 12 tests, zero skipped.
- `cargo test --frozen -p windows-platform --profile verify-release`: the same
  12 tests and three doctests passed with optimized code and mandatory checks.
- Host and `x86_64-pc-windows-msvc` `cargo clippy --frozen -p windows-platform
  --all-targets -- -D warnings` passed; rustdoc and focused rustfmt passed.
  Windows compilation used the existing v3/hardening vector and performed no
  link, executable launch, SCM call or native Windows test.
- External rustc consumers compiled a legal constructor neighbor, rejected raw
  budget/status construction with E0451 and permit cloning with E0599.
- `python3 -B -m unittest scripts.tests.test_verify
  scripts.tests.test_slint_dependency_profile
  scripts.tests.test_verify_vendor_admission`: 168 passed. Strict workspace
  adoption and actual source/manifest/lock admission passed. Graph-owner frozen
  metadata/workspace/platform trees matched both targets (215 Linux/249 Windows
  selected external identities), without new registry identities/features.
- Independent review found the earlier report-failure/clean-terminal issue. The
  actual public-consumer regression first failed because Clean was accepted;
  the smallest `FailureOutcomeRequired` guard then made it pass beside valid
  clean and explicit-failed terminal controls. Final independent review found
  no remaining material issue within this portable scope.

**Retained first failures:** the initial compile rejected the pinned compiler's
now-deprecated atomic `fetch_update`; replacing it with `try_update` restored
compilation. Clippy first rejected missing contract documentation and one test
assertion style; those were corrected without lint exemptions. An initial Python
`-I -m unittest scripts...` invocation could not import the local package and
executed no controls; the corrected command above passed. None of these setup/
lint failures is credited as a behavioral regression kill.

**Incomplete/next action:** this is source authoring and ordinary-check evidence,
not CHECK-27/S0.2/S3.1 completion. Preserve the 90% per-security-file coverage and
95% per-critical-file mutation gates; no service campaign or full sandboxed
workspace verifier was run here. Complete that focused qualification before
claiming this owner qualified. Actual readiness/native completion are still
adapter assertions; counter isolation does not qualify stale-result publication.
Then admit one narrow SCM wrapper/binding path and implement dispatcher,
registration, callback custody, Interrogate/unsupported-control behavior and
native error/status mapping under these contracts. Normal installed service work
still needs the supported MSIX provisioning/account/isolation boundary; native
execution additionally needs the existing hold lifted and its disposable-machine
card. Real tokens/rights, effective mitigation/error policies, diagnostics,
service arguments, durable broker recovery, IPC, package/remove/update and clock
effects remain with their existing owners. No service was installed or started.


### 4.34 Active two-service completion goal — 7 November 2026

The original owner goal covers both own-process services and their complete
contract/test integration, not only §4.33's portable source child. This is the
historical goal setup; the current source handoff and next action are in
[§4.35.8](#4358-service-source-handoff-and-integration-prerequisites--8-november-2026).
§4.33 supplies predecessor evidence, not a parent completion claim.
Use this existing W11-W05/S0.2/S3.1 chain with W11-W08/W11-W10 and their canonical
security/lifecycle owners. Keep the separate engine/Core work and existing
Windows/fuzz/storage/formal dispositions intact.

| Obligation / owner | Current disposition and next action | Required observable exit |
| --- | --- | --- |
| Portable lifecycle, SCM, privilege validation and actual-completion custody; platform/CHECK-1/27 | Implemented; retained ordinary/release platform cohort is 39 tests plus eight doctests. Current integrated per-file coverage/mutation remains open; see §4.35.8. | Explicit contracts, direct/bypass/healthy/fault/boundary/concurrency cases, >=90% executable lines per security file and >=95% viable mutation per critical file with complete inventory/omitted behavior reconciliation; integrated source and custody unchanged. |
| Process entry, dispatcher, ServiceMain, bounded callback notification and checked status mapping; platform/CHECK-27 | Two fixed-role laboratory hosts and native adapter source exist; retained ordinary/release fixture cohort is 37 tests. Product adoption, current Windows linking and actual SCM evidence remain open. | Prompt dispatcher connection; registration and status failures remain failures; one transition owner; no FFI unwind, dangling context, false Running/Stopped or use after terminal status. Host injection, Windows compilation/link and actual SCM observations remain separate exits. |
| Service identity, required privileges, creation-time object security, process/token isolation, unattended-error and mitigation profiles; platform/security/CHECK-1/15/16 | Supported complete-MSIX provisioning/account mapping is unresolved; ordinary installed service work and native probes remain held. | Effective engine token lacks clock/debug/impersonation rights; broker narrow allowed controls succeed, injection/network/foreign-owner paths fail beside valid neighbors; actual configured/effective phases and profiles observed. Use existing decision procedure for material topology/security choice. |
| Protected management, arguments, maintenance inhibition and service-specific diagnostics; IPC/security/W11-W05/W11-W08 | Contracts apply before dependent adapters; actual IPC and diagnostic owners remain separate incomplete work. | Process/SCM/durable arguments cannot substitute identity, roots, endpoints or clock authority; unauthorized/stale/direct-handler bypasses have no protected effects, benign Interrogate/unsupported control handling, bounded redacted required-event failures and safe cleanup. |
| Startup/shutdown/restart and worker cancellation; lifecycle/CHECK-27 | Laboratory capture/control-loop custody exists. Product workers, native completion/recovery and per-role restart/maintenance behavior remain open. Clock journal/recovery algorithms retain their existing external owner. | Actual jobs/callbacks retain fixed budgets and drain before terminal acknowledgement; finite per-role restart and maintenance behavior, diagnosed failures and valid neighboring recovery. Service lifetime cannot renew external clock authority. |
| Two executable compositions and source adoption; W11-W05/P3, with W11-W10 provisioning dependency | Two laboratory executable compositions exist; no current Windows artifact or final product-service integration pass. Full package delivery remains with W11-W10/P6. | Separate own-process engine/broker composition without privilege/runtime leakage; qualified handler/worker integration with simulated effects. Intended accounts/pre-start provisioning and native evidence gate installed hosting; package delivery is a separate exit. |
| Final independent acceptance; coordinator/security/QA | Entire parent remains incomplete. | Requirement-by-requirement audit of actual delivered/built/native behavior and persisted outcomes, full applicable security/resource/package gates, no missing required evidence; portable or aggregate green cannot substitute. |

Original bounded campaign plan: one service.rs owner, one writer for source, one for
public tests, one coordinator for tooling/output. Inspect cheap tool/resource
readiness first. Planned initial ceilings are six ordinary build jobs, four
mutation workers with one build/test slot per worker, <=512 generated variants,
300 seconds coverage, 120 seconds enumeration, 1200 seconds total mutation,
90 seconds per build and 60 seconds per mutant test. Preserve all attempts and
first failures; revise only for an identified cause. No formal or fuzz execution,
Windows command, clock write, service registration or package mutation is admitted.


#### 4.34.1 Current lifecycle repair and qualification progress

The independent public-consumer review expanded the suite from12 to21 tests.
Actual REDs: a duplicate stop at an earlier monotonic time returned false success;
startup/stop expiry followed by cleanup accepted a clean service exit. The owner
now observes duplicate-stop time without repeating status or extending its
fixed deadline, and latches every observed pending-phase deadline failure into
the required nonzero terminal outcome. The effect-side guards also detect expiry
when callers omit a separate deadline poll. All21 tests, three doctests,
optimized verify-release tests, host and Windows-target Clippy passed after these
repairs; the earlier §4.33 source identity is historical, not the current candidate.
The suite now includes an81-input independent raw-budget grid, cap2/3/8 partial
refill plus barrier-synchronized real-thread completion, failed pending reporting,
permanent failure fencing, exact/reversed observations and preserved typed causes.

The first listing attempt used the new fixed wrapper with the generic full
analysis profile. Standalone hash-only preflight passed in118.92seconds. The real
listing invocation then failed before creating its owned output while scanning
an unrelated nightly toolchain vendor/getopts directory with `ENOMEM`. No
compiler, mutation baseline/campaign, formal or fuzz command executed, and no
mutation/coverage evidence was obtained. The live handle91987 is terminal.
The corrective tooling slice admits only this exact stable-only service role
through the existing Session; it retains the complete pinned stable inventory,
separate pinned coverage/mutation binaries, sealed vendor/source bookends,
90/95 gates and finite budgets. Existing historical role semantics remain under
regression controls; no unrelated proof payload or receipt is repaired/refreshed.
The new source-policy binding is being renewed after that deliberate helper edit.


#### 4.34.2 Owner-requested stop for PC resources

David requested stopping when PC resources are insufficient and said he will
restart the agent when they are free. Goal work is paused at that request; no
new check or guard-control retry was started afterward. The running qualifier
handle28500 was collected and is terminal. The one-off guard-control handle58294
had already failed and is terminal. No service qualification job remains live.

Preserved current results:21 ordinary and21 verify-release lifecycle tests,
three doctests, host/Windows-target Clippy;151 portable-helper/service verifier
controls; coverage227/237 executable lines (95.78059%); complete53-variant listing
bound to hash6b0c26ca69165cc0b1eca40786c7883413835757a7c5be5b7a38a713a41ed031.
The full campaign produced raw53 outcomes (31 caught,22 unviable, zero misses/
timeouts observed), but final attribution rejected a foreign compiler error.
Do not call the mutation gate passed or credit these provisional raw counts as
qualified evidence. Final receipt status is incomplete, metrics_passed=false,
qualification_passed=false. Actual guarded omission controls are also incomplete:
the first recipe used a host manifest path inside the namespace and failed before
behavioral execution. A corrected v2 recipe was prepared but has NOT been run.

Retain `/tmp/nevrotick-service-final-20261107-v2/verification.json` and its complete
raw mutation outputs; `/tmp/nevrotick-service-coverage-20261107-v2/verification.json`;
`/tmp/nevrotick-service-enumerate-20261107-v3/verification.json`; the frozen source
root `/tmp/nevrotick-service-candidate-20261107-v1` (416-input bdd6ceaeee4e6d09c50ea8c9c3899c326bd39198b2cbb9edec0203def80339a3);
the exact533-package vendor `/tmp/nevrotick-service-vendor-20261107-v1` (27101
admitted files); and both guard recipe/evidence v1 paths. Corrected unexecuted
recipe: `/tmp/nevrotick-service-guard-controls-20261107-v2.py`.
The original shared vendor and all unrelated checkout changes remain preserved.
The live checkout changed an unrelated engine quality test during admission, so
final metrics used the already-admitted immutable source snapshot. Selected
service/helper bytes matched; no whole-live-worktree pass follows. Live service
policy remains bound to that frozen candidate and must be reconciled on resume.

On David's resume: inspect current source/resources and these terminal receipts;
diagnose the actual unviable compiler attribution before changing its classifier
or rerunning any campaign. Complete source-bound generation/Garde omission
controls and independent qualification, then the source-only direct SCM adapter
slice. The independent API-fit review favors the permitted narrow windows-sys
fallback over the pinned windows-service macro because of eager raw-argument
allocation and hidden callback/error custody; no native dependency, identity,
provisioning or wake mechanism was adopted. Preserve all Windows/native, fuzz,
storage and post-V1 formal holds. Both actual services and the full goal remain
incomplete.


#### 4.34.3 Resumed source qualification and resource custody

David resumed the goal and authorized WSL cleanup if space becomes insufficient.
Initial available memory was about10GiB and disk4.5GiB, adequate for these bounded
checks. The read-only cleanup inventory found only86.55MiB of optional temporary
build output and actively mapped repository target libraries; no deletion was
necessary or performed. Preserve all raw receipts, frozen source/vendor and
staged work. At resume the index SHA-256 was
`92902e18b318515fc58af46c9460e2a1ef504c04ee2cfb04666b32e93d6e92cc`, HEAD unchanged;
there were no unstaged edits. New authorized implementation/helper work is left
unstaged over the owner's staging.

Independent triage confirmed v2's classifier was correct. Of53 raw variants,
31 were behavioral caught,21 legitimate selected source/compiler unviable and
one unresolved proc-macro2 linker/resource failure. Its firstbad log SHA-256 is
`5d1d6aefee14f4b18bac7b3c6cfa80a8e2ca64fb0fd07086bb7caa883413fe6a`.
No classifier fix, fabricated unviable credit or raised resource limit was used.
After resource availability changed and the actual Garde build/valid controls
succeeded, the same frozen53-variant profile was rerun with the existing four
outer workers, one inner job and128-task bound.

The fresh `/tmp/nevrotick-service-final-20261107-v3/verification.json` now passes
its selected actual metrics:227/237 executable lines (95.78059%),53 complete
variants comprising32behavioral caught,21source/compiler unviable, zero missed,
timeouts or exclusions; viable score32/32. Complete baseline executed21ordinary
and3doctests. Receipt SHA-256:
`53052737e9266bc07db8eb91eb2e3aa5473707f96fa5534158d15c60e73bd546`.
The old failed attempt remains incomplete. Raw metric success still leaves the
independent source/generation audit and required external-contract evidence as
separate obligations.

The corrected sealed omission-control recipe v2 passed allthree actual constructor
faults: omitted mandatory Garde invocation, weakened zero-hint rule, and relational
validation bound to the wrong field. Every fault built, retained an executed
valid neighbor and failed the actual81-input raw-budget grid. None receives
credit in generated mutation counts. Guard receipt:
`/tmp/nevrotick-service-guard-evidence-20261107-v2/verification.json`, SHA-256
`a48180b05d8acf1cbb8c8f2f88ba7471a0e8612d59f7be4ceeb97ac836efb94d`.
An external privacy-consumer replay is preserving exact diagnostics; its initial
v1 recipe omitted the checker's required source-line binding and therefore did
not qualify. The corrected v2 remains a separately observed run, not assumed pass.

Parallel admitted source preparation now adds portable SCM seven-field status
projection, bounded Stop/Interrogate classification and a sticky atomic Stop
notification with no callback, allocation, native call, name/account or wake
mechanism. Seven independent SCM tests and three new doctests passed alongside
the existing21/3, for28public tests and6doctests; packageClippy passed after one
documentation lint correction. This new scm.rs owner is not covered by the
historical lifecycle-only qualification. Its separate critical/security gates and
current two-owner integration are being added before any qualification claim.


#### 4.34.4 Independent lifecycle checkpoint exit

The independent reviewer completed the frozen portable lifecycle audit after the
retained privacy v2 controls passed. Exact qualified service.rs SHA-256:
`947a54b02859ec6fbfe998fa7a19a24ca522ab3c84dfca1fd70033fe7f545c31`;
tests/service.rs:
`d531c98ed085c16dc8dfc4d3f1330b59ebd88ec40e78d38a0aba511cf5009c7a`.
Both still match the live files. The old frozen lib.rs remains
`7e1096c73e9c310e056a6da1b887d1aa979b394e77c30e28545c5aa2ddb1c527`;
its later SCM addition is outside that checkpoint. Raw metric receipts retain
qualification_passed=false; this source-bound independent audit supplies the
bounded qualification, not native or whole-crate approval.

All27concrete source bodies were reconciled to the complete53-descriptor
inventory:25names have generated variants; PendingBudget::new and
ServiceLifecycle::new have no generated faults. Constructor omission controls,
raw81-input and extreme-boundary cases, both-role inert construction and
separate-capacity/counter traces cover those omissions independently.
StatusReporter::report is abstract; actual native acknowledgement remains open.
Every getter/status projection of the existing owner, state transition, pending
budget/deadline/current-time guard, reporting-failure fence, terminal attempt,
job capacity/completion and bounded error/cause path maps to its named public
positive/negative/control tests. All53cause-log hashes and426captured source/
helper hashes match the receipts; compiler-unviable cases have zero kill credit.
The failed original hint relation fault compiled and was behaviorally caught in
v3, resolving the earlier resource failure without changing source or gates.

The retained external privacy receipt
`/tmp/nevrotick-service-privacy-evidence-20261107-v2/verification.json` contains
an actual Cargo-attributed frozen rlib, a compiled/executed legitimate consumer,
raw budget/status field rejection with E0451 and non-cloneable permit rejection
with E0599 at their intended lines. Earlier v1 remains a failed harness setup.

Qualification limits remain explicit: no LLVM branch denominator,32/32function
summaries but only32/88generic instantiations covered; fixed error-formatting
arms, defensive unreachable current=None and checkpoint/deadline overflow error
regions unexecuted. Checked arithmetic remains. No native Windows, callback,
account/provisioning, live process/IPC/clock, packaged artifact or final service
qualification follows. The new SCM owner and integrated two-owner candidate
retain fresh per-file gates under service-analysis schema2; full goal stays active.


#### 4.34.5 Two-owner source extension and conservative resource replan

The coherent source candidate now freezes418product inputs at
`ce638e487fdeb9be206ba99cb8bdea479e2c7e1a8e77968bd4a3fc341cd250f4`
under `/tmp/nevrotick-service-candidate-20261107-v2`. It executes28public tests
and6doctests. Fresh coverage passed independently per declared owner:
service.rs227/237 (95.78059%) and scm.rs63/63 (100%); no branch-denominator claim.
Complete discovery has53lifecycle descriptors (unchanged hash above) and24SCM
descriptors with hash
`826a85bdb95825d972a1460054a8c05c76a6b33692d9592b23579e140281a5db`.
Both inventories were reviewed and bound; this source extends, not retroactively
expands, the independently qualified old lifecycle snapshot.

The first two-owner campaign at
`/tmp/nevrotick-service-two-owner-final-20261107-v1` is terminal/incomplete because
its strict classifier again rejected a foreign compiler resource failure. Keep
all raw logs/outcomes and never credit it as selected-source unviability or a
passing per-file score. The exact underlying host-versus-per-namespace limit is
not established. The revised source-bound schema3 reduces four outer mutation
workers to one while preserving one inner job,128tasks,90/95per-file gates,
complete77-variant selection and all existing per-command/aggregate time bounds.
Historical schema1/2 profiles remain exact; no limit is raised or test filtered.
The next actual run will determine whether that conservative peak-load reduction
resolves the infrastructure failure.

SCM source-generation omissions receive separate real semantic controls:
wrong outgoing own-process/package namespace, a failed exit projected as Win32
success, discarded Stop publication and an already-stopped fresh notification.
Their source-bound sealed builds/valid neighbors/actual selected-test failures
must be observed before independent qualification; they add no generated score
credit. The control run is separate from any assumption that generated mutation
scores cover literal projection tables or excluded constructors.

#### 4.34.6 Independent two-owner portable checkpoint

The completed schema3 campaign is retained at
`/tmp/nevrotick-service-two-owner-final-20261107-v2/verification.json`, with
the coherent source under `/tmp/nevrotick-service-candidate-20261107-v3`.
The product input digest remains
`ce638e487fdeb9be206ba99cb8bdea479e2c7e1a8e77968bd4a3fc341cd250f4`.
Both owners passed their unchanged per-file gates: lifecycle coverage 227/237
(95.78059%), with 32 behavioral kills and 21 selected-source compiler-unviable
variants; SCM coverage 63/63 (100%), with 21 behavioral kills and three
selected-source compiler-unviable variants. All 77 descriptors were accounted
for, with no misses, timeouts or exclusions. Actual mutation wall time was
43.175 seconds plus 30.323 seconds, or 73.499 seconds in total.

Independent review reconciled all generated descriptors and raw causes against
the two source owners. The separately retained constructor/Garde omission,
four SCM semantic-fault and seven external privacy controls passed with healthy
neighbors; none contributes generated mutation credit. Relevant receipts are
`/tmp/nevrotick-service-guard-evidence-20261107-v2/verification.json`,
`/tmp/nevrotick-scm-semantic-evidence-20261107-v2/verification.json` and
`/tmp/nevrotick-service-privacy-evidence-20261107-v3/verification.json`.
This closes only this portable checkpoint. Branch coverage, complete generic
instantiation coverage, atomic memory-order guarantees and native enforcement
are not established by those results. Earlier infrastructure failures retain
their original incomplete status.

One outer worker was a temporary diagnostic profile for this completed small
cohort, not a general concurrency policy. The next larger campaign will first
admit four outer workers with 512 tasks, worker-owned targets, one inner Cargo
job and one test thread. The existing 512-task runtime guard requires those
last three conditions. Preserve historical schemas, exact source/tool/vendor
binding, full inventory, timeouts, thresholds and target hardening. Record
resource peaks before increasing concurrency further. The earlier linker
failure establishes a foreign build failure; its precise exhausted resource
remains unproved. Ambient Rust flags must not replace the Windows v3/protection
vector.

#### 4.34.7 Four-worker replay and measured resources

Schema4 restores four outer workers and admits the existing 512-task mutation
profile. The current wrapper and policy select exactly that row; shared admission
retains schemas1–3 at their original worker/task values. All 162 selected helper
and runtime controls passed, including rejection of crossed profiles, changed
gates and ambient compiler-flag overrides. Inner Cargo/test concurrency remains
one per worker. Ordinary lanes retain their original task limit. No Windows
compiler flag, hardening requirement, assertion, exclusion or timeout changed.

The actual frozen replay is
`/tmp/nevrotick-service-two-owner-parallel-20261107-v1/verification.json`, using
`/tmp/nevrotick-service-candidate-20261107-v4`. All 77 variants passed the same
accounting: lifecycle 32 behavioral kills/21 selected-source compiler failures;
SCM 21 behavioral kills/three selected-source compiler failures. Both healthy
baselines executed all 28 ordinary tests and six doctests. There were no misses,
timeouts or exclusions. Raw metrics remain distinct from independent review.

The external observer receipt at
`/tmp/nevrotick-service-parallel-resources-20261107-v1/resources.json` records
61.529 seconds for the complete invocation, including admission. At nominal
0.1-second sampling it observed peaks of 18 processes, 72 threads, 1,690,184 KiB
summed RSS and 2,216,076 KiB maximum individual virtual size. Minimum host
available memory was 8,266,432 KiB (about 7.88 GiB); minimum free disk was
2,287,112,192 bytes (about 2.13 GiB). PID/start-time and namespace identities are
retained. These are sampled observations, not guaranteed peaks; summed RSS
counts shared pages more than once and host headroom includes unrelated work.
No cleanup was needed for this run.

This supports the selected four-worker cohort under its actual conditions; it
does not establish eight-worker capacity, a general speedup or the cause of the
earlier 128-task linker failures. The older 73.499-second number measures the
serial mutation commands only, while 61.529 seconds includes admission; do not
present them as a controlled timing comparison.


#### 4.34.8 Product identity and provisioning gate

**Disposition: Researching — OPEN-3/12/14; owners:**
[W11-W05](#w11-w05-observe-effective-scm-maintenance-and-process-policy)
platform/security/lifecycle and
[W11-W10](#w11-w10-re-enter-conditional-mechanisms-only-for-a-concrete-need)
package/release. The portable and laboratory checkpoints do not resolve this
product gate. [Architecture §4.1](../docs/windows-time-architecture.md#41-selected-account-topology)
requires two own-process services. Its virtual-broker account example belongs
to the retained MSI design; neither a supported MSIX mapping nor two
LocalService services is an adopted, qualified substitute. Final SCM names,
SIDs and per-role configuration remain pending with the existing
[metadata owner](../metadata/product.toml), including `services.identities`.

The effective engine token must lack `SeSystemtimePrivilege`,
`SeDebugPrivilege`, `SeImpersonatePrivilege` and unnecessary process/token rights;
present-but-disabled is insufficient. The broker retains
`SeSystemtimePrivilege` plus demonstrably necessary rights, with the documented
OS-retained `SeChangeNotifyPrivilege` exception. Required-privilege allowlists,
service-SID restrictions, object ownership/ACLs and creation-time process/thread/
token protection remain separate configuration and effective-state obligations.
Descriptive metadata applicability does not make these controls optional.

The [pre-start gate](../docs/plans/windows-msix-store.md#75-pre-start-provisioning-authority-and-admission--5-november-2026)
still needs both the intended helper's necessary rights on deployment-created
objects and supported unattended invocation/reentry through servicing. Elevation,
SCM Running, a saved admission marker or a later descriptor fix supplies neither
guarantee. The [selected package profile](../docs/plans/windows-msix-store.md#71-concrete-source-inventory-and-manifest-profile)
excludes `localSystemServices`; a broker launching a restricted engine child is
unselected and changes the two-SCM-service topology. No mandatory first-UI setup
or account broadening follows from the current implementation goal.

**Permitted continuation:** finish the real source hosting path; prepare typed
fail-closed admission, read-only token/configuration observations, fixed
per-service records, bounded diagnostics and servicing/allowed-denied controls
within those contracts. The [initially disabled services plus fixed elevated
helper](../docs/plans/windows-msix-store.md#provisioning-candidates-and-decision-boundaries)
remain a candidate requiring evidence, not an adopted working bridge. Keep
normal product hosting dependent on that bridge. If concrete evidence makes an
extra LocalSystem bootstrap, child-process topology or changed activation UX
necessary, bring the bounded choice to David through
[NV-DEC-001/OD-04](../docs/nevrotick-owner-decisions.md#6-od-04--decisions-when-evidence-makes-them-concrete).
Do not ask him to approve an unproven platform fact. No owner decision or
Windows/native, fuzz, storage or formal execution disposition changes here.


### 4.35 S0.2 fixed-role laboratory host source card — 7 November 2026

**Source-only authority:** the existing service start/full-goal request admits an
inert S0.2 source prototype with real callers. It does not admit ordinary product
hosting before supported MSIX provisioning/account isolation, or lift the current
Windows execution stop. No Windows command, package installation, service
creation/start/stop, privilege change, W32Time/clock effect or public network is
authorized by this card. Source preparation and host-only/cross-compiler checks
remain separate from later native observations. The product goal stays complete
in scope and incomplete in status.

**Selected bounded source definitions (laboratory only):**

| Property | Engine composition | Broker composition |
| --- | --- | --- |
| Fixed SCM fixture identity | `NevroTick-Lab-S02-Engine` | `NevroTick-Lab-S02-Broker` |
| Role source | Distinct compiled engine entrypoint | Distinct compiled broker entrypoint |
| Startup/stop pending hint | 1000ms in each phase | 1000ms in each phase |
| Fixed startup/stop deadline | 10000ms per phase, never extended by progress | Same |
| Maximum laboratory Running duration | 60000ms; finite self-stop | Same |
| Effects | Status/control-only laboratory host | Same; no actual clock backend |
| Account, service SID, privileges, package provisioning | Unselected/unqualified; no source-side install/configuration action | Same |

These limits select only a finite laboratory experiment; they are not production
defaults, correction budgets or a new timing/security policy. Technical fixture
identities do not assign final product SCM names or SIDs. Metadata product
identities remain with their existing owner. Process/SCM arguments cannot select
role, identity, roots, endpoints or clock authority. Extra SCM start arguments
are rejected by count before pointer traversal/string allocation; metadata-only
argument zero is ignored. No debug/console fallback can qualify SCM behavior.

**Implementation owner/path:** the standalone experimental
`tools/windows-service-fixture` owns two fixed-role caller compositions and
private native hosting mechanics, consuming the checked `windows-platform`
lifecycle/SCM model. It is not a new production workspace member or a second
maintained product adapter. Qualified reuse/adoption later must preserve one
owner and the package/account gate; dormant raw-handle wrappers are not an
integration substitute. Existing root pure-library unsafe prohibitions remain.
The experimental workspace mirrors adopted lints/profiles; reviewed unsafe is
confined to its Windows module, with per-operation safety and error-capture
contracts. The proposed Windows-only binding edge is existing exact
windows-sys0.61.2/defaults-off with Foundation+System_Services, no new registry
identity, wrapper, installer, runtime or privilege dependency. Inspect/admit that
actual standalone graph before resolution/build.

**Real source flow and required outcomes:** fixed process entry -> exactly one
SCM dispatcher with one service row and null terminator -> fixed-role ServiceMain
-> immediate bounded registration -> private non-cloneable status token ->
existing lifecycle -> checked seven-field projection -> native status call.
Native errors are captured immediately; terminal tokens are invalidated before
the sole Stopped attempt and never CloseHandle'd or reused. The callback retains
only process-lifetime bounded notification custody and never formats, allocates,
logs, calls application code, reports status or mutates lifecycle. A selected
standard-thread notification/finite parking arrangement needs source support for
its callback contract; no Win32 wait API is adopted by assumption. Owner checks
Stop/time after every spurious wake. Unexpected ServiceMain panic cannot cross
FFI or resume work; inert-fixture abort selects no shipping broker fault policy.
Fixture Running describes only this control loop, never production readiness,
identity isolation, authenticated time, IPC availability or clock authority.

**Source exit before any future permission request:** exact role/budget/argument
and status contracts; healthy and injected dispatcher/register/status/wait/panic
ordering cases with valid neighbors; single terminal cleanup and coalesced Stop;
owned callback/thread lifetime; non-Windows refusal; actual Linux unit/contract,
strict lint/rustdoc and admitted Windows-target compilation/link evidence on the
frozen source/graph. Native calls/tokens/callbacks/package behavior remain NOT RUN
until a concrete separate disposable-machine card is prepared and David lifts
the relevant hold. The existing W11-W05/W11-W10/S0.2 and CHECK-1/27 owners retain
actual identities, mitigations, resources, recovery and two-family observations.

#### 4.35.1 Source implementation and qualification checkpoint

The standalone fixture now contains both fixed executable entries, the actual
shared host and narrow Windows SCM calls. Offline GNU/MSVC graph admission is
retained in `/tmp/nevrotick-service-fixture-graph-20261107-v1/graph-admission.json`:
two local packages and 16 existing registry identities, with no new versions,
checksums, licences or execution roles. Only the tool's resolved lock was copied
back. Its SHA256 is
`12f770f2eebb343c19b6c962bda9c367115daad4c88bfad1aebd734bad1e330f`.
The root product workspace and account/provisioning decisions are unchanged.

Observed source regressions and controls remain separate from native evidence:

| Frozen evidence | Actual result and causal repair |
| --- | --- |
| Graph-v1 `source-red.json` | 14 host tests: 12 passed; late Stop hid run expiry and persistent reversed time stranded Running. Both repaired. |
| Fixture-source-v3 `source-test.json` | 19 host tests: 18 passed; a dispatcher error was hidden by an existing successful owner receipt. Separate failure custody repaired this; the owner receipt remains immutable. |
| Fixture-source-v4 `source-test.json` | 21 host tests plus the actual GNU executable refusal test passed, including healthy/aborting children of the production-used panic guard. Windows-target check and strict rustdoc passed. Formatting/Clippy failures remained explicit. |
| Fixture-source-v5 `source-test.json` | 22 host tests: 21 passed; all six Ready-delay cases exposed an incorrectly renewed Running budget. One absolute Ready-anchored deadline now governs work admission and waits. |
| Fixture-source-v6 `source-test.json` | 22 host tests plus the executable refusal test passed; GNU Clippy passed. One formatting wrap remained. |
| Fixture-source-v8 `source-test.json` | 23 host tests: 22 passed; exact/late stop finalization omitted the Failed terminal report. The existing pending deadline is now checked before selecting the sole terminal outcome. |

Each `fixture-source-vN` prefix above denotes
`/tmp/nevrotick-service-fixture-source-20261107-vN`. The earlier source-v2
harness refusal occurred before compilation because its work directory was not
private; the replacement explicitly creates mode0700. It receives no behavioral
RED credit. Formatting/compiler refusals likewise receive no behavioral credit.
All first failures, source maps and recipes remain retained.

The next frozen source is v9, with 435 bound inputs at
`/tmp/nevrotick-service-fixture-source-20261107-v9/source.json`. It contains 24
host tests, including advance-then-partial-rollback and explicit pre-work read
counts. The latter observes refusal before entering the loop, rather than only
the eventual identical failure. Current host SHA256 is
`253c9d09d2eadaaaed7fb84cb713e5a75737d80726fb669976d93bf87c45e869`;
host-test SHA256 is
`7d560f4282f06c8ac678fe682d6a2dd54e32d021d3550a9633f5d95c3f0596a4`.
**V9 execution, complete source-exit qualification and host coverage/mutation
remain pending.** A review question about preexisting Stop combined with startup
expiry also remains unexecuted; resolve it against the existing phase contract.

The retained source checker is
`/tmp/nevrotick-service-fixture-check-20261107-v9.py`. Next run its formatting,
ordinary tests, GNU/Windows Clippy, doctest/rustdoc and optimized lanes. The
external `/tmp/nevrotick-service-fixture-cross-link-20261107.py` supplies explicit
final-freeze binding, cross-link and PE inspection with the full v3/hardening
vector. Its first cross-v1 binding used superseded v6 and performed no build;
bind fresh final bytes before link/inspection. Image declarations do not qualify
runtime enforcement, installed identity, native callbacks or complete packaging.

The completed four-worker replay received independent raw-outcome review: all
77 outcomes, exact worker/task commands, 428 captured inputs, tool bookends,
53 behavioral causes and 24 selected-source compiler causes reconciled. Do not
rerun this unchanged portable cohort solely because the separate host advances.
Native host function/guard generation reconciliation and applicable per-file
gates remain separate future evidence; no host score is inferred from that run.

Under the owner's cleanup authorization, inactive ordinary `.rlib`/`.rmeta`/`.d`
caches from earlier owned runs were removed. The receipt is
`/tmp/nevrotick-service-cache-cleanup-20261107-v1.json`: 454 cache paths removed,
1,107 retained neighboring file hashes unchanged, about 170 MiB net free-space
increase over the measured window. Sources, reports, executables, build logs,
coverage artifacts, active checkout targets and unrelated services were retained.
An expected hardlink ctime change stopped the first deletion pass; its exact
same-inode cause was checked before resuming by inode group. Two non-dumpable
user-manager processes were documented without intervention. Git custody matched
across cleanup, including the original index and staged diff.

Subsequent external staging changed the index from `92902e18...` to
`633197e6778ae9cc7387995da6f7af96ee86fd305a51f79e9bf7c16be109ac62`.
V9 binds the latter and all its source bytes still match. Preserve that staging;
the current host/test repairs remain unstaged above it. No coordinator staging,
commit, push, native execution or service installation was performed.

#### 4.35.2 Current source, image and metric evidence

The startup-expiry review question above became an actual regression in
`/tmp/nevrotick-service-fixture-source-20261107-v10/source-test.json`:
24 of 25 host tests passed; exact/late startup expiry with preexisting Stop
omitted Stopped. The host now checks StartPending at its existing fresh stop
observation before replacing that phase's deadline. It preserves earlier errors,
adds no clock read and still attempts terminal reporting once. The preceding
stop-finalization repair and advance/rollback/read-count controls passed in that
same RED run. Formatting/documentation and Windows FFI lint repairs are separate
from behavioral RED credit.

Current frozen source is
`/tmp/nevrotick-service-fixture-source-20261107-v14/source.json`, with 435 inputs.
Host SHA256 is
`5f65bd0d498395eb9b5ba59953e02e174a49038e7c73eeb684f06f146377bd72`;
native adapter SHA256 is
`8c9d11fd383707894b37fd8d8bd03d41bc3de51319087baf0b93044fce62c493`.
All seven source-check lanes passed on v14: formatting, ordinary 25 host tests
plus one actual GNU executable-refusal test, GNU Clippy, Windows-target Clippy,
strict rustdoc, stable doctest execution (explicitly zero examples), and the same
26 tests under `verify-release`. The panic guard's isolated healthy/aborting
children ran with core dumps disabled. These remain GNU behavioral and Windows
cross-compilation evidence. Independent source review found no remaining
material defect within the declared laboratory hosting scope.

Both fixed Windows images cross-linked under `verify-release`. The exact
v3/hardening vector was checked in each final rustc invocation. Receipts are
`/tmp/nevrotick-service-fixture-cross-20261107-v4/cross-link-result.json` and
`pe-inspection.json` in that directory. Raw PE/LLVM inspection observed AMD64
PE32+, relocation data, ASLR/DEP/CFG declarations, nonempty CFG tables,
DependentLoadFlags `0xA00`, both CET declaration bits (`0x3`) and CodeView records.
Engine executable SHA256 is
`0b52116d75a887245a4d6a6b518e803dd79bf16279b713473b960cffdd0a6fb3`;
broker executable SHA256 is
`324b170cf139ba0e62d4383f4c02e4b83f9ce07915f171700cb501e5c6f3c330`.
PDB bytes/hashes are retained; independent PDB GUID readback is not performed.
No PE was executed, signed, packaged, installed or submitted. These declarations
do not establish effective process mitigations, account isolation or service
qualification.

The first actual host coverage run is retained at
`/tmp/nevrotick-service-host-coverage-20261107-v1/verification.json` and
`work/coverage.json` there. It executed all 26 tests. Host lines were 157/162
(96.91358%) and library lines 31/31. The command correctly failed its unchanged
per-file 90% gate: `error.rs` was 19/37 (51.35135%), and each binary entrypoint
was 4/5 (80%). Improve meaningful diagnostic-contract tests; keep the native
success branches explicitly pending. Do not hide production lines, fabricate a
console-success mode or infer a package pass from the host row.

Actual host enumeration produced 24 descriptors at digest
`bae824489a81c0dc8575fb872649ebde64484caa668c9eb6876bfef171776eb5`.
The independent review at
`/tmp/nevrotick-service-host-inventory-review-20261107-v1.json` admitted a bounded
four-worker/512-task campaign: all coordinates/diffs match, eight of nine concrete
bodies are generated, and `ObservedClock::new` is omitted. Constructor, closure,
deadline, permit, first-error and fixed-literal omissions remain separate
semantic obligations with zero generated credit. The first campaign attempt
stopped before any tool job because staging had advanced since the source
freeze. Its refusal supplies no mutation score. The immutable source still
matches; analysis-epoch staging custody is being separated from original-freeze
provenance without weakening source/tool/configuration checks or per-job custody.

David confirmed the workspace-member removal was a temporary test edit and
authorized reverting needed code/docs. The coordinator restored only the
`windows-platform` member and corresponding root lock entry, recovering their
original `ed63de18...`/`fa9103e4...` hashes. Unrelated Q1 test strengthening and
staged content were preserved. Source-v12/13 held later work when staging moved;
v14 binds the then-current staging. No coordinator staging/commit/push followed.
The second authorized cache cleanup removed only inactive compiler/procedural-
macro/build-script artifacts, preserving 1,713 neighboring hashes and staged/
working content; its receipt is
`/tmp/nevrotick-service-cache-cleanup-20261107-v2.json`, with 530,280,448 bytes of
observed net recovery. Later global free-space changes are not attributed to it.

Next: finish the frozen host campaign and independent cause/omission review,
improve diagnostic tests, retain native coverage/entry/SCM/resource and packaging
gaps, and continue the permitted admission/configuration work under §4.34.8.
Neither this source checkpoint nor the full two-service completion goal is
complete. All native, fuzz, storage and post-V1 formal dispositions remain.

#### 4.35.3 Host mutation reconciliation and diagnostic continuation

The actual Source14 host campaign completed with 24 descriptors: 11 behaviorally
caught and 13 compiler-unviable, with no survivors, timeouts or exclusions.
Compiler failures receive no kill credit; the viable denominator is 11. The
original v2 receipt remains incomplete because its post-processing rejected
secondary unused-item diagnostics outside `host.rs`. A separately identified
v3 offline replay reconciled those diagnostics only where each variant also had
its primary error anchored in the mutated host source. No Rust job was rerun or
original receipt rewritten. The independent receiving review accepted this
bounded numeric result; it does not establish whole-fixture qualification.

Retained receipts are
`/tmp/nevrotick-service-host-reconciliation-20261107-v3/verification.json`
(SHA256 `84475fdeb40850afa60faa229c0f3311c0db26e2704bbbf2ea1f2f43b61279d2`)
and `/tmp/nevrotick-service-host-receiving-review-20261107-v3.json`.
Original-freeze index provenance remains immutable. Subsequent commands bind
their current analysis-epoch index before and after each job; staging changes
between commands require fresh admission, rather than rewriting historical
provenance or dropping source/tool/configuration custody.

Five independent source-omission controls also passed on Source14. Each compiled,
passed its healthy neighbor and failed at the intended assertion: initial clock
reversal, omitted last-trusted update, omitted pre-work Running guard, omitted
StartPending expiry before Stop, and omitted terminal stop-deadline check. The
seven-test healthy baseline and five neighboring controls passed. These are
separate semantic evidence, with zero generated-mutation credit. All 23 jobs and
their source/diff/log/index identities are retained in
`/tmp/nevrotick-service-host-omission-evidence-20261107-v1/verification.json`
(SHA256 `13c7c540d05177c8e8b3e1cd04ac32cd54d77e8225f59fd70a01257b1a764ab8`).

Five diagnostic tests have now been adopted in `error_tests.rs`, exercising fixed
ASCII/one-line/64-byte categories independently from original numeric codes,
all fixture error categories, and actual budget/lifecycle/native typed causes.
This changes the test cohort to 30 library tests plus the actual GNU executable
refusal test. Source15 freezes 436 inputs at
`/tmp/nevrotick-service-fixture-source-20261107-v15/source.json`. All seven fresh
source lanes passed: ordinary and `verify-release` 31-test cohorts, formatting,
GNU/Windows-target Clippy, strict rustdoc and stable doctests (zero examples).
Each `source-<lane>.json` receipt records unchanged per-job index bytes and passed
source/tool/configuration bookends. The producer/freeze provenance is retained
separately from the continuation checker and each current analysis epoch.
Fresh coverage at
`/tmp/nevrotick-service-source15-coverage-20261107-v1/verification.json` executed
all 31 tests successfully. Its raw production rows show `error.rs` 37/37 (100%),
`host.rs` 157/162 (96.91358%) and `lib.rs` 31/31 (100%). Each executable remains
4/5 (80%): the unchanged per-file 90% tool gate returned exit 1, and the receipt
correctly remains incomplete. The diagnostic-file gap is closed for this GNU
cohort; the earlier failure and all raw rows remain retained. Native-only binary
branches are neither excluded nor replaced with a console-success path, and no
whole-fixture coverage or native pass follows.

The separate receiving assessment is
`/tmp/nevrotick-service-source15-coverage-20261107-v1/coverage-assessment.json`
(SHA256 `b7a03c1043fe2cfcb2d2e26cc31187a62028d13d68d684b50ce187596c65da41`).
It independently rechecked source, tools, vendor, configuration and index after
the expected gate failure, preserving the raw incomplete receipt. Its remaining
owner map retains unenumerated/unqualified 95% mutation obligations for
`lib.rs`, `error.rs`, the native adapter and both executable entrypoints. The
host campaign remains historical Source14 evidence on identical host bytes;
neither new coverage nor the five semantic controls creates new mutation credit.

The next-candidate resource preflight had about 2.3GiB free. Authorized cleanup
v3 removed 722 inactive ordinary compiler/procedural-macro/build-script cache
paths from completed source-check targets, retaining test binaries, reports,
source and coverage evidence. All 1,532 retained-file hashes and staged/working
Git content matched after cleanup. The measured net recovery was 608,223,232
bytes, leaving 3,048,677,376 available bytes at that observation; receipt:
`/tmp/nevrotick-service-cache-cleanup-20261107-v3.json`. No running process,
checkout, installed tool, database or Windows resource was removed.

**Authority refresh:** concurrent documentation commits advanced HEAD to
`75ccd769fc998cf8290680a5b7dd3e4a6d38e892`. Reconciliation with development §7,
its imported queue and OD-16/17 preserves this service source continuation:
WinGet capability preparation belongs to W11-W10/P6 before V1, its native and
public-availability evidence remains open in §4.36, and MT-0 remains deferred
until actual V1 release. These amendments change neither the frozen compiler
inputs above nor the service provisioning and execution holds.

#### 4.35.4 Next source slice: fixed-role self-token observation

**Owners and requirements:** W11-W05 platform/security/lifecycle under
REQ-1/2/15, architecture §3.4/4.1 and CHECK-1/27; bounded diagnostic projection
supports CHECK-17. W11-W10 retains the unresolved package/provisioning gate.
This card admits source preparation only under §4.34.8, with all execution holds
unchanged. It selects no service account, product identity or privilege change.

The smallest next real caller path is registration → StartPending → fresh pending
guard → synchronous self-token observation and actual cleanup → fresh deadline/
Stop guard → existing Ready. Its checked result is consumed in this startup
attempt, not cached as an authorization capability. Engine subset acceptance
requires complete enumeration showing `SeSystemtimePrivilege`, `SeDebugPrivilege`
and `SeImpersonatePrivilege` all absent. Present-but-disabled and unknown fail.
Broker observation is descriptive and establishes no broker allowlist or clock
authority. Incomplete capture/cleanup prevents Ready in either laboratory role.

The unsafe-forbidden shared owner validates a bounded privilege representation;
the existing private native owner queries only its current process/thread token,
with no arbitrary PID, name, handle or caller-selected privilege. Check for an
unexpected thread token around the primary-token capture; only definite
`ERROR_NO_TOKEN` counts as no thread token. This is an immediate observation,
not proof against transient external changes, other threads or future mutations.
Use query-only non-inheritable token custody, fixed local privilege lookup and
one initialized aligned capture bounded to 128 entries (a laboratory limit, not
a Windows maximum). Validate lengths, count arithmetic, target uniqueness and
attributes before constructing the immutable result. Unknown is never absence.

Close each opened handle once before Ready. Failed close remains unsuccessful
cleanup, with possible handle retention until process termination; preserve the
original error, refuse Ready and do not retry or start a replacement observation.
The work permit remains occupied through actual synchronous operation/cleanup,
including Stop or expiry. A deadline does not interrupt an arbitrary native call.

Before adoption, require actual offline graph admission for only the existing
windows-sys0.61.2 `Win32_Security`/`Win32_System_Threading` feature additions,
independent raw-buffer/absent-disabled-enabled/unknown and cleanup cases, and
host controls for observation failure, Stop/expiry during observation and direct
caller bypass. Reuse the configured static/behavioral/cross checks on the new
candidate; review its applicable coverage/mutation inventory. Native handle,
effective-token, latency and ABI behavior remains NOT RUN. This source slice
does not close the provisioning bridge or authorize ordinary product hosting.

#### 4.35.5 Restart and main-repository recovery

David reported restarting Windows after it froze. Fresh inspection at
18:18 UTC on 7 November found no `nevrotick` service directories in `/tmp` or
`/var/tmp`: the temporary recipes, source freezes, vendor, raw verification
reports and unfinished self-token prototype above were lost. Their dated
results remain historical observations recorded in this task and conversation;
the referenced raw receipts are no longer available for current inspection.
Do not reconstruct those receipts as executed evidence or use their absence as
a passed gate. The cause of the Windows freeze is not established by this fact.

The main repository survived. Its existing host/native bytes still match
`5f65bd0d...`/`8c9d11fd...`, and the diagnostic tests, shared lifecycle/SCM source,
workspace restoration and staged work remain. David explicitly corrected the
workflow: work in the main repository. Recover and implement the self-token slice
directly in its real source/test files; keep the existing task as the plan owner.
New raw records are retained under the repository's narrowly ignored
`artifacts/windows-services/recovery-20261107-v1/`, starting with
`restart-observation.json`. Preserve these records during cleanup. Any isolated
execution copies are derived from the main repository and are not the only copy
of unfinished work; existing sandbox and input-admission controls still apply.

The post-restart preflight observed 18GiB available RAM, zero swap use and 8.3GiB
available disk. Re-admit tools/dependency sources and run new candidate-specific
checks after recovery; new records must identify actual fresh executions.
The first new behavioral controls and repair are recorded below. Native/package
qualification and unexecuted critical-owner mutation obligations remain open.
Windows/native, fuzz and storage holds and the post-V1 formal deferral are
unchanged.

#### 4.35.6 Recovered startup observation: actual RED and GREEN

The implementation is now authored directly in the main repository. The new
`windows-platform::privilege_observation` owner has private checked snapshot
construction, mandatory garde-backed count/byte bounds and independent raw-byte
tests. Its security/critical registration retains the existing 90% coverage and
95% mutation requirements; those measurements are not yet executed on this
candidate. The fixture's private `native_token` producer opens only current
process/thread query tokens, resolves three fixed privilege names and consumes
one bounded complete capture. It retains the first failure and both cleanup
outcomes; failed close is not retried or reported as released custody.

Both actual ServiceMain compositions call the explicit observer during
StartPending under the existing work permit. Unknown capture/context/cleanup
refuses Ready for either role. The engine immediately requires all three
forbidden privileges absent, including refusal of present-but-disabled values.
Broker capture is descriptive, without a broker allowlist or clock-authority
claim. Fresh deadline/Stop guards remain after actual observation completion.

The versionable `tools/windows-service-fixture/qualification.py` retains fresh
raw records under `artifacts/windows-services/recovery-20261107-v1/`. Initial
readiness v1 correctly stopped before any command because the default vendor
contained an obsolete extra package. No vendor or pointer was modified. Explicit
selection of an existing exact 533-package store passed complete admission;
readiness v2 and the actual four-command GNU/MSVC metadata/feature-tree graph
passed with source/tool/vendor/index bookends. Registry identities are unchanged;
only `Win32_Security` and `Win32_System_Threading` were added. Exact local target
sets and negative extra-bin controls also passed.

Actual first RED is `recovered-engine-admission-red-v1.json`: compilation
succeeded, the real startup test failed with “engine admitted a present forbidden
privilege,” exit 101, and complete bookends passed without a custody error.
`recovered-role-healthy-v1.json` then passed its actual startup/stop neighbor.
Only the engine snapshot absence predicate was added to the host. A fresh
matching graph and `recovered-engine-admission-green-v1.json` passed on the
441-input `0453cb59935ef7ab9e3881a6b24f5e6da01972fffdabdadde6ab2cc17f808158`
candidate. These are selected controls, not the full test cohort or native proof.

A separate exact two-arm `service_role` swap compiled and failed the independent
ClockBroker-versus-Engine assertion, with complete custody. Its graph and RED
records are `recovered-role-mutant-graph-v1.json` and
`recovered-role-mutant-red-v1.json`. The healthy library bytes were restored
exactly before further tests; this semantic control receives no generated
mutation credit. The unfiltered ordinary cohorts then passed: 39 platform tests
(21 lifecycle, 7 SCM, 3 primary privilege and 8 independent raw-byte contracts)
and 36 fixture tests (35 library plus the actual GNU entrypoint test), with no
failed, ignored or filtered cases and complete custody. Their records are
`recovered-platform-full-v1.json` and `recovered-fixture-full-v1.json`.
Formatting, documentation markup and the stable checked-chunk iteration idiom
were subsequently repaired through actual fmt/Clippy feedback without changing
the exact count/length admission guard. The later
[§4.35.7 checkpoint](#4357-run-deadline-representability-repair-and-metric-scope)
records completed final-source static/documentation and optimized checks;
per-file metrics and native/package obligations remain pending.

#### 4.35.7 Run-deadline representability repair and metric scope

Independent review identified a possible CHECK-27/S0.2 readiness-ordering defect:
the host acknowledged Running before constructing its finite run deadline.
A new actual-host oracle uses a bounded 64-step checked construction and directly
verifies an Instant where the 10-second startup allowance is representable but
the 60-second run allowance is not. Both roles also exercise a valid 60-second
neighbor and simultaneous exact startup expiry, with all six traces collected
before comparison. Expected failure has no Running row, control-loop work or
wait, and one failed terminal report; startup expiry retains priority.

`recovered-overflow-red-v1.json` compiled successfully, passed the previous 35
library tests and failed the new oracle. Both overflow traces returned
ClockOverflow but falsely included Running. Healthy and startup-expiry neighbors
matched independently expected outcomes. Complete source/tool/vendor/index
bookends passed. The host now checks the existing pending deadline, constructs
the checked run deadline, then publishes Ready using the same observation.
Capture, cleanup, permits and raw-clock read counts are unchanged.
`recovered-overflow-green-v1.json` passed all 37 fixture tests (36 library plus
the real GNU entrypoint case), with zero failures, skips or filtered cases and
complete custody, on candidate
`2f492f0e6e7145052586e7ec076fdd4b0cfed5ada51faa7340c396c416216e5b`.
Exact executed before/after sources are retained in the main repository's
`artifacts/windows-services/recovery-20261107-v1/semantic-controls/`.

The existing portable service-analysis wrapper now requires schema 5 with exactly
lifecycle, SCM and privilege owners. Common's historical schemas 1–4 retain
their original scopes. Current candidate binding and descriptor inventories were
cleared rather than borrowing old measurements. The 90% per-file coverage,
95% per-file mutation, four outer workers, one inner worker, 512-task campaign
profile and combined 512-descriptor limit remain unchanged, with no exclusions.
All 21 live service-wrapper and 144 Common Python controls passed after the
five-file adoption; `schema5-live-controls-v1.json` and logs retain execution.
This is analysis admission/control evidence, not a product mutation score.
On the repaired frozen candidate, fresh ordinary platform 39 / fixture 37 and
actual `--release` platform 39 / fixture 37 cohorts all passed with zero failures,
skips or filtered cases. All three Clippy targets, both formatting checks,
platform eight doctests, fixture zero-doctest execution and strict fixture
rustdoc also passed. The `recovered-qualified-<lane>-v1.json` records retain
per-job source/tool/vendor/index custody; `recovered-overflow-green-v1.json`
owns the current ordinary fixture cohort. Optimized commands use `--release`
with debug assertions disabled, not `verify-release`.
Actual per-file measurements, Windows artifact and native/package qualification
remain open. Windows/native, fuzz, storage and formal holds remain.

The 7 November metric checkpoint bound the then-current 421 portable product inputs at
`ff8148597f0950d46e50b7986a5b318f8ea5fb61dc2e10c507242e9697a65346`,
the exact protected helper inputs, 39 actually executed ordinary/release names
and eight executed doctests. At that checkpoint, `read_policy(..., campaign=False)` passed;
`portable-binding-v1.json` retains the full map and source-checkpoint lineage.
Descriptor inventories remain empty: this admits enumeration/coverage, not a
mutation campaign or a metric pass. New primary records stay under the main
repository's `artifacts/windows-services/recovery-20261107-v1/`.

#### 4.35.8 Service source handoff and integration prerequisites — 8 November 2026

**Disposition:** implemented source foundation; ready for bounded source
integration preparation. Qualified production-service adoption and installed/native
integration remain incomplete. W11-W05/platform/security owns the S3.1 service
child; W11-W08 owns reached diagnostic behavior and W11-W10 owns the necessary
account/provisioning interface. Keep this task as the single execution/evidence
chain. NTP/NTS algorithms, clock discipline/journal algorithms and full MSIX/Store
delivery belong to their existing workstreams, not this service child.

**Available implementation:** commit `89f035e2233e6d86d6d29319b0c4b29e0f667fe7`
contains the portable `windows-platform` lifecycle, SCM projection and private
checked privilege snapshots, plus two standalone fixed-role laboratory hosts.
Their actual source connects executable entry → dispatcher → ServiceMain → handler
registration → serialized startup/token observation → checked status/Stop owner.
The hosts still execute an inert control loop with laboratory deadlines; neither
is a production engine or broker service. Reuse one hosting owner and its checked
types/custody rather than introducing another wrapper or parallel implementation.

**Evidence reviewed:** local `source-checkpoint-v2.json` under
`artifacts/windows-services/recovery-20261107-v1/` retains the 441-input candidate
`2f492f0e...`; all 13 referenced receipt hashes still match. Recorded normal and
release cohorts passed 39 platform and 37 fixture tests, with eight platform
doctests, formatting, GNU/Windows-target Clippy and strict fixture rustdoc.
At handoff entry, all 23 service/fixture input bytes matched that checkpoint;
this documentation edit subsequently updates the fixture README only within that
set. Service Rust, manifests, lockfiles and the qualification recipe are unchanged.
Three concurrent engine-series source changes also mean the whole 441-input
freeze and portable 421-input `ff814859...` binding no longer describe the complete
current checkout.
Retain these scoped results; refresh the relevant candidate before dependent jobs.
No new Cargo, analyzer or Windows execution was performed for this documentation
handoff. The raw records are local retained evidence, not committed artifacts.

| Next stage / owner | Entry and next action | Independent observable exit / current limit |
| --- | --- | --- |
| Source integration preparation — W11-W05/platform/security | Available now: map existing role-specific hosts and portable APIs to one S3.1 adapter/worker startup/status/stop child. Freeze inputs, readiness meaning, failure results, actual-completion custody, cancellation and cleanup before implementation. | A reviewed producer/worker/status/stop ownership map and finite direct-call, healthy, denied, boundary and fault controls; no new account, production deadline, runtime or effect authority inferred from the lab. |
| Qualified source adoption — platform/QA | Refresh ordinary candidate/bindings; finish current >=90% per-security-file coverage and >=95% per-critical-file mutation with complete reviewed inventories, guard/derive alternatives and applicable host/native-source scopes. Complete current Windows cross-link and independent image readback. | Candidate-bound source/contract/metric and static artifact evidence. Current mutation inventories are empty; no fresh metric pass or current Windows link/PE pass exists. The latest SDK readiness attempt is incomplete after index drift. |
| Product service composition — platform with engine/broker/IPC/diagnostic owners | Consume qualified adapters and explicit worker/handler contracts; final SCM identities, intended tokens/SIDs, enforcement of [the selected privilege profiles](../docs/windows-time-architecture.md#411-service-privilege-profiles), creation-time object protection and supported pre-start provisioning must be resolved before ordinary installed hosting. | Truthful availability, enforced startup admission, bounded diagnosed stop/restart and management isolation. At this historical handoff, engine observation checked three forbidden privileges and broker observation was descriptive. [§4.38](#438-service-privilege-profile-handoff--9-november-2026) now records complete portable capture, both-role admission and its selected qualification. Native enforcement and product diagnostics remain separate. |
| Native service qualification — W11-W05/security/QA | Existing Windows hold must be lifted separately; use admitted disposable fixtures and intended roles on the adopted two-family matrix. | Actual SCM callbacks/status failures, effective token/object isolation, native completion, resource retention and failure/recovery behavior satisfy CHECK-1/27 and reached CHECK-17/21 beside valid controls. Host injections and compiler checks do not close this stage. |

**Next service action:** the owner-assigned child in [§4.35.9](#4359-w11-w05s31-worker-lifecycle-child--8-november-2026)
now records reviewed contract/source authoring and its blocked verification.
Restore admitted tooling, qualify that candidate and close source-adoption
prerequisites before claiming product integration readiness. The current
Core/engine queue is unchanged.
This handoff lifts no Windows/native, fuzz or optional-storage hold and starts no
formal work before V1. Full P3, package delivery and release remain separate exits.

**Independent documentation review:** `docs_handoff_review`, fresh context
(`fork_turns: none`), completed a read-only review of the nine-file documentation
delta against `89f035e`, before this review receipt. It inspected the manifests,
hosting source, canonical account/IPC/lifetime contracts and retained checkpoint;
no required review input was missing. Verdict: **ready for source integration
preparation**, with no material documentation findings. This is no new source,
metric or native qualification. The reviewer's comprehension was:

> The proposed handoff makes the existing service foundation discoverable and directs S3.1 to reuse it. It permits bounded source integration preparation while retaining separate gates for qualified source adoption, installed product hosting and native Windows qualification. NTP/NTS algorithms, clock discipline and complete Store delivery remain with their existing owners.

> The current system has seven product-workspace members, including portable `windows-platform`, plus two standalone inert laboratory SCM hosts. Their dispatcher, ServiceMain, callback, status and self-token adapter source exists. Historical ordinary/static checks do not establish final identities, provisioning, effective isolation, production worker integration or native behavior.

Documentation checks passed: all 18 added local links/anchors across the nine
Markdown files resolve; `check_skills.py` passes all 66 skills; scoped
`git diff --check` reports no whitespace errors. No application tests were rerun
for this documentation-only change.

<a id="433-winget-distribution-before-v1--7-november-2026"></a>

**Cloud source-integration handoff — 8 November 2026.** Start from the exact
published baseline named in the coordinator's publication message. This is the
bounded W11-W05/platform/security S3.1 adapter-to-worker lifecycle child, primarily
CHECK-27 with reached CHECK-1/17/21/24/25/26 obligations. It prepares source
integration; it establishes no product-service, installed or native readiness.
Reuse `windows-platform` and the existing fixed-role service fixture, keeping one
serialized lifecycle/status/Stop owner and separate engine/broker processes.

**Writable scope:** `tools/windows-service-fixture/src/host.rs`, `host_tests.rs`,
`error.rs` and `error_tests.rs`; `src/native.rs` only for necessary worker wiring
through the existing adapter, preserving its callback, status-token and FFI
custody. If the reviewed worker contract exposes a concrete lifecycle API gap,
record that gap before changing only `crates/windows-platform/src/service.rs`
and `crates/windows-platform/tests/service.rs`. Record the finite contract and
results in this existing subsection. Core/engine algorithms, shared manifests,
lockfiles, dependency/features, policies, verifier scripts and qualification
bindings remain coordinator-owned. Do not add another dispatcher, runtime,
supervisor, privileged command surface or service identity.

Before implementation, name the real worker, its startup inputs, readiness
producer/acknowledgement, actual-completion owner, Stop admission cutoff and
failure cleanup. Running requires acknowledged worker readiness and successful SCM status
publication. Stop accepted by the lifecycle owner closes new work admission;
late results cannot reopen it. Cancellation, timeout and Drop alone prove no actual
completion, release no job capacity and authorize no clean Stopped result.
Retain first failure, separate cleanup failures and the existing explicit failed
terminal outcome after reporting/deadline failure; never retry a terminal status
context that may have been released.

The current `ready()` rejects every outstanding work permit. Freeze whether
startup work actually returns before active work admission, or whether a
long-running worker started before Running needs a reviewed lifecycle API
refinement. A readiness acknowledgement must never call `complete()` for a job
that is still running. The laboratory host currently uses sequential completed
token-capture and active control-loop permits. It supplies neither a production
engine worker nor a broker worker; do not fabricate either or convert laboratory
budgets into shipping defaults. An unavailable worker/provider remains an
explicit source-integration limit.

Use current `AGENTS.md`, `TOOLING.md`, the service/contract/security skills and
configured verification entrypoints. First admit the cloud's exact pinned tools,
vendor and source; local installation pointers, ignored historical receipts and
MSVC SDK inventories may be absent. Recreate admitted prerequisites through the
existing workflow or report the affected check unavailable, without ambient Cargo
fallbacks, floating versions or weaker admission. Run fresh applicable
platform/fixture graph, format, compiler/Clippy, ordinary/optimized tests,
doctests and rustdoc lanes. Use deterministic worker barriers for healthy
readiness, Stop before/during startup, status failure, repeated Stop, pending
actual work, late completion and failed cleanup. Host injections establish their
portable/source scope only.

The coordinator owns current roster/source bindings, independent review,
registration of changed hosting owners and their required >=90% executable-line
coverage/>=95% viable mutation gates. The existing service analysis role covers
only `service.rs`, `scm.rs` and `privilege_observation.rs`; its result cannot
qualify changed fixture hosting code. Preserve complete inventories, generation
gaps, original failures and actual raw results. Deliver the bounded reviewed
source change and honest remaining gates; do not manufacture a whole-service
qualification pass.

Final identities/accounts/SIDs, full privilege admission, supported provisioning,
actual SCM/effective-token/resource/recovery observations, broker clock ownership
and complete MSIX/Store qualification remain with their existing owners. Retain
the Windows/native execution hold, separate fuzz hold, optional-storage pause,
UI holds and post-V1 formal deferral. This assignment authorizes no service
registration/start, clock effect, signing, installation, package publication or
external-agent contact.

#### 4.35.9 W11-W05/S3.1 worker lifecycle child — 8 November 2026

**Active goal:** own the bounded adapter-to-worker startup/readiness/status/Stop
child, reuse `windows-platform` and `windows-service-fixture`, implement its
reviewed source slice, and record exact verification results and remaining gaps.
Original source-authoring baseline was `origin/main`
`1237c072573be36805efe8569ff4e5a652886bce`; branch
`service/w11-w05-s3.1`, separate worktree `/workspace/NevroTick-service`.
Existing uncommitted cloud audit handoffs remain in the main checkout, backed up
under `/workspace/.nevrotick-cloud/service-lifecycle-pull-f6yd87pr`.

**Scope and owners:** W11-W05/platform owns the existing private fixture host
and worker seam; CHECK-27 is the lifecycle exit, with CHECK-1's existing startup
privilege admission and reached CHECK-17/21 error/resource controls. The
coordinator retains Core/engine algorithms, shared manifests/locks and all
verification policies. No production worker, account, runtime, deadline, IPC,
network/clock effect or installed-service authority is selected. Formal work
remains post-V1; Windows/native, fuzz, storage and UI execution holds remain.

<a id="s31-real-engine-composition--20-november-2026"></a>

**Coordinator-selected real Engine composition — 20 November 2026.** The renewed
full-V1 owner goal and [current development selection](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
admit this finite source child after the current Engine receipt checkpoint.
`peer_input_final_judge` independently accepts this scoped scheduling reconciliation
and contract preparation; it does not lift native or security holds. The old
inert implementation and its qualification remain historical prerequisites.

**Observable exit:** one explicitly selected managed-loopback acquisition through
`SourceHandle::session` → `AcquisitionSession::prepare` →
`PreparedAcquisition::execute`, consumed by the actual `Worker` and portable
`ServiceInvocation`. Preparation constructs one genuine dormant `AcquisitionSession`
on the host, with no acquisition thread or unfinished preparation job. Only after
acknowledged Running and its separate active permit is that owned session moved
to its sole Engine thread. That thread prepares the borrowing plan and publishes
its original `ControlHandle` through the same sticky cancellation gate before
executing. No socket/runtime/send is started before Running. No automatic retry or recurring acquisition is
selected. Test peers are owned loopback only; no shipped plain-NTP fallback,
positive time eligibility, mapping, broker authority or production service is
inferred. Initial composition is compiled/tested in the existing standalone
fixture; native.rs, both executable defaults and installed identities stay outside
this stage. Later native/production integration must consume the same checked
kernel, rather than treating this stage as completion of those exits.

**Contract revision 3, independently accepted before source editing:**

1. Retain at most one acquisition thread, one original control, a bounded result
   slot, one host wake and one activation/cancellation gate. Before activation,
   retain the actual checked session without spawning a thread. Activation moves
   it exactly once; failed spawn retains a typed failure and starts no work.
   Sticky cancellation and control publication share one serialized gate, so a
   Stop between activation and control publication cannot be lost. The Engine
   thread alone prepares/executes/retires its moved session. Stop fences host admission before
   requesting that original control. Result publication and a wake precede thread
   return; neither proves physical exit. Only `is_finished` followed by actual
   `join` permits the worker's Finished observation and release of its permit.
2. Distinguish pending work, a retained return marker awaiting thread exit, and
   joined completion. Resolve the wake-before-thread-exit race with a narrow
   **20 ms fixture-only join-tail observation**, using the existing owner's wait;
   no new thread, runtime, supervisor, global timer policy or heartbeat while
   acquisition remains active. At most one such wait is armed. Before expiry,
   clamp it to the original remaining phase deadline. After expiry, retain the
   overdue failure and allow only join-tail observation at that same bounded
   timer request; it grants no new deadline, progress checkpoint, work, Clean
   outcome or time-to-completion promise. Use a distinct private `InvocationPoll::JoinTail`
   observation so this cleanup wait is not mistaken for ordinary deadline allowance.
   The 20 ms value bounds the requested wait, not a minimum polling interval,
   measured wake rate, latency or CPU guarantee: the wait may return early or
   spuriously. Such observations never renew the original deadline or count as
   progress. A stalled operation without a return marker stays event-driven.
   Native termination/recovery and performance remain open.
3. The private Worker finish interface becomes a closed `WorkerFinish` observation:
   `Pending`, `JoinTailPending`, or `Finished(WorkerCompletion)`. The completion
   contains named `operation_failure` and `cleanup_failure` optional fixed
   `WorkerError` facts. Existing `Poll<Result<(), WorkerError>>` fixture adapters
   map their old sole error to cleanup, preserving their meaning. A joined
   completion carries separate optional operation and cleanup failures.
   Preserve the first already observed cause. If it is already occupied, retain
   a later operation failure and later cleanup failure in separate fixed slots,
   beside existing StopPending and terminal-report failure slots. Never overwrite
   a slot or encode a known operation failure as successful work. Existing error
   getters keep their meaning; a new bounded getter exposes the secondary
   operation fact. The actual acquisition result remains owned and inspectable
   without formatting arbitrary Engine error content.
4. Required diagnostics retain existing IDs 1–8, nine-byte encoding and two
   sixteen-record routes. Add fixture event **9**, operation failure, with the
   existing fixed worker cause codes and ERROR severity. It routes once per
   retained operation-failure slot even when optional output is disabled. Existing
   cleanup event 5 retains its meaning. Sink full/unavailable cannot alter status,
   failure precedence, permit custody or cleanup. No arbitrary text/secret fields
   or production logger schema is introduced.
5. Successful one-shot completion leaves the laboratory invocation Running until
   requested Stop or its original run bound. Stop before activation yields zero
   acquisition; failed Running publication never activates. Stop during work
   reaches the real Engine handle. Late completion resolves only the original
   owner and cannot reopen admission. Drop/timeout/cancellation release no charge;
   unresolved whole-owner destruction retains the existing fatal fixture guard.

**Revision-3 independent review resolutions:**

- Every ordinary preparation, execution or gate-publication failure must publish
  a retained terminal marker and owner wake. Use a non-panicking final notifier
  independent of the result mutex. A poisoned gate permits cleanup/cancellation
  and historical inspection only, never resumption or new work. Unexpected Rust
  unwind in the acquisition thread uses the existing fixture unwind-to-abort
  guard, rather than silently terminating the thread. It produces no Finished
  acknowledgement and no clean Stopped report. Abort, native faults and panic-hook
  privacy retain the existing explicit limitations; child controls need finite
  supervision and no core dumps.
- Classify the genuine result before projecting worker health. Outer Err is an
  operation failure. For Ok(report), retain its complete original series: a primary
  failure, Stale, or Failure without any recorded cleanup cause is an operation
  failure; each stop/invalidation cleanup cause is a cleanup failure. Failure
  caused solely by those cleanup facts does not invent another operation cause.
  Completed, Stopped and SeriesDeadline without recorded causes are legitimate
  terminal neighbors, not freshness or synchronization claims. Missing numerical
  qualification remains data, not successful time admission. Gate/publication
  failure is separately retained as worker failure without rewriting that report.
- A known operation or cleanup failure observed while Running requests host
  shutdown promptly. Use a private tagged `WorkerFailure::{Operation,Cleanup}`
  health error so its origin is retained. Finished still carries both canonical
  facts; the host deduplicates the same previously observed kind/code and retains
  a distinct late operation/cleanup fact in its separate fixed slot. Completion
  never depends on a caller remembering to observe health first.
- Reversed-clock handling remains sticky and performs no further Clock reads,
  ordinary work or deadline calculation from the historical fallback. The inert
  worker's existing no-wait completion is unchanged. For this real-thread child
  only, a retained terminal marker permits the same join-only relative wait via
  the existing wait primitive even when time is unusable. Its duration is a fixed
  cleanup observation request, independent of the rejected Clock/fallback, and
  supplies no trusted current time, timing/rate guarantee or new allowance.
  Update the nearby host/README contract consistently; this exception provides
  no native clock-recovery or product termination policy. Actual join is still
  required, and the earlier clock failure prohibits Clean.

The initial independent review returned REVISE for these three details. The
completed inherited-context `peer_input_final_judge` re-review accepted all three
explicit resolutions as READY. The concrete interfaces below are frozen for the
isolated writers; this is contract review, not a source or execution pass.

**Exact source ownership:** coordinator owns the standalone Cargo manifest/lock,
its qualification.py graph/roster admission and shared verification scope. Add only
an existing local Engine dev-dependency (defaults disabled), preserving the admitted
registry cohort, licences, Rust 1.100 and MSVC/v3 target. Host/error/diagnostic
writer owns their existing source and focused tests. A separate writer owns the
private Engine worker module and its real-composition tests after interface freeze.
`src/lib.rs` registers that first-stage module under test configuration. Neither
writer changes native.rs, either binary, workspace manifests, broker/IPC code or
production policies. Register all changed critical owners before metrics; historical
platform gates do not qualify new host/worker code.

**Required evidence:** real-loopback healthy series; denied and pending startup;
failed preparation and Running publication; Stop before/during execution; original
report and no admitted time sample; delayed thread tail before and after the fixed
deadline; wake/check/park race; paired operation/cleanup/status failure precedence;
no Stopped or charge release before join; bounded required-event output with healthy,
full and unavailable sinks. Use independent literal/state expectations and actual
host/Engine consumers. Preserve current configured GNU ordinary/optimized,
MSVC test compilation, Clippy/rustdoc/doctest and per-file 90%/95% gates with complete
inventories/generation gaps. Structured input fuzzing, native memory tools and
other QA follow actual triggers; no held campaign is reopened. One execution owner,
one inner Cargo job/test thread, initial development bound 900 seconds; freeze
metric budgets separately after development and complete candidate admission.

Status: real source composition implemented and development verified below.
Current per-file coverage/mutation qualification is next; the parent S3.1/native
and W11-W01 mapping/broker issuer exits remain open.

<a id="s31-real-engine-development--21-november-2026"></a>

**Real Engine composition — development exit, 21 November 2026.** Two isolated
writers supplied the host and private Engine worker, coordinated at `78f8ae9`.
The integrated path now creates a genuine dormant managed session before Ready,
activates its one thread after acknowledged Running, serializes sticky Stop with
original-control publication, retains the actual result, and requires physical
exit plus join before releasing charge. Operation/cleanup/status failures remain
separate; event 9 routes fixed operation codes. Both native executable defaults
remain inert. This is an initial P3 source integration, not an installed service.

The frozen 647-input development candidate is
`85551119db977803da105f84d7405a2101cd93121c99c91b85a6183f915f6a9a`.
Its standalone manifest/lock hashes are `0a9e2f25…`/`7174fe52…`; the unchanged
root lock remains `26dd7d34…`. Actual guarded root resolution reproduced zero
registry additions/removals before rebinding the complete manifest context and
code-owned policy pin. All 26 standalone registry identities reuse exact accepted
root sources/checksums; ten are newly selected in this standalone lock, no new
registry version. GNU normal/test trees select 16/27 packages, MSVC 18/29;
Engine is dev-only with KE disabled. No licensing/feature/build grants changed.

All twelve configured `qualification.py privilege-checkpoint` lanes passed:
format, GNU/Windows all-target Clippy, ordinary/optimized tests, separate doctests
and rustdoc. Each mode ran 105 fixture-library cases plus one executable-entry
case, including all eight genuine Engine compositions; each also ran 45 platform
cases. Platform doctests ran eight cases. Fixture doctest selection was empty:
no positive case or general contract proof is counted for that zero result.
The Engine controls cover literal T2/T3 and zero admitted samples, denied startup
and Running, failed preparation, pre-publication and in-flight Stop, delayed join
including expiry, Stale, poisoned publication and supervised actual-thread unwind.
The wake-before-park case uses an independent Condvar witness and honors the
original Pending rather than masking a missing notification with a heartbeat.

The original 900-second development owner closed at 891.910 seconds. Five exact
candidate attempts retain the initial import/lint failures; the final native lane
results are not attributed to earlier candidates. Its final receipt is
`artifacts/windows-services/recovery-20261107-v1/engine-compose-development-dev05.json`
(SHA-256 `0f1df4fb9eb4d156ecb1320896b4ff5153bdb84ceef326e5174118c2b7965fc6`).
Graph, raw first failures and coordinator closeout remain in the same evidence
chain under `engine-composition-20261120-v1/`. Derived builds lived on a private
3-GiB RAM mount and were retired after each attempt; no compiler outputs retained.

Seventeen admission controls and twenty metric-reader controls pass. The latter
retain a reproduced HEAD failure: deferred zero-viable accounting returns an
explicit failed score, while strict receiving admission still refuses it. The
corrected test checks both; no unviable variant receives kill credit. Independent
fresh-context graph and combined-source reviewers completed, with later scoped
rereviews. They found a stale reversal expectation and a wake-masking test gap;
both were corrected before final execution. One exact non-test-only dead-code
expectation covers the join-tail variant whose real producer is test-only; test
builds do not suppress it. Inert native behavior is unchanged.

Process-only core refusal was needed for WSL's piped crash collector: matching
Microsoft kernel source rejects RLIMIT_CORE 1 before collector launch, whereas
zero is ignored by pipes. The configured runner checks both inherited limits;
file collectors use zero, socket collectors remain unadmitted. No host collector
setting changed. Actual GNU fault children aborted without a core and retained
no Finished/Clean acknowledgement; Windows crash execution remains held.

**QA disposition at the development-only exit (historical):** at that point,
four-owner line coverage and mutation qualification remained unexecuted. Pinned cargo-mutants skips the test-gated worker
during native discovery; inspect actual ordinary and direct-file inventories
before selecting a complete reviewed alternative. Empty selection earns no score.
Runtime outer-error/cleanup producer paths are not replaced by scripted host-fact
controls. Existing model/boundary/lifecycle, sink-failure and actual-consumer tests
ran; no generalized property/fuzz campaign or native memory/leak result is claimed.
No new byte codec/unsafe/FFI was added; held fuzz, native memory/service, storage,
UI and post-V1 formal obligations retain their owners. Genuine Engine-to-Core
mapping/eligibility, native workers and complete MSIX remain separate exits.


<a id="s31-four-owner-metric-preparation--21-november-2026"></a>

**Four-owner metric preparation — 21 November 2026.** The actual generator
preflight completed on the same 647-input `85551119…` candidate in 24.833 seconds,
under a separate 240-second allowance. Ordinary worker file/descriptor lists are
both empty; the unchanged native host positive lists 92 variants. Pinned direct
file generation lists 33 worker descriptors covering thirteen named functions.
All four native tool exits are zero with source/vendor/index bookends and no
compilation/test run. Receipt `engine-composition-20261120-v1/generator-preflight-01.json`
has SHA-256 `a534758e59a465bed4c0e8a6277767838602ce78ec2f1fb37d5e065025c70ec5`.
These are generated rows, not executed kills. The current native parser correctly
rejects empty inventories and the direct generator's `single_file` package.

**Next-stage proposal for independent review, not an executed qualification:**
register host, error, diagnostics and the test-only worker as four selected
critical owners before coverage. Preserve native cargo-mutants for the existing
three owners. For the worker alone, extend the existing fixture metric entrypoint
with a separately tagged complete direct-generated replay; add no general runner,
dependency, compiler extension, production cfg, feature or dependency edge. Keep
the original `cfg(test)` and every ordinary/native hold. Do not relabel its rows as
a native cargo-mutants campaign or manufacture native outcomes.

Admit the complete bounded raw JSON and exact rendered diffs from the pinned
direct generator against the independently reviewed source, tools and graph.
Retain its raw `single_file` identity; any fixture-path projection is explicit,
source-bound and round-trip checked, never fed silently to the native parser.
Use the existing exact-span overlay method from the Engine facade preparer and
existing source/inventory review. Require all rows, unique complete descriptors,
valid in-file spans, unchanged source context and exact one-file changed bytes;
reject altered/missing/duplicate/out-of-owner/oversized rows or diff identity.
Register generation gaps separately, including ungenerated individual wake/join
checks. Do not move or disguise the test gate to influence discovery.

Replay only in an owned private derivative, with immutable original inputs and
per-case byte bookends. Reuse the admitted Q sandbox/tool/vendor/graph and ordinary
fixture case roster. Establish an unmodified full Build/Test/doctest baseline;
record actual raw Build/Test argv, exits, output, duration and custody for every
complete descriptor. A compiler refusal is unviable with zero kill credit only
when the changed owner explains it. A Test failure requires a meaningful selected
behavioral cause through the real host/Engine consumer; setup/native abort alone,
missing results or timeouts grant no kills. Surviving, timed-out or unclassified
rows remain open. Restore the exact original before the healthy closing control.
No proposed projection or handwritten verdict is execution evidence.

The native and direct-replay result schemas remain distinct. The worker's score,
if admitted, identifies all 33 direct-generated cases and the full-row replay,
with a nonempty viable denominator and unchanged >=95% threshold. No equivalence
or exclusion is preapproved. Source/requirement reconciliation and independent
review of actual effects remain mandatory; manual sensitivity controls add zero
generated credit. Score passing never closes the held native/production exits.

**Finite exit:** first qualify the parser/custody/scorer with independent healthy
and malformed fixtures, then a representative real healthy/fault replay before
the full row set. Budget preparation and representative replay together at 900
seconds; declare the full campaign's separate cumulative allowance only after
that exit. One execution owner, one inner Cargo job/test thread, four mutation
workers maximum, private RAM scratch, 4-GiB root-disk reserve and 1-GiB free-memory
stop. Preserve all original failures and record timing/cleanup. Fresh-context
`s31_metrics_plan_review` completed READY for the preparation/pilot only; its
historical memory lookup was disclosed and material facts rechecked. The fourth
95% gate remains open.

The same pilot origin now has a 1,800-second cumulative ceiling, retaining its
initial 900-second allowance and all preparation usage. This single bounded
amendment addresses a concrete prerequisite: the existing shared subprocess loop
had no in-run resource probe. An optional fixed-reason guard now stops/reaps its
owned process group, while default callers retain their old behavior. The service
caller checks the declared disk/memory floor during execution; healthy, resource
stop, invalid guard and exact-floor controls qualify it. Independent review also
requires a child-readiness witness rather than callback-count timing in the stop
control. This is no new full campaign or allowance reset. Proceed with fresh
helper-bound graph/ordinary/direct enumeration, then the named representative
replay; the full campaign still awaits its fixed exit.

The first real replay (`engine-compose-worker-pilot01.json`) stopped before
Build/Test: Cargo 1.100 refused package cleaning in a newly created empty target
without `CACHEDIR.TAG` (exit 101). It remains incomplete with zero kill credit.
The causal repair admits an absent or strictly empty canonical owned initial
target and records that observation, leaving Cargo to create its cache marker.
Nonempty, symlink and non-directory initial targets refuse without deletion.
Every variant and closing Build still requires successful package-scoped Cargo
clean plus exact freshly compiled fixture libtest evidence. Independent reviewer
`s31_replay_code_review` accepted this bounded amendment before implementation.
The same pilot origin retains all earlier usage and now has a 2,700-second
cumulative ceiling for this repair, fresh admission and one representative retry;
this is not full-campaign permission or an allowance reset.

The second replay retained a successful real baseline Build/Test/doctest, then
both variant/closing Cleans refused the same missing marker. It remains
incomplete with zero kills. Source tracing found the service runner precreated
`target` on every command, including direct enumeration; upstream Cargo marks
newly created cache directories. The causal preparation control failed in both
ordinary/mutation modes before repair, then all nineteen controls passed after
removing only `target` from that mkdir list. This leaves Cargo responsible for
creating its own cache and marker through the existing private writable work
mount; no tag is forged and no clean/refusal is bypassed. Reviewer
`s31_replay_code_review` accepted this bounded hypothesis and required actual
subsequent successful variant/closing Cleans before confirming it. The same
2,700-second pilot ceiling and original usage remain unchanged.

**Representative pilot exit passed.** Current 648-input candidate
`f5d2083a8ae38f9e3215d8689c71e27b03ac272d4a4716f2abb641fd6e70646a`
passed fresh six-command graph admission, all 106 ordinary fixture cases and
all 33 complete direct descriptors with current independent review. The actual
healthy → fault → restored-healthy replay passed in 69.921 outer-owner seconds
(67.058 seconds inside the receipt): both package
Cleans succeeded; baseline and closing each require seven fresh fixture artifacts
including the exact libtest; the fault produced the intended poisoned-publication
assertion at `engine_worker_tests.rs:876`. Receipt
`engine-compose-worker-pilot03.json` SHA-256
`f090f9292357ecf2c098af0ef83072ad809f4d8b1bbb1f89845ce54ef07774a5` retains
source/vendor/tool/index bookends and raw commands/results. This confirms the
cache initialization repair on the admitted Cargo, not arbitrary toolchains.
The pilot owner closed at 2395.130/2700 cumulative seconds, including preparation,
reviews and both earlier failed attempts. All private RAM builds were retired.
Its score correctly remains incomplete (one executed of 33, no full percent or
gate pass); no four-file metric/native/production qualification is inferred.

The independently accepted next stage is one 2,400-second cumulative full-metric stage,
including preparation/admission/review: four >=90% line gates, the existing three
native mutation owners and the separate complete 33-row worker replay with
unchanged >=95% per-file requirements. Before that run, retain each exact changed
worker overlay and reconstruction identity beside raw command evidence, then
retire only its fully checked owned derivative after actual process return and
classification. This avoids exporting 33 duplicate complete source trees. Keep
the original base once; any mismatch, unresolved custody or cleanup failure keeps
the derivative and stops. Independent `s31_metrics_plan_review` completed READY before dependent execution.
The shared exporter gains an optional strictly stronger disk-reserve parameter:
existing callers retain 2 GiB; this metric caller explicitly requests 4 GiB,
checked before and during copies and recorded in its ledgers. Selection/retention
rules, native/direct schemas and open generation/native obligations remain
unchanged. Coordinator owns that shared helper; the isolated writer owns only
the metric reader and its controls. Preparation, source review, fresh admission,
actual execution, export and cleanup consume the same declared stage origin.

Full-stage review also found a replay input alias: `/work/source` was read-only,
but the same derivative remained writable at its natural path under the work
mount. The fix seals both aliases. A real isolated Python canary first opened the
natural alias for writing while the canonical path refused with EROFS; after the
fix both reads succeed and both write-only opens refuse with EROFS, beside a real
writable scratch neighbor. No canary bytes were changed. RED/GREEN receipts
`source-alias-actual-red01.json`/`source-alias-actual-green01.json` retain the
commands and cleanup; this is a verification-boundary repair with no demonstrated
host escape or product vulnerability. Nineteen Q controls pass. Sparse/export
controls pass 34 cases; current environment/resource controls pass 39. Skills
applied: plan/code contracts and causal failure triage for preparation; mutation,
resource, boundary and independent-review procedures for the current metric
stage. These synthetic and process-boundary checks add no generated kill credit.

The independent source reviewer accepted sparse deletion and export changes,
with a final receiving prerequisite: after all subprocesses/export, independently
verify every archived worker/manifest against its recorded hashes, full original
and variant identities, exact descriptor and command linkage. Later writable work
could otherwise alter earlier sparse files. No final qualification claim follows
from score fields alone; all prior source/native/generation gaps remain explicit.

The first complete coverage execution ran all 106 cases: diagnostics 118/118,
error 132/132, host 547/582 (93.99%) and worker 162/184 (88.04%). The gate remained
failed. One new component case exercises actual dormant/prepared/retired Worker
transitions, pre-activation Stop and first-failure retention with a genuine checked
session and peer. It changes no worker implementation or denominator. Its first
integration reused source index 0, and the healthy neighbor correctly refused a
second first-install-only configuration. The fixture now admits ten bounded
sources and gives the new case index 9; original cases retain indices 0–8. This
fixes test isolation, not an established product defect. Original failures remain.

Independent review accepted the new case and the causal fixture repair. The same
full-stage origin now has a 3,600-second cumulative ceiling, retaining its original
2,400-second allowance and all usage, because these demonstrated prerequisites
require changed-candidate qualification. Per-lane 1,200-second caps, resource
stops, 90%/95% requirements and native holds remain unchanged. Budget grants no
passing result. Completed private writer/preparation trees were removed after
preserving their exact patches/untracked sources and verifying root Git custody.
Unused dependency/object caches from the failed coverage build were deleted;
actual executables/shared objects, profiles and raw coverage/command/source data
remain. Three duplicate pilot source copies were deleted only after complete hash
parity with retained direct-enumeration source archives. Explicit cleanup ledgers
identify historical export subsets and exact retained source locations.

**Causal qualification continuation — 21 November 2026.** The partial
`engine-compose-full-worker-replay03` stopped after eleven of 33 descriptors:
three behavioral kills, five compiler-unviable, two survivors and one unclassified
watchdog failure. No score passed and the watchdog earns no kill. Real controls
now observe original Stop/control publication and Stale retirement before releasing
the physical thread-tail barrier; cleanup precedes assertions without changing
the captured oracle. A separately retained original control is used only to
rescue an adverse test path, so the observer cannot manufacture the claimed Stop.
Four literal inputs test the private cleanup OR policy in the same critical file;
its real report-getter/cleanup-error-producer path remains a separate open gap.
The fresh worker inventory has 35 descriptors, not the historical 33.

The first native-three run stopped before any mutation because the pinned tool's
ordinary source copy omitted the external Engine **dev-dependency** path. The
supported confined in-place mode now keeps the complete private derivative,
serializes one mutator and one Cargo/test worker, seals both parent aliases and
permits writes only to the selected existing owner file. Original source,
manifests, tests, dependencies, configuration, vendor and tools remain read-only.
Actual mount controls observed selected writes/restoration and eighteen neighbor,
manifest, unlink/rename refusals. Dirty or interrupted derivatives remain failed;
no tool patch, graph substitution or fault exclusion was introduced.

`engine-compose-final-native-mutation05` then reached a healthy baseline but was
killed by the actual `DISK_RESERVE` guard. Root-headroom loss was observed; its
cause was not established. Full export failed, and the former supervisor then
unmounted its private RAM workspace: raw outcomes/dirty derivative are unavailable.
Its two stdout misses cannot establish scores or exclusions. This retention
mistake is preserved, not reconstructed as a complete native run. The corrected
supervisor requires actual 6-GiB launch headroom, records numerical root/work/RAM
observations and preserves failed work independently before unmount. If preservation
itself fails, its real signal-resistant custodian remains idle, with no campaign
or automatic release. Actual retention/refusal controls and independent source
review are retained in `engine-composition-20261120-v1/supervisor-retention-controls06.json`
and `supervisor06-final-retention-review01.json`. The enforced execution/export
reserve remains 4 GiB; no threshold was lowered.

A surviving Stop-helper fault exposed a masked postcondition: the existing exact
representability case now checks the actual `begin_stop` deadline, overdue fence
and charge immediately, before the later poll. The legal representable neighbor
and all original failure/status/cleanup assertions remain. Pinned Rust's
`From<T> for Poll<T>` constructs exactly `Ready(t)`; the observed `Poll::from(Ok(()))`
replacement has a source-domain equivalence argument, but no killed-mutant credit
or exclusion is inferred from the lost run.

A probe also created one ignored Python bytecode input. Its exact bytes were
retained; independent comparison of all 39 recursive code objects found unchanged
execution/location attributes and constants. Raw marshal payloads were **not**
byte-identical. Historical development04/coverage04 remain bound to their actual
649-input candidate; no receipt was rewritten. The cache was removed before the
fresh 648-input candidate and all subsequent imports suppress bytecode.

The single full-stage origin and all earlier usage remain: independently reviewed
causal amendments extended 2400 → 3600 → 5400 → 6300 → 8100 seconds for the actual
coverage/isolation, native-layout/sensitivity, generated-cache and disk/retention
prerequisites. Per-lane caps, 90%/95%, complete inventories and scoped holds are
unchanged; an allowance supplies no result. The authorized cleanup deleted 83 exact
superseded Nov14–17 audit subdirectories (2,139,627,520 allocated bytes), 37 old npm
download-cache files (197,988,352 bytes recovered) and fourteen obsolete native PDBs.
Root reports/patches, current Core Nov18 evidence, current service evidence,
repositories, installed tools, vendor and Git custody were preserved. Historical
raw replay/symbol availability is explicitly lost at the paths listed in
`obsolete-audit-retirement06.json`, `old-npm-cache-retirement06.json` and
`obsolete-native-symbol-retirement06.json`; those older raw files must not be
reported as independently inspectable.

The current freeze is 648 inputs, candidate
`75328629358e6159cde07debf7395cf17217e3a396e76dd82ff565577ba91960`,
HEAD `78f8ae931c7139d37e0ff36ed18e41dabf8c72ff`, with preserved dirty source and
unchanged index. Graph06 (six commands), development06 (all twelve configured
lanes), ordinary06 (109 cases) and both complete inventories pass with current
source/tool/vendor/index bookends. Both independent inventory reviews accepted
the exact 150 native and 35 direct descriptors. Fresh current coverage/mutation
results and final independent archive receiving review are still required;
this paragraph does not report their execution or qualification.

**Three-owner receiving complete; worker-policy follow-up active.** On candidate
`75328629…`, native06 completed all 92 host variants, but its receiving parser
correctly stopped at three assertion-free aborts. Its original incomplete receipt
is unchanged. The separately reviewed `engine-compose-native-adjudication07.json`
then replays those exact three generated overlays through the existing supervised
healthy driver: each has an actual parent assertion, with identical healthy
baseline/closing controls (one selected library case, 107 explicitly filtered).
Original raw diffs, all three full derivatives and original source are retained.
It also executes the two previously unrun native owners, without repeating host92.
Independent receiving checked all 2,837 exported files, all 150 native raw diffs,
all cause links, source/tool/vendor/proof/index bookends and every retained variant.

| Selected source | Distinct generated outcomes | Qualified accounting |
| --- | --- | --- |
| Host | 92: 53 meaningful kills, 36 compiler-unviable, three exact source equivalents; zero timeouts | Raw 53/56 = 94.642857% remains recorded. Three independently reviewed equivalents remove only their own denominator entries: 53/(56−3) = 100%. The three semantic supplements are existing generated IDs, not three additional kills. |
| Error | 42: 17 meaningful kills, 25 compiler-unviable, zero survivors/timeouts/exclusions | 17/17 viable, 100%. |
| Diagnostics | 16: three meaningful kills, 13 compiler-unviable, zero survivors/timeouts/exclusions | 3/3 viable, 100%. |

The source-domain arguments live in the bounded companion
[service-fixture equivalences](windows-service-fixture-equivalences.md); they
identify exact descriptors, private-owner invariants, side effects, adversarial
counterexamples and invalidation conditions. They are source review, not formal
jobs or behavioral kills. Both independent receiving reviews retain the original
failed raw status. The adjudication's `metrics_passed`/`qualification_passed`
fields remain false; this is three-owner source qualification, not four-owner or
native service completion.

The complete worker06 replay executed all 35 variants: 19 meaningful kills,
15 compiler-unviable and one survivor, no timeouts. Its raw 19/20 = 95% still
fails the existing **no unresolved survivor** rule. All 35 sparse archives,
original/variant maps, exact byte reconstructions, command links and causes were
independently verified. The survivor deletes `!cleanup_failed` from operation
classification; current real report cases have no cleanup failure, so the pure
cleanup-OR table did not distinguish it. No threshold was relaxed and this
survivor was not declared equivalent or hidden by an exclusion.

The causal worker08 refactor keeps the exact expression and negation in the same
critical source file, behind one private total policy helper. Its actual consumer
still supplies the genuine immutable report's end/primary-cause getters and
original cleanup boolean. The existing test now has 20 independent literal rows
covering every current end/boolean combination, including cleanup-only failure;
no report is forged and no test name, dependency, filter or native entrypoint
changes. The actual Engine cleanup-error producer/getter transfer remains open.
The two source files were integrated only after the 753… jobs and custody closed.
Fresh new-candidate graph, all twelve development lanes, ordinary cases, all four
line gates, complete direct inventory/review and full worker replay are required.
Three unchanged native owners may reuse their exact 753… qualified evidence only
through the independently reviewed unchanged-source/semantic-consumer bridge;
old receipts and scores are never rebound as new-candidate executions.

The same origin retains every earlier charge and now has an independently reviewed
11,100-second ceiling, extended for the demonstrated semantic-cause admission and
worker-policy sensitivity failures. No new campaign name resets elapsed usage or
per-lane caps. The unchanged actual `WorkerFinish` handling rule also has retained
compiler controls: a legal private-host consumer succeeds, bare discard fails
solely for `unused_must_use`; no visibility/setup warning supplies that refusal.
Fresh review checked both full changed source maps and actual diagnostic spans.
This supplies compiler-rule evidence, not linear consumption or native cleanup.

<a id="s31-real-engine-selected-qualification--21-november-2026"></a>

**Selected portable qualification complete — 21 November 2026.** The final
648-input candidate is
`02c515363237d1c3c3f7adfc1808dcd0de62546105f97e83390d44675d0256e4`,
against HEAD `78f8ae931c7139d37e0ff36ed18e41dabf8c72ff`, with preserved dirty
work and unchanged index. Fresh graph08 passes all six commands; development08
passes all twelve configured lanes, including GNU ordinary/optimized 108 library
plus one entrypoint case, 45 platform cases per mode, eight platform doctests,
GNU/MSVC all-target Clippy, formatting and rustdoc. Zero fixture doctests earn
zero credit. This is current integrated source evidence, not a native Windows run.

| Current selected owner | Fresh executable-line gate | Mutation evidence and scope |
| --- | --- | --- |
| Host | 547/582, 93.9863% | Qualified unchanged 753… source: 53 meaningful kills, 36 compiler-unviable, three reviewed zero-credit equivalents; adjusted 53/53. |
| Error | 132/132, 100% | Qualified unchanged 753… source: 17/17 viable killed, 25 compiler-unviable. |
| Diagnostics | 118/118, 100% | Qualified unchanged 753… source: 3/3 viable killed, 13 compiler-unviable. |
| Real Engine worker | 186/193, 96.3731% | **Fresh current37/37 replay:** 20/20 viable killed, 17 compiler-unviable; zero survivors, timeouts or exclusions. |

The current direct generator still selects the negation fault at
`series_operation_failed` line180:80. Its actual variant fails the independent
literal policy assertion; the baseline and healthy closing each execute all109
cases after seven fresh fixture artifacts. All37 Clean/Build and20 reached Test
phases, every exact raw descriptor, all648 base hashes, all37 sparse manifests,
complete variant maps, byte reconstruction and command/cause links were
independently checked. Compiler refusals earn zero behavioral kills. Branch
counts are zero: this supplies no branch qualification.

The three older native-owner results and must-use compiler controls retain their
actual **753…** labels. The only source delta is the worker's exact expression
extraction and its literal table. All three native owners/tests, Q/M/helpers,
manifest/lock/features, compiler/target/vendor/configuration and expected effects
are unchanged; both moved getters are pure borrowed reads and every helper input
preserves the original result. Independent review accepts this explicit unaffected-
state bridge alongside fresh current integrated checks. No old receipt, source
binding or score was rewritten as a new-candidate execution. The failed35-worker
result and original native06 refusal remain preserved.

Current timer-cleanup production has a narrow applicability result: the sole
`EngineWorker::new` factory fixes `ControlSchedule(None,None)`; actual Engine
`start_timer(None)` produces no job and `finish_timer(None)` succeeds. All cleanup-
field assignments were traced. A positive timer-cleanup error is therefore
source-inapplicable to this exact factory path, **not** equivalent over the broader
helper domain. General Some-schedule/alternate-factory genuine producer/getter
transfer and later outer-error evidence remain open before that adoption. No
report was forged and no current Some-cleanup execution is claimed.

A fresh-context independent judge completed source and raw receiving review:
`engine-composition-20261120-v1/final-integrated-judge06.json`, SHA-256
`d68cf7f9c7d1f83bdca395a308ca83800bf629afafd5302efde2c03d868e1635`.
The combined source record is `combined-source-checkpoint08.json` in the same
existing evidence directory. Its full source/configuration/lock identities,
actual receipt hashes, reuse argument, original failures, reviewer completions,
cumulative budgets and cleanup records remain separate from Git publication.

**Scope and next action.** QUALIFIED for this None-schedule portable source
composition and selected four-owner line/generated gates. Individual constructor,
wake/join/atomic/cfg and generated/macro omissions retain the inventory review's
limits; no exhaustive schedule, general fuzz/property-framework, leak/sanitizer,
physical power, installed Windows/two-family, production mapping/positive Core,
provider/service/IPC/clock, MSIX or release exit is closed. Current source adds no
new decoder/byte codec or unsafe/FFI boundary; held fuzz/native/UI/storage and
post-V1 formal work remain with their owners. The actual native hosts remain
inert. Continue the existing W11-W08 first portable logger stage by freezing its
**whole** catalogue, producer facts, limits, consumer and health contract and
obtaining independent review before implementation. An enum-only/helper-only
extension cannot close that exit. No commit/signing/publication action occurred.

**Frozen contract, revision 2 (independently accepted before implementation):**

1. One invocation retains the existing `ServiceLifecycle`, reporter, worker and
   sole outstanding permit. Host startup/token admission precedes worker
   preparation. All worker phase methods, activation and cancellation return
   promptly. Worker events are level-triggered: publish state before coalesced
   owner wake, retain it until observed, and retain an unpark token across the
   check-to-park race. Every wake rechecks Stop/time; waits use the remaining
   fixed phase/run deadline, without a polling heartbeat or invented progress.
   The existing SCM callback continues to publish only sticky Stop and wake the
   owner. A real worker must supply this wake protocol before later adoption.
2. Preparation returns Pending, Ready or a typed failure. Ready acknowledges
   physically completed initialization and a prepared, dormant worker. Only this
   acknowledgement completes the startup permit. The owner rechecks Stop/time
   and calls `ready`; operational activation needs an acknowledged Running
   report and a separately admitted running permit. Neither preparation nor
   Running means trustworthy time or clock authority.
3. The invocation permanently fences all its admission before any fallible time,
   StopPending/report operation or worker cancellation. This private fence also
   applies when `ServiceLifecycle::request_stop` refuses reversed/unrepresentable
   time before changing its state. Worker failure and cancellation requests are
   distinct from actual completion; a failed Running report never activates.
4. Completion is an explicit Pending/Finished(Result) observation. Only Finished
   acknowledges completed jobs and resolved cleanup custody (or an acknowledged
   recovery handoff), and completes the
   permit. A first worker/status/time failure remains the primary result;
   subsequent worker cleanup and status failures remain bounded, inspectable
   secondary facts: one completion outcome and one outcome for each permitted
   StopPending/terminal report, without an accumulating error list. An unresolved
   cleanup attempt stays Pending. Cleanup cannot replace the primary cause or
   manufacture Clean.
5. Deadline expiry and Drop release no charged work. An overdue invocation stays
   retained with its original lifecycle/reporter/worker; finite step methods
   return Pending/Overdue without consuming it. It emits no Stopped and admits
   no replacement work. Dropping a borrowed poll/observation does not drop that
   owner. The private host keeps the invocation through Finished and cannot
   return ServiceMain on an overdue step. Whole-owner destruction while a worker
   is unresolved must be prevented by the private ownership path or the inert
   fixture's fatal custody guard; a leaked counter alone is insufficient.
   Late completion can settle only this invocation, with a failed terminal
   outcome. Polling/repeated Stop never renews deadlines or fabricates progress.
   The source driver cannot force completion of a stalled native job; actual
   native termination/recovery remains a held product integration prerequisite,
   not an assertion made by this source child.

Independent controls use the actual host/worker owner: healthy readiness/Stop;
denied startup invokes no worker; pending/failed preparation never runs; Running
report failure never activates; Stop/failure/exact deadline close admission;
cancellation/timeout/drop cannot complete charged work; late completion cannot
release successor work; cleanup/reporting faults preserve the first failure.
Both roles retain existing laboratory identity, privilege checks and limits.

Fresh-context read-only reviewer `service_contract_review` required revision 2's
early fence, wake, retained-overdue custody, resolved-completion and bounded
secondary-outcome clarifications. It then accepted revision 2 for bounded source
implementation with no residual contract blocker. The fixture-only fatal custody
guard must abort before unresolved worker/reporter field destruction; it selects
no product fault policy. No source or native test is implied by that review.

**Entry preflight:** configured `qualification.py ready` exited 1 before any
Cargo/compiler/analyzer command: missing
`/home/agent/.local/share/nevrotick/analysis-installation.json`. Receipt
`artifacts/windows-services/recovery-20261107-v1/cloud-s31-ready-entry.json`,
SHA256 `f2874bfdbdc4f1a90c08f0d9840b2b5586eb5a709d5c899ecc8a5799895b2250`.
Current runner also requires the canonical private compiler and pinned stable
inventory absent here. Existing cloud vendor contains one extra package versus
the current 533-directory policy; it was preserved, not used as admitted input.
Ordinary execution and current metrics remain incomplete. Restore/admit the
configured installation and exact vendor without weakening policy; then create
a fresh graph receipt. The coordinator must separately refresh the historical
421-input metric binding and nonempty reviewed mutation inventories. The
90% per-security-file line and 95% per-critical-file mutation gates are unchanged;
platform metrics alone cannot qualify changed fixture host/worker source.

**Source-authoring checkpoint (not qualified source adoption):** the existing
native `run_service` now drives one persistent private `ServiceInvocation`.
`WorkerCustody` distinguishes dormant, preparing, prepared, active, cancelling
and actually finished work. Its worker starts dormant; phase methods cannot
receive a lifecycle, reporter or permit. The original thread-bound owner alone
releases a charge after Ready/Finished acknowledgement, reports status and
extracts its result once. Whole-owner destruction while unresolved aborts only
this inert fixture before its worker/reporter fields are destroyed. Overdue
steps retain the original owner; notification-only retention after the shutdown
deadline makes **no bounded native-stop guarantee**. No production worker or
effect was connected.

The independent test owner authored 18 worker controls through
`start_invocation`/`poll`; the coordinator added an independently reviewed live
Stop-deadline overflow case, for **19 worker controls**. These include both roles, denied startup, readiness and
Running acknowledgement, Stop inside Running reporting, reversed/expired time,
all worker failure phases, pending cancellation, independent phase deadlines,
late completion and successor isolation, bounded failure facts, one-shot result,
discarded observations, retained wake and healthy/fatal whole-owner Drop children.
One new error-projection test checks typed causes and all secondary slots; the
existing category test includes both new variants. Existing host tests retain
their behavioral expectations, with two additional reversal boundaries and clock
scripts adjusted from eight to ten healthy observations for preparation/health
guards. First-status-failure testing now also observes both later report failures.
**All these Rust tests are authored, uncompiled and unexecuted.** No behavioral
RED/GREEN, formatter, compiler, Clippy, rustdoc, metric or Windows result is claimed.

Fresh-context `service_source_review` inspected the source, all 19 worker-test
oracles and teardown. Before final freeze, review corrected stale pending-time
guards, one-shot result custody, the preceding-phase/stop-deadline distinction,
private-item documentation and the wake control. Its final source-only verdict
found no blocking lifecycle/language/harness issue and was ready for configured
verification; this is not evidence that those commands pass. The wake control
uses a fresh owner thread and explicit notification/publication handshake beside
a bounded quiet neighbor; spurious wakes remain legal and it proves no scheduler
or native latency guarantee. The final added overflow case retains a live
preparation charge while shutdown deadline construction fails, beside a
representable neighbor. It checks the original startup-expiry cause, independent
Stop/terminal refusal facts and no false Stopped acknowledgement. Independent
review accepted its oracle; the case remains unexecuted.

Final read-only use of the existing qualification recipe's `source_identity` and
`manifest_admission` passed: **443 source inputs**, including all **16 fixture
inputs**, candidate SHA256
`7bfcca4bf6956583c500d02e395ff5da9e7c5646c7b2ce2a0cda4ebf85baafd6`.
This identifies bytes only; it is no tool/vendor/graph admission or executed test.
`git diff --check` exited 0. All `crates/`, manifests/locks, `.cargo/`, policies,
verification scripts and the Git index remain coordinator-owned and unchanged.

| Final source input | SHA256 |
| --- | --- |
| `tools/windows-service-fixture/src/host.rs` | `e69b4d96d92170d8d037d61b8dba5a175acc6f393ff2d20c0be5c76cdc91a178` |
| `tools/windows-service-fixture/src/worker.rs` | `2a2468721612a5a12263278b06ccdf0cfd0e84234de8567336146475efbac101` |
| `tools/windows-service-fixture/src/worker_tests.rs` | `55c2b6e1d5ab1728be9fa8abaa9c903acc3f64bfd8c8a2cc4f8b31fa128ccb75` |
| `tools/windows-service-fixture/src/host_tests.rs` | `3cb90733188e1d104d2710c6c13182ffd81d956b60443ef7ba522af3c4a1cc80` |
| `tools/windows-service-fixture/src/error.rs` | `18b29f03f343edfcba22268eac2eaadc4ded468d72fc7076604f3c86f307826e` |
| `tools/windows-service-fixture/src/error_tests.rs` | `0db5bc9afcdd399354349c7d6864f1c121bef010f3c5b38c98fac0afd9150305` |
| `tools/windows-service-fixture/src/lib.rs` | `c13d107f0c302738f3daec760616dce840ce39704e66e752287aa40106c50ef7` |
| `tools/windows-service-fixture/README.md` | `99ffb6a2add8ba1645ec0fc08b922e72124ce962089e7a0162ffd92be7cbbd0d` |

**Tool installation continuation — 8 November 2026:** David explicitly requested
installation of the missing tools. Pinned rustup 1.29.3 and Rust 1.99.1 are now
installed at the canonical `/home/agent/.cargo` and `/home/agent/.rustup` paths,
with the declared Clippy, rustfmt, rust-analyzer, rust-src, LLVM tools and Windows
MSVC target components. All eight compiler executable digests match the existing
policy. The configured `setup-unix.py` installer completed all five pinned tool
installs and version smokes: nextest 0.9.150, cargo-deny 0.20.2, cargo-audit 0.22.2,
cargo-mutants 27.1.0 and cargo-llvm-cov 0.9.1. All five installed binary digests
also match policy. Shell PATH/profile files were not changed.

The first nextest download timed out; that incomplete receipt is preserved.
Already verified nextest/deny archives were reused. The other three direct
GitHub downloads encountered 503s; public GitHub release-asset API downloads
then matched the exact same pinned archive hashes. No floating version, package
source, compiler flag or verification policy was substituted.

External retained evidence is under
`/workspace/.nevrotick-cloud/service-tools-install-20261108/`:

| Record | Actual result / SHA256 |
| --- | --- |
| `installation-api.json` | Complete five-tool installation/version smoke; `a1ca83aac8b8636e9482c3b61bcd3530e9db4391f18aea04606e26535bd71343` |
| `vendor-admission.json` | Official `verify.admit_vendor` passed all 533 packages / 27,101 files, no hardlinks; `86c2d6c113eeb808d7ea2f31193bf52de6eab7d46d80331352ec54848be69b65` |
| `evidence/stable-complete-payload-inventory.json` | Fresh complete 3,924-entry local compiler inventory, **not policy-admitted**; `11c41bffe7f0b5c808e5ea61f7385c3d3fb79eff99185be41b1d8a9246329719` |
| `host-helper-comparison.json` | Read-only required-helper path/hash inspection; `b2ab48895901a49c7c54dd94ce418756797ec3a7ccec0d8dfde06150eae70556` |

The exact fresh vendor is at that directory's `vendor/`; inventory SHA256 is
`6cd84bd39e0d73d42e3e2412ed6c314b9976461ba087d94f806b257ce7366c5d`.
The previous CodeQL vendor and its extra obsolete package remain untouched.
A private, owned mode-0600 installation pointer now exists at the required path;
it explicitly describes stable-only local installation, not policy admission.
Independent read-only `installed_tools_review` checked all five tool archives,
versions and installed binaries, all eight compiler binary pins, every one of
the 3,924 stable inventory entries, the complete fresh vendor and pointer custody.
These checks passed. They establish installation integrity, not product execution.
The service source candidate remains the same 443-input `7bfcca4b...` checkpoint.

**New actual refusal after installation:** configured `qualification.py ready
--label cloud-s31-ready-installed-v1 --vendor-dir <fresh vendor>` exited 1 with
`Inventory SHA256 mismatch; refusing to parse substituted bytes.` Its retained
record is `artifacts/windows-services/recovery-20261107-v1/cloud-s31-ready-installed-v1.json`,
SHA256 `de15a6e370e7ce1e320a42502299726c683f11f3791aa513006eec9c33963114`;
zero compiler/Cargo/analyzer commands ran. The original missing-pointer refusal
remains historical evidence. This retry follows actual prerequisite changes.

The remaining admission restriction cannot be repaired by installing Rust tools:
the service consumer pins the old inventory's complete JSON hash
`8b7bd1c19a976090ce4c8e8246a4ca1d7dfef26d76e04ae3bc9672101694190e`,
including machine-specific absolute roots. It also demands exact Ubuntu helper
payloads at fixed public paths. This Debian cloud's bwrap/prlimit/linker hashes
differ, and the required GCC-13/Python-3.12 paths are absent. Normal Debian package
installation cannot reproduce the original inventory/root binding or establish
equivalence of those Ubuntu payloads. No old inventory, checker or pinned hash was
overwritten to manufacture admission. The original verification policy stays
unchanged, as required by the coordinator-owned scope.

**Next action / remaining exits:** coordinator reviews an explicit cloud host
admission profile, retaining the original Ubuntu profile and binding this complete
installed compiler inventory, exact supported host helpers, read-only mounts and
existing resource/custody controls. The installation and helper records above are
concrete inputs to that review, not an automatic new trust root. Then run fresh
`ready` and `graph` receipts through `qualification.py`, followed by affected
fixture/platform ordinary and optimized tests, format, GNU/Windows Clippy and
rustdoc/doctest lanes with that receipt. Coordinator-owned metric rebinding,
complete inventories and fixture-specific measurement admission must retain
90% executable lines per security file and 95% viable mutation per critical file,
including generation/guard/derive reconciliation. Current policies/denominators
were neither edited nor bypassed. Windows cross-link/independent PE readback,
supported final accounts/provisioning, full per-role token/object/mitigation
admission, actual worker wake/completion and bounded process escalation,
diagnostic sinks, installed/native recovery and product composition remain with
their existing owners and holds. No Core/engine, production service, native,
package or release acceptance is closed by this checkpoint.

**Owner-requested pull and goal renewal — 8 November 2026:** both main and the
separate service worktree now fast-forward to published `origin/main`
`e76863fc2b5acc393d317fd93857213f410fe609`. The local task merged without
conflicts; all eight other service files and both main-checkout audit handoffs
retain their pre-pull bytes. Backup:
`/workspace/.nevrotick-cloud/service-goal-repull-p6k1vank/`.
The new cloud handoff in §4.35.8 is adopted as the current scope; the updated
development queue leaves this parallel child separate from the coordinator's
Core frequency-prior/holdover work.

The existing source preparation and contract revision 2 are preserved. Reconcile
the pre-existing private `worker.rs`/`worker_tests.rs` and `lib.rs` wiring with
the newly published bounded writable-file list before further implementation or
adoption; preserving those files does not broaden the assignment. Tools and the
exact vendor remain installed, while the cloud inventory/helper admission and
metric bindings remain coordinator-owned. The new read-only source identity is
**453 inputs**, SHA256
`9c5e6bc239eee89f036e64bdd789275788795909729363985ef3b8dca03538ca`.
The earlier 443-input source results and refusal receipts remain historical;
they are not current admission or execution evidence. No new Rust/test/native
execution occurred for this pull or goal message.

**Renewed goal message:**

Own and complete the admitted source work for the existing W11-W05/S3.1 service
adapter-to-worker lifecycle child from baseline
`e76863fc2b5acc393d317fd93857213f410fe609`, in the separate service worktree.
Start with current `AGENTS.md`, `docs/README.md` and
`tasks/windows11-integration.md` §4.35.8, then load the relevant repository service,
Rust-contract, security and testing skills.

Reuse `windows-platform`, `windows-service-fixture` and the existing task chain.
Continue from the frozen, independently reviewed startup/readiness/status/Stop
contract and preserved source checkpoint; independently review any necessary
contract changes before implementing the admitted source slice. Cover worker
failure, cancellation, actual-completion custody and cleanup while preserving
the original failure and bounded secondary outcomes.

Demonstrate that Running requires acknowledged worker readiness and successful
SCM status publication, Stop closes new admission, and cancellation, timeout or
Drop cannot fabricate completed work or release capacity prematurely. Preserve
the current testing workflow, complete reviewed inventories, at least 90%
executable-line coverage per security file and at least 95% mutation score per
critical file.

Keep Core/engine algorithms, shared manifests, lockfiles, dependencies/features,
verification policies, scripts and qualification bindings coordinator-owned.
Preserve all existing execution holds. Record exact source identities, actual
commands/results, unavailable checks and remaining native/product gaps in the
existing task chain; claim only the qualification stages actually demonstrated.

**Scope-reconciled continuation checkpoint — 8 November 2026:** the separate
worktree remains on agreed baseline `e76863fc2b5acc393d317fd93857213f410fe609`.
The worker types now live directly in the existing `src/host.rs`; its 19 controls
are a private nested module in the existing `src/host_tests.rs`. The earlier
separate worker files were folded after exact backup at
`/workspace/.nevrotick-cloud/service-scope-fold-su9avfc2/`. `src/lib.rs` and the
fixture README were restored to their published baseline bytes. Only the four
admitted host/error Rust files and this task differ from the baseline. No native
FFI wiring, platform lifecycle API, dependency, manifest, lock, policy, verifier
or qualification binding changed.

Contract revision 2 is unchanged. Structural comparison against the backup
passed: moved worker declarations differ only in the necessary standard-library
qualification; the original host owner body and host tests remain intact; the
19 new test bodies differ only in module/import paths and the single fatal-child
selector. That selector is now
`host::tests::worker_tests::unresolved_owner_drop_aborts_before_worker_or_reporter_destruction`.
The worker module's prompt-call, retained Pending, publish-before-wake and
failure-versus-completion obligations remain in the host documentation. Error
payload types stay public through the public `FixtureError` variants and are
imported internally from `crate::error`, without changing the library entry file.

Fresh-context read-only `scope_fold_review` independently compared the actual
backup/current source and found no blocking body, import, privacy, selector or
documentation issue. It confirmed all 19 controls were preserved. This is source
inspection, **not compiler, lint or behavioral execution**. `git diff --check`
and the admitted-writable-path check both exited 0.

The existing recipe's read-only `source_identity`/`manifest_admission` checks
passed for **451 inputs**, including all **14 fixture inputs**, candidate SHA256
`e92f619630de0f9d00f80ff5f8263d77acb8566a4cac8f111d89dfe8fc044e3b`.
The earlier 443/453-input snapshots are historical and supply no current pass.

| Current changed Rust input | SHA256 |
| --- | --- |
| `tools/windows-service-fixture/src/host.rs` | `6b3de31d12830940f78f0d42034f9d3112d3c800db4bb9a48f828dd2ff764c59` |
| `tools/windows-service-fixture/src/host_tests.rs` | `4357bfe70b8213bbecaf398fd3e44805fe8297745d53ecceada3cb34b8d32aa2` |
| `tools/windows-service-fixture/src/error.rs` | `18b29f03f343edfcba22268eac2eaadc4ded468d72fc7076604f3c86f307826e` |
| `tools/windows-service-fixture/src/error_tests.rs` | `0db5bc9afcdd399354349c7d6864f1c121bef010f3c5b38c98fac0afd9150305` |

**Goal disposition: blocked at coordinator-owned verification admission.**
Read-only reconciliation confirms the installed inventory is still
`11c41bffe7f0b5c808e5ea61f7385c3d3fb79eff99185be41b1d8a9246329719`,
while policy still requires
`8b7bd1c19a976090ce4c8e8246a4ca1d7dfef26d76e04ae3bc9672101694190e`.
Neither that policy nor its admission/runtime checker changed between the actual
installed-readiness refusal and `e76863f`. Its prerequisite failure is therefore
unchanged; no identical blocked compiler campaign was resubmitted. The service
metric policy still binds 421 historical inputs, has zero mutation inventories
and registers only the three portable platform owners, not the changed host.
No 90%/95% result, current graph/ordinary/optimized/format/Clippy/rustdoc pass or
Running/Stop behavioral demonstration can be claimed.

Next the coordinator admits the exact cloud tool/helper profile and registers
the changed host/error owners with current source/test bindings, complete
inventories and required measurement roles. Then run fresh ready/graph and the
existing qualification lanes on this candidate, preserving every actual failure
and raw result. Production workers/providers, final identities/provisioning,
effective privileges/mitigations, actual SCM/resource/completion/recovery and
bounded native escalation, broker clock ownership, full package and release
qualification remain open. All Windows/native, fuzz, storage, UI and post-V1
formal execution holds remain intact. The goal is not complete.

**Owner portability/metric amendment — 8 November 2026:** David explicitly removed
the machine-specific checksum requirement and directed mutation qualification at
the code this child changes, while another AI continues Core/engine work. This
supersedes the preceding admission/ownership blocker for this service lane.
The service-local `qualification.py` now uses the existing public-host sandbox,
observes current helper/compiler inventory identities instead of requiring an
old machine's inventory JSON or Ubuntu helper digest, and retains upstream
compiler/tool executable pins. Source/vendor/tool change detection, sealed
read-only inputs, isolated namespaces, offline Cargo, finite process/wall/CPU/
memory/file limits, immutable records and failed-run custody remain. Shared
analysis policies/runners, Core/engine sources, manifests and lockfiles are not
edited. No additional Ubuntu installation is required for these portable roles.

The fixture-local metric lane selects exactly the changed production Rust owners
`tools/windows-service-fixture/src/host.rs` and `src/error.rs`, with >=90%
executable lines and >=95% viable mutation per file, complete reviewed descriptor
inventories and explicit generation omissions. Unchanged platform owners retain
their separate evidence; their score cannot substitute for this host's result.
Python runner/parser controls qualify the tooling separately and grant no Rust
metric or product pass.

Initial new evidence: all 17 service-local Python controls passed. An actual
isolated boundary control confirmed writable owned output, read-only source,
hidden host home/credentials and host-loopback isolation; pinned rustc's version
smoke passed in the same sandbox. Record
`artifacts/windows-services/recovery-20261107-v1/owner-portable-runtime-v1.json`.
Fresh configured `ready` passed one command and `graph` passed all four literal
target/feature metadata observations, with tool/vendor/source bookends. Independent
read-only `portable_runner_review` found no blocking admission, graph, namespace
or resource-custody issue and independently passed the five admission controls.
These are environment/graph results, not yet service behavior or metric evidence.

**Portable execution and first metric attempt — 8 November 2026:** the owner
amendment unblocked actual configured execution on Debian. The v3 source matrix
passed fixture ordinary/optimized tests, GNU and Windows-target Clippy,
formatting, rustdoc/doc, and unchanged platform ordinary/optimized/Clippy/doc
lanes. Earlier `owner-portable-fixture-fmt-v1` and
`owner-portable-fixture-clippy-v2` failures remain retained: exact rustfmt changes
and a redundant outer module documentation attribute were repaired before those
passes. A finite direct inert-worker control was then added; v4 graph,
formatting, GNU/Windows Clippy and ordinary/optimized fixture lanes passed
**58 tests** (57 library, one integration), with no ignored or filtered cases.

`owner-fixture-coverage-v1.json` reports **435/470 host executable lines
(92.55319148936171%)** and **127/127 error lines (100%)** on candidate
`23d762fbaa9033f11164722b86a76e56a8fe99858ca30943b585e66a160252bb`.
It is line evidence, not LLVM branch or native Windows evidence. Full enumeration
and independent review preceded mutation: 72 host descriptors and 40 error
descriptors, with authored-function, guard, derive/macro and cfg/native omissions
recorded in `owner-fixture-inventory-review-v2.json`.

The first mutation attempt, `owner-fixture-mutation-v1.json`, is **incomplete**.
Host native exit 3 retained 72 actual variants: 32 reported caught, 30 compiler
unviable, eight missed and two timed out; the error campaign was not reached.
No final kill score is claimed. Some reported catches were bare process aborts
or resource exhaustion and require meaningful behavioral attribution, not
automatic credit. Raw listing, diffs, logs and all individual outcomes were
exported in full. Genuine test gaps include preparation after startup expiry,
preservation of an overdue deadline before later cleanup failure, a single
secondary failure, helper deadline/clock domains and initial shutdown allowance.
Earlier caller-only equivalence suggestions for the default Wait method and
`begin_stop` were rejected after reviewing the helpers' own admitted domains.
They remain non-equivalent until actually caught by discriminating controls.

The fixture-local metric helper now supports exact independently reviewed
equivalence records, retaining raw misses and zero kill credit, and records
unbuffered selected-test assertion witnesses before any later custody abort.
Abort/OOM/timeout alone remains no kill evidence. Its 17 Python controls and
five portable admission controls pass, independently repeated by
`fixture_metrics_review`. Ordinary Cargo/libtest and all shared gate policies
remain unchanged. These Python results supply no service metric credit.

Independent source review also identified a concrete Stop boundary defect:
an already published Stop during a clock observation could be followed by
Running acknowledgement, operational activation or health polling because only
an earlier private flag was inspected. Deterministic real-owner RED controls
are being added before the minimal fresh-Stop-guard repair. The final Stop read
will be the continuation's admission point; a later racing callback is handled
at the next boundary, without a claim of retroactive cancellation. This finding
and the test changes invalidate v4 as the final qualification candidate; its
earlier passes retain their exact source scope.

##### S31-EQ-POLL-FROM

**Exact reviewed equivalent, zero kill credit:** replacing
`InertWorker::poll_prepare`'s `Poll::Ready(Ok(()))` with
`Poll::from(Ok(()))` changes no output, error, effect or lifecycle obligation.
The complete native descriptor remains in the generated and executed inventory;
the first campaign actually reported it `MissedMutant`. Final scoring requires
that same exact descriptor to remain a raw miss in the fresh campaign.

Boundary/domain: the actual method accepts its mutable inert worker and retained
wake. It reads neither argument and performs no mutation, notification, resource
admission or cleanup; preparation readiness is distinct from finished work.
The argument applies to every admitted worker state and wake, not only today's
fixed driver or a narrower caller population. Counterexample search includes
dormant, activated and cancelled worker states, wake-token presence, readiness
and cleanup observations; neither expression branches on those facts.

Pinned Rust 1.99.1's `core/src/task/poll.rs:219` implements `From<T> for Poll<T>`;
its body at lines 228–230 is exactly `Poll::Ready(t)`. Here the inferred `T` is
`Result<(), WorkerError>` and `t` is the identical `Ok(())`. Both expressions
construct the same value, without fallible conversion, allocation or changed
short-circuit effects. Pinned rust-src file SHA256:
`dbaf9d13639e373517116869dbced2a438dc0cb297308b431da7484823294ee0`.
Independent reviewer `service_generation_map` checked the installed source and
matching rustc/rust-src 1.99.1 manifest before accepting this argument.

The descriptor's canonical SHA256 (all native fields, excluding optional diff)
is `8203999635e8ef330143fbb5bb779d936267bfe9714aa461ad052b2bdf52ede1`.
The final exact review also binds the complete fixture candidate, native
inventory, owner file and these task proof bytes. New source, contract,
replacement, descriptor, compiler/standard-library behavior or configuration
requires reconciliation; no function wildcard or inferred future exclusion is
authorized. The actual raw survivor stays visible, contributes **zero kills**
and is removed only from the adjusted viable denominator after exact admission.
The separate Wait, pending-check and shutdown-helper survivors receive no such
exclusion; concrete legal helper-boundary witnesses refuted those suggestions.

##### Final bounded service-source exit — 8 November 2026

**Result: PASS for this portable source child, not the parent S3.1/P3 or installed
product service.** The installation/admission blocker is resolved by the owner
amendment above. Final candidate:
`7eb81c688ad538b2744253bf686657dda93e15e315d87d989064ddc1e43a515f`
(454 admitted source/configuration/runner inputs), on baseline
`e76863fc2b5acc393d317fd93857213f410fe609`, branch `service/w11-w05-s3.1`,
separate worktree `/workspace/NevroTick-service`.

Final production owner hashes:

- `tools/windows-service-fixture/src/host.rs`:
  `63e7ee47f7805e72cfc1cbc533721b79009b08352ab8f31a70e38a7f0c5a5468`.
- `tools/windows-service-fixture/src/error.rs`:
  `18b29f03f343edfcba22268eac2eaadc4ded468d72fc7076604f3c86f307826e`.
- Final host tests:
  `d60399e9ca925334caa43b3a07f1a9d4384e8eb5667d304df802946cc837b874`;
  error tests:
  `d3a533edb4856cdd77881161d78cce4f969659b7d7f8839244ec2967e122c2d8`.

The actual owner retains its lifecycle, reporter, dormant worker, phase charge,
trusted clock history and original failure through cancellation and real
completion. Ready finishes initialization, acknowledged Running precedes a
separate operational charge and activation, and Stop fences admission before
fallible cleanup. Overdue/drop/cancellation release no unresolved work; only
Ready/Finished worker acknowledgements settle their phase charge. The driver keeps
the same overdue owner. Unresolved whole-owner destruction aborts this fixture
before worker/reporter field destruction; this chooses no product recovery policy.
One completion and two status secondary slots preserve later facts without
replacing the earliest cause or constructing an unbounded error chain.

The independent Stop finding has a complete RED→GREEN record.
`owner-stop-clock-regression-red-v5.json` ran the unchanged implementation:
66/69 library cases passed and exactly the readiness, activation and health
clock-Stop cases failed their intended status/effect assertions, native exit 101,
with healthy source/tool/vendor/index custody. The minimal repair adds a fresh
Stop read at those three post-observation continuation guards. The final guard
read is the admission point; later callbacks are observed at the next boundary.
Initial preparation already had the correct guard. The same expectations pass
on v6/v7; healthy no-Stop and pre-preparation Stop neighbors remain.

All final records below are under
`artifacts/windows-services/recovery-20261107-v1/`:

| Check | Exact final evidence | Observed result |
| --- | --- | --- |
| Locked feature/target graph | `owner-portable-graph-v7.json` | Four exact GNU/Windows metadata/tree commands pass. |
| Format / GNU Clippy / Windows-target Clippy | `owner-portable-fixture-fmt-v7.json`, `owner-portable-fixture-clippy-v7.json`, `owner-portable-windows-clippy-v7.json` | All pass; no lint capping or policy change. Windows is a cross-check, not native execution. |
| Ordinary / optimized fixture | `owner-portable-fixture-test-v7.json`, `owner-portable-fixture-optimized-v7.json` | Each passes 69 library plus one integration test; zero failed/ignored/filtered. |
| Executable-line coverage | `owner-fixture-coverage-v3.json` | Host **442/473 = 93.446088794926%**; error **127/127 = 100%**. Both exceed 90%; no LLVM branch coverage claim. |
| Full reviewed mutation inventory | `owner-fixture-enumerate-v3.json`, `owner-fixture-inventory-review-v7.json` | All **75 host + 40 error descriptors**; function/guard, derive/macro and cfg/native generation omissions reconciled. |
| Full mutation outcome/cause accounting | `owner-fixture-mutation-v4.json` | Both per-file gates pass, with healthy bookends and complete raw export. |

Mutation accounting, without favorable aggregate substitution:

- Host: 75 generated; **44 behavioral catches**, 30 compiler/lint-unviable and
  one exact independently reviewed `Poll::from` raw survivor. Native exit **2**
  is retained, not relabelled as zero. Raw viable score **44/45 =
  97.77777777777777%**; adjusted score **44/44 = 100%** after the sole exact
  equivalence. It receives **zero kill credit**.
- Error: 40 generated; **16 behavioral catches**, 24 compiler/lint-unviable,
  native exit **0**, viable score **16/16 = 100%**, no exclusion.
- Total inventory is 115, comprising 60 meaningful catches, 54 unviable and one
  equivalent miss. Both owners have **zero unresolved survivors and timeouts**.
  Every baseline runs the same 70 actual named tests. Compiler failures, bare
  aborts, OOM and timeouts receive no independent kill credit.

The direct helper controls actually catch the previously missed Wait, pending
expiry and reversed-shutdown guards. Other repaired test gaps check no further
preparation after startup expiry, the deadline as primary before later failed
completion, each single secondary fact and valid startup metadata. The three
new Stop OR mutants are caught by independently specified forbidden
Running/activation/health observations. A bounded healthy-driver child checks
valid argc=1, exact statuses, one remaining wait, success and completion marker;
its parent asserts exit code 0, preserving meaningful evidence even when a
defective child later hits a fatal custody guard. Tests retain the existing
Cargo/libtest workflow; finite scripts replace mutation-induced busy loops.

Earlier failures remain immutable: mutation v1 timed out; v2 produced the full
host 44/30/1/0 native outcomes but strict attribution rejected custom-message or
Result-return failures without explicit assertion evidence. The expectations
were made explicit, not weakened. Mutation v3 stopped after enumeration because
the reviewer's descriptive string exceeded the existing 200-character schema
bound; v7 review shortens only that identity text. No mutation ran in v3.
Only v4 is the final complete mutation pass. Earlier format, Clippy, RED and
incomplete metric records are preserved alongside the successful records.

The source-unchanged v6 fixture rustdoc/doc and v3 platform regression/doc
receipts retain their exact compiled scope: later changes affect only fixture
test assertions, not those compiled inputs. They are not a fresh whole-product
pass. The 17 metric and five portable-runner Python controls passed and were
independently repeated; their files are unchanged in the final candidate.

Independent `portable_runner_review` accepts the source repair, scope and final
bounded exit. Independent `fixture_metrics_review` recomputed all 115 outcomes,
70 baseline identities per owner, all failure-cause dictionaries and 249
retained mutation/proof file hashes. `service_generation_map` independently
reconciled the full inventory and the sole exact equivalence. Source, sealed
copies, vendor, tools, configuration and index bookends pass; raw exports are
complete. No source, artifact or index is staged, committed or pushed.

The final campaign retained the complete task proof snapshot with SHA256
`89bb275f4a04ef80d60bfb2440f652e56de3e783639bb579913f5b04e800febf`
under its exported `work/equivalence-proofs/tasks/windows11-integration.md`.
This closeout is appended after successful campaign bookends; it changes neither
the admitted Rust candidate nor that preserved historical proof snapshot.

**Scope and next owner:** the six tracked changes are this task, fixture-local
`qualification.py` and the four admitted host/error Rust files; the three new
files are fixture-local `metrics.py`, `test_metrics.py` and
`test_qualification.py`. Core/engine algorithms, shared manifests, lockfiles,
verification policies and shared runners remain untouched. Another AI's Core/
engine work is independent of this child.

The coordinator can review/adopt this tested source slice, then owns the next
explicit production-worker/handler integration contract and admitted source
composition. `InertWorker` and scripted controls are not production suppliers.
Actual Windows SCM/callback/completion and native-job termination/recovery,
accounts/provisioning/tokens/service SIDs/object rights, effective maximum
compatible mitigations, current linked-image/PE and installed/package/Store/
release evidence remain open under their existing owners. Native memory and
resource diagnostics remain held; no Windows, fuzz, storage, UI or formal hold
is lifted. Formal work remains post-V1. The metric record intentionally retains
`qualification_passed=false` for those broader native/product obligations.

##### Remote integration and commit preparation — 8 November 2026

David subsequently authorized pulling remote changes, resolving conflicts and
committing/pushing this service branch. It first fast-forwarded to remote main
`c4e21fd29906c9b48fed4e22fdd2386af2ca52b3`. A final remote check observed
`c4717471cbc87c53eea2e16b63ffa1e79028cb25`, which was also integrated before
publication. Both pulls reapplied only the nine explicitly selected service/task
paths from preserved private backups. Git merged the task histories automatically;
there were no unresolved textual conflicts. The incoming metadata preflight
repair, seven-member fixture corrections, published metadata CI record and
coordinator's reviewed frequency-prior contract/policy update remain intact.
The latter routes the other AI's next Core fixture work; this service child
changes no Core/engine algorithm or shared policy. Main/UI worktrees and their
existing changes were not altered.

Neither the first eight nor the later four incoming paths intersects the
service's 454 admitted inputs.
Post-integration source inventory and standalone manifest/lock/feature admission
match the final qualified candidate `7eb81c688ad538b2744253bf686657dda93e15e315d87d989064ddc1e43a515f`
exactly. Coverage-v3, mutation-v4 and the service Rust checks therefore retain
their exact source scope; the unrelated pull requires no repeat Rust campaign.
Independent `portable_runner_review` confirms that impact/scope assessment and
preservation of both evidence histories and all existing holds. The archived
task proof remains its historical snapshot, not the current mutable task file.

Commit preparation uses the complete materialized Git index, candidate-owned
entrypoints, the affected incoming Python controls, fixture runner/parser
controls and configured pinned Gitleaks controls/history/tree scanning. The
private publish receipt retains the exact index tree, source identity, command
results and commit/push outcome; only the nine service/task paths are staged.
GitHub CLI's configured identity has confirmed repository push permission;
publication uses that identity without changing global Git authentication.

**Publication-rule correction:** GitHub accepted the first local commit
`ffdbfed9be02c0f44a0c11634bf670b0b2e474da` but reported bypass of its verified
signature requirement and its no-merge rule for inherited main merge
`e0c8516db9d1b284eb410bc2c8bd548c103a35fb`. This event is retained; a later
correction does not erase it. Shared main history is preserved. The correction
uses GitHub's documented automatically signed `createCommitOnBranch` operation
on a temporary branch from the same reviewed coordinator baseline, verifies the
exact staged tree and signature, then replaces only this child's own new commit
with an explicit old-head lease. No account signing key, global signing setting,
hook or shared branch history is changed. The external publication receipt
records the final server-signed hash, signature verification and remote readback.

#### 4.35.10 W11-W05/W11-W08 portable service diagnostics child — 8 November 2026

**Owner and baseline.** This child continues the actual S3.1 invocation from
§4.35.9 in `/workspace/NevroTick-diagnostics`, branch
`service/portable-diagnostics`, baseline
`c4f434c5cbbe5702e5c126a869c46be0bb132ef4`. Core/engine algorithms, shared
manifests/locks, policies and schemas remain coordinator-owned. The ordinary
Core queue in development-plan §7 is unchanged. All native/Windows, fuzz,
storage and UI holds and post-V1 formal deferral remain in force.

**Bounded contract proposed for independent freeze review.** Reuse the original
`ServiceInvocation` decision owner and its earliest failure and three secondary
failure slots. A fixture-private portable diagnostics module owns a closed v1
event/field projection, bounded sink admission and loss accounting. It adds no
dependency, logger framework, file writer, native event provider, diagnostic
control endpoint, verbosity authorization or new failure chain.

The composition uses two separate finite in-memory sinks, each at most sixteen
fixed-size encoded records. Required lifecycle/failure attempts are independent
of the optional sink, including INFO events with optional output off. Enqueue
executes bounded ordinary memory operations: no callbacks, locks, I/O, retries,
flush wait or recursively logged failures. Full/unavailable sinks preserve their
first accepted records and count loss with saturating arithmetic. A sequence
cannot wrap or reuse an earlier identity. It orders only this invocation;
cross-process/restart identity, UTC truth and durable delivery remain unqualified.
Only role, event/version, honest severity, fixed failure category and local
sequence are admitted; no arbitrary string, error formatting/source chain,
packet, path, identity, key, cookie or approval token reaches a sink.

Events distinguish invocation start, acknowledged Running, closed admission,
retained primary/secondary failures and finished owner custody. Finished is not
an independent success acknowledgement and cannot occur while actual completion
remains pending. Repeated polls and repeated result reads must not duplicate an
event. Logging results do not feed readiness, admission, permit completion,
cleanup, terminal status or the retained original result.

**Acceptance and planned evidence.** CHECK-17: inspect actual admitted encoded
outputs for useful fixed fields and recognizable synthetic-secret exclusion,
including required INFO with optional output off and healthy/full/unavailable
sinks. CHECK-27/14: compare actual owner/status/worker traces and original failure
with each sink condition through pending readiness, Stop, cancellation, timeout
and late failed completion; no completion output or work-charge release before
acknowledgement. CHECK-21: finite records/memory/loss and no sink waiting; no
native performance claim. CHECK-26: unchanged dependency/features plus GNU and
Windows-target graph/lint lanes. Require the existing ordinary/optimized,
format/Clippy/rustdoc/doc checks, at least 90% executable-line coverage and 95%
mutation per affected critical file with complete generation reconciliation and
independently reviewed equivalences receiving zero kill credit.

The root agent owns `host.rs`, `lib.rs`, the new diagnostics module/unit tests
and this task. Separate workers own `host_tests.rs` and the fixture-local metrics
extension and its controls. Independent contract, integrated-source/privacy and metric
reviews are required before qualification closeout. No shared schema/policy
change is requested by this slice; any newly necessary shared change returns to
its coordinator before dependent work.

**Contract freeze review.** Independent read-only `diagnostics_contract_review`
accepted the contract before implementation. Explicit conditions retained:
stable wire mapping rather than Rust enum layout, useful lifecycle categories,
Finished distinct from successful Stopped, required INFO independent of optional
output, once-only failure projection, saturated loss and no source-chain
formatting. This is an eight-event fixture contract. Native/provider and
pre-invocation reporting, production metadata, runtime health consumption,
draining/flush and debug-session authorization remain separate gaps.

**Baseline execution.** Existing candidate
`7eb81c688ad538b2744253bf686657dda93e15e315d87d989064ddc1e43a515f`
(454 inputs) passed the fresh four-command GNU/Windows graph check and all
70 Rust tests (69 library + one entrypoint integration; none failed, ignored or
filtered). Records: `diagnostics-baseline-graph-v1.json` and
`diagnostics-baseline-test-v1.json` under this worktree's existing
`artifacts/windows-services/recovery-20261107-v1/` chain. Initial readiness
preflights executed no compiler command: the new worktree needed the existing
artifact directory, then the explicit installed vendor path instead of a missing
historical `ui-m0.json` pointer. Both prerequisites were supplied without changing
the runner. The successful graph/test receipts use the explicit vendor path.

**Causal output control.** `diag-red-test-v3.json` retained an intentionally
missing sink-byte copy on candidate
`f86e77d7e94c5defff57ef11e180feb4ec7f686ae4d6b20cde83f042d6f78961`. It executed all
82 library tests: 72 passed and ten output assertions failed, native exit 101,
with healthy source/tool/vendor/config/index bookends. Five failures were the
new actual-owner output/privacy/routing cases; five were portable output cases.
Restoring the single record assignment repairs that deliberate fault. This is
test sensitivity evidence, not a discovered pre-existing product vulnerability.
Earlier `diag-red-test-v1.json` refused a stale graph before compilation;
`diag-red-test-v2.json` retained two empty-array type-inference compiler errors.
The errors were corrected with an explicitly typed empty slice. Neither attempt
is behavioral RED. `diag-clippy-v1.json` separately retained three documentation/
empty-assertion lint findings; no lint or policy was suppressed.

**Integrated source checkpoint.** Independent `portable_runner_review` found no
source/privacy/custody blocker after the sink copy was restored. It required
direct default-composition and immediate startup-refusal output observations;
the added test now captures the actual default constructor's required bytes,
optional Off, first failure, fence and charge before any poll, for both roles
and healthy/argument-refusal/initial-status-failure cases.

`diag-test-v2.json` then retained 81 passing and two failing library tests.
Both failures were incorrect new charge expectations: revision 2 item 2 above
already requires physically acknowledged preparation to complete its permit,
and refused Running admits no running permit. Independent review confirmed
zero charge with unresolved cleanup/result in those two cases; pending
initialization expiry retains one charge. Corrected only those expectations,
adding cancellation-once and unavailable-result assertions. Production code and
the exact-output expectations did not change.

The frozen 456-input candidate is
`5fc4d5e36db86801abb0691ef3a95f69a8ea598aaefc51268a64026a2c14493c`.
Fresh graph (`diag-graph-v3`, four commands), GNU Clippy (`diag-clippy-v3`),
ordinary tests (`diag-test-v3`), format (`diag-fmt-v1`), optimized tests
(`diag-optimized-v1`), Windows-target Clippy (`diag-windows-clippy-v1`) and
private rustdoc (`diag-rustdoc-v1`) all passed with exact-source/tool/vendor/
configuration/index bookends. Ordinary and optimized each execute **84 Rust
tests: 83 library plus one entrypoint integration**, none failed, ignored or
filtered. `diag-doc-v1` ran the configured doctest lane with zero examples;
that supplies no behavioral-test credit. The 20 fixture metric and five
qualification Python controls also pass; they are separate from the Rust count.

| Frozen changed Rust input | SHA256 |
| --- | --- |
| `src/diagnostics.rs` | `326c39efd8da786441bdeda446815f06507e80ff16fe279addf156f61ff5e78e` |
| `src/diagnostics_tests.rs` | `e93b4f0dc0c67bc892a3161a3b2fc05e1f0d08a3ca4af3c8341d08ce69dec2ab` |
| `src/host.rs` | `3ff3b32d717448e78994a76047be57c0567af7561e2d1936530626f6bb1fbb96` |
| `src/host_tests.rs` | `873ad622beaac756ca9e7b215d676a4702c83bc281aeb225807fd11de5608a3e` |
| `src/lib.rs` | `599151e85cd7dd94ac9a3150364adc60a56a17d7b573f3c33d591ed6018e38f4` |

All five paths are relative to `tools/windows-service-fixture/`. The final
metric helper hashes are `906e52fe012f4b972c80fb9210ee51549de16aef7b50adc248217f4faac33237`
(`metrics.py`) and `952fe4802c6a57b42d0ca3d827f1d28a421270f62744caf952537db51ad26894`
(`test_metrics.py`). Shared manifests, locks, policy, schemas, platform and
Core/engine source remain unchanged from the agreed baseline.

**Coverage and complete inventory checkpoint.** `diag-coverage-v2.json` passes
all three separate executable-line gates on the frozen candidate: host
**507/538 (94.23791821561338%)**, error **127/127 (100%)**, diagnostics
**119/119 (100%)**. LLVM reports zero branch entries; this establishes no branch
coverage result. Its instrumented run retains the same 84 Rust test identities.
`diag-enumerate-v1.json` retains the complete nonempty inventories: host **85**,
error **40**, diagnostics **16**, total **141**, for independent function/guard,
derive/macro and cfg/unsafe omission reconciliation before mutation. Both lanes
preserve full custody and export their raw evidence. `diag-coverage-v1.json`
refused relative receipt paths before any command; the corrected invocation uses
canonical absolute receipt paths, without changing the source or runner.

The eight ordinary lane elapsed times were 15.99 s graph, 19.10 s GNU Clippy,
20.55 s tests, 14.26 s format, 57.34 s optimized, 21.26 s Windows-target Clippy,
18.41 s doctest and 18.76 s private rustdoc. The ordinary test body took 0.07 s;
retained lane time also includes exact acquisition/admission, sealed-copy work,
compilation where needed and custody bookends. No limit, pin, isolation or gate
was weakened to improve these timings.

**Final bounded source exit.** `diag-mutation-v1.json` passes all three per-file
gates on the exact 456-input candidate above. The 110.37 s campaign retained all
141 generated outcomes, all three 84-test baselines, each actual native command
exit, strict selected semantic-assertion/compiler cause attribution, full
source/tool/vendor/configuration/index bookends and complete raw export.

| Critical owner | Generated | Behavioral caught | Compiler/lint-unviable (zero credit) | Raw missed | Timeouts | Adjusted score |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| host | 85 | 48 | 36 | 1 exact reviewed equivalent | 0 | 48/48 = 100% |
| error | 40 | 16 | 24 | 0 | 0 | 16/16 = 100% |
| diagnostics | 16 | 3 | 13 | 0 | 0 | 3/3 = 100% |

Total: **67 behavioral catches, 73 unviable and one exact equivalent**, zero
unresolved survivors or timeouts. Host raw 48/49 is 97.95918367346938%; its native
exit 2 remains recorded for the actual raw survivor. Error and diagnostics native
exits are 0. The sole equivalent receives zero kill credit: current
`InertWorker::poll_prepare` at line 112, exact descriptor canonical SHA256
`c6d0657ebd35ce904a84c92a35f1e93573ab0de48da0ca7235f3198816a07da9`,
returns the identical `Ready(Ok(()))` through pinned Rust's `Poll::from` for the
full helper domain. Historical descriptors are not substituted for the current
one. `diag-equivalence-review-v1.json` binds the current source, candidate and all
three inventories and reconciles the unchanged method with S31-EQ-POLL-FROM.

`diag-inventory-review-v1.json` independently reconciles all production bodies,
guards, derives/macros and cfg/unsafe omissions against the complete inventories.
Diagnostics has only **three viable generated mutations**, not sixteen
behavioral kills. Its constructor, individual mapping/literal, encoding-order
and arithmetic generation omissions rely on the named independent exact-output,
privacy and boundary controls, not invented generated kills. All generated
descriptors have now been exercised. Windows-only source/native omissions retain
their native evidence gaps.

Independent `fixture_metrics_review` recomputed all 141 outcomes and assertion/
compiler causes from the archived logs, verified every current input and all
three complete inventories/baselines, and checked 773 noncompiler archive-file
hashes. It accepted the bounded metric/source exit with no blocker and the
low-viable diagnostics limitation above. Earlier independent source review
accepted field privacy, bounded sinks, original error/custody preservation and
the default/startup controls; the later oracle correction was independently
confirmed against revision 2 without changing production code.

The immutable proof used by this campaign is retained at
`diag-mutation-v1-evidence/raw/work/equivalence-proofs/tasks/windows11-integration.md`,
SHA256 `71caaf461aead45506ad8490a8975b2b17d5784cbce16d112a1f05beb11df8ac`.
That identifies the retained pre-closeout snapshot, **not this subsequently
updated task file**. Its raw review receipts and export remain in this worktree's
existing `artifacts/windows-services/recovery-20261107-v1/` chain.

**Goal disposition: complete within the frozen portable child.** The actual
service invocation now emits finite typed outputs at its decision owner, reuses
its original primary/secondary failure facts, attempts required INFO with optional
output off and preserves service readiness, admission, charged work, cleanup and
the original result under healthy/full/unavailable sink conditions. Actual
outputs exclude the exercised synthetic worker secrets and retained native
payloads while preserving useful event/cause categories. No shared manifest,
lock, dependency, policy or schema changed. At this qualification checkpoint,
work remained isolated on `service/portable-diagnostics`; no commit, push or
external publication had yet occurred.

**Owner-authorized local-main integration — 8 November 2026.** David subsequently
requested committing this child into `main`. The intended commit contains only
this task and the seven fixture implementation/test/metric paths above. Remote
main still matches the agreed `c4f434c` baseline. Preserve the two existing dirty
CodeQL/Slint handoff files in the primary worktree, verify the complete staged
tree and unchanged qualified inputs, run its Python/secret controls, then commit
and fast-forward local main. No remote push is part of this instruction. This
integration changes no source qualification or native/product hold.

**Native/product boundary and next owners.** Portable queue admission is a sink attempt, not
Windows Event Log delivery, installer/provider/channel resources, access,
retention, WER/dump privacy or a support export. Those W11-W08/ST-W07 and package
owners remain open. This child does not qualify production workers, service
accounts/provisioning, installed lifecycle, final mitigations, MSIX or release.
W11-W08 and the diagnostic/platform/package coordinator next own production
instance/time-quality/generation/correlation metadata and its shared schema,
runtime health consumers, finite drain/flush, actual event-channel resources/
permissions/retention and authorized diagnostic sessions. ST-W07 retains named
dump/export secret cases. W11-W05 retains real worker/provisioning and native
SCM/completion/recovery. Adopt those integrations only at their admitted phase;
the metric receipt deliberately retains `qualification_passed=false` for the
unqualified whole product. This completed source child lifts no execution hold.

#### 4.35.11 Production event logger plan — 17 November 2026

**Status and authority.** Documentation/planning continuation of §4.35.10 under
W11-W08, not a new logger workstream or implementation receipt. David requested
critical/high/mid/low/info events and a concrete module plan. The
[canonical classification contract](../docs/windows-time-architecture.md#1232-levels-and-required-events)
keeps operational importance separate from technical level and required routing.
Event-specific mappings and native descriptors must be frozen before dependent
implementation; this plan assigns no numeric encoding or Windows-level mapping.
F-09/AC-16 and CHECK-17 own the result, with CHECK-2/14/21/26/27 and the existing
storage/package checks at their affected boundaries. The active Core/Engine
queue, current Windows/fuzz/storage/UI holds and post-V1 formal deferral remain.
Planning alone starts no product, native or qualification campaign.

**Observed baseline:** read-only inspection at
`72b4ae5a92b413a45193e47ef7857aa60f3d524e` found the eight-event
[service fixture](../tools/windows-service-fixture/src/diagnostics.rs) with only
INFO/ERROR, two separate fixed memory routes and test-only record/loss readers.
Its actual invocation composition enables required memory output, with optional
output off; it supplies no production drain, installed event channel or export.
The [Engine observer](../crates/nevrotick-engine/src/bin/nevrotick-observe.rs)
has separate typed console diagnostics; the
[GUI diagnostic owner](../crates/nevrotick-ui/src/diagnostics_ui.rs) remains
presentation-only. Preserve those useful foundations and their evidence limits;
neither console text, fixture records nor dependency presence is an integrated
product logger. Recheck source/callers before the implementation child starts.

**Ownership and path.** Use
[architecture §3.4](../docs/windows-time-architecture.md#34-shared-windows-adapters-and-diagnostics)
and [§12.3](../docs/windows-time-architecture.md#123-logging-and-support):

| Owner | Enforced boundary and handoff |
| --- | --- |
| Actual operation/decision owner | Select the reviewed event/outcome once from admitted facts. Lower layers retain typed errors; completion records require the actual acknowledgement. Log generation grants no control authority. |
| Shared platform-neutral diagnostics | Private checked construction binds stable ID/version, permitted fields, level, importance and routing policy. Bound/encode before queue admission; foreign strings, error chains and whole domain objects cannot supply a record. |
| Each executable's composition | Initialize its one subscriber/router, finite required/optional allocations, writer lifetime and independent loss/health projection. Libraries cannot replace the process composition; engine logs do not pass wholesale through the broker. |
| `windows-platform` with W11-W08/W11-W10 | Implement native sinks against installed fixed provider/channel/resources and effective rights; preserve bounded failure, rendering and retention. Record/queue admission, registration availability, native invocation/result and independently observed retained/readable history are distinct. Unknown downstream loss remains unknown. |
| Existing secured read path and S5.3 UI | Publish bounded redacted event/history/health views with gap and process identity. UI previews and writes its own export. Authoritative synchronization state still comes from coherent snapshots; event text cannot manufacture it. |

No logger daemon, generic event bus, arbitrary privileged export path, new clock
permission or automatic telemetry is introduced. The separate correction
journal retains its durability/authority rules. ST-W07 owns sensitive-memory,
dump and export privacy; W11-W05 retains worker/SCM completion and provisioning.

**Staged implementation and exits.** Rows are a bounded dependency sequence,
not authorization to run held stages. Reuse their existing task/evidence owners.

| Stage / owner | Prerequisites and bounded change | Observable exit |
| --- | --- | --- |
| Portable catalogue and routing / diagnostics coordinator, service owner, privacy/QA | Freeze the first-stage contract below, exact source/feature/licence graph and finite portable limits. Extend/reuse the existing invocation seam with platform-neutral policy; choose a separate crate only when actual consumers need its dependency boundary. | Real admitted service decisions reach an inspectable bounded consumer; required INFO survives optional Off/WARN/ERROR and importance filtering. Closed classification, privacy, loss and unchanged operation outcomes pass independent portable controls. No native or durable-delivery claim. |
| Per-process composition and producers / W11-W05 plus each actual component owner | Consume the portable contract; freeze each process's budgets, producer instance/sequence/provenance, subscriber/bridge initialization order and partial-install disposition, and actual writer/guard owner. Add lifecycle, policy, denial, trust/source, bootstrap, correction, ownership, recovery and diagnostic-session producers only as their real owners exist. | A source-to-sink coverage table binds every required event to its reached owner/caller and consumer. Startup/stop/failure paths distinguish admission from writer health, preserve producer facts through delayed consumption, and report actual drain/completion or unresolved work without duplicate failures or blocked cleanup. Missing producers remain explicit open rows. CHECK-14/17/21/26/27. |
| Windows delivery and retention / W11-W08, platform, W11-W10, ST-W03/ST-W07 | Admit native execution; freeze provider/channel choice, message resources, event/version/level mapping, checked native descriptor/encoding envelope, effective access matrix, protected paths and flush deadline under §15.3 and metadata §12.3. Declare active-channel size/full behavior and any archive count/byte/age bounds with their authorized lifecycle owner. | On the exact installed candidate, intended writers/readers work; unauthorized publication/read/clear/configuration/export cases have their declared outcome. Known events render against matching resources. Registration failure, native rejection/no-recording-session, full/denied sinks, selected archive growth/recovery and flush/restart cases retain bounded useful health, privacy and safe cleanup. API success is not retained-history evidence; native/package evidence stays separate from host tests. |
| Authorized temporary diagnostics / P3 broker/helper/target owners, security/QA | Use the existing authorized command boundary and §12.3.3; freeze finite profiles, scopes, duration/byte caps and target/process/session binding. Normal required routing is already independent. | A current authorized session changes only optional collection; each target expires locally across UTC changes, suspend, restart, revocation or control loss. Warm an active verbose callsite, then expire/revoke while reset/reload fails: the next attempted verbose capture is refused despite cached interests/optimizations. Active collection and required INFO remain useful controls. Stale/replayed commands or partial activation cannot extend collection or imply success. CHECK-2/14/17/21/27. |
| Recent events and export / S5.3, diagnostics/ST-W07, UI/accessibility | Qualified per-process output/privacy, secured bounded reads, D2 status re-entry and applicable D1/D3/native gates. Connect one real event/history/export journey; W11-W09 separately owns production notifications. | All five importance categories remain distinguishable without color alone, with honest unavailable/lost/history state. Filtering never changes producer collection or required retention. Preview/export excludes secrets, reports incompleteness and uses ordinary-user destination authority. Closing UI leaves service ownership unchanged. CHECK-14/17/21/22/23/27, AC-16. |

The composition freeze accounts for producer evaluation/formatting/copying,
concurrent staging and selected retained span/worker state, in addition to queue
entries. Review actual blocking modes and feature/generated-code behavior before
admission; an inline buffer, post-format truncation or queue-count limit is not
the aggregate budget. Bound the record before allocation/encoding can grow
without limit. A threaded appender or compatibility bridge is not required for
the portable child. If selected later, classify prior-subscriber/prior-logger
conflicts and any partial global installation; no blind retry or library-owned
repair. Fence producers at the chosen shutdown boundary, preserve custody of
outstanding work, and report actual completion separately from unresolved/deadline
outcomes. Guard drop and drain acknowledgement alone do not establish completed
flush/teardown.

At the Windows freeze, distinguish active-channel configuration from measured
active-plus-archive allocation and missing-old/rejected-new history. Preserve
administrator policy; automatic backup is not a finite archive policy. Native
bounds include descriptor count, field order/types, transformed encoding,
terminators and relevant header/metadata/session-buffer limits. Reject a field
that cannot retain its admitted meaning; upstream/API ceilings are not selected
NevroTick allocations. Use the corresponding healthy/fault cases in §9.8.

If the selected native path provisions provider/session ACLs, compare the complete
resulting policy with the intended identity/operation matrix.
[EventAccessQuery](https://learn.microsoft.com/windows/win32/api/evntcons/nf-evntcons-eventaccessquery)
may return defaults without a GUID policy;
[add versus set DACL operations](https://learn.microsoft.com/windows/win32/api/evntcons/ne-evntcons-eventsecurityoperation)
preserve versus replace prior entries;
[EventAccessRemove](https://learn.microsoft.com/windows/win32/api/evntcons/nf-evntcons-eventaccessremove)
restores defaults. API success, one intended ACE or policy removal alone does not
establish the required restriction; defaults may satisfy the adopted matrix.
Pair useful intended access with missing-policy, additive-update and owned
maintenance/restoration cases under W11-W08/W11-W10 and CHECK-1/15/17/23/27.
Preserve administrator/successor changes; this selects no provisioning API or
automatic policy replacement.

If the S5.3 secured read adapter later selects native subscriptions/bookmarks,
freeze cursor identity, cleared/overwritten/unavailable history, reconnect
duplicates and partial-query status. Pair valid resume with an owned missing-
bookmark case and useful later records; require strict or equivalently evidenced
explicit discontinuity handling, never silently label resumed history complete.
This condition selects no reader or persistent cursor and grants no direct UI
channel access. General bounded-ring gaps retain their own mechanism.

If that secured reader selects native query/subscription/rendering APIs, retain
these cases with S5.3/W11-W08 and CHECK-14/17/21/26/27.
[Callback events](https://learn.microsoft.com/en-us/windows/win32/api/winevt/nc-winevt-evt_subscribe_callback)
are borrowed only during delivery and must not be closed; discriminate Action
before reading Event and hand off only bounded admitted owned data.
[EvtNext](https://learn.microsoft.com/en-us/windows/win32/api/winevt/nf-winevt-evtnext)
returns owned handles; account for its initialized count and
[parent-close invalidation](https://learn.microsoft.com/en-us/windows/win32/api/winevt/nf-winevt-evtclose).
Preserve setup/callback errors without interpreting a quiet subscription as healthy.

Freeze [query-handle thread ownership](https://learn.microsoft.com/en-us/windows/win32/api/winevt/nf-winevt-evtquery)
alongside the documented
[cross-thread cancel, synchronize and close sequence](https://learn.microsoft.com/en-us/windows/win32/api/winevt/nf-winevt-evtcancel).
Select the correct cancellation target, including session scope where required;
retain actual-work custody until pending calls finish. Neither cancellation nor
an elapsed deadline establishes completion. Pair healthy delivery with delayed
consumption, parent invalidation and cancellation during pending work, without
unrelated cancellation or leaked handles before harness cleanup.

Bind the complete locally constructed query to admitted sources: structured XML
can supply its own path, and
[enumeration remains live](https://learn.microsoft.com/en-us/windows/win32/wes/consuming-events).
Freeze finite record/work/result-window bounds and explicit incomplete history.
Pair useful permitted results with an attempted alternate source and continuing
publication; no raw-query interface or additional read authority follows. Check
[EvtRender byte counts](https://learn.microsoft.com/en-us/windows/win32/api/winevt/nf-winevt-evtrender)
separately from
[EvtFormatMessage character counts](https://learn.microsoft.com/en-us/windows/win32/api/winevt/nf-winevt-evtformatmessage),
typed property/count validity and bounded size-probe retries. Healthy rendering
and missing/oversized/mismatched values or formatting failure must preserve useful
bounded redacted status; localized text supplies no event classification or
application authority.

**Next portable stage: freeze, implement, verify, stop.** Start at
`ServiceInvocation` in `tools/windows-service-fixture/src/host.rs`, its existing
`diagnostics.rs`, `diagnostics_tests.rs` and `host_tests.rs`. The added observable
outcome is bounded record consumption and health/loss projection through that
actual invocation, not another enum or only repeated helper tests. The
consumer and health surface must be reached by non-test invocation composition;
new `#[cfg(test)]` accessors alone cannot satisfy this exit. Portable harness
observations still establish no native delivery or installed product behavior. The
coordinator freezes shared-module placement, exact source/feature graph,
consumer ownership and finite test budget here before coding; expand only this
stage. No proposed shared crate is presumed to exist.

1. Freeze a minimal initial catalogue from actual service decisions. Each entry
   names ID/version, decision owner, permitted cause/context, technical level,
   operational importance, required/optional route and coalescing rule. Record
   Critical/High/Medium/Low/Info examples independently; categories without a
   current real producer use explicitly synthetic controls, never invented live
   incidents. Translation, remote text and caller-supplied raw labels cannot
   change classification. Neither importance filters nor optional verbosity can
   suppress required events, admit DEBUG outside its session or authorize an
   operation/notification.
2. Define private fallible construction and checked transformations using the
   shared garde/domain contract where applicable. Explicit closed enums and
   typed event-specific fields enforce semantic combinations; generic strings,
   maps and a final scrubber do not. Preserve caller provenance without treating
   a role/instance field as proof of identity. Add process instance, local
   sequence, relevant generations/correlation and UTC quality with explicit
   unavailable states; no fabricated zero/current/trusted defaults. Freeze
   restart/sequence-exhaustion behavior and the scope of ordering before use.
   Capture these facts at the producer and preserve them through delayed
   consumption; a later writer cannot substitute its own time or context.
3. Freeze finite encoded record/field bytes, required/optional queue and recent
   memory capacities, per-key/aggregate rate and cardinality limits, counter
   saturation and overflow/coalescing policy for this portable slice. Required
   capacity must not be consumed by optional floods. Export, native retention,
   writer/flush and session ceilings remain named later-stage inputs, selected
   before their consumers exist. The fixture's 16-by-9-byte allocation and
   §12.3's proposed 10 MiB aggregate disk budget are not adopted product limits.
4. Wire the existing service decision owner through the checked record and real
   router to an inspectable bounded consumer; expose loss/availability through
   a separate health projection. Preserve earliest failure, actual completion,
   admission and cleanup outcomes under healthy/full/unavailable sinks, including
   an available queue with a failing consumer. A local drop count cannot establish
   successful writes or complete history. No sink
   callback, retry, blocking I/O or recursive failure logging enters the
   time-control/decision path. Retain original fixture compatibility explicitly
   or version the changed representation; never silently reuse its codes.
5. Independently check accepted events and rejected raw/transformed inputs,
   classification mismatch, required INFO with optional output off, flood and
   sequence/counter exhaustion, backward UTC/restart, synthetic-secret exclusion
   and useful ordinary diagnostics. Compare literal expected fields and actual
   operation outcomes, not solely encoder/decoder round trips. Include legal
   construction beside constructor bypass refusals, and healthy consumers beside
   faulty sinks. Extend existing tests/metrics at the affected owners; preserve
   90% security-module coverage and 95% critical-file mutation requirements with
   honest generated-fault reconciliation. Run only admitted applicable checks.

<a id="w11-w08-first-portable-contract--21-november-2026"></a>

**Whole first portable stage — contract freeze, 21 November 2026.** Planning and
fresh-context independent review are complete; source is applied and the initial
integrated development selection has executed. Current static/compiler/fuzz and
metric qualification below remains **in progress**. This continues all five obligations above,
not only catalogue enums. Baseline HEAD is `78f8ae931c7139d37e0ff36ed18e41dabf8c72ff`;
the preserved dirty 648-input source is `02c515363237d1c3c3f7adfc1808dcd0de62546105f97e83390d44675d0256e4`.
The root index remains unchanged. Current S3.1 qualification retains its exact
scope and receipts; this plan grants no new execution or qualification claim.

The exact contract is
`artifacts/windows-services/recovery-20261107-v1/engine-composition-20261120-v1/logger-stage1-contract10.json`,
SHA256 `220405f5b2d1ab006847cc2ed2c96ec265a8976d8689a0ed60b150e44dde7fd2`.
Fresh reviewer `/root/logger_stage1_plan_judge` read the actual source,
canonical authorities and frozen contract without conversation inheritance;
`logger-stage1-plan-review10.json` is **READY for this implementation stage**.
Its preserved predecessor `logger-stage1-plan-review09.json` was REVISE:
LOGGER09-R01 identified garde rejection allocation despite a no-allocation
claim; LOGGER09-R02 identified loss of an overflow fault when wire time becomes
Unavailable. Contract10 resolves those planning issues; the source and scoped
execution record below retain the remaining acceptance work. Reviewer agreement
is no product/native pass.

**Owners and contract.** Add the sole reusable catalogue/checked codec/router to
`crates/windows-platform/src/diagnostics.rs`; keep actual error projection and
service decisions in the fixture. No dependency or runtime is added. Preserve
all nine event IDs and their cause meanings: StartObserved1,
RunningAcknowledged2, AdmissionClosed3, PrimaryFailure4,
CompletionFailure5, StopReportFailure6, TerminalReportFailure7, Finished8,
OperationFailure9. Events1/2/3/8 are INFO/Info, the other whole-invocation failures
ERROR/High; all nine attempt the required route. No absent Critical/Medium/Low
producer is invented. Synthetic five-level by five-importance controls test the
independent policy. Optional Off/Error/Warn/Info and a checked five-bit importance
mask affect only optional duplicates; no DEBUG/TRACE session or knob is added.

Closed event-specific causes and private fallible construction own acceptance.
One mandatory root garde rule validates the complete fixed-size candidate through
one semantic owner, at most one error, empty root path and constant `invalid-v2`
code. Valid typed production does not allocate; rejected raw candidates can
allocate a bounded temporary Report, dropped before a closed error escapes.
No caller text, report formatting, arbitrary payload or error chain is retained.
Before final qualification derive actual requested rejection-allocation bytes from
the admitted garde entry layout and pinned Vec growth; the fixed-storage bound
below is not an allocator-total/RSS claim. No duplicated validator or waiver is
permitted to obtain an allocation-free label.

**Exact v2 encoding.** A record is128 bytes, integers little-endian; reserved
bytes must be zero. This coordinated private producer/consumer change preserves
historical v1 nine-byte literal evidence and rejects v1 at the new decoder;
there was no deployed v1 decoder or persistent migration to qualify.

| Byte positions, inclusive | Meaning |
| --- | --- |
| 0..7 | Version2, kind1, fixed component, event ID, technical level, importance, required route1, event-specific cause |
| 8..15 | Nonzero invocation-local u64 sequence |
| 16;17..24 | Lifecycle observation kind and checked elapsed nanoseconds; observed zero is valid |
| 25;26..41 | Process-instance tag and nonzero u128 claim when present |
| 42;43..50 / 51;52..59 | Policy/configuration generation tags and nonzero u64 claims |
| 60;61..68 / 69;70..77 | Association/clock generation tags and nonzero u64 claims |
| 78;79..94 | Correlation tag and nonzero u128 claim |
| 95;96..103;104..107 | UTC quality, signed Unix floor seconds, nanoseconds below1e9 |
| 108..127 | Reserved zero bytes |

Technical codes are ERROR1/WARN2/INFO3/DEBUG4/TRACE5; importance codes are
Critical1/High2/Medium3/Low4/Info5, never native Windows ordinals. Unavailable tags
are0 with canonical zero payload; observed metadata claims are1. UTC1 means
SystemWallUnqualified only; no trusted-UTC code exists. All current live issuer
fields/UTC are explicitly Unavailable. The actual fixed role, checked local
sequence and last genuine lifecycle observation are frozen before handoff; no
new clock read is added. Internal NotObserved does not fault the rate clock;
checked overflow/origin-order faults encode Unavailable but permanently fault
refill, while actual clock reversal retains the accepted coordinate as historical
kind2. Neither delayed consumption nor the passive decoder can mint producer
provenance. Decoded records have no route into the live producer queue.

**Finite fixture profile.** Each route owns16 pending and16 recent records;
the two routes consume8192 encoded storage bytes, plus fixed bookkeeping.
Bound diagnostic-owned fixed storage and simultaneous copies at24576 bytes;
verify actual layouts/lifetimes. Rate keys are exactly the nine IDs per route:
initial32 credits partition into nine first-ID reservations plus23 repeats,
with four credits per ID. Charge before queue attempt, never refund full or
unavailable admission. First valid time establishes an anchor without restoring
spent initial credits; only a checked nondecreasing1s window can refill once.
Bad/unavailable time never refills; overflow/reversal faults latch permanently.
Coalescing retains first bounded cause/counters without new cardinality.
Sequence exhaustion refuses reuse; all u64 diagnostic counters saturate explicitly.
These are portable fixture allocations, not shipped/native/disk/session defaults.

**Actual consumer and receipt.** The normal invocation pumps at most one original
per route after each lifecycle poll and at most16 per route when sealing.
Concrete Healthy/Full/Reject/Unavailable consumers perform no callback, lock,
I/O or retry; an available queue with rejected consumption remains observable.
Queue admission, consumer attempt/acceptance, refusal, overwrite and unknown
native delivery stay separate. Snapshot completeness covers producer loss and
both selected memory routes. Deliberate optional filtering leaves the selected
history complete when production and delivery succeed. A single private
`HostReceipt` pairs the original
operation result with its bounded snapshot, through `run_service`, normal native
OUTCOME and dispatcher projection. Pre-invocation registration/OWNER failures
retain their original error and useful PrimaryFailure without inventing
StartObserved/Finished or extra clock reads. Work/Stop/readiness/actual-join and
primary/secondary error precedence are unchanged; native executable workers
remain inert. Snapshot retention grants no service or clock authority.

**Implementation and QA exit.** Use the existing configured service and fixture
runners. Register shared diagnostics as a security module/critical file and
extend the existing service-analysis package owner for its actual coverage and
mutation selection; fixture-only instrumentation cannot qualify another package.
Preserve historical schema1-5 roles and current thresholds. Require independent
raw/tag/classification/version refusals and legal compiler controls, literal
producer-to-consumer observations, rate/flood/sequence/counter boundaries,
backward-time/restart and synthetic-secret neighbors, real lifecycle/fault
integration, ordinary/optimized/static/doctest lanes and Windows source compilation.
All affected owners retain90% executable-line and95% per-file mutation gates with
honest omissions/outcomes. Reuse unchanged error/worker evidence only through an
explicit caller/source bridge and fresh affected integration. Native delivery,
real Windows lifecycle, held fuzz/storage/UI, authorized verbose sessions,
production suppliers and post-V1 formal work retain their separate exits.

Planning applied plan-code-change, plan-architecture, plan-threat-model,
evolve-data-contracts, security-privacy-logging, qa-test-planning and
plan-independent-review to the actual owner/error/codec/consumer path and
THR-8/10/18 with CHECK-14/17/21/25/26/27. Implementation applies its Rust/service
and contract owners. Its current evidence is scoped below; unexecuted obligations
remain open.

**Applied source and development evidence — 21 November 2026.** The shared
`windows-platform::diagnostics` owner now supplies the sole catalogue, checked
128-byte codec, bounded routes, concrete consumers and inspectable health. The
actual service invocation emits through that owner and retains one `HostReceipt`
containing its original result and diagnostic snapshot. The normal native source
projects the same retained receipt; its worker remains inert. These are source
changes, not installed/native service qualification.

At source candidate `b34301289f1c9e0a0616e08eceff425a2a381e99b85be597fbd5e2bfcd1a8067`
(651 admitted inputs), the configured offline GNU runs passed112 fixture cases
(111 library plus one entrypoint),67 platform cases (22 new diagnostics cases,
45 existing) and26 doctests (18 new, eight existing). Receipts are
`logger-stage1-fixture13.json`, `logger-stage1-platform13.json` and
`logger-stage1-doc13.json` under
`artifacts/windows-services/recovery-20261107-v1/`. The writer's earlier claim
of14 external cases was a source-count error:13 external plus nine private
diagnostic cases actually executed. No missing case is counted.

Fresh-context source review found LOGGER-PIPELINE10-R01: decoded event/component
getters exposed active producer types. The repair returns separate passive
`ServiceEventClaim`/`ComponentClaim` observations with no conversion to active
producer authority. `logger-pipeline-source-review13.json` accepts that source
boundary for configured qualification. On private candidate `0fa4dffc…`,
`logger-codec-contracts31` retained both intended E0308 causes and executed
the legal consumer against the copied Cargo library. The doctests also passed.

Independent service-reader review found LOGGER-ADMISSION10-R01: crossed compiler
target/source attribution could exclude a schema6 variant from its viable
denominator. The bounded repair restricts attribution to the selected diagnostics
targets and their actual owned source. Its real-reader RED had five failing
subcases; GREEN ran28 wrapper controls. Eleven shared-profile and ten target/graph
controls passed separately. The public policy remains closed, with no invented
binding, inventory, exclusion or mutation credit. Historical schema5 criteria
and the90%/95% thresholds remain unchanged.

The first Rust attempts retain an unused-mut compilation refusal and a test
configuration-revision collision; the latter was repaired by distinct, bounded
test-owned source IDs, preserving production process-once admission. Subsequent
Clippy attempts retained production documentation/redundant-attribute and test
documentation findings. Their causal repairs preserve the snapshot type's
`must_use` obligation and all product guards. Failed receipts and owned failure
artifacts remain available; later attempts do not relabel them passing.

| Stage/skill and requirement | Actual action and status | Remaining condition/owner |
| --- | --- | --- |
| Rust/contracts/service/privacy; CHECK-14/17/21/25/26/27 | `logger-stage1-development20.json` records 12 passing lanes at `5d70e25c…`, including GNU and Windows-target Clippy. | Final combined review and affected checks after changes to bound helpers. |
| Positive/negative/boundary/property/model/integration/lifecycle/fault QA | Development20 passed 112 fixture cases and 67 platform cases per GNU test mode, plus 26 platform doctests. The tests include literal wire refusals, all-byte substitutions, bounded histories and real service/Engine consumer controls. | Windows behavioral qualification remains a separate obligation. |
| Contract/static QA | Contracts31 passed both intended raw E0308 checks and the executed legal consumer. Reader controls passed after repairs for cleanup error precedence, legacy helper custody and GNU graph admission. | Final candidate custody and review. |
| Structured fuzzing; P3 raw decoder | Fuzz32 admitted the GNU graph and ran 256 callbacks each with the healthy and restored decoder. It observed 145 accepted and 111 rejected inputs initially, and 146 accepted and 110 rejected inputs after restoration. Both fault controls reached their specified assertions and exited 77. The independent reviewer checked the raw callbacks and custody. | Fuzz27 and Fuzz31 retain their notice and inventory refusals. Fuzz32 covers decoder semantics on `0fa4dffc…`. ASan detector qualification remains open. |
| Resource QA | Development20 passed the strengthened fixed storage and single garde-report layout checks on the pinned GNU compiler. | The checks observe the selected layout and do not measure allocator RSS. |
| Coverage/mutation QA | Shared41 passed the current shared-owner line and mutation gates. Coverage35 and Adjudication38 passed fixture line and selected generated gates at their original source identity. | The reviewed fixture bridge below retains the scoped evidence, including the earlier unchanged-worker replay. Branch and native delivery evidence remain separate. |
| Native/sanitizer/UI/package/formal applicability | This source adds no unsafe/native ownership or UI/package effect; no new sanitizer/Loom/Miri campaign is selected for its fixed safe, synchronous memory owner. | Native delivery/service lifecycle, packaging and broader qualification remain separate open scopes. Formal execution is deferred by OD-10; AES, storage and UI holds remain. |

The original stage and codec execution windows expired over the interruption.
Their incomplete records retain the original deadlines. The 21 November resumed
checkpoint records its separate allowance and actual commands in
`logger-finish-owner22.json` under the same Engine composition evidence directory.
The root index has remained unchanged through these checks.

The integrated review found one reporting defect and one attribution defect.
The public Rust regression at `1669102f…` reached an optional consumer rejection
with complete required delivery, then failed because the aggregate snapshot
reported complete history. The repair includes both routes in the aggregate.
The intentional-filtering neighbor still calls neither the optional queue nor
its consumer. Isolated producer-counter cases exercise the exact limit and the
first saturated increment through actual emissions. Separately, ten synthetic
reader cases reproduced incorrect mutation credit from legacy service and
privilege errors under diagnostics schemas 6 and 7. The corrected reader
requires a diagnostics-owned assertion cause for those schemas and retains
schema 5's legacy handling. All 37 service reader, 193 shared analysis, 56
fixture-tooling and 39 environment controls passed on the integrated inputs.
The reviewer accepted the five-file source change on `cb8ada38…`.
Development35 passed all twelve configured lanes on the integrated 651-input
fixture candidate `51746b33…`. The public history regression passes in both
GNU test modes. Each mode ran 67 platform and 112 fixture cases. The run also
passed 26 platform doctests, format checks, GNU and Windows-target Clippy,
and fixture rustdoc. Its receipt is `logger-integrated-development35.json`,
SHA-256 `670be0660e6c6b02d4de0d2bbe2a80a4dcec2fa3a9cb8ff93c4ba09e7448a5e1`.

Independent comparison of all 641 retained Fuzz32 input paths found six changed
files in the new private snapshot. The Rust change affects only aggregate
snapshot reporting, and the target reaches the passive decoder and its original
helpers. The source changes to tests, mutation attribution and the private
binding leave the decoder, corpus, compiler profile, graph and fuzz driver
intact. Reuse Fuzz32 for that unchanged decoder scope at its original candidate.

Metrics33 retained both extension-discovery failures before any measurement.
The tools were installed with matching reviewed hashes. The launcher omitted
their directory from its host path. The graph command passed before the
launcher's additional free-space threshold stopped the batch. The next batch
includes the two installed extensions on its host path, keeps the configured
4 GiB disk stop and rebinds the corrected source.

The fixture's fresh ordinary run executed all 112 selected cases. Its
instrumented run passed the four file gates, with 602/645 host lines, 132/132
error lines, 90/90 fixture diagnostics lines and 186/193 worker lines. The
fixture enumerator retained 99 host, 42 error and six diagnostics descriptors.
These measurements belong to `51746b33…`. The subsequent shared enumeration
stopped before dispatch when the RAM workspace held the previous fixture builds.
After verifying the complete fixture receipts and exports and confirming the
failed lane had created no work directory, the coordinator retired that
namespace. Shared metrics use a fresh workspace under the same allowance.
Shared coverage36 passed with 884/912 executable lines (96.93%) and all 67
selected platform cases on `cb8ada38…`. Enumeration36 retained 398 exact
descriptors for the shared diagnostics owner. Neither line result records
branch evidence. The independent fixture inventory review covers all 147
listed descriptors and records the ungenerated diagnostic category arms.

Fixture mutation37 ran all 99 host descriptors. Cargo-mutants recorded 53
catches, 43 compiler refusals, three survivors and zero timeouts. The receiving
check refused three abort-only logs. Error and adapter mutation had not started
in that run. The three survivors match the independently reviewed current
equivalences recorded in [the fixture task](windows-service-fixture-equivalences.md#wsf-logger-eq-inert-poll-from).
Retain the original incomplete receipt. Adjudication38 uses the exact three
generated overlays through the supervised healthy-driver case, with clean
controls on both sides, and runs the outstanding 42 error and six adapter variants.
Adjudication38 passed those healthy controls and each exact overlay produced
the selected parent assertion with exit 101. Across the host file, the result
retains 53 attributable catches, 43 compiler refusals and three reviewed
equivalences. The adjusted score is 53/53 viable. Error passed 17/17 viable with
25 compiler refusals. The adapter passed 1/1 viable with five compiler refusals.
All three file gates passed with complete export and custody on `51746b33…`.

Shared mutation37 ran all 398 diagnostics descriptors. Cargo-mutants recorded
235 catches, 158 compiler refusals, five survivors and zero timeouts. The
reader refused 14 raw catch logs that did not contain an owned behavioral
assertion. Those include valid-input operation panics, iterator assertions in
the standard library and an iterator that drove a collector to process abort.
No raw catch count serves as a qualification score here. The original full
export is retained under `logger-runtime-receipts/logger-shared-mutation37`.
Two existing tests now observe equal and distinct causes independently, verify
chronological records and exact remaining iterator counts, and exercise the
first real history overwrite. A bounded test observer checks that valid
routing and iteration complete without a panic. It caps the sequence read at
the sixteen retained records plus one overrun observation. This leaves the
reader's assertion attribution policy in force.

After the WSL restart, retain the original completed fixture and shared
receipts. Charge the elapsed wall interval against the same checkpoint
allowance. Shared39 stopped at an unused test import. Shared40 caught all
five survivors with fresh artifacts, then failed assertion attribution
because the panic-boundary test used a custom assertion message. The ordinary
assertion form reached the intended reader rule in Shared41.

Shared41 passed 67 instrumented cases and 884/912 executable lines on
`ebcd4425…`. Its 93-case mutation baseline passed. The complete 398-variant
run caught all 239 viable variants, with 159 compiler refusals, zero
survivors, zero timeouts and zero exclusions. Independent artifact review
matched every viable row to fresh Build output for the platform library,
unit tests and diagnostics integration target.

The freshness review of the original Shared37 archive found five viable
rows that reused artifacts. In one row the reused executable produced the
previous overlay's panic. The schema 7 reader now checks each viable Build
record against its exact command and rejects stale, missing, duplicate,
wrong-phase and wrong-package artifacts. It checks freshness and behavior
from the same bounded log snapshot. The focused control reproduced six
missing refusals before the repair. All 38 service reader and 193 shared
analysis controls passed after integration. The repaired reader refused the
five old rows and accepted the unchanged Shared41 raw export, all 398
classifications and the 93-case baseline. That readback used the final
reader's own hash and retains the execution's original policy binding.

Only the two platform test files changed inside the fixture's admitted
source set after Adjudication38. One is compiled only with the platform's
`cfg(test)`, and the other is a separately selected platform integration
target. The fixture builds the platform library as a normal dependency and
executes neither test target. Independent review matched the complete
fixture context, all retained rosters, all five direct host build/test
bookends and the fresh raw artifacts for the host, error and adapter.
Carry the fixture line and mutation gates forward through that scoped bridge.

**Portable qualification exit — 21 November 2026.** Development41 passed
all twelve lanes at `6ee49a91b9bffbb6b531285465dbd2f72954803ee0042636ffa928ad6a621bd6`.
Each GNU mode ran 67 platform and 112 fixture cases. The run also passed
26 platform doctests, both format checks, all three Clippy lanes and fixture
rustdoc. The receipt SHA-256 is
`5b803a9baf2422f5ec8a2dd81e9a370acc4ec6a9b990684ca93fb12729aa5360`.
Independent closeout checked all 239 viable Shared41 entries against fresh
subject artifacts and the executed test paths, and accepted the original
execution and guarded readback for the scoped portable exit.

The first-stage catalogue, checked diagnostics codec, bounded memory routes,
real service event projection and original outcome receipt meet the portable
acceptance criteria. Next, freeze installed provider and channel delivery,
retention, access and cleanup under the existing W11-W08/W11-W10 plan.
Native delivery, credentials and provisioning, the two-family matrix, and
hardened MSIX qualification remain open.

The earlier S3.1 checkpoint was published as signed commit
`65b8e9a253fc6440beab61ebd53681940997380a`. Its detached staged tree matched all
648 qualified inputs, passed the redacted secret scan and retained the original
qualification scope. The fast-forward to `origin/main` was verified after the
push. GitHub reported its configured bypass for two pending checks. The
working checkout retained its active source and index during logger measurement.

The worker source and all 37 retained original mutation variants were checked
byte-for-byte. All historical behavioral kills used worker tests retained by
the current roster. The updated receipt projection preserves the assertions
and the expansion to 15 fixture source IDs reserves four IDs for the added
diagnostic cases. The current 112-case integration and worker line gate pass.
The historical worker replay remains evidence at its original binding.

The portable exit is a reached, bounded, inspectable event pipeline and health
consumer at the declared source boundary. It is not production Windows delivery,
qualified service control, complete event coverage or a GUI connection. Stop at
that exit; retain absent producers and later native/UI prerequisites in this
same chain instead of substituting more helper-only tests or launching a held
campaign. Freeze the integrated candidate before independent review and reuse
unaffected evidence under the existing testing contract.

**Threats and remaining decisions.** Existing THR-6/22, THR-8, THR-10 and THR-18
cover unauthorized control, flooding, disclosure and misleading status. Apply
the unchanged controls and valid/hostile pairs in
[hardening §9.8](../docs/windows-time-development-hardening.md#98-diagnostics-and-shared-adapter-verification),
including privileged fallback-path confinement and installed-provider spoofing
limits. The coordinator resolves event-specific classification and portable
limits first; native sink/resource/access mappings and finite disk/session
allocations stay explicit prerequisites for their stages. `tracing` remains the
candidate facade, with no dependency/version or generic filter-string adoption
here. Optional ETW, crash collectors and notification delivery retain separate
owners and evidence. This documentation amendment ran no product/native tests
and establishes no completed logging capability or release qualification.

**Independent plan review — 17 November 2026.** `logging_plan_judge`, fresh
conversation (`fork_turns: none`), completed read-only review of base
`72b4ae5a92b413a45193e47ef7857aa60f3d524e` and semantic candidate tree
`ae744836d401808243562388a7b22b7bbb3808c4`; this review record was added afterward.
The reviewer inspected all seven Markdown changes, the canonical/phase contracts,
predecessor task, actual service/CLI/UI source and selected fixture assertions.
Verdict: **ready for the documentation/planning handoff and its portable
contract-freeze step**; no material finding within that scope. Implementation
scheduling and native/UI holds remain unchanged. The coordinator checked all
17 added local links/anchors, preserved existing heading order and passed
`git diff --cached --check`; these are documentation checks only.

Reviewer comprehension, retained verbatim:

> The proposed change turns the existing diagnostics requirements into a staged delivery plan under W11-W08. It adds **Critical, High, Medium, Low and Info operational importance**, independently of technical log level and required-event routing, then connects the shared event contract to per-process composition, Windows delivery, authorized temporary diagnostics and the later UI/export consumer. The first portable stage starts at the existing service invocation and must produce an inspectable event and health pipeline, with catalogue, limits and ownership frozen before coding. The current development-plan §7 queue remains authoritative.
>
> The current source implements useful but narrower foundations. The service fixture has eight closed event variants, explicit INFO/ERROR encodings, separate finite memory routes and test-only record/loss readers; its ordinary invocation selects required output with optional output off. The Engine observer writes its own bounded console projections, while the UI diagnostics module explicitly owns presentation only. Production native delivery, runtime event/health consumption, comprehensive producer coverage, authorized diagnostic sessions and the GUI event stream remain planned or separately gated. The candidate preserves that distinction.

##### Windows logging research and judge dispositions — 17 November 2026

**Scope and completed inputs.** David requested separate Windows web research,
Rust/GitHub research, other-MCP research and an independent judge. All three
researchers and the judge used fresh task contexts (`fork_turns: none`) against
clean baseline `877aacf3fa7f422c7a4f0b3d6494ce7d457cf837`. `windows_logging_web`
completed W-01–W-07 from twelve opened Microsoft pages; `rust_logging_github`
completed R-01–R-07 from six pinned repositories, separating libraries/examples
from actual application consumers; `logging_other_mcps` completed M-01–M-08 using
Context7 resolution/documentation queries and Windows Learn search/full-page
fetch, then primary-source corroboration. Context7 IDs
`/websites/rs_tracing-subscriber_tracing_subscriber` and `/tokio-rs/tracing` were
discovery aids, not version pins. This bounded survey is not exhaustive internet
coverage, dependency adoption or native qualification.

`logging_research_judge` personally read all three completed reports, the current
canonical/phase contracts, fixture/invocation/Engine/GUI source, follow-up §4 and
metadata Appendix C; it independently checked the consequential Microsoft and
pinned Rust evidence. Verdict: refine the existing plan narrowly; its portable
contract-freeze step remains ready within the current schedule and authority.
The local J identifiers below track research disposition, not new acceptance
gates or demonstrated NevroTick vulnerabilities. Existing CHECK owners remain.

| Judge / research inputs | Disposition and retained owner |
| --- | --- |
| J-01 / W-01; R-01/02/04; M-03 | Adopt: separate known admission/refusal, registration, native result and observed retained history. §12.3.4, §9.8 and composition/native exits; unavailable downstream completeness stays unknown. |
| J-02 / W-03; R-02/03/04/05; M-02/08 | Adopt: account for producer work/staging/retained state and the later native representation. §12.3.4, resource/native controls and each stage's finite-limit freeze. |
| J-03 / W-04; R-02/05; M-01 | Adopt: producer provenance survives writer handoff and bounded session drain. §12.3.1, provenance controls and composition freeze. |
| J-04 / R-02/06; M-03/04 | Adopt: requests/acknowledgements differ from worker/flush/native-cleanup completion. §12.3.4, lifecycle controls and W11-W05 composition owner. |
| J-05 / M-01/07; W-07/R-06 context | Adopt: exercise a warmed callsite after expiry with failed reset and qualify the actual filter composition. §12.3.3, session controls and the later temporary-diagnostics exit. |
| J-06 / M-05; R-06 | Adopt: executable-owned initialization/bridge order and partial-install disposition, with successful and conflicting neighbors. Composition freeze and §9.8; no bridge selected. |
| J-07 / M-06; R-01/02/04 | Adopt: inspect reached formatter/span failures and fallback outputs under existing pre-format privacy. §9.8; preserve the source qualification below. |
| J-08 / W-02; R-03 | Small elaboration of existing policy: freeze selected full-channel behavior and any archive bounds/lifecycle at the native stage. Appendix C.9 already rejects unbounded automatic-backup assumptions. |
| J-09 / W-05 | Conditional later: explicit cursor/history discontinuity if the secured S5.3 reader selects native subscriptions/bookmarks. No new current-stage gate. |
| J-10 / W-06 | Conditional later: actual capture-clock and raw/converted timestamp provenance for optional ETW evidence. §12.3.4 and §9.8; no trace campaign selected. |
| J-11 / W-04/07; R-03/06/07 and overlapping R/M findings | Already covered: schema/importance, required routing, installed resources, separate ACLs, privacy, cleanup, session authority and dependency intake. Retain relevant source cautions below without duplicating policy or skills. |
| J-12 / all three tracks' limits | Reject unsupported uses: selecting a crate/API from examples, importing numeric defaults, claiming complete/durable delivery from counters, blanket filter/formatter leak claims, or promoting source research to product/native qualification. Preserve current priority and all holds. |

**Dated primary evidence and admission cautions.** Links pin inspected GitHub
source; manifest versions describe that source, not byte identity with a packaged
crate. Recheck the exact selected revision/features and reachable calls before
dependent implementation. No upstream code was copied or executed.

- Microsoft [EventWrite](https://learn.microsoft.com/en-us/windows/win32/api/evntprov/nf-evntprov-eventwrite),
  [EventRegister](https://learn.microsoft.com/en-us/windows/win32/api/evntprov/nf-evntprov-eventregister)
  and [EVENT_DATA_DESCRIPTOR](https://learn.microsoft.com/en-us/windows/win32/api/evntprov/ns-evntprov-event_data_descriptor)
  support J-01/J-02: write success can occur without a recording session; failed
  registration leaves an inert zero handle. Native descriptor/layout and total
  encoded-size constraints remain separate from product budgets.
- `tokio-rs/tracing` at `8d4d4d6ceaba222a1fcf94d98f030d2970a26611`
  declares tracing 0.1.44, subscriber 0.3.23 and appender 0.2.5.
  Its [enqueue/guard path](https://github.com/tokio-rs/tracing/blob/8d4d4d6ceaba222a1fcf94d98f030d2970a26611/tracing-appender/src/non_blocking.rs)
  copies before enqueue; its lossy counter measures enqueue failure. The
  [worker](https://github.com/tokio-rs/tracing/blob/8d4d4d6ceaba222a1fcf94d98f030d2970a26611/tracing-appender/src/worker.rs)
  can fail writes/flushes without updating that counter. `NonBlocking::flush`
  does not drain; guard drop neither joins nor returns worker completion, and
  a final flush follows its shutdown rendezvous. These details support
  J-01/J-02/J-04, not product deadlines or a durability promise.
- At that same pin, [initialization](https://github.com/tokio-rs/tracing/blob/8d4d4d6ceaba222a1fcf94d98f030d2970a26611/tracing-subscriber/src/util.rs)
  can install the global dispatcher before a failed LogTracer install when the
  subscriber's `tracing-log` feature is enabled. Earlier dispatcher failure is
  different; not every error partially installs state. The
  [Filter contract](https://docs.rs/tracing-subscriber/0.3.23/tracing_subscriber/layer/trait.Filter.html)
  and [reload source](https://github.com/tokio-rs/tracing/blob/8d4d4d6ceaba222a1fcf94d98f030d2970a26611/tracing-subscriber/src/reload.rs)
  support J-05/J-06: dynamic guard behavior depends on the full composition, and
  lock-protected mutation is not automatic product-level rollback.
- **J-07 source qualification:** the pinned
  [fmt layer](https://github.com/tokio-rs/tracing/blob/8d4d4d6ceaba222a1fcf94d98f030d2970a26611/tracing-subscriber/src/fmt/fmt_layer.rs)
  formats before enqueue. Span-creation failure Debug-prints attributes to stderr
  without consulting `log_internal_errors`; their ValueSet can contain values.
  The ordinary event-format fallback instead uses field metadata, not the event
  ValueSet. The judge checked [event](https://github.com/tokio-rs/tracing/blob/8d4d4d6ceaba222a1fcf94d98f030d2970a26611/tracing-core/src/event.rs),
  [span](https://github.com/tokio-rs/tracing/blob/8d4d4d6ceaba222a1fcf94d98f030d2970a26611/tracing-core/src/span.rs)
  and [field](https://github.com/tokio-rs/tracing/blob/8d4d4d6ceaba222a1fcf94d98f030d2970a26611/tracing-core/src/field.rs)
  implementations. Disclosure still requires unsafe values to reach an output;
  no NevroTick disclosure was observed.
- `microsoft/tracing-etw` at `a5f6815d659c5612fe440e14c749262b4229db02`
  (source 0.4.0) [discards native registration/write status](https://github.com/microsoft/tracing-etw/blob/a5f6815d659c5612fe440e14c749262b4229db02/src/native/etw.rs);
  its [inline formatter spills into a growing String](https://github.com/microsoft/tracing-etw/blob/a5f6815d659c5612fe440e14c749262b4229db02/src/values/mod.rs).
  `microsoft/tracelogging` at `d8e68c949160e63bde71fc5c815dffdc21c9a358`
  [defaults omitted activity to the writing thread](https://github.com/microsoft/tracelogging/blob/d8e68c949160e63bde71fc5c815dffdc21c9a358/etw/rust/tracelogging_dynamic/src/builder.rs).
  These support J-01–J-03; builder success, vendor identity and inline capacity
  do not qualify the selected product sink.
- `austinwagner/etw-wrapper-rs` at `8f050bef71d2844203866ab794dd39a8af4ed762`
  exposes [generated error-mode choices](https://github.com/austinwagner/etw-wrapper-rs/blob/8f050bef71d2844203866ab794dd39a8af4ed762/etw-wrapper-macro/src/lib.rs)
  and [field transformations](https://github.com/austinwagner/etw-wrapper-rs/blob/8f050bef71d2844203866ab794dd39a8af4ed762/etw-wrapper/src/field.rs).
  Actual consumers [USNJMon](https://github.com/jsmnsr/USNJMon/blob/21d7450c947d12ff708c0f09db97702183f74e3e/src/windows/event_log/mod.rs)
  and [Mullvad](https://github.com/mullvad/mullvadvpn-app/blob/a51da85f2b99d3cecbf08a7b55f79c3d2f527666/mullvad-daemon/src/logging.rs)
  illustrate explicit resource/status checks and writer ownership respectively.
  USNJMon's preview status and each consumer's local policy, allocation order,
  features and component licences require independent intake; examples are not
  transferable defaults.
- `locryus/eventlog` at `e39fddc5770ead28308b91d9848cbe67264aaead`
  [uses a debugger-only write-error fallback and empty flush](https://github.com/locryus/eventlog/blob/e39fddc5770ead28308b91d9848cbe67264aaead/src/lib.rs).
  Its [build script prints environment key/value pairs](https://github.com/locryus/eventlog/blob/e39fddc5770ead28308b91d9848cbe67264aaead/build.rs#L100-L103).
  Existing architecture §18.2/CHECK-26 dependency/build-script intake owns that upstream caution
  before execution. Runtime redaction cannot protect build output; this is not
  an observed NevroTick leak or a new scan campaign.
- Microsoft [channel logging](https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-channelloggingtype-complextype)
  and [wevtutil](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/wevtutil)
  support J-08's distinction between active capacity, full behavior and archives.
  [Subscription flags](https://learn.microsoft.com/en-us/windows/win32/api/winevt/ne-winevt-evt_subscribe_flags)
  support conditional J-09; permissive missing-bookmark/query recovery cannot
  silently establish complete history. [ProcessTrace](https://learn.microsoft.com/en-us/windows/win32/api/evntrace/nf-evntrace-processtrace)
  and [WNODE_HEADER](https://learn.microsoft.com/en-us/windows/win32/etw/wnode-header)
  support conditional J-10's ordering and timestamp-conversion limits. No blanket
  channel-disable rule or new trace controller is inferred.

**Integrated independent verification — completed 17 November 2026.**
`logging_research_judge` read the complete three-file amendment and surrounding
contracts at staged tree `34cfaa6b347d51df3838c8aed7815ad5744def09`, then verified
the affected correction at `5d90f91da5beda734b0649439c82403536d49140`, against
baseline `877aacf3fa7f422c7a4f0b3d6494ce7d457cf837`; the index matched that final
semantic tree with no unstaged changes. This attributed receipt was added
afterward without changing the reviewed contract. Verdict: **ready for the
documentation/planning handoff; no unresolved material finding in this scope**.

J-04 initially allowed an unresolved/deadline report to be read as ending custody
of unfinished work. The correction preserves custody and reports completion
separately; the reviewer verified it against architecture and hardening. J-05
now explicitly requires refusal of the next verbose capture after expiry,
alongside useful active/required-INFO controls. The reviewer verified consistent
integration of J-01–J-03/J-06–J-08, conditional J-09/J-10, and preserved J-11/J-12
limits. The coordinator checked the exact three-Markdown scope, existing heading
order, the added relative link/anchor, syntax and immutable GitHub revision shape
of 26 external URLs, and `git diff --cached --check`. These checks establish no
product/native behavior; no application tests, builds, dependency installation
or scanner/native campaign formed part of this documentation work.

Reviewer comprehension, retained verbatim:

> The amendment turns the bounded Windows, Rust/GitHub and other-MCP research into precise diagnostics requirements and acceptance cases in the existing architecture, hardening matrix and W11-W08 task. It clarifies observable delivery status, producer provenance and resource costs, writer completion, dynamic expiry and partial initialization, then retains native history readers and trace-clock evidence as conditional later-stage work. The task preserves source pins and J-01–J-12 dispositions without selecting a logger or creating another plan.
>
> The implemented system remains the narrower service-memory fixture, bounded Engine console output and presentation-only GUI diagnostic surface described in the original plan. The next logger child still stops at a reached portable event/health pipeline through the real service invocation. Native delivery, complete producers, service collection and GUI/export integration retain their own prerequisites; Core/Engine priority, Windows/fuzz/storage/UI holds and post-V1 formal deferral remain intact. The added documentation supplies no product, native or release qualification.

**Windows Learn follow-up — 17 November 2026.** David requested another pass
through Windows Learn MCP on logging and secure OS integration. The coordinator's
native-consumer track (C-01–C-04), `learn_logging_security` (L-01–L-04) and
`learn_service_integration` (S-01–S-06) completed their bounded primary-source
reads against clean baseline `c5a4625fb9cfb79d6b9aca8825a4900903fa0137`.
Each used actual Windows Learn MCP searches and full-page fetches. The two
researchers and `learn_integration_judge` used fresh task contexts
(`fork_turns: none`); the judge personally read all three completed reports,
reconciled current canonical/task/skill coverage and independently fetched the
consequential Microsoft pages. The sources for the additions are linked at their
existing owners above. These local finding IDs preserve research disposition,
not new requirements or demonstrated NevroTick vulnerabilities.

| Follow-up input | Disposition and retained owner |
| --- | --- |
| C-01–C-04 | Adopt conditional S5.3/W11-W08 reader cases: borrowed/owned handles, parent invalidation, thread/cancellation contract, complete query scope, live enumeration and bounded typed rendering. Preserve the documented cross-thread cancellation sequence; choose no wrapper topology or reader API. |
| L-01 | Adopt conditional W11-W08/W11-W10 access-policy maintenance cases: compare complete resulting rights with the adopted matrix, including defaults and additive updates. Policy removal restores defaults; it proves neither deny-all nor immediate revocation. Preserve owned maintenance and administrator/successor state. |
| S-04 | Adopt conditional W11-W05 enumeration cases: a missing row alone establishes neither absence nor access denial. Retain allowed/denied/genuinely absent controls and useful reconnection under CHECK-13/14/23/27. |
| L-02/03; S-01/02/03/05/06 | Already covered: separate channel/provider/session/file permissions, installed metadata versus process registration, exact actual-token profiles, service-SID/read confidentiality, minimum observation rights, supported MSIX pre-start provisioning and package/service-state recovery. Existing unresolved native/package evidence remains open. |
| L-04 | Conditional detail only: freeze actual writable channel properties and lifecycle effects if selected. Existing metadata/configuration ownership suffices; no generic channel-configuration interface follows. |

Only this task receives semantic additions; the general architecture, hardening
and skill contracts already cover their governing rules. The first portable
catalogue/router/health-consumer steps and their exit are unchanged. Conditional
native cases enter before their dependent adapters under existing owners; no
API/dependency, privilege expansion, direct UI channel access, numeric allocation,
new workstream or lifted Windows/fuzz/storage/UI hold follows. Core/Engine
priority and post-V1 formal deferral remain. These are planned controls, not
executed product/native tests or package/release qualification.

**Integrated follow-up verification — completed 17 November 2026.**
`learn_integration_judge` verified the complete 86-line addition and relevant
W11-W05/§4.35.11 context at staged semantic tree
`90ddb4571ad9940cb273f551f673009628149487`, against baseline
`c5a4625fb9cfb79d6b9aca8825a4900903fa0137`. The index matched the supplied tree
and had no unstaged changes. The reviewer reused its completed source
reconciliation and verified primary API passages; no new fetch was needed.
Verdict: **ready for the documentation handoff; no material findings or revision
required within this scope**. This attributed receipt was added afterward.

The coordinator checked the sole Markdown change, the syntax of all twelve added
Microsoft source links, unchanged heading order and byte-identical first portable
steps/exit, and passed `git diff --cached --check`. These are documentation
checks only; no product/native test, build, installation or scanner campaign ran.

Reviewer comprehension, retained verbatim:

> This candidate adds three conditional native-integration refinements to the existing Windows task: event-reader handle/query/rendering cases, provider/session access-policy maintenance cases, and service-enumeration visibility cases. It also records which completed research observations were adopted, already covered or left conditional. The additions retain existing acceptance owners and source links; they do not select APIs or move native work into the portable logger child.
>
> The relevant system contract remains a staged diagnostics design. The first logger child must reach a bounded portable event and independent health consumer through the real service invocation. Production Windows sinks, secured event history, SCM observers, complete service-token/package provisioning and UI integration require their separate later admissions and evidence. Logging cannot confer service/clock authority or manufacture readiness. The current Core/Engine queue, Windows/fuzz/storage/UI holds and post-V1 formal deferral remain in force.

##### Installed event delivery contract preparation — 21 November 2026

At `78c52cef65bf4161540316087f8febf4977d3779`, the fixed native service
entrypoints reach the qualified memory routes with the inert worker. The Engine
composition is exercised in the fixture tests. The installed provider name,
GUID, schema and message resource are still pending in the
[metadata source](../metadata/product.toml). The generator supplies partial
authoring inputs. There is no installed provider candidate to exercise yet.

**Package candidate.** Evaluate one enabled, manifest-based Operational channel
under package-level `desktop8:Extension Category="windows.eventTracing"`.
The schema supports fixed provider identity, package resource/message paths and
channel access, isolation and logging settings. Check service visibility before
interactive logon. Declaring `desktop7:Scope="machine"` requires the
`Microsoft.classicAppCompatElevated_8wekyb3d8bbwe` custom capability. W11-W10
owns establishing applicability and authorization for the real package identity,
then qualifying installation, upgrade, repair and removal on both supported
Windows families.
The separate SCM privileges, service SIDs and pre-start provisioning bridge
remain with W11-W05. [Provider schema][logger-appx-provider],
[Channel schema][logger-appx-channel], [Custom capabilities][logger-appx-cap].

**Publication and retention.** Treat installed manifest registration and the
writer's process-local `EventRegister` lifetime separately. Keep a failed
registration visible as a sink failure even though writes with its zero handle
have no effect. Evaluate Custom isolation for the required channel
since Application and System isolation ignore the write permissions in the
channel access descriptor. Custom isolation consumes a separate ETW session.
Freeze distinct rights for publication, provider registration/enabling, session
control and file access, channel read/clear/configure and redacted export. Check
legitimate and denied access using the actual tokens, including an unrelated
process requesting the same provider identity. The ordinary UI uses the secured
read path. Evaluate a finite
circular active log with automatic backup disabled as the first bounded profile.
Its overwrite behavior and history gaps must be visible. Derive a limit from the
selected event size/rate and history requirement, measure physical allocation
separately and preserve administrator-owned settings. [Runtime registration][logger-event-register],
[Provider/session rights][logger-event-access], [Channel isolation][logger-channel-type],
[Channel logging semantics][logger-channel-logging].

| Contract and owner | First discriminating observation | Disposition |
| --- | --- | --- |
| W11-W08, CHECK-17/21/27. Project `RunningAcknowledged` (ID 2, INFO) and the `PrimaryFailure` (ID 4, ERROR) from a refused Running report. | Required INFO with optional Off reaches the writer after actual acknowledgement. The refused report emits ID 4 as its first failure with no ID 2. The bounded handoff preserves original producer facts and accepts only a live producer record. | Source traced. Freeze the native descriptors, template fields and writer contract before implementation. |
| W11-W05/W11-W08, CHECK-14/17/27. Fence and drain to the admitted deadline, resolve native call custody and unregister the provider before the single `SERVICE_STOPPED` call on a complete shutdown. | A stalled or refused write leaves explicit loss or unresolved work. A deadline or successful write return cannot acknowledge teardown or durable history. | Source traced. Freeze the pre-terminal SCM disposition and custody for unfinished calls at the deadline. The owner determines `Finished` and any terminal failure after the `finish_stop` attempt, which can refuse before invoking SCM. Keep their installed-history obligation open. Windows can terminate the process after `SERVICE_STOPPED`. [SetServiceStatus][logger-service-stop], [EventUnregister][logger-event-unregister]. |
| W11-W10 with W11-W08, CHECK-18/23/26/27. Match descriptors and signed message resources to the installed publisher and package. | Independently query and render both events on the exact installed candidate. Distinguish missing resources, wrong identities, channel refusal and retention loss. | Pending final identities, capability applicability and exact package/resource inputs. `EventWrite` can succeed even when no session records the event. [EventWrite][logger-event-write]. |

Planning, metadata, privacy and package review applied to the publisher and
shutdown boundaries above. On this checkout, the metadata CLI validated 731
fields, generated 15 files into an owned temporary tree and passed read-only
`check`. Independent readback confirmed the partial MSIX inputs, unset provider
values and absence of an AppxManifest. The first readback used the wrong MSIX
JSON leaf name after successful generation and checking. The corrected readback
passed and the temporary tree was removed. No package build or Windows check ran.

Next, bind the candidate provider/resource values,
reader and writer access, and finite retention/drain policy to the existing
owners. Then implement the pre-terminal producer slice with independent write,
loss and completion controls. Native checks need that exact candidate and its
owned Windows environment.

[logger-appx-provider]: https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-desktop8-provider
[logger-appx-channel]: https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-desktop8-channel
[logger-appx-cap]: https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/app-capability-declarations#custom-capabilities
[logger-channel-type]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-channeltype-complextype
[logger-channel-logging]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-channelloggingtype-complextype
[logger-service-stop]: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-setservicestatus
[logger-event-register]: https://learn.microsoft.com/en-us/windows/win32/api/evntprov/nf-evntprov-eventregister
[logger-event-unregister]: https://learn.microsoft.com/en-us/windows/win32/api/evntprov/nf-evntprov-eventunregister
[logger-event-access]: https://learn.microsoft.com/en-us/windows/win32/api/evntcons/nf-evntcons-eventaccesscontrol
[logger-event-write]: https://learn.microsoft.com/en-us/windows/win32/api/evntprov/nf-evntprov-eventwrite

##### Two-event compiler input checkpoint — 21 November 2026

W11-W08/W11-W10, CHECK-17/18/21/23/26/27. The metadata CLI now admits an
optional, complete provider candidate from the owned TOML. It emits a fixed
Message Compiler input for ID 2 `RunningAcknowledged` and ID 4
`PrimaryFailure`, plus the same candidate identity and DLL leaf in the
existing authoring JSON. The Windows payload version is 1. Its five ordered
fields are the component, local sequence, lifecycle observation kind,
original elapsed nanoseconds and first-cause code. The portable record
remains version 2. The checked-in TOML leaves the provider candidate unset.
The emitted compiler input has a disabled channel and no deployment paths.
The installed-provider and MSIX obligations remain open.

Validation used the metadata, privacy and contract-testing skills. The
implementation worker supplied the typed TOML admission in an isolated tree.
The coordinator integrated it with independent output verification. The
fresh-context review accepted the combined source after the parser repair.
A real CLI regression first refused the newly specified candidate. It passed
after implementation. The review then found a producer mutation that placed
a nonbreaking space in an element-only container. The initial observer
accepted it. The repaired observer accepts only XML formatting whitespace
and rejects that output. The source controls also refuse a substituted
identity, channel, namespace, enablement, event ID, payload width and installed
path, and retain the absent-profile and well-formed fixture neighbors.

On CPython 3.12.3, `python3 -I -B -m unittest discover -s metadata/tests -q`
passed all 251 tests. The real validate → generate → check journey for the
temporary provider fixture produced 16 files. The 1,948-byte manifest's
SHA-256 is `f5239d8b6121ca99cc4db4edd0ccc6929137cf7f3038f3a6e2e43faadeddca6b`.
All 14 default payloads match `78c52ce` byte for byte. The generated receipt
binds the current checker and generator. The initial strict typing pass
found unchecked library types at two new boundaries. The corrected source
passed the configured `metadata/qualification/typing.py` lane with all eight
production modules, the legal consumer and five intended rejections. The
local evidence is
`service-event-manifest-20261121.752l_rie/typing/result.json` under
`~/.local/share/nevrotick/`, SHA-256
`77f0ae5841bf42dedb1df55d2e610c588c5b8ea91c249a1e081993976dc2b49f`.
The disposable authoring outputs, writer worktree and checker store were
removed. Full current per-file metrics remain open. The targeted producer
controls do not contribute to the generated mutation score.

The independent service trace found that one poll can refuse Running, emit
the first failure, and reach `finish_stop` before the outer diagnostics
pump. The writer child needs a reached pre-terminal handoff inside the Stop
owner before `finish_stop`. Test the same-poll refused Running path with
immediate worker completion, delayed writer completion and a terminal-only
failure control. Guard against re-polling an already finished worker when
the writer is still in flight. Preserve the first failure and the separate
post-terminal event meanings. Message Compiler readback, live writer
integration, package capability and installed behavior are future gates.

##### Service verification workflow repair — 21 November 2026

W11-W05/W11-W08, CHECK-19/25/27. The interval `20be018..c034f4b`
contains six commits. The repair order was to reproduce the reader's
false admission, trace the actual Cargo targets and consumers, then fix
repeatable setup, fixture and custody failures. The implementation and
checks use the existing standalone fixture and shared readers.

| Owner and guarantee | Observed counterexample | Repair or disposition |
| --- | --- | --- |
| Fixture mutation reader, Cargo Build → actual test executable | The same caught assertion survived a missing, cached, duplicate or wrong-command Build record. | Validate ordered Build/Test evidence from each bounded log, compiled roles for that file, launched program and causal observation. Apply build and roster checks to surviving overlays, including equivalents. Check direct worker replay separately against its own Cargo output. |
| Shared and wire tool admission, selected extension → private executable | The stripped supervisor PATH hid both installed extensions. Replacing the selected tool between its first hash and copy could make subsequent origin/copy hashes agree on different bytes. | Use the installed Cargo bin directory as the fallback for the two known extensions and verify the private copy against the original manifest pin. A mismatched explicit PATH selection still refuses. Retain final-reader and exporter controls alongside the selected producer mode. |
| Real service-worker fixture, scenario → process-owned source | Historical `engine-compose-full-ordinary02` and `logger-stage1-fixture11` each reached an unintended second initial installation. | Reserve one of the 15 process-local sources under the authority lock before binding a socket. Callers no longer assign the index. Supervised children own a fresh process and catalogue. |
| Fixture campaign owner, raw work and cumulative time → retry/retirement | Native05 lost its dirty derivative after the reserve stop and failed export. The later Shared41 supervisor could also choose unmount with a missing expected directory and an unaccounted sibling. The latter was reproduced at the decision callback. | Use the [disk-backed stage](../tools/windows-service-fixture/README.md) for the next fixture metric campaign. Bind the original stage/boot/deadline, retain original files on failure, and admit export independently of score. Native05 stays incomplete. |
| Engine response and service diagnostics, accepted input → actual consumer | Version/mode and authenticated inner plaintext needed independent controls despite high generated scores. The diagnostics owner needed distinct cause and overwrite histories. | Existing consumer tests already exercise these cases. Keep the healthy v4 server next to the two header refusals, and the authenticated empty body next to the valid-tag forbidden body. Retain the diagnostics' first-cause and chronology checks. These controls add no generated kills. |
| Engine scope inventory, managed configuration → joined result | The manifest still described only never-send Preview and denied protected integration. The Sandbox consumer calls `invoke_nts_protected` with the bounded successor plan. | Correct the two scope descriptions. Keep the separate Preview and explicitly authorized protected loopback contracts. At `c034f4b`, the closed KE binding differs for managed preview, datagram, response and source policy. Historical scores retain their original graph, test selection and candidate. |

**Evidence and reuse.** The original fixture cohort is `51746b33…`.
`logger-fixture-mutation37` contains the 99 host variants.
`logger-fixture-adjudication38` contains 42 error and six diagnostics
variants plus separate supervised host observations. Reader-only
reconciliation checks the original export hashes and the 112-case ordinary
roster. Three raw host aborts still need that separate observation. The
37 worker variants belong to `engine-compose-final-worker-replay08`.
The shared platform's five stale Shared37 artifacts were a separate
incident. The earlier response, service and metadata exits retain their
recorded scope. The first failed collector, stop and export receipts remain.
The existing outer owner recorded 10,537.0987 seconds used from its
11,100-second allowance across attempts. Native05's lost bytes cannot be
reconstructed from that timing. No second loss or stale historical fixture
execution was established.

The two execution modes have different pilot evidence. The ten-command
`engine-compose-worker-pilot03` recorded the direct worker's healthy,
faulted and restored results and completed its export. In the Cargo-mutants
mode, Mutation37 completed its host export but remained incomplete, and
Adjudication38 recorded restored derivatives for error and diagnostics.
The revised consumer checks these actual logs. When the producer, target
layout or export path changes, exercise that mode's fault and restoration
before admitting a full cohort. The disk-backed runner's first full run
still belongs to the next admitted campaign.

The additional wire check found one inherited oracle mismatch. Commit
`0e19ed1` changed the private `RawDatagram` field from `tail` to `input`
and updated its construction probe. Of the 89 historical consumer
definitions since `e76863f`, that one literal changed and all diagnostic
requirements stayed identical. The source-inventory test now checks the
current field and the reviewed complete fingerprint.

**Final reader and interrupted-run controls.** The independent review
found that an unrelated executable could still supply an assertion, and
an interruption after provisional scoring could leave positive flags.
The revised reader checks which executable actually failed and checks
compiler refusals against the fixture package, manifest, target and source.
The runner clears provisional credit on interruption. A real child wrote
a known line, timed out and was reaped. The receipt kept its command,
output, exit code and timeout reason exactly once. Output still buffered
inside an interrupted child may be unavailable.

For the combined 652-input fixture candidate
`17f71526efa15c7be7f8078f73a62445eddb659eafa945019db348e582999ae7`,
68 fixture and 336 selected shared-reader, environment, wire and Core
Python controls passed (404 total). Hash-bound readback of the 147
Cargo-mutants and 37 direct worker variants took 1.09 seconds. The reader
refused all three abort-only records. The 20 direct behavioral catches and
17 compiler refusals agree with the original replay. The checks provide
no new candidate mutation score. The final independent reviewer accepted
the tested attribution, interruption and deadline corrections.

The configured Rust 1.100 GNU fixture lanes passed with that same input
digest and verified source and index before and after execution. The four
`workflow-*-2111c.json` receipts in `artifacts/windows-services/recovery-20261107-v1/`
record six graph commands (27.35 s), formatting (22.49 s), all-target
Clippy (29.75 s) and 111 library tests plus one entrypoint test (36.50 s).
These are successive whole-lane times, including preparation and checks.
The first graph attempt `workflow-graph-2111a` stopped before any command
because its outer namespace lacked a usable `/dev/null`. Providing the
device corrected that setup. The two temporary build and comparison
directories were removed after checking the durable receipts. The full
disk-backed mutation campaign remains open at 4.3 GiB available versus
its 6 GiB launch requirement. Windows and formal execution were outside
this workflow repair.

The portable service and protected-successor work remain genuine product
progress. The next Windows delivery checkpoint still requires the
pre-terminal writer handoff and Message Compiler/native readback described
above. No mutation threshold, package gate or post-V1 proof decision changed.

##### Portable pre-terminal writer handoff — 21 November 2026

W11-W05/W11-W08, CHECK-14/17/27. Baseline `82ea4c6` begins with an empty
index and worktree. On this baseline the outer memory pump follows the
terminal decision. The same poll may refuse Running, record its first
failure, retire the worker and attempt Stopped. Move the selected writer
handoff into the serialized Stop owner before `finish_stop`. Fence writes
at Stop entry and observe both worker and writer custody before the terminal
attempt. Keep the memory router available for terminal facts.

Use the shared checked producer for a live handoff of IDs 2 and 4 admitted
to the required queue. The fixture's writer can accept live records until
Stop begins. It retains the one invocation's original records and
uses the already established Stop deadline. Expiry stops new write work;
outstanding work keeps its owner in custody until completion or resolution.
An acknowledged close with loss preserves the original lifecycle outcome.
First causes discovered after writer fencing and terminal decision facts
remain available through the memory route. No native provider is selected
for this portable handoff.

Applied planning and dispatch skills to complementary fresh-context reviews
of the actual host Stop path and the compiler-input-to-package boundary.
The host review identified `finish_stop`'s local refusal and the distinction
between a terminal-only ID 4 and an ID 7 accompanying a previous failure.
The package review found no Microsoft Message Compiler in the inspected
tool locations. It identified the need to read the exact compiled descriptor
and message resource independently under W11-W10/MD-P4. The worker owns
the shared record under the Rust contract and diagnostics privacy skills.
The coordinator owns the fixture lifecycle and independent reporter/writer
trace, using the service, lifecycle, contract and regression skills.

The source connects the checked writer records to the actual fixture lifecycle
with an injected portable writer. The fixed Windows compositions still select
the memory routes. The writer closes independently of worker cancellation.
An already finished worker is never re-polled for a writer wake. A fresh time
observation after each writer poll prevents a stale positive wait. The original
deadline accompanies drain, and an unresolved drain that crosses the cutoff
immediately receives retire-only mode. Timeout and destruction supply no
completion. Early local refusal contributes to a loss observation in the
portable control and preserves the service result.

**Writer join-tail continuation.** At `82ea4c68710f` with the preceding
working changes, the W11-W08/CHECK-17/21/27 service owner now receives an
explicit pending disposition. A writer that has published its final marker
can still be awaiting physical exit. The invocation retains that writer and
uses the existing bounded relative join observation even after expiry. The
original deadline and retire mode remain in force. The `Stopped` attempt
requires both resolved owners. This reaches the fixture host with an injected
writer.

Applied the Rust contracts and service skills to the writer-to-host handoff,
and the lifecycle skill to the actual host control
`overdue_writer_final_marker_keeps_join_observable`. The fixture first leaves
the overdue owner waiting for a notification, then publishes the final marker.
The control checks that the marker retains join-tail custody through repeated
polls and that the separate writer completion permits one terminal status
with the original deadline failure. The control scripts the
writer observation. A real writer thread still needs a held-after-marker exit
control before portable backend qualification.

Independent source review traced both completion owners, the final-marker
wake and the adjacent cutoff control. The reviewed type-binding suggestions
are applied. Source SHA-256 prefixes are `007f67b842b1` for `diagnostics.rs`,
`67aec63c16cc` for `host.rs`, and `c929939cf9a4` for `host_tests.rs`.

The focused Rust 1.100 rustfmt and `git diff --check` passed. Fixture compilation,
the new control and coverage/mutation execution await WSL ext4 capacity. The
configured runner requires 4 GiB free through ordinary development; the durable
metrics stage requires 6 GiB at launch and retains the 4 GiB reserve. Read-only
inspection found no disposable set large enough to clear these floors. The
registered Ubuntu WSL 2 disk is under `D:\Ubuntu`, where about 73 GiB is free.
Resizing that disk requires a scheduled WSL shutdown. Existing service receipt
and tool custody remain in force.

**Portable threaded writer source.** W11-W08/CHECK-17/21/27 now reaches a
real writer thread through the fixture's service invocation. Rejected
startup leaves the writer dormant and retains the original error in memory.
The admitted writer starts on the first selected record. Its finite mailbox
holds one queued record and allows one writer-owned record. The service
publishes the writer fence before attempting `StopPending`. A write claim
checks current mode and the original exclusive cutoff. Retirement discards
unstarted work and retains the running backend. Backend close precedes the
final wake. A pending thread tail retains the original join handle until
physical completion. The portable backend contract requires finite cleanup,
including unwinding and thread-local teardown. Native effect admission and
provider teardown have their own contracts.

The three added controls select actual rejected startup, a held backend close
and post-wake thread tail, and a held first write with one queue slot. The
queue control pairs the expired drain with a healthy second write and offers
a later deadline to detect accidental renewal. The host control releases
both gates and drives the original invocation to completion before returning
an observed failure. An independent source reviewer checked gate ordering,
deadline custody, panic cleanup and the closure-to-thread-exit interval.
The review led to a tighter final mode check, an explicit portable cleanup
contract and bounded test retirement. Rustfmt and whitespace checks passed.

The initial candidate at `82ea4c68710f` with the preserved working changes
recorded source SHA-256 prefixes `6f8d17790938` (`diagnostics.rs`),
`17c052b25c7c` (`diagnostics_thread.rs`), `7ed3235e6988` (`host.rs`) and
`f08e5d89f11d` (`host_tests.rs`).

**Portable writer development execution.** WSL cleanup cold-archived 12 older
successful raw trees into a POSIX tar archive in the user's existing Windows
NevroTick data directory. Complete export-map digests and archive comparison
passed before removal of the original ext4 copies. Current reuse carriers,
incomplete stages, local top-level receipts and export maps retained their
original locations. The measured net recovery was 1,124,413,440 bytes. The
archive SHA-256 is
`198854409fd47890d45b9873c8c626f2bd4b4f055baade6b74205be14a08120a` at
`/mnt/c/Users/perso/AppData/Local/NevroTick/service-cold-20261121.qobzvd8z/service-history.tar.zst`.
The nearby `custody.json`, `retirement.json`, and `RESTORE.txt` record the
original paths, identities, measured space and exact restore selection.
The follow-up WSL cleanup retired six duplicate fixture source seals, the
deferred local Verus/Z3 tool payloads, and byte-matched extracted dependency
scratch. It recovered a further 433,217,536 bytes. Both distinct fixture
test seals, the 57 original crate archives, selected vendor roots, proof
sources and current receipts remain available. Post-V1 proof execution
requires reacquiring the removed pinned tool payloads.

The existing fixture runner then admitted candidate `aedffe5acdfd…` and
passed 120 library and one integration tests. All new portable writer
controls and the preceding overdue join-tail control executed. The first
GNU Clippy check refused the missing `# Errors` contract section on
`EventWriter::poll_close`. Its failed result and bookends are retained as
`writer-threaded-clippy-2111a.json`. The repaired candidate changes only
the trait's documentation from the tested predecessor and was rebound and
re-executed through the real fixture runner.

The repaired 653-input candidate is
`99c3c78337cbcd31c537d2513b10bad85e720c8413fdf3b04e3ed48d9fba26f9`.
Receipts in `artifacts/windows-services/recovery-20261107-v1/` share its
admitted graph, toolchain, vendor and fixture configuration. The graph ran
six commands. Ordinary and optimized each executed 120 library plus one
integration test, with zero failed, ignored or filtered selected cases.
Both test lanes exercised rejected startup, the queue deadline, and the
held-close/final-wake/physical-join sequence.

| Lane and receipt | SHA-256 | Elapsed |
| --- | --- | ---: |
| `graph`, `writer-threaded-graph-2111b.json` | `6c83ef77fa9c7d684e8c608d41cb0b0a1824e53d37a116f8f16c5598411896a6` | 26.72 s |
| `fixture-clippy`, `writer-threaded-clippy-2111b.json` | `2824fa496749241e6dc14124ce344d519b45cbb610d1a7c598ea859cde3bce5b` | 31.50 s |
| `fixture-test`, `writer-threaded-test-2111b.json` | `8bbb346eb5f67aeaeb36617a911f9b016a24e8e3dd46b186991692803b6562ff` | 36.44 s |
| `fixture-optimized`, `writer-threaded-opt-2111b.json` | `3c5c4e238f522805e09fa0c321237302bff717a33baf037591692dfad9c1b2f7` | 75.75 s |
| `windows-clippy`, `writer-threaded-msvc-clippy-2111b.json` | `a6e665755323ac5b5d7218c27bea642a456bfafeb1c5ac088028e17b12f1312f` | 34.07 s |

Final readback matched the 653 physical inputs across the five receipts
and live source. Each lane retained its source, tool, vendor and index
bookends. Both test rosters contained all four writer controls. The MSVC
Clippy lane selected `x86_64-pc-windows-msvc` with `--all-targets`.
The completed temporary target directories were retired after checking
their receipts and process ownership.

Coverage and mutation require an explicit implementation-only subject for
`diagnostics_thread.rs`, exact test-frame attribution, a fresh binding and
the durable 6 GiB admission. The fixture-only writer needs the existing
direct replay pattern for test-configured code. The maintained thresholds
remain 90% executable-line coverage and 95% per-critical-file generated
mutation. The native provider and installed retention keep their existing
admission and exit requirements.

**Writer metric scope and reader controls.** W11-W08 now selects the portable
writer implementation as a separate coverage owner. Its queue control lives
in `diagnostics_thread_tests.rs`, so that control contributes no writer
executable lines. The writer's direct enumeration and replay lanes bind the
exact file, test roster, independently reviewed descriptor inventory and
separate overlay archive. A behavioral catch must identify the named writer
control and its expected test source. The existing worker and cargo-mutants
owners retain their respective command, artifact and scoring rules. A writer
pilot establishes the execution and restoration path. Per-file qualification
still requires full line and generated replay evidence plus independent
readback of archived overlays after the final subprocess.

The mutation and regression-selection skills guided the separate subject,
retained attribution and fixture-reader selection. The first 72-case Python
fixture run had four errors because a synthetic compiler row lacked its
source filename. The corrected fixture reflects the tool's observed output.
`python3 -IB -m unittest discover -s tools/windows-service-fixture` passed
74 cases, and the separate service-analysis reader selection
(`-s scripts/tests -p test_verify_service_analysis.py`) passed 38.
Controls check required roster admission, wrong-owner receipt and archive
refusal, valid and wrong-file assertion attribution, and separate line
denominators. Pinned Rust 1.100 formatting passed. Independent source
readback compared the extracted queue control to the sealed predecessor and
confirmed the same test body, module name and writer implementation.

The first 654-input metric-scope candidate is
`2533c0de5057601eb634054d156685267bb468aa433e166177fb4d56ec6b9071`.
The 653-input predecessor differs in only the fixture README, metric reader,
reader tests and writer test extraction, which adds one physical file.
That candidate's source hashes match the retained receipts under
`artifacts/windows-services/recovery-20261107-v1/`.

| Lane and receipt | Result | SHA-256 |
| --- | --- | --- |
| `writer-metric-scope-graph-2111a.json` | Six commands passed in 25.77 s | `313eee5ffcd7bd0bf4f12617ed14635862873a623954b1b6e139773c353b8992` |
| `writer-metric-scope-clippy-2111a.json` | GNU fixture Clippy passed in 32.69 s | `e53f1dd11528bf3a803bfadfbd97db1bbdc663610002df64472bcf6f6e1f2ea2` |
| `writer-metric-scope-test-2111a.json` | Disk reserve stop while compiling, exit −9 in 29.04 s, zero tests executed | `99657917663dcd8965949532784887711918f1450b12619ae4f5331bdd2b1b3b` |

The stopped build's private target was retired after process and mount
checks. The failure receipt and source seal were retained. This recovered
192,741,376 bytes and left 4,833,325,056 bytes free. Resume with an ordinary
test receipt for this exact candidate, then the existing durable WSL
metrics route at 6 GiB free. Supply `--durable-stage` and
`--deadline-monotonic` as documented. The earlier 121-case result remains
evidence for its own candidate. Writer coverage, generated replay and
installed/native delivery remain open.

**Current ordinary writer prerequisite.** After integration of the linked
PE checks, the shared verifier changed the fixture's complete input map.
Environment preflight and regression selection used the current graph and
unfiltered GNU `fixture-test` commands on the unchanged Rust 1.100.0 tools
and admitted vendor. A private 3 GiB tmpfs over the ordinary work cache
kept the compiler outputs off ext4. The outside supervisor held the
canonical writer lock. An ordinary UID with zero effective capabilities
started each original qualification command inside the private mount. The
runner retained the 4 GiB root-disk and 1 GiB available-memory guards.
Raw receipts stayed on ext4 and both owned scratch mounts retired.

The first retry's outer read-only mount left `/proc` read-only and the
first metadata command failed to set up its nested UID map. A separate
probe confirmed that giving the outer mount its own PID namespace and
procfs permits the nested sandbox. The corrected attempt used fresh
labels and the same cumulative allowance. It took 60.81 seconds after
80.31 seconds had elapsed from the first recorded start, within 310 seconds.

| Receipt in `artifacts/windows-services/recovery-20261107-v1/` | Result | SHA-256 |
| --- | --- | --- |
| `writer-current-graph-2111b.json` | Failed namespace setup, zero tests | `916837f398c320ca59ad4e3cb4243443bd57011e10139a136aee0e5e6ed28009` |
| `writer-current-graph-2111c.json` | All six GNU/MSVC graph commands passed | `dcc98522f4c2c2a33385dfc687580397ae30e56d73ac5e4c8696faf546cb6a2f` |
| `writer-current-ordinary-2111c.json` | 120 library tests and one integration test passed | `56b43321a7a3d2b519137fa93ace0b708f66e87b604d091f54dd43b85dd1a02a` |

Independent readback reconciled all 654 current inputs, the matched graph
and test bindings, and unchanged index bookends. The canonical candidate is
`5d824949954caab6dc193928e853db61b2e83cdb78b6d2506cf4c369658251c8`.
The full 121-name roster matches the earlier successful writer roster. It
includes the real queue cutoff, refused startup, final wake before physical
join and overdue join-tail controls. Zero tests failed, were ignored or
were filtered. The configuration, tool and vendor bookends passed. The
outer recipes and `custody-retry.json` are in
`~/.local/share/nevrotick/service-writer-current-20261121.1y0jy9c6/`.
The retry custody SHA-256 is
`01408797aca2b475faa5a58006edc54e308e0fb1164da59437095fdc9b311b9c`.
This supplies the required ordinary test receipt. Durable coverage and
generated mutation remain at the existing 6 GiB ext4 launch prerequisite.
Provider admission, installed delivery and native qualification remain open.

The fresh-context integrated review found that the first draft let pending
worker cleanup delay writer retirement and used the pre-writer clock for
a pending result. The repaired source retains the independent owners and
adds a control that crosses the cutoff inside the writer poll. The reviewer
also checked the revised ready-at-cutoff consumption guard, exclusive
ID 4 transfer on refused Running, and isolated destruction controls. The
review accepts source ordering, field provenance and custody within the
portable scope. It did not execute the integrated candidate.

Independently of Rust, the bounded metadata CLI run validated 731 fields and
generated and checked 16 files. The two-event manifest matched published
fixture SHA-256 `f5239d8b6121ca99cc4db4edd0ccc6929137cf7f3038f3a6e2e43faadeddca6b`.
With network disabled, libxml2 2.9.14 rejected the installed SDK XSD's
literal-dollar spelling at schema compilation on both inputs (exit 5).
A private schema copy that changed only those two literal spellings
accepted the original manifest and refused the invalid channel type for
the intended enumeration (exits 0 and 3). The temporary trees were
removed. The original XSD result remains incomplete. This is source
feedback only. MD-P4 still requires the admitted Message Compiler and
independent compiled descriptor and resource readback.

The changed-file line and mutation gates retain their 90%/95% requirements.
Raw record admission and the wire format stay unchanged. The observation
accessor now uses the same mapping as the writer; Fuzz32 exercised its
previous implementation. Current ordinary checks exercise the refactored
mapping. Keep that historical fuzz receipt with its original bytes and the
current projection outside any fresh fuzz claim. Native writer and resource lifetimes,
provider registration, installed access and retention, and actual history
remain separate exits.

**Configured development result.** The existing fixture runner's `graph`
and `privilege-checkpoint` lanes passed on Rust 1.100.0 with the same
652-input candidate
`3573199cb2af9a7cdd4d0a9faddd70e082350297cf49dc299b0e92a7c5e4c087`.
All six graph and twelve development commands exited zero. GNU ordinary
and optimized each executed 70 platform and 117 fixture cases, including
the GNU child controls for worker and writer destruction. All 28 platform
doctests passed, including the private-construction and non-Clone controls.
Both formatting checks, GNU platform and fixture Clippy, MSVC-target
fixture Clippy and fixture rustdoc passed. Vendor, tools, original source
and index bookends passed. Local receipts are under
`artifacts/windows-services/recovery-20261107-v1/`, with graph
`writer-graph-2111d.json` SHA-256
`2638b98c88aad74a3adb6c1ac8b3e4747875923a59223d4c5d9748604c7b9d6b`
and development `writer-development-2111d.json` SHA-256
`94cee716727a4bb1eeba72d6b81bb17e75a907194a2d6137d8bf4493c8f1decf`.
Whole-lane times were 26.00 and 164.07 seconds. The three earlier attempts
retain their fixture-test compilation, lint and MSVC dead-code refusals.
The coordinator retired its detached worktree and build copies after
receipt readback. The disk-backed metrics launch floor is 6 GiB and WSL
had 4.3 GiB available. Current coverage and generated mutation acceptance
remain open for the selected owners. Keep the public campaign closed.

**Metrics admission follow-up.** The standalone fixture-test lane passed
all 117 named cases with source, index, tool, vendor and graph bookends.
The matching `writer-fixture-test-2111a.json` receipt has SHA-256
`20efc38c1e8ac9369f4b2717285956c7286c6251327f736f4385a6576a11bdad`.
Independent readback matched all 652 named inputs to the pre-assertion
candidate. The later shared-only unit assertion changes that full freeze;
the fixture does not execute the shared unit target, while the configured
fixture receipt reader still requires a refreshed binding. The durable
fixture lanes remain pending until the distro has enough
space for their 6 GiB preflight and subsequent retained output. The only
observed npm installation large enough to close the original gap serves
live MCP sessions; its files and the retained product evidence remain in place.

**Shared diagnostics metrics.** The first private schema 7 binding froze
623 product inputs and 38 protected inputs at `a8f497f4…`. The line lane
executed 70 named tests and covered 936 of 963 executable lines (97.20%).
Independent GNU inventory review checked all 414 exact variants and their
source spans. The inventory digest is
`27cb9a026be3199e028194967afb934831c783fad00b8b87d884ac249f9c80f3`.
The enumeration generates whole-body changes for the shared routing helper.
It does not target each internal sequence, candidate and required-queue
guard separately. The executed writer, queue/refusal, sequence, Garde and
compiler-privacy witnesses are separately retained.

The first generated run executed all 414 variants and its 98-case baseline.
The tool reported 240 catches and 174 compiler refusals. The unchanged
reader accepted 239 behavioral causes. One `Importance::code` variant
returning zero reached an unsigned subtraction overflow in optional
routing. That panic earned no kill credit. Independent readback matched
the exact refused log and confirmed complete export and intact custody.
The `analysis-writer-shared-mutation-2111a` receipt remains incomplete.
The provenance unit now compares the actual retained wire bytes for
Running and PrimaryFailure with independent codes 5 and 2. This runs
through the required consumer with optional routing off to distinguish the
zero-code variant by assertion. Neither production code nor
the classifier changed for this repair.

**Current shared line and mutation gates pass.** Rebound only the additive
unit-test change at `875dd119a0e242da4a0e31d164ebb4147a6461378ae0ec7082e6928c91098bd0`.
The second line lane passed 70 tests and covered 936/963 lines (97.20%);
the independently reviewed 414-variant inventory digest stayed identical.
The shared mutation lane executed the 98-case baseline and all 414
variants. Fresh Build and attributable tests established 240/240 viable
behavioral catches, 174 compiler-unviable rows, zero survivors, timeouts
or exclusions. The same zero-code variant now fails the literal
`Some(5)` assertion with `Some(0)` in the required history. The neighboring
overflow receives no credit.

The existing local recovery directory retains the raw and frontend
receipts under `analysis-writer-shared-{coverage,enum,mutation}-2111b`.
The raw mutation receipt SHA-256 is
`461a7c0123ad33dc483b0e76782e37f98c8c4220900bce13523d681d08c6cbab`.
Source, index and export bookends passed. The corrected line, enumeration
and mutation lanes took 27.42, 21.27 and 222.83 seconds including custody
and export. Independent closeout rechecked the exact candidate, all variant
identities, fresh Build roles, selected assertions and export hashes. The
first failed lane remains recorded. The temporary build copies and private
worktree were removed after raw archive verification. This closes the two
numeric gates for the one shared file. The fixture-wide metrics,
generation gaps, Windows writer and installed delivery remain under their
existing owners.

**MD-P4 Windows SDK inputs.** The exact Microsoft SDK BuildTools
`10.0.28000.2705` archive matches the 22,297,017-byte size and SHA-512 in
the [official NuGet catalogue](https://api.nuget.org/v3/catalog0/data/2026.08.26.05.01.44/microsoft.windows.sdk.buildtools.10.0.28000.2705.json).
SHA-256 is `8bfdfb6ca2633f531cf80b5fa22512ba61a394d7988f0970db83baadc67929ed`.
Inert AMD64 PE readback identified `mc.exe`, `rc.exe`, `rcdll.dll` and
`ServicingCommon.dll`. Static imports show no dependency on the separate
VC++ Redistributable. The exact four files, import reports, catalogue and
qualified source manifest are frozen in
`~/.local/share/nevrotick/mdp4-event-sdk-20261121.jWQDL8/`, with
`preparation.json` SHA-256
`912bc02fd6ec8e3a6375bd0cd648db5a2ee425871104b1c510fa5d53b1898141`.
The first local generate attempt correctly refused output under the input
root. Its sibling output passed the CLI check and reproduced the
`f5239d8b…` fixture.

Archive hash verification establishes the published content identity.
Execution still needs executable trust admission and authorization for the
archive's [Windows SDK agreement](https://aka.ms/WinSDKLicenseURL). The exact
archive's `.nuspec` declares `requireLicenseAcceptance=true`. The retained
agreement SHA-256 is
`dd07eb178e00c6bba4148457fc00ff77cd4887eb521d504186fe59c9ec8bbe62`.
The earlier xwin record names a different Visual Studio tools agreement. The prior
SDK diagnostics installation retained a different licence document and
no applicable compiler-tool payload. The next
compiler-only result needs one owned offline guest run for MC and RC,
WSL linkage with the pinned `rust-lld`, and independent PE readback of both
descriptors, complete keyword values, template field widths and localized
message-table bindings. Pair the healthy result with an absent or substituted
resource control. Publisher installation, privileges and the final MSIX
remain outside that compiler-only result.

Microsoft Learn MCP confirmed the user-mode manifest path and the offline
Sandbox settings. In guest-private scratch, run `mc.exe -um service-events.man`,
then `rc.exe /x /fo service-events.res service-events.rc`. Keep the header,
template and message binaries, RC source, and compiled resource for independent
readback. The selected tools' existing import readback requires no separate
Visual C++ Redistributable for this run. SDK execution and compiled readback
remain open. [MC](https://learn.microsoft.com/windows/win32/wes/message-compiler--mc-exe-),
[RC](https://learn.microsoft.com/windows/win32/menurc/using-rc-the-rc-command-line-),
[Sandbox](https://learn.microsoft.com/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-configure-using-wsb-file).

The host-control preflight observed Windows build `26300`, Sandbox CLI
`0.8.114.0` and 5,549 MiB of available physical memory. CLI help and version
returned successfully. The 12-second `wsb list --raw` attempt timed out before
returning a session inventory. Confirm ownership and refresh headroom before
launch. This preflight launched no guest or SDK tool.

##### Engine collection and timer cleanup transfer — 21 November 2026

W11-W05/S3.1 and CHECK-27 retain the original series decision independently
of timer cleanup. After the third real completion, a final invalidation can
produce `Stale` with every attempt populated. A separate timer failure used
to replace the only series result with `Failure`. The service worker could
then classify it as cleanup-only because there was no primary error and no
unstarted attempt carrying the stale decision.

`SeriesReport::collection_end()` now gives the validated collection decision
to the fixture worker. `end()` still gives the aggregate result used by
existing callers, including session retirement. The worker keeps the original
operation and cleanup observations independently. A primary error still
forces the collection decision to `Failure`. A clean Stop retains `Stopped`.

The focused Rust 1.100.0 WSL run used the admitted default graph, owned
loopback peers, and fresh sandbox boundary and custody checks. The final
checkpoint and cleanup-neighbor tests passed. The checkpoint runs all three
actual replies, observes invalidation immediately before or after the final
decision, and reaps a real timer that returns `GenerationExhausted` from a
privately seeded counter. Restoring only the old masking behavior failed at
the original collection assertion with exit 101. The exact restored source
passed the checkpoint again, with the same source, tool and vendor bookends.
The setup compiler refusal in the earlier observer revision is retained.
The integrated default Engine `clippy --all-targets -D warnings` run
passed in the same bounded WSL sandbox after restoring the configured
offline Cargo-home source replacement for the Clippy subprocess.

Independent review traced the Engine producer and fixture consumer. The
integrated four-path patch SHA-256 is `0527026692e3…`; raw tests, the fault
and restoration are under `~/.local/share/nevrotick/service-series-outcome-20261121.sA5Nm6/`.
The ordinary fixture schedules neither timer. The joint control below
exercises both causes through an owned fixture invocation. Per-file
qualification remains subject to the existing durable disk floors.

**Joint receiving control prepared.** A private derivative of `82ea4c68710f`
and the captured working changes injects the final invalidation and timer
exhaustion in Engine. The fixture control handles three real loopback replies,
holds the real worker's exit tail, and checks the original report, join,
failure receipt and both diagnostic causes. The enclosing child is bounded
and reuses the existing watchdog. Pre-release observations return errors
through a path that first requests release of the paused worker.
The control-only patch SHA-256 is
`58424d2c2c0fc76bf5e996dec620713835179e789a30bbb203a7607cda54f9f7` at
`~/.local/share/nevrotick/service-series-joint-20261121.55qxvoti/joint-control.patch`.
The companion `handoff.json` binds the captured source and edited paths.
Rustfmt and whitespace checks passed. Initial preparation stopped at the 4 GiB
fixture-development floor.

**Joint receiving control executed.** The cleanup restored fixture capacity.
The selected private control was rebound to the current 653-input writer
candidate `99c3c78337cb…`. Its two instrumentation paths are the Engine's
series and the fixture's worker test. The test requires an actual spawned
timer before accepting the reaped `GenerationExhausted`, and its parent
preserves bounded child failure output. The real three loopback replies,
retained `Stale` and cleanup failure were observed through the original
worker, its physical join and host receipt.

The fixture graph and compiled RED reproduced the missing operation event.
The child reached `S31-JOINT-operation-diagnostic-after-join` with `[]` in
place of `[(9, 22)]`. The expected test exit was 101. Restoring only the
classification from `collection_end()` passed the same control, with 121
library and one integration cases passing. That control requires both
decoded causes, the original Stale decision and the reaped cleanup outcome
after the worker's retained permit reaches zero. The graph, compiler,
vendor, configuration and source/index bookends passed on both sides.

The independent review checked the actual arm condition, assertion output,
the 653-input maps and their one-file RED/GREEN difference in
`engine_worker.rs`. The two graph lanes took 28.23 and 28.53 seconds, and
test lanes 38.18 and 42.51 seconds. The raw records are under
`~/.local/share/nevrotick/service-series-joint-20261121.55qxvoti/source/artifacts/windows-services/recovery-20261107-v1/`.
The RED and GREEN test receipt SHA-256 values are,
respectively, `176e319112c71670bedbe24b3715497e6b77fd76066835bd64aaeee4664f6c26`
and `9b281117b3227f97df744ca45411683efe803bb69df0da6b6cb859771623531a`.
Both completed targets and the two byte-identical graph-only source copies
were retired. The test seals, exact instrumentation and raw receipts retain
the portable derivative's scope. The normal fixture, per-file metrics and
installed Windows behavior have their own acceptance evidence.

#### MD-P4 VERSIONINFO compiler and readback pilot — 21 November 2026

For W11-W10/MD-P4 and CHECK-18/20/23/26/27, the metadata CLI validated,
generated and checked the current source-mode inputs at `82ea4c68710f`.
The WSL pilot compiled the generated UI and Observe RC files with LLVM RC
22.1.8 and UTF-8 code page 65001. The Rust 1.100.0 bundled `rust-lld` linked
each `.res` into an inert AMD64 carrier and produced a no-resource sibling.
The build ran in an offline, bounded Linux sandbox with read-only inputs.
The compiler, linker, readback tools and input hashes matched before and
after the run. No PE was executed.

The independent readback decoded the actual `VERSIONINFO` leaf from
`llvm-readobj`, including resource ID 1, language 1033, translation
`(1033, 1200)`, fixed version fields and all eight UTF-16 strings. It
accepted both correctly labelled carriers, refused the UI carrier as
Observe despite its identical resource count, and refused the absent
resource. A changed Observe description and numeric revision 1043 caused
the metadata CLI to reject the old generation and the readback to reject
the old PE. An overwrite rebuild and a clean rebuild produced identical
new `.res` and PE bytes. The raw decoder recovered the complete changed
description including `𐐀`. ExifTool 12.76 misreported that scalar as `?`
characters, so its presentation output cannot serve as the Unicode oracle.

The raw inputs, binaries, reports and decoder are in
`~/.local/share/nevrotick/mdp4-versioninfo-20261121.EcX1x4/`.
`readback.json` has SHA-256
`fb58c6dcc2986d48ee9dfe778540b11c609d14abdc137a113827f3ef0216e62c`.
The linked UI carrier has SHA-256
`528442c4cb73207277790ccd27c535106fdcc797663e0fda381cfaacb8059fe2`.

The configured Windows input admission rehashed all 623 selected source
inputs, 29,447 files in the retained 585-package vendor, and 529 selected
xwin library files. Its `configured-ui-admission.json` has SHA-256
`e1c85f9fd72ab1281bcaa4f64ecb118d0d41bef59776daaae5060976be617031`.
The retired default vendor pointer requires an explicit override with the
admitted `engine-ke-association-v1/preparation-v2/root-vendor` directory.

An isolated no-dependency Cargo binary exercised the selected build-script
mechanism at `x86_64-pc-windows-msvc` with the configured v3 and mitigation
flags. Seven controls covered the default, delivery of the UI resource, a
same-path replacement at a new selected digest, matching clean readback,
missing selected input, restoration, and removal of selection. The selected
resource was mounted read-only. Cargo reported the changed digest in its
rerun decision, and decoded resource values agreed in the warm and clean
images. The first readback predicate rejected the reader's valid empty
output because it expected a final newline. The first missing-file attempt
in fact hit the expected empty-file guard because an empty mount placeholder
remained. The run retained both raw failures. The final missing-file case
removed that placeholder first and reached `NotFound`. Final case receipts
in `cargo-invalidation/` have SHA-256 `e6f20bc76ed9…` and `69c8a29eef99…`.

S0.5 and W11-W10 select the existing Slint build script for the actual UI
binary link. Feed it the fixed `.res` path and runner-verified digest only
in the configured UI link row. Preserve the exact row command and target
budget. Verify the actual PE independent of the compiler before accepting
the row, including when artifact export is unselected. Repeat the relevant
warm and clean controls through Slint. Observe remains a diagnostic role.
Event Message Compiler admission and installed publisher, package and
release evidence retain their existing prerequisites.

The first actual-link candidate passed the sandbox boundary and native-input
controls. It stopped before Cargo at `ui-text-controls`, with one failing
healthy-source case among 85 tests. The unchanged `82ea4c68710f` UI and
checker sources reproduce the same failure. The tray and notification
source witnesses refer to older owner shapes, leaving current text sinks
unresolved. The full failure and baseline are retained under
`~/.local/share/nevrotick/mdp4-ui-source-20261121.UQIlfb/local/`.
The first runner receipt SHA-256 is `ee46627b1065…`.

The S0.5 checker repair scopes the tray witness to `TrayBridge::new` and
follows the chosen notice through its admitted attempt, exact preparation,
and guarded submission. Independent review challenged constructor decoys,
substituted prepared fields and notifier aliases, and the retained
preparation-failure exits. The integrated source checker resolved all
509 active IDs with zero errors. The Python coverage, native-source and
codegen selections passed 85, 41 and 14 tests, respectively. These exercise
lexical source witnesses. Native rendering and event delivery retain their
separate qualification owners.

The corrected detached candidate reached the actual Slint UI executable in
the default MSVC release row on WSL. All 18 selected rows passed, including
the sandbox, localization, locked Windows graph, and exact UI link. The
private runner mounted the admitted `.res` read-only for that link only.
Independent readback found the exact expected `nevrotick-ui` VERSIONINFO
leaf with resource ID 1, language 1033, translation `(1033, 1200)`, all
eight strings, and the prototype version and prerelease flags. The 784-byte
leaf matches the compiler pilot byte for byte. The first failed selection
and the successful retry consumed 469.15 seconds of the original 1,100
second allowance. Temporary Cargo build copies were retired by the runner.

The exact 23,532,032-byte PE and source/tool/row receipt are under
`~/.local/share/nevrotick/mdp4-ui-source-20261121.UQIlfb/local-retry/`.
The PE SHA-256 is `aa333805dc333231b1eb79f3cec0a9bb49ae5748bbf04600522bd98a51c0e7d9`
and `verification.json` is `1a1e3cb3e73390dfdc6991fc15ebd9ccf8a6ed3ec7fde327bd048cd36fbcc7f5`.
The pilot kept the UI build hook and its exact policy pins in the detached
candidate to prepare the maintained runner and its artifact exit.

##### Maintained Windows UI resource build — 21 November 2026

The default WSL MSVC lane now generates and checks the source-mode metadata
from the frozen inputs. It invokes the pinned LLVM 22.1.8 resource compiler
and Rust 1.100.0 bundled PE reader inside the existing namespaces. Cargo
receives the read-only `.res` and digest only in `windows-link`. The runner
decodes the linked fixed fields, UTF-16 strings and translation and compares
them with the admitted UI source facts. The export accepts the exact observed
UI hash and admits both release EXE/PDB roles before copying them. The actual
CLI refused a same-size compiler substitution before executing a row or
creating an artifact directory. The verifier and reader tests passed 138
and 11 cases respectively.

The first maintained cold run reached the Windows compiler row and stopped
on rustc and LLD thread-creation failures. The retry applied six-CPU affinity
to the owned WSL process with the original twelve Cargo jobs, sandbox limits,
selection and release-link commands. All 28 configured rows passed, on 635
frozen source inputs. The receipt SHA-256 is
`d3687f0a464c6a038515993cb5afc1b807036d1b7d9c50109946a7bb548961a9`
under `~/.local/share/nevrotick/mdp4-ui-source-20261121.UQIlfb/maintained-cold-retry/`.
The failed cold receipt remains in the sibling `maintained-cold/`.

The exported UI SHA-256 `a895674dbb57…` equals the independently observed
image hash. Inert inspection matched both CodeView GUIDs, ages and names to
the actual exported PDBs. The exact export inventory has the two binaries,
their two PDBs and the verification receipt.

The actual Slint warm control changed only the private source copy's UI
description, including `café 𐐀`, and kept the same Cargo work directory and
fixed resource mount. `FileDescription` and `Comments` changed in the linked
PE. The clean comparison used an empty separate Cargo target and produced
the same 824-byte resource leaf, SHA-256 `266a12a330db…`. The earlier leaf
was 784 bytes. A further warm link with the selector removed produced an
executable that the maintained observation path refused for absent VERSIONINFO.
The warm and clean exercise retained the same configured release commands and
finished in 1,028.08 seconds within its 1,080-second allowance.
The receipt SHA-256 is
`ec8e79c02ec717254e95f46ef767198296b03703421235ff0902ba022ca38cfe`
under `~/.local/share/nevrotick/mdp4-ui-source-20261121.UQIlfb/maintained-warm/`.
It demonstrates the combined resource and selector invalidation path.

The nine verified build, tooling and control paths were integrated into the
main worktree. All 635 compiled-path inputs matched the terminal receipt at
integration. The maintained source-mode UI resource slice now reaches the
ordinary export. Native execution, installation, signing and complete package
readiness stay with their existing owners.

##### Maintained Observe VERSIONINFO build — 21 November 2026

W11-W10/MD-P4 and CHECK-18/20/23/26/27 now bind the existing Observe resource
to `nevrotick-observe.exe` in the configured Windows release row. The two
roles share a single generated and checked source tree. Each LLVM RC command
can write only its own output directory. The UI still consumes its selected
digest through the existing build script. The exact Observe `cargo rustc`
row binds its read-only resource by digest in the leaf linker argument.
It preserves the selected target, flags, feature graph and release limits.

The independent review verified the different subsystems from the retained
AMD64 PE headers. The reader now selects GUI for UI and console for Observe,
then compares the resource's fixed fields, UTF-16 strings and translation
with that role's source facts. Both executable hashes are rechecked after
the build and before export. The linked-format distinction is documented in
[Microsoft's PE format](https://learn.microsoft.com/windows/win32/debug/pe-format),
and the resource fields in its [VERSIONINFO contract](https://learn.microsoft.com/windows/win32/menurc/versioninfo-resource).

The metadata, package-artifact and regression skills guided actual producer,
reader, wrong-role and export selection. The verifier passed 140 Python
controls. The reader passed 12, including console acceptance and refusal
of each swapped subsystem. The pre-change controls exposed unsupported
Observe invocation, export binding and reader inputs. The two selected
release command shapes cover default and EP1 budgets in these controls.

The actual configured default Windows cross-build passed all 31 rows with
the existing admitted MSVC libraries, Rust 1.100.0 and LLVM RC 22.1.8. It used
the established six-CPU WSL affinity and the admitted Windows-backed scratch.
The 635-input source map at `82ea4c68710f` plus working changes has canonical
SHA-256 `a62d226c5651e3ee0b55c5d6e7fafa97b61697a5bc0712915b1d8d8861ad87fd`.
The image SHA-256 for `nevrotick-observe.exe` is
`198bb5f238f7ead8d59e2189e32188b9c06978695c390eac0ade84208eb3993a`.
Independent inert inspection matched both CodeView GUIDs, ages and symbol
names to the exported PDBs and checked the exact five-file export inventory.
The maintained link and independent export receipts are under
`~/.local/share/nevrotick/mdp4-observe-link-20261121.8cuov5d6/`.

| Receipt | SHA-256 |
| --- | --- |
| `verification.json` | `441b52be6533a6b1384e077494649a77620bec31b1f55ecbe6fc4cf2c9f158a3` |
| `inspected-exports.json` | `ffc7853b5d01d22e90b909668bc25873cc96c871b7835de82f1ad06dc7bb1e6a` |

The real Observe cold, warm and clean control finished in 198.10 seconds
within its 720-second allowance. Changing only the isolated Observe
description changed the link's selected digest. The same Cargo target
then emitted the new description and comments, including `café 𐐀`.
A clean target emitted the same decoded fixed fields, strings and
translation. The original VERSIONINFO leaf was 880 bytes and both changed
leaves were 792 bytes. A hash-matching UI resource produced a successful
console link and the Observe readback refused its wrong component fields.
Removing the resource selection also linked successfully and produced the
missing VERSIONINFO refusal. The source, tools, vendor and library bookends
passed and both scratch targets were retired. Independent review checked
the recipe's actual link, source freeze and discriminating refusal paths.
`warm-verification.json` has SHA-256
`6659a021494ae79049cc4de422faa4875342bed44fbf370fc284830f2cc802cb`.
The same evidence directory retains the recipe and all five raw case logs.

Observe remains a diagnostic component. Windows execution, installed service,
signing and MSIX gates retain their existing acceptance and inputs.

### 4.36 WinGet distribution before V1 — 7 November 2026

**Status:** researched/planned, not implemented or natively qualified.
[OD-17](../docs/nevrotick-owner-decisions.md#21-od-17--winget-capability-before-v1)
and [NV-WGT-001](../docs/plans/winget-distribution.md) place this capability before
V1 under existing W11-W10/P6/P7, with W11-W05 for service/lifecycle effects and
the existing metadata/release/security/independent-QA owners.

First execute WGT-W01 source/identity preparation: re-read actual release/package
state, source-specific IDs, metadata pending fields, current tools and holds.
Under §4.39, qualify the selected complete direct MSIX through a non-Store source;
Community publication requires an authorized public trusted complete artifact. No MSI
selection, external service prerequisite, new signing entitlement, reserved
identifier or public download URL follows from this assignment.

WGT-W02 owns reached metadata generation/readback; WGT-W03 owns actual
client/package/platform lifecycle; WGT-W04 owns bounded release preparation and
applicable automation dry runs. WGT-W05 independently reconciles **WGT-C01**
before V1 approval. WGT-W06 records **WGT-C02** only after authorized external
publication and actual source-specific discovery/acquisition. Review delays
remain external, while missing mandatory native evidence remains open work.

Use the plan's WGT-T01–WGT-T12 with existing CHECK owners. In particular retain
the two-family matrix and baseline-safe v3 admission, actual elevation/agreements,
multiuser services/UI, one clock controller, final-byte trust, interrupted
maintenance and W32Time restoration. A WinGet exit code, YAML validator or
Community sandbox does not close the whole product lifecycle.

Record exact source/diff, artifacts/manifests, client/generator/schema, OS/CPU,
source/identity/scope, commands, independent observed outcomes, raw redacted
evidence, unexecuted rows and next action here. Keep documentation review in
the linked plan. This entry executes no Windows/package/signing/credential/Store
or Community operation and lifts none of the current holds. Under
[OD-16 §20.1](../docs/nevrotick-owner-decisions.md#201-mathematics-experiments-first--13-november-2026)
and the 14 November [OD-20 amendment](../docs/nevrotick-owner-decisions.md#24-od-20--supplementary-certificate-distrust-after-v1),
post-V1 work starts with MATH-0 evaluation/disposition, then TRUST-0/TRUST-C01,
MT-0 and Khronos. The original dated assignment and review receipts retain their
evidence scope.

### 4.37 OS-integration vulnerability research — 8 November 2026

**Request and inspected source:** identify NevroTick's actual/proposed OS
integration points, research relevant vulnerability databases through MCP, and
challenge applicability with separate agents. The read-only source baseline was
clean `a78a8852bbd05dfd5f837a571fccb32b7b08fb81`. The
[supporting review and handoff](../docs/supporting-reviews/windows-os-integration-risk-research.md)
retains the inventory, primary sources, CVE dispositions, exact-version query
scope, counterevidence, paired controls and research limitations.

**Result:** no new exploitable NevroTick vulnerability was substantiated in this
bounded pass. Existing service/IPC/file/loading/certificate/lifecycle contracts
already cover most researched failure classes; their native implementation and
qualification remain distinct. No complete code audit or complete Windows CVE
inventory is claimed.

| Existing owner | Integrated refinement / disposition |
| --- | --- |
| R-05 / CRYPTO-04; OPEN-8; CHECK-8/9/26/27 | Separate chain-building and revocation cache-only controls, per-URL versus cumulative revocation URL retrieval timeout, and actual native-job completion. Retain service-context, wrong-clock, unknown-status and direct/delegated-traffic cases. The selected wrapper/policy must be qualified; no blanket offline or soft-fail policy is adopted. |
| ST-W03 and W11-W05/08/10; ST-T03/10/17; CHECK-13/15/17/23 | Include elevated error/crash/rollback/cleanup writes in the actual-object authority tests. A failed primary operation must leave unrelated sentinel files unchanged, with useful admitted diagnostics as its valid neighbor. Native execution remains held. |
| S0.2/S0.3/P3, W11-W05 and THR-26 owners | Preserve already-defined effective-token, pipe-identity/rights, failed-reversion, continuous-custody and completion cases. Portable contracts/laboratory hosts establish no final service/provisioning or native IPC pass. |
| W11-W10/P6–P7 and two-family matrix | Reuse signed-carrier/untrusted-input and DLL-search analogues, exact OS build/UBR/KB applicability and final package/component identity. Current CVE records do not prove an installed vulnerable target. |
| CHECK-26 and existing exact graph owners | OSV summary: 629 registry tuples from five named lockfiles, five informational unmaintained matches. Every flagged row was fetched in full and is metadata-only, absent selected features in both maintained target rows. No fresh Cargo graph/build pass or blanket safety claim follows; local derivatives/native/toolchain surfaces remain separate. |

The parent used static source/data inspection, NVD/OSV, Microsoft Learn and
Ansvar freshness/KEV. Separate inherited-context researchers covered
network/TLS/CNG and services/IPC/files; a separate judge reviews the integrated
handoff. Database screens and primary-source access limits are recorded in the
review, rather than described as exhaustive coverage. Existing architecture,
skills and CHECK IDs remain the enforcing/qualification owners.

**Current stage:** source-based research and documentary integration. The
current development §7 queue, Store-first direction, admitted interim MSI
signing profile and all Windows/fuzz/storage/UI/post-V1-proof dispositions
remain. No product code, dependency policy, OS settings, installer, signing key
or credential changes are made by this amendment. No target/native execution
ran; the security-audit skill's required execution isolation is unavailable in
this session, and no unchanged failed isolation attempt was repeated.

**Independent review, completed on the initial candidate:** the separate
read-only judge `/root/signing_coverage_judge` inspected the entire new report,
all five routing additions and relevant neighboring/source authorities. It
verified the six file hashes against manifest SHA-256
`6ce00fe71ded8b5a012060bd0629a1de6513a8cfc6ea9cf638f072f8cbcff517`
and tracked-diff SHA-256
`4ec2b3173936d15e6ec46ca537a58c605bcb1205d7cc90d20881782913c566f3`.
This reviewer inherited context; it was not a fresh-context review. Earlier
failed fresh-context launches supplied no usable review or assurance.

The judge's two comprehension paragraphs are retained verbatim:

> This candidate adds a dated, bounded investigation of NevroTick’s Windows integration assumptions. It distinguishes implemented experiments from planned production boundaries, classifies external CVEs by applicability, records the scope of dependency lookups, and routes two concrete refinements into existing work: separate Windows certificate-retrieval controls, and protected handling of privileged failure-side writes.

> The current system contains portable contracts, a constrained loopback observer, fixture UI adapters and standalone laboratory Windows hosts. Those sources do not establish production TLS/NTS, native IPC, service provisioning, package qualification or OS patch status. The existing architecture, owner decisions and CHECK/THR assignments remain authoritative; Store-first delivery, all-file signing, the admitted interim self-signed MSI profile, execution holds and post-V1 sequencing remain unchanged.

**Initial verdict: NEEDS REVISION, documentation scope only.** The judge found
no additional material scope/policy regression and retained two issue identities:

- **OS-CVE-J01:** the fifth Rustls record needed its own upstream advisory and
  explicit CVE-to-GHSA provenance. The corrected report links the 0.23.45 fix's
  advisory, attributes mappings to NVD, retains the upstream "No known CVE"
  discrepancy and does not mistake the analogous Go patch for a Rustls fix.
- **OS-CVE-J02:** cumulative timeout wording needed the exact revocation URL
  retrieval scope. The report, R-05 and this W11 summary now state that scope
  while preserving the lack of any overall TLS/native-job deadline guarantee.

**Bounded follow-up completed: READY — documentation and research handoff only.**
The same read-only judge verified both corrections and the verbatim receipt,
closed **OS-CVE-J01** and **OS-CVE-J02**, and found no further material issue in
that bounded recheck. Revised manifest SHA-256:
`a919336685098975eccd8ec43099b8f0a3088c2261b4b510860471ee59c0051e`;
revised tracked-diff SHA-256:
`47fbcdbb56f741f7a7c4908ede30d1dfdec01ce73d5b7f2631c94951fd61e18c`.
The follow-up additionally retrieved exactly the five Rustls NVD records and
confirmed their reference mappings. All were in `Received` status; this is
association evidence, not independent NVD exploitability analysis. The original
candidate's verdict remains historical. This final receipt records the returned
result without extending its scope to product/native/release readiness.

The judge corroborated the consequential vendor/CNA mechanisms, Windows affected
rows, relevant dependency ranges, certificate semantics and the five maintenance
classifications. It did not repeat the full 629-tuple query or every discovery
screen, and its direct CISA catalog access failed; the parent's Ansvar KEV receipt
does not become independently corroborated. No native/build/test execution ran.
Parent validation of the initial and corrected candidates checked all 32 added
local links/anchors and `git diff --check`, with no failures.
`docs/SHA256SUMS.txt` is absent, so no bundle manifest integrity is claimed.
These documentation results remain separate from any future product result.


#### 4.37.1 Skill and canonical guidance integration — 9 November 2026

**Request and baseline:** David asked where the OS-integration findings must enter
skills and documentation. The parent and two read-only inherited-context
researchers inspected clean `bb4536b2a72d698bdb18bab600bce3c419ff1ac1`, its existing
skill routes, canonical contracts and task owners. The
[integration map](../docs/supporting-reviews/windows-os-integration-risk-research.md#8-skill-and-document-integration--9-november-2026)
records changed surfaces and adequate existing coverage. This note uses
Asia/Tbilisi; the preceding research evidence retains its 8 November UTC date.

**Integration:** architecture §12.1 now explicitly includes privileged secondary
failure writes; §12.3.4 routes diagnostic sinks to that contract. Hardening §9.8
owns the paired sentinel/useful-diagnostics control, reached through persistence,
privacy/logging, the service adapter guide and fault-recovery QA. Architecture
§18.7.2, crypto/protocol skills, Windows host guidance and shared Windows QA now
reach R-05/S2.2 and the existing actual-job lifetime owner; exact cache-only flags
remain in R-05. Advisory triage and conditional MSI property forwarding are
clarified in their current security/dependency/deployment owners. Native patch
claims remain tied to the actual two-family component/build/servicing matrix.

Existing token/pipe/completion, DLL/signature and package-QA contracts were checked
and retained. No new skill leaf, schema, runtime verifier, dependency resolution,
warning ignore or parallel gate is introduced. Current development ordering,
Store-first/all-file/interim-signing policy and Windows/fuzz/storage/UI/post-V1
proof dispositions remain. This source/document work resumes no native operation.

**Validation scope and limits:** check changed Markdown links/anchors, unchanged
skill registration/frontmatter/UI metadata and preserved imported inventories;
review the exact integrated diff and relevant canonical neighbors. The
`security-audit` skill's required OS isolation for target-controlled execution is
unavailable in this session. Repository validators, application code and native
fixtures were not executed, and unchanged failed namespace attempts were not
repeated. Required `check_skills.py` and its regression execution remain unrun here;
parent-authored static checks and read-only review supply narrower evidence.
A later appropriately isolated pack-validation run retains its existing owner;
this is a session evidence gap, not a new product gate or permanent execution hold.

**Independent review completed — READY for guidance integration and documentary
handoff.** Separate read-only reviewer `/root/signing_coverage_judge` read the
complete 17-file diff, relevant canonical neighbors and actual skill entry points.
It verified all candidate file hashes against manifest SHA-256
`fc8280573be89304004b133d0ceb42e0fa755b73981f4ef77c8c1ba7badc7db0`
and tracked-diff SHA-256
`10559f04599581a1be7b03972c077c19f6457cbc07a592089e0f16b85d6e7176`.
It found no material issue, corroborated the conditional MSI distinction in
Microsoft's property/custom-action documentation, and confirmed that the unchanged
service/IPC/privilege/lifecycle routes already cover their assigned boundaries.

The reviewer's two comprehension paragraphs are retained verbatim:

> This candidate moves the reusable lessons from the completed OS-integration research into existing implementation and verification routes. It makes privileged failure-side filesystem effects explicit in the architecture and diagnostic checks, connects certificate work to R-05 and the S2.2 handoff, and adds scoped advisory, servicing and MSI-input guidance. Dated CVE and package inventories remain in the research record rather than becoming permanent skill rules.

> NevroTick’s guidance keeps distinct owners: the architecture defines requirements, tasks retain work and evidence, skills route executors, and shared QA references specify the observations needed for a claim. This change preserves that structure and the distinction between source preparation and qualified Windows behavior. Store-first delivery, all-file signing, the admitted interim self-signed MSI profile, current development order and execution holds remain intact.

This was inherited-context review, not a fresh-context review. Earlier failed
fresh launches supply no usable evidence. The reviewer inspected source-level
routes; no blind skill-use trial, measured behavior improvement, product/native
qualification or release approval follows from its result.

**Parent static validation:** all 47 added local links/anchors resolved; metadata
and UI records for 66 registered skills were checked as data; all 20 recorded
imported-file hashes matched; registration/UI/frontmatter were unchanged;
`git diff --check` passed. No repository Python module was imported or executed
for these checks. The repository pack checker and its regression suite remain
**NOT RUN** under the session limitation above; this is narrower structural
validation. `docs/SHA256SUMS.txt` is absent, so no bundle-manifest pass is claimed.
The final receipt changes only this review/evidence record; the parent's final
candidate check and publication identity remain separate from the reviewed hash.

### 4.38 Service privilege profile handoff — 9 November 2026

**Request, source and scope.** David asked to select the right privileges for
each service and update documentation and relevant skills. Baseline:
`13ce18d728e0ca552805c8160c2649b338d472f9`, initially clean. This is a
documentation/skill amendment using Microsoft primary sources and bounded source
inspection. The later implementation and final qualification below complete the
selected portable observer/consumer checkpoint; no runtime setter or installed
Windows qualification follows. Reviewed guest execution uses §4.19.
Dates here use Asia/Tbilisi.

**Current disposition:** the platform primitive and standalone caller checks,
including all six selected files' 90%/95% gates, are independently accepted in
their Linux fixture scope. See [final qualification](#portable-service-privilege-checkpoint-final-qualification--9-november-2026).
Native token enforcement, provisioning and production adoption remain open.

**Decision owner:** [architecture §4.1.1](../docs/windows-time-architecture.md#411-service-privilege-profiles).
The engine's exact set is `{SeChangeNotifyPrivilege}`; the broker's exact set is
`{SeChangeNotifyPrivilege, SeSystemtimePrivilege}`. Both SCM records use explicit
nonempty names. All other present privileges fail admission, even when disabled.
The broker's clock privilege is disabled outside its guarded serialized effect.
The [source rationale](../docs/plans/windows11-integration.md#service-privilege-source-review--9-november-2026)
records API prerequisites and provider/process/memory compatibility limits;
[hardening's profile matrix](../docs/windows-time-development-hardening.md#service-privilege-profile-qualification)
owns the independent expected observations.

**Threat and contract refinement.** A compromised network engine must not
activate a retained clock/debug/impersonation or other unneeded privilege.
A partial observer cannot prove the complete set. Configuration/readback cannot
prove a changed running token, and a required-privilege list cannot grant an
account missing clock rights. Peer identification is a bounded thread context,
not permission for service effects under client authority. A broker's temporary
clock-privilege disabling does not protect it from code already executing inside
that privileged process. Preserve actual SID/owner/DACL/handle isolation,
admitted application/actor/effect guards and current clock ownership.
These refine existing BND-3/4/5 and CHECK-1/2/3/10/12/13/15/23/27; no new
requirement/check identifier family or generic privilege framework is introduced.

#### Implementation boundary inspected at the initial baseline — historical

The parent and read-only researchers inspected
`crates/windows-platform/src/privilege_observation.rs` and
`tools/windows-service-fixture/src/native_token.rs`:

- The native adapter queries tokens and structurally captures `TokenPrivileges`,
  but resolves three fixed forbidden-engine LUIDs; the portable snapshot retains
  those named facts.
- `require_engine_absence` checks those three facts. Broker observation is
  descriptive, not an exact broker allowlist or enabled-state admission.
- The native self-observer's `OpenThreadToken(..., OpenAsSelf=TRUE)` detects an
  unexpected ambient token; it is not an implementation of IPC client capture.
- The bounded scan of these two components found no service-SID setter,
  `ChangeServiceConfig2` provisioning or complete service/restricted-SID/default-
  owner qualification. This is not a claim about every file in the repository.

Retain the historical narrow source/host receipts at their recorded scope. The
new policy does not retroactively make them complete product-token evidence.
Reuse these existing observation/admission owners and their contracts; do not
scaffold a second policy engine or expand the inert lab into production services.

#### Ordered continuation in the existing workstream

**First stage — independently revalidate the implementation premise.** Before
the dependent observer/provisioning extension, a reviewer other than its author
must freeze the current HEAD and relevant dirty/source/evidence identities and
try to confirm **or refute** each current-state assertion above. The earlier
two-component inspection is a dated, bounded observation, not an instruction to
reproduce a presumed missing feature.

1. Trace raw capture, retained representation and actual admission callers from
   [the portable observer](../crates/windows-platform/src/privilege_observation.rs)
   and [the native adapter](../tools/windows-service-fixture/src/native_token.rs).
   Distinguish structural checks on all captured entries from retention of only
   three named facts. Search relevant alternative consumers before concluding
   that complete role admission is absent.
2. Inspect engine and broker paths separately, including startup and the guarded
   clock-effect path. Establish whether complete present-set comparison and
   required enabled-state checks actually reach the authoritative consumer;
   a descriptive snapshot, unused helper or test-only check cannot establish it.
3. Recheck current package/provisioning sources and candidate-bound receipts for
   the complete MSIX pre-start bridge. Distinguish an available API, implemented
   setup, effective running tokens and observed install/update/repair/recovery;
   absence in the two observer files does not prove repository-wide absence.

Record each claim as **confirmed for the inspected scope**, **refuted**,
**partially supported** or **unresolved**, with exact source/consumer references,
candidate identity, counterevidence and missing observations. If later code or
receipts refute a stale assertion, correct current-status guidance and remove
only the disproved work; preserve historical receipts and avoid a duplicate
observer/provisioner. Assign implementation only for the substantiated residual
gap. This review does not relax the selected role policy or lift execution holds;
unavailable native evidence remains unresolved, not a passed check.

**Independent premise revalidation — 9 November 2026:** completed for source
`61fd244a6bbd12cee887e6974da1b730a5aa1561`, with a separate read-only reviewer
and coordinator readback of the actual decoder/startup branch. The 36 relevant
file identities were stable across closing reads; their sorted
`path + NUL + SHA256 + newline` aggregate is
`061ea9484365db0d0d2ed4995f1fd256f111873bfe598097cd6369b0bd62018a`.

| Revalidated premise | Disposition and evidence |
| --- | --- |
| Complete raw capture versus retained facts | **Confirmed:** `native_token.rs` captures bounded `TokenPrivileges`; `PrivilegeSnapshot::decode` structurally checks every entry, but retains three named SystemTime/Debug/Impersonate facts. It is inaccurate to say only three raw privileges are queried. |
| Actual role admission | **Confirmed:** `host.rs` calls `require_engine_absence` only for Engine before worker preparation. Neither that predicate nor the broker branch enforces the complete selected role sets or required enabled state. Existing test source explicitly accepts empty snapshots and broker observations with the three privileges present/enabled; those tests were not rerun. |
| Context and clock effect | **Confirmed within this adapter:** thread-token checks surround primary-token observation; they supply no IPC-client capture or continuous context guarantee. The inspected composition uses an inert worker. Finite first-party searches found no guarded clock setter; P4/S0.4 implementation and native lifetime evidence remain open, rather than a demonstrated setter bypass. |
| MSIX pre-start bridge | **Not established in the inspected source:** alternative-consumer searches found partial metadata/service authoring (`complete_manifest=False`), not a complete provisioner or installed-token proof. Effective activation/update/repair/recovery remain unresolved under W11-W10. |

The actual reviewed parser/native/host hashes are respectively
`f5862459e0cce5e69373b985288c3f81d6f1af11763e50a5a190067dcdbd5d33`,
`4a8a5c4280cfccdcb470960b5cff55d083c1f278f9c5d4c835b52f8ca435e1c5`
and `3ff3b32d717448e78994a76047be57c0567af7561e2d1936530626f6bb1fbb96`.
Recovered local receipts are available under
`artifacts/windows-services/recovery-20261107-v1`, but this is no current-caller
qualification: `source-checkpoint-v2.json` (SHA-256
`f1c6196a6538207acad7517af528fb8621900c97d72e1fdfacf0335759f270ac`)
binds the parser/native adapter and **not current host.rs**. Its full source join
is 408 matching/33 different; all 13 referenced receipt digests remain intact.
`cross-sdk-ready-v1.json` has the same source join and explicitly records no
native execution and failed bookends. Historical metrics/self-reported match
flags do not qualify this candidate. Across the broader 78 top-level recovered
JSON files, 22 bind the current native-token hash, 25 another hash and 31 lack
that top-level binding; these counts are not the separate secret-scan cohort.

The substantiated residual is the existing platform/security source child:
retain the complete bounded LUID/attribute set, resolve allowed names locally,
and enforce both exact role profiles through this shared startup consumer.
Freeze that contract before implementation, using valid, extra-enabled,
extra-disabled, unknown-LUID, missing-required and malformed/failure neighbors.
Reuse the existing structural/context/cleanup owners. Provisioning stays with
W11-W10 and clock effects with P4/S0.4. This review ran no Rust tests/builds or
native commands, adopted no metrics and lifted no execution hold.

#### Exact-role observation and startup source contract — 9 November 2026

**Historical accepted preimplementation contract.** The implementation and
candidate-specific platform/caller results follow in the addenda below; this
contract card itself supplies no execution pass.
This W11-W05/S0.2 child freezes only the substantiated residual above under
BND-3/4/5 and CHECK-1/17/21/25/26/27. Architecture §4.1.1 and the existing
hardening profile matrix supply the selected policy; no privilege, right,
dependency, metadata, schema or verification-policy change is proposed.
Preparation baseline is clean `a31835d6af400fa35f10bd5ccee68cf302b52505`.
Current parser/native-observer/host bytes match the three hashes recorded above.
Existing `error.rs` and `diagnostics.rs` bind respectively to
`18b29f03f343edfcba22268eac2eaadc4ded468d72fc7076604f3c86f307826e` and
`326c39efd8da786441bdeda446815f06507e80ff16fe279addf156f61ff5e78e`.

The actual path is `NativeTokenObserver::observe` → `resolve_ids` and
`capture_privileges` → `PrivilegeSnapshot::decode` →
`host::start_invocation_with_diagnostics` → worker preparation/readiness.
`start_invocation` delegates to that shared consumer. The current consumer calls
`require_engine_absence` only for Engine; the extension must enforce the selected
complete profile for **both** fixed roles before worker preparation, activation
or any Running acknowledgement. Successful SCM registration/StartPending remains
distinct from admitted ordinary work.

| Existing boundary / owner | Inputs, checked output, refusal and permitted effects |
| --- | --- |
| `native_token::{NativeTokenObserver::observe, resolve_ids, capture_privileges}` | Query only the current process primary token, resolve admitted names on that Windows instance, and capture the complete successful `TokenPrivileges` result within the existing 128-entry/1,540-byte limits. Failed/partial lookup, lookup collision, capture failure or incomplete output yields no successful observation. Keep before/after unexpected-thread-token checks, unique handle custody, explicit closes and actual-completion work accounting. Preserve the primary cause and independent primary/thread cleanup outcomes; cleanup failure prevents admission. No token adjustment, extra access right, powerful duplicate or silent repair is permitted. |
| `privilege_observation::{ServicePrivilegeIds, PrivilegeSnapshot::decode}` and their checked constructors | Retain the whole present LUID/attribute set in private bounded representation, including unknown entries. Preserve mandatory bounds validation, exact count/length agreement, complete 64-bit LUID identity, duplicate refusal, `SE_PRIVILEGE_REMOVED` refusal and the existing accepted attribute mask `0x8000_0003`; preserve actual accepted bits. Failure constructs no snapshot. Public raw decoding and caller-supplied lookup values establish representation only, never successful native capture, live role identity or current authority. Fresh OS facts, context and cleanup remain the native observer's obligations. |
| Exact-role comparison in the same portable owner, consumed by the shared startup function | Bind the comparison to the actual fixed `LabRole`/`ServiceRole`, with locally resolved admitted identities. Engine's present set must be exactly `{SeChangeNotifyPrivilege}`; Broker's exactly `{SeChangeNotifyPrivilege, SeSystemtimePrivilege}`, with Systemtime disabled for ordinary startup. Every extra LUID, including an unknown or disabled privilege, and every missing required privilege refuses admission. This adds no enabled-state requirement for ChangeNotify. Retain the existing typed SystemTime/Debug/Impersonate diagnostic facts and absence/disabled/enabled distinctions where applicable; new refusal categories remain bounded and typed without exposing raw token data. |
| `host::start_invocation_with_diagnostics`, error and diagnostic consumers | An observation or role refusal closes admission before any worker preparation/activation or Running. Preserve fixed-role selection, existing argument/Stop/time checks, typed observation/admission distinction, original failure, bounded secondary cleanup/report outcomes and required-event routing. Diagnostics cannot turn refusal into readiness. A successful portable comparison grants neither clock authority nor persistent/native token assurance; existing actor, thread-context, operation and current-authority checks retain their separate owners. |

**Compatibility and next source-write boundary.** Review all current callers of
the snapshot, lookup mapping, named projections and `require_engine_absence`
before changing signatures. Retain their narrower useful representation and
typed-error contracts where compatible; the old three-name helper cannot remain
the authoritative startup gate. Concrete layout/API choices may vary within the
checked contracts above. Expected source writes are limited to
`crates/windows-platform/src/privilege_observation.rs`, its existing
`tests/privilege_observation.rs` and `tests/privilege_observation_contracts.rs`,
and `tools/windows-service-fixture/src/{native_token,host,host_tests,error,error_tests}.rs`.
Adapt existing diagnostics mappings/tests only where the changed typed admission
result requires it. Core/engine, shared manifests/locks, schemas, policies and
qualification bindings remain coordinator-owned; provisioning and clock effects
remain with W11-W10/P6 and P4/S0.4. No parallel observer or policy engine is needed.

**Independent observations required after review, before full qualification.**
First retain a behavioral RED through the actual shared startup consumer: a
well-formed set with an extra unknown disabled LUID currently reaches worker
startup, or a Broker set with Systemtime enabled does so. The expected refusal
must fail for that admission defect, beside an independently encoded exact-set
healthy neighbor that reaches acknowledged readiness. Make the smallest causal
observer/representation/consumer repair, then establish GREEN for both roles;
do not alter the selected oracle or qualify the known deficient caller first.
The current source inspection is not an executed RED receipt.

Use independently written raw captures to challenge empty/missing required sets,
known and unknown extra privileges both enabled and disabled, Broker clock
privilege enabled, duplicate LUIDs, removed/unsupported attributes, malformed/
truncated/trailing data and 127/128/129-entry boundaries. Pair exact role sets
with both ChangeNotify enabled states where the remaining attributes are valid.
Exercise failed/colliding lookups, failed/incomplete capture, unexpected ambient
thread token and each explicit cleanup failure through the reachable owner seams;
preserve the primary failure alongside secondary facts and observe no worker
preparation/Running on refusal. Fixture API failures test portable propagation,
not actual Windows lookup, token or cleanup enforcement. External negative
consumers must challenge private snapshot/mapping mutation and guard bypass with
a compiling legal neighbor; do not claim that a public representation decoder
proves native provenance. Preserve required diagnostic fields and bounded output.

**Finite exit and next action.** After independent contract acceptance, author
one coherent source repair and its focused RED→GREEN controls, format it, then
freeze one resource-admitted configured qualification checkpoint. Require actual
ordinary and release tests, external contracts, relevant lint/docs/target checks,
at least 90% executable-line coverage for each affected security file and 95%
mutation per affected critical file, with complete descriptor/guard reconciliation,
independent alternatives for ungenerated behavior and no kill credit for unviable
or equivalent variants. Retain failures, discovery, finite campaign budgets and
candidate-bound results; no thresholds or resource floors change. Host tests cover
portable representation/admission/lifecycle behavior. Windows-only lookup/capture/
cleanup paths retain explicit coverage gaps; cross-builds establish compiler
compatibility, not native enforcement. Following David's authorization to use
Windows Sandbox, native guest checks require exact owned-Sandbox/job admission;
live host changes remain unauthorized. The coordinator owns Sandbox management
and guest execution; this draft author runs neither. Native token/account/thread,
package and installed lifecycle qualification retain their actual prerequisites
and unresolved evidence under the existing owners. Separate fuzz/storage/UI holds
and the post-V1 proof deferral remain unchanged.
No Rust/compiler, native, proof or fuzz execution occurred for this draft.
**Independent review completed:** fresh reviewer `privilege_contract_review`
traced both actual ServiceMain paths through the shared consumer, checked the
selected architecture/hardening policy and all five source bindings, and returned
READY without material findings. Its comprehension preserves whole-set versus
three-fact observation, both-role gating, representation versus native authority,
typed primary/cleanup outcomes and finite diagnostic projection. The reviewed
draft task bytes hash to
`3d2a36a31f2fd33be9edb60ea2137c0b202fb0aafed298d45ba216709c98d34d`;
this acceptance and the owner-authorization clarification are later amendments.
No compiler or Windows execution formed part of that read-only review.
**One permitted next action:** add the focused actual-consumer RED and healthy
neighbor, then perform the reviewed source repair and GREEN under the configured
runner. Implementation owner is the coordinator's platform/security child;
Git, shared manifests/locks/policies and all guest operations stay coordinator-owned.
The completed NTS/Core metrics and cloud metadata assignment are unaffected.

#### Exact-role repair — RED retained; source prepared — 9 November 2026

The actual shared-host regression ran against preparation baseline `a31835d`:
**84 passed, two intended failures**, with no ignored or filtered library cases.
The exact Engine and Broker profiles passed the healthy readiness neighbor.
Empty Engine, Engine with an unknown disabled extra, Broker with enabled clock
privilege, and Broker with an unknown disabled extra all wrongly admitted work,
reached Running and explicitly returned `Some(Ok(()))` after Stop. Each retained
six-field observation includes admission, worker phases, statuses and final result;
the tests did not manufacture the finalization transition. Later target cohorts
were not executed after the failed library.

The original outer result remains **incomplete**, Cargo exit 101, qualifier exit 1,
outer exit 1: its reader missed Rust's numeric thread ID in the panic header.
A separate bounded offline adjudication corrected only that anchored header
recognition, retained the original tuple/count/healthy-control checks and challenged
six malformed/incorrect neighbors plus a header without an ID. Independent review
accepted the actual behavioral RED without rerunning Cargo or changing the five
historical artifacts. External evidence under
`phase-plan-20261108-v1/nts-ke-graph-v1/`:

- `service-privilege-red-v2/fixture-test-command-0.log`, SHA-256
  `94be25e8fc0e12e1895b9e9ce7b63bf82f078e9457c3d6a73d6e78dbb9754ae4`;
- `adjudicate-service-privilege-red-v2-v1.py`, SHA-256
  `a0f8dd6785f67e95bdf2f79aa9cbb7b74673fa4eb7723788029a17df1e62fb7f`;
- `service-privilege-red-v2-offline-adjudication-v1.json`, SHA-256
  `cf4247b355fcbded11de0d6941f4720410df6d4144e1f43c9b2e7a3eb1a90a5b`.

The prepared repair retains all bounded LUID/attribute entries privately, resolves
ChangeNotify with the three existing names, and checks the exact selected role
before worker preparation. It borrows the complete snapshot, preserves typed known
refusal and cleanup facts, and excludes unknown LUIDs/raw captures from diagnostic
formatting. Both-role healthy fixtures now encode their actual required sets;
raw missing/extra/attribute/lookup/length/boundary controls and external privacy
consumers are updated. Source hashes after pinned formatting:
`privilege_observation.rs` =
`88fabdfc2f317cc38fdcd02f8f2fe2bcb84831be91ac01ad71aa7aceaea2c509`;
`native_token.rs` =
`56a1d575473ba85d40632ddf0c1a3b1dac1270a780040a991f0bb1a132c33791`;
`host.rs` =
`6139e99e3f7151afa885a7ce2b72befd481693f785d8c63c31947ee777774b22`.
The intervening `33a4a44` imports documentation/skill guidance; it changes no
product source. Its new guidance was preserved.

Coordinator support changes recognize only the new closed typed-refusal log
shapes, retain rejection of unknown/near-prefix errors, and rename the focused
admission filter. The updated Python control failed for the missing recognition
before that repair; **all 21 service-analysis Python controls then passed**.
The qualifier's fixed `privilege-checkpoint` lane composes the existing twelve
format/debug/release/doctest/GNU-and-Windows-Clippy/rustdoc commands under one
source/tool/vendor admission and target. Existing isolation, per-command limits,
bookends and stop-on-first-failure behavior remain. This avoids twelve independent
compiler-store admissions and duplicate targets; it grants no Windows execution.

**Next checkpoint:** independent source/runner review, a fresh frozen graph and
the twelve configured commands under one heavy-job owner, 5 GiB startup / 4 GiB
stop free-space limits, 768 MiB sampled aggregate owned output and 2 GiB sampled
owned RSS. GREEN, current external contracts, coverage/mutation and native token
qualification remain pending. The completed RED targets were deleted after review;
raw evidence/source snapshots remain. No proof, fuzz, live-host mutation or
service installation occurred in this source preparation.

**First configured checkpoint, retained as partial:** the fresh graph passed.
Both format commands, debug and release tests (**42 platform + 91 fixture cases
per profile**), eight platform doctests and platform Clippy passed. The fixture
contains zero doctests; no fixture-contract execution is claimed. Fixture Clippy
then correctly rejected an expanded 102-line test against the 100-line limit;
Windows Clippy and fixture rustdoc were not reached. Source/tool/vendor/index
bookends passed; the original outer result remains incomplete and the configured
lane remains failed. The repair extracts its unchanged finite refusal inventory
into a checked helper, preserving every behavioral case and the original test
name, followed by pinned formatting. A new checkpoint will use fresh labels and
freeze; the previous outcome is not replaced. Its reader also records the existing
zero-doctest fixture explicitly while requiring the nonzero platform contract
cohort. Raw records: `privilege-green-{graph,checkpoint}-20261109-v1.json` under
the existing service artifact directory; external supervisor result:
`service-privilege-green-v1/result.json`. Coverage/mutation/native gates remain open.

**Fresh GREEN v2 — independently accepted:** the graph and all twelve configured
commands passed. Debug and release each executed **42 platform + 91 fixture
tests**; all three original host regression/healthy tests passed once in each
profile. Eight platform doctests passed, including the current external privacy
consumers; fixture doctests remain zero. Format, GNU platform/fixture Clippy,
Windows-target fixture Clippy and fixture rustdoc passed. Independent review
rehashed all 467 freeze pins, both 461-file source inventories, twelve raw logs and
receipt bindings. The test-helper extraction changed no refusal case or oracle.
The 136.37-second outer run peaked at 492,953,600 sampled owned RSS bytes; its final
sample retained 5,040,607,232 available filesystem bytes, above the 4 GiB stop floor.
No native execution occurred.

External evidence under the same `nts-ke-graph-v1/` owner:

- `run-service-privilege-green-v2.py` SHA-256
  `08d17975192e4d5752f023479576d8c7d86fa7fb111a82564a363b33a4b018ca`;
- `service-privilege-green-freeze-v2.json` SHA-256
  `6a37b0c8f90b17b6559225c621f8b8e147c1aa404d981454e75ff88703868a5d`;
- `service-privilege-green-v2/result.json` SHA-256
  `0e9c29870b6e6d6633d6886f0e26ba822e39983cf5ddf6e72cb977375d8e3f5e`.

Main-repository checkpoint receipt:
`privilege-green-checkpoint-20261109-v2.json`, SHA-256
`a4b14000e5e9e3ff44d43d1cda1aba859f37911a2cd92f25171efd597f82fceb`.
The current service-analysis binding now uses the actual **42 ordinary/eight
doctest** inventory, current protected-helper hashes and the 436-file product
identity `fef3c7bd5d84706b5a61f1ea2855ce7afd68bfcb41d2513d6f0a08f365574685`.
Its old mutation inventories are cleared for fresh enumeration; no metric pass is
inferred from rebinding. The standalone fixture's larger 461-file inventory
includes its own source/runner, rather than changing the product metric scope.
**Next:** current descriptor enumeration and coverage, independent generation-gap
review, then the required per-file mutation campaign. Native lookup/token/SCM,
provisioning and package evidence remain open; Sandbox permission supplies no pass.

**Metric preflight correction:** the first enumeration attempt stopped before any
analyzer command because the outer supervisor's deliberately narrow
`PATH=/usr/bin:/bin` hid the already installed Cargo extensions. The configured
consumer uses `shutil.which` followed by exact binary-hash admission. Preserve
`analysis-privilege-role-enumerate-20261109-v1` and external
`service-privilege-metrics-v1/result.json` as incomplete; coverage did not start.
The next outer invocation supplies the literal existing Cargo-bin path through
the admitted system `env` executable. It retains the configured consumer's hash
checks, the unchanged supervisor and all resource/source/index controls. No tool
installation, analyzer-policy change or product failure follows from this setup
error. Native/product qualification remains unchanged.

**Actual metric results after PATH correction:** enumeration passed with **119
descriptors**, independently reconciled to current source/spans/diff hunks:

| Owner | Descriptors | Inventory SHA-256 |
| --- | ---: | --- |
| Lifecycle | 53 | `6b0c26ca69165cc0b1eca40786c7883413835757a7c5be5b7a38a713a41ed031` |
| SCM | 24 | `826a85bdb95825d972a1460054a8c05c76a6b33692d9592b23579e140281a5db` |
| Privilege | 42 | `5c5aaaa980cee306352d59333ab865951c1fb57e2843d5213386ee555462540b` |

These represent 50 of 55 concrete bodies. Constructors `PendingBudget::new`,
`ServiceLifecycle::new`, `StopNotification::new`, `PrivilegeLuid::new` and
`ServicePrivilegeIds::new` have no generated descriptors; the abstract reporter
has no executable body. Garde/derived equality, literal projections and native
capture/cleanup also need separate behavioral reconciliation. This enumeration
is no campaign score. Its raw report SHA-256 is
`4aca19745354b34a226bf22fd32780a98148bdf5d98efecee887b9c3e5ba7165`.

Coverage ran all 42 instrumented tests successfully but correctly failed the
**per-file 90% gate**: privilege 138/165 lines (**83.64%**), lifecycle 227/237
(**95.78%**), SCM 63/63 (**100%**). The missed privilege lines primarily concern
bounded useful error formatting and preserving the original typed error source.
Both the coverage lane and outer result remain incomplete. Retained coverage JSON
SHA-256: `996d8501dd46f62ffac59dacb1c1d8f456b95424e6b41e71709b0681b63fb1ca`.

The tests-only follow-up adds three actual-produced-error contract checks to the
platform test owner, covering redaction/useful categories, the known typed source
chain and absence of fabricated sources. It also strengthens the snapshot Debug
oracle: an empty string must not pass when named states and count are promised.
No product source changed. New test file SHA-256:
`82627b681210b56c7f4b845603bb5537736c1964a201ad80c0b2d47a081b11fa`.
These controls still need execution; a fresh candidate rebind follows the next
ordinary checkpoint. Old passes and the failed coverage report retain their scopes.

**Follow-up checkpoint v3, retained partial:** the three new contract tests passed;
debug and release each executed **45 platform + 91 fixture cases**, and eight
platform doctests passed. Platform Clippy then rejected three nonempty-string
assertions under `assert_is_empty`; the remaining lint/doc commands did not run.
The repair uses `assert_ne!(value, "")`, preserving the assertions and improving
failure output. The next fixed twelve-command batch runs format and Clippy before
ordinary/release tests, avoiding late discovery of this kind of lint failure;
it drops no check. Current binding expects the observed 45/eight test names.

Independent review accepts the 119 descriptor identities for their unchanged
production/tool/configuration scope. Those identities are now bound without
crediting a campaign or reusing the failed coverage result. The independent
generation-gap alternatives are the executed 81-input budget grid, typed
count-129-header / 1,541-byte capture boundaries and valid neighbors, direct
constructor/default/Stop cases, all six lookup-collision pairs, and raw high-bit/
maximum-length/tail-attribute equality cases. These discriminate the omitted
constructors and Garde invocation; another full historical fault campaign is not
required absent a survivor or unresolved distinction. The new error/source/Debug
oracles remain subject to current lint, coverage and mutation execution.
Current product binding: `d64665077745c91ff8278149eaa5873fe40e80bc46b3db2a53c40deb22cf5654`.

**Current ordinary checkpoint v4 passed:** all twelve commands completed after
lint was moved earlier. Both profiles passed 45 platform + 91 fixture tests;
eight platform doctests and the unchanged explicit zero fixture-doctest inventory
were observed. The three original host regression/healthy cases passed once in
each profile. Source/tool/vendor/index bookends passed. External result SHA-256:
`85de2f1b7d86cfbc12d7303d782a83e0ffd508d2633bb36d46bf2da45130a59b`;
main checkpoint receipt SHA-256:
`be56a2d7eb23385440f72546719f6560661fd690f64fc65e37fdda6073918e15`.
The prepared next run uses the configured `all` metric lane, so current coverage
must pass before mutation starts. It retains the 90%/95% per-file gates, four
mutation workers and existing compiler/test/time/isolation limits. Its separate
sampled aggregate output allowance is **1 GiB**, to include the coverage image,
four owned mutation targets and retained export together; 5 GiB startup / 4 GiB
stop, 2 GiB sampled owned RSS and a 600-second outer pilot remain. This is a new
bounded campaign budget, not a passed gate or weaker coverage/mutation threshold.
Unique historical fault sources and primary coverage artifacts were preserved
while unused compiler outputs and identical old source copies were deleted.

**First complete metric execution — measurements retained, acceptance incomplete:**
current coverage reached privilege **165/165 (100%)**, SCM **63/63 (100%)** and
lifecycle **227/237 (95.78%)**, with 45 instrumented tests. All 119 generated
mutation attempts ran: lifecycle **32 caught / 21 compiler-unviable**, SCM
**21 / 3**, privilege **29 / 13**, with no survivors, timeouts or exclusions.
Compiler-unviable cases receive no kill credit.

The strict causal consumer then rejected three returned string errors, so the
configured producer and outer result remain **incomplete**, not a qualified
campaign. Independent inspection confirmed successful build → test exit 101,
zero compiler errors and the intended named failure in each actual log:
legacy absence replaced with `Ok(())` returned “present privilege admitted”;
source replaced with `None` and its Forbidden arm deleted each returned “known
refusal lost its original source.” These were meaningful behavioral catches,
but `.ok_or(...)?` supplied no owned assertion span for this verifier.

The minimal tests-only repair stores the result/option, adds a standard
`assert!(refusal.is_err())` or `assert!(source.is_some())`, and retains fallible
extraction. It changes neither production behavior nor the expected contract.
No classifier whitelist or relaxed cause rule is introduced. Requalify the new
candidate and require all three cases to fail at those assertions. Keep
`analysis-privilege-role-all-20261109-v1` and external
`service-privilege-metrics-v3/result.json` immutable as incomplete history.

The standard assertion change passed the full ordinary checkpoint v5: all twelve
commands, both 45 + 91 test profiles and eight platform doctests, with all
bookends intact. Result SHA-256:
`ac975ddd8739d891acdd56b0fd8161bbad422fa3957ea5145e0bf96948359c98`;
checkpoint SHA-256:
`0a8b058245e1a068127c3de8919381d4d5b0572d9e6dc7f40d25025b78299c88`.
Current test source SHA-256:
`eafe62765ce5f34776c4155e50bb3e6ddecbd21fefbd16c8af890b7af17ffcfd`.
The final metric rerun uses product identity
`5962e80bc494ee227d9a2a424af090f8d28ea91a1335870ee3cb530969de1b47`,
the same production sources, 45/eight inventory and all 119 reviewed descriptors.
No metric pass is inferred before that execution and independent classification.

#### Platform checkpoint accepted; caller gates required at this stage — 9 November 2026

The final platform run passed coverage and all 119 mutation attempts with current
45-test/eight-doctest baselines. Independent review accepted every descriptor/diff,
all 82 attributed behavioral catches and 37 selected-source compiler failures with
zero kill credit. There were no survivors, timeouts or exclusions. Line results:
lifecycle **227/237 (95.78%)**, SCM **63/63 (100%)**, privilege **165/165 (100%)**;
per-file viable mutation results **32/32**, **21/21**, **29/29** respectively.
All 909 exported-file hashes matched, and the three formerly boxed-string-only
failures now have their intended owned assertion spans. Ordinary v5 checks and
the agreed direct-constructor/handler generation-gap alternatives remain accepted.

External final result `service-privilege-metrics-v4/result.json` SHA-256:
`a5af17a0d27eecf49365817e0e5e716e1b7ac2ff61b1cfc44f9abfc7ee852cfc`;
main record `analysis-privilege-role-all-20261109-v2/raw/verification.json`:
`dce4f69bb9c7cf3e3f96abb65b48888cf2b2269d2083f1feb3db968482851172`.
The 105.04-second run peaked at 1,715,318,784 sampled owned RSS bytes. Its outer
final owned-output sample was 294,650,825 bytes; the command's earlier final sample
was 258,608,145 bytes. These are separate observations, not interchangeable peaks.

**Required scope reconciliation before commit:** platform metrics do not qualify
the changed standalone `host.rs` and `error.rs`. Existing §4.35.9/§4.35.10 and
`tools/windows-service-fixture/metrics.py` require current host/error/diagnostics
90%/95% gates. Those remain pending. Its consumer requires a current single-command
`fixture-test` receipt; the batched v5 receipt is not admissible. All 25 fixture
Python admission/metric controls passed. Run a fresh graph and single fixture test,
then fixture coverage/enumeration, independent inventory review and mutation,
without relaxing receipt checks. Native token/SCM, installed identity and product
adoption remain separate unresolved owners.

The stale verification-manifest description now names complete representation and
both-role comparison instead of denying a broker allowlist. This is a description-
only semantic-delta rebind: registrations, thresholds, Rust source/tests, dependency
graph, tools and actual platform compiler/mutation commands are unchanged. The
retained platform results keep their original execution identities; they are not
relabeled as new runs. Current metadata-admitted product identity is
`212546396aff08c9b7bf84452137124d5ea91f7dff01627c0b271854d6b58a24`.
Apply this correction before freezing the already-required fresh fixture graph.

Before caller execution, local `main` fast-forwarded from `33a4a44` to `6b947c5`,
importing three documentation/skill commits. All 22 owned dirty paths were
preserved; architecture and this task were merged without conflicts, and staging
stayed empty. The new Tokio/panic-boundary guidance and W11-W02 resume refinement
retain their existing owners and native gaps. All 461 fixture execution inputs
and the current platform metadata admission were unchanged. The unused v1 caller
freeze/drivers remain unexecuted history; caller execution uses fresh v2 identities
and the new HEAD/index. No platform Rust or mutation rerun is implied by this
documentation-only integration.

#### Portable service privilege checkpoint final qualification — 9 November 2026

The previously pending standalone caller gates are now complete and independently
accepted. This closes the selected portable privilege observer/consumer child,
alongside the platform results above; it does not close W11-W05, S3.1, native
Windows services or product adoption. Requirements remain BND-3/4/5 and the
mapped CHECK-1/17/21/25/26/27 scope.

Fresh `fixture-caller-{graph,test}-20261109-v2.json` receipts passed four graph
commands and one exact ordinary fixture command: **91 named tests**, no failures,
ignored or filtered tests. The 461-input fixture candidate is
`fc85b335e9c9aa763499b89b63d406f72ec751305e64c2cfa25fc6ebfbb11b5e`.
Enumeration and instrumented coverage used that same candidate and all 91 tests:

| Selected caller | Executable lines | Generated | Behavioral caught | Compiler-unviable | Raw equivalent miss | Adjusted score |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `host.rs` | 505/536 (94.22%) | 83 | 47 | 35 | 1 | 47/47 (100%) |
| `error.rs` | 127/127 (100%) | 40 | 16 | 24 | 0 | 16/16 (100%) |
| `diagnostics.rs` | 119/119 (100%) | 16 | 3 | 13 | 0 | 3/3 (100%) |

The complete **139-variant** campaign has no timeout or unresolved survivor.
Actual mutation command exits are **2/0/0**; host raw coverage of viable faults
is **47/48 (97.9167%)**. The one exact `S31-EQ-POLL-FROM` raw miss actually ran
all 91 tests, retains `MissedMutant`, and earns zero kill credit. The separate
`poll_finished` replacement is not equivalent. All 66 catches have selected
semantic assertions; all 72 unviable variants have selected compiler/lint causes
and zero kill credit. Each owner ran a successful 91-test baseline. Independent
review reconciled every descriptor, diff and cause log, all ten omitted production
bodies against direct tests, and the small viable denominators. No branch-coverage
or native-collaborator claim follows.

Execution used four mutation workers, one inner Cargo job/test thread, unchanged
90-second build/60-second test limits, the configured aggregate 1200-second
campaign allowance and an explicit 1800-second outer allowance. Actual duration
was **119.92 seconds**, peak sampled owned RSS **1,440,985,088 bytes**, final
sampled owned outputs **62,140,566 bytes**; 5/4 GiB free-space admission/stop floors,
1 GiB output and 2 GiB RSS caps remained in force. Independent export readback
matched all **774 files / 29,921,366 bytes**. These are observed scopes, not hard
resource quotas or Windows measurements.
After acceptance, 1,707 byte-identical metric cache files (73,000,053 logical
bytes) were deleted; the complete primary raw archives and receipts remain.
No compression was used.

Current local evidence under `artifacts/windows-services/recovery-20261107-v1/`:

- `fixture-caller-coverage-20261109-v2.json`, SHA256
  `cd322746dbc2a7ab80f4db684a8ed51c134764d2d28e361b4b4584d49b41ff86`.
- `fixture-caller-mutation-20261109-v3.json`, SHA256
  `fa7582329b9d1025471d57fa22d5bcab85595e6ec917f75f59751eddb6cf7546`.
- External `service-fixture-mutation-v2/result.json`, SHA256
  `e58e63d967cc1f87fefa28279a96f9f05ce375e778a71d49fe32a83e865377ea`;
  freeze `1573ecf1fdbf79b701012b2b270af86f22f2e93f9d1031dd4bac2bc9a7dba8c1`.

David staged the same 22 work files before mutation. Coordinator Git-blob
readback proved exact mode/byte parity for all 461 verification inputs plus the
staged paths (471-file union), with unchanged HEAD and source hashes. Mutation
therefore used the new raw index `6966e171…`; A2/B2 retain their original
`8c713f4a…` execution identities. The index-only rebind record is
`service-fixture-index-rebind-v1.json`, SHA256
`41d5fca7e6c5248ce7b22518c1b00146da17157264184f01436f3bf7896dd591`.
The executed equivalence document `bf571782…` is retained byte-exact in the
archive; subsequent result documentation changes no compiled input or the
maintained equivalence argument. No historical result is relabeled as a rerun.

**Next:** finish staged-candidate/publication checks, then continue development
§7. Scoped Windows work uses the owned Sandbox and separately admitted exact
images/jobs under §4.19. Complete native token capture, role workflows, SCM,
accounts/provisioning, cleanup and two-family/package evidence remain open.
The blocked S134 artifact, live-host mutations, AES fuzz hold, optional-storage
pause, UI deferrals and post-V1 formal work retain their boundaries.

| Owner / stage | Concrete next work | Observable exit and current disposition |
| --- | --- | --- |
| Read-only reviewer separate from the extension author, W11-W05/S0.2 with W11-W10 for packaging | Preserve the source and contract reviews above; repeat affected claims only after relevant source/contract drift. | **Source revalidation and bounded contract review complete** on the recorded inputs. Current-caller implementation/qualification and native gaps remain explicit. |
| Coordinator's platform/security child, W11-W05/S0.2 | Preserve the completed portable observer/consumer checkpoint and actual-consumer RED/healthy controls; prepare the next exact native job under §4.19 when its contract and environment are admitted. | Platform and standalone caller ordinary/90%/95% gates are independently accepted in their selected Linux fixture scope. Native and installed qualification remain OPEN. |
| IPC/platform, S0.3/P3 | Bind identification-only QoS on every open/reconnect to actual token type/level, request/read ownership, query-only capture and checked same-thread reversion. Authorize explicit captured identity after reversion, without adding `SeImpersonatePrivilege`. | Allowed real user/admin/engine paths work; anonymous, unexpected level, failed token open and ambiguous attribution cause no protected effect. Existing failed-reversion recovery stays required. Native evidence remains OPEN; scoped guest work requires its admitted §4.19 contract/environment card. |
| Clock/lifecycle, P4/S0.4 | Bind the selected setter to guarded synchronous privilege enable/use/disable, full API results, disabled baseline and actual outcome custody. Permanent removal is not a temporary guard. Keep retained-rate/ownership cleanup in the existing broker recovery contract. | Valid bounded operation and later owned cleanup work; enable/not-all-assigned/setter/disable/readback failures cannot produce false success or unguarded continuation. Native evidence remains OPEN pending the existing disposable-fixture contract and §4.19 job admission; live-host clock changes are outside scope. |
| Package/platform, W11-W10/P6 with W11-W05 | Establish supported no-UI pre-start provisioning of exact nonempty lists, own-process type and object/identity policy; prove actual activation and preservation through update/repair/recovery. | Actual final package/service tokens match policy before ordinary work. The MSIX account/provisioning bridge remains OPEN; source API existence and elevation are insufficient. The retained child-launch alternative supplies no privilege exception. |
| Security/QA with provider, storage and diagnostics owners | Apply the existing paired profile matrix to useful real-role workflows, same-account adversaries, complete token state, bounded secret-memory/provider behavior and denied extra authority. | Candidate-bound native results on the two-family matrix, with valid controls, useful failures and external restoration. Current source-only observations provide no native, package or release pass. |

**Configuration and data contract.** Keep privilege names in the reviewed
role-policy source used by both provisioning and admission; do not accept them
from user settings, IPC or freely editable metadata. Serialize Windows
`MULTI_SZ` explicitly; do not derive an empty-list safety guarantee. Account
rights, service SID type, token privileges, token-object access masks and default
owner are separate fields with separate effects. A late setter/observer cannot
prove creation-time isolation or revoke already issued handles.

**Routing and unchanged decisions.** Updated architecture, source rationale,
MSIX compatibility note, hardening matrix, documentation guide, development
queue, service metadata row, privilege/service skill entry points and shared
service/Windows-evidence references. Existing IPC and package/security skill
routes already reach the canonical contracts and remain applicable. Normal-user
Slint UI and elevated lifecycle helper retain their distinct role contracts.
Store-first complete MSIX, all-file/interim-MSI signing, the current Core/service
queue and post-V1 formal work remain unchanged. Scoped owned-Sandbox jobs follow
§4.19; the blocked S134 artifact, live-host mutation restriction and separate
fuzz/storage/UI holds remain in force.

#### Guidance validation and review

**Judge review completed — READY for documentation and architectural handoff.**
Separate read-only reviewer `/root/signing_coverage_judge` inspected the complete
12-file diff, relevant canonical neighbors, the two changed skill entry points,
both changed shared references and representative observation/admission source.
It found no material revision-required issue and verified all candidate file
lengths/hashes against manifest SHA-256
`a5e522b0705624cf61e5d408ca7875118be88d1b2bfd874b915551db0f01bf1c`
and tracked-diff SHA-256
`a01576d6821631aa0c8cfaf1a34675e1e77a9855d1a0466a56a41c30cf4ea98e`.

The reviewer's two comprehension paragraphs are retained verbatim:

> The change selects one canonical privilege policy for NevroTick’s two runtime services: the engine retains only `SeChangeNotifyPrivilege`, and the broker additionally retains `SeSystemtimePrivilege`. It connects that policy to provisioning, complete token admission, identification-only IPC, guarded clock effects, existing qualification cases and the relevant skill entry points. This is an architectural and implementation handoff; it does not implement or qualify those behaviors.

> The current repository has a portable privilege observer and query-only laboratory service adapters, but their retained observations cover three named forbidden engine privileges rather than either complete service profile. Broker observation likewise does not establish exact privilege admission. The handoff accurately separates this existing source from future enforcement, actual Windows compatibility and the still-unqualified MSIX account/provisioning bridge, while preserving the current work queue and execution holds.

This was inherited-context review, not a fresh-context review. Earlier failed
fresh launches supply no additional evidence. Its READY result applies only to
this guidance/handoff stage, not implementation, native compatibility, service
isolation or release. No blind skill-use trial or measured behavior improvement
is claimed.

**Parent static validation:** all 39 added local links/anchors resolved; metadata
and UI records for all 66 registered skills matched the inventory; all 20 recorded
imported-file hashes matched; registration/UI/frontmatter were unchanged;
`git diff --check` passed. These checks parsed repository files as inert data;
no repository Python module was imported or executed.

The repository pack checker and its regression suite remain **NOT RUN**:
the `security-audit` skill requires OS isolation unavailable in this session.
Unchanged failed namespace setup was not retried and unisolated execution was
not substituted. These parent-authored checks provide narrower structural
evidence; the execution limitation is a session gap, not a new product gate.
No repository code, build, test, Windows fixture or installer was executed.
The final receipt changes only this review/evidence record; final parent
validation and publication identity remain distinct from the reviewed hash.

### 4.39 Direct MSIX distribution decision — 21 November 2026

**Accepted scope:** [OD-05 §7.9](../docs/nevrotick-owner-decisions.md#79-direct-msix-distribution-microsoft-store-rejected--21-november-2026)
records David's rejection of Microsoft Store publication and its broad publisher
indemnity requirements. Prepare one complete directly distributed MSIX. Signing
and certificate trust will be addressed later; continue independent product,
resource and package-authoring work. This supersedes every earlier instruction
in this record to attempt Store publication first, wait for a refusal, obtain
Partner Center identity or qualify a Store-delivered product. Historical research
and observed results remain unchanged and supply no direct-route qualification.

| Existing owner | Current assignment and exit |
| --- | --- |
| W11-W10/P6; package authoring | Prepare the complete direct MSIX and actual publisher/manifest requirements without Store identity or submission assumptions. Keep UI, engine, broker, helper and required payloads in one product; no automatic MSI fallback or external service prerequisite. |
| MD-P4–MD-P5; metadata/build | Reconcile actual Store fields, projections, pending dispositions and release validators with the direct route in a bounded implementation change. Preserve structural validation, independent output/privacy controls and source-mode resource work. Do not invent Partner Center values or silently delete required registry keys; this documentation checkpoint changes no generator or TOML. |
| W11-W05/W11-W08 with W11-W10 | Qualify services, provisioning/removal/recovery and event resources on the intended OS families. Resolve the proposed machine-scope event-provider custom capability on the non-Store route; sideloading alone does not waive capability requirements. |
| Security/signing; later release checkpoint | Select signer and certificate trust later. Requalify Store-dependent CIG/image policies against actual direct-signed payloads; preserve strongest supported compatible controls. Required signatures, correct identity and native admission remain future release evidence, with no customer trust-store/protection bypass. |
| WGT-0/P6/P7 | Preserve pre-V1 capability scheduling; qualify a non-Store source, identity and complete MSIX. WGT-C01 preparation remains distinct from WGT-C02 public catalog availability. No `msstore` ProductId, public URL, catalog submission or automatic updater is selected here. |
| Release/security | Bind built, signed, published and installed bytes through the existing custody/SBOM/licence/lifecycle checks. Hosting, updates and containment need the actual later-selected channel. Optional enhanced provenance remains optional. |

**Documentation workflow:** `plan-code-change` coordinates the owner-decision,
architecture/delivery and agent-guidance consumers; `plan-independent-review`
checks the integrated candidate. Local link/anchor checks, instruction inventory,
the AGENTS byte limit and `git diff --check` verify documentation only. No product
code, metadata values, SDK agreement, signer, package build, Windows operation,
release or Store operation is adopted or executed by this change. Unrelated
execution holds and current Core/Engine work remain with their existing owners.

#### Independent review and documentation evidence

**Reviewer:** `/root/direct_msix_independent_review`; fresh conversation
(`fork_turns: "none"`), read-only, no authorship or commit. **Completion:** completed.
The review covered the integrated delivery decision and its README, canonical
architecture, package/metadata/release plans and agent-guidance consumers at base
`de17282dddb59f54e069cecd71341e10cb0e5c72`. The reviewer independently verified the
49-file candidate's `git diff --binary HEAD` SHA-256:
`69c3d7c6b12d2a0fec5294f7e03e2d9a65d35d9210bd6e3dae0f09cbf487234a`.
The final publication adds only this receipt to that reviewed candidate.

The reviewer's comprehension paragraphs are retained verbatim:

> The proposed change keeps NevroTick’s release format as one complete hardened MSIX and changes its distribution route to direct delivery outside Microsoft Store. The README and OD-05 §7.9 explicitly record David’s objection to the Store publisher’s defense and indemnity obligations. Signing and certificate trust are deferred while source, resource and package preparation continue. Store submission, refusal, Partner Center identity and `msstore` qualification cease to be prerequisites.

> The current architecture retains a network engine without clock-setting privilege, a separate clock broker, an ordinary-user Slint UI and a narrow lifecycle helper. Source components and metadata tooling exist, but complete-package provisioning, native lifecycle, signing and release qualification remain open. The metadata generator still contains Store-specific consumers; the revised documents identify their future reconciliation rather than claiming a code fix. Required signatures, final-artifact custody, privacy controls, recovery and per-role security protections remain intact, including explicit reconsideration of Store-dependent CIG assumptions and custom-capability requirements.

**Reviewer verdict: READY for documentation integration and publication.**
Completed independent, read-only review. No material finding remains open in the
frozen candidate. The reviewer authored no candidate changes and performed no
commit or write. Scope: 47 Markdown files and two skill UI YAML files; no product
code, generator implementation, metadata values or native execution changed.

The reviewer's finding-resolution record is retained below:

| ID | Original issue | Final disposition |
| --- | --- | --- |
| **DIR-01 — current routing consistency** | OD-05 §7.7, the documentation reading guide and update-assessment routing still described Store-first delivery. Two live WinGet links also targeted the wrong section. | **Resolved.** Current guidance now follows §7.9. Retained Store history is labelled accordingly. The relevant WinGet links now target §4.36. |
| **DIR-02 — canonical metadata applicability** | `docs/plans/windows-package-metadata.md` still actively required Partner Center inputs, Store exports and Store-delivered readback despite the amended direction. | **Resolved.** Current identity, version, generation and final readback guidance follows direct MSIX. Existing Store fields, constraints and outputs are explicitly identified as legacy implementation requiring an owned future change. Structural validation, independent readback and private-field exclusion controls remain. |
| **DIR-03 — historical evidence preservation** | The server-catalogue draft’s historical coordinator reconciliation had been rewritten to include today’s distribution decision while nearby text promised that the historical record remained unchanged. | **Resolved.** The historical row is restored. A separate dated notice near the document’s beginning provides current routing without expanding the old review’s evidence. |

The reviewer checked the README's owner-decision wording and the cited Microsoft
agreement and platform documentation. It found no claim of legal invalidity,
automatic GPL incompatibility, an installable package or signing readiness.
Its verification independently covered candidate identity, scope, repaired
passages and the 32,647-byte `AGENTS.md`; the coordinator's structural checks
below are separate evidence, not reviewer reruns. Its permitted next action was
to append this review, verify that receipt-only delta and complete the authorized
documentation publication. No application, native Windows, signing or release
qualification follows from this verdict.

**Coordinator static validation:** `git diff --check` passed. Parent-authored
inert-data checks inspected 3,431 local link occurrences in the changed Markdown,
including 55 new/changed occurrences with no new failures. An unchanged
`[qa-performance-resources]: predeclare` prose-label finding in the development
plan remains outside the new-link result. All 66 registered skill frontmatters
and UI records passed structural checks; all 20 imported-file hashes matched.
`AGENTS.md` is 32,647 UTF-8 bytes, below 32,768. No root `docs/SHA256SUMS.txt`
exists; historical evidence-bundle hashes were not rewritten. These checks parse
repository files as inert data and import or execute no repository module.

The full repository skill checker is **NOT RUN**. The `security-audit` skill's
required OS-isolation preflight failed before repository code executed:
`bwrap: loopback: Failed to create NETLINK_ROUTE socket: Operation not permitted`.
Unchanged isolation setup was not retried and unisolated target execution was not
substituted. The checker regression suite is unchanged and was not selected for
this documentation scope. This session limitation is separate from product
qualification; it supplies neither a checker pass nor a new release requirement.
The final receipt delta receives the same bounded parent static checks before
publication; its final commit identity is separate from the reviewed diff hash.

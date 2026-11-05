# Deferred development parents

**Status:** DEFERRED / not ready for implementation. This single grouped record
fills parent-task gaps in [the task map](README.md); it does not replace the
[development plan](../docs/plans/development-plan.md), existing active children
or canonical architecture. Unless explicitly marked active, rows below are future
cards; none is completed evidence.
Role owners are responsibilities, not staffing assignments.

Activate only the next bounded child when its named prerequisites and current
authorization hold. In that existing chain, freeze candidate/source/graph,
literal independent expected facts, valid/rejected controls, environment,
finite execution limits and cleanup before implementation. Record actual exit
observations and gaps through the [testing contract](../.agents/skills/testing/CONTRACT.md).
Do not pre-create one task file per stage or start a second plan.

The current [Core continuation and holds](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
govern throughout: blocked S134 stager/artifact; general host Windows execution
and host elevation/security/service/clock changes; separate AES fuzz no-retry
hold; optional-storage experiment pause. [Owned-Sandbox permission](windows11-integration.md#419-owner-ready-sandbox-scope--3-november-2026)
is scoped management/readiness and separately admitted guest work, not a release
of those holds. UI-M0 remains fixture-only with D1/D2/D3 deferred.

P6 S6.1–S6.4 already has an owner: [W11-W10 with W11-W05](windows11-integration.md)
and the [selected MSIX plan](../docs/plans/windows-msix-store.md). Keep mitigation,
provisioning/recovery, one-product lifecycle and submission/delivered provenance
there. Complete CHECK-20/26, SBOM and non-Cargo payload reconciliation belong to
that exit; MSI is an alternative for a later David decision, not a prerequisite.

[Enhanced source-to-Store proof](../docs/plans/release-provenance.md) is optional
under [OD-05 §7.5](../docs/nevrotick-owner-decisions.md#75-optional-enhanced-build-provenance--4-november-2026).
Attestations, independent rebuilds and a public proof bundle add no P6/P7
activation or release-blocking requirement. Required custody, SBOM/licence,
package integrity and installed-product checks above keep their existing scope.

## P0 platform and policy

**Owner:** platform/security, with IPC, clock, UI and architecture/policy owners
as below. **Status:** deferred parents; existing S0.1 and W11-W05/W11-W10 S0.6
records remain authoritative. **Exit:** [plan §6/P0](../docs/plans/development-plan.md#p0--admit-sources-and-prove-platform-assumptions)
and [architecture §21](../docs/windows-time-architecture.md#21-delivery-phases-and-readiness-gates).

| Future card / role owner | Prerequisites and next bounded action | Independent observation required for exit |
| --- | --- | --- |
| S0.2 service tokens / platform/security | Intended account topology, admitted wrapper/graph, disposable Windows fixture and current native gates; prepare one two-service token/lifecycle experiment. Clock-writing subcase also needs the canonical temporary experiment card. | Intended broker operation succeeds within policy while engine/UI clock and broker-injection access fail; real callback/stop ownership remains bounded. CHECK-1/27 subset. |
| S0.3 mutual IPC / IPC/security | Relevant S0.2 identity evidence and §5.4 endpoint/caller-lifetime contract; specify one real status method plus one protected method and their allowed/denied principals. | Correct caller/server pair succeeds; fake server, wrong/revoked role, old session and restart/PID-reuse cases cannot gain effects. CHECK-2 subset; retain the canonical local-RPC alternative if pipe proof fails. |
| S0.4 backend semantics / clock/platform | Owned guest, controlled reference, current owner, independent recovery and predeclared operation/phase/rate/duration/attempt bounds; apply W11-W02/leap child and begin read-only/denied calls. | Measured sign/units/quantization and delivered/cancelled effects match independent time; kill/restart/power and ambiguous outcomes do not imply a finite-expiry guarantee. CHECK-10/11/12. Unset write bounds mean no write. |
| S0.5 full UI feasibility / UI/accessibility | Accepted [Slint default, OD-13](../docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit) and narrower UI-M0/comparison evidence; consume M0-SL-01/02 source preparation and admitted M0-SL-03–05 candidate evidence from the [existing migration handoff](ui-m0.md#slint-migration-handoff--5-november-2026), then define representative window/source/settings/elevation/tray qualification in its own scope. Restore relevant full-feasibility requirements without expanding UI-M0. | Named keyboard/Narrator/UIA journeys work at required contrast/scales; missing focus/labels or unusable scaling fails feasibility. CHECK-22; UI-M0/comparison success alone cannot close it. |
| S0.7 decision freeze / architecture/policy/security | Relevant P0 measurements, accepted owner decisions and remaining OPEN issues; prepare the next dependent decision card rather than guessing production limits. | ASVS/native-profile, step/date/holdover, supported-build, service trust/revocation/egress and native-work limits have explicit decisions or blockers. Unsupported alternatives remain blocked; CHECK-4/9/10/16/27 as applicable. |

**S0.4/S0.7 clock-limit preparation:** apply the [existing W11 matrix](windows11-integration.md#431-clock-correction-limits-preparation--5-november-2026)
to field definitions, bounded evidence and the next dependent OD-04 decision.
Native capability, model assumptions and acceptable policy tolerances remain
separate; source review adopts no production values or execution permission.

**S0.4/S0.7 platform preparation:** [W11 §4.32](windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026)
records OD-14's two intended OS families and first engineering fixtures. Platform,
build and W11-W05/10 owners resolve admission/delivery and actual fixture inputs;
P4/P6/P7 consume row-specific timing, protection, lifecycle and resource evidence.
OPEN-2, the current proof-first queue and all execution holds remain open/intact.

### Windows component reuse at existing stage entry

**Source findings integrated 5 November 2026; dependent implementation remains
deferred.** Apply [plan §4.4](../docs/plans/development-plan.md#44-apply-windows-component-reuse)
and [architecture §18.6.2](../docs/windows-time-architecture.md#1862-windows-component-reuse-and-binding-coverage)
inside these existing parents. This is a shared entry checklist, not another
workstream or an instruction to evaluate every Windows crate now.

| Existing parent / owner | Trigger and next action | Required evidence or retained disposition |
| --- | --- | --- |
| S0.2/S0.3, S2.1, S3.1–S3.4, S4.1; platform/network/IPC/clock | Before the next dependent native adapter, map its operation to a selected published binding/wrapper, version/features/SDK symbols and remaining product contract in W11 §4.29. | API coverage and compatible ownership/error/callback behavior; subsequent native controls under existing holds. Retain Mullvad candidate and Microsoft plural service hold. |
| S2.3 and reached protected-storage consumers; ST-W01/ST-W04 | Before VBS-dependent implementation, complete storage §6.7's binding/SDK coverage card and reconcile the missing flags. | Compatible generated/source-pinned API plus separate actual-key/OS evidence; REQUIRE semantics and policy-controlled fallback preserved. Native storage experiments remain paused. |
| Existing settings/state adapter; platform/UI | A concrete registry operation justifies comparing `windows-registry`. | Exact rights/DACL, bounded values and failure recovery preserved; no unconditional dependency or move of authoritative state into the registry. |
| S0.5/S5.1–S5.4; UI | OD-13 selects Slint and the existing M0-SL chain owns migration from the Win32 reference. A concrete issue preventing selected delivery and the bounded revisit procedure precede any alternative. | Preserve equivalent fixture, resource/lifecycle/error, accessibility, package/runtime and mitigation evidence. Retain the existing Reactor/Slint measurements; no general toolkit reselection or assumption that `windows-window` can own Slint's event loop. |
| S5.3; UI/diagnostics | Export needs a native picker: recheck working published `windows-pickers` or use the admitted OS dialog path. | Cancellation and error remain distinct, output stays bounded/redacted and written by the ordinary UI; no service arbitrary-path write. |

Retain [UI handoff](ui-m0.md#windows-component-reuse-handoff--5-november-2026),
[toolkit comparison trigger](s0.5-ui-comparison.md#conditional-windows-toolkit-candidates--5-november-2026),
[W11 routing](windows11-integration.md#429-windows-component-reuse-routing--5-november-2026)
and [storage coverage](windows-protected-storage.md#67-windows-binding-coverage-and-reuse--5-november-2026)
in their existing task chains. No optional crate becomes a current-Core,
UI-M0 or global release prerequisite; existing CHECK/ST-T evidence still governs.

### Deferred production research contracts

S0.7's architecture/security coordinator owns these research-to-contract handoffs
with the named production consumers. They remain open until their existing owner
decision and candidate-specific checks; they do not delay the current inert Core
checkpoint. [R-02/R-03/R-04](../docs/nevrotick-research-handoff.md) and accepted
OD-02/03 retain policy authority.

| Existing research / consumer | Deliverable before the dependent behavior | Independent exit and retained constraints |
| --- | --- | --- |
| R-02/R-04, OPEN-5/6; Core/acquisition policy before production voting | Freeze routine versus exceptional denominators, operator/dependence groups, timeout versus statistical rejection, observation age and uncertainty growth under architecture §7.2. | Compare four fresh groups, two unavailable groups, duplicate identities, a 2–2 split and one fresh source plus old predictors. Missing peers or predictions cannot manufacture fresh independent votes, lower the configured minimum or select a silent single-peer fallback. CHECK-7/8/14. |
| R-02, OPEN-6; security/persistence/P2 before persisted-floor recovery | Specify independently evidenced, narrowly authorized supersession of an intact but wrong future floor, including refusal and dependent model/association invalidation. | Valid recovery beside forged/stale/unauthorized supersession preserves old evidence/provenance, consumed correction budgets and installer/controller ownership; deleting a file or resetting optional state grants no fresh authority. CHECK-9/15; future TA-C10 does not replace this ordinary P2 gate. |
| R-03, OPEN-5/9; product/security/Core before S4.2 plain-profile correction | Propose plain-NTP versus authenticated recovery envelopes with cumulative absolute movement, count, restart and holdover semantics. | Malicious common-path/repeated-shift cases are discriminated from legitimate offline/RTC drift; report security and availability consequences for the owner decision. Twelve hours, 1,000 seconds and upstream step thresholds are comparison candidates, not adopted constants. CHECK-8/10/12/15; preserve OD-02's separate authenticated-recovery goal. |

## P1 Core completion

**Owner:** numerical Core/domain; tooling/dependency/QA maintains verification.
**Status:** deferred general parent; current S1.1–S1.3 and pure S1.4 consumers
remain in [their existing task chain](s1.3-source-selection-readiness.md).
**Exit:** [plan §6/P1](../docs/plans/development-plan.md#p1--pure-protocols-and-independently-checked-algorithms),
CHECK-6/7/8/19/20 and pure CHECK-25; this proves no Windows effect.

**Active bounded continuation, 5 November 2026:** the
[Verus addition pilot](s1.3-source-selection-readiness.md#verus-addition-pilot--5-november-2026)
stays in the existing Core task under OD-10 and CHECK-7/8/25. The full checked-pair
addition property remains unproved after Kani's **60,025 ms timeout with zero
checks**. That named gap and David's conditional request to use Verus if better
justify this trial; broader adoption remains **DEFER**.

| Card / role owner | Prerequisites and next bounded action | Independent observation required for exit |
| --- | --- | --- |
| S1.3 Verus addition pilot / numerical Core; tooling/QA | **Ready for compatibility/admission only.** Freeze the same full-domain addition contract and finite budget in the existing Core card; source proof depends on compatible actual APIs and completion within those limits. | Verify actual `Fraction::new`, `compare`, `complement` and `add_fractions` bodies with exact executable correspondence and source/tool custody. Require three compiled semantic defects to fail their intended obligations and a restored healthy pass. No product `external_body`, assumed central theorem, alternate executable behavior or model-only proof closes this exit. |
| S1.4 discipline/holdover / Core/domain | Applicable S1.3 selection/combination and current reference, achieved-effect and active-time delivered-rate contracts; finish the current child first, then freeze one abstract controller/holdover transition. | Independent trajectories give bounded step/slew/holdover proposals; stale generations, duplicate/unknown effects and unacknowledged requests cannot create delivered correction or fresh observations. CHECK-7/8/10/11/25, AC-19. Current S1.4a–c and rate-delivery receipts stay in S1.3. |
| S1.4 leap child / domain/Core/protocol | Use [the explicit entry and finite-input tables](s1.4-leap-policy.md#11-entry-conditions-after-the-current-core-checkpoint) after the current Core checkpoint. S157/S158 and selected S1.4a–c retain completed predecessor scopes; freeze the leap child's own date/data/import limits before dependent implementation. Static constructors do not require every later production contract. | Pure history/coverage/rule cases and atomic replacement are the first exit. Full silent-source acquisition, guard/reacquisition and LP-C04 stay in protocol integration; native mapping stays S0.4/P4. Existing LP-C cases and CHECK-6/7/8/11/14/25/26 retain their owning slices. |
| S1.5 verification closure / tooling/dependency/QA | Existing configured application gates and selected critical owners; maintain required checks now, close remaining phase scope at its actual checkpoint. Deferred QF-02 recipe maintenance stays acknowledged in the active Core task. | Healthy code/inputs pass and intended lint/SAST/discovery/admission faults fail actual gates; retained provenance, applicable per-file coverage/mutation and exact receipt scope support closure. CHECK-19/20/25/26. No new implementation prerequisite or duplicate campaign. |

Reuse unchanged Kani proofs within their recorded scope; the held Kani extensions
remain held. `ScaledValue::split` remains an independent fallback. The pilot adds
no global CI gate or phase blocker and lifts no execution hold; missing required
mathematical evidence still leaves its existing dependent acceptance open.

## P2 acquisition

**Owner:** engine/protocol/security with platform/dependency reviewers.
**Status:** deferred production parent. Reuse [engine](engine-transport-pilot.md),
[protocol](s0.1-protocol-comparison.md) and [crypto](s0.1-crypto-feasibility.md)
work at their qualified seams; fixtures, isolated kernels and host receipts do
not inherit production admission. **Exit:** [plan §6/P2](../docs/plans/development-plan.md#p2--secure-bounded-ntpnts-communication).

Under [OD-11](../docs/nevrotick-owner-decisions.md#15-od-11--socks5-scope-and-deferral),
P2/P5 retain Direct interface/family policy, upstream-failure recovery and path
reuse. Proxy support and its specific checks/UI are deferred to SOCKS-0 below;
they are not a prerequisite to first Windows delivery. Unsupported proxy state
must still fail explicitly instead of becoming Direct.

| Future card / role owner | Prerequisites and next bounded action | Independent observation required for exit |
| --- | --- | --- |
| S2.1 UDP/DNS observation / engine/platform | Applicable pure admission/correlation contracts, admitted graph and W11-W01 provenance; freeze one real controlled transport/scheduler child, including KoD next-send guards. | Actual IPv4/IPv6 valid exchanges succeed; wrong endpoint/correlation/generation and forged/stale KoD cannot create samples or bypass RATE/DENY/RSTR controls. Healthy peer remains usable. CHECK-6/8/21/25/27. |
| [S2.2 ordinary composition](engine-transport-pilot.md#ordinary-full-nts-composition--s22-preparation-5-november-2026) / engine/protocol/security | Source card prepared; implementation deferred. Exact TLS/provider/verifier/AEAD graph, ordinary Windows profile, finite runtime/lifetime contracts and local zero-cookie disposition precede dependent integration. | Fresh KE publishes one complete association; protected UDP completes once and eligible evidence reaches the actual Core consumer beside authentic numerical refusal. Paired path/packet/resource/trust controls retain independent vectors/interoperability. CHECK-6/7/8/9/17/20/21/25/26/27, AC-04. |
| S2.3 reuse/recovery / engine/security/storage | Qualified ordinary NTS first; OPEN-8/relevant recovery decisions and any optional-state construction/provider/pause gates; freeze one compatible-restart or provisional-recovery child. | Compatible reuse succeeds beside unavailable/stale/substituted state; strict authentication remains, no restored pending request, plain hint vote or global clock effect. Two-helper failure creates no silent one-helper path. CHECK-8/9/15/25/26, AC-02/09/10; applicable ST-T cases. |
| S2.4 adversarial closure / protocol/security/QA | Reached S2.1–S2.3 entrypoints and frozen retry/endpoint/native-job contracts; choose the remaining discriminating failure case. | Replay/endpoint changes, storms, stuck native work and cancellation preserve bounded real-job custody, atomic state and redacted output while valid neighboring acquisition works. CHECK-6/8/9/17/26/27; complete the linked checklist. |

Before cached/resumed association support, apply the existing
[R-05 authentication-lifetime card](s0.1-crypto-feasibility.md#deferred-nts-authentication-lifetime--r-05).
Before persisted-floor recovery or production voting, close the corresponding
S0.7 research contract above. These are dependent-behavior gates, not a new wait
for unrelated research before ordinary full NTS acquisition.

## P3 service boundary

**Owner:** platform/IPC/security, with Core and diagnostics owners.
**Status:** deferred full-services parent; apply W11-W04/05/08 and relevant
ST-W03–ST-W07 before dependent adapters. Optional-cache pause does not pause the
separate mandatory policy/authority/floor state owner. **Exit:** [plan §6/P3](../docs/plans/development-plan.md#p3--windows-services-secure-ipc-and-protected-state).

| Future card / role owner | Prerequisites and next bounded action | Independent observation required for exit |
| --- | --- | --- |
| S3.1 final services / platform/security | Relevant S0.2/S0.7 evidence, admitted narrow SCM adapter and native fixture; freeze one service's startup/status/stop child. | Intended service becomes actually ready; denied token actions and failed/delayed callbacks cannot report false readiness or escape shutdown ownership. CHECK-1/27, AC-01/13. |
| S3.2 authenticated handlers / IPC/security | S0.3 proof, final endpoints/tokens and strict framing/domain/current-authorization contracts; expand one permitted/denied method pair. | Valid current caller succeeds; malformed/oversized/stale/unauthorized/fake-endpoint/partial-I/O cases cause no protected effect; native buffers/permits survive actual completion. CHECK-2/3/25/27. |
| S3.3 mandatory policy/state / platform/security | Frozen revisions/migration/durable-outcome and directory-authority contracts; select one atomic mandatory-state transition and recovery case. Adopted optional storage remains a separately gated child under its existing task. | Valid current policy commits; corrupt/old/unauthorized records, disk-full/interruption and path substitution preserve safe state and charged authority. Optional reset cannot erase mandatory state. CHECK-15/25/27, AC-07; relevant W11/ST-W refinements. |
| S3.4 simulated integration/status / Core/platform/diagnostics | Actual secured services/handlers plus qualified pure proposals; integrate one journey against a simulated clock backend. | Valid proposal/status is coherent; stale generations, unavailable dependencies and malformed snapshots cannot claim Tracking, applied effects or success. Required redacted diagnostics remain bounded. CHECK-14/17/25/27, AC-05/16; actual clock effects stay P4. |

### Shared adapters, errors and diagnostics

**Owner:** platform/IPC/diagnostics with protocol/security review; executable
compositions retain their own sinks. **Status:** deferred interface handoff,
applied before the first dependent production adapter, handler or event emitter.
P0 prepares finite limits/sink policy; P2 uses it at reached protocol boundaries;
S3.1–S3.4 integrate it and S5.3 consumes redacted output. Current Core stays separate.

Use [architecture §3.4](../docs/windows-time-architecture.md#34-shared-windows-adapters-and-diagnostics),
[§6.5](../docs/windows-time-architecture.md#65-errors-diagnostics-and-proof-obligations),
[§12.3](../docs/windows-time-architecture.md#123-logging-and-support) and
[hardening §9.8](../docs/windows-time-development-hardening.md#98-diagnostics-and-shared-adapter-verification).
Freeze narrow role-specific `windows-platform`, portable `ipc-contract` and
platform-neutral diagnostics interfaces; no generic privileged Windows manager
or separate logging service is selected. Concentrate shared product-error mapping
and sanitization behind one facade in one or two shared modules; domain-specific
errors remain with their enforcing owners and retain useful internal causes.
Do not scaffold crates solely to satisfy this task.

Before exposing errors, unify bounded parser/Serde/Garde failures into stable
product codes and allowlisted paths. Unknown field names stay generic; freeze
finite count/byte limits and bound work before constructing a report. A valid
useful error is the neighbor for malformed input, secret sentinels, control
characters and raw-library-error leakage at actual IPC/UI/events/exports.

One decision owner emits one useful event. Required events reach their required
sink despite optional WARN/ERROR/OFF filters. Per-process authorized DEBUG
sessions expire locally; helper/UI exit, restart, lost acknowledgement and a
full/unavailable sink preserve bounded resources and authority. Test the actual
feature/sink closure and privacy, not just a formatting helper. Diagnostics
neither publish authoritative UI state nor acknowledge clock effects. Existing
CHECK-1/2/3/14/15/17/21/25/26/27 apply at their reached boundaries.

## P4 clock integration

**Owner:** clock/platform/security with independent reference/QA owners.
**Status:** deferred; relevant S0.2–S0.4/S0.7 and P3 evidence, native admission,
owned disposable fixture/reference/recovery and explicit finite experiment card
precede real effects. Apply W11-W02, leap child and relevant state/adapter
refinements. **Exit:** [plan §6/P4](../docs/plans/development-plan.md#p4--windows-clock-integration-ownership-and-recovery).

Use [W11 §4.31](windows11-integration.md#431-clock-correction-limits-preparation--5-november-2026)
for the ordered producer/owner/consumer handoff and paired controls. Account actual
combined movement and late durable outcomes even when Core rejects publication;
qualify holdover assumptions and conditional recovery exposure. OPEN-5/7/13 and
all current holds remain; no parallel clock task or numerical envelope is added.

| Future card / role owner | Prerequisites and next bounded action | Independent observation required for exit |
| --- | --- | --- |
| S4.1 backend/accounting / clock/Core | Measured sign/units/quantization and actual-outcome contracts; freeze one acknowledged operation and lost-acknowledgement case. | Independent time matches delivered effect; unknown/duplicate outcomes cannot trigger a blind repeat or mischarge requested effects as achieved. CHECK-10/15/25/27, AC-19. |
| S4.2 limits/budgets / broker/security | Explicit ordinary/initial/exceptional/date/era limits and durable boot-budget decisions; select one within-limit proposal and forbidden neighbor. | Current approved bounded effect succeeds; oversize/stale/replayed approval and reconnect/sign-change/crash cannot renew authority or budgets. CHECK-3/10/12/15, AC-08/11. |
| S4.3 power/generations / platform/clock | Qualified active-awake/elapsed clock mapping, cancellation and leap guard contracts; choose one suspend/resume or mapping-change journey. | Fresh post-transition quorum restores useful tracking; late/crossing work is rejected, sleep is not delivered slew and no synthetic leap correction occurs. CHECK-10/11/27, AC-12. |
| S4.4 ownership/recovery / platform/security/lifecycle | Final W32Time/controller ownership, takeover/restore and outcome-journal contracts; select one current-owner recovery plus successor/ambiguous-owner control. | Owned cleanup/restoration succeeds; GPO/competing controller/unknown successor blocks blind resets, crash loops retain consumed budgets. CHECK-5/10/12/13/15, AC-08/13/14. |
| S4.5 reference soak/faults / QA/performance/platform | Integrated bounded controller, declared convergence/recovery envelope and supported native fixture; run the next prescribed reference/soak child once admitted. | Independent trajectories and cleanup/resource observations meet the declared profile; outages/crashes cannot preserve false freshness or hide surviving native work. CHECK-10/11/12/21/27, AC-19/20. |

## P5 production UI

**Owner:** UI/accessibility with service/security/lifecycle owners.
**Status:** deferred; [UI-M0](ui-m0.md) remains its own restricted milestone.
[OD-13](../docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit)
selects Slint as the default, with the measured winit/software configuration as
the starting point. Product graph/licence admission and exact-candidate
qualification remain open in the existing UI-M0 migration handoff: M0-SL-01/02
are source-preparation rows; M0-SL-03–05 remain held on their named prerequisites.
Consume those exact-candidate results without repeating the pilot; no new toolkit
selection round or implicit transfer of Win32 evidence is required.
Restore D2 freshness/reconnect/generation semantics before any live read-only
status, then revision/authorization/confirmed-failed-unknown results before
protected commands. Restore D1/D3 before general-user preview/release or an
earlier corresponding qualification claim. **Exit:** [plan §6/P5](../docs/plans/development-plan.md#p5--truthful-accessible-production-ui).

| Future card / role owner | Prerequisites and next bounded action | Independent observation required for exit |
| --- | --- | --- |
| S5.1 real status/sources / UI/service | Admitted Slint adapter/checked bridge from the UI-M0 chain, actual secured current snapshots, relevant P2/P3 evidence and D2 status re-entry; wire one read-only journey using canonical design references. | Valid real status matches authentication/agreement/ages/principal/supporters; unavailable/stale/malformed/old-generation evidence cannot look current. CHECK-14/25, AC-02/03/05; applicable ST-T01/14. |
| S5.2 settings/helper/review / UI/security/platform | Admitted Slint adapter/checked handler bridge, actual receiver/helper/policy revisions and D2 command re-entry, plus applicable P4 authority; expand one current operation and rejection/cancellation pair. | Authorized current request succeeds; UAC cancellation, stale edit, denied/lost/unknown result and replayed approval cannot appear successful. CHECK-2/3/10/15/25, AC-06/07/11. |
| S5.3 diagnostics/tray / UI/diagnostics | Qualified per-process output/privacy and observer lifecycle; freeze/admit any actual Slint tray feature/native graph expansion, then select one export or optional-tray journey. The pilot omitted tray; normal Close must still leave no UI by default. | Useful redacted export and enabled observer work; sink failure/stale status cannot mislead, disabled tray/default close retains no UI, services survive UI exit. CHECK-12/14/17/21/23, AC-01/16/18. |
| S5.4 full usability/resources / accessibility/QA | Reached production journeys, D1/D3 restored, admitted accessibility-enabled Slint graph/Windows bridge and supported display profile; qualify one complete critical journey before expanding the matrix. The pilot's omitted feature and Win32 receipts are no accessibility pass. | Actual keyboard/Narrator/UIA, contrast/scaling/long-text and native resource observations pass; lost focus/inaccessible action/retained resources fail qualification. CHECK-21/22/27, AC-17/18/20. |

### Production UI contract handoff

Before each relevant S5 child, read the maintained
[UI specification](../docs/nevrotick-design-v0.2/docs/02-ui-spec.md),
[source editor contract](../docs/nevrotick-design-v0.2/docs/03-server-configuration.md),
[state/acceptance matrix](../docs/nevrotick-design-v0.2/docs/04-states-and-acceptance.md)
and [feature correspondence](../docs/nevrotick-design-v0.2/docs/06-features-review.md).
Canonical architecture and later owner decisions govern semantics; archived
source copies, MSI wording and rendered scenes are references. Do not execute
the archived React tooling until its recorded dependency findings are resolved.

- S5.1/D2: retain separate source-reply, eligible-estimate, acknowledged-correction
  and UI-snapshot ages. Unknown health differs from confirmed service stop.
  Certificate-time basis/bootstrap history survives later NTS success; calendar
  uncertainty and per-store protection/readiness remain separate published facts.
- S5.1/D2 refresh: coalesce repeated Sync now requests, preserve existing evidence
  and report actual checking/waiting/limited/blocked/unavailable outcomes. A check
  completion does not update last-applied correction; UI close cancels no
  service-owned measurement or correction.
- S5.2/D2: preserve active versus draft policy and personal preferences. Apply
  revalidates at the receiver against the current revision/authority; UAC cancel
  and conflict preserve draft/focus and active policy. New-generation sources
  become Waiting; accepted configuration does not remove Blocked/ownership limits.
  Lost acknowledgement after admission requires result reconciliation before retry.
- Source editing: NTS-KE TCP and negotiated NTP UDP endpoints stay distinct.
  Host/IDN/IP/port validation agrees at UI and receiver; permitted LAN sources work
  beside forbidden post-DNS/delegation targets. Profile changes never silently
  rewrite protocols, manufacture independent votes or reduce minimum agreement.
- S5.3/S5.4: qualify Copy summary preview and explicit export/redaction, source
  detail/recovery history and expired one-use approvals; tray/sign-in choices are
  per-user, off by default, and do not alter services. Restore D1/D3 at their
  existing triggers, including field-associated errors and actual focus/UIA.
  Use the [AVIF manifest](../docs/NevroTick-AVIF-kit-v1/manifest.json) and preserve
  proprietary brand and third-party notices separately from the code licence.

These are later production observations under existing CHECK/AC cases; they add
no UI-M0 implementation or qualification requirement.

## P7 final qualification

**Owner:** independent verifier with security/platform/dependency/release owners;
David remains exact-candidate release approver. **Status:** deferred. Applicable
P0–P6 results, supported environment, selected complete MSIX and actual
Store-delivered bytes/trust precede release closure. Next freeze the candidate
and reconcile earlier evidence invalidation before choosing the first remaining
black-box observation. **Exit:** [plan §6/P7](../docs/plans/development-plan.md#p7--final-integrated-acceptance).

Independent valid journeys must match admitted inputs, actual status/effects,
persisted state and installed payload. Malformed/unauthorized/stale/tampered
neighbors must fail without protected effects while valid controls remain usable.
Reconcile CHECK-1–27, AC-01–20, applicable ST-T cases, critical-file gates,
native accessibility/authority/lifecycle and baseline/soak evidence. Missing
required evidence stays incomplete; only David's explicit applicable decisions
can accept exceptions. Fresh review and release approval bind the exact artifact.
CHECK-4 uses the [pinned ASVS worksheet](../docs/asvs-5.0.0-applicability.md):
reconcile its 345 rows and individual outstanding questions against actual
surfaces/evidence, including owner-reviewed N/A/native interpretations and
OD-04/OPEN-9 decisions. This restates the existing gate, not a new compliance claim.

## Integrated slices

These are joins of the same parents, using [feature plan §9](../docs/windows-time-features.md#9-delivery-slices-for-agentic-development);
they introduce no second execution chain or independent readiness claim.
The development coordinator owns activation, one integrated candidate and the
combined exit for each slice; participating role owners retain their existing
boundaries. In particular, S-2 closes only on the joined acquisition, secured
status and real read-only UI observations, not on separate child reports.

| Slice | Parent tasks and observable combined exit |
| --- | --- |
| S-0 foundations | P0 cards, including existing S0.1/S0.6 owners: finite platform/authority/UI/package evidence and explicit dependent decisions/blockers. UI-M0 alone does not close S-0. |
| S-1 checked model | Existing P1 children plus deferred Core completion: independent admission, trajectories, selection/principal/supporters and applicable critical-file gates. |
| S-2 observe-only client | P2 acquisition + P3 secured service status + early P5/S5.1 with real D2 re-entry. Actual admitted NTS/explicit NTP, denied controls and ordinary-user real read-only UI; no global clock writes. Headless pilot or fixture window alone is insufficient. |
| S-3 unattended sync | P3/P4 plus existing installation owner: real bounded clock trajectory before login/with UI closed, actual ownership, stop/restart/recovery and handover. |
| S-4 recovery/admin | P2/P4/P5 under applicable OPEN-5/6/8 decisions: AC-07–14 valid/failure pairs, current one-use approval and no downgrade/hint-derived clock authority. |
| S-5 finished experience | P5 plus existing P6 owner: usable real screens/tray/export, keyboard/Narrator/scaling, resource and interrupted-maintenance observations. |
| S-6 exact release | Existing P6 owner + P7: complete hardened MSIX preparation, David's Store outcome, actual delivered/installed trust and full applicable acceptance before public release. |

Later [roadmap handoffs](../docs/plans/development-plan.md#86-handoff-after-the-windows-plan)
are Windows P0–P7 → KHR-C01 disposition → GEO-C01 disposition → Android, with later RGT-0 → CHR-0
and the [shared time-assurance task](time-assurance-integration.md) retaining
their distinct exits. Future DNS retains its own DoQ-only/DNS-C owners; do not
invent ordering between DNS and Android or add either as a current Core gate.


## Future roadmap parents

**Status:** DEFERRED, outside current Windows P0–P7 execution. These parent cards
retain the [roadmap](../ROADMAP.md) and focused-plan owners; they authorize no
device/host changes, dependency adoption, public probes or clock effects. Expand
only the next admitted child when its future stage is reached. Do not invent an
ordering between DNS and Android. KHR/RGT stay in the existing time-assurance task.

| Future parent / role owner | Entry and first bounded child | Existing exit / controls |
| --- | --- | --- |
| GEO-0 / catalogue/release/engine/QA/security | After KHR-C01, before Android. The [existing catalogue task](server-catalogue-and-statistics.md#deferred-geoip-experiment--after-khronos) freezes a bounded comparison of ordinary acquisition with optional geography ordering, reusing C/R/Q ownership. | GEO-C01 records actual evidence/gaps and `ADOPT_LIMITED`, `DEFER` or `REJECT`; adoption needs measured benefit within unchanged constraints. No changed reserve sampler, trust vote, mandatory client-IP lookup, parallel database or dependency on deferred Roughtime/SOCKS5 runtime. Any GeoIP implementation is separately scoped. |
| AND-0–AND-6 / Android/platform/protocol/ADB/QA | Windows P0–P7, then KHR-C01 disposition and qualified Khronos implementation if adopted, then GEO-C01 disposition before Android. GeoIP implementation is not required. Start AND-0 read-only device/build discovery, then an ordinary-UID loopback fixture under explicit device/restore bounds. | AND-C01–06 retain device/APK, local responder/NTS, reversible ADB setup/restoration, lifecycle and signed-preview evidence. No generic Android clock-write privilege is assumed. AND-6/AND-C07 server scope remains separately conditional. |
| CHR-0 / numerical/protocol/provenance/rights | Immediately after RGT-C01 in the existing RGT-0 → CHR-0 sequence; freeze the source/material ledger, consumer boundary and independent contract before replacement work. | CHR-C01 requires independently checked behavior, complete material/dependency provenance and reviewed rights. Current Chrony GPL obligations remain until an actual qualified replacement; public licence and Microsoft permission decisions stay with David. |
| SOCKS-0 / engine/protocol/Windows/security/QA/UI | Beyond first Windows P0–P7 delivery, when a real use case and maintenance capacity justify it; placement among future work remains open without reordering KHR/Android or RGT/CHR. Reuse the [existing engine task](engine-transport-pilot.md#socks5-feasibility-and-deferral--5-november-2026) to freeze the manual loopback/no-auth/local-DNS subset and qualified relay/provider envelope. | SOCKS-C01 and applicable PATH/AC/CHECK cases: actual CONNECT plus UDP, strict no-escape/recovery, endpoint identity, service lifetime, DNS/verifier routing, timing and bounded resource evidence. Remote/auth/proxy-DNS extensions are separate later admissions, not first-subset requirements. |
| DNS-0–DNS-6 / DNS/platform/security/resources | At its future phase, start [DNS-0 contract freeze](../docs/plans/system-dns-cache-design.md#12-staged-delivery-and-next-action), then the plan's baseline → DNSSEC → native integration → shadow demand → bounded refresh sequence; DNS-6 remains conditional. | DNS-C01–18/DOQ-C01, DoQ-only upstream and leaf/intermediate pinning; qualify actual application routing rather than configuration readback. Apply [source-derived ownership/publication/retained-memory/async controls](../docs/plans/dns-upstream-lessons.md) at DNS-1/2 and demand/forget/privacy/persistence controls at DNS-4/5. W11/ST refinements and optional-storage pause retain their scope. |

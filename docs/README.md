# NevroTick — documentation guide

**Prepared for:** David Osipov  
**Initial snapshot date:** 29 October 2026; later amendments are dated below

**Canonical architecture:** RWTC-ARCH-001, revision 1.4

The earlier guide inventory recorded eleven principal Markdown documents, seven supporting independent reviews, a UI toolkit comparison report, an x64 runtime-optimization research report, a critical architecture-contract assessment, a state-guided fuzzing experiment report and a Verus assessment; use the routing below for maintained additions. The owner-decision/research addition is a scoped amendment, not a repeated full-product qualification. The feature plan translates the architecture into basic user capabilities, screens, defaults, acceptance cases and delivery slices. The architecture and development-hardening guide include 29 October 2026 amendments making upstream-code verification and stress testing explicit, adopting garde-backed construction plus explicit function/module contracts, and integrating installed native memory diagnostics into agent workflows. The release policy adds operational preparation with proposed support terms. Proposed behavior and planned checks are not implementation evidence.

## Plans, acceptance and execution evidence

For the single editable product metadata source and build-time pipeline, start
with [metadata/](../metadata/README.md). Its [five-stage contract](plans/windows-package-metadata.md#13-metadata-pipeline-implementation--7-november-2026)
keeps generated source separate from actual component, native and release evidence.


Agents start with [AGENTS.md](../AGENTS.md), choose the existing task and relevant
rows below, then load the matching [repository skills](../.agents/skills/README.md).
Use the [testing contract](../.agents/skills/testing/CONTRACT.md) to select evidence.
The plan is distributed by responsibility; these links locate its owners without
creating another specification.

The [development execution plan](plans/development-plan.md) consolidates the
P0–P7 sequence, NTP/NTS priorities, upstream evaluation, explicit implementation
contracts, mandatory skill reading by phase and phase-end acceptance. Use it to
assign the next bounded development task; the canonical documents below continue
to own requirements, and task records own executed evidence.

| Question | Source of truth |
| --- | --- |
| What code exists now, and which results apply to it? | [Current implementation summary](windows-time-architecture.md#1-current-state-capsule), backed by the workspace manifests, lockfile, source and linked candidate-specific task evidence. Dated baseline and review records below remain historical. |
| How is service management limited to NevroTick while preserving administrative recovery? | [Canonical route/admission policy](windows-time-architecture.md#421-service-management-admission-and-administrative-recovery), [Microsoft source review and hardening map](plans/windows11-integration.md#service-management-hardening--7-november-2026) and [W11-W05 assignment](../tasks/windows11-integration.md#w11-w05-observe-effective-scm-maintenance-and-process-policy). Separate SCM principal rights, protected application admission and current effect guards; MSIX peer/provisioning/recovery behavior remains unqualified. |
| How should agents create, manage, harden, log and optimize Windows services? | [implement-windows-service](../.agents/skills/implementation/implement-windows-service/SKILL.md) routes the maintained SCM/native guide, canonical service/diagnostic/resource contracts and W11-W05/W11-W08/W11-W10. The [7 November source refinement](plans/windows11-integration-follow-up.md#additional-service-practices--7-november-2026) adds argument-channel separation, effective unattended errors, mutex crash/thread semantics and conditional environment/startup cases. Source preparation supplies no native permission or service/package pass. |
| What should a development agent do next, and which skills must it read and use? | [Development execution plan](plans/development-plan.md), especially its phase skill map, acceptance gates and next executable task. |
| Which task owns each phase, integrated slice and deferred continuation? | [Task index](../tasks/README.md) maps existing implementation and evidence records; the [grouped deferred backlog](../tasks/development-backlog.md) supplies uncovered phase parents and activation criteria. The development plan still controls order and acceptance; task presence proves no readiness or completion. |
| What must formal proofs actually establish, and what is still unproved? | [OD-10 §14.2](nevrotick-owner-decisions.md#142-proof-purpose-and-completion--6-november-2026) and the [product-guarantee debt](../tasks/development-backlog.md#product-guarantees-still-open) define scope, missing composition and completion; the [agent workflow](../.agents/skills/testing/references/formal-verification.md) prevents helper/tooling counts from replacing useful guarantees. All formal execution remains post-V1 under §14.1. |
| Where are the personal project direction, future platforms and DNS plans recorded? | [Product roadmap](../ROADMAP.md): Android local SNTP bridge, ADB setup/restoration and feasibility gates; [system-wide Windows DNS/cache design](plans/system-dns-cache-design.md) for DoQ-only upstream resolution with leaf/intermediate pinning, DNSSEC acceleration/multicore work and bounded adaptive caching. The Windows execution plan continues to own current work, phase readiness and acceptance. |
| How should Roughtime work with NTS and Khronos/Ananke? | [Time-assurance coordination design](plans/time-assurance-integration.md) and [shared future task](../tasks/time-assurance-integration.md): preferred Roughtime cold bootstrap, conditional Khronos/Ananke provisional-date reserve, then full NTS; dated server census, provenance, failure classification, epochs, conflict/resource rules and Windows gates. KHR-C01 and RGT-C01 remain separate future exits, with no new current Core prerequisite. |
| What data should the default-source catalogue and source-quality history retain, and where might it live? | [Server catalogue and statistics draft](plans/server-catalogue-and-statistics-draft.md): data inventory, authority and provenance, proposed RAM/SQLite/compressed-history allocation, catalogue verification, database protection and optional NTS pinning. [Storage and format tables](plans/server-catalogue-storage-tables.md) provide the companion data/location/encoding/rationale/capacity/security matrices. Exact client representations, budgets, cryptographic providers and runtime implementation remain unselected or unqualified as stated there; the later shipped-catalogue direction and separate publisher input stage are recorded below. |
| How should a compact release catalogue include hundreds of sources, Roughtime keys and future collection tooling? | [Binary catalogue refinement](plans/server-catalogue-binary-format.md): prefer binary delivery, with constrained CBOR and generated immutable tables as candidates; require Roughtime key/profile material in the first schema; the [offline publisher collector](../tools/server-catalogue/README.md) implements bounded local candidates and semantic diff. The [existing catalogue task §8](../tasks/server-catalogue-and-statistics.md#8-offline-publisher-collector--4-november-2026) owns its exact verification evidence and remaining build/admission work. Catalogue size does not enlarge the active-source set, settle OD-03 or implement RGT-0; Q0/Q-J retains its separate scope. |
| Which catalogue decisions remain open, including expected DNSSEC protection? | [Catalogue open-question register](server-catalogue-open-questions.md) consolidates OPEN-CAT-01–03 and the proposed per-source DNSSEC continuity rule. It preserves Roughtime-first bootstrap, existing DNS validation owners and the current Core queue; no new preset, encoding, validator or release blocker is selected. |
| Where are the origins and qualification scenarios for borrowed DNS mechanisms? | [DNS upstream lessons and adaptation plan](plans/dns-upstream-lessons.md): pinned Unbound, AdGuard Home/dnsproxy, Pi-hole FTL and dnscrypt-proxy sources; eighteen lessons, including the [DNSSEC/cache follow-up](plans/dns-upstream-lessons.md#dns-r11-reuse-verification-results-bound-to-exact-content-and-proof-scope), with observed mechanisms, proposed adaptations and existing DNS-C owners; the [Security-kit follow-up](plans/dns-upstream-lessons.md#security-kit-follow-up-cache-contracts-and-regression-cases) adds TS cache contracts and source-derived regression cases under the same owners. |
| Where are protected key storage, VBS fallback and their evidence gates owned? | [Architecture §12.1.2](windows-time-architecture.md#1212-protected-optional-storage-and-explicit-fallback), [protected-storage plan](plans/windows-protected-storage.md), [development workstream](plans/development-plan.md#84-protected-optional-storage-design-and-qualification) and [provider qualification](windows-time-development-hardening.md#641-protected-storage-provider-qualification). |
| Where are source examples and adaptation limits for protected Windows storage? | [First upstream review](plans/windows-protected-storage-upstream-lessons.md) and [second upstream review](plans/windows-protected-storage-upstream-lessons-2.md): pinned implementation/test sources, adaptation limits and future checks. The second adds directory authority, indeterminate commits, crash/protocol models, per-call CNG contracts, behavioral mitigation checks, RedirectionGuard and conditional rollback research. |
| What should an agent do with the protected-storage research? | [Protected-storage agent task](../tasks/windows-protected-storage.md): bounded work items, prerequisites, owners, independent expectations, healthy/defect controls and candidate-specific evidence, mapped to the existing storage plan. The native experiment pause and current development priority remain unchanged. |
| Which Microsoft Windows 11 mechanisms should be evaluated, and where does their work belong? | [Windows 11 integration research](plans/windows11-integration.md), [Microsoft Learn follow-up](plans/windows11-integration-follow-up.md), [phase workstream](plans/development-plan.md#85-windows-11-integration-research-and-qualification) and [agent task](../tasks/windows11-integration.md): packet timestamps, sleep/timer behavior, DNS/WSH integration, service/MSI/COM lifetimes, resource signals, CNG, diagnostics and future UI. The follow-up adds exact completion, metadata, permission, tracing and partial-replacement cases plus source discrepancies. Existing CHECK/DNS-C/ST-T owners and conditional triggers apply; no native pass or automatic API/dependency adoption. |
| Which Windows work can reuse Microsoft components, and what remains product-owned? | [Architecture reuse register](windows-time-architecture.md#1862-windows-component-reuse-and-binding-coverage), [development-plan handoff](plans/development-plan.md#44-apply-windows-component-reuse) and [grouped backlog](../tasks/development-backlog.md#windows-component-reuse-at-existing-stage-entry). [W11 task §4.29](../tasks/windows11-integration.md#429-windows-component-reuse-routing--5-november-2026), [storage §6.7](../tasks/windows-protected-storage.md#67-windows-binding-coverage-and-reuse--5-november-2026) and [UI handoff](../tasks/ui-m0.md#windows-component-reuse-handoff--5-november-2026) own the exact API/version map, VBS-symbol gap and conditional wrappers/toolkits. Published packages and upstream main differ; this review selects no dependency or toolkit and lifts no execution hold. |
| How are production clock-correction limits prepared? | [W11 §4.31](../tasks/windows11-integration.md#431-clock-correction-limits-preparation--5-november-2026) maps fields, provenance, proposed measurements and admission controls to S0.4/S0.7/P4 and W11-W02/04/05. Actual combined-rate accounting, late durable settlement, holdover assumptions and conditional recovery exposure refine OPEN-5/7/13; production values, decisions and native evidence remain open. Current product work follows development §7; formal work follows OD-10 §14.1's post-V1 deferral. All execution holds remain. |
| Which Windows configurations are intended, and what evidence is missing? | [OD-14](nevrotick-owner-decisions.md#18-od-14--compact-windows-platform-scope) and [W11 §4.32](../tasks/windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026): Windows 11 26H2 x64 and Server 2025 x64 Desktop Experience, two first fixtures and shared controls. Exact profiles, actual availability, Server delivery and candidate-specific results remain open under existing owners; no stable support or native pass is declared. |
| Which product capabilities and user journeys are intended? | [Feature specifications, F-01–F-10](windows-time-features.md#2-feature-specifications) and [essential journeys](windows-time-features.md#6-essential-user-journeys). |
| Is SOCKS5 part of first Windows delivery? | No. [OD-11](nevrotick-owner-decisions.md#15-od-11--socks5-scope-and-deferral) and [SOCKS-0](../ROADMAP.md#socks-0--optional-socks5-time-acquisition) defer proxy support beyond P0–P7. Direct interface/family and successful-path reuse stay in current work. The [limited future profile](windows-time-architecture.md#limited-socks5-profile--deferred) uses loopback/no-auth/local-DNS; the [existing engine task](../tasks/engine-transport-pilot.md#socks5-feasibility-and-deferral--5-november-2026) owns implementation/check preparation. |
| How is the selected complete MSIX hardened and prepared for a Store attempt with services? | [MSIX/Store plan](plans/windows-msix-store.md), [amended OD-05](nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership), [architecture §19.4](windows-time-architecture.md#194-future-complete-msix-gate), [signing guide §6.1](windows-time-signing-guide.md#61-store-msix-trust-and-publication-boundaries) and [current W11-W10 task](../tasks/windows11-integration.md#48-owner-selected-hardened-msix--3-november-2026): explicit content integrity, sealed executable paths, Store-compatible CIG, per-role protections, supported service provisioning and recovery. Local mechanics, owner submission and final Store-delivered qualification are separate stages; no working package or Store acceptance is claimed. |
| How do we maintain extensive, correct Windows product/package metadata for either MSIX or a later MSI route? | [Windows metadata contract](plans/windows-package-metadata.md): per-binary VERSIONINFO, MSIX identity/resources, MSI product/ARP/summary fields, Store listings, shared value sources and independent readback through verified windows-rs APIs. Its [service/diagnostics refinement](plans/windows-package-metadata.md#12-service-and-diagnostics-metadata-refinement--7-november-2026) owns applicability of the single [bounded field catalogue](supporting-reviews/windows-service-metadata-catalogue.md). Use [maintain-windows-metadata](../.agents/skills/implementation/maintain-windows-metadata/SKILL.md) for harvesting, field authority, TOML/JSON storage, generation and artifact comparison. OD-05 §7.6 records the owner objective; W11-W10/P6 retain implementation and artifact qualification. |
| Which stable releases are supported, and how are unsafe releases or compromised publication authority handled? | [Release policy](release-policy.md): current no-stable status, proposed first-stable support/backport baseline, exact candidate promotion, Store containment, compatible corrective releases and incident recovery. [Existing W11 task §4.21](../tasks/windows11-integration.md#421-release-operations-integration--4-november-2026) owns documentation evidence and remaining decisions/rehearsals; no support commitment or native/account pass follows from the document. |
| Can a Store release carry independently checkable evidence of its source and build? | [Enhanced build provenance](plans/release-provenance.md) records the possible attestation, reproducible-build and delivered-payload chain and its limits. [OD-05 §7.5](nevrotick-owner-decisions.md#75-optional-enhanced-build-provenance--4-november-2026) makes this additional assurance optional, with no new release blocker; existing artifact custody, SBOM/licence, integrity and installed-product checks remain required. |
| What owns the behavior, authority and state? | [Component architecture](windows-time-architecture.md#3-component-architecture-and-authority), the task-relevant technical sections and [accepted owner decisions](nevrotick-owner-decisions.md). |
| How are leap seconds, silent-LI servers and Windows event ownership handled? | [Architecture §10.5](windows-time-architecture.md#105-leap-seconds-and-time-scales) owns the selected contract; [NV-LP-001](plans/leap-second-policy.md) records primary evidence, local knowledge, source/guard/Windows behavior; [S1.4's task](../tasks/s1.4-leap-policy.md) owns implementation cases and remaining native gates. |
| What exactly admits the first pure leap-knowledge child? | [S1.4 entry and finite-input tables](../tasks/s1.4-leap-policy.md#11-entry-conditions-after-the-current-core-checkpoint) retain its prerequisites and finite bounds. The [selected pure checkpoint](../tasks/s1.4-leap-policy.md#pure-leap-final-qualification--6-november-2026) now qualifies history/coverage/rule construction, interpretation and atomic replacement, with exact receipt reconciliation. Full acquisition and native integration remain separate. |
| Which phase is ready, and what must it prove? | [Architecture §21](windows-time-architecture.md#21-delivery-phases-and-readiness-gates): P0–P7, including the bounded S0.1–S0.7 experiments. |
| What integrated slice should the task deliver? | [Feature plan §9](windows-time-features.md#9-delivery-slices-for-agentic-development): UI-M0 and S-0–S-6, with prerequisites, owners and exit evidence. |
| What must a user-visible product scenario demonstrate? | [Feature plan §8](windows-time-features.md#8-acceptance-scenarios-and-evidence): AC-01–AC-22, mapped to technical checks. |
| What technical/security result is required? | [Architecture §20.2](windows-time-architecture.md#202-acceptance-catalog): CHECK-1–CHECK-27, independent expected results, protected effects, valid controls and stage/owner. |
| What is in scope for the current UI prototype? | [Architecture §16.5](windows-time-architecture.md#165-restricted-first-ui-milestone-ui-m0): M0-01–M0-08 and D1–D3 deferrals/re-entry gates; [UI-M0 task](../tasks/ui-m0.md) for implementation ownership and evidence. |
| Which UI toolkit should the next agent use? | [OD-13](nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit) selects Slint and incorporates the accepted local migration/licence scope. The [GUI handoff](../tasks/slint-gui-handoff.md) and [S0.5](../tasks/s0.5-ui-comparison.md) own implemented fixture behavior, GPL-3.0-only original GUI, branch evidence and remaining native/backend/text-approval/hardening work. Win32 and comparison receipts retain their exact historical scope; merged-candidate checks are separate. |
| How should translators add UI languages and how are binary catalogs delivered? | [UI localization](plans/ui-localization.md), under [OD-15](nevrotick-owner-decisions.md#19-od-15--externalized-ui-translations-and-binary-catalogs): editable JSON → validated CBOR → shared typed Rust presentation catalog. All raw catalogs remain untrusted; the 6 November threat amendment covers independent contracts, isolated conversion, bounded admission and critical-meaning review/fallback. Slint, tray/notifications and accessible names share one policy; source work belongs to S0.5, actual adopted-locale/native evidence to P5. |
| Which unresolved decisions or investigations gate dependent work? | [Open decisions](windows-time-architecture.md#22-open-decisions-and-release-blockers), [owner-decision procedure](nevrotick-owner-decisions.md) and [research assignments](nevrotick-research-handoff.md). |
| How are checks run, and where are results kept? | [TOOLING.md](../TOOLING.md#starting-implementation) and [configured tool execution](../.agents/skills/testing/references/configured-tooling.md); the existing task record owns candidate-specific results and gaps. |
| How do agents prepare and manage a disposable Windows Sandbox from Windows or WSL? | [manage-windows-sandbox](../.agents/skills/environment/manage-windows-sandbox/SKILL.md): concise procedure with detailed operations, security/configuration, resource, winapp, evidence and troubleshooting references. [Windows task §4.16](../tasks/windows11-integration.md#416-windows-sandbox-management-skill--3-november-2026) records the skill delivery; current execution holds and product CHECK owners remain. |

Keep the identifiers distinct: S-0 is a delivery slice; S0.1–S0.7 are P0
experiments. UI-M0's M0 checks do not complete the AC/CHECK catalogs or P0/P5.
Select only the criteria applicable to the authorized task and retain deferred
requirements with their owners and restoration triggers.

The [UI-M0 task](../tasks/ui-m0.md#remaining-evidence-and-next-work) retains the
Win32 prototype and its incomplete full native-memory instrumentation. The
[current Slint handoff](../tasks/slint-gui-handoff.md) and S0.5 own the successor
fixture GUI and its separate portable/capture evidence. Neither old Win32 passes
nor branch-only checks establish merged-candidate or native qualification.
A task receipt records execution; a catalog specifies expectations.

## Current baseline and amendments

**Current Slint source continuation — 6 November 2026:** the
[GUI handoff](../tasks/slint-gui-handoff.md) records the implemented v0.4 fixture
GUI, strict text-contract2 producer/loaders, separate GPL-3.0-only original GUI
grant and PR10 merge `3ff7e91`. Its
[hosted reconciliation](../tasks/slint-gui-handoff.md#hosted-merge-reconciliation--6-november-2026)
records final-head/merge portable, Security gate and secret-scan passes; the PR
aggregate CodeQL/GitGuardian checks remain failed and external dispositions open.
Local source controls on the later numerical checkpoint `5dcc465` pass 509/509
text IDs and 53 selected controls; its hosted/rendered/native results are not qualified.
GUI42/43/44 retain historical scopes, with private raw evidence absent locally.
Backend/native qualification, independent critical-text approval, GUI security
coverage/critical-file mutation and P3/P5/P6 hardening remain open; retain the two
recorded polish follow-ups, D1–D3 and all execution holds.

**Controlling owner amendment — 6 November 2026:** [OD-10 §14.1](nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026)
defers all formal-proof work until after the first released software version
(V1). Proofs are not V1 implementation, phase-acceptance or release prerequisites.
Preserve the historical checkpoints below without extending their scope or
resuming their jobs, source preparation, metadata repairs or verifier work.
The [selected pure-leap checkpoint](../tasks/s1.4-leap-policy.md#pure-leap-final-qualification--6-november-2026)
is complete, including four current mutation cohorts and qualified receipt
reconciliation. [S2.1's controlled measurement source checkpoint](../tasks/engine-transport-pilot.md#first-controlled-measurement-source-checkpoint--6-november-2026)
now passes focused host tests, public-contract probes, dependency checks and
Windows cross-builds on its recorded source. The subsequent
[three-owner measurement qualification](../tasks/engine-transport-pilot.md#three-owner-measurement-qualification--6-november-2026)
completed host tests, coverage and mutation gates on its recorded source.
The later [two-owner rational-seconds/measurement checkpoint](../tasks/s1.2-regression-readiness.md#exact-rational-seconds-final-qualification--two-selected-owners)
completed on 7 November:240/243 and352/355 covered lines, with103 and99 generated
variants respectively. Five converter and three measurement misses have exact
independently reviewed equivalence dispositions, no kill credit and no unresolved
survivor; both adjusted viable scores are70/70. Engine-v1 remains incomplete
because its manifest minimum changed during execution; the valid engine-v2
receipt follows restoration of1.99.1. These results retain their exact Linux
conversion/measurement scope. The [common-run reference bridge's final record](../tasks/engine-transport-pilot.md#common-run-reference-bridge--final-qualification-and-next-action)
now closes its bounded GNU-hosted exit on the recorded `46513296...` snapshot:
134 ordinary/135 optimized tests,64 doctests and both library/relative per-file
coverage/mutation gates. Two exact library equivalences receive no kill credit;
no unresolved survivor or timeout remains. Reuse across the latest pull follows
review of unchanged compiled inputs, locked features, vendor and tools; it is no
new whole-candidate execution pass. Fresh two-package Windows-target compiler/
Clippy checks supply no native Windows evidence. The pure
[N1b conditioned-window child](../tasks/engine-transport-pilot.md#n1b-final-qualification--two-selected-core-owners)
now completes its selected two-owner Linux exit: ordinary/optimized/doctest,
two-target, validation, coverage and reviewed mutation gates. Its record retains
the interrupted ten-worker run, eight-worker fallback, exact manual equivalents
and post-run adjudication. The
[N2a final two-owner exit](../tasks/engine-transport-pilot.md#n2a-final-qualification--two-selected-core-owners)
now completes its pure Linux scope on385-input `2fc732f1...`:396 ordinary/396
optimized tests,88 doctests, both executable-line gates and304 accounted variants
(181 behavioral caught,121 compiler/lint-unviable, two reviewed equivalents).
Raw native2/0, scoped reuse of six producer controls and both regression kills
remain explicit. The
[N3a final qualification](../tasks/engine-transport-pilot.md#n3a-final-qualification--two-selected-engine-owners)
completes its two-owner controlled Linux scope on407-input `e71aab7...`:91 Engine
tests,91 optimized tests,42 doctests, external contracts and both line gates.
The357 accounted variants comprise214 fresh library outcomes and143 explicitly
reused receiver outcomes, with four reviewed equivalents, no unresolved survivor
or timeout and no equivalent kill credit. Independent cause/function review
retains the earlier failed attempts and scoped validator controls. The next
[receiving-contract card](../tasks/engine-transport-pilot.md#conditional-next-product-contract-after-n2s-finite-exit)
must settle its four ownership/transaction choices and actual suppliers before
Rust integration or an Engine-to-Core edge.
Production history insertion remains open.
The existing history receiver remains fixture-only;
physical/date/source/policy admission, NTS, native Windows and full P1/product
acceptance remain open.
Runtime validation, independent behavioral tests, coverage/mutation, security,
native/package and release gates remain; all existing execution holds remain.

**Latest Core proof checkpoint, 6 November 2026:** [actual lifecycle source-counter invariant](../tasks/s1.3-source-selection-readiness.md#lifecycle-source-counter-terminal-qualification--6-november-2026) passed five full raw2^80/three-defect/restored Kani jobs:145 properties and exact restoration. All291 selected Python checks pass;32 inputs/nine actually executed Python sources and final guards bind the candidate. Earlier ordinal/append-depth/rational proofs remain preserved. The later [complete constructor source checkpoint](../tasks/s1.3-source-selection-readiness.md#source-state-constructor-source-checkpoint--6-november-2026) prepares genuine Garde-backed source/harness/Cargo and authenticates14 selected vendor packages, with51 source checks and independent review. It has zero compiler/proof commands and is not execution-ready; source-specific admission/target/allocator/drop/unwinding and constructor proof remain open. Transactional lifecycle, prefix/depth/topology/whole-selection, fraction-model/bridge/caller/addition and Core/P1 remain open; all execution holds are unchanged.

**Independent P1 proof, 6 November 2026:** [RequestTiming constructor/getters](../tasks/s1.1-exchange-association.md#request-timing-terminal-qualification--6-november-2026) passed five ordinary Kani jobs over every raw endpoint pair, three semantic defects and restored health. Its158-check/34-input/nine-executed-Python candidate remains preserved; later shared-runner changes have separate291-check compatibility evidence, with no native rerun claimed. Actual clock callees are compiled, with no certificate imported. This arithmetic proof grants no native-clock, lifecycle or P1 authority.

The [supporter-label work](../tasks/s1.3-source-selection-readiness.md#next-independent-core-source-assignment--supporter-labels) and [constructor execution gap](../tasks/s1.3-source-selection-readiness.md#constructor-execution-gap-and-return-trigger) are retained post-V1 backlog. Neither supplies a current product priority, retry or private-patch adoption. The supporter-label healthy attempt timed out with zero checks; its new report consumer also needs a post-V1 correction. No supporter-label theorem or restored qualification was obtained.


**Core verifier checkpoint, 6 November 2026:** the [Kani contract-mode pilot](../tasks/s1.3-source-selection-readiness.md#addition-contract-mode-terminal-assessment--6-november-2026) ended DEFER at a primitive CBMC internal error, with zero product proofs or callee certificates. Completed ordinary proofs retain their scopes; full-domain addition and Core/P1 remain open.

**Mandatory mathematical proofs, 5 November 2026:**
[OD-10](nevrotick-owner-decisions.md#14-od-10--mandatory-mathematical-proofs)
requires machine-checked product mathematical proofs under
[architecture §20.1.1](windows-time-architecture.md#2011-mandatory-mathematical-proofs--5-november-2026)
and [hardening §9.6.1](windows-time-development-hardening.md#961-mandatory-product-mathematical-proof-gate--5-november-2026).
The [next-task record](plans/development-plan.md#7-next-executable-task-for-a-development-agent)
owns the current sequence and remaining domain/Core proof inventory. The existing
[S1.1c proof chain](../tasks/s1.1-time-domain.md#mandatory-proof-continuation-and-remote-integration--5-november-2026)
retains its scoped results and open public-constructor obligation.
This supersedes earlier optional-proof wording; actual-source proofs of the
remaining domain, leap, exchange and Core mathematics are still required.
Historical bounded checks do not establish the stronger gate. Tool selection,
Windows/native qualification and OD-05's optional enhanced release provenance
retain their separate status.

**Verus readiness result, 5 November 2026:** David's conditional request and the
named full-domain addition gap justified the
[bounded Core pilot](../tasks/s1.3-source-selection-readiness.md#verus-addition-pilot--5-november-2026),
routed through the [development plan](plans/development-plan.md#7-next-executable-task-for-a-development-agent)
and [P1 backlog](../tasks/development-backlog.md#p1-core-completion). The pinned
payload's isolated version/loader check passed, but the final preparation command
rejected the unchanged `u64::checked_shl` API before SMT execution. The readiness
result is **DEFER**: three preparation commands used, zero product-proof commands
and no adoption. Reconsider only after official support for the same actual call
graph and a separate finite card; no assumed project specifications or runtime
rewrite closes the gap. Completed Kani proofs remain reusable; addition and all
other mandatory open obligations and execution holds retain their scopes.

The [proof workflow](../.agents/skills/testing/references/formal-verification.md)
and shared [testing contract](../.agents/skills/testing/CONTRACT.md#product-specific-evidence)
carry this requirement through planning, Rust/contract implementation and final
qualification. Missing or invalidated proof evidence keeps dependent mathematical
acceptance incomplete; choosing a different tool does not waive the obligation.

**Ordinary S2.2 preparation, 5 November 2026:** the [engine composition card](../tasks/engine-transport-pilot.md#ordinary-full-nts-composition--s22-preparation-5-november-2026)
connects fresh normal-date KE, complete association publication, protected UDP and
the actual numerical consumer through planned paired exits. Protocol, crypto and
[W11 §4.30](../tasks/windows11-integration.md#430-ordinary-s22-windows-qualification--5-november-2026)
retain their existing semantics/trust/native owners. This is source preparation;
provider/profile admission, finite contracts and execution remain open. The current
queue stays in [development §7](plans/development-plan.md#7-next-executable-task-for-a-development-agent).

**S2.1 preparation, 5 November 2026:** the existing [engine integration card](../tasks/engine-transport-pilot.md#s21-preparation--real-transport-to-an-admitted-time-measurement--5-november-2026)
consolidates three research lanes and their analysis into the real acquisition
producer/consumer handoff. It distinguishes the existing exchange-to-Core fixture
chain, the first experimental real-UDP measurement and later justified
uncertainty/Core admission. [W11-W01 §4.26](../tasks/windows11-integration.md#426-s21-timestamp-provenance-preparation--5-november-2026)
remains partial source preparation. Development §7 controls the continuation;
no implementation, proof, native result, new gate or execution permission is
established by this addition.

**Independent MSIX removal, 5 November 2026:** the
[recovery continuation](plans/windows-msix-store.md#76-independent-removal-with-outstanding-recovery--5-november-2026)
and [W11 task §4.25](../tasks/windows11-integration.md#425-independent-removal-with-outstanding-recovery--5-november-2026)
refine the held inert test pair to begin with recovery pending at removal and
observe fresh independent activation in the already-stopped/crashed case. Native
package retention plus an independent task
remains unselected; retained bytes do not establish usable recovery. Supported
activation, authority, version handoff and retirement remain unresolved. This
source refinement preserves the separate provisioning gap, selected complete
MSIX, Core priority, scoped Sandbox/native restrictions, fuzz/storage holds,
UI-M0 deferrals and optional enhanced provenance.

**Pre-start MSIX provisioning, 5 November 2026:** the
[focused continuation](plans/windows-msix-store.md#75-pre-start-provisioning-authority-and-admission--5-november-2026)
and [W11 task §4.24](../tasks/windows11-integration.md#424-pre-start-service-provisioning-refinement--5-november-2026)
separate actual service-object rights from supported no-UI invocation/reentry.
The first future provisioning probe only inspects state and opens/closes required
handles; later controlled management start precedes fresh-process admission and
normal work, with creation-time object protection already required. The complete
MSIX remains selected and the three provisioning candidates remain unselected.
No supported production bridge or native result is established. The independent
removal pair, Core priority, scoped Sandbox disposition, blocked artifacts,
fuzz/storage holds, UI-M0 deferrals and optional enhanced provenance remain.

**Public status, 4 November 2026:** the [current implementation summary](windows-time-architecture.md#1-current-state-capsule)
now distinguishes the six-member bounded workspace from the original
documentation-only architecture review and later UI-only milestone. README and
SECURITY already included the unauthenticated loopback observer; the remaining
workspace, Core/protocol inventory and next-action wording is reconciled in the
[existing plan record](plans/development-plan.md#public-project-status-reconciliation--4-november-2026).
This changes documentation status, with no new product qualification or gate.

**Optional provenance, later on 4 November 2026:** David asked to retain the
[source-to-Store research](plans/release-provenance.md) without making its
additional proof a release blocker. This is an accepted instruction in
[OD-05 §7.5](nevrotick-owner-decisions.md#75-optional-enhanced-build-provenance--4-november-2026),
distinct from the still-proposed stable-support commitment below. Attestations,
independent bit-for-bit rebuilds and a public proof bundle remain optional;
required custody, SBOM/licence, package integrity and installed/native lifecycle
checks keep their existing owners. No reproducible release is claimed.

**Core checkpoint, 4 November 2026:** [modeled settled active-time delivery](../tasks/s1.3-source-selection-readiness.md#settled-rate-final-qualification--four-selected-owners)
completed its selected four-owner Linux fixture gates on the sealed V8 candidate:
contracts, coverage, four mutation cohorts and exact union passed. All 136 viable
variants were caught; 151 were unviable, with no survivors/timeouts/equivalences.
This is neither the full 24-owner manifest nor P1/Windows/product qualification.
The active bounded continuation is the existing leap task's pure implementation
qualification, with explicit date-scope inputs and frozen ingestion limits.
Original failures, exact candidates, scoped holds and UI-M0 deferrals remain.

**Release operations, 4 November 2026:** the [release policy](release-policy.md)
consolidates supported-version records, backport decisions, bad-release
containment, state-preserving correction and recovery of publishing authority.
Its latest-qualified-stable baseline is **proposed**, pending David's first-stable
decision; no supported stable release or fixed response/remediation deadline
exists. Store listing changes do not repair already updated devices, and
authentic same-product publication through compromised authority has no proven
independent on-device rejection gate. The [W11 continuation](../tasks/windows11-integration.md#421-release-operations-integration--4-november-2026)
keeps actual rehearsals, account readiness, native recovery and final-artifact
evidence open under existing owners. The current Core priority, scoped Sandbox
permission, blocked artifacts, fuzz/storage holds and UI-M0 deferrals remain.

**Leap policy, 4 November 2026:** [OD-09](nevrotick-owner-decisions.md#13-od-09--leap-policy-and-silent-servers)
records the requested multi-agent GitHub MCP/Windows Learn MCP research and
document integration. The selected design carries immutable reviewed history
and effective UTC rules, accepts silent LI=0 sources with admitted unsmeared
provenance, retains useful tracking when future coverage is unknown, and leaves
explicit leap execution to Windows. Bounded event guards lead to fresh-quorum
measured-residual recovery without synthetic seconds. [NV-LP-001](plans/leap-second-policy.md)
and its task distinguish this decision from absent implementation and OPEN-7
native evidence; the current Core checkpoint in the development plan, execution
holds and UI-M0 remain intact.
Affected maintained plans/skills are reconciled; archived source copies and
historical qualification receipts retain their original scope.

**Binary catalogue refinement, 4 November 2026:** the owner requested a small,
secure native Windows utility with a release catalogue for hundreds of NTP/pool,
NTS and Roughtime records, binary shipment preferred over JSON, Roughtime public
keys represented from the first schema, and Python publisher tooling under
`tools/`. Its [first offline input stage](../tools/server-catalogue/README.md) now
collects hash-bound local NTS/NTP/Roughtime snapshots and compares revalidated
candidates. It emits unapproved R evidence; approval and C generation remain
future work. The [task receipt](../tasks/server-catalogue-and-statistics.md#8-offline-publisher-collector--4-november-2026)
separates host-fixture results from product/native qualification. The [focused draft](plans/server-catalogue-binary-format.md) records
that direction and its open format, trust, compatibility and resource questions.
It selects no measured winner, provider roster, runtime dependency or new update
mechanism. The current Core priority and scoped execution holds remain.

**Roughtime coordination research, 3 November 2026:** the owner requested a
recorded design for Roughtime's cooperation with Khronos/Ananke. The
[focused plan](plans/time-assurance-integration.md) and [future task](../tasks/time-assurance-integration.md)
separate authenticated bootstrap, primary NTS, a candidate watchdog and pool
admission. They retain one broker, conditional provenance, comparison epochs,
bounded conflict behavior and OPEN-8. The roadmap's current Windows/Khronos/Android
order and later RGT-0 → CHR-0 sequence remain. The research selects no production
providers, library, extra quorum, preliminary clock write or watchdog veto;
implementation and native evidence remain future work.

**Later bootstrap-priority refinement, 3 November 2026:** the owner selected
Roughtime first for the initial date, then NTS, with Khronos/Ananke only as a
conditional reserve. NV-TA-001 revision 1.1 supersedes the earlier continuous
watchdog proposal for this profile and specifies a provisional-date reserve
followed by the same full NTS/broker gates. It distinguishes date-specific
failures from other TLS/network failures and retains signed conflicts. Its
source census found 14 candidate hostnames/about 10 nominal operator groups,
but only two explicit RFC-v1 operator advertisements plus a third documented
candidate; no live availability was measured. Current implementation/defaults
remain until the scoped extension is qualified; source minima, Windows gates
and current work/execution boundaries remain.

**Windows execution stopped, 2 November 2026:** the owner reported a Defender
detection during attempted CNG fixture staging and is keeping it blocked. All
Windows execution and retries are suspended under the
[incident record](../tasks/s0.1-crypto-feasibility.md#defender-alert--windows-execution-stopped).
The later [owner-ready Sandbox disposition and readiness result](../tasks/windows11-integration.md#419-owner-ready-sandbox-scope--3-november-2026)
permit scoped owned-Sandbox management and separately admitted guest work. The
blocked S134 stager/artifact remains blocked; this supersedes the blanket stop
only within that recorded Sandbox scope. The [next-task plan](plans/development-plan.md#7-next-executable-task-for-a-development-agent)
routes independent work to the current portable Core slice. The standalone
AES-SIV checkpoint remains evidence for its recorded candidate; the separately
held fuzz runner must not be retried through another invocation or agent.
Neither a confirmed false positive nor host compromise is established.

**Owner-selected hardened MSIX, later on 3 November 2026:** David selected one
complete MSIX and will try Store publication with its services, reconsidering
delivery if refused. The [OD-05 amendment](nevrotick-owner-decisions.md#71-accepted-choices)
and [hardening plan](plans/windows-msix-store.md) supersede the earlier MSI-first
decision. W11-W10/W11-W05 now prepare that target without Store preapproval;
MSI remains a retained alternative, not an automatic fallback or predecessor.
The [current task](../tasks/windows11-integration.md#48-owner-selected-hardened-msix--3-november-2026)
separates source/design, local candidate qualification, David's controlled Store
attempt and actual delivered-artifact verification before broad release. Explicit
package integrity, immutable executable content and per-process mitigations are
requirements, not active protections. Core priority, Windows/fuzz holds, storage
pause and UI-M0 deferrals remain; service topology and recovery must still be
supported and qualified.

**Earlier MSIX and Store research, 3 November 2026:** the
[focused assessment](plans/windows-msix-store.md) records free Store MSIX signing,
current free developer-onboarding flows, common catalog trust, native Win32
packaging, web installation and controlled distribution. It distinguishes those
benefits from restricted service/elevation approval, the selected virtual-account
broker's manifest mismatch and unresolved recovery-before-removal provisioning.
The [task refinement](../tasks/windows11-integration.md#46-msix-and-microsoft-store-refinement--3-november-2026)
reused W11-W10, W11-W05 and existing CHECK owners. At that checkpoint MSIX was
conditional and MSI remained selected; the later OD-05 decision above supersedes
that delivery choice. Source review established no account/provider approval,
installation result or independently trusted extracted executable.

**Microsoft Learn MCP follow-up, 3 November 2026:** the
[second source pass](plans/windows11-integration-follow-up.md) records callback
invalidation and ancillary-data boundaries, conditional native socket retirement,
COM initialization/teardown, service-managed state lifetime, declarative MSI ACLs,
SCM observers, ETW authorization/delivery, partial file replacement and DNS/cost
hints. It preserves contradictory Microsoft overview/ABI descriptions as explicit
SDK/binding evidence gaps. VBS claim extensions, side-channel fields, UDP batching
and VM-generation signals remain conditional reserves. The existing W11/ST-W
tasks carry the applicable cases into their phase owners; no parallel plan or
acceptance catalog is created. The integration uses the later `45ff247` baseline
and retains its portable Core continuation, broad Windows execution stop,
separate fuzz no-retry hold and earlier adoption/review receipts. Source research
and this documentation amendment establish no implementation or native result.

After the pull, the coordinator reconciles the refined W11 and ST-W cases at the
[next task checkpoint](plans/development-plan.md#7-next-executable-task-for-a-development-agent)
and records their dispositions in the existing tasks before dependent work.
The two source-only cards are not recorded as executed by this handoff.

**Microsoft Windows 11 integration research, 3 November 2026:** the
[source review](plans/windows11-integration.md) records primary Microsoft evidence,
project-specific adaptations, compatibility limits and distinguishing checks.
The [agent assignments](../tasks/windows11-integration.md) and
[development workstream](plans/development-plan.md#85-windows-11-integration-research-and-qualification)
place them within existing phases. Canonical refinements cover actual packet/
interface provenance, Modern Standby execution limits, configured versus effective
service protection, Restart Manager admission, DNS ownership and native resource
signals. WSH, packet timestamp adapters, CNG batching, PerfLib and new fault
profiles remain candidates pending their stated evidence. LPAC and full enclaves
remain conditional; PPL is not an ordinary utility-service route. This original
research checkpoint left the complete MSI selected; the later OD-05 amendment
above selects MSIX preparation. Current work, DoQ-only upstream, storage pause
and UI-M0/D1–D3 deferrals remain. Documentation integration establishes no
implemented feature, measured speedup or Windows pass.

The coordinator adopts W11-W01–W11-W10 through
[development §8.5](plans/development-plan.md#85-windows-11-integration-research-and-qualification)
and records their current dispositions in the existing Windows 11 task. This
shares the checkpoint procedure used for the imported storage queue; it does not
change the current Core priority or activate held Windows/fuzz execution.

**Protected-storage second upstream review, 3 November 2026:** the
[new source supplement](plans/windows-protected-storage-upstream-lessons-2.md)
retains the completed second search and critical review. The
[agent task](../tasks/windows-protected-storage.md) turns the findings into bounded
source/contract preparation, future portable implementation and separately held
Windows qualification. Canonical clarifications distinguish operation outcome
from owner usability, validate opened objects before destructive effects, retain
native custody through recovery, and qualify individual CNG/mitigation operations.
RedirectionGuard remains a candidate; attestation, hardware revocation epochs,
chunked snapshots and database adoption remain conditional. The original ST-T,
CHECK and DNS-C owners, construction freeze, rollback residuals, experiment pause
and UI-M0/D2 boundaries remain. No implementation or native pass is claimed.

On continuation, use the [imported hardening work queue](plans/development-plan.md#imported-hardening-work-queue)
to adopt ST-W01–ST-W07 into the existing task chain. It distinguishes the ready
source-only assignment from later implementation and held native observations.

**Protected optional storage, 2 November 2026:** the [architecture amendment](windows-time-architecture.md#1212-protected-optional-storage-and-explicit-fallback)
records the reviewed CNG VBS/TPM/fresh-state/no-cache direction requested by the
owner. The [focused plan](plans/windows-protected-storage.md) preserves the
18-scenario threat model, independent A1–A5/HR-10 corrections, actual-key evidence,
signed exact snapshots, class-specific recovery, per-role mitigation and future
D2 notification gates. It supersedes the earlier DPAPI-first optional NTS/DNS
candidate, not Windows DPAPI itself. Exact algorithms/defaults/native results
remain open; storage experiments are paused. Current fixtures, UI-M0 and phase
priority remain unchanged. The subsequent [upstream implementation review](plans/windows-protected-storage-upstream-lessons.md)
records actual-key lookup/isolation candidates, ACL-preserving file replacement,
secret serialization barriers, cancellation/completion tests and conditional
enclave patterns. Source inventory is complete; exact design freeze and native
qualification remain open.

**System-wide DNS direction, 2 November 2026:** [OD-08](nevrotick-owner-decisions.md#12-od-08--system-wide-windows-dns-and-dnssec)
records the owner's intended Windows DNS service for the whole computer and its
applications, plus DNSSEC using hardware acceleration and multiple CPU cores.
The [detailed design](plans/system-dns-cache-design.md) distinguishes local DNS
ingress from pinned DoQ upstream transport and proposes protected demand
statistics, temperature classes, a bounded refresh scheduler and measurable
adoption criteria. Its DNS-C01–DNS-C18 cases are future evidence requirements;
no service, application coverage, benchmark or native qualification is claimed.
Current Windows work and the roadmap's KHR-0 ordering remain unchanged.
The [source-attributed supplement](plans/dns-upstream-lessons.md), revision 1.2,
records eighteen reuse lessons. Its DNSSEC/cache follow-up adds exact-content
validation reuse, chain/delegation evidence, shared prerequisites, active-material
custody, individual response construction, failure caching, optional SIEVE/sharding
and time/trust transitions. The main DNS plan, revision 1.4, distinguishes
chain-material expiry from already validated answers and qualifies recursive
key-prefetch applicability. This is future guidance with concrete ownership,
resource, routing and privacy scenarios, not imported code or executed acceptance.
The additional [Security-kit assessment](plans/dns-upstream-lessons.md#security-kit-follow-up-cache-contracts-and-regression-cases)
refines demand/inspection APIs, immutable publication, retained-memory/work budgets
and private events. It retains two source-derived capacity/async counterexamples
and concrete stronger test scenarios; no reproduction, benchmark or DNS-C pass
is claimed.

**Engine workspace integration, 1 November 2026:** the bounded loopback engine is
now the sixth main workspace member. Its
[current checkpoint](../tasks/engine-transport-pilot.md#main-workspace-checkpoint)
records actual GNU portable tests/doctests/optimized cases, both frozen dependency
rows, real controlled-loopback CLI execution and Windows cross-linking. The
[native checkpoint](../tasks/engine-transport-pilot.md#native-windows-cli-checkpoint)
adds eight actual ordinary-user Windows CLI cases while retaining an earlier
observer timing failure. Four additional
[public-API cases](../tasks/engine-transport-pilot.md#native-live-process-port-release-checkpoint)
observe port reuse while their Windows children remain alive. The
[S42 coverage](../tasks/engine-transport-pilot.md#s42-current-critical-file-coverage)
and [S43 mutation checkpoint](../tasks/engine-transport-pilot.md#s43-complete-current-engine-mutation-checkpoint)
pass for the recorded S40 candidate; those receipts do not qualify later
candidates. Native completion/heap/resource and production acquisition gates
remain open. This does not complete P1/P2 or introduce clock effects.

**Pure NTS-KE preparation, 2 November 2026:** bounded request encoding and
caller-buffer incremental framing now have
[recorded qualification checkpoints](../tasks/s0.1-protocol-comparison.md#incremental-framing-final-qualification):
host checks and tests, direct contracts, per-file line/mutation gates, bounded
instrumented fuzzing and Windows-target compilation. The
[inert NTS authenticator envelope checkpoint](../tasks/s0.1-protocol-comparison.md#inert-nts-authenticator-envelope--2-november-2026)
adds complete outer framing before original-input AAD/nonce/ciphertext views,
with per-file host gates and seven independent source-fault controls. These
inert owners provide no live TLS, authentication, endpoint authority or complete
NTS operation. The
development plan retains the native crypto/trust and complete-execution gates.
The [inner plaintext framing checkpoint](../tasks/s0.1-protocol-comparison.md#inert-nts-plaintext-framing)
adds complete bounded framing and immutable original-input views, with focused,
contract, line-coverage, mutation, fuzz and Windows-target compiler evidence.
It performs no decryption or authentication.

**Verus assessment, 1 November 2026:** the
[research report](supporting-reviews/verus-assessment.md) recommends deferring
integration until a bounded actual-code comparison demonstrates additional value.
A valuable contract and a justified benefit hypothesis can trigger that pilot;
prior Kani failure is not required. The report retains candidate properties, the
Kani alternative, proof/trust boundaries and failure triage. No application proof
was executed or new tool/CI gate adopted **by that assessment**; the
[S1.1c task](../tasks/s1.1-time-domain.md#verus-proposal-assessment--1-november-2026)
owns its history. The report's dated 5 November addenda and
[current Core pilot](../tasks/s1.3-source-selection-readiness.md#verus-addition-pilot--5-november-2026)
record the triggered evaluation and actual readiness **DEFER** after the pinned
candidate rejected `checked_shl`. No SMT or product proof ran; broader adoption
remains deferred.

**State-guided fuzzing experiment, 1 November 2026:** the
[completed experiment report](supporting-reviews/state-guided-fuzzing-experiment.md)
records four private defect variants, strict/effects oracles and a bounded
mutation comparison. Lifecycle mutation helped cancellation search but slowed
context-invalidation search; retain strict/baseline defaults and keep the new
operator optional. The experiment is closed; the existing product plan continues.
Detailed execution evidence remains in the [S1.1e task](../tasks/s1.1-deterministic-harness.md#bounded-fault-suite-and-mutation-follow-up-1-november-2026).

**Architecture contract assessment, 30 October 2026:** the [critical review](supporting-reviews/architecture-contract-review.md) distinguishes valid specification gaps from existing controls and stale external snapshots. [Canonical acceptance refinements](windows-time-architecture.md#2021-contract-review-acceptance-refinements), [hardening §9.10](windows-time-development-hardening.md#910-contract-review-qualification-matrix) and [the execution workstream](plans/development-plan.md#82-contract-review-workstream-and-interface-gates) cover ownership, durable correction, adjustment modes, trust lifetimes, fresh agreement and lifecycle/tooling evidence. Unresolved numerical and trust-policy decisions remain held; no native qualification is claimed.

**Current UI toolkit decision, 5 November 2026:**
[OD-13](nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit)
selects **Slint as the default**, starting with the measured winit/software
renderer and Fluent-style widgets. The
[Slint comparison result](../tasks/s0.5-ui-comparison.md#slint-software-renderer-live-host-result--5-november-2026)
records 7.730 MiB visible private memory and no detected CPU-time increment in
short idle trials; it is not full qualification. The integrated [Slint GUI handoff](../tasks/slint-gui-handoff.md)
records later fixture implementation, bounded graph admission and the original
GUI GPL-3.0-only grant; backend/native and critical-text approval remain open.
The earlier [Win32/egui report](supporting-reviews/ui-toolkit-comparison.md)
preserves the 30 October decision and measurements, superseded as a default by
OD-13. The checked-in Windows GUI now selects Slint; Win32 receipts and the resource targets remain historical reference.

David reaffirmed Slint after the subsequent iced comparison. The consolidated
[Slint/iced assessment](supporting-reviews/slint-iced-assessment-2026-11-05.md)
retains the reported resource results, pinned source review, unresolved findings,
dated community/development census and decision rationale. It records no clear
runtime-resource winner and routes its remaining source findings through the existing UI-M0/S0.5 chain.

**Protocol follow-up, 30 October 2026:** [NTP client behavior](windows-time-architecture.md#73-ntp-client-behavior) makes KoD support explicit through pinned Chrony/independent-codebase analysis and reuse. The [existing protocol task](../tasks/s0.1-protocol-comparison.md#kod-reuse-and-rfc-8915-alignment-follow-up) records RFC 8915 alignment and remaining implementation checks; [TLS policy](windows-time-architecture.md#1871-license-gate-and-finite-provider-choice) prioritizes AES-GCM while retaining provider/CPU qualification. These are P2 requirements, not implemented NTS/KoD support or measured AES speedup.

**Upstream code is not presumed bug-free or vulnerability-free.** Chrony and other reused projects, libraries and tools require independent verification of the behavior NevroTick relies on. See [architecture section 20.1](windows-time-architecture.md#201-independent-numerical-truth) and [the test and stress matrix](windows-time-development-hardening.md#97-verification-of-upstream-code-and-stress-testing).

**Upstream testing tools, 30 October 2026:** the [clknetsim/chrony-fuzz reference](../.agents/skills/testing/references/chrony-upstream-tools.md)
adds pinned scenario/seed reuse to the existing plan and skills. clknetsim supplies
reference trajectories through an explicit seam; chrony-fuzz supplies testcase
ideas from an AFL++ harness that deliberately simplifies protocol/crypto checks.
Neither is a native Windows harness or an independent authentication oracle.
The [protocol task follow-up](../tasks/s0.1-protocol-comparison.md#upstream-simulation-and-fuzz-tool-follow-up)
records source-only inspection and the next bounded qualification work.

**Chrony/NTPsec mechanism assessment, 30 October 2026:** the
[critical source assessment](../tasks/s0.1-protocol-comparison.md#chrony-and-ntpsec-mechanism-assessment)
supports targeted refinements for estimate lifecycle, bounded scheduling, route
relearning, atomic NTS publication/send enforcement and restoration validity.
The [existing workstream map](plans/development-plan.md#41-apply-the-chrony-and-ntpsec-mechanism-assessment)
keeps current S1.3 qualification before dependent controller integration. Chrony
remains the numerical reference; NTPsec supplements protocol comparison alongside
ntpd-rs. No upstream constants, additional controller or implementation pass follows
from this review; the relevant implementation/security/testing skills carry the checks.

**Development baseline, 30 October 2026:** The [Cargo workspace](../Cargo.toml) contains the fixture-only `nevrotick-ui`, dependency-free `ntp-wire`, checked numerical `time-domain`, single-slot `ntp-exchange` and pure fixture-regression `time-core` owners. [The tooling record](../TOOLING.md#starting-implementation) describes the verification entry points; [UI-M0](../tasks/ui-m0.md), [S1.1a fixed headers](../tasks/s1.1-header.md), [S1.1b framing](../tasks/s1.1-framing.md), [S1.1c arithmetic](../tasks/s1.1-time-domain.md), [S1.1d exchange association](../tasks/s1.1-exchange-association.md) and [S1.2 regression](../tasks/s1.2-regression-readiness.md) own candidate-specific results and gaps. The five-member graph retains the sixteen external package records from the UI-M0 cohort; it admits no production networking or TLS. The [planned runtime dependency refresh](windows-time-architecture.md#186-runtime-dependency-register) still leaves held AWS-LC/Schannel providers and the production graph unresolved.

**Native memory diagnostics, 29 October 2026:** Application Verifier, GFlags/PageHeap and CDB are installed and configured; healthy, heap-overrun and invalid-handle bootstrap controls passed. [Development hardening section 9.5](windows-time-development-hardening.md#95-fuzzing-and-native-memory-diagnostics) defines when agents must select corruption checks and separate retention measurements; [the shared skill workflow](../.agents/skills/testing/references/windows-memory.md) provides execution and cleanup rules. These are tool controls, not application leak/soak or supported-product qualification. Native runs remain owned manual checks until a separate CI lane is qualified.

**Performance guidance:** [Architecture section 17](windows-time-architecture.md#17-resource-and-performance-targets) owns the proposed budgets. Use [performance optimization](../.agents/skills/implementation/optimize-rust-performance/SKILL.md) to locate and improve a measured bottleneck, paired with [resource QA](../.agents/skills/testing/qa-performance-resources/SKILL.md) for comparable measurements and acceptance. This workflow preserves correctness/security and does not claim measured product performance before implementation.

**Application build optimization:** Release settings now explicitly select optimization level 3, ThinLTO and one codegen unit while retaining correctness checks, symbols and Windows protections. [Hardening section 6.5](windows-time-development-hardening.md#65-windows-x64-optimization-policy-and-evidence-limits) records the compiler-selfbuild Windows issue, CPU compatibility, PGO and BOLT limits; [the task record](../tasks/build-optimization.md) owns actual checks. These optimization settings alone establish no measured speedup or native CPU compatibility.

The [LLVM settings matrix](windows-time-development-hardening.md#66-llvm-settings-for-this-workload) selects the bundled backend's optimized pipeline, ordinary floating-point behavior and rustc-managed Windows linker defaults. Its experiment controls preserve exact numerical outputs, latency, resource and power observations; raw LLVM option recipes are not adopted defaults.

**Required CPU optimization, 30 October 2026; baseline configured 6 November:** [OD-07](nevrotick-owner-decisions.md#11-od-07--x64-baseline-and-runtime-optimization) requires an `x86-64-v3` minimum. Following David's clarification, the Windows target flags and exact checker now select v3. The [research report](supporting-reviews/x86-runtime-optimization.md), [hardening §6.7](windows-time-development-hardening.md#67-v3-baseline-and-runtime-cpu-dispatch) and [required execution workstream](plans/development-plan.md#81-x64-baseline-and-runtime-optimization-workstream) retain lower-baseline-compatible launch admission, native CPU/OS/VM qualification and measured runtime-selected specializations as remaining work. Historical generic-x64 receipts keep their original scope. V3 already includes AVX2; optional AVX-512 and independent AES/provider capabilities still need exact feature/OS checks, numerical parity and useful-work evidence. Configuring the required floor does not establish a native pass or measured speedup.

**Contract adoption, 29 October 2026:** [DEC-18 and section 6](windows-time-architecture.md#61-validation-architecture-and-enforcement-order) select garde for mandatory shared DTO/settings validation inside private fallible construction. [Every function/module contract](windows-time-architecture.md#631-contracts-for-every-function-and-module) must establish preconditions, checked outputs, failure/effect behavior and current-state obligations through types and the relevant checks. The [agent enforcement rules](windows-time-development-hardening.md#34-mandatory-contracts-for-agent-written-code) preserve one rule owner and prohibit construction bypasses. The UI-M0 model now implements checked preview construction using the admitted garde graph; its behavioral evidence remains distinct from the original design adoption and from future production contracts.

**Windows mitigation refinement, 29 October 2026:** [architecture section 15.4](windows-time-architecture.md#154-per-process-mitigation-profiles-and-startup-admission) defines process roles, activation limits, startup failure handling and the mitigation manifest; [hardening section 6.4](windows-time-development-hardening.md#64-per-process-mitigation-qualification) supplies the native qualification matrix. Existing CFG/ASLR/DEP configuration and bootstrap evidence are distinguished from pending application enforcement. Read these before adding process policies or treating every mitigation as a runtime switch.

**Diagnostics and shared Windows adapters, 29 October 2026:** [architecture section 3.4](windows-time-architecture.md#34-shared-windows-adapters-and-diagnostics) distinguishes reusable OS adapters, portable IPC contracts and per-process composition. [Section 12.3](windows-time-architecture.md#123-logging-and-support) owns logger structure, INFO defaults, authorized bounded DEBUG sessions, required events, redaction and resource/failure rules; [hardening section 9.8](windows-time-development-hardening.md#98-diagnostics-and-shared-adapter-verification) supplies the acceptance matrix. Those amendments remain component contracts; the fixture-only UI-M0 implementation does not implement the production diagnostics or shared service/IPC adapters.

**IPC external comparison, 29 October 2026:** [architecture section 5.4](windows-time-architecture.md#54-native-ipc-implementation-and-qualification) records refinements from Microsoft guidance and pinned Tokio/Mio, Kestrel and OpenVPN source: caller-token association/reversion, effective rights, continuous pipe ownership and cancellation/completion. [Hardening section 9.9](windows-time-development-hardening.md#99-ipc-identity-framing-and-native-lifetime-verification) maps them to existing native acceptance checks. The transport, runtime, dependency and UI-M0 decisions remain; external comparison is not implementation evidence.

**Current UI milestone:** [UI-M0, architecture §16.5](windows-time-architecture.md#165-restricted-first-ui-milestone-ui-m0), is a restricted native fixture-only prototype. The checked-in fixture GUI uses Slint under OD-13; current source/evidence and remaining backend/native/text-approval work are in the GUI handoff and S0.5. Win32 remains a historical reference. D1 display/theme, D2 production service/policy-state and D3 full keyboard/Narrator/UIA work are explicitly **Deferred**, not prerequisites for UI-M0. Restore them at their specified integration/preview gates. Other workstreams and eventual product/release requirements remain unchanged; completing UI-M0 does not complete P0 toolkit qualification.

**Latest owner instructions:** [nevrotick-owner-decisions.md](nevrotick-owner-decisions.md) records accepted reuse/recovery/release choices and unresolved default-profile policy. [nevrotick-research-handoff.md](nevrotick-research-handoff.md) assigns the remaining investigations, including the supplied NTS directory and proposed operator candidates. Read these before interpreting older unconditional signing language in retained historical reviews.

**Engine runtime clarification, 29 October 2026:** [architecture section 18.6.1](windows-time-architecture.md#1861-engine-runtime-decision-and-alternatives) records the rationale, component boundaries and reconsideration criteria for the existing minimal current-thread Tokio prototype. Section 18.7.1 keeps a bounded synchronous NTS-KE worker available alongside the held async transport candidate. Runtime choice establishes no measured resource, security or timing result; existing dependency and Windows qualification gates apply.

**Deterministic fixture work, 30 October 2026:** [S1.1e](../tasks/s1.1-deterministic-harness.md) records its qualified private driver/oracle candidate; its reviewed test delta is in the shared checkout. [S1.2](../tasks/s1.2-regression-readiness.md) records the qualified private coefficient, residual, normalization and atomic-history candidates, their independent expectations and actual test, per-file analysis, fuzz and cross-build evidence. The reviewed core and current build-policy merge are now in the shared checkout. The exact integrated staging candidate passed 191 ordinary tests, 53 doctests and 164 optimized tests, Linux and Windows dependency checks, and a Windows cross-build. Native Windows behavior, production integration and full S1.2/P1 remain incomplete.

## Reading guide

| Order | Document | Scope and status |
| --- | --- | --- |
| A | [nevrotick-owner-decisions.md](nevrotick-owner-decisions.md) | Owner instructions of 29 October; accepted choices, recommendations and unresolved policy distinguished. |
| B | [nevrotick-research-handoff.md](nevrotick-research-handoff.md) | Bounded future research, NTS operator candidates, evidence requirements and owner decision cards. |
| C | [release-policy.md](release-policy.md) | Current support status and proposed stable-maintenance model; exact-candidate promotion, withdrawal, corrective recovery and publishing-authority incidents. Required operational rehearsals and product qualification remain open. |
| 1 | [windows-time-features.md](windows-time-features.md) | V1 product feature plan: ten capabilities, main journeys, authority, recovery, AC-01–AC-22 acceptance scenarios and staged delivery. |
| 2 | [windows-time-architecture.md](windows-time-architecture.md) | Canonical creation specification, revision 1.4: technical contracts, threat model, NTP/NTS, Windows authority and release gates. |
| 3 | [windows-time-diagrams.md](windows-time-diagrams.md) | Editable Mermaid architecture views aligned with revision 1.4. |
| 4 | [windows-time-development-hardening.md](windows-time-development-hardening.md) | Strict Rust/Windows agentic development policy, CI, auditors, tool qualification and evidence requirements. See the current tooling record for adopted bootstrap checks; application/native/release qualification remains pending. |
| 5 | [asvs-5.0.0-applicability.md](asvs-5.0.0-applicability.md) | All 345 ASVS 5.0.0 requirements mapped to proposed applicability/verification; not completed compliance. |
| 6 | [windows-time-signing-guide.md](windows-time-signing-guide.md) | Selected Store MSIX trust/workflow and retained MSI signing alternatives; final artifact verification and provider acceptance remain open. |
| 7 | [windows-time-update-assessment.md](windows-time-update-assessment.md) | TUF/update research and constraints; no automatic updater or TUF architecture has been adopted. |
| 8 | [windows-time-review.md](windows-time-review.md) | Independent architecture review and revision 1.4 reconciliation. Its earlier reviewed-baseline label is intentional. |

Accepted owner decisions govern their stated scope; recommendations and open questions remain proposals. The architecture controls the corresponding technical/security contracts. The feature plan supplies proposed presentation and product scope; unresolved provider, numerical, Windows, licensing and signing decisions retain their gates. The selected target is one complete hardened MSIX for David's Store attempt with services. Retained MSI Authenticode/OpenPGP alternatives have separate evidence and Windows compatibility claims; none of these routes is qualified by documentation. Historical MSI-first reviews and receipts describe their original candidates, not the amended OD-05 direction.

## UI design and brand references

David selected design v0.4 as the current HTML reference on 5 November 2026.
Use it for screen composition and interaction flows, and the AVIF kit for brand
artwork and variants. Design v0.2 remains historical with its original scene IDs.

| Reference | Start here |
| --- | --- |
| Design guidance | [Current design v0.4](nevrotick-design-v0.4/README.md), [usage and scenes](nevrotick-design-v0.4/README-RU.md), [tray/exit update](nevrotick-design-v0.4/review/tray/REPORT-RU.md), [status/notification audit](nevrotick-design-v0.4/review/status-notifications/REPORT-RU.md). |
| Visual and interaction examples | [Interactive HTML preview](nevrotick-design-v0.4/NevroTick-UI-v0.4.html), [editable sources](nevrotick-design-v0.4/src/), [tests](nevrotick-design-v0.4/tests/). |
| Historical design | [Design v0.2](nevrotick-design-v0.2/README.md), [retained UI specification](nevrotick-design-v0.2/docs/02-ui-spec.md) and [old screen/scene catalogue](nevrotick-design-v0.2/screens/README.md). |
| Brand assets | [AVIF kit README](NevroTick-AVIF-kit-v1/README.md) and [asset manifest](NevroTick-AVIF-kit-v1/manifest.json), including transparent, light/dark and size variants. |
| Asset rights and provenance | [Brand notice](NevroTick-AVIF-kit-v1/BRAND-LICENSE.txt) and the kit's per-image metadata; keep these separate from the application source-code licence. |

Apply the references within the current UI-M0 scope and D1–D3 deferrals. HTML
validators, dependencies and mock outcomes do not define the Rust/Slint
implementation selected under OD-13. Imported Win32 descriptions reflect the
pre-migration baseline. Simulated service, ownership and notification flows in v0.4 do
not implement or qualify native behavior. The live canonical architecture and accepted owner
decisions retain precedence over archived copies in the design bundle.

## Supporting reviews and comparison

These preserve first-pass findings and correction checks, not competing specifications or claims of runtime qualification.

- [tuf-protocol-review.md](supporting-reviews/tuf-protocol-review.md): protocol, time assurance and recovery.
- [tuf-dependency-review.md](supporting-reviews/tuf-dependency-review.md): Rust clients, crypto graphs and evidence limits.
- [tuf-windows-review.md](supporting-reviews/tuf-windows-review.md): authority, staging, MSI handoff and lifecycle.
- [signing-guide-review.md](supporting-reviews/signing-guide-review.md): provider onboarding and signing-guide corrections.
- [development-compiler-review.md](supporting-reviews/development-compiler-review.md): compiler/lint/profile and Windows mitigation review.
- [development-evidence-review.md](supporting-reviews/development-evidence-review.md): CI trust, isolation, mutation contract and correction verification.
- [basic-features-review.md](supporting-reviews/basic-features-review.md): integrated product feature, UX, authority and acceptance review.
- [ui-toolkit-comparison.md](supporting-reviews/ui-toolkit-comparison.md): historical Win32/egui measurements and decision, now superseded by OD-13's Slint default; subsequent GPUI, Reactor and Slint evidence stays in the S0.5 task.
- [slint-iced-assessment-2026-11-05.md](supporting-reviews/slint-iced-assessment-2026-11-05.md): consolidated resource, source-quality, accessibility, verification and community/development assessment; David's reaffirmation of Slint, exact source pins, evidence limits and existing migration follow-through.
- [x86-runtime-optimization.md](supporting-reviews/x86-runtime-optimization.md): v3 baseline and ahead-of-time runtime dispatch, precise feature/OS gates, numerical constraints, compatibility and benchmark qualification; implementation follows OD-07 and the existing build task.
- [architecture-contract-review.md](supporting-reviews/architecture-contract-review.md): critical disposition of the external assessment, primary-source basis, contract amendments and remaining decision/native-evidence gates.
- [state-guided-fuzzing-experiment.md](supporting-reviews/state-guided-fuzzing-experiment.md): completed four-defect mutation/oracle comparison, retained defaults, per-class results, reusable lessons and frozen evidence references.
- [verus-assessment.md](supporting-reviews/verus-assessment.md): historical source assessment and dated bounded Core pilot, retained Kani evidence, same-contract comparison, proof/trust boundaries and conditional adoption criteria.
- [hifitime-lessons.md](supporting-reviews/hifitime-lessons.md): critically checked proof-specification, harness-reachability, conversion-anchor and leap-data lessons; current proof-status corrections and bounded adoption through existing domain/leap owners, without a new numerical dependency.

Some records retain workspace paths, initial hashes and source identifiers for provenance. Read correction addenda alongside the first-pass findings. Main documents control current requirements; the tooling record identifies completed tool controls separately from pending application, native-product and signing qualification.

## Format and integrity

All documents are UTF-8 Markdown. Mermaid remains editable fenced source. Keep the extracted directory together for relative document links.

An exported bundle may supply `SHA256SUMS.txt` for byte-integrity comparison; this checkout currently has no root documentation manifest, so it establishes no bundle-integrity result. The design subdirectories maintain their own separate manifests. Checksums do not authenticate a publisher or replace a digital signature.

Older PDF/CSV exports, intermediate candidates, helper scripts, credentials and executable tools are excluded. No application build, Windows installation, clock change, scanner execution, signing request or conformance test was performed to create the feature-plan addition.


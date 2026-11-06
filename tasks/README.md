# NevroTick task map

Read [AGENTS.md](../AGENTS.md), the [documentation guide](../docs/README.md), then
the [development plan](../docs/plans/development-plan.md). Architecture §21 owns
P0–P7 and stage meanings; the plan owns ordering/readiness; linked tasks own
bounded work and candidate-specific evidence. This index does not certify exits.

**Current continuation:** use [plan §7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
and the engine's [next source checkpoint](engine-transport-pilot.md#next-source-checkpoint-observation-to-core-admission).
The [pure-leap qualification](s1.4-leap-policy.md#pure-leap-final-qualification--6-november-2026)
is its completed predecessor. Historical
checkpoint numbers and passes never replace the current candidate or next action.
**Deferred** below means a recorded parent, not ready for implementation; expand
only its next activated child under the [grouped backlog](development-backlog.md).
Role owners identify engineering responsibility, not assigned people.

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

For formal coverage and resumption, use the consolidated
[product-guarantee debt and completion criteria](development-backlog.md#product-guarantees-still-open)
under [OD-10 §14.2](../docs/nevrotick-owner-decisions.md#142-proof-purpose-and-completion--6-november-2026).
Keep detailed receipts with the existing source owners below; a local helper or
tooling pass does not close its parent algorithm.

**Current Slint continuation — 6 November 2026:** the [GUI handoff](slint-gui-handoff.md)
and [S0.5 active goal](s0.5-ui-comparison.md#active-goal--finish-the-complete-slint-gui)
own the implemented fixture GUI, GPL-3.0-only original GUI grant, bounded graph
admission and GUI42/43 evidence. The checked-in Windows entrypoint selects Slint;
Win32 is retained historical source. Backend/native qualification, independent
critical-text approval and the planned hardening handoff remain open. Earlier
M0-SL preparation states below are historical and do not require repeating their
completed source stages. Main's Slint/iced measurements and reaffirmation remain
retained, with no toolkit-choice rerun. One owner serializes shared graph/schema
changes; branch receipts do not qualify the merged candidate. Live P5 and D1–D3
retain their existing prerequisites and re-entry gates.

## Phase and stage ownership

| Phase / stages | Task and role owner | Readiness, prerequisites and observable exit |
| --- | --- | --- |
| P0 S0.1 | [Source readiness](s0.1-source-readiness.md), [protocol comparison](s0.1-protocol-comparison.md), [crypto feasibility](s0.1-crypto-feasibility.md); protocol/dependency/security | Existing bounded source/kernel/provider records. Exact graph/provenance and held native/provider choices remain separate; CHECK-20/26 and S0.1 exit remain scoped. |
| P0 S0.2 | [Platform parent](development-backlog.md#p0-platform-and-policy); platform/security | Deferred service-token/lifecycle experiment; intended accounts, admitted adapter and disposable fixture precede actual allowed/denied CHECK-1/27 observations. |
| P0 S0.3 | [Platform parent](development-backlog.md#p0-platform-and-policy); IPC/security | Deferred mutual-identity experiment after token/endpoint contracts; real permitted/denied status/protected calls and fake endpoints establish CHECK-2 subset. |
| P0 S0.4 | [Platform parent](development-backlog.md#p0-platform-and-policy), [W11-W02](windows11-integration.md#w11-w02-reconcile-sleep-timers-and-clock-presentation); clock/platform | Deferred measured backend experiment; bounded owned guest/reference/recovery precede sign/rate/step/power/cancel CHECK-10/11/12 observations. |
| P0 S0.5 | [Slint GUI continuation](s0.5-ui-comparison.md#active-goal--finish-the-complete-slint-gui), [handoff](slint-gui-handoff.md); UI/accessibility + dependency/licence | OD-13 selects Slint. The v0.4 fixture GUI, original GUI GPL3 grant and branch portable/capture/source-control evidence are delivered; merged-candidate, backend/native, critical-text approval and hardening work remain open. Full CHECK-22 feasibility retains D1/D3 triggers; historical Win32/toolkit results do not transfer. |
| P0 S0.6 | [W11-W05](windows11-integration.md#w11-w05-observe-effective-scm-maintenance-and-process-policy), [W11-W10](windows11-integration.md#w11-w10-re-enter-conditional-mechanisms-only-for-a-concrete-need); packaging/platform/security | Existing owner for selected complete MSIX laboratory mechanics; provisioning/recovery and admitted guest gates precede CHECK-5/13/23 subset. No duplicate package task. |
| P0 S0.7 | [Platform parent](development-backlog.md#p0-platform-and-policy); architecture/policy/security | Deferred finite decision freeze after relevant evidence; explicit decisions or dependent blockers, including native-work and supported-environment limits. |
| P1 S1.1a–b | [Header](s1.1-header.md), [framing](s1.1-framing.md); protocol | Existing inert `ntp-wire` tasks; candidate-specific framing/admission evidence, not acquisition/authentication/sample authority. |
| P1 S1.1c–e | [Time domain](s1.1-time-domain.md), [exchange](s1.1-exchange-association.md), [deterministic harness](s1.1-deterministic-harness.md); domain/protocol/QA | Existing checked numerical and fixture-association children. [RequestTiming proof](s1.1-exchange-association.md#request-timing-terminal-qualification--6-november-2026) passed five full-domain/defect/restored jobs and158 selected Python checks, with34 inputs/nine executed Python sources bound. Constructor arithmetic only; lifecycle/native/P1 gates remain open. |
| P1 S1.2 | [Regression/history](s1.2-regression-readiness.md); numerical Core | Existing task; retain selected-owner receipts, remaining scope and independent trajectory/critical-file gates. |
| P1 S1.3 | [Selection/principal/combination](s1.3-source-selection-readiness.md); numerical Core | [Actual source-counter invariant](s1.3-source-selection-readiness.md#lifecycle-source-counter-terminal-qualification--6-november-2026) passed five full raw2^80/three-defect/restored jobs,145 properties and291 Python checks;32 inputs/nine executed Python sources bound. Earlier ordinal/append-depth/rational evidence remains preserved. [Complete constructor source preparation](s1.3-source-selection-readiness.md#source-state-constructor-source-checkpoint--6-november-2026) now has six genuine Garde-backed Cargo projects,14 authenticated vendor packages,51 source checks and independent review; zero compiler/proof commands, execution unready. Source-specific admission/target/allocator/drop/unwinding/constructor proof, transactional lifecycle, prefix/depth/topology/whole-selection, held fraction models/bridge/import/addition, Garde admission, `EndpointParts`, complete enclosure and Core/P1 remain open. All holds and the full P0–P7 objective remain. |
| P1 S1.4 | [Core completion parent](development-backlog.md#p1-core-completion), [leap entry table](s1.4-leap-policy.md#11-entry-conditions-after-the-current-core-checkpoint); Core/domain | Abstract discipline/holdover parent remains deferred. The separate pure-leap owner has completed its recorded final host qualification; preserve current receipts and continue with the observation-to-Core contract under plan §7. Formal obligations are post-V1. Later protocol/native scopes remain open. |
| P1 S1.5 | [Core completion / verifier maintenance](development-backlog.md#p1-core-completion); tooling/dependency/QA | Cross-cutting verification and eventual phase closure; existing actual gates remain mandatory. Deferred maintenance is not a new implementation prerequisite. |
| P2 S2.1–S2.4 | [Acquisition parent](development-backlog.md#p2-acquisition); engine/protocol/security | Deferred real UDP/DNS → NTS → qualified reuse/recovery → adversarial closure. The [ordinary S2.2 card](engine-transport-pilot.md#ordinary-full-nts-composition--s22-preparation-5-november-2026) prepares fresh KE/protected UDP/actual Core composition; graph/trust/contracts and implementation evidence remain open. Reuse the engine, protocol and crypto owners without inheriting production qualification. |
| P3 S3.1–S3.4 | [Service-boundary parent](development-backlog.md#p3-service-boundary); platform/IPC/security/Core | Deferred final services → authenticated handlers → mandatory state → simulated-backend integration. W11/ST-W assignments refine these owners; optional storage retains its separate pause. |
| P4 S4.1–S4.5 | [Clock parent](development-backlog.md#p4-clock-integration); clock/platform/security/QA | Deferred measured backend/accounting → finite policy → power generations → ownership/recovery → soak; relevant P0/P3 evidence and bounded native admission precede effects. |
| P5 S5.1–S5.4 | [Production-UI parent](development-backlog.md#p5-production-ui); UI/accessibility/platform | Deferred Slint status → settings/helper → diagnostics/tray → accessibility/resources, consuming the admitted adapter and its exact feature graph. Restore D2 before live status/commands and D1/D3 at their preview/qualification triggers. |
| P6 S6.1–S6.4 | [Existing Windows package owner](windows11-integration.md); W11-W10 with W11-W05/release/security | Selected complete hardened MSIX remains in this task: mitigations, lifecycle, one-product behavior, then submission/delivered provenance. CHECK-20/SBOM covers the full payload, including non-Cargo files; local and Store-delivered exits differ. |
| P7 | [Final-qualification parent](development-backlog.md#p7-final-qualification); independent QA/release, David approves release | Deferred exact integrated candidate after applicable phase evidence; reconcile CHECK-1–27 and applicable AC-01–22, installed bytes, unresolved blockers and actual release-profile trust. AC-21/22 preserve the canonical OD-11 direct-path/proxy deferral split. |

The [clock-limit preparation card](windows11-integration.md#431-clock-correction-limits-preparation--5-november-2026)
supplies S0.4/S0.7/P4 and W11-W02/04/05 with unresolved fields, evidence provenance,
paired controls and ordered handoff. It selects no production values, preserves
current Core priority and lifts no native, fuzz or storage hold.

The [two-family platform matrix](windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026)
records OD-14's Windows 11 26H2/Server 2025 direction at the same S0.4/S0.7,
W11-W02/04/05/10 and P4/P6/P7 owners. Exact profiles, delivery and actual
qualification remain open; no new task chain or native authorization follows.

Every exit also uses the [phase gates](../docs/plans/development-plan.md#6-phase-implementation-and-acceptance-gates),
[CHECK catalog](../docs/windows-time-architecture.md#202-acceptance-catalog) and
[AC catalog](../docs/windows-time-features.md#8-acceptance-scenarios-and-evidence).
The [S-0–S-6 map](development-backlog.md#integrated-slices) combines these same
parents; it creates no second execution sequence.

## Cross-cutting records and evidence

- [Windows integration](windows11-integration.md) and [protected optional storage](windows-protected-storage.md) own W11-W01–W11-W10/ST-W01–ST-W07 dispositions. Apply their mapped contracts before dependent adapters; source-ready does not authorize a native probe.
- [Build/CPU optimization](build-optimization.md), [catalogue/statistics](server-catalogue-and-statistics.md) and [future time assurance](time-assurance-integration.md) keep their existing owners, prerequisites and evidence chains. They add no blanket prerequisite to current Core work.
- `*-equivalence*.md` / `*-equivalences.md`, cohort plans, inventory-admission JSON and the Chrony ledger support their owning tasks. They are evidence/review/accounting records, not independent implementation assignments or proof that a phase passed.

## Retained research dispositions

The [Windows component reuse handoff](../docs/plans/development-plan.md#44-apply-windows-component-reuse)
is routed through the [existing-stage backlog](development-backlog.md#windows-component-reuse-at-existing-stage-entry),
[W11 §4.29](windows11-integration.md#429-windows-component-reuse-routing--5-november-2026),
[storage §6.7](windows-protected-storage.md#67-windows-binding-coverage-and-reuse--5-november-2026)
and [UI handoff](ui-m0.md#windows-component-reuse-handoff--5-november-2026).
It records generated API reuse, the VBS binding gap and conditional registry/UI
candidates. Publication/source coverage does not admit a dependency, activate a
native experiment or reopen the closed egui comparison; current holds and Core
priority remain with plan §7.

| Source / record | Disposition and re-entry owner |
| --- | --- |
| [Win32/egui comparison](../docs/supporting-reviews/ui-toolkit-comparison.md), [task](s0.5-ui-comparison.md) | Historical egui comparison closed; the later Slint pilot led to accepted [OD-13](../docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit), superseding retain-Win32 direction. Keep resource limits and candidate-specific receipts; use the current S0.5/GUI handoff for remaining merged-candidate, native/backend and package qualification. |
| [State-guided fuzzing experiment](../docs/supporting-reviews/state-guided-fuzzing-experiment.md), [harness task](s1.1-deterministic-harness.md) | Closed; reuse its strict oracle, corpus and targeted lifecycle lessons. Reopen only for a concrete uncovered transition, changed input model or demonstrated search gap. |
| [Verus assessment](../docs/supporting-reviews/verus-assessment.md), [time-domain task](s1.1-time-domain.md#verus-proposal-assessment--1-november-2026) | The 1 November assessment deferred integration and scheduled no pilot. **5 November:** the addition timeout justified the [bounded Core pilot](s1.3-source-selection-readiness.md#verus-addition-pilot--5-november-2026); its three readiness commands are now complete and the candidate is DEFER because required `u64::checked_shl` is unsupported. Zero product-proof commands ran. Return only after official support for the unchanged API and a newly reviewed finite checkpoint; broader adoption remains deferred. No global CI gate or phase blocker is added. |
| [Update assessment](../docs/windows-time-update-assessment.md) and retained TUF reviews | TUF and a bespoke privileged updater remain unadopted. W11-W10 owns the selected MSIX delivery; a materially different update design requires its existing decision process. |
| [Enhanced build provenance](../docs/plans/release-provenance.md) | Optional under OD-05 §7.5: attestations, independent rebuilds and public source-to-Store evidence add no release blocker or P6/P7 prerequisite. W11-W10 retains the existing required custody, SBOM/licence, trust and installed-payload checks. |

[Future roadmap parents](development-backlog.md#future-roadmap-parents) now retain
DNS, Android, Chrony-replacement, deferred SOCKS5 and the post-Khronos GeoIP
experiment ownership explicitly. The [existing catalogue task](server-catalogue-and-statistics.md#deferred-geoip-experiment--after-khronos)
owns GEO-C01 evaluation/disposition before Android; GeoIP adoption remains optional.
The future workstream contracts stay in the linked plans; none becomes a prerequisite
to current Core work.
Under [OD-11](../docs/nevrotick-owner-decisions.md#15-od-11--socks5-scope-and-deferral),
Direct family/interface/path-history work stays in P2/P5; proxy-specific work is
outside first Windows delivery. The [existing engine task](engine-transport-pilot.md#socks5-feasibility-and-deferral--5-november-2026)
owns its limited future profile and SOCKS-C01 checks.

Preserve [current execution holds and scoped Sandbox permission](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent),
Core priority and UI-M0 deferrals. The blocked S134 artifact, general host Windows
execution/host changes, separate AES fuzz no-retry hold and storage pause remain
distinct. No task-index entry lifts a hold, selects a dependency or waives a gate.


## Documentation handoff review — 4 November 2026

Reconciled the operative documentation contracts with existing tasks at `dcda544`
plus the captured dirty candidate. The review used targeted/structured reads of
maintained authorities, workstream and design contracts; the security pass
examined all 345 ASVS records/dispositions and 39 open questions. Historical raw
appendices and archived copies remain reference material, not new authority or
requalified implementation. Existing CHECK-4 already owns the ASVS gate; its new
backlog sentence is a navigation clarification.

The core/protocol, Windows/security and future-plan reviewers completed their
bounded source-to-task checks. `review_docs_task_sync`, with `fork_turns: none`,
then reviewed the four task changes and relevant canonical sections. Verdict:
**Ready for tasks-only synchronization; no material findings.** Its two
comprehension paragraphs were:

> The revision makes deferred production work discoverable without changing the active Core checkpoint. Research cards preserve separate observation freshness, association/full-authentication ages, vote populations, persisted-floor authority and correction budgets. Their gates activate at the relevant production consumer; they neither establish qualification nor prevent unrelated ordinary acquisition. Existing Core work, scoped Sandbox permissions, blocked artifacts, fuzz/storage holds and UI-M0 deferrals remain controlling.

> Shared adapters preserve process authority and domain error ownership while concentrating public error mapping and sanitization. Production UI cards retain draft/current-policy separation, receiver validation, reconciliation after lost acknowledgements and independent freshness facts. Release rehearsal stays with release/security under the selected MSIX profile and David’s authority. Future Android, DNS and Chrony replacement remain deferred under their existing dependencies; closed research does not become unfinished implementation work.

Only task records changed. Local links and changed-document whitespace checks
passed; staging and source/document bytes outside the task edits were preserved.
Frozen inputs, source-coverage notes, edit identities and review/custody results
are retained locally under `/tmp/nevrotick-docs-task-sync-6lm64nwo/`. This review
executes no product/native gate and changes no readiness or execution permission.

## Documentation and source reconciliation — 6 November 2026

David requested verification of his PR12 merge, followed by a comparison of
documentation with actual code. Reviewed main:
`5ef985388d3350b707e91be1b78e8f8e9039f6ff`, tree
`721baa7b4d012a645fdb9710f3dfe997ad8426a5`. The
[build task's closeout](build-optimization.md#merge-and-final-hosted-verification--6-november-2026)
records the exact merge, valid GitHub signature and successful required PR and
main-push checks. The documentation correction is a separate candidate; it does
not inherit a fresh product/native pass from that base.

The review compared current entry points and task instructions with the six
workspace manifests, selected Rust interfaces, dependency policy, verification
scripts, workflows and later task evidence. It distinguished current assertions,
explicit historical receipts and planned/deferred product requirements. Three
read-only reviewers completed build/tooling, Core/engine and UI/Windows scopes
with inherited conversation context; these are complementary source reviews,
not fresh-context review or statistically independent test evidence. The
coordinator independently reconciled the findings and the P7 acceptance map.
This was a bounded audit, not an exhaustive review of every callback, archived
design, raw historical receipt, upstream dependency or security requirement.

| Finding | Evidence and consequence | Documentary correction |
| --- | --- | --- |
| DCA-01 — current UI route | [TOOLING.md](../TOOLING.md) still directed current commands to Win32; [the actual entry](../crates/nevrotick-ui/src/main.rs) selects `slint_host`. | Route current work to S0.5/GUI handoff; retain Win32 evidence as historical and native/backend gaps separately. |
| DCA-02 — next task | TOOLING's opening instruction restarted pure-leap qualification, a completed predecessor under [development §7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent). | Route to the maintained current queue and its prerequisites; preserve the post-V1 formal deferral. |
| DCA-03 — execution budgets | The general twelve-worker claim omitted six-job selected test builds in [verify.py](../scripts/verify.py) and ten-worker current Core/engine mutation policies. | State the general budget and admitted per-lane exceptions without changing code or limits. |
| DCA-04 — dependency admission | The active setup explanation still described 25 sources and global grants. [Preparation](../scripts/prepare-product.py), [schema-3 policy](../policy/ui-m0-dependencies.json) and `product_deny` use the current 534 registry identities and target/per-crate controls. | Explain current source maps, separate local patches, licence/build-script/duplicate permissions and pinned archive-file bypasses; preserve historical schema-2 evidence. |
| DCA-05 — engine macro gate | [The engine task](engine-transport-pilot.md#dependency-gate-before-compilation) still ordered macro2.7.1/Syn2 and the sixteen-source cohort; the current manifest/lock and [accepted refresh](s0.1-source-readiness.md#refreshed-graph-and-current-evidence) use macro2.7.2 with a separate Syn3 host closure. | Mark the original gate historical and route compilation to the current exact graph, with Garde's Syn2 closure retained. |
| DCA-06 — GUI graph identity | [The GUI handoff](slint-gui-handoff.md) labelled old lock and canonical-policy hashes current. Executable policy and code-owned anchor agree on the refreshed graph. | Update both current identities, distinguish canonical from raw-file hashing, and preserve the prior identities as history. |
| DCA-07 — engine evidence routing | [The current-state capsule](../docs/windows-time-architecture.md#1-current-state-capsule) linked only the first source checkpoint, hiding later bounded measurement/exact-nominal host qualification. | Link the completed named scopes while keeping refreshed numerical critical-owner and native gaps open; no blanket current pass. |
| DCA-08 — final acceptance inventory | The P7 plan, backlog and task-map ranges ended at AC-20, while [the canonical catalogue](../docs/windows-time-features.md#8-acceptance-scenarios-and-evidence) also owns AC-21/22. | Reconcile applicable AC-01–22 and preserve OD-11's direct-path/shared scope versus deferred proxy cases. |

All eight corrections are documentary. No implementation defect or build-policy
bypass was demonstrated by these findings, and no runtime, dependency, CI,
mitigation, permission, acceptance threshold or execution hold was changed.
Pure protocol/numerical components, controlled unauthenticated engine observations
and the Slint fixture GUI remain distinct from production synchronization.
The missing production observation-to-Core bridge, NTS, services/IPC, clock
effects, effective runtime protections, independent critical-text approval and
complete Windows/MSIX qualification retain their existing owners and gaps.

The localization plan's old "no packer/runtime" paragraph and the MSIX plan's
old Win32 inventory were examined and excluded from findings: each is expressly
bound to an earlier historical snapshot. Their presence does not contradict the
current source routes. No historical receipt was rewritten as a newer pass.

Verification used source comparison, independent standard-library hashing and
JSON canonicalization, added-link/heading resolution and `git diff --check`.
The lock digest agrees with the policy; the canonical policy digest agrees with
the code-owned pin and corrected GUI handoff. No repository program or product
test was executed for this documentation-only audit. `docs/SHA256SUMS.txt` is
absent, so no bundle-manifest integrity result is claimed.

# NevroTick agent guide

## Approach: the skeptical expert

Accuracy/intellectual honesty first. Test material/uncertain claims and competing explanations against strongest evidence. Separate facts/inference/opinion/uncertainty. Numerical confidence needs explicit method. Correct respectfully.

Bound external feasibility through [next-stage planning](.agents/skills/planning/plan-code-change/SKILL.md); block only dependent work. Finish coherent changes before expensive [qualification](.agents/skills/testing/CONTRACT.md#execution-efficiency); retain prerequisite checks and valid evidence reuse. Repo skills preferred unless user selects plugin; add only for concrete gaps.

## Start each task

1. Resolve `git rev-parse --show-toplevel`; inspect `git status --short`, nested `AGENTS.md`, actual manifests/source. Keep work; no historical-state assumptions.
2. Read [docs guide](docs/README.md), relevant authority, [plan and acceptance map](docs/README.md#plans-acceptance-and-execution-evidence): workstream/phase/slice/prerequisites/exit.
3. Reuse task/plan. UI: [Slint GUI handoff](tasks/slint-gui-handoff.md), [S0.5 migration task](tasks/s0.5-ui-comparison.md), [OD-13](docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit), then [tasks/ui-m0.md](tasks/ui-m0.md): original fixture scope/ownership, [acceptance](tasks/ui-m0.md#acceptance-and-checks), [remaining evidence](tasks/ui-m0.md#remaining-evidence-and-next-work), candidate addenda. Otherwise `tasks/`, `.agents/plans/`; [planning guide](.agents/skills/planning/README.md) for plan changes. One evidence chain. After pull/doc update, reconcile [next task](docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)/[imported queue](docs/plans/development-plan.md#imported-hardening-work-queue). Before dependent work, import applicable assignments/ready-held-deferred status into existing task. Docs grant no completion/hold lift.
4. Smallest relevant [skill set](.agents/skills/README.md); full `SKILL.md`. Before check selection read [testing contract](.agents/skills/testing/CONTRACT.md). Existing task/concise chat: requirement/acceptance IDs, independent expectation, source owner, environment.
5. Use [configured verification](TOOLING.md#starting-implementation). Handoff: candidate/results/gaps/deferrals/next action. Plans/history/cross-builds prove no current native/release qualification.

Every development stage: [lifecycle/skill owners](docs/plans/development-plan.md#development-lifecycle-and-skill-ownership), then [checkpoint skill use](docs/plans/development-plan.md#checkpoint-skill-application). List required/triggered skills; apply full instructions. Existing task: stage → skill → requirement → action → evidence/status. Reading ≠ use. Reconcile every required QA obligation; held/unavailable ≠ N/A/pass. Reuse unchanged guidance; no read-all/all-tools ritual.

Proactively delegate useful independent work across this lifecycle via [testable slices](.agents/skills/planning/references/testable-slices.md#development-delegation). Coordinator owns shared contracts/integration; verify combined candidate.

[OD-10's first-release deferral](docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026): formal work after first released version (V1); missing proofs block no V1 impl/acceptance/release. Until then: no formal compiler/solver/proof jobs, verifier extensions, pending proof admission/metadata-consumer repairs. Continue product plan. Existing tasks retain post-V1 obligations/exact historical proofs. Runtime validation, garde/private invariants, checked arithmetic, independent behavioral/property/boundary tests, 90% security-primitive coverage, 95% critical-file mutation, security/native/package/release gates remain. No Windows/fuzz/storage/UI hold lift.

Formal planning/review: [OD-10's proof-purpose rule](docs/nevrotick-owner-decisions.md#142-proof-purpose-and-completion--6-november-2026), named guarantee from [post-V1 debt](tasks/development-backlog.md#product-guarantees-still-open), actual source/caller/constructor chain, independent semantic controls, bounded pilot exit. Helper proofs/green tooling/counts never close parent. Use [proof workflow](.agents/skills/testing/references/formal-verification.md); policy/docs resume no formal execution.

## Read the right authority

NevroTick: chrony-derived Rust NTP/NTS client/native GUI. Intended initial scope: Windows 11 26H2 x64, Windows Server 2025 x64 Desktop Experience, `x86_64-pc-windows-msvc`; [OD-14](docs/nevrotick-owner-decisions.md#18-od-14--compact-windows-platform-scope). Exact profiles/availability/native/support qualification remain open in [two-family matrix](tasks/windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026). Start [docs/README.md](docs/README.md), then relevant sections:

| Document | Authority and use |
| --- | --- |
| [Owner decisions](docs/nevrotick-owner-decisions.md) | Accepted/proposed decisions/procedure; read before reuse/correction/source-default/release policy changes. |
| [Research handoff](docs/nevrotick-research-handoff.md) | Evidence/NTS provider candidates/gates; research ≠ impl qualification. |
| [Architecture](docs/windows-time-architecture.md) | Canonical technical/security contracts, ownership, decisions, delivery phases, CHECK requirements. |
| [Leap policy](docs/plans/leap-second-policy.md) and [S1.4 task](tasks/s1.4-leap-policy.md) | Leap/LI, UTC scale, calendar metadata, Windows mapping changes. Architecture §10.5: local reviewed knowledge, useful silent-source admission, kernel-only explicit events; impl/native evidence pending. |
| [Feature plan](docs/windows-time-features.md) | Proposed V1 capabilities, journeys, presentation, acceptance, delivery slices; architecture governs technical semantics. |
| [Development hardening](docs/windows-time-development-hardening.md) | Enforcement/qualification; actual config and TOOLING.md distinguish adopted bootstrap checks from pending app/native evidence. |
| [ASVS applicability](docs/asvs-5.0.0-applicability.md) | Applicability/planned verification; no completed compliance. |
| [Signing guide](docs/windows-time-signing-guide.md) | Provider/signing/complete-package workflow; evidence required for eligibility/native checks. |
| [Release policy](docs/release-policy.md) | Support status/proposed first-stable terms/promotion/withdrawal/publication-authority recovery. W11 owns rehearsals/results; docs grant no support/release approval. |
| [Update assessment](docs/windows-time-update-assessment.md) | OD-05 §7.10 adopts TUF + complete MSIX; one Update now decision. Client/time/Windows/signing gates open. |

Accepted owner decisions govern stated scope; recommendations remain proposals. Reviews explain decisions/corrections, never replace current contracts or prove impl. Keep REQ/BND/DEC/THR/CHECK identities.

Material unresolved choice: NV-DEC-001 evidence/options/recommendation/dependent-action card; ask David. Routine accepted work continues; record outcome canonically. Silence/research deferral ≠ risk acceptance. OD-05 §7.9: direct complete MSIX; Store rejected. Signing deferred; source/package work continues. 8 November §7.7: package/all supported shipped files signed, incl. nonbinary; architecture §19.7/signing guide §4.1. Foundation display/David release authority remain. MSI needs later decision; unsigned-MSI/OpenPGP exception superseded. Required signing unavailable: release blocked, source preparation continues. Later §7.8: if MSI selected, interim self-signed accepted, all-file signatures required, absent public CA alone no blocker. Signing guide §7.1: approved-key integrity, clean-machine chain distrust, actual runtime/channel results separately. No customer cert import/protection bypass/fake trust pass.

Exploit protection: read [architecture §15.4](docs/windows-time-architecture.md#154-per-process-mitigation-profiles-and-startup-admission), [hardening §6.4](docs/windows-time-development-hardening.md#64-per-process-mitigation-qualification). Distinguish process roles, activation phase, required/optional policy, effective state, native evidence; compiler flags prove no complete runtime profile.

**Required CPU and hardening policy, clarified 6 November 2026:** OD-07 Windows minimum **x86-64-v3**, selected in target config/exact admission. Never restore generic x86-64/v2 or substitute v4, `native`, ambient flags. OD-05: every applicable final-product protection at strongest compatible supported setting; existing build/runtime/package owners implement/qualify. Missing evidence = open work, never optionality. Record concrete platform/toolchain/necessary-feature exclusions; David owns security tradeoffs/weaker release exceptions. Follow [hardening §6.2–6.4](docs/windows-time-development-hardening.md#62-target-specific-windows-compilerlinker-settings), [build task](tasks/build-optimization.md#required-v3-and-maximum-compatible-hardening--6-november-2026). Historical generic receipts, pending Windows runs, hardware-conditional CET neither change requirement nor raise CPU floor.

Shared OS/IPC: [architecture §3.4](docs/windows-time-architecture.md#34-shared-windows-adapters-and-diagnostics). Reuse narrow role-specific `windows-platform` adapters/portable `ipc-contract`; no shared-code privilege or generic Windows command surface.

Engine runtime/concurrency: [architecture §18.6.1](docs/windows-time-architecture.md#1861-engine-runtime-decision-and-alternatives), section 17.1, TLS gate section 18.7.1. Tokio stays engine-only current-thread prototype; qualify bounded native work/complete NTS-KE arrangement. No required `tokio-rustls` or runtime dependency expansion to core/broker/UI/helper.

IPC: [architecture §5.4](docs/windows-time-architecture.md#54-native-ipc-implementation-and-qualification), [hardening §9.9](docs/windows-time-development-hardening.md#99-ipc-identity-framing-and-native-lifetime-verification), `security-local-ipc`. Qualify actual wrapper rights, thread-bound caller capture, fatal reversion, continuous endpoint ownership, partial/native I/O completion; examples/mocks prove no Windows enforcement.

Protected optional storage: [agent task](tasks/windows-protected-storage.md), [architecture §12.1.2](docs/windows-time-architecture.md#1212-protected-optional-storage-and-explicit-fallback), [storage plan](docs/plans/windows-protected-storage.md). Task maps [second upstream review](docs/plans/windows-protected-storage-upstream-lessons-2.md) to bounded work/ST-T/CHECK/DNS-C owners. Keep indeterminate outcomes, actual native completion, privacy barriers, separate mandatory-state owner. Examples admit no dependency/weaker fallback; RedirectionGuard/hardware rollback epochs keep candidate gates. Storage experiment pause/current product priority remain.

ST-W01–ST-W07 are imported. ST-W01 documentary card complete; engineering freeze open; ST-W02 unadmitted; native held. ST-W03–ST-W07 apply before related Windows filesystem/CNG/mitigation/IPC-completion/sensitive-memory/diagnostic adapters, even before optional-cache integration. [Storage task](tasks/windows-protected-storage.md): disposition/next action; no resurvey/new plan.

Windows 11: [source review](docs/plans/windows11-integration.md), [Microsoft Learn follow-up](docs/plans/windows11-integration-follow-up.md), [agent task](tasks/windows11-integration.md), [existing-phase workstream](docs/plans/development-plan.md#85-windows-11-integration-research-and-qualification). Map packet provenance, power/timers, service/network confinement, MSI maintenance, DNS ownership, background-resource candidates to CHECK/DNS-C/ST-T owners. Separate configured/effective state, source/native evidence. APIs/analyzers/dependencies conditional until owner adoption; amended OD-05 separately selects MSIX. Product priority/experiment pause/UI-M0 deferrals remain.

**V1/W11:** [OD-02](docs/nevrotick-owner-decisions.md#v1-protocol-and-hardware-scope--27-november-2026): NTP/NTS + Roughtime bootstrap; no PPS/GNSS/refclock input or owned-hardware prerequisite. W11-W01 physical trial blocked: no inventory/survey repeats on handoff; resume only on changed evidence/method. Continue §7's source child; physical accuracy/positive mapping/Core claims stay open. W11-W01–10 retain §8.5 phase/contract gates and all holds.

Follow-up cases: callbacks/completion, ancillary data, COM, MSI permissions, ETW sessions, partial-file replacement; existing owners. Qualify SDK/generated bindings against source discrepancies, never blindly copy overview declarations. Service-managed storage/native cost hints/SCM observers remain candidates; prerelease VBS/side-channel settings conditional. Refinements precede dependent adapters; record W11/storage disposition. Keep scoped Windows/fuzz holds/storage pause. Reconcile [Windows task](tasks/windows11-integration.md#44-microsoft-learn-second-pass-integration--3-november-2026)/[storage task](tasks/windows-protected-storage.md#65-microsoft-learn-follow-up-routing--3-november-2026); import applicable W11/ST-W cases with owner/disposition/next action before adapters. W11-W01 partial; ST-W01 engineering/native open; neither permits native probes/hold lifts.

Direct MSIX: [OD-05 §7.9](docs/nevrotick-owner-decisions.md#79-direct-msix-distribution-microsoft-store-rejected--21-november-2026) rejects Store publication/indemnity; [plan](docs/plans/windows-msix-store.md) and [architecture §19.4](docs/windows-time-architecture.md#194-future-complete-msix-gate) retain one complete package. No Store trial/refusal/account prerequisites. Signing/trust deferred; continue source/package preparation. W11-W10 owns; W11-W05/release/security/storage supply provisioning/mitigation/recovery evidence. Core priority/holds remain. Preserve package integrity/sealed code/per-role protections; requalify Store-dependent CIG for direct signing, never assume trust or silently weaken protection. Inspect final distributed/installed bytes before release. MSI requires later choice; no split package, broader service rights, customer cert import or unqualified release.

Release/incidents: [release policy](docs/release-policy.md), [existing W11 task §4.21](tasks/windows11-integration.md#421-release-operations-integration--4-november-2026). First-stable terms proposed pending David; P6/P7 owners rehearse account/containment/corrective-state/recovery. Runbooks lift no holds/blockers. [OD-05 §7.5](docs/nevrotick-owner-decisions.md#75-optional-enhanced-build-provenance--4-november-2026)/[enhanced provenance](docs/plans/release-provenance.md): attestations/independent reproducible builds/public proof bundle optional, no new blocker. Required custody/SBOM/licence/package integrity/installed identity/native-lifecycle evidence remain.

Logging/verbosity: [architecture §12.3](docs/windows-time-architecture.md#123-logging-and-support), [hardening §9.8](docs/windows-time-development-hardening.md#98-diagnostics-and-shared-adapter-verification). Explicit per-process sinks, typed fields, required-event routing, locally expiring authorized DEBUG sessions; diagnostics ≠ authoritative UI state/clock journal.

Localization: [OD-15](docs/nevrotick-owner-decisions.md#19-od-15--externalized-ui-translations-and-binary-catalogs), [catalog design](docs/plans/ui-localization.md), architecture §16.4 before Slint/i18n skills. Selected editable JSON → validated CBOR → typed Rust presentation → Slint/tray/notification/accessibility. App-owned catalog, no selected native Slint CBOR loader. English fallback/stable codes-arguments/adopted-locale scope remain. Codec/formatter/generated-resource/loader evidence: S0.5/GUI handoff. External arbitrary user-import unselected; no native/Core execution waiver.

Inspect [Cargo workspace](Cargo.toml)/actual source before commands; never scaffold crates for doc/tool checks. Linked tasks own exact current and historical candidates, compiler/graph admission and results. Source/cross-build/scripted Windows evidence proves no production sample eligibility, integrated controller/acquisition, native TLS/CNG/UDP, services, clock authority or phase completion. Proposed components remain plans until source exists.

- [nevrotick-ui](crates/nevrotick-ui/Cargo.toml): portable presentation/Slint adapter; native/backend open.
- [ntp-wire](crates/ntp-wire/Cargo.toml): inert [header](tasks/s1.1-header.md)/[framing](tasks/s1.1-framing.md); [protocol task](tasks/s0.1-protocol-comparison.md) owns inert NTS-KE/negotiation/request/envelope/plaintext. No exchange/era/sample admission or TLS/AEAD/authenticated-source authority.
- [time-domain](crates/time-domain/Cargo.toml): checked numerical/era/clock arithmetic; bounded [task](tasks/s1.1-time-domain.md), no network/clock/authentication effects.
- [ntp-exchange](crates/ntp-exchange/Cargo.toml): single-slot unauthenticated [fixture association](tasks/s1.1-exchange-association.md)/[scenario harness](tasks/s1.1-deterministic-harness.md).
- [time-core](crates/time-core/Cargo.toml): pure regression/history/selection/combination/readiness/modeled correction; [regression task](tasks/s1.2-regression-readiness.md)/[current Core task](tasks/s1.3-source-selection-readiness.md). [Settled-rate receipt](tasks/s1.3-source-selection-readiness.md#settled-rate-final-qualification--four-selected-owners) stays four-owner Linux fixture evidence. Follow [current continuation](docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent).
- [nevrotick-engine](crates/nevrotick-engine/Cargo.toml): engine-only [transport task](tasks/engine-transport-pilot.md); controlled loopback/default-off NTS protected source and managed laboratory successors; probe one-shot/never-send default. Production mapping/quality/Core admission and native per-role gates/holds remain. KE metrics require rebound complete source/helpers/test roster/reviewed inventories for 90% coverage/95% mutation; campaigns closed until admission. History scores retain original candidates.
- [windows-platform](crates/windows-platform/Cargo.toml): portable lifecycle/SCM/privilege contracts; [windows-platform-native](crates/windows-platform-native/Cargo.toml): narrow clock/DNS/process/service adapters. Two fixed-role [lab hosts](tools/windows-service-fixture/README.md) remain outside workspace. [Windows task](tasks/windows11-integration.md) owns scoped laboratory receipts and open product identity/provisioning/hosting/runtime qualification.

## Current UI milestone

[OD-13](docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit): Slint default/full Win32 replacement. [S0.5 task](tasks/s0.5-ui-comparison.md) owns design v0.4 migration; start current addenda/[Slint GUI handoff](tasks/slint-gui-handoff.md). Windows entrypoint selects Slint; unexported Win32 historical. Portable rendering/model/Windows-target checks retain scope; native/backend/accessibility/resources/independent critical-text approval open. Win32 receipts never qualify Slint.

David approved GUI GPL-3.0-only, Chrony Core/engine GPL-2.0-only. Reopen neither licence/toolkit. Keep GUI grant/exact runtime-build-asset provenance; graph/composition/generated-code unsafe-lint/security/native/package review remains. Source/admitted fixture UI may parallel Core; one Cargo graph/schema owner, all holds.

[Architecture §16.5](docs/windows-time-architecture.md#165-restricted-first-ui-milestone-ui-m0) before UI/QA skills. Original UI-M0: unelevated fixture-only native prototype; current S0.5 addenda own full GUI source. UI-M0 **D1** theme/contrast/text/DPI, **D2** production freshness/reconnect/policy/administrative-result handling, **D3** full keyboard/Narrator/UIA **Deferred**: keep task triggers; no UI-M0 blockers. Admitted source ≠ native qualification. Keep basic toolkit keyboard/editing, meaningful labels, Rust/FFI checks, sample labels, disabled protected actions, native window observations. Slint inherits no Win32-control accessibility pass. No live network/DNS/production IPC/persistence/services/elevation/clock effects. Backlog/next stage retain deferred owners/triggers; restore before §16.5 integrations/preview claims. UI-M0 neither completes P0 toolkit qualification nor constrains other workstreams.

Use David's [design v0.4 reference](docs/nevrotick-design-v0.4/README.md); read [guide](docs/nevrotick-design-v0.4/README-RU.md), [tray update](docs/nevrotick-design-v0.4/review/tray/REPORT-RU.md), [status/notification audit](docs/nevrotick-design-v0.4/review/status-notifications/REPORT-RU.md), then [interactive preview](docs/nevrotick-design-v0.4/NevroTick-UI-v0.4.html) for composition/interaction; dependency-free HTML/tests in bundle. [Design v0.2](docs/nevrotick-design-v0.2/README.md), React viewer/scenes historical; IDs not v0.4 links. [AVIF brand kit](docs/NevroTick-AVIF-kit-v1/README.md)/[manifest](docs/NevroTick-AVIF-kit-v1/manifest.json): mascot/logos/wordmarks/icons/backgrounds, separate asset notices/code licence. Milestone limits apply. HTML/archived architecture reference-only; current contracts govern. Richer v0.4 service/notification examples simulated; no UI-M0 expansion/D1–D3 hold lift. [UI-M0 task](tasks/ui-m0.md#publication-and-hosted-qualification) records archived React dependency findings: resolve/verify before execution; native Rust does not use it.

## Architectural constraints

Canonical sections set exact limits; keep boundaries:

- Separate network engine without clock privilege, narrow privileged clock broker, normal-user Slint UI, elevated lifecycle helper. Complete MSIX under amended OD-05 needs supported service provisioning/full lifecycle qualification. MSI-first superseded; MSI mechanics only later owner-selected alternative. Crate proposals express ownership; no immediate all-crate scaffolding.
- Product/user/external programmatic input potentially malicious. UI controls, local origin, authenticated transport, administrator identity/prior acceptance never establish payload safety. Bound/validate each admission path; independently check current authority at effect. Keep raw-input/direct-handler bypass tests, healthy neighbors, aggregate budgets, bounded redacted rejection.
- All raw JSON/CBOR potentially malicious: contributor/translator data, English, schemas/metadata, generated, signed/packaged/previously accepted files. Reuse immutable typed values only within valid context. Localization: independent code-owned contract/checker controls IDs/types/plural/action/semantic approval; catalogs cannot redefine. Apply [OD-15's threat controls](docs/plans/ui-localization.md): isolated conversion, bounded decode/format/render, literal sinks, whole critical-group English fallback. Origin/signature/syntax prove neither safety nor truthful translated consent.
- Fixed JSON IPC/config/persisted-record boundaries: bounded strict Serde DTOs; adopted shared garde inside fallible private domain construction; current trusted authorization/generation/budget guards. Serialize protected decisions/effects/actual acknowledgements. Never use `serde_json::Value` normalization to bypass admission or add second runtime schema engine.
- Every function/module/private helper: explicit input/output/failure/effect/lifecycle obligations; enforce typed signatures, private checked construction/transformations, current owner guards. Reuse validated immutable values; recheck changed representations/state/context. Never rely on optional caller validation, `debug_assert!`, raw public fields/unchecked mutation. [Contract enforcement](.agents/skills/implementation/implement-rust-contracts/references/contract-enforcement.md): no `garde(allow_unvalidated)`; each skipped field needs invariant owner/independent review. Garde does not qualify product Cargo graph.
- Explicit types/consequential assumptions: [source-visible type rule](docs/windows-time-architecture.md#source-visible-types-and-assumptions--18-november-2026). Default explicit handwritten-local types; keep domain/role types, checked conversion targets, actual assumption owners. Annotation alone proves no units/provenance/current authority/native completion.
- Bounded NTP/NTS parsers. OD-23: default minimum three fresh independent operator groups plus majority; no automatic reduction after loss/disagreement. §7.2.2 owns the production profile; fixture evidence stays scoped.
- Distinct timestamp units/UTC/monotonic/active-awake clocks. Positive offset = Windows behind. Estimator history uses acknowledged applied correction, never merely requested correction.
- Keep NTS-only baseline while resolving OD-03 Pool/default-profile interaction; no unqualified shipped preset/silent downgrade. Explicit visibly labelled plain-NTP profiles. Bootstrap hints allow bounded provisional TLS-clock recovery only; no global clock writes/authenticated-tracking claims.
- OD-02: qualify very large authenticated recovery. OD-23 fixes the default agreement floor; era/date, freshness/recovery and correction bounds remain open; plain-NTP 12 hours unapproved. Keep clock ownership, stale-state rejection, cumulative budgets, suspend/resume invalidation and W32Time handover/restoration. “Sync now” requests a bounded fresh check, never arbitrary correction or policy change.
- Bound actual native work/resources. Dropped/aborted awaiting future ≠ native completion; permits/custody follow actual operation through completion/recovery.
- Unsafe limited to narrow reviewed Windows adapters. Useful errors; secret-redacted events/exports. UI distinguishes authentication/agreement/freshness/readiness/applied effects.
- Keep Chrony Core/engine GPL-2.0-only provenance/original GUI approved GPL-3.0-only grant. Qualify exact dependency/features/license graphs/held crypto providers before adoption. Versionable `Cargo.lock`; exclude `target/` from Git/search indexes.

## Choose skills by the changed guarantee

Use [skills index](.agents/skills/README.md): relevant leaves/conditional references only. Small understood edit: concise action/check; no report tree/orchestration campaign.

| Work | Start with |
| --- | --- |
| Uncertain scope/stages/ownership | [plan-code-change](.agents/skills/planning/plan-code-change/SKILL.md); architecture/threat/review when decisions change. |
| Rust logic/concurrency/Windows adapters | [implement-rust](.agents/skills/implementation/implement-rust/SKILL.md), conditional SCM/chrony/simulator references. |
| Windows service creation/SCM lifecycle/management/hardening/logging/optimization | [implement-windows-service](.agents/skills/implementation/implement-windows-service/SKILL.md); detailed service/recovery/access matrices, security/QA owner. |
| Function/module contracts, wire admission, domain invariants | [implement-rust-contracts](.agents/skills/implementation/implement-rust-contracts/SKILL.md). |
| Wire/persisted-state/semantic compatibility | [evolve-data-contracts](.agents/skills/implementation/evolve-data-contracts/SKILL.md). |
| Native UI/accessibility | [implement-native-ui](.agents/skills/implementation/implement-native-ui/SKILL.md), [implement-slint](.agents/skills/implementation/implement-slint/SKILL.md) toolkit mechanisms, applicable accessibility/DPI checks. |
| Trust/privileges/IPC/crypto/logging/packaging | [Security index](.agents/skills/security/README.md), actual boundary owner. |
| Windows product/package metadata | [maintain-windows-metadata](.agents/skills/implementation/maintain-windows-metadata/SKILL.md); package QA owns artifact/native qualification. |
| Cargo dependencies | [dependency-updates](.agents/skills/environment/dependency-updates/SKILL.md). |
| Windows Sandbox config/WSL-to-guest control/disposable-lab evidence | [manage-windows-sandbox](.agents/skills/environment/manage-windows-sandbox/SKILL.md); current task/incident holds before any Windows command. |
| Verification/failure triage/commit preparation | [Testing index](.agents/skills/testing/README.md), [testing contract](.agents/skills/testing/CONTRACT.md); smallest adequate evidence layer. |
| Scheduling races/generated histories/Python admission semantics | [Lifecycle](.agents/skills/testing/qa-state-concurrency-lifecycle/SKILL.md), [model](.agents/skills/testing/qa-model-based-testing/SKILL.md), [property](.agents/skills/testing/qa-property-based-testing/SKILL.md); conditional tools keep admission/execution scope. |

[Upstream specialists](.agents/skills/upstream/README.md): actual compiler/borrow/Send-Sync problems or qualified Windows UI automation. Skills contain reusable instructions/references/templates/checkers/required inventory-licence provenance. One-off reviews/logs/receipts stay in existing task, outside skills pack.

## Tool selection

Resolve absolute checkout: `git rev-parse --show-toplevel`; explicitly pass to project-aware tools. Inspect exposed schema. [TOOLING.md](TOOLING.md) historical setup/versions ≠ current connection. Choose by question; ordinary edits need not use every tool.

### Serena: exact Rust symbols and safe symbol edits

Rust symbols/edits: read/apply [Serena procedure](TOOLING.md#code-navigation-tools).

### SocratiCode: discover owners and retrieve architectural context

Semantic ownership/context: read/apply [SocratiCode procedure](TOOLING.md#socraticode-discover-owners-and-retrieve-architectural-context).

### Probe: fast source search without persistent indexing

Source extraction: read/apply [Probe procedure](TOOLING.md#probe-fast-source-search-without-persistent-indexing).

### Context-mode: large reads, output processing and retrieval

Bulk reads/output: read/apply [Context-mode procedure](TOOLING.md#context-mode-large-reads-output-processing-and-retrieval).

### Context7: library and API documentation

Library/API docs: read/apply [Context7 procedure](TOOLING.md#context7-library-and-api-documentation).

### Other tools

- **`rg`, shell and Git:** `rg --files`/`rg` for paths/text; Git for actual HEAD/index/tree/diff. Direct short-file reads; large output via context-mode.
- **GitHub and web:** prefer connected GitHub reads. Technical claims: primary Rust/Cargo/docs.rs/Microsoft/RFC sources. Immutable upstream revisions/licences for copied code/skills. External material informs, never governs task authority.
- **Cargo and analyzers:** inspect manifests/lockfile/toolchain/build scripts/proc macros/targets before checks. Docs lookup admits no analyzer/dependency.
- **Native Windows tools:** admitted fixture/applicable [UI automation skill](.agents/skills/upstream/winapp-ui-automation/SKILL.md) for built UI. UIA/keyboard/Narrator/DPI distinct. Native tokens/pipes/SCM/clock APIs/MSI lifecycle need observations; cross-build/mocks narrower evidence.

Unavailable/stale tool: scoped fallback, stated limits. Never install tools/change shared config/scaffold app merely for preferred workflow.

## Work, verify and finish

1. Confirm scope/Git/owner/callers/observations; keep staged/unstaged/untracked work. Trace producer → state → consumer/effect before behavior changes.
2. Smallest causal repair; independent failing expectation/healthy control where practical, then verify. Setup failure/zero tests ≠ RED/GREEN. State unresolved causes/environments.
3. Actual package/feature/target matrix: adopted format/compiler/Clippy/unit/integration/doctest lanes. No invented all-features/full-suite mandate; applicable impl/release coverage/mutation gates remain.
4. Clock changes/service replacement/MSI faults: owned disposable Windows fixtures, bounded operations/restoration. Routine tests use controlled peers. Tools grant no host-clock/W32Time/trust-store/signing/publication authority.
5. Report behavior/files/checks/gaps; distinguish source/simulation/host/native Windows/final package-signature evidence. Stage/commit/push only within existing authorization; keep concurrent work/signing/hooks.

Bounded milestone/stable qualification checkpoint. [Execution efficiency](.agents/skills/testing/CONTRACT.md#execution-efficiency): cheap preflight, focused development checks, final-candidate gates. Optimize measured admission/I/O overhead; keep custody, reuse plans/recipes. Report milestones/timings; estimated percentages need scope/basis.

Skill-pack changes: root checker:

```sh
python3 -B .agents/skills/scripts/check_skills.py
```

Checker/covered-contract changes: `python3 -B .agents/skills/scripts/test_check_skills.py -v`. Imported-leaf edits: keep upstream hashes/notices.

Keep `AGENTS.md` below 32768 UTF-8 bytes; verify after edits.

Docs: check links/canonical consistency. Verify `docs/SHA256SUMS.txt` **when present**; update for changed covered docs. Missing/stale manifest proves no integrity. Transient plans/reports outside maintained bundle unless task assigns there.

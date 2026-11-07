# NevroTick agent guide

## Approach: the skeptical expert

Prioritize accuracy and intellectual honesty. Verify consequential/uncertain
claims against strongest available evidence; test competing explanations that
could change conclusions. Distinguish facts, inference, opinion, uncertainty.
Numerical confidence requires explicit method. Correct mistaken assumptions
firmly and respectfully.

Keep investigation/verification proportional. Reuse evidence while relevant state
unchanged. Prefer repository skills over overlapping plugins unless user selects
plugin; load another skill only for concrete gap.

## Start each task

1. Resolve the checkout with `git rev-parse --show-toplevel`; inspect
   `git status --short` and applicable nested `AGENTS.md` before edits. Preserve
   existing work and inspect current manifests/source rather than assuming a
   historical repository state.
2. Read [the documentation guide](docs/README.md), then the relevant authority
   below. Use its [plan and acceptance map](docs/README.md#plans-acceptance-and-execution-evidence)
   to identify the workstream, phase/slice, prerequisites and observable exit.
3. Reuse the existing task/plan. For current UI work, start with the
   [Slint GUI handoff](tasks/slint-gui-handoff.md) and
   [S0.5 migration task](tasks/s0.5-ui-comparison.md) under
   [OD-13](docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit),
   then [tasks/ui-m0.md](tasks/ui-m0.md) for the original fixture scope, ownership,
   [acceptance](tasks/ui-m0.md#acceptance-and-checks),
   [remaining evidence](tasks/ui-m0.md#remaining-evidence-and-next-work) and later
   candidate addenda. For other work, inspect `tasks/` and any `.agents/plans/`
   records; follow [the planning guide](.agents/skills/planning/README.md) when
   creating or revising a plan. Keep one plan/evidence chain.
   After a pull or documentation update, reconcile the active task with the
   development plan's [next task](docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
   and [imported hardening work queue](docs/plans/development-plan.md#imported-hardening-work-queue).
   Carry applicable assignments and their ready/held/deferred status into the
   existing task before its next dependent implementation; a new document alone
   does not establish task completion or lift an execution hold.
4. Select the smallest relevant [skill set](.agents/skills/README.md) and read
   its `SKILL.md`. Before selecting checks, read
   [the testing contract](.agents/skills/testing/CONTRACT.md). State the applicable
   requirement/acceptance IDs, independent expected result, source owner and
   required environment in the existing task record or a concise in-chat plan.
5. Use [the configured verification entry points](TOOLING.md#starting-implementation)
   for implementation checks. At handoff, record the candidate, actual results,
   remaining gaps/deferrals and next action. Planned criteria, historical passes
   and cross-builds do not establish current native or release qualification.

Subagent assignments include the bounded question, canonical sections and
acceptance IDs, current task/candidate, writable scope and expected evidence.
The coordinator reconciles results against the integrated candidate.

Apply [OD-10's first-release deferral](docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026):
formal proof work resumes after the first released software version (V1).
Missing proofs do not block V1 implementation, acceptance or release. Do not run
formal compiler/solver/proof jobs, extend verifiers or repair pending proof
admission/metadata consumers before then. Continue the ordinary product plan;
keep proof obligations in the existing tasks as post-V1 work and retain completed
proofs only within their exact historical scopes. Runtime validation, garde/private
invariants, checked arithmetic, independent behavioral/property/boundary tests,
90% security-primitive coverage, 95% critical-file mutation, security, native,
package and release gates remain. This lifts no Windows/fuzz/storage/UI hold.

Apply [OD-10's proof-purpose rule](docs/nevrotick-owner-decisions.md#142-proof-purpose-and-completion--6-november-2026)
when planning or reviewing formal work: select a named product guarantee from
the [post-V1 debt](tasks/development-backlog.md#product-guarantees-still-open),
require its actual source/caller/constructor chain and independent semantic
controls, and stop at the bounded pilot's exit. Helper proofs, green tooling and
check counts do not close the parent guarantee. Use the existing
[proof workflow](.agents/skills/testing/references/formal-verification.md);
this policy/documentation maintenance does not resume formal execution.

## Read the right authority

NevroTick: chrony-derived Rust NTP/NTS client, native GUI; intended initial scope
is Windows 11 26H2 x64 and Windows Server 2025 x64 with Desktop Experience,
`x86_64-pc-windows-msvc`, under [OD-14](docs/nevrotick-owner-decisions.md#18-od-14--compact-windows-platform-scope).
The [two-family matrix](tasks/windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026)
keeps exact profiles, availability and native/support qualification open. Start with
[docs/README.md](docs/README.md), then task-relevant sections:

| Document | Authority and use |
| --- | --- |
| [Owner decisions](docs/nevrotick-owner-decisions.md) | Accepted instructions, pending recommendations and individual owner-decision procedure. Read before changing reuse, correction, source defaults or release policy. |
| [Research handoff](docs/nevrotick-research-handoff.md) | Remaining evidence assignments, NTS provider candidates and decision gates; research findings are not implementation qualification. |
| [Architecture](docs/windows-time-architecture.md) | Canonical technical/security contracts, ownership, decisions, delivery phases, CHECK requirements. |
| [Leap policy](docs/plans/leap-second-policy.md) and [S1.4 task](tasks/s1.4-leap-policy.md) | Read for leap/LI, UTC scale, calendar metadata or Windows mapping changes. Architecture §10.5 selects local reviewed knowledge, useful silent-source admission and kernel-only explicit events; implementation/native evidence remain pending. |
| [Feature plan](docs/windows-time-features.md) | Proposed V1 capabilities, journeys, presentation, acceptance scenarios, delivery slices; architecture controls technical semantics. |
| [Development hardening](docs/windows-time-development-hardening.md) | Enforcement/qualification policy; inspect actual configuration and TOOLING.md for adopted bootstrap checks versus pending application/native evidence. |
| [ASVS applicability](docs/asvs-5.0.0-applicability.md) | Applicability/planned verification; no completed compliance. |
| [Signing guide](docs/windows-time-signing-guide.md) | Provider, signing, complete-package workflow; eligibility/native verification require evidence. |
| [Release policy](docs/release-policy.md) | Current support status, proposed first-stable commitment, promotion, withdrawal and publication-authority recovery. Existing W11 task owns actual rehearsals/results; documentation supplies no support approval or release pass. |
| [Update assessment](docs/windows-time-update-assessment.md) | Retained research; TUF/automatic updater not adopted. |

Accepted owner decisions govern their stated scope; recommendations remain
proposals. Supporting reviews explain decisions/corrections; neither replace
current contracts nor prove implementation. Preserve canonical
REQ/BND/DEC/THR/CHECK identities.

For a material unresolved choice, prepare the decision card in NV-DEC-001 with
evidence, options, recommendation and the dependent action, then ask David.
Routine work within accepted contracts continues. Record the outcome in the
canonical documents; do not treat silence or research deferral as risk acceptance.
Amended OD-05 selects complete hardened MSIX preparation and David's Store
attempt with services. It retains Foundation display and initial OpenPGP/MSI
policies as alternatives for a later delivery decision, with David as release
approver; profile-specific trust and Windows qualification gates remain.

Windows exploit-protection work: read [architecture section 15.4](docs/windows-time-architecture.md#154-per-process-mitigation-profiles-and-startup-admission)
and [hardening section 6.4](docs/windows-time-development-hardening.md#64-per-process-mitigation-qualification).
Keep per-process roles, activation phase, required/optional policy, effective state
and native evidence distinct; existing compiler flags prove no complete runtime profile.

**Required CPU and hardening policy, clarified 6 November 2026:** OD-07's Windows
product minimum is **x86-64-v3**, now selected in the target configuration and
exact admission controls. Do not restore generic x86-64/v2 or substitute v4,
`native` or ambient flags. OD-05 requires every applicable final-product
protection at the strongest supported compatible setting. Implement and qualify
it under the existing build/runtime/package owners; missing evidence is open
work, never permission to call the control optional. Record concrete platform,
toolchain or necessary-feature exclusions; David owns security tradeoffs and
weaker release exceptions. Follow [hardening §6.2–6.4](docs/windows-time-development-hardening.md#62-target-specific-windows-compilerlinker-settings)
and [the build task](tasks/build-optimization.md#required-v3-and-maximum-compatible-hardening--6-november-2026).
Historical generic receipts, pending Windows runs and hardware-conditional CET
do not change this requirement or establish a new CPU floor. Preserve the
post-V1 proof deferral and unrelated native/fuzz/storage/UI execution holds.

Shared OS/IPC work: read [architecture section 3.4](docs/windows-time-architecture.md#34-shared-windows-adapters-and-diagnostics). Reuse narrow role-specific `windows-platform` adapters and portable `ipc-contract`; shared code grants no privilege or generic Windows command surface.

Engine runtime/concurrency work: read [architecture section 18.6.1](docs/windows-time-architecture.md#1861-engine-runtime-decision-and-alternatives), section 17.1 and the TLS execution gate in section 18.7.1. Tokio remains an engine-only current-thread prototype; qualify bounded native work and the complete NTS-KE arrangement. Its selection does not require `tokio-rustls` or extend runtime dependencies to the core, broker, UI or helper.

IPC implementation/security work: read [architecture section 5.4](docs/windows-time-architecture.md#54-native-ipc-implementation-and-qualification) and [hardening section 9.9](docs/windows-time-development-hardening.md#99-ipc-identity-framing-and-native-lifetime-verification) alongside `security-local-ipc`. Qualify actual wrapper rights, thread-bound caller capture, fatal reversion, continuous endpoint ownership and partial/native I/O completion; examples and mocks do not prove Windows enforcement.

Protected optional-storage work: start at [the agent task](tasks/windows-protected-storage.md),
[architecture §12.1.2](docs/windows-time-architecture.md#1212-protected-optional-storage-and-explicit-fallback)
and [the storage plan](docs/plans/windows-protected-storage.md). The task maps the
[second upstream review](docs/plans/windows-protected-storage-upstream-lessons-2.md)
to bounded work and existing ST-T/CHECK/DNS-C owners. Preserve indeterminate
operation outcomes, actual native completion, privacy barriers and the separate
mandatory-state owner. Source examples authorize no dependency or weaker fallback;
RedirectionGuard and hardware rollback epochs retain their candidate gates. The
storage experiment pause and current product-work priority remain in force.

The coordinator adopts ST-W01–ST-W07 through the imported queue above at the next
checkpoint. ST-W01 is the ready source-only storage assignment. ST-W02 needs its
relevant frozen contracts and authorized slice; native work remains held. Apply
the mapped ST-W03–ST-W07 contracts before related Windows filesystem, CNG,
mitigation, IPC-completion or sensitive-memory/diagnostic implementation, even
when that adapter work precedes optional-cache integration. Record disposition
and next action in [the existing storage task](tasks/windows-protected-storage.md),
without repeating the upstream survey or creating another plan.

Windows 11 integration research: use [the source review](docs/plans/windows11-integration.md),
the [Microsoft Learn follow-up](docs/plans/windows11-integration-follow-up.md),
[the agent task](tasks/windows11-integration.md) and the
[existing-phase workstream](docs/plans/development-plan.md#85-windows-11-integration-research-and-qualification).
They map packet timestamp provenance, power/timer semantics, service/network
confinement, MSI maintenance, DNS ownership and background-resource candidates
to current CHECK/DNS-C/ST-T owners. Record configured versus effective Windows
state and distinguish source evidence from native qualification. Conditional
APIs, analyzer profiles and dependencies remain unselected unless adopted by
their current owner; MSIX direction is separately selected under amended OD-05.
the current product priority, experiment pause and UI-M0 deferrals remain.

At its next checkpoint the coordinator also adopts W11-W01–W11-W10 through
section 8.5, recording owner, ready/held/deferred disposition and next action in
the existing Windows 11 task. W11-W01 has [partial source preparation](tasks/windows11-integration.md#426-s21-timestamp-provenance-preparation--5-november-2026)
through the [engine integration card](tasks/engine-transport-pilot.md#s21-preparation--real-transport-to-an-admitted-time-measurement--5-november-2026);
complete its selected capture/mapping contract when the acquisition owner reaches
it, preserving the ordinary product queue and OD-10 first-release deferral. Other items enter at
their named phase or conditional trigger. Before related Windows integration,
apply the mapped contracts and controls while retaining the distinction between
required product behavior and optional API candidates. This handoff lifts no hold.

The follow-up supplies concrete callback/completion, ancillary-data, COM, MSI
permission, ETW-session and partial-file-replacement cases for those same owners.
Use its source discrepancies to qualify selected SDK/generated bindings, rather
than copying overview declarations. Service-managed storage, native cost hints
and SCM observers remain candidates; prerelease VBS claims and side-channel
settings remain conditional. Apply the relevant refinement before dependent
adapter work and record its disposition in the existing W11 or storage task.
Retain the current Windows execution stop, separate fuzz hold and storage pause.
At the next coordinator checkpoint reconcile the follow-up's refinements in the
[Windows task](tasks/windows11-integration.md#44-microsoft-learn-second-pass-integration--3-november-2026)
and [storage task](tasks/windows-protected-storage.md#65-microsoft-learn-follow-up-routing--3-november-2026).
Carry applicable cases into the existing W11/ST-W assignments before their
dependent adapters; record owner, disposition and next action in those tasks.
W11-W01 remains partially prepared and ST-W01 remains unexecuted; neither source
card permits a native probe or lifts the existing holds.

MSIX/Store hardening and delivery: read the [selected plan](docs/plans/windows-msix-store.md),
[architecture §19.4](docs/windows-time-architecture.md#194-future-complete-msix-gate)
and the [current task amendment](tasks/windows11-integration.md#48-owner-selected-hardened-msix--3-november-2026).
The 3 November OD-05 amendment selects one complete hardened MSIX for preparation;
David will try Store submission with services and reconsider delivery if refused.
W11-W10 owns this selected work; W11-W05 and existing release/security/storage
owners supply provisioning, mitigation and recovery evidence. Do not wait for
Store preapproval to prepare the candidate or repeat the general survey. Preserve
the current Core priority and all execution holds. The plan defines explicit
package integrity, sealed executable content, Store-compatible CIG and per-role
protections. Local test-signing and actual Store-delivered qualification are
separate stages; final Store bytes are checked after controlled submission and
before broad release. Store signing is neither helper authorization nor a passed
application-security gate. Retain MSI profiles as alternatives for a later owner
decision, not a prerequisite or automatic fallback. No split package, broader
service rights, assistant account/upload action or unqualified release follows.

Release support and incidents use [the release policy](docs/release-policy.md)
and [existing W11 task §4.21](tasks/windows11-integration.md#421-release-operations-integration--4-november-2026).
Keep first-stable support terms proposed until David decides them, and carry
account, containment, corrective-state and recovery rehearsals into the existing
P6/P7 owners. A written runbook lifts no execution hold or release blocker.
Follow [OD-05 §7.5](docs/nevrotick-owner-decisions.md#75-optional-enhanced-build-provenance--4-november-2026)
for [enhanced build provenance](docs/plans/release-provenance.md): attestations,
independent reproducible builds and a public proof bundle are optional, with no
new release blocker. Preserve the separately required custody, SBOM/licence,
package integrity, installed identity and native/lifecycle evidence.

Logging/verbosity work: read [architecture section 12.3](docs/windows-time-architecture.md#123-logging-and-support) and [hardening section 9.8](docs/windows-time-development-hardening.md#98-diagnostics-and-shared-adapter-verification). Keep per-process sinks, typed fields, required-event routing and locally expiring authorized DEBUG sessions explicit; diagnostics are not authoritative UI state or the clock journal.

Localization work: read [OD-15](docs/nevrotick-owner-decisions.md#19-od-15--externalized-ui-translations-and-binary-catalogs),
[the catalog design](docs/plans/ui-localization.md)
and architecture §16.4 before the Slint or i18n skills. The selected design uses
editable JSON → validated CBOR resources → typed Rust presentation → Slint and
tray/notification/accessibility consumers. Slint has no selected native CBOR
loader. Keep English fallback, stable codes/arguments and actual adopted-locale
scope. Current codec/formatter, generated resources and loader evidence belong
to S0.5 and the GUI handoff; arbitrary external user-import admission remains
unselected. The application-owned catalog is not a native Slint CBOR facility.
This supplies no native or Core execution waiver.

The current [Cargo workspace](Cargo.toml) contains
[nevrotick-ui](crates/nevrotick-ui/Cargo.toml): a checked portable presentation
model and the Slint Windows adapter, with native/backend qualification still open,
and [ntp-wire](crates/ntp-wire/Cargo.toml): the pure inert
wire owner under [S1.1a qualification](tasks/s1.1-header.md) and
[S1.1b framing qualification](tasks/s1.1-framing.md). It provides structural
framing without exchange, era, authentication or sample admission.
The [protocol task](tasks/s0.1-protocol-comparison.md) also records its inert
NTS-KE record processing, negotiation/request encoding and NTS envelope/plaintext
readers; these confer no TLS, AEAD or authenticated-source authority.
[time-domain](crates/time-domain/Cargo.toml) is the third member under
[S1.1c](tasks/s1.1-time-domain.md): checked numerical/era/clock-domain arithmetic,
with qualification evidence limited to the linked bounded scope and no network,
clock or authentication effects.
[ntp-exchange](crates/ntp-exchange/Cargo.toml) owns a single-slot, unauthenticated
fixture association under [S1.1d](tasks/s1.1-exchange-association.md) and the
[S1.1e scenario harness](tasks/s1.1-deterministic-harness.md).
[time-core](crates/time-core/Cargo.toml) owns pure regression/history,
selection/principal/supporter combination, reference readiness and modeled
correction accounting. [S1.2](tasks/s1.2-regression-readiness.md) and the
[current Core task](tasks/s1.3-source-selection-readiness.md) retain the exact
implementation and qualification scopes. The [settled-rate receipt](tasks/s1.3-source-selection-readiness.md#settled-rate-final-qualification--four-selected-owners)
records completed qualification within its four-owner Linux fixture scope; the current
continuation follows [development §7](docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent).
These scoped results do not establish production sample eligibility, an integrated
controller, clock authority or P1.
[nevrotick-engine](crates/nevrotick-engine/Cargo.toml) owns the engine-only
[bounded transport pilot](tasks/engine-transport-pilot.md): two explicit loopback
peers, a private current-thread runtime and the unauthenticated, ineligible
`nevrotick-observe` CLI. Its host experiment does not qualify production
acquisition, native Windows teardown, NTS, services or clock effects.
[windows-platform](crates/windows-platform/Cargo.toml) owns portable service
lifecycle/SCM/privilege contracts. Two fixed-role inert
[laboratory hosts](tools/windows-service-fixture/README.md) supply native SCM
adapter source outside the product workspace. For their next integration child,
reuse the [W11-W05/S3.1 handoff](tasks/windows11-integration.md#4358-service-source-handoff-and-integration-prerequisites--8-november-2026);
source availability and retained host checks do not establish qualified product
hosting, final identities/provisioning or native Windows evidence.
Other proposed components remain architecture plans until their source exists.
Inspect current manifests/source before choosing commands;
never scaffold product crates merely to pass documentation/tool checks.

## Current UI milestone

[OD-13](docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit)
selects Slint as the default UI toolkit and the complete Win32 UI replacement.
The [existing S0.5 task](tasks/s0.5-ui-comparison.md) owns the complete migration
against design v0.4; start with its current addenda and the
[Slint GUI handoff](tasks/slint-gui-handoff.md). The checked-in Windows entrypoint
now selects Slint; the unexported Win32 adapter is retained historical source.
Portable rendering, model tests and Windows-target checks retain their recorded
scopes; native/backend, accessibility/resources and independent critical-text
approval remain open. Historical Win32 receipts do not qualify Slint.

David approved GPL-3.0-only for the original GUI while Chrony-derived Core/engine
remains GPL-2.0-only. Do not reopen this choice or toolkit selection. Apply the
GUI-specific grant and exact runtime/build/asset provenance inventory; that
licence direction does not replace graph, composition, generated-code unsafe/lint,
security, native or package review. Source preparation and admitted fixture UI
work may parallel Core; retain one Cargo graph/schema owner and all execution holds.

Apply [architecture §16.5](docs/windows-time-architecture.md#165-restricted-first-ui-milestone-ui-m0)
before UI/QA skills. The original UI-M0 is an unelevated, fixture-only native
prototype; current S0.5 addenda own the complete GUI source scope. Within UI-M0,
**D1** theme/contrast/text/DPI work, **D2** production freshness/reconnect/policy/
administrative-result handling and **D3** full keyboard/Narrator/UIA work are
**Deferred**: preserve their existing task triggers and do not make them UI-M0
blockers. A later admitted source slice does not establish native qualification.
Retain toolkit-provided basic keyboard/editing behavior and meaningful labels,
Rust/FFI quality checks, sample-data labelling, disabled protected actions and
native window observations. Slint widgets inherit no Win32-control accessibility pass.
No live network/DNS, production IPC, persistence, services, elevation or clock
effects. Carry deferred owners/triggers into the backlog and next-stage plan;
restore them before the integrations/preview claims specified in §16.5.
UI-M0 does not complete P0 toolkit qualification or constrain other workstreams.

For UI work, use David's current [design v0.4 reference](docs/nevrotick-design-v0.4/README.md):
read its [guide](docs/nevrotick-design-v0.4/README-RU.md) and
[tray update](docs/nevrotick-design-v0.4/review/tray/REPORT-RU.md) and
[status/notification audit](docs/nevrotick-design-v0.4/review/status-notifications/REPORT-RU.md), then consult
the [interactive preview](docs/nevrotick-design-v0.4/NevroTick-UI-v0.4.html) for composition and
interaction references. Its dependency-free HTML sources and tests are in the same
bundle. [Design v0.2](docs/nevrotick-design-v0.2/README.md), its React viewer and
scene catalogue are historical; their scene IDs are not v0.4 links.
Use the [AVIF brand kit](docs/NevroTick-AVIF-kit-v1/README.md)
and its [manifest](docs/NevroTick-AVIF-kit-v1/manifest.json) for mascot, logos,
wordmarks, icons and background variants; preserve its asset notices separately
from the code licence. Apply these references within the milestone above. The
HTML previews and archived architecture copies are reference material; current
canonical contracts govern behavior. The richer v0.4 service/notification examples
are simulated and do not expand native UI-M0 or lift D1–D3 execution holds.
The archived React tooling has open dependency findings recorded in the
[UI-M0 task](tasks/ui-m0.md#publication-and-hosted-qualification). Resolve and verify
them before executing that tooling; the native Rust build does not use it.

## Architectural constraints

Read canonical sections for exact limits; preserve these boundaries:

- Separate network-facing engine without clock privilege, narrow privileged
  clock broker, normal-user Slint UI, elevated lifecycle helper. Deliver one
  complete MSIX under amended OD-05, subject to supported service provisioning
  and complete lifecycle qualification. Older MSI-first summaries are superseded;
  retain MSI-specific mechanics only for a later owner-selected alternative.
  Suggested crate boundaries propose ownership;
  scaffolding every crate immediately not required.
- Treat product user input and external programmatic input as potentially
  malicious. UI controls, local origin, authenticated transport, administrator
  identity or prior acceptance never establish payload safety. Bound and validate
  each applicable admission path, then independently check current authority at
  the effect. Preserve raw-input tests, direct-handler bypass cases, valid
  neighboring controls, aggregate budgets and bounded redacted rejection.
- Every JSON/CBOR input is potentially malicious at raw admission, including
  contributor/translator data, English, schemas/metadata, generated output and
  signed/packaged or previously accepted files. Reuse admitted immutable typed
  values only within their valid context. For localization, an independent
  code-owned contract/checker controls IDs, types, plural/action rules and semantic
  approval; catalogs cannot redefine them. Follow [OD-15's threat controls](docs/plans/ui-localization.md)
  for isolated conversion, bounded decode/format/render, literal sinks and
  whole critical-group English fallback. Origin/signatures and syntactic validity
  prove neither payload safety nor truthful translated consent text.
- Fixed JSON IPC/configuration/persisted-record boundaries: bounded strict Serde
  DTO decoding, adopted shared garde rules invoked inside fallible private
  domain construction, current trusted
  authorization/generation/budget checks. Serialize protected decisions/effects
  and actual acknowledgements. Never normalize through `serde_json::Value` to
  bypass strict admission or introduce second runtime schema engine.
- Every function/module boundary, including private helpers, has explicit input
  invariants, output guarantees, failure/effect and lifecycle obligations. Enforce
  them through typed signatures, private checked construction, checked
  transformations and current owner guards. Reuse immutable validated values;
  recheck changed representations, state and context. Never rely on optional
  caller validation, `debug_assert!`, public raw fields or unchecked mutations.
  Follow [contract enforcement](.agents/skills/implementation/implement-rust-contracts/references/contract-enforcement.md):
  no `garde(allow_unvalidated)`; each skipped field needs an actual invariant owner
  and independent review. Garde adoption does not qualify a product Cargo graph.
- Parse NTP/NTS with bounded protocol-specific byte parsers. Preserve interval
  majority selection/configured minimum agreement; no automatic single-peer
  profile after disagreement.
- Keep timestamp units and UTC, monotonic, active-awake clocks distinct. Positive
  offset means Windows behind. Estimator history reflects actual acknowledged
  applied correction, not merely requested correction.
- Preserve the existing NTS-only baseline while resolving OD-03's Pool/default-
  profile interaction; no unqualified shipped preset or silent trust downgrade.
  Plain-NTP profiles stay explicit and visibly labelled. Bootstrap hints support only bounded
  provisional TLS-clock recovery; authorize neither global clock writes nor
  authenticated-tracking claims.
- OD-02 makes very large authenticated recovery a qualification goal. Era/date,
  quorum and correction bounds remain unresolved; 12 hours is not an approved
  plain-NTP constant. Keep clock ownership, stale-state rejection, cumulative correction allowances,
  sleep/resume invalidation, W32Time handover/restoration explicit. Ordinary
  “Sync now” requests fresh bounded check; grants neither arbitrary clock
  correction nor administrative policy changes.
- Bound actual native work/resources. Dropping/aborting awaiting future does not
  establish native-job completion; permits/custody follow real operation through
  completion/recovery.
- Keep unsafe code in narrow reviewed Windows adapters, preserve useful errors,
  redact secrets from events/exports. UI status accurately distinguishes
  authentication, agreement, freshness, readiness, applied effects.
- Preserve Chrony-derived Core/engine GPL-2.0-only provenance and the original
  GUI's approved GPL-3.0-only grant. Qualify exact
  dependency/features/license graphs and held crypto-provider choices before
  adoption. Keep `Cargo.lock` versionable; exclude `target/` from Git/search indexes.

## Choose skills by the changed guarantee

Use [the skills index](.agents/skills/README.md); load only relevant leaves and
conditional references. Small understood edit needs concise action/check record,
no new report tree or orchestration campaign.

| Work | Start with |
| --- | --- |
| Uncertain scope, stages or ownership | [plan-code-change](.agents/skills/planning/plan-code-change/SKILL.md); architecture/threat/review skills when their decisions change. |
| Rust logic, concurrency or Windows adapters | [implement-rust](.agents/skills/implementation/implement-rust/SKILL.md), including conditional SCM/chrony/simulator references. |
| Windows service creation, SCM lifecycle, management, hardening, logging or optimization | [implement-windows-service](.agents/skills/implementation/implement-windows-service/SKILL.md), routing detailed service/recovery/access matrices and the relevant security/QA owner. |
| Function/module contracts, wire admission and domain invariants | [implement-rust-contracts](.agents/skills/implementation/implement-rust-contracts/SKILL.md). |
| Wire, persisted-state or semantic compatibility | [evolve-data-contracts](.agents/skills/implementation/evolve-data-contracts/SKILL.md). |
| Native UI and accessibility | [implement-native-ui](.agents/skills/implementation/implement-native-ui/SKILL.md), [implement-slint](.agents/skills/implementation/implement-slint/SKILL.md) for toolkit mechanisms, then applicable accessibility/DPI checks. |
| Trust, privileges, IPC, crypto, logging or packaging | [Security index](.agents/skills/security/README.md), matching boundary owner. |
| Windows product/package/Store metadata | [maintain-windows-metadata](.agents/skills/implementation/maintain-windows-metadata/SKILL.md); package QA retains artifact/native qualification. |
| Cargo dependency changes | [dependency-updates](.agents/skills/environment/dependency-updates/SKILL.md). |
| Windows Sandbox configuration, WSL-to-guest control or disposable lab evidence | [manage-windows-sandbox](.agents/skills/environment/manage-windows-sandbox/SKILL.md); check current task/incident holds before any Windows command. |
| Verification, failure triage or commit preparation | [Testing index](.agents/skills/testing/README.md), [testing contract](.agents/skills/testing/CONTRACT.md); smallest relevant evidence layer. |

Use [upstream specialists](.agents/skills/upstream/README.md) for actual
compiler/borrow/Send-Sync problems or qualified Windows UI automation. Skills hold
reusable instructions, references, templates, checkers, required inventory/licence
provenance. Keep one-off reviews/logs/task receipts in existing task location,
outside skills pack.

## Tool selection

Resolve absolute checkout root with `git rev-parse --show-toplevel`; pass
explicitly to project-aware tools. Inspect exposed tool schema before use;
historical setup/version receipts in [TOOLING.md](TOOLING.md) provide context,
not current-connection guarantees. Choose tool for current question; ordinary
edits need not use every tool.

### Serena: exact Rust symbols and safe symbol edits

Once Rust source exists: definitions, implementations, references, diagnostics,
symbol-aware edits. Call `initial_instructions` first, then `activate_project`:
`project` = absolute checkout; `session` = returned **Serena session ID**, not
MCP/client ID. Use symbol overview, definition/body, referencing-symbol queries
as needed. Retrieve body before replacement; inspect affected callers.
`.serena/project.yml` selects Rust; restart Serena connection after project
configuration changes. Empty application symbol graph before source expected.

### SocratiCode: discover owners and retrieve architectural context

Semantic search for unclear owner/terminology; relationship queries for cross-file
impact. Pass absolute `projectPath`; check `codebase_status`, wait for completed
indexing before semantic code search. List `codebase_context`, then
`codebase_context_search` with actual `artifactName` from
[.socraticodecontextartifacts.json](.socraticodecontextartifacts.json).
Read canonical file directly if unregistered/unindexed. Respect configured watcher
mode; `codebase_update` for necessary immediate catch-up. Confirm consequential
results in current files/callers; index locates evidence.

### Probe: fast source search without persistent indexing

Use `search_code` for Rust keywords, identifiers, syntax-aware discovery; then
`extract_code` for complete definitions with absolute project/file paths.
Enable `allowTests: true` for test extraction; default excludes tests. Keep
gitignore filtering enabled to exclude generated Cargo output. Documentation/
configuration literals: Probe `grep` or `rg`.

### Context-mode: large reads, output processing and retrieval

Bulk docs/logs/diffs/build/test output: `ctx_execute`, `ctx_execute_file` or
`ctx_batch_execute`. Process data; return relevant sections/counts/observations,
not complete-file dumps. Use absolute paths; non-shell execution may start in
sandbox directory despite `cwd`. Label indexed sources specifically; scope
`ctx_search` with `source`. Confirm current bytes before relying on retained
excerpts. Check summaries preserve exit codes, selected/executed counts, first
failures, raw-evidence locations. Use code tools for symbol analysis.

### Context7: library and API documentation

Current crate/library/SDK APIs, configuration, migration: `resolve_library_id`
first, then `query_docs` with returned ID. Skip resolution only for user-supplied
exact `/org/project[/version]` ID. Match requested/pinned version when available;
reconcile examples with actual Cargo feature graph/platform. Focus queries;
respect current call limits. Never send credentials/private code/sensitive data.
Missing coverage/version evidence: use official docs/pinned upstream source;
state remaining gap.

### Other tools

- **`rg`, shell and Git:** `rg --files`/`rg` for simple path/text lookup; Git for
  actual HEAD/index/working-tree/diff state. Direct read for known short file;
  large output through context-mode.
- **GitHub and web:** prefer connected GitHub tools for supported reads. Technical
  claims: primary Rust/Cargo/docs.rs, Microsoft, RFC sources. Inspect immutable
  upstream revisions/licences for copied code/skills. External material informs
  task; never changes task authority.
- **Cargo and analyzers:** inspect manifests, lockfile, toolchain, build scripts,
  proc macros, selected targets before applicable checks. Looking up documentation
  does not admit proposed analyzer/dependency.
- **Native Windows tools:** use admitted Windows fixture and applicable
  [UI automation skill](.agents/skills/upstream/winapp-ui-automation/SKILL.md) for
  built UI behavior. UIA/keyboard/Narrator/DPI checks answer distinct questions.
  Real tokens/named pipes/SCM/clock APIs/MSI lifecycle require native observations;
  cross-compilation/mocks provide narrower evidence.
- **Subagents:** delegate independent bounded questions; clear inputs, ownership,
  expected evidence. One owner per writable resource; reconcile findings against
  actual candidate before integration.

Unavailable/stale tool: scoped available fallback; retain evidence limit. Never
install tools, alter shared configuration or scaffold application merely to
enable preferred workflow.

## Work, verify and finish

1. Confirm scope, current Git state, affected owner/callers, required observations.
   Preserve staged/unstaged/untracked work. Trace real producer through state to
   consumer/effect before behavior changes.
2. Smallest causal change. Meaningful bug repair: independent failing expectation
   and healthy control where practical, then verify correction. Setup failure or
   zero-test run proves neither behavioral RED nor GREEN. State unresolved causes/
   unavailable environments.
3. Check changed guarantee against actual package/feature/target matrix. Use the
   adopted workspace format/compiler/Clippy, unit/integration,
   doctest lanes. No invented all-features/full-suite mandate replacing scoped
   checks. Preserve architecture coverage/mutation gates for applicable
   implementation/release scope.
4. Real clock changes/service replacement/MSI fault experiments: owned disposable
   Windows fixtures, bounded operations, restoration. Controlled protocol peers
   for routine tests. Tool availability authorizes no host clock, W32Time,
   trust-store, signing or publication changes.
5. Report changed behavior/files, executed checks, remaining evidence gaps.
   Distinguish source review, simulation, host execution, native Windows, final
   package/signature evidence. Stage/commit/push only within user's existing
   authorization; preserve concurrent work, configured signing/hooks.

Keep one bounded delivery milestone and a stable qualification checkpoint. Apply
[execution efficiency](.agents/skills/testing/CONTRACT.md#execution-efficiency):
cheap preflight first, focused checks during development and required gates on
the final candidate. Optimize measured admission/I/O overhead while preserving
configured custody checks; reuse existing plans and recipes. Report observable
milestones and timings; label estimated percentages with their scope and basis.

Skill-pack changes: run package checker from repository root:

```sh
python3 -B .agents/skills/scripts/check_skills.py
```

Run `python3 -B .agents/skills/scripts/test_check_skills.py -v` when changing
checker/covered contracts. Retain upstream inventory hashes/notices when
intentionally changing imported leaves.

Documentation changes: check links/canonical consistency. Verify
`docs/SHA256SUMS.txt` **when present**; update existing manifest when intentionally
changing covered documents. Never claim bundle integrity from missing/stale
manifest. Keep transient plans/reports outside maintained documentation bundle
unless task explicitly places them there.

# UI-M0 implementation task

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


## Current UI direction — integrated Slint continuation

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

This UI-M0 record retains **Win32 implementation and exact-candidate history**,
plus the separately recorded HTML import and main's 5 November migration queue.
Its unexecuted full AppVerifier/PageHeap gap remains on that old Win32 candidate;
Slint needs its own reached-native-path, lifecycle, cleanup and resource evidence.
M0-01–M0-08, sample labelling, shared checked editing and disabled protected effects
carry into the fixture GUI. D1/D3 return before general-user preview/release or a
corresponding claim; D2 returns before live status and real settings/commands.
The pilot's omitted accessibility feature is no product exemption. Existing
native/fuzz/storage holds and scoped Sandbox admission remain binding.

## Scope and custody

Authorized by David's “Do it” after the agent-readiness recommendations on
29 October 2026. Starting candidate: clean `e0c8516db9d1b284eb410bc2c8bd548c103a35fb`.
Architecture §16.5 and M0-01–M0-08 control acceptance. This task record is outside
the maintained documentation bundle.

Retained Win32 reference status: the coding foundation and functional prototype
are implemented and observed. The successor Slint fixture GUI is implemented in
the separate S0.5 evidence chain; merged-candidate and native qualification remain
open. Required full native-memory instrumentation for the retained Win32 candidate remains incomplete;
the narrower debugger and functional results below do not close that gap.
The latest UI-M0 Win32 candidate and results are in the
[design-reference adaptation](#design-reference-adaptation-29-october-2026) below.

Deliver a fixture-only, unelevated native Windows UI, its minimal real Cargo
workspace, shared verification command and applicable CI. Align GitHub's existing
security gate with the deliberate hosted CodeQL deferral. David subsequently
authorized committing and pushing the completed work on 29 October 2026. That
authorization does not change services/clock/trust stores, add production
IPC/network/persistence, or establish P0/P5/release qualification. Only require
a new hosted Rust gate after that check is available on the branch.

## Ownership and paths

The table describes the historical Win32 reference. The current Slint adapter,
generated-code and input/text/native owners are recorded in the GUI handoff;
the retained M0-SL-02 table describes their earlier preparation. Old path names
grant no Slint acceptance or automatic module layout.

| Owner | Writable scope | Input → output |
| --- | --- | --- |
| Integration/dependencies/domain | Workspace/manifests/lock, graph policy/acquisition, `crates/nevrotick-ui/src/model.rs`, library/main, this record and setup documentation | Pinned dependency sources → admitted graph; raw local preview fields → private checked values → in-memory preview |
| Native UI | `crates/nevrotick-ui/src/native.rs`, optional `src/native/`, `scripts/ui-m0-windows.ps1` | Fixtures/checked preview → standard Win32 controls; local editor → shared checked model; close → owned resource release |
| Verification | `scripts/verify.py`, `scripts/tests/test_verify.py`, `.github/workflows/rust.yml` | Exact source/graph/tool identity → bounded isolated selected checks and explicit outcome |
| Independent reviewer | Read-only plan and candidate review | Canonical requirements and separate acceptance expectations → findings |

Serialize Cargo graph changes, shared build outputs and desktop interaction.
Use an external Windows-drive cache/scratch for build artifacts; preserve all
unrelated files/settings. No shared compiler/configuration changes.

## Acceptance and checks

- M0-01: real Windows build and applicable format/compiler/Clippy/documentation,
  portable model tests, dependency and policy-inheritance checks.
- M0-02/03: native unelevated launch, reference display recorded, all three pages,
  persistent “Prototype — sample data”, resizing, minimize/restore and Close.
- M0-04: valid Update preview, invalid edit preserving accepted state, Cancel
  discarding unaccepted control text, restart resetting to fixtures. Direct model
  calls challenge admission independently of control limits. External compile-fail
  and legal-use controls challenge checked-construction bypass.
- M0-05: disabled protected actions with explanations; sample identities and
  truthful sample status/units; no real administrative or synchronization result.
- M0-06: inspect actual source and configuration for fixture-only data and no
  transport or privileged effect paths; supplement with native lifecycle evidence.
- M0-07: independent ownership/lifetime/error-handling review of the narrow FFI
  adapter; preserve real failures and skipped observations.
- M0-08: identify exact inputs, candidate, environment, results, gaps and deferrals.
  Observe USER/GDI/kernel-handle resource counts during repeated use before process
  exit; exit alone cannot establish cleanup.
- Select corruption diagnostics for reached FFI paths. Record any unavailable
  elevated per-image verifier coverage explicitly; no automatic elevation or
  machine-wide configuration follows from this task.

Portable preview field limits are scoped to this prototype, not adopted production
protocol limits: label 1–64 printable ASCII characters without edge spaces;
host 1–253 ASCII characters, DNS labels up to 63 or an IP literal, no URI/path;
port 1–65535, 1–5 decimal digits. All construction invokes shared garde rules;
errors are bounded static messages and invalid replacements preserve state.

## Deferred work and re-entry

UI owner retains D1 (themes/contrast/text/DPI) and D3 (full keyboard/Narrator/UIA)
before general-user preview/release or earlier corresponding claims. D2 freshness,
reconnect/generations returns before live status; policy/authorization/result
contracts return before real settings or protected commands. All remain Deferred.
The disposable service/clock/MSI laboratory remains a later prerequisite.

### Network settings handoff — 5 November 2026

Before real settings/P5 re-entry, consume the planned
[Network connection controls](../docs/windows-time-features.md#331-time-network-settings--5-november-2026)
and the engine's [network-profile card](engine-transport-pilot.md#network-path-history-and-user-routing--5-november-2026).
Under [OD-11](../docs/nevrotick-owner-decisions.md#15-od-11--socks5-scope-and-deferral),
first Windows delivery exposes Direct interface/family controls. Every proxy
control and proxy-specific case below is deferred to
[SOCKS-0](../ROADMAP.md#socks-0--optional-socks5-time-acquisition); authentication
and proxy DNS require later extensions beyond its first loopback/no-auth/local-DNS
subset. Do not enable a proxy editor or discard unsupported imported proxy state
into Direct. These cases are future acceptance, not new UI-M0/P5 blockers.
Provide a reachable Settings entry and source-detail link; preserve requested
versus effective state, hard versus preferred family policy, direct/proxy leg
labels, resolver choice and unavailable-adapter/UDP-relay remedies. A local
proxy's onward interface and a proxy-resolved target family are not client-known
facts. Reject unsupported combinations through the shared domain/IPC owner,
including raw-handler bypass cases, and apply only an authorized current revision.

Keep saved policy and effective acquisition revision separate through pending,
blocked and successful activation; stale completions cannot update current UI.
Use the existing machine-policy/secret/history owners, not per-user network
configuration or direct Q access. Explain cold path learning after initial
service restart, unavailable optional history and missing proxy credentials
without promising durable cache support or silent direct/no-authentication fallback.

This is a full-product handoff under F-06/07/09/10 and AC-21/22, not an implemented
widget or M0 pass. UI-M0 retains deterministic fixtures, no live probes/settings
effects and its existing D1–D3 deferrals. No HTML prototype or branding asset was
changed by this documentation amendment.

## Windows component reuse handoff — 5 November 2026

**Source-only continuation:** read
[architecture §18.6.2](../docs/windows-time-architecture.md#1862-windows-component-reuse-and-binding-coverage).
The inspected UI at `e82f3f6b352fde88beceece4d3084862d8778338` already uses
`windows-sys` 0.61.2. Retain that Win32 adapter as reference evidence while
OD-13 and M0-SL-01–05 govern its Slint successor. Preserve M0-07/M0-08 lifecycle
contract. `windows-window` covers the top-level window/loop, not controls or
layout; its inspected `run()` does not expose the `GetMessageW` -1 error branch
that this adapter distinguishes. This is a source-fit concern, not a reproduced
crash or permission to replace/refactor the current UI.

Before a later window-wrapper replacement, the UI owner must show a concrete
maintenance benefit and preserve error/quit/message distinctions, borrowed HWND
and callback state through destruction/reentrancy, close-versus-tray behavior
and relevant USER/GDI/handle observations. Follow
[the conditional toolkit record](s0.5-ui-comparison.md#conditional-windows-toolkit-candidates--5-november-2026)
for retained Reactor observations and any bounded revisit of selected Slint
delivery. This source handoff does not select another toolkit/runtime or put a
`windows-window` loop around Slint.

At S5.3/F-09 export implementation, the UI/dependency owner first rechecks a
working published picker package and its exact source/graph. The inspected
`windows-pickers` index contains only 0.0.0; useful API in main does not qualify
a Cargo dependency. A selected OS dialog must preserve cancellation versus
failure, UI-thread/apartment/owner lifetime and bounded ordinary-UI output.
Pair a valid redacted export with cancellation, denied/unavailable destination
and stale/replaced-path cases; never forward a caller-selected filename to a
privileged writer. The dialog's returned path is not an already opened safe file.

**Disposition:** deferred at existing S0.5/P5 triggers and CHECK-17/21/22/26/27
owners. No new UI-M0 acceptance, live export/persistence, dependency or native
experiment is introduced; D1/D2/D3 and current execution holds remain intact.

## Implementation and evidence

The one-member workspace contains `nevrotick-ui` and its private checked preview
model. No other product crate is scaffolded. The exact sixteen external packages,
features, archive/file identities, selected license grants and three build scripts
are admitted through `policy/ui-m0-dependencies.json`; the GPL-only bootstrap
`deny.toml` remains unchanged. Dependency build scripts and proc macros execute
inside the shared verifier's isolated build environment with read-only source.

Evidence is retained outside the checkout under
`%LOCALAPPDATA%\NevroTick\ui-m0` (WSL path
`/mnt/c/Users/perso/AppData/Local/NevroTick/ui-m0`). This is local evidence, not
a published CI result. This evidence was captured uncommitted on the starting HEAD;
the verification receipts record hashes of dirty and untracked source inputs.

| Evidence | Result and limit |
| --- | --- |
| Independent plan, dependency and model review | Direct-handler/bypass expectations, immutable checked construction, exact source/selected licenses and finite scope reviewed. Findings corrected and rechecked. |
| `dependency-check-6.json` | Linux and Windows advisory, license, source and build-script checks passed against the recorded frozen snapshot. Advisory freshness expires; this is not a permanent clean bill. |
| `portable-check-5.json` | Final source: format/compiler/Clippy/rustdoc passed; five integration tests and three doctests executed. Empty/skipped required selections fail. |
| Verification/acquisition controls | 56 new Python controls passed; 24 existing security controls and workflow validation passed. |
| Workspace/docs/skills | Manifest policy inheritance, 58-skill package check, relevant local links, all three existing checksum manifests and whitespace checks passed. |
| `windows-check-5.json` | Corrected native source passed Windows compiler, all-target Clippy and linking. `windows-candidate-5/pe-inspection.json` confirms AMD64 GUI and emitted CFG/ASLR/DEP flags; this is not a complete runtime profile. |
| `native-observations-5-final/observations.json` | Corrected candidate passed unelevated launch, caption/sample-banner control, editor rejection/update/Cancel, programmatic oversize bypass, mouse selection/navigation, resize, minimize/restore, disabled actions, normal Close and fixture reset after restart. Six screenshots retained; root independently inspected Status, invalid edit, resized Sources and Settings. |
| `native-cdb-2/observation.json` and raw `cdb.log` | Ordinary-user debugger and target reached creation/message loop, invalid/valid editing, navigation and normal Close. Actual access-violation, breakpoint, invalid-handle and security-check filters were inspected; no exact selected fault markers occurred. Both processes exited zero and stopped. Full AppVerifier/PageHeap was not enabled. |
| Final independent native review | M0-07 review passed on the matching final source, binary and helper: no unresolved material ownership, lifetime or error-handling finding. The reviewer also inspected caption RED/GREEN controls, screenshots, resource samples and raw CDB results. |

The dependency row first rejected incorrectly formatted build-script exceptions;
the exact package-spec correction passed with the neighboring permitted graph.
The portable row exposed an unwritable default nextest store; the snapshot-only
store correction passed without relaxing read-only source custody. The first
desktop observation stopped because foreground activation failed; it is retained
as incomplete, not a product failure. Screenshot review then found a blank native
caption; the independent caption expectation and causal repair are tracked with
the Windows receipts. `native-observations-4-caption-red` pairs the missing-caption
failure with a valid sample-banner control. Successful `WM_NCCREATE` association
now preserves default Windows initialization. `windows-check-4.json` records an
environment allocation error before compilation; the same SDK inventory and
complete Windows row then passed in `windows-check-5.json` without relaxed checks.

The initial implementation candidate is `windows-candidate-5/nevrotick-ui.exe`, SHA-256
`6345d34e0273d73ac62ff422d645a9fe99a03c4baa95de0afc7a5b4d2e569469`.
Portable and Windows receipt source hashes matched the implementation at that
initial checkpoint.
Only native callback code changed after the dependency receipt; manifests,
lockfile, dependency policy, vendored sources and checker stayed unchanged.

Initial implementation's native helper SHA-256:
`5ae2f032e002ddb89212c67c3e3c56e469ad1417e39b310585bf33a57b90fe6d`.
The receipt checks both helper and executable bytes before/after the run. The
reference environment was Windows `10.0.26300.0`, ordinary user, light theme,
144 system DPI (150%); this unscaled prototype reported 96 window DPI and Windows
virtualized it. Explicit text-scale override was unavailable. The observed window
resized from 1350×1020 to 1230×960 physical pixels and back; these observations
do not qualify the deferred display matrix. After warmup, all five samples across
20 navigation/Cancel cycles remained at 192 handles, 9 GDI objects, 35 USER objects
and 2,387,968 private bytes before Close. This is bounded stability evidence, not
a sustained leak or soak result.

Actual final row invocations used `python3 -B scripts/verify.py --lane portable`
and `--lane windows`, each with
`--vendor-dir /tmp/nevrotick-ui-m0-inputs-rabl89fi/vendor` and its named external
JSON receipt. Windows additionally used the new external `windows-candidate-5`
output directory. The native observation used ordinary PowerShell `-NoProfile
-File` with the byte-identical `native-harness-5/ui-m0-windows.ps1`, explicit final
image path and new `native-observations-5-final` directory. The durable reviewed
vendor and default pointer remain on the Windows drive; the temporary Linux copy
only avoids DrvFS per-file overhead.

The first CDB observer stopped on a shared-log read error before its required
target/filter markers; `native-cdb-1` remains incomplete. The corrected reader
used bounded shared access; the final observer and debugger-command hashes are
retained in `native-cdb-2/observation.json`. Its raw log preserves a skipped SDK
`I3cKd.natvis` visualizer warning and an ambient Nahimic audio DLL load. These are
host/tooling observations, not product faults; the session establishes no isolated
module set. Neither diagnostic run changed elevated per-image configuration.

`final-source-inventory.json` records the final HEAD/index, all changed and
untracked file identities, observation receipts and final candidate/helper hashes.

The new `rust.yml` acquires/admit-checks exact inputs, checks the frozen dependency
graph, then runs the same portable verifier. At this implementation checkpoint it
had not run on GitHub; publication requires observing the actual hosted result.
Live rules now retain signed/linear-history and deletion/force-push protection,
require the existing **Security gate** on `main`, and remove the obsolete skipped
CodeQL requirement. Ruleset IDs: `26069359` (existing protection), `26110199`
(main checks). Prior bypass actors are unchanged. The new Rust check must be
observed successfully after publication before making it required.

## Remaining evidence and next work

### Slint migration handoff — 5 November 2026

**Historical main planning queue:** the table and next-action wording below
record main before this branch's fixture GUI implementation. The current
[S0.5 goal](s0.5-ui-comparison.md#active-goal--finish-the-complete-slint-gui)
and [GUI handoff](slint-gui-handoff.md) supersede completed source stages. Keep
the original row identities, research concerns and still-open native/product
gates; do not rerun completed stages or transfer branch receipts to this merge.

**Owner:** UI with dependency/security, accessibility and Windows QA owners.
**Historical disposition:** Slint selection accepted; migration not started on
that dated main candidate. The product continuation in development plan §7 remains the primary task.
This update defines work; it does not execute a Rust migration or native campaign.

M0-SL-01–M0-SL-05 are **task-local work labels**, not replacements for canonical
M0/CHECK IDs. Ready means the named preparation can run when the UI owner takes
this work; it is not a completed result, admitted product dependency or Windows
permission. Keep one candidate/evidence chain here and in S0.5. M0-SL-01 preparation may
finish with an unresolved blocker; that records **admission held**, not a pass
that unblocks M0-SL-03.

The [Slint/iced assessment](../docs/supporting-reviews/slint-iced-assessment-2026-11-05.md)
records David's subsequent reaffirmation of Slint and the source-review inputs
for this same queue. M0-SL-01/02 must disposition its unconfirmed SR-1 generic
weak-handle and SR-2 future-destruction concerns against the exact admitted
version and reachable application paths. These are local research labels, not
confirmed product defects or new canonical gates; owner selection does not
resolve them. The report's activity figures and private idle results grant no
licence, generated-code, native or accessibility pass.

| Work item / role owner | Current state and predecessors | Concrete output and observable exit |
| --- | --- | --- |
| **M0-SL-01 — exact graph and licence route** / dependency/security with UI | **Ready: source-only preparation.** Use OD-13 and the retained Slint 1.18.1 winit/software/Fluent pilot inputs; do not reopen general toolkit selection. Product graph adoption remains held. | Inventory exact runtime/build/proc-macro/native dependencies, selected grants/features and Windows/runtime payloads. Resolve the Slint and Apache-2.0-only winit questions against retained GPL-2.0-only obligations; produce an admitted route or a concrete OD-04 blocker/card. Freeze the candidate graph/build inputs before Cargo adoption. OPEN-1/13, CHECK-20/26; no clearance or paid agreement may be inferred. |
| **M0-SL-02 — safety, input and lifecycle contract** / UI/Rust-contracts with security/reviewer | **Ready: source-only preparation**, using the retained candidate; recheck the result against M0-SL-01's final graph. | Map the real fixture → widget → callback → shared checked model → view path and module/write ownership. Inventory generated unsafe code and resolve its `allow` versus workspace `forbid` conflict without a silent policy relaxation. Specify UI-thread affinity, weak handles, editor/raw-field and aggregate bounds, error preservation, last-window/event-loop shutdown and actual resource release. Independently review these source-bound contracts, including allocation before handler guards. M0-04/06/07/08. |
| **M0-SL-03 — fixture adapter migration** / UI with domain/build owners | **Held** until positive product graph/licence admission from M0-SL-01, the reviewed M0-SL-02 contract, and activation of the UI implementation slice under current priorities. | Implement the selected Slint adapter in the real UI crate, reusing the checked preview model. Status/Sources/Settings, sample labels, source details, checked Update preview/Cancel and disabled protected actions preserve the existing fixture semantics. No network/DNS/IPC/persistence/service/elevation/clock paths enter the slice. Freeze source, generated code, graph/configuration and executable identity; keep the Win32 reference receipts unchanged. M0-01/03/04/05/06/07. |
| **M0-SL-04 — compiler, contracts and preview behavior** / Rust/contract QA with independent reviewer | **Held** on the frozen M0-SL-03 candidate and admitted graph/check inputs. | Run applicable configured format/compiler/Clippy/docs/dependency checks. Exercise actual model/adapter test seams admitted by the test graph: valid neighboring edit accepted, invalid/oversized and direct-call bypass rejected with prior state preserved, Cancel discards unaccepted text, fresh model construction resets fixtures; actual Windows GUI editing/restart remains M0-SL-05; retain legal-use controls for compile-fail claims. Review generated/native ownership and useful redacted errors. Record exact executed/failed/skipped scope; source annotations alone do not pass. M0-01/04/05/06/07/08. |
| **M0-SL-05 — native UI-M0 and resource checkpoint** / Windows QA/UI with independent reviewer | **Held** on M0-SL-04, an admitted Windows candidate/fixture and the applicable execution authorization. Earlier live-host pilot permission/results qualify only that bounded experiment. | Observe ordinary token, all three pages/sample labels, valid/rejected editor controls, Cancel/restart, resizing, minimize/restore and normal close. Measure private/working-set bytes, CPU and USER/GDI/kernel handles through repeated use and before exit, with actual cleanup/retention and reached native-memory coverage recorded. Freeze workload/duration/repeats, controls, helper and cleanup before execution. Record remaining D1/D2/D3 and CHECK-21/22 gaps rather than claim full feasibility or release. M0-02/03/04/05/06/08; CHECK-21/22 subset only. |

#### Next UI action — M0-SL-01

Inspect the already retained pilot manifests/lockfile, exact upstream grants,
Windows selected-feature tree and native payload inventory. The independent
expected outcome is a precise permitted product graph or a reproducible,
bounded licence/admission blocker with its dependent action identified. Record
that result here; a framework-level grant or process split alone cannot close it.
This first preparation writes only the existing task/admission record, performs
no Windows command and adds no dependency or licence-policy exception. In parallel,
M0-SL-02 may prepare contracts from the same pinned source, then reconcile any
M0-SL-01 graph change before its own exit.

The private pilot is reference input, not production code to copy wholesale:
its raw widget allocations, omitted accessibility feature and generated-unsafe
accommodation still need the named owners above. Retain shared mandatory garde
construction and UI-M0 field/domain rules. This migration changes no numerical
algorithm by design; if a conversion or mathematical contract does change,
record its actual-source proof obligation with the existing OD-10/CHECK-7/8/25
owner for after V1; it is not a first-version acceptance blocker. Retain checked
conversions and independent behavioral/boundary checks.

#### Observation mechanism — source preparation in M0-SL-02

The existing `scripts/ui-m0-windows.ps1` addresses Win32 child-control IDs through
`GetDlgItem` and native text messages. Slint-drawn widgets do not supply that
control model. M0-SL-02 must explicitly adapt or replace those field-interaction
and expected-output observations before M0-SL-05; do not reuse the old helper and
assume it reached Slint callbacks. Generic owned PID/start-time/image/HWND,
geometry, capture and resource-custody checks can be reused after revalidation.
Choose a supported, bounded toolkit observation mechanism or an owned-window
manual interaction record; either must exercise the real editor callbacks and
inspect visible/persisted-in-memory results against independent input facts.
A direct model call or screenshot alone cannot establish the whole journey.
Do not restore full D3 or enable a new accessibility feature solely to pretend
the old Win32 automation passed. Record unavailable native observations as gaps.

#### Downstream handoff and deferred work

- **S0.5 / CHECK-22:** full feasibility remains deferred beyond UI-M0. Consume
  M0-SL-01/02/05 receipts, then restore the relevant keyboard/UIA/Narrator,
  contrast/text/DPI, elevation and optional-tray observations in its own scope.
  M0 passes and the private idle experiment cannot close full S0.5.
- **P5 S5.1–S5.4:** remain deferred in the grouped backlog. S5.1 restores D2 before
  live read-only status; S5.2 before policy/protected effects; S5.3 owns actual
  tray/export/close-versus-service behavior; S5.4 restores D1/D3 and qualifies
  the exact Slint accessibility feature/bridge and complete resource workload.
  Accessibility disabled in the pilot is not a product policy.
- **W11-W09:** receives the exact Slint/backend/renderer and native resource and
  power/notification obligations; expand only at its existing trigger.
  **W11-W05/W11-W10 and P6 S6.1–S6.4** receive final UI executable/runtime/asset
  inventory, licence/SBOM, per-role mitigations, activation and install/update/
  repair/removal obligations. A standalone pilot EXE supplies no MSIX/Store pass.

### OD-13 documentation checkpoint — 5 November 2026

Slint default selection is recorded in NV-DEC-001 §17 and amended DEC-3, and
propagated through the architecture, entry points, feature/hardening/diagram and
research guides, development/MSIX/Windows plans, current design guidance,
UI task/backlog records and relevant skills. Historical measurements and archived
source copies remain identifiable as historical; the checked-in Win32 adapter
has not changed. The migration handoff above owns the next UI implementation.

Documentation verification passed: the 63-skill metadata/link/inventory checker,
all 25 checker regression tests, added/changed local Markdown target/anchor checks
and `git diff --check`. Independent review's remaining inherited-control and
historical-default wording issues were corrected and rechecked. These are
documentation/skill results, not a Slint product, native or release qualification.
No Rust/Cargo change or native command was part of this decision update;
unrelated concurrent formal-verification work remains in place.

### Slint task-queue checkpoint — 5 November 2026

The four-step OD-13 documentation handoff is now M0-SL-01–05 with owners,
ready/held states, predecessor gates and existing M0/CHECK exits. Source-only
preparation is the next UI action; the current Core continuation is unchanged.
The task index, S0.5 disposition, grouped P5 backlog and actual W11-W09/P6 rows
consume this same chain. Added observation-method preparation explicitly replaces
the Win32 child-control assumptions, while preserving D1–D3 and every role's
package mitigation obligations.

A fresh-context read-only `slint_task_gap_review` completed the source-to-task gap
review and the five-file recheck. Finding SL-TASK-01 narrowed S6.1 to UI-only
protections; the row was repaired to retain engine/broker/helper/UI scope, and
the reviewer closed that finding. Its attributed comprehension paragraphs and
returned rechecks are retained at `/tmp/nevrotick-slint-task-sync-av7ogx5w/review-result.txt`.
All local links in the five changed task records and changed-task whitespace
checks passed. The Git index stayed unchanged. This task update ran no Rust,
Cargo, native Windows or product qualification and preserves concurrent work.

### Slint implementation-skill research — 5 November 2026

Added [implement-slint](../.agents/skills/implementation/implement-slint/SKILL.md)
and six conditional references alongside the native-UI skill. The requested
dedicated path was absent from inspected base `5e98c79901c90de77db6c6047cb483ec10b8cbd5`;
it is registered as an original local workflow, with no imported implementation.
The skill index, native-UI route and agent guide now make it discoverable.

Three research lanes used GitHub, Context7, Microsoft Learn, Exa and Firecrawl,
with primary web documentation and coordinator source checks. Slint 1.18.1 at
`372cf0ee5577c3dfec309a45e7b778ba4e81b734` is the inspected upstream baseline,
not an admitted product graph. DeepWiki and Firecrawl's separate GitHub-research
endpoint returned tool-availability errors; no findings rely on those calls.

The references support M0-SL-01/02 with exact runtime/build feature and licence
separation, generated unsafe/lint ownership, precision-safe presentation,
binding/draft semantics, model notifications/identity, UI-thread delivery and
lifecycle, asset derivation, supported editor/viewer tools and bounded testing.
They record the AVIF encoder/decoder discrepancy and later tray broadcast/recovery
concern with source links and explicit limits. Ordinary no-pixel tests, viewer
rendering, optional developer MCP, native Windows and package evidence remain
distinct. Examples are mapped to current owners; no example activates later work.

Static verification: the package checker passes **64 skills**, including metadata,
UI prompts, local Markdown links and inventories; changed-document link/anchor
and whitespace checks pass. Pinned external GitHub targets were checked against
complete upstream trees. The coordinator read the integrated references and
reconciled their claims with the cited source and current repository contracts.
Four fresh-context review attempts and two subsequent peer-review attempts
returned context-window errors without usable analysis. They are failed review
attempts, not independent approval or a behavioral skill-evaluation pass.

This is source/documentation preparation only. No Cargo dependency, Rust source,
brand asset, application execution or Windows state changed. M0-SL-01 licence/
graph admission and M0-SL-02 candidate-bound contract review remain incomplete;
M0-SL-03–05, current priorities, execution holds and D1/D2/D3 deferrals are unchanged.

### Retained candidate evidence gaps

- Run the required Application Verifier/full-PageHeap profile on an owned image
  through the documented elevated per-image setup and restoration workflow when
  that environment is available. The ordinary-user CDB result is narrower; full
  native-memory and runtime-mitigation qualification remain incomplete.
- Hosted publication and the required Rust check are completed; see
  [publication and hosted qualification](#publication-and-hosted-qualification).
- Retain D1–D3, production coverage/mutation, provider/source-default decisions,
  disposable service/clock/MSI laboratory and complete-package/signing gates.

## Design-reference adaptation, 29 October 2026

David authorized using [design v0.2](../docs/nevrotick-design-v0.2/README.md) to
adapt this fixture-only prototype. Starting executable: the recorded
`windows-candidate-5` above. The changes reorganize the existing sample content
into the design's page/profile/evidence hierarchy, distinguish offset from
uncertainty, clarify source identity/details/editor regions and group Settings.
Standard native controls, checked in-memory editing and existing scope remain.
No production service/freshness state, policy flow, theme/tray subsystem or new
dependency is part of this adaptation. D1–D3 remain Deferred.

Ownership: native UI owner changes `native/view.rs` and `native/controls.rs`, plus
the causally required parent style in `native.rs`;
integration owner updates the native observation helper and this record. Review
and actual Windows observations cover the changed presentation and selected
lifecycle paths, including visible rejection text after resizing. The earlier
source inventories/receipts remain historical evidence of their exact candidates.

The adaptation uses 44 fixed standard controls: a compact shared header, clear
Status/profile/headline hierarchy, sample evidence, side-by-side offset and
uncertainty, a local **View sources** action, a compact source list showing names
and hosts, distinct selected-source/editor areas, and three read-only Settings
groups. Field validation, fixture values, dependencies and protected-action
availability retain their existing owners. No branded artwork is embedded by
this structural adaptation.

Evidence is under the existing external `ui-m0/design-adaptation/` directory:

- `observations-red` retains the old candidate's missing design heading alongside
  a healthy caption/banner and disabled synchronization control.
- `observations-2-hit-test` identifies editor group 203 above intended button 220
  in cross-thread `WindowFromPoint`. This was an observer mismatch, not proof of
  failed mouse input. The guarded observer now resolves only an owned sibling
  `BUTTON/BS_GROUPBOX`, with matching parent/process/thread and foreground root,
  through `RealChildWindowFromPoint`; the result must be the exact intended control,
  and is checked again after cursor movement. `observations-3` exercises 64 such
  resolutions with real input and successful editor semantics on unchanged bytes.
- Visual review of `observations-3/03d-resized-status.png` found stale Sources
  pixels inside Status groups despite passing visibility/geometry checks.
  `paint-probe-1` preserved that defect after a delay and after full redraw.
  Removing only the owned window's `WS_CLIPCHILDREN` flag cleared the background;
  subsequent navigation stayed clean without forced redraw. The temporary style
  was restored and the ordinary-user process exited zero. The source repair only
  removes that parent flag; no custom painting or additional redraw API was added.
- `portable-check-2.json` passes format/compiler/Clippy/rustdoc, five integration
  tests and three doctests on the final source. `windows-check-2.json` passes
  Windows compiler, all-target Clippy and linking. The dependency graph, admission
  policy and portable model remain unchanged from their earlier qualified rows.
- `observations-4-final` passes all three page hierarchies, exact control visibility
  and client containment, real local navigation/editor input, invalid-input
  preservation, Cancel, disabled actions, minimize/restore, normal Close and
  restart reset. All three resized pages and the full visible rejection text were
  inspected; Status has no residual Sources pixels. The ordinary resize was
  1350×1020 to 1230×960 physical pixels in the same Windows 10.0.26300.0,
  144-system-DPI reference environment. Actual interactive-minimum-size and full
  D1 display qualification were not performed.
- Five samples across 20 warmed interaction cycles stayed at 185 handles,
  9 GDI objects, 57 USER objects and 2,355,200 private bytes before normal Close.
  The observer exercised 64 narrowly checked transparent-group hit resolutions.
  These are bounded stability observations, not a sustained leak/soak result.
- `native-cdb-2/observation.json` and `cdb.log` record the final candidate's
  ordinary-user creation, editing, navigation and Close under the inspected fault
  filters: no selected fault markers, both exits zero and both processes stopped.
  The retained SDK visualizer warning and ambient Nahimic module have the same
  evidence limits as the earlier session. Full AppVerifier/PageHeap remains
  unexecuted.
- Independent source/helper/native review found no remaining material ownership,
  lifetime, effect or repaint defect and closed the observed visual finding on
  the final rebuilt screenshots.

Current candidate: `design-adaptation/windows-candidate-2/nevrotick-ui.exe`, SHA-256
`81f01f0b9486f9d9aacaef55b95a3c2039d74b2d40026bd60cf84e1ee0331fbf`.
Current helper SHA-256:
`b2d78c96624575a2105a39f2d7040bdcbaac23178a56126056b30687c767b3ee`.
The final invocations use the same documented verification commands and reviewed
vendor, new `design-adaptation` output/receipt directories, and byte-identical
`harness-3/ui-m0-windows.ps1` for the final native run. The local
`design-adaptation/final-source-inventory.json` binds the final working tree,
build/observation evidence and screenshots. No commit or publication had been made
at that checkpoint. The later commit/push authorization is recorded above.

Pre-commit review also made the hosted Linux sandbox prerequisite explicit:
the first published `rust.yml` installed bubblewrap on the temporary runner before
tool acquisition. Product execution still required the unchanged isolation controls.

## Publication and hosted qualification

Signed commit `8a7d9a9839687eb72f38c56837b957ce095b5e0c` published the reviewed
28-file candidate to `main`; its tree matched the complete staged materialization.
The [security workflow](https://github.com/DavidOsipov/NevroTick/actions/runs/40860783004)
passed. The [first Rust workflow](https://github.com/DavidOsipov/NevroTick/actions/runs/40860783028)
stopped before dependency code or product tests: Ubuntu 24.04 denied bubblewrap's
isolated loopback setup (`Failed RTM_NEWADDR: Operation not permitted`). The
runner had bubblewrap 0.9.0-1ubuntu0.3; this was a sandbox capability failure.

The follow-up loads the distribution's `bwrap-userns-restrict` AppArmor profile
from `apparmor-profiles`, following the documented
[Ubuntu sandbox setup](https://github.com/openai/codex-security/blob/621bad47e69dc41880320259f6fa47bf293c097c/github-action/README.md#ubuntu-sandbox-setup),
and probes namespace setup before acquisition. No global AppArmor setting or
shared development host is changed; the actual verifier and isolation controls
are unchanged. Signed follow-up `905c0e296f7e93fa3031f1e15e23752ea67d86e4`
passed the [hosted Rust workflow](https://github.com/DavidOsipov/NevroTick/actions/runs/40861569865)
and [security workflow](https://github.com/DavidOsipov/NevroTick/actions/runs/40861569914).
The runner passed the early namespace probe, all admission controls, both frozen
dependency target rows and the portable product tests/doctests. This is the hosted
GREEN evidence for the runner repair, separate from the retained native evidence.

After observing success, ruleset `26110199` was updated and read back: both
`Security gate` and `UI-M0 portable model` are required on `refs/heads/main`, bound
to GitHub Actions integration 15368. Strict checks, creation exemption and existing
bypass actors are unchanged; the signed/linear-history/deletion/force-push rules
remain unchanged. Direct publication used the existing owner bypass while each
new commit awaited checks; these observed passes do not prove bypass prevention.

GitHub's publication warning was separately inspected: all 15 open Dependabot
alerts on 29 October target the unchanged design reference's
`docs/nevrotick-design-v0.2/prototype/package-lock.json` (7 high, 7 moderate, 1 low),
not `Cargo.lock`. Current workflows do not install or run that npm prototype.
Its Vite server binds `0.0.0.0`; repair and qualify its dependency graph before
running/exposing that server or processing untrusted build input. These findings
remain open and are not suppressed by the native product checks.


## HTML design-reference replacement, 5 November 2026

David explicitly requested replacing the repository prototype with the latest
reviewed HTML design from this session. The current reference is now
[design v0.4](../docs/nevrotick-design-v0.4/README.md), including its sources,
original AVIF assets, tests, generated entry points and bounded review evidence.
Top-level agent/docs/development guidance points to v0.4. The v0.2 React viewer,
43-scene links and dated evidence remain historical and are not installed or run.
Only its README history marker and that manifest's corresponding entry changed.

Imported production source: `62aa9713548f8bfefc7ea087a01d788bc6f39c10`;
browser-test revision: `bcbd4fc30becad3c0dd5109b7df93c3bc21c0099`;
standalone SHA-256:
`4d8c3d211ec19ddea21a4b37d6b164f453a8ab8352cca6c554c8c8ac28370f86`.
The 176 delivered files were compared byte-for-byte before adding repository
instructions. `IMPORTED-SHA256SUMS.txt` retains the imported manifest;
the current manifest also covers the added README/AGENTS and import receipt.

Owner: repository integration; independent reviewer: read-only prototype-import
reviewer. Requirement/evidence scope: M0-08 reference identity and fixture-only
boundaries, not new native acceptance. Independent expected result: the reviewed
standalone bytes remain unchanged after import/rebuild; local links reach the
current design; all imported asset/source receipts validate. Selected checks:
197 existing Node tests, deterministic build, structural/CSP/AVIF checks, current
and imported manifests, focused repository-path Chromium preview and changed
Markdown links.

Executed import checks in this Linux cloud checkout:

- `node build.cjs`: deterministic rebuild, 30 scenes, 822403-byte standalone;
  SHA-256 remains the exact imported value above.
- `node --test tests/*.test.cjs`: 197/197 pass, no skipped or failed tests.
- `node tests/render-fixtures.cjs /tmp/nevrotick-repo-v04-views.json` and
  `python3 tests/check_structure.py /tmp/nevrotick-repo-v04-views.json`:
  1247 documents, zero structural errors, matching final-script CSP, no external
  standalone dependencies and original AVIF bytes preserved.
- Imported/current v0.4 manifests and the maintained v0.2 manifest pass.
  Independent review also compared all 175 originally covered source/assets/
  documentation files byte-for-byte and the imported receipt itself.
- 742 relative Markdown link targets in the changed guidance/documents exist;
  external URL/mailto links were excluded from the local-path check.
- Chromium 155 from the actual repository URL, both HTML entry points at
  360/1280: Simple About, Notifications details in Power user, master preference
  and in-page test notification pass; no page errors, unexpected remote requests
  or horizontal overflow. The repository-path screenshot was inspected.
- Read-only independent import review: no material findings. `git diff --check`
  passes. Full prior browser/semantic audits remain attributed to the imported
  exact candidate; no native/Cargo qualification is claimed for this import.

No Rust/native code, dependencies, services, actual Windows notifications, clock,
machine configuration or publication is changed. The v0.4 simulations do not
close UI-M0's native-memory gap or re-enter deferred D1/D2/D3 work. Current Core
priority and the existing native/analysis/storage holds remain unchanged.

## HTML tray design extension, 5 November 2026

David requested a concrete tray icon/menu design and then required a warning
before tray exit, explaining continued background-service operation and the
route to stop/disable it. Updated the current v0.4 HTML source/model/styles,
generated entries, guide, tests and [review](../docs/nevrotick-design-v0.4/review/tray/REPORT-RU.md).
F-10 and architecture §16.1 record the user-visible exit contract. This amends
the design reference; the earlier imported-file identity above remains historical.

The outside-window Windows-area sample, contextual status icon/tooltip and menu
retain opt-in defaults and separate sign-in preference. Quit opens a confirmation;
Manage background service navigates to Power Settings without issuing a command.
Only explicit Quit UI only marks the sample presenter absent; service state,
clock owner, history and preferences remain unchanged. Cancel preserves the observer.
Unknown/stopped service wording is conditional, and no menu-opening action grants
service/clock authority. Modal forms are protected from replacement.

Scope: M0-08 design custody and later F-10/AC-18 composition, not native M0/P5
acceptance. Final standalone SHA-256 is
`a363c40201f4e238a49523bc7fb45c1a789f37114fe5c7e53f5fb56627c9b9fb`
(865972 bytes). Actual checks: 207 Node tests, 1427 structural variants, six
Chromium tray journeys plus 60 scene views, the existing 360 Status/12 About
layout matrix and four notification-entry/width matrices. Final screenshots
were inspected. Independent status/focus review passed after corrections.
Detailed limits and source-bound evidence are in the linked report; logs remain
under `/tmp/nevrotick-tray-*`. Original AVIF bytes and imported receipt are preserved;
the current manifest covers intentional source/generated/test/guide additions.

No Rust/Cargo, native Slint/Windows, host settings, service, clock, notification
delivery or publishing work is performed. D1–D3 and existing holds retain their
owners/triggers. Later native tray work remains under S5.3/S5.4 and its re-entry gates.

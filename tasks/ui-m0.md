# UI-M0 implementation task

## Scope and custody

Authorized by David's “Do it” after the agent-readiness recommendations on
29 October 2026. Starting candidate: clean `e0c8516db9d1b284eb410bc2c8bd548c103a35fb`.
Architecture §16.5 and M0-01–M0-08 control acceptance. This task record is outside
the maintained documentation bundle.

Status: the coding foundation and functional prototype are implemented and
observed. Required full native-memory instrumentation remains incomplete;
the narrower debugger and functional results below do not close that gap.
The latest candidate and results are in the
[design-reference adaptation](#design-reference-adaptation-29-october-2026) below.

Deliver a fixture-only, unelevated native Windows UI, its minimal real Cargo
workspace, shared verification command and applicable CI. Align GitHub's existing
security gate with the deliberate hosted CodeQL deferral. David subsequently
authorized committing and pushing the completed work on 29 October 2026. That
authorization does not change services/clock/trust stores, add production
IPC/network/persistence, or establish P0/P5/release qualification. Only require
a new hosted Rust gate after that check is available on the branch.

## Ownership and paths

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

- Run the required Application Verifier/full-PageHeap profile on an owned image
  through the documented elevated per-image setup and restoration workflow when
  that environment is available. The ordinary-user CDB result is narrower; full
  native-memory and runtime-mitigation qualification remain incomplete.
- Publish through the normal reviewed commit/PR workflow when authorized, observe
  the new hosted Rust job, then add that actual check to main's required checks.
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
`rust.yml` installs bubblewrap on the temporary runner when absent, before tool
acquisition. Product execution still requires the unchanged isolation controls.

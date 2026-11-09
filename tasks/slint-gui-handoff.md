# Slint GUI handoff for the next AI

Prepared 6 November 2026. English working notes; keep product copy readable.
This is a continuation guide, not a second implementation plan. The
[S0.5 task](s0.5-ui-comparison.md) owns candidate history and evidence.
Remote review: [PR10](https://github.com/DavidOsipov/NevroTick/pull/10).

The [9 November imported-findings repair](codeql-cloud-handoff.md#imported-full-repository-findings--9-november-2026)
adds process-owned AccessKit subclass custody and standalone actual-module facade
controls. Reuse that exact source/evidence chain; this memory-safety correction
does not activate D1–D3 or establish native USER32/UIPI/UIA, resource-retirement,
critical-file metric or release qualification.

## Read first

Read the current checkout's `AGENTS.md` and [documentation guide](../docs/README.md),
then S0.5's active goal, [current native/text2 integration](s0.5-ui-comparison.md#current-nativetext2-integration--gui30-and-later)
and [hardening handoff](s0.5-ui-comparison.md#planned-hardening-gui-readiness-review--6-november-2026).
Consult the [v0.4 reference](../docs/nevrotick-design-v0.4/README.md),
[GUI source guide](../crates/nevrotick-ui/ui/README.md),
[input audit](../docs/nevrotick-gui-input-admission-audit.md),
[native residuals](../docs/nevrotick-gui-native-input-residuals.md) and
[text development guide](../docs/nevrotick-gui-text-development.md).
Historical checkpoints retain their original failures, scope and holds.

## Branch and integration state

**Merged continuation — 6 November 2026:** PR10 merged as
`3ff7e91c18f4c478b81c016cdefe31538dcf8086`; its final head was
`5bc22f5a884067f4dbee82d1073d94bc80a2a6c4`. The later numerical checkpoint
`5dcc465b534399ccdc4c9be58e3579a1df62c636` changes no GUI product source.
The [hosted reconciliation](#hosted-merge-reconciliation--6-november-2026)
below records the two GUI commits separately from that later workspace candidate.

The following bullets retain the historical PR-preparation environment:

- Worktree: `/workspace/NevroTick-slint`; branch: `ui/slint-gui`. The PR head
  identifies committed source. Main worktree `/workspace/NevroTick` was kept clean;
  Core, protocol, time-domain and mathematical/proof source were not edited by
  this GUI implementation.
- GUI work started at `d0e6ca21c12eca0d55fee5e536d46672c0792a9f`.
  That earlier local commit brings the v0.4 HTML prototype, UI guidance and
  localization design; it was not present on remote `main` when preparing this PR.
- Remote `main` observed during handoff:
  `b808eb74fd82f91f207fc03b9c890441a73af4fb`. Common ancestor:
  `8a66af924c509e898a9bab9bb3282f11b24fba93`. Main has newer engine, policy,
  metadata, platform and documentation work. The PR now integrates that main snapshot through a conflict-resolution merge.
  The ordinary combined-candidate checks are recorded below; native and release
  qualification remain separate.
- Owner-decision collisions are resolved: published OD-11 SOCKS5, OD-12 GeoIP
  and OD-14 platform scope stay intact. OD-13 now retains both Slint selections
  and the accepted GUI licence/composition amendment; localization is OD-15.
  Maintained routes use those canonical identities; old receipts keep their scope.
- Remote main also defers formal proof execution until after V1 under its OD-10
  amendment. Preserve that newer decision when integrating; this branch's older
  mandatory-proof wording is historical and must not restore a V1 proof blocker.
  The GUI work ran no formal compiler/solver/proof jobs.

## Implemented and verified within fixture scope

| Area | Delivered behavior |
| --- | --- |
| Slint shell | Windows entry selects Slint/Winit/software/Fluent. Simple has Status, Settings, About; Power adds Sources, Statistics, Diagnostics. About describes the program, author, fixed GitHub repository and component libraries. Original branded PNG and separate notices retained. |
| Source workflow | Single checked source owner; add/edit/remove/undo, stable identities, search, details and change review. Protocol/default-port/manual-port provenance stays explicit. Edits are local previews; Apply remains unavailable. |
| Settings and recovery | Grouped synchronization/service/notifications/time-policy/personal preferences. Custom interval accepts positive whole minutes with bounded raw/numeric admission. Policy and Roughtime/helper recovery previews preserve drafts and distinguish NTS, plain NTP, observe-only, provisional date and clock authority. |
| Status and diagnostics | Closed typed presentation separates authentication, agreement, freshness, service liveness and clock effects. Historical observations cannot show current Tracking. Diagnostics and correction review show unavailable/sample facts and safe navigation. |
| Shell/tray/toast source | UI-only quit warning preserves service continuity. Opt-in native tray and six fixed menu rows, explicit sample test notification, bounded activation token, current-context guards and acknowledged cleanup source are implemented. OS acceptance does not imply a visible/read notification. Native behavior remains unqualified. |
| Hostile input | Seven raw fields retain at most 5248 content bytes; original field caps precede retained allocation. Receivers independently check private route/phase/modal/current raw snapshot. Fixed redacted rejection preserves accepted state. Native keyboard/IME guards and reviewed AccessKit conversion/range admission are integrated; earlier OS/toolkit allocation remains outside those bounds. |
| Localization | Strict original UTF-8 JSON, duplicate/unknown-field rejection, closed schema, canonical bounded CBOR, Unicode/emoji policy, typed placeholders, literal sinks and atomic catalog publication. Missing/blank/orphan consumer checks run before Cargo. Catalogs cannot choose actions, severity, permissions, routes or URIs. |
| Dependencies/tooling | One pinned graph and schema3 admission policy; Windows-only Winit/AccessKit/WinRT/Shell features, narrow Linux software test profile, reviewed local `spin_on` and AccessKit patches, guarded verifier/captures and negative checker controls. No new Core runtime or broad execution exception. |

Text identities: schema 2, text contract 2, English revision 2; informational GUI
build 2. There are 509 active IDs, 596 stable ordinals, 87 inactive identities and
426 immutable output rows. Fingerprint:
`34c97f8a74a9559ae998f55389bedd9f92bb3f878e72df84d090372053a69e80`.
Do not reuse retired ordinals or equate the GUI build number with compatibility.

GUI original code is GPL-3.0-only; inherited Core/workspace code remains
GPL-2.0-only. Preserve [GUI provenance/licensing](../crates/nevrotick-ui/LICENSING.md),
patch notices and separate brand/font/Unicode terms. The selected 534 registry
identities include inert resolution entries; that number is not the runtime
package count. Distribution obligations remain separate from private checks.

Current Slint runtime/build pin: 1.18.1; Rust: 1.99.1. Current lock SHA256:
`9f31370aaf3649527926acf48a927ce2b627d8de633d8f7a50b467cf2f07c24e`.
Current schema3 policy anchor:
`c1735811cefab5f108906b44c40bb634a18bed3490a253e724bc429c4a6dc5cf`.
The 7 November documentation/source reconciliation changes only the UI package
description and its bound manifest hash. It removes an unsupported unelevated-token
claim; versions, features, grants, package scopes, patches and the lock stay fixed.
The prior anchor
`6cf46ae3b59889507f5e78942ea68cd68f0d9c7afe0169d4cd7f04caaa98488b`
identifies the [6 November dependency refresh](s0.1-source-readiness.md#refreshed-graph-and-current-evidence),
including seven transitive updates and the engine-scoped Tokio-macro Syn 3
closure. The policy anchor is the checker's canonical JSON digest, not the raw
file SHA256. A metadata amendment does not transfer historical execution receipts
to the changed candidate. Earlier lock `56833523c30ef64a3300fcc905d99039470fa1b88fc22ce295ada364bfd8a286`
and anchor `a3a008828e6d8c72b7441c29bafbc67f8aa22a4c9efddbb99e32bf705cfda3f6`
remain historical identities; their receipts are not transferred to the refresh.
The graph includes main's normal engine→time-domain edge and direct engine→garde
derive/defaults-off edge. Registry identities, features, grants, workspace and
non-engine package scopes retain their reviewed boundaries. The AccessKit
derivative pins include the range-lock repair below. The earlier GUI41–43
receipts retain the old graph and cannot qualify this current candidate.

## Not implemented or not qualified

1. **Production backend:** no secured production UI IPC, coherent live status,
   service/clock/protected Apply, Windows Time handover, real incident producer,
   sign-in registration, persistence or diagnostic collection/export. Current
   fixture ports cannot become production authority through an implementation
   swap; restore D2 contracts and independent receiver authorization first.
2. **Native Windows:** admitted SDK/CRT inventory, target linking and actual
   ordinary-user execution remain open. Qualify keyboard/IME cancellation,
   clipboard, COM/proxies, menu timing, tray/toast callbacks, UIA/Narrator, DPI,
   focus, shutdown and retained heap/handle/GDI/USER/task resources. Windows-target
   checking on Linux is not native qualification.
3. **Process/package hardening:** desktop-role mitigation startup/readback,
   required-protection refusal, actual loader/activation paths and complete
   hardened MSIX/Store-delivered qualification remain with their existing
   process/UI/package/release owners and applicable P0–P7 gates. OD-05's
   6 November clarification makes every applicable protection mandatory at the
   strongest supported compatible setting; unimplemented/untested is open work.
   The current Windows build selects v3 and the expanded hardening vector under
   [the existing build task](build-optimization.md#required-v3-and-maximum-compatible-hardening--6-november-2026).
   Do not copy headless Win32k/child restrictions into the GUI or add user-facing
   mitigation-disabling switches.
   `host_ready` means fixture binding readiness, not security readiness.
4. **Critical text approval:** production `REVIEWS` is deliberately empty. Exact
   critical English/catalog/bootstrap meaning needs independent human approval
   bound to content and contract. Test-only grants, signatures and parser success
   cannot approve production packs. Do not disable this gate to make an import work.
   The prepared private bundle at
   `/tmp/nevrotick-localization-contract-v2/human-review-v1/` covers all 509 active
   meanings, 87 inactive identities and eight separate bootstrap words. Its
   presence is neither approval nor guaranteed availability in the next cloud.
5. **Remaining polish/provenance:** scope the service administrator badge to
   changes rather than read-only inspection; explain the consumed session test
   identity after the first notification attempt. PR #10 review reconciled
   Unicode `derivation.json` to the current `repertoire.rs` hash after independently
   rederiving all 81 hard ranges and checking retained source identities and the
   English positive set. No Rust policy bytes or historical generator claims
   changed; see the review receipt below.

The existing S134/general native, separate fuzz no-retry and storage experiment
holds remain. D1/D2/D3 retain canonical re-entry triggers. This PR does not certify
ASVS/accessibility, full GUI completion, native cutover or release readiness.
GUI security-module coverage, critical-file mutation and native resource/soak
qualification are not established by the ordinary test or source-control counts.

## Evidence and replay

### Hosted merge reconciliation — 6 November 2026

Live GitHub checks were read at **14:15:59 UTC**; no alert or incident was
dismissed. The connected job-step reads confirm successful dependency admission
and isolated actual-product verification in both portable jobs.

| Exact candidate | Observed hosted result |
| --- | --- |
| Final PR head `5bc22f5` | [Portable](https://github.com/DavidOsipov/NevroTick/actions/runs/42228676978/job/127928626695), [Security gate](https://github.com/DavidOsipov/NevroTick/actions/runs/42228676593/job/127928980992) and [secret scan](https://github.com/DavidOsipov/NevroTick/actions/runs/42228676593/job/127928624233) passed. Default Actions/Rust/C++/Python analyses passed. Aggregate CodeQL and GitGuardian failed. |
| Merge `3ff7e91` | [Portable](https://github.com/DavidOsipov/NevroTick/actions/runs/42230373102/job/127934412937), [Security gate](https://github.com/DavidOsipov/NevroTick/actions/runs/42230373254/job/127934786357) and [secret scan](https://github.com/DavidOsipov/NevroTick/actions/runs/42230373254/job/127934414509) passed. Default Actions/C++/Python analyses passed; Rust remained in progress at the observation. |

The failed [PR CodeQL aggregate](https://github.com/DavidOsipov/NevroTick/runs/127928935171)
contains 11 Python path annotations and a missing-Rust-configuration warning.
It completed before the successful final-head Rust analysis, so that warning
does not establish a Rust extraction failure; the aggregate remains failed.
Alerts [#105](https://github.com/DavidOsipov/NevroTick/security/code-scanning/105)
and [#106](https://github.com/DavidOsipov/NevroTick/security/code-scanning/106)
remain open on merged main. They flag `lstat`/`os.open` in `scripts/ui_source_io.py`
through the same developer-selected root and fixed generator paths previously
reviewed below; earlier #95/#96 instances are marked fixed after those reads moved
into the shared helper. No new unauthorized path access was demonstrated under
that local CLI contract. External disposition remains open.

The failed [GitGuardian check](https://github.com/DavidOsipov/NevroTick/runs/127928604764)
still reports [incident 39281800](https://dashboard.gitguardian.com/workspace/311486/incidents/39281800?occurrence=310809530),
the public fixture regex at `.gitleaks.toml:55`, matching the earlier assessment.
The incident remains triggered. These individual passes do not make overall CI
uniformly green or qualify native Windows, GUI coverage/mutation, critical text,
backend, package or release behavior.

### Local source continuation — 6 November 2026

After the signed/pushed numerical checkpoint `5dcc465`, the local read-only
text-source gate passed with 509 active/509 used IDs, zero errors, schema/text
contract/build 2 and the unchanged fingerprint above. The selected
`test_ui_text_native` and `test_native_input_source` controls passed 53/53 under
`-X dev -Werror` in 28.943 seconds, with zero errors or failures. Raw log:
`/tmp/nevrotick-merged-gui-source-controls-20261106-v1.log`.
These source controls ran no Rust product, rendered GUI or native Windows path.
Hosted checks for `5dcc465` are not yet qualified; earlier hosted results remain
bound to their recorded final-head/merge commits. The ordinary fixture source
does not close backend, exact-text approval, coverage/mutation or native gates.

### Historical cloud receipts

Historical canonical receipts under private cloud directory
`/workspace/.nevrotick-cloud/slint-uia-fork.2020wssl/`:

| Receipt | Observed result / SHA256 |
| --- | --- |
| `gui41-integrated.json` | Passed format/compiler/Clippy/rustdoc; 736 ordinary tests, 164 doctests, 447 optimized tests. GUI: 183 tests/17 doctests; 143 pre-Cargo controls, 509/509 text consumers, 50 captures. SHA256 `badd0f7ee0cba45fc009065e2add507985b0ba22b35714ad51b13d71091c5dcc`. |
| `windows-source7.json` | Linux-hosted Windows-target all-target type checking and Clippy passed; no SDK/CRT/link/run. SHA256 `27f1e89885a69171fa45d9e225ca50e21cc286dd5fae4e9b922444bf8b027732`. |
| `gui38-dependencies.json` | Both-target graph/scanner checks passed at their admitted advisory snapshot. SHA256 `db98db71574953fe5a09b16c9f55a4e133214d3b9937d3ef2d57a62e9cab4f8c`. Advisory freshness must be re-established for a new dependency check. |

**Historical PR preparation:** `gui42-pr-portable.json` passed the complete configured
portable/capture row, with the same 736/164/447 test totals, 183 GUI tests,
17 GUI doctests,143 controls and50 captures. Receipt SHA256:
`62a286f37888cc0e3df83b7cb7920c6863ad114946d4c5a2c3c1a0b7e796a77e`.
Compared with GUI41, its366-path source map changes only the updated UI README.
Root and an independent reviewer rehashed both sets of actual PPM originals;
all50 are byte-identical, preserving the existing visual-review lineage.

After that run, PR hygiene removed one trailing ASCII space from
`scripts/check_ui_text_coverage.py` and two redundant EOF newline bytes from
`scripts/ui_text_bindings.py`. Those are the only final source-map deltas;
restoring those bytes reproduces GUI42's exact hashes. The affected source
controls were then rerun with the canonical admission/sandbox and no Cargo
selection: `gui43-pr-source-controls.json`, SHA256
`0ae7c2158c3556c13ed62e50e029f953412600bd18ccd9d5f28d1a88b397038c`,
passed all143 controls and509/509 coverage. Its365 admitted sources matched that
recorded candidate; the optional SDK policy belongs to the full GUI42 row.
GUI43 is a Python-only diagnostic, not another product/native test run.

**Historical combined candidate, GUI44:** this ordinary source checkpoint incorporated
main `b808eb7`. `gui44-merge-portable.json`, SHA256
`e72e4ba6f3e15aac80c15a7d3118b3cecd59e4405456e8913cf85a02398b2d0b`,
passes all configured build/Clippy/rustdoc,759 ordinary tests,174 doctests,
470 optimized tests,143 source controls and50 captures. GUI remains183 tests/
17 doctests; main's engine now has47 ordinary/47 optimized tests and12 doctests.
All368 source entries (367 product plus SDK policy) matched its recorded candidate.
Its 50 actual capture files were byte-identical to the reviewed GUI42 lineage.

Both-target dependency/scanner row `gui44-merge-dependencies.json` passes under a
new lock-bound public advisory snapshot, SHA256
`1f2dbe65f19447bfa15927d26939efabb89bbc628fd7298eecacb91b8e21f7b5`.
The separately captured18 metadata/tree observations and independent review admit
only the named engine/domain closure. No registry grant, patch pin or non-engine
scope was expanded. Focused inert tooling controls pass223 cases; the skill
checker admits65 leaves and its25 tests pass. Private Python receipt
`/tmp/nevrotick-merge-observation-v1/python-targeted.json`, SHA256
`8a43b9a5bff80339e431dd6e5b21c34bd568fb849ac146e5e9b633af353ff038`.

An optional broader Core campaign-control run is **incomplete** (exit137), with
inherited checked-in-campaign failures before termination. Its unchanged policy
binds158 files; main already selects168 and has11 stale bound hashes. The merged
product selects367. This is not a new Core runtime defect or a current campaign
pass. Eight affected inert Core registry-projection checks and five domain
identity checks pass; do not rebind historical campaign/proof metadata or resume
deferred formal work merely to clear that broader run.

S0.5 records exact later deltas, failed attempts and independently viewed image
lineage. GUI41's 50 captures matched the reviewed GUI40 set; GUI37's full 50-frame
set and two emergency frames were viewed separately. These are software-rendered
fixtures, not Windows screenshots. Handoff preparation updates the previously
stale UI README; account for that documentation input when comparing source maps.

Raw receipts, images, review reports and the human-review bundle live outside
Git. The listed `/workspace/.nevrotick-cloud` directory and private `/tmp` review
bundle/targeted receipt are absent in the current local checkout environment.
Their input maps and image bytes were not revalidated here. Preserve them through
an approved artifact channel or reproduce the affected configured checks. Never
manufacture a missing receipt or claim a new
test/native pass from this summary.

Use [configured tooling](../TOOLING.md#starting-implementation), not bare Cargo.
In this cloud, after validating installed inputs, the portable replay is:

```sh
python3 -B scripts/verify.py --lane portable --capture-slint \
  --vendor-dir /workspace/.nevrotick-cloud/slint-uia-fork.2020wssl/vendor \
  --scratch-root /workspace/.nevrotick-cloud/scratch \
  --toolchain-dir /workspace/.nevrotick-cloud/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu \
  --nextest /workspace/.nevrotick-cloud/cargo/bin/cargo-nextest \
  --linux-sdk-dir /workspace/.nevrotick-cloud/fontconfig-native-sdk.astjd5d_/work/native \
  --output-dir /new/owned/external/artifacts \
  --json /new/owned/external/receipt.json
```

Paths are machine-specific, not installation instructions. The verifier checks
exact tools, graph, SDK and sources; it uses offline execution, cleared allowlisted
environment, read-only inputs, owned scratch and fixed resource/time bounds.
Keep acquisition separate, proxy/TLS intact, negative controls and actual test
counts visible. Missing prerequisites are incomplete evidence; do not relax pins,
limits, lints, graph roles or trust to obtain green output.

## Next work and dependencies

At the next applicable source/native checkpoints, carry the
[R4 build-diagnostic and tray-panic follow-ups](s0.5-ui-comparison.md#build-diagnostics-and-tray-panic-follow-ups--9-november-2026)
into their existing owners: optional cache-aware Slint warning denial, explicit
caught-payload fault policy, and narrow unwind assertions during a natural edit.
Their controls remain proposed; they add no current defect, release gate or
execution exception and do not displace S2.2.

1. Continue from merged PR10 and the later numerical checkpoint. Preserve the
   current S2.2 producer/receiver freeze in development section 7 and reconcile
   only subsequent deltas. Preserve the canonical decision identities. Keep
   lock, manifest and graph policy changes with one owner; re-admit affected
   graphs before target execution.
2. Preserve the local text/native-source-control pass on `5dcc465`, then select
   the next configured product checkpoint against its frozen inputs.
   Reuse unchanged evidence only after matching all relevant inputs; historical
   GUI42/43/44 counts do not establish results for later workspace bytes. Retain
   the two named wording-polish follow-ups above for a separate text revision.
   At that localization checkpoint, consider the [R3-P-01/P-02 source experiments](s0.5-ui-comparison.md#localization-cost-experiments--9-november-2026)
   with their approval, invariant-failure and notification controls; they are
   unexecuted proposals and do not displace the current S2.2 producer/receiver freeze.
3. Complete the independent exact-text review; revalidate any wording changes and
   their native/action/accessibility consumers. The supplied review bundle is
   review material, not approval.
4. Native qualification can start with the exact fixture GUI once existing
   Windows holds/prerequisites are resolved. Exercise its lifecycle/resource
   matrix; restore D1/D3 only at their triggers. Neither Linux captures nor
   Windows metadata checks replace those observations.
5. In parallel where prerequisites permit, prepare authenticated status/incident
   producers and narrow service/helper requests. Activate live adapters only at
   D2 using the existing hardening table. Preserve outcome-unknown, privacy,
   current-generation and actual-completion semantics; repeat affected native
   production journeys after integration. Final package, mitigation and release
   gates remain with their existing owners.

PR10 is merged. This continuation authorizes no deployment, Store submission or
OS/service changes; existing Windows/fuzz/storage holds and D1–D3 triggers remain.

## PR #10 detection review and corrections — 6 November 2026

Reviewed PR head `a3640f83b7f97f1062d2adb2fb72f4391699d031` against main
`b808eb74fd82f91f207fc03b9c890441a73af4fb`, including all 11 inline CodeQL
findings, the GitGuardian/Socket reports and failed hosted checks. Corrections
were prepared in the isolated `codex/pr10-audit` worktree. The concurrent staged
Core work in the main checkout was outside this task and was not modified.

Scope: ordinary source/tooling repair under OD-13/15 and CHECK-19/20/22/24/25/26;
explicit fixture/native/release limits remain. Expected results were bounded
file rejection before blocking, exact admitted archive acceptance without a
general limit increase, useful visible preview fixtures, and secret detection
outside exact independently checked public-data lines. No formal execution,
proof-consumer repair, Windows execution or fuzz campaign was selected.

### Reported detections

| Detection | Assessment and disposition |
| --- | --- |
| CodeQL #94, `check_structure.py:12` | The developer explicitly selects the local rendered-fixture file. This reference-only checker is not a server or privileged reader; no lower-trust filename is appended to a protected directory. No unauthorized path traversal was established. No scanner suppression or behavioral change. |
| CodeQL #95–98, generator input/root operations | The CLI selects the repository; the contract/template paths are fixed, symlinks are rejected and complete source identities are checked before code emission. No demonstrated path-injection exploit. A distinct regular-file replacement race was reproduced and repaired below. |
| CodeQL #99–104, generator output operations | The CLI selects an external output parent; the generator uses a new private directory, a closed basename set and exclusive no-follow creation. Cleanup targets its own new directory. No demonstrated path-injection exploit. No output-policy weakening or suppression. |
| GitGuardian incident 39281800 | The reported `.gitleaks.toml:55` line is an exact regex for a public unauthenticated u64 test fixture, identical to the reviewed main bytes. It is not a credential; no rotation or history rewrite is warranted. External incident state was not changed. |
| Gitleaks retained-history/working-tree failure | Both matches are public source digests: native-input Rust token content and FreeType `ftmodapi.h`. The first was recomputed from current Rust; the second exactly matches the public [FreeType VER-2-13-3 header](https://github.com/freetype/freetype/blob/VER-2-13-3/include/freetype/ftmodapi.h). Added rule/path/full-line exceptions with adjacent-secret, same-line-secret and wrong-path controls. Both local scans pass. |
| Socket reports | Hosted checks report no new dependency alerts. Scores and successful report generation are not independent licence/advisory qualification; exact graph and existing distribution gates remain. |
| Portable hosted job | Preparation rejects the newly selected `windows 0.62.2` source archive at the old 80 MiB expansion ceiling. Confirmed on the exact cached archive and corrected as below. The local repair is not a hosted build pass. |

These path-warning conclusions apply to current explicit local CLI use and fixed
source/output contracts. A future remotely driven or privileged caller needs a
new assessment. GitHub alert/incident dispositions were not silently changed;
the PR's historical red check results remain distinct from the local corrections.
The initial missing Rust CodeQL configuration warning cleared when its default
setup Rust analysis finished successfully; it reported no additional inline
finding in this review snapshot. Prepared custom CodeQL lanes remain deferred.

### Confirmed corrections

- **PR10-R1 — source-reading denial of service:** replacing a checked regular
  file with a FIFO blocked codegen, protected JSON reading, source coverage
  snapshotting and native-input source reading before their checks could reject
  it. Healthy regular-file controls pass; new regressions against the original
  sources fail by bounded child timeout. All four readers now share
  `ui_source_io.read_regular`: nonblocking/no-follow open, descriptor type and
  identity checks, bounded bytes and post-read custody checks. Existing root,
  count and aggregate guards remain. The helper enters the verifier source and
  GUI-tool licence inventories; semantic oracles remain separately owned.
- **PR10-R2 — fresh dependency preparation fails:** the exact `windows 0.62.2`
  archive has 114,755,072 tar bytes, 114,212,919 file bytes and 701 regular files.
  Its archive and file-map identities match policy. Admission now uses those
  measured limits only for that complete four-field identity. Default limits,
  aggregate limits, member/type/path checks and final source identity remain.
  The real cached archive changes from rejection to successful bounded parsing;
  synthetic controls reject altered identities and exceeded ceilings.
- **PR10-R3 — standalone source preview is stale:** its three rows did not reach
  `visible-sources`, and the migrated status projection was empty. The preview
  now binds visible/selected fixture state and existing status copy while keeping
  `host-ready: false`. Source review only; no new compiled/rendered preview pass.
- **PR10-R4 — current design instructions point at the wrong decision:** corrected
  the v0.4 AGENTS/README UI links from the former unpublished OD-11 to canonical
  OD-13. Updated only their current manifest entries; imported receipts remain.
- **PR10-R5 — current Unicode provenance hash is stale:** reconciled the Rust
  hash after independent checks of all six source identities, the 81 derived
  hard-denial intervals, the positive English set and its disjointness/assigned
  noncontrol properties. No Rust/table bytes or historical generator claims
  changed. This is data provenance, not resumed formal-proof work.

### Independent review and executed checks

All three workers used fresh contexts (`fork_turns: none`) and the named PR/base,
current contracts and explicit read/write ownership. Their results completed:

| Reviewer | Inspected scope and result |
| --- | --- |
| `pr10_rust_contract_review` | Portable Rust model/domain owners, handwritten localization/parsers/formatters/approval, catalog projection, all capture source and selected integration/binding controls. Independently matched 509 active specifications, 596 ordinals, 426 output slots and the exact 46,303-byte CBOR fixture. No new Rust defect confirmed. Subsequently reviewed the final correction delta and ran seven focused controls successfully. |
| `pr10_native_slint_review` | Active Windows tray/toast/input/lifecycle and host paths, modified AccessKit and complete spin_on repair, maintained Slint pages, HTML build/app and selected model/view/test flows. Found/fixed R3 and independently checked R5. Embedded HTML JavaScript/CSS and CSP hash agree. No further actionable defect confirmed. |
| `pr10_ci_triage` | Dependency/schema/graph admission, archive identity, patch custody, verifier command/results, CI and changed Core/domain projections. Reproduced/fixed R2, reproduced sibling R1 readers and identified the two secret-scan matches. No formal consumers were executed or repaired. |

Executed Linux Python checks: 14 generator, 78 text-coverage, 14 native-input,
26 acquisition, 118 verifier, 28 dependency-profile and eight SDK controls pass;
24 security-helper tests and 25 skill-checker tests pass. The skill inventory
admits 65 leaves. The real source gate covers 509/509 active text consumers with
zero errors; all 183 current design-manifest entries match. The pinned secret
scanner passes its healthy/fault/restored/disposition controls and both retained
history and working-tree scans. Both workflows pass actionlint, ShellCheck and
pedantic offline zizmor with two healthy and three rejection controls. The focused reviewer controls are corroboration,
not additional product coverage. Earlier failed original-source regressions are
retained as the causal basis, not counted as healthy passes.

Coverage limits: not every assertion, historical screenshot or unchanged vendored
upstream line was audited. The HTML protocol switch's handling of manually entered
default ports has no located explicit v0.4 preservation contract and was not
promoted to a confirmed defect. No Cargo/native/rendered product replay, current
full dependency/advisory lane, critical-text approval or package/release result
follows. Historical GUI44 execution receipts retain their original input maps;
new source/checker bytes require fresh admission at the next configured product
checkpoint. The archive and secret repairs do not themselves clear hosted checks.

### Final documentation and hosted-build follow-up

Corrected the eight production JSON seed/visitor `# Errors` comments that
described test-control preparation. They now identify actual decoding/type,
integer-range, text-policy, duplicate-key and shared resource-budget failures.
Independent source review confirmed that executable Rust is unchanged; the
509/509 text-source gate still passes with the same semantic fingerprint.

At `9606631`, hosted secret scanning and the security gate pass. The required
portable job then reaches compilation but cannot find `fontconfig.pc`. Its
ordinary host-native route already uses read-only OS mounts; it does not select
the optional, separately pinned Debian SDK. The workflow now installs Ubuntu's
authenticated `pkg-config`/`libfontconfig-dev` host prerequisites and checks
Fontconfig availability with a cleared environment. Product graph,
compiler and optional SDK pins remain. A fresh complete hosted portable result
was then required before merge; the later final-head and merge successes are
recorded above. This prerequisite repair supplied no Windows or packaged-product
qualification.

The workflow correction passes 24 security-helper tests, eight SDK controls and
the pinned actionlint/ShellCheck/pedantic offline zizmor checks over both
workflows, including two healthy and three rejection controls.

The first follow-up run installed Fontconfig successfully but the additional
partial-filesystem sandbox probe exited 139 before any product checks. Restored
the previously passing full read-only-root sandbox capability smoke check and
kept Fontconfig availability as a separate cleared-environment host preflight;
the ordinary verifier still owns its complete sandbox and product execution.
The setup failure remains a failed run, not evidence of product behavior.

## Rust data-flow audit and text-range lock repair — 6 November 2026

This bounded follow-up applies CHECK-19/20/24/25 and preserves CHECK-22's native
qualification boundary. It uses `qa-data-flow-analysis`, the CodeQL reference,
`qa-subagent-dispatch` and the concurrency/lifecycle skill. No formal, fuzz or
Windows product execution is authorized by this receipt.

**Custody and candidates.** Main was actively owned by another AI. The reviewed
snapshot is committed base `268598191a3a50a41bd51849daa9d0dfda2969b5` plus the
15 then-current uncommitted files, copied consistently into the isolated
`codeql-work/fixtures/rust-audit-20261106` worktree. Their SHA-256 identities are
in the external `codeql-work/results/rust-audit-20261106/candidate.json` receipt.
The repair is separately prepared on `codex/rust-dataflow-fix-20261106` in
`/home/david_osipov/NevroTick-rust-dataflow-fix`, based on that same committed
base. It does not include the other AI's dirty engine/policy work. The main
checkout/index was not edited, staged, committed, pulled or pushed by this audit.

### Reviewed flows and independent review

Three fresh-context workers performed complementary source reviews; all completed.
`rust_flow_protocol` covered wire/KE framing, request encoding, association receipt
and era/arithmetic/rounding. `rust_flow_engine_core` covered the real transport,
measurement/CLI outputs and Core history, principal/combination transaction,
readiness and modeled correction/rate receiving seams. `rust_flow_ui` covered
raw drafts, source submission, catalog publication, clipboard and native callback
consumers, then the selected AccessKit derivative. Their inspections were
read-only; they did not independently execute product tests or CodeQL.

- Packet bytes reach inert wire fields and checked numerical outputs. Association
  receipt refusal precedes pending-slot consumption. The engine has its own
  direct framing/measurement path: association guards are not barriers for it.
  The current engine has no runtime Core connection; observations remain
  unauthenticated and ineligible. No new defect was confirmed in these paths.
- Raw UI text reaches private bounded `RawText` construction only after field
  limits and character admission. Source edits check the current raw snapshot,
  route/modal/editor context and checked source construction. JSON/CBOR catalog
  admission is bounded/strict, with approval required before atomic publication.
  No new bypass was demonstrated in these paths. These source reviews do not
  close native clipboard, marshaling, receiving-service or production gates.
- A same-range COM comparison reaches the range's state lock twice, while range
  mutations can wait for that lock exclusively. This inherited dependency defect
  is corrected below. `rust_flow_protocol` independently reviewed the integrated
  repair and found no additional actionable defect, including in COM error
  ordering, distinct-state snapshot semantics, guard cleanup and regression scope.

### Confirmed inherited range-state deadlock

**7 November supersession:** this section records the earlier alias-only repair.
Its retained left-then-right contract for distinct owners is superseded by the
[distinct-range repair](#distinct-range-lock-order-repair--7-november-2026)
below. Historical tests and candidate identities retain their original scope.

In the retained `accesskit_windows` source, `Compare` could take two read guards
for the same state when both interfaces identify one range. `CompareEndpoints`
explicitly recognized that identity and still acquired the same lock twice. A
writer queued between those reads blocks the second read; the first read prevents
that writer completing. A waiting range mutation can also retain a tree read
guard, delaying later tree updates. The pinned Rust 1.99.1 source selects the same
writer-preferring futex lock for Linux and the selected ordinary Windows target.
This is the interleaving documented by [Rust's RwLock contract](https://doc.rust-lang.org/std/sync/struct.RwLock.html).
The pinned COM implementation advertises agility; apartment serialization is not
an established exclusion. No actual Windows deadlock was executed.

Both native consumers now use the same small `range_state::with_read_pair`
helper. Identical state owners provide one guard to both comparison operands;
distinct owners retain both guards throughout comparison and the previous
left-then-right order. Fallible implementation casting, context checks, invalid
endpoint errors and comparison of old weak endpoints remain at their original
owners. Acquisition failures skip comparison and release earlier guards. Native
callers preserve the existing poison-failure policy. No new COM or action
authority is supplied by the helper.

The exact helper is included by the existing GUI AccessKit ingress test. The
queued-writer regression uses real shared locks: it holds the first read, starts
a writer and observes reader exclusion with a bounded `try_read` probe. An
erroneous second acquisition returns a failure instead of hanging the runner.
An isolated helper variant retaining the original two-read sequence compiled and
passed its healthy comparison control, then failed this regression with two
acquisitions instead of one (exit 101). The repaired helper passes that same case.
This is behavioral evidence for the shared acquisition algorithm, not a native
COM reproduction or an execution of the complete original Windows methods.

Source/licence admission now accounts for the one new original GPL-3.0-only
helper and 27 derivative files. The original AccessKit revision, archive and
source-map identities, licences, package graph, features and Windows-only
normal dependency role remain fixed. The independent missing-helper rejection
and existing preserved-source/grant rejection controls remain active.

### CodeQL observations and limits

Local admission passed for CLI 2.27.3 and `codeql/rust-all` 0.2.24. A fresh
`/work/databases/rust-audit-20261106` database extracted the frozen dirty candidate.
The function inventory selected 2,983 functions in 95 source files across all six
product crates, including test/inactive-code scope rather than an exclusively
production denominator. A separate endpoint inventory returned 20 functions;
archived measurement, raw-draft and AccessKit text source bytes matched the frozen
files. The default extraction is the host/all-features/test-cfg configuration,
with dependency signatures, not a Windows or full dependency-body qualification.
Toolchain-discovery and macro-expansion warnings remain coverage gaps.

The existing interprocedural control query was re-executed on its owned fixture:
the positive input-to-sink route through `relay` produced one row and the constant
clean caller produced none. This qualifies that control, not product analysis.
The product global selector query exceeded its request budget and closed the MCP
transport. After reconnecting through the same guarded launcher, a narrower
same-callable selector diagnostic compiled but execution was killed (exit 137;
cause not established). It produced no admitted selector/path results. Further
local-flow work was interrupted after that failed prerequisite. A second offline
extraction with an independently admitted copy of the 534-package vendor cache
also hit the unchanged 600-second deadline and remains unusable. The temporary
CodeQL Cargo configuration was restored; tool budgets/policies were not weakened.
These incomplete stages establish neither absent flow nor absence of defects.

Separately, GitHub's current hosted analysis **2112729222** scans the exact
committed base `268598191a3a50a41bd51849daa9d0dfda2969b5`, with CodeQL **2.27.4**.
Its SARIF has **16 Rust results**, all critically inspected against six
byte-identical frozen/base files. The 15 dirty snapshot files are outside that
hosted candidate. The live API reports zero open Rust alerts, which describes
alert disposition and does not erase those results or establish a clean scan.
The connector lacks scanning-report reads, so the read-only GitHub API supplied
the analysis/SARIF; no alert disposition or suppression was changed.

| Hosted results | Current source-to-sink assessment |
| --- | --- |
| Two AES-SIV hard-coded keys | Public known-answer material inside unit/integration tests; the actual staged API takes its caller's key. No product graph edge to these separate tool workspaces. |
| CNG AEAD zero-initialized nonce | Test-only Wycheproof storage; all 12 bytes are overwritten from the case nonce before encryption/decryption. |
| CNG TLS 1.2 zero-initialized nonce | Both contiguous nonce spans are overwritten from the session IV and checked record nonce before `open`; TLS 1.2 is also disabled in the declared lifecycle cohort. No constant nonce reaches the reported sink. |
| Four state-guidance path results | All are inside `#[cfg(test)]`, from private temporary storage to an exclusively created fixed-name/PID witness and assertion reads. The configured runner enforces private temporary storage; they are not CLI input paths. |
| Eight engine runtime-boundary path results | Test-only private temporary capture directory; creation must succeed exclusively before ownership, stdout/stderr names are fixed, reads are bounded after child exit and cleanup deletes only those owned files/directory. No peer-provided path or authority crossing was demonstrated. |

No actionable vulnerability was confirmed from these 16 detections under their
current scopes. The independent source/lock review found the inherited range
comparison defect above; it was not a CodeQL-reported discovery. Hosted analysis,
local inventories, fixture controls, failed product flow queries and behavioral
repair evidence remain separate. Raw SARIF, comparisons, query hashes and BQRS
are retained in the external evidence directory. On 7 November, authorized WSL
cleanup retired six verbose evaluator/tuple/performance traces from the three
failed or interrupted `LocalSelectors`, `selectors` and `LocalFlow` jobs. Their
commands, timestamps, all 89 warning lines and failure context remain in
`dependency-refresh-20261106-v1/q1-space-preflight-v2/codeql-traces.json` under
the same external NevroTick evidence store, with deletion results in that
directory's `execution.json`. Unique source, queries, meaningful failure receipts,
successful result artifacts and hosted SARIF were preserved. Per-predicate and
performance replay detail was discarded; no failed job became a flow result or
qualification pass. Active CodeQL services and their caches were not changed.

### Executed verification and remaining work

The admitted pinned compiler ran the exact existing GUI ingress integration-test
source directly as a focused test binary in the configured read-only, offline
product sandbox. All 11 tests passed (three range-state and eight existing value
admission tests), with no ignored cases. The same source passed Clippy with every
current workspace Rust/Clippy deny rule. This focused direct compilation supplies
neither a full Cargo workspace pass nor Windows target/COM qualification.
Python controls passed: 14 AccessKit path-patch, 28 Slint dependency-profile and
118 product-verifier tests. Rust formatting, exact canonical policy admission,
all 27 derivative-file hashes and `git diff --check` passed. The UI text gate
also passed 509/509 active consumers with its unchanged semantic fingerprint.

Retained evidence is outside the skills pack under
`/home/david_osipov/.local/share/nevrotick/codeql-work/results/rust-audit-20261106/`:
`range-checks/report.json` records command/tool/source identities and the
RED/healthy/GREEN results; individual raw outputs remain beside it. The patch
notice records the helper's purpose, grants and native limitations.

The main AI should integrate only this bounded repair, reconcile the existing
`policy/ui-m0-dependencies.json` derivative hashes and the canonical policy anchor
with its concurrent engine graph edits, then run the affected integrated Cargo
and Windows cross-compilation rows. Native self-comparison/concurrent-mutation
and continued tree-update cases remain in CHECK-22's existing qualified Windows
lane. This audit lifts no Windows, fuzz, storage, GUI-integration or release hold.

### Commit integration — 6 November 2026

The owner requested committing this repair into main. Integration is based on
`cb223c57efb292f3038fc1fd84e1666dcd490a5a`, which already commits the separate nominal-engine
measurement work. Only the AccessKit derivative inventory/allowlist and the
canonical policy anchor are reconciled with that newer dependency policy; its
engine edges, packages, features, grants and manifest/lock identities are retained.
The initial audit receipts above retain their original candidates and limits.
Further local CodeQL analysis is a separate continuation, not a retrospective
pass for the failed product queries. No native Windows or formal execution resumes.

### Cloud continuation — 6 November 2026

The owner stopped local analysis because resources were low. The reviewed runner
corrections and portable queries are carried into active main; the other AI's
rational-seconds/engine work is preserved separately. The
[cloud handoff](codeql-cloud-handoff.md) is the next execution entry point.
Local product extraction/import and all twelve endpoint expectations passed;
the local data-flow evaluation ended with disk exhaustion, and global/negative
query execution remains incomplete. No completed-flow, native or release pass is
established by these setup results. Do not restart local analysis from this note.

## AccessKit password-disclosure applicability — 6 November 2026

The dependency refresh independently reviewed AccessKit
[PR841](https://github.com/AccessKit/accesskit/pull/841) and exact commit
[`d62a49614ffbb0d85ce2d34e0340df711eb86e46`](https://github.com/AccessKit/accesskit/commit/d62a49614ffbb0d85ce2d34e0340df711eb86e46),
included in upstream `accesskit_windows 0.35.2`/`accesskit_consumer 0.39.2`.
The substantive safeguards are in the consumer: mask unmasked password values
and text ranges, suppress word starts and normalize character metrics. The
Windows adapter change itself only replaces its password-role predicate.
Upstream still assigns the toolkit responsibility for supplying masked text;
its repeated-character heuristic is not a complete secrecy guarantee.

**Current disposition: no concrete affected NevroTick password path found.**
The selected Windows entrypoint runs the compiled Fluent Slint UI. Its seven
code-owned editable fields are source label, host, port, source search, custom
minutes and two helper identities
([field owner](../crates/nevrotick-ui/src/ui_drafts.rs)). All use
[GuardedLineEdit](../crates/nevrotick-ui/ui/guarded-line-edit.slint), which exposes
no input-type selector and leaves the stock `InputType` at its Text default.
Inspection of all 33 maintained `.slint` sources found no password-mode binding
or password/credential control. Input/catalog text cannot choose that code-owned
control type. The existing generated `app.rs` contains stock password-icon
branches, which does not establish an application password field; it was
inspected without rebuilding or claiming a fresh source-to-generated binding.

The Slint 1.18.1 AccessKit emitter source also overwrites a text input's value
with its display-derived text before layout lookup and derives text runs from
that displayed layout; `RenderString` masks text in password mode. This is
source evidence for toolkit-side masking, not native leakage qualification.
The older selected consumer 0.38.0 still writes supplied values/text runs without
the new defensive masking. Our local Windows adapter's value getter calls that
consumer, and its text-range `GetText` calls the consumer's range writer. The
existing local SetValue admission, interface-type and range-lock repairs address
different paths and must not be described as password-output protection.

Keep the present local 0.34.0 patch. Selected `accesskit_winit 0.33.2` requires
Windows `^0.34.0` and AccessKit `^0.24.1`; Slint 1.18.1's core also selects
AccessKit `^0.24`. Upstream Windows adapter 0.35.2 instead requires AccessKit 0.25.2,
consumer 0.39.2 and hashbrown 0.17. There is no newer 0.34.x release. Substituting
0.35.2 is an incompatible family/source migration, and that source still lacks
the independent NevroTick ingress/type/lock repairs. This review selects no bump
or backport and changes no licence grant: original GUI/added local helpers retain
GPL-3.0-only; upstream dual MIT/Apache material and additional notices retain
their existing separately selected terms.

**Trigger and owner:** before the GUI owner introduces a password/credential
control, reveal mode, or service-secret value/text-run producer, reconcile
toolkit and consumer masking at that actual producer under existing GUI security,
CHECK-22 and privacy owners. Independently challenge value, text-range and
changed-value/metrics outputs with secret and ordinary-text neighbors; do not
rely solely on a Password role or the upstream heuristic. Qualify the actual
selected Windows/renderer/UIA path when its existing execution gate permits.
The present source finding adds no UI-M0 blocker and lifts no D1–D3, Windows,
backend, package or release hold.

Exact upstream response and a compact source/hash review are retained under
`/home/david_osipov/.local/share/nevrotick/dependency-refresh-20261106-v1/direct-research/password-applicability/`.
This assessment ran no Cargo, native Windows, UIA, solver or product disclosure
test. Reassess after relevant control, toolkit, consumer, profile or secret-flow
changes; the current absence of a password producer is not a general dependency
security certification.


## BH-NATIVE-01 popup cleanup repair — 7 November 2026

David authorized analysis/repair of the supplied logic-bug handoff. Independent
source review confirmed the conditional failure-accounting path: `DestroyMenu`
returning zero became a drained ordinary failure, allowing later cleanup to be
acknowledged without that negative result. No actual leaked handle or native
failure was reproduced.

The actual destruction result now enters the private sticky bridge cleanup
acknowledgement before transient notification. Failed destruction returns the
existing cleanup error; a later successful call cannot clear uncertainty. The
popup entry rejects before another allocation. Existing worker completion,
`try_finish`, `cleanup_confirmed` and Slint `poll_stop` all retain that failure.
Ordinary creation/build/foreground errors remain distinct when destruction
succeeds. No destructive retry, new unsafe block or FFI argument change was added.
Two authored portable tests cover failure/drain/restoration/later-success and a
successful neighboring destruction. They have not executed.

Independent reviewer `bugfix_independent_review`, created with `fork_turns="none"`,
completed source review through the actual popup/worker/host stop chain and found
no material repair defect. A subsequent formatter-only signature change leaves
its behavioral conclusions unchanged. Current source hashes:

- `src/native_tray.rs`: `0ffc9a81df1f418400cead00448b94d517c0992b34e1e388c38e917fc42a8ea2`.
- `src/native_tray/windows.rs`: `14cd1bc3b46c4e7596a29b8344a4ddb7e38c13aef80f0db102a35ea837d8bbca`.

Evidence is retained under
`/home/david_osipov/.local/share/nevrotick/dependency-refresh-20261106-v1/`.
`bughunt-native-source-controls-v1.log` passes14 Python source-custody controls,
not Rust/native behavior. `bughunt-format-v1` retains its formatting-only failure;
`bughunt-format-v2/verification.json` passes admitted boundary controls and full
workspace formatting on377-input `44509d32073d69ea61a30ae4da9c0e838c8c02aa9102d8d90f4ec84d468ce482`,
with unchanged source/tool/vendor bookends. No compiler/test pass follows from
formatting. The working tree and existing staging remain separate; nothing was
staged, committed or pushed by these repairs.

**Still unqualified:** GUI compiler/Clippy/portable Rust tests lack the admitted
host SDK here: read-only preflight found no Fontconfig/Freetype pkg-config entries
or development headers; the retained private Debian13 SDK is not qualified for
this Ubuntu host. No direct-Cargo or altered-SDK fallback ran. Native Windows
execution remains held. When admitted, exercise the real popup-to-worker-to-host
stop chain with controlled failed destruction, healthy wake/subsequent cleanup,
blocked further popup allocation and failed final cleanup acknowledgement, plus
a successful-destruction neighbor. Observe actual USER resources separately.
This source repair lifts no UI/native/package/Windows hold.


## Distinct-range lock-order repair — 7 November 2026

David supplied the remaining-source audit on historical5ef9853 and requested
continuing the bug-fix workflow. Its private `/tmp` artifacts are absent here;
those reported experiments were not treated as current execution. A fresh
read-only reviewer and a separate repair owner inspected the actual unchanged
4427ab42... helper, active `Compare`/mutation paths, cloned same-context ranges,
pinned std futex backend and generated COM agility. Opposite comparisons could
retain different first read guards while queued writers exclude their second
reads. Context tree read guards do not supply serialization. This is distinct
from the earlier same-lock alias repair; no Windows hang was reproduced.

The helper now detects aliases first and uses one guard. Distinct live owners
are acquired by increasing address, with both guards retained through the single
comparison and values restored to original left/right callback order. Addresses
supply only a temporary lock order, never COM/persistent identity. Acquisition
failure skips comparison and releases earlier guards; callback unwind releases
both. Caller casts, same-context/endpoint checks, poison policy, dependency graph,
features, versions, licences and Windows-only role remain unchanged. No unsafe
code, cloning or snapshot replacement was introduced.

**Actual RED/GREEN:** the isolated Linux fixture uses the exact std-only helper
source, the re-admitted shipping1.99.1 compiler and existing verifier sandbox,
cleared environment, read-only inputs, bounded output/processes and60-second
outer command limits. RED retained the actual original production prefix and
added the independent tests: compilation succeeded, seven tests executed, five
passed and exactly the two new acquisition-order assertions failed at native101.
The raw wrapper retained `incomplete` because nocapture panic diagnostics broke
its row parser. Separate `red-v1-reviewed.json` adjudicates the complete raw
records without replacing that receipt or rerunning the tests. An independent
reviewer checked those exact failures and accepted behavioral RED. Assertions
following the first queued-writer failure were not executed in RED.

After repair, green-v3 passes all seven ordinary and seven optimized tests,
workspace-equivalent Rust/Clippy deny rules, formatting and both Windows-target
compiler/Clippy metadata checks with the full configured flag vector. No linking
or Windows execution occurred. Cases cover both operand orientations, physical
acquisition-rank poison, first/second acquisition failure, simultaneous real
protection, alias queued-writer behavior and callback unwind. The forced opposing
writer probes deliberately refuse their second nonblocking acquisitions and
verify common first-owner selection; they do not demonstrate successful blocking
COM comparisons under actual native contention. The healthy noncontended helper
controls preserve callback values/order. Source/tool/driver bookends are unchanged.
Green-v1 retains a formatter failure; green-v2 retains seven ordinary passes and
test-only Clippy failures, subsequently repaired with fallible setup and two
reasoned, narrowly scoped intentional-panic expectations. No failed attempt was
relabelled as a pass.

Evidence root:
`/home/david_osipov/.local/share/nevrotick/dependency-refresh-20261106-v1/bughunt-distinct-range-v1/`.
Final helper SHA256 is
`cb6bb3924d2e6bd99edf7269b2ab6ec59ac42bed9313579f1ba6108b5497b473`;
patch-notice SHA256 is
`907b63541dd48cb12b14e8c66d0780176c174e25d25fe230e093a5c629f03b7e`.
Fresh-context reviewer `accesskit_pair_independent` independently reviewed the
contracts, RED records and final repair/test changes; it ran no jobs or edits
and found no additional actionable defect. Actual native scheduling, compatibility,
full-GUI coverage/mutation, resources and teardown remain open CHECK-22/27 work.
The full Slint build remains subject to its SDK/native holds; this std-only
control establishes no full GUI, package or release qualification.

**Source admission integrated:** only the helper and patch notice changed within
the27-file local derivative. Original archive/revision/source-map/grant fields,
modified-file allowlist, registry identities, lock/manifests and selected
features/roles remain unchanged. The derivative map and code-owned policy pin
were rebound; canonical dependency-policy digest is
`cbce0fe93225659982120df573e98a1276155f2a9f5c11af7c02a8e45885cdb5`.
`source-adoption.json` retains the exact delta and377-input current map
`f6764d23bc6169c6d9d18be6ce4f83de3e4326342efa987294b7d98f9593cfdb`.
Python controls pass14 AccessKit-patch,28 Slint-profile,120 verifier and51 engine
consumer cases. Prior numeric/native receipts keep their executed candidates;
rebinding supplies no new Rust coverage/mutation score. No commit/push or native,
formal, fuzz, storage or runtime-policy waiver follows. Resume the existing
product queue and its pending N1 contract review after this bounded repair.


**Concurrent guidance integration:** HEAD advanced externally fromef3c0ab to
c8df999, changing six testing-skill guidance files. The coordinator read the
applicable testing-contract/index delta; meaningful-repair RED/GREEN and existing
qualification boundaries remain compatible with this work. All377 tested/adopted
product input hashes remain identical. The index was also updated externally;
its new staged state was preserved, not rolled back. This agent issued no stage,
commit or push. `closeout.json` records both HEAD/index identities and scoped reuse:
after helper execution only the unused Slint canonical-policy pin changed among
its Python helper inputs; the actual Rust helper/config/compiler/tool bytes stay
unchanged. Current source admission and consumer controls are separate fresh
checks. These are bounded source/host results, not a new full-product score.


## Text-inventory checker repair — 7 November 2026

The [current tooling follow-up](codeql-cloud-handoff.md#local-text-inventory-and-analyzer-output-repairs--7-november-2026)
records the long-spacing live-control omission, actual full509-ID RED/GREEN,
85 passing text-inventory tests and the real509/509 CLI result. Header parsing
now retains named/live control custody under existing source work limits. Source
checking grants no translated-consent approval, rendered/native/accessibility
pass or new UI scope. The current helper/candidate and retained failures are in
that same record; no additional implementation plan or execution waiver follows.

## AccessKit inclusive word-endpoint repair — 7 November 2026

Accepted the fourth remaining-code review's navigation finding after reproducing
it against the actual `accesskit_consumer` 0.38.0 public API. A valid 256-character
run admits its inclusive endpoint, but `backward_to_word_start` narrowed that
`usize` position to `u8`. The multiword endpoint returned 0 instead of 254.
This is a text-navigation correctness defect under CHECK-22, not demonstrated
memory corruption or a privilege crossing.

The [local consumer derivative](../third-party/accesskit_consumer/NEVROTICK-PATCH.md)
widens the word-start keys before binary search. Root Cargo patch routing and the
lockfile select that derivative. Exact-start previous-word semantics, ordinary
cross-run fallback and input limits remain. Original archive members, immutable
revision/checksum, original manifest and MIT/Apache notices are retained; the
eight new original Rust regression cases carry their separate GPL-3.0-only grant.

Evidence: `/tmp/nevrotick-review-fixes-20261107-z36bdzau/`. The initial fixture
compile error, E0716, is retained as setup failure, not behavioral RED. The corrected
REDv2 fixture compiled, then failed four intended assertions while four neighboring
controls passed. Unchanged test bytes pass after the causal search repair among
all 213 library tests in both ordinary and optimized Linux builds on Rust 1.99.1.
Doctests selected zero tests and supply no additional coverage. Cases include
endpoint 256/neighbor 255, exact word starts, document start, a single word,
word offset 255, UTF-16 surrogate pairs and a second run's actual origin.
The configured namespace boundary control passed; source/vendor mounts stayed
read-only, dependency archives matched their locked checksums, and no host
network, Windows UIA/COM/Narrator or Slint layout execution occurred.

The [dependency admission follow-up](s0.1-source-readiness.md#accesskit-consumer-local-source-admission--7-november-2026)
records the three-patch graph. Vendored packages are excluded from workspace-member
test selection: before changing/removing this consumer patch, run its named tests
in a separately admitted pure host scope with explicit consumer package selection,
ordinary and optimized builds. The existing full GUI command matrix is unchanged;
this repair adds no portable-row admission shortcut or zero-doctest waiver.
Native caller behavior remains with the existing admitted Windows/accessibility
owner. Historical GUI receipts do not qualify the modified combined candidate.

Independent fresh-context review, `review_two_dependency_repairs`, accepted the
scoped correction without finding an introduced defect:

> Independent read-only review found no introduced defect in the consumer correction. The non-narrowing search preserves previous-word semantics at exact starts and the existing cross-run fallback. Eight valid public-API cases retain independent literal expectations, including endpoint 256, neighboring positions, offset 255 and UTF-16 surrogate pairs. Raw REDv2 evidence records four behavioral failures with four healthy controls; unchanged regression bytes pass in the ordinary and optimized 213-test GREEN snapshots. Final source hashes match those snapshots; these results qualify portable consumer behavior only.

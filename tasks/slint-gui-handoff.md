# Slint GUI handoff for the next AI

Prepared 6 November 2026. English working notes; keep product copy readable.
This is a continuation guide, not a second implementation plan. The
[S0.5 task](s0.5-ui-comparison.md) owns candidate history and evidence.
Remote review: [PR10](https://github.com/DavidOsipov/NevroTick/pull/10).

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

Current Slint runtime/build pin: 1.18.1; Rust: 1.99.1. Merged lock SHA256:
`0683420c01ac6af80554100b402be4673628bfb5fca514476cd3585572b9c297`.
Merged schema3 policy anchor:
`b76a65d04a7c99f78ba299c87e39a3ee9172e725604f220f43e2677a0ebb0455`.
The graph adds only main's normal engine→time-domain edge and nine existing
dependency contexts in each engine package scope. Registry identities/grants,
patch pins, workspace and other package scopes remain unchanged. The earlier
GUI41–43 receipts below retain the old graph and cannot qualify this merged one.

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
   process/UI/package/release owners and applicable P0–P7 gates. Do not
   copy headless Win32k/child restrictions into the GUI or add mitigation switches.
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

Historical canonical receipts under private cloud directory
`/workspace/.nevrotick-cloud/slint-uia-fork.2020wssl/`:

| Receipt | Observed result / SHA256 |
| --- | --- |
| `gui41-integrated.json` | Passed format/compiler/Clippy/rustdoc; 736 ordinary tests, 164 doctests, 447 optimized tests. GUI: 183 tests/17 doctests; 143 pre-Cargo controls, 509/509 text consumers, 50 captures. SHA256 `badd0f7ee0cba45fc009065e2add507985b0ba22b35714ad51b13d71091c5dcc`. |
| `windows-source7.json` | Linux-hosted Windows-target all-target type checking and Clippy passed; no SDK/CRT/link/run. SHA256 `27f1e89885a69171fa45d9e225ca50e21cc286dd5fae4e9b922444bf8b027732`. |
| `gui38-dependencies.json` | Both-target graph/scanner checks passed at their admitted advisory snapshot. SHA256 `db98db71574953fe5a09b16c9f55a4e133214d3b9937d3ef2d57a62e9cab4f8c`. Advisory freshness must be re-established for a new dependency check. |

**Fresh PR preparation:** `gui42-pr-portable.json` passed the complete configured
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
passed all143 controls and509/509 coverage. Its365 admitted sources match current
bytes; the optional SDK policy is unchanged and belongs to the full GUI42 row.
GUI43 is a Python-only diagnostic, not another product/native test run.

**Merged candidate, GUI44:** the current ordinary source checkpoint incorporates
main `b808eb7`. `gui44-merge-portable.json`, SHA256
`e72e4ba6f3e15aac80c15a7d3118b3cecd59e4405456e8913cf85a02398b2d0b`,
passes all configured build/Clippy/rustdoc,759 ordinary tests,174 doctests,
470 optimized tests,143 source controls and50 captures. GUI remains183 tests/
17 doctests; main's engine now has47 ordinary/47 optimized tests and12 doctests.
All368 source entries (367 product plus SDK policy) match current bytes. All50
actual capture files remain byte-identical to the reviewed GUI42 lineage.

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
Git. Paths and recorded hashes alone do not make them available on another
machine. Preserve them through an approved artifact channel or reproduce the
affected configured checks. Never manufacture a missing receipt or claim a new
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

1. Review the resolved integration against current main. The b808eb7 snapshot is
   now incorporated; if main moves, reconcile only its new delta. Preserve newer
   engine/source/proof-policy work and the canonical decision identities. Keep
   lock, manifest and graph policy changes with one owner; re-admit affected
   graphs before target execution.
2. Freeze that combined candidate. Repeat affected format/compiler/Clippy,
   behavioral/doctest, generator/consumer/native-source controls and captures.
   Reuse unchanged evidence only after matching all relevant inputs; old GUI
   counts do not establish results for the newer engine.
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

Keep the worktree for PR iteration. A PR request authorizes branch publication;
it does not authorize merging, deployment, Store submission or OS/service changes.

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

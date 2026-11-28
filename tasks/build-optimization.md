# Windows x64 application build optimization

Initial record: 29 October 2026; its executed checks below are historical.
**Current checkpoint, 6 November:** required v3 and the expanded hardening vector
are configured and admission controls are updated; final-image, startup and
native qualification remain open. See the [current amendment](#required-v3-and-maximum-compatible-hardening--6-november-2026).

**Current closure queue, 28 November:** the [bounded artifact and coverage work](#exploit-hardening-closure--28-november-2026)
below imports the source audit at `a95da288d07cb4fc1bf0ff925ec7f453eaa2d70b`.
Earlier build and native receipts retain their original candidate and scope.

David requested heavy x64 build optimization settings, comments in build config,
and documented Windows issues in the maintained docs and repository skills.
This is a bounded build-policy task under the existing
[development plan](../docs/plans/development-plan.md), not a new product milestone.

## Scope and independent expectations

- Owners: workspace `Cargo.toml`, `.cargo/config.toml`, admitted policy/checkers,
  product Windows release-link/export path and direct allocation-observer profile.
- CHECK-19/20 subsets: the actual release profile must select level 3, ThinLTO and
  one codegen unit; `verify-release` inherits these settings. Healthy configuration
  passes, changed/missing optimization settings and mitigation/CPU overrides fail.
- Preserve overflow checks, explicit unwind qualification policy, nonstripped
  matching symbols and Windows CFG/ASLR/DEP. The original qualified configuration
  retains generic x86-64. OD-07's later v3 direction and pending migration are
  recorded in the follow-up below; historical checks do not qualify that floor.
- Windows compiler/Clippy and release cross-link answer build questions. CHECK-21
  native performance, effective mitigations, UI instrumentation and release gates
  remain separate. No faster-code claim, public-network training, tool installation,
  service/clock changes, signing or publication is included.

## Exploit-hardening closure — 28 November 2026

**Scope and authority:** this is a source-audit continuation at
`a95da288d07cb4fc1bf0ff925ec7f453eaa2d70b`, under OD-05/OD-07 and existing
CHECK-16/18/19/20/23/26. It implements the planning handoff from
[architecture §15.4](../docs/windows-time-architecture.md#154-per-process-mitigation-profiles-and-startup-admission)
and [hardening §6.4](../docs/windows-time-development-hardening.md#64-per-process-mitigation-qualification).
The review found coverage and qualification gaps; it did not establish a missing
effective OS protection or an exploitable defect. Build flags, source checks and
historical native receipts remain different evidence layers.

**Current coverage:** target Rust flags reach the selected rebuilt Windows code,
including generated Rust and selected path patches. They do not retrofit host
build scripts/procedural macros, native C/assembly, prebuilt standard libraries/
CRT or OS providers. The current UI links `nevrotick-ui`; Observe links the
Engine, wire, time-domain and Windows-native adapter graph. `time-core` and
`ntp-exchange` have no normal dependency edge into either current workspace
release binary; `windows-platform` belongs to the separate laboratory services.
Keep normal, build and dev edges distinct. The general Roughtime-only and combined
verifier profiles currently supply dependency-lane evidence, not a complete
Windows link/runtime result. The exact source/configuration and actual final
images must determine the next matrix.

The five labels below are local work items, not new CHECK IDs. **Source preparation
is ready when its existing owner reaches the affected slice.** Build, native,
signing and installed qualification require their existing admitted candidates,
environment and phase gates. This queue starts no campaign and does not displace
[development §7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent),
the Windows/fuzz/storage holds or UI-M0 D1–D3.

| Local work / existing owner | Prerequisites and concrete next action | Independent acceptance and finish / re-entry |
| --- | --- | --- |
| **B1 — bind every image to a role**; build/dependency owners, CHECK-20/26; W11-W05 owns the policy record | Source-ready from the exact manifests, selected feature graphs and direct build recipes. Implement §15.4's required manifest through the existing artifact/verification owners, covering every current and planned executable and its actual module/build inputs: UI, Observe, both lab services, the CNG/TLS automatic main plus three declared binaries including feature-gated init-probe, allocation observer, generated native-observer helpers, and future production Engine/Broker/helper/updater entry points. Include selected patches/generated Rust, C/assembly, prebuilt std/CRT/providers, managed hosts and the optional resource DLL. Mark missing production composition as planned. | Independently reconcile the roster with Cargo targets/features, direct recipes and emitted/staged payloads; a dev-only edge or host tool cannot count as linked product code. Every row identifies role, image class, activation owner, source/toolchain/features and required versus observed evidence. Finish the source roster with all omissions resolved or a named dependent blocker; reopen affected rows on target, feature, dependency, generated-input or payload changes. No native coverage follows from enumeration. |
| **B2 — match the PE gate to the artifact**; build/artifact owner, CHECK-19/20/26 | Source-ready after B1 identifies each class. Reconcile [`windows_pe.py`](../scripts/windows_pe.py)'s EXE-only, exact-`0xA00` profile with future independently justified `0x800` static-import profiles and code-bearing DLLs. Resolve [`verify-time-domain-windows-allocation.py`](../scripts/verify-time-domain-windows-allocation.py)'s narrower AMD64/PE32+/DllCharacteristics gate by sharing the appropriate complete reader or explicitly enforcing its narrower diagnostic claim. Preserve the separate resource-only DLL reader. | For each selected profile, independently inspect required load-config/CFG, CET, sections, relocations, import mask and debug/PDB identity beside healthy and deliberately incomplete artifact controls. An eligible `0x800` image must use that declared expectation; no broad mask acceptance or weakened EXE gate. Code DLLs require their own applicable checks; a validated no-code resource DLL gains no executable CFG/CET credit. Finish source enforcement separately from actual emitted-image qualification; re-enter after the checker, profile, linker or final bytes change. |
| **B3 — close native and prebuilt coverage**; build/security with native dependency owners, CHECK-16/20/26 | Source-ready after B1. Record exact C/assembly flags and emitted-object/library coverage independently of Rust flags. The current UI and service C-entry recipes select strong stack protection and CFGuard; determine whether protected pre-CRT frames are emitted and, if so, how their cookie is initialized before use. Assess supported Spectre protection for sensitive native paths and the exact CRT/library variant, including uncovered prebuilt std/native code. | Use compiler/object/source evidence appropriate to each component; a CFG-positive final EXE or absence of literal `/GS` settles neither whole-graph coverage nor a C-stack defect. Distinguish MSVC `/sdl` and Spectre switches from supported pinned-Clang/Rust mechanisms. Finish with implemented coverage or a concrete supported/incompatible/unavailable disposition, owner and remaining evidence; reopen on compiler, object, library or entry-order changes. Missing coverage remains open required work where applicable. |
| **B4 — separate diagnostic hosts and children**; existing allocation/native-observer and W11-W08 owners, CHECK-16/19/20/26/27 | Source-ready after B1 identifies each direct/generated executable, managed C#/PowerShell host and compiler/runtime input. Declare which fixed product flags are inherited, which diagnostic-only CPU substitution is already admitted, and which JIT/interop/supervision requirements belong to the host. Keep Observe and all four CNG/TLS entries in their explicit diagnostic roles; absence of the UI/service startup sequence is an applicability/readiness task, not evidence that OS mitigations are disabled. | Independently compare actual host/child recipes and artifacts with each profile; a protected Rust child cannot qualify its CLR/PowerShell observer. Useful observation, bounded cleanup and diagnostic failure must remain distinguishable from product behavior. Finish the declared source/artifact scope without promoting it to shipping qualification; native observation stays held until its existing gate, and changes to host, generated code or child policy invalidate affected evidence. |
| **B5 — reconcile the final build with installed protection**; build plus W11-W05/W11-W10/S6.4/P6/P7, CHECK-16/18/20/23/26 | Source preparation follows B1–B4; final execution is held on the admitted complete graph, actual packaging/signing choices and applicable native gates. Feed the exact build and artifact identities into the [Windows closure queue](windows11-integration.md#exploit-hardening-closure--28-november-2026), [GUI queue](slint-gui-handoff.md#exploit-hardening-closure--28-november-2026) and existing package/signing owners. Reconcile build, signed, distributed and installed inventories, including supported individual-file signatures and resource-only content identity. | Finish only the evidence layer actually observed. Final closure requires effective per-role policies, healthy operation, required denial and startup/lazy/recovery/shutdown module coverage for exact installed images under real launch owners. Failed, audit-only, unsupported and unexecuted results stay distinct. Re-enter on affected source/feature/compiler/package/signature/OS/hardware/launch changes; no previous fixture result substitutes for the changed candidate. |

When selecting a row, freeze its finite case/work budget in this task and stop on
its stated exit or concrete prerequisite failure; retry only after the relevant
input or justified method changes. [OD-22](../docs/nevrotick-owner-decisions.md#26-od-22--sanitizer-qualification-cadence)
keeps routine sanitizer qualification at coherent module completion and the
applicable product matrix before V1. Such diagnostics contribute their declared
instrumented coverage; they neither postpone required startup integration nor
replace final Windows exploit-protection evidence. No new tool, dependency,
signing provider or verification campaign is selected here.

### B1 first source child — bind the current Windows export pair

After the `e89066b` pull, the selected independent source child binds the existing
`verify.py::export_windows_artifacts` UI/Observe assignments through the existing
verification manifest. `plan-code-change`, `security-build-deployment`,
`evolve-data-contracts`, `qa-contract-tests` and `qa-static-checks` own the source
and checks. The strict configuration reader and direct exporter share one closed
section validator. An alternate source checkout must use its own declaration.
Role names, target/entry references and required features must agree with the
selected Cargo target; unknown, duplicate, substituted or malformed rows refuse.

This first child covers the current workspace release export pair only. The
remaining B1 image roster, per-image/module graphs and full W1 runtime schema
are still open. A fixed pending-qualification state cannot be changed into an
observed/enforced claim. The Rust entry reference does not qualify pre-CRT C,
toolchain or loader execution; its separate owner remains explicit. No native
mitigation setter, target flag, receipt field, signing or package route changes.

Independent plan review required validation from the helper's `source`, preservation
of the existing receipt shape and a precise failure boundary: before PE/PDB reads
or payload/public-output creation. An outer private staging directory may already
exist and keeps its existing cleanup contract. Healthy sealed PE/PDB export must
remain useful beside missing/duplicate/unknown role, incorrect target/entry/
feature, role substitution and false-qualification controls.

Budget: one focused isolated Python checkpoint over the new role controls and
existing Windows export/publication controls, with two causal repair passes and
one final independent source review. Keep actual failures and consumed runs.
These inert source/artifact controls need no Rust build or Windows execution;
the existing WSL-root reserve and earlier UI compiler allocation failure still
block the separate Rust checkpoint. No native, coverage, mutation, sanitizer,
installed or release pass follows from this child.

**Source checkpoint:** the shared strict reader now returns immutable checked
UI/Observe records from the supplied checkout. The exporter reconciles both
records before opening either PE/PDB pair; original receipt keys, complete-pair
admission and publication behavior remain. The record explicitly limits entry
coverage to the Rust source and leaves complete-role qualification pending.
Fresh independent source review found no material issue in this child.

The first isolated Python cohort passed 76 role, strict-policy and existing
export/publication tests in 2.359 seconds. The additional 48 product-admission
tests reached three diagnostic assertions whose expected older guard was now
preceded by the mandatory role check. All three malformed inputs were refused.
The repair assigns the specific new source/entry diagnostic only to those
faults, retains the original diagnostics elsewhere and pairs restoration with
healthy admission. The final frozen cohort passed all 124 tests in 8.603 seconds.
Syntax and whitespace checks passed; no cast, suppression or additional tool was
introduced. These runs used inert artifact bytes and executed no Windows image.

The exact source/control receipt is
`~/.local/share/nevrotick/b1-export-role-bindings-20261128.mcqrhid4/receipt.json`,
SHA-256 `49eec60b8422517212e84e4ade7e9e92f2b4fd5816058e5351a57543a8420ca1`.
It retains the initial failures, final logs, twelve source/input pins, the patch
and the new untracked control file. This is partial B1 enforcement for the two
existing exports. The remaining image roster and module attribution, W1/G1,
actual artifact/native/installed qualification and applicable metric/sanitizer
checkpoints remain open. Changed verifier/manifest inputs require re-admission
of dependent current-candidate recipes; earlier hash bindings are historical.

### B1/B2 allocation-observer pair admission — 28 November 2026

The next independent source child covers the existing test-only allocation
observer, not a new product executable. Its role is a console diagnostic emitted
by `verify-time-domain-windows-allocation.py::build` from
`policy/time-domain-windows-allocation-observer.rs` and the admitted `time-domain`
library/dependency artifacts. The fixed basename is `NevroTick-<uuid>.exe` with
matching PDB; activation belongs only to that script's explicit native lane and
its generated bounded ordinary-user supervisor. Debug, release and verify-release
retain the admitted compiler/std, exact `ENV.MSVC_FLAGS` (`0xA00` loader profile,
CFG/CET and v3) and existing profile flags. No runtime or shipping-role qualification
is claimed. This identifies the artifact needed for B2; the complete B1
machine-readable image/module roster remains open.

| Stage / skills | Required change and useful/refused observation | Evidence / remaining owner |
| --- | --- | --- |
| Scope and independent plan review; `plan-code-change`, `security-build-deployment`; B1/B2, CHECK-19/20/26 | Trace the actual build-export and native-staging calls. Existing full target flags justify the unchanged shared console-EXE `0xA00` reader. | Independent scouts agree the class is source-ready. No shared PE-mask relaxation or Windows execution is needed for this source child. |
| Producer/consumer evolution; `evolve-data-contracts`, contract enforcement | Admit one bounded EXE/PDB snapshot pair through `windows_pe.py` before retaining either artifact; retain the admitted bytes/hashes and matching GUID/age. Native admission must recheck both original artifacts before creating its Windows directory or dispatching. | Outer build/native evidence advances to schema 2. Schema-1 receipts remain historical and cannot authorize the new native consumer; allocator counter JSON retains its separate schema 1. |
| Focused controls; `qa-contract-tests`, `qa-test-planning`, `qa-static-checks` | Independent complete PE/PDB healthy controls beside the old 160-byte header, missing CFG/CET/relocations/loader protection, wrong subsystem/RWX, missing/swapped/stale symbols. Observe refusal before retention or native staging. | One focused RED/GREEN cohort plus one final existing allocation/PE regression checkpoint, at most two causal repair rounds and one fresh source review. No compiler, native, fuzz, mutation or sanitizer execution is selected. |

The existing shared reader bounds and source/tool admission stay mandatory.
Static declarations and matching symbols establish neither effective OS policy
nor absence of allocator faults. Actual emitted-image and supported-Windows
qualification, complete B1 module attribution and affected metric gates remain
with their existing owners. This budget is independent of the closed P5 UI
checkpoint and does not reopen its test-build retries. Symbol generation is
configured but actual PDB filename/page-size evidence is unavailable in the
retained old artifacts; a new pair must pass the fixed default reader profile.

**Source checkpoint:** the actual build now admits the full console EXE/PDB
pair through the unchanged shared reader before retaining either file. It writes
the admitted snapshots and records both hashes plus CodeView/PDB GUID and age.
Native admission rechecks the original pair and frozen record before creating
its Windows directory, then stages only the admitted executable bytes under its
new name. Original symbols remain retained on Linux and are checked again at
final readback. TerminalServerAware remains optional. Shared PE/PDB and immediate
control sources are now included in the explicit initial/snapshot/bookend input
map; the independent reviewer caught and resolved the initial reader-pin omission.

The admitted RED specimen showed the old header-only reader accepting a 160-byte
flag-bearing file despite the new complete-artifact expectation. The focused
cohort then passed 25 controls. The first broader run reached 124 tests with two
sparse-file fixture setup errors at the harness's 64-MiB file ceiling; this was
not a product refusal or a pass. The corrected harness permits 128 MiB for those
known boundary specimens while retaining 512-MiB AS, 90-second CPU, 120-second
wall, one CPU, read-only source and credential-free/network-isolated execution.
No product/compiler resource limit changed. A direct native-receiver case also
reproduced an untyped `RuntimeError` on a cyclic artifact path; the receiving
guard now returns `Incomplete` before staging.

The final combined allocation/PE/Windows-export controls passed **126 tests in
10.346 seconds**. They include actual native-receiver refusal before directory
creation/dispatch and both normal staging and post-admission replacement of the
original EXE/PDB paths. Dispatch is mocked; no Windows program was executed.
Afterward, two explicit declaration annotations were added. Their normalized
executable AST matches the retained tested source; syntax/whitespace and source
identity were refreshed, and independent review confirmed the annotation-only
delta. This justifies reuse of those behavioral observations, not new runtime
validation from types. No mypy/compiler/coverage/mutation result is claimed.

The source/patch/log/identity receipt is
`~/.local/share/nevrotick/b2-allocation-pair-20261128-x_rsxm6l/receipt.json`, SHA-256
`cef20326e9ebe2d70f41eb826d653fa3072736b9c5c5fb46690d9f5eafa39982`.
It preserves setup failures, REDs, both final attempts and the pre-annotation
tested runner. Fresh independent final source review found no remaining scoped
material issue. The finite source/checkpoint budget is consumed. This completes
this B2 allocation-admission source child; actual new MSVC emission, PDB naming/
page-size and supported-Windows outcomes remain unqualified. Complete B1
machine-readable image/module attribution, other B2 profiles and all applicable
native, metric, sanitizer, signing and installed-product gates remain open.

### B3 pre-CRT stack-cookie observation — 28 November 2026

The next qualification question is the existing conditional requirement for the
two separately compiled C entries: determine whether the selected strong stack
protection emits cookie-dependent frames before CRT startup. This is component
artifact evidence, not another whole-product build or a source rewrite.
`security-build-deployment`, the unchanged CPU-entry contracts,
`qa-environment-preflight`, `qa-test-planning` and `qa-positive-paths` own it.

Reuse the retained UI `entry.obj` and service `service-entry.obj` only after their
recorded compiler/library pins, exact compile flags, current C/header bytes and
object hashes agree. The independent source review found the no-LTO C inputs
unchanged; new Rust/catalog inputs do not change these exact objects. They do
invalidate any broader final-image or runtime reuse. Inspect the complete symbol
and relocation records with the already pinned LLVM reader. Calibrate observation
with one benign address-taken stack-buffer helper compiled under the same Clang
Windows object flags, once with strong stack protection and once without it.
The latter is a control artifact only; no product protection is removed.

Budget: one tool/source/object admission pass, two control-only Clang compilations
and four object readbacks, at most 30 seconds per command / 180 seconds total,
one CPU, 1-GiB AS, 15-second CPU, 128 descriptors and 8-MiB output-file ceiling.
Use credential-free, network-isolated read-only inputs and one owned work root.
Stop on a pin/binding failure or non-discriminating controls; no install, Cargo,
Windows execution, mitigation mutation or automatic retry is selected. A current
object with cookie references requires a separate causal startup-order repair;
absence closes only this conditional observation for those exact C objects.

**Observed result:** the pinned-source/tool/object admission passed. Both
control compilations and all four complete COFF readbacks returned zero in
1.111 seconds total. The protected control contains relocations to
`__security_cookie` and `__security_check_cookie`; its unprotected neighbor
contains neither. Neither exact retained product entry object contains those
symbols or `__stack_chk` references. Independent raw COFF review confirmed the
symbol/relocation observations. This closes the conditional cookie-reference
question for the two C objects only. It does not qualify final-image startup,
CRT behavior, every native dependency or Windows runtime mitigation. Product
strong stack protection remains selected; no entry-source change was needed.

The full source, tool, receipt and object bindings, commands and readback logs
are retained under `~/.local/share/nevrotick/b3-precrt-20261128-ltv7_51k/`.
`receipt.json` SHA-256:
`d5525d519c8e8cbfe2b17d383ed64be2b7287e7b088fd7d8d16cd68ee212e225`.
The UI object is `eef45628c5d35e6faa1dae6954da700043b7a54a777bf27dd2d1b0a5adbb2b7b`;
the service object is `4e6b70fb7b0ffc723c0f617dd2a724da9accdd8ab2ed22dbca5905feab023575`.
The finite budget is consumed, with no Cargo build, Windows execution or product
protection change. Complete B1 module attribution and native/final-package
qualification retain their existing owners.

### B1 laboratory service role binding — 28 November 2026

The next source child binds the two existing standalone laboratory service
targets in the same verification manifest, through a separate closed section.
`plan-code-change`, `security-build-deployment` and contract/regression QA own
CHECK-20/26 attribution. Reuse the immutable role record and bounded source
reader; retain the workspace UI/Observe section's exact two-role contract.
Both service entries remain fixtures with pending complete runtime qualification.
The cross-link receiver must validate current bindings before setup and the
copied read-only source before compilation, then record each admitted image's
binding and the selected ordinary or guarded preentry owner. Library-test
artifacts cannot receive service-role attribution.

Independent expectations cover the real export record, missing/duplicate/crossed
roles, substituted package/entry/features, redirected source and false runtime
claims, with ordinary and guarded healthy exports. The existing graph, source,
PE/PDB and publication checks remain mandatory. Budget: one focused source-test
cohort and one final combined role/export regression checkpoint, two causal
repair rounds total, one fresh independent source review, 120 seconds per test
command, one CPU, 512-MiB AS and 128-MiB output-file limit. No Cargo compilation,
Windows execution or metric campaign is selected. Required source identities
change and need fresh admission before a later real cross-link.

**Source checkpoint:** the separate closed manifest section and immutable rows
now bind the actual service package, declared binary paths, empty required
features, console/fixture classification and pending runtime qualification.
The cross-link owner checks the original source before cache/work setup,
rechecks the copied source before compiler dispatch and uses those copied rows
in each retained artifact record. Ordinary and guarded routes retain distinct
selected preentry owners; library-test artifacts receive neither service field.
Existing source bookends, graph, PE/PDB and publication checks are retained.

An actual mocked export produced the intended pre-change RED because its record
lacked `source_role`. The focused role cohort passed 16 tests. The final combined
role/strict/service-export/PE/verifier/text-generation run reached 349 cases:
348 passed and one kernel-limit control failed because the outer harness's hard
ceilings prevented its declared child limits. A targeted rerun of that same
control passed with the original soft limits and sufficient hard ceilings for
its exact 4/8-GiB AS, 300-second CPU, task/descriptor/file probes. No compiler or
package code runs in that control. Its parent limits remain unchanged, as the
test independently checks. The earlier import-only setup failure without a
private HOME and both failed logs are retained; neither is product RED or GREEN.
Fresh independent source review found no material issue in the combined patch.
The bounded source/checkpoint budget is consumed.
After execution, the new `baseline_entry` test parameter received an explicit
boolean annotation. With postponed annotations retained, its executable AST is
unchanged; the separate annotation readback binds tested and current bytes.

Raw outputs, the original RED, runner, source hashes and patch are retained at
`~/.local/share/nevrotick/b1-service-roles-20261128-1brasep9/receipt.json`, SHA-256
`cc4de55725bb526f29fb7507aba9f7069c277e21464129ce9a7ca012a7974251`.
These are source and mocked-artifact observations. A fresh graph/source
checkpoint and cross-link must admit the changed inputs before real emitted
service images can carry the new record. Complete B1 module attribution,
B3 prebuilt/native coverage, effective policy, final package/signing and
installed-service qualification remain open.

### Standalone strict-checker role-source custody — 28 November 2026

The independent `78bbbda` review identified a current B1 verification-integrity
gap: standalone `check-strict.py` validates four workspace role sources outside
its snapshot/hash/final-recheck set. `qa-failure-triage`, contract implementation
and regression selection own the causal repair. Capture those bounded no-follow
inputs separately from parsed TOML configuration, validate the captured bytes,
record their digests and recheck them before standalone success. The broader
product verifier's inventory is retained; no product-export bypass is inferred.

Use actual standalone `main()` controls with environment/tool inspection mocked:
unchanged source, each role input changed after admission, changed live source
after capture but before validation, and restored healthy source. No Cargo or
native work is selected. Budget: one focused RED/GREEN cohort, one affected
combined strict/role/export regression checkpoint, two causal repair rounds,
120-second wall/90-second CPU per test command, one CPU, 512-MiB AS and 128-MiB
file ceiling. Independent source review must reconcile captured/validated/hashed/
rechecked identities. The accompanying eight allocation-test return annotations
are a separate clarity-only edit, with AST comparison and no new runtime tests.

**Observed closeout:** actual standalone `main()` returned success after an
owned UI manifest changed during mocked environment inspection, and its healthy
receipt lacked the four role hashes. The new controls retain both pre-change
RED observations. The repaired standalone path captures bounded immutable source
bytes, validates that capture, emits `role_source_sha256` and rechecks those
same paths through the bounded no-follow reader before passing. Configuration
parsing still sees only its TOML roster. Default product/service callers retain
their live source admission and existing outer custody guards.

The focused cohort passed 18 cases. The final affected strict/role/service-export/
product-export cohort passed all **173 tests in 25.280 seconds**, including the
captured-before-validation mutation, each later edited role source, restored
healthy cases and malformed supplied snapshots. A fresh independent source
review found no material issue. Environment/tool inspection is mocked in the
standalone controls; this is no Cargo, Windows or final-artifact qualification.
The finite repair/checkpoint budget is closed.

Exactly eight allocation-test methods introduced by `78bbbda` now declare
`-> None`. Postponed annotations remain enabled, and an AST comparison with
only those annotations removed is identical to the reviewed source. No method
body, argument or runtime assertion changed; no annotation-counting or new
behavioral test was added for this clarity-only correction.

The existing B1 evidence root retains the RED/GREEN/final raw logs, source map,
patch and annotation comparison in
`~/.local/share/nevrotick/b1-service-roles-20261128-1brasep9/strict-custody-receipt.json`,
SHA-256 `96325e1ce2596b94992a26acb8019e154ce08735cd18adcfe1c1fba5813e09d3`.
The review's catalogue-migration issue is addressed in `fa1d7dc` and `b4ae328`
as recorded in the GUI handoff; current hosted product execution retains its
own result and must not be inferred from the commit titles or these Python tests.

## Candidate and custody

Initial HEAD: `bd78e324f697ae9492e9c567b5234d408e5eb636`, with concurrent staged,
unstaged and untracked implementation work. The review captured task-owned input
copies and the existing index digest under
`/home/david_osipov/.local/share/nevrotick/build-optimization-9wgak8ww/`.
The task does not stage, commit, refresh historical receipts or alter dependency
versions. Only build-input hashes affected by this profile change are refreshed;
inherited mismatches remain explicit rather than silently admitted.

## Changes and source-backed constraints

The canonical policy and primary sources are in
[hardening section 6.5](../docs/windows-time-development-hardening.md#65-windows-x64-optimization-policy-and-evidence-limits).
The cited Rust guide builds the compiler itself. Its linked unstable DLL/ThinLTO
issue does not establish a blanket failure of ordinary Cargo application LTO.
Conditional PGO needs representative native execution and compatible profile
tools; its effective flags must retain target mitigations. BOLT's documented ELF
inputs provide no adopted Windows PE lane. CPU specialization and cross-language
LTO require separate admitted experiments. Dev build settings remain unchanged.

## Executed checks and remaining evidence

- RED: the new explicit-profile expectation and Windows release-link expectation
  both failed through their real configuration/command owners; the existing
  healthy policy control passed. Observer release/verify-release parity assertions
  separately failed before adding LTO/codegen flags, with debug healthy.
- GREEN: 127 focused strict-policy, environment, product-verifier and
  allocation-observer unit controls passed after configuration changes. These are
  guard/accounting controls, not product/native performance observations.
- `python3 -B scripts/check-strict.py --workspace --fixtures`: passed; four workspace
  members adopt policy and 37 isolated fixture lanes passed, including positive
  release arithmetic, expected overflow rejection and inherited `verify-release`.
- `python3 -B scripts/verify.py --lane portable` with the admitted Linux vendor
  copy and private `/tmp` work: passed format/compiler/Clippy/rustdoc, **93 ordinary
  tests, 51 doctests and 66 optimized tests**, none skipped. The package counts and
  commands are retained in `portable.json` under the evidence directory above.
- `python3 -B scripts/verify.py --lane windows` with the same vendor copy and new
  private output: passed Windows compiler/Clippy and the actual release UI link.
  `windows.json` retains the `--release` command, executable/PDB hashes and profile.
  Both files are preserved under `windows-artifacts/`. This is cross-compilation
  and linking; no Windows executable was run by this task.
- Read-only `objdump -p` on that emitted PE64 reports `HIGH_ENTROPY_VA`,
  `DYNAMIC_BASE`, `NX_COMPAT` and `GUARD_CF`; raw output is in
  `windows-pe-headers.log`. These are emitted-header observations, not effective
  runtime-policy or indirect-call enforcement qualification.
- Skill package checker: **59 skills passed**; checker regression controls:
  **25 passed**. All **245** checked local Markdown link targets exist, the new
  section anchor resolves, TOML profile policy matches and `git diff --check`
  passes. No `docs/SHA256SUMS.txt` exists; no bundle-integrity claim is made.
- Independent source review found no material issue in profile/flag admission or
  release/PDB export. PDB custody is the unique expected companion from a fresh
  build plus hashes, not an independent PE/PDB GUID comparison.
- At the initial build-settings handoff, strict and Windows receipt input hashes
  matched that source/config snapshot;
  the captured Git index digest remains unchanged. Historical receipts retain
  their original candidate identities. No staging or dependency-version changes
  were performed by this task.

Remaining evidence: representative native Windows performance/resource comparison,
new-profile native memory/runtime-mitigation observations and the normal later
package/release gates. The pre-existing time-domain fuzz dependency-policy pin
already differed from this task's captured baseline; it was deliberately retained.
Its owner must re-admit that graph before claiming a new fuzz result. This task
refreshes only root-manifest/dependency-policy identities directly changed by its
build settings and leaves inherited analysis gaps explicit.

## LLVM settings follow-up

David additionally requested the best LLVM settings for this workload and a written
record. Research selects a defensible baseline, not a demonstrated global optimum:
the existing O3/ThinLTO/one-CGU profile, bundled backend defaults and no additional
raw LLVM arguments or custom pass pipeline. Source-backed decisions belong to
[hardening section 6.6](../docs/windows-time-development-hardening.md#66-llvm-settings-for-this-workload);
the performance skill's measurement reference owns the small future native
comparison matrix and diagnostic controls.

The admitted compiler's binary digest matched policy. Metadata commands confirmed
Rust **1.99.1**, commit `58cf8f9c35b355199727627d07ae34a4e2846d67`, embedded LLVM
**23.1.1**, and the MSVC target's default `x86-64` CPU. Pinned source establishes
normal optimized vectorization/linker behavior; the actual compiler help confirms
supported diagnostics, removed `inline-threshold` and available raw option names.
Help defaults alone do not establish the effective options of every pipeline.
Raw outputs and the compiler hash are retained in `llvm-settings/` under the same
external evidence directory. No application code or native workload was executed
to obtain this metadata.

This follow-up changes prose and build comments only. It preserves every parsed
Cargo profile and target flag, along with the dependency/CPU/ABI/security contract.
Earlier executed build receipts retain their original byte identities; reuse of
their semantic settings requires the recorded TOML equality check, not relabelling
them as fresh builds. Native performance remains unmeasured.

Follow-up checks: the 59-skill package checker, strict workspace-policy admission,
whitespace checks and all 198 scoped local link targets passed; the new section
fragments resolve. Parsed `Cargo.toml` and `.cargo/config.toml` are identical to
the captured pre-follow-up settings. Independent documentation review found no
material issue; source links were reconciled to the exact compiler/LLVM commits.
The captured index digest is unchanged. Receipts are in `llvm-settings/`.
No application rebuild, benchmark, native run, tool installation or dependency
change was performed for this prose/comment-only follow-up.

## Windows ThinLTO issue review

David challenged the Windows choice with #109114 and broader #27438. Fresh
independent research confirmed that the compiler-selfbuild reproducer distinction
is correct but incomplete: ordinary ThinLTO applications also had related import
failures (#81408), and #27438 includes native-data import concerns independent of
LTO. Both cited issues remain open in the inspected records.

The actual pinned compiler contains later #129079/#122790 handling and a native
regression test exercising ordinary ThinLTO with both MSVC linkers. This was
verified in source, not executed by this task. The retained UI artifact imports
Win32/CRT DLLs and no Rust DLL; that narrows the specific #109114 trigger but proves
no general runtime immunity. No current-candidate miscompilation was demonstrated.

Decision: retain the existing parameters solely as an unqualified optimization
candidate under the requested tuning work. Replace the overly strong
"best-supported baseline" wording, expand the canonical caveat and build comment,
and keep explicit native `"off"` versus `"thin"` correctness/performance comparisons
before Windows release qualification. A conservative non-LTO baseline is a valid
alternative policy, not a demonstrated fix for all DLL-import bugs. No compiler,
linker, LTO mode, CPU requirement, panic behavior or security setting changes here.
Evidence and pre-edit copies are retained in `lto-windows-review/` under the same
external directory. Historical build/native receipts retain their original scope.

Fresh independent review preferred `lto="off"` as a conservative baseline until
native qualification, while recognizing that the pinned in-process ThinLTO path
is technically defensible as an experiment. The coordinator retains the existing
candidate because later pinned handling/regression coverage materially changes
the historical failure analysis; no current failure or measured winner is shown.
This disposition grants no Windows release approval or residual-risk acceptance.

Review checks passed: strict workspace-policy admission, the 59-skill checker,
all 39 scoped local link targets and whitespace checks. Parsed Cargo settings are
unchanged, comment-affected manifest input hashes match, and the Git index digest
is unchanged. No application/native test or benchmark was rerun for these prose
and input-identity changes.

## Required v3 baseline and runtime-dispatch follow-up — 30 October 2026

**Status:** owner direction and implementation plan recorded; CPU configuration,
runtime fast paths, native compatibility and performance qualification pending.
David first proposed v2, then explicitly revised the baseline to **x86-64-v3**.
[OD-07](../docs/nevrotick-owner-decisions.md#11-od-07--x64-baseline-and-runtime-optimization)
is the final accepted direction. Earlier generic-build observations above remain
historical results for their exact flags.

The maintained [research](../docs/supporting-reviews/x86-runtime-optimization.md)
records Rust function multiversioning and OS-aware detection, cumulative processor
levels, pinned compiler feature queries, mitigation-vector custody, numerical
constraints and a supported/unsupported native matrix. The [architecture](../docs/windows-time-architecture.md#182-dependency-and-build-controls)
and [hardening §6.7](../docs/windows-time-development-hardening.md#67-v3-baseline-and-runtime-cpu-dispatch)
now require a v3 migration and measured higher-feature optimization. The
[execution workstream](../docs/plans/development-plan.md#81-x64-baseline-and-runtime-optimization-workstream)
assigns build, platform/package, kernel, independent QA and performance owners.

Next action: at the next single-owner build-policy checkpoint, prepare a v3
candidate with coordinated checker/control and input-binding changes. Preserve
CFG/ASLR/DEP, checked arithmetic, fault policy, symbols and the current graph.
Qualify a lower-baseline-compatible prerequisite before starting v3 payloads;
missing AVX OS state rejects v3, while missing only AVX-512 state selects usable
v3. Then profile a real admitted workload and compare narrowly specialized
AVX-512 functions against the v3 path. AVX2 is already in v3. No global v4,
unsafe pure-core exception, relaxed FP arithmetic or automatic provider admission
follows from the decision. Existing CHECK-19/20/21/25/26 remain.

The original local [implement-cpu-dispatch skill](../.agents/skills/implementation/implement-cpu-dispatch/SKILL.md)
provides implementation/review instructions; the existing performance skill owns
measurement. It distinguishes pure selection controls from actual supported
instruction execution, and a feature probe from a demonstrated optimization.
No compiler flag, Rust function, dependency, native setting or historical receipt
is changed by this documentation/skill amendment. Record actual implementation
and qualification here when reached; do not infer them from the plan.

Documentation/skill validation: a fresh-context independent review found no
material policy, safety or plan-consistency gap. The new leaf passed the skill
validator; the package checker admitted all 61 leaves and its 25 regression
controls passed. One fresh-context use exercise repaired a pure selector model:
the original failed 80 of 128 independently checked feature/baseline/enablement
cases, while the resulting artifact passed all 128. Fixture costs were supplied
facts; no SIMD instructions or product benchmarks were executed, and no general
agent-quality improvement is inferred from this one exercise. Link/anchor and
whitespace checks cover this amendment; the root docs checksum manifest remains
absent. Task evidence is retained under
`~/.local/share/nevrotick/cpu-policy-20261030-ruh2q4my/`.

The subsequent external-feedback review clarified full-v4 bundles versus narrow
kernels and Rust's existing atomic feature cache. The research and skill now
distinguish cached CPU discovery from an application's cached kernel choice;
typed owner/optional `OnceLock` selection is preferred when useful, with measured
lookup cost and cold/concurrent initialization checks. No blanket five-feature
attribute rule, per-packet-syscall assumption or custom `AtomicPtr` requirement
was adopted. The earlier 128-case exercise covers selector safety, not shared
cache initialization or native instruction execution.

## Hidden-caveat research and skill review — 30 October 2026

Three fresh-context read-only research assignments covered CPU/OS/VM and Windows
admission, compiler/ABI boundaries, and cache/performance lifecycle. The
coordinator also checked vector memory/arithmetic and pinned-version limitations
using Context7, primary Rust documentation and installed intrinsic source. No
product code, CPU flag, dependency, native setting or historical receipt changed.

The skill now routes to [seven caveat groups and concrete controls](../.agents/skills/implementation/implement-cpu-dispatch/references/x86-caveats.md):
inherited flags/DLL startup and direct entry paths; cfg/helper and optimized-code
boundaries; callable/SIMD ABI; AVX512VL/OS state and Windows CONTEXT semantics;
OnceLock initialization and platform cache lifetime; per-length and sparse-work
profitability; and vector tails, arithmetic and rejection precedence. Sources
support named platform/compiler observations, not universal penalties, affinity
requirements or a newly discovered compiler defect.

A fourth fresh-context agent used the revised skill on a review/repair exercise.
It rejected the supplied v3-compiled checker, deleted specialization, reentrant
initializer and out-of-bounds tail-load proposals, while accepting the supplied
exact AVX512F-only feature boundary under its stated preconditions. Its repaired
pure Python model passed the coordinator's 72 ordered-length-pair cases; the
original passed 54 and failed 18 because it cached a first-request cost decision.
The agent also reported 300 individual checks on its larger exercise selection.
These are fixture policy facts and source arguments, not hardware measurements,
executed Rust cache/ABI/load controls or general agent-reliability evidence.
Coordinator imports created bytecode caches during grading; source-input hashes
remained unchanged and no grader answers were exposed in the agent's input scope.

The current skill/package/local-link checks pass. Evidence and source identities
are retained under `~/.local/share/nevrotick/cpu-caveats-20261030-pg3xgcyy/`.

## Required v3 and maximum compatible hardening — 6 November 2026

**Authority and scope:** David explicitly confirmed that the Windows product's
minimum is **x86-64-v3**, required its immediate configuration and instructed
agents to deliver every applicable hardening. This implements OD-07 and clarifies
OD-05; it supersedes the earlier pending-migration instructions above without
rewriting the historical generic-build results. Base: main
`66f69b37d27309ca0882b6fb237b922c16f4ddbb`, isolated worktree/branch
`fix/v3-required-hardening-20261106`. Existing worktrees remain untouched.

**Changed enforcement:** `.cargo/config.toml` and `check-environment.py` now
select v3, full CFG, disabled Rust switch jump tables, ASLR/DEP, explicit
relocation/large-address prerequisites, CET plus strict-CET image declarations,
the `0xA00` static-import search mask and fatal linker warnings. The exact
supported vector and primary-source exclusions are owned by
[hardening §6.2–6.3](../docs/windows-time-development-hardening.md#62-target-specific-windows-compilerlinker-settings).
The pinned bundled-LLD/xwin lane is the selected linker route; another linker
needs admission rather than silently deleting an unsupported switch.

The common configuration validator is used by environment inspection, actual
`verify.py` product admission and the prepared CodeQL admission consumer.
Previously, product admission hashed Cargo configuration without checking this
exact vector, while CodeQL's duplicate comparison omitted the build/test budget
tables. Both now consume the same complete contract. CodeQL execution remains
deferred; fixing its policy consumer does not activate hosted extraction.

**Independent expectations:** the complete selected v3 profile passes the
configuration boundary; generic/v2/v4/host-native replacements, missing required
flags, metadata-only CFG, CET relaxations, broad/empty import-search settings and
trailing overrides fail before a compiler or target process starts. Controls
exercise the real environment and product-admission entry points. The existing
Rust CI job now explicitly runs the environment controls and repository skill
checker inside an offline, empty-environment sandbox with read-only inputs,
bounded scratch storage and process/resource/time limits. Product-admission
controls remain in its existing selected test file.

**Policy reconciliation:** active architecture, tooling, plans, UI/Windows task
handoffs and repository skills now agree on mandatory v3 and final-product
hardening. Untested or unimplemented protection is **open required work**.
Record exact toolchain, platform or necessary-feature exclusions with evidence
and an owner; a weaker supported release profile needs David's decision.
Hardware-conditional CET must be enforced on its supported class without
silently making CET hardware a new universal CPU floor. Runtime fallbacks may
disable optional higher-feature kernels, never replace the product's v3 minimum.

| Existing owner / acceptance | Remaining observation and exit |
| --- | --- |
| Build/artifact; CHECK-19/20/26 | Cross-compile/link the actual graph with the pinned toolchain; inspect final CFG instrumentation/tables, ASLR/DEP/relocations, both CET bits and the real `DependentLoadFlags` field. Linker acceptance alone cannot pass. Old artifact/signature/configuration receipts remain historical. |
| Platform/package; S0.6/P6 and CHECK-19/20/26 | Implement a lower-baseline-compatible prerequisite that rejects unsupported CPU/OS/VM state before any v3 payload, including direct/service/update launch. Observe a valid v3 baseline, below-v3 rejection and masked/disabled vector-state cases; an illegal-instruction crash is not admission. |
| Loader/package; CHECK-16/18/20/23/26 | Freeze the actual CRT/native/import closure and protected executable directories. Tighten each role to `0x800` when app-local executable imports are unnecessary, otherwise record the exact required modules. Exercise legitimate cold/lazy loads and harmless forbidden-path canaries; static imports do not replace dynamic-load or package-identity checks. |
| Runtime/security; CHECK-16/18/23/26, W11-W05/W11-W10 | Implement and qualify the strongest applicable per-role runtime/package profile, including Strict CFG, supported strict CET, ACG, selected CIG, image/extension/handle/heap controls and compatible service Win32k/child restrictions. Record real SCM/UAC/package activation, effective fields/audit state, normal work and denied operations. Preserve GUI-required Win32k and approved activation. Missing production adapters are not implemented by flags. |
| Numerical/performance; CHECK-21/25 | Preserve checked arithmetic, unwind/callback contracts, symbols and normal FP semantics. Qualify new code generation and native correctness/resources before claiming a speedup or complete release pass; keep useful measured higher-feature work under §8.1. |

**9 November lab prerequisite preparation:**
[`tools/windows-sandbox/cpu-os-v3-admission.rs`](../tools/windows-sandbox/cpu-os-v3-admission.rs)
is the selected std-only diagnostic for the next owned-Sandbox witness. Its
direct-build recipe derives baseline x86-64 from the exact product flag vector
for this probe only, preserving the other hardening/profile/lint settings. It
checks complete v2/v3 bits and guarded OS XCR0 state, with three positive and 30
negative embedded controls. Source, format/Clippy/rustdoc, baseline linking and
static PE/import readback are independently accepted. David installed the paired
x64 VC++ runtime in the guest, but the subsequent launch was refused by
application-control policy (guest exit 4551). No CPU admission or native control
execution was established; dependent v3 witnesses remain unexecuted. Retain the
block under [W11 §4.19](windows11-integration.md#419-owner-ready-sandbox-scope--3-november-2026).
The production prerequisite row above remains open: this tool does not establish
installer/service/update launch enforcement, masked-VM execution or release
support. Root CPU configuration and validators remain v3.

**Historical policy-checkpoint verification boundary:** local source/diff and configuration consistency checks
are possible, but local target-code execution is unavailable: bubblewrap cannot
create the required network namespace and unshare cannot establish its UID map;
Rust/Cargo are also absent from the inspected PATH. No local RED/GREEN, native
execution, PE observation or benchmark is claimed. The policy test was prepared
before its repair and its expected discrimination inspected in source; that is
not an executed failing test. Exact automated results belong to the associated
PR's checks and review record. Required failed/missing checks cannot be reported
as passes.

That policy change neither refreshed old proof receipts nor ran formal work.
Its native-stop statement is historical; current scoped Sandbox authorization
and the new application-control refusal follow W11 §4.19 above. OD-10's
post-V1 deferral, S134 block, separate fuzz/storage holds and UI-M0 scope remain.
Source preparation is authorized; final release still needs
the existing nonformal product, native, package and owner-approval gates.

### Interrupted-work continuation and hosted harness correction

The resumed review recovered published candidate
`6a698337b59a199992a90d61802bf81d3385cb3e`, tree
`1b301c7c6cd365876525b4575962e865a35db802`, exactly matching the retained staged
changes. [PR #12](https://github.com/DavidOsipov/NevroTick/pull/12) owns its hosted
checks and final integration. A separate worktree merged main's dependency
refresh `cb0a4ec254cf45e6efd7f49079b4bdb6e071dd35` without conflicts; the three
overlapping documents retain both changes. Independent guard and Windows-source
reviews found no material defect in their bounded scopes. The integration review
also checked the new compiler minimum and dependency-policy bindings; none of
these source reviews executed product or native code.

The [first hosted Rust run](https://github.com/DavidOsipov/NevroTick/actions/runs/42282353428)
failed in the new isolated environment-control step: 24 tests ran, with six
subcase errors in the environment-override test. Clearing the environment removed
`TMPDIR` before Python's first temporary-directory lookup, while the sandbox had
no standard `/tmp`. Those errors occurred before the intended policy rejection;
they are a harness failure, not behavioral RED. An initial `/tmp` symlink into
the existing scratch tmpfs let those controls reach their intended rejection,
but the [second hosted run](https://github.com/DavidOsipov/NevroTick/actions/runs/42282841308)
then exposed two receipt-fixture path-identity failures: the fixture retained
`/tmp/...` while the real CLI correctly resolved it to `/scratch/tmp/...`.
The final harness correction mounts the single 64 MiB scratch tmpfs directly at
`/tmp`, with its private home at `/tmp/home`. It supplies the standard fallback
without a path alias. It adds no host mount, network access or writable repository
path and preserves all resource limits and every test assertion. Fresh
final-candidate hosted results remain authoritative in the PR; this record does
not predict a passing rerun.

Local isolation remains unavailable: the resumed bubblewrap preflight failed
opening `/proc/5/ns/ns` before repository code ran. No unisolated test substitute,
local compiler run, PE inspection or Windows qualification is claimed.

### Merge and final hosted verification — 6 November 2026

David merged [PR #12](https://github.com/DavidOsipov/NevroTick/pull/12) as
`5ef985388d3350b707e91be1b78e8f8e9039f6ff`. GitHub reports a valid signature,
parent `cb0a4ec254cf45e6efd7f49079b4bdb6e071dd35` and tree
`721baa7b4d012a645fdb9710f3dfe997ad8426a5`, exactly matching the inspected local
integration and tested PR merge tree. The final PR head was
`c091c697a672d3d4a4dd26fbe6ecc2d7bb214501`.

Both required PR checks completed successfully:
[UI-M0 portable model](https://github.com/DavidOsipov/NevroTick/actions/runs/42283219785)
and [Security gate](https://github.com/DavidOsipov/NevroTick/actions/runs/42283219754).
The subsequent main-push [Rust product checks](https://github.com/DavidOsipov/NevroTick/actions/runs/42285216535)
and [Repository security](https://github.com/DavidOsipov/NevroTick/actions/runs/42285216718)
also completed successfully on that exact merge. CodeQL remained explicitly
deferred; a passing Security gate supplies no CodeQL SAST result. These results
close the hosted harness continuation without transferring portable checks to
Windows linking, final PE, runtime mitigation, CPU prerequisite or package
qualification. Those existing required owners remain open.

## SBOM admission follow-up from source review — 7 November 2026

**Status: OPEN, bounded admission/evidence gap; no demonstrated exploit.** The
documentation/source recheck of main `b4f78a5864d0e994b577a5494233220c45319ec4`
compared [the SBOM helper](../scripts/generate-sbom.py) with the owner's hostile
JSON/CBOR rule. The helper reads regular reports with a 16 MiB per-file,
64 MiB aggregate and 128-report budget, then calls plain `json.loads` and checks
the consumed CycloneDX structure and exact Cargo inventory closure. It retains
the original report bytes for output. These are real controls, but decoded
object validation cannot recover duplicate-key distinctions, and no explicit
raw nesting/node policy is established by that source. Its own validator
correctly disclaims full CycloneDX schema validation.

The pinned generator runs offline against a private snapshot, source JSON is
excluded from generated reports, and source/lock custody is checked. The review
did not establish a contributor-controlled path to arbitrary malformed generator
output, inspect the generator's entire producer path or execute it. A recursion
failure before output publication is a failure, not successful admission. The
catalog-specific closed-schema rule is not silently extended into a claim that
this helper already implements a complete CycloneDX validator.

**Owner and exit:** build/release under CHECK-19/20/26 and P6 S6.4, using the
existing [package task](windows11-integration.md#48-owner-selected-hardened-msix--3-november-2026).
Before claiming complete raw-report admission for an SBOM/release candidate,
trace the actual pinned producer and define the applicable original-encoding,
duplicate-key, nesting/node and consumed-field contracts. Challenge the real raw
report boundary with independently specified healthy and malformed/overbudget
neighbors; observe atomic rejection, bounded work and unchanged accepted output.
Repair demonstrated deviations in that owner rather than weakening the hostile
input rule or using successful inventory comparison as parser qualification.
This note adds no formal-work campaign or native/release authorization.

The current receipt and tooling guidance now name the selected complete
native/resource/package and submitted/Store-delivered MSIX reconciliation.
That scope-label correction performs no new inventory generation and closes
neither this raw-admission follow-up nor the complete-product SBOM gate.

**Bounded raw-report source continuation — 29 November 2026.** On the
`c91107846e7a5941cc1856f55d26e799457f6de4` checkout, the SBOM wrapper
now checks strict UTF-8, decoded duplicate field names, finite and bounded
numbers, nesting, node count and the consumed CycloneDX field shapes before
retaining a report for publication. The existing 16 MiB per-report, 64 MiB
aggregate and 128-report budgets remain. The added limits are depth 64,
262,144 JSON token starts and 128 characters per number. The wrapper still
publishes the original admitted bytes and does not claim full CycloneDX schema
validation. `scripts/generate-sbom.py` hashes to
`a609512d7e7a50d51ca6a204d0b79b05039477f8d63161550a6a7c74774027d9`;
the focused test file hashes to
`920632f1dee3207208be2b0f8ccb0d72511b4bf54afb193b1d67fcb5670fed90`.

The real two-member generator fixture first reproduced the gap: adding an
escaped-equivalent duplicate `version` field to one emitted report allowed
the old wrapper to create output. After the repair, the same case refused
after one other healthy report was checked and left no output directory.
Restored reports were retained byte for byte. The focused command
`timeout 110s python3 -X dev -Werror -B -m unittest discover -s scripts/tests -p test_sbom.py -v`
passed all 16 cases in 56.013 seconds, including original Cargo graph,
target and feature controls and new malformed/healthy boundaries.
An independent source review found no new defect in the decoder and checked
its exact 64-depth, 262,144-node and 128-character number boundaries with
pure calls. Focused permanent controls for those actual boundaries were then
added; six `ReportAdmissionControls` cases passed in 0.086 seconds. The same
full focused command was repeated on the resulting test file and passed all
17 cases in 53.395 seconds, including the actual pinned generator fixtures.
`git diff --check` passed. The reviewer also identified pre-existing graph
topology and component-multiplicity limits in the wrapper's set comparison;
the pinned producer was not shown to emit those shapes. Write failure after
output-directory creation can still leave partial files. These remain
separate complete-graph/publication checks. This is source and local fixture
evidence only; native/generated/package inventory reconciliation, signing and
installed MSIX qualification remain open under their existing owners.

## Cargo intermediate-layout upgrade check — 9 November 2026

**R4-B-02: future compatibility watch; no current failure or upgrade selected.**
The [fourth Rust review](../.agents/plans/2026-10-30-skill-quality.md#fourth-rust-research-disposition--9-november-2026)
checked the official [beta Cargo 1.100 changelog](https://doc.rust-lang.org/beta/cargo/CHANGELOG.html#cargo-1100-2026-11-12),
dated 12 November, after this research date. It changes intermediate artifact
layout; current Rust 1.99.1 remains pinned. Recheck the released behavior when
admitting a later compiler. This is separate from the existing `build.build-dir`
and publication-age guidance.

The actual consumers include `verify-time-domain-windows-allocation.py`'s
legacy custom-build/deps path validation, `verify-core-analysis.py`'s build-script
`out_dir` validation and `copy_contract_dependencies`, and the ordinary engine/
time-domain analysis collectors' `target/debug/deps` enumeration. Some consume
Cargo JSON yet still assume old path shapes. Their current strict rejection
protects evidence integrity; final top-level EXE/PDB exports are not established
to break. No candidate compiler or consumer was executed by this research.

**Owner/trigger:** existing build/toolchain admission CHECK-19/20/26, coordinated
with the ordinary time-domain/Core/engine contract owners at the next admitted
upgrade. Keep old healthy parser fixtures and add actual new-layout JSON from
that compiler, with wrong package/host-target role/profile, redirected/escaped
path, substituted bytes and incomplete inventory as rejection neighbors. Use one
bounded admitted build reaching a library, host proc macro and build script,
then the smallest affected external-consumer check. Discover artifacts through
messages while retaining identity, root containment, inventory and byte custody.
Do not resolve incompatibility by removing checks or accepting arbitrary paths.
Native allocation observations retain their independent gate; formal work and
its deferred consumers remain after V1. No parallel campaign is created.

## Rust 1.100 candidate admission — 11 November 2026

**Research prepared; no toolchain upgrade or candidate execution.** The
[dated release review](../docs/supporting-reviews/rust-1.100-upgrade-review.md)
adds language/library/lint/Windows/tool coverage to R4-B-02 above. Its inspected
notes are still a candidate for 12 November; rolling beta docs may already
identify another release. Keep the actual 1.99.1 product baseline and all
existing execution holds. CHECK-19/20/26 and the existing affected product/native
owners retain this work; no parallel upgrade campaign or new prerequisite for
unrelated current implementation is created.

**13 November handoff — receiver source checkpoint before the requested upgrade.**
The owner requested finishing the current work before another agent upgrades and
tests Rust 1.100. The [conditional numerical receiver](engine-transport-pilot.md#conditional-numerical-receiver--source-development-13-november-2026)
is implemented on the retained 1.99.1 baseline, with 594 product inputs,
409 ordinary tests (228 KE), format and GNU/MSVC-target Clippy passing in its
exact recorded scope. Its actual caller supplies `None`; the checked explicit
model remains a component control. No production eligibility, Core transaction,
clock effect or native execution is added. The final source closeout retains
its actual compiler-contract/Python/staged results in that same task.

Start the upgrade from the signed receiver checkpoint in local/remote `main`,
after confirming its clean Git state. Keep the existing owner/hold boundaries.
The older KE analysis role still describes `964a9573` (593 inputs, 400 tests /
222 KE) and its old helper/source hashes. It is closed and intentionally cannot
admit this changed receiver; the 607 outcomes/E9 remain historical. Before using
the role, re-admit and bind the complete new compiler/source/helper/roster and
reconcile generation. Include the fixed roster checks in
`validate_engine_ke_execution` and `_admit_engine_ke_debug_retirement`, the new
privacy compiler matrix and diagnostic fixtures, role/dossier bindings and
relevant analysis-tool identities. Do not substitute a count edit or stale
receipt for actual discovery, execution, custody and independent review.
Optimized tests, current doctests/compiler controls as applicable, 90% file-line
coverage, 95% per-owner mutation and generation gaps remain required; record
what the final source closeout actually completed before selecting reruns.
No old metric result transfers across the compiler/source change.

At the next admitted toolchain checkpoint:

1. Reconcile the final 1.100 release/tag against the review, including omitted or
   changed entries. Freeze an exact compiler/Cargo/rustfmt/Clippy cohort, source
   and lockfile, targets/features, installer/manifest and binary identities.
   `beta` alone is not a version: inspect `rustc -Vv` and `cargo -V`. An available
   newer beta cannot supply a 1.100 result under that label.
2. Inventory the coupled controls before changing any pin: `rust-toolchain.toml`,
   workspace `rust-version`, `policy/tool-manifest.json`, environment admission,
   `.github/security/prepare_rust.py` and the CI toolchain-directory paths.
   Decide MSRV separately from the build compiler rather than raising it
   incidentally. Obtain and verify real new distribution/compiler/LLVM/tool
   identities; preserve wrong-release/commit/LLVM, substituted-byte and source-
   override rejection controls in `test_tool_admission.py`. Never manufacture
   hashes or loosen admission so an unqualified candidate can run.
3. Extend the existing R4-B-02 collectors and parser fixtures with candidate
   layout evidence. Include host proc-macro/build-script and target-library
   identity, missing/multiple artifacts and escaped/redirected paths. Preserve
   original healthy controls and copied byte custody. A separately pinned Kani
   verifier already knowing another layout is not a product pass; leave formal
   execution and its deferred consumers after V1.
4. With the admitted candidate and unchanged graph first, select the existing
   format/compiler/Clippy/rustdoc/doctest and contract suites that cover the
   review's inference, trait, macro, lint, Unicode/output and ABI changes.
   Include generated code, explicit vendored regressions and host build tools.
   Match negative tests to intended diagnostics with a legal-use neighbor;
   retain meaningful expectations instead of bulk suppressions or automatic
   snapshot updates. Compare effective test-harness arguments/discovery.
5. Run the affected native Windows/MSVC boundary, mitigation and resource
   qualification only in its authorized environment when its prerequisites and
   holds allow. Rebuild consistent final artifacts; source/host/cross-link
   success is not native correctness, compatibility or performance evidence.
   Retain the v3 floor, overflow checks, panic policy, complete hardening vector,
   symbols and numerical/security contracts. New allocator APIs, sample PGO and
   Windows conveniences remain optional separate proposals, not upgrade repairs.
6. Record exact candidate, commands/results, inherited versus new failures,
   review dispositions and unresolved native/companion-tool checks here. Promote
   only after the required current gates pass; retain a rollback baseline and
   never relabel old receipts as results for the new compiler. Diagnose genuine
   dependency incompatibility before any separately scoped graph change.

Documentation-only verification is separate from these planned acceptance steps.
This amendment performs no build, beta installation, global-toolchain switch,
format sweep, dependency change or native/formal execution.

## Rust 1.100 implementation — 13 November 2026

**Owner/request:** David requests upgrading Rust 1.99.1 to 1.100.0. This is the
execution child of R4-B-02 / CHECK-19, CHECK-20 and CHECK-26 above, incorporating
the supplied independently reviewed final-release handoff; it creates no new
campaign or acceptance owner. The final-release reconciliation is appended to
the [existing review](../docs/supporting-reviews/rust-1.100-upgrade-review.md#final-release-implementation-follow-up--13-november-2026).

**Starting candidate/custody:** clean HEAD
`a25696b8bf67a53ebbcf40c4c0ba43c613159ff1`, 2,078 tracked input files.
The complete initial file/index/HEAD identities are retained in
`~/.local/share/nevrotick/rust1100-20261113-v1/baseline.json`.
`Cargo.lock` SHA-256 is
`2ccabde33a800b745c42b4fe96dc4a6c300b1961ac87e149d313cae41fde5f2e`.
No dependency, patch, feature or product-semantic change is selected.

**Declared minimum decision:** raise workspace `rust-version` separately to
1.100.0, matching the exact build/CI admission cohort. This avoids advertising
continuing minimum-version qualification on a second compiler without its own
ongoing lane. No new standard-library API is required. Preserve the installed
1.99.1 cohort and starting commit for rollback; historical receipts remain
bound to their original compiler/source.

**Bounded execution/independent expectations:**

1. Admit the authentic distributed 1.100.0 compiler/Cargo/rustfmt/Clippy and
   matching host/Windows components. Update coupled live pins, CI/setup and
   ordinary-analysis pointer/inventory, preserving separate analysis compilers
   and deferred proof consumers. Wrong release/commit/LLVM, substituted bytes
   and ambient source overrides must still fail before product execution.
2. Repair the six reached artifact consumers using message-derived identities
   and actual new-layout evidence alongside old healthy layouts. Wrong package,
   host/target role or profile, escaped/symlink paths, missing/ambiguous artifacts,
   incomplete inventory and changed bytes must fail. A shared narrow parser may
   serve the ordinary collectors; final EXE/PDB exports remain separately checked.
3. Run affected admission/preparation/collector/strict tests, then the admitted
   default and explicit `engine-ke-association` dependency/portable/Windows rows
   as prerequisites permit. Linux host and Windows cross-builds establish their
   own scopes only. No direct Cargo fallback follows a failed preflight.
4. Keep the changed Engine analysis role closed until complete W2R source/helper/
   roster/role/dossier rebinding. Preserve historical 400/222, 607/E9 and current
   409/228/34/69 receipts without metric transfer. Its capture-forwarding control,
   optimized/coverage/mutation/generation exits remain with the existing owner.

The coordinator owns graph, task/review and integrated verification; separate
workers own cohort admission and artifact collectors; independent review checks
the resulting candidate. Existing resource/native Windows, fuzz, storage and UI
holds remain. Formal execution and pending proof-tool repairs remain post-V1.
Actual results, source/tool identities, limitations and next action follow here.

### Cohort and reached compatibility results

The WSL cohort is installed and environment/complete `stable-core` admission
passes: Rust/Cargo **1.100.0**, compiler
`0ef1bc386059596dada8fb65b50425aab2ea637a`, LLVM/LLD **23.1.5**,
rustfmt **1.11.0**, Clippy **0.1.100**. The distributed manifest SHA-256 is
`3051e859570f64a8c5acb9d09e87425db52c7d97287c67e8f6d57d7d34ef9258`.
The complete stable inventory has 3,975 entries, 1,110,579,075 regular-file
bytes, SHA-256
`ee3099ad012b8b779bb7b3e4d6c5b2bc735009e535d2add2c51127af010c0d90`.
Bundled LLD SHA-256 is
`9fe419ad20d121580a5289b36b4c32a1d4ce13cf6c5d113d71d890a1a43cdeba`.
The previous compiler, inventories and private installation pointer have preserved
rollback copies. The native Windows installed inventory remains **1.99.1**;
only its setup default and the WSL cross-target cohort change here.

The existing strict gate passes **47 strict + six pure-API lanes**. Cargo 1.100
exposed the synthetic workspace's absent `nevrotick-ui` profile override, then
rustc rejected two needless `Future` qualifications. Both RED receipts remain;
the fixtures now name their actual adapter and use the edition's prelude type.
Real product profile values and warning enforcement are unchanged. The actual
offline workflow gate passes all three workflows using pinned actionlint,
ShellCheck and pedantic zizmor, including two healthy/three rejection controls.
Exact command outputs, focused Python suites and tool/pointer bookends reside in
`~/.local/share/nevrotick/rust-1.100-admission-20261113/`.

Fresh metadata acquisition used a private mount namespace and a dedicated
512-MiB/16,384-inode tmpfs; it exported 2,083 metadata files, preserved the lock
and retired its mount. The default and explicit KE dependency rows each passed
28 lanes, including Linux/Windows graph and warning-free cargo-deny checks, on
their retained source/helper candidates. The deliberate root MSRV change was
independently checked as the sole parsed manifest delta; only its manifest hash
and the canonical code-owned dependency-policy pin were rebound. All package,
feature, licence and patch records remain unchanged. Receipts are in
`~/.local/share/nevrotick/rust1100-20261113-v1/`.

The ordinary product snapshot currently has 594 inputs, canonical candidate
`a514f0dcfc898bce49c9502515341c80cd6e80f0a5af82504ed17495b1567a72`.
The full portable default row passes patched-dependency controls and formatting.
Its initial cold 12-job compiler command failed with rustc/LLD thread-creation
`Resource temporarily unavailable` under the unchanged 128-task sandbox. The
small causal repair uses the existing six-job budget for cold workspace compiler,
Clippy/rustdoc and corresponding Engine/Windows links. Command-construction RED
controls and 154 focused verifier/profile Python controls pass. A real six-job
retry clears that failure and reaches the inherited missing `fontconfig.pc`
prerequisite (`yeslogic-fontconfig-sys`); the full portable row remains incomplete.
The retained Debian-13 optional SDK also mismatches this WSL host's four runtime
hashes. No ambient SDK, dependency or admission exception is substituted.

The six artifact boundaries have old/new-layout and rejection controls. A real
Windows debug library build emits libraries, host proc macros and build scripts
in the new layout, after a reached preflight caught and repaired the old LLD pin.
Its collector then exposed an inherited full-lockfile unique-name assumption;
the repaired closure resolves exact locked versions/sources. Separate replay of
the immutable successful build output attributes and independently copies 28
artifacts, preserving original bytes. This is parser/copy evidence; a fresh
complete observer build and other current execution rows remain separately
recorded below. Core build-script output admission follows the actual emitted
`build/<package>/<hash>/out` shape. The shared ordinary collectors validate each
observed unit and its complete files; wholly omitted unrelated units still need
an independent selected closure and are not claimed covered by Cargo JSON alone.

The time-domain ordinary contract driver stops before compiler execution: its
nested `ntp-wire-analysis` policy hash is already stale at starting HEAD, and its
166-input role predates the current 594-input snapshot. The Engine legacy role
likewise declares 168 inputs. Their source/role admission remains open. No
unrelated fuzz metadata repair, count-only bypass or old-campaign resurrection
is made. The changed KE role's complete W2R, capture-forwarding discriminator,
coverage/mutation/generation and native obligations remain open. No product Rust,
lockfile, verification inventory or hardening configuration change follows.

### Final bounded handoff

All results below bind the final 594-input product candidate
`a514f0dcfc898bce49c9502515341c80cd6e80f0a5af82504ed17495b1567a72`;
each receipt also retains its actual helper, tool, vendor and configuration
identities. Product Rust source, lockfile, v3/mitigation configuration,
verification inventory and the closed KE campaign policy remain byte-identical
to the starting candidate. The workspace MSRV and its exact manifest/policy
admission are deliberately changed as recorded above.

| Executed row | Actual result and boundary |
| --- | --- |
| Final default dependency row | **Passed**, 28 lanes, Linux/Windows metadata/features and cargo-deny. `dependencies-final-default.json`. |
| Final explicit KE dependency row | **Passed**, 28 lanes, same unchanged graph with the separately selected prototype. `dependencies-final-engine-ke-association.json`. |
| Full portable default | **Incomplete** at the inherited missing Fontconfig prerequisite. Format and reached patched-dependency controls pass. The six-job retry clears the earlier thread-creation failure. `portable-default-v3.json`; no prototype portable retry against the identical missing SDK. |
| Windows explicit KE row | Workspace compiler and strict Clippy **pass** for all seven members, including Slint. Release UI rustc is then killed by signal 9 during LTO; a matching-time kernel record reports `CPU Watchdog Timeout (hard)`. The existing 90-CPU-second cap remains. UI release/export and the later observe release link are **incomplete**; no full default Windows repeat against the same release blocker. `windows-ke.json`, `kernel-cpu-watchdog.jsonl`. |
| Fresh allocation observer, debug | **Passed**, all seven stages: boundary/compiler/format, actual library build, sealed dependency boundary, observer Clippy and Windows link. Exactly 28 attributed artifacts are copied and sealed. `allocation-debug-v3/verification.json`; emitted AMD64/PE32+ image SHA-256 `9a69ae18128dbbac5c9a654068537f8fd556694bb8c3bafde480f2d25d8c3da9`. No native execution. |
| Engine Windows libtest build | **Passed**, including actual new-layout executable selection and separate PE checks for test/control/launcher/CPU images. `engine-libtest-build/verification.json`; no native lane. |
| Fresh bounded Engine compatibility | Actual **409 ordinary + 409 optimized** tests, **228 KE** each; GNU/MSVC Clippy, **69 doctests** (56 compile-only) and **34 ownership/privacy compiler controls** (six healthy/28 intended refusals). Exact names/causes match the independently retained roster and new freeze. **Aggregate incomplete**, as explained below. |

The task-owned Engine recipe adapts the existing source-closeout/resource owner,
binding all 594 product inputs, 24 reached helper inputs, authentic tool/vendor
identities and unchanged Rust source to fresh recipe/freeze v2. It uses the
unchanged 3-GiB RAM, 512-MiB swap, 512-task outer caps, inner sandbox limits,
four-GiB startup headroom, finite 600/660-second child/parent budgets and
768-MiB aggregate allocated-output cap. Earlier prepared-only freezes remain
historical; no previous compiler execution or metric result is reused.

The optimized command itself exits zero and executes the exact 409 cases, but
its **after-command** storage check observes **846,266,368 bytes** against
**805,306,368 bytes** (768 MiB). The raw workload and resource receipts remain
incomplete, `qualification_passed=false` and `campaigns_reopened=false`.
Independent offline adjudication verifies the actual ordinary/optimized names,
doctest classifications and intended compiler diagnostics without converting
that failed resource gate into a pass:
`engine-ke-compatibility-v2-offline-adjudication.json`, SHA-256
`b274b20b40f38d8f75c20e5d7e6cf660cb60544460692917100687c7aef1fa65`.

Observed Engine peak RAM is 1,612,906,496 bytes, swap 20,766,720 bytes and
51 tasks/one Cargo process; no OOM or PID-limit event occurs. Lowest observed
host available memory is 3,265,007,616 bytes. Independent live readback verifies
the unit and cgroup retired, no recorded PID/start identity remains alive and
no observer/cleanup error occurred. Exact input bookends and private-source
restoration pass. This supplies bounded behavioral/compiler evidence, not W2R,
coverage/mutation/generation, native or release qualification.

Final focused Python evidence: shared collector 154, Core 358, Engine 125,
time-domain 29, allocation 19 and native selector 19 tests pass. Affected Core
and Engine cases were rerun after the last shared-helper changes. Cohort,
strict, workflow, cold-command and linker controls retain their separate
receipts and overlaps; no pooled test or product-completion percentage is used.
Independent integrated review found no remaining material defect in this bounded
patch. `git diff --check` passes; nothing is staged, committed or published.

**Next single bounded action:** keep R4-B-02 as owner and prepare a resource
recipe that retires only verified disposable profile caches between the completed
compiler/ordinary and optimized phases, or separates optimized execution into
its own admitted slice, while preserving source/raw evidence and the 768-MiB cap.
Retain this first failure; do not delete evidence or raise the cap to manufacture
GREEN. Then finish the existing SDK/release resource admission and complete the
W2R source/helper/roster/dossier binding before current metric campaigns. The
Linux Fontconfig profile, UI release CPU budget, ordinary analysis roles and
all native/fuzz/storage/UI/post-V1 proof holds retain their existing owners.
No further heavy campaign is running at handoff. Rollback remains the coherent
preserved 1.99.1 cohort plus starting commit; revert only this task's changes if
needed, without discarding later work.

## Accepted 14-day dependency cooldown — 13 November 2026

**Authority/owner:** the new supplied implementation handoff records David's
settled **“14 days it is”** decision, now [OD-19](../docs/nevrotick-owner-decisions.md#23-od-19--14-day-registry-dependency-cooldown).
Continue R4-B-02 / CHECK-19, CHECK-20 and CHECK-26; no second campaign or new
package-identity programme. This supersedes the earlier optional/no-delay
status for this policy only. Fourteen days needs no further owner approval.

**Custody/prerequisite:** HEAD remains `a25696b8bf67a53ebbcf40c4c0ba43c613159ff1`.
Preserve all 37 dirty files from the preceding Rust upgrade. The complete
starting worktree hashes/status/index are in
`~/.local/share/nevrotick/cooldown-20261113-v1/baseline.json`.
The installed and admitted Cargo is 1.100.0, source `3acc79c0c841844675bc53a605922dbfc441e750`;
its compiler/MSRV selection and outstanding qualification remain above. This
policy adds no real dependency upgrade and changes no MSRV, provider or product
Rust behavior. The accepted lock remains SHA-256
`2ccabde33a800b745c42b4fe96dc4a6c300b1961ac87e149d313cae41fde5f2e`.

**Bounded contract and implementation sequence:**

1. Preserve exact whole-config admission and Windows v3/hardening while adding
   `registry.global-min-publish-age = "14 days"` and
   `resolver.incompatible-publish-age = "deny"`. Refuse global/named-registry/
   resolver age environment overrides, including empty/equal values, using
   names-only diagnostics. Preserve ancestry/home/legacy refusal.
2. Reuse the existing schema-3 graph owner before archive/vendor publication
   and frozen verification. Grandfather only the exact accepted graph. New,
   imported, manual or bot lock changes need guarded baseline reproduction,
   complete graph/delta comparison and verified publication-time admission for
   every new registry identity, including host/build transitives. Unknown age
   remains unadmitted unless an individually recorded OD-04 exception applies.
3. Share config/cwd/ancestry checks with locked metadata refresh. Keep its lock
   byte-identical, no crate downloads and dedicated filesystem quota. This
   refresh is not graph-update approval or a cooldown attestation. Use fixed
   reviewed argv and cleared environments; one urgent acquisition child may
   receive `allow` only under an exact concrete decision covering the whole delta.
4. Run isolated sparse-registry controls with real admitted Cargo: old/young
   direct and transitive neighbors; exact-young refusal and eligible control;
   locked/missing-timestamp coverage limits; per-registry precedence; transient
   override followed by normal refusal; publication, frozen/metadata and cleanup
   controls. Fixtures execute no package code and contact no public registry.
5. Update maintained owner/review/skill/source/tooling/CI surfaces, and record
   configuration, route integration, fixture qualification and product/native
   status separately. Existing resource/native/fuzz/storage/UI/proof holds remain.

The coordinator owns graph integration, refresh and maintained documents; separate
workers own config admission, sparse fixtures and the shared changed-graph owner.
No direct Cargo or relaxed-admission fallback follows a missing prerequisite.
Results and remaining gaps follow after the integrated candidate is frozen.

### Cooldown completion receipt

**Decision recorded:** OD-19 is accepted and implemented in this bounded tooling
scope. **Configuration/guards implemented:** exact 14 days/deny; CPU/test-thread
budgets, complete v3/Windows hardening vector and whole-config equality are
preserved. The shared `require_cargo_context` performs bounded unchanged config
reads, rejects inherited age overrides by name only, and checks canonical Cargo
home plus ancestor/home/legacy shadows. Acquisition and verification reuse it;
CI explicitly unsets only its inherited manager selection at the affected direct
entrypoints. Arbitrary command/configuration overrides are not accepted.

**Relevant update paths integrated:** the current schema-3 owner requires
`cooldown_admission`, with an exact pre-policy lock/registry checkpoint and no
retrospective age claim. Product execution and both direct/CLI vendor preparation
must pass the actual graph decision; acquisition rechecks it before publishing.
Changed graph records reconstruct the complete direct/transitive registry delta,
retain raw publication rows/checksums and compare the entire reproduced lock,
including local records and dependency edges. Unknown ages fail ordinary approval.
The small code-owned checkpoint attaches each accepted graph/admission hash and
prior exception references, so later updates start from the current accepted
baseline and do not reclassify its unchanged previously approved young versions.
No new audit database or full provenance inventory is created.

The new structured metadata-only `dependency_cooldown.py` producer covers scoped
precise updates (including safe qualified selectors), manifest/feature/target/
patch resolution, independent initial workspaces and imported-lock comparison.
It binds candidate manifests/config/tool/source before and after acquisition,
uses fixed argv, an isolated cleared child and the existing dedicated quota, and
exports only new lock/decision/result files through an owned directory descriptor.
It never writes the accepted checkout/policy/vendor. Output path aliases, extra
or escaping filenames and oversized payloads fail. The production urgent-exception
map is **empty**; a JSON assertion of approval cannot grant an exception. Future
concrete OD-04 approval binds the entire delta and only one precise-update child
receives `allow`. Parent/subsequent normal environments remain 14/deny.

The only earlier live Cargo update route, metadata refresh, remains exactly
`cargo update --workspace --locked` followed by RustSec acquisition. Its new
context/cwd binding and drift checks do not turn it into graph approval or
cooldown attestation. Source/download/export bounds, identical lock and no-crate
checks remain. Frozen CI/verification consumes the approved graph. No updater,
bot, real dependency/provider change, cargo-install route or global config is added.

**Isolated qualification and current checks:**

| Boundary | Executed result |
| --- | --- |
| Real admitted Cargo sparse fixtures | **2 tests / 14 Cargo calls pass** across two bounded groups: old/young direct and transitive selection/backtracking, exact-young refusal beside eligible control, safe qualified selector, one-child exception/complete delta, locked-young retention, subsequent fresh normal refusal, missing timestamp and registry precedence limits, and frozen offline metadata. Three calls intentionally return 101; eleven return zero. 15 + 17 HTTP requests remain inside the isolated namespaces; zero cooldown archives or compiled artifacts. |
| Original sparse metadata/archive control | **1 test passes**, retaining metadata-only refresh versus explicit inert archive-fetch discrimination. This archive positive control is separate from the zero-archive cooldown cases. |
| Graph/admission/producer/custody controls | **28 tests pass**, including sequential approved updates with a retained young exception, forged/missing raw publication rows, complete delta/edge comparison, generated/imported lock mismatch, manifest/config/source/tool drift, oversized-before-read and canonical output bypasses. Five controls exercise the actual `resolve(args)` dataflow while substituting only Cargo acquisition and quota capacity. |
| Final focused environment/graph/profile/archive/refresh/Cargo-policy suites | **204 tests pass** together. Environment/context has 36; Slint profile 33; Engine profile 19; archive/publication 30; metadata refresh 37; Cargo policy 21; the graph 28 above is included, not extra. |
| Product admission/result accounting | **123 tests pass**, including mandatory age-boundary refusal before execution. Legacy archive/source-profile fixtures explicitly isolate the higher age authority; separate real-boundary missing-decision, manual-lock, post-download drift and healthy baseline controls prove it. |
| Environment + strict configured entrypoints | **Pass**. Strict has 47 fixture lanes + six pure-API lanes; no compiler/MSRV/cohort change is made by this policy. |
| Security/workflow/skill validation | **48 Python security tests pass**; the actual three-workflow actionlint/ShellCheck/pedantic-offline-zizmor gate passes with two healthy/three rejection controls; **66 skills** pass metadata/link/inventory checks. Hosted CodeQL remains deferred. |
| Current frozen dependency rows | **Both pass, 28 lanes each**, Linux/Windows default and explicit Engine-KE graph scopes. They consume the unchanged approved lock/vendor and fresh admitted advisory snapshot; they do not prove fresh selection age or execute package code. |

Meaningful RED receipts remain: the previous acquisition entry reaches its
transport boundary with no age decision; the new entry refuses before network
and preserves the accepted-baseline healthy neighbor. Metadata refresh previously
accepted inherited/shadowed age settings and config drift; three new rejection
cases plus a positive context receipt now pass. Early paired dependency rows
completed their Cargo checks but correctly returned incomplete when the last
publisher guard changed bound helper bytes; final rows use fresh destinations
and unchanged input bookends. No earlier result is silently relabeled.

The earlier Rust upgrade left six-job command expectations stale in the Engine
profile tests. Their historical command digests remain unchanged: controls now
independently require only the documented eight cold-build rows to use six jobs,
normalize just that difference for historical comparison, and retain every other
command/flag/test-selection assertion. This fixes the test expectation inherited
by this policy slice; it changes no production command here.

**Final identity/custody:** source candidate
`e7cd91df3737cac9f5f820d0228179698603ee76d876dfcb6a89d11bddbc12a1`,
595 product inputs. Graph helper SHA-256
`9ce605317da2e4054614e385c54e3d3ab4a5f1efbb9d3f97a37fefcb1e5c2f94`;
shared environment checker
`df124d3c05865882fda7a18ff6b5c6e70d76e453a0196e777fa39ca0aa6a5b27`;
Cargo config
`0f56f93b35b5d85a490afe3d62f53511f5697465b0d18564ffee805b761232a5`;
dependency policy raw SHA
`30c5b238af6a94d379348b71341e4d6422a32f82fd458d7821fb3a2603d3aac4`.
The package/source/feature/grant/patch policy data differs only by the new
`cooldown_admission` subrecord; the existing independent canonical policy pin
is updated coherently. Lock SHA remains `2ccabde33...f2e` as recorded above;
291 compiler/MSRV/Rust/lock/verification/closed-role files match this slice's
initial dirty snapshot. No staged content, commit, push or deployment is made.

Exact commands/exits, RED/GREEN logs, source maps and current frozen receipts
are in `~/.local/share/nevrotick/cooldown-20261113-v1/`. Final real-Cargo qualification
is `~/.local/share/nevrotick/cooldown-fixtures-20261113-v1/fixture-qualification-v3.json`,
SHA-256 `7e7f7e6feaa0dd55baf335cba237660ac89b6388a4d11b13382f704ff9ef393f`.
It binds the final helper/guard/fixture bytes, exact Cargo, UTC intervals and
publication data, expected failures and cleanup. Graph-producer and fixture
receipts remain separate; no pooled product-completion percentage is used.

**Product/native qualification:** no gratuitous real version update or public-
registry producer invocation was performed. Actual Cargo selection/environment
semantics are exercised in the local sparse peer; producer source-copy/quota/
import/output plumbing has the explicit controlled seams above. A future real
update must execute that guarded producer with its own reviewed candidate and
retained decision before new graph approval. The earlier full UI/CPU/scratch
qualification gaps remain above; native/fuzz/storage/UI and post-V1 proof holds
are unchanged. No current product/native/release pass is inferred from fixtures.

**Next action:** use this selected rule for the next authorized dependency
change. This bounded policy implementation has no remaining required work;
continue the ordinary product/build queue with the existing resource/role/native
owners and holds. No owned acquisition or test process remains running.

### Final qualification decision — 13 November 2026

This retained checkpoint precedes David's prerequisite installation and explicit
resumptions. The [current continuation](#rust-1100-qualification-continuation--13-november-2026)
below supersedes its stop and next-action instructions; its original failures
and candidate-specific observations remain historical evidence.

**Request/scope:** David requests final qualification of this Rust-upgrade and
cooldown work. R4-B-02 / CHECK-19, CHECK-20 and CHECK-26 retain ownership. This
checkpoint closes the cooldown tooling assessment; it does **not** close the
incomplete Rust application/build qualification above.

**Cooldown: qualified within the declared tooling scope.** Independent final
review found no remaining required cooldown gate or must-fix defect. It reviewed
the last publisher tightening on helper `9ce60531…`: exact three-file payload
inventory and bounds, canonical private parents, no-follow directory descriptors
and exclusive publication. The 28 producer/admission controls, 14 actual Cargo
fixture calls and both 28-lane frozen dependency rows retain their separate scopes.
Controlled producer acquisition/quota seams are explicit; a gratuitous real
public-registry version update is not required to complete this policy slice.
The next actual update must execute that producer and the existing graph/source
review before approval.

**Evidence reuse/current identity:** both dependency receipts still bind all
595 current inputs to candidate
`e7cd91df3737cac9f5f820d0228179698603ee76d876dfcb6a89d11bddbc12a1`,
with zero drift. The final sparse-fixture and producer receipts bind the current
helper, environment checker and tests; all twelve executable identities in each
real-Cargo group match their retained hashes. All worktree bytes match the prior
handoff before these two documentation amendments. Unchanged checks are reused;
no fresh execution or broader result is inferred. HEAD, index and lock are
preserved; no stage, commit, publication, installation or package change occurs.

**Rust qualification: incomplete, not promoted.** The remaining actions are:

| Open gate / owner | Actual limit and required next action |
| --- | --- |
| Portable UI / existing host-SDK owner | Ubuntu 24.04 has Fontconfig/Freetype runtimes but lacks development headers, pkg-config records and linker inputs. Follow the existing CI host preparation (`pkg-config`, `libfontconfig-dev`), verify both pkg-config queries, then execute the configured full portable rows on a fresh admitted snapshot. The retained Debian-13 SDK has mismatching runtime/pkgconf hashes and cannot substitute. Installing prerequisites alone supplies no test pass. |
| Windows release/export / R4-B-02 resource owner | Retain the release rustc SIGKILL and matching hard CPU-watchdog record. A reviewed finite release-link resource profile is needed before retry; the present 90 CPU seconds/process and other bounds remain. No lower-job-count remedy or completed executable/PDB export is claimed. Native Windows remains separately held. |
| Engine compatibility / existing source-closeout resource owner | Preserve v2's 846,266,368-byte after-command failure against 805,306,368 bytes. Prepare a fresh v3 recipe/freeze for the current source/helper bindings. After exact 409/228 ordinary, 34 compiler-control and 69/56 doctest identities pass and all commands are reaped, narrowly retire only that new slice's verified disposable GNU `work/target/debug` cache before optimized execution. Record inventory/hash, allocated bytes, protected-input bookends and pre/post storage checks. The historical retirement guard expects 400/222 and cannot be reused unchanged or given fabricated records. Keep all raw results and the failed v2 tree. |
| Analysis/source binding / existing ordinary-role and W2R owners | The old time-domain/Engine source roles and changed KE dossier/roster/helper bindings remain unadmitted. Complete their source admission before the respective required rows or any new coverage/mutation/generation campaign. Historical counts supply no current metric pass; all campaigns remain closed. Formal work remains post-V1. |

The read-only cache measurement found 518,811,648 bytes exclusively owned by the
retained GNU debug cache. Removing an equivalent **fresh-run** cache projects
327,491,584 bytes for that measured tree. This is a sizing estimate, not an
executed retirement or resource pass. The proposed v3 recipe does not yet exist.
Keep the 768-MiB output cap, 3-GiB RAM, 512-MiB swap, 512-task outer cap, inner
limits, six Cargo jobs, 600/660-second budgets and existing disk floors unchanged.

**Current stop/resumption:** the recorded preflight observes **3,195,342,848
bytes available RAM**, below the **4,294,967,296-byte** Engine startup minimum.
No heavy workload, mount, installation or product process was started. No other
process was stopped. Stop heavy work here; do not wait for resources or restart
automatically. On David's resumption after resource recovery, the next single
bounded action is the reviewed v3 cache-retirement recipe preparation above,
followed by its fresh resource preflight and execution. SDK preparation and the
release-link budget remain separate required gates; this checkpoint lifts none
of the native/fuzz/storage/UI/proof holds.

The existing `~/.local/share/nevrotick/cooldown-20261113-v1/` evidence chain now
also contains `qualification-start.json`, `qualification-preflight.json`,
`qualification-tools.json` and the final qualification/custody receipt. They
record the current stop and reuse argument, not a successful heavy rerun.

### Qualification retry — 13 November 2026

**Resumption/authority:** David explicitly requests another attempt. Continue the
same R4-B-02 / CHECK-19, CHECK-20 and CHECK-26 qualification, preserving the prior
failures and all product/native execution holds. Initial worktree bytes still
match the final qualification receipt; the resumed freeze is retained as
`cooldown-20261113-v1/qualification-retry-start.json`. Available RAM initially
improves to about 3.8 GiB, still below the unchanged 4-GiB Engine launch minimum.
Prepare/review the bounded recipes first; repeat that preflight immediately
before any heavy launch. No unattended restart follows a terminal resource stop.

**Reviewed release-build resource decision:** permit **one fixed 300-CPU-second
per-process trial** for the two existing Windows release-link commands only.
The original 90-second failure remains recorded. A CPU limit counts a process's
threads, so a multithreaded LTO process can exhaust it before the independent
300-second wall timeout. The observed failure establishes that 90 was
insufficient; it establishes neither a sufficient nor an optimal replacement.
Three hundred is a predeclared finite compilation ceiling for this trial, not a
measurement or an inference equating CPU with wall time. Do not escalate again
automatically if it fails. Extending the selected profile to the Engine release
link prepares that coherent artifact cohort; only the UI failure was observed.
The [Linux resource-limit contract](https://man7.org/linux/man-pages/man2/getrlimit.2.html)
supplies the per-process/thread, inheritance and hard-limit SIGKILL semantics;
[util-linux prlimit](https://github.com/util-linux/util-linux/blob/master/sys-utils/prlimit.1.adoc)
defines a single numeric limit as both soft and hard.

The coordinator and independent resource reviewer select this narrow build
profile within the existing build-owner action above. Admit only the exact
reviewed Windows row identity/argv/profile; expose no free-form CLI/environment
budget and record the effective profile in each lane receipt. All other commands
keep 90 CPU seconds. Keep the 300-second command wall timeout, 4-GiB address-space
limit, 128-task inner limit, selected build jobs, isolation and complete product
hardening unchanged. This changes no product runtime/CHECK-21 budget, privilege
or security exception. Discriminating selection/refusal and actual lightweight
kernel-limit controls precede any trial; they do not establish a release pass.

**Other bounded preparation:** a fresh external v3 Engine recipe must guard
cache retirement against the current independently retained ordinary/compiler/
doctest rosters and protected source/helper/tool bindings. Its 768-MiB storage
cap, 3-GiB RAM, 512-MiB swap and all other prior limits stay unchanged. The old
failed v2 evidence and caches are preserved. For portable UI, passwordless sudo
is unavailable; David has been asked to supply the existing Ubuntu development
prerequisites. No missing-capability fallback or development-header installation
is treated as a completed portable check. Actual results follow after source
freeze, focused controls and fresh resource preflight.

**Reviewed ordinary-contract resource decision:** the two inert external-contract
rows have no global 4-GiB host-RAM launch rule. Their shared 2-GiB output
requirement is a **disk** floor. Independently review and attempt them sequentially
under a separate owned **1-GiB memory / 512-MiB swap / 512-task** envelope, with
fresh host available RAM at least **2 GiB**, a **360-second** outer deadline and
the unchanged disk floor. Keep each analysis command's existing deadlines, jobs,
8-GiB address-space limit and timeout-derived CPU limit unchanged. This is a
separately bounded compatibility attempt, not a reduction of the larger Engine
recipe's 4-GiB minimum or a claim that 1 GiB is sufficient. Retain actual caps,
peak/OOM/PID events, exits and owned cleanup; do not raise limits automatically
after an OOM or timeout.

Before either row, admit its exact complete current product count **and digest**,
capture the full source/helper map and retain the normal input bookends. The
time-domain contracts-only role must not consume or re-admit the stale fuzz
bindings. The Engine role remains limited to checked configuration/schedule
construction; neither role admits metrics, the closed KE campaign or formal work.

**Host capability and actual ordinary results:** David reports installing the
requested Ubuntu development inputs. Live `pkg-config --modversion fontconfig
freetype2` succeeds (2.15.0 / 26.1.20); dpkg identifies Fontconfig development
2.15.0-1.1ubuntu2 and Freetype development 2.13.2+dfsg-1ubuntu0.2. The original
missing-SDK stop is preserved; this enables a later full portable execution.
The optional Debian SDK is neither changed nor used.

Both real stable-core contract entrypoints now **pass** on the current
595-input candidate
`9e5b1ecfcb5bdb3d2ecd2a01090f94336e16776431b9f92c77c72bef74e054eb`.
Time-domain executes 80 lanes, two legal runtime consumers and 75 intended
compiler refusals; Engine executes 15 lanes, one legal runtime consumer and
12 intended refusals. Their observed maximum memory is 721,420,288 and
647,159,808 bytes respectively, zero swap, and observed task maxima 37 / 27.
The 1-GiB envelopes pass, with source/tool bookends, retired units/cgroups,
no remaining owned PIDs and no cleanup errors. Raw driver/resource evidence is
under `rust1100-20261113-v1/ordinary-{time-domain,engine}-contracts-v1/`.
These are current external-contract/compiler results, not metric or native passes.

The fixed wrapper's independent review first found a parent-loss exclusion gap,
then an initial-readback foreign-InvocationID adoption gap. Retained RED controls
reproduce both. One shared systemd unit plus complete exact mode/nonce argv proof
before ownership adoption closes those paths; all 11 controls pass on frozen
wrapper `c31a4487…`. Actual runs use that reviewed version. Historical variants
and preparation receipts remain separate.

**Later explicit resource steering:** after being told that available RAM was
about 3.96 GiB, David says **“carry on. Windows would adapt”**. For this attempt,
select a **3.5-GiB startup-availability floor** in new Engine v4 and product
envelopes. Keep the **3-GiB hard RAM / 512-MiB swap / 512-task** caps and the
**1-GiB live available-memory stop**. This changes only startup headroom; actual
observation determines whether the workload can finish. It is not a pass of
the earlier 4-GiB admission. Preserve prepared v3 / freeze `4d46b474…` unchanged;
v4 uses fresh versioned output/freeze names. Independent normalized-diff review
and 29 cache-retirement controls confirm all other Engine limits unchanged.

For the four fixed full-product rows (portable/Windows, default/explicit KE),
select the same hard caps and new startup floor, 5-GiB startup / 4-GiB live disk
floors, and a finite **1,800-second** outer deadline with **1,750-second** child
budget. The configured CI job has a 35-minute overall timeout; this separately
declared 30-minute per-row trial leaves its existing 300-second command deadlines
and all compiler/test limits unchanged. Expose no arbitrary argv or budgets.
Use one shared unit per product envelope family and the reviewed ownership
predicate; run only one compiler workload at a time. No automatic resource
escalation follows a failure. Actual full-row results remain pending below.

**Engine v4 actual result: bounded compatibility/resource pass.** Fresh freeze
`1e8d904c…` binds the same 595 product inputs and 25 helpers. All 42 raw lanes
reconcile to the independent exact 409 ordinary / 409 optimized (228 KE each),
34 compiler controls and 69 doctests (56 compile-only). The guarded, quiescent
retirement records 1,591 cache entries / 518,815,744 allocated bytes, then removes
only `work/target/debug`. All 87 storage samples stay below 805,306,368 bytes;
maximum 626,487,296 and final optimized after-sample 327,929,856 bytes. This closes
the earlier aggregate scratch failure without raising its cap.

The actual envelope passes with observed maxima 1,820,184,576 memory bytes,
53,248 swap bytes and 56 tasks, zero OOM/PID events, matching input/private-source
bookends and complete owned cleanup. Independent review reparses the raw roster,
diagnostic and resource receipts and confirms the unit/cgroup retired. The raw
`qualification_passed=false` and `campaigns_reopened=false` intentionally remain:
this is the named compatibility/resource exit, not W2R/metric/native completion.
Evidence remains in `rust1100-20261113-v1/engine-ke-compatibility-v4/`.

**Full portable v1 first failure: retained cgroup OOM.** The 3-GiB full-product
trial ends incomplete after about 202 seconds, at simultaneous UI library/libtest
compilation. Systemd records `Result=oom-kill`, SIGKILL and the kernel names the
exact unit with `CONSTRAINT_MEMCG`. The observed RAM/swap maxima equal the hard
3-GiB / 512-MiB caps. The last sampled zero OOM counters precede unit loss and do
not contradict those terminal kernel/systemd outcomes. No driver result survived;
partial artifact timestamps establish no completed portable row. Owned cleanup
passes, with no remaining PIDs/unit/cgroup. Keep the original source and complete
`product-portable-default-v1/` evidence.

**Separately reviewed next full-product profile:** the host minimum during that
failure is 2,488,844,288 available bytes, above the unchanged 1-GiB live stop.
Current free capacity after cleanup is about 5.6 GiB. The recorded failure is
aggregate compiler allocation, not a product runtime-budget defect. The original
3-GiB product envelope was an unqualified build trial borrowed from the smaller
Engine slice. Select one manually reviewed fresh **4-GiB build-RAM** experiment,
requiring **5 GiB available at startup**, with unchanged 512-MiB swap, task, CPU,
address-space, job, wall, disk and live-memory bounds. Four GiB is not yet shown
sufficient. Stop if this experiment also fails; implement no automatic ladder.
The successful Engine **3-GiB / 768-MiB** profile is unchanged.

New external product v2 output paths preserve the failed v1 tree. Both versions
share the same fixed systemd-unit family, preventing overlap after parent loss.
Independent review and 12 preparation controls check the exact profile delta;
the product driver report has a fixed 16-MiB read bound, independently exercised
with actual prior report sizes and oversized refusal before `read_text`. Stdout
stays bounded at 4 MiB. These are compiler-build controls; shipped runtime
budgets and product hardening do not change. Actual v2 results follow separately.

**Product v2 result and causal repair:** the 4-GiB attempt reaches and passes
the workspace compiler, strict Clippy, host Engine build and rustdoc. It then
returns a normal exit 101 while linking the UI `tray_ui` test: bundled LLVM/LLD
throws `std::system_error: Resource temporarily unavailable`. The observed task
peak is 131, with the inner 128-task limit and surrounding wrapper processes;
the host exposes 14 CPUs. OOM counters remain zero. Memory-limit reclaim is
recorded separately and is not relabeled OOM. The driver partial receipt survives
with 25 lanes, and owned cleanup passes. Preserve
`product-portable-default-v2/`; it is no complete portable pass.

Reduce **only** portable `product-tests:nevrotick-ui` cold compilation from six
Cargo jobs to **two**, in both dependency profiles. Keep all test execution,
other package build concurrency, selections, compiler flags, hardening and
resource/time limits unchanged. The actual linker failure is RED; 130 verifier
and 19 profile controls pass after the narrow repair. Existing historical
command digests remain, with an independent exact UI-two-jobs assertion before
normalizing that one token to its historical six. Independent comparison of
1,167 configured rows confirms no other command delta.

**New final product identity:** 595 inputs,
`e4c4ebd162f392098fa6284579e35f995d4075253840ff0d90df8e6932e3ec8f`.
The only two changed product entries are the verifier and included profile test;
all 351 Rust inputs, manifests, lock, configuration, tool and vendor inputs are
unchanged. Rebind only the three ordinary-contract candidate-pin literals to
this complete map; 29 focused admission controls pass. All historical policy
fields and closed campaigns remain unchanged.

**Explicit unaffected-state evidence reuse:** retain the original `9e5b…`
Engine-v4 and ordinary-contract receipts and their actual source maps unchanged.
They are not executions of the new full product snapshot. Their Rust/tool/vendor/
configuration and selected Cargo/rustc commands remain identical; neither path
invokes the changed portable UI command-builder branch. All other verifier
functions/classes, including sandbox, admission, collectors and result accounting,
are AST-identical. Ordinary role changes are only the independently reviewed
candidate-pin literals. Independent review accepts those observations for their
unchanged component/compiler guarantees. Fresh full-product executions on
`e4c4…` must still qualify the changed UI scheduler and complete integrated rows.

New external product v3 binds `e4c4…` and fresh output paths. Its inverse
normalization reproduces v2 exactly after reversing only candidate/output/doc
labels; the fixed unit family and every 4-GiB/startup-5-GiB limit are unchanged.
Twelve preparation controls pass. No further memory/process/time escalation is
selected; actual new-candidate rows follow below.

**Product v3 disk stop and storage disposition:** the two-job portable attempt
reaches 231 seconds with no OOM, observed task maximum 50 and orderly owned cleanup,
then the unchanged 4-GiB live disk reserve stops it. No complete driver receipt
survives. Keep this separate from both the v1 OOM and v2 linker failure; it is
not a completed portable row.

The two task-created failed target caches occupy 7,912,312,832 unique allocated
bytes. Their exact paths are
`product-portable-default-v1/scratch/nevrotick-product-hgvhcbu0/work/target`
and `product-portable-default-v3/scratch/nevrotick-product-9f6x_0xz/work/target`
under the existing Rust evidence root. Before retirement, a bounded streamed
inventory retains 19,670 path/inode/size/block/hash entries, all protected
non-target source/raw bytes, and 1,633 forensic mappings to 93 unique objects /
11,930,222 retained bytes. These include all six generated Rust identities,
the 11.86-MB generated UI source and the compiler/build-script diagnostics.

Independent review approves only those two terminal failed-cache trees, under
script `5841f95d…` and preparation SHA
`a88ffe9f738915606fdb362fd7f70562d4ea3312ecc358ec770401bab5e905e5`.
Fresh unit/cgroup absence and all 533 recorded PID/start identities being dead
precede fixed-path, descriptor-anchored removal; all inventory/protected/forensic
bytes are revalidated. The actual retirement succeeds, recovering exactly
7,912,312,832 available bytes (3,935,936,512 to 11,848,249,344). Earlier statements
retaining the complete failed trees now exclude only these explicitly retired
generated caches. Raw failure/source/forensic evidence remains; those discarded
cache bytes cannot be replayed from the manifest alone. The failed Engine v2
tree, current sources, tools, vendors and unrelated data remain untouched.

**Configured Windows scratch route:** the existing pointer already selects
`/mnt/c/Users/perso/AppData/Local/NevroTick/ui-m0/work`, with about 226 GiB free.
A fresh owned probe confirms bytes, hardlinks and symlinks; the configured
isolated std-only compiler fixture also passes all eight lanes, two unit tests,
one doctest and the MSVC compile check on that filesystem. This is readiness
evidence, not a product or native-Windows pass. Drvfs reports mode 0777; no Linux
private-mode or Windows ACL qualification is inferred.

New product wrapper v4 puts only fresh, fixed per-mode generated scratch there;
run control, raw reports and exported artifacts stay on Linux. It binds the
pointer bytes and exact selected route, refuses existing/symlinked scratch, and
enforces the same 5-GiB startup / 4-GiB live disk floors on **both filesystems**.
All source `e4c4…`, RAM/swap/task/CPU/time/job/hardening, ownership and isolation
guards remain unchanged. Fifteen preparation controls and independent review
pass. Ordinary analysis remains on Linux. Actual full-product results follow.

**WSL memory reconfiguration and fresh continuation:** David increases WSL's
configured RAM from 12 to 17 GiB and requests continuation. Live Linux readback
now reports 16,576 MiB total / 9,248 MiB available. The prior C-scratch portable
v4 run has no terminal driver/workload/resource receipt; its last sample is at
156.271 seconds. Preserve it as interrupted, not passed or as a new resource
failure. The fixed unit is now absent and all 595 current source bytes still
match `e4c4…`.

External v5 changes only version labels and fresh output/C-scratch leaves; every
per-build limit remains unchanged. Fifteen preparation controls pass. The
post-resumption custody snapshot also records externally changed staging; this
task preserves that index state and qualifies the actual working-tree candidate.

**Portable v5 reached result:** after the WSL increase, the fresh C-scratch row
passes 152 Python controls, patched `spin_on` three tests / one doctest, UI 192,
wire 119, time-domain 66, exchange 63 and Core 557 tests. Format, workspace
compiler, strict Clippy, host Engine build and rustdoc also pass. Engine test
compilation then fails before execution: its remaining default 12-job row
produces rustc worker-spawn `errno 11` and LLD `Resource temporarily unavailable`.
The row is incomplete (29 successful lanes / one failed), with no exported
artifacts. Preserve all three terminal receipts under
`product-portable-default-v5/`.

The actual resource envelope reports zero OOM/kills and cgroup PID-limit events,
sampled task maximum 83 / 512 and complete owned cleanup. This sample does **not**
establish saturation of the inner 128-task limit. It supports the narrower
observed conclusion: concurrent compiler/linker workers could not be created.

**Remaining default Engine scheduler repair:** add only `nevrotick-engine` to
the existing six-job bounded-package set. Default ordinary and optimized test
builds change from 12 to six jobs; explicit KE rows already use six and remain
unchanged. UI stays at two. All test workers, selections, flags, bounds and other
commands remain unchanged. Four causal default-row RED controls retain their
KE healthy neighbors; 131 verifier and 19 profile controls pass after repair.
The earlier six-job KE compatibility run is a healthy resource neighbor, not a
pass of the newly changed default nextest rows.

The new 595-input source identity is
`369bec84fada1a1e1c18460d1a9ec6a44380cf5564bd825d6ac61c4c9e211fa3`.
Again only the verifier and included profile test change; all 351 Rust inputs,
lock, manifests, configuration and dependency graph remain unchanged. Rebind
the ordinary source-identity pins to this actual map. Earlier `9e5b…` / `e4c4…`
raw observations retain their exact original scopes; the changed default
Engine ordinary/optimized rows require fresh execution in the new full matrix.

### Rust 1.100 qualification continuation — 13 November 2026

**Retained bounded-worker UI-trial candidate:** all 595 product inputs bind
`e63103cd62483664b9a000adf6d466847e3b64342aac38f03599127a6ac9b301`.
The retained v6 executions below bind their original `369bec84…` source map;
the reviewed UI-only resource delta and unaffected-state bridge follow below.
The later [owner-requested worker restoration](#owner-requested-worker-restoration-and-publication--13-november-2026)
owns the current publication candidate and its execution limitation.
The 351 Rust inputs and lockfile remain unchanged. This continuation owns
R4-B-02 / CHECK-19, CHECK-20 and CHECK-26 within the Rust-upgrade and cooldown
tooling scope. Expected results are complete configured host/compiler/test
rows, Windows cross-build/export rows, exact source/tool/artifact custody and
successful bounded-resource cleanup. Native Windows and release qualification
retain their separate owners and holds.

Both retained `dependencies-candidate369-{default,engine-ke-association}.json`
receipts pass all 28 ordered lanes each. Independent review reconciles every
receipt-bound source byte, GNU/Windows metadata and selected trees, exact command and
resource profiles, and successful advisory/bans/licences/sources outcomes.
Fresh dependency rows also execute on the recorded candidate `e63103cd…` and pass all
28 lanes each. Their corresponding `dependencies-candidatee631-*.json` SHA-256
identities are `614ff61d8ca0af070aa136a4ea69620fa8b40e9ddfb4760d5801d0b76cc2c622`
and `88c87e0e79ae486557ce60f1e7e9d5570214cfcb1c30f1ffdfdc0b7420e647bf`.

The fixed v6 product recipe is
`~/.local/share/nevrotick/rust1100-20261113-v1/run-product-qualification-resource-v6.py`,
SHA-256 `3e23838f261ed3e634e7d7e071ef444156be1f05c9af2c7a7dd69d134e5227f2`.
It retains the reviewed 4-GiB RAM, 512-MiB swap, 512-task and 1,800-second
envelope, startup/live memory and both-filesystem disk reserves, fixed invocation
ownership, per-command limits, isolation and product hardening. The modes run
sequentially with fresh scratch. David's larger WSL allocation supplies host
headroom; it does not increase the per-job cap.

| Retained v6 row | Actual terminal result |
| --- | --- |
| Portable default | **Passed**, all 42 lanes, 152 Python controls, 1,226 ordinary tests, 868 optimized tests and 327 doctests; no required skips. The repaired default Engine rows execute 181 ordinary and 181 optimized cases. The GNU dev observe export is sealed as `a06e5a1dc0885c68d390fd60ee6e96358c43e7daf99d888ed801f823f1643c2a`. |
| Portable explicit KE | **Passed**, all 42 lanes, 152 Python controls, 1,454 ordinary tests, 1,096 optimized tests and 327 doctests; no required skips. Engine executes all 409 ordinary and 409 optimized cases, including 228 KE cases each. The GNU dev observe export is sealed as `1e7c745877b88beb9934ffc047192e83145bea8e4c85fd9704f89b9a4c1229a8`. |
| Windows default | **Incomplete**, 19 successful lanes including MSVC compiler/strict Clippy, then the UI release binary's LLVM allocation failure. No Windows artifacts exported. |
| Windows explicit KE | Unattempted after the default-row failure; no native execution selected. |

Portable-default completes in 975.77 seconds. Observed peak RAM is 4,294,967,296
bytes, swap 120,479,744 bytes and 83 tasks; minimum host available RAM is
5,365,870,592 bytes. Reclaim at the cap is recorded separately from the zero
OOM/kills/PID-limit events. Source bookends match its `369bec84…` candidate, the
unit/cgroup and all owned PIDs retire, scratch is empty and the shared lock is
available before the next row. The stable receipts remain under
`product-portable-default-v6/{resource-verification.json,verification.json,driver-verification.json}`
in the existing evidence root. Their SHA-256 identities are respectively
`c2e2d0fd46edf2f281d8383ddce40ccd560697130df6c79b62d603ade76daf74`,
`fc47bd8ad08eb925e4f9ea01381b796e775f21d752b869068dcb2fa7fd91e0c1` and
`d4dacd21daa7ac8443018e01874d2ffd32f239ec061d8791e070c987b58dceaa`.

Portable KE completes in 1,060.95 seconds, with peak RAM 4,294,967,296 bytes,
swap 189,734,912 bytes and 83 tasks; minimum host available RAM is 5,210,062,848
bytes. It also has zero OOM/kills/PID-limit events, matching `369bec84…` source
bookends and complete owned cleanup. Its same three receipts under
`product-portable-ke-v6/` have respective SHA-256 identities
`c7a27623be83cefcf0042bdd3fcd7d3bb2007f1c7b2faac5200b797bdc858091`,
`d0e3ccb815db9146571785743f3e92f0987ba662eab5b92e6e70a75ae16e13df` and
`c0876eae9c9637bb1ca18923758800229fd3dc2547c43682067a646a50c5f5fd`.
Independent review of portable default additionally reparses the exact raw
Nextest pass indices/names, doctest outcomes, all ordered command/resource rows,
exported bytes and resource hash chain; it confirms the scoped pass with zero
source drift and all 1,268 recorded PID/start identities dead. Portable KE's
independent terminal review also confirms the scoped pass: exact 42 command
rows, raw test names/counts with zero skips, byte-identical metadata/tree
outputs, exported ELF identity, 1,061 resource samples and all 1,275 recorded
PID/start identities dead. It finds no material inconsistency in either host row.

**Windows v6 first failure:** the final UI release binary reports
`rustc-LLVM ERROR: out of memory` / `Allocation failed`, followed by SIGABRT and
Cargo exit 101. This is a compiler allocation failure, not a recorded CPU/wall
timeout. The resource envelope peaks at 3,323,604,992 RAM bytes, 125,206,528 swap
bytes and 73 tasks, with zero cgroup OOM/PID-limit events. Minimum host available
RAM is 3,707,154,432 bytes. The row completes its failure/cleanup path in 733.29
seconds; Engine release/export is unreached. Source bookends still match all
595 inputs of its `369bec84…` candidate, with empty scratch, retired
unit/cgroup/PIDs and free lock.

A direct `/proc` observation retained in the execution transcript samples the
final UI rustc at virtual size 4,294,950,912 bytes and RSS 2,354,073,600 bytes
(CPU 128.14 seconds), beside the configured 4-GiB per-process address-space cap.
There is no standalone per-process sample or `/proc/limits` receipt. Together
with the admitted exact `prlimit` wrapper and absence of outer OOM, this strongly
supports address-space exhaustion; the raw LLVM allocation diagnostic is the
direct observed failure. More WSL RAM does not change that process cap.

Preserve `product-windows-default-v6/` and its resource/workload/driver receipt
SHA-256 identities, respectively
`e58146569786afdb4fdeb6d82fca988e64fafc3c98f07d1e79e4b0981504d799`,
`ed5068f14da1f9c8890be6739d46f70a3d6de74a9482ffd6502f20fd5eaa55eb` and
`3a4ebd374cb3c4bfff6f4663aa16bde1bb917b38446618256c35c1af637d69b1`.
The next bounded action is independent causal review and the smallest exact
UI-release resource-profile repair. No failed row is relabelled, no dependent
Windows retry starts before review, and the host passes retain their actual scope.

**Reviewed UI-only correction:** admit exactly the configured Windows UI release
Cargo invocation and its children to AS 8,589,934,592 bytes, CPU 300 seconds and
wall 600 seconds. Keep Engine AS 4 GiB / CPU 300 / wall 300, all other selected
commands AS 4 GiB / CPU 90 / wall 300, and every outer RAM/swap/task/reserve/
isolation/hardening bound unchanged. The failed cold release command had already
used approximately 272–280 of 300 wall seconds before unfinished allocation
work failed. The 600-second selection is finite headroom for this new trial,
not a claim that a timeout occurred or that 600 seconds is proven necessary.
Stop for review on a new failure; implement no automatic limit ladder.

The sole changed product input is `scripts/verify.py`, SHA-256
`4b3aaf37abc52b90119dca7f0cd6a2f6bde95d8befd01827225a6b0556f4b696`.
Only `command_resources` and `sandbox` ASTs change. Focused AS/wall expectations
are RED before repair; 132 verifier and 19 profile controls pass afterward,
including three actual Python-only kernel CPU/AS limit readbacks. Independent
comparison of 51 cases / 1,167 command rows finds identical Cargo argv throughout;
only 17 exact UI Windows resource occurrences and their AS sandbox token change.
All non-UI resource dictionaries and complete sandbox frames are identical.
This establishes the explicit unaffected-state bridge to both successful v6
host rows and unchanged dependency/Engine/ordinary-contract paths. Their raw
receipts stay tied to their actual original maps. New Windows rows must execute
the new profile; no host compiler repeat is needed for this unused branch.

Three ordinary-role candidate literals are independently rebound to `e63103cd…`;
byte-inverse review verifies no other policy changes. All 29 focused controls
pass, actual source-only Engine/domain admission accepts 595 inputs, and legacy
full analysis/fuzz roles remain closed. External v7 runner SHA-256
`98443b6486f82168842e797d24359ad7686b1219407d3f98046255796909beae`
and preparation receipt
`afa6d24484e1801847bda315146ea5acab7c1a1c20f4b2ed0fb90ed4162e3176`
pass independent prelaunch review. Exact inverse normalization preserves the v6
runtime/ownership/cleanup logic, changing only fixed candidate/versioned output
and scratch identities plus the profile description. Fifteen preparation controls
pass, including forged UI AS/wall/CPU rejection and Engine/bootstrap neighbors.
The first preparation run's stale v6 test-path expectation is retained as a
fixture-only failure; changing those two literals supplies no product pass.

| Fresh v7 Windows row | Actual terminal result |
| --- | --- |
| Windows default | **Passed**, all 21 ordered lanes and 152 Python controls, MSVC all-target compiler/strict Clippy and both release links. Two executables and their two PDBs are exported and sealed; no native execution. |
| Windows explicit KE | **Passed**, all 21 ordered lanes and 152 Python controls, MSVC all-target compiler/strict Clippy and both release links. Two executables and two PDBs are exported and sealed; no native execution. |

Windows-default v7 completes in 820.04 seconds with peak RAM 3,755,732,992 bytes,
swap 74,887,168 bytes and 87 tasks; minimum host available RAM is 3,837,775,872
bytes. It has zero OOM/PID-limit events and matching current-source bookends.
All 825 owned PID/start identities, unit and cgroup retire, scratch is empty,
and the lock is free before the KE row. Supplementary UI library/binary process
snapshots retain their actual identity and CPU/AS kernel limits. No sample above
4-GiB virtual size was captured between polls; none is claimed.

| Windows default release export | SHA-256 |
| --- | --- |
| `nevrotick-ui.exe` | `16cc27cb5b07c5ea65f43a4c52249a6418e1057ffcc8e0adbce616cf66635f0c` |
| `nevrotick_ui.pdb` | `008dd58da78b8d4c8829d0c5b326ecd93c6f1868698a06f2b2cb78c2b7d8719b` |
| `nevrotick-observe.exe` | `b23567b5ff5c50434ef96cbbb80c436f16bd950db408bea3876fd39f58128042` |
| `nevrotick_observe.pdb` | `d7cb581584cb8310a81ea46d2852e1cac2313d053d1931e5972437b35dd253b5` |

The stable `product-windows-default-v7/` resource/workload/driver receipt SHA-256
identities are respectively
`749bd093d57ecf3fb53798ca6cd963d8247e6f8646a11d1f0a8d2a71a3f170e1`,
`74d9b2a41f7d73fa503b73c28519744db685b32fffe053e6629b8ae1ab2594d6` and
`92cbe8bc32d35bbc6638f94cc3818769260e715bbcf656f4c9e9cff8abda98c0`.

Windows KE v7 completes in 841.30 seconds with peak RAM 3,683,254,272 bytes,
zero swap and 84 tasks; minimum host available RAM is 3,839,242,240 bytes.
Its current-source bookends, every exported byte, fixed resource dictionaries
and cleanup reconcile. All 921 owned PID/start identities retire; unit/cgroup
are absent, scratch is empty, the lock is available and OOM/PID-limit counters
remain zero. There is no remaining task-owned compiler job.

| Windows KE release export | SHA-256 |
| --- | --- |
| `nevrotick-ui.exe` | `5791add876ae15035e14e086d60ee10d7904ef7e52767837d1f057e36fa9ae26` |
| `nevrotick_ui.pdb` | `1cd7250b48d7685c78c7987d19644bfffd1404ab82e92a297fdfcf96465c2301` |
| `nevrotick-observe.exe` | `b9ddcdac232bfc7bf8a103595bd0332d0edf6826f5145ce4f4fc28a9d70a7033` |
| `nevrotick_observe.pdb` | `5702140bfae26ada52adbca00fd5a7c6ebdd05fbdde808d15eff7bf3ddc77a88` |

Its `product-windows-ke-v7/` resource/workload/driver receipt SHA-256 identities
are respectively
`f2f379d45cbb46ed7e0880c07bacf32f4817d250372819e1a7fc92fe4e7ed899`,
`6980ac38a9b27c8b1e1614e544b95944e542a69decdf3ba7ce5243fddcc0bf15` and
`ff29b908900606f8f9b03664f808c44d253a56582ff81927efd51e29b2938627`.

The supplementary `ui-release-bin-over4gib-process.json`, SHA-256
`79951ae9cdfa578875f079954b9d9264718e93b80f91f8750887ae3c6006c4a8`,
directly samples final UI rustc at 6,078,521,344 virtual bytes, 226,852,864 RSS
bytes and 157.15 CPU seconds, under actual soft/hard AS 8,589,934,592 and CPU
300 limits. PID/start, cgroup, timestamp and InvocationID bind it to the owned
run. This actual virtual use exceeds the old AS cap while remaining within the
unchanged RAM envelope. The Engine process snapshot, SHA-256
`b4fbc7ab6d1630de08d8f7e35908f5480f6b7900be5165c9ca8dda10f5e11d25`,
separately confirms its unchanged AS 4-GiB / CPU 300 limits. Neither sample
claims peak per-process RSS or a Windows-native runtime observation.

Independent review of Windows default confirms all 21 ordered commands and
152 controls, paired metadata/tree bytes, exact profiles and complete hash chain.
Both executables are PE32+ AMD64 (UI/console respectively), both PDBs are MSF7,
all four exports rehash exactly and every recorded owned PID/start is dead.
PE header readback remains static header evidence, not effective native-process
mitigation qualification. Independent KE review likewise confirms the full
ordered command/graph/export/resource chain, zero source drift and all 921 owned
PID/start identities dead. The fixed unit is absent at final handoff.

The independently accepted unaffected-state bridge for the original Engine-v4
compatibility and ordinary-contract executions remains exact: unchanged Rust,
tool/vendor/configuration and selected compiler commands; only unrelated
scheduler branches and ordinary-role identity literals differ. Their raw source
maps and results are not relabelled as executions of either newer full snapshot.
No additional standalone, metric, mutation, formal, fuzz or native campaign is
required by those reviewed scheduler/identity changes. Coverage/mutation/native
and product-release obligations remain open under their existing owners.

**Retained bounded-worker local checkpoint:** **qualified within its declared
Rust-upgrade and 14-day cooldown tooling scope**. Both dependency rows pass on `e63103cd…`, both
full portable rows pass on `369bec84…` with the independently verified unchanged-
frame bridge, and both fresh Windows release/export rows pass on `e63103cd…`.
Independent review confirms the KE row's 21 commands, 152 controls, paired graph
outputs, all four export bytes/formats, resource limits and complete cleanup.
The exact Engine-v4 and ordinary-contract component bridges remain bounded as
recorded above. No required local gate remained open for that bounded-worker
checkpoint; all earlier failures and interrupted attempts retain their outcomes.
The later worker restoration below changes command concurrency and is not covered
by this full-execution conclusion.

Checkpoint source admission verifies 595 inputs / 351 Rust files and candidate
`e63103cd62483664b9a000adf6d466847e3b64342aac38f03599127a6ac9b301`.
HEAD remains `a25696b8bf67a53ebbcf40c4c0ba43c613159ff1`, the preserved index SHA-256
is `97e22fb280501e8c8dc67055cc60baf186550d3b01b6ac546e1c030047f66907`, and
the unchanged lock SHA-256 is
`2ccabde33a800b745c42b4fe96dc4a6c300b1961ac87e149d313cae41fde5f2e`.
The final full dirty-tree/source/receipt custody snapshot is
`~/.local/share/nevrotick/rust1100-20261113-v1/final-local-qualification-e631-v1.json`.
Skill metadata/links/inventories pass for all 66 skills and `git diff --check`
passes. No staging, commit or publication was performed at that checkpoint.

**Remaining product work:** native Windows execution and effective mitigations,
fresh per-owner coverage/mutation/generation admission, production/Core-positive
receiving, package and release qualification retain their existing owners and
holds. Historical 607/E9 metrics do not transfer. Formal work remains post-V1.
Continue the existing receiver qualification/consumer work from development §7;
this local toolchain exit does not reopen a held campaign or grant release approval.

### Owner-requested worker restoration and publication — 13 November 2026

David explicitly requests restoring the previous worker limits, then committing
and pushing the upgrade. The restored 595-input product candidate is
`8a70ac66f3b5f9275750f357cbf02ec79f23c0a5dd4a2732a65f679496beb5d1`.
Only `scripts/verify.py` and its included KE-profile test change from `e63103cd…`;
the allocation helper's worker restoration is outside this product source map.
The Rust 1.100.0 selection, accepted 14-day cooldown, lockfile, v3 hardening and
all 351 product Rust files remain unchanged by this restoration.

The shared verifier's Cargo argv now match the pre-upgrade HEAD in all 51
reviewed cases / 1,167 constructed commands. Of these, 115 rows differ from the
qualified bounded-worker commands only in worker arguments; their resource
profiles and sandbox prefixes remain unchanged:

| Owner | Restored previous worker selection |
| --- | --- |
| Default workspace compiler/Clippy/rustdoc, observe build and Windows compiler/Clippy/release links | 12 build jobs. The original explicit KE workspace/Engine override remains six. |
| Portable ordinary/optimized tests | UI ordinary builds return from two to six; default Engine ordinary/optimized builds return from six to 12. Existing wire/domain/Core bounded builds stay at six; exchange/platform and other original 12-job selections remain. Explicit KE Engine builds stay at six. |
| Patched `spin_on` controls | Existing six build jobs, unchanged. |
| Windows time-domain allocation observer build | 12 build jobs, restored. |

The existing Cargo default of 12, Nextest/libtest's 12 test workers, ten outer
mutation workers and one inner build job/test thread per mutation worker are
unchanged. Worker restoration changes no isolation, hardening, resource ceiling
or test selection. The exact Windows UI release profile retains AS 8 GiB / CPU
300 seconds / wall 600 seconds; Engine retains AS 4 GiB / CPU 300 / wall 300,
and other commands retain AS 4 GiB / CPU 90 / wall 300. The existing outer
RAM/swap/task caps and memory/disk reserves remain.

**Publication evidence limit:** the full restored-concurrency Rust matrix is
**not rerun**. Current verification passes 132 verifier, 19 KE-profile and 19
allocation controls. These Python/policy and command-construction checks, plus
publication source admission, do not execute all restored Cargo rows.
The successful v6 portable results on `369bec84…`, v7 Windows results on
`e63103cd…`, earlier component bridges and every first failure retain their exact
source maps, worker selections and raw receipts above. They are historical
bounded-worker evidence, not full execution of the restored candidate. Known
resource failures are not rerun during this publication closeout.

Commit and push follow David's explicit request with that limitation recorded.
The complete staged-tree checks retain one first fixture failure: the Slint
preflight test's sandbox double rejected the new `lane`/`label`/`profile`
keywords before its intended graph rejection. Updating only that double's
keyword acceptance restores all 33 profile controls without changing the
production path or its preflight/no-compiler expectations. Publication checks
run from a byte/mode-verified complete staged worktree, with the original index
and patches preserved externally before authorized staging.
All 19 affected Python modules then pass, totaling 1,165 tests, alongside 48
repository-security controls, the 66-skill pack, environment/strict workspace
policy, pinned workflow analyzers and redacted history/working-tree secret scans.
The initial whole-Core Python invocation exceeded the closeout runner's
120-second unit-test budget without an assertion failure; a fresh full run
passes all 358 tests in 512.648 seconds under one declared 600-second unit-test
budget. Both logs remain retained. This changes no product or compiler limit.
Future dependent execution must admit the restored source and actual worker
commands under the existing R4-B-02 / CHECK-19, CHECK-20 and CHECK-26 owners.
Native Windows, metrics/generation, package/release gates and all existing
native/fuzz/storage/UI holds and post-V1 formal deferral remain unchanged.

### Hosted startup and explicit EP1 routing — 14 November 2026

H08/H09 belong to the owner's [additional contract handoff](engine-transport-pilot.md#additional-contract-and-integration-handoff--14-november-2026),
under CHECK-19/25/26/27. Current main/guidance is `02e7e907`; the dirty/staged
product is separate. Live push runs 43509873376 (Rust) and 43509875354 (Security)
retain startup_failure, attempt 1 and zero jobs/check-runs. Their startup annotations
explicitly reject upstream checkout and CodeQL action references under the actual
repository policy enabled=true/allowed_actions=local_only/sha_pinning_required=true.
No billing or Rust-error inference is needed. CodeQL's false execution conditions
do not avoid startup validation. Existing rulesets 26069359/26110199 retain the
Security gate/UI-M0 portable model checks under integration 15368. Read API admin
status is not proven Administration-write capability.

[NV-DEC-001 §6.4](../docs/nevrotick-owner-decisions.md#64-hosted-actions-trust-policy--14-november-2026)
owns the concrete pending two-repository/immutable-SHA allowlist and rollback.
Only that owner trust decision/settings transition and actual hosted execution
are held; no settings write/rerun is performed without it. Keep read-only tokens,
full pinning, rules and CodeQL deferral. Old-main runs cannot qualify new dirty
source. Raw current API/annotation observations are retained under
`/home/david_osipov/.local/share/nevrotick/contract-integration-hardening-20261114-v1/hosted-audit/`.

**Recheck on `b161229` — 26 November 2026.** Both required workflows
again stopped at startup with zero jobs. The actual Rust annotation lists
the pinned upstream checkout action. The Security annotation lists checkout
and both pinned CodeQL actions. The repository API still reports
`local_only` and requires full-length commit SHAs. The existing §6.4 owner
decision remains the prerequisite to restoring these hosted runs.
[Rust startup annotation](https://github.com/DavidOsipov/NevroTick/actions/runs/45785019484)
[Security startup annotation](https://github.com/DavidOsipov/NevroTick/actions/runs/45785021276)

**Policy transition and hosted readback — 27 November 2026.** David approved
the exact checkout and CodeQL commits in §6.4. GitHub rejected the proposed
disabled-and-selected REST request, and the original policy read back intact.
After David saved the selected-policy form, the coordinator corrected the
readback to disable both broad allowances and select the exact checkout,
CodeQL `init` and CodeQL `analyze` action paths at the approved commits.
Full-SHA pinning and both required checks remained active. GitHub refused to
retry the startup-failed runs, so two exact-head workflow dispatches were
started. The first Security dispatch still refused the repository-level
CodeQL pattern; the path-qualified readback admitted the next run, whose
Security gate passed with the four planned CodeQL jobs skipped.

The Rust dispatch reached its actual runner. Its environment test selection
failed in two cases because the isolated environment contained no passwd
entry and the fixtures cleared their `HOME`. This is a test-fixture failure
before the intended assertions. The existing targeted fixture repair keeps
the controlled home and cleared-environment cases independent. The new
candidate still needs its own hosted run.
[Security run](https://github.com/DavidOsipov/NevroTick/actions/runs/45896284360)
[Rust run](https://github.com/DavidOsipov/NevroTick/actions/runs/45896174041)

The signed test-fixture repair `c123ee52af79174d55083b994b3b774b93091327`
passed the 39 isolated environment tests, the skill checker and the pinned
secret scan before publication. Its hosted Security gate passed. The Rust
job passed the former failure, then reached two cooldown-publication test
failures under the actual product-admission step. One bounded diagnosis and
repair interval runs from 2026-11-26 21:24 UTC through 21:44 UTC, with at
most two causal candidate repairs and their focused controls. Retain each
first failure and do not infer a complete product run from earlier steps.
[Current Security](https://github.com/DavidOsipov/NevroTick/actions/runs/45897406767)
[Current Rust](https://github.com/DavidOsipov/NevroTick/actions/runs/45897407288)

The two failures came from test fixtures retaining an accepted-baseline
assumption after the selected policy moved to a resolved graph. The fixture
also omitted 13 manifests required by the resolution, so its intended
post-download control stopped before transport. The corrected tests admit the
actual current resolution, check the complete manifest set, and require the
specific pre-download lock and post-download configuration refusals. The
isolated 30-case acquisition cohort and 29-case age/graph cohort passed.
The exact historical baseline still passes its age control. Replaying that
historical policy at today's actual acquisition entry refused the current
code-owned schema pin before transport. Signed test-only commit `f760b9c`
publishes the correction. Its hosted gates require fresh results.
An additional coordinator isolation attempt initially lacked the expected
passwd entry and then lacked the fixture toolchain and sparse-file allowance.
The final isolated graph control supplied a task-owned passwd entry, the
admitted read-only toolchain and a 256-MiB file limit. Its 29 cases passed.
The earlier failed outputs remain under
`~/.local/share/nevrotick/hosted-cooldown-publication.tAB5SV6H/`.

The second and final repair in that interval is signed commit
`ec78e4408791d72b963b90f8f2113df4ebb89f95`. The real graph-admission
suite now runs after pinned compiler acquisition and selects the private
`RUSTUP_HOME`; its digest checks remain enforced. The 29 focused controls,
eight workflow controls, pinned workflow analyzers and secret scan passed
locally, with an independent source review and full staged-tree custody.
The [hosted Security run](https://github.com/DavidOsipov/NevroTick/actions/runs/45899384282)
passed, and the formerly failing graph-admission step passed in the
[hosted Rust run](https://github.com/DavidOsipov/NevroTick/actions/runs/45899384381).
That run then failed before its portable product commands because the workflow
exports `NEXTEST_BINARY`, which the verifier rejects as an unreviewed build
override even though the workflow also passes its value as `--nextest`.
The retained failed-step log SHA-256 is
`447a26adf46438a9a326f007f69e160126e67f0862e2891158fe0e138e485b1e`.
Both permitted repairs are consumed; the hosted Rust gate remains incomplete.
The separate [CodeQL workflow](https://github.com/DavidOsipov/NevroTick/actions/runs/45899384195)
passed all five configured language jobs on `ec78e44`; it is not a substitute
for the failed Rust product gate.
The signed pushes used the repository administrator bypass while required
checks were pending; subsequent hosted results are recorded separately.
On later `6e8aaac`, the [Security gate](https://github.com/DavidOsipov/NevroTick/actions/runs/45902612864)
passed and the [Rust gate](https://github.com/DavidOsipov/NevroTick/actions/runs/45902612907)
failed at the same `NEXTEST_BINARY` environment refusal before portable
product execution. Its failed-step log SHA-256 is
`879f215828f5d61447505e176b77ac50ad1f3a7cd081200af0408c3cd0fa431d`.
The intervening `74eddba` Rust run was cancelled by the newer run and supplies
no product result. The separate [CodeQL run](https://github.com/DavidOsipov/NevroTick/actions/runs/45902612989)
passed on `6e8aaac`; it does not replace the Rust gate.

**Distinct hosted test-oracle failure — 27 November 2026.** The
[Rust run for `45e2fca`](https://github.com/DavidOsipov/NevroTick/actions/runs/45954309654)
failed earlier in the isolated environment checks: 39 cases ran and
`test_descendant_cannot_outlive_a_successful_leader` raised
`FileNotFoundError` when `/proc/7/stat` disappeared between `exists()` and
`read_text()`. The real product row was not reached. This is an observed
test-oracle race, not a demonstrated failure of the process runner. The prior
`NEXTEST_BINARY` issue and its exhausted repair interval remain unchanged.

A separate, failure-specific source/control stage runs `06:52:05Z`–`07:12:05Z`
on 27 November 2026. It allows one causal test-only correction, independent
review and at most one focused failing-control and one corrected-control
invocation after isolation and resource admission. Its exit is the actual
disappearance race accepted as retirement while live, malformed and denied
observations remain failures, with the existing healthy process neighbor.
Preserve real cleanup ownership and do not signal an unrelated reused PID.
No product Cargo or Windows operation is authorized by this stage. A setup
failure, missing selection or expired allowance remains incomplete; this
distinct cause does not replenish the old hosted repair allowance or establish
that the full Rust gate passes.
The private draft at
`~/.local/share/nevrotick/hosted-proc-retirement-fix-20261127/retirement.patch`
has SHA-256 `3fc1c228b716cdda0bcc46f177a8586f60632f30241294263a9d48984b8d330c`.
Independent review found that its disappearance control reaches the extracted
helper but not the actual failing test loop; the inherited numeric-PID
fallback's ownership across PID reuse also remains unproved. The stage was
interrupted for the owner's time-sensitive GPS observation and stopped
**incomplete**: no corrected candidate was admitted, no test ran and the
shared source was not changed. These gaps remain open; the earlier hosted
failure and the separately exhausted `NEXTEST_BINARY` issue are preserved.

**Audit follow-up — 27 November 2026.** The owner brought a fresh review of
`45e2fca`. The follow-up took one test-only correction and a bounded WSL run
through the actual failing test. The observer reads the process status once
per poll. It accepts disappearance or observed termination and still fails
at the original deadline for a live child. The runner retains process-group
cleanup. The fixture has a finite lifetime and the enclosing PID namespace
provides teardown, so the observer sends no signal to a potentially reused PID.
Injected disappearance reproduced the original error and passed on the updated
test. Live, denied and malformed observations retained their failures. All 39
tests in the hosted environment-policy test module passed locally in the
isolated Linux boundary. The source digests, command and logs are under
`~/.local/share/nevrotick/audit-45e2fca-python-20261127.2xd6Tc6T/`.
`environment.log` has SHA-256
`c167c9726e898ac5952ac0eb40023a14032ea3ec59a37ccf3090359da83aa106`.
The earlier interrupted stage remains recorded. Hosted downstream product
results remain unverified on the correction.

### Bound the GNU UI test build separately from test execution

The [About cancellation checkpoint](slint-gui-handoff.md#about-cancellation-candidate--27-november-2026)
stopped in its UI test build on a rustc/LLVM allocation failure. Its one
attempt ran 8 minutes 52 seconds and ended before its original deadline; the
failed process did not establish which allocation boundary caused the error.
The current verifier builds and runs that UI row together under the ordinary
4-GiB per-process address-space limit. Raising the combined command's limit
would also raise the test-process limit.

The independently reviewed next tooling stage proposes one exact default-GNU
UI archive build at a finite 8-GiB per-process address-space limit, followed
by separately bounded discovery and replay at the existing 4-GiB limit. The
three steps share their original wall deadline and retain current CPU, source,
tool, environment, test-status and custody controls. Exact selector and full
wrapper checks must reject changed commands, lanes, profiles, flags or paths.
Reconcile complete discovered and executed qualified case identities, fresh
JUnit and mandatory current UI cases; test actual effective process limits and
healthy/refusal neighbors. The six-job build exception also reaches build
subprocesses, so aggregate headroom is a separate prerequisite.

Stage A starts at `2026-11-26T22:10:00Z` and ends at `23:10:00Z`, including
implementation, focused controls, review and cleanup. It permits one coherent
candidate and at most one causal repair within that same interval, with the
fixed test invocations and limits in the retained design. No product Cargo or
UI test run is admitted by Stage A. The reviewed design is
`~/.local/share/nevrotick/ui-test-build-resource-2711/design-v3.md`, SHA-256
`fe111ccf16180b0538797bfe83df3e1c60a669987868d8e89c6f8c94e37c37a2`.

The independent reviewer traced the existing combined UI row, the Engine
archive route and current result consumer, then corrected the prior elapsed
time and the stage's test/repair bounds. A second recheck corrected the new
synthetic-control timeout without changing existing supervisor-test limits.
The final review marked Stage A ready only. Archive compatibility, safe
extraction, actual JUnit placement, UI roster, aggregate resources, remaining
budget and a feasible product-run deadline remain prerequisites for any later
Stage B. Preserve the earlier failed receipts and do not relabel them as a
successful or unspent attempt.

Stage A stopped incomplete at 22:13 UTC before candidate edits or test
invocations. The selected Nextest archive listing extracts and invokes
discovery while its work tree is writable; neither safe prevalidation of the
complete compressed archive nor immutable replay inputs and actual remapped
JUnit placement were established with the currently admitted primitives.
The existing Engine archive mocks do not establish these UI properties. No
decoder was admitted and no product Cargo or native work ran. The sampled
resource-monitor log SHA-256 is
`e73f98ce7df65f10302e68e6f2afee8d5953382a34e9bbbf3357edf898f89813`
under `~/.local/share/nevrotick/ui-stage-a-2711/`; its clean detached
worktree was retired. Re-entry requires a reviewed, bounded approach to those
specific custody and reporting gaps, not merely a new label or timeout.

The independently reviewed re-entry approach avoids archive extraction:
the pinned Nextest `binaries-only` listing builds without executing test
binaries, then separate metadata-based discovery and replay use the existing
4-GiB limit. An owned build tree is mounted read-only for replay, with a
separate writable report store and explicit checking of the complete binary,
dynamic-library, `OUT_DIR` and loader closure. This bounds writes from the
sandbox; it assumes no concurrent same-user host writer and does not claim
that the mount prevents such a writer. If that custody cannot be established,
stop. Fresh JUnit, independent mandatory UI identities, full discovery/result
equality and effective process limits still need actual controls.

The re-entry uses the original 23:10 UTC deadline, original candidate and
repair limits, and already consumed work; it does not authorize Stage B.
The reviewed delta and concrete synthetic-fixture proposal are retained as
`~/.local/share/nevrotick/ui-test-build-resource-2711/no-archive-alternative-review-v3.md`,
SHA-256 `9faee74642080c0aca067ee9315b64e98509b2440fd130a747567e6094c7acee`.
The fixture's loader, nested build output, discovery-entry marker, five-second
subprocess bounds and report placement are unexecuted requirements. An
independent reviewer initially mistook escaped display text for literal
backslashes in the proposed arguments, then withdrew that finding after
byte-level readback; the exact arguments contain no such backslashes.

Re-entry stopped incomplete at 22:30 UTC. A read-only target and restricted
`PATH` do not by themselves prove that executable or library resolution
cannot use writable temporary or report paths, and no admitted mechanism was
established for the reviewed complete-closure claim. The disabled partial
scaffold is retained in `~/.local/share/nevrotick/ui-stage-a-2711/reentry-tree`;
its patch SHA-256 is
`6ed02cd127cdb6085fd56b9af63a9ee8c329177cc701339e05f1ac355399615b`.
It has no connected selector or test methods. No test, Cargo, compiler or
native operation ran in this re-entry. The sampled resource-monitor log
SHA-256 is `b53f067f2df23f389b8e2fdc63e20e14e35fdad144ced3bd24524e06ba544372`.
Further work needs a reviewed enforceable confinement mechanism or an
explicitly narrower evidence claim; the UI candidate remains unverified.

Independent source and contract review found that the existing §3.3 sandbox
does not establish a universal ban on execution or dynamic loading from its
writable work and temporary paths; that stronger claim was introduced by the
stopped proposal, not an existing completed gate. The reviewed narrower
re-entry keeps the original isolation, exact selection, result and resource
checks and mounts the built artifacts read-only to the sandbox, while stating
the writable-path and same-user-host-writer limits explicitly. It scopes the
no-build claim to the pinned Nextest metadata-reuse mechanism and requires
real known-path refusal controls rather than treating `PATH` masking as proof.

The independently reviewed mount and custody delta is retained as
`~/.local/share/nevrotick/ui-test-build-resource-2711/no-archive-alternative-review-v5-delta.md`,
SHA-256 `5b5ca64438224755fd3556e11f682a70e0cba621b3e42b9bac40ab20d67fc9d9`.
The follow-up v6 corrects the source-anchor explanation without changing scope.
The existing disabled partial scaffold remains the same unfinished candidate;
no repair or test invocation was consumed. Resume only within the original
23:10 UTC deadline and original remaining control limits. Stage B stays
unadmitted; no universal loader closure, hostile same-user test-code protection
or product pass follows.

The focused controls first ran 37 cases with one synthetic fixture link
failure. The one permitted causal repair added its missing library linkage;
the second focused invocation passed all 37, including the pinned synthetic
build, discovery and replay controls. Independent source review then found
that the new JUnit reader opens the sandbox-writable report before checking
its type, without nonblocking mode. A FIFO at that path could block the
verifier outside the subprocess deadline. The repair allowance was already
spent, so the broader suite did not run and Stage A stopped incomplete. The
focused pass does not establish the missing FIFO refusal or a UI product pass.
The first and repaired focused receipts are retained under
`~/.local/share/nevrotick/ui-stage-a-2711/`, with SHA-256 values
`b2488fd5fbf0490d10a675d6d83d76728d324779f33bd2ca5d60156027cba89d`
and `806e9245212c18a5eec2de88e4afdf06be77d85e81de035c385413c5ff86ee59`.
The frozen repaired patch has SHA-256
`15251fcd167c7c50db398ccc06445deb269c2f8faf8d20f9278370ba33c8f1f9`.
Further source review found that its unfiltered Cargo metadata is checked
against a host-filtered graph policy; the synthetic control replaces that
validation. A real workspace rejection is a source-supported concern, not an
executed result. The focused synthetic replay also compares identities in a
test helper rather than challenging the actual UI dispatch with equal-count or
wrong-binary substitutions; required entry-point metadata-refusal controls
were not located. The actual UI dispatch does compare full sets in source, but
that is not an executed discriminator. These findings remain to be resolved
before reuse.

The H09 source child retains both default verification rows and adds explicit
engine-ke-association dependency/portable rows with unique receipts and the same
complete freshly acquired lockfile/vendor/tool/advisory inputs. There is no new
prepare-product profile flag, dependency or all-features row. Existing verifier
selection requires twelve KE cases in ordinary and optimized Engine runs. Extend
the actual-shell workflow controls for profile routing, fail-fast and fresh
receipts; their inert execution is not an actual hosted product run. Preserve the
35-minute job/worker bounds and native/provider holds. Record integrated source
checks separately from the still-required hosted exact-candidate results.

The source checkpoint now passes all 51 repository-security Python controls and
pinned workflow analyzers. Local explicit-EP1 Engine tests, default compatibility
and GNU/MSVC static rows pass within their focused source scope. The broad local
folder secret scan remains blocked on ignored retained service receipts; current
tracked-candidate paths are not those findings. Preserve that raw failure and
check the complete materialized candidate with unchanged scanner configuration.
H08 owner approval and actual hosted exact-candidate execution remain missing;
source routing and passing inert shell controls are not their substitutes.

Complete staged-source materialization now passes the same pinned scanner and
unchanged-config directory check, with all raw failure/scope distinctions retained.
This satisfies this source candidate scan only; ignored local service receipts
remain outside it and their broad-folder findings remain unresolved. H08/H09
settings/hosted qualification remains open under the pending owner decision.

### Pure-role indirect-clock restrictions — source-only H12, 14 November 2026

**Later execution result:** the [focused hardening verification](engine-transport-pilot.md#focused-hardening-verification--14-november-2026)
records all twelve configured pure-role lanes passing under Rust 1.100.0.
Each of the six added role/method cases fails with its intended Clippy
`disallowed_methods` diagnostic; both healthy controls compile. These fixture
functions never execute clock reads. The earlier source-only/hold record below
is superseded by that scoped result, not whole-product qualification.

The [unified continuation](engine-transport-pilot.md#unified-handoff-source-only-continuation--14-november-2026)
routes selected H12 through this strict-policy owner under CHECK-19/25/26/27.
Baseline is `bad2db638bb112496d16bb1172835473440b421e`, declared Rust 1.100.0.
No compiler, test, mutation or configured check driver executes under David's
current hold. Existing future qualification commands do not override it.

**Applied, not executed:** `scripts/check-strict.py::PURE_METHOD_PATHS` and
the `time-domain`, `ntp-exchange` and `time-core` Clippy configurations now ban
exactly `std::time::Instant::elapsed` and `std::time::SystemTime::elapsed`, with
the existing pure-role reason. These methods read the ambient clock; retaining
a timestamp type or computing a difference between two supplied coordinates
does not. No product call to either method was identified in these pure crates
in this source slice.

Six separate proposed compiler lanes cover each role/method. Each receives its
timestamp as a parameter, so a forbidden `now()` constructor cannot mask an
absent elapsed restriction. Healthy explicit-pair arithmetic and the permitted
adapter remain in the same fixture; existing alias/function-reference controls
are retained. Configuration omission controls now independently remove each of
the four clock paths in each pure role, restoring healthy bytes afterward.
Nothing claims these controls have compiled or run.

H13 acceptance comparison against `bad2db63`: only the two selected bans and
six corresponding negative lanes are added; thresholds, old selections,
diagnostic requirements and admission are retained. H14 review finds no new or
widened lint suppression in this delta. Source-aware suppression automation is
deferred: this bounded path-list edit supplies no selected syntax/scope contract
for conditional or generated-code exceptions. No dependency/parser framework,
blanket API ban or graph/toolchain change is introduced. Current-source compiler
and policy results remain pending; historical receipts retain their own bytes.

### Effective mutation compiler concurrency — 14 November 2026

The [current Core trial and causal correction](s1.3-source-selection-readiness.md#seven-owner-mutation-admission--14-november-2026)
show that ten outer workers plus ten inherited jobserver tokens admit twenty
compiler jobs despite `CARGO_BUILD_JOBS=1`. This is an execution-recipe defect;
it does not establish a Rust-upgrade regression or product mathematical defect.
The GNU-only phase/Step fix selects zero additional tokens and strict 10/8/6
outer profiles, preserving all existing caps and gates. Its 302 Python controls
pass. After David's resource authorization, the actual control observes sixteen
compiler processes under eight extra tokens versus eight under zero, one per
Cargo. Its 88 compiler traces have independent raw-log/identity/interval review;
both five-case baselines and ten-variant fixture sets complete, with zero product
kill credit. Peak memory is 414,908,416 bytes in 19.758 seconds, without OOM or
PID-limit events. Subsequent product trials and causal-reader repairs retain
their exact historical scopes in the Core task; they supply no current score.

The [V5 numerical assertion stop and final V6 envelope](s1.3-source-selection-readiness.md#v5-numerical-assertion-stop-and-final-v6-envelope--14-november-2026)
preserves ten outer workers with zero extra jobserver tokens. V5 product
`a3facab1` ran all 1,243 variants and passed six scoped owner gates; phase-numeric
remains incomplete after 23 caught rows lack admitted causal evidence, including
three without a semantic assertion. Its original failure and V5's baseline,
coverage, compiler and validation results retain their historical scope.

The [V6 final qualification](s1.3-source-selection-readiness.md#v6-final-qualification--seven-selected-owners)
passes on 603-input product `7701e3b3`, private policy `5e127c28`: 720 baseline
cases (608 ordinary/112 doctests), seven line gates, 3 legal/32 negative compiler
controls, 21 validation specimens and all seven 95% mutation gates. All 1,243
variants reconcile as 819 caught, 408 compiler-unviable and 16 exact equivalents,
with zero owner timeouts and no added kills from equivalence.
The first union remains incomplete at 90 seconds; its one reviewed data-only
completion passes in 156.470110368 seconds, peak 322,715,648 bytes. Both attempts
remain charged to the original 9505.109141993/10,800-second ledger; no reset,
Rust rerun or old-receipt adoption follows. Main stays closed.

After staging, [development §7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
routes the accepted [truthful numerical CLI projection](engine-transport-pilot.md#truthful-numerical-cli-projection--contract-preparation-14-november-2026)
child. Freeze its source/checks/budget before implementation; this routing claims
no implementation or new qualification.

Outer-worker configuration is not a universal compiler-process-count ceiling.
Reconcile effective implicit/shared slots before admitting other profiles.
No compiler/tool binary, dependency graph, shipping profile or Windows setting
changes follow. Native 235-case evidence remains tied to historical `1cbddfb3`;
full Core/native/release qualification and the separate H02/H08/H09 and supplier
gates remain open.

### Engine resource continuation — 15 November 2026

The [V13 association exit](engine-transport-pilot.md#v13-association-gate-and-bounded-resource-exit--15-november-2026)
uses the current two-thread full-inventory checker and reviewed fd-relative
observer, after owner-requested RAM cleanup and explicit heavy-work restart.
Six mutation workers/zero extra tokens execute all 35 descriptors beside the
484-case baseline. Independent raw review accepts 28/28 viable kills and this
bounded six-GiB resource cohort: peak 5,724,635,136 bytes/112 tasks, zero limit
events, final tool/source/vendor custody and actual cgroup retirement confirmed.
The two-second guard stays unchanged; its observed maximum age is
1.375076063 seconds. Disposable copies are deleted and raw results retained.
No compiler, graph, shipping policy, Windows setting or security threshold
changes. Eleven GNU mutation owners and four additional critical/native owner
gaps remain. Further selections retain shared deadlines, fresh private admission
and cumulative budgets; one cohort is no whole-profile/resource guarantee.

### Shared-verifier case-identity follow-up — 18 November 2026

**Source review, planned implementation:** on `32e56be3efae69adf70dc349fbd2eb7d87fa8930`,
`scripts/verify.py::validate_test_result` rejects empty, wrong-package, skipped
and failed results and checks selected Engine/optimized sentinels. It does not
generally reject absent testcase names or duplicate qualified testcase identities,
nor require every claim-critical ordinary case. `summarize_test_counts` already
rejects missing or duplicate package lanes; retain that distinct protection.
Stronger Engine analysis readers already check their selected rosters. This
source observation establishes no omitted actual product test or invalid receipt.

At the next substantive shared-verifier edit, the existing build/tooling owner
under CHECK-6/19/25 should bind each selected mandatory case to package, test
binary/suite and case identity in the affected target/feature/profile. Keep
planned, discovered and executed identities separate. Reuse current roster and
receipt owners; no second accounting framework. Observable exit: the actual
result consumer rejects missing names, missing mandatory cases, duplicate
qualified identities and same-count substitutions; healthy selections and equal
test names from distinct legitimate binaries remain accepted. Preserve existing
package/sentinel, status, native-exit, custody and failure-propagation checks.
Apply [harness qualification](../.agents/skills/testing/references/harness-qualification.md)
and validate only the affected admitted lanes when implementation is authorized.

If precise result types help that bounded change, apply
[Python verification typing](../.agents/skills/testing/references/python-verification-typing.md)
to this result and its immediate dynamic-import/consumer seam under the admitted
checker. Complete/partial records, real module compatibility and bounded runtime
validation remain separate obligations; do not hide unchecked data behind casts
or expand to a repository-wide conversion. A non-executing plan view remains
conditional on a measured need and the existing command builder. The command
`cargo test --no-run` compiles and may execute build scripts; it is not a plan view.

The stale Clippy version comment is in a bound product input, `clippy.toml`.
Refresh it with the next substantive admitted pin/configuration maintenance,
including normal source re-admission; this documentation slice leaves those
bytes and historical receipts intact. The current product priority remains
[development §7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent).
No runner implementation, tool admission, product/native execution, campaign
restart or post-V1 formal work is performed by this follow-up.

#### Bind selected Nextest cases to their binaries — 26 November 2026

CHECK-6/19/25. The review of `b161229c` confirmed the scheduled gap on
`a5d877c`. The previous JUnit consumer admitted unnamed and duplicate
cases, a same-count substitution and a required Engine case reported
under the wrong binary. These were directed report controls.

The [shared verifier](../scripts/verify.py) now checks successful JUnit
identities by package, Nextest binary ID and case name. Equal names
remain valid in different binaries. The planned portable selections
cover the two retained-reply controls beside the existing Engine
processing and profile-specific KE cases, the four optimized
discriminators and the patched `spin_on` controls. For these rows a
separate Nextest JSON listing must contain the mandatory identities.
The passing JUnit identities must match that live selection. The receipt
keeps planned, discovered and executed identities and the prior count
fields. Doctest accounting remains a separate lane.

The release EP1 lane builds the same archive, lists the original
extraction and runs against its cargo and binary metadata. Archive build,
discovery and execution share the original 300-second wall deadline.
Only the original archive build receives 240 CPU seconds. Discovery
and replay keep the ordinary 90-second limit, and a fresh JUnit is
required for execution credit. The focused controls cover the same
name in different binaries, a missing mandatory discovered case, a
missing discovered case at replay and expiry during discovery.

The guarded GNU EP1 run passed 28 selected lanes on 761 inputs, including
the Engine compiler, strict Clippy, 91 doctests and 674 qualified cases in
each mode across the same nine binaries. Each listing matched its fresh
JUnit, and both modes executed the two retained-reply cases. The optimized
execution reused the listed archive without compiling and started with
163.775 seconds of the shared wall budget remaining. The final verifier
retains semantic refusals in the command row and reports a pass only
after validation. Its current parser reproduced both rosters from the
guarded raw listings and JUnit files. The isolated suites passed 206 shared-verifier
and 63 dependency-profile controls.

| Record under `~/.local/share/nevrotick/gnu-ep1-q01-audit-2611.pf9FLDhg/` | SHA-256 |
| --- | --- |
| `verification.json` | `ff151afd70585af949f5c20f0d6ea905f63b0582f04e8f6cd56a8bece9013e64` |
| `final-parser-readback.json` | `bcd50a3f844eb0339618eb497bee4b8caa41daddc064ead596a9da67aaf769eb` |
| `reply-outcome-diagnostics.json` | `de78ec252c5170916a39f053e232506dab78e357df3c5403ee01408d18305297` |

The final verifier also passed the six selected GNU default-profile
identity comparisons. Engine matched 315 cases in each mode. The
optimized domain, exchange and Core rows matched 71, 63 and 624,
respectively. The patched `spin_on` row matched all three required
cases. The 29-lane receipt at
`~/.local/share/nevrotick/q01-portable-selected-2611.NdHsuHpI/verification.json`
hashes to `6a6c7d2d2a3d1b24426456fec7a531f296f5fe205b673aee2f72772a835e2e3c`.

The same audit confirmed the missing Rust `must_use` conditions at the
direct managed result and the serial tuple result. Type-level attributes enforce
those conditions. The two wrappers that return an optional retained reply
also carry the handling requirement because an ignored `Option` can hide it.
Pinned Rust 1.100.0 diagnostics at the extracted wrapper bodies pair each
ignored call with legal handling. The new expressible locals in the retry
and tray paths use the existing domain types. Four source-extracted portable
tray controls passed. Native Windows and fresh metric qualification remain
with their existing owners.

#### Type Nextest discovery and exercise equal-count replay substitution

CHECK-6/19/25. The follow-up against `91265bd` confirmed two gaps in
the current verifier. The discovery reader now keeps decoded values
unknown until the existing shape checks admit them. The command runner
declares its text-or-case-set result. An unexpected result kind fails
the discovery row before it records case identities or emits a pass.
The discovery caller narrows the result before replay.

The archive control keeps every mandatory case and replaces one other
discovered case with a different successful JUnit case. It checks the
replay row's retained failure, absence of test credit and absence of a
pass message despite a successful subprocess exit. The identical
inventory and missing-case neighbors still distinguish success and
omission. A one-change count-only comparator passes those neighbors
and fails the new substitution control. Removing only the early
result-kind guard fails the new discovery control. The admitted Linux
Python 3.12.3 namespace passed 211 verifier and 63 dependency-profile
controls on the final source. The separate repository mypy lane covers
its selected accounting helpers and supplies no verifier typing result.

All four historical receipt digests above match. Independent readback
reconciled the recorded source and tool hashes and exact command
selections. The two original EP1 listing/JUnit pairs each contain 674
qualified passing cases in nine binaries, and the retained output
records 91 passing doctests. The current parser reproduced both pairs.
It also reproduced all six default inventories from the retained
listings. Their case records agree with the original PASS transcripts.
The historical compiler receipt and reconstructed specimens match their
source hashes and record four intended `unused_must_use` rejections
alongside successful legal handling. Its compiler identity comes from the
retained GNU rustc 1.100.0 version output. The receipt lacks a compiler
binary hash for that specific specimen invocation.

Raw default JUnit XML, the original EP1 archive and extraction, the
historical vendor snapshot and sandbox mount state, and separate logs
for the earlier 206 and 63 controls were unavailable to this readback.
The current controls and parser replay ran without a new Rust product
build or Nextest execution.

| Record under `~/.local/share/nevrotick/q01-type-and-substitution-2611.vAYiVX3G/` | SHA-256 |
| --- | --- |
| `source-map.json` | `a3c0c5219c615ca9d2ac5cbfb8897e8abf33eb7d6e814edcda9748851faf7af8` |
| `verifier.log` | `a9f03898c27dede5871d067f50ad77687a015803fe59716b58f021072aaf71c7` |
| `profiles.log` | `ac416affc13e220a8242e3c3cb8831e4afebcfa6989941cbc01fbd4b6283cb0d` |
| `ep1-profile.log` | `1ee2ed7722dcda75bef051a7403abf8bdb1e9f0a6c76c04b37f57e1fabafac40` |
| `count-only.log` | `a3fa155a4fbf1f009d1941bf32f0b8235b7426f496c9f4d7ce3f11a30a31274b` |
| `no-kind-check.log` | `de8850f6716a26dae8b5bed22e602c9befd351bec0b159975149ee20b4dea20e` |
| `current-parser-readback.json` | `9132aca9e6c62cc07ef658fc0d378ea6ab6d38735bc4360a6ab620f14bb9a5a7` |

## Linked Windows PE hardening gate — 21 November 2026

W11-W10/P6, CHECK-18/20/23. The configured Windows build now checks the
hardening declarations on the actual UI and Observe executable bytes before
accepting VERSIONINFO or exporting either image. It reuses the service
fixture's static PE reader with separate GUI and console expectations. The
bounded reader checks AMD64 PE32+, executable and relocation declarations,
CFG, ASLR, DEP, high-entropy VA, strict CET, the selected dependent-load
mask and the absence of writable executable sections. It bounds the
file-backed CFG function table using its encoded stride, and checks that
the RVA and on-disk pointer name the same CET and CodeView records.
The parsed bytes' SHA-256 controls subsequent LLVM/readback and export.
The selected PDB basename must agree with CodeView before either pair
is copied. The service and Engine artifact owners use the same reader and
keep the captured image digest through inspection and export.

Build-security, package-artifact and regression skills guided the
selection of actual image controls, the parser extraction and output
custody. Microsoft Learn MCP confirmed the [PE CFG metadata layout](https://learn.microsoft.com/windows/win32/secbp/pe-metadata),
[RVA and raw-pointer meanings](https://learn.microsoft.com/windows/win32/debug/pe-format#special-sections)
and the [static dependent-load flag scope](https://learn.microsoft.com/cpp/build/reference/dependentloadflag).
The parsed declarations and file-backed CFG extent keep their static
scope. Code-path instrumentation, effective Windows mitigations, the
per-role `0x800` import closure, package contents, signatures and
installed trust retain their existing acceptance owners.

On baseline `82ea4c68710f` with the prior verified VERSIONINFO working
changes, the actual former consumer admitted both retained release images
even with only `NX_COMPAT` cleared. The same modified bytes now fail the
static check before LLVM runs. Both original artifacts still pass with
their exact original build hashes. The reader controls also cover missing
flags, wrong subsystem, bad CFG table spans, inconsistent debug addresses,
relaxed CET, disallowed load flags, truncated images and nonregular or
oversized inputs. Final host selections passed 141 product-verifier,
93 Engine-admission, 12 VERSIONINFO and seven PE reader tests. The
Engine admission control changes the artifact after its initial
inspection. The old exporter accepted it and the repaired consumer
refuses it. A separate product control accepts the exact CodeView
basename and refuses the other allowed spelling before either copy.

Independent review checked the combined helper bindings and found the
inspection-to-copy identity gap and the PDB selection ambiguity. It
accepted the follow-up guards. Two retained Engine facade images also
passed the new static reader through the fixture's console adapter.
Their former native and compile receipts retain their original scope.
The actual NX controls are retained at
`~/.local/share/nevrotick/windows-pe-admission-20261121.daxkxk4o/`.
`red.json` has SHA-256 `6f3adbe69b990f5becb9810158cdfd2f17e5b2d7fdf5fdb9246e0951c7646557`
and `green.json` has SHA-256 `2698b2cf80e4ac7f278aeb32fc4d5148c8f2506f5fd2a87f8d30802cbf1a6274`.

The amended default `scripts/verify.py --lane windows` passed all 32
configured rows. Rust 1.100.0 and LLVM RC 22.1.8 used the admitted vendor
and MSVC libraries. The 637-input candidate's canonical map SHA-256 is
`f4445b252e22c8bc646df4864313c0e5ec631604bcf708105c47d5f770318cc8`.
Every input matched the working bytes after export. With six CPUs and an
owned `/dev/shm` scratch directory, the run finished in 392.66 seconds
within its 1,000-second budget. The scratch volume retained at least
4.13 GiB free and the run retired the temporary build tree. The
durable service-metric admission remains a separate ext4 requirement.

Both final PE files report `0xC160` DLL characteristics, `0x10500` CFG
flags, `0xA00` dependent-load flags and CET declarations `0x3`. The
file-backed CFG table contains 7,914 UI entries and 302 Observe entries.
The independent export inspection confirmed the exact two EXE/PDB pairs
by image hash, CodeView GUID, age, and basename. It checked the complete
five-file export inventory. The same directory retains the full records.

| Receipt | SHA-256 |
| --- | --- |
| `fresh-build.json` | `f0058423d481da67119f4b892f55ddb32f99de47e8d24c85572672c007752cff` |
| `fresh-build-custody.json` | `49a7c1fee2ef31f11aadaf0e5a49be890dc4ee0c80b9ebe400377c9a49347a90` |
| `inspected-exports.json` | `f6a2487b0f474756d49d68e423771a38a597ef9f205cc656ba6515485477b9e1` |

### Configured export needs EXE/PDB pair identity — 21 November 2026

Under CHECK-18/20/23, the actual `export_windows_artifacts` accepted both
genuine pairs and an Observe export using the genuine UI PDB under the exact
`nevrotick_observe.pdb` basename. Both executable hashes stayed identical.
Independent LLVM readback recorded Observe's expected CodeView GUID
`5385F3A4-5460-E43A-4C4C-44205044422E` and the substituted PDB's actual GUID
`E0A6EABA-C806-42DB-4C4C-44205044422E`. Both are age 1. The receiver checks
the symbol basename and copy identity. Pair matching also requires the PDB
signature and age. [Microsoft Learn](https://learn.microsoft.com/visualstudio/debugger/debug-interface-access/idiadatasource-loadandvalidatedatafrompdb?view=visualstudio)

The offline readback used live exporter SHA-256
`dd14e14624e0817cec3392d3801d43b3355bdccb036269e649e521b4668bf780`, the admitted
retained images, a private output tree and independent readers. The receipt
`pdb-pair-export-readback-20261121.json` under the same PE admission directory
has SHA-256 `958ede85790a95c0dcc6760294098c539ba517758185a5ef254f440dd6d35c4e`.
It preserves the initial import setup failure. The retained inputs and tool
hashes matched at both bookends, and the probes retired scratch. The earlier
real matching pairs retain their recorded scope.

#### Pair admission implementation and controls

The configured exporter loads `windows_pe.py` from its frozen source snapshot.
It captures both regular PE/PDB pairs, compares the RSDS basename, GUID and age,
then writes the inspected bytes. Both image hash checks and the source and
output digest bookends remain in place. The reader follows the bounded MSF 7
directory to the VC70 Info header. It rejects malformed extents, reserved pages
and overlapping block references. The admitted emitter uses 4 KiB pages. A PE
may be at most 64 MiB and a PDB at most 256 MiB, matching the existing file cap.
This gate checks symbol identity. Debug stream payloads remain opaque.
[LLVM MSF layout](https://llvm.org/docs/PDB/MsfFile.html) · [LLVM PDB info stream](https://llvm.org/docs/PDB/PdbStream.html)

The maintained export function accepted the genuine retained pairs and their
restored copies byte-for-byte. A same-name wrong-role PDB and a same-GUID
wrong-age PDB both failed with an empty output directory. A separately seeded
page-layout check executed 32 healthy two-page directories and 96 malformed
neighbors, covering scattered page positions, directory aliases, stream aliases
and an overlong directory map. Each case used a physical input and the real
reader. The test corpus added no tool dependency.

All 199 selected Python tests passed after restoring the existing vendor-first
admission order. An initially oversized sparse PDB fixture was refused by the
configured sandbox before the reader could run. The corrected fixture exercises
the same size guard with a lower test-only bound. The exact isolated PE/PDB test
command passed all ten cases before restarting the Windows row. The reported
first attempt and its setup failure remain in the evidence directory.

The `windows-pdb-pair-current-20261121.6Zfr92yL` evidence directory under
`~/.local/share/nevrotick/` holds the current checks. The first retained-pair
green receipt remains with the original PE evidence. A later I/O or source-drift
failure can leave partial copied output. Such an exit is incomplete.

The configured `--lane windows --profile default` replay passed all 32 rows
on 637 inputs. The canonical source-map SHA-256 is
`bdf556427cf0d885dbc0db5b15a75ebe15514b9077f82363e1050f97dfb36762` and the
unchanged lockfile SHA-256 is
`26dd7d3427afd0b0907c48afc1bc6017bdd0242f1748884f7e2d848132c4e5db`.
It used the pinned Rust 1.100.0/MSVC cross toolchain and admitted libraries.
The first stopped attempt and successful replay consumed 469.58 seconds,
including admission and final custody, within the original 1,000-second
allowance. The replay kept at least 4.13 GiB free in its owned scratch and
retired that scratch after source and output verification.

Independent `llvm-readobj` and `llvm-pdbutil` readback matched basename,
GUID, age and the four emitted file hashes against the export receipt. The
real export then accepted healthy and restored copies of the newly emitted
pairs. It refused both a wrong-role PDB and a separate age-only change at the
same Observe basename with zero destination files.

| Evidence leaf | SHA-256 |
| --- | --- |
| `custody.json` (first attempt) | `65a2d201ea11068220550848679c5ac76bffe73beff88d1dc598f6c03713b721` |
| `pdb-page-layout-controls.json` | `bfaa3c6750b4f192c2dad3c18d3738ec251a9e9096c69a09a5fe17ff0b44089a` |
| `replay/final-python.json` | `847725c06a26ddda34df2a9ed1467c87c80ebb41476cd6393236adf145274dba` |
| `replay/runner.json` | `941ebee105a151093cd1dda822d817ee37f922f7572aae01a3f4837b049ee7cf` |
| `replay/custody.json` | `dae441d8c0dcb6242d5988f626da2715b99b7bdae05e49e978a365b05c11096e` |
| `replay/independent-pair-readback.json` | `1e4b408b9b1230c977ba5b34bea0533ffb3a89d9b494bf5a4b1612c76af2bf59` |
| `replay/fresh-pair-controls.json` | `9289e2686ca534bf8ea9aaa119e3a156bc22e1da7da0134754916ddb45d5c018` |

The service fixture and Engine native reports establish byte custody. Semantic
PDB pair admission remains open for those two exporters. The shared reader
hash changes their future candidate bindings. Installed behavior and complete
MSIX delivery remain under W11-W10 and their respective native owners.

### Engine and service fixture symbol admission

The sibling exporters now require the original CodeView basename, GUID and
age for every selected pair before publishing any pair. The Engine exporter
uses private staging and retires already published owned links if a later
publication fails. It preserves a foreign replacement. The service exporter
uses one private directory publication after verifying PE declarations,
LLVM inspection, input hashes and the final index. Failed publication or
receipt custody leaves the lane incomplete.

The baseline accepted substituted symbols with the correct basename. The
Engine and service controls refuse wrong-role and age-only substitutions
before export. Healthy and restored controls export exact bytes. The service
controls also reject a drifting symbol, index drift and an existing linked
receipt. All 110 focused Python cases pass. The first actual Engine lane
refused the missing test-helper import in sealed source. Adding that exact
test fixture to the runner's helper inventory made the import reachable.

The configured Engine facade cross-build passed all 20 lanes after the fix.
The 623 product inputs and 17 helper inputs matched that checkpoint and
all four physical PE/PDB pairs passed independent LLVM CodeView, GUID, age
and hash comparison. Both attempts consumed 141.74 seconds cumulatively.
Evidence is under `~/.local/share/nevrotick/windows-sibling-pdb-20261121.XcyOx9CT/`.
The independent readback has SHA-256
`096782e577cacfea5d080c2a8b72cb601558ed5990b8b56a32a7630ac5b5b8ca`.
The later service receipt repair changed a shared Engine helper. The earlier
Engine build receipt keeps its original helper identity.

The service cross-link receiver previously required a four-command graph
while the configured graph producer and its own semantic reader required
six. It also expected receipt paths and test counts from an old source
checkpoint. The receiver now checks the exact current six-command graph and
twelve-command source checkpoint, reads and hashes the same receipt bytes,
reconciles the source, index, command, vendor and tool identities, and binds
the independently reviewed full test names. Controls reject stale or
substituted receipts, wrong commands, same-count wrong tests and skipped
cases. Direct invocation now reaches all 13 service reader/export controls.

Fresh configured graph and source checks passed on 654 inputs, candidate
`c142837a97520cfb5cc11fb51bdcd8f31aa2061588bf72ba65a47ec9e9e1da6a`.
Each profile ran exactly 70 platform and 121 fixture tests. The matched source
checkpoint also ran 28 platform doctests and the explicitly empty fixture
doctest selection. Six graph, twelve source and three cross-link commands
passed under the current tool, vendor, index and resource guards. Both fixed
AMD64 console roles cross-linked and exported. Independent LLVM matched the
exact CodeView basename, GUID, age and hashes of both PE/PDB pairs. The shared
reader refused the freshly built broker PDB under the engine PDB basename
and accepted the restored file. An independent reviewer confirmed the command
and named-roster evidence against the current test declarations.

The source graph, checkpoint and link receipts `pair-current-graph-2111a.json`,
`pair-current-development-2111a.json` and `cross-pair-current-link-2111a.json`
are under `artifacts/windows-services/recovery-20261107-v1/`. The cross-link
accepts the exact checkpoint via `--source-checkpoint`. The link receipt's
SHA-256 is `437c8b73dcf84fd3cde76c3bcfcf85646d7fe27952a60569b0e3c7c509a87213`.
The `service-cross-current-20261121.sC3OnkTu` evidence directory under
`~/.local/share/nevrotick/` retains both bounded scratch supervisors and
independent PE/PDB readback SHA-256
`1f39e02a3e8308a6b2c6f55dece3df872abeeed0c8f0f4312437f16663eba212`.
Native Engine and service qualification and each existing metric gate retain
their separate evidence.

### Preview admission source checks — 23 November 2026

The strict checker has a dedicated source-only lane for the private
`PreviewInput` declaration in the UI model. It requires the direct
Garde derive and the three current private field-rule associations, and
reports the prohibited `allow_unvalidated` option. The selected rules
return an occurrence location and a structural finding. Changes to the
reviewed declaration require a corresponding checker and control review.

The ordinary Rust workflow executes the lane and the focused Python
controls, alongside both existing strict-policy Python suites. Local controls
use actual model-source copies and the real CLI dispatch. They distinguish
healthy syntax and comment/string decoys from removed or weakened rules,
conditional derives, widened visibility, missing owners and unsupported
syntax. The workflow shell control checks exact routing and propagates an
injected lint failure. Hosted execution remains open.

On HEAD `8066540e0c2649178b096fbaa6fbb5ba47da00f9`, tree
`c34d5877c1f94b42cb20460b563085fc778631ba`, the checker overlay
passed the 62 strict and preview controls, seven workflow routing controls,
189 shared-verifier controls and the skill-pack check. The selected source
SHA-256 is `e433cba003ba7dff0c20e3259f5815ea6962182f85b999ef09739622aa63a49f`.
Independent review caught a literal scanner false positive and a workflow
selection that could succeed with a missing suite. An ordinary string ending
in `r` now passes, and CI names each suite directly.

This is the declaration portion of CR-03. CR-01, constructor call and order,
the fixture and actual Slint commit chains, and occurrence-specific closure
for advisory findings retain their separate design and acceptance work.
The selected rules do not infer effective cfg, Garde expansion, effect ordering or runtime
behavior from source spelling. The Stage A Engine receipt retains its
original source and tooling identity.

### Review selected preview contracts and commit seams

This continues the 23 November declaration check under CHECK-25. The local
baseline and live Main were `70586c24ba2ebad6d531db6be5e6fa669c72625d`,
tree `96ff3089ff1fc528278759894ec892838203bfd9`. The report inspects the
unchanged handwritten model, fixture presentation, source-draft and Slint
workflow source. It adds no Cargo dependency, generated input, toolchain
selection or proposal to change hosted trust.

CR-03 retains the declaration checks and reports token changes to the
constructor and its checked commit body. The rest of the small model module
is a separate surface check for new construction/mutation routes, changed
bounds and changed validation rules. CR-01 screens nine named interfaces
and the named receiving bodies for edits that may change error or commit
behavior. An exact adjacent CR-01 expectation can close a supported
advisory and retains its raw record, owner, consumer, evidence and recheck
condition. The control with a well-formed but false forwarding rationale
checks attachment only. Independent source review refuses that specimen
because it swaps host and label. No production expectations were added.
Use the [tool instructions](../TOOLING.md#ui-preview-source-report) for
selection, syntax and review. Import and alias resolution, expanded macros,
unselected callers and effective cfg are outside the selected syntax report.

The candidate keeps the previous declaration refusal cases and adds controls
for omitted validation, bounds moved after validation, a scoped output
field, added unchecked construction and mutation routes, early state
mutation, all named consumers, conditional impl scope, and missing rule or
backend. Valid source and ordinary comments pass. Exception controls keep
the finding attached and fail malformed, misplaced, duplicate or stale
comments. A lexical control transplants the actual Core `RowCandidate`
declaration into a copied consumer file. Its typed `NonZeroU64` skip is
accepted and the prohibited-option companion is reported. This changes
no Core source and adds no Core execution credit. Sources that use
unsupported literals fail as incomplete. The
unchanged Rust workflow runs the real command and names the focused suites.
Its seven local shell-routing controls verify nonzero status propagation.

Python 3.12.3 produced a passing source report with no findings.
`scripts/preview_contract.py` SHA-256 is
`6a6becae40202e441a578094f321b96c1e39ac322d7d5d0ad88b36d8b229d506`.
The captured report records hashes for all four sources in
`/tmp/nevrotick-preview-strict.M2mB2V9T/preview.json` with SHA-256
`d2803866e52617597241a9c316136d0f3813ed6e5289535fc59781b14824abbf`.
The local pinned Rust 1.100.0 strict run passed 47 compiler/tool controls and
12 pure API controls. The final `fixtures.json` beside that report hashes
to `8bde21f5876ba7f856641cf238805b048c9e60d42cf8db2a45c5cf32c8321692`.
An earlier 59-lane attempt correctly returned incomplete when concurrent
editing changed the checker input. Its receipt remains at
`/tmp/nevrotick-stage-b-fixtures.RfPMArw4/result.json`.
Independent review traced both UI consumer chains and challenged the report
with its own source changes. The [UI owner record](slint-gui-handoff.md#preview-admission-contract-controls--23-november-2026)
binds GNU validation, visibility, and callback checks to their exact scope.
Hosted execution and native evidence remain open.

The focused Python suites passed 70 tests. The first combined invocation
of the separate verifier suite used module loading and seven cases could
not import their sibling test helper. The configured discovery command
passed all 189 tests.

The Stage B standalone-policy comparison confirmed that CNG lacks
`let_underscore_drop`, `unused_macro_rules`, `unused_lifetimes` and
`string_slice`, and retains its stronger unsafe prohibition. The existing
OD-19 resolution accepted 21 frozen manifests and the separate AES
admission accepted its 43-file inventory. Either proposed lint-table edit
alone failed complete manifest-context admission. Updating the AES manifest
and inventory hashes in a control still failed the independent exact
manifest-shape gate. Route these changes through a fresh OD-19 resolution,
coordinated AES admission and affected GNU/MSVC checks if adopting them.
The hosted Python suites and source-report route are active. Hosting the
full compiler-fixture runner also requires its pinned bootstrap and
extension setup, which the current workflow does not supply. These
assessments made no standalone policy or hosted trust changes.

### Route strict fixture controls through the Rust workflow

The default Ubuntu product job now runs the 47 strict and 12 pure API
fixture lanes after acquiring the exact Rust and cargo-nextest binaries and
before product verification. The explicit tool paths select existing product
tool admission. The workflow fixes system PATH, removes the inherited
Rustup override and records the observed helper identities. An eight-lane
environment smoke run qualifies the actual sandbox before the fixture rows.
The normal local command retains exact bootstrap helper admission and its
full extension inventory. Neither route changes any fixture oracle or product
Clippy policy.

On the working-tree overlay of `70586c24ba2ebad6d531db6be5e6fa669c72625d`,
tree `96ff3089ff1fc528278759894ec892838203bfd9`, both invocations passed
all 59 lanes under pinned Rust 1.100.0 and nextest 0.9.150. The nextest
positive ran two tests. The empty selection refused with status 4. The
explicit runner positively observed isolation and passed the additional
eight smoke lanes. A wrong nextest executable refused before smoke or
fixtures. The first mismatch returned a traceback because the product
module loads its own exception type. The corrected entry point retains an
incomplete JSON receipt with the intended digest diagnostic. The focused
Python suites passed 72 tests, and the eight workflow controls passed,
including exact shell routing and injected nonzero propagation.
Independent H13 review compared both routes against the prior local
receipt. All 59 lane commands, expected outcomes and observed outcomes
matched in order. The additional smoke controls belong to the explicit
runner path.

Local receipts are in `/tmp/nevrotick-runner-final.HMUIr1rw/`. SHA-256 values
are `da8e6ad3e451861c1ad1767f36fdac4424d0e40a02dd3937e87ada1370936039`
for `explicit.json`,
`fb3e6c02bb3d9e4563517b02571a271854d7b357b2399a5e62eb2e39b524fbf9`
for `local.json` and
`1e44dc8d16b2ac6a8603a73956cc428a716a5884b2aa729cca165d5a99fe496c`
for `wrong-nextest.json`. The explicit command was executed on the local
Linux host with the workflow's argument and environment selection. A
GitHub-hosted run remains open under the existing trust decision.
The explicit route passed again after integrating the standalone manifest
and AES admission changes below. Its receipt is
`/tmp/nevrotick-goal-integrated.5zNkWrp6/explicit.json`, SHA-256
`4348d8e242b5979d1cff0407f721463b3b81ffca0939dd487bcc22f36b45b428`.

The admitted local workflow analyzer passed actionlint, ShellCheck and
pedantic offline zizmor on all three workflows, including two healthy
and three rejection controls. The live Actions permissions readback still
showed enabled Actions, `local_only`, and mandatory SHA pinning. The last
Main Rust and security runs for this base both ended in startup failure
with zero jobs.
The local workflow change leaves that owner decision pending.
[Rust run](https://github.com/DavidOsipov/NevroTick/actions/runs/45247944834)
[Security run](https://github.com/DavidOsipov/NevroTick/actions/runs/45247942367)

### Standalone crypto lint policy — 23 November 2026

The standalone CNG lifecycle fixture now carries the four missing workspace
rules. The standalone AES witness carries the canonical Rust, Clippy and
rustdoc tables. Both use the stronger `unsafe_code = "forbid"`. Product
admission still checks the AES nonmember graph and source inventory and now also
requires the exact lint tables. A focused control recomputes both AES source
pins after removing `unused_must_use` or weakening the unsafe rule. The real
admission function refuses both changes and accepts the healthy manifest.

The coordinated OD-19 decision on base `70586c24ba2ebad6d531db6be5e6fa669c72625d`,
tree `96ff3089ff1fc528278759894ec892838203bfd9`, reproduced the unchanged
root lock `26dd7d3427afd0b0907c48afc1bc6017bdd0242f1748884f7e2d848132c4e5db`
in a private 512 MiB/16,384-inode acquisition filesystem. Only the two
standalone lint tables changed in the 21-manifest context. The resolver
recorded zero registry additions, zero removals and no exception. The first
attempt refused the missing code-owned baseline checkpoint before Cargo.
The retry registered the exact prior pinned and admitted lock, registry and
admission identities through the existing checkpoint mechanism. It changed no
registry selection. Both standalone lockfiles also matched their previous bytes.

The independent strict configuration reports for Rust/Cargo 1.100.0 and Clippy
0.1.100 used the root Clippy discovery context, pinned inputs, read-only
vendored sources and bounded offline commands. CNG all-target Clippy passed
with `init-probe` on GNU and MSVC. AES all-target Clippy and private-item
documentation checks exposed genuine omissions in the test fixture contracts.
The repair documents actual setup errors and assertion failures at those test
owners. A fulfilled `clippy::too_many_lines` expectation on
`generate_ep2_response_fixtures` keeps its complete ordered corpus visible beside
the healthy case. Revisit the expectation if fixture ownership or corpus
layout changes. The original and repaired generator each ran once through the
existing result reader, producing the same 36 fixtures and 15,832 packet
bytes. The emitted source hashes to
`7c903e833aa2913a6e6239359a5d214c275f00208bd99d9fa45640087930b2b8`.

The final AES lane passed formatting, all-target Clippy and rustdoc on both
targets, 21 GNU unit/integration cases, and 11 GNU doctests. An earlier
qualification wrapper expected one combined doctest total, while the admitted
rustdoc emitted two successful groups of 2 and 9. The corrected wrapper
requires both groups and retains the earlier output. Initial Python attempts
on an incomplete archive omitted configuration, metadata and discovery
inputs. Configured discovery on the integrated worktree passed 22 KE,
36 Slint dependency, 189 product verifier and two cooldown tests. Actual
product source admission passed with 651 inputs, including all 43 bound AES
files. The dependency policy pin is
`429f31495d870e6f7474bca58d621ce405177fad592cc17683bcfe5ad97f5113`.

Raw results and the input inventories are retained in
`~/.local/share/nevrotick/standalone-lints-20261123.TuOt7c9V/`.

| Result | SHA-256 |
| --- | --- |
| `product/cooldown-resolution.json` | `fc66bac33cb262316ac86b0e79bc5ee9235eb36a1fb423387b495d29376f5210` |
| `cng-with-config-compile-status.json` | `0ed5f0045f87073c216cf03ec0a300abf0f61d3ac1c25d94eb2a24ba61842b12` |
| `aes-complete-compile-status.json` | `f7067788d21700c7b365d3283dcf05f3adb8d2253c68f724aa07d324f2ab4d3a` |
| `aes-complete-generator-reconciliation.json` | `bea3278a1a2030f98fcea69fd0757a241b12e664229a5380659356f79640e8c7` |

These results cover standalone policy, compiler selection and the actual AES
host checks. The CNG Clippy receipt predates the final AES test-only
documentation edits. Its dependency's production body, both manifests, lockfiles
and CNG source are identical in the integrated candidate. Earlier CNG and
Engine records retain their own source identities. Native Windows execution,
hosted status and any product release remain with their existing owners.

### Guard the preview files against inner outcome lint overrides

H14 applies to a specific gap in the CR-01/CR-03 report. An inner
attribute in a selected consumer file can weaken outcome-handling
diagnostics while leaving all selected method tokens unchanged. The
report now examines inner `allow`, `expect` and `warn` for the selected
Rust and Clippy result, future and drop diagnostics and their broad
groups. It follows attributes applied by nested `cfg_attr` without
evaluating the predicates. Each finding points to the owning attribute
span and names the matched lint. Exact item-level expectations,
including the AES generator expectation, retain their own review.

Actual fixture-source controls cover a direct inner waiver, a nested
conditional group, multiline syntax, comments, raw identifiers, and
lowering a deny to warn. Quoted lint names, commas in reason strings,
comments, an unrelated lint and an inert nested attribute remain healthy.
Independent review found the reason-string and inert-attribute
counterexamples in the first implementation. The repaired scanner uses
balanced token arguments and visits only applied conditional attributes.
This is a structural check of the four selected source files. Effective
compiler cfg, outer module attributes outside these files, and macro
expansion retain their separate review.

Python 3.12.3 passed all 73 selected strict, preview and analysis-policy
tests. The real preview command passed with no current findings. The
checker source SHA-256 is
`20461815348c7eabfd6cbbdfe0eadc26a36a6e1468271bba65c107e71fa0e861`.
The four selected source hashes are recorded in
`/tmp/nevrotick-h14.HyNxyGqq/preview.json`. The skill pack and
whitespace checks passed on the integrated overlay of `70586c24`.

### Replay the integrated dependency graphs — 24 November 2026

The affected default and explicit KE graph selections both validated frozen
metadata and all ten workspace and package trees for GNU and MSVC. Rust/Cargo
1.100.0 used the same 651-file source map,
`a8f8ab4c1404d8b59ca36999f1cedf3c5cc593c97e3013afe6136d1e3101f834`,
and the admitted read-only 585-package vendor. The fresh advisory receipt
bound 2,156 files. The normal verifier stopped after GNU for each profile.
The supplemental MSVC rows used the same sandbox, admitted tool/source/vendor
and advisory inputs, graph validators and unchanged cargo-deny commands.
Their source, vendor and tool bookends passed.

All four cargo-deny 0.20.2 invocations returned status 1. Bans, licences
and sources passed. The configured advisory gate reported unmaintained
`chrono 0.4.45` through `i-slint-core` and a security vulnerability in
`url 2.5.8` through the Slint compiler and through `webbrowser` in the
MSVC UI graph. The former has no patched version. Independent review found
both advisories in the selected UI scope and did not find calls to the
affected URL functions in either admitted direct consumer. The graph
findings remain open under the configured advisory criteria.
[RUSTSEC-2026-0543](https://rustsec.org/advisories/RUSTSEC-2026-0543.html)
[RUSTSEC-2026-0592](https://rustsec.org/advisories/RUSTSEC-2026-0592.html)

The first fixed URL version, `2.5.9`, permits the current Slint compiler
constraint and raises `form_urlencoded` from `^1.2.2` to `^1.2.3`.
The current lock contains `1.2.2`. Registry publication times are
23:03:07 UTC for `url 2.5.9` and 23:02:49 UTC for
`form_urlencoded 1.2.3` on 20 November 2026. Their earliest joint ordinary
OD-19 eligibility is 4 December at 23:03:07 UTC, or 5 December at
03:03:07 in Tbilisi. This is a manifest-based lower bound on a future
resolution, subject to review of the complete delta. An earlier
acquisition would require an exact OD-04 exception. Slint 1.19.0 still
declares `chrono ^0.4`; the maintenance gate needs removal or replacement
in the selected graph, or its own owner-approved risk disposition.
The lock, cooldown and advisory criteria remain in force.
[URL registry record](https://index.crates.io/3/u/url)
[Form encoder registry record](https://index.crates.io/fo/rm/form_urlencoded)
[Slint Core registry record](https://index.crates.io/i-/sl/i-slint-core)

For the next dependency decision, keep the two findings separate. The
default and KE Engine package scopes contain neither dependency on either
target. The affected URL API was absent from the admitted Slint compiler
and browser backend call sites reviewed. The maintained UI browser action
uses a fixed HTTPS repository link. This supports waiting for normal
cooldown eligibility for the URL update unless further reachability or
urgency evidence warrants an early OD-04 decision. The projected two-package
change still requires exact graph resolution and qualification.

Chrono is a direct, nonoptional dependency of Slint Core, even with
the public Slint crate's defaults disabled. Slint 1.19.0 keeps that
dependency and changes the Windows accessibility path to AccessKit
`0.25` and `accesskit_winit 0.34`. That path would require review of
the existing exact AccessKit patches and separate UI qualification.
The ordinary repair is a reviewed Chrono-free upstream release or
replacement in the selected Slint dependency. A temporary acceptance
of the maintenance finding would need a separate owner decision that
binds the exact Slint/Chrono graph, expiry and migration trigger. This
handoff records the choice and retains the failing gate.
[Slint 1.19 Core manifest](https://raw.githubusercontent.com/slint-ui/slint/v1.19.0/internal/core/Cargo.toml)
[Slint 1.19 Windows backend manifest](https://raw.githubusercontent.com/slint-ui/slint/v1.19.0/internal/backends/winit/Cargo.toml)

The exact root-lock compiler and Clippy follow-up remains open. The
configured product commands select the entire workspace and require
successful dependency admission first. The existing Engine-only
`ke_association` analysis entry point requires the separately admitted
620-input candidate and 7 GiB of scratch. The current 651-input
candidate and both available Linux scratch stores failed those entry
conditions. The earlier GNU and MSVC standalone CNG Clippy receipts
include the real Engine and AES path dependencies, but resolve
`ctutils 0.4.7` and `inout 0.2.5`, while the root lock resolves
`0.4.8` and `0.2.6`. The follow-up launched no compiler.
[Product instructions](../TOOLING.md#starting-implementation)
[Engine analysis gate](../scripts/verify-engine-analysis.py)

Raw receipts are in
`~/.local/share/nevrotick/integrated-graphs-20261123.NxbuUknf/`.

| Receipt | SHA-256 |
| --- | --- |
| `advisories.json` | `4ab31284672b539396aa1f2f73934652c2ae68ca1b1f139bc7dd0af2b30b70f3` |
| `default.json` | `0d61f0f2a469b476f88c04b63f6457cdd3fb3a542f5a7228eb8b8a88fab07a82` |
| `engine-ke-association.json` | `4843232a201a55761d4bbaaf544b7696b9f764326f17bf84a26f2f89c250c5a9` |
| `default-msvc-supplement.json` | `bab94a0572348dc27875505d89436b427ec564654d3c3beebc6ff4cdd9743650` |
| `engine-ke-association-msvc-supplement.json` | `baed3b9cebc3aeefff77a76f84a7148bfc80f464f435e943d932b847503e3a90` |

### Replace Chrono in the selected Slint desktop graph

The owner decision is recorded under
[OD-13](../docs/nevrotick-owner-decisions.md#slint-calendar-dependency--24-november-2026).
This replacement clears the Chrono finding in the preceding graph replay.
That replay retains its original candidate and results.

CHECK-19/20/26 and S0.5. The actual published Slint Core 1.18.1
calendar module is the sole Chrono consumer in the accepted root graph.
The calendar functions take civil Gregorian days, format or parse
DatePicker input, and read a local date. The maintained NevroTick GUI
contains no DatePicker or direct calendar call. The replacement still
preserves the widget defaults and existing civil date bounds.

The comparison used checksum-verified published archives of
`jiff 0.2.41` (upstream revision
`649fa73ffac9ed41023aa95ea0d81616b6978e3c`) and
`time 0.3.55` (upstream revision
`857d9c404c5b6f6cb64d1bfa604c695be3369e12`).
Jiff provides checked allocation-only civil parsing and formatting.
Its year limit is ±9999, and its Windows system-zone implementation
uses cached CLDR/IANA rules even where the active Windows rules
differ. The `time` candidate preserves the original year interval
with `large-dates` and uses the system's Windows local-time conversion
with an explicit error return. Its allocation-backed strftime
description parser and output writer have different costs from
Chrono's formatter. Whole-UI throughput, binary size, and Windows
local-date timing require independent measurements for any claim.
[Jiff Windows source](https://github.com/BurntSushi/jiff/blob/649fa73ffac9ed41023aa95ea0d81616b6978e3c/crates/jiff/src/tz/system/windows/mod.rs)
[time Windows source](https://github.com/time-rs/time/blob/857d9c404c5b6f6cb64d1bfa604c695be3369e12/time/src/sys/local_offset_at/windows.rs)

The RustSec Chrono finding is a maintenance notice. The known
`time` stack-exhaustion advisory was patched before 0.3.55. The selected
adapter bounds untrusted descriptions and input, uses a fallible
calendar library, rejects extra Gregorian fields that the library
might ignore, and propagates local-offset unavailability as the
existing unavailable-date sentinel. The adapter does not mutate the
environment or refresh time zones. On Unix the selected local-offset
path calls `localtime_r`. On Windows it validates OS success before
reading the output record and checks the returned offset. These are
selected implementation findings, with native local midnight, changed
timezone and DST behavior still needing native observations.
[Chrono notice](https://rustsec.org/advisories/RUSTSEC-2026-0543.html)
[time advisory](https://rustsec.org/advisories/RUSTSEC-2026-0009.html)

The selected derivative is desktop std only. It retains the default
DatePicker format and its one- or two-digit month and day plus
variable-width signed and unsigned year rules within a 1,024-byte
input cap. Other accepted input patterns must specify one Gregorian
year, month and day; arbitrary Chrono-specific directives and
alternate representations are outside this derivative's contract.
The original Slint package, original manifest, license grant and
unchanged GDB visualizer remain traceable to the archive.

The guarded OD-19 resolver reproduced the prospective desktop candidate
without an exception. It selected `time 0.3.55`, `deranged 0.5.9`,
`num-conv 0.2.2`, `num_threads 0.1.8`, `powerfmt 0.2.1`,
`time-core 0.1.9`, and `time-macros 0.2.32` in the lock. It removed
Chrono and both IANA zone packages, and substituted the local
Slint Core for the registry copy. Registry `time-core 0.1.9`
and first-party `time-core 0.0.0` retain separate identities.
The lock SHA-256 is
`044f4a7e97a97da0400299a9990c5395bfc2ec8290e0d2ba25baa175f1c61d52`.

The initial candidate failed against the real Chrono-backed module
on variable-width widget input. Its generic date parser also accepted
contradictory fields. The revised module
(`1099e71d220fa94f2ce06d52ab51feb365b8ac412088b7c91c87602d592d3552`)
passed five sandboxed GNU tests. The oracle compared 12,600 default
parse inputs, 840 calendar inputs, and 8,820 default rendering
inputs with the original module. Directed cases covered conflicting
and duplicate fields and the 1,024-byte boundary. The focused
harness substitutes `String` for Slint's `SharedString`. Its raw receipt
is `~/.local/share/nevrotick/calendar-controls-20261124.SuCaqtiG/differential_after-result.json`,
SHA-256 `1c49775f8835955e836ba78689ff241bc1a0247d8072e5a66d4522b98d899d55`.

The published archive reconstruction independently matched all 100
original file identities except the selected manifest and calendar
module. The derivative retains the original normalized manifest and
adds one patch notice. The existing GDB visualizer bytes match the
reviewed hash.

The bounded optimized GNU microbenchmark alternated the old and
new module calls in one process with the same `String` shim. Median
nanoseconds per call over seven 150,000-call batches were 132.9 → 29.0
for default padded input, 128.8 → 28.0 for unpadded input, 114.1 →
71.5 for a month header, 161.0 → 87.5 for the day summary, and 17.5 →
9.6 for month length plus weekday layout. These are isolated numbers
for the adapters and common widget patterns. GUI redraw timing, native
Windows timing and artifact size remain unmeasured. Raw output is in
`~/.local/share/nevrotick/calendar-controls-20261124.SuCaqtiG/calendar-benchmark-result.json`.

A separate three-way GNU benchmark used version-pinned Chrono, the
actual revised time adapter and a Jiff civil adapter with `std` only.
The Jiff arm does not include system-zone features. Measurements are
medians in nanoseconds per call over seven rotated 150,000-call samples.
The harness requires matching results for the timed inputs first.

| Calendar operation | Chrono 0.4.45 | time adapter | Jiff civil adapter |
| --- | ---: | ---: | ---: |
| Padded default input | 105.9 | 26.8 | 59.2 |
| Month header | 115.8 | 71.0 | 71.9 |
| Day caption | 160.2 | 88.0 | 93.6 |
| Month length and weekday | 16.8 | 9.2 | 3.3 |

The Jiff probe also verified its ±9999 bounds directly. The two
adapters are comparable on these selected common cases. They have
different parsing and local-date contracts. Raw output
`~/.local/share/nevrotick/calendar-controls-20261124.SuCaqtiG/jiff-compare-result.json`
hashes to
`e69afdcb7fc5fc53a8920e0035fd88af1cdd7e7f665582ee1888696755777a86`.

The calendar patch is integrated in
`third-party/i-slint-core` and the root patch and lock. The full
product source admission accepted 753 inputs. The source map
SHA-256 is
`18b7b0633cc67760f557610f6a867350929e044f32175fc831d521327f8c55f2`.
The active root lock contains no Chrono package. The published
Slint lock and manifests are retained as source provenance inside
the derivative. They do not govern the patched product graph.

Actual GNU and MSVC default and KE graph validation passed. The
first advisory replay also identified that Cargo-deny treated the two
unrelated `time-core` packages as duplicates. The reviewed projection
records both exact versioned identities with the existing per-version
mechanism. Active product package selectors explicitly address the
first-party `time-core@0.0.0`. Other versions and widened matches
retain rejection. Historical Core analysis receipts retain their own
package selectors and exact source bindings.

The 254 focused Python controls passed, including the exact-source
derivative refusals and duplicate-name control. Final Cargo-deny
0.20.2 on each of the four graph rows reported the existing URL
advisory alone and passed bans, licences and sources. The Chrono
advisory and the new duplicate-name refusal both cleared. The final
receipt is
`~/.local/share/nevrotick/jiff-graph-20261124.gTF6LF5c/calendar-final-cargo-deny-projections.json`,
SHA-256 `287ed64ef1b9821e98b3b81a7dee5dc93b34459cf16a5df557ad32f9d59add24`.
The same directory retains the frozen source bundle, official
resolution and fresh 2,159-file advisory metadata receipt. The
URL vulnerability gate remains open with its existing cooldown.
Root-lock product compilation and native UI qualification remain
open under their respective prerequisites.

### Reconcile the state-and-error handoff after the calendar change

The 23 November handoff reviewed `2868a53d97dd308015a0c5ff055bf815af06e439`.
This checkpoint is the working-tree candidate based on
`70586c24ba2ebad6d531db6be5e6fa669c72625d`, tree
`96ff3089ff1fc528278759894ec892838203bfd9`. Reuse the earlier source
and behavioral receipts within their original candidate scope.
The calendar derivative changes the UI dependency graph. It leaves the
selected Engine classifier, preview constructors, notification wrappers,
retry boundary, ledger and regression owner source unchanged from the
respective preceding handoff checks.

The calendar candidate passed the explicit runner's 47 strict fixtures,
12 pure API fixtures and eight environment smoke lanes under
Rust 1.100.0. The preview CLI returned no findings and all eight shell
routing tests passed. These replay the existing local CI controls on the
changed verifier. They do not establish hosted execution. The live
GitHub settings still report Actions enabled, `local_only`, and
required SHA pinning.
The receipts are in `/tmp/nevrotick-handoff-post-calendar.79iIYF9k/`.
The explicit runner receipt SHA-256 is
`377c89c44cb82714b6cfc498f65f987c340c3f8f09dffc25df3cc4f1e9c6f31c`.
The preview report SHA-256 is
`d2803866e52617597241a9c316136d0f3813ed6e5289535fc59781b14824abbf`.

| Handoff item | Disposition and owner |
| --- | --- |
| H01–H06 and AUDIT-BINDING-01 | Retain the implemented source protections and the scoped reader closure under their [Engine record](engine-transport-pilot.md#focused-hardening-verification--14-november-2026). Preserve their current consumers. |
| H07, H15, H16 | Keep the ledger, retry and regression triggers with the [Engine and Core owners](engine-transport-pilot.md#recheck-the-conditional-recovery-and-regression-triggers). The calendar change reaches none of those seams. |
| H08–H09 | Hosted approval and exact-candidate execution remain open. Existing default and KE selection and the new strict runner routing are locally exercised. The [owner decision](../docs/nevrotick-owner-decisions.md#64-hosted-actions-trust-policy--14-november-2026) retains its scope. |
| H10–H11 | H10 stays at the next notification-boundary edit. The [GUI record](slint-gui-handoff.md#private-apartment-trait-controls--h11-23-november-2026) has separate actual MSVC Send and Sync controls on its recorded candidate. Native notification cleanup retains its own exit. |
| H12 | Retain the existing three-role elapsed-clock restrictions and their scoped compiler controls. |
| H13–H14 | Independent acceptance comparison remains part of every affected admission edit. The [selected H14 structural check](#guard-the-preview-files-against-inner-outcome-lint-overrides) is verified. Wider suppression scope and semantic rationale remain with their reviewers. |

| Rule | Implemented coverage and remaining owner |
| --- | --- |
| CR-01 and CR-03 | The [preview pilot](#review-selected-preview-contracts-and-commit-seams) covers the named declaration, constructor, commit and nine receiving seams. The source gate and focused Rust receipts have separate scopes. The occurrence-specific exception is a structural attachment, with meaning reviewed at the actual caller. |
| CR-02 and CR-04 | Retain the Engine and numerical owners' existing typed inputs and checked outputs. The calendar change had its own conflicting-field and boundary controls. No resolved detector was selected for those rule families. |
| CR-05 | The [session classifier](engine-transport-pilot.md#require-an-explicit-session-coherence-decision--23-november-2026) has two actual-source exhaustiveness controls and legal neighbors. H11 carries the separate capability controls. Additional equality, trait and variant decisions remain with the changed owner. |
| CR-06 | GUI and static-policy owners retain H10's wrapper diagnostic obligation. A named-result detector needs its residual handling case after that compiler mechanism. |
| CR-07 and CR-08 | The preview pilot has a selected early-write control. Wider failure custody and current-effect binding remain with the actual UI, Engine, service and Windows owners. No interprocedural backend was selected for those guarantees. |

The standalone CNG and AES policies and the bounded preview pilot have
local verification. The full root-lock compiler and Clippy rows still
require dependency admission. The calendar replacement clears the
Chrono finding, and the separate URL advisory remains the configured
product refusal. There is no selected typed or interprocedural backend
with qualified current extraction. Existing native and post-V1 holds
retain their scope.

### Clear the URL advisory under the approved cooldown exception

David's exact OD-04 decision authorizes the two registry replacements
required by RUSTSEC-2026-0592. The ordinary precise-resolution control
refused `url 2.5.9` at three days old under the 14-day policy. The
approved precise Cargo 1.100.0 child then replaced only
`url 2.5.8 → 2.5.9` and `form_urlencoded 1.2.2 → 1.2.3`.
Independent review compared all 603 lock records and their edges,
the package archives and published checksums, and the accepted
manifest context. Both replacement archives identify upstream release
commit `ad1c66cbff6e318393d872641e30589ff9687b9c`. Both packages
retain their existing license choices and selected feature roles.
[Owner exception](../docs/nevrotick-owner-decisions.md#url-dependency-cooldown-exception--24-november-2026)
[Upstream release](https://github.com/servo/rust-url/releases/tag/v2.5.9)

The lock SHA-256 is
`d0cbe60cb93954940854e14cbf0b27f6d4edececcb06b79dd09377fbd5378515`.
The ordinary cooldown and deny settings retain their existing values.
The exact exception is retained as resolution provenance. Frozen
verification selected the exact admitted vendor and a fresh 2,159-file
advisory/index snapshot. Both configured dependency runs passed
all 30 lanes each, covering GNU/MSVC metadata, package trees, and
cargo-deny. All four advisory, bans, licences and sources results
passed. This clears the URL advisory and product dependency gate
recorded in the preceding handoff. Product execution and hosted
qualification retain their own evidence.

Both dependency receipts match all 753 admitted working-tree inputs.
Their canonical source map hashes to
`41d83f55a856314eb37ad9399b36046fd64a3bc32388481f9e83e28b906dc4dc`.
The 282 affected Python controls passed. The historical age tests
now use the separately pinned original registry inventory. The
current admission control verifies the live exception-backed graph
and its exact predecessor. This preserves the original refusal
controls after the calendar and URL graph changes. The graph-admission
suite passed again with 29 tests after adding exact exception-delta
and publication-age controls. Removing either tested guard in memory
made its intended control fail.

**Next ordinary resolution checkpoint — 27 November 2026.** The live-root
control reproduced a missing-checkpoint refusal at the lookup that the
guarded product resolver uses before Cargo. The registered
checkpoint binds the existing lock, the accepted registry digest and the
original `OD-04-URL-20261124` history. The old exception stays historical
under the ordinary 14-day deny policy. The current manifest and lock remain
the accepted inputs. The new live-root control detected the missing entry
and the final isolated selection passed all six root and child controls.
The first receipt and final log are retained under
`~/.local/share/nevrotick/url-cooldown-checkpoint.vaalLW2G/receipt.json`
(`9559836ca50ca2a3b4ece8d52946445314e2f9662d3657cff7bc60a5521508bb`) and
`~/.local/share/nevrotick/url-baseline-control-final.9aZoHa1p/tests.log`
(`2d7fc66d71ec581a11a7af125ead50be91d5dea88157e2e08c658a2439266176`).

Raw receipts are in
`~/.local/share/nevrotick/url-20261124.FRJBoTOA/`.

| Receipt | SHA-256 |
| --- | --- |
| `resolution/cooldown-resolution.json` | `ff31c57183702b4b8ebca4ca14f6354de65b1b7d98c0820030ea0c2bcb3707bb` |
| `advisories.json` | `801de52514db60f319419628ad6646f794f1bf9cec2c469479c98b42a791f8b7` |
| `default.json` | `9ef6d7351ed84d6b4b164969418151c9f435cb44ee67dcb3be98124df8c0ba72` |
| `engine-ke-association.json` | `363d67db90252bd553a5c6456cc634b60585cbb2c16f86c4c9c3b24e3c6c0ac0` |

The default portable follow-up used the same admitted source map.
Its first attempt stopped during the `rustversion` build-script link
after the linker reported unavailable resources. A retry limited host
CPU affinity to four CPUs with the same verifier and command limits.
Workspace compilation, Clippy, the Engine build and rustdoc passed.
The UI ran and passed 202 tests, including the generated Slint
callback and fixed repository URL controls. Wire, time-domain and
exchange package tests also passed. The later first-party
`time-core@0.0.0` test build ran out of tmpfs space before completing.
The full portable row remains incomplete with a separate storage
limit. This does not change the passing dependency gate.

The initial and retry receipts are `default-portable.json` and
`default-portable-affinity.json` in the same directory. Their
SHA-256 values are
`3bf9961034c18a8d7957cb1e9f235f53b5836e9d69dd9050db5da0e39172905b`
and `d103b650f9d71954436d8b031aead5f4c003d19bb46ae143f4b753342531b9e2`.

The next default run used fresh owned scratch on the Windows data volume
and retained the four-CPU affinity. All 45 configured GNU portable lanes
passed with Rust and Cargo 1.100.0, Clippy 0.1.100 and nextest 0.9.150.
The verifier accounted for 1,531 ordinary tests, 1,069 optimized tests and
384 doctests. UI ran 202 tests and Engine ran 311 in each selected test
profile. The previously stopped first-party `time-core@0.0.0` completed
624 tests in each selected profile and 112 doctests. The receipt retains
the same 753 source hashes as the passing dependency rows.
`default-portable-drvfs.json` hashes to
`1969ba999688581353c12981efc4344e18151756103e1d775251b8e9412d1eab`.

The default MSVC cross-build passed all 35 configured lanes, including
all-target compiler and Clippy checks, generated version-resource checks,
release linking, and static import readback for the UI and Observe.
It admitted the pinned MSVC toolchain, the 529-file CRT and SDK library
inventory, and 767 source inputs. The 753 shared inputs match the
portable and dependency receipts. The resource generation hashes to
`4c91da3ab7d67d9f7ad45a1e67c89530f299a68705d3e917d6c07311c607d740`.
The UI and Observe image hashes are
`5690460e4b6d6dc6985f39dc6b20709939fefae1f1ea2695dc26279791985fb6`
and `a60b8f9a575266c23cfbc7e533dd262162c79af211cb9f2f838c177c5cf7ac8f`.
The MSIX source inspection ran and returned `assembly-inputs-incomplete`.
The lane executed no Windows image. The `default-windows-drvfs.json`
receipt hashes to
`ea7bf0ade745c490fd589176a491b4b5fe62b84b051eae3349ef679bea11b5e1`.

The configured GNU KE run passed 42 lanes before the optimized Engine
unit-test build exited 101. The kernel journal recorded a hard CPU
watchdog during ThinLTO. A scoped retry retained the 90-second process
CPU limit, selected the same release command, and moved its scratch
to tmpfs with two physical CPUs. That attempt reproduced the compiler
SIGKILL and a second watchdog entry. Both stopped before optimized
Engine tests ran.

A separate scoped replay used the
[reviewed Engine recovery allowance](engine-transport-pilot.md#original-receive-brackets-and-final-cutoff--w11-w01)
of 240 CPU seconds for the exact optimized Engine command. It kept six
build jobs, ordinary release settings, the 300-second wall deadline
and the remaining sandbox limits. Independent review read the live
Rustc process arguments and limits. The two remaining Linux
`windows-platform-native` lanes used the original 90-second limit.
The replay passed all three test lanes, including 660 optimized Engine
cases, 69 adapter tests and four doctests. The Engine case names match
the ordinary Engine run exactly. Independent comparison matched the
753 input hashes, tools, effective configuration and selected argv.

Together the two passing selections cover all 45 GNU KE obligations,
with 1,880 ordinary tests, 1,418 optimized tests and 385 doctests.
The standard 90-second GNU KE verifier remains incomplete on this host.
The exact Engine behavior under the separate resource allowance belongs
to the scoped receipt below. Shared verifier limits retain their
existing owner.

| Retained GNU KE receipt | SHA-256 |
| --- | --- |
| `ke-portable-drvfs.json` | `25245bbf3c88dec5e1cfebaafcffb1fc1597249620c5466ecf58804f39963fed` |
| `ke-portable-tail.json` | `e6a84c8cf1b7f9e10e81d089905df38399ed900b7dfb2076e34ab961991a051a` |
| `ke-portable-tail-engine240.json` | `aecdbf777237fa97c608956b78449b0b3a948c5e42718035fa5c26543285b8c1` |
| `ke-portable-cpu-watchdog.log` | `cd5fafac3061ecce95c13f2605af68fe5b61a3bc657e559d3cf7f40c274eeaa9` |

The explicit KE MSVC cross-build also passed all 35 lanes. It reused
exactly the default MSVC source, tool, CRT/SDK and metadata generation
identities with the KE graph selection. The linked UI and Observe image
hashes are
`1cd8584951260df7045f2c1931817c0a23df5b1c15960caa3ede5127f28c5925`
and `e504214ea45e322392bac4f545df7de9b0bfade73c688a2805903e7001bd9995`.
The MSIX inspection retains `assembly-inputs-incomplete`. The
`ke-windows-drvfs.json` receipt hashes to
`e53c4c091909f49bdbacb18d221a3a0de2e6a57c4eaf1c49bf0274eef84856b7`.

### Bind preview exceptions to their reviewed seams

CHECK-25, CR-01 and CR-03. Independent readback found that a structurally
valid exception could name invented owner and consumer symbols. The
selected source-draft baseline receiver also had no report coverage.
Under the same HEAD and tree as the URL checkpoint, the preview report
now binds rule, context, occurrence, owner and related selected tokens.
Its exception requires the exact owner and a named related symbol from the
finding. The two model method bodies have their own first-line exception
slots. An interface exception attaches immediately before its own selected
declaration. The report retains both raw findings when a change reaches both
slots. Declaration, Garde, inner outcome lint and whole-model surface
findings still need a code fix or a reviewed baseline update as applicable.

The tenth seam, `SourceDrafts::from_baseline`, checks capacity before
reserving storage and derives each candidate through `DraftSource::parse`.
It rejects duplicate identities before retaining each entry. The Slint
startup path reaches it through `SourceDrafts::fixture`.
`RecoveryPreview::parse` forwards its refusal as `RecoveryError::Source`.
The independent reviewer traced both real callers. The source report selects
the common baseline constructor and the other nine existing seams. It names
the two callers for review of guarantees beyond that selected token chain.

Under Python 3.12.3, the old checker failed six new expectations on
actual-owner copies. The repaired checker passed all 18 preview controls.
The controls remove the actual baseline capacity guard, change the two
model bodies, try invented or unrelated owner and consumer identities,
and distinguish the two legal attachment slots. Stale, misplaced, duplicate
and broad exceptions are refused. A structurally valid rationale for
swapping host and label still passes attachment. The independent caller
review rejects that rationale. No production exception is present.
The first scratch run's two setup errors are retained separately.

The integrated Rust workflow suite command passed 76 Python tests, its
shell-routing tests passed all eight cases, and the source-only CLI
returned no findings. Skill-pack admission and all 25 checker controls
passed. Source report syntax, imports, effective cfg and unselected call
coverage retain their documented limits. The independent reviewer confirmed
that all 753 admitted input bytes still match both prior dependency receipts.
The selected Rust behavior, compiler and graph receipts retain their
individual source scopes.

| Input or receipt | SHA-256 |
| --- | --- |
| `scripts/preview_contract.py` | `a5f0ae567b2bf44d237d0e93c4863ee2ef2079a2a3841a2f9e0cfebdefa98774` |
| `scripts/tests/test_preview_contract.py` | `8546af66715cd44110d7b2e3bd9f570e1747d737611cf93b2e2dfe247da50a7d` |
| `preview-exact-baseline-red.log` | `badcb2214e147babb95e1fbe88aa5d3f18fd0e7ea9cf00d488225996b6e54ee0` |
| `preview-exact-integrated-tests.log` | `597341d1fdc7f16d43a5f9e77e5aa832813747591d93d8df2a996d55e8461b2f` |
| `preview-exact-integrated-report.json` | `d2803866e52617597241a9c316136d0f3813ed6e5289535fc59781b14824abbf` |
| `preview-exact-workflow-tests.log` | `cb9eff82ad433c153d837cbafbfb8dcdc1b4486e0e38347daa1cf39f5f6ba678` |

Receipts are in `~/.local/share/nevrotick/url-20261124.FRJBoTOA/`.
The exact tracked and untracked working-tree input inventory is
`handoff-final-worktree.sha256` in that directory. The handoff retains
H10 at the next native notification-boundary edit and the separate
GUI, hosted, native, metrics and post-V1 gates at their named owners.

### Repair the audit finding on inner lint groups

The audit of `4f3a1baeddcddb4b24fa59407852606f52ef0164` found that the
H14 source report missed inner overrides of `clippy::suspicious` and
`let_underscore`. Rust and Clippy 1.100.0 confirmed those groups can
suppress the selected future and drop diagnostics. The compiler probes
also found that `clippy::default` is the group behind `clippy::all`.
The deprecated bare spellings `clippy_all`, `clippy_suspicious` and
`clippy_restriction` still take effect when the renamed-lint diagnostic
is waived. The report now recognizes all six spellings.

The fixture adds attributes to a copy of the real `presentation.rs`
without changing a method. The new direct and nested `cfg_attr` cases
require a nonzero result and the exact
`preview-inner-outcome-suppression` finding. Restoring only the
published scanner roster reproduced nine failed expectations. The
repaired source passed all 76 focused Python tests and eight workflow
routing controls, and the repository preview report returned zero
findings. Healthy unrelated suppressions still pass. Raw logs are in
`~/.local/share/nevrotick/url-20261124.FRJBoTOA/audit-followup/`.
The scanner SHA-256 is
`279ba6599c395acacb3c4a709c1fad16e41adfd06f248909e1310ee215c35e96`.

The startup annotations on the audited commit and its parent establish
that `local_only` refused the same pinned external actions before any
job ran. The [owner decision](../docs/nevrotick-owner-decisions.md#64-hosted-actions-trust-policy--14-november-2026)
records the exact proposed allowlist and pending authorization. GitHub
CodeQL default setup reported its separate Rust success. The current
dependency graph checkpoint is needed before the next dependency
resolution under OD-19.

### Run calendar controls with the UI integration tests

The two Slint calendar controls previously lived in the excluded dependency
crate. The workspace UI now owns them in
[`slint_calendar.rs`](../crates/nevrotick-ui/tests/slint_calendar.rs),
reachable through the existing `nevrotick-ui` Nextest `--lib --tests`
selection without a new workflow or Cargo dependency. They call the real
patched adapter through Slint 1.18.1's generated-code exports. The cases cover
month lengths, leap centuries, weekdays, year limits, default formats, and
refusal of invalid and conflicting fields. The third test pairs
otherwise valid 1,024-byte input and format cases with 1,025-byte refusals.

The adapter now spells the split iterator and format description types
explicitly. The Slint inventory, the calendar file pin and the canonical
policy pin were refreshed. Independent comparison found only the two
expected hash changes in the policy JSON. The three dependency admission
suites passed all 92 controls after the canonical policy pin was updated.

The scoped GNU replay passed all 20 selected lanes under Rust 1.100.0,
including format, frozen graph admission and targeted Clippy with warnings
denied. The ordinary UI Nextest command ran all 205 tests. Independent
readback matched the 202 prior cases plus the three calendar cases and all
754 frozen inputs. The replay used default GNU features. Native date
behavior retains its separate Windows execution requirement.
The receipt is
`~/.local/share/nevrotick/calendar-audit-followup.8mg5JKHD/result.json`
with SHA-256
`84fe032236c21e3edbd0d4bede475aa71b54e9857e9f0a25a4836040aab0fa2a`.

The dependency-updates and preflight skills guided a fresh frozen replay
from `a17d499412c0b4d6ec61b96e47bbcff1bac5d507` with the existing worktree
overlays. The selected preview, strict-policy and graph suites passed 155
Python controls. Both configured `--lane dependencies` profiles, `default`
and `engine-ke-association`, passed all 30 lanes each. GNU and MSVC both
passed Cargo metadata and feature-tree admission, followed by cargo-deny
advisory, bans, licence and source checks. The verifier rechecked the
admitted advisory snapshot and its age. It used the existing lock and
vendor without fetching. Both receipts match the current 754 product and
checker inputs. The canonical source map SHA-256 is
`cbd1386df292416b40b43dacb36fa19df6347f881d440ab3704ea115b87ad7e2`.
Scratch was removed after both runs exited zero.

| Receipt under `~/.local/share/nevrotick/calendar-current-graphs-20261124.hAoE6BwZ/` | SHA-256 |
| --- | --- |
| `default.json` | `54d812a0baad0c4a6bc7cec843cb0eda58302e33b9d844879ee1c61fef643cf2` |
| `engine-ke-association.json` | `a9d2b83bc6c8065a26fc2242f3c417ea4ec65012edb212347c27de0367a1bbe8` |

Independent readback located the separate default MSVC build for the
committed `a17d499` calendar candidate. It passed 35 lanes including
compiler and Clippy checks, both release links and PE readback. The
`cross-build.json` under `%LOCALAPPDATA%/NevroTick/msix-default-a17d499/`
hashes to `60a573d0cef8e99378611af70dd1be85f394b1fe3c914130c8bc5226cda6a666`.
The Windows build was reused for that candidate. Native local-date
behavior and installed-package qualification remain open.

### Close alternate outcome lint spellings

Audit `R-A17-01` reproduced 16 missed findings against the
`a17d499` scanner by adding inner lint attributes to copies of
`presentation.rs`. The checker now derives the bare spellings from its
protected Clippy names and recognizes `clippy::let_underscore_drop` as
the moved Rust lint. Findings retain the spelling used in the source.

Metadata-only Clippy 1.100.0 probes produced each of the eight expected
denials, then confirmed that the direct and nested overrides each change
the intended level when the renamed-lint and unknown-lint diagnostics
are allowed. All 16 scanner regressions now return 1 with the exact
`preview-inner-outcome-suppression` finding. The unrelated lint, other
tool namespace, quoted reason, documentation and comment controls pass.

The 77 selected Python tests and eight workflow routing controls passed.
The preview CLI returned zero findings on the repository sources. Evidence
is in `~/.local/share/nevrotick/a17-lint-alias-followup/`. The scanner
SHA-256 is `7ca2e6e95b0e08c5f68b795fca2542047565ae105564ce54d8239f46ad61b120`.

### Correct Sunday-based weekday formatting in Slint

Audit `SEC-CAL-01` reproduced Sunday `"1"` instead of `"0"` through
the real Slint export in the default GNU UI Nextest lane. The `a17d499`
snapshot with only the new tests ran 207 cases with exactly the new
`%w` test failing. Its `%u` and literal `%%w` controls passed.

The adapter now sets zero indexing on the parsed Sunday weekday
component before formatting. The two UI tests check all seven weekday
values, ISO numbering, literal `%%w`, and a mixed date and weekday format.
The first fixed build stopped during an unrelated UI binary link with
`Resource temporarily unavailable`. The four-CPU-affinity retry passed
all 20 selected default GNU lanes on Rust 1.100.0, including all 207 UI
tests and targeted Clippy. The two calendar files in the integrated checkout
match the tested bytes. The combined source-policy admission covered 754
inputs and the three dependency admission suites passed 92 tests.

Receipts are in `~/.local/share/nevrotick/calendar-weekday-audit.ECSEzGvc/`.
The failing `red.json` hashes to
`d4dd22df7014be69ca5589db9472c057a2ee7d3f630e332636e80b441517f880`.
The passing `green-affinity.json` hashes to
`409a9f35e83522ce5f8825d045a20f6df325bc2c0b2e4bb69ab353961ad5b19b`.
That receipt binds the isolated audit snapshot. The separately admitted
integrated policy includes existing rustls-platform-verifier edits and
uses canonical SHA-256
`47204cbff0a0790b36ae69c12d1c573c4f8f768df454290fc2ac5b1ccf168ed3`.

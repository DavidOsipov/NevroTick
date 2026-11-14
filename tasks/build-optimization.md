# Windows x64 application build optimization

Initial record: 29 October 2026; its executed checks below are historical.
**Current checkpoint, 6 November:** required v3 and the expanded hardening vector
are configured and admission controls are updated; final-image, startup and
native qualification remain open. See the [current amendment](#required-v3-and-maximum-compatible-hardening--6-november-2026).

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

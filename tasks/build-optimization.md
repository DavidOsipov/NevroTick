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

**Verification boundary:** local source/diff and configuration consistency checks
are possible, but local target-code execution is unavailable: bubblewrap cannot
create the required network namespace and unshare cannot establish its UID map;
Rust/Cargo are also absent from the inspected PATH. No local RED/GREEN, native
execution, PE observation or benchmark is claimed. The policy test was prepared
before its repair and its expected discrimination inspected in source; that is
not an executed failing test. Exact automated results belong to the associated
PR's checks and review record. Required failed/missing checks cannot be reported
as passes.

This change neither refreshes old proof receipts nor runs formal work. OD-10's
post-V1 deferral, the native/S134 stop, separate fuzz/storage holds and UI-M0
scope remain. Source preparation is authorized now; final release still needs
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

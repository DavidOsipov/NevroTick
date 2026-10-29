# Performance measurement and profiling

Use with [the optimization workflow](../SKILL.md) and
[resource QA](../../../testing/qa-performance-resources/SKILL.md). The
[architecture](../../../../../docs/windows-time-architecture.md#17-resource-and-performance-targets)
owns budgets, clock domains, runtime limits and milestone applicability. This
reference selects evidence; it admits no new tools or dependencies.

## Build optimization and Windows x64

An explicit build-configuration request may select settings before comparative
measurements exist. Record the exact adopted candidate and its pending evidence;
configuration, successful linking and improved runtime performance are separate
results. [Hardening §6](../../../../../docs/windows-time-development-hardening.md#6-build-profiles-and-windows-mitigations)
owns the selected profile and Windows protections. Inspect current manifest,
policy, runner and effective flags rather than relying on an earlier receipt.

Cargo `opt-level`, LTO and codegen units tune application code generation; their
names promise no measured speed or size improvement. Preserve overflow checks,
the selected panic strategy, debugging symbols, validation and native mitigations.
Compare optimization levels or Thin/fat LTO as bounded experiments when a measured
question warrants it; do not silently change the selected profile or fault policy.
Cargo Rust LTO does not establish cross-language LTO for C/C++ dependencies,
prebuilt native objects or system DLLs. That needs separately admitted compatible
compiler/linker/bitcode inputs and final-artifact qualification. See
[Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html#lto).

The [rustc optimized-build guide](https://rustc-dev-guide.rust-lang.org/building/optimized-build.html)
describes building the compiler itself. Its Windows warning links
[issue #109114](https://github.com/rust-lang/rust/issues/109114), whose reproducers
involve nightly `-Zdylib-lto`, ThinLTO, dynamic Rust linking and imported data on
Windows MSVC. This is a concrete configuration risk, not evidence that every
stable Windows application LTO build miscompiles or that this candidate is safe.
Do not import compiler `bootstrap.toml`, `opt-dist`, allocator or nightly flags
into the application. Qualify its actual toolchain, linker, graph and artifacts.

That reproducer distinction is not immunity from Windows import bugs. Broader
`dllimport` failures have affected native global imports and ordinary Rust
ThinLTO applications. Read [hardening §6.5's qualification addendum](../../../../../docs/windows-time-development-hardening.md#65-windows-x64-optimization-policy-and-evidence-limits)
for #27438, later fixes and the pinned regression test. Keep ThinLTO a candidate
until relevant native correctness and useful-work comparisons pass; neither a
clean cross-link nor switching LTO/linkers alone establishes runtime correctness.

Keep distributed Windows x64 builds at the explicitly admitted `x86-64` CPU
baseline. `target-cpu=native` depends on the build machine; it is unsuitable for
distributed artifacts or portable cross-build qualification. An explicit higher
CPU/feature floor needs a compatibility decision, declared supported hardware and
native positive/unsupported-machine behavior before adoption. No incidental AVX,
AVX2 or x86-64-v2/v3 requirement follows from enabling optimization. Review native
dependency/compiler flags as well as Rust flags; one baseline flag does not prove
the complete package's instruction-set requirements. See
[rustc target-cpu](https://doc.rust-lang.org/rustc/codegen-options/index.html#target-cpu).

### LLVM policy and native screening

[Hardening §6.6](../../../../../docs/windows-time-development-hardening.md#66-llvm-settings-for-this-workload)
owns the selected LLVM policy. Start with the existing O3/ThinLTO/one-CGU
candidate and no extra `-Cllvm-args` or `-Cpasses` list. Preserve default inlining,
vectorization, unrolling/interleaving and target cost models. Rust 1.99.1 already
enables loop vectorization at O2/O3 and SLP at O3, then supplies those choices to
LLVM's ordinary pass builder. Do not force vector widths, large unroll/inline
thresholds or extra reroll passes from generic tuning recipes. See the pinned
[Rust module configuration](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_codegen_ssa/src/back/write.rs#L229-L237),
[LLVM wrapper](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_llvm/llvm-wrapper/PassWrapper.cpp)
and [LLVM vectorizer cost-model guidance](https://llvm.org/docs/Vectorizers.html).

Preserve ordinary Rust floating-point semantics and explicit finite/range checks.
Do not add blanket `fast`, `reassoc`, `nnan`, `ninf`, `nsz`, `arcp` or `afn`
assumptions, approximate division/math, or FTZ/DAZ flushing to win a benchmark.
Do not change thread/process floating-point rounding state incidentally.
Compiler-proven transformations that preserve the existing semantics are not
equivalent to imposing those assumptions on arbitrary inputs. Keep exact integer
time arithmetic exact; future estimator changes need their own analytic cases,
tolerances and rejection controls. [LLVM fast-math semantics](https://llvm.org/docs/LangRef.html#fast-math-flags)
explain the relaxed guarantees.

Do not globally disable FMA or describe the default as LLVM `strictfp`.
LLVM 23.1.1 defaults to Standard contraction, not unrestricted fusion; explicit
FMA operations remain honored. A deliberate Rust `mul_add` changes intermediate
rounding and must be a reviewed numerical-algorithm decision with independent
tests, not a build-wide shortcut or assumed speedup. See
[pinned LLVM contraction defaults](https://github.com/rust-lang/llvm-project/blob/1b9c0d5ff9bbe7634aead059efe6b11a7eeba145/llvm/lib/CodeGen/CommandFlags.cpp#L283-L291),
[explicit-FMA semantics](https://github.com/rust-lang/llvm-project/blob/1b9c0d5ff9bbe7634aead059efe6b11a7eeba145/llvm/include/llvm/Target/TargetOptions.h#L365-L381)
and [Rust `mul_add`](https://doc.rust-lang.org/std/primitive.f64.html#method.mul_add).

Keep Windows target defaults for code/relocation models, ABI, stack, exceptions
and CPU tuning; preserve the admitted ISA floor and mitigations. Raw LLVM args
and extra passes lack rustc's normal CLI stability guarantee. In Rust 1.99.1,
`-Cinline-threshold` is removed and `tune-cpu` is an unstable `-Z` option.
Recheck the pinned compiler/help before proposing a dependent experiment; these
are not stable shipping knobs. See
[rustc controls](https://doc.rust-lang.org/rustc/codegen-options/index.html#llvm-args),
[removed inline-threshold](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_session/src/options.rs#L2242-L2245)
and [unstable tune-CPU](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_session/src/options.rs#L2922-L2923).

When a native comparison is authorized and useful work exists, screen this finite
matrix rather than searching arbitrary LLVM switches. Each experimental row
changes one factor relative to the selected row:

| Candidate | `opt-level` | `lto` | `codegen-units` | Comparison question |
| --- | --- | --- | --- | --- |
| Selected baseline | `3` | `"thin"` | `1` | Establish the actual current useful-work result. |
| Optimization level | `2` | `"thin"` | `1` | Does the less aggressive pipeline improve the declared workload/cost? |
| Fat LTO | `3` | `"fat"` | `1` | Does extra whole-program work justify build, size and runtime costs? |
| No LTO | `3` | `"off"` | `1` | Does cross-crate LTO improve this component at all? |
| Codegen parallelism | `3` | `"thin"` | `16` | Does the compile/runtime trade-off favor more codegen units? |

Cargo `lto=false` is not generally identical to `lto="off"`: it permits local
ThinLTO across codegen units, although none is performed with one unit. Use the
explicit table value so the comparison remains clear. See
[Cargo LTO semantics](https://doc.rust-lang.org/cargo/reference/profiles.html#lto).
Prepare each row in a fresh task-owned clone with its exact manifest/profile,
matching temporary experiment policy and runner admission before execution.
Keep the selected production policy unchanged; no environment override or
unreviewed profile is a backdoor around the guarded runner. Freeze source,
features, tools, protections, workload, CPU/power conditions and all other
relevant settings. Retain each binary, matching symbols and effective command
identity; reuse no stale receipt under a new profile label.

Independent numerical/admission correctness and preserved authority are gates.
For qualifying rows, prioritize idle CPU/wakeups, useful cold startup and
end-to-end latency with enough observations for p95, then steady/peak memory and
power under the declared workload. Include bounded hostile/rejection and
recovery paths; a lower workload or reject-all implementation is no improvement.
Measure build time and artifact size as trade-offs, not substitutes. Throughput
alone proves no idle/power benefit; scheduling/timer behavior still needs native
observation. Use only the implemented component's useful workload: UI-M0 or a
header loop cannot train or qualify a future production engine.

### Bounded compiler diagnostics and source privacy

Prefer stable `-Cremark` for selected optimization remarks, selected
`--emit=llvm-ir,asm` outputs and optional `-Csave-temps` to investigate a concrete
transformation. Confirm support with the pinned compiler, select the owning
crate/path and admit a separate diagnostic command/output lane. Bound duration,
log/artifact volume and retention; do not enable whole-workspace dumps by default
or persist diagnostic switches as production tuning. A missing remark proves no
unreachable optimization or code path. Preserve the full hardening flag vector
and matching source/artifact identity. See
[pinned diagnostic options](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_session/src/options.rs#L2313-L2318)
and [rustc emission options](https://doc.rust-lang.org/rustc/command-line-arguments.html#--emit-specifies-the-types-of-output-files-to-generate).

IR, assembly, remarks and saved intermediates can expose source, constants,
paths and build arguments. Use synthetic inputs and restricted owned storage;
retain matching private symbols for diagnosis. `--remap-path-prefix` is only
best-effort textual path rewriting: Windows linkers may independently embed
absolute paths and arguments into PDB files. Inspect actual EXE/PDB/intermediate
outputs before any publication or path-privacy claim; a remap flag proves no
complete sanitization. See [rustc remapping caveats](https://doc.rust-lang.org/rustc/remap-source-paths.html#paths-generated-by-linkers).

### Conditional native PGO

PGO is a separate admitted workflow, not a profile switch or an installed tool:

1. Freeze source/dirty inputs, lock/features, CPU/target, compiler/LLVM tools,
   linker/SDK, effective flags, matching symbols and the training-corpus identity.
   Admit a bounded instrumented build through the isolated runner, with fresh
   owned profile/output directories and explicit target selection.
2. Run that exact Windows executable under its intended token on representative
   native hardware. Select useful startup/steady work, accepted/rejected input,
   source churn and recovery as applicable to the implemented component. Include
   guard metrics and workload weights; synthetic loops alone cannot qualify a
   production workload. UI-M0 training remains fixture-only; future clock/service
   work retains disposable-fixture/restoration requirements. Bound runs, retries,
   resources and raw-profile storage; install no tool or run public-server stress
   as an incidental step.
3. Preserve `.profraw` identities, run outcomes, tool versions and corpus mapping;
   merge only matching owned training data with the admitted compatible
   `llvm-profdata`. Hash the merged `.profdata`, retain its provenance and reject
   empty, unrelated or stale profiles. Use synthetic inputs and restricted storage;
   profile artifacts remain build inputs and need custody through consumption.
4. Build the profile-use candidate with matching relevant inputs/flags, changing
   only the recorded instrumentation/use settings. Keep absolute profile paths
   and target selection explicit. Inspect missing/mismatched-function diagnostics
   and intended workload coverage; a successful merge alone proves no useful
   training. Never ship the training executable as the optimized candidate.
5. Re-run affected correctness/fault checks and inspect final PE protections,
   imports, symbols and hashes. Compare representative uninstrumented native
   baseline/candidate runs before claiming a gain. Keep training, debugger,
   AppVerifier and performance results distinct; package/signing evidence must
   identify the final rebuilt artifact.

The [rustc PGO examples](https://doc.rust-lang.org/rustc/profile-guided-optimization.html)
use `RUSTFLAGS`; Cargo flag sources can replace configured target protections.
Do not paste those examples into this checkout as qualification commands. A
reviewed runner must preserve the complete admitted CPU/mitigation flag set in
both phases and keep target instrumentation away from host build scripts/proc
macros. Missing runner/tool/profile admission remains incomplete; there is no
ad-hoc host-Cargo fallback.

[LLVM BOLT's documented inputs](https://github.com/llvm/llvm-project/blob/main/bolt/README.md#input-binary-requirements)
are x86-64/AArch64 ELF binaries. This supplies no supported Windows PE lane; do
not add BOLT to this product's build/signing workflow from compiler examples.

## Define a comparison that can answer the question

Record baseline/candidate source including dirty content, binary and matching
symbols, lockfile/features, target, compiler/linker and effective build flags.
Match workload/corpus, seeds, relevant input sizes, process identity, OS/hardware,
power/thermal conditions, Windows security settings and background activity.
Do not compare debug with optimized builds and attribute the whole difference
to a code edit. Profile/feature changes are themselves the experimental factor;
keep other relevant inputs fixed and record both configurations.

Define useful work and start/end events before sampling. Separate initialization,
cold cache, warm cache, steady tracking, invalid-input rejection and recovery.
For latency/queue tests, specify offered arrival rate, concurrency and completed,
rejected, timed-out and dropped work; a faster reject-all or silently reduced load
is a failed comparison. Include relevant cache misses, source churn and bounded
hostile traffic so a common-case gain cannot hide unbounded worst-case work.

Choose the primary metric and regression constraints before seeing results.
Use bounded repeated runs, warm-up where appropriate and alternating baseline/
candidate order when practical. Retain all raw observations and reasons for
failed/missing samples. Report sample count, units, distribution/spread and the
preselected statistic; never select only the fastest trial or discard a bad
tail without explanation. A p95 needs enough observations to be informative;
do not invent confidence from a handful of trials or correlated loop iterations.

For a lower-is-better metric, report both the absolute difference and, when the
baseline is positive, `(baseline - candidate) / baseline * 100%` as reduction.
Distinguish reduction from speedup/throughput. Use a justified uncertainty method
if giving intervals; a difference near measurement noise remains inconclusive.
No universal percentage improvement or fixed trial count replaces the workload's
declared decision rule. Report whether the canonical budget is actually met
separately from whether a local change improved a metric.

Use an interval clock appropriate to the requirement, never wall-clock UTC for
elapsed benchmarking in a clock-changing system. Windows QPC is independent of
system-time adjustments but includes suspended time; it is not the active-awake
clock. Record suspend/resume, clock adapter, resolution and timer overhead, and
keep very small durations above measurement noise. Do not substitute direct TSC
reads or change global timer resolution. See Microsoft's
[high-resolution timestamps](https://learn.microsoft.com/en-us/windows/win32/sysinfo/acquiring-high-resolution-time-stamps).

## Select an observation, then verify the real workload

| Question | Smallest useful evidence and limits |
| --- | --- |
| Where is CPU spent? | Sample the owning process with matching symbols; separate running stacks from waiting. A sample share is attribution within the capture, not the promised saving from removing a function. |
| Why is a request/UI action slow? | Measure its full admission-to-completion interval and queue/blocking stages. Include tail latency, cancellation and busy/rejection outcomes. Pure-function speed alone does not explain end-to-end delay. |
| Why does memory grow? | Distinguish allocation rate, peak/retained private allocations and allocator caching; count owning handles/tasks/queues and release events before teardown. Use the shared native-memory workflow for corruption evidence. |
| Why are idle cost or wakeups high? | Observe timer/event/job sources and service ownership over the declared window. Record idle and active work separately; throughput tests cannot establish idle efficiency. |
| Why are startup or size high? | Observe cold usable-state events and the emitted binary/MSI plus required payloads. Inspect initialization and actual feature/dependency contributions; neither stripped symbols nor omitted required runtime payloads prove a valid product saving. |

Tool selection is conditional:

- **Existing native counters or scoped in-process timing** may already answer
  the question. Define what they count and corroborate the owning process/work.
- **Windows Performance Recorder/Analyzer (WPR/WPA)** are optional native
  profiling candidates. Inspect installation, version, available profiles,
  privilege/session requirements and symbol support before use. Installing SDK
  Debugging Tools/AppVerifier does not establish that this profiling lane is
  installed or admitted. Select only needed providers/stacks, duration, trace
  storage and an owned session; do not cancel unrelated recordings or run an
  ordinary candidate elevated merely to collect a trace.
- **A focused Rust benchmark harness** can isolate a measured pure hot path.
  Reuse admitted infrastructure before proposing a crate. Inspect the actual
  Cargo benchmark target, effective profile and selected features. Cargo's
  `bench` profile inherits `release`, but configuration/environment overrides
  affect effective settings; keep required overflow checks, panic policy and
  mitigations. Higher optimization levels need measurement rather than an
  assumption of speed. See [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html).
- Use runtime-varying representative inputs and observe meaningful output so
  the compiler cannot simply remove useful work. Rust's
  [`black_box`](https://doc.rust-lang.org/std/hint/fn.black_box.html) is a
  best-effort benchmark aid, not a guarantee against optimization and not a
  constant-time or security primitive. Validate output independently and inspect
  the timed region; include allocation/setup when it belongs to the claimed cost.

WPR memory recording is circular and can overwrite earlier events; file recording
can grow until storage is exhausted. Bound capture duration/storage externally,
choose buffer/detail settings deliberately, and inspect dropped events and
scenario coverage before interpreting absence. Trace files may expose paths or
sensitive payloads; use synthetic inputs, restricted local storage and the
existing redaction policy. Stop/export only owned sessions and verify cleanup.
See [WPR logging modes](https://learn.microsoft.com/en-us/windows-hardware/test/wpt/logging-mode).

WPA's comparison view can compare baseline and candidate traces, but selected
intervals must represent the same scenario. It does not establish causality or
statistical significance. See
[WPA comparative analysis](https://learn.microsoft.com/en-us/windows-hardware/test/wpt/comparative-analysis-views).

Use profiled/instrumented runs to locate a cause, then confirm the proposed
benefit with comparable representative runs without heavy instrumentation.
AppVerifier/PageHeap, Miri, sanitizer, coverage and debugger timings are not
shipping performance measurements. Preserve symbols and raw evidence, plus
differences between instrumented and measurement artifacts. Host/Linux results
support only their measured scope; Windows runtime/UI/service claims need
the corresponding native observations. Local improvements leave applicable
CHECK-21 and package qualification outstanding.

# Select sanitizers and memory checks

Use for a concrete address, initialization, aliasing, race or leak question.
Read the [testing contract](../CONTRACT.md); use the existing
[configured analysis](configured-tooling.md#additional-rust-analysis) and
[Windows memory workflow](windows-memory.md). Existing Miri, fuzzing, Loom,
Kani and Lockbud guidance remains applicable. Select the smallest useful set;
this reference creates no new runner lane, installation or per-edit campaign.

## Choose by defect and environment

| Tool | Useful question | Admission and evidence limits |
| --- | --- | --- |
| ASan | Does reached code access memory out of bounds, after free or outside a valid lifetime? | Keep the admitted portable fuzz profile. Instrument the selected code and identify native/allocator gaps; an uninstrumented DLL is not automatically covered. |
| LSan | Are unreachable allocations leaked on the exercised lifecycle? | Linux ASan normally includes leak detection; verify the emitted runtime, options and a real leak control. Reachable caches/retained owners and Windows handles need separate measurements. |
| Miri | Does portable Rust violate initialization, alignment, validity, provenance or aliasing rules? | Reuse the admitted Miri toolchain/sysroot and focused tests. It complements ASan; unsupported Win32/FFI calls and unexplored executions remain outside its evidence. |
| TSan | Does an exercised concurrent execution contain a data race? | Qualify a supported target and visibility of relevant code/synchronization. Rust documents gaps for atomic fences and assembly synchronization. Keep Loom for bounded modeled schedules and Lockbud for compatible static screening; TSan does not prove higher-level protocol correctness. |
| MSan | Is uninitialized memory consumed on a reached path? | Qualify instrumentation across the standard library and native/dependency graph. Missing instrumentation can invalidate reports; add this lane for a named initialization gap rather than treating it as the next default. |
| UBSan | Does compiled native C/C++ code violate selected language rules? | Instrument the actual C/C++ source and link a compatible runtime. Rust has no documented standard UBSan sanitizer mode; linking the runtime adds no missing checks to Rust code. Preserve Rust-specific Miri and checked-arithmetic tests. |
| HWASan | Does a future Android ARM64 native path contain address/lifetime errors? | Qualify the exact Rust/NDK/runtime/ABI/device combination when Android work is active. HWASan and hardware MTE/memtag have different requirements; neither is a current Windows gate. |
| Application Verifier / PageHeap | Does the actual Windows process misuse observed heaps, handles or native APIs? | Follow the existing per-image checks, intended token, allocator/API coverage, debugger capture and restoration. Measure allocation, handle and GUI-object retention separately. |

## Qualify the selected mechanism

Keep three independent claims distinct:

1. **Input delivery:** a valid neighbor reaches the actual target and succeeds.
2. **Oracle and propagation:** a controlled semantic fault reaches its intended
   assertion and the runner preserves the failure.
3. **Detector operation:** an owned test-only fault representative of the selected
   mechanism produces its expected diagnostic at the intended site. For ASan,
   use a genuine address/lifetime error; for LSan, a genuinely unreachable leak;
   for TSan/MSan, a reached race/uninitialized read. Pair with a repaired healthy
   control. Keep deliberate UB outside production and pure `forbid(unsafe_code)`
   owners; use an isolated fixture where necessary.

A panic, changed numerical result, nonzero exit or `libFuzzer: deadly signal`
alone establishes no ASan memory-error detection. Require the selected tool's
diagnosis and relevant stack/site, not merely a matching exit code or any crash.
Reuse detector controls while their compiler/runtime/target/instrumentation
assumptions remain unchanged; do not inject unsafe examples into every fuzz
target or repeat the full campaign to restate the same mechanism evidence.
Missing detector evidence limits that claim; retain legitimate semantic and
fuzz results with the gap stated explicitly.

Record raw fuzzer callbacks separately from accepted inputs, constructed domain
values, reached operations and distinct cases. Rejected inputs still count as
fuzzer executions. A bounded run is no coverage percentage or whole-system proof.
Replay retained counterexamples at the real entry point before claiming a fix.

## Preserve the build and platform boundary

- Record the candidate, compiler/LLVM/runtime, target, features, effective flags,
  instrumented modules/standard library/native dependencies, runtime options,
  controls, native outcomes and remaining gaps. Keep separate owned build/output
  directories and finite work/resource budgets. Diagnostic builds have different
  CPU/memory behavior from the production profile.
- ASan, TSan and MSan need separate configurations; they cannot be combined in
  one Clang program. LSan can accompany ASan where supported. New configurations
  require their own admission. The separately configured
  [standalone detector checker](configured-tooling.md#additional-rust-analysis)
  has fixed control profiles; this adds no names to `check-analysis.py` and
  supplies no application sanitizer lane merely by describing one.
- Use the explicit target so sanitizer flags apply to target code, not host
  build scripts/procedural macros. Inspect standard-library instrumentation;
  rebuilding/instrumenting `std` is particularly important for MSan/TSan.
  Mixed Rust/C/C++ builds need compatible compiler runtimes. An option such as
  `-Zexternal-clangrt` is a qualified integration choice, not a universal recipe.
- Keep separate analysis compiler pins and the shipping compiler/profile.
  Preserve the guarded runner's complete flag vector and mitigation policy;
  ad-hoc `RUSTFLAGS` can replace configured protections. The existing ASan
  address-space exception belongs only to its admitted runner profile; it is
  not permission to remove other limits or reuse that profile for every tool.
- **Windows ASan:** upstream support documents differ. The Rust book omits
  Windows from its ASan list, while an inspected compiler target admits ADDRESS
  for `x86_64-pc-windows-msvc` and the Rust Fuzz Book documents MSVC setup.
  Check the actual pinned compiler, runtime and native controls; do not declare
  Windows support either qualified or impossible from one list. Linux/WSL ASan
  cannot replace Windows heap/handle/FFI observations.
- Inspect effective ASan/LSan options and the invocation path. A `cargo-fuzz run`
  convenience default need not apply when the runner executes a built binary
  directly. A clean LSan exit does not establish bounded retained allocations,
  task/permit release or Windows resource cleanup.

## Sources

Checked **31 October 2026**; research is not qualification of a repository lane.
The [source register](../SOURCES.md#sanitizer-selection-and-controls--31-october-2026)
retains the primary Rust, LLVM, Android and Windows sources and the immutable
Windows target snapshot. Verify rolling guidance against the selected compiler.

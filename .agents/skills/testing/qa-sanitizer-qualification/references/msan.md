# Qualify MemorySanitizer evidence

Load for an initialization-sensitive path selected by the
[coordinator](../SKILL.md). The [shared sanitizer rules](../../references/sanitizers.md)
own candidate admission, mechanism controls, bounds and evidence reuse.

## Establish an interpretable instrumentation graph

- Trace memory initialization from allocation or native output through the
  consuming operation. Include partial writes, error returns, buffer lengths,
  representation changes and reused storage where present. Safe handwritten
  Rust does not establish initialization coverage for unsafe/native dependencies.
- Qualify the selected supported target, matching instrumented Rust standard
  library and native/dependency graph. Classic nightly `build-std` and the
  Rust 1.100.0 `x86_64-unknown-linux-gnumsan` target are alternative preparation
  routes; neither silently updates the admitted toolchain or product profile.
- MSan needs instrumentation across program code. Its libc interceptors bridge
  particular external calls; they do not cover arbitrary uninstrumented native
  libraries. Inventory actual interceptors, custom allocators and memory-writing
  assembly. An opaque initialization boundary leaves an explicit coverage or
  interpretation gap, even with instrumented `std`.
- Record origin-tracking configuration, effective `MSAN_OPTIONS` and compatible
  compiler runtimes. Cargo-fuzz 0.13.2 requests `build-std` for its memory mode
  outside coverage mode and adds origin tracking; verify the actual admitted
  invocation. Avoid copying those assumptions to another launcher or release.

## Qualify the detector and consuming paths

Use an isolated uninitialized-value control that reaches the intended consumer
and produces MSan's expected diagnosis/site, paired with a fully initialized
healthy neighbor. An arbitrary crash or a changed numerical result is not MSan
evidence. The existing standalone `core`/`compiler_builtins` controls establish
no full-standard-library or application/native-graph qualification.

Select product cases that discriminate complete initialization from partial
success and failure paths. Retain the reported consuming stack and available
origin stack/store chain. Mere allocation or a parser rejection before the
initialization-sensitive operation does not show that operation was checked.
Record accepted inputs and reached consumers separately from raw executions.

## Diagnose coverage and setup failures

Uninstrumented external writes can produce misleading shadow-state reports.
Investigate their causal path rather than assuming every report is a product
defect or dismissing it as a false positive. Blanket unpoisoning, unconditional
zeroing added only to the test build, or broad suppressions cannot qualify the
unchanged production path. Retain any reviewed boundary treatment and excluded
scope explicitly.

Shadow-memory/startup, static-linking, symbolization and resource failures are
incomplete setup evidence. Keep MSan's separately admitted address-space and
resource profile. For a selected supported profile, if the required graph cannot
be instrumented or soundly observed, report the applicable row blocked/incomplete
and retain complementary
Miri, checked-construction and native-boundary evidence under their own limits.
Use the coordinator's separate target-applicability decision for an upstream
unsupported native mechanism; the initialization obligation still needs evidence.

Primary sources and checked revisions: [28 November research amendment](../../SOURCES.md#sanitizer-qualification-manuals--28-november-2026).

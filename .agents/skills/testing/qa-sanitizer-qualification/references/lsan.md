# Qualify LeakSanitizer evidence

Load for an applicable allocation-lifecycle row selected by the
[coordinator](../SKILL.md). The [shared sanitizer rules](../../references/sanitizers.md)
own candidate admission, controls, execution bounds and evidence reuse.

## Establish what the leak check observes

- Choose a qualified standalone LSan profile or supported ASan/LSan combination.
  Treat them as distinct mechanisms even when one execution supplies both.
  Windows ASan support does not establish native Windows LSan support; portable
  Linux results do not observe the Windows process's allocations or handles.
  Apply the coordinator's evidenced target-applicability decision and retain
  required native ownership/resource evidence when that mechanism is unsupported.
- Identify the actual allocator/interceptors, native allocation paths, process
  and thread lifetimes, retained roots and shutdown/checkpoint. LSan searches
  for unreachable allocations. Reachable caches, retained owners, pending tasks,
  permits, handles and GUI objects need their separate lifecycle measurements
  through [Windows memory diagnostics](../../references/windows-memory.md).
- Inspect effective `ASAN_OPTIONS` and `LSAN_OPTIONS`, embedded defaults,
  suppressions and programmatic exclusions in the admitted graph. LSan-specific
  options still use `LSAN_OPTIONS` when integrated with ASan. Record whether the
  actual runner invokes cargo-fuzz or a built binary directly; wrapper defaults
  cannot be inferred for another launch path.
- Require a completed leak-check phase after the declared application lifecycle.
  An aborted/killed process, early crash or harness teardown that masks retained
  product ownership cannot supply a clean lifecycle result. Use an explicitly
  admitted checkpoint when ordinary process exit is not the selected observation.

## Qualify the mechanism and product cases

Use the existing isolated detector controls: a genuinely unreachable allocation
must produce LSan's leak diagnosis with the expected allocation site, while its
healthy release neighbor completes cleanly. A growth counter, nonzero exit or
semantic assertion alone does not qualify LSan. The historical standalone
`no_std` control is detector evidence only; it executes no product lifecycle.

Select product cases from the owning module: successful release, partial setup
failure, cancellation after actual native completion, and repeated lifecycle
transitions where those paths exist. Observe product cleanup before harness
cleanup, and retain reported allocation counts/bytes and source stacks. A clean
bounded run covers only the exercised graph, lifecycle and leak-check point.

## Classify limitations without hiding them

Failure to initialize the runtime, reach the leak-check phase or symbolize the
required control site is incomplete mechanism evidence. A reported allocation
needs ownership triage; do not suppress it merely because it comes from a
dependency. Any necessary scoped suppression retains its rationale, affected
coverage and remaining obligation in the coordinator's matrix. An unavailable
supported runner for an applicable row stays blocked/incomplete.

Primary sources and checked revisions: [28 November research amendment](../../SOURCES.md#sanitizer-qualification-manuals--28-november-2026).

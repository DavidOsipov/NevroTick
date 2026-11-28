# Qualify ThreadSanitizer evidence

Load for relevant shared-memory concurrency selected by the
[coordinator](../SKILL.md). Apply the [shared sanitizer rules](../../references/sanitizers.md)
for candidate admission, controls, bounded execution and evidence reuse.

## Admit the synchronization graph

- Map actual threads, native callbacks, helper pools, shared state and
  synchronization primitives. A current-thread async executor does not by itself
  exclude concurrency in native code or dependencies. Record N/A through the
  coordinator's evidenced scope/target decision; absence of relevant shared-memory
  concurrency is one valid reason. A demonstrably unsupported native target needs
  the underlying concurrency obligation and alternative evidence retained, not an
  invented TSan pass or a permanent unsupported-tool prerequisite.
- Use a separately qualified supported TSan target/profile. Classic nightly
  instrumentation requires the selected target and matching instrumented
  standard library; Rust 1.100.0 also documents the preinstrumented
  `x86_64-unknown-linux-gnutsan` target. Its availability is a qualification route,
  not admission of the current installation or application graph.
- Establish visibility of synchronization in Rust, `std`, native libraries and
  loaded modules. Rust documents unsupported atomic fences and assembly-based
  synchronization. Missing instrumentation can hide races or create false
  reports. Preserve the real algorithm; a sanitizer-only synchronization change
  cannot silently qualify the production implementation.
- Retain effective `TSAN_OPTIONS`, suppressions and excluded modules. Cargo-fuzz
  0.13.2 appends `report_signal_unsafe=0`; a direct binary launch has its own
  options. Do not make reporting disappear by ignoring a module or strengthening
  atomic orderings and then label the original execution clean.

## Qualify the mechanism and workloads

The isolated reached race control must produce TSan's race diagnosis with the
intended accesses/stacks; its repaired synchronized neighbor must complete
cleanly. An assertion about a lost update establishes a semantic failure, not
TSan detection. Historical instrumented `core`/`compiler_builtins` controls
qualify neither full `std` nor product/native synchronization.

Exercise the relevant module's shared-state and lifecycle operations with
bounded schedules/workloads: start, concurrent use, cancellation, completion
and teardown as applicable. Retain workload/seed parameters and actual reached
operations. A clean TSan execution establishes no exhaustive schedule coverage,
deadlock freedom, linearizability or higher-level protocol correctness. Keep
the existing [concurrency QA](../../qa-state-concurrency-lifecycle/SKILL.md)
and applicable Loom/semantic obligations.

## Separate runtime failure from race evidence

Shadow-memory mapping, incompatible static linking, startup, symbolization and
resource failures are setup/incomplete results, not race diagnoses. Keep ASLR
and the profile's separate virtual-address-space/resource policy; a TSan error
does not authorize weakening unrelated limits or shipping mitigations. Diagnose
a possible instrumentation false positive against the actual synchronization
path before assigning it that classification. Neither an unsupported required
path nor a timed-out workload becomes N/A or a clean result.

Primary sources and checked revisions: [28 November research amendment](../../SOURCES.md#sanitizer-qualification-manuals--28-november-2026).

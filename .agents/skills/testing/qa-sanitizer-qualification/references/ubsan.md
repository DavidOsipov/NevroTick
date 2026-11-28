# Qualify native UndefinedBehaviorSanitizer evidence

Load for compiled native C/C++ code identified by the
[coordinator](../SKILL.md). Apply the [shared sanitizer rules](../../references/sanitizers.md)
for candidate admission, controls, bounded execution and evidence reuse.

## Identify the language and check boundary

- Inspect actual native source, build scripts, static/shared libraries and FFI
  consumers in the selected target/feature graph. Rust has no documented
  standard UBSan sanitizer mode; a linked UBSan runtime instruments no missing
  Rust operations. Record bounded N/A for a scope without compiled native
  C/C++ code, preserving Rust-specific Miri and checked-arithmetic obligations.
- Instrument the native translation units that contain the selected operations.
  A prebuilt library or operating-system DLL is not covered merely because its
  caller is instrumented. Missing source or unsupported instrumentation for an
  applicable native path is a gap, not an automatic N/A.
- Declare the selected check set and exclusions. Clang's `undefined` group is
  not every available check; some integer/conversion checks diagnose suspicious
  but defined behavior. Tie findings to the language rule and product contract
  rather than treating every integer report as undefined behavior.
- Qualify the actual native compiler, optimization, link driver and compatible
  runtime, including the Rust/native FFI boundary. Standalone C++ controls do not
  establish mixed-runtime compatibility. Do not copy GNU/Linux linker settings
  into a Windows or other target profile.

## Preserve diagnostics and failure propagation

Record whether selected checks recover, terminate with a diagnostic or trap.
Many UBSan checks recover by default, so an exit-zero process may still contain
findings. An admitted runner must retain diagnostics and preserve the required
failure decision. Trap-only termination without an attributable site/mechanism
does not satisfy a diagnostic control by itself; retain sufficient debugger or
other admitted evidence, or leave that mechanism claim incomplete.

Use the existing isolated native control for the selected class, requiring its
expected UBSan diagnosis/site and a successful repaired neighbor. A generic
crash or a Rust panic is not native UBSan detection. Product tests then exercise
the actual native entry points, arguments, callback/return handling and cleanup
covered by the selected obligation; fixture success executes none of them.

## Interpret the result within its check set

Separate native language-rule findings, configured additional-check findings,
ABI/runtime setup failures and unsupported/uninstrumented paths. Fix the causal
source or boundary under the owning task; broad exclusions or recovery that
loses diagnostics cannot establish a pass. A clean bounded run covers only the
reached instrumented native operations and enabled checks. It proves neither
Rust soundness nor absence of other native memory/concurrency faults.

Primary sources and checked revisions: [28 November research amendment](../../SOURCES.md#sanitizer-qualification-manuals--28-november-2026).

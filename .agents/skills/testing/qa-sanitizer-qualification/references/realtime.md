# Qualify RealtimeSanitizer evidence

Load for an explicitly owned synchronous prohibited-operation contract selected
through the [coordinator](../SKILL.md). The module owner defines the scope and
allowed behavior; [performance/resource QA](../../qa-performance-resources/SKILL.md)
retains timing and resource measurements. Apply the coordinator's admission,
checkpoint, bounded-execution and evidence-reuse rules.

## Establish a real contract before annotating

RTSan checks intercepted or explicitly marked blocking calls while execution is
inside a marked nonblocking context. A time-synchronization product name supplies
no such context. A local contract prohibiting allocation or blocking can make
RTSan useful without claiming that the whole application is hard real-time.

Read the complete selected function, callers and success/refusal/recovery
obligations. Record the exact entry and exit, operations prohibited within them,
and preparation allowed outside. Do not infer an all-path no-allocation promise
from fixed storage or from a successful-path comment.

- `select_fixture` in [selection.rs](../../../../../crates/time-core/src/selection.rs)
  is a concrete pilot candidate: its checked inputs precede a fixed-scratch,
  no-allocation selection operation. Construct and Garde-validate those inputs
  before the marked selection scope. Exercise legitimate selection outcomes as
  well as internal refusal paths; preserve its fixture-only claim.
- [Conditioned fitting](../../../../../crates/time-core/src/conditioned_fit.rs)
  and [measurement arithmetic](../../../../../crates/nevrotick-engine/src/measurement.rs)
  permit bounded temporary Garde reports on rejection while promising
  allocation-free successful work. Blanket annotations would strengthen those
  contracts. Select a justified narrower phase or retain another applicable
  measurement method; changing the accepted contract belongs to its owner.
- Never move required rejection handling, callbacks or native work outside a
  marked region merely to report the original scope clean. Any changed scope
  needs an explicit claim boundary and the omitted obligations remain owned.

These examples establish selection questions, not adopted pilots, executed
evidence or an additional mandatory campaign for every module.

## Admit Rust instrumentation and observable operations

The recorded Rust 1.100.0 source documents unstable `-Zsanitizer=realtime` and
`#[sanitize(realtime = "nonblocking")]`, with `blocking` and `caller` modes.
Its GNU x64 target includes REALTIME; its Windows x64 MSVC target lists only
ADDRESS. The inspected LLVM runtime gate enables RTSan on Darwin/Linux. A
supported portable Rust route exists, but current native Windows behavior stays
outside that evidence. Use the coordinator's unsupported-versus-unavailable
classification; preserve the shipping compiler and existing execution holds.

Bind the separate analysis compiler/runtime, target/features, annotations and
actual call graph. Establish which allocator, library calls and application
blocking operations the runtime can observe. A custom allocator or blocking
implementation can evade the selected interceptors; account for it explicitly
and use supported application blocking annotations where appropriate. A bespoke
C++ shim is not inherently required for the documented Rust route.

Record suppression lists, disabled scopes, excluded functions, runtime options
and symbolization. Treat an exclusion as a coverage limit. Annotating one
function does not qualify every caller or dependency, and an annotation alone
does not prove the runtime is linked or reporting.

## Keep asynchronous and native claims separate

Rust's `rtsan_nonblocking_async` lint identifies a specific gap: an annotated
async function body does not cover executor work at every `.await`. The engine
also legitimately performs I/O, asynchronous waiting and bounded native work.
Do not mark the whole Tokio event loop to claim general responsiveness.

An OS nonblocking socket is not automatically an allowed RTSan operation: the
inspected runtime intercepts `send`, `sendto`, `recv` and `recvfrom` without
classifying the socket mode. Select synchronous computation/commit sections
whose actual contract matches the marked context. Linux interception provides
no native Windows callback, provider-completion or scheduling evidence.

## Qualify the mechanism and selected section

Use an owned isolated control that reaches a representative prohibited operation
through the relevant runtime/allocator path. Require the expected RTSan diagnosis
at the intended site and a permitted healthy neighbor. An assertion, timeout or
generic process failure supplies no detector control. Reuse valid controls only
within their recorded instrumentation and runtime assumptions.

Exercise the frozen section with its independent accepted/refused cases. Record
the actual selected entry point, annotation boundary, reached outcomes, raw
diagnostics, exclusions, first failures and consumed budget. Distinguish a
prohibited-operation finding from permitted work outside scope, a changed
contract, missing interception, setup failure or resource exhaustion.

A scoped clean result means no reported prohibited intercepted or marked call
in the exercised context under that configuration. It proves no worst-case
execution time, numerical deadline, scheduler fairness, complete absence of
custom blocking, timestamp quality or UTC accuracy. Pure computation can exceed
a latency budget without calling an observed operation. Keep ordinary timing,
resource, cancellation/completion and native observations with their existing
owners; instrumented execution cannot replace uninstrumented performance data.

Primary references and immutable compiler/runtime pins:
[28 November additional-mechanism research](../../SOURCES.md#additional-sanitizers-and-hardening--28-november-2026),
[LLVM RTSan manual](https://clang.llvm.org/docs/RealtimeSanitizer.html) and
[Rust async-annotation limitation](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/compiler/rustc_lint_defs/src/builtin.rs).

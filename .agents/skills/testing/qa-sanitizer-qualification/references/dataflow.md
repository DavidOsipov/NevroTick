# Qualify DataFlowSanitizer evidence

Load for a named dynamic information-flow question selected through the
[coordinator](../SKILL.md). The existing
[data-flow analysis owner](../../qa-data-flow-analysis/SKILL.md) owns the property,
sources, transformations, guards and sinks; the coordinator owns profile
admission, checkpoint timing, controls, bounds and evidence reuse.

## Select a useful property and supported target

DFSan propagates configured labels through executed code. It supplies no
application-specific violation detector until the analysis defines the labels,
permitted flows and assertions at actual sinks. State the protected property and
the incremental evidence sought before preparing a profile. A confidentiality
question about secret-derived diagnostic output is a useful candidate; merely
enabling another sanitizer is not a qualification objective.

The recorded Rust 1.100.0 source documents unstable `-Zsanitizer=dataflow` and
includes DATAFLOW for `x86_64-unknown-linux-gnu`. Its Windows x64 MSVC target lists
only ADDRESS; the inspected LLVM runtime build gate enables DFSan only on Linux.
Use the coordinator's evidenced native-target disposition while retaining the
underlying information-flow requirement and its existing owner. A supported
portable analysis can still be useful; it does not execute Windows-only paths.
Code generation support admits neither the installed runtime nor a product
runner. Preserve the shipping compiler and separately admit the analysis profile.

## Freeze the label model and reached graph

- Name actual input representations and labeling points, transformed/stored
  values, real output consumers, permitted transformations and the violation
  checked at each sink. Use owned dummy secrets. Preserve the existing task's
  flow ledger rather than creating a competing property inventory.
- Separate ancestry from current authorization. An authorized evidence export
  can legitimately contain sensitive data while an ordinary log cannot. Labels
  alone establish no audience, consent, generation, expiry or revocation check.
  Model those state requirements with the existing semantic owner.
- Define allowed cryptographic output and disclosure precisely. A hash or
  encryption call supplies no universal permission to clear labels; propagating
  a secret label also does not by itself make every intended protocol output a
  violation. Independently justify any property-specific declassification.
- Bind compiler/runtime, target/features, instrumented `std` and dependencies,
  ABI lists and custom wrappers. Every relevant uninstrumented function needs
  the selected boundary behavior accounted for. Assembly, native providers and
  Windows APIs cannot acquire coverage from a Linux stub. Process/IPC boundaries
  require explicit models; labels do not automatically follow serialized bytes
  into a different process.
- Retain effective pointer/offset-label propagation, select-condition tracking,
  conditional callbacks, origin settings, ABI categories, suppressions and
  `force_zero_labels` behavior. A discard rule or zero label can erase the very
  dependency being investigated. Do not repair a negative result by introducing
  an unreviewed neutral wrapper or broad label-clearing rule.

DFSan can track select conditions and expose conditional callbacks. Those
facilities do not establish complete implicit-flow coverage or noninterference
for all branch-dependent outputs. Record what the selected model observes and
what remains outside it, including unexecuted paths and relevant side channels.

## Qualify propagation and the application oracle separately

1. Show that admitted input reaches the actual labeling point and intended
   consumer; a useful permitted case must complete with its expected output.
2. Observe known labels propagating through representative operations and each
   material modeled boundary. An unmodeled call or unrelated assertion failure
   cannot establish that the instrumentation preserves the intended dependency.
3. With owned non-sensitive controls, demonstrate that the named forbidden flow
   reaches the sink oracle and is reported, while its permitted neighbor is
   accepted. The expected sink and label observations come from the independent
   property, not from copying the candidate's propagation result.

DFSan-backed application assertions can provide valid domain evidence when
label delivery/propagation and oracle sensitivity are independently established.
There is no built-in ASan-style memory diagnosis to require for this property.
A generic panic or nonzero exit without those observations proves no DFSan
mechanism or information-flow result. Preserve the separate control outcomes.

Run the admitted bounded cases through actual selected entry points. Include
relevant formatting, error/recovery and permitted-disclosure behavior. Inspect
outputs before harness cleanup hides them, preserving candidate/model identity,
selected/reached case counts, label observations and first failures. Evidence
reuse must also match the labeling points, boundary models and sink oracles.

## Report the narrow result

Distinguish a confirmed property violation from missing input delivery, label
loss, inadequate instrumentation/modeling, oracle failure, setup failure or
resource exhaustion. A clean sink can reflect a missing path or lost label;
resolve those alternatives before recording a scoped pass. A required supported
profile without adequate instrumentation or admission remains incomplete.

Return reached sources/sinks, permitted and refused cases, modeled boundaries,
effective exclusions and remaining owner/gaps to the existing flow task and
coordinator. Dynamic observations complement source/CodeQL and semantic evidence;
they establish neither complete confidentiality nor native Windows enforcement.
This conditional manual creates no runner, installation or blanket V1 campaign.

Primary references and immutable target/runtime pins:
[28 November additional-mechanism research](../../SOURCES.md#additional-sanitizers-and-hardening--28-november-2026),
[LLVM DFSan manual](https://clang.llvm.org/docs/DataFlowSanitizer.html) and
[design](https://clang.llvm.org/docs/DataFlowSanitizerDesign.html).

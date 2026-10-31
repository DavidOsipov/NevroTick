# Selecting interacting cases

Use for potentially hidden optional-field/default/state/interacting-guard
failures. Small deterministic table suffices if selected mechanism covered;
no factor matrix per task required.

Before execution name requirement, actual entrypoint, independent
preserve/transform/omit/reject expectations. Record relevant identity, cardinality,
ordering, exactness, forbidden effects. Derive dimensions from real
producer/consumer; convenient fixtures alone insufficient.

Distinguish omitted/defaulted values from explicit null/false/zero/empty where
contract meaning differs. Include applicable cold/warm state, optional-cache
absence, replay/repeat/restart paths beside explicit healthy case. Cached results
must not hide missing fresh behavior or create correction authority.

For interacting rules, write a finite decision table: relevant inputs/pre-state,
constraints making the combination feasible, expected decision/effect, and its
independent justification. Examples include source count with authentication
policy, approval with policy/generation changes, or missing persisted state during
recovery. Reach the intended guard: an earlier unrelated rejection does not test
the selected interaction. Keep a legitimate neighboring control.

For a selected critical compound decision, retain independent-condition pairs:
identify its condition occurrences, actual entrypoint and expected decision.
In a unique-cause pair, the selected condition and decision change while other
condition values stay fixed. Reach both cases through valid construction; record
short-circuited operands as unevaluated rather than false. Repeated/coupled
conditions can make a pair infeasible: explain the constraint, retain alternative
evidence and never fabricate invalid state to complete a table. If using a masking
variant, justify why other changed conditions cannot explain the changed decision.
Relate pairs to existing guard/mutation obligations; neither substitutes for an
independent behavioral expectation.

A reached logical table supplies condition-influence evidence, not an instrumented
MC/DC percentage, exhaustive path claim or certification. Clang instrumentation
does not establish Rust support: verify the exact admitted compiler/coverage tool
before proposing flags; do not copy a historical nightly MC/DC recipe.
Primary distinctions: [Clang MC/DC reporting](https://clang.llvm.org/docs/SourceBasedCodeCoverage.html#interpreting-reports),
[Rust MC/DC implementation history](https://goals.rust-lang.org/2026/mcdc-coverage-support.html),
[Rust coverage options](https://doc.rust-lang.org/unstable-book/compiler-flags/coverage-options.html).

Choose cases by plausible mechanism/budget. Include known higher-order
interactions directly; pairwise coverage/aggregate counts do not prove them.
Avoid unreachable Cartesian products/invented requirements. Retain selected,
executed, checked cases/facts, exact candidate/environment, unresolved
combinations in existing task record.

The original website procedures and scope of this adaptation are recorded in
[testing sources](../SOURCES.md) and [provenance](../../PROVENANCE.md).

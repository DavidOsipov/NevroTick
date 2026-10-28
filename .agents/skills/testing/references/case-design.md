# Selecting interacting cases

Use when optional fields, defaults, state or interacting guards could hide a
failure. A small deterministic table is enough when it covers the selected
mechanism; this reference does not require a factor matrix for every task.

Before execution, name the requirement, actual entrypoint and independently
expected preserve/transform/omit/reject behavior. Record identity, cardinality,
ordering, exactness and forbidden effects where relevant. Pick dimensions from
the real producer and consumer, not just convenient existing fixtures.

Distinguish omitted and defaulted values from explicit null, false, zero and empty
values when the contract gives them different meaning. Include applicable cold
and warm state, optional-cache absence, replay, repeat and restart paths beside
the explicit healthy case. A cached result must not hide missing fresh behavior
or create new correction authority.

For interacting rules, write a finite decision table: relevant inputs/pre-state,
constraints making the combination feasible, expected decision/effect, and its
independent justification. Examples include source count with authentication
policy, approval with policy/generation changes, or missing persisted state during
recovery. Reach the intended guard: an earlier unrelated rejection does not test
the selected interaction. Keep a legitimate neighboring control.

Choose cases by plausible mechanism and budget. Include a known higher-order
interaction directly; pairwise coverage or aggregate counts do not prove it.
Avoid unreachable Cartesian products and invented requirements. Retain selected,
executed and checked cases/facts, their exact candidate/environment and unresolved
combinations in the existing task record.

The original website procedures and scope of this adaptation are recorded in
[testing sources](../SOURCES.md) and [provenance](../../PROVENANCE.md).

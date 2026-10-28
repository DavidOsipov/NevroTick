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

Choose cases by plausible mechanism/budget. Include known higher-order
interactions directly; pairwise coverage/aggregate counts do not prove them.
Avoid unreachable Cartesian products/invented requirements. Retain selected,
executed, checked cases/facts, exact candidate/environment, unresolved
combinations in existing task record.

The original website procedures and scope of this adaptation are recorded in
[testing sources](../SOURCES.md) and [provenance](../../PROVENANCE.md).

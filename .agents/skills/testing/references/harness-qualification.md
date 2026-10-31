# Qualify the driver and observer

Use when adding or changing a simulator, fixture driver, fuzz adapter, comparator
or result collector whose mistake could make the selected behavior appear tested.
Reuse adequate controls while their relevant bytes and assumptions are unchanged.
This supports CHECK-6/19/25; it does not add a campaign to every small test edit.

For changed Python accounting/report interfaces, use
[verification typing](python-verification-typing.md) without replacing runtime
admission. For repeated positional counter mappings across Rust/Python, consider
[mechanical counter generation](counter-transport-generation.md); shared field
definitions must not become the independent expected-result oracle.

## Bind facts to the real route

Before observing candidate output, identify the facts that distinguish the chosen
case: exact input bytes or field values, source identity, event order, capture
instant, clock domain, expected output/state and forbidden effects. Use a small
table in the existing test/task when several events or consumers interact.

Trace each fact through **intended stimulus → actual candidate call → observation
→ assertion**. Inspect where the driver delivers or transforms the value; its
self-reported label alone does not prove the input reached the candidate. A
recorded digest can bind exact bytes when retaining the full payload is unsuitable,
but length or digest equality alone does not prove a call occurred. Preserve the
required relationship between input identity, call ordinal and observed result.
Keep expected facts independent of production parsing/formatting and of the
driver's classification of its own actions.

For each claimed absence, identify the observation window and owner being watched.
For each retained-resource claim, observe the application before harness cleanup.
An observer must not perform the transition it is intended to verify.

## Worked failure: a malformed case that never exercised its boundary

Suppose a fixture declares a complete frame of length `L` and a particular
malformed prefix of length `L-1`. Its comparator accepts any rejected input whose
length is at most `L`. Substituting an empty buffer still produces rejection and
passes that comparator, although the intended truncation boundary was never used.

Bind the malformed case to its independently authored prefix bytes and exact
`L-1` length at the actual receiving call. Bind the healthy case to its own exact
bytes and expected successful outcome. Checking only the shared rejection category
or borrowing the driver's current length as the expectation preserves the gap.
Here `L` belongs to the example's declared format, not a new product limit.

## Select discriminating controls

Choose controls for plausible mistakes in the changed harness; no fixed number
or full Cartesian product is required. Use task-owned variants or a fake subject
for observer-only controls, keeping a real-candidate healthy run separate.

| Harness obligation | Useful controlled perturbation | Required distinction |
| --- | --- | --- |
| Exact admitted input | Substitute different bytes/length while keeping the same result category | The input-fact assertion detects the substitution. |
| Identity/order/cardinality | Swap identities or ordinals; omit or duplicate one event | The specific promised fact fails, even if totals match. |
| Clock/transition routing | Advance a different clock or deliver after invalidation | The declared state/effect expectation detects the wrong route. |
| Comparator sensitivity | Change one semantic sentinel; suppress one required comparison | The intended comparison/control fails; permitted variation still passes. |
| Result propagation | Cause an intended assertion failure through the adopted runner | Child failure reaches the collector/supervisor; it is not reported as a passing candidate. |

A comparator unit control does not establish that the real driver exercised the
candidate. Conversely, a real candidate rejection does not qualify the comparator.
Retain both layers when both are claimed. For fuzzing, qualify the adapter with a
finite accepted seed and a targeted fault before interpreting the bounded campaign;
seed/corpus counts alone show neither semantic reachability nor correct forwarding.

For sanitizer claims, also use [detector-specific controls](sanitizers.md#qualify-the-selected-mechanism).
An assertion proves comparator sensitivity or failure propagation; qualifying
ASan detection requires a representative memory fault with an ASan diagnosis.
Reuse matching mechanism evidence and keep its scope separate from product tests.

Record exact variant/input identity, selected/executed checks, intended failing
assertion, actual outcome and exit propagation. Build/setup failures are not
successful discrimination. A surviving harness fault limits the affected claim;
repair the observer/driver and rerun the affected controls without relabeling it
as a product defect. Preserve original failures and meaningful healthy neighbors.

Method extracted from the corrected input-oracle finding in
[S1.1e](../../../../tasks/s1.1-deterministic-harness.md#first-integrated-execution-and-exact-input-correction).
That historical result is evidence for its own candidate only.

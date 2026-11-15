# Generated sequential state machines with Proptest

Use for bounded lifecycle histories with independently modeled expectations.
Read [model testing](../qa-model-based-testing/SKILL.md),
[property testing](../qa-property-based-testing/SKILL.md) and the [contract](../CONTRACT.md).
Source review: **2026-11-15**, upstream `proptest-state-machine` **0.9.0**.
Conditional guidance; no installed/admitted dependency or CLI lane.
Inspect current manifests/locks before use; follow [dependency admission](../../environment/dependency-updates/SKILL.md),
including selected 14-day registry cooldown and whole-graph review.

## Define the guarantee and actual route

Name requirement/CHECK, owner, admitted domain, independent expected outputs,
state changes, forbidden effects. Small contract-derived reference model; no
production reducers, validators or eligibility helpers computing expectations.
Retain analytic healthy/refusal examples.

Map commands to actual production constructor/handler/consumer. Inspect delivered
arguments and call ordinal. Never substitute test-only setters for claimed
transitions. Compare result, state, observable effects after every command.
Include relevant policy/current-generation invalidation, stale result identity,
take-once settlement, cancellation, retry, late completion. Model expected
rejection reason and unchanged protected state.

Generate related values jointly. Explicitly model invalid API calls as executable
commands with expected rejection; never discard via `preconditions`,
`prop_assume!` or early return. Preconditions restrict only scenario construction
constraints, including dependencies broken by shrinking. Distinguish unavailable
model objects from requests the API must refuse.

Inject deterministic clock inputs; separate UTC, monotonic, active-awake time.
Time-advance commands name domain, origin, expected effect. Never derive expected
expiry from application clock/timeout calculations. Sequential ordering covers
explicit histories, not thread interleavings, weak memory, native Windows
completion or real clock behavior.

## Verified API shape

| Role | Required implementation and trap |
| --- | --- |
| `ReferenceStateMachine` | `State` and `Transition`: `Clone + Debug`; `init_state()` and `transitions(&State)` return boxed strategies; `apply(State, &Transition)` returns next reference state. |
| `preconditions` | Optional; default true. Checked during generation and shrinking; state-dependent filtering uses global rejection limits. |
| `StateMachineTest` | Define `SystemUnderTest`, `Reference`, `init_test` and `apply`; initialize corresponding fresh SUT state. `apply` receives reference state **after** its transition. |
| `check_invariants` | Default no-op; override or assert equivalently in `init_test`/`apply`. Default runner invokes initially and after each transition. |
| `teardown` | Optional; default drops state. Runner calls at normal sequence end; no panic-safe cleanup guarantee. |

Implement traits, then use this macro shape:

```rust,ignore
prop_state_machine! {
    #[test]
    fn lifecycle_history(sequential 1..20 => LifecycleTest);
}
```

Illustrative range; choose task-owned nonempty bounds. Currently `sequential` only.
Supply explicit Proptest configuration with `#![proptest_config(...)]`; reconcile
selected version. Adapters use actual entry points; macro supplies no semantic oracle.

## Isolation, reachability and evidence

Fresh model, SUT, clocks, owned resources per generated case, shrink attempt and
concrete replay. Register RAII/guard cleanup before fallible steps; restore
external fixture resources on success and failure. Observe product
cleanup before harness cleanup masks leaks. Failed reset invalidates subsequent
reductions. Preserve intended state only within one command history.

Bound cases, transitions, rejected draws, bytes, shrink work, elapsed time, cleanup.
Count actual receiver calls and reached semantic partitions, including selected
current/stale and accept/reject neighbors. Nonempty generation alone cannot
establish each partition; retain concrete sparse-path witnesses. Unreached
antecedents, empty selections and exhausted budgets stay incomplete.

Use [harness qualification](harness-qualification.md) for driver/comparator changes.
Link plausible controlled fault to each changed guarantee: stale identity
acceptance, consumed-budget refund, omitted rejection, swapped clock domains or
suppressed comparison. Require intended assertion through real route, healthy
neighbor and failure propagation. Unrelated setup failure or model-only
disagreement earns no evidence.

Retain original/minimized concrete history, initial state, seed, Proptest/crate
versions, strategy/configuration and source/lock/feature/target identity. Bind
current policy, driver, oracle, result bytes; prior passes transfer only while
those dependencies remain valid. Independently replay minimized history from
clean state; keep named regression. After generator changes, seeds/regression
storage alone are not durable inputs. Report actual execution, exclusions,
remaining concurrency/native gaps; no campaign opening or admission policy change.

## Primary sources

- [Reference model API](https://docs.rs/proptest-state-machine/latest/proptest_state_machine/strategy/trait.ReferenceStateMachine.html).
- [Test API](https://docs.rs/proptest-state-machine/latest/proptest_state_machine/test_runner/trait.StateMachineTest.html) and [runner source](https://docs.rs/proptest-state-machine/latest/src/proptest_state_machine/test_runner.rs.html).
- [Macro](https://docs.rs/proptest-state-machine/latest/proptest_state_machine/macro.prop_state_machine.html) and [upstream guide](https://github.com/proptest-rs/proptest/blob/main/book/src/proptest/state-machine.md).

# Generated sequential state machines with Proptest

Use for bounded Rust histories with independently modeled expectations. Read
[model testing](../qa-model-based-testing/SKILL.md), [property testing](../qa-property-based-testing/SKILL.md)
and the [testing contract](../CONTRACT.md). The [selection guide](generated-testing-selection.md)
compares tools and owns the shared NevroTick pilot workflow.

**Research checkpoint: 2026-11-15 UTC.** Rolling docs resolve to
[`proptest` 1.12.0](https://docs.rs/proptest/latest/proptest/) and
[`proptest-state-machine` 0.9.0](https://docs.rs/proptest-state-machine/latest/proptest_state_machine/).
Source checked at upstream revision
[`70cf37d46b7d83393b6ed94c611f8c52209f585d`](https://github.com/proptest-rs/proptest/tree/70cf37d46b7d83393b6ed94c611f8c52209f585d).
Neither package occurs in inspected product manifests/lockfile. These research
versions are not an adopted graph, installed tool or qualified product lane.
Reinspect actual source, lock, features and permissions before use.

## Contents

- [Choose the right test shape](#choose-the-right-test-shape)
- [Admit the dependency and execution lane](#admit-the-dependency-and-execution-lane)
- [Define the oracle and actual receiver](#define-the-oracle-and-actual-receiver)
- [Construct useful strategies](#construct-useful-strategies)
- [Understand the trait lifecycle](#understand-the-trait-lifecycle)
- [Complete educational example](#complete-educational-example)
- [Set bounds and run only through admission](#set-bounds-and-run-only-through-admission)
- [Shrink, persist and replay](#shrink-persist-and-replay)
- [Diagnose incomplete or misleading results](#diagnose-incomplete-or-misleading-results)
- [Qualify the harness and close the decision](#qualify-the-harness-and-close-the-decision)

## Choose the right test shape

| Question | Smallest useful approach |
| --- | --- |
| Known boundary, refusal or prior defect? | Named deterministic example through the real receiver; retain alongside generation. Without framework admission, use existing deterministic sequences. |
| One input or related input tuple determines the result? | Ordinary `proptest!` property with bounded strategy and independent oracle. A state-machine framework adds little. |
| Correctness depends on earlier commands, generations, acknowledgements or cancellation? | `proptest-state-machine`: generated initial state and sequential action history, checked after each action and shrunk on failure. |
| Correctness depends on a thread schedule or atomic ordering? | Route to [lifecycle testing](../qa-state-concurrency-lifecycle/SKILL.md), admitted Loom or conditional [Shuttle](shuttle-concurrency.md). Sequential histories cannot explore those mechanisms. |

The [upstream state-machine guide](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/book/src/proptest/state-machine.md)
describes generation and shrinking: sampled testing, not exhaustive proof.
The current macro supports `sequential` only. Actual native completion, Windows
authority, services, clocks and resource restoration require separate evidence;
OD-10's post-V1 formal-work deferral and all existing holds remain unchanged.

## Admit the dependency and execution lane

Follow [dependency updates](../../environment/dependency-updates/SKILL.md),
[Cargo operations](../../environment/dependency-updates/references/cargo-operations.md)
and [configured tooling](configured-tooling.md). No
`check-analysis.py --tool proptest` lane exists. For a missing library, follow
[setup and availability](generated-testing-setup.md#rust-libraries-and-the-actual-test-build)
through dependency preparation and actual runner. Current task authority
controls installation/resolution; this reference creates no product crate,
configured lane or direct-Cargo fallback.

For an authorized proposal, exact requirements for the researched pair:

```toml
# Candidate dev-dependencies only; not an instruction to edit a product manifest.
[dev-dependencies]
proptest = "=1.12.0"
proptest-state-machine = "=0.9.0"
```

The reviewed [state-machine manifest](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/proptest-state-machine/Cargo.toml)
defaults to `std` and requests Proptest defaults plus `fork`, `timeout`, `bit-set`.
The [Proptest manifest](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/proptest/Cargo.toml)
declares Rust 1.88, as does the state-machine manifest. Inspect the resolved
archives/graph: a direct `default-features = false` cannot disable another edge's
features. Optional `attr-macro` is unnecessary for these macros.
Review transitive compiler compatibility, host/build code, licences, checksums,
patches, advisories, target support and feature unification before package execution.

Apply [OD-19](../../../../docs/nevrotick-owner-decisions.md#23-od-19--14-day-registry-dependency-cooldown):
14-day minimum age and `deny` for every newly selected registry version in the
whole direct/transitive/host/build/proc-macro delta. Reproduce imported locks from
the accepted baseline; missing publication times remain unadmitted. Research age,
a cached archive or an exact version string supplies no admission. Only a concrete
OD-04 exception for the exact whole delta can change one owned acquisition child.

## Define the oracle and actual receiver

Record requirement/CHECK, owner, admitted domain, expected result, state change,
permitted/forbidden effects and environment before constructing strategies.
Derive a small reference model from contract or analytic facts; production
reducers, validators, formatters and eligibility helpers cannot compute its truth.
Retain accepted and refused analytic neighbors to challenge model and SUT.

Map every action to the actual constructor/handler/consumer and inspect received
arguments, identity and call ordinal. Test-only setters or a second implementation
of the product are not its SUT. After every action compare return/error category,
relevant state and observable effects; inspect rejection atomicity and absent
protected effects. Observers must not perform the transition they claim to observe.
For acknowledgements, retain requested versus actually completed effects separately.

Compute expected return/effects from the model's **pre-state** and action in
`ReferenceStateMachine::apply`; carry expectations in its returned post-state.
The SUT adapter receives that **post-state**. Preserve a separate model pre-state
snapshot when needed; never infer expected old state from application output.
Inject deterministic clock inputs, with separate UTC, monotonic and active-awake
domains. Time advances name domain/origin and independent expiry expectations.

**Preconditions restrict scenario construction, not negative API testing.** A
symbolic handle whose creating action disappeared during shrinking may need a
model precondition. An executable unknown/stale/unauthorized raw request must
reach the API and assert its documented refusal; do not discard it with
`preconditions`, `prop_assume!`, an early return or a production validator.
See [reference-model semantics](https://docs.rs/proptest-state-machine/latest/proptest_state_machine/strategy/trait.ReferenceStateMachine.html)
and [harness qualification](harness-qualification.md).

## Construct useful strategies

A strategy specifies generation **and shrinking**. Keep both deterministic and
side-effect free: model generation precedes every SUT history.
Use [strategy basics](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/book/src/proptest/tutorial/strategy-basics.md)
and the [Strategy API](https://docs.rs/proptest/latest/proptest/strategy/trait.Strategy.html).

| Construction | Intended use and trap |
| --- | --- |
| `any::<T>()`, typed ranges, bounded `collection::vec` | Set explicit numeric, length and allocation domains. `1..65` requests lengths 1–64; avoid implicit large strings/collections. |
| Tuples plus `.prop_map(...)` | Generate related fields together; derive dependent values while preserving source shrinking. Extract reusable `impl Strategy<Value = T>` functions; trait methods here require `.boxed()`. |
| `.prop_flat_map(...)` | Build a second strategy from the first value, e.g. a valid index into its generated vector. Bound regeneration; deeply nested dependencies can be expensive. |
| `Just(value)`, weighted `prop_oneof!` | Add explicit anchors and action families. `Just` cannot shrink its value; weights influence sampling without proving a partition was reached. |
| `sample::select(owned_values)` | Choose an existing model ID from a deterministic nonempty snapshot. Clone/sort IDs as needed; do not borrow model state into a `'static` boxed strategy or call `select` with an empty collection. |
| `.prop_filter(...)`, `prop_assume!` | Local versus whole-case rejection. Reserve for domain constraints; excessive filtering slows generation, blocks useful shrinks and can abort the property. |

This complete strategy fragment constructs a dependent valid pair:

```rust,ignore
use proptest::prelude::*;
fn bytes_and_valid_index() -> impl Strategy<Value = (Vec<u8>, usize)> {
    proptest::collection::vec(any::<u8>(), 1..17).prop_flat_map(|bytes| {
        let length = bytes.len();
        (Just(bytes), 0..length)
    })
}
```

Add a separate invalid-index strategy when rejection is the property. For state-
dependent actions, generate useful candidates from the model and implement
`preconditions`: deleting an earlier action can invalidate a formerly valid ID.
Keep sparse paths as concrete witnesses and count actual received actions.
Sources: [dependent strategies](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/book/src/proptest/tutorial/higher-order.md),
[transforms](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/book/src/proptest/tutorial/transforming-strategies.md),
[filtering](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/book/src/proptest/tutorial/filtering.md),
[selection](https://docs.rs/proptest/latest/proptest/sample/fn.select.html).

## Understand the trait lifecycle

| API | Contract |
| --- | --- |
| `ReferenceStateMachine: 'static` | `State` and `Transition` require `Clone + Debug`. `init_state() -> BoxedStrategy<State>`; `transitions(&State) -> BoxedStrategy<Transition>`; `apply(State, &Transition) -> State`. |
| `preconditions(&State, &Transition) -> bool` | Defaults to true. Rechecked during generation/shrinking; rejected generation consumes the local rejection allowance in the reviewed implementation. |
| `StateMachineTest` | Associate `SystemUnderTest` and `Reference`. `init_test(&Reference::State) -> SystemUnderTest` creates fresh corresponding state. |
| SUT `apply(SystemUnderTest, &Reference::State, Transition) -> SystemUnderTest` | Receives the reference post-state; execute the real action once and assert its result/effects. Use ordinary assertions here: `prop_assert!` returns `TestCaseError` and does not fit this return type. |
| `check_invariants(&SystemUnderTest, &Reference::State)` | Defaults to no-op; override. The default runner checks initial state and the state after each action. |
| `teardown(SystemUnderTest, Reference::State)` | Defaults to dropping state; called at normal sequence completion. Not a panic-safe `finally` hook. |

The trait prose says global rejection, but the reviewed
[`Sequential::new_tree` source](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/proptest-state-machine/src/strategy.rs)
calls `reject_local` for a failed precondition. Use the implementation's counter
when diagnosing exhaustion; a test's `prop_assume!` still uses global rejection.

The [runner source](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/proptest-state-machine/src/test_runner.rs)
orders initialization → initial invariant → model apply → SUT apply → invariant,
then normal teardown. Its internal `seen_counter` increments before applying an
action; it helps trim an unseen failure tail, not count successful receiver calls.
Prefer the macro to overriding this bookkeeping. Low-level concrete replay
uses `test_sequential(config, initial_state, transitions, None)`; the macro supplies
the generated counter. Older two-argument runner prose is abbreviated.

## Complete educational example

This original example tests real standard-library `VecDeque`, with an
independent list model of FIFO order. It is **not NevroTick wiring or a
test-only replacement for a production component**. API signatures/imports were
checked against the sources above; it was **not compiled or executed** in this
documentation-only review; not a workspace lint/qualification result.
The empty-pop behavior follows [VecDeque's contract](https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.pop_front).
After admission, this could be an integration-test source file:

```rust,ignore
use std::collections::VecDeque;
use proptest::prelude::*;
use proptest::test_runner::Config;
use proptest_state_machine::{prop_state_machine, ReferenceStateMachine, StateMachineTest};

#[derive(Clone, Debug)]
enum Action { Push(u8), Pop }
#[derive(Clone, Debug)]
struct Model { items: Vec<u8>, popped: Option<u8> }
struct FifoModel;
struct FifoTest;

impl ReferenceStateMachine for FifoModel {
    type State = Model;
    type Transition = Action;
    fn init_state() -> BoxedStrategy<Model> {
        proptest::collection::vec(0u8..=15, 0..5)
            .prop_map(|items| Model { items, popped: None }).boxed()
    }
    fn transitions(_: &Model) -> BoxedStrategy<Action> {
        prop_oneof![Just(Action::Pop), (0u8..=15).prop_map(Action::Push)].boxed()
    }
    fn apply(mut model: Model, action: &Action) -> Model {
        model.popped = None;
        match action {
            Action::Push(value) => model.items.push(*value),
            Action::Pop if !model.items.is_empty() => {
                model.popped = Some(model.items.remove(0));
            }
            Action::Pop => {}
        }
        model
    }
    // Every action is executable, including Pop on an empty queue.
    // Keep the default preconditions=true; do not filter the empty case.
}

impl StateMachineTest for FifoTest {
    type SystemUnderTest = VecDeque<u8>;
    type Reference = FifoModel;
    fn init_test(model: &Model) -> VecDeque<u8> {
        VecDeque::from(model.items.clone())
    }
    fn apply(mut sut: VecDeque<u8>, after: &Model, action: Action) -> VecDeque<u8> {
        match action {
            Action::Push(value) => sut.push_back(value),
            Action::Pop => assert_eq!(sut.pop_front(), after.popped),
        }
        sut
    }
    fn check_invariants(sut: &VecDeque<u8>, model: &Model) {
        assert_eq!(sut.iter().copied().collect::<Vec<_>>(), model.items);
    }
}

fn fifo_config() -> Config {
    Config {
        cases: 64,
        max_local_rejects: 256,
        max_global_rejects: 256,
        max_flat_map_regens: 1024,
        max_shrink_iters: 512,
        max_shrink_time: 2000,
        timeout: 1000,
        ..Config::default()
    }
}
prop_state_machine! {
    #![proptest_config(fifo_config())]
    #[test]
    fn fifo_generated_history(sequential 1..65 => FifoTest);
}

#[test]
fn fifo_empty_pop_then_order() {
    FifoTest::test_sequential(
        fifo_config(),
        Model { items: vec![], popped: None },
        vec![Action::Pop, Action::Push(2), Action::Push(1),
             Action::Pop, Action::Pop, Action::Pop],
        None,
    );
}
```

The fixed witness expects `None`, then `Some(2)`, `Some(1)`, `None` at its pops.
The model derives the popped value from its old queue; the adapter checks that
output against the post-model field and checks full queue order separately.
Changing the adapter to `pop_back()` must fail the two-distinct-values witness.
That would qualify this educational comparator, not any product guarantee.

## Set bounds and run only through admission

Use the [Config API](https://docs.rs/proptest/latest/proptest/test_runner/struct.Config.html)
and [configuration source](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/proptest/src/test_runner/config.rs).
Replace the illustrative settings with task-owned
admitted values before product use:

| Bound | Meaning and additional custody |
| --- | --- |
| `cases: 64`; history `1..65` | 64 successful generated cases, each requesting 1–64 actions. Persisted replays, rejections and shrinking add work. Initial invariant failure can precede every action. Count cases separately from transitions and assertions. |
| Initial length `0..5`; byte values `0..=15` | At most 68 logical queue elements after 64 pushes. This does not bound total allocator overhead, child processes, traces or compiler RAM. |
| Local/global rejection `256`; flat-map regeneration `1024` | Rejection exhaustion aborts the property. The separate flat-map cap stops further dependent-value regeneration during shrinking; it is not the same abort condition. Inspect which limit was reached before changing the strategy or budget. |
| Shrink iterations `512`; shrink time `2000` ms | Finite reduction work; shrink time does not interrupt a running case. Default `u32::MAX` iterations means four times `cases`, not an unlimited explicit budget. Zero iterations disables shrinking. |
| `timeout: 1000` ms | Requires `timeout`; nonzero automatically enables subprocess execution. Startup can consume the allowance. It is a case limit, not a whole-property deadline. |
| Outer execution envelope | Separately admit process count, total wall time, RSS, disk/output and cleanup deadline. For a tiny pilot, example envelope: one test at a time, 60 s wall, 256 MiB aggregate test-process RSS, 10 MiB output and 5 s cleanup; these are proposed limits requiring an enforcing supervisor, not library guarantees or compiler limits. |

**Environment is part of the bound.** In 1.12.0,
[`proptest!` re-applies `contextualize_config`](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/proptest/src/sugar.rs)
to explicit macro configuration. Ambient `PROPTEST_*` values can replace
shown fields, including cases, timeout and persistence. Admit/scrub those variables,
record effective settings and reject zero generated cases where generation is
required. Keep config expressions deterministic. Older book examples suggesting
an explicit field always defeats an environment override do not control this version.
The [flattening implementation](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/proptest/src/strategy/flatten.rs)
shows the regeneration cap separately from rejection and case limits.

These are **underlying command shapes inside a separately admitted runner**,
not current repository commands to execute. Retain its exact toolchain, target,
profile and features; discovery itself may compile test code:

```sh
cargo test --frozen -p ADMITTED_PACKAGE --test ADMITTED_TARGET -- --list
cargo test --frozen -p ADMITTED_PACKAGE --test ADMITTED_TARGET FULL_TEST_NAME -- --exact --nocapture --test-threads=1
```

Require the intended full test name and nonempty selected/executed roster. Use the
same admitted selector for the concrete witness separately. Direct `test_sequential`
does not create a Proptest runner, apply `cases` or enforce `Config.timeout`;
the fixed replay still needs the outer deadline. Admit `PROPTEST_VERBOSE=1` for
transition diagnostics and `PROPTEST_RNG_SEED=<u64>` for a recorded seeded run only
with bounded redacted output. Preserve normal failure exits and stderr.

## Shrink, persist and replay

The [sequential strategy source](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/proptest-state-machine/src/strategy.rs)
first removes unseen tail actions, then attempts action deletion, action-value
shrinking and initial-state shrinking while respecting preconditions. A result is
the smallest found within this strategy/budget, not a globally minimal history.

Create fresh model, SUT, owned resources and clock origins for **every** generated
case, shrink attempt and concrete replay. Register RAII guards before fallible
setup. Assertion unwinding skips normal `teardown`; abort/timeout may skip even
destructors. Own external cleanup through the admitted supervisor and verify it.
Observe product cleanup before harness cleanup masks leaks. Failed reset invalidates
subsequent reductions; do not reuse a poisoned process or half-restored fixture.
See [forking/timeouts](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/book/src/proptest/forking.md).

Default `std` persistence uses `SourceParallel("proptest-regressions")`; its exact
path depends on `source_file`, with fallback when no `lib.rs`/`main.rs`
ancestor exists. A custom runner must configure source identity/persistence.
These files store **seeds**, not a stable serialized history; earlier seeds replay
before novel cases. Review and retain intended regression files with the test
change, using an owned writable output path when the admitted source is read-only.
Persistence warnings or missing outputs do not prove a failure was saved.
Sources: [persistence guide](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/book/src/proptest/failure-persistence.md)
and [file persistence API](https://docs.rs/proptest/latest/proptest/test_runner/enum.FileFailurePersistence.html).

For each failure retain original and minimized concrete initial state/actions,
input identities/ordinals, observed results/effects and violated requirement.
Also bind seed/RNG algorithm, both crate versions, strategies/configuration,
source/dirty-tree/lock/feature/target/compiler identity, policy, oracle and raw
result bytes. Capture the original attempt before reduction obscures it; verbose
action debug output alone is not a complete receiver/effect trace.
Independently replay the concrete minimized history from clean state, then keep
a named regression with analytically reviewed expectations. A strategy, version,
RNG or collection-order change can alter the same seed's meaning; persistence and
raw traces complement this regression and cannot replace it.

## Diagnose incomplete or misleading results

| Observation | Next discriminating check |
| --- | --- |
| Local/global rejection or flat-map exhaustion | Inspect rejection reasons, state-dependent IDs and shrink preconditions; construct valid neighbors jointly. Do not filter an expected API refusal or silently increase budgets. |
| Timeout before a receiver call | Separate child startup/setup from actual application behavior. Check resource limits and cleanup. An environment timeout is not evidence of a product semantic defect. |
| Failure changes during shrinking/replay | Check shared state, unordered collections, real clocks/randomness, external I/O and incomplete resets. Preserve the original; nondeterministic replay remains unresolved. |
| Green result with few useful actions | Reconcile actual receiver calls and accept/refuse, current/stale, empty/nonempty partitions. Weights, case count and the internal seen counter do not prove semantic reachability. |
| Generated history looks valid but comparison is wrong | Check pre-state/post-state confusion, wrong delivered arguments/order, copied production logic and omitted return/effect checks. Challenge the comparator with a fixed witness. |
| Missing persistence or unexpectedly changed bounds | Inspect source path, output permissions, effective `PROPTEST_*` environment and file contents; retain concrete inputs independently. |

Use [TestError](https://docs.rs/proptest/latest/proptest/test_runner/enum.TestError.html)
to distinguish abort from a failing case. Preserve selected/discovered/executed
counts, successful cases, rejections, shrinks/replays, first failure, skipped or
cancelled work and cleanup status. One outer libtest pass does not automatically
expose all these counts; add bounded receiver/partition evidence where needed.

## Qualify the harness and close the decision

Apply [harness qualification](harness-qualification.md) to driver/oracle changes.
Use a controlled, task-authorized fault at the real route: stale identity accepted,
budget incorrectly refunded, duplicate settlement, omitted refusal, wrong clock
domain or suppressed comparison. Require intended assertion, healthy neighbor
and failing exit propagation; unrelated setup failure earns no semantic evidence.

Before reporting success, answer in the existing task:

- Which requirement, real receiver, independent oracle and selected domain ran?
- Which generated/concrete histories and semantic partitions actually reached it?
- Which effective bounds, dependency graph, environment and candidate were used?
- Which healthy/fault controls, original/minimized replay and cleanup completed?
- Where are named regressions, persistence files and bounded raw effect traces?
- Which rejected, unreachable, timed-out, concurrent/native or held obligations remain?

Report observed bounded behavior and remaining gaps. Agreement with a sequential
model supplies no thread-schedule, weak-memory, native-Windows or release proof,
and documentation/admission preparation alone supplies no execution result.

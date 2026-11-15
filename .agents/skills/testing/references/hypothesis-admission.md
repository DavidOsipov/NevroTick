# Generated admission checks with Hypothesis

Use for Python receipt/policy readers with named admission contracts. Read
[property testing](../qa-property-based-testing/SKILL.md),
[harness qualification](harness-qualification.md) and the [contract](../CONTRACT.md).
For alternatives and a bounded pilot, use [generated-testing selection](generated-testing-selection.md).

Source review: **2026-11-15 UTC**. Official latest documentation and release
identify **Hypothesis 6.170.15**; tag `v6.170.15` resolves to
`9ab0be61aaaa27396fc1bbf4b34e7925cc68d1e2`. Documentation observation,
not an installed/admitted package pin. Inspect actual Python, dependency,
pytest/plugin and runner identities before use. Follow
[configured tooling](configured-tooling.md#additional-rust-analysis): no automatic
installation, adopted CI profile or `check-analysis.py --tool hypothesis` lane.
Cargo's 14-day registry cooldown supplies no Python package-admission policy.

For a missing interpreter, package or runner, follow
[isolated Python setup](generated-testing-setup.md#hypothesis-in-an-owned-python-environment).
Covers prerequisites, compatible wheels, exact dependency closure,
installation commands and readiness without extending the unrelated mypy environment.

## Contents

- [Choose the smallest useful method](#choose-the-smallest-useful-method)
- [Build strategies from independent contracts](#build-strategies-from-independent-contracts)
- [Worked example: physical files and a real reader](#worked-example-physical-files-and-a-real-reader)
- [Exercise semantic admission and forbidden effects](#exercise-semantic-admission-and-forbidden-effects)
- [Control effective settings and budgets](#control-effective-settings-and-budgets)
- [Select, execute and observe](#select-execute-and-observe)
- [Use state machines for actual histories](#use-state-machines-for-actual-histories)
- [Retain and replay failures](#retain-and-replay-failures)
- [Triage and report](#triage-and-report)
- [Primary sources](#primary-sources)

## Choose the smallest useful method

| Evidence question | Method and stopping condition |
| --- | --- |
| A small finite list of statuses, boundary neighbors or known regressions | Ordinary parameterized tests or `unittest.subTest`; enumerate required rows. No generator dependency needed. |
| One input has many meaningful combinations, with an independent expected result | `@given` and bounded strategies; each invocation starts clean. Stop at agreed cases/time budget; retain unreached partitions. |
| A few correlated operations fit one clear scenario | A generated tuple/list of operations or `@given(st.data())` with labeled draws can suffice. Do not introduce a machine merely because files change. |
| Replacement, invalidation, retry or cached admission changes later outcomes | `RuleBasedStateMachine`; model state and generated operation history separately from the reader's implementation. |
| Malformed bytes must fail at a raw boundary | A separate byte/schema-negative property, preserving the intended refusal cause. Valid-domain generators alone cannot establish this. |

Hypothesis explores and shrinks sampled cases. A pass is neither exhaustive
proof nor native Windows qualification. Keep deterministic analytic anchors and
named semantic regressions even when generation is available. The official
[stateful guide](https://hypothesis.readthedocs.io/en/latest/stateful.html)
explains when ordinary `@given` tests suffice.

## Build strategies from independent contracts

Before coding, name requirement, receiving function, admitted domain,
preconditions, exact expected facts and forbidden effects. Trace fixture bytes
through the real call to the assertion. Define a plausible wrong implementation
the property must distinguish; a round trip through two production helpers
can preserve the same defect.

Use [strategy primitives and composition](https://hypothesis.readthedocs.io/en/latest/reference/strategies.html)
deliberately:

- Bound integers, text/binary lengths, list sizes and nesting explicitly. Choose
  `sampled_from` for independently reviewed enums; include meaningful endpoints.
- Use `@st.composite` to draw a parent first and derive dependent fields: source
  identity, graph, lane roster, raw-record references, counts and parent paths.
  `st.builds`/`.map` suit simple constructors; `st.data()` suits draws depending
  on an earlier observation. Do not call `strategy.example()` inside a test.
- Use `st.lists(..., unique=True)` only where uniqueness is a valid-domain rule.
  Duplicate-rejection properties must deliberately construct duplicates. Avoid
  mutable shared values in `st.just`; allocate fresh case data.
- Construct useful cases directly. Heavy `.filter`/`assume` rejection wastes
  budget and may remove the defect class. Never filter by the production reader,
  validator or acceptance result; never `assume` away a malformed case under test.

Separate valid typed inputs, invalid raw representations and invalidated context.
For typed postconditions, independently choose legal values, pass them through
the real checked constructor, then exercise the consumer. Type annotations and
`from_type` do not encode all semantic invariants. For raw rejection, include
wrong field types, missing/extra keys, duplicate JSON keys, malformed UTF-8,
nonfinite numbers and count booleans where exact integers are required. JSON
object generation cannot preserve duplicate keys; construct those raw bytes.
For context changes, vary current authority/generation separately from an
unchanged immutable value. Production validation helpers are not oracles.

Keep healthy/invalid partitions distinguishable, with concrete witnesses for
rare antecedents. Use `event()` for bounded labels after observing the relevant behavior;
use `note()` for nonsecret input facts. Recorded draws or high case
counts do not prove required branch execution.

## Worked example: physical files and a real reader

Example targets existing `read_json` in
[`verify-engine-analysis.py`](../../../../scripts/verify-engine-analysis.py), using
the import pattern in its [current tests](../../../../scripts/tests/test_verify_engine_analysis.py).
Checks exact object decoding and non-object refusal with valid content hashes.
The synthetic `count`/`ids` relationship demonstrates correlated generation;
this raw reader does **not** establish their semantics. Full receipt
admission needs the next section's real semantic reader and consumer checks.

Illustrative test-file location: `scripts/tests/test_hypothesis_admission.py`;
this reference adopts neither file nor dependency. Review actual import
closure before authorized execution. Example limits are tutorial choices,
not product policy. The disabled per-case deadline requires a separately bounded
runner. No reader, pytest or Hypothesis ran during this documentation refresh.

```python
import hashlib
import importlib.util
import json
from pathlib import Path
from tempfile import TemporaryDirectory

import pytest
from hypothesis import Phase, event, example, given, settings, strategies as st

ROOT = Path(__file__).resolve().parents[2]  # scripts/tests/<test_file>.py
SPEC = importlib.util.spec_from_file_location(
    "hypothesis_engine_reader", ROOT / "scripts/verify-engine-analysis.py"
)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("Real reader module could not be loaded")
ENGINE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ENGINE)


def read_real(path: Path, expected_digest: str) -> dict:
    # Adapter forwards to the actual bounded reader; no mock of this boundary.
    return ENGINE.read_json(
        path, maximum=4096, expected_sha256=expected_digest
    )


@st.composite
def counted_records(draw):
    count = draw(st.integers(min_value=0, max_value=8))
    ids = draw(st.lists(st.integers(min_value=0, max_value=65535),
                        min_size=count, max_size=count, unique=True))
    return {"count": count, "ids": ids}


ADMISSION = settings(
    max_examples=40, deadline=None, derandomize=True, database=None,
    suppress_health_check=(), print_blob=True,
    phases=(Phase.explicit, Phase.generate, Phase.shrink),
)


@ADMISSION
@example(record={"count": 0, "ids": []})
@given(record=counted_records())
def test_physical_json_boundary(record):
    # Runs for every generated, shrinking, explicit and replayed invocation.
    with TemporaryDirectory(prefix="admission-case-") as temporary:
        path = Path(temporary).resolve() / "record.json"
        payload = json.dumps(record, sort_keys=True,
                             separators=(",", ":")).encode("utf-8")
        path.write_bytes(payload)
        actual = read_real(path, hashlib.sha256(payload).hexdigest())
        if actual != record:
            raise AssertionError("Decoded object differs from independent input")
        event("object accepted exactly")

        # Well-formed JSON, but forbidden top-level shape; refresh its hash.
        invalid = json.dumps(record["ids"]).encode("utf-8")
        path.write_bytes(invalid)
        with pytest.raises(ENGINE.Incomplete, match=r"must be an object"):
            read_real(path, hashlib.sha256(invalid).hexdigest())
        event("top-level array refused")
        if path.read_bytes() != invalid:
            raise AssertionError("Reader changed the rejected fixture")
```

`TemporaryDirectory` is inside the property; each shrink/replay gets fresh
files. Pytest function-scoped fixtures and ordinary unittest `setUp` run once
per test invocation, not per generated case. A reusable fixture may provide
an immutable factory; the property must invoke it to allocate/reset case state.
Register cleanup immediately, including partial setup. Observe owned effects and
resources before cleanup masks them. See the
[fixture health check](https://hypothesis.readthedocs.io/en/latest/reference/api.html#hypothesis.HealthCheck.function_scoped_fixture).
Example omits artifact persistence: an evidence harness must capture bounded
original failing bytes before leaving this context, re-raise the failure, and
retain the final minimized case separately as below.

## Exercise semantic admission and forbidden effects

Build a valid physical receipt tree from independently reviewed schema/contract
facts. Change one semantic fact, rewrite the affected leaf, recompute its digest,
then rewrite and rehash each affected parent up to the reader's actual argument.
Preserve unrelated integrity and custody. A mismatched digest must be a separate
test: early hash refusal otherwise conceals a missing semantic check. Retain
the valid neighboring fixture and exact bytes delivered to both calls.
For the named development-evidence reader, the real adapter call is
`ENGINE.admit_source_policy_development(binding, identity)`: supply the fixture's
top-level path/digest in `binding`, and the independently expected current source
identity separately. Do not replace either with the reader's computed result.

| Contract | Perturbation and independent expected result |
| --- | --- |
| Current source/graph/policy binding | Substitute one identity while maintaining valid hashes; require the named currentness refusal. |
| Complete lane roster | Omit, duplicate or substitute a lane while preserving plausible totals; compare exact lane identities, status and graph. |
| Actual execution in every selected graph | Default **and EP1** ordinary/optimized/doctest lanes: omit, duplicate or substitute a test, or swap raw logs between graphs. Require that lane's independently frozen roster. |
| Terminal custody | Mark success with missing observer join, terminal failure or incomplete output; refuse before downstream Session/runner/effect construction. |
| Paths and bounds | Wrong parent, linked/stale reference or selected size boundary; require that rule's refusal and retain an accepted neighbor. |

Use `Counter(actual_names) == Counter(expected_names)` or sorted-list equality
when order is irrelevant; both preserve multiplicity. Set equality loses
duplicates; counts lose identity. If order is contractual, compare sequences.
Freeze separate graph/lane expectations from reviewed requirements and actual
discovery evidence; never copy the production roster helper as expected truth.
The missing-EP1-execution-validation class associated with
`admit_source_policy_development` is a historical regression target, not a claim
that today's reader is defective or a license to reuse historical test counts.

Valid input requires exact admitted identities/result facts. Invalid input must
raise the expected exception with the required reason, preserving valid state and
excluding protected effects. Match diagnostic meaning/context; exact wording is
required only when contractual. `except Exception: pass`, an arbitrary exception,
or setup/import failure cannot satisfy rejection.
[`pytest.raises`](https://docs.pytest.org/en/stable/how-to/assert.html#matching-exception-messages)
accepts subclasses; inspect `excinfo.type` if exact exception identity is
contractual. Its `match` parameter is a regex, not literal-string equality.

For refusal-before-effect, exercise the actual caller and reader.
A qualified `execute_source_policy_profile` fixture can observe the
`COMMON.Session` construction seam and require zero calls on malformed evidence.
Keep admission real; a spy or fail-fast sentinel belongs at the forbidden effect,
not around the claimed validation. Retain a healthy control proving the
same caller can reach the seam. These controls establish ordering, not
actual analyzer/native execution. Inspect resulting state before fixture teardown.

## Control effective settings and budgets

[Settings inheritance](https://hypothesis.readthedocs.io/en/latest/tutorial/settings.html)
is fixed at construction. Load/register an approved profile
before importing decorated tests; changing profiles later does not change
existing objects. Record `settings.get_current_profile_name()`, `settings()` and
the explicit per-test settings object, plus runner/CLI overrides. Active
profile alone does not reveal a decorator's effective configuration.

At 6.170.15, `CI` or recognized vendor variables can automatically select `ci`.
Its defaults include `derandomize=True`, `database=None`, `deadline=None`,
`print_blob=True` and suppression of `HealthCheck.too_slow`; `default` has a
200 ms deadline and no suppressions. Profiles can be replaced. Inspect actual
values rather than inferring them from a machine's label. Verified against
[pinned settings source](https://github.com/HypothesisWorks/hypothesis/blob/9ab0be61aaaa27396fc1bbf4b34e7925cc68d1e2/hypothesis/src/hypothesis/_settings.py).

| Setting/budget | What to bound and record |
| --- | --- |
| `max_examples` | Satisfying generated cases, not total calls. Explicit cases, discarded draws, reuse, shrinking, explanation and final replay need separate accounting. Small finite domains can exhaust early. |
| `stateful_step_count` | Bound history length together with case count; record actual rules reached, including initialization. No rule-coverage guarantee. |
| `deadline` | Soft per-case timing check, in milliseconds or `timedelta`; `None` disables it. It cannot interrupt blocking I/O or cap total campaign time. |
| External limits | Owned runner wall time, per-operation I/O time, memory, file count/bytes, cleanup and failure-artifact budget. No public `max_shrinks` setting should be invented. |
| `phases` | Retain `explicit` for `@example`, `generate` for new cases, `shrink` for reduction; database replay uses `reuse`. Report omitted phases. |
| Health checks | Record all effective suppressions. Fix filtering, giant inputs and state leaks; suppress only a named justified check, not all checks to obtain green. |

See [actual invocation counts](https://hypothesis.readthedocs.io/en/latest/explanation/test-case-count.html).
One property invocation can make several reader calls, as above;
preserve both counts when relevant. Timeout/cancellation is incomplete,
not evidence that the remaining cases would pass.

`derandomize=True` requires no database; explicitly supplying a non-None database
with it is invalid. For an ordinary decorated property, `@seed` overrides
derandomization and sets the decorated function's database to `None`. Keep
`database=None` explicit in a stateful seeded replay: the stateful adapter forwards
the class's seed/reproduction attributes, not its decorated settings object.
For an admitted reusable cache, explicitly choose `derandomize=False`, an owned
`DirectoryBasedExampleDatabase(path)` and `Phase.reuse`, with a clear persistence
and cleanup policy. Database state is a search aid, not permanent regression
truth. See the [seed source](https://hypothesis.readthedocs.io/en/latest/_modules/hypothesis/core.html#seed)
and [database API](https://hypothesis.readthedocs.io/en/latest/reference/api.html#database).

## Select, execute and observe

Only after admitting the interpreter, dependencies, import closure and bounded
runner, adapt these illustrative pytest commands to the actual test file:

```sh
python3 -B -m pytest scripts/tests/test_hypothesis_admission.py --collect-only -q
python3 -B -m pytest scripts/tests/test_hypothesis_admission.py::test_physical_json_boundary --hypothesis-show-statistics -q
```

The [pytest plugin](https://hypothesis.readthedocs.io/en/latest/reference/integrations.html#the-hypothesis-pytest-plugin)
provides `--hypothesis-profile=<registered-name>`, `--hypothesis-seed=<integer>`
and `--hypothesis-verbosity=verbose`; use each for a named need and retain its
effect on settings. `-m hypothesis` selects Hypothesis-marked tests only when
the plugin is active. [Explicit node selection](https://docs.pytest.org/en/stable/how-to/usage.html)
simplifies auditing a small pilot. Collection alone executes no property;
zero selected tests, skips and unexpected plugin absence remain gaps.

Record statistics by phase, passing/failing/invalid cases and stopping reason.
Events help inspect partition reachability, but event percentages are not source
coverage or a statistically justified defect-detection probability. An absent
selected partition needs a concrete witness or remains untested.

If the selected Python consumer must work under optimization, also
exercise that same boundary under `python3 -O -B -m pytest ...` in its admitted
lane, retaining ordinary and optimized results separately. Python
[`-O` removes ordinary `assert` statements](https://docs.python.org/3/using/cmdline.html#cmdoption-O).
Use explicit checks/raises or `unittest` assertion methods in non-rewritten
helpers; qualify the assertion/control through the chosen runner. Pytest's
rewritten test assertions do not restore production assertions. `-OO` is a
separate requirement only when selected; do not infer it from `-O` success.

## Use state machines for actual histories

For a real cached/replacement reader, first write the model: current receipt
generation, independent accepted identities, retired references and allowed
effects. Specify actions such as write valid receipt → admit → replace bytes →
attempt old binding → admit new binding. Keep state through one history;
reset the entire physical tree, cache and observer for the next history.

The [stateful guide](https://hypothesis.readthedocs.io/en/latest/stateful.html)
and [pinned implementation](https://github.com/HypothesisWorks/hypothesis/blob/9ab0be61aaaa27396fc1bbf4b34e7925cc68d1e2/hypothesis/src/hypothesis/stateful.py)
define the mechanics; import these names from `hypothesis.stateful`:

- Subclass `RuleBasedStateMachine`, call `super().__init__()`, then create owned
  resources and a separate model. Register cleanup before the next setup step;
  constructor failure precedes the runner's normal teardown protection.
- A `Bundle("receipts")` stores results from `@rule(target=receipts, ...)` for
  later rules taking `receipt=receipts`. Bundle contents are references, not
  independent expected truth or automatically fresh snapshots. Prefer immutable
  IDs and keep the model's identity/generation map separately.
- `consumes(bundle)` removes a drawn value from that bundle. This does not retire
  the application's object or prove invalidation; assert that effect separately.
  Retain stale references in another model collection when testing their refusal.
- `@initialize(...)` runs before ordinary rules; multiple initializers have no
  fixed order. Put dependent setup in one initializer. Initializers may populate
  bundles but cannot have `@precondition`.
- `@precondition(lambda self: ...)` controls rule eligibility using model state,
  not bundle access. Avoid `assume` inside most rules. Do not exclude intentionally
  invalid actions: give them rules asserting the required refusal. Ensure useful
  progress remains possible; a state with no enabled rule is a model error.
- `@invariant()` checks after rules; by default waits until initialization
  finishes. Use `check_during_init=True` only for invariants meaningful then.
  Keep per-operation assertions too, and observe forbidden effects before cleanup.
- `teardown()` releases resources after each history, including failed/shrunk
  runs. Must tolerate partial initialization. Assert lifecycle obligations
  before deleting the fixture; preserve primary and cleanup failures separately.

Expose an implemented machine as `TestAdmission = AdmissionMachine.TestCase`,
then assign `TestAdmission.settings = settings(max_examples=40,
stateful_step_count=20, deadline=None, suppress_health_check=(),
derandomize=True, database=None)`, or use
`run_state_machine_as_test(AdmissionMachine, settings=the_reviewed_settings)`.
Alternative runner recipes, not two required campaigns. Inspect
effective settings: this version's generated `.TestCase` otherwise sets
`deadline=None` and suppresses **all** health checks. Record actual enabled rules,
reached transitions and missing antecedents; increasing steps does not establish
coverage of a history the model cannot generate.

## Retain and replay failures

| Mechanism | Correct use and limit |
| --- | --- |
| Named fixture / explicit operation trace | Permanent regression. Reconstruct real files and state, call the same entry point and assert the original reason/effect. |
| `@example(...)` | Permanent concrete argument values on a `@given` property, with `Phase.explicit` enabled. Explicit examples do not shrink or count toward `max_examples`; they do not encode a state-machine history automatically. |
| `@seed(value)` | Reproduce a search under the same Python, Hypothesis, test/strategy and environment; overrides `derandomize`. Strategy/version changes and unmanaged nondeterminism can change the outcome. |
| `--hypothesis-seed=<integer>` | Select the search seed only when the effective test has `derandomize=False` and no decorator seed. The explicit tutorial settings above therefore need adjustment for this option; it does not override them. |
| `@reproduce_failure(version, blob)` | Temporary replay of the reported internal input, using the matching Hypothesis version. It expects failure and may raise `DidNotReproduce` after a repair. Do not retain it as the regression. |
| Example database | Convenient reuse, possibly invalidated by code/version/key changes. Do not depend on cache presence for correctness. |

Use [replay guidance](https://hypothesis.readthedocs.io/en/latest/tutorial/replaying-failures.html),
with the API's `Phase.reuse` for database replay. The reviewed
[random selection implementation](https://github.com/HypothesisWorks/hypothesis/blob/9ab0be61aaaa27396fc1bbf4b34e7925cc68d1e2/hypothesis/src/hypothesis/core.py)
uses this precedence: decorator seed, then `derandomize`'s test-identity seed,
then the global seed assigned by the
[pytest option](https://github.com/HypothesisWorks/hypothesis/blob/9ab0be61aaaa27396fc1bbf4b34e7925cc68d1e2/hypothesis/src/_hypothesis_pytestplugin.py).
Use `@seed` or construct effective settings with `derandomize=False` for a
CLI-seeded replay. Loading a later profile cannot replace an already constructed
explicit test settings object. Record the method and its effective settings;
do not assume a supplied CLI seed was used.

Retain **original** concrete
failing files/operation sequence when first observed, before shrinking and
temporary-directory cleanup; then preserve minimized version separately.
Record logical values, exact bytes/digests, parent relationships, operation order,
rejection cause, source/policy/strategy identities, effective settings and available
seed/blob. Bound and redact retained artifacts. `note()` normally reports the
minimal failure, so it alone does not preserve the original case.
For temporary stateful replay, apply `@seed` or the emitted `@reproduce_failure`
to the machine class; the stateful runner forwards them. A seed need not appear
in an ordinary failure report, so preserve the concrete case regardless.

Replay the minimized concrete case alone from clean state, then the original
and healthy neighbor. A printed state-machine trace includes explicit calls;
turn it into a deterministic regression using actual fixture/reader operations
and assertions, preserving relevant invariant checks and `try/finally` cleanup.
Do not assume directly calling rule methods enforces generator preconditions.
If reset/replay changes the failure, retain both observations and diagnose the
harness or nondeterminism before declaring a product defect.

## Triage and report

| Observation | Next bounded action |
| --- | --- |
| `FailedHealthCheck` / `Unsatisfiable` | Check filter rate, input size, slow generation and fixture lifetime. Repair the strategy/reset; a suppression needs a named reason and evidence. |
| `FlakyFailure` / `FlakyStrategyDefinition` | Preserve first failure; inspect global state, cache/filesystem reset, clocks, randomness and draw order. Replay independently. Do not retry until green or blanket-suppress. |
| `DeadlineExceeded` | Distinguish product slowness from generation/setup/shared-host noise; measure the selected operation. Review a changed deadline with the external job budget. |
| Expected refusal with wrong cause | Determine which earlier gate was reached. Repair fixture integrity/setup; do not count incidental failure as semantic validation. |
| Shrunk input no longer reaches the intended rule | Keep the original; fix dependent generation or the assertion and replay both. Shrink size is not causal proof. |
| Import, discovery, setup or cleanup failure | Harness/environment evidence only; intended product property remains incomplete. |

See [flaky-failure mechanics](https://hypothesis.readthedocs.io/en/latest/tutorial/flaky.html)
and [failure triage](../qa-failure-triage/SKILL.md). Qualify sensitivity with a
requirement-linked omitted-guard or forged-roster fault, a healthy control and
restoration, through the selected runner. Require the intended assertion and
failure exit propagation. An unrelated import failure is not a successful fault
control; a mutant surviving because the fixture never reaches it limits the claim.

Report in existing task/evidence chain:

- Requirement, actual reader/caller, source/configuration identity and independent
  oracle; raw-reader, semantic-admission and downstream-effect scope separately.
- Python/Hypothesis/pytest/plugin identities, command/node selection, effective
  profile/settings, phases, budgets and any health-check suppression.
- Actual discovered/executed/skipped cases, reader calls, reached partitions or
  transitions, stop reason, native exit, cleanup and fault/healthy controls.
- Original/minimized regression locations, successful clean replay, retained
  failure cause, affected interpretation and unresolved environment/coverage gaps.

Documentation/source/AST checks establish no Hypothesis runtime result. A bounded
property pass establishes only its exercised domain and candidate; it does not
admit dependencies, lift execution holds, qualify native behavior or close a
full-domain proof obligation.

## Primary sources

- [Hypothesis 6.170.15 release](https://github.com/HypothesisWorks/hypothesis/releases/tag/v6.170.15) and [reviewed commit](https://github.com/HypothesisWorks/hypothesis/tree/9ab0be61aaaa27396fc1bbf4b34e7925cc68d1e2).
- [API, settings, phases and health checks](https://hypothesis.readthedocs.io/en/latest/reference/api.html); [strategies](https://hypothesis.readthedocs.io/en/latest/reference/strategies.html).
- [Settings tutorial](https://hypothesis.readthedocs.io/en/latest/tutorial/settings.html), [invocation-count explanation](https://hypothesis.readthedocs.io/en/latest/explanation/test-case-count.html), [pytest integration/statistics](https://hypothesis.readthedocs.io/en/latest/reference/integrations.html#the-hypothesis-pytest-plugin).
- [Stateful guide](https://hypothesis.readthedocs.io/en/latest/stateful.html), [replay](https://hypothesis.readthedocs.io/en/latest/tutorial/replaying-failures.html), [flakiness](https://hypothesis.readthedocs.io/en/latest/tutorial/flaky.html).
- [Pinned settings implementation](https://github.com/HypothesisWorks/hypothesis/blob/9ab0be61aaaa27396fc1bbf4b34e7925cc68d1e2/hypothesis/src/hypothesis/_settings.py), [stateful implementation](https://github.com/HypothesisWorks/hypothesis/blob/9ab0be61aaaa27396fc1bbf4b34e7925cc68d1e2/hypothesis/src/hypothesis/stateful.py), [seed/reproduction implementation](https://hypothesis.readthedocs.io/en/latest/_modules/hypothesis/core.html#seed).

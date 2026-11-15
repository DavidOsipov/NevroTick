# Generated admission checks with Hypothesis

Use for Python receipt/policy readers with named semantic admission contracts.
Read [property testing](../qa-property-based-testing/SKILL.md),
[harness qualification](harness-qualification.md) and the [contract](../CONTRACT.md).
Source review: **2026-11-15**, upstream Hypothesis **6.170.15** documentation.
Installed/admitted availability unestablished. Inspect actual pinned
Python/dependency/runner lane; follow existing admission policy before adoption.
No package installation, CLI profile or campaign authority follows.

## Generate the contract, then call the real reader

Prefer `@given` for independent receipt inputs. Use `RuleBasedStateMachine` only
for real histories: replacement, invalidation, retry, cached admission. Its
`@rule` actions and `@invariant` assertions need separate model; expose `.TestCase`
or use `run_state_machine_as_test(..., settings=...)` through adopted runner.
Preserve state inside one history; reset between histories.

Bound valid generator by independently reviewed schema and semantics: correlated
source/graph/policy identities, exact lane/test rosters, raw records, terminal
custody, path relationships. Never label inputs with production reader or its
validators. Independently define exact expected acceptance/result identity; retain
explicit healthy examples and boundary neighbors. Avoid arbitrary dictionaries
mostly rejected by syntax before the semantic contract.

Create physical files in fresh `TemporaryDirectory` context **within each
property invocation**, including shrink/replay; call real receiving reader.
Restrict necessary path/configuration patches to owned fixtures; never stub
claimed admission boundary. Pytest function fixtures and ordinary unittest setup
do not reset per generated example. Observe protected effects before cleanup;
register cleanup for partial setup too.

Start semantic-negative cases from valid fixtures; change one contract fact,
recompute every affected content hash and parent reference, preserve integrity.
Keep corrupt-hash cases separate: early digest rejection can hide missing semantic
checks. Retain intended bytes, paths, actual call inputs.

## Challenge meaning beyond successful exits and digests

| Selected obligation | Generated perturbation and independent expectation |
| --- | --- |
| Current identity | Change source/helper, policy or graph binding; retain unrelated fields and valid integrity. Require named currentness refusal. |
| Complete lane roster | Omit, duplicate or substitute lane; preserve plausible totals. Require exact lane identity, status, feature graph. |
| Actual test execution | Default **and EP1**: remove/duplicate/substitute test or swap graph logs; preserve counts. Require exact independently frozen roster equality with multiplicity, per execution lane. |
| Terminal custody | Forge success with missing join, terminal failure or incomplete output; reject before downstream Session/runner/effect construction. |
| Paths and bounds | Wrong receipt parent, selected size boundary or stale raw reference; require applicable custody/bounds refusal and valid neighbor. |

Readers such as `admit_source_policy_development` in
[`verify-engine-analysis.py`](../../../../scripts/verify-engine-analysis.py) illustrate
historical class: valid exit codes and hashes with missing EP1 execution validation.
Retain class regression; no claim current reader is broken. Derive current expected
rosters from independently reviewed evidence, never historical counts or reader's
own roster helper.

Invalid properties assert expected rejection type/reason and absence of protected
effects before rejection. Broad `except Exception: pass`, any-error assertions or
`assume` removing invalid inputs establish no contract. Valid inputs require exact
admitted identities/result facts, beyond no error. Requirement-linked
omitted-guard/forged-roster fault plus healthy control qualifies sensitivity and
exit propagation. Setup/import failures do not count.

## Budgets, settings and durable replay

Record effective settings/profile; upstream may auto-select CI profile. Bound
`max_examples` and history `stateful_step_count`. Separately account discarded
draws, explicit examples, replay, shrinking; `max_examples` is no total invocation
cap. Require nonempty actual executions per selected valid/invalid partition;
unreachable cases remain gaps.

`deadline`: per-case soft timing limit, milliseconds; `None` disables. Keep
independent total/IO/cleanup budget. Fix excessive filtering and fixture leaks;
suppress only justified named health checks. Never suppress all to hide
nondeterminism or per-example isolation failures. Record inherited suppressions.

`derandomize=True` implies `database=None`; `@seed` disables database use and
overrides derandomization. Determinism depends on Python/Hypothesis/test identity.
Database reuse: owned store, compatible explicit settings, `Phase.reuse`; retain
generation/shrinking phases when claiming them. Database entries are caches,
possibly invalidated by code/version changes.

Retain original/minimized concrete files or command history, failure reason,
available seed, Python/Hypothesis/strategy versions, settings, source/policy/raw
result identities. Replay from clean state. Keep permanent named fixtures or
`@example` values (explicit examples do not shrink); stateful traces as explicit
regressions. `@reproduce_failure` blobs: temporary, version-specific replay aids.
Seed/blob/database-only storage cannot replace concrete regressions.

## Primary sources

- [API/settings and health checks](https://hypothesis.readthedocs.io/en/latest/reference/api.html) and [settings implementation](https://hypothesis.readthedocs.io/en/latest/_modules/hypothesis/_settings.html).
- [Stateful testing](https://hypothesis.readthedocs.io/en/latest/stateful.html).
- [Replay guidance](https://hypothesis.readthedocs.io/en/latest/tutorial/replaying-failures.html) and [seed implementation](https://hypothesis.readthedocs.io/en/latest/_modules/hypothesis/core.html#seed).

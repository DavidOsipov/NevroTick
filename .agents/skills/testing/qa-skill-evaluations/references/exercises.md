# Reusable skill exercises

Select cases that challenge the changed instruction. These examples are original
instructional fixtures, not application code, product constants or qualification.
The coordinator retains grading materials outside the task agent's supplied
directory. Reuse the normal model and tools; do not install a framework for this.

## Executable Rust state-update exercise

Copy [the task](../assets/atomic-history/task.md) and
[the candidate](../assets/atomic-history/history.rs) into a fresh owned directory.
Give the task agent those copies and the selected implementation skill/reference.
Permit edits and tests only in that directory. Do not supply this grading section,
the grader source, a suggested fix or a previous answer. Freeze original bytes.

The coordinator first inspects the candidate, compiler and local execution rights,
then uses [the fixture grader](../scripts/grade_history.py):

```sh
python3 -B .agents/skills/testing/qa-skill-evaluations/scripts/grade_history.py \
  --candidate /absolute/owned-trial/history.rs \
  --out-dir /absolute/owned-results/new-directory \
  --rustc /absolute/path/to/admitted/rustc
```

The output directory must not exist. The grader copies the candidate, builds a
standard-library-only host Rust test binary and runs exactly its named five tests.
It does not use Cargo, access the network or run product tests. Its finite process
timeouts are not a sandbox, memory quota or safe execution of hostile submitted
code; source inspection and an appropriate execution environment remain required.
Use the toolchain already admitted by the task; missing compiler means incomplete.

Expected evaluator observations: the supplied candidate compiles and fails the
full-history rejection check; healthy append/rollover and boundary values pass.
A repaired candidate must pass all five independent checks. A reject-everything
variant must fail healthy controls. The runner rejects compilation errors, zero
selected tests, ignored tests and setup failures. Its exit/count checks rely on
libtest output from the same process as the inspected candidate; they do not
authenticate that output against adversarial process/report manipulation. The
coordinator must reject candidate changes that bypass tests, terminate the test
process or forge reporting, and inspect the actual diff as well. A matching
summary alone cannot discharge that review. This exercise's success does not establish
production API safety, concurrency, allocation bounds or Windows behavior.

The task agent should add its own meaningful regression and retain a healthy
control; the coordinator's tests do not substitute for that requested deliverable.
After grading, check the candidate's own test output and claimed failure cause.
Never present supplied-example success as evidence on unseen tasks.

## Numerical review exercise

Give the agent an explicit coefficient contract, a proposed ordered binary64
calculation and an independently authored set of inputs. Include a constant
series, unequal weights, a legitimate nonconstant line and representable neighbors
around a declared computed cutoff. Ask for a bounded review and executable math
evidence before implementation. The task must state whether constant preservation,
underflow behavior and computed versus exact-real thresholds are contractual.

The evaluator derives rational expectations separately and inspects the proposed
operation sequence. Grade whether the agent distinguishes arithmetic correctness,
conditioning and physical accuracy; detects a violated invariant without changing
tolerance after observing output; preserves healthy controls; and avoids invented
unreachable overflow tests. Use
[numerical kernels](../../../implementation/implement-rust/references/numerical-kernels.md)
for the method. Its published example is a practice case: reserve a different,
independently checked input family for an improvement comparison.

## Harness and mutation review exercises

For harness work, supply a small driver, scenario contract and comparator. Include
healthy inputs and a named malformed input whose exact length is contractual.
Ask the agent to assess whether a successful run proves that scenario and to repair
the test if authorized. The evaluator perturbs bytes, event cardinality/order or
comparison execution and checks actual failures using
[harness qualification](../../references/harness-qualification.md). Keep a variant
whose malformed result category stays unchanged while its actual input changes.

For mutation review, supply two tiny pure functions, their explicit input domains
and proposed mutant dispositions. Ask for proofs or counterexamples without
revealing the intended labels. Include a helper with a wider raw-input contract
than its current callers and a separate checked-input function. Grade against
independently derived domain and effect arguments, following
[mutation equivalence](../../references/mutation-equivalence.md). Equivalent does
not mean untested; a missing test is not justified by present public reachability.

## Minimal evaluation record

Use the existing task record; no new report hierarchy is necessary:

| Field | Record |
| --- | --- |
| Identity | Skill/reference, prompt, fixture, candidate, grader and tool versions/hashes |
| Setup | Model/settings if exposed, context mode, supplied inputs, permitted actions and exposure limits |
| Cases | Selected and executed tasks, healthy controls, intentional faults and independent expected facts |
| Outcomes | Actual artifacts/tests, first failures, source review, setup/grader/task classification and remaining gaps |
| Comparison | Old/new instruction identities, comparable conditions, all attempts, budget and any confounders |

Deterministic grading plus scoped code review is usually sufficient. Use a
reviewer for semantic judgments, with an explicit rubric and an unknown outcome
when evidence is missing. Several agreeing reviewers are not independent runtime
executions. This follows the outcome-grading and isolation guidance in
[Anthropic's agent-evaluation research](https://www.anthropic.com/research/demystifying-evals-for-ai-agents);
the fixtures and criteria above are project-authored examples.

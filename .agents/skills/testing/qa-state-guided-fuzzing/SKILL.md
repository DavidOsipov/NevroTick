---
name: "qa-state-guided-fuzzing"
description: "Design or run bounded NevroTick semantic-state fuzzing with an independent transition oracle and qualified AFL++ IJON or LibAFL feedback when ordinary coverage misses meaningful states."
---

# Explore meaningful states with a checked oracle

Read [the testing contract](../CONTRACT.md). Use this for a concrete search gap:
different relevant states or histories revisit familiar code, so edge novelty
alone provides weak guidance. Keep ordinary parser/conversion fuzzing in
[structured fuzzing](../qa-structured-fuzzing/SKILL.md); use
[cost-directed search](../references/cost-directed-search.md) for expensive inputs.
Selecting this skill installs no backend or changes production acceptance.

1. Name the actual consumer, admissible trace format, bounded actions and state
   distinctions needed by the requirement. Reuse the existing task, candidate and
   CHECK obligations. For fixture association work, preserve its unauthenticated,
   inert scope; no network, clock or authentication authority follows from a trace.
2. Build one shared driver through the real public entry points and a separately
   authored transition oracle. Check actual outputs, rejected-action state and
   forbidden effects after each action through
   [model-based testing](../qa-model-based-testing/SKILL.md). Model agreement and
   feedback novelty are separate observations. Preserve valid construction and
   real guards; annotations must not bypass cryptography, identity or authority.
3. Define a small explicit abstraction of **actual target observations** for
   feedback. Do not report the model's predicted state as target state. Bound
   cardinality and test distinctions that matter: summing two Boolean flags can
   accidentally merge different states. Track partial/rejected traces explicitly;
   successful decoding or a high score does not establish a reached transition.
4. Use the [backend and comparison procedure](references/backend-workflow.md).
   Admit exact engine, compiler/instrumenter, target/features, driver and corpus.
   Qualify public forwarding, actual coverage, feedback sensitivity and reset
   before a campaign. Manual action labels are not ordinary LLVM edge coverage.
   A built executable or help output supplies no reached-feedback evidence.
5. Compare baseline and guided runs using the same driver/oracle, initial seeds,
   input domain and declared budgets. Bound actual executions, steps, bytes, wall
   time, memory, corpus/output and owned resources; a loop-iteration count may
   include multiple target executions. Keep collective workers within the adopted
   twelve-slot budget rather than giving twelve workers to every engine.
6. Reset target state and per-input observations between independent traces;
   preserve state within each trace and preserve the fuzzer's search history.
   Challenge stale feedback and reset failure alongside healthy cases through
   [harness qualification](../references/harness-qualification.md). Simulated
   effects do not qualify actual Windows tokens, IPC, SCM or clock lifetimes.
7. Retain concrete traces, observed state/cost frontiers, failures, source/tool
   identities and actual execution counts. Reproduce useful witnesses from fresh
   state through the independent oracle. Import/minimize corpora only under their
   matching format and objective; coverage minimization can discard semantic or
   high-cost witnesses. Keep those frontiers separately.

Report reached behavior, control outcomes, incomplete attempts and comparison
limits in the existing task. More retained inputs or one favorable run establishes
neither better defect detection nor a global state-space/resource bound. Keep
CHECK-19/25 harness evidence, CHECK-21 resource observations and native/product
qualification distinct. Miri, Kani, Loom, mutation and native diagnostics continue
to answer their own selected obligations.

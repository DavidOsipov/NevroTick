# Choose and qualify generated tests

Use this guide when choosing a method for an input, state-history or scheduling
defect. Read the [testing contract](../CONTRACT.md), then only the selected tool
manual. Existing focused tests remain useful; another framework is justified by
a concrete exploration or replay gap, not by its availability.

Selection and implementation procedure, reviewed **2026-11-15**.
The current manuals describe candidate frameworks, not an installed product
test suite. [Configured tooling](configured-tooling.md) and the current task own
admission, commands, permissions and remaining execution holds.

## Contents

- [Choose the question first](#choose-the-question-first)
- [Map NevroTick failure classes](#map-nevrotick-failure-classes)
- [Prepare the smallest useful pilot](#prepare-the-smallest-useful-pilot)
- [Build the harness around the real boundary](#build-the-harness-around-the-real-boundary)
- [Combine input generation and scheduling deliberately](#combine-input-generation-and-scheduling-deliberately)
- [Classify results before changing anything](#classify-results-before-changing-anything)
- [Retain evidence another agent can reproduce](#retain-evidence-another-agent-can-reproduce)
- [Maintain the guidance](#maintain-the-guidance)

## Choose the question first

| Question | Start with | What it explores | What still needs separate evidence |
| --- | --- | --- | --- |
| Can a known ordering or a small finite set of boundary inputs violate the contract? | Existing unit/component runner with explicit cases and deterministic barriers | The specified inputs and orderings, with a directly inspectable oracle | Unselected inputs, schedules and platform effects. No generator is needed merely to enumerate a short fixed list. |
| Does a Rust function preserve an invariant across a structured input domain? | Ordinary Proptest under [property QA](../qa-property-based-testing/SKILL.md); see the [Proptest manual](proptest-state-machines.md) | Related values, boundary distributions, generated failures and shrinking | History-dependent behavior unless the generated input represents a real history; thread schedules and native effects. |
| Does a Rust component remain correct across reserve/cancel/replace/retry/complete sequences? | [proptest-state-machine](proptest-state-machines.md) under [model QA](../qa-model-based-testing/SKILL.md) | Sequential command histories against an independent model | Thread interleavings inside or between actions. An async API does not make a sequential driver a scheduler explorer. |
| Can task ordering break ownership, cancellation or publication? | [Shuttle](shuttle-concurrency.md) under [lifecycle QA](../qa-state-concurrency-lifecycle/SKILL.md), after controlled runtime cases | Schedules visible to the admitted instrumented primitives and adapters | Uninstrumented dependencies, real OS completion, elapsed-time semantics and weak-memory behavior. |
| Is the suspected defect in a small atomic or synchronization algorithm? | Admitted Loom through [Rust techniques](rust-techniques.md) and [configured analysis](configured-tooling.md#additional-rust-analysis) | An explicitly instrumented small synchronization model, including its modeled memory orderings | Loom's documented C11 omissions and possible false alarms, ordinary OS synchronization and the actual product adapter. |
| Can a Python reader accept an invalid record, duplicate identity or incomplete receipt? | Deterministic physical fixtures first; then ordinary `@given` in [Hypothesis](hypothesis-admission.md) | Independently generated inputs and their expected acceptance/refusal | Cached or historical behavior unless explicitly exercised; later consumers and protected effects not observed by the test. |
| Does a Python component's history affect its answer? | Hypothesis `RuleBasedStateMachine` in the same [manual](hypothesis-admission.md) | Rules, model state and invariants over replacement, invalidation or cached admission | Concurrent schedules; behavior filtered out by rule preconditions; effects outside the tested receiving boundary. |
| Is the question about malformed bytes reaching a parser, or a source reaching a sensitive sink? | Existing [structured fuzzing](../qa-structured-fuzzing/SKILL.md) or [CodeQL data-flow analysis](../qa-data-flow-analysis/SKILL.md), according to the question | Byte-oriented target behavior or extracted source-to-sink paths | Semantic contract truth, current authorization and runtime effects still need independent checks. Neither replaces the methods above merely by reporting no finding. |

Loom is not a blanket weak-memory oracle. Its [upstream limitations](https://github.com/tokio-rs/loom#unsupported-features)
include incomplete C11 behavior; inspect the exact selected version and model.
Shuttle likewise requires its own instrumentation map. Do not interpret two
different tools' green outputs as independent proof when they share a copied
oracle, omitted dependency or inaccurate adapter.

## Map NevroTick failure classes

These classes guide investigation; they do not assert an unfixed defect in the
current tree. Inspect implementation and existing tests before extending them;
another agent's uncommitted repair is not visible in a
different checkout.

| Boundary to inspect | Independent observation | Smallest first experiment | Add exploration when useful |
| --- | --- | --- | --- |
| [Engine series](../../../../crates/nevrotick-engine/src/series.rs): stop or replacement near publication | Returned outcome, emitted checkpoint and actual publication agree with the contract's decision point | Force stop/replacement around the final check and publication using barriers; include the legitimate publication-first ordering | Shuttle if relevant schedules remain unexplored and the actual path can be instrumented. |
| Background-operation ownership in [sessions](../../../../crates/nevrotick-engine/src/session.rs) | Cancelled awaiting calls retain custody of unfinished work; capacity becomes reusable exactly when the owner permits | Hold completion, cancel the caller, attempt another operation, complete the old operation and observe useful recovery before fixture teardown | Generated histories for repeated lifecycle operations; scheduler exploration for a specific interleaving gap. |
| [Source policy](../../../../crates/nevrotick-engine/src/source_policy.rs): restrictions, replacement and current authority | State and allowed/refused effects match the accepted policy after each event, with separate clock domains | Explicit contract-derived histories with current and stale identities; preserve rejected operations | Proptest state-machine histories with independently expected states and effects. No invented timeout or policy duration. |
| [Engine receipt admission](../../../../scripts/verify-engine-analysis.py) | Exact current identities, test rosters and terminal custody; rejection before downstream construction/effects | A healthy physical fixture and one semantically invalid fixture whose integrity remains valid | Hypothesis input properties when multiple correlated fields or partitions make enumeration insufficient. The historical EP1 class is a regression idea, not current-status evidence. |

Define the publication decision/commit point before asserting order. A
cancellation or generation change that takes effect before commit may forbid the
effect; a change after a legitimate commit does not automatically make that
earlier effect invalid. Preserve atomic production ownership boundaries instead
of inserting a test seam that manufactures an impossible interleaving.

## Prepare the smallest useful pilot

1. State one changed guarantee, its source owner, real consumer and existing
   acceptance/CHECK scope. Name the concrete wrong behavior and a nearby useful
   behavior that must still succeed. Resolve contract ambiguity before writing
   an oracle that chooses behavior accidentally.
2. Inspect current tests and admitted tooling. Reuse a sufficient deterministic
   driver or framework. For Rust dependency adoption follow [dependency updates](../../environment/dependency-updates/SKILL.md),
   including the complete changed graph and selected registry cooldown. Python
   needs its approved environment and package policy; Cargo policy is not
   a Python installer policy. Missing admission does not prevent design review
   or existing deterministic cases.
3. Read the selected manual's setup, example and execution sections. Its example
   is not a product harness: name every API, observer and fixture connection
   requiring replacement or mapping to current production behavior. Source review,
   syntax checks, compilation and actual test execution are distinct evidence.
4. Define finite cases/history length/schedules, rejected draws, shrink attempts,
   bytes, elapsed time, output and cleanup limits. Account for persisted replays,
   explicit examples, fault controls and retries too. Settings called "cases" or
   "max examples" are not necessarily total invocation caps.
5. Record effective settings after profiles, environment variables, macros and
   runner behavior are applied. An explicit configuration object alone may not
   identify the framework's effective values. Never silently remove
   required isolation, timeout, diagnostic or admission checks to obtain a run.
6. Define the pilot exit: the intended faulty control is caught, the healthy
   control succeeds, the selected semantic partitions actually execute, the
   bounded run is accounted for and a concrete failure can be replayed when
   present. Stop with that scope; expanding to a full campaign needs a real gap
   or an existing required qualification checkpoint.

## Build the harness around the real boundary

Maintain three responsibilities, even within one test module:

| Responsibility | Required behavior | Common misleading substitute |
| --- | --- | --- |
| Driver | Deliver the intended input/action, identity, clock domain and ordinal to the actual receiving API; record what was delivered | A test-only setter that performs the transition, an omitted invalid command, or a mock that never reaches the protected owner. |
| Independent model/oracle | Derive expected output, refusal reason, state and allowed/forbidden effects from the contract | Calling the production reducer, validator or formatter to calculate its own expected answer. |
| Observer | Read returned values and actual state/effects, including resource custody and absence of forbidden effects before cleanup | Reading only the final report, trusting a matching digest, or using fixture teardown to hide missing product cleanup. |

Use [harness qualification](harness-qualification.md) when changing any of these
responsibilities. Bind delivered inputs and observations independently; model
agreement is insufficient when the driver executed the wrong command.

Construct valid related values, then create explicit invalid
requests with independently expected refusal. A precondition can protect the
generator's own construction requirements; it must not eliminate a request the
real API is required to reject. Keep current/stale and accepted/refused neighbors
visible in actual reached-case records.

Start with a healthy case and plausible controlled defect through the actual
owner. Examples include premature permit release, a missing generation check,
acceptance of a duplicate settlement or an omitted receipt-roster guard. Require
the intended assertion and propagated failure; an import failure, random panic
or broad "any exception" check does not establish sensitivity. Keep controlled
faults isolated from the candidate being delivered.

Reset model, SUT, clocks, owned files and external fixtures between cases and
between shrink/replay attempts. Preserve state only inside the declared history.
Register cleanup before fallible setup. A failed reset invalidates subsequent
reductions; aborts and retained background/native work need an external owner.

## Combine input generation and scheduling deliberately

Begin with the implicated axis. For example, keep one fixed
stop/publication scenario while exploring schedules, or generate policy histories
through a deterministic sequential driver. Adding both at once makes failures
harder to explain and can multiply work without additional value.

When a coupling gap justifies both, select a small set of independently
replayable generated histories and explore bounded schedules for each. Verify
that the chosen runner and randomness are supported inside the scheduler; a
nested generator, real clock, hash iteration or fixture library can introduce
uncontrolled nondeterminism. Do not assume either framework observes the other.

Retain the input/history **and** scheduler artifact. Keep both fixed during the
first replay. Minimize one dimension at a time and replay every accepted reduction
against the original independent oracle. Report completed cases and completed
schedules separately, with aggregate limits across their combinations. Neither
the seed nor a count of schedule selections identifies all executed behavior.

## Classify results before changing anything

| Observation | Disposition and next action |
| --- | --- |
| The intended contract assertion fails on the actual boundary | Candidate defect until discriminated from model/adapter error; preserve original inputs and raw failure before repair or shrinking. |
| Healthy control fails or deliberately faulty control passes | Harness/oracle/admission problem. Repair the specific gap before interpreting a larger run. |
| A modeled execution deadlocks | Inspect blocked actors, joins, modeled primitives and permitted fairness. Establish whether the scenario is reachable and violates the actual contract; do not dismiss every deadlock as a timeout or count every one as a product bug. |
| Rejection limits, filtering or empty selection prevent required cases | Incomplete exploration. Repair strategies/selection or use explicit cases; do not count unreachable invalid inputs as tested refusals. |
| Timeout, shrink budget or external resource budget expires | Preserve the reached evidence and identify incomplete work. A minimized artifact may be only partially reduced; an incomplete run is no all-clear. |
| A stored seed/schedule/blob no longer reproduces | Compare source, strategies, runtime, environment and configuration. Retain the original; use concrete inputs and diagnose nondeterminism instead of weakening assertions. |
| All selected checks pass | State the exact reached domain/model/runtime and controls. A finite or sampled pass is not universal correctness, native qualification or a measured probability of no bugs. |

Shrinking discovers a simpler witness within the framework's strategy and search
limits; it does not promise a globally smallest failure. Preserve the original
and independently verify any smaller case.

## Retain evidence another agent can reproduce

Use the existing task record and its owned artifact location; this table is a
content checklist, not a mandatory new receipt schema or reporting framework.

| Evidence | Minimum useful contents |
| --- | --- |
| Guarantee and oracle | Requirement, real owner/consumer, admitted domain, expected result and forbidden effects, independent oracle source. |
| Candidate | Commit plus relevant dirty bytes, source/driver/oracle/fixture identities, dependency graph, target/features and selected runtime. |
| Execution | Actual tool versions, command/test selector, effective profiles/environment/settings, isolation and aggregate budgets. |
| Reachability | Delivered calls/actions and semantic partitions; selected/completed histories or schedules, invalid/rejected draws and missing paths. |
| Sensitivity | Healthy and requirement-linked faulty controls, intended diagnostic/assertion, actual failure propagation. |
| Failure/replay | Original and reduced concrete inputs or history, initial state, original schedule/blob/seed where useful, replay command and observed same failure. |
| Cleanup and limits | Product resource observations before teardown, fixture restoration, retained work, incomplete paths, runtime/native/instrumentation exclusions. |
| Verdict | Proposed, source-checked, syntax-checked, compiled, executed or qualified scope; next action only for a concrete remaining obligation. |

Never substitute a code example, installed crate, default profile, wrapper exit
or produced report for those observations. If a required capability is absent,
record the gap and continue independently useful authorized work.

## Maintain the guidance

Tool manuals contain detailed API/source links and dated observations.
Recheck relevant upstream docs against the **actually selected version** before
adoption or a version-sensitive change. Update the narrow stale recipe; preserve
existing evidence with its original version and candidate. New upstream support
does not retroactively instrument a past run or admit a dependency graph.

Keep short selection hooks in the skill entrypoints and full examples here or
in the selected manual. Compress redundant prose, not required implementation
steps, failure semantics, code or qualifications. Guidance changes alone need
documentation/skill checks and proportionate instruction review, not a product
schedule/property campaign. Existing nonformal gates and the post-V1 formal
deferral remain unchanged.

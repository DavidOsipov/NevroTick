---
name: "qa-independent-commit-audit"
description: "Audit commits or a commit range with parallel specialists and a fresh judge for logical bugs, vulnerabilities, testing gaps and repository-skill compliance."
---

# Audit commits with independent specialists and a judge

Use for a requested independent commit, range or PR audit. Routine editing,
pre-commit checks and questions about one suspected defect keep their narrower
owners. Audit defaults to read-only; repairs, commits and external publication
need their own existing authorization. This skill adds no product acceptance gate.

Apply the [testing contract](../CONTRACT.md),
[independent review](../qa-independent-review/SKILL.md) and
[subagent dispatch](../qa-subagent-dispatch/SKILL.md). Reuse one existing task or
coordinator record for scope, assignments, evidence, findings and completion.

## 1. Freeze the actual change and its authority

- Resolve repository/root, requested branch or PR, full base/head SHAs, selected
  commits/parents and dirty state. For "latest N", refresh the intended remote
  ref and state the traversal/merge-parent choice. Do not silently substitute a
  local branch, combine dirty content with committed source or chase a moving head.
- Inspect each commit and the combined result. Follow changed guarantees into
  complete relevant bodies, constructors, callers, consumers, configuration,
  generated artifacts and sibling paths. A repaired intermediate defect remains
  historical; unchanged reachable defects remain identifiable as pre-existing.
- Read current `AGENTS.md`, the [documentation guide](../../../../docs/README.md),
  affected task/phase/CHECK owners and accepted decisions. Enumerate current skill
  descriptions; read/apply full triggered implementation, contract, security,
  planning and QA leaves. Candidate edits to standards or acceptance criteria
  need comparison with the accepted baseline and authority, not self-approval.
- Give the review a finite scope/budget, permitted checks and explicit frontier.
  Record unavailable raw artifacts and current task-specific execution holds.
  Source review continues when execution is unavailable; missing evidence is
  neither a pass nor a demonstrated defect. Preserve authorized deferrals.

## 2. Run parallel specialist reviews

Spawn **at least three distinct specialist reviewers**, with complementary
assignments below, then **one separate judge**. Run specialists concurrently within
available capacity. If capacity is smaller, use bounded waves or sequential fresh
reviewers; retain all roles and disclose actual parallelism. If delegation or a
required role cannot run, report the audit incomplete and provide the available
scoped review; do not silently replace the team or judge with self-review.

| Specialist | Required investigation |
| --- | --- |
| **Logic, contracts and integration** | Derive expected behavior independently of tests. Trace algorithms, numerical meaning, state transitions, constructors, helpers and actual consumers. Challenge wrong-but-well-typed values, omitted guards, error/cleanup ordering, freshness, generations, budgets and cross-module composition. |
| **Security, resources and effects** | Trace lower-trust input/caller through current authorization to effects. Inspect reached protocol, IPC, privilege, crypto, storage, diagnostics, build/dependency and package boundaries; bound aggregate work and actual native lifetimes. Establish actor, conditions and concrete consequence before calling a concern a vulnerability. |
| **QA, evidence and repository standards** | Reconcile every applicable strategy using the [QA map](references/qa-strategies.md). Inspect actual tests, oracles, harnesses, runners, thresholds, exclusions and evidence consumers. Check full applicable skill requirements, source-visible types, documentation/source claims and unjustified completion. |

Split large subsystem assignments further when useful; keep the coordinator
responsible for cross-worker seams and combined-candidate coverage. For docs-only
changes, the same lanes assess semantic/dependency correctness, protection
requirements and evidence claims; they do not invent runtime tests.

Every reviewer receives the neutral request, frozen candidate, requirements,
assigned boundaries, evidence locations, operation/resource limits and return
contract. Use [fresh context and completion accounting](../../planning/references/review-and-handoff.md#freeze-review-inputs-and-account-for-completion):
explicitly disable conversation inheritance (`fork_turns: "none"` or the supported
equivalent). Do not supply the author's desired verdict, another reviewer's
findings or private discussion before the initial independent return. Shared files
remain shared; assign exclusive writable resources for any permitted checks.

Require actual results: inspected source/skill identities, scope, findings and
counterevidence, concrete test gaps, executed checks versus attributed receipts,
unresolved inputs, consumed budget and cleanup. A timeout, empty response or
launched task is not a completed review or a clean result.

## 3. Challenge behavior, not just syntax or test totals

Apply [source-backed flow analysis](../qa-data-flow-analysis/SKILL.md) to the
logical lane and [contract QA](../qa-contract-tests/SKILL.md) to changed interfaces.
Inspect both input-to-effect and effect-to-caller paths. In particular:

- Check mathematical/domain assumptions, sign, units, clock/epoch, rounding,
  bounds and combinations of valid inputs. Compilation, explicit types and
  individually correct helpers cannot establish correct composition.
- Compare checked versus consumed values and authority at the actual effect.
  Challenge alternate constructors, defaults, mutations, stale snapshots, early
  returns, retries, cancellation, duplicate delivery and context changes at awaits.
- Observe failure after partial progress, not only first-item refusal: earlier
  valid work, later rejection, prior-state preservation, transient forbidden
  effects, charged resources and actual cleanup/completion. Include healthy
  neighbors so rejection-only implementations cannot satisfy the contract.
- Inspect tests/checkers changed alongside implementation for shared oracles,
  weakened expectations, lost cases, incomplete-to-pass conversion, source/byte
  substitution, bypassed consumers and incorrect evidence credit. Apply
  [harness qualification](../references/harness-qualification.md).
- Report code smells only with a concrete correctness, security or maintenance
  consequence and source evidence. Missing defense-in-depth, a framework name,
  or an unimplemented accepted future feature alone is not a current bug.

Validate plausible findings with the smallest authorized discriminating check or
an explicit source/domain argument. Reuse current adequate evidence; do not rerun
unrelated campaigns. Preserve [workstation isolation](../../../../docs/windows-time-development-hardening.md#33-agent-workstation-and-sandbox-rules),
current native/fuzz/storage permissions and the post-V1 formal-work deferral.
Unavailable execution stays recorded; changing runners or delegating cannot lift
a hold. Never claim a modeled counterexample or proposed mutant was executed.

## 4. Have the fresh judge challenge the combined review

The judge is distinct from the coordinator, author and specialists. Launch it with
conversation inheritance disabled and the same frozen request/source/authority.
First obtain its own source assessment and coverage questions without specialist
conclusions. For substantial plans/contracts, retain the two comprehension
paragraphs required by the [shared review procedure](../../planning/references/review-and-handoff.md#check-comprehension-before-critique).
Then give that judge the specialist results, raw evidence and applicability map.

Require it to inspect supporting source, seek counterexamples to the findings,
resolve contradictions, detect omitted logical/integration/QA obligations and
adjudicate every candidate. Agreement or majority vote is no evidence substitute.
Keep confirmed defects, test/evidence gaps, source-grounded unresolved hypotheses,
refutations and accepted deferrals separate. A missing test is not itself proof
that behavior is wrong. Priority follows demonstrated consequence; certainty and
execution limits remain explicit.

If the judge discovers a material new candidate, obtain a bounded independent
source challenge from the relevant specialist before final adjudication. Recheck
only affected conclusions after corrections. Preserve unresolved disagreements;
do not force consensus or require arbitrary repeated clean rounds. Wait for the
judge's actual final result; a missing judge leaves that stage incomplete.

## 5. Return one evidence-grounded result

For each finding retain a stable ID, category, new/pre-existing/repaired status,
reviewed SHA and exact location, violated requirement, reachable scenario or
missing discriminating test, consequence, counterevidence, smallest causal repair
and required verification. Separate code defects, vulnerabilities, test gaps,
skill deviations and uncertain external/native assumptions.

Report the frozen commits, scope/frontier, specialist and judge completion/context
modes, severity-ranked actionable findings, QA applicability/evidence gaps,
checks actually executed and remaining native/package/release qualifications.
Credit repaired historical findings and explain material refutations. A scoped
no-finding result is valid; invent no issue quota or whole-product safety claim.

Recheck worktree/candidate integrity and the requested remote head before closing.
If the remote advanced, identify the reviewed range and new unreviewed commits;
do not present stale work as the latest review or restart indefinitely. Reviewer
completion establishes review coverage, not runtime, phase or release acceptance.

# Source-backed flow and invariant audits

Use [testing contract](../CONTRACT.md),
[data-flow analysis](../qa-data-flow-analysis/SKILL.md),
[lifecycle testing](../qa-state-concurrency-lifecycle/SKILL.md),
[independent review](../qa-independent-review/SKILL.md). Reuse existing task ledger;
cloud audit continues [its handoff](../../../../tasks/codeql-cloud-handoff.md).
Use the [acceptance catalog](../../../../docs/windows-time-architecture.md#202-acceptance-catalog):
CHECK-19 for applicable lint/SAST/discovery controls; CHECK-25 for decoder,
construction and module invariants; CHECK-24 for exact-candidate review/artifact
identity. CHECK-22 covers reached accessibility scope.
Canonical requirements retain authority/execution scope.

## Establish roots and protected effects

Freeze source/dirty changes, configuration, target/features, dependencies,
generated inputs, models. Name bounded guarantee, independent expected result,
input domain, source owner, required environment. All user/external input hostile:
stored, local, authenticated/admin included. UI restrictions are no receiver guard.

Inventory roots/effects independently of CodeQL: public handlers, UI callbacks,
IPC dispatch, network/file/config readers, service callbacks, timers, completions;
clock/state mutation, privileged calls, publication, persistence, retained resources,
secret output. Check root-to-effect and effect-to-caller directions. CodeQL/search/
call graphs navigate; none proves complete inventory. Expose unextracted,
unresolved and dynamic edges.

Prioritize authority changes, hostile ingress, irreversible effects, secrets,
concurrent ownership, numerical/time boundaries. Retain accepted neighbors and
lower-risk roots awaiting review. Priority changes order, never evidence credit
or permission to omit open branches.

Choose by remaining uncertainty: direct invariant review for state/semantic
questions; CodeQL for unresolved propagation or recurring caller patterns.
Use local flow within one callable, global when propagation crosses functions.
Disputed transitions need deterministic histories
at the actual consumer; use [generated-test selection](generated-testing-selection.md)
for a remaining history/schedule gap. Set a finite query/model-development budget
and stop condition before expansion; at the limit retain unresolved edges and
their owners. Selection preserves existing required checkpoints and gates.

## Review bounded connected slices

Existing ledger worklist: root/effect, requirement, actual symbols, inspected
boundary, result, continuation owner. Split at explicit contract; record exported
caller assumptions and remaining callee obligations. Local result closes only
slice until composition and actual caller wiring are checked.

Trace complete relevant functions/callers, fields, branches:

- Typed values, units/scales, conversions, bounds, rounding, checked arithmetic.
- Control dependence, authorization, success/errors, guarded effects.
- State transitions, generation/freshness, order before/after awaits.
- Constructors, alternate construction/mutation, checked versus original DTOs.
- Selected traits, callbacks, retry/recovery/startup.
- Task/permit/handle ownership, cancellation, timeout, `Drop`, late completion.
- Native/generated/macro/dependency/cross-process seams and model limits.

Frontier: unresolved callees, branches, assumptions; revisit before parent claim.
Cycles require bound/maintained invariant, never silent truncation. Inspect failures
as carefully as success. Timing/numerics: carry origin, unit, epoch/scale,
uncertainty, rounding, cumulative budget to consumer; plausible numbers prove
neither admissibility nor clock authority.

Guards cover exact property, consumed value and current context only. Authentication,
decoding, validation names and prior success are no blanket sanitizer. Check
mandatory invocation, returned checked object, alternate constructors, mutation,
changed authority, guard/effect order. Value, taint, reachability, authorization differ.

Cancellation request, logical timeout, dropped observation differ from acknowledged
worker/native completion. Trace actual completion custodian, terminal result,
resource release, late results. Stop closes new admission where contract requires.
Preserve original failure when cleanup fails; inspect cleanup before teardown masks it.

## Challenge the integrated guarantee

Derive expectations from requirements/independent facts before production output
or scanner results. Review integrated root-to-effect chains after slices; matching
summaries alone prove no composition. Challenger receives requirements, candidate,
evidence; challenges assumptions, missing callers, stale context, competing
explanation. Agreement proves neither statistical independence nor correctness.

Credible defect: seek authorized bounded behavioral reproduction at real receiving/
consumer seam, useful accepted neighbor, discriminating expectation. Separate
behavioral failure from setup/import/timeout-before-behavior/zero-selection failure.
Unavailable reproduction: retain source trace, consequence, assumptions and
uncertainty. A source-confirmed defect needs concrete admitted input/state,
complete reachable path through actual callers/callees, independently
specified violated contract and resolved relevant assumptions. Unresolved
reachability, configuration or platform behavior remains an open lead. Label
source confirmation separately from executed behavioral/native reproduction;
never infer execution from reasoning. Repair follows existing scope/workflow.

Analyzer paths: [CodeQL evidence controls](codeql-data-flow.md); name sources/sinks,
queries/models, actual extraction. Empty alerts, giant diagram or successful
compiler cannot discharge worklist. Execute through [configured tooling](configured-tooling.md);
preserve coverage/mutation/native/package gates at existing checkpoints. Method
grants no native/fuzz/formal authority; formal stays post-V1 under testing contract.
Required unavailable checks stay incomplete; review cannot replace them.

## Reuse evidence and invalidate affected chains

Reuse only with unchanged relevant source, callee contracts, caller wiring,
configuration/models/evidence identity. Verify binding; history, path, mtime or
HEAD alone insufficient. Changed guards, constructors, authority/context or callee
behavior invalidate dependent assumptions/affected transitive chains. New callers
need fresh admission/composition even if callee evidence remains valid. Generated
code, features, platform bindings may invalidate same guarantees.

Use [regression selection](../qa-regression-selection/SKILL.md): affected checks,
justified unaffected exclusions. Unrelated edits need no wholesale rescan. Unknown
impact: explicit gap or broader applicable review. Reuse never changes admission,
thresholds or required gates. No measured-efficiency claim without comparative
observations. Map is accounting, not full coverage, AI-proofness or safety score.

## Record results in the existing ledger

Compact Markdown entry: actual symbols, values, evidence paths; replace placeholders.
Diagram may supplement, never replace record.

| Field | Required content |
| --- | --- |
| Question and authority | Requirement/CHECK, allowed/forbidden outcome, independent facts. |
| Identity and scope | Source/configuration/model/evidence binding, owner, target/features, environment. |
| Root and effect | Actual entry/caller/input field, consumer/protected effect. |
| Trace | Actual functions/traits, fields, branches, units, state, guard/effect order. |
| Slice contract | Admitted inputs, guaranteed outputs/effects/errors, assumptions, continuation owner. |
| Frontier | Open branches/callees/constructors, cancellation/retry/Drop/native/model gaps. |
| Challenge | Independent expectation, accepted/fault controls, competing explanation, reproduction. |
| Evidence | Separate source review, analyzer execution, host behavior, native observations. |
| Disposition | Reviewed slice, open lead, source-confirmed defect, reproduced defect, executed check or incomplete stage; evidence basis and limits. |
| Reuse and next action | Dependencies, invalidation reason, affected reruns, remaining owner. |

Retain failures/exits, actual selected/completed counts, retries, cancellation,
cleanup. Separate planned/reviewed/executed/qualified stages; local slices or
sampled runs prove neither all paths nor native/release safety.

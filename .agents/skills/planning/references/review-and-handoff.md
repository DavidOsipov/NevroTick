# Independent reviews, handoffs and document drift

Copyright 2026 Google LLC.

SPDX-License-Identifier: Apache-2.0

Use for substantial plan handoffs, independent review coordination, repeated
findings or document/source consistency work. Keep the existing plan/coordinator
record and stage-scoped verdict; a small understood edit needs no extra review
pipeline or report tree. Current repository instructions, accepted owner decisions
and canonical requirements control the work.

Read the needed section: [review inputs and completion](#freeze-review-inputs-and-account-for-completion),
[comprehension](#check-comprehension-before-critique),
[finding lifecycle](#preserve-finding-identity-and-resolution-evidence),
[decision reconciliation](#reconcile-the-handoff-with-current-decisions),
[document drift](#classify-document-and-source-differences),
[attribution](#attribution-and-adaptation).

## Freeze review inputs and account for completion

Give each reviewer the actual request, named stage, target revision including
relevant dirty content, applicable canonical decisions/requirements, selected
source/evidence and allowed operations/budget. Let it read those inputs; do not
supply the author's persuasive verdict, expected paraphrase, suspected answer or
parent conversation. Necessary published requirements are not forbidden context.

When a fresh reviewer is required or selected, inspect the native delegation
schema and explicitly disable conversation inheritance: `fork_turns: "none"`,
`fork_context: false`, or the supported equivalent. A persona instruction alone
does not create a fresh context. Require the actual child result before synthesis.
Fresh conversation does not isolate shared files, project instructions or memory
services; do not retrieve private discussion/memory to fill gaps in the review
packet. Preserve current task permissions and one owner per writable resource.
Document review normally reads only; QA may use its already-authorized bounded
discriminating checks. No new tool, installation or host mutation follows.

In the existing coordinator record retain reviewer/scope, candidate identity,
context mode, required inputs actually inspected, completion status and raw result
or failure. Use `completed` only for the requested review that actually finished;
it can still find defects. Partial work or missing required input is `incomplete`;
no usable review is `not run`. Timeout, launch failure or an empty return is never
zero findings. Record only reads/completion supported by the available result or
trace; disclose unavailable trace evidence instead of inventing it.

Retry only within the existing time/attempt budget when useful. If independent
review is a required gate and unavailable, that gate remains incomplete; disclosed
self-review supplies only its narrower evidence. Optional review gaps do not
freeze unrelated authorized work. Reviewer agreement neither adds executions nor
establishes statistical independence.

## Check comprehension before critique

For a substantial plan, changed architectural contract or handoff, ask the fresh
reviewer to begin with two short paragraphs in its own words: the proposed change,
and the relevant current system as established by the supplied sources. Distinguish
existing, planned and deferred behavior. If the text cannot support a coherent
explanation, identify the missing relationship or assumption.

Preserve that attributed paraphrase alongside findings, even when no finding is
reported. A synthesizer must not discard or rewrite it to match the author's
intent. Compare interpretation with the actual requirements and target text;
investigate whether a mismatch comes from the document, missing inputs or a
reviewer mistake. A confused reviewer is not automatically proof of a product
defect or a Blocker. Severity/readiness follows the affected requirement and stage.

For example, a NevroTick handoff must keep service availability, trusted time and
clock-write authority distinct. A read-only Blocked broker may be available;
understanding that distinction does not prove native Windows enforcement.

## Preserve finding identity and resolution evidence

For multi-round work keep one stable issue ID across reviewers, deduplication,
revisions and moved lines. Preserve links from raw findings to that issue; do not
replace its identity with each round's numbering. Record only the fields needed:
requirement/boundary, semantic location, reviewed candidate, observation/evidence,
material consequence, disposition/reason, correction or next action and conditions
that invalidate the conclusion. Re-resolve locations after edits.

Keep open findings, proposed fixes, applied-but-unverified fixes, verified fixes,
evidence-based refutations and owner decisions distinct. Applying an edit is not
verification. Check the affected guarantee against the actual revised candidate;
reuse unaffected evidence. If the issue recurs or relevant evidence/requirements
change, reopen it. Moving a heading alone does not invalidate a semantic result.
Refute a finding with inspectable evidence; author intent or reviewer votes alone
cannot close it. Record missing evidence explicitly.

Accepted residual risk requires the existing authorized owner-decision process
and its recorded scope/reason; it is not a verified fix or satisfaction of a
mandatory check. A requirement change must reach the canonical owner. Previously
accepted/refuted findings remain subject to their invalidation conditions. Silence,
"continue", a skipped fix or exhaustion of the review budget accepts no new risk
and grants no new edit, release or privileged-operation authority.

If repairs alternate between incompatible outcomes, inspect the disputed invariant
and evidence rather than oscillating indefinitely. At the agreed budget or
unresolved churn, stop repeating reviews, retain open items and the next permitted
action. Only a material unresolved owner choice needs escalation; routine work
within current authorization continues. No fixed swarm size or two-clean-round
ritual replaces the stage's actual acceptance requirements.

## Reconcile the handoff with current decisions

Before a substantial revision/handoff, compare the candidate with its identified
predecessor and current canonical decisions. Record meaningful obligations that
were retained, changed, added or removed, with source/reason and affected consumers.
Separate an unresolved contradiction from a documented authorized supersession.
Keep rejected alternatives/reasons when relevant; do not reopen settled choices
merely because a new reviewer lacks their linked record.

Inspect new consequential architecture, interface, dependency, privilege or
lifecycle decisions as well as lost obligations. Reuse canonical IDs and update
the existing requirement/evidence inventory for newly exposed boundaries. Correct
routine transcription mistakes under current authority. For a material unresolved
choice, use the existing owner-decision procedure before its dependent action;
the predecessor alone cannot override a later accepted decision. Plan the next
executable stage without inventing all later files/APIs or asking for decisions
already made.

## Classify document and source differences

Start from the documentation guide's authority order and current milestone.
Identify the claim, normative owner, implementation status, inspected source/dirty/
configuration identity and available evidence before comparing documents and code.
Inspect both documented obligations and relevant source behavior missing from the
record; a source match alone is not a correctness oracle.

| Observation | Treatment |
| --- | --- |
| Planned component or file does not exist yet | Record pending implementation, not an established implementation defect. |
| Capability is explicitly deferred for this milestone | Preserve the deferral and its trigger; do not create a new current-stage blocker. |
| Implementation exists but required behavior has not been observed | Record missing/partial evidence; neither pass nor demonstrated defect. |
| Older text conflicts with a later accepted decision | Identify superseded/stale documentation and the current authority; preserve historical receipt identity. |
| Current-state assertion names a nonexistent component or unsupported result | Record the evidenced documentation/claim contradiction, separate from an unobserved runtime failure. |
| Implemented behavior contradicts an applicable requirement | Record the actual divergence and evidence/reachability limits; correct it or obtain the required canonical decision. |
| Source contains an undocumented consequential effect or assumption | Record the newly exposed boundary and its owner; do not require prose duplication of every function. |

Keep each difference linked to its decision/requirement, stage, source evidence,
classification, consequence and correction owner. Distinguish unknown, missing,
partial, stale and adequate evidence in the existing coverage inventory. Do not
rewrite a security requirement to match possibly faulty code or promote generated
descriptions into approved contracts. Report inspected/excluded scope and unresolved
inputs; "no differences found" applies only to the stated comparison, not whole
product correctness. Revalidate only conclusions affected by changed inputs and
their dependencies, retaining earlier receipts under their original identities.

## Attribution and adaptation

Adapted from Google's Elephant-Goldfish Model at immutable commit
`f0ba151b252c9abf1318d9fe857a43265d06de89`, inspected 29 October 2026:

- [Fresh-context integration](https://github.com/google/elephant-goldfish-model/blob/f0ba151b252c9abf1318d9fe857a43265d06de89/docs/codex.md).
- [Comprehension reviewer](https://github.com/google/elephant-goldfish-model/blob/f0ba151b252c9abf1318d9fe857a43265d06de89/agents/comprehension.md).
- [Review coordination and finding lifecycle](https://github.com/google/elephant-goldfish-model/blob/f0ba151b252c9abf1318d9fe857a43265d06de89/skills/egm-review-loop/SKILL.md).
- [Evidence-based resolution](https://github.com/google/elephant-goldfish-model/blob/f0ba151b252c9abf1318d9fe857a43265d06de89/skills/egm-resolve/SKILL.md).
- [Predecessor reconciliation](https://github.com/google/elephant-goldfish-model/blob/f0ba151b252c9abf1318d9fe857a43265d06de89/skills/egm-draft/SKILL.md).
- [Document/source comparison](https://github.com/google/elephant-goldfish-model/blob/f0ba151b252c9abf1318d9fe857a43265d06de89/skills/egm-drift-report/SKILL.md).

Modified for NevroTick by David Osipov with AI assistance on 29 October 2026.
This reference is licensed under Apache-2.0; retain the accompanying
[full upstream license](../../provenance/licenses/egm/LICENSE) when copying it.
Local changes preserve stage/authorization/evidence owners, distinguish planned
and deferred work from defects, and replace automatic risk acceptance, compulsory
approval gates and fixed review rounds with the existing bounded workflow. No EGM
plugin, bootstrap rewrite or runtime is adopted. These separately licensed
development instructions do not select the application license, relicense
chrony-derived code or imply Google endorsement.

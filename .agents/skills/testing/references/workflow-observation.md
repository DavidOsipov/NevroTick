# Observe agent effort and repeated failures

Use when evaluating an agent-workflow change or investigating recurring wasted
work. Ordinary edits need no telemetry ledger. Reuse the existing task/checkpoint
and [skill evaluation](../qa-skill-evaluations/SKILL.md); application requirements,
configured verification and independent acceptance remain unchanged.

## What to measure

Measure the effort needed for an independently accepted change, including initial
implementation, failed attempts, review, repairs, verification and observer work.
Retain every assigned task as accepted, failed or incomplete, including tasks with
missing usage. A single wrapper pass or a normally completed conversation cannot
establish acceptance. Report cost on its separately identified complete subset;
do not remove unsuccessful tasks from the quality denominator.

Record input tokens including cached input, cached tokens separately, output
tokens, observed worker effort/wait and actual task wall time when available.
Worker effort and overlapping waits must not be summed into wall time. Unknown
usage is unknown, not zero. Dollar values need a recorded billing basis or an
explicit estimate; token weights and fallback prices are not actual charges.
Include observer/model overhead even in shadow mode. Retain model/settings,
wrapper/environment identities and all attempts in the existing evidence chain.

## Shadow advisory and evidence

The useful trigger is a repeated remedy against the same observed failure,
unchanged relevant source/harness/environment context and no new discriminating
observation. Attribute each worker separately. A successful coordinator does not
erase a worker's failed attempts. Long productive campaigns, intended RED,
caught mutants and successful rejection controls are neighboring controls;
duration and nonzero child exits alone do not indicate a loop.

Use the outer configured verifier receipt and the declared experiment purpose.
The current product verifier reports `passed` or `incomplete`; incomplete does
not identify a product defect. Engine analysis can report `enumerated` with
`qualification_passed: false` and exit zero. This completes listing only. An
intended inner failure needs its intended cause actually observed; an incomplete
outer campaign remains incomplete. Missing, changed, unsupported or conflicting
evidence never becomes successful qualification.

The original standard-library helper
[observe_workflow.py](../qa-skill-evaluations/scripts/observe_workflow.py) reads an
explicit local ledger and hash-pinned outer receipts. It emits accounting and
shadow advisories on stdout. It executes no recorded commands, calls no model or
network, writes no configuration/memory, and neither interrupts workers nor
changes tools. Exit zero means analysis completed, not product qualification.
It is an offline observer over supplied records, not automatic session capture.
Worker coverage, result/cause labels and effort completeness remain supplied
observations requiring review; hashes identify bytes, not truthful provenance.

```sh
python3 -B .agents/skills/testing/qa-skill-evaluations/scripts/observe_workflow.py \
  --ledger /absolute/owned-results/workflow-ledger.json
```

Keep ledgers/results beside the existing task's evidence, outside the maintained
skills pack. Supply sanitized IDs and explicit receipt paths, not transcripts,
credentials, full command output or private source excerpts. Missing worker data
must remain visible. No background collection or export is authorized by this
procedure.

## Ledger format

One JSON object per comparison arm; array order is observation order. The helper
rejects unknown/duplicate fields, repeated event IDs, nonfinite/negative metrics
and malformed booleans. Optional values are omitted when unknown; do not supply
`null`. Input bounds are 4 MiB, 256 workers, 1,024 tasks and 4,096 events; receipts
are limited to 8 MiB each and 64 MiB in aggregate.

```json
{
  "schema_version": 1,
  "workers": ["coordinator", "parser-worker"],
  "tasks": [
    {"id": "parser-repair", "outcome": "incomplete", "accounting_complete": false}
  ],
  "events": [
    {
      "id": "attempt-1", "task": "parser-repair", "worker": "parser-worker",
      "role": "repair", "context": "source-wrapper-environment-identity",
      "outcome": "unexpected-failure",
      "failure": "same-observed-assertion", "remedy": "same-remedy",
      "observation": "retained-evidence-identity"
    }
  ]
}
```

This minimal example has unknown evidence/usage and generates no loop advisory.
Each real attempt gets a unique ID. `context` identifies the relevant source,
wrapper/harness, configuration and environment; `observation` identifies the
discriminating evidence, rather than merely a fresh log filename. The helper
compares supplied identities; it cannot infer that different edits or logs carry
the same semantic evidence.

| Object | Fields |
| --- | --- |
| Task | Required `id`, reviewed `outcome` (`accepted`, `failed`, `incomplete`), exact JSON boolean `accounting_complete`; optional measured `wall_seconds`. The boolean declares all relevant worker effort captured, not that the task passed. |
| Event | Required unique `id`, declared `task`/`worker`, `role` (`implementation`, `review`, `repair`, `observer`, `verification`), `context`, `outcome` (`passed`, `expected-failure`, `unexpected-failure`, `incomplete`, `unknown`). Optional `failure`, `remedy`, `observation`, `receipt`, `expected_cause`, `effort_seconds`, `wait_seconds`, `usage`. |
| Receipt | `kind` (`product` for `scripts/verify.py`, `engine-analysis` for `scripts/verify-engine-analysis.py`), absolute `path`, lowercase `sha256`. Reads the outer JSON report. Other runner schemas need a separately reviewed adapter. |
| Expected cause | `reviewed` as an actual boolean, `intended` and `observed` IDs. An intended-failure observation requires reviewed matching causes and a successful outer receipt; this is supplied review evidence, not automatic diagnostic validation. |
| Usage | Optional nonnegative integer `input_tokens`, `cached_input_tokens`, `output_tokens`; cache is a subset of inclusive input. Optional `cost_usd` requires `pricing_basis` (`recorded` or `estimate`); keep the billing/pricing source in the task record. Omitted metrics remain unknown. |

Loop advisories require two consecutive supported, pinned unexpected-failure
observations for the same worker/task with identical context/failure/remedy/
observation IDs. Their cause remains unclassified. Different context or new
evidence ends that chain; unrelated workers remain separate. Missing or changed
receipts cannot establish a repetition. This deliberately narrow heuristic can
miss loops; review misses as well as false advisories before expanding it.

## Operational facts and comparison

A recurring execution fact can be proposed in the existing checkpoint: its narrow
claim, supporting result, applicable source/wrapper/tool/environment identity,
contradicting observations and invalidation condition. Examples include a verified
wrapper invocation or a reproducible environment failure with a verified remedy.
Review current evidence before promoting it through a normal repository change.
Do not automatically inject mined facts into future prompts or write global
memory. Suggestions to omit required verification are policy proposals, not facts.

Start in shadow mode: record what an observer would advise, then review false
advisories and missed loops. Compare one mechanism at a time on equivalent tasks
under fixed acceptance, tools, model/settings and finite budgets. Use separate
ledgers and learned state per comparison arm; retain unsuccessful and incomplete
attempts. Evaluate accepted outcomes, consequential findings, repair effort,
cost/time and false interventions. Missing accounting can invalidate a cost
comparison without erasing the task's quality outcome. A small trial establishes
only its tested scope; no NevroTick savings or defect reduction is established by
this helper's controls.

## TokenEconomizer assessment and provenance

Inspected upstream commit
[`7bbad83b5215ee46c75f89370efda820590298af`](https://github.com/microsoft/TokenEconomizer/tree/7bbad83b5215ee46c75f89370efda820590298af),
2 November 2026. Borrowed ideas are accounting, evidence-aware loop advice,
reviewed fact lifetimes and shadow evaluation. No upstream implementation was
copied, installed or admitted into the product graph.

- [Judge parser](https://github.com/microsoft/TokenEconomizer/blob/7bbad83b5215ee46c75f89370efda820590298af/tokeneconomizer/loopguard/judge.py#L30-L63): `bool(obj["stuck"])` accepts the malformed string `"false"` as true. Require actual JSON types in any adopted observer.
- [Turn classifier](https://github.com/microsoft/TokenEconomizer/blob/7bbad83b5215ee46c75f89370efda820590298af/tokeneconomizer/loopguard/turn.py): misses NevroTick's Python verifier wrappers; its normal-turn success proxy is not our acceptance authority. Receipt-aware adaptation adds more than another command regex.
- [Live controller](https://github.com/microsoft/TokenEconomizer/blob/7bbad83b5215ee46c75f89370efda820590298af/tokeneconomizer/plugin/controller.py): skips subagents. Shadow suppresses intervention but can still persist state and incur model calls. Installing defaults is not an observer-only trial.
- [Configuration writer](https://github.com/microsoft/TokenEconomizer/blob/7bbad83b5215ee46c75f89370efda820590298af/tokeneconomizer/plugin/denylist.py): current Codex targets cover collaboration, goals and image viewing, and tracked configuration is protected. Automatic pruning has weak value for our workflow; retain capabilities and the safeguard.
- [Plugin/experiment differences](https://github.com/microsoft/TokenEconomizer/blob/7bbad83b5215ee46c75f89370efda820590298af/docs/PLUGIN.md#L50-L65): published benchmark savings do not qualify the live plugin or forecast NevroTick savings. A local comparison is still needed.
- [Fact lifecycle](https://github.com/microsoft/TokenEconomizer/blob/7bbad83b5215ee46c75f89370efda820590298af/tokeneconomizer/microskills/lifecycle.py): candidate activation/contradiction/retirement is useful; detector matches do not independently establish a natural-language fact.

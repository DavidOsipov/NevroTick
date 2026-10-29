---
name: "qa-skill-evaluations"
description: "Evaluate whether NevroTick skill instructions improve an agent's implementation, review or testing behavior using isolated tasks and independent outcome checks; use when creating or materially revising skills."
---

# Evaluate skill behavior

Read [the testing contract](../CONTRACT.md). This workflow tests instructions;
it grants no production changes, dependency installation or native effects.
Use it for a concrete instruction change or demonstrated agent failure. A wording
edit needs no evaluation campaign. Keep one existing task/plan record.

1. Name the decision or output the skill should improve, its trigger and a nearby
   request where it should not add work. Freeze the skill/reference bytes, prompt,
   fixture, grader and permitted actions. Package validation is a separate check.
2. Prepare a realistic task with raw inputs and an observable result. Include an
   accepted neighboring case and a plausible wrong result. The evaluator owns
   expected facts and grading; the task agent receives the requirement and working
   artifacts, without the desired diagnosis, repair or prior verdict. Use the
   [exercise procedure](references/exercises.md) for concrete cases.
3. Run the task in an owned disposable directory with a fresh agent context when
   available. Give only needed context and assign writable scope. Preserve the
   candidate's artifacts and actual commands/results. Shared filesystem access is
   not an isolation boundary: separate supplied inputs from grader material and
   record accidental exposure. Do not call a task with leaked answers independent.
4. Check the artifact through an independent oracle and healthy/fault controls.
   Inspect code and actual effects as applicable; correct prose, selected tool
   names or a promised test are not executed evidence. Grade semantic outcomes,
   not a preferred implementation or exact narrative. Classify setup, grader and
   task failures separately; preserve all attempts, including incomplete ones.
5. Assess scope discipline, useful contracts, failure behavior, test sensitivity
   and truthfulness of the final claims. For a review-only task, require a concrete
   discriminating case or sound domain argument; do not count invented defects as
   thoroughness. Required missing evidence remains incomplete.
6. If claiming an improvement, compare old/new instructions on fresh equivalent
   fixtures with the same model/configuration, tools, environment and finite
   budgets. Record order and trial counts; repeat when variability matters. A
   single successful exercise demonstrates that case, not a measured uplift or
   general reliability. Changed grading criteria require regrading both sides.

Use [campaign budgeting](../references/campaign-budget.md) for multiple trials.
Review grader sensitivity through [harness qualification](../references/harness-qualification.md).
Keep independent holdout variants for regression checks; an example whose answer
is taught in the reference is a practice exercise, not a blind generalization test.
Improve the narrow instruction responsible for a failure and rerun affected cases;
do not accumulate unrelated universal rules.

Report source identities, selected/executed tasks and controls, context/exposure,
actual outcomes, code-quality observations, attempts/budget and limits. Keep
one-off transcripts outside the skills pack. Reusable fixtures belong here;
product gates and native/release claims still need their own executed evidence.

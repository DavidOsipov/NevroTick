AI-assisted contributions are welcome and follow the same requirements as other contributions: applicable repository skills, explicit contracts, checks, and workflows. Report what was actually verified, not what an AI claimed or planned to verify.

## Summary

<!-- What changes, why, and what deliberately remains out of scope? Link the
existing issue/task and applicable requirement or acceptance IDs, if any.
Do not include undisclosed vulnerability details in a public PR. Use
https://github.com/DavidOsipov/NevroTick/security/advisories/new or SECURITY.md. -->

## Applicable guidance and contracts

<!-- Name the relevant repository skills/workflows and affected contracts or
requirement IDs. For changed function/module contracts, identify the enforcement
owner and evidence for input/output, error/effect and lifecycle guarantees.
Keep this proportional: say not applicable with a reason for docs-only changes
or other unaffected areas. Do not claim that every skill applies or was run. -->

## Verification

<!-- Record actual commands/checks, results (including skipped or failed checks),
candidate revision/local changes, and relevant OS, target and feature selection.
For a behavioral fix, describe the independent failing expectation and healthy
neighboring control, or explain why reproduction was unavailable.
For docs-only changes, link/canonical-consistency checks are sufficient. -->

## AI-assisted code: tests, mutation testing, and QA gates

<!-- For AI-assisted code contributions, tests must be written for the changed
behavior/contracts, the new tests must be challenged through mutation testing
of the affected production code, and the other applicable QA gates from the
repository skills/workflows must be completed. Green existing tests alone are
not sufficient. Preserve the configured mutation scope and thresholds.

List each applicable gate below, its exact command, environment, actual result,
and evidence location. Include test discovery/execution counts, mutation scope
and score, and surviving mutants or other failures where relevant. Use "not run"
for checks not executed and explain the blocker and required next action; this
is an open requirement, not a pass. Use "not applicable" only with a concrete
scope-based justification. A docs-only change need not run application tests.
Do not invent results or turn a required but unavailable check into N/A. -->

| Test or QA gate | Command / environment | Actual result and evidence | Open gap / justified non-applicability |
| --- | --- | --- | --- |
| New or updated tests for changed behavior/contracts | | | |
| Mutation testing challenging the new tests | | | |
| Other applicable skill/workflow QA gates (one row per gate) | | | |

## Remaining gaps and impact

<!-- State checks not run and why, compatibility/security implications, and
follow-up work. Distinguish host tests, cross-builds, native Windows execution,
and package/release qualification. A green check does not prove all layers.
Note affected contracts, trust/privilege boundaries, dependencies/provenance,
user-visible behavior, and documentation when applicable. -->

## Checklist

- [ ] I read [CONTRIBUTING.md](https://github.com/DavidOsipov/NevroTick/blob/main/CONTRIBUTING.md) and the applicable repository guidance.
- [ ] The scope and evidence above match the change; unavailable checks are explicit.
- [ ] Relevant documentation/task records are updated, or no update is needed.
- [ ] Public content contains no secrets, personal data, or undisclosed vulnerability details.

<!-- Follow TOOLING.md and the testing contract for applicable gates, not an
invented all-features/full-suite command. Formal proof execution remains deferred
until after V1 under OD-10; this does not waive runtime, security or native gates.
Do not change live clocks, W32Time, services, trust stores or protections on a
developer machine to satisfy this checklist. Use authorized disposable fixtures. -->

# Contributing to NevroTick

Bug reports, focused patches, documentation corrections and accessibility
feedback are welcome. NevroTick is in active development; design documents and
successful individual checks are not promises of a production-ready release.

Please follow our [Code of Conduct](CODE_OF_CONDUCT.md). Report suspected
vulnerabilities privately using [SECURITY.md](SECURITY.md), not public issues or
pull requests. Accessibility reports are covered by [ACCESSIBILITY.md](ACCESSIBILITY.md).

## Before starting

1. Read the [README](README.md) and [documentation guide](docs/README.md).
2. Check existing [issues](https://github.com/DavidOsipov/NevroTick/issues),
   [tasks](tasks/README.md) and the
   [development plan](docs/plans/development-plan.md). Reuse the existing task
   and evidence chain rather than creating a competing plan.
3. Read [AGENTS.md](AGENTS.md), applicable nested agent guides and the smallest
   relevant set of [repository skills](.agents/skills/README.md). These describe
   project engineering constraints and also apply to AI-assisted contributions.
4. For substantial architectural, dependency, licensing or product-policy
   changes, discuss the proposal first. Accepted
   [owner decisions](docs/nevrotick-owner-decisions.md) govern their stated scope;
   an open recommendation is not approval.

Keep patches focused. Do not mix a behavior change with unrelated cleanup or
replace another contributor's unfinished work. Check the working tree before
editing. AI-assisted contributions are welcome. AI-generated changes need the same
source review, provenance and independent verification as any other contribution;
the submitter remains responsible for understanding the patch, checking its
claims and addressing review feedback. An agent's report or a green test suite
is not a substitute for that responsibility.

## Reporting ordinary bugs and proposing features

Use [GitHub Issues](https://github.com/DavidOsipov/NevroTick/issues). For bugs,
include the commit or artifact, relevant configuration, OS/build and target,
minimal reproduction, expected behavior and actual result. Say whether you used
native Windows, Linux/WSL, a cross-build or a simulated fixture. Remove secrets
and personal data from logs and screenshots. A partial report is still useful;
you do not need to identify the faulty code first.

For features, describe the user problem, intended outcome and relevant existing
plan. Avoid treating a mockup, toolkit capability or upstream example as an
implemented product feature.

## Required QA for AI-assisted code

AI-assisted code contributions must include new or updated tests for the changed
behavior, mutation testing that challenges the code exercised by those new or
updated tests, and all other applicable QA gates required by the repository
skills and owning task. Mutation testing must demonstrate that the tests detect
meaningful faults; merely adding tests or reporting a green ordinary run is not
sufficient. Preserve the configured per-file mutation requirements and report
caught, surviving, unviable and reviewed equivalent mutants honestly.

Proportionate scope means selecting the relevant checks and affected code, not
waiving these obligations. If a required run is blocked, unavailable or not run,
label that gate **open / not satisfied**, explain why and record the next action.
Do not present the contribution as fully verified or ready on the strength of
partial results, and do not invent an exception or silently lower a threshold.

Documentation-only or other genuinely non-executable changes need the relevant
non-code checks, not fabricated executable tests or meaningless mutation runs.
That distinction must not be used to exempt executable code, generators, scripts
or behavior-changing configuration from their applicable testing obligations.

## Engineering expectations

Apply the repository skills that match the changed guarantee, not every skill or
check in the repository. Small, understood changes need a concise scope and
check record; complex work needs the existing plan, explicit acceptance and
bounded independent review. Do not create a parallel process just for a PR.

- **Explicit contracts:** give every changed function/module boundary input
  invariants, output guarantees, failure/effect behavior and lifecycle obligations.
  Use typed interfaces, private checked construction and mandatory guards rather
  than caller convention or debug-only assertions. Follow
  [Rust contract enforcement](.agents/skills/implementation/implement-rust-contracts/SKILL.md).
  Revalidate changed representations or authority context; immutable validated
  values can be reused within their valid scope.
- **Untrusted input and authority:** validate and bound input at admission,
  including local, authenticated, signed and generated data. Check current
  authority where a protected effect occurs. Exercise raw-input/direct-handler
  bypasses and valid neighboring cases. Select the relevant
  [security boundary skill](.agents/skills/security/README.md).
- **Behavior and ownership:** trace the real producer, state transitions and
  consumer/effect. Preserve clock units, checked arithmetic, resource budgets,
  cancellation/completion ownership and useful redacted errors where applicable.
  Mocks cannot establish an OS-level property. For Rust changes, start with
  [implement-rust](.agents/skills/implementation/implement-rust/SKILL.md).
- **Compatibility and dependencies:** assess actual callers and wire/persisted
  consumers through [data-contract evolution](.agents/skills/implementation/evolve-data-contracts/SKILL.md).
  Dependency changes require the applicable
  [dependency workflow](.agents/skills/environment/dependency-updates/SKILL.md),
  exact features/target and provenance/licence review, not just a successful build.
- **Evidence and handoff:** separate observations from assumptions, plans and
  historical passes. Record what changed, what ran, what failed or remains
  unavailable, and the next action. Never invent test runs or imply a broader
  qualification than the evidence supports.

## Development and verification

Use the repository's [pinned toolchain](rust-toolchain.toml), manifests and
lockfile. [TOOLING.md](TOOLING.md#starting-implementation) provides the maintained
setup and verification entry points. Read the selected task's current
prerequisites and execution holds before running its commands; historical
receipts are not admission for changed source or a different machine.

Select checks using the [testing contract](.agents/skills/testing/CONTRACT.md):

- State the affected requirement or acceptance criterion and an independently
  determined expected result. For a bug fix, include a regression test and a
  valid neighboring case where practical.
- Run the applicable formatting, compiler, lint and behavioral checks for the
  actual package, features and target. Record exact commands, candidate revision,
  results, skipped checks and remaining gaps. Do not report a zero-test run as
  behavioral verification.
- Preserve the applicable security-primitive coverage and critical-file mutation
  gates. An unviable mutant is not a caught bug, and a green suite does not
  establish mathematical correctness or native Windows behavior.
- Distinguish source inspection, simulation, host execution, Windows cross-builds,
  native Windows results and final package evidence. Do not generalize one to
  the others.
- Formal-proof execution is deferred until after V1 under the current owner
  decision. This does not waive ordinary correctness, security, native or release
  checks. Preserve the separately recorded Windows, fuzzing, storage and UI holds.

Do not change a developer's clock, W32Time, trust store or services just to test a
patch. Native system-changing experiments need an authorized disposable fixture,
bounded execution and restoration. Use controlled loopback peers for routine
network tests, not public time services.

For documentation-only changes, check relative links and canonical consistency.
Do not run unrelated expensive test campaigns or create placeholder product
crates merely to satisfy tooling.

## Pull requests

Describe the problem, scope, affected contracts, actual checks and unverified
behavior. Link the relevant issue/task and retain the source-to-evidence chain.
Call out dependency, schema, security-boundary and platform changes explicitly.
Update the owning documentation when behavior changes, rather than adding an
independent specification. Keep current implementation, selected plans and
historical evidence visibly separate.

Preserve copyright notices, upstream attribution and file/component licensing.
The Chrony-derived Core/engine and original GUI have different recorded licence
scopes; see the current manifests, [owner decisions](docs/nevrotick-owner-decisions.md)
and [third-party records](third-party/). Do not assume that code licences
also permit reuse of branding assets. Do not import code, assets or dependencies
without checking their exact provenance and compatibility. This guide does not
create a new licence grant or contributor agreement.

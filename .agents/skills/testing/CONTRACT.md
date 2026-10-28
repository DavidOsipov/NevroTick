# Testing contract for NevroTick

Use for testing, verification, fixtures and reported qualification. The user request and repository instructions govern scope. Start with [the documentation guide](../../../docs/README.md); [the architecture](../../../docs/windows-time-architecture.md), especially sections 6, 18 and 20, defines product expectations. Its CHECK catalog is planned evidence, not executed evidence.

## Scope, expectations and repairs

- Select planning, read-only review, tests-only or implementation mode from the request. Tests-only work does not authorize product repairs. These skills add no approval round, dependency installation or full-system campaign.
- Identify the requirement, independent expected result, admitted input, protected effect and forbidden effects. Trace producer → representation/state → consumer, including failure/recovery. Reuse an adequate current map and tests.
- For a meaningful behavioral repair, demonstrate a product failure through a real entry point, keep a valid neighboring control, make the smallest causal repair, then establish GREEN. A fixture/import failure, missing Windows capability or timeout before application behavior is a harness/environment failure. If RED cannot reasonably be obtained, state the limitation and use the strongest available alternative.
- Before altering an oracle, baseline or accepted semantics, locate the authoritative requirement. Retain the counterexample and an earlier case that must remain valid. Output agreement with a production formatter or the translated upstream algorithm is insufficient by itself.
- Preserve strict decoding, private domain invariants, exact units, current authorization, source identity, freshness/generations, cumulative clock authority and failure atomicity. Do not weaken checks, introduce unchecked construction or swallow errors for GREEN.

## Execution and evidence

- Freeze the relevant source, configuration, lockfile, fixture and candidate artifact identity. Record dirty changes as part of that identity; HEAD alone is insufficient. Preserve pre-existing staged, unstaged and untracked work. Do not stage, commit, push or replace receipts unless the task authorizes it.
- Inspect actual Cargo manifests, selected features/targets, test discovery, runner/tool versions, setup and effects before execution. Do not assume a Cargo workspace, Windows runner or optional tool exists. The initial repository is documentation-only; verify current state before relying on that observation.
- Choose the smallest environment that proves the claim. Distinguish pure simulation, host Rust execution, cross-compilation, real Windows services/IPC/FFI, native UIA/Narrator, and clean-machine package evidence. Neither a mock nor a successful Windows-target compilation proves Windows enforcement.
- Clock mutation, service replacement and MSI fault injection belong in a disposable Windows environment with an owned snapshot/restoration plan. Do not alter the developer's clock, W32Time, trust stores or services merely to obtain evidence. Use controlled loopback protocol fixtures rather than public time services for routine tests.
- Bound cases, duration, retries, concurrency and repair attempts. One owner controls each writable environment, artifact output, simulator clock or Windows machine. Independent workers may inspect complementary boundaries; avoid concurrent Cargo jobs that contend for shared outputs or conflicting system mutations.
- Preserve native exit codes, selected/discovered test counts, failures, skipped/ignored tests, retries, cancellations and cleanup failures. A zero-test success, caught panic, generated report or exit zero alone proves no behavior. A required unavailable/failed/cancelled check remains incomplete, including an inherited failure.
- Keep independent fixture truth and production entry points. Test observers must not perform the transition they claim to observe. Inspect application cleanup before harness teardown masks leaked handles, tasks, timers, privileges or temporary clock changes. Restore owned resources on every exit; abort/crash tests need external cleanup because destructors may not execute.
- Reuse evidence while all relevant state and prerequisites are unchanged. Rerun affected checks after changes that invalidate it, including features, generated code, platform bindings, packaging or fixtures. Do not rerun unrelated expensive gates as a ritual.

## Product-specific evidence

Use [numerical and protocol truth](references/numerical-protocol.md) for time models, packets and strict contracts; [Windows evidence](references/windows-evidence.md) for OS/UI/package claims; and [Rust techniques](references/rust-techniques.md) when selecting runner, property, fuzz, model or mutation tooling. Load only the relevant reference.

The architecture retains **90% executable-line coverage for security-primitive modules** and **95% mutation score for each critical source file**. Branch evidence and named adversarial cases remain separate. Preserve the reviewed critical-file manifest and denominator; no favorable repository average, empty selection or unviable mutant counts as a pass. These are implementation/release obligations. A skill/documentation edit does not require nonexistent application gates.

## Coordination and closeout

For substantial multi-worker or interrupted work, keep one discoverable coordinator record with source/candidate identity, scope, owners, consumed budgets, live processes, evidence paths and unresolved gaps; use [context continuity](qa-context-checkpoint/SKILL.md). Small local work can keep this in its existing task record.

Whole-system stabilization or explicit input-to-output acceptance requires [black-box acceptance](qa-black-box-acceptance/SKILL.md) on the integrated frozen candidate. Inspect representative actual admitted inputs and actual status/persisted/package outputs directly against independent facts. Reports alone do not replace observation.

Report what was checked, exact command/configuration/target/environment, tested identity, observed results, controls, substitutions and remaining gaps. Separate behavioral GREEN, engineering review, native Windows qualification, staged-tree readiness and release approval. Report only stages actually completed; a scoped pass is neither full correctness nor ASVS/WCAG certification.

# Testing contract for NevroTick

Use for testing, verification, fixtures, qualification reports. User request/repository instructions govern scope. Start [the documentation guide](../../../docs/README.md); [the architecture](../../../docs/windows-time-architecture.md), especially sections 6, 18 and 20, defines product expectations. CHECK catalog plans evidence; proves no execution.

## Scope, expectations and repairs

- Select planning/read-only review/tests-only/implementation mode from request. Tests-only authorizes no product repair. Skills add no approval round, dependency installation or full-system campaign.
- Identify requirement, independent expected result, admitted input, protected/forbidden effects. Trace producer → representation/state → consumer, including failure/recovery. Reuse adequate current map/tests.
- For meaningful behavioral repair: demonstrate product failure through real entry point, retain valid neighboring control, make smallest causal repair, establish GREEN. Fixture/import failure, missing Windows capability, timeout before application behavior: harness/environment failure. If RED unreasonable to obtain, state limitation; use strongest available alternative.
- Before altering oracle/baseline/accepted semantics, locate authoritative requirement. Retain counterexample/earlier valid case. Agreement with production formatter or translated upstream algorithm alone is insufficient.
- Preserve strict decoding, private domain invariants, exact units, current authorization, source identity, freshness/generations, cumulative clock authority and failure atomicity. Do not weaken checks, introduce unchecked construction or swallow errors for GREEN.

## Execution and evidence

- Freeze relevant source/configuration/lockfile/fixture/candidate artifact identity, including dirty changes; HEAD alone insufficient. Preserve existing staged/unstaged/untracked work. No stage/commit/push/receipt replacement without task authorization.
- Before execution inspect actual Cargo manifests, selected features/targets, test discovery, runner/tool versions, setup, effects. Assume no Cargo workspace/Windows runner/optional tool. Initial repository documentation-only; verify current state before relying on observation.
- Choose smallest claim-proving environment. Distinguish pure simulation, host Rust execution, cross-compilation, real Windows services/IPC/FFI, native UIA/Narrator, clean-machine package evidence. Mocks/successful Windows-target compilation do not prove Windows enforcement.
- Clock mutation/service replacement/MSI fault injection: disposable Windows environment with owned snapshot/restoration plan. Do not alter developer clock, W32Time, trust stores or services merely for evidence. Routine tests: controlled loopback protocol fixtures, no public time services.
- Bound cases/duration/retries/concurrency/repair attempts. One owner per writable environment/artifact output/simulator clock/Windows machine. Independent workers may inspect complementary boundaries; avoid concurrent Cargo contention for shared outputs or conflicting system mutations.
- Preserve native exit codes, selected/discovered test counts, failures, skipped/ignored tests, retries, cancellations, cleanup failures. Zero-test success/caught panic/generated report/exit zero alone proves no behavior. Required unavailable/failed/cancelled check remains incomplete, including inherited failure.
- Keep independent fixture truth/production entry points. Observers must not perform claimed transition. Inspect application cleanup before harness teardown masks leaked handles/tasks/timers/privileges/temporary clock changes. Restore owned resources on every exit; abort/crash tests need external cleanup: destructors may not execute.
- Reuse evidence while all relevant state/prerequisites unchanged. Rerun affected checks after invalidating changes, including features/generated code/platform bindings/packaging/fixtures. No ritual reruns of unrelated expensive gates.

## Product-specific evidence

Use [numerical and protocol truth](references/numerical-protocol.md) for time models, packets and strict contracts; [Windows evidence](references/windows-evidence.md) for OS/UI/package claims; and [Rust techniques](references/rust-techniques.md) when selecting runner, property, fuzz, model or mutation tooling. Load only the relevant reference.

The architecture retains **90% executable-line coverage for security-primitive modules** and **95% mutation score for each critical source file**. Branch evidence and named adversarial cases remain separate. Preserve the reviewed critical-file manifest and denominator; no favorable repository average, empty selection or unviable mutant counts as a pass. These are implementation/release obligations. A skill/documentation edit does not require nonexistent application gates.

## Coordination and closeout

For substantial multi-worker/interrupted work, keep one discoverable coordinator record: source/candidate identity, scope, owners, consumed budgets, live processes, evidence paths, unresolved gaps; use [context continuity](qa-context-checkpoint/SKILL.md). Small local work: existing task record suffices.

Whole-system stabilization/explicit input-to-output acceptance requires [black-box acceptance](qa-black-box-acceptance/SKILL.md) on integrated frozen candidate. Directly inspect representative actual admitted inputs/status/persisted/package outputs against independent facts. Reports cannot replace observation.

Report checked scope, exact command/configuration/target/environment, tested identity, observed results, controls, substitutions, remaining gaps. Separate behavioral GREEN, engineering review, native Windows qualification, staged-tree readiness, release approval. Report only completed stages; scoped pass proves neither full correctness nor ASVS/WCAG certification.

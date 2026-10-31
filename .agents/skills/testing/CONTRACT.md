# Testing contract for NevroTick

Use for testing, verification, fixtures, qualification reports. User request/repository instructions govern scope. Start [the documentation guide](../../../docs/README.md); [the architecture](../../../docs/windows-time-architecture.md), especially sections 6, 18 and 20, defines product expectations. CHECK catalog plans evidence; proves no execution.

## Scope, expectations and repairs

- Select planning/read-only review/tests-only/implementation mode from request. Tests-only authorizes no product repair. Skills add no approval round, dependency installation or full-system campaign.
- Identify requirement, independent expected result, admitted input, protected/forbidden effects. Trace producer → representation/state → consumer, including failure/recovery. Reuse adequate current map/tests.
- Treat user-entered product input and external programmatic input as potentially malicious, even from local/authenticated/admin callers or previously accepted storage. Challenge the actual receiving handler independently of UI controls: malformed and well-formed unauthorized/stale inputs, resource abuse and aggregate request pressure. Verify bounded redacted rejection, preserved valid state, absence of protected effects and useful accepted neighbors; UI-M0 remains fixture-only.
- For meaningful behavioral repair: demonstrate product failure through real entry point, retain valid neighboring control, make smallest causal repair, establish GREEN. Fixture/import failure, missing Windows capability, timeout before application behavior: harness/environment failure. If RED unreasonable to obtain, state limitation; use strongest available alternative.
- Before altering oracle/baseline/accepted semantics, locate authoritative requirement. Retain counterexample/earlier valid case. Agreement with production formatter or translated upstream algorithm alone is insufficient.
- Preserve strict decoding, private domain invariants, exact units, current authorization, source identity, freshness/generations, cumulative clock authority and failure atomicity. Do not weaken checks, introduce unchecked construction or swallow errors for GREEN.
- Every changed function/module contract needs an identified enforcement owner and discriminating evidence for its input/output, error/effect and lifecycle guarantees. Use [contract enforcement](../implementation/implement-rust-contracts/references/contract-enforcement.md) and [contract tests](qa-contract-tests/SKILL.md): challenge construction/consumer bypasses with external compile-fail plus legal-use controls, and observe invalid-input/transition rejection at actual consumers. Garde annotations, private-item documentation and lint success alone prove no executed validation. Preserve validated type guarantees across unchanged calls and test transformed outputs/current guards where they change.

## Execution and evidence

- Freeze relevant source/configuration/lockfile/fixture/candidate artifact identity, including dirty changes; HEAD alone insufficient. Preserve existing staged/unstaged/untracked work. No stage/commit/push/receipt replacement without task authorization.
- Before execution inspect actual Cargo manifests, selected features/targets, test discovery, runner/tool versions, setup, effects. Assume no Cargo workspace/Windows runner/optional tool. Initial repository documentation-only; verify current state before relying on observation.
- When selecting configured compiler, lint, runner, scanner or artifact commands, use [configured tool execution](references/configured-tooling.md). Apply only the checks needed for the changed guarantee; retain tool-admission, application execution and native/package evidence as separate layers.
- Select Miri, cargo-fuzz, Loom, Kani, Lockbud or cargo-vet only for a named evidence gap and admitted source/harness/graph. Respect separate analysis compilers, bounds, assumptions and store custody. Successful tool controls do not execute an application gate.
- For sanitizer or memory-detection claims, use [sanitizer selection and controls](references/sanitizers.md). Separate semantic assertions from detector-specific faults; retain exact instrumentation/platform coverage and independent resource-lifecycle evidence.
- Native heap/handle/FFI or resource-lifecycle changes require an explicit coverage decision through [Windows memory diagnostics](references/windows-memory.md). Select reached Application Verifier/PageHeap checks and CDB fault capture where applicable; measure retained allocations, handles, GUI objects and tasks separately. Own per-image configuration and restoration, run the candidate with its intended token, and report missing coverage or restoration as incomplete. A clean debugger exit proves neither absence of faults nor absence of leaks.
- For uncertain Rust data propagation, use [qa-data-flow-analysis](qa-data-flow-analysis/SKILL.md) and its [CodeQL reference](references/codeql-data-flow.md). Tie paths to actual extracted source/target/features and query/model identity; challenge reachable healthy/faulty controls. AST/CFG, query compilation and empty results prove no complete data-flow or authorization claim. Local MCP use leaves hosted CodeQL deferral unchanged.
- Choose smallest claim-proving environment. Distinguish pure simulation, host Rust execution, cross-compilation, real Windows services/IPC/FFI, native UIA/Narrator, clean-machine package evidence. Mocks/successful Windows-target compilation do not prove Windows enforcement.
- Clock mutation/service replacement/MSI fault injection: disposable Windows environment with owned snapshot/restoration plan. Do not alter developer clock, W32Time, trust stores or services merely for evidence. Routine tests: controlled loopback protocol fixtures, no public time services.
- Bound cases/duration/retries/concurrency/repair attempts. One owner per writable environment/artifact output/simulator clock/Windows machine. Independent workers may inspect complementary boundaries; avoid concurrent Cargo contention for shared outputs or conflicting system mutations.
- Preserve native exit codes, selected/discovered test counts, failures, skipped/ignored tests, retries, cancellations, cleanup failures. Zero-test success/caught panic/generated report/exit zero alone proves no behavior. Required unavailable/failed/cancelled check remains incomplete, including inherited failure.
- Keep independent fixture truth/production entry points. Observers must not perform claimed transition. Inspect application cleanup before harness teardown masks leaked handles/tasks/timers/privileges/temporary clock changes. Restore owned resources on every exit; abort/crash tests need external cleanup: destructors may not execute.
- Reuse evidence while all relevant state/prerequisites unchanged. Rerun affected checks after invalidating changes, including features/generated code/platform bindings/packaging/fixtures. No ritual reruns of unrelated expensive gates.

## Execution efficiency

- Run cheap resource/cache/readiness checks before preparing expensive campaigns,
  inventories or admission paperwork. Preserve a failed preflight as incomplete;
  retry only after its relevant condition changes. Continue useful independent
  work without repeatedly submitting the same blocked campaign.
- Keep genuine focused RED→GREEN during implementation. Complete a coherent
  bounded slice and its planned cases, then format before freezing the expensive
  qualification candidate. Select affected checks and indirect consumers through
  [regression selection](qa-regression-selection/SKILL.md); run the applicable full
  phase/release gates at their declared checkpoints. Name required gates, owners
  and checkpoint exits in the existing plan; never move a checkpoint just to
  avoid a missing or failing required check. Later bound-byte changes
  still require configured re-admission and honest candidate-specific reporting.
- Before a large campaign, exercise representative healthy/faulty harness controls
  with the admitted compiler. A negative consumer must fail for its intended rule;
  match the required diagnostic meaning/context rather than incidental rendering
  unless exact text is the contract. Retain a legal-use control; unrelated compiler
  failures do not count. Retain mechanism-specific sanitizer controls.
- Reuse the existing plan, campaign recipe and source-work accounting tools where
  applicable. Add another script, inventory or receipt layer only for a concrete
  enforcement gap or configured-runner requirement. Keep necessary accounting in
  the existing evidence chain; avoid duplicate full campaigns across workers.
- When verification cost becomes material, measure acquisition, hashing/admission,
  compilation and test execution separately in the existing task record. Profile
  the dominant cost before changing worker counts or adding tools. Filesystem
  metadata and serial admission work need not scale with twelve CPU slots.
- Improve tool-store placement or admission implementation through a scoped,
  reviewed change preserving exact identity, read-only inputs and configured
  per-command custody. Paths, version strings, mtimes and file sizes alone cannot
  prove unchanged payloads. Never skip required re-admission, drop limits or lower the 90% coverage/95% mutation,
  security or native-evidence requirements to improve throughput.
- Report completed acceptance, actual execution, setup failures and remaining
  product capabilities separately. Give any estimated percentage a declared scope
  and basis; changing coarse estimates cannot establish development velocity.

## Product-specific evidence

Use [numerical and protocol truth](references/numerical-protocol.md) for time models, packets and strict contracts; [Windows evidence](references/windows-evidence.md) for OS/UI/package claims; and [Rust techniques](references/rust-techniques.md) when selecting runner, property, fuzz, model or mutation tooling. Load only the relevant reference.

Apply the current [UI-M0 scope](../../../docs/windows-time-architecture.md#165-restricted-first-ui-milestone-ui-m0) before UI analysis or qualification. Additional tools do not activate deferred D1/D2/D3 work or permit live service, network or clock effects in that fixture-only milestone.

The architecture retains **90% executable-line coverage for security-primitive modules** and **95% mutation score for each critical source file**. Branch evidence and named adversarial cases remain separate. Preserve the reviewed critical-file manifest and denominator; no favorable repository average, empty selection or unviable mutant counts as a pass. These are implementation/release obligations. A skill/documentation edit does not require nonexistent application gates.

Mutation qualification also reconciles requirement-bearing functions/guards with generated faults, including automatically skipped unsafe functions and macro/build-generated behavior. Record uncovered scope and its independent alternative evidence through [mutation testing](qa-mutation-testing/SKILL.md). A complete nonempty generated inventory and passing percentage do not establish coverage of every critical function; never add ungenerated behavior to kills or equivalent-mutant exclusions.

## Coordination and closeout

For substantial multi-worker/interrupted work, keep one discoverable coordinator record: source/candidate identity, scope, owners, consumed budgets, live processes, evidence paths, unresolved gaps; use [context continuity](qa-context-checkpoint/SKILL.md). Small local work: existing task record suffices.

Whole-system stabilization/explicit input-to-output acceptance requires [black-box acceptance](qa-black-box-acceptance/SKILL.md) on integrated frozen candidate. Directly inspect representative actual admitted inputs/status/persisted/package outputs against independent facts. Reports cannot replace observation.

Report checked scope, exact command/configuration/target/environment, tested identity, observed results, controls, substitutions, remaining gaps. Separate behavioral GREEN, engineering review, native Windows qualification, staged-tree readiness, release approval. Report only completed stages; scoped pass proves neither full correctness nor ASVS/WCAG certification.

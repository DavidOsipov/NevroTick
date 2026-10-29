# NevroTick agent guide

## Approach: the skeptical expert

Prioritize accuracy and intellectual honesty. Verify consequential/uncertain
claims against strongest available evidence; test competing explanations that
could change conclusions. Distinguish facts, inference, opinion, uncertainty.
Numerical confidence requires explicit method. Correct mistaken assumptions
firmly and respectfully.

Keep investigation/verification proportional. Reuse evidence while relevant state
unchanged. Prefer repository skills over overlapping plugins unless user selects
plugin; load another skill only for concrete gap.

## Start each task

1. Resolve the checkout with `git rev-parse --show-toplevel`; inspect
   `git status --short` and applicable nested `AGENTS.md` before edits. Preserve
   existing work and inspect current manifests/source rather than assuming a
   historical repository state.
2. Read [the documentation guide](docs/README.md), then the relevant authority
   below. Use its [plan and acceptance map](docs/README.md#plans-acceptance-and-execution-evidence)
   to identify the workstream, phase/slice, prerequisites and observable exit.
3. Reuse the existing task/plan. For current UI work, start at
   [tasks/ui-m0.md](tasks/ui-m0.md): scope, ownership,
   [acceptance](tasks/ui-m0.md#acceptance-and-checks),
   [remaining evidence](tasks/ui-m0.md#remaining-evidence-and-next-work) and later
   candidate addenda. For other work, inspect `tasks/` and any `.agents/plans/`
   records; follow [the planning guide](.agents/skills/planning/README.md) when
   creating or revising a plan. Keep one plan/evidence chain.
4. Select the smallest relevant [skill set](.agents/skills/README.md) and read
   its `SKILL.md`. Before selecting checks, read
   [the testing contract](.agents/skills/testing/CONTRACT.md). State the applicable
   requirement/acceptance IDs, independent expected result, source owner and
   required environment in the existing task record or a concise in-chat plan.
5. Use [the configured verification entry points](TOOLING.md#starting-implementation)
   for implementation checks. At handoff, record the candidate, actual results,
   remaining gaps/deferrals and next action. Planned criteria, historical passes
   and cross-builds do not establish current native or release qualification.

Subagent assignments include the bounded question, canonical sections and
acceptance IDs, current task/candidate, writable scope and expected evidence.
The coordinator reconciles results against the integrated candidate.

## Read the right authority

NevroTick: chrony-derived Rust NTP/NTS client, native GUI; initially supported
Windows 11 x64, `x86_64-pc-windows-msvc`. Start with
[docs/README.md](docs/README.md), then task-relevant sections:

| Document | Authority and use |
| --- | --- |
| [Owner decisions](docs/nevrotick-owner-decisions.md) | Accepted instructions, pending recommendations and individual owner-decision procedure. Read before changing reuse, correction, source defaults or release policy. |
| [Research handoff](docs/nevrotick-research-handoff.md) | Remaining evidence assignments, NTS provider candidates and decision gates; research findings are not implementation qualification. |
| [Architecture](docs/windows-time-architecture.md) | Canonical technical/security contracts, ownership, decisions, delivery phases, CHECK requirements. |
| [Feature plan](docs/windows-time-features.md) | Proposed V1 capabilities, journeys, presentation, acceptance scenarios, delivery slices; architecture controls technical semantics. |
| [Development hardening](docs/windows-time-development-hardening.md) | Enforcement/qualification policy; inspect actual configuration and TOOLING.md for adopted bootstrap checks versus pending application/native evidence. |
| [ASVS applicability](docs/asvs-5.0.0-applicability.md) | Applicability/planned verification; no completed compliance. |
| [Signing guide](docs/windows-time-signing-guide.md) | Provider, signing, complete-package workflow; eligibility/native verification require evidence. |
| [Update assessment](docs/windows-time-update-assessment.md) | Retained research; TUF/automatic updater not adopted. |

Accepted owner decisions govern their stated scope; recommendations remain
proposals. Supporting reviews explain decisions/corrections; neither replace
current contracts nor prove implementation. Preserve canonical
REQ/BND/DEC/THR/CHECK identities.

For a material unresolved choice, prepare the decision card in NV-DEC-001 with
evidence, options, recommendation and the dependent action, then ask David.
Routine work within accepted contracts continues. Record the outcome in the
canonical documents; do not treat silence or research deferral as risk acceptance.
OD-05 already accepts the Foundation publisher display, David as release approver
and an initial OpenPGP alternative if free Authenticode is unavailable; the
profile-specific trust and Windows qualification gates remain.

Windows exploit-protection work: read [architecture section 15.4](docs/windows-time-architecture.md#154-per-process-mitigation-profiles-and-startup-admission)
and [hardening section 6.4](docs/windows-time-development-hardening.md#64-per-process-mitigation-qualification).
Keep per-process roles, activation phase, required/optional policy, effective state
and native evidence distinct; existing compiler flags prove no complete runtime profile.

Shared OS/IPC work: read [architecture section 3.4](docs/windows-time-architecture.md#34-shared-windows-adapters-and-diagnostics). Reuse narrow role-specific `windows-platform` adapters and portable `ipc-contract`; shared code grants no privilege or generic Windows command surface.

Engine runtime/concurrency work: read [architecture section 18.6.1](docs/windows-time-architecture.md#1861-engine-runtime-decision-and-alternatives), section 17.1 and the TLS execution gate in section 18.7.1. Tokio remains an engine-only current-thread prototype; qualify bounded native work and the complete NTS-KE arrangement. Its selection does not require `tokio-rustls` or extend runtime dependencies to the core, broker, UI or helper.

IPC implementation/security work: read [architecture section 5.4](docs/windows-time-architecture.md#54-native-ipc-implementation-and-qualification) and [hardening section 9.9](docs/windows-time-development-hardening.md#99-ipc-identity-framing-and-native-lifetime-verification) alongside `security-local-ipc`. Qualify actual wrapper rights, thread-bound caller capture, fatal reversion, continuous endpoint ownership and partial/native I/O completion; examples and mocks do not prove Windows enforcement.

Logging/verbosity work: read [architecture section 12.3](docs/windows-time-architecture.md#123-logging-and-support) and [hardening section 9.8](docs/windows-time-development-hardening.md#98-diagnostics-and-shared-adapter-verification). Keep per-process sinks, typed fields, required-event routing and locally expiring authorized DEBUG sessions explicit; diagnostics are not authoritative UI state or the clock journal.

The current [Cargo workspace](Cargo.toml) contains
[nevrotick-ui](crates/nevrotick-ui/Cargo.toml): a portable preview model and native
Windows UI-M0 adapter, and [ntp-wire](crates/ntp-wire/Cargo.toml): the pure inert
wire owner under [S1.1a qualification](tasks/s1.1-header.md) and
[S1.1b framing qualification](tasks/s1.1-framing.md). It provides structural
framing without exchange, era, authentication or sample admission.
[time-domain](crates/time-domain/Cargo.toml) is the third member under
[S1.1c](tasks/s1.1-time-domain.md): checked numerical/era/clock-domain work in
progress, with no network, clock or authentication effects.
Other proposed components remain architecture plans until their source exists.
Inspect current manifests/source before choosing commands;
never scaffold product crates merely to pass documentation/tool checks.

## Current UI milestone

Apply [architecture §16.5](docs/windows-time-architecture.md#165-restricted-first-ui-milestone-ui-m0)
before UI/QA skills. UI-M0 is an unelevated, fixture-only native prototype.
**D1** theme/contrast/text/DPI work, **D2** production freshness/reconnect/policy/
administrative-result handling and **D3** full keyboard/Narrator/UIA work are
**Deferred**: do not implement or qualify them now, or make them UI-M0 blockers.
Retain standard controls' inherited behavior and labels, Rust/FFI quality checks,
sample-data labelling, disabled protected actions and native window observations.
No live network/DNS, production IPC, persistence, services, elevation or clock
effects. Carry deferred owners/triggers into the backlog and next-stage plan;
restore them before the integrations/preview claims specified in §16.5.
UI-M0 does not complete P0 toolkit qualification or constrain other workstreams.

For UI work, use David's [design v0.2 reference](docs/nevrotick-design-v0.2/README.md):
read the [UI specification](docs/nevrotick-design-v0.2/docs/02-ui-spec.md), then consult
the [interactive preview](docs/nevrotick-design-v0.2/NevroTick-UI-preview.html) and
[screen catalogue](docs/nevrotick-design-v0.2/screens/README.md) for composition and
interaction references. Use the [AVIF brand kit](docs/NevroTick-AVIF-kit-v1/README.md)
and its [manifest](docs/NevroTick-AVIF-kit-v1/manifest.json) for mascot, logos,
wordmarks, icons and background variants; preserve its asset notices separately
from the code licence. Apply these references within the milestone above. The
React viewer and archived architecture copies are reference material; current
canonical contracts govern behavior, and rendered scenes may predate specification
corrections.
The archived React tooling has open dependency findings recorded in the
[UI-M0 task](tasks/ui-m0.md#publication-and-hosted-qualification). Resolve and verify
them before executing that tooling; the native Rust build does not use it.

## Architectural constraints

Read canonical sections for exact limits; preserve these boundaries:

- Separate network-facing engine without clock privilege, narrow privileged
  clock broker, normal-user Win32 UI, elevated lifecycle helper. Deliver one
  complete per-machine MSI. Suggested crate boundaries propose ownership;
  scaffolding every crate immediately not required.
- Treat product user input and external programmatic input as potentially
  malicious. UI controls, local origin, authenticated transport, administrator
  identity or prior acceptance never establish payload safety. Bound and validate
  each applicable admission path, then independently check current authority at
  the effect. Preserve raw-input tests, direct-handler bypass cases, valid
  neighboring controls, aggregate budgets and bounded redacted rejection.
- Fixed JSON IPC/configuration/persisted-record boundaries: bounded strict Serde
  DTO decoding, adopted shared garde rules invoked inside fallible private
  domain construction, current trusted
  authorization/generation/budget checks. Serialize protected decisions/effects
  and actual acknowledgements. Never normalize through `serde_json::Value` to
  bypass strict admission or introduce second runtime schema engine.
- Every function/module boundary, including private helpers, has explicit input
  invariants, output guarantees, failure/effect and lifecycle obligations. Enforce
  them through typed signatures, private checked construction, checked
  transformations and current owner guards. Reuse immutable validated values;
  recheck changed representations, state and context. Never rely on optional
  caller validation, `debug_assert!`, public raw fields or unchecked mutations.
  Follow [contract enforcement](.agents/skills/implementation/implement-rust-contracts/references/contract-enforcement.md):
  no `garde(allow_unvalidated)`; each skipped field needs an actual invariant owner
  and independent review. Garde adoption does not qualify a product Cargo graph.
- Parse NTP/NTS with bounded protocol-specific byte parsers. Preserve interval
  majority selection/configured minimum agreement; no automatic single-peer
  profile after disagreement.
- Keep timestamp units and UTC, monotonic, active-awake clocks distinct. Positive
  offset means Windows behind. Estimator history reflects actual acknowledged
  applied correction, not merely requested correction.
- Preserve the existing NTS-only baseline while resolving OD-03's Pool/default-
  profile interaction; no unqualified shipped preset or silent trust downgrade.
  Plain-NTP profiles stay explicit and visibly labelled. Bootstrap hints support only bounded
  provisional TLS-clock recovery; authorize neither global clock writes nor
  authenticated-tracking claims.
- OD-02 makes very large authenticated recovery a qualification goal. Era/date,
  quorum and correction bounds remain unresolved; 12 hours is not an approved
  plain-NTP constant. Keep clock ownership, stale-state rejection, cumulative correction allowances,
  sleep/resume invalidation, W32Time handover/restoration explicit. Ordinary
  “Sync now” requests fresh bounded check; grants neither arbitrary clock
  correction nor administrative policy changes.
- Bound actual native work/resources. Dropping/aborting awaiting future does not
  establish native-job completion; permits/custody follow real operation through
  completion/recovery.
- Keep unsafe code in narrow reviewed Windows adapters, preserve useful errors,
  redact secrets from events/exports. UI status accurately distinguishes
  authentication, agreement, freshness, readiness, applied effects.
- Preserve chrony source provenance and GPL-2.0-only obligations. Qualify exact
  dependency/features/license graphs and held crypto-provider choices before
  adoption. Keep `Cargo.lock` versionable; exclude `target/` from Git/search indexes.

## Choose skills by the changed guarantee

Use [the skills index](.agents/skills/README.md); load only relevant leaves and
conditional references. Small understood edit needs concise action/check record,
no new report tree or orchestration campaign.

| Work | Start with |
| --- | --- |
| Uncertain scope, stages or ownership | [plan-code-change](.agents/skills/planning/plan-code-change/SKILL.md); architecture/threat/review skills when their decisions change. |
| Rust logic, concurrency or Windows adapters | [implement-rust](.agents/skills/implementation/implement-rust/SKILL.md), including conditional SCM/chrony/simulator references. |
| Windows service architecture, SCM lifecycle or recovery | [Service adapter guide](.agents/skills/implementation/implement-rust/references/windows-service-adapters.md), with per-service recovery and object-access matrices; [Windows privileges](.agents/skills/security/security-windows-privileges/SKILL.md) for authority. |
| Function/module contracts, wire admission and domain invariants | [implement-rust-contracts](.agents/skills/implementation/implement-rust-contracts/SKILL.md). |
| Wire, persisted-state or semantic compatibility | [evolve-data-contracts](.agents/skills/implementation/evolve-data-contracts/SKILL.md). |
| Native UI and accessibility | [implement-native-ui](.agents/skills/implementation/implement-native-ui/SKILL.md), then applicable accessibility/DPI checks. |
| Trust, privileges, IPC, crypto, logging or packaging | [Security index](.agents/skills/security/README.md), matching boundary owner. |
| Cargo dependency changes | [dependency-updates](.agents/skills/environment/dependency-updates/SKILL.md). |
| Verification, failure triage or commit preparation | [Testing index](.agents/skills/testing/README.md), [testing contract](.agents/skills/testing/CONTRACT.md); smallest relevant evidence layer. |

Use [upstream specialists](.agents/skills/upstream/README.md) for actual
compiler/borrow/Send-Sync problems or qualified Windows UI automation. Skills hold
reusable instructions, references, templates, checkers, required inventory/licence
provenance. Keep one-off reviews/logs/task receipts in existing task location,
outside skills pack.

## Tool selection

Resolve absolute checkout root with `git rev-parse --show-toplevel`; pass
explicitly to project-aware tools. Inspect exposed tool schema before use;
historical setup/version receipts in [TOOLING.md](TOOLING.md) provide context,
not current-connection guarantees. Choose tool for current question; ordinary
edits need not use every tool.

### Serena: exact Rust symbols and safe symbol edits

Once Rust source exists: definitions, implementations, references, diagnostics,
symbol-aware edits. Call `initial_instructions` first, then `activate_project`:
`project` = absolute checkout; `session` = returned **Serena session ID**, not
MCP/client ID. Use symbol overview, definition/body, referencing-symbol queries
as needed. Retrieve body before replacement; inspect affected callers.
`.serena/project.yml` selects Rust; restart Serena connection after project
configuration changes. Empty application symbol graph before source expected.

### SocratiCode: discover owners and retrieve architectural context

Semantic search for unclear owner/terminology; relationship queries for cross-file
impact. Pass absolute `projectPath`; check `codebase_status`, wait for completed
indexing before semantic code search. List `codebase_context`, then
`codebase_context_search` with actual `artifactName` from
[.socraticodecontextartifacts.json](.socraticodecontextartifacts.json).
Read canonical file directly if unregistered/unindexed. Respect configured watcher
mode; `codebase_update` for necessary immediate catch-up. Confirm consequential
results in current files/callers; index locates evidence.

### Probe: fast source search without persistent indexing

Use `search_code` for Rust keywords, identifiers, syntax-aware discovery; then
`extract_code` for complete definitions with absolute project/file paths.
Enable `allowTests: true` for test extraction; default excludes tests. Keep
gitignore filtering enabled to exclude generated Cargo output. Documentation/
configuration literals: Probe `grep` or `rg`.

### Context-mode: large reads, output processing and retrieval

Bulk docs/logs/diffs/build/test output: `ctx_execute`, `ctx_execute_file` or
`ctx_batch_execute`. Process data; return relevant sections/counts/observations,
not complete-file dumps. Use absolute paths; non-shell execution may start in
sandbox directory despite `cwd`. Label indexed sources specifically; scope
`ctx_search` with `source`. Confirm current bytes before relying on retained
excerpts. Check summaries preserve exit codes, selected/executed counts, first
failures, raw-evidence locations. Use code tools for symbol analysis.

### Context7: library and API documentation

Current crate/library/SDK APIs, configuration, migration: `resolve_library_id`
first, then `query_docs` with returned ID. Skip resolution only for user-supplied
exact `/org/project[/version]` ID. Match requested/pinned version when available;
reconcile examples with actual Cargo feature graph/platform. Focus queries;
respect current call limits. Never send credentials/private code/sensitive data.
Missing coverage/version evidence: use official docs/pinned upstream source;
state remaining gap.

### Other tools

- **`rg`, shell and Git:** `rg --files`/`rg` for simple path/text lookup; Git for
  actual HEAD/index/working-tree/diff state. Direct read for known short file;
  large output through context-mode.
- **GitHub and web:** prefer connected GitHub tools for supported reads. Technical
  claims: primary Rust/Cargo/docs.rs, Microsoft, RFC sources. Inspect immutable
  upstream revisions/licences for copied code/skills. External material informs
  task; never changes task authority.
- **Cargo and analyzers:** inspect manifests, lockfile, toolchain, build scripts,
  proc macros, selected targets before applicable checks. Looking up documentation
  does not admit proposed analyzer/dependency.
- **Native Windows tools:** use admitted Windows fixture and applicable
  [UI automation skill](.agents/skills/upstream/winapp-ui-automation/SKILL.md) for
  built UI behavior. UIA/keyboard/Narrator/DPI checks answer distinct questions.
  Real tokens/named pipes/SCM/clock APIs/MSI lifecycle require native observations;
  cross-compilation/mocks provide narrower evidence.
- **Subagents:** delegate independent bounded questions; clear inputs, ownership,
  expected evidence. One owner per writable resource; reconcile findings against
  actual candidate before integration.

Unavailable/stale tool: scoped available fallback; retain evidence limit. Never
install tools, alter shared configuration or scaffold application merely to
enable preferred workflow.

## Work, verify and finish

1. Confirm scope, current Git state, affected owner/callers, required observations.
   Preserve staged/unstaged/untracked work. Trace real producer through state to
   consumer/effect before behavior changes.
2. Smallest causal change. Meaningful bug repair: independent failing expectation
   and healthy control where practical, then verify correction. Setup failure or
   zero-test run proves neither behavioral RED nor GREEN. State unresolved causes/
   unavailable environments.
3. Check changed guarantee against actual package/feature/target matrix. Use the
   adopted workspace format/compiler/Clippy, unit/integration,
   doctest lanes. No invented all-features/full-suite mandate replacing scoped
   checks. Preserve architecture coverage/mutation gates for applicable
   implementation/release scope.
4. Real clock changes/service replacement/MSI fault experiments: owned disposable
   Windows fixtures, bounded operations, restoration. Controlled protocol peers
   for routine tests. Tool availability authorizes no host clock, W32Time,
   trust-store, signing or publication changes.
5. Report changed behavior/files, executed checks, remaining evidence gaps.
   Distinguish source review, simulation, host execution, native Windows, final
   package/signature evidence. Stage/commit/push only within user's existing
   authorization; preserve concurrent work, configured signing/hooks.

Skill-pack changes: run package checker from repository root:

```sh
python3 -B .agents/skills/scripts/check_skills.py
```

Run `python3 -B .agents/skills/scripts/test_check_skills.py -v` when changing
checker/covered contracts. Retain upstream inventory hashes/notices when
intentionally changing imported leaves.

Documentation changes: check links/canonical consistency. Verify
`docs/SHA256SUMS.txt` **when present**; update existing manifest when intentionally
changing covered documents. Never claim bundle integrity from missing/stale
manifest. Keep transient plans/reports outside maintained documentation bundle
unless task explicitly places them there.

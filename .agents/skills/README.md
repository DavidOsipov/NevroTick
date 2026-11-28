# NevroTick skills

52 David Osipov website workflow adaptations, four selected upstream specialists
and twelve original NevroTick workflows for the Windows time client in
[the documentation guide](../../docs/README.md) and its development host.
Select smallest task-matching set. Names/descriptions support discovery; load
full instructions/conditional references when needed. Index navigates; never
requires loading every skill.

Before choosing a workflow, locate the existing task, phase/slice and acceptance
IDs through [the plan and acceptance map](../../docs/README.md#plans-acceptance-and-execution-evidence).
For UI-M0, reuse [its task record](../../tasks/ui-m0.md) and canonical restricted
scope; a skill supplies the method, not a new product requirement or evidence of
completion. Read [the testing contract](testing/CONTRACT.md) before selecting checks.
WSL host maintenance uses the requested storage scope and preservation checks;
product acceptance IDs and native qualification gates do not apply to that work.

For the whole development workflow, use the
[lifecycle skill map](../../docs/plans/development-plan.md#development-lifecycle-and-skill-ownership)
and [development delegation](planning/references/testable-slices.md#development-delegation).
Select skills by lifecycle stage, product phase and changed guarantee. Planning
coordinates decisions, implementation and delivery; testing coordinates the
verification portion. Each applicable owner supplies its concrete work product
and evidence, with shared decisions and integration reconciled centrally.

| Task | Start with |
| --- | --- |
| Implement Rust ownership, errors, concurrency or Windows adapters | [implement-rust](implementation/implement-rust/SKILL.md) |
| Adopt Rust 1.100 APIs or diagnostics while preserving explicit contracts | [Feature contracts](implementation/implement-rust/references/rust-1.100-contracts.md) under `implement-rust`; [lint enforcement](testing/references/rust-1.100-lints.md) under `qa-static-checks` |
| Implement typed errors, safe diagnostic projection, retry or cleanup failure handling | [Error-handling procedure](implementation/implement-rust/references/error-handling.md) under `implement-rust`; [privacy/logging](security/security-privacy-logging/SKILL.md) for output sinks |
| Optimize a measured CPU, allocation, latency, wakeup, startup or size cost | [optimize-rust-performance](implementation/optimize-rust-performance/SKILL.md); [resource QA](testing/qa-performance-resources/SKILL.md) for measurement/acceptance |
| Implement or review x64 baseline admission and runtime CPU-feature dispatch | [implement-cpu-dispatch](implementation/implement-cpu-dispatch/SKILL.md); pair with [performance optimization](implementation/optimize-rust-performance/SKILL.md) for measured adoption |
| Create, manage, harden, diagnose or optimize Windows services | [implement-windows-service](implementation/implement-windows-service/SKILL.md); detailed [service adapter guide](implementation/implement-rust/references/windows-service-adapters.md) and boundary-specific security/QA when needed |
| Enforce function/module contracts, IPC/configuration admission or domain invariants | [implement-rust-contracts](implementation/implement-rust-contracts/SKILL.md) |
| Implement numerical kernels or adapt upstream algorithm semantics | [Numerical procedure](implementation/implement-rust/references/numerical-kernels.md) and [Chrony caller-path extraction](implementation/implement-rust/references/chrony-port-simulator.md) under `implement-rust` |
| Change wire, persisted or domain compatibility | [evolve-data-contracts](implementation/evolve-data-contracts/SKILL.md) |
| Implement native UI presentation, interactions or status | [implement-native-ui](implementation/implement-native-ui/SKILL.md) |
| Implement or debug Slint components, Rust models, event-loop ownership or resources | [implement-slint](implementation/implement-slint/SKILL.md), alongside native-UI product semantics |
| Harvest, maintain, generate or reconcile Windows product/package metadata | [maintain-windows-metadata](implementation/maintain-windows-metadata/SKILL.md); [package-artifact QA](testing/qa-package-artifacts/SKILL.md) owns final artifact/native qualification |
| Plan a change, architecture, review or threat model | [Planning guide](planning/README.md) |
| Change privilege, IPC, NTS, logging or installation security | [Security guide](security/README.md) |
| Choose or run testing and qualification | [Testing guide](testing/README.md) |
| Qualify applicable sanitizers at module completion or before V1; assess a named diagnostic mechanism and target | [qa-sanitizer-qualification](testing/qa-sanitizer-qualification/SKILL.md), with selected family manuals and conditional mechanism guides |
| Challenge scheduling races, generated histories or Python admission semantics | [Tool-selection guide](testing/references/generated-testing-selection.md), then [lifecycle](testing/qa-state-concurrency-lifecycle/SKILL.md), [model](testing/qa-model-based-testing/SKILL.md) or [property](testing/qa-property-based-testing/SKILL.md) testing; extensive conditional tool manuals preserve admission and evidence limits |
| Explore semantic states that revisit familiar code paths | [qa-state-guided-fuzzing](testing/qa-state-guided-fuzzing/SKILL.md), with an independent transition oracle |
| Evaluate new or materially revised skills, or demonstrated agent failures; wording-only edits need no campaign | [qa-skill-evaluations](testing/qa-skill-evaluations/SKILL.md) |
| Investigate native heap/handle faults or retained resources | [Windows memory diagnostics](testing/references/windows-memory.md), then [resource QA](testing/qa-performance-resources/SKILL.md) for retention |
| Audit logical data/state chains or trace Rust inputs and secrets to effects | [qa-data-flow-analysis](testing/qa-data-flow-analysis/SKILL.md) |
| Independently audit commits/ranges with parallel logic, security and QA specialists plus a fresh judge | [qa-independent-commit-audit](testing/qa-independent-commit-audit/SKILL.md) |
| Assess or change Cargo dependencies | [dependency-updates](environment/dependency-updates/SKILL.md) |
| Analyze WSL storage or clean authorized caches, artifacts and unused Docker data | [wsl-cleanup](environment/wsl-cleanup/SKILL.md) |
| Configure or manage an owned Windows Sandbox from Windows/WSL and collect disposable-lab evidence | [manage-windows-sandbox](environment/manage-windows-sandbox/SKILL.md), with selective operations, configuration, resource, winapp, evidence and troubleshooting references |
| Diagnose a specific compiler, borrow/drop or Send/Sync mechanism; use qualified Windows UIA commands | [Selected upstream specialists](upstream/README.md) |

The [canonical architecture](../../docs/windows-time-architecture.md) controls
product decisions. Use bounded Serde decoding into strict wire records, adopted
shared garde rules inside fallible private domain construction, and current
authorization/effect checks. All function/module interfaces follow
[contract enforcement](implementation/implement-rust-contracts/references/contract-enforcement.md),
including checked transformations and explicit failure/effect obligations.
The normal-user Slint UI, unprivileged network engine, privileged clock broker,
administrative helper and complete package have distinct owners.
[OD-13](../../docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit)
selects Slint as the default and complete Win32 UI replacement. The checked-in
entrypoint now selects Slint; retained Win32 source and receipts are historical.
[S0.5](../../tasks/s0.5-ui-comparison.md) and the
[GUI handoff](../../tasks/slint-gui-handoff.md) own current source, evidence and gaps.
The original GUI uses David's approved GPL-3.0-only direction; Chrony-derived
Core/engine retains GPL-2.0-only. Exact graph/composition, native and package
qualification remain distinct. Prototype crates require dependency qualification.
Apply [OD-05 §7.10](../../docs/nevrotick-owner-decisions.md#710-tuf-authenticated-direct-msix-updater--25-november-2026)
and [architecture §19.8](../../docs/windows-time-architecture.md#198-tuf-authenticated-direct-msix-updater)
for the adopted TUF complete-MSIX updater. One target-bound Update now consent
continues download/verification/installation; independent helper admission and
both trust/time/storage and Windows recovery gates remain required. Client,
keys/provider and native qualification are unresolved. No second product or
resident privileged updater follows; dated research remains historical.

The current [OD-05 §7.9 delivery decision](../../docs/nevrotick-owner-decisions.md#79-direct-msix-distribution-microsoft-store-rejected--21-november-2026)
selects one complete hardened MSIX for direct distribution, including services,
and rejects Microsoft Store publication because of its publisher indemnity.
Use [MSIX hardening §3](../../docs/plans/windows-msix-store.md#3-package-trust-and-the-privileged-helper)
and [its staged work §7](../../docs/plans/windows-msix-store.md#7-bounded-continuation-and-existing-acceptance-owners):
source/package preparation, admitted local/test-signed mechanics, then exact
signed/distributed/installed artifact qualification before release. Signing and
trust choices are deferred; do not block source/resource work on them or require
Store approval, a submission attempt, refusal or Partner Center identity.
Requalify Store-dependent CIG assumptions under the eventual direct-signing
profile; do not infer publisher trust or silently weaken required protections.
[OD-05 §7.7](../../docs/nevrotick-owner-decisions.md#77-sign-every-supported-shipped-artifact--8-november-2026)
retains package and supported individual-signature obligations; it grants no
current release pass. SignPath remains a candidate, not a selected signer. MSI
requires a later decision; no automatic fallback, customer trust-store change or
unsigned-MSI/OpenPGP substitute. Preserve historical sources/upstream signatures.
No direction choice proves implementation, lifts the Windows
stop/separate fuzz hold/storage pause or displaces the current Core task.

Preserve source skills' evidence discipline: existing work, producer → state →
consumer tracing, independent behavioral oracles; distinguish compiler, simulated,
native Windows, packaged-product proof. Inspect [current workspace members](../../Cargo.toml)
and their task records through [the documentation guide](../../docs/README.md);
the UI-M0 and pure protocol/numerical fixture owners have separate evidence scopes.
Use [the shared verifier](../../TOOLING.md#starting-implementation) for the selected
portable, dependency and Windows cross-build rows. Skills establish no passing application test,
native observation or release qualification. UI-M0's D1/D2/D3 deferrals remain.

When selecting existing compiler, lint, test, security or inventory commands, use
[configured tool execution](testing/references/configured-tooling.md). It maps
changed guarantees to current entry points and their execution/evidence limits.
Select Miri, cargo-fuzz, Loom, Lockbud or cargo-vet only for their relevant
memory, input, concurrency or dependency-review obligation. Kani/formal proof
work follows the post-V1 deferral below. For uncertain
source-to-sink propagation, use the data-flow skill and its local CodeQL MCP
profile. Installed tools and fixture controls establish no product qualification;
hosted CodeQL scanning remains deferred.

Apply [OD-10’s first-release deferral](../../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026): formal proof work,
verifier extensions and pending proof admission/metadata repairs resume after V1.
Missing proofs do not block V1 implementation, acceptance or release. Continue
the ordinary product plan and all nonformal gates; keep proof obligations in
the existing post-V1 backlog and historical evidence within its exact scope.
The [proof workflow](testing/references/formal-verification.md) applies on resumption.

## Adaptation and research

[ADAPTATION.json](ADAPTATION.json) maps every original website skill to target and
original entrypoint digest. [LOCAL.json](LOCAL.json) registers original
NevroTick workflows separately, without inventing donor or upstream provenance.
Group research explains Rust/Windows decisions with
dated primary sources:

- [Implementation sources](implementation/SOURCES.md)
- [Environment sources](environment/SOURCES.md)
- [Planning sources](planning/SOURCES.md)
- [Security sources](security/SOURCES.md)
- [Testing sources](testing/SOURCES.md)
- [Attribution and the PdfCraft contribution](PROVENANCE.md)
- [Pinned upstream imports, licenses and local corrections](upstream/README.md)

Canonical files live once here. Keep normal implicit discovery; no global skill/
tool installation needed. Explicit invocation, e.g. `$implement-rust-contracts`,
for precise selection. The pack registers 68 grouped skills without aliases;
inventory validation and current-host discovery are separate checks.

## Maintaining the pack

Run standard-library-only package check from repository root:

```sh
python3 .agents/skills/scripts/check_skills.py
python3 -B .agents/skills/scripts/test_check_skills.py -v
```

Checks supported YAML subset, unique names, UI prompts, relative Markdown links,
source-to-target coverage. Entrypoints require JSON-quoted YAML `name` and
`description`, optional quoted `license` identifier. UI metadata: `interface`
mapping with quoted `display_name`, `short_description`, `default_prompt`. Extend
checker deliberately for changed authoring needs. Compare available original
checkout via `--source-root /path/to/personal-website-templated-1/.agents`.

CLI regressions check exact skill invocations, punctuation, multiple mentions;
reject attached name suffixes/different invocation. [UPSTREAM.json](UPSTREAM.json)
accounts separately for imported leaves, exact commits, retained licenses,
complete installed-file hashes. `LOCAL.json` records locally authored leaves;
its entries do not claim imported-source licensing or hashes.
Checker rejects unregistered leaves, mutable pins,
license-metadata mismatches, missing license files, inventory drift. Recorded
hash alone neither legally clears license nor authenticates unfetched remote source.

Use [skill evaluations](testing/qa-skill-evaluations/SKILL.md) for realistic
independent use exercises when material instruction changes warrant them.
Discovery checks and behavioral exercises provide separate evidence.
Package checks do not test Rust behavior. Select the shared product verifier and
applicable native observations for actual implementation changes.

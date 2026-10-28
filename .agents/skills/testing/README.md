# NevroTick testing skills

Read [CONTRACT.md](CONTRACT.md): shared scope/evidence/closeout rules. Choose only claim-proving leaves. Understood local work needs no orchestrator/full-system campaign. [SOURCES.md](SOURCES.md): all 36 donor mappings/current primary-source research; maintenance provenance, no mandatory runtime reading.

## Plan, coordinate and close

| Skill | Use |
| --- | --- |
| [qa-test-orchestrator](qa-test-orchestrator/SKILL.md) | Coordinate a substantial NevroTick verification task across Rust, protocol, Windows, UI and package evidence without imposing a full suite on local work. |
| [qa-test-planning](qa-test-planning/SKILL.md) | Design a bounded, requirement-grounded test plan for a NevroTick behavior or change before choosing fixtures, environments and tools. |
| [qa-environment-preflight](qa-environment-preflight/SKILL.md) | Establish tool, Cargo, target, Windows-machine and fixture readiness for selected NevroTick checks before substantial execution. |
| [qa-coverage-gap-analysis](qa-coverage-gap-analysis/SKILL.md) | Survey NevroTick behavior independently of existing tests to prioritize unassessed requirements and evidence gaps in a broad audit. |
| [qa-regression-selection](qa-regression-selection/SKILL.md) | Choose the smallest justified NevroTick regression set while accounting for indirect consumers, targets, features and recovery paths. |
| [qa-failure-triage](qa-failure-triage/SKILL.md) | Classify a NevroTick failed check causally before changing production behavior, assertions, fixtures or environment. |
| [qa-system-stabilization](qa-system-stabilization/SKILL.md) | Resolve NevroTick multi-stage or uncertain-coupling failures through finite obligation mapping, causal repair and integrated acceptance. |
| [qa-independent-review](qa-independent-review/SKILL.md) | Independently review NevroTick verification claims, tests and candidate changes against requirements and actual evidence. |
| [qa-subagent-dispatch](qa-subagent-dispatch/SKILL.md) | Dispatch independent NevroTick test/review tasks with clear boundary ownership, evidence identity and bounded shared resource use. |
| [qa-context-checkpoint](qa-context-checkpoint/SKILL.md) | Checkpoint or resume substantial NevroTick verification with current scope, evidence, owners, live effects and consumed budgets intact. |
| [qa-black-box-acceptance](qa-black-box-acceptance/SKILL.md) | Independently assess a frozen NevroTick candidate against known input and output facts after system stabilization or when input-to-output acceptance is requested. |
| [qa-final-qualification](qa-final-qualification/SKILL.md) | Assess final NevroTick verification evidence against a declared scope and exact candidate without promoting plans, skips or host checks into release passes. |
| [qa-pre-commit](qa-pre-commit/SKILL.md) | Review or prepare NevroTick staged content with preserved user staging and verification tied to the complete intended Git candidate. |

## Choose a behavior/evidence layer

| Skill | Use |
| --- | --- |
| [qa-unit-tests](qa-unit-tests/SKILL.md) | Write or review focused Rust unit tests for NevroTick numerical, domain and state logic using independent expectations and controlled effects. |
| [qa-component-tests](qa-component-tests/SKILL.md) | Verify a bounded NevroTick component such as a controller, IPC client or native view with explicit collaborator and platform limits. |
| [qa-contract-tests](qa-contract-tests/SKILL.md) | Verify strict NevroTick IPC, configuration, domain-construction and consumer compatibility contracts through their actual admitted paths. |
| [qa-integration-tests](qa-integration-tests/SKILL.md) | Verify NevroTick behavior across real Rust modules, processes, protocol peers or persistence adapters at an explicitly chosen integration boundary. |
| [qa-e2e-tests](qa-e2e-tests/SKILL.md) | Test a NevroTick user or service journey through built executable entry points and actual observable Windows results. |
| [qa-positive-paths](qa-positive-paths/SKILL.md) | Choose positive controls and expected successful NevroTick paths that prevent rejection-only or disabled-effect implementations from appearing correct. |
| [qa-negative-paths](qa-negative-paths/SKILL.md) | Test NevroTick invalid inputs, unavailable collaborators and denied operations for correct error, atomicity and forbidden-effect behavior. |
| [qa-boundary-edge-cases](qa-boundary-edge-cases/SKILL.md) | Build explicit just-below, exact and just-above NevroTick boundary cases for time, resources, representation and lifecycle rules. |
| [qa-adversarial-inputs](qa-adversarial-inputs/SKILL.md) | Design bounded hostile NevroTick input cases that challenge the actual protocol, IPC, persistence and resource admission path. |
| [qa-security-boundaries](qa-security-boundaries/SKILL.md) | Test NevroTick trust boundaries using real caller identity, current guards and observable protected effects. |
| [qa-state-concurrency-lifecycle](qa-state-concurrency-lifecycle/SKILL.md) | Verify NevroTick asynchronous state, concurrency, cancellation, service and resource lifecycle behavior at its actual ownership boundaries. |
| [qa-fault-recovery](qa-fault-recovery/SKILL.md) | Test NevroTick controlled failures for bounded recovery, preserved authority, durable state and honest status. |

## Choose a focused technique or product surface

| Skill | Use |
| --- | --- |
| [qa-property-based-testing](qa-property-based-testing/SKILL.md) | Use bounded generated cases and shrinking to test a NevroTick invariant with an independent oracle and reproducible counterexamples. |
| [qa-model-based-testing](qa-model-based-testing/SKILL.md) | Test NevroTick controller, authority and recovery sequences against an independently defined transition model. |
| [qa-structured-fuzzing](qa-structured-fuzzing/SKILL.md) | Run or design a bounded NevroTick fuzz target for structured IPC/configuration, NTP/NTS parsing or numeric conversion with useful seeds and explicit oracles. |
| [qa-mutation-testing](qa-mutation-testing/SKILL.md) | Use a bounded NevroTick mutation campaign to assess behavioral assertions, preserving the architecture per-critical-file gate and honest outcome accounting. |
| [qa-golden-fixture-parity](qa-golden-fixture-parity/SKILL.md) | Compare NevroTick captured outputs or pinned reference trajectories with independent semantic facts and a deliberately tested comparator. |
| [qa-static-checks](qa-static-checks/SKILL.md) | Select and validate NevroTick compiler, format, Clippy, source-policy and feature/MSRV checks with meaningful admission controls. |
| [qa-package-artifacts](qa-package-artifacts/SKILL.md) | Verify emitted NevroTick binaries, MSI contents, metadata, provenance and delivered-package consistency independently of source configuration. |
| [qa-accessibility](qa-accessibility/SKILL.md) | Assess NevroTick native control semantics, keyboard use, Narrator behavior and accessibility error/status communication. |
| [qa-visual-dpi](qa-visual-dpi/SKILL.md) | Test NevroTick visual usability across Windows DPI, text scaling, contrast, window sizing and multi-monitor changes. |
| [qa-i18n-localization](qa-i18n-localization/SKILL.md) | Test NevroTick message catalogs, units and native layout for adopted locales while preserving wire and security semantics. |
| [qa-performance-resources](qa-performance-resources/SKILL.md) | Measure NevroTick CPU, memory, wakeups, handles, queues and responsiveness under declared steady, hostile and recovery workloads. |

## Conditional shared references

- [Numerical, NTP/NTS and strict contract truth](references/numerical-protocol.md) for analytic or byte-level oracles, domain construction and input-to-effect obligations.
- [Windows evidence](references/windows-evidence.md) for native tokens/IPC/clock/SCM, complete MSI lifecycle, UIA/Narrator and DPI proof.
- [Rust techniques](references/rust-techniques.md) for Cargo/feature/MSRV selection, generated tests, modeled concurrency and optional tools.
- [Staged candidate](references/staged-candidate.md) for faithful full-index review when commit-candidate readiness is requested.

## Maintenance

Keep one shared-guidance copy/valid leaf-relative links. Use root skills router pack checker/metadata validation for structure/discovery claims. For material instruction changes, exercise representative requests/raw requirements; assess observed choice/evidence, including valid controls/unavailable capabilities. Structural checks prove no task-selection quality, native host discovery or application behavior. Do not copy website Node/Vitest wrappers or proposed PdfCraft xtask commands.

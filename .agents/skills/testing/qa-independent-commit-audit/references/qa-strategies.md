# Reconcile the full QA strategy surface

Read the live [testing index](../../README.md) and enumerate current testing
`SKILL.md` descriptions. The groups below route existing owners; they are not a
closed inventory. Classify every current strategy/workflow for this audit, adding
newly discovered owners. Read full instructions only for applicable leaves and
triggered references. Audit completeness requires applicability accounting,
not execution of every technique, framework or installed tool.

Use a compact table in the existing coordinator record:

| Changed guarantee / consumer | Strategy and skill owner | Independent healthy/refused expectation | Source/test/raw evidence | Disposition and next action |
| --- | --- | --- | --- | --- |

Use the existing disposition vocabulary: planned; executed with result; qualified
evidence reused; blocked/incomplete; N/A with applicability evidence; deferred by
cited accepted decision. Separately state whether a needed test exists and is
adequate. Missing tools, uninspected artifacts or a held applicable run are not
N/A. For source review, read the actual inputs, calls and assertions before
crediting coverage; test names, counts and framework imports only locate evidence.
Grouped owners may have different dispositions; preserve those differences.

## Strategy owners and questions

| Group | Existing skill owners | What the audit must challenge when applicable |
| --- | --- | --- |
| Scope and omitted requirements | [Planning](../../qa-test-planning/SKILL.md), [coverage gaps](../../qa-coverage-gap-analysis/SKILL.md), [regression selection](../../qa-regression-selection/SKILL.md) | Derive obligations independently of existing tests. Include indirect consumers, versions/features/targets and release-relevant effects; explain exclusions. |
| Pure logic and component behavior | [Unit](../../qa-unit-tests/SKILL.md), [component](../../qa-component-tests/SKILL.md), [positive paths](../../qa-positive-paths/SKILL.md) | Independent numerical/domain truth, decision combinations, useful healthy neighbors and actual component effects. Mocks cannot establish the replaced boundary. |
| Contracts and representations | [Contract](../../qa-contract-tests/SKILL.md) | Constructor/transform/mutation bypasses, strict raw admission, units/owner/generation mismatch, direct consumers, external or sibling compile-fail plus legal-use controls. Attributes and type annotations alone establish no runtime validation. |
| Rejection and limits | [Negative](../../qa-negative-paths/SKILL.md), [boundary](../../qa-boundary-edge-cases/SKILL.md), [adversarial](../../qa-adversarial-inputs/SKILL.md) | Reach the intended guard; test below/exact/above bounds, valid-shaped unauthorized inputs, aggregate pressure and late failure after valid progress. Preserve old state and forbid premature effects. |
| Module-to-product composition | [Integration](../../qa-integration-tests/SKILL.md), [E2E](../../qa-e2e-tests/SKILL.md), [black-box acceptance](../../qa-black-box-acceptance/SKILL.md) | Real producer/adapter/consumer, wrong-but-well-typed inputs, actual entrypoints and independently expected outputs. Separate fixture, cross-build, native and delivered-product evidence. |
| Generated input invariants | [Property testing](../../qa-property-based-testing/SKILL.md) | Independent invariants/metamorphic relations, realistic and extreme domains, useful shrinking and concrete replay. Existing deterministic cases may establish a property without a named generator. |
| State, schedules and recovery | [Model-based](../../qa-model-based-testing/SKILL.md), [state/concurrency/lifecycle](../../qa-state-concurrency-lifecycle/SKILL.md), [fault recovery](../../qa-fault-recovery/SKILL.md) | Independent transition oracle, intermediate effects, event ordering, interleavings, retry/cancel/restart, invalidation and lost acknowledgements. Trace actual native completion and preservation of spent authority. Final state alone can hide transient violations. |
| Fuzz exploration | [Structured fuzzing](../../qa-structured-fuzzing/SKILL.md), [state-guided fuzzing](../../qa-state-guided-fuzzing/SKILL.md) | Actual boundary reachability, valid/invalid seeds, real feedback, independent oracles, bounded aggregate cost and fresh replay. Line novelty and semantic-state novelty differ. Preserve scoped holds without treating them as blanket exemptions. |
| Oracle independence | [Golden/differential parity](../../qa-golden-fixture-parity/SKILL.md) | Pinned independent reference, meaningful tolerances, actual delivered input and comparator sensitivity. Shared algorithms, round trips or self-generated expected output can agree incorrectly. |
| Assertion strength and metric honesty | [Mutation](../../qa-mutation-testing/SKILL.md) | Per-critical-file denominator, viable/generated/completed outcomes, real kills, domain-grounded equivalence and incomplete runs. Inspect skipped unsafe functions, macros/build-generated behavior and other ungenerated guards; retain independent alternatives without inventing mutation credit. |
| Static enforcement and logical flow | [Static checks](../../qa-static-checks/SKILL.md), [data-flow analysis](../../qa-data-flow-analysis/SKILL.md) | Effective lint/type/configuration scope, suppressions, unchecked conversions, actual source-to-effect paths and analyzer controls. An empty alert set or successful compilation does not establish semantic correctness. |
| Security boundaries | [Security QA](../../qa-security-boundaries/SKILL.md) and affected [security owner](../../../security/README.md) | Actual identities/current authorization, lower-trust data, protected sinks, same-input healthy controls, confidentiality/integrity/availability consequences and forbidden effects. Admin/local/authenticated origin grants no payload safety. |
| Artifact and native behavior | [Package artifacts](../../qa-package-artifacts/SKILL.md), [Windows evidence](../../references/windows-evidence.md) | Actual emitted/loaded/installed bytes, role/publisher/release identity, provisioning, mitigation and servicing/recovery. Preserve distinct source, build, signed, distributed and installed identities. |
| Memory, performance and resource lifetime | [Performance/resources](../../qa-performance-resources/SKILL.md), [sanitizer qualification](../../qa-sanitizer-qualification/SKILL.md), [Windows memory diagnostics](../../references/windows-memory.md) | Bounded CPU/memory/queues/wakeups, sustained and hostile loads, measured retained allocations/handles/tasks, real cleanup. Reconcile module/pre-V1 sanitizer applicability, actual product instrumentation and controls; distinguish semantic controls from detector-specific faults and scope each platform/tool. |
| Native UI and language | [Accessibility](../../qa-accessibility/SKILL.md), [visual/DPI](../../qa-visual-dpi/SKILL.md), [i18n](../../qa-i18n-localization/SKILL.md) | Keyboard/Narrator/UIA, scaling/contrast/layout, critical wording and typed data presentation, hostile catalog admission and real freshness/authority. Keep accepted milestone deferrals and their restoration triggers. |
| Changed instructions and discovery | [Skill evaluations](../../qa-skill-evaluations/SKILL.md) | Realistic fresh-agent task, hidden independent expected result, healthy/false-positive control, actual method compliance and truthful outcome. Structural package validation proves no review quality. |
| Execution and integrated closeout | [Orchestrator](../../qa-test-orchestrator/SKILL.md), [preflight](../../qa-environment-preflight/SKILL.md), [triage](../../qa-failure-triage/SKILL.md), [stabilization](../../qa-system-stabilization/SKILL.md), [independent review](../../qa-independent-review/SKILL.md), [dispatch](../../qa-subagent-dispatch/SKILL.md), [checkpoint](../../qa-context-checkpoint/SKILL.md), [final qualification](../../qa-final-qualification/SKILL.md), [pre-commit](../../qa-pre-commit/SKILL.md) | Review their applicability as workflows, not extra test types. Preserve actual selection, exit/failure/skip/retry/cleanup results and complete candidate identity. A committed audit does not authorize staging; integrated acceptance must observe integrated behavior. |

## Cross-cutting evidence discipline

Use [case design](../../references/case-design.md) for decision/condition influence
and semantic control selection. Require observed discriminating cases; do not
infer tool-supported MC/DC from a hand-written decision table. Use
[numerical/protocol truth](../../references/numerical-protocol.md) for mathematical
and wire expectations, and [harness qualification](../../references/harness-qualification.md)
for actual-input forwarding, observer sensitivity, failure propagation, order
pollution and fresh-output isolation.

Preserve the [testing contract](../../CONTRACT.md)'s 90% executable-line gate for
security-primitive modules and 95% mutation gate for each critical source file.
Those metrics supplement semantic, branch, adversarial and integration evidence;
they do not prove all requirements. A docs-only change does not require unrelated
product campaigns. Changed test/verifier criteria must retain accepted authority.

Apply current [OD-10](../../../../../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026):
formal compiler/solver/proof work and pending proof admission/metadata repairs
resume after V1. Record affected obligations in their existing owners; missing
proofs are no V1 blocker. Read current task/incident authority for each Windows,
fuzz, storage, UI or analyzer scope; general permission, one admitted fixture or
one blocked lane cannot silently authorize or prohibit all neighboring work.

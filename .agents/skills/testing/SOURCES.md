# Testing adaptation and source evidence

Research checked **28 October 2026**. Provenance/selection reasoning; no executed product verification. Canonical [architecture](../../../docs/windows-time-architecture.md), especially sections 6, 16, 18 and 20, controls requirements. CHECK-1–27 catalog planned; no retained passing run. Tools remain candidates unless adopted.

## Chrony testing-tool references — 30 October 2026

The [upstream tools reference](references/chrony-upstream-tools.md) records inspected
clknetsim `e9f11e60c72f2c0674a072c4ddcfe343f8634e58` and chrony-fuzz
`904a8789164ac294ab7e85523792ed906a7b5a57`, their primary sources, file-specific
rights and harness limits. Existing port/fuzz skills route scenario and seed
reuse without importing code/corpus, switching engines or admitting the C/build
graphs. Invasive protocol/crypto bypasses preclude treating patched outputs as
independent security truth. Source inspection runs no upstream campaign.

## Chrony and NTPsec mechanisms — 30 October 2026

The [source-task assessment](../../../tasks/s0.1-protocol-comparison.md#chrony-and-ntpsec-mechanism-assessment)
records Chrony 4.9 `8df4f1263e206585e6c4e61352d042c2dd12916a` and NTPsec
`097ea276d9e1b860aac548830a8bc03f1ceb6889`, exact functions and file-specific rights.
The port recipe, numerical/protocol truth and security leaves add consumer-level
aging/correction, scheduling, atomic KE publication, send-guard and restoration
checks. Shared lineage and implementation-derived tests are not independent truth.
This original local guidance imports no source/test bodies, tuning constants,
second controller or dependency graph; no upstream/native execution is claimed.

The repeated-proposal review also checks [RFC 9109 §§3.2–4](https://www.rfc-editor.org/rfc/rfc9109.html#section-4)
and [RFC 8915 §5.7](https://www.rfc-editor.org/rfc/rfc8915.html#section-5.7)
for port-lifecycle qualification and the control-only KoD/NAK test distinction.

## Skill-quality procedures — 30 October 2026

Original local procedures extend existing numerical, harness, mutation and
campaign skills; `qa-skill-evaluations` is registered separately in `LOCAL.json`.
No donor mapping or imported-file hash is changed. Corrected local findings in
[S1.1e](../../../tasks/s1.1-deterministic-harness.md),
[S1.1c mutation review](../../../tasks/s1.1-time-domain-equivalences.md) and
[S1.2 preparation](../../../tasks/s1.2-regression-readiness.md) motivated reusable
methods; their numeric limits and candidate results remain task-specific.

[Anthropic's agent-evaluation research](https://www.anthropic.com/research/demystifying-evals-for-ai-agents),
published 9 January 2026 and checked 30 October 2026, supports isolated tasks,
outcome-based grading and separately identified trials. The local exercise,
Rust fixture and standard-library grader are original examples, with no imported
implementation or framework dependency. Execution receipts and limits belong to
[the skill-quality task](../../plans/2026-10-30-skill-quality.md), not this source
registry. Neither packaging validation nor a single exercise proves general agent
reliability, application behavior or an improvement over untested old instructions.

## Donor skills and adaptations

Original user-owned source: `/home/david_osipov/personal-website-templated-1/.agents/skills/testing/`. All 36 entrypoints accounted for below. Retained central testing contract, independent expectations/controls, causal repair, evidence custody, finite stabilization, proportional qualification. Rust/native Windows evidence replaces website browser/DOM/Vitest/Cloudflare details. No imported per-leaf duplicate shared trees/website command wrappers/website fixture data. Shared guidance has one canonical owner under `references/`.

| Website skill | NevroTick skill |
| --- | --- |
| `qa-accessibility` | [qa-accessibility](qa-accessibility/SKILL.md) |
| `qa-adversarial-inputs` | [qa-adversarial-inputs](qa-adversarial-inputs/SKILL.md) |
| `qa-black-box-acceptance` | [qa-black-box-acceptance](qa-black-box-acceptance/SKILL.md) |
| `qa-boundary-edge-cases` | [qa-boundary-edge-cases](qa-boundary-edge-cases/SKILL.md) |
| `qa-build-content-metadata` | [qa-package-artifacts](qa-package-artifacts/SKILL.md) |
| `qa-component-tests` | [qa-component-tests](qa-component-tests/SKILL.md) |
| `qa-context-checkpoint` | [qa-context-checkpoint](qa-context-checkpoint/SKILL.md) |
| `qa-contract-tests` | [qa-contract-tests](qa-contract-tests/SKILL.md) |
| `qa-coverage-gap-analysis` | [qa-coverage-gap-analysis](qa-coverage-gap-analysis/SKILL.md) |
| `qa-e2e-tests` | [qa-e2e-tests](qa-e2e-tests/SKILL.md) |
| `qa-environment-preflight` | [qa-environment-preflight](qa-environment-preflight/SKILL.md) |
| `qa-failure-triage` | [qa-failure-triage](qa-failure-triage/SKILL.md) |
| `qa-fault-recovery` | [qa-fault-recovery](qa-fault-recovery/SKILL.md) |
| `qa-final-qualification` | [qa-final-qualification](qa-final-qualification/SKILL.md) |
| `qa-golden-fixture-parity` | [qa-golden-fixture-parity](qa-golden-fixture-parity/SKILL.md) |
| `qa-i18n-localization` | [qa-i18n-localization](qa-i18n-localization/SKILL.md) |
| `qa-independent-review` | [qa-independent-review](qa-independent-review/SKILL.md) |
| `qa-integration-tests` | [qa-integration-tests](qa-integration-tests/SKILL.md) |
| `qa-model-based-testing` | [qa-model-based-testing](qa-model-based-testing/SKILL.md) |
| `qa-mutation-testing` | [qa-mutation-testing](qa-mutation-testing/SKILL.md) |
| `qa-negative-paths` | [qa-negative-paths](qa-negative-paths/SKILL.md) |
| `qa-performance-resources` | [qa-performance-resources](qa-performance-resources/SKILL.md) |
| `qa-positive-paths` | [qa-positive-paths](qa-positive-paths/SKILL.md) |
| `qa-pre-commit` | [qa-pre-commit](qa-pre-commit/SKILL.md) |
| `qa-property-based-testing` | [qa-property-based-testing](qa-property-based-testing/SKILL.md) |
| `qa-regression-selection` | [qa-regression-selection](qa-regression-selection/SKILL.md) |
| `qa-security-boundaries` | [qa-security-boundaries](qa-security-boundaries/SKILL.md) |
| `qa-state-concurrency-lifecycle` | [qa-state-concurrency-lifecycle](qa-state-concurrency-lifecycle/SKILL.md) |
| `qa-static-checks` | [qa-static-checks](qa-static-checks/SKILL.md) |
| `qa-structured-fuzzing` | [qa-structured-fuzzing](qa-structured-fuzzing/SKILL.md) |
| `qa-subagent-dispatch` | [qa-subagent-dispatch](qa-subagent-dispatch/SKILL.md) |
| `qa-system-stabilization` | [qa-system-stabilization](qa-system-stabilization/SKILL.md) |
| `qa-test-orchestrator` | [qa-test-orchestrator](qa-test-orchestrator/SKILL.md) |
| `qa-test-planning` | [qa-test-planning](qa-test-planning/SKILL.md) |
| `qa-unit-tests` | [qa-unit-tests](qa-unit-tests/SKILL.md) |
| `qa-visual-responsive` | [qa-visual-dpi](qa-visual-dpi/SKILL.md) |

Website English/Russian baseline adapted to architecture English-first scope; Russian conditional. Artifact/content metadata: emitted PE/MSI/payload/provenance inspection. Responsive visual checks: native DPI/text scaling/multiple monitors. Preserve architecture 90% security-module executable-line/95% per-critical-file mutation implementation/release gates; no fabricated application checks for instruction files.

## Earlier user-owned Rust adaptation

The user supplied [PdfCraft PR 2912](https://github.com/storytold/pdfcraft/pull/2912). It is a closed, unmerged proposal; it does not establish an installed or qualified toolset here. Inspected the fork head `DavidOsipov/pdfcraft@c9a02b378271f92a73cb44be9fd9e4acc6921516`:

- [Rust verification workflow](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-qa-unit-tests/references/rust-workflow.md): explicit target/feature/compiler matrix, separate build/test concurrency, Cargo flag scope, doctests and runner isolation, actual external consumers. Adapted into `references/rust-techniques.md`, static/contract and environment leaves; no PdfCraft package/version/xtask assumptions retained.
- [Evidence identity](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-qa-unit-tests/references/evidence.md) and [task contract](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-qa-unit-tests/references/contract.md): selected versus executed semantic identities, first failures, known limitations and actual candidate identity. Adapted into this testing contract and qualification/coordination leaves; no nonexistent evidence command/schema introduced.
- [Staged-candidate preparation](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-qa-prepare-commit/references/commit-preparation.md): full-index candidate, transformation/input completeness, ownership and expiring readiness. Condensed into `references/staged-candidate.md` and qa-pre-commit; no automatic exporter, index replacement or staging grant.
- [Independent generation/replay](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-qa-structured-fuzzing/references/generative.md): per-case reset, exact input replay, minimization distinctions and modeled boundary limits. Adapted into Rust techniques and generated-test leaves.

David Osipov's 2026 authorship/MIT grant was inspected in [the pinned notice](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-rust-implementation/THIRD_PARTY_NOTICES.md). Pack-wide retained notices and donor identity are documented in [PROVENANCE.md](../PROVENANCE.md). External linked implementations and third-party skill bodies were not imported.

## Primary Rust and Windows research

| Evidence | Consequence for this pack | Limit |
| --- | --- | --- |
| [Cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html), [Cargo targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html), [rustdoc tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html) | Check real selection/features/ignored tests and named execution; all-targets does not include doctests; compiler rejection must have the intended diagnostic | Rolling documentation is not proof of a chosen toolchain's APIs or of executed tests |
| [Clippy usage](https://doc.rust-lang.org/clippy/usage.html) | Use repository/pinned-toolchain policy; select restriction/nursery lints deliberately and validate enforcement with valid/invalid controls | New Cargo warning configuration described in rolling docs cannot be assumed on the documented host/toolchain |
| [Proptest failure persistence](https://proptest-rs.github.io/proptest/proptest/failure-persistence.html), [upstream implementation](https://github.com/proptest-rs/proptest/blob/main/proptest/src/test_runner/failure_persistence/mod.rs) | Preserve seeds and concrete minimized inputs; strategy changes can alter seed replay; keep named deterministic regressions | Optional framework; no dependency added, runner executed or version selected |
| [Loom upstream README](https://github.com/tokio-rs/loom/blob/master/README.md) | Explicit primitive instrumentation and bounded small models; acknowledge documented model incompleteness/false alarms | No automatic OS/FFI coverage or all-interleavings proof |
| [Tokio pause](https://docs.rs/tokio/latest/tokio/time/fn.pause.html), [Tokio advance](https://docs.rs/tokio/latest/tokio/time/fn.advance.html) | Paused Tokio time is separate from std/Windows time; require current-thread/time-test support and observable completion | Runtime helper cannot prove suspend/resume or native actuation |
| [cargo-fuzz README](https://github.com/rust-fuzz/cargo-fuzz/blob/main/README.md), [Rust Fuzz Book setup](https://rust-fuzz.github.io/book/cargo-fuzz/setup.html), [Windows setup](https://rust-fuzz.github.io/book/cargo-fuzz/windows/setup.html) | **Source conflict:** README says Unix-like only/not Windows; current book describes Windows MSVC AddressSanitizer. Require exact release/runner smoke qualification and retain the disagreement | Neither source alone establishes a usable host harness; no nightly/tool installation or fuzz result here |
| [cargo-mutants upstream](https://github.com/sourcefrog/cargo-mutants/blob/main/README.md), [tool guide](https://mutants.rs/) | Use stable baseline and bounded requirement-grounded mutations; reached code differs from assertion sensitivity | Tool is optional until adopted; full critical-file gate still requires actual per-file evidence |
| [Microsoft native accessibility tools](https://learn.microsoft.com/en-us/windows/win32/winauto/accessibility-testingtools), [Inspect](https://learn.microsoft.com/en-us/windows/win32/winauto/inspect-objects/) | UIA inspection, keyboard and Narrator observations remain distinct; prefer Accessibility Insights, with legacy Inspect fallback | Tool output alone is not complete native accessibility qualification |
| [Microsoft DPI behavior](https://learn.microsoft.com/en-us/windows/win32/hidpi/high-dpi-desktop-application-development-on-windows) | Exercise live DPI changes across monitors; manifest declaration alone is insufficient | Retain canonical native Win32 choice; the page's generic UWP recommendation does not replace project architecture |
| [MSI rollback custom actions](https://learn.microsoft.com/en-us/windows/win32/msi/rollback-custom-actions), [Windows Installer practices](https://learn.microsoft.com/en-us/windows/desktop/Msi/windows-installer-best-practices) | Fault-test actual compensation and rollback-disabled behavior; arbitrary service/system changes are not automatically restored | Baseline remains a complete per-machine MSI; generic per-user advice is not a product requirement |

Online sources were read through the web tool and public upstream GitHub connector. No application compilation, native Windows check, property/mutation/fuzz campaign or release qualification was executed as part of authoring this testing pack.

## EGM review coordination adaptation — 29 October 2026

The [review and handoff reference](../planning/references/review-and-handoff.md)
records exact source paths from Google's Elephant-Goldfish Model at
`f0ba151b252c9abf1318d9fe857a43265d06de89`, its Apache-2.0 license and modifications.
Dispatch, checkpoint and coverage skills use that one reference for actual
reviewer completion, stable findings and authority/stage-aware document comparison.
It preserves failed/missing evidence, current authorization and focused rechecks;
reviewer counts or budget exhaustion establish neither correctness nor accepted
risk. No EGM runtime, tester, plugin metadata or fixed swarm was installed.

## CodeQL data-flow workflow refinement — 29 October 2026

Improved the existing locally authored `qa-data-flow-analysis` leaf and its
shared references; no additional overlapping skill or imported skill body.
Context7 `/github/codeql`, official documentation, the installed CodeQL MCP and
pinned library source were compared. Live examples and retrieval summaries are
discovery aids: the admitted CLI 2.27.3 / Rust library 0.2.24 and its source SHA
`a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c` control executable recipes.

| Primary evidence | Applied guidance |
| --- | --- |
| [Rust flow guide](https://codeql.github.com/docs/codeql-language-guides/analyzing-data-flow-in-rust/), [pinned Node API](https://github.com/github/codeql/blob/a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c/rust/ql/lib/codeql/rust/dataflow/internal/Node.qll) | Local/global value flow versus taint; the installed `asExpr()` returns AST `Expr` directly, unlike the newer guide's CFG-node example. Compile against the admitted pack. |
| [Pinned call API](https://github.com/github/codeql/blob/a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c/rust/ql/lib/codeql/rust/elements/internal/CallImpl.qll) | Identify actual callees, runtime targets, receiver and positional argument; fixture name matching is not a production identity model. |
| [Path queries](https://codeql.github.com/docs/writing-codeql-queries/creating-path-queries/), [BQRS interpretation](https://docs.github.com/en/code-security/reference/code-scanning/codeql/codeql-cli-manual/bqrs-interpret) | Flow-specific `PathGraph` and path-node columns; reconcile alert rows, path steps, pagination and limits separately. |
| [Partial-flow debugging](https://codeql.github.com/docs/writing-codeql-queries/debugging-data-flow-queries-using-partial-flow/), [pinned implementation](https://github.com/github/codeql/blob/a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c/shared/dataflow/codeql/dataflow/internal/DataFlowImplStage1.qll) | Verify source/sink membership first, then bounded forward/reverse exploration. Partial reachability does not establish a complete flow or absence. |
| [Rust build options](https://docs.github.com/en/code-security/reference/code-scanning/codeql/build-options-for-compiled-languages), [pinned extractor](https://github.com/github/codeql/blob/a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c/rust/codeql-extractor.yml), [cfg handling](https://github.com/github/codeql/blob/a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c/rust/extractor/src/config.rs) | `none` can run build scripts/proc macros. Host target, all features, test cfg and signature-only dependencies require explicit coverage accounting. The current repository launcher exposes no matrix override. Contrary to a retrieved summary, `-test` can disable the initially enabled test cfg. |
| [Modeling guide](https://codeql.github.com/docs/codeql-language-guides/customizing-library-models-for-rust/), [pinned model predicates](https://github.com/github/codeql/blob/a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c/rust/ql/lib/codeql/rust/dataflow/internal/ModelsAsData.qll), [pack manifest](https://github.com/github/codeql/blob/a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c/rust/ql/lib/qlpack.yml) | Property-specific barriers, branch-sensitive guards and justified summaries; pinned YAML extension loading, rather than assuming newer JSON guidance applies. |
| [Installed MCP source](https://github.com/advanced-security/codeql-development-mcp-server/tree/b0fae2809de0a5ff4fb984677354b7015b6ad57f), [local policy](../../../policy/codeql-mcp.json) | Discover actual schemas; `query_compile.database` is incompatible with the CLI, imported queries need real-file compilation, outputs stay in the admitted workspace, and expected results must not be auto-accepted. |

The recipe and any isolated fixture controls qualify query-development mechanics
only. They supply no current product extraction, Windows behavior, vulnerability
absence or hosted-scanning result; candidate-specific receipts remain outside
the skills pack.
